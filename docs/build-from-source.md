# Build from source and try it

Build BOGO yourself, and try it on your Mac with a **software test approver**
in place of the phone. You need this page to work on BOGO, or to watch a
protected call wait for approval without an iPhone. To use BOGO for real,
[install it](setup.md) instead.

The software test approver signs on the Mac without Face ID, and anyone with
your Mac account can use it. It exists only in builds with the
`test-approvers` feature. Never use it to protect real actions.

## Try it without an iPhone

This runs BOGO on your Mac with a **software test approver** in place of the
phone, so you can watch a protected call wait for approval, without an iPhone.
The software test approver signs without Face ID: never use it to protect real
actions.

Prerequisites: Xcode Command Line Tools, [Rust via rustup](https://rustup.rs/),
[Lean via elan](https://github.com/leanprover/elan#installation), Node.js and
npm, Python 3, and an installed, signed-in
[Claude Code](https://code.claude.com/docs/en/setup) CLI (`claude`). The repo
pins Lean in `lean/lean-toolchain`. Put elan first on `PATH`, because some
Python installs ship an unrelated program called `lean`.

Run everything from this repo's root, in **one terminal**. The demo uses a
fresh private temporary directory and leaves `~/.bogo` alone. Port 18790 must
be free. No LaunchAgent is loaded.

```sh
export PATH="$HOME/.elan/bin:$PATH"
cargo build -p bogo -p bogo-daemon -p bogo-shim --features bogo/test-approvers
export BOGO_BIN="$PWD/target/debug/bogo"
export BOGO_SHIM="$PWD/target/debug/bogo-shim"
export BOGO_DEMO="$(mktemp -d /tmp/bogo-quickstart.XXXXXX)"
export BOGO_DEMO_HOME="$BOGO_DEMO/home"
umask 077
mkdir -p "$BOGO_DEMO/scratch" "$BOGO_DEMO/client"
"$BOGO_BIN" --home "$BOGO_DEMO_HOME" init
npm install --prefix "$BOGO_DEMO/upstream" --no-audit --no-fund \
  @modelcontextprotocol/server-filesystem@2.3.0
```

Configure a demo policy: at most four live requests per agent, file reads
allowed, file writes protected. Any tool not listed still needs approval. The
filesystem server can reach only the scratch directory.

```sh
cat > "$BOGO_DEMO_HOME/bogo.toml" <<'TOML'
version = 1
[server]
port = 18790
[defaults]
policy = "approve"
max_pending = 4
[tools.files__read_text_file]
policy = "allow"
[tools.files__write_file]
policy = "approve"
justification = "required"
TOML
"$BOGO_BIN" --home "$BOGO_DEMO_HOME" upstream add files -- \
  "$(command -v node)" \
  "$BOGO_DEMO/upstream/node_modules/@modelcontextprotocol/server-filesystem/dist/index.js" \
  "$BOGO_DEMO/scratch"
"$BOGO_BIN" --home "$BOGO_DEMO_HOME" serve > "$BOGO_DEMO/daemon.log" 2>&1 &
BOGO_DAEMON_PID=$!
```

Check the daemon, register the software test approver and a named agent, and
write the agent's MCP config. `status` waits for startup and should show
`running: true` and `upstreams: {files: true}`; if startup failed, read
`$BOGO_DEMO/daemon.log`. The agent's token goes into an environment variable,
never into the config file or the terminal.

```sh
until "$BOGO_BIN" --home "$BOGO_DEMO_HOME" status | grep -q '"running": true'
do kill -0 "$BOGO_DAEMON_PID" || break; sleep 1; done
"$BOGO_BIN" --home "$BOGO_DEMO_HOME" status
"$BOGO_BIN" --home "$BOGO_DEMO_HOME" approver add 'Demo approver' --kind soft
export BOGO_AGENT_TOKEN="$("$BOGO_BIN" --home "$BOGO_DEMO_HOME" agent add 'Demo agent' \
  | python3 -c 'import json,sys; print(json.load(sys.stdin)["token"])')"
export BOGO_URL=http://127.0.0.1:18790/mcp
python3 - <<'PY'
import json, os
from pathlib import Path
p = Path(os.environ['BOGO_DEMO']) / 'client' / 'mcp.json'
p.write_text(json.dumps({'mcpServers': {'bogo': {'command': os.environ['BOGO_SHIM']}}}))
PY
"$BOGO_BIN" --home "$BOGO_DEMO_HOME" tools
```

Ask Claude to read a file and then write one. It runs in the background from
the empty client directory, with only BOGO's tools enabled. That restriction
keeps the demo reproducible; it is not a sandbox.

```sh
printf 'Hello from BOGO\n' > "$BOGO_DEMO/scratch/hello.txt"
(
  cd "$BOGO_DEMO/client"
  claude -p "Use files__read_text_file to read $BOGO_DEMO/scratch/hello.txt. Then use files__write_file to write exactly BOGO APPROVED to $BOGO_DEMO/scratch/approved.txt, with justification 'Quick start protected write'. Report both tool results." \
    --mcp-config "$BOGO_DEMO/client/mcp.json" --strict-mcp-config \
    --tools '' --allowedTools 'mcp__bogo__*' --permission-mode dontAsk \
    < /dev/null > "$BOGO_DEMO/claude.out" 2>&1
) &
BOGO_CLAUDE_PID=$!
until BOGO_REQUEST_ID="$("$BOGO_BIN" --home "$BOGO_DEMO_HOME" request list \
  | python3 -c 'import json,sys; print(json.load(sys.stdin)[0]["id"])' 2>/dev/null)"
do kill -0 "$BOGO_CLAUDE_PID" || break; sleep 2; done
```

That waits until the write is pending and keeps its request ID. The read
already ran. The write is waiting for you. Look at what it will do, and see
that the file doesn't exist yet:

```sh
"$BOGO_BIN" --home "$BOGO_DEMO_HOME" log search files__write_file
test ! -e "$BOGO_DEMO/scratch/approved.txt" && printf 'Not written before approval\n'
```

If the logged `agent_name`, `tool` and `arguments` are the demo write, approve
it:

```sh
"$BOGO_BIN" --home "$BOGO_DEMO_HOME" soft 'Demo approver' "$BOGO_REQUEST_ID"
wait "$BOGO_CLAUDE_PID"
cat "$BOGO_DEMO/claude.out"
cat "$BOGO_DEMO/scratch/approved.txt"
"$BOGO_BIN" --home "$BOGO_DEMO_HOME" log verify
```

Expect `decision recorded`, Claude's report of both real tool results, the file
content `BOGO APPROVED`, and `chain intact` with one counted approval. Stop the
daemon and check its log once more:

```sh
kill -TERM "$BOGO_DAEMON_PID"
wait "$BOGO_DAEMON_PID"
"$BOGO_BIN" --home "$BOGO_DEMO_HOME" log verify
unset BOGO_AGENT_TOKEN
```

The demo directory holds the call arguments, the log and the software key. Keep
it private, and delete that directory when you're done. To run BOGO for real,
see [Run BOGO on your Mac](daemon.md).

## Try `bogo setup` with the software test approver

`bogo setup` can add a software test approver in place of the phone, so you
can walk through the [five-minute start](../README.md#get-started-in-five-minutes)
without an iPhone. This uses the same test build as above, and it changes the
same things on your Mac as the real setup ([what setup changes](setup.md#what-bogo-setup-changes)):
it loads BOGO's LaunchAgent, and it rewrites the MCP client configs it finds.
To keep your own configs out of it, point it at a scratch folder with
`--client-home` and a scratch home with `--home`, as here:

```sh
export BOGO_TRY="$(mktemp -d /tmp/bogo-try.XXXXXX)"
mkdir "$BOGO_TRY/clients"
target/debug/bogo --home "$BOGO_TRY/home" setup --approver-kind soft \
  --client-home "$BOGO_TRY/clients"
```

Keep the `--home` path short, as `mktemp` in `/tmp` makes it: BOGO's socket
lives in the home, and macOS limits socket paths to about 100 characters.

Setup asks its questions and starts BOGO. The phone step adds the software
test approver, named `iPhone` unless you give another name. With no client
config in the scratch folder, the agent step writes a Claude Code config of
its own into the home and prints the `claude` command that uses it. The test
step then waits:

```
5/5 Make one protected test call, and approve it on your phone.
  Waiting for you: approve "bogo-check__ping" from agent BOGO setup check on your phone.
```

Approve it from a second terminal, in place of the phone:

```sh
BOGO_REQUEST_ID="$(target/debug/bogo --home "$BOGO_TRY/home" request list \
  | python3 -c 'import json,sys; print(json.load(sys.stdin)[0]["id"])')"
target/debug/bogo --home "$BOGO_TRY/home" soft iPhone "$BOGO_REQUEST_ID"
```

The first terminal prints `Approved, and the protected call ran. BOGO is
protecting your agent.` Then `bogo doctor` checks the result; it notes that a
software test approver never uses Face ID:

```sh
target/debug/bogo --home "$BOGO_TRY/home" doctor --client-home "$BOGO_TRY/clients"
```

When you're done, stop that BOGO with `target/debug/bogo --home
"$BOGO_TRY/home" stop`, which unloads the LaunchAgent, and delete
`$BOGO_TRY` and `~/Library/LaunchAgents/si.eilmer.bogo.plist`. The LaunchAgent
has one name per Mac account, so don't do this while you run BOGO for real.

## Checks

`make check` runs everything CI runs: formatting, Clippy for both feature
sets, the tests, the Lean build, the vector match and `cargo deny`.
[CONTRIBUTING](../CONTRIBUTING.md) says what evidence a change needs.
