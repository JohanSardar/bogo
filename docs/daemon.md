# Running BOGO on the Mac

## The short way: `bogo setup`

```sh
bogo setup
```

Setup says in one line what each step does, and skips a step that is already
done, so it is safe to run again:

1. **home**: creates `~/.bogo` and asks for the few owner values (how many
   requests an agent may have waiting, how long Approve stays off on the phone,
   and another port if 8790 is taken). The home accepts the TestFlight, App
   Store and Xcode builds of the app.
2. **start**: starts BOGO in the background with a user LaunchAgent.
3. **phone**: pairs your iPhone with the same 6-digit check as `bogo approver
   add`; skipped when an approver exists.
4. **agent**: finds the MCP servers in Claude Code (`~/.claude.json`), Claude
   Desktop and Cursor (`~/.cursor/mcp.json`) and offers to move each one behind
   BOGO. Each becomes an upstream; credentials in its `env` or `headers` move to
   `secrets.toml` and are referred to by name. Setup lists each server's tools
   and suggests a policy from the annotations the server publishes:
   `readOnlyHint` suggests allow; `destructiveHint`, a name that sends, pays,
   writes or deletes, or no hint at all suggests approve. You confirm or change
   each one. The client's entries are replaced by one `bogo` entry that runs
   `bogo-shim` as its own agent.
5. **test**: makes one protected call to BOGO's own check tool, which does
   nothing but answer "pong", and waits for you to approve it on the phone.

Run one step with `bogo setup STEP`. `--yes` takes every suggestion without
asking, except that it never allows a tool: a suggested allow stays approve.
`--client-home DIR` looks for client configs in DIR instead of `HOME`.

Setup backs up each client file before its first change, to
`~/.bogo/setup/backups`. `bogo setup --undo` puts each one back byte for byte;
the upstreams and secrets it added stay in `bogo.toml` and `secrets.toml`, and
the agents it added stay until you revoke them.

**Where the agent's token goes.** Setup never shows it. It writes the token to
`~/.bogo/agents/NAME.token` (0600, in BOGO's private home, which no upstream
may be pointed at) and puts only that file's path in the client's config, as
`BOGO_AGENT_TOKEN_FILE`. An agent that has a shell running as you can still
read that file, as it can read anything your account can; run such agents in
the [reference sandbox](../sandbox/README.md).

## Recipes

`bogo upstream add NAME` with no command sets up a known server at a pinned
version, asks for its credentials (with echo off) and writes a policy:

