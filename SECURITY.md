# Security policy

BOGO decides whether an AI agent's tool call may run. A flaw in it can let an
agent do something you never approved, so security reports are the most
useful thing you can send.

## What BOGO promises

For any tool call that passes through BOGO, whatever the agent sends:

- **It fails closed.** If anything is missing, invalid, ambiguous, revoked,
  expired or broken, the call does not run.
- **The agent is untrusted.** The guarantees hold for any messages, in any
  order, at any rate.
- **Approvals are exact and single use.** An approval covers one call: the same
  upstream, tool, arguments, justification, agent and session. It allows at
  most one execution.
- **You see what you sign.** The phone builds the Review card from the exact
  bytes it signs, never from a separate summary.
- **New tools are protected.** A tool BOGO hasn't been told about needs
  approval.
- **The core is proved.** The part that decides "run or reject" is written in
  Lean and proved, and the daemon runs that same code.
- **The relay is trusted with nothing.** Requests are encrypted end to end and
  decisions are signed, so the relay can't read arguments or forge approvals.
- **The log can be checked.** Every request and outcome goes into an
  append-only, hash-chained log that `bogo log verify` checks.

Where these promises stop is written down:

- [Threat model](docs/threat-model.md): what BOGO defends against, and what it
  doesn't, such as an agent with its own shell.
- [What is proved / What is assumed](docs/proved-and-assumed.md): the exact
  proved statements and every assumption they rest on.
- [What is verified](docs/status.md): what has been run, and what hasn't yet,
  such as a session on a real iPhone.

## Reporting a vulnerability

Report it privately through GitHub:
[**Report a vulnerability**](https://github.com/JohanSardar/bogo/security/advisories/new).
Please don't open a public issue, pull request or discussion for it.

This covers everything in this repo, the BOGO tab of the
[Apple Bridge iPhone app](https://github.com/JohanSardar/apple-bridge-iphone),
and the relay at `bogo.eilmer.si`. Other problems in the iPhone app go to its
[own private reporting](https://github.com/JohanSardar/apple-bridge-iphone/security/advisories/new).
If you're not sure which one fits, pick either: a report in the wrong place
gets moved, not lost.

Include what you can: the version or commit, your setup, steps to reproduce,
and what an attacker gains. A report that shows a call running without a
matching approval, an approval being reused or stretched, the Review card
showing something other than what runs, or the relay learning request
contents is the most serious kind.

BOGO is maintained by one person, so replies can take a few days. You'll get
an acknowledgement, then a fix or a plan once the report is understood. You'll
be credited in the advisory unless you ask not to be.

Please test only against your own installation. Don't test against the public
relay in ways that could affect other people, and don't access data that isn't
yours.

## Supported versions

There are no releases yet. Only the latest `main` gets security fixes.

## Out of scope

These are known limits, described in the threat model, not vulnerabilities:

- An agent acting outside MCP, through its own shell, browser or credentials,
  when it isn't run in the [reference sandbox](sandbox/README.md).
- The software test approver signing without Face ID. It exists only in builds
  with the `test-approvers` feature, for demos and tests.
- A compromised Mac account, phone OS or upstream MCP server.
- The owner approving a harmful call that BOGO displayed accurately.
