//! `bogo setup`: one guided command from a new install to a protected agent.
//!
//! Each step says in one line what it does and skips itself when it is already done, so
//! setup is safe to run again. Every change to a client's config file is backed up first,
//! and `bogo setup --undo` puts each file back byte for byte.
use crate::owner::{self, Command};
use crate::{Result, error};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    io::{BufRead, Write},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    time::Duration,
};

/// The steps, in order. Each can also run on its own: `bogo setup STEP`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    Home,
    Start,
    Phone,
    Agent,
    Test,
}
impl std::str::FromStr for Step {
    type Err = String;
    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "home" => Step::Home,
            "start" => Step::Start,
            "phone" => Step::Phone,
            "agent" => Step::Agent,
            "test" => Step::Test,
            _ => return Err("the steps are home, start, phone, agent and test".into()),
        })
    }
}
pub struct Options {
    /// Answer every question with its suggested answer. Never opens a tool: a suggested
    /// `allow` stays `approve` until the owner confirms it.
    pub yes: bool,
    /// Where to look for client configs instead of `$HOME`.
    pub client_home: Option<PathBuf>,
    /// The approver kind the phone step registers: `iphone`, or `soft` in a test build.
    pub approver_kind: String,
}

/// The agent the test step makes for its one call, and revokes after.
pub(crate) const CHECK_AGENT: &str = "BOGO setup check";
pub(crate) const CHECK_UPSTREAM: &str = "bogo-check";
pub(crate) const CHECK_TOOL: &str = "bogo-check__ping";

pub(crate) struct Ask {
    pub yes: bool,
}
impl Ask {
    /// One question on the terminal. An empty answer, end of input or `--yes` takes the
    /// suggestion.
    pub fn line(&self, q: &str, suggested: &str) -> Result<String> {
        if self.yes {
            println!("{q} [{suggested}] {suggested}");
            return Ok(suggested.into());
        }
        print!("{q} [{suggested}] ");
        std::io::stdout().flush().map_err(error)?;
        let mut s = String::new();
        std::io::stdin().lock().read_line(&mut s).map_err(error)?;
        if !std::io::IsTerminal::is_terminal(&std::io::stdin()) {
            // Answers from a script: show them, so the transcript reads like a session.
            println!("{}", s.trim());
        }
        let s = s.trim();
        Ok(if s.is_empty() { suggested } else { s }.into())
    }
    pub fn yes_no(&self, q: &str, suggested: bool) -> Result<bool> {
        loop {
            let a = self.line(q, if suggested { "Y/n" } else { "y/N" })?;
            match a.to_ascii_lowercase().as_str() {
                "y/n" | "y/n " => return Ok(suggested),
                "y" | "yes" => return Ok(true),
                "n" | "no" => return Ok(false),
                _ => println!("Answer y or n."),
            }
        }
    }
    /// A credential, read with the terminal's echo off so it never shows on screen.
    pub fn secret(&self, q: &str) -> Result<Option<String>> {
        if self.yes {
            return Ok(None);
        }
        print!("{q} (hidden; leave empty to skip) ");
        std::io::stdout().flush().map_err(error)?;
        let tty = std::io::IsTerminal::is_terminal(&std::io::stdin());
        let stty = |arg: &str| {
            let _ = std::process::Command::new("stty")
                .arg(arg)
                .stdin(std::process::Stdio::inherit())
                .status();
        };
        if tty {
            stty("-echo");
        }
        let mut s = String::new();
        let read = std::io::stdin().lock().read_line(&mut s);
        if tty {
            stty("echo");
        }
        println!();
        read.map_err(error)?;
        let s = s.trim();
        Ok((!s.is_empty()).then(|| s.to_owned()))
    }
}

pub async fn setup(root: &Path, step: Option<Step>, o: &Options) -> Result<()> {
    let ask = Ask { yes: o.yes };
    let steps = match step {
        Some(s) => vec![s],
        None => vec![
            Step::Home,
            Step::Start,
            Step::Phone,
            Step::Agent,
            Step::Test,
        ],
    };
    for (i, s) in steps.iter().enumerate() {
        if steps.len() > 1 {
            print!("{}/{} ", i + 1, steps.len());
        }
        match s {
            Step::Home => home(root, &ask).await?,
            Step::Start => start(root).await?,
            Step::Phone => phone(root, &ask, &o.approver_kind).await?,
            Step::Agent => agent(root, &ask, o).await?,
            Step::Test => test(root).await?,
        }
    }
    if steps.len() > 1 {
        println!(
            "Done. `bogo doctor` checks this setup at any time; `bogo setup --undo` puts back the client files setup changed."
        );
    }
    Ok(())
}

// ---- Step 1: the home ----

