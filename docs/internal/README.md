# Internal records

How BOGO was built, kept for review and audit. You don't need these to use
BOGO; the [docs index](../README.md) has the user pages. These records describe
decisions and evidence at the time they were written, so they may describe
earlier states of the code, and they may name the owner's local checkouts. On
GitHub those are this repo and
[apple-bridge-iphone](https://github.com/JohanSardar/apple-bridge-iphone).

Until 2026-11-17 these folders sat directly under `docs/` (for example
`docs/tickets/`). Links inside both repos point here now.

- **[Verification records](verification/)**: what was run for each piece, and
  what was seen.
  - [Quick start and user docs](verification/user-docs.md)
  - [Daemon and CLI](verification/daemon-cli.md)
  - [MCP front and shim](verification/front-shim.md)
  - [Sandbox, bypass suite and real upstreams](verification/sandbox-e2e.md)
  - [Display roles](verification/display-roles.md)
  - [Evidence: upstreams and filter](evidence/T5-upstreams-filter.md)
- **[Tickets](tickets/)**: the brief each piece of work was built from, T1
  onward.
- **[Architecture rounds](architecture/)**: the two competing designs and their
  critiques that became [ARCHITECTURE.md](../ARCHITECTURE.md), with the
  [settle note](architecture/settle-note.md).
- **[Research](research/README.md)**: how BOGO's repos compare with five
  similar projects.
- **[Relay deployment record](../../deploy/relay/DEPLOYMENT.md)**: what was run
  when the public relay was deployed.