| Recipe | Server | Allowed | Waits for the phone |
|---|---|---|---|
| `gmail` | [workspace-mcp](https://github.com/taylorwilsdon/google_workspace_mcp) 2.3.0 | searching and reading mail, labels and filters | sending, drafting, attachments, changing labels or filters |
| `github` | [github-mcp-server](https://github.com/github/github-mcp-server) 2.0.3, checked against its SHA-256 | tools GitHub marks read-only, once you confirm | writes, pull requests, issues, merges |
| `slack` | [slack-mcp-server](https://github.com/korotovsky/slack-mcp-server) 1.3.0 | what you confirm | posting messages |
| `stripe` | Stripe's hosted server (test-mode keys only) | what you confirm | payment links, refunds, invoices and other writes |
| `files` | `@modelcontextprotocol/server-filesystem` 2.3.0, one folder | reading and listing | writing, editing, moving |

A recipe prints the sign-in steps you do yourself; for Gmail that is your own
Google Cloud OAuth client and a Google sign-in link the first call returns.
Stripe hosts its server, so that recipe can't pin a version. Servers install
under `~/.local/share/bogo/upstreams`, outside BOGO's home. `bogo upstream
recipes` lists them.

## Checking a setup: `bogo doctor`

`bogo doctor` checks, one line each, that the config loads, BOGO runs and
answers on its port (or the port is free), a phone is paired, the App Attest
environments accept TestFlight, the relay answers, each upstream starts, and
each client config's `bogo` entry reaches BOGO with an active agent's token.
Each problem comes with its fix. It exits nonzero if anything failed.

## Logs

BOGO keeps two records:

- **The audit log**, `~/.bogo/log/audit.jsonl`: every call, request,
  decision and outcome, hash-chained. Read it with `bogo log`.
- **Failure traces**, on the daemon's stderr: `~/.bogo/run/stderr.log` when
  BOGO runs in the background, the terminal when you run `bogo serve`. A
  failed relay fetch, a refused or failed push, a request that could not be
  delivered to the phone, and an upstream that does not start, does not answer
  or drops each leave one timestamped line, such as `upstream gmail did not
  connect: it could not be started or reached, or closed before answering`.
  These lines carry only names and a fixed reason: never an argument, a token,
  a key, a credential or anything an upstream writes to its own stderr, and
  `BOGO_LOG` can't turn on more.

## By hand

Build the CLI, daemon and shim together:

```sh
cargo build -p bogo -p bogo-daemon -p bogo-shim
./target/debug/bogo init
```

`init` creates the private BOGO directory, keys, an owner-edited configuration,
an empty 0600 `secrets.toml`, and a concrete user LaunchAgent plist. It preserves
existing configuration. No system service, Keychain write, or password is used.
Use an absolute path to an installed binary; launchd must still find it at login.
`bogo-daemon --home DIRECTORY` is also available for foreground operation;
the user LaunchAgent runs `bogo --home DIRECTORY serve`.

Set `[defaults] max_pending` explicitly in `~/.bogo/bogo.toml`, then add an upstream:

```sh
bogo upstream add files -- npx -y @modelcontextprotocol/server-filesystem@2.3.0 /absolute/scratch/folder
bogo agent add 'Trip planner'
bogo start
bogo status
bogo tools
```

All commands accept `--home DIRECTORY` to select a separate BOGO directory
without overriding the process's home. The upstream add command requires BOGO
to be stopped. Config changes take effect on restart. Default policy `allow`,
unknown configuration keys, unsafe secrets permissions, path secrets outside
BOGO's directory, and upstream paths into that directory or its parents are
refused. Commands and arguments with path syntax are resolved through symlinks.
The strict body parser currently supports at most 1MiB; larger configured limits
are refused explicitly.

Copy the token printed by `agent add` once into the client's private connection
configuration. Only its SHA-256 enters BOGO's registry and log. For Claude Code,
use the stdio shim to bind the lifetime of pending calls to the client process:

```json
{"mcpServers":{"bogo":{"command":"/absolute/path/bogo-shim"}}}
```

Give that process `BOGO_URL=http://127.0.0.1:8790/mcp` and
`BOGO_AGENT_TOKEN` in its environment. HTTP MCP clients can use the same URL with
`Authorization: Bearer <agent token>`. The shim is preferred for clients that can
silently abandon otherwise live HTTP sessions; it keeps a separate liveness
stream and closes the BOGO session when the process dies.

Unlisted tools require approval. To open a particular read and protect writes,
edit `bogo.toml` and restart:

```toml
[tools.files__read_text_file]
policy = "allow"
[tools.files__write_file]
policy = "approve"
justification = "required"
# require = { k = 2 }
# approvers = ["Johan iPhone", "Second approver"]
[agents.Researcher.tools.files__write_file]
policy = "deny"
```

`bogo tools` reports effective policies, agent overrides, missing approvers,
thresholds and schema risks. Missing approvers do not open a tool. Disconnected
upstreams keep their cached tools discoverable; calls are not sent.

## Approvers

Configure `[apns]` and `[approvers.iphone]` following the architecture.
`approve_arm_delay` has no shipped value. Set it deliberately. The relay needs
no setting: BOGO uses the free public relay unless `[relay] url` names your own
([self-hosting](self-hosting-relay.md)). Retention and mailbox size are the
relay's own flags; older `[relay]` tables that still set `max_message_age` or
`mailbox_capacity` load, and those keys are ignored.

Put the APNs key under BOGO's private directory and refer to it with a path
secret in `secrets.toml`. `BOGO_APP_BUNDLE_VERSIONS` (comma-separated build
numbers) is optional: real App Attest data carries no build number, so it only
matters if Apple ever includes one. For a private test relay,
`BOGO_RELAY_CA` names its PEM certificate. `BOGO_MAC_NAME` sets the registration
label. These three non-secret settings are copied into the user LaunchAgent;
credentials are not copied into it.

```sh
bogo approver add 'Johan iPhone'
bogo approver list
bogo approver revoke 'Johan iPhone'
```

While `approver add` runs, the Mac shows up in BOGO → Macs → Add Mac on any
iPhone on the same network. Tap it, check that the terminal and the phone show
the same 6 digits, answer `y` in the terminal, then confirm on the phone with
Face ID. A mismatch or a "no" on either side registers nothing. The command,
not the daemon, shows the Mac on the network, so run it from a terminal.
Reusing a name registers a new key and revokes the old approver. Every completed registration
and revocation enters the audit log before it affects requests. Registry caches
are never used as authority by the daemon.

For a deliberately insecure verification build only:

```sh
cargo build -p bogo --features test-approvers,test-registration
bogo approver add 'Soft A' --kind soft
bogo request list
bogo soft 'Soft A' FULL_REQUEST_ID
bogo soft 'Soft A' FULL_REQUEST_ID --deny --note 'wrong amount'
```

`test-approvers` adds the software route and Ed25519 approval verifier; neither
is in the production daemon's dependency graph. Test seeds stay in
`secrets.toml`. `test-registration` allows the existing simulator registration
path without App Attest or APNs credentials; the production build requires both.
An arm delay is still required whenever the iPhone route is
configured. A software route never counts as Face ID or hardware-backed approval.

## Owner operations and recovery

```sh
bogo agent list
bogo agent pause 'Trip planner'
bogo agent unpause 'Trip planner'
bogo agent revoke 'Trip planner'
bogo request list
bogo request cancel FULL_REQUEST_ID
bogo log
bogo log search denied
bogo log verify
bogo export --format jsonl
bogo log export --format csv
bogo stop
```

Mutations use the owner-only Unix socket while running. It checks the peer UID
and accepts bounded JSON lines. Stopped agent and revocation commands take the
same single-writer lock. Stopped log queries and exports are read-only.

The effects shell serializes lifecycle events, owner changes, approver decisions
and transport dispatch. It commits call, request, decision, authorization and
a dispatch-intent record before the final check, and result summaries before returning results.
After the intent is durable, dispatch is requeued behind lifecycle events observed
during that sync. An accepted core `forward` is returned without another await;
the forwarding record is queued before the gate reply and committed before any
result can be returned. A crash in between remains unknown from the already
durable call and authorization records. This ordering preserves the final gate
when durability and freshness would otherwise conflict.
It never stores upstream result bodies. The actual result is scrubbed by the
upstream manager and filtered before reaching the agent. Log verification checks
the hash chain, registered keys, canonical request bytes and approval signatures
through the compiled Lean decoder and verifier hook.

Every daemon boot starts a fresh core. Pending requests end; anything authorized
or forwarded without a terminal result is conservatively unknown. Nothing is
replayed. `stop` unloads KeepAlive, closes admission and sessions, and drains
those lifecycle events through a durability barrier. Startup refuses corruption;
only a torn final line is moved aside by the existing log recovery code.

A host agent with built-in shell access can bypass MCP; this daemon does not
claim to sandbox that process. Run agents inside the reference sandbox
([sandbox/README.md](../sandbox/README.md)): set `[server] allowed_hosts =
["172.30.0.2"]` so BOGO accepts the sandbox gate's address in the `Host` header.
