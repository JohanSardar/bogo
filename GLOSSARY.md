# BOGO

BOGO stands between an AI agent and the MCP servers that give it tools, so that a protected tool call runs only after the owner approves that exact call on a registered approver.

## People and parts

**Owner**:
The one person who configures BOGO and decides approval requests.
_Avoid_: user, admin

**Agent**:
An AI program the owner has named and given its own connection to BOGO; always treated as untrusted.
_Avoid_: client, model, bot

**Paused agent**:
An agent whose every request BOGO rejects until the owner un-pauses it on the Mac.
_Avoid_: blocked, suspended

**Agent name**:
The name the owner gives an agent, such as "Trip planner"; unlike the client's self-reported name, BOGO knows it is true.
_Avoid_: client name, user

**BOGO**:
The gateway on the owner's Mac that the agent sees as one MCP server, which holds all upstream credentials and enforces policy.
_Avoid_: Gate, proxy, firewall

**Upstream**:
A real, unmodified MCP server whose tools BOGO exposes to the agent.
_Avoid_: backend, provider, connector

**Approver**:
One registered route that shows the owner an approval request and returns a signed decision, such as the owner's iPhone.
_Avoid_: device, signer, authenticator

**Relay**:
BOGO's own internet mailbox that carries opaque messages between BOGO and approvers; trusted with nothing and unrelated to Apple Bridge's relay.
_Avoid_: server, proxy, cloud

**Registration**:
The one-time step in which an approver's public key becomes trusted by BOGO.
_Avoid_: pairing, enrollment, linking

**Session**:
One MCP connection from an agent to BOGO; approvals belong to the session that asked.
_Avoid_: conversation, client

## Calls and decisions

**Tool call**:
One request from the agent to run one upstream tool with specific arguments.
_Avoid_: command, action, invocation

**Policy**:
The rule for one tool: allow, approve or deny.
_Avoid_: rule, permission

**Policy key**:
The name a tool call's policy is looked up by: the tool itself, or `tool[arg=value]` when the owner gave that exact value of the tool's selected argument its own policy.
_Avoid_: action name, sub-tool

**Protected tool**:
A tool whose policy is approve.

**Justification**:
The agent's own, unverified explanation attached to a tool call; never forwarded upstream. The Review card labels it "Reason".
_Avoid_: reason, metadata, note

**Approval request**:
The exact tool call, justification, nonce, times and session that approvers are asked to decide.
_Avoid_: prompt, ticket, job

**Approval**:
An approver's signature over the exact bytes of one approval request, authorizing at most one execution before it expires.
_Avoid_: consent, grant, permission

**Denial**:
An approver's refusal of one approval request.
_Avoid_: rejection (reserved for BOGO refusing a call)

**Rejection**:
BOGO refusing to execute a tool call for any reason, including denial, expiry or invalid input.

## Software and repos

**Daemon**:
`bogo-daemon`, the process that is BOGO on the owner's Mac; `bogo serve` and the LaunchAgent run it.
_Avoid_: server, service, gateway process

**Shim**:
`bogo-shim`, the small stdio MCP server an agent's client starts, which passes every message to the daemon unchanged and ends the session when the client dies.
_Avoid_: proxy, bridge, adapter

**Software test approver**:
An approver whose key is a file on the Mac and which signs without Face ID; only in builds with the `test-approvers` feature, for demos and tests. Its CLI kind is `soft`.
_Avoid_: soft approver (in prose), mock approver

**Apple Bridge**:
The owner's separate product that gives agents the Mac's Messages, Mail, Calendar and more through a local MCP server; BOGO shares no code, server, relay or key with it.
_Avoid_: bridge (alone), gateway (for BOGO)

**Apple Bridge iPhone app**:
The iPhone app in the `apple-bridge-iphone` repo; it is Apple Bridge's iPhone companion and also carries the BOGO tab.
_Avoid_: BOGO app, the companion (alone)

**BOGO tab**:
The tab of the Apple Bridge iPhone app, built from `ios/BogoKit`, that is BOGO's iPhone approver.
_Avoid_: BOGO section, Approvals tab, BOGO app

**Review card**:
The BOGO tab's screen for one approval request, showing what will run with Slide to approve and Deny; All details shows every value of the exact request.
_Avoid_: approval screen, prompt, popup

**Stand-in Mac**:
A fake Mac built into Debug builds of the iPhone app, which sends realistic requests for demos and tests without a real BOGO.
_Avoid_: mock Mac, demo Mac