async fn home(root: &Path, ask: &Ask) -> Result<()> {
    println!("Create BOGO's home at {}.", root.display());
    if !root.join("bogo.toml").exists() {
        owner::init_quiet(root)?;
        println!("  Created it, with new keys and a private secrets file.");
    } else {
        println!("  It exists; keeping it.");
    }
    let path = root.join("bogo.toml");
    let mut text = std::fs::read_to_string(&path).map_err(error)?;
    let doc: toml::Table = text.parse().map_err(|_| "bogo.toml is not valid TOML")?;
    let mut changed = false;
    if doc
        .get("defaults")
        .and_then(|d| d.get("max_pending"))
        .is_none()
    {
        let n = loop {
            let a = ask.line(
                "  How many requests may one agent have waiting at once?",
                "4",
            )?;
            match a.parse::<u32>() {
                Ok(n) if n > 0 => break n,
                _ => println!("  Give a whole number above 0."),
            }
        };
        text = set_line(&text, "max_pending", &n.to_string(), "[defaults]")?;
        changed = true;
    }
    // Another program (or another BOGO home) may hold the port: offer a free one.
    let port = doc
        .get("server")
        .and_then(|s| s.get("port"))
        .and_then(toml::Value::as_integer)
        .and_then(|p| u16::try_from(p).ok());
    if let Some(port) = port
        && !free(port)
        && !running(root).await
    {
        let next = (port.saturating_add(1)..u16::MAX)
            .find(|p| free(*p))
            .ok_or("no free port")?;
        let p = loop {
            let a = ask.line(
                &format!("  Port {port} is in use by another program. Which port should BOGO use?"),
                &next.to_string(),
            )?;
            match a.parse::<u16>() {
                Ok(p) if p > 0 && free(p) => break p,
                _ => println!("  Give a free port number."),
            }
        };
        text = replace_key(&text, "[server]", "port", &p.to_string())?;
        changed = true;
    }
    let iphone = doc.get("approvers").and_then(|a| a.get("iphone"));
    if iphone.is_none() {
        // A home made before the template listed the app: add it, accepting both App
        // Attest environments so TestFlight, App Store and Xcode builds all pair.
        text.push_str(&format!(
            "\n# The BOGO iPhone app. \"production\" accepts TestFlight and App Store builds,\n# \"development\" accepts builds run from Xcode.\n[approvers.iphone]\napp_id = {}\napp_attest = [\"production\", \"development\"]\n",
            q(APP_ID)
        ));
        changed = true;
    }
    if iphone.and_then(|i| i.get("approve_arm_delay")).is_none() {
        let d = loop {
            let a = ask.line(
                "  How long should Approve stay off when a request opens on the phone, so you read it first?",
                "2s",
            )?;
            if bogo_config::parse_duration(&a).is_some_and(|d| !d.is_zero()) {
                break a;
            }
            println!("  Give a duration such as 2s or 1500ms.");
        };
        text = set_line(&text, "approve_arm_delay", &q(&d), "[approvers.iphone]")?;
        changed = true;
    }
    if !doc
        .get("upstreams")
        .and_then(|u| u.get(CHECK_UPSTREAM))
        .is_some()
    {
        text.push_str(&check_upstream_toml()?);
        changed = true;
    }
    if changed {
        write_config(root, &text, None)?;
        println!("  Saved bogo.toml.");
    }
    Ok(())
}
pub(crate) const APP_ID: &str = "AX3C2B78YD.com.johansardar.applebridge.ios";

/// The setup check: one harmless tool, served by `bogo check-upstream`, hidden from every
/// agent but the check agent, and protected for it.
fn check_upstream_toml() -> Result<String> {
    let bin = std::env::current_exe().map_err(error)?;
    Ok(format!(
        "\n# BOGO's own check tool for `bogo setup test`: it only answers \"pong\".\n[upstreams.{CHECK_UPSTREAM}]\ncommand = {}\nargs = [\"check-upstream\"]\n[tools.{CHECK_TOOL}]\npolicy = \"deny\"\n[agents.{}.tools.{CHECK_TOOL}]\npolicy = \"approve\"\n",
        q(&bin.to_string_lossy()),
        q(CHECK_AGENT)
    ))
}

/// Set `key = value` in the table that `header` opens: replace the commented template line
/// (`# key = ...`) if there is one, else add the line just after the header.
fn set_line(text: &str, key: &str, value: &str, header: &str) -> Result<String> {
    let mut out = Vec::new();
    let mut done = false;
    let mut section = String::new();
    for line in text.lines() {
        let t = line.trim();
        if t.starts_with('[') {
            section = t.to_owned();
        }
        if !done
            && section == header
            && t.strip_prefix('#')
                .map(str::trim_start)
                .is_some_and(|r| r.starts_with(&format!("{key} =")))
        {
            out.push(format!("{key} = {value}"));
            done = true;
            continue;
        }
        out.push(line.to_owned());
        if !done && t == header {
            // Placed now unless a commented template line follows further down.
            let rest = text.split_once(line).map_or("", |(_, r)| r);
            let commented = rest
                .lines()
                .take_while(|l| !l.trim().starts_with('['))
                .any(|l| {
                    l.trim()
                        .strip_prefix('#')
                        .map(str::trim_start)
                        .is_some_and(|r| r.starts_with(&format!("{key} =")))
                });
            if !commented {
                out.push(format!("{key} = {value}"));
                done = true;
            }
        }
    }
    if !done {
        return Err(format!("could not place {key} in bogo.toml under {header}"));
    }
    Ok(out.join("\n") + "\n")
}

fn free(port: u16) -> bool {
    std::net::TcpListener::bind(("127.0.0.1", port)).is_ok()
}
/// Replace an existing `key = ...` line in the table `header` opens.
fn replace_key(text: &str, header: &str, key: &str, value: &str) -> Result<String> {
    let mut section = String::new();
    let mut done = false;
    let mut out = vec![];
    for line in text.lines() {
        let t = line.trim();
        if t.starts_with('[') {
            section = t.to_owned();
        }
        if !done && section == header && t.split_once('=').is_some_and(|(k, _)| k.trim() == key) {
            out.push(format!("{key} = {value}"));
            done = true;
        } else {
            out.push(line.to_owned());
        }
    }
    if !done {
        return Err(format!("could not find {key} under {header} in bogo.toml"));
    }
    Ok(out.join("\n") + "\n")
}
/// A TOML string, quoted and escaped.
pub(crate) fn q(s: &str) -> String {
    toml::Value::String(s.into()).to_string()
}
/// A TOML key: bare when it can be, else quoted.
pub(crate) fn k(s: &str) -> String {
    if !s.is_empty()
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
    {
        s.into()
    } else {
        q(s)
    }
}

/// Check `text` (with `secrets`, if given, as the new secrets file) the way the daemon will,
/// then save both. Nothing is written if either would be refused.
pub(crate) fn write_config(root: &Path, text: &str, secrets: Option<&str>) -> Result<()> {
    let home = bogo_state::Home::open(root).map_err(error)?;
    let _lock = home.lock().map_err(|e| match e {
        bogo_state::StateError::Locked(_) => {
            "BOGO is running from this home; stop it first, then run this again".to_owned()
        }
        e => e.to_string(),
    })?;
    let secrets_path = root.join("secrets.toml");
    let secrets_text = match secrets {
        Some(s) => s.to_owned(),
        None => std::fs::read_to_string(&secrets_path).map_err(error)?,
    };
    let parsed = bogo_config::Secrets::parse(
        &secrets_text,
        std::env::var_os("HOME").as_deref().map(Path::new),
    )
    .map_err(error)?;
    bogo_config::Config::parse(text, &parsed).map_err(error)?;
    if let Some(s) = secrets {
        bogo_state::write_atomic(&secrets_path, s.as_bytes()).map_err(error)?;
    }
    bogo_state::write_atomic(&root.join("bogo.toml"), text.as_bytes()).map_err(error)?;
    Ok(())
}

