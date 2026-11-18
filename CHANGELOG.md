# Changelog

Changes a BOGO user would notice. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Each release is a
`v<version>` tag with downloads on the
[releases page](https://github.com/JohanSardar/bogo/releases).
Changes to the BOGO tab of the iPhone app are listed in the
[iPhone app's changelog](https://github.com/JohanSardar/apple-bridge-iphone/blob/main/CHANGELOG.md).

## Unreleased

### Added

- A five-minute start on the front page: install, get the app, `bogo setup`.
- [Install and set up](docs/setup.md): what the install, `bogo setup` and
  `bogo doctor` each change on your Mac.
- A page per [recipe](docs/recipes/README.md), starting with
  [Gmail](docs/recipes/gmail.md).
- FAQ answers on who sends the push and what Apple sees, and on why the
  iPhone part lives in the Apple Bridge app.
- A [Code of Conduct](CODE_OF_CONDUCT.md), and a Dependencies section in the
  [security policy](SECURITY.md).

### Changed

- The build-from-source quick start moved to
  [Build from source and try it](docs/build-from-source.md).
- Internal records (tickets, design rounds, verification records, research)
  moved under [`docs/internal/`](docs/internal/README.md).

## [0.1.0] - 2026-11-17

### Added

- `bogo setup`: one guided command from install to a protected agent. It
  creates the home, starts BOGO in the background, pairs your iPhone, moves
  the MCP servers already set up in Claude Code, Claude Desktop and Cursor
  behind BOGO (credentials into `secrets.toml`, a policy suggested from each
  tool's annotations for you to confirm), and makes one protected test call.
  Each step is skipped when done; `bogo setup --undo` restores every client
  file it changed.
- `bogo doctor`: checks the daemon, port, paired phone, App Attest
  environments, relay, each upstream and each client config, with a fix for
  each problem.
- Recipes: `bogo upstream add gmail` (also `github`, `slack`, `stripe` in test
  mode and `files`) installs a pinned, open-source MCP server and writes a
  policy that protects sending, writing and deleting.
- Failure traces: a failed relay fetch, push, delivery or upstream start now
  leaves one line in `run/stderr.log`, with no secrets or arguments.
- Connected agents are told when an upstream's tool list changes.
- `bogo-shim` reads its token from `BOGO_AGENT_TOKEN_FILE` as well.

- One-line install for Apple Silicon Macs:
  `curl -fsSL https://bogo.eilmer.si:8443/install.sh | sh`, from GitHub releases
  that CI builds when a version tag is pushed ([release rule](docs/release.md)).
- `bogo --version` prints the commit it was built from.
- The relay wakes your phone, so a Mac needs no `[apns]` section or push key.
  The push says only "Approval requested".
- Pairing from another network: `bogo approver add` prints a code and a QR code
  of it. On the iPhone, tap "Mac not listed? Use a code". The 6 digits are
  compared as before, and nearby pairing is unchanged.
- [Add your iPhone](docs/add-your-iphone.md), a guide from install to the first
  approval.
- The relay answers `/healthz`, a one-line `/`, and `/install.sh`, which
  redirects to the install script in this repository.
- Front page, [docs index](docs/README.md), [FAQ](docs/faq.md),
  [agent setup](docs/agents.md), [what is verified](docs/status.md),
  security policy, contributing guide and issue and pull request templates.
- Display roles: requests can mark a title, subtitle and image for the Review
  card (`title`, `subtitle`, `image` highlight formats).
- Per-action policy for wrapper tools such as Link's `call_write_tool`:
  `[tools."tool[arg=value]"]` entries, with a recommended Link setup in the
  [configuration reference](docs/configuration.md).
- The public relay at `bogo.eilmer.si:8443`. BOGO uses it by default, with no
  setting; `[relay] url` points a Mac at [your own](docs/self-hosting-relay.md).
- Pairing over the relay: `bogo approver add` advertises the Mac over Bonjour,
  so the phone's Add Mac lists Macs nearby, and the 6-digit check runs over a
  private relay room.
- User docs: the verified [quick start](README.md#quick-start-on-macos),
  [configuration reference](docs/configuration.md),
  [threat model](docs/threat-model.md) and
  [What is proved / What is assumed](docs/proved-and-assumed.md).
- The [reference sandbox](sandbox/README.md) and its bypass suite.
- The daemon and `bogo` CLI: agents, approvers, tool policies, pending
  requests, pause and revoke, the hash-chained audit log with `bogo log
  verify`, a user LaunchAgent, and the software test approver for demos.
- The approver hub and the iPhone route: numeric-comparison registration with
  App Attest, encrypted delivery through BOGO's relay, and APNs pushes.
- The MCP front agents connect to, and `bogo-shim` for stdio clients.
- The Lean decision core, its proofs and the request encoding, compiled into
  the daemon.
- Upstream MCP servers behind BOGO, with output filtering.
- End-to-end encryption and relay crates.

### Fixed

- A dangling symlink path secret is judged by its target.
- Real App Attest data, which has no extension map, is accepted.

### Changed

- A new home accepts both App Attest environments, so the TestFlight and App
  Store app pairs with no config edit.
- `bogo log verify` says what it checked in plain words, and a `--home` too
  long for BOGO's socket is explained instead of failing with `SUN_LEN`.
- A failed `bogo approver add` prints what the phone reported, for example a
  Keychain or App Attest error, instead of always "the phone cancelled".
- Secrets shorter than 8 characters are refused.
- `[relay] max_message_age` and `mailbox_capacity` are ignored. Older files
  still load; retention and capacity are now `bogo-relay` flags.
