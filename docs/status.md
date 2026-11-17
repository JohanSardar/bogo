# What is verified

BOGO is early. This page says what has been run and recorded, and what hasn't,
so you can decide how far to trust it. Each line links to the record. When a
record and this page disagree, the record wins.

## Verified

- **The Mac side, end to end, with a software test approver.** The daemon, the
  `bogo` CLI and the shim ran with a real filesystem MCP server and real Claude
  Code. A protected write waited, was approved, and ran once; `bogo log verify`
  passed. The [quick start](../README.md#quick-start-on-macos) is that run.
  Records: [quick start](verification/user-docs.md),
  [daemon and CLI](verification/daemon-cli.md),
  [MCP front and shim](verification/front-shim.md).
- **The Mac's iPhone route, with a software phone.** The daemon's real iPhone
  route, registration, encrypted envelopes, a local relay and outcome notices
  worked with `bogo-simphone` standing in for the phone.
  Record: [daemon and CLI §4](verification/daemon-cli.md#4-thresholds-and-iphone-route).
- **The reference sandbox.** The bypass suite made 145 attempts to get around
  BOGO from inside the sandbox; all were stopped.
  Record: [sandbox and real upstreams](verification/sandbox-e2e.md).
- **The proofs.** The Lean decision core and its proofs build, and
  `#print axioms` reports only the pinned axioms.
  See [What is proved / What is assumed](proved-and-assumed.md).
- **The relay is deployed.** `bogo-relay` runs at `bogo.eilmer.si:8443`, and
  two Mac clients exchanged encrypted messages through it in both directions.
  Record: [relay deployment](../deploy/relay/DEPLOYMENT.md).
- **The BOGO tab in the iOS Simulator.** Registration, the Review card, All
  details, slide to approve and Face ID work against the app's built-in
  stand-in Mac, using a simulator-only key. The screenshots in the
  [README](../README.md#see-it-work) come from that app.

## Not verified yet

- **A real iPhone.** No session on a physical iPhone has been recorded: Face
  ID on hardware, a Secure Enclave key, App Attest on hardware and push
  delivery are all unverified.
- **The phone and the Mac through the public relay.** The deployed relay has
  been used by Mac clients only, not by the iPhone app.
- **Link payments.** The Link login never completed, so the payment that BOGO
  forwarded came back "Not authenticated". That is not evidence of a
  successful payment or of output filtering on a real Link response.
  Record: [sandbox and real upstreams](verification/sandbox-e2e.md).
- **Install without building.** There are no release binaries for the Mac.
  The iPhone app has a [public TestFlight link](https://testflight.apple.com/join/dQu75bv1),
  but its first build has not passed Apple's Beta App Review yet, so the link
  admits no testers until it does.
- **Other MCP clients.** Only Claude Code has been run against BOGO. The
  [other client configs](agents.md) follow the documented protocol but have
  not been tried.

## Known issues

- `bogo request list` prints request IDs and request bytes in hex, not a
  decoded view. Use `bogo log search TOOL` to read a request's agent, tool and
  arguments.
- The [quick start record](verification/user-docs.md) also lists a reported
  path-validation bug found while writing the docs.