// ---- Step 2: start in the background ----

pub(crate) async fn running(root: &Path) -> bool {
    matches!(owner::request(root, Command::Status).await, Ok(v) if v["running"] == true)
}
async fn wait_for(root: &Path, up: bool, secs: u64) -> Result<()> {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(secs);
    while running(root).await != up {
        if tokio::time::Instant::now() > deadline {
            return Err(if up {
                format!(
                    "BOGO did not start within {secs}s. Run `bogo doctor`, and read {} for the reason.",
                    root.join("run/stderr.log").display()
                )
            } else {
                format!("BOGO did not stop within {secs}s")
            });
        }
        tokio::time::sleep(Duration::from_millis(300)).await;
    }
    Ok(())
}
async fn start(root: &Path) -> Result<()> {
    println!("Start BOGO in the background, so it runs at login.");
    if running(root).await {
        println!("  It is running; skipping.");
        return Ok(());
    }
    owner::launch(root, true)?;
    wait_for(root, true, 120).await?;
    println!("  Started.");
    Ok(())
}

/// Run `f` with BOGO stopped, then start it again if it was running in the background.
/// Config changes need this: BOGO reads its config only at startup.
pub(crate) async fn with_stopped<T>(root: &Path, f: impl FnOnce() -> Result<T>) -> Result<T> {
    if !running(root).await {
        return f();
    }
    if !owner::launchd_loaded() {
        return Err(
            "BOGO is running in a terminal (`bogo serve`); stop it there, then run this again"
                .into(),
        );
    }
    println!("  Restarting BOGO to load the new config.");
    owner::launch(root, false)?;
    wait_for(root, false, 30).await?;
    // The socket goes before the process does: wait until its writer lock is free too.
    let home = bogo_state::Home::open(root).map_err(error)?;
    let deadline = tokio::time::Instant::now() + Duration::from_secs(30);
    while let Err(bogo_state::StateError::Locked(_)) = home.lock() {
        if tokio::time::Instant::now() > deadline {
            return Err("BOGO did not stop within 30s".into());
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
    let result = f();
    owner::launch(root, true)?;
    wait_for(root, true, 120).await?;
    result
}

// ---- Step 3: the iPhone ----

pub(crate) fn registry(root: &Path) -> Result<bogo_log::Registry> {
    let path = root.join("log/audit.jsonl");
    if !path.exists() {
        return Ok(bogo_log::Registry::default());
    }
    let (entries, _) = bogo_log::read(&path).map_err(error)?;
    Ok(bogo_log::Registry::replay(&entries))
}
async fn phone(root: &Path, ask: &Ask, kind: &str) -> Result<()> {
    println!("Add your iPhone, so protected calls wait for your Face ID.");
    let reg = registry(root)?;
    if let Some(a) = reg.active_approvers().next() {
        println!("  {} is already an approver; skipping.", a.entry.name);
        return Ok(());
    }
    if !running(root).await {
        return Err("BOGO is not running; run `bogo setup start` first".into());
    }
    let name = ask.line("  What should BOGO call this phone?", "iPhone")?;
    if kind == "iphone" {
        println!(
            "  On the iPhone, open BOGO, then Macs, then Add Mac, and pick this Mac. Check both show the same 6 digits."
        );
    }
    let v = owner::request(
        root,
        Command::ApproverAdd {
            name: name.clone(),
            kind: kind.into(),
        },
    )
    .await?;
    println!(
        "  Added {} as an approver.",
        v["name"].as_str().unwrap_or(&name)
    );
    Ok(())
}

// ---- Step 4: connect an agent, importing its MCP servers ----

/// One MCP client's config file BOGO knows how to read.
pub(crate) struct Client {
    pub name: &'static str,
    pub path: PathBuf,
}
pub(crate) fn clients(home: &Path) -> Vec<Client> {
    vec![
        Client {
            name: "Claude Code",
            path: home.join(".claude.json"),
        },
        Client {
            name: "Claude Desktop",
            path: home.join("Library/Application Support/Claude/claude_desktop_config.json"),
        },
        Client {
            name: "Cursor",
            path: home.join(".cursor/mcp.json"),
        },
    ]
}
pub(crate) fn client_home(o: Option<&Path>) -> Result<PathBuf> {
    match o {
        Some(p) => Ok(p.to_path_buf()),
        None => std::env::var_os("HOME")
            .map(PathBuf::from)
            .ok_or_else(|| "HOME is not set; use --client-home".into()),
    }
}
pub(crate) fn slug(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase())
        } else if !out.ends_with('-') {
            out.push('-')
        }
    }
    out.trim_matches('-').to_owned()
}

/// A server found in a client's config, ready to become an upstream.
struct Found {
    client_key: String,
    upstream: String,
    /// The `[upstreams.NAME]` body, already written as TOML lines.
    toml: String,
    /// New `secrets.toml` entries: name, value.
    secrets: Vec<(String, String)>,
    /// Plain env or header values and secret references, as the temporary discovery config.
    moved: Vec<String>,
}

