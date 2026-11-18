//! `bogo upstream add RECIPE`: a known, open-source MCP server at a pinned version, with a
//! policy that protects everything that writes, sends, pays or deletes.
use crate::setup::{self, Ask, k, q};
use crate::{Result, error};
use std::path::{Path, PathBuf};

enum Install {
    /// An npm package, run with node from its `bin` entry.
    Npm {
        package: &'static str,
        version: &'static str,
    },
    /// A Python package, in its own virtual environment made by uv.
    Uv {
        package: &'static str,
        version: &'static str,
        script: &'static str,
    },
    /// A release archive, checked against its published SHA-256 before it is unpacked.
    Release {
        version: &'static str,
        /// (target, URL, SHA-256)
        archives: &'static [(&'static str, &'static str, &'static str)],
        binary: &'static str,
    },
    /// A server its maker hosts. There is nothing to install and no version to pin.
    Hosted { url: &'static str },
}
enum Value {
    /// A credential: asked for with echo off, kept in secrets.toml.
    Secret {
        prompt: &'static str,
        check: fn(&str) -> std::result::Result<(), &'static str>,
    },
    /// A non-secret value the owner gives.
    Ask { prompt: &'static str },
    /// A private folder inside BOGO's home for the server's own credential files.
    Folder,
}
struct Recipe {
    name: &'static str,
    about: &'static str,
    install: Install,
    args: &'static [&'static str],
    /// Environment variables (local) or headers (hosted).
    values: &'static [(&'static str, Value)],
    /// Ask for a folder the server may reach, passed as its last argument.
    folder: bool,
    allow: &'static [&'static str],
    /// Protected, with the agent's justification required.
    approve: &'static [&'static str],
    /// Shown on the phone first, per protected tool: (tool, argument, role).
    highlight: &'static [(&'static str, &'static str, &'static str)],
    sign_in: &'static str,
}
fn any(_: &str) -> std::result::Result<(), &'static str> {
    Ok(())
}
fn stripe_test(s: &str) -> std::result::Result<(), &'static str> {
    if s.starts_with("sk_test_") || s.starts_with("rk_test_") {
        Ok(())
    } else {
        Err("this recipe takes a test-mode key only (sk_test_… or rk_test_…)")
    }
}
fn slack_token(s: &str) -> std::result::Result<(), &'static str> {
    if s.starts_with("xoxp-") || s.starts_with("xoxb-") {
        Ok(())
    } else {
        Err("a Slack token starts with xoxp- (user) or xoxb- (bot)")
    }
}

