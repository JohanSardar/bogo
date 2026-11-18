use bogo_daemon::{Command, init, launch, request, serve};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
#[derive(Parser)]
#[command(version, about = "BOGO owner commands")]
struct Args {
    #[arg(long, global = true)]
    home: Option<PathBuf>,
    #[command(subcommand)]
    command: Cli,
}
#[derive(Subcommand)]
enum Cli {
    Init,
    Start,
    Stop,
    Serve,
    Status,
    Tools,
    Agent {
        #[command(subcommand)]
        action: Agent,
    },
    Approver {
        #[command(subcommand)]
        action: Approver,
    },
    Request {
        #[command(subcommand)]
        action: Request,
    },
    Log {
        #[command(subcommand)]
        action: Option<Log>,
    },
    Export {
        #[arg(long, default_value = "jsonl")]
        format: String,
    },
    Upstream {
        #[command(subcommand)]
        action: Upstream,
    },
    /// One guided setup: the home, BOGO in the background, your iPhone, an agent and one
    /// protected test call. Each step is skipped when it is already done.
    Setup {
        /// Run only this step: home, start, phone, agent or test.
        step: Option<bogo_daemon::SetupStep>,
        /// Take every suggested answer without asking. Never allows a tool.
        #[arg(long)]
        yes: bool,
        /// Put back every client config file setup changed, byte for byte.
        #[arg(long, conflicts_with_all = ["step", "yes"])]
        undo: bool,
        /// Look for Claude Code, Claude Desktop and Cursor configs here instead of in HOME.
        #[arg(long)]
        client_home: Option<PathBuf>,
        /// The approver the phone step adds; `soft` (a software test approver) needs a test build.
        #[arg(long, default_value = "iphone")]
        approver_kind: String,
    },
    /// Check everything a working setup needs, and say how to fix what is wrong.
    Doctor {
        /// Look for client configs here instead of in HOME.
        #[arg(long)]
        client_home: Option<PathBuf>,
    },
    /// The setup check's MCP server; `bogo setup` configures it.
    #[command(hide = true)]
    CheckUpstream,
    #[cfg(feature = "test-approvers")]
    Soft {
        name: String,
        id: String,
        #[arg(long)]
        deny: bool,
        #[arg(long)]
        note: Option<String>,
    },
}
#[derive(Subcommand)]
enum Agent {
    Add { name: String },
    List,
    Revoke { name: String },
    Pause { name: String },
    Unpause { name: String },
}
#[derive(Subcommand)]
enum Approver {
    Add {
        name: String,
        #[arg(long, default_value = "iphone")]
        kind: String,
    },
    List,
    Revoke {
        name: String,
    },
}
#[derive(Subcommand)]
enum Request {
    List,
    Cancel { id: String },
}
#[derive(Subcommand)]
enum Log {
    Search {
        text: String,
    },
    Verify,
    Export {
        #[arg(long, default_value = "jsonl")]
        format: String,
    },
}
#[derive(Subcommand)]
enum Upstream {
    /// Add an upstream: a recipe (gmail, github, slack, stripe, files), a URL, or a command.
    Add {
        name: String,
        #[arg(long)]
        url: Option<String>,
        /// With a recipe, take every suggested answer. Never allows a tool the recipe doesn't.
        #[arg(long)]
        yes: bool,
        #[arg(trailing_var_arg = true)]
        command: Vec<String>,
    },
    /// List the recipes `bogo upstream add` knows.
    Recipes,
}
#[tokio::main]
async fn main() {
    if let Err(e) = run(Args::parse()).await {
        eprintln!("bogo: {e}");
        std::process::exit(1)
    }
}
async fn run(args: Args) -> Result<(), String> {
    // An upstream: BOGO runs it with a cleared environment, so it must not need HOME.
    if matches!(args.command, Cli::CheckUpstream) {
        return bogo_daemon::check_upstream();
    }
    let root = args
        .home
        .or_else(|| std::env::var_os("HOME").map(|p| PathBuf::from(p).join(".bogo")))
        .ok_or("HOME is not set; use --home")?;
    let command = match args.command {
        Cli::Init => return init(&root),
        Cli::Start => return launch(&root, true),
        Cli::Stop => return launch(&root, false),
        Cli::Serve => return serve(&root).await,
        Cli::Status => Command::Status,
        Cli::Tools => Command::Tools,
        Cli::Agent { action } => match action {
            Agent::Add { name } => Command::AgentAdd { name },
            Agent::List => Command::AgentList,
            Agent::Revoke { name } => Command::AgentRevoke { name },
            Agent::Pause { name } => Command::Pause { name },
            Agent::Unpause { name } => Command::Unpause { name },
        },
        Cli::Approver { action } => match action {
            Approver::Add { name, kind } => Command::ApproverAdd { name, kind },
            Approver::List => Command::ApproverList,
            Approver::Revoke { name } => Command::ApproverRevoke { name },
        },
        Cli::Request { action } => match action {
            Request::List => Command::Requests,
            Request::Cancel { id } => Command::Cancel { id },
        },
        Cli::Log { action } => match action {
            None => Command::Log { search: None },
            Some(Log::Search { text }) => Command::Log { search: Some(text) },
            Some(Log::Verify) => Command::Verify,
            Some(Log::Export { format }) => Command::Export { format },
        },
        Cli::Export { format } => Command::Export { format },
        Cli::Upstream {
            action:
                Upstream::Add {
                    name,
                    url,
                    yes,
                    command,
                },
        } => {
            if url.is_none()
                && command.is_empty()
                && bogo_daemon::recipe_names().contains(&name.as_str())
            {
                return bogo_daemon::add_recipe(&root, &name, yes).await;
            }
            return bogo_daemon::add_upstream(&root, &name, url, command);
        }
        Cli::Upstream {
            action: Upstream::Recipes,
        } => {
            println!("{}", bogo_daemon::recipe_names().join("\n"));
            return Ok(());
        }
        Cli::Setup {
            step,
            yes,
            undo,
            client_home,
            approver_kind,
        } => {
            if undo {
                return bogo_daemon::undo(&root);
            }
            return bogo_daemon::setup(
                &root,
                step,
                &bogo_daemon::SetupOptions {
                    yes,
                    client_home,
                    approver_kind,
                },
            )
            .await;
        }
        Cli::Doctor { client_home } => {
            return bogo_daemon::doctor(&root, client_home.as_deref()).await;
        }
        Cli::CheckUpstream => unreachable!("handled above"),
        #[cfg(feature = "test-approvers")]
        Cli::Soft {
            name,
            id,
            deny,
            note,
        } => Command::Soft {
            name,
            id,
            deny,
            note,
        },
    };
    let v = request(&root, command).await?;
    if let Some(s) = v.as_str() {
        println!("{s}")
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&v).map_err(|e| e.to_string())?
        )
    }
    Ok(())
}
