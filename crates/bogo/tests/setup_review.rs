use serde_json::{Value, json};
use std::{
    path::PathBuf,
    process::{Command, Stdio},
    sync::atomic::{AtomicUsize, Ordering},
};

struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let s = Self(PathBuf::from(format!(
            "/tmp/bogo-review-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        )));
        std::fs::create_dir_all(s.0.join("client")).unwrap();
        s.cli(&["setup", "home", "--yes"]);
        s
    }

    fn cli(&self, args: &[&str]) {
        let result = Command::new(env!("CARGO_BIN_EXE_bogo"))
            .env_clear()
            .env("HOME", self.0.join("client"))
            .env("PATH", "/usr/bin:/bin:/usr/sbin:/sbin")
            .arg("--home")
            .arg(self.0.join("home"))
            .args(args)
            .stdin(Stdio::null())
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "CLI failed: {}",
            String::from_utf8_lossy(&result.stderr)
        );
    }

    fn put_client(&self, relative: &str, doc: Value) {
        let path = self.0.join("client").join(relative);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, serde_json::to_vec(&doc).unwrap()).unwrap();
    }

    fn read_client(&self, relative: &str) -> Value {
        serde_json::from_slice(&std::fs::read(self.0.join("client").join(relative)).unwrap())
            .unwrap()
    }

    fn server(&self) -> PathBuf {
        let path = self.0.join("mcp-server");
        std::fs::copy(env!("CARGO_BIN_EXE_bogo"), &path).unwrap();
        path
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn setup_imports_claude_code_local_scope() {
    let s = Scratch::new();
    s.put_client(
        ".claude.json",
        json!({"projects": {"/tmp/review-project": {
            "mcpServers": {"mail": {
                "type": "stdio", "command": s.server(), "args": ["check-upstream"],
                "env": {"API_KEY": "review-placeholder-alpha"}
            }}
        }}}),
    );
    s.cli(&["setup", "agent", "--yes"]);
    let doc = s.read_client(".claude.json");
    assert!(
        doc["projects"]["/tmp/review-project"]["mcpServers"]
            .get("mail")
            .is_none(),
        "Claude Code's local-scope server remains connected directly, with its credential in the client file"
    );
}

#[test]
fn setup_keeps_distinct_account_environments() {
    let s = Scratch::new();
    let server = s.server();
    s.put_client(
        ".claude.json",
        json!({"mcpServers": {"work": {
            "command": server, "args": ["check-upstream"],
            "env": {"API_KEY": "review-placeholder-alpha"}
        }}}),
    );
    s.put_client(
        ".cursor/mcp.json",
        json!({"mcpServers": {"personal": {
            "command": server, "args": ["check-upstream"],
            "env": {"API_KEY": "review-placeholder-beta"}
        }}}),
    );
    s.cli(&["setup", "agent", "--yes"]);
    let secrets = std::fs::read_to_string(s.0.join("home/secrets.toml")).unwrap();
    assert!(
        secrets.contains("review-placeholder-beta"),
        "the second account's credential was discarded instead of importing its own upstream"
    );
}

#[test]
fn setup_repairs_an_existing_bogo_entry() {
    let s = Scratch::new();
    s.put_client(
        ".claude.json",
        json!({"mcpServers": {"bogo": {
            "command": "/no-longer-installed/bogo-shim", "args": [],
            "env": {"BOGO_URL": "http://127.0.0.1:1/mcp",
                "BOGO_AGENT_TOKEN_FILE": s.0.join("missing.token")}
        }}}),
    );
    s.cli(&["setup", "agent", "--yes"]);
    let doc = s.read_client(".claude.json");
    assert_ne!(
        doc["mcpServers"]["bogo"]["command"], "/no-longer-installed/bogo-shim",
        "setup skipped the broken entry instead of repairing it as doctor instructs"
    );
}

#[test]
fn setup_never_confirms_allow_on_end_of_input() {
    let s = Scratch::new();
    let script = s.0.join("readonly.py");
    std::fs::write(&script, r#"import json, sys
for line in sys.stdin:
    m = json.loads(line)
    if 'id' not in m:
        continue
    if m['method'] == 'initialize':
        result = {'protocolVersion': m['params']['protocolVersion'], 'capabilities': {'tools': {}}, 'serverInfo': {'name': 'review', 'version': '1'}}
    elif m['method'] == 'tools/list':
        result = {'tools': [{'name': 'read', 'inputSchema': {'type': 'object', 'properties': {}}, 'annotations': {'readOnlyHint': True}}]}
    else:
        result = {}
    print(json.dumps({'jsonrpc': '2.0', 'id': m['id'], 'result': result}), flush=True)
"#).unwrap();
    s.put_client(
        ".claude.json",
        json!({"mcpServers": {"readonly": {
            "command": "/usr/bin/python3", "args": [script]
        }}}),
    );
    // No --yes and no answers: EOF is not the owner's confirmation of allow.
    let _result = Command::new(env!("CARGO_BIN_EXE_bogo"))
        .env_clear()
        .env("HOME", s.0.join("client"))
        .env("PATH", "/usr/bin:/bin:/usr/sbin:/sbin")
        .arg("--home")
        .arg(s.0.join("home"))
        .args(["setup", "agent"])
        .stdin(Stdio::null())
        .output()
        .unwrap();
    let config = std::fs::read_to_string(s.0.join("home/bogo.toml")).unwrap();
    assert!(
        !config.contains("policy = \"allow\""),
        "setup wrote allow without receiving any owner confirmation"
    );
}

#[test]
fn setup_does_not_import_a_project_bogo_connection_as_an_upstream() {
    let s = Scratch::new();
    s.put_client(".claude.json", json!({"mcpServers": {}}));
    s.cli(&["setup", "agent", "--yes"]);
    let doc = s.read_client(".claude.json");
    // Claude Code's local scope stores an MCP connection under its project.
    // Use a working BOGO connection with an active token, not a malformed entry.
    s.put_client(
        ".claude.json",
        json!({"projects": {"/tmp/review-project": {
            "mcpServers": {"bogo": doc["mcpServers"]["bogo"]}
        }}}),
    );
    s.cli(&["setup", "agent", "--yes"]);
    let config = std::fs::read_to_string(s.0.join("home/bogo.toml")).unwrap();
    assert!(
        !config.contains("[upstreams.bogo-upstream]"),
        "setup imported the project's own BOGO connection as an upstream back to itself"
    );
}