const RECIPES: &[Recipe] = &[
    Recipe {
        name: "gmail",
        about: "Gmail through workspace-mcp 2.3.0 (taylorwilsdon/google_workspace_mcp, MIT)",
        install: Install::Uv {
            package: "workspace-mcp",
            version: "2.3.0",
            script: "workspace-mcp",
        },
        args: &["--tools", "gmail", "--single-user"],
        values: &[
            (
                "GOOGLE_OAUTH_CLIENT_ID",
                Value::Ask {
                    prompt: "Google OAuth client ID (ends in .apps.googleusercontent.com)",
                },
            ),
            (
                "GOOGLE_OAUTH_CLIENT_SECRET",
                Value::Secret {
                    prompt: "Google OAuth client secret",
                    check: any,
                },
            ),
            ("WORKSPACE_MCP_CREDENTIALS_DIR", Value::Folder),
        ],
        folder: false,
        allow: &[
            "search_gmail_messages",
            "get_gmail_message_content",
            "get_gmail_messages_content_batch",
            "get_gmail_thread_content",
            "get_gmail_threads_content_batch",
            "list_gmail_labels",
            "list_gmail_filters",
        ],
        approve: &[
            "send_gmail_message",
            "draft_gmail_message",
            "get_gmail_attachment_content",
            "modify_gmail_message_labels",
            "batch_modify_gmail_message_labels",
            "manage_gmail_label",
            "manage_gmail_filter",
            "start_google_auth",
        ],
        highlight: &[
            ("send_gmail_message", "subject", "title"),
            ("send_gmail_message", "to", "subtitle"),
            ("send_gmail_message", "body", "text"),
            ("draft_gmail_message", "subject", "title"),
            ("draft_gmail_message", "to", "subtitle"),
        ],
        sign_in: "Sign in to Google yourself; BOGO never sees your Google password:
  1. In Google Cloud Console (console.cloud.google.com), make a project and enable the Gmail API.
  2. Under APIs & Services, OAuth consent screen, add yourself as a test user.
  3. Under Credentials, create an OAuth client ID of type Desktop app. Give its ID and secret above.
  4. Start BOGO, then ask your agent to read your mail. The first call answers with a Google
     sign-in link: open it, sign in and allow Gmail. The server keeps its Google token in the
     private folder BOGO made for it.
Sending mail and changing labels or filters wait for your phone. Reading and searching don't.",
    },
    Recipe {
        name: "github",
        about: "GitHub through github-mcp-server 2.0.3 (github/github-mcp-server, MIT)",
        install: Install::Release {
            version: "2.0.3",
            archives: &[
                (
                    "aarch64-apple-darwin",
                    "https://github.com/github/github-mcp-server/releases/download/v2.0.3/github-mcp-server_Darwin_arm64.tar.gz",
                    "5c95c01f75a7ec2370607e989b2968d314b02a54d78af8a9fdf38f3dfe6135f4",
                ),
                (
                    "x86_64-apple-darwin",
                    "https://github.com/github/github-mcp-server/releases/download/v2.0.3/github-mcp-server_Darwin_x86_64.tar.gz",
                    "115350858c3287843274efbf45e821d98f15d3304cb2c074740461ed32b45598",
                ),
            ],
            binary: "github-mcp-server",
        },
        args: &["stdio"],
        values: &[(
            "GITHUB_PERSONAL_ACCESS_TOKEN",
            Value::Secret {
                prompt: "GitHub fine-grained personal access token",
                check: any,
            },
        )],
        folder: false,
        allow: &[],
        approve: &[
            "create_or_update_file",
            "push_files",
            "delete_file",
            "merge_pull_request",
            "create_pull_request",
            "create_issue",
            "add_issue_comment",
        ],
        highlight: &[],
        sign_in:
            "Make a fine-grained token at github.com/settings/personal-access-tokens, limited to the
repositories the agent needs, and give it above. Tools GitHub marks read-only are offered as allow;
everything that writes waits for your phone.",
    },
    Recipe {
        name: "slack",
        about: "Slack through slack-mcp-server 1.3.0 (korotovsky/slack-mcp-server, MIT)",
        install: Install::Npm {
            package: "slack-mcp-server",
            version: "1.3.0",
        },
        args: &["--transport", "stdio"],
        values: &[(
            "SLACK_MCP_XOXP_TOKEN",
            Value::Secret {
                prompt: "Slack user OAuth token (xoxp-…)",
                check: slack_token,
            },
        )],
        folder: false,
        allow: &[],
        approve: &["conversations_add_message"],
        highlight: &[
            ("conversations_add_message", "channel_id", "subtitle"),
            ("conversations_add_message", "payload", "text"),
        ],
        sign_in:
            "Make a Slack app at api.slack.com/apps, add the user scopes it needs (channels:history,
channels:read, users:read, and chat:write to post), install it to your workspace and give its
user OAuth token above. Posting waits for your phone; posting is off unless you set
SLACK_MCP_ADD_MESSAGE_TOOL in its env.",
    },
    Recipe {
        name: "stripe",
        about: "Stripe in test mode, through Stripe's hosted MCP server (mcp.stripe.com)",
        install: Install::Hosted {
            url: "https://mcp.stripe.com",
        },
        args: &[],
        values: &[(
            "Authorization",
            Value::Secret {
                prompt: "Stripe test-mode secret or restricted key (sk_test_… or rk_test_…)",
                check: stripe_test,
            },
        )],
        folder: false,
        allow: &[],
        approve: &[
            "create_payment_link",
            "create_refund",
            "create_invoice",
            "finalize_invoice",
            "create_customer",
            "create_product",
            "create_price",
            "cancel_subscription",
            "update_subscription",
            "create_coupon",
        ],
        highlight: &[],
        sign_in:
            "Use a test-mode key from dashboard.stripe.com/test/apikeys; a restricted key with only
the access the agent needs is best. Stripe hosts this server, so there is no version to pin: Stripe
changes it on its own schedule, and BOGO lists any new tool as needing approval.",
    },
    Recipe {
        name: "files",
        about: "Files in one folder, through @modelcontextprotocol/server-filesystem 2.3.0 (MIT)",
        install: Install::Npm {
            package: "@modelcontextprotocol/server-filesystem",
            version: "2.3.0",
        },
        args: &[],
        values: &[],
        folder: true,
        allow: &[
            "read_text_file",
            "read_media_file",
            "read_multiple_files",
            "read_file",
            "list_directory",
            "list_directory_with_sizes",
            "directory_tree",
            "search_files",
            "get_file_info",
            "list_allowed_directories",
        ],
        approve: &["write_file", "edit_file", "create_directory", "move_file"],
        highlight: &[
            ("write_file", "path", "title"),
            ("edit_file", "path", "title"),
            ("move_file", "destination", "title"),
        ],
        sign_in:
            "Nothing to sign in to. The agent can read the folder you gave; writing, editing and
moving files wait for your phone.",
    },
];

