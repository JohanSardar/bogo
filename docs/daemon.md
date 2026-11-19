# Running BOGO on the Mac

<a id="the-short-way-bogo-setup"></a>

## The short way: `bogo setup`

```sh
bogo setup
```

Setup creates the home, starts BOGO in the background, pairs your iPhone,
moves your MCP clients' servers behind BOGO and makes one protected test call.
`bogo doctor` checks the result at any time, and `bogo setup --undo` puts back
the client files setup changed. [Install and set up](setup.md) covers each
step and what it changes on your Mac. The rest of this page is every command
setup runs for you, and the ones it doesn't.

## Recipes

`bogo upstream add NAME` with no command sets up a known server (`gmail`,
`github`, `slack`, `stripe` or `files`) at a pinned version, asks for its
credentials (with echo off) and writes a policy. [Recipes](recipes/README.md)
has a page for each.

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
# approvers = ["My iPhone", "Second approver"]
[agents.Researcher.tools.files__write_file]
policy = "deny"
```

`bogo tools` reports effective policies, agent overrides, missing approvers,
thresholds and schema risks. Missing approvers do not open a tool. Disconnected
upstreams keep their cached tools discoverable; calls are not sent.

## Grouped approvals

With BOGO 0.2.0 or later and Apple Bridge 0.4.0 or later, an orchestrator can
offer several alternatives for the same task. Check that the MCP initialize
result advertises `capabilities.experimental["bogo.dev/approvalGroup"]`
before sending grouped calls. Submit each option as a
separate MCP `tools/call` with the same `params._meta["bogo.dev/approvalGroup"]`
object. Each call keeps its own tool, arguments, worker identity, and session.

```json
{
  "jsonrpc": "2.0",
  "id": 7,
  "method": "tools/call",
  "params": {
    "name": "mail__send_reply",
    "arguments": {
      "to": "maya@example.test",
      "body": "I'll send a preview on Thursday."
    },
    "_meta": {
      "bogo.dev/approvalGroup": {
        "id": "e9208b4f7e89424083d86c97bb34f4988",
        "title": "Reply to Maya"
      }
    }
  }
}
```

Generate a fresh random 128-bit ID per task and encode it as 32 lowercase hex
digits. Share the ID and title with the workers you want to contribute to
that task. Submit the options concurrently and keep each call open. Options
appear in the group as they arrive. Use protected tools in `wait` mode.
BOGO treats the title as agent-provided text and strips the metadata from
the upstream call. Configure display hints on the tools to highlight values
such as an item name, price, or draft body in the comparison.

The first option to meet its normal approval threshold closes the group.
Its agent receives the normal call result. The other live calls return an
error explaining that another option was selected. If a worker disconnects
while the group is waiting, only that worker's requests end. The selection
survives daemon restarts and closes the group ID even if the selected call
could not be sent. A fresh group ID represents a new decision.

## Approvers

To add your iPhone, follow [Add your iPhone](add-your-iphone.md). It needs
only `[approvers.iphone]`; the relay wakes the phone, so `[apns]` is optional.
`approve_arm_delay` has no shipped value. Set it deliberately. The relay needs
no setting: BOGO uses the free public relay unless `[relay] url` names your own
([self-hosting](self-hosting-relay.md)). Retention and mailbox size are the
relay's own flags; older `[relay]` tables that still set `max_message_age` or
`mailbox_capacity` load, and those keys are ignored.

If you do add `[apns]`, put the APNs key under BOGO's private directory and
refer to it with a path secret in `secrets.toml`. `BOGO_APP_BUNDLE_VERSIONS` (comma-separated build
numbers) is optional: real App Attest data carries no build number, so it only
matters if Apple ever includes one. For a private test relay,
`BOGO_RELAY_CA` names its PEM certificate. `BOGO_MAC_NAME` sets the registration
label. These three non-secret settings are copied into the user LaunchAgent;
credentials are not copied into it.

```sh
bogo approver add 'My iPhone'
bogo approver list
bogo approver revoke 'My iPhone'
```

While `approver add` runs, the Mac shows up in BOGO → Macs → Add Mac on any
iPhone on the same network. Tap it, check that the terminal and the phone show
the same 6 digits, answer `y` in the terminal, then confirm on the phone with
Face ID. A mismatch or a "no" on either side registers nothing. A phone on
another network taps "Mac not listed? Use a code" and scans the QR code the
command prints, or types the `bogo1-` code under it; the 6 digits are compared
the same way. If registration fails, the command prints the reason the phone
gave. The command,
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
path without App Attest; the production build requires it.
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
