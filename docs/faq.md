# FAQ

### Why should I trust BOGO?

You shouldn't take it on faith. The part that decides whether a call may run is
written in Lean and proved, and [What is proved / What is
assumed](proved-and-assumed.md) lists exactly where that proof stops and what
it relies on. The [threat model](threat-model.md) says what BOGO defends
against and what it doesn't, and the [verification records](status.md) show
what has actually been run, including what hasn't been yet. The code is MIT
licensed and every change is reviewed against recorded evidence
([CONTRIBUTING](../CONTRIBUTING.md)).

### Is BOGO ready to protect real actions?

Not yet. The Mac side works end to end with a software test approver, and the
iPhone side works in the Simulator, but no real-iPhone session has been
recorded. See [what is verified](status.md).

### Does BOGO stop my agent from doing anything it likes?

Only through MCP. BOGO controls the tool calls that pass through it. If the
agent also has a shell, a browser, or credentials of its own, it can act
without asking BOGO. Disable those tools, or run the agent in the [reference
sandbox](../sandbox/README.md), where BOGO is the only way out.

### What exactly do I approve?

One tool call, byte for byte: the upstream, the tool and every argument, plus
which agent asked and its justification. The phone decodes and shows the same
bytes it signs; **All details** on the Review card shows every value. If the agent
changes one character, that is a new request that needs a new approval.

### Can I trust the agent's justification?

No. It is the agent's own explanation, shown so you have context, and the
Review card shows it under "Reason", marked "Not verified by BOGO". It is never
sent upstream. Judge the call by what will run.

### What happens if I don't answer?

Nothing runs. A request ends when the agent's MCP session ends or its client
gives up, or when an optional per-tool expiry passes. Approving after that is
refused. See [ADR 0002](adr/0002-approvals-bound-to-session-no-default-expiry.md).

### What if my phone is offline?

The request waits. BOGO sends it through the relay, which holds it until your
phone collects it, and the push wakes the app when it can. If the session ends
first, the request ends unanswered and nothing runs.

### What does the relay see?

Ciphertext and routing data: which mailboxes talk, when, and how much. It holds
no key that can read a request or approve one. The relay's own documents
describe it in detail: [relay protocol](RELAY-PROTOCOL.md) and
[deployment](../deploy/relay/README.md).

### Why does the quick start use a "software test approver"?

So you can try BOGO without an iPhone. It signs on the Mac without Face ID,
and anyone with your Mac account can use it. It only exists in builds with the
`test-approvers` feature. Never use it to protect real actions.

### What if the network fails after BOGO sends a call?

The provider may already have acted. BOGO records the outcome as unknown and
never retries it on its own. Check with the provider before you try again.

### Which agents work with BOGO?

Any MCP client. Claude Code is the one that has been run; see
[connect an agent](agents.md) for configs.

### Why is the iPhone part in the Apple Bridge app?

The owner chose to build BOGO's iPhone approver into the existing Apple Bridge
iPhone app rather than ship a second app. BOGO shares no server, relay or key
with Apple Bridge; it only shares the app. See
[Which repo do I want?](../README.md#which-repo-do-i-want).

### Where do I report a bug or a vulnerability?

Bugs in BOGO, including in the BOGO tab, go to
[this repo's issues](https://github.com/JohanSardar/bogo/issues/new/choose).
Vulnerabilities go through [private reporting](../SECURITY.md), never a public
issue.