pub fn recipe_names() -> Vec<&'static str> {
    RECIPES.iter().map(|r| r.name).collect()
}

/// Where a recipe installs its server: outside BOGO's home, which upstreams may not reach.
fn install_dir(name: &str) -> Result<PathBuf> {
    let home = std::env::var_os("HOME").ok_or("HOME is not set")?;
    Ok(PathBuf::from(home)
        .join(".local/share/bogo/upstreams")
        .join(name))
}
fn run(cmd: &mut std::process::Command, what: &str) -> Result<()> {
    let out = cmd.output().map_err(|e| format!("{what}: {e}"))?;
    if !out.status.success() {
        let tail: String = String::from_utf8_lossy(&out.stderr)
            .lines()
            .rev()
            .take(5)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .join("\n");
        return Err(format!("{what} failed:\n{tail}"));
    }
    Ok(())
}
fn need(tool: &str, hint: &str) -> Result<String> {
    which(tool).ok_or_else(|| format!("{tool} is not on PATH; {hint}"))
}
fn which(tool: &str) -> Option<String> {
    std::env::split_paths(&std::env::var_os("PATH")?)
        .map(|d| d.join(tool))
        .find(|p| p.is_file())
        .map(|p| p.to_string_lossy().into_owned())
}
/// Install the pinned server. Returns the command and its leading arguments, or the URL.
fn install(r: &Recipe) -> Result<(Option<String>, Vec<String>, Option<String>)> {
    match &r.install {
        Install::Hosted { url } => Ok((None, vec![], Some(url.to_string()))),
        Install::Npm { package, version } => {
            let node = need("node", "install Node.js first")?;
            let npm = need("npm", "install Node.js first")?;
            let dir = install_dir(r.name)?;
            std::fs::create_dir_all(&dir).map_err(error)?;
            println!("Installing {package}@{version} into {}…", dir.display());
            run(
                std::process::Command::new(npm)
                    .args(["install", "--prefix"])
                    .arg(&dir)
                    .args(["--no-audit", "--no-fund", "--save-exact"])
                    .arg(format!("{package}@{version}")),
                "npm install",
            )?;
            let pkg_dir = dir.join("node_modules").join(package);
            let manifest: serde_json::Value = serde_json::from_slice(
                &std::fs::read(pkg_dir.join("package.json")).map_err(error)?,
            )
            .map_err(error)?;
            let bin = match &manifest["bin"] {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Object(m) => m
                    .values()
                    .next()
                    .and_then(|v| v.as_str())
                    .ok_or("the package names no program")?
                    .into(),
                _ => return Err("the package names no program".into()),
            };
            Ok((
                Some(node),
                vec![pkg_dir.join(bin).to_string_lossy().into_owned()],
                None,
            ))
        }
        Install::Uv {
            package,
            version,
            script,
        } => {
            let uv = need("uv", "install uv (docs.astral.sh/uv) first")?;
            let dir = install_dir(r.name)?;
            std::fs::create_dir_all(&dir).map_err(error)?;
            let venv = dir.join("venv");
            println!("Installing {package}=={version} into {}…", dir.display());
            if !venv.join("bin/python").exists() {
                run(
                    std::process::Command::new(&uv)
                        .args(["venv", "--quiet", "--python", ">=3.11"])
                        .arg(&venv),
                    "uv venv",
                )?;
            }
            run(
                std::process::Command::new(&uv)
                    .args(["pip", "install", "--quiet", "--python"])
                    .arg(venv.join("bin/python"))
                    .arg(format!("{package}=={version}")),
                "uv pip install",
            )?;
            Ok((
                Some(venv.join("bin").join(script).to_string_lossy().into_owned()),
                vec![],
                None,
            ))
        }
        Install::Release {
            version,
            archives,
            binary,
        } => {
            let target = if cfg!(target_arch = "aarch64") {
                "aarch64-apple-darwin"
            } else {
                "x86_64-apple-darwin"
            };
            let (_, url, sha) = archives
                .iter()
                .find(|(t, _, _)| *t == target)
                .ok_or("this recipe has no build for this Mac")?;
            let dir = install_dir(r.name)?.join(version);
            let exe = dir.join(binary);
            if !exe.exists() {
                std::fs::create_dir_all(&dir).map_err(error)?;
                println!("Downloading {binary} {version}…");
                let archive = dir.join("download.tar.gz");
                run(
                    std::process::Command::new(need("curl", "it ships with macOS")?)
                        .args(["-fsSL", "-o"])
                        .arg(&archive)
                        .arg(url),
                    "download",
                )?;
                use sha2::Digest;
                let got = hex::encode(sha2::Sha256::digest(
                    std::fs::read(&archive).map_err(error)?,
                ));
                if got != *sha {
                    let _ = std::fs::remove_file(&archive);
                    return Err(format!(
                        "the download's SHA-256 is {got}, not the pinned {sha}; nothing installed"
                    ));
                }
                run(
                    std::process::Command::new("tar")
                        .arg("-xzf")
                        .arg(&archive)
                        .arg("-C")
                        .arg(&dir)
                        .arg(binary),
                    "unpack",
                )?;
                let _ = std::fs::remove_file(&archive);
            }
            Ok((Some(exe.to_string_lossy().into_owned()), vec![], None))
        }
    }
}

