# Changelog

Changes a BOGO user would notice. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/). There are no tagged
releases yet; the first tag will collect everything under **Unreleased**.
Changes to the BOGO tab of the iPhone app are listed in the
[iPhone app's changelog](https://github.com/JohanSardar/apple-bridge-iphone/blob/main/CHANGELOG.md).

## Unreleased

### Added

- One-line install for Apple Silicon Macs:
  `curl -fsSL https://bogo.eilmer.si/install.sh | sh`, from GitHub releases
  that CI builds when a version tag is pushed ([release rule](docs/release.md)).
- `bogo --version` prints the commit it was built from.
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

- Secrets shorter than 8 characters are refused.
