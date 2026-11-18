# Install and set up

Three commands take you from nothing to a protected agent: the install, then
`bogo setup`, then `bogo doctor` whenever you want to check. This page says
what each one does and what it changes on your Mac. You also need the iPhone
app: [add your iPhone](add-your-iphone.md) covers it.

## Install

```sh
curl -fsSL https://bogo.eilmer.si:8443/install.sh | sh
```

The `:8443` is part of the address: it is the port BOGO's relay answers on.
The short URL forwards to [`install.sh`](../install.sh) in this repo, so you
can read the script first, or run it straight from
`https://raw.githubusercontent.com/JohanSardar/bogo/main/install.sh`.

The script:

1. checks that this is an Apple Silicon Mac;
2. downloads `bogo-aarch64-apple-darwin.tar.gz` and `SHA256SUMS` from the
   latest [GitHub release](https://github.com/JohanSardar/bogo/releases);
3. checks the archive's SHA-256, and stops with `Nothing was installed.` if it
   doesn't match;
4. copies `bogo` and `bogo-shim` into `~/.local/bin`, replacing older copies;
5. runs `bogo --version`, the only downloaded program it runs.

```
bogo-install: downloading the latest release
bogo-install: installed bogo 0.1.0 (commit 7ee1e72b3f92) into /Users/you/.local/bin
bogo-install: next step: bogo setup
```

**What it changes on your Mac:** the two files in `~/.local/bin`, and nothing
else. It needs no sudo. If that folder isn't on your `PATH`, it prints the line
to add to `~/.zshrc`, but doesn't add it. Set `BOGO_VERSION=v0.1.0` to pin a
release, or `BOGO_INSTALL_DIR` to install elsewhere. Run it again to upgrade.

The programs are ad-hoc signed, not notarized. `curl` doesn't mark files as
downloaded, so they run with no Gatekeeper prompt. A browser download is
blocked; [releasing BOGO](release.md#signing-and-gatekeeper) has the fix.

## `bogo setup`

```sh
bogo setup
```

Setup runs five steps. Each one says in one line what it does, and skips
itself when it's already done, so you can run setup again at any time:

```
1/5 Create BOGO's home at /Users/you/.bogo.
2/5 Start BOGO in the background, so it runs at login.
3/5 Add your iPhone, so protected calls wait for your Face ID.
4/5 Connect an agent: move each MCP client's servers behind BOGO.
5/5 Make one protected test call, and approve it on your phone.
```

1. **home** creates BOGO's home, `~/.bogo`, with new keys and a private
   secrets file. It asks three things: how many requests one agent may have
   waiting at once, how long Approve stays off when a request opens on the
   phone (so you read it first), and another port if 8790 is taken.
2. **start** starts BOGO in the background with a user LaunchAgent, so it also
   runs at login.
3. **phone** asks what to call your phone, then pairs it. On the iPhone, open
   BOGO, then Macs, then Add Mac, and pick this Mac; both screens show the same
   6 digits. [Add your iPhone](add-your-iphone.md#3-pair-them) covers this
   step, including pairing from another network with a code.
4. **agent** finds the MCP servers in Claude Code, Claude Desktop and Cursor
   and offers to move each one behind BOGO. It starts each server once to list
   its tools and suggests a policy for each tool: allow if the server marks it
   read-only, approve otherwise. You confirm or change each one. Then the
   client's entries are replaced by one `bogo` entry, which runs `bogo-shim` as
   a new agent. If it finds no client config, it writes one for Claude Code
   into BOGO's home and prints the `claude --mcp-config …` command to use it.
5. **test** makes one call to BOGO's own check tool, which only answers
   "pong", and waits for you to approve it on the phone. Then it prints
   `Approved, and the protected call ran. BOGO is protecting your agent.`

Run a single step with `bogo setup STEP`, for example `bogo setup agent` to
change the policies you chose.

`--yes` takes every suggestion without asking, but it never allows a tool: a
tool setup would suggest allowing stays at approve until you choose. If input
ends, setup stops rather than take a suggestion.

### What `bogo setup` changes

| Where | What |
|---|---|
| `~/.bogo/` | BOGO's private home (mode 0700): `bogo.toml`, `secrets.toml`, keys, the audit log and its state. Agents' tokens go in `~/.bogo/agents/`. |
| `~/Library/LaunchAgents/si.eilmer.bogo.plist` | The LaunchAgent that runs `bogo serve` at login. BOGO's logs go to `~/.bogo/run/`. |
| `~/.claude.json` (including each project's own servers) | Claude Code's MCP servers, if you move them. |
| `~/Library/Application Support/Claude/claude_desktop_config.json` | Claude Desktop's MCP servers, if you move them. |
| `~/.cursor/mcp.json` | Cursor's MCP servers, if you move them. |
| `~/.bogo/setup/backups/` | A copy of each client file, taken before its first change. |

**Credentials.** Values in a server's `env` or `headers` that look like
credentials move into `~/.bogo/secrets.toml`, and `bogo.toml` refers to them by
name. They no longer appear in the client file.

**The agent's token.** Setup never shows it. It goes in
`~/.bogo/agents/NAME.token` (mode 0600), and the client's `bogo` entry names
that file as `BOGO_AGENT_TOKEN_FILE`:

```json
"bogo": {
  "command": "/Users/you/.local/bin/bogo-shim",
  "args": [],
  "env": {
    "BOGO_AGENT_TOKEN_FILE": "/Users/you/.bogo/agents/claude-code.token",
    "BOGO_URL": "http://127.0.0.1:8790/mcp"
  }
}
```

An agent that has a shell running as you can still read that file, as it can
read anything your account can. Run such agents in the [reference
sandbox](../sandbox/README.md).

### Undo

```sh
bogo setup --undo
```

This puts back each client file setup changed, byte for byte and with its file
mode. The agents setup added stay registered until you revoke them (it prints
the `bogo agent revoke` commands), and the upstreams and secrets stay in
`bogo.toml` and `secrets.toml`. To stop BOGO, run `bogo stop`.

### Other options

- `--home DIR` uses DIR as BOGO's home instead of `~/.bogo`. Keep it short:
  macOS limits the socket path inside it to about 100 characters, and BOGO
  says so if it's too long.
- `--client-home DIR` looks for the client configs in DIR instead of your home
  folder.
- `--approver-kind soft` adds a software test approver in place of the phone.
  It works only in a test build; see [build from source and
  try it](build-from-source.md#try-bogo-setup-with-the-software-test-approver).

## `bogo doctor`

```sh
bogo doctor
```

Doctor checks everything a working setup needs and changes nothing. Each line
is `ok`, `note` or `FAIL`, and each failure comes with a one-line fix. It exits
nonzero if anything failed.

```
ok    bogo.toml and secrets.toml load
ok    BOGO is running
ok    BOGO answers on port 8790
ok    1 iPhone(s) paired
ok    App Attest accepts TestFlight, App Store and Xcode builds
ok    the relay https://bogo.eilmer.si:8443 is reachable
ok    upstream files starts
ok    Claude Code reaches BOGO as agent Claude Code
Everything BOGO needs is in place.
```

(An example. The healthy run recorded for this page used a software test
approver, so its phone line was a `note` saying so.)

It checks the config, that the daemon runs and answers on its port, that a
phone is paired, that App Attest accepts the app you'll use, that the relay
answers, that each upstream starts, and that each client config's token
belongs to an agent BOGO knows. `--client-home DIR` looks for client configs in
DIR.

## Next

- [Recipes](recipes/README.md): add Gmail, GitHub, Slack, Stripe or a folder of
  files with a policy that protects sending, writing and deleting.
- [Connect an agent](agents.md) by hand, for clients setup doesn't know.
- [Run BOGO on your Mac](daemon.md): every other `bogo` command.
- [Configuration](configuration.md): every `bogo.toml` key.