pub async fn add_recipe(root: &Path, name: &str, yes: bool) -> Result<()> {
    let r = RECIPES.iter().find(|r| r.name == name).ok_or_else(|| {
        format!(
            "no recipe called {name}; the recipes are {}",
            recipe_names().join(", ")
        )
    })?;
    let ask = Ask { yes };
    let (config, _) = crate::owner::load(root)?;
    if config.upstreams.contains_key(r.name) {
        return Err(format!("an upstream called {} already exists", r.name));
    }
    println!("{}", r.about);
    // Ask for everything before installing anything.
    let secrets_text = std::fs::read_to_string(root.join("secrets.toml")).map_err(error)?;
    let mut secrets = secrets_text.trim_end().to_owned() + "\n";
    let mut items = vec![];
    for (key, v) in r.values {
        match v {
            Value::Secret { prompt, check } => {
                let value = loop {
                    let Some(s) = ask.secret(prompt)? else {
                        return Err(format!(
                            "nothing added: the {} server needs its {prompt}. Run this again when you have it.",
                            r.name
                        ));
                    };
                    let s = if r.name == "stripe" && *key == "Authorization" {
                        if let Err(why) = check(&s) {
                            println!("{why}");
                            continue;
                        }
                        format!("Bearer {s}")
                    } else {
                        if let Err(why) = check(&s) {
                            println!("{why}");
                            continue;
                        }
                        s
                    };
                    if s.chars().count() < 8 {
                        println!("That is too short to be a credential.");
                        continue;
                    }
                    break s;
                };
                let secret = format!("{}_{}", r.name, setup::slug(key).replace('-', "_"));
                secrets.push_str(&format!("{} = {}\n", k(&secret), q(&value)));
                items.push(format!("{} = {{ secret = {} }}", k(key), q(&secret)));
            }
            Value::Ask { prompt } => {
                let a = ask.line(prompt, "")?;
                if a.is_empty() {
                    return Err(format!(
                        "nothing added: the {} server needs its {prompt}.",
                        r.name
                    ));
                }
                items.push(format!("{} = {}", k(key), q(&a)));
            }
            Value::Folder => {
                let dir = root.join("upstreams").join(r.name);
                std::fs::create_dir_all(&dir).map_err(error)?;
                use std::os::unix::fs::PermissionsExt;
                for d in [root.join("upstreams"), dir.clone()] {
                    std::fs::set_permissions(&d, std::fs::Permissions::from_mode(0o700))
                        .map_err(error)?;
                }
                let secret = format!("{}_folder", r.name);
                secrets.push_str(&format!(
                    "{} = {{ path = {} }}\n",
                    k(&secret),
                    q(&dir.to_string_lossy())
                ));
                items.push(format!("{} = {{ secret = {} }}", k(key), q(&secret)));
            }
        }
    }
    let mut folder = None;
    if r.folder {
        let home = std::env::var_os("HOME").map(PathBuf::from);
        let f = loop {
            let a = ask.line("Which folder may the agent reach?", "")?;
            if a.is_empty() {
                return Err("nothing added: give the one folder the agent may reach".into());
            }
            let p = crate::owner::normalize(Path::new(&a))?;
            if p == Path::new("/") || Some(&p) == home.as_ref() || !p.is_dir() {
                println!("Give an existing folder, not / or your whole home folder.");
                continue;
            }
            break p;
        };
        folder = Some(f.to_string_lossy().into_owned());
    }
    let (command, mut args, url) = install(r)?;
    args.extend(r.args.iter().map(|a| a.to_string()));
    args.extend(folder);
    let mut block = format!(
        "\n# Added by `bogo upstream add {}`: {}.\n[upstreams.{}]\n",
        r.name, r.about, r.name
    );
    match (&command, &url) {
        (Some(c), _) => {
            block.push_str(&format!(
                "command = {}\nargs = [{}]\n",
                q(c),
                args.iter().map(|a| q(a)).collect::<Vec<_>>().join(", ")
            ));
            if !items.is_empty() {
                block.push_str(&format!("env = {{ {} }}\n", items.join(", ")));
            }
        }
        (None, Some(u)) => {
            block.push_str(&format!("url = {}\n", q(u)));
            if !items.is_empty() {
                block.push_str(&format!("headers = {{ {} }}\n", items.join(", ")));
            }
        }
        _ => unreachable!(),
    }
    // The recipe's own lists first; for any other tool the server lists, suggest from its
    // annotations and let the owner choose.
    let mut policy = String::new();
    let tool = |t: &str| format!("{}__{t}", r.name);
    let write = |policy: &mut String, name: &str, p: &str, required: bool| {
        policy.push_str(&format!("[tools.{}]\npolicy = {}\n", k(name), q(p)));
        if required {
            policy.push_str("justification = \"required\"\n");
        }
        let hl: Vec<String> = r
            .highlight
            .iter()
            .filter(|(t, _, _)| tool(t) == name)
            .map(|(_, a, role)| format!("{} = {}", k(a), q(role)))
            .collect();
        if !hl.is_empty() {
            policy.push_str(&format!("highlight = {{ {} }}\n", hl.join(", ")));
        }
    };
    let probe = format!("version = 1\n[server]\nport = 1\n[defaults]\nmax_pending = 1\n{block}");
    println!("Starting it once to list its tools…");
    let (tools, health) = setup::discover(&probe, &secrets).await?;
    let listed: Vec<&String> = tools.keys().collect();
    if health.get(r.name) != Some(&true) {
        println!(
            "It did not start with these settings; the policy below covers the tools the recipe knows."
        );
    }
    for t in r.allow {
        write(&mut policy, &tool(t), "allow", false);
    }
    for t in r.approve {
        write(&mut policy, &tool(t), "approve", true);
    }
    for name in listed {
        let short = name.split_once("__").map_or("", |(_, t)| t);
        if r.allow.contains(&short) || r.approve.contains(&short) {
            continue;
        }
        let (suggested, why) = setup::suggest(&tools[name]);
        let chosen = loop {
            let a = ask.line(
                &format!("{name}: {why}. allow, approve or deny?"),
                suggested,
            )?;
            if matches!(a.as_str(), "allow" | "approve" | "deny") {
                break a;
            }
            println!("Answer allow, approve or deny.");
        };
        let chosen = if yes && chosen == "allow" {
            "approve".to_owned()
        } else {
            chosen
        };
        write(&mut policy, name, &chosen, false);
    }
    let text = std::fs::read_to_string(root.join("bogo.toml")).map_err(error)?;
    let text = text.trim_end().to_owned() + "\n" + &block + &policy;
    setup::with_stopped(root, || setup::write_config(root, &text, Some(&secrets))).await?;
    println!("Added upstream {}.", r.name);
    for t in r.allow {
        println!("  allow    {}", tool(t));
    }
    for t in r.approve {
        println!("  approve  {}", tool(t));
    }
    println!("{}", r.sign_in);
    Ok(())
}