fn credential_name(name: &str) -> bool {
    let up = name.to_ascii_uppercase();
    up.contains("TOKEN")
        || up.contains("SECRET")
        || up.contains("PASSWORD")
        || up.contains("PASSWD")
        || up.contains("CREDENTIAL")
        || up.contains("APIKEY")
        || up.contains("COOKIE")
        || up.split(['_', '-']).any(|p| {
            matches!(
                p,
                "KEY" | "PAT" | "AUTH" | "AUTHORIZATION" | "PRIVATE" | "SESSION" | "BEARER"
            )
        })
}
/// Whether an argument list carries a credential: `--token=…`, `--api-key …` and the like.
fn credential_in_args(args: &[String]) -> bool {
    args.iter().enumerate().any(|(i, a)| {
        let flag = a.trim_start_matches('-');
        if !a.starts_with('-') {
            return false;
        }
        match flag.split_once('=') {
            Some((name, value)) => credential_name(name) && !value.is_empty(),
            None => credential_name(flag) && args.get(i + 1).is_some_and(|n| !n.starts_with('-')),
        }
    })
}
/// The absolute path of a command, searched on PATH like a shell would. Launchd needs it.
fn resolve(command: &str) -> Option<String> {
    if command.contains('/') {
        return Some(command.into());
    }
    std::env::var_os("PATH").and_then(|paths| {
        std::env::split_paths(&paths)
            .map(|d| d.join(command))
            .find(|p| {
                p.metadata()
                    .is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
            })
            .map(|p| p.to_string_lossy().into_owned())
    })
}

fn found(
    key: &str,
    v: &Value,
    taken: &mut Vec<String>,
    secret_names: &mut Vec<String>,
) -> std::result::Result<Found, String> {
    let mut upstream = slug(key);
    if upstream.is_empty() {
        upstream = "server".into();
    }
    if upstream == "bogo" || upstream == CHECK_UPSTREAM {
        upstream.push_str("-upstream");
    }
    let base = upstream.clone();
    let mut n = 2;
    while taken.contains(&upstream) {
        upstream = format!("{base}-{n}");
        n += 1;
    }
    let mut secrets = vec![];
    let mut lines = vec![];
    let mut moved = vec![];
    let mut secret_for = |field: &str, value: &str| -> std::result::Result<String, String> {
        if value.trim().chars().count() < 8 {
            return Err(format!(
                "its credential {field} is shorter than 8 characters, so BOGO couldn't hide it in what the upstream sends back; move it by hand"
            ));
        }
        let base = format!(
            "{}_{}",
            upstream.replace('-', "_"),
            slug(field).replace('-', "_")
        );
        let mut name = base.clone();
        let mut n = 2;
        while secret_names.contains(&name) {
            name = format!("{base}_{n}");
            n += 1;
        }
        secret_names.push(name.clone());
        secrets.push((name.clone(), value.to_owned()));
        Ok(name)
    };
    let kind = v["type"].as_str().unwrap_or("");
    if let Some(url) = v["url"].as_str() {
        if kind == "sse" {
            return Err("it uses the older SSE transport, which BOGO doesn't connect to".into());
        }
        lines.push(format!("url = {}", q(url)));
        if let Some(h) = v["headers"].as_object().filter(|h| !h.is_empty()) {
            let mut items = vec![];
            for (name, value) in h {
                let value = value.as_str().ok_or("a header value is not text")?;
                if credential_name(name) {
                    let s = secret_for(name, value)?;
                    items.push(format!("{} = {{ secret = {} }}", k(name), q(&s)));
                } else {
                    items.push(format!("{} = {}", k(name), q(value)));
                }
            }
            lines.push(format!("headers = {{ {} }}", items.join(", ")));
        }
    } else {
        let command = v["command"].as_str().ok_or("it has no command or url")?;
        let args: Vec<String> = match &v["args"] {
            Value::Null => vec![],
            Value::Array(a) => a
                .iter()
                .map(|a| {
                    a.as_str()
                        .map(str::to_owned)
                        .ok_or("an argument is not text")
                })
                .collect::<std::result::Result<_, _>>()?,
            _ => return Err("its args are not a list".into()),
        };
        if credential_in_args(&args) {
            return Err(
                "it passes a credential in its arguments; move that to an environment variable first, then run setup again"
                    .into(),
            );
        }
        let exe =
            resolve(command).ok_or_else(|| format!("its command {command} is not on PATH"))?;
        lines.push(format!("command = {}", q(&exe)));
        lines.push(format!(
            "args = [{}]",
            args.iter().map(|a| q(a)).collect::<Vec<_>>().join(", ")
        ));
        if let Some(env) = v["env"].as_object().filter(|e| !e.is_empty()) {
            let mut items = vec![];
            for (name, value) in env {
                let value = value.as_str().ok_or("an env value is not text")?;
                if credential_name(name) {
                    let s = secret_for(name, value)?;
                    items.push(format!("{} = {{ secret = {} }}", k(name), q(&s)));
                    moved.push(name.clone());
                } else {
                    items.push(format!("{} = {}", k(name), q(value)));
                }
            }
            lines.push(format!("env = {{ {} }}", items.join(", ")));
        }
    }
    taken.push(upstream.clone());
    Ok(Found {
        client_key: key.into(),
        upstream: upstream.clone(),
        toml: format!("[upstreams.{upstream}]\n{}\n", lines.join("\n")),
        secrets,
        moved,
    })
}

