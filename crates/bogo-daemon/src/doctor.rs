//! `bogo doctor`: check each thing a working setup needs, and say in one plain line what is
//! wrong and how to fix it.
use crate::owner::{self, Command};
use crate::setup::{self, CHECK_UPSTREAM};
use crate::{Result, error};
use std::path::Path;

struct Report {
    failed: usize,
}
impl Report {
    fn ok(&mut self, what: &str) {
        println!("ok    {what}");
    }
    fn fail(&mut self, what: &str, fix: &str) {
        self.failed += 1;
        println!("FAIL  {what}. Fix: {fix}");
    }
    fn note(&mut self, what: &str) {
        println!("note  {what}");
    }
}

pub async fn doctor(root: &Path, client_home: Option<&Path>) -> Result<()> {
    let mut r = Report { failed: 0 };
    if !root.join("bogo.toml").exists() {
        r.fail(
            &format!("there is no BOGO home at {}", root.display()),
            "run `bogo setup`",
        );
        return finish(r);
    }
    let (config, _) = match owner::load(root) {
        Ok(c) => {
            r.ok("bogo.toml and secrets.toml load");
            c
        }
        Err(e) => {
            r.fail(
                &format!("BOGO's config does not load: {e}"),
                "run `bogo setup home`, or correct the line it names",
            );
            return finish(r);
        }
    };
    let status = owner::request(root, Command::Status).await;
    let running = matches!(&status, Ok(v) if v["running"] == true);
    let port = config.server.port;
    if running {
        r.ok("BOGO is running");
        match tokio::net::TcpStream::connect(("127.0.0.1", port)).await {
            Ok(_) => r.ok(&format!("BOGO answers on port {port}")),
            Err(_) => r.fail(
                &format!("BOGO runs but nothing answers on port {port}"),
                "restart it with `bogo stop` then `bogo start`",
            ),
        }
    } else {
        let why = match &status {
            Err(e) => format!(" ({e})"),
            _ => String::new(),
        };
        r.fail(
            &format!("BOGO is not running{why}"),
            &format!(
                "run `bogo start`; if it stops again, read {}",
                root.join("run/stderr.log").display()
            ),
        );
        match std::net::TcpListener::bind(("127.0.0.1", port)) {
            Ok(_) => r.ok(&format!("port {port} is free for BOGO")),
            Err(_) => r.fail(
                &format!("port {port} is taken by another program"),
                "set another [server] port in bogo.toml, or stop that program",
            ),
        }
    }

    let reg = setup::registry(root)?;
    let approvers: Vec<_> = reg.active_approvers().collect();
    let phones = approvers
        .iter()
        .filter(|a| a.entry.kind == "iphone")
        .count();
    match (phones, approvers.len()) {
        (0, 0) => r.fail("no phone is paired", "run `bogo setup phone`"),
        (0, n) => r.note(&format!(
            "{n} software test approver(s) and no iPhone: test approvers never use Face ID, so never protect real actions with them; `bogo setup phone` adds your iPhone"
        )),
        (n, _) => r.ok(&format!("{n} iPhone(s) paired")),
    }

    match &config.iphone {
        None => r.fail(
            "bogo.toml has no [approvers.iphone], so no iPhone can pair",
            "run `bogo setup home`, which adds it",
        ),
        Some(i) => {
            let prod = i
                .app_attest
                .contains(&bogo_config::AttestEnvironment::Production);
            let dev = i
                .app_attest
                .contains(&bogo_config::AttestEnvironment::Development);
            if !prod {
                r.fail(
                    "app_attest lists only \"development\", so this home rejects the TestFlight and App Store app",
                    "set app_attest = [\"production\", \"development\"] under [approvers.iphone]",
                );
            } else if dev {
                r.ok("App Attest accepts TestFlight, App Store and Xcode builds");
            } else {
                r.ok("App Attest accepts TestFlight and App Store builds");
            }
            if i.app_id != setup::APP_ID {
                r.note(&format!(
                    "app_id is {}, not the BOGO app's {}; only a build with that ID pairs",
                    i.app_id,
                    setup::APP_ID
                ));
            }
        }
    }

    // The relay: any HTTP answer means the network path and TLS work.
    let relay = &config.relay.url;
    let root_pem = std::env::var_os("BOGO_RELAY_CA")
        .map(std::fs::read)
        .transpose()
        .map_err(error)?
        .map(|p| reqwest::Certificate::from_pem(&p))
        .transpose()
        .map_err(|_| "BOGO_RELAY_CA is not a PEM certificate")?;
    let http = bogo_relay_proto::Client::http(root_pem).map_err(error)?;
    let reached = tokio::time::timeout(std::time::Duration::from_secs(10), http.get(relay).send())
        .await
        .is_ok_and(|r| r.is_ok());
    match reached {
        true => r.ok(&format!("the relay {relay} is reachable")),
        false => r.fail(
            &format!("cannot reach the relay {relay}"),
            "check this Mac's internet connection; a self-hosted relay must be up (docs/self-hosting-relay.md)",
        ),
    }

    // Upstreams: the running daemon's view, else start each one once.
    let health: std::collections::BTreeMap<String, bool> = if running {
        status
            .as_ref()
            .ok()
            .and_then(|v| serde_json::from_value(v["upstreams"].clone()).ok())
            .unwrap_or_default()
    } else if config.upstreams.is_empty() {
        Default::default()
    } else {
        let text = std::fs::read_to_string(root.join("bogo.toml")).map_err(error)?;
        let secrets = std::fs::read_to_string(root.join("secrets.toml")).map_err(error)?;
        setup::discover(&text, &secrets).await?.1
    };
    let log = root.join("run/stderr.log");
    if config.upstreams.keys().all(|u| u == CHECK_UPSTREAM) {
        r.note("no upstreams yet: add one with `bogo upstream add gmail` (or github, slack, stripe, files)");
    }
    for name in config.upstreams.keys() {
        if health.get(name) == Some(&true) {
            r.ok(&format!("upstream {name} starts"));
        } else {
            r.fail(
                &format!("upstream {name} does not start"),
                &format!(
                    "check its command and credentials in bogo.toml and secrets.toml; BOGO's log {} says why",
                    log.display()
                ),
            );
        }
    }

    // Each agent's client config should reach BOGO with that agent's token.
    let home = setup::client_home(client_home)?;
    let url = format!("http://127.0.0.1:{port}/mcp");
    let mut connected = vec![];
    for c in setup::clients(&home) {
        let Ok(bytes) = std::fs::read(&c.path) else {
            continue;
        };
        let Ok(doc) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
            r.fail(
                &format!("{} is not valid JSON", c.path.display()),
                "repair it, or run `bogo setup --undo`",
            );
            continue;
        };
        let entry = &doc["mcpServers"]["bogo"];
        if entry.is_null() {
            r.note(&format!("{} does not go through BOGO", c.name));
            continue;
        }
        let command_ok = entry["command"]
            .as_str()
            .is_some_and(|p| Path::new(p).exists());
        let url_ok = entry["env"]["BOGO_URL"].as_str() == Some(url.as_str());
        let agent = entry["env"]["BOGO_AGENT_TOKEN_FILE"]
            .as_str()
            .and_then(|p| setup::token_hash(Path::new(p)))
            .and_then(|h| {
                reg.agents
                    .values()
                    .find(|a| !a.revoked && a.token_sha256 == h)
            });
        if !command_ok {
            r.fail(
                &format!(
                    "{}'s bogo entry names a bogo-shim that does not exist",
                    c.name
                ),
                "run `bogo setup agent` again",
            );
        } else if !url_ok {
            r.fail(
                &format!("{}'s bogo entry points at another port than {port}", c.name),
                "run `bogo setup agent` again",
            );
        } else if let Some(a) = agent {
            connected.push(a.name.clone());
            r.ok(&format!("{} reaches BOGO as agent {}", c.name, a.name));
        } else if entry["env"]["BOGO_AGENT_TOKEN"].is_string() {
            r.note(&format!(
                "{} keeps its BOGO token in its config file; `bogo setup agent` moves it to a private file",
                c.name
            ));
        } else {
            r.fail(
                &format!("{}'s BOGO token belongs to no active agent", c.name),
                "run `bogo setup agent` again",
            );
        }
    }
    for a in reg.agents.values().filter(|a| !a.revoked) {
        if !connected.contains(&a.name) && a.name != setup::CHECK_AGENT {
            r.note(&format!(
                "agent {} is in no client config BOGO knows (fine if you connected it by hand)",
                a.name
            ));
        }
    }
    finish(r)
}
fn finish(r: Report) -> Result<()> {
    if r.failed == 0 {
        println!("Everything BOGO needs is in place.");
        Ok(())
    } else {
        Err(format!("{} problem(s) found", r.failed))
    }
}
