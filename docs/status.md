# What is verified

BOGO is early. This page says what has been run and recorded, and what hasn't,
so you can decide how far to trust it. Each line links to the record. When a
record and this page disagree, the record wins. Last checked 2026-11-17, at
release 0.1.0.

**In short:** the Mac side, the one-line install and `bogo setup` work end to
end with a software test approver. The iPhone app works in the Simulator. No
session on a real iPhone has been recorded, and the TestFlight link doesn't
admit testers yet.

## Verified

- **The Mac side, end to end, with a software test approver.** The daemon, the
  `bogo` CLI and the shim ran with a real filesystem MCP server and real Claude
  Code. A protected write waited, was approved, and ran once; `bogo log verify`
  passed. The [build-from-source demo](build-from-source.md) is that run.
  Records: [quick start](internal/verification/user-docs.md),
  [daemon and CLI](internal/verification/daemon-cli.md),
  [MCP front and shim](internal/verification/front-shim.md).
- **The one-line install.** Release 0.1.0 was built by CI and published on
  GitHub. `curl -fsSL https://bogo.eilmer.si:8443/install.sh | sh`, run with a
  scratch home and install folder, installed `bogo 0.1.0 (commit
  7ee1e72b3f92)` with no Gatekeeper prompt, and the release build refuses the
  software test approver. A corrupted download installs nothing.
  Records: [install pull request](https://github.com/JohanSardar/bogo/pull/12),
  and the [0.1.0 install run](https://github.com/JohanSardar/bogo/pull/15).
- **`bogo setup` and `bogo doctor`, with a software test approver.** In a
  scratch home, with its own LaunchAgent name, setup created the home, started
  BOGO in the background, added the software test approver in place of the
  phone, moved a Claude Code config's filesystem server behind BOGO, and made
  the protected test call, which waited until it was approved. `bogo doctor`
  then reported every check ok. `bogo setup --undo` restored the client file
  byte for byte. Through the client's new `bogo` entry, reads ran and a write
  waited for approval. The same run was repeated at 0.1.0.
  Records: [setup pull request](https://github.com/JohanSardar/bogo/pull/14),
  [0.1.0 run](https://github.com/JohanSardar/bogo/pull/15).
- **The Gmail and files recipes, without a Google account.** Each installed
  its pinned server, listed its tools and wrote its policy. Through BOGO, a
  Gmail search ran without approval and a send waited and was denied.
  Record: [setup pull request](https://github.com/JohanSardar/bogo/pull/14).
- **The Mac's iPhone route, with a software phone.** The daemon's real iPhone
  route, registration, encrypted envelopes, a local relay and outcome notices
  worked with `bogo-simphone` standing in for the phone.
  Record: [daemon and CLI §4](internal/verification/daemon-cli.md#4-thresholds-and-iphone-route).
- **The reference sandbox.** The bypass suite made 145 attempts to get around
  BOGO from inside the sandbox; all were stopped.
  Record: [sandbox and real upstreams](internal/verification/sandbox-e2e.md).
- **The proofs.** The Lean decision core and its proofs build, and
  `#print axioms` reports only the pinned axioms.
  See [What is proved / What is assumed](proved-and-assumed.md).
- **The relay is deployed.** `bogo-relay` runs at `bogo.eilmer.si:8443`, and
  two Mac clients exchanged encrypted messages through it in both directions.
  Its `/healthz` reports a push key, and `/install.sh` forwards to the install
  script. Record: [relay deployment](../deploy/relay/DEPLOYMENT.md).
- **Pushes from the relay, with a stand-in for Apple.** A local relay with a
  stand-in for Apple's push service sent exactly one push per request, saying
  only "Approval requested". Pairing from another network with a code worked
  with the Simulator, with matching and mismatched digits.
  Record: [relay push pull request](https://github.com/JohanSardar/bogo/pull/13).
- **The BOGO tab in the iOS Simulator.** Registration, the Review card, All
  details, slide to approve and Face ID work against the app's built-in
  stand-in Mac, using a simulator-only key. The screenshots in the
  [README](../README.md#see-it-work) come from that app.

## Not verified yet

- **A real iPhone.** No session on a physical iPhone has been recorded: Face
  ID on hardware, a Secure Enclave key, App Attest on hardware and push
  delivery are all unverified.
- **The phone and the Mac through the public relay.** A Simulator build met a
  test Mac daemon through the public relay: Bonjour found the Mac, both
  screens showed the same 6 digits, and Face ID passed. Then the phone failed
  to save its keys, which the app has since fixed. A full pairing and approval
  through the public relay hasn't been recorded since.
  Record: [research, after round one](internal/research/README.md#after-round-one-2026-11-17).
- **A real push.** The public relay has a push key, but no push has been seen
  arriving on a real iPhone.
- **Link payments.** The Link login never completed, so the payment that BOGO
  forwarded came back "Not authenticated". That is not evidence of a
  successful payment or of output filtering on a real Link response.
  Record: [sandbox and real upstreams](internal/verification/sandbox-e2e.md).
- **Installing the iPhone app without building it.** The app has a
  [public TestFlight link](https://testflight.apple.com/join/dQu75bv1), but it
  doesn't admit testers yet: the first build hasn't been submitted to Apple's
  Beta App Review. Until it passes, build the app from
  [its repo](https://github.com/JohanSardar/apple-bridge-iphone#build-and-check).
- **`bogo setup` with a real phone and real clients.** Setup's phone step with
  an iPhone, and its agent step against real Claude Code, Claude Desktop and
  Cursor configs, haven't been recorded. Its runs used a software test
  approver and test configs.
- **A real Google sign-in, and the GitHub, Slack and Stripe recipes.** No
  recipe has been run against a real account.
- **Other MCP clients.** Only Claude Code has been run against BOGO. The
  [other client configs](agents.md) follow the documented protocol but have
  not been tried.

## Known issues

- `bogo request list` prints request IDs and request bytes in hex, not a
  decoded view. Use `bogo log search TOOL` to read a request's agent, tool and
  arguments.
- The [quick start record](internal/verification/user-docs.md) also lists a reported
  path-validation bug found while writing the docs.
