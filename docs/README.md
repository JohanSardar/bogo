# BOGO docs

New here? Start with the [README](../README.md): what BOGO is, a quick start on
your Mac, and how it works. Words like approver, relay and Review card are
defined in the [glossary](../GLOSSARY.md).

## Use BOGO

- [Quick start](../README.md#quick-start-on-macos): try BOGO on your Mac with a
  software test approver, no iPhone needed.
- [Add your iPhone](add-your-iphone.md): install the app, pair it with your
  Mac on the same network or with a code, and approve your first call.
- [Connect an agent](agents.md): MCP configs for Claude Code and other clients.
- [Configuration](configuration.md): every `bogo.toml` key, tool policies,
  per-action and per-agent rules, and the iPhone settings.
- [FAQ](faq.md): trust, what you approve, what happens when nobody answers.
- [What is verified](status.md): what has been run and recorded, what hasn't,
  and known issues.
- The iPhone app: [join the TestFlight beta](https://testflight.apple.com/join/dQu75bv1)
  (open once Apple's Beta App Review passes the first build), or
  [build it](https://github.com/JohanSardar/apple-bridge-iphone#build-and-check)
  from the Apple Bridge iPhone app repo.

## Run BOGO

- [Run BOGO on your Mac](daemon.md): build, start and stop the daemon, add
  approvers and agents, pause, revoke, cancel, export and recover.
- [Reference sandbox](../sandbox/README.md): run an agent so BOGO is its only
  way out, and the bypass suite that checks it.
- [Relay deployment](../deploy/relay/README.md): how the public relay at
  `bogo.eilmer.si` is set up.

## How it works

- [Threat model](threat-model.md): what BOGO defends against, what it doesn't,
  and what it assumes.
- [What is proved / What is assumed](proved-and-assumed.md): the Lean proofs,
  in plain words, and where they stop.
- [Spec](SPEC.md): what BOGO does when finished.
- [Architecture](ARCHITECTURE.md): every part and how they fit.
- [Relay protocol](RELAY-PROTOCOL.md): the encrypted envelope, mailboxes and
  iPhone messages.
- Decisions (ADRs):
  - [0001 Our own signed encoding, defined in Lean](adr/0001-own-signed-encoding-defined-in-lean.md)
  - [0002 Approvals are bound to the session, with no default expiry](adr/0002-approvals-bound-to-session-no-default-expiry.md)
  - [0003 BOGO owns its relay and push](adr/0003-bogo-owns-its-relay-and-push.md)
- [Security policy](../SECURITY.md) and [contributing](../CONTRIBUTING.md).

## Internal records

How BOGO was built, kept for review and audit. You don't need these to use
BOGO. They record decisions and evidence at the time they were written, so
they may describe earlier states of the code.

- [Verification records](verification/): what was run for each piece, and
  what was seen.
  - [Quick start and user docs](verification/user-docs.md)
  - [Daemon and CLI](verification/daemon-cli.md)
  - [MCP front and shim](verification/front-shim.md)
  - [Sandbox, bypass suite and real upstreams](verification/sandbox-e2e.md)
  - [Display roles](verification/display-roles.md)
  - [Evidence: upstreams and filter](evidence/T5-upstreams-filter.md)
- [Tickets](tickets/): the brief each piece of work was built from, T1 onward.
  They name the owner's local checkouts as they were when written; on GitHub
  those are this repo and
  [apple-bridge-iphone](https://github.com/JohanSardar/apple-bridge-iphone).
- [Architecture rounds](architecture/): the two competing designs and their
  critiques that became [ARCHITECTURE.md](ARCHITECTURE.md), with the
  [settle note](architecture/settle-note.md).
- [Research](research/README.md): how BOGO's repos compare with five similar
  projects.