/// The upstream already configured for this client entry, if any.
fn existing(config: &bogo_config::Config, v: &Value) -> Option<String> {
    let args: Vec<String> = v["args"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|s| s.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default();
    let command = v["command"].as_str().and_then(resolve);
    config.upstreams.iter().find_map(|(name, u)| {
        let same = match u {
            bogo_config::Upstream::Local {
                command: c,
                args: a,
                ..
            } => command.as_deref() == Some(c.as_str()) && *a == args,
            bogo_config::Upstream::Remote { url, .. } => v["url"].as_str() == Some(url.as_str()),
        };
        same.then(|| name.clone())
    })
}
/// The policy setup suggests for one tool, from the annotations its server publishes.
/// Only `readOnlyHint` suggests allow; a write, a send or no hint at all suggests approve.
pub(crate) fn suggest(tool: &Value) -> (&'static str, &'static str) {
    let a = &tool["annotations"];
    let name = tool["name"]
        .as_str()
        .unwrap_or_default()
        .to_ascii_lowercase();
    let risky = [
        "send", "pay", "write", "delete", "remove", "create", "update", "edit", "move", "post",
        "push", "merge", "transfer", "charge", "refund", "publish", "execute", "run",
    ]
    .iter()
    .any(|w| name.split(['_', '-', '.']).any(|p| p == *w));
    if a["destructiveHint"] == true {
        ("approve", "it says it can destroy or change things")
    } else if risky {
        ("approve", "its name says it writes, sends, pays or deletes")
    } else if a["readOnlyHint"] == true {
        ("allow", "it says it only reads")
    } else {
        ("approve", "it publishes no hint that it only reads")
    }
}

/// List each new upstream's tools, the way BOGO will, without the daemon: start them with
/// a gate that refuses every call, read the catalog, stop them.
pub(crate) async fn discover(
    text: &str,
    secrets: &str,
) -> Result<(BTreeMap<String, Value>, BTreeMap<String, bool>)> {
    struct Refuse;
    impl bogo_upstream::Gate for Refuse {
        fn dispatch(
            &self,
            _: bogo_upstream::CallId,
        ) -> futures::future::BoxFuture<
            '_,
            std::result::Result<bogo_upstream::Forward, bogo_upstream::Error>,
        > {
            Box::pin(async { Err(bogo_upstream::Error::Rejected) })
        }
    }
    let secrets =
        bogo_config::Secrets::parse(secrets, std::env::var_os("HOME").as_deref().map(Path::new))
            .map_err(error)?;
    let config = bogo_config::Config::parse(text, &secrets).map_err(error)?;
    let manager = bogo_upstream::Manager::start(
        config,
        &secrets,
        std::sync::Arc::new(Refuse),
        Default::default(),
    )
    .await;
    let catalog = manager.catalog().await;
    let health = manager.health().await;
    Ok((
        catalog
            .tools
            .into_iter()
            .map(|(n, t)| (n, t.original))
            .collect(),
        health,
    ))
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct Undo {
    files: Vec<Backup>,
    #[serde(default)]
    agents: Vec<String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
struct Backup {
    path: PathBuf,
    /// The copy of the file as it was before setup first changed it.
    backup: PathBuf,
    mode: u32,
}
fn undo_path(root: &Path) -> PathBuf {
    root.join("setup/undo.json")
}
fn read_undo(root: &Path) -> Result<Undo> {
    match std::fs::read(undo_path(root)) {
        Ok(b) => serde_json::from_slice(&b).map_err(|_| "setup/undo.json is damaged".into()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Undo::default()),
        Err(e) => Err(error(e)),
    }
}
fn private_dir(p: &Path) -> Result<()> {
    std::fs::create_dir_all(p).map_err(error)?;
    std::fs::set_permissions(p, std::fs::Permissions::from_mode(0o700)).map_err(error)
}
/// Keep a byte-for-byte copy of `path` before its first change, for `--undo`.
fn back_up(root: &Path, undo: &mut Undo, path: &Path) -> Result<()> {
    if undo.files.iter().any(|b| b.path == path) {
        return Ok(());
    }
    let dir = root.join("setup/backups");
    private_dir(&root.join("setup"))?;
    private_dir(&dir)?;
    let meta = std::fs::metadata(path).map_err(error)?;
    let backup = dir.join(format!(
        "{}-{}",
        undo.files.len() + 1,
        path.file_name().unwrap_or_default().to_string_lossy()
    ));
    bogo_state::write_atomic(&backup, &std::fs::read(path).map_err(error)?).map_err(error)?;
    undo.files.push(Backup {
        path: path.into(),
        backup,
        mode: meta.permissions().mode() & 0o7777,
    });
    save_undo(root, undo)
}
fn save_undo(root: &Path, undo: &Undo) -> Result<()> {
    private_dir(&root.join("setup"))?;
    bogo_state::write_atomic(
        &undo_path(root),
        &serde_json::to_vec_pretty(undo).map_err(error)?,
    )
    .map_err(error)
}
/// Write a client's file in place, keeping its permissions.
fn write_client(path: &Path, bytes: &[u8]) -> Result<()> {
    let mode = std::fs::metadata(path)
        .map(|m| m.permissions().mode() & 0o7777)
        .unwrap_or(0o600);
    let tmp = path.with_extension("bogo-setup.tmp");
    std::fs::write(&tmp, bytes).map_err(error)?;
    std::fs::set_permissions(&tmp, std::fs::Permissions::from_mode(mode)).map_err(error)?;
    std::fs::rename(&tmp, path).map_err(error)
}

pub(crate) fn shim_path() -> Result<PathBuf> {
    let shim = std::env::current_exe()
        .map_err(error)?
        .with_file_name("bogo-shim");
    if !shim.exists() {
        return Err(format!(
            "bogo-shim is not next to bogo at {}; install both together",
            shim.display()
        ));
    }
    Ok(shim)
}
pub(crate) fn mcp_url(root: &Path) -> Result<String> {
    let (config, _) = owner::load(root)?;
    Ok(format!("http://127.0.0.1:{}/mcp", config.server.port))
}
pub(crate) fn token_path(root: &Path, agent: &str) -> PathBuf {
    root.join("agents").join(format!("{}.token", slug(agent)))
}
/// Register an agent and keep its token in a private file in BOGO's home, never on screen
/// and never in the client's config. Only the token's hash enters BOGO's log.
async fn new_agent(root: &Path, name: &str, undo: &mut Undo) -> Result<(String, PathBuf)> {
    let reg = registry(root)?;
    let mut agent = name.to_owned();
    let mut n = 2;
    while reg.agents.values().any(|a| !a.revoked && a.name == agent) {
        agent = format!("{name} {n}");
        n += 1;
    }
    let v = owner::request(
        root,
        Command::AgentAdd {
            name: agent.clone(),
        },
    )
    .await?;
    let token = v["token"].as_str().ok_or("agent add returned no token")?;
    private_dir(&root.join("agents"))?;
    let path = token_path(root, &agent);
    bogo_state::write_atomic(&path, format!("{token}\n").as_bytes()).map_err(error)?;
    undo.agents.push(agent.clone());
    save_undo(root, undo)?;
    println!("  Added agent {agent}; its token is in {}.", path.display());
    Ok((agent, path))
}
fn bogo_entry(shim: &Path, url: &str, token: &Path) -> Value {
    json!({"command": shim.to_string_lossy(), "args": [],
        "env": {"BOGO_URL": url, "BOGO_AGENT_TOKEN_FILE": token.to_string_lossy()}})
}

async fn agent(root: &Path, ask: &Ask, o: &Options) -> Result<()> {
    println!("Connect an agent: move each MCP client's servers behind BOGO.");
    let home = client_home(o.client_home.as_deref())?;
    let shim = shim_path()?;
    let url = mcp_url(root)?;
    let mut undo = read_undo(root)?;
    let mut any = false;
    for client in clients(&home) {
        let Ok(bytes) = std::fs::read(&client.path) else {
            continue;
        };
        any = true;
        let mut doc: Ordered = serde_json::from_slice(&bytes)
            .map_err(|_| format!("{} is not valid JSON; skipping it", client.path.display()))?;
        let servers = doc
            .get("mcpServers")
            .cloned()
            .unwrap_or(Ordered::Obj(vec![]));
        let Ordered::Obj(servers) = servers else {
            println!("  {}: mcpServers is not an object; skipping.", client.name);
            continue;
        };
        let has_bogo = servers.iter().any(|(k, _)| k == "bogo");
        let others: Vec<_> = servers.iter().filter(|(k, _)| k != "bogo").collect();
        if has_bogo && others.is_empty() {
            println!("  {}: already goes through BOGO; skipping.", client.name);
            continue;
        }
        println!(
            "  {} ({}) has {} MCP server(s).",
            client.name,
            client.path.display(),
            others.len()
        );
        // Each server the owner agrees to move becomes an upstream.
        let (config, _) = owner::load(root)?;
        let mut taken: Vec<String> = config.upstreams.keys().cloned().collect();
        let secrets_text = std::fs::read_to_string(root.join("secrets.toml")).map_err(error)?;
        let mut secret_names: Vec<String> = bogo_config::Secrets::parse(&secrets_text, None)
            .map_err(error)?
            .names()
            .map(str::to_owned)
            .collect();
        let mut moving = vec![];
        // Servers already behind BOGO (the same command and arguments, or URL): only the
        // client's entry goes, pointing at BOGO instead.
        let mut reused: Vec<String> = vec![];
        for (key, value) in &others {
            let v = value.to_value();
            if let Some(name) = existing(&config, &v) {
                println!("    {key} is already behind BOGO as upstream {name}.");
                reused.push(key.clone());
                continue;
            }
            match found(key, &v, &mut taken, &mut secret_names) {
                Ok(f) => {
                    if ask.yes_no(&format!("    Move {key} behind BOGO?"), true)? {
                        moving.push(f)
                    } else {
                        taken.retain(|t| *t != f.upstream);
                    }
                }
                Err(why) => println!("    Leaving {key} as it is: {why}."),
            }
        }
        if moving.is_empty() && reused.is_empty() && has_bogo {
            continue;
        }
        let mut new_secrets = secrets_text.clone();
        if !new_secrets.ends_with('\n') {
            new_secrets.push('\n');
        }
        for f in &moving {
            for (name, value) in &f.secrets {
                new_secrets.push_str(&format!("{} = {}\n", k(name), q(value)));
            }
        }
        // List the new servers' tools and suggest a policy for each.
        let mut policy = String::new();
        if !moving.is_empty() {
            let probe = format!(
                "version = 1\n[server]\nport = 1\n[defaults]\nmax_pending = 1\n{}",
                moving.iter().map(|f| f.toml.as_str()).collect::<String>()
            );
            println!("    Starting them once to list their tools…");
            let (tools, health) = discover(&probe, &new_secrets).await?;
            for f in &moving {
                if health.get(&f.upstream) != Some(&true) {
                    println!(
                        "    {} did not start; its tools wait for approval until it does.",
                        f.upstream
                    );
                }
                if !f.moved.is_empty() {
                    println!(
                        "    Moved {} of {} into secrets.toml.",
                        f.moved.join(", "),
                        f.upstream
                    );
                }
                for (name, tool) in tools.range(format!("{}__", f.upstream)..) {
                    if !name.starts_with(&format!("{}__", f.upstream)) {
                        break;
                    }
                    let (suggested, why) = suggest(tool);
                    let chosen = loop {
                        let a = ask.line(
                            &format!("    {name}: {why}. allow, approve or deny?"),
                            suggested,
                        )?;
                        match a.as_str() {
                            "allow" | "approve" | "deny" => break a,
                            _ => println!("    Answer allow, approve or deny."),
                        }
                    };
                    // --yes never opens a tool: allow needs the owner to say it.
                    let chosen = if ask.yes && chosen == "allow" {
                        println!(
                            "    Left at approve: --yes never allows a tool. Run `bogo setup agent` to choose."
                        );
                        "approve".to_owned()
                    } else {
                        chosen
                    };
                    policy.push_str(&format!("[tools.{}]\npolicy = {}\n", k(name), q(&chosen)));
                }
            }
        }
        // Write BOGO's side first; only then change the client.
        if !moving.is_empty() {
            let text = std::fs::read_to_string(root.join("bogo.toml")).map_err(error)?;
            let mut text = text.trim_end().to_owned() + "\n";
            for f in &moving {
                text.push_str(&format!(
                    "\n# Moved from {} by `bogo setup`.\n{}",
                    client.name, f.toml
                ));
            }
            text.push_str(&policy);
            with_stopped(root, || write_config(root, &text, Some(&new_secrets))).await?;
        }
        let (agent, token) = new_agent(root, client.name, &mut undo).await?;
        back_up(root, &mut undo, &client.path)?;
        let moved: Vec<&str> = moving
            .iter()
            .map(|f| f.client_key.as_str())
            .chain(reused.iter().map(String::as_str))
            .collect();
        let mut kept: Vec<(String, Ordered)> = servers
            .iter()
            .filter(|(k, _)| k != "bogo" && !moved.contains(&k.as_str()))
            .cloned()
            .collect();
        kept.insert(
            0,
            (
                "bogo".into(),
                Ordered::from_value(bogo_entry(&shim, &url, &token)),
            ),
        );
        doc.set("mcpServers", Ordered::Obj(kept));
        let mut out = serde_json::to_vec_pretty(&doc).map_err(error)?;
        out.push(b'\n');
        write_client(&client.path, &out)?;
        println!(
            "  {} now reaches {} through BOGO, as agent {}.",
            client.name,
            if moved.is_empty() {
                "its tools".to_owned()
            } else {
                moved.join(", ")
            },
            agent
        );
    }
    if !any {
        // No client config to change: give Claude Code a config file of its own.
        let (_, token) = new_agent(root, "Claude Code", &mut undo).await?;
        let dir = root.join("clients");
        private_dir(&dir)?;
        let file = dir.join("claude-code.mcp.json");
        let out = serde_json::to_vec_pretty(
            &json!({"mcpServers":{"bogo":bogo_entry(&shim,&url,&token)}}),
        )
        .map_err(error)?;
        bogo_state::write_atomic(&file, &out).map_err(error)?;
        println!(
            "  Found no Claude Code, Claude Desktop or Cursor config in {}. Run Claude Code through BOGO with:\n    claude --mcp-config {} --strict-mcp-config",
            home.display(),
            file.display()
        );
    }
    Ok(())
}

pub fn undo(root: &Path) -> Result<()> {
    let undo = read_undo(root)?;
    if undo.files.is_empty() && undo.agents.is_empty() {
        println!("Setup changed no client files here; nothing to undo.");
        return Ok(());
    }
    for b in &undo.files {
        let bytes = std::fs::read(&b.backup).map_err(error)?;
        let tmp = b.path.with_extension("bogo-undo.tmp");
        std::fs::write(&tmp, &bytes).map_err(error)?;
        std::fs::set_permissions(&tmp, std::fs::Permissions::from_mode(b.mode)).map_err(error)?;
        std::fs::rename(&tmp, &b.path).map_err(error)?;
        println!("Restored {}.", b.path.display());
    }
    std::fs::rename(undo_path(root), root.join("setup/undo.done.json")).map_err(error)?;
    if !undo.agents.is_empty() {
        println!(
            "The agents setup added stay registered until you revoke them: {}.",
            undo.agents
                .iter()
                .map(|a| format!("bogo agent revoke {}", shell_word(a)))
                .collect::<Vec<_>>()
                .join("; ")
        );
    }
    println!(
        "The upstreams and secrets setup added stay in bogo.toml and secrets.toml; remove them there if you no longer want them."
    );
    Ok(())
}
fn shell_word(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

// ---- Step 5: one protected test call ----

async fn test(root: &Path) -> Result<()> {
    println!("Make one protected test call, and approve it on your phone.");
    let (config, _) = owner::load(root)?;
    if !config.upstreams.contains_key(CHECK_UPSTREAM) {
        let text = std::fs::read_to_string(root.join("bogo.toml")).map_err(error)?;
        let text = text + &check_upstream_toml()?;
        with_stopped(root, || write_config(root, &text, None)).await?;
    }
    if !running(root).await {
        return Err("BOGO is not running; run `bogo setup start` first".into());
    }
    let reg = registry(root)?;
    if reg.active_approvers().next().is_none() {
        return Err("there is no approver yet; run `bogo setup phone` first".into());
    }
    if reg
        .agents
        .values()
        .any(|a| !a.revoked && a.name == CHECK_AGENT)
    {
        owner::request(
            root,
            Command::AgentRevoke {
                name: CHECK_AGENT.into(),
            },
        )
        .await?;
    }
    let v = owner::request(
        root,
        Command::AgentAdd {
            name: CHECK_AGENT.into(),
        },
    )
    .await?;
    let token = v["token"]
        .as_str()
        .ok_or("agent add returned no token")?
        .to_owned();
    let result = check_call(root, &token).await;
    drop(token);
    let _ = owner::request(
        root,
        Command::AgentRevoke {
            name: CHECK_AGENT.into(),
        },
    )
    .await;
    match result? {
        Some(text) if text == "pong" => {
            println!("  Approved, and the protected call ran. BOGO is protecting your agent.");
            Ok(())
        }
        Some(text) => Err(format!("the test call did not run: {text}")),
        None => Err("the test call got no answer".into()),
    }
}
/// The check agent's one call, made through `bogo-shim` like any agent. The token goes to
/// the shim in its environment only.
async fn check_call(root: &Path, token: &str) -> Result<Option<String>> {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
    let mut child = tokio::process::Command::new(shim_path()?)
        .env("BOGO_URL", mcp_url(root)?)
        .env("BOGO_AGENT_TOKEN", token)
        .env_remove("BOGO_AGENT_TOKEN_FILE")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .map_err(error)?;
    let mut stdin = child.stdin.take().ok_or("no shim stdin")?;
    let mut lines = tokio::io::BufReader::new(child.stdout.take().ok_or("no shim stdout")?).lines();
    async fn send(stdin: &mut tokio::process::ChildStdin, v: Value) -> Result<()> {
        stdin
            .write_all(format!("{v}\n").as_bytes())
            .await
            .map_err(error)
    }
    async fn reply(
        lines: &mut tokio::io::Lines<tokio::io::BufReader<tokio::process::ChildStdout>>,
        id: u64,
    ) -> Result<Value> {
        loop {
            let l = lines.next_line().await.map_err(error)?;
            let l = l.ok_or("bogo-shim stopped")?;
            if let Ok(v) = serde_json::from_str::<Value>(&l)
                && v["id"] == id
            {
                return Ok(v);
            }
        }
    }
    send(&mut stdin, json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"bogo-setup","version":env!("CARGO_PKG_VERSION")}}})).await?;
    let init = tokio::time::timeout(Duration::from_secs(30), reply(&mut lines, 1))
        .await
        .map_err(|_| "BOGO did not answer the test agent")??;
    if init.get("error").is_some() {
        return Err("BOGO refused the test agent".into());
    }
    send(
        &mut stdin,
        json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
    )
    .await?;
    send(&mut stdin, json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":CHECK_TOOL,"arguments":{"justification":"bogo setup: the one test call"}}})).await?;
    println!("  Waiting for you: approve \"{CHECK_TOOL}\" from agent {CHECK_AGENT} on your phone.");
    let v = reply(&mut lines, 2).await?;
    let _ = child.kill().await;
    if let Some(e) = v.get("error") {
        return Ok(Some(e["message"].as_str().unwrap_or("refused").to_owned()));
    }
    Ok(v["result"]["content"][0]["text"]
        .as_str()
        .map(str::to_owned))
}

/// `bogo check-upstream`: the setup check's MCP server on stdin and stdout. Its one tool
/// does nothing but answer "pong".
pub fn check_upstream() -> Result<()> {
    let stdin = std::io::stdin();
    let mut out = std::io::stdout();
    for line in stdin.lock().lines() {
        let line = line.map_err(error)?;
        let Ok(m) = serde_json::from_str::<Value>(&line) else {
            continue;
        };
        let Some(id) = m.get("id").cloned() else {
            continue;
        };
        let result = match m["method"].as_str() {
            Some("initialize") => json!({
                "protocolVersion": m["params"]["protocolVersion"].as_str().unwrap_or("2025-11-25"),
                "capabilities": {"tools": {}},
                "serverInfo": {"name": "bogo-check", "version": env!("CARGO_PKG_VERSION")}}),
            Some("tools/list") => json!({"tools": [{
                "name": "ping",
                "description": "BOGO's setup check. It only answers \"pong\" and changes nothing.",
                "inputSchema": {"type": "object", "properties": {}}}]}),
            Some("tools/call") => json!({"content": [{"type": "text", "text": "pong"}]}),
            Some("ping") => json!({}),
            _ => {
                writeln!(
                    out,
                    "{}",
                    json!({"jsonrpc":"2.0","id":id,"error":{"code":-32601,"message":"method not found"}})
                )
                .map_err(error)?;
                out.flush().map_err(error)?;
                continue;
            }
        };
        writeln!(out, "{}", json!({"jsonrpc":"2.0","id":id,"result":result})).map_err(error)?;
        out.flush().map_err(error)?;
    }
    Ok(())
}

/// JSON that keeps every object's key order, so a client's file changes only where setup
/// changes it.
#[derive(Clone, Debug)]
pub(crate) enum Ordered {
    Obj(Vec<(String, Ordered)>),
    Arr(Vec<Ordered>),
    Leaf(Value),
}
impl Ordered {
    pub fn get(&self, key: &str) -> Option<&Ordered> {
        match self {
            Ordered::Obj(o) => o.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }
    fn set(&mut self, key: &str, v: Ordered) {
        if let Ordered::Obj(o) = self {
            if let Some(slot) = o.iter_mut().find(|(k, _)| k == key) {
                slot.1 = v;
            } else {
                o.push((key.into(), v));
            }
        }
    }
    pub fn to_value(&self) -> Value {
        match self {
            Ordered::Obj(o) => {
                Value::Object(o.iter().map(|(k, v)| (k.clone(), v.to_value())).collect())
            }
            Ordered::Arr(a) => Value::Array(a.iter().map(Ordered::to_value).collect()),
            Ordered::Leaf(v) => v.clone(),
        }
    }
    fn from_value(v: Value) -> Ordered {
        match v {
            Value::Object(o) => Ordered::Obj(
                o.into_iter()
                    .map(|(k, v)| (k, Ordered::from_value(v)))
                    .collect(),
            ),
            Value::Array(a) => Ordered::Arr(a.into_iter().map(Ordered::from_value).collect()),
            v => Ordered::Leaf(v),
        }
    }
}
impl serde::Serialize for Ordered {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        use serde::ser::{SerializeMap, SerializeSeq};
        match self {
            Ordered::Obj(o) => {
                let mut m = s.serialize_map(Some(o.len()))?;
                for (k, v) in o {
                    m.serialize_entry(k, v)?;
                }
                m.end()
            }
            Ordered::Arr(a) => {
                let mut q = s.serialize_seq(Some(a.len()))?;
                for v in a {
                    q.serialize_element(v)?;
                }
                q.end()
            }
            Ordered::Leaf(v) => v.serialize(s),
        }
    }
}
impl<'de> serde::Deserialize<'de> for Ordered {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        struct V;
        impl<'de> serde::de::Visitor<'de> for V {
            type Value = Ordered;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("JSON")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                mut m: A,
            ) -> std::result::Result<Ordered, A::Error> {
                // serde_json's arbitrary_precision numbers arrive as a one-key map.
                let mut out = vec![];
                while let Some(k) = m.next_key::<String>()? {
                    if k == "$serde_json::private::Number" {
                        let n: String = m.next_value()?;
                        let v: Value =
                            serde_json::from_str(&n).map_err(serde::de::Error::custom)?;
                        return Ok(Ordered::Leaf(v));
                    }
                    out.push((k, m.next_value()?));
                }
                Ok(Ordered::Obj(out))
            }
            fn visit_seq<A: serde::de::SeqAccess<'de>>(
                self,
                mut s: A,
            ) -> std::result::Result<Ordered, A::Error> {
                let mut out = vec![];
                while let Some(v) = s.next_element()? {
                    out.push(v);
                }
                Ok(Ordered::Arr(out))
            }
            fn visit_bool<E>(self, v: bool) -> std::result::Result<Ordered, E> {
                Ok(Ordered::Leaf(Value::Bool(v)))
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Ordered, E> {
                Ok(Ordered::Leaf(v.into()))
            }
            fn visit_u64<E>(self, v: u64) -> std::result::Result<Ordered, E> {
                Ok(Ordered::Leaf(v.into()))
            }
            fn visit_f64<E>(self, v: f64) -> std::result::Result<Ordered, E> {
                Ok(Ordered::Leaf(json!(v)))
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Ordered, E> {
                Ok(Ordered::Leaf(Value::String(v.into())))
            }
            fn visit_string<E>(self, v: String) -> std::result::Result<Ordered, E> {
                Ok(Ordered::Leaf(Value::String(v)))
            }
            fn visit_unit<E>(self) -> std::result::Result<Ordered, E> {
                Ok(Ordered::Leaf(Value::Null))
            }
        }
        d.deserialize_any(V)
    }
}

/// The SHA-256 of a token file's token, as the registry keeps it.
pub(crate) fn token_hash(path: &Path) -> Option<String> {
    let text = std::fs::read_to_string(path).ok()?;
    let token = text.lines().next()?.trim();
    Some(hex::encode(Sha256::digest(token.as_bytes())))
}
