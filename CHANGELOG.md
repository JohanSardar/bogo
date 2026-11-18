# Changelog

Changes a BOGO user would notice. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/). There are no tagged
releases yet; the first tag will collect everything under **Unreleased**.
Changes to the BOGO tab of the iPhone app are listed in the
[iPhone app's changelog](https://github.com/JohanSardar/apple-bridge-iphone/blob/main/CHANGELOG.md).

## Unreleased

### Added

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
- The public relay at `bogo.eilmer.si:8443`.
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

- A failed `bogo approver add` prints what the phone reported, for example a
  Keychain or App Attest error, instead of always "the phone cancelled".
- Secrets shorter than 8 characters are refused.
