# BOGO docs

New here? Start with the [README](../README.md): what BOGO is, the five-minute
start, and how it works. Words like approver, relay and Review card are
defined in the [glossary](../GLOSSARY.md).

## Get started

- [Install and set up](setup.md): the one-line install, `bogo setup` and
  `bogo doctor`, and what each one changes on your Mac.
- [Add your iPhone](add-your-iphone.md): install the app, pair it with your
  Mac on the same network or with a code, and approve your first call.
- [Recipes](recipes/README.md): put [Gmail](recipes/gmail.md),
  [GitHub](recipes/github.md), [Slack](recipes/slack.md),
  [Stripe](recipes/stripe.md) or a folder of [files](recipes/files.md) behind
  BOGO, with a policy that protects sending, writing and deleting.
- [Connect an agent](agents.md): MCP configs for Claude Code and other clients.
- The iPhone app: [join the TestFlight beta](https://testflight.apple.com/join/dQu75bv1)
  (it admits testers once Apple's Beta App Review passes the first build), or
  [build it](https://github.com/JohanSardar/apple-bridge-iphone#build-and-check)
  from the Apple Bridge iPhone app repo.

## Use BOGO

- [Configuration](configuration.md): every `bogo.toml` key, tool policies,
  per-action and per-agent rules, and the iPhone settings.
- [FAQ](faq.md): trust, what you approve, what happens when nobody answers.
- [What is verified](status.md): what has been run and recorded, what hasn't,
  and known issues.

## Run BOGO

- [Run BOGO on your Mac](daemon.md): every `bogo` command: start and stop the
  daemon, add approvers and agents, pause, revoke, cancel, export and recover.
- [Running your own relay](self-hosting-relay.md): host the relay yourself,
  on a server or on your Mac.
- [Reference sandbox](../sandbox/README.md): run an agent so BOGO is its only
  way out, and the bypass suite that checks it.
- [Build from source and try it](build-from-source.md): build BOGO, try it
  with a software test approver in place of the phone, and run the checks.
- [Releasing BOGO](release.md): versions, how a release is cut, and signing.

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
- [Security policy](../SECURITY.md), including what BOGO depends on, and
  [contributing](../CONTRIBUTING.md).

## Internal records

[How BOGO was built](internal/README.md), kept for review and audit: the
verification records, the tickets each piece was built from, the design rounds
and the research. You don't need them to use BOGO. They describe the code as
it was when they were written.

- [Verification records](internal/verification/), starting with the
  [quick start record](internal/verification/user-docs.md).
- [Relay deployment](../deploy/relay/README.md): how the public relay at
  `bogo.eilmer.si:8443` is run. For hosting your own, see
  [Running your own relay](self-hosting-relay.md).
