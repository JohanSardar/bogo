# Let anyone use BOGO with a real iPhone: the relay sends pushes, and pairing works off the Mac's network

## Goal
Today only the owner can use BOGO with a real iPhone.
- **Pushes:** each Mac's daemon sends its own pushes, so it needs an Apple push key from the team that owns the app's bundle ID. Production config refuses an iPhone approver without one, so nobody else can wake the TestFlight app, even once TestFlight opens.
- **Pairing:** it only works when the phone is on the Mac's network, because the phone finds the Mac with Bonjour.

The owner chose to fix both on 2026-11-17, so that anyone who installs BOGO and joins the TestFlight can approve their agent's calls on their own iPhone. Bark, ntfy and Happy do it the same way: their server sends the push.

When this is done, each of these is true on its own:
- **The public relay wakes the phone.** When a sealed request reaches a phone's mailbox, the relay sends the push with the owner's push key. A Mac using the public relay needs no `[apns]` section, and pairing a real iPhone has no push setup.
  - The push carries nothing about the request: no tool, argument, agent name or amount. It says only that something is waiting.
  - A self-hosted relay can send pushes with its own key. A Mac can still send its own pushes with its own key if it is configured to.
- **Pairing works off the Mac's network.** When the phone can't find the Mac nearby, for example on cellular, guest Wi-Fi or a network that blocks Bonjour, it can pair through the relay. The owner scans or types a short code that `bogo approver add` shows, and then still compares the 6 digits and confirms on both devices. Nearby stays the default: when the Mac is found nearby, the flow is exactly today's tap-and-compare, and nearby wins.
- **The Mac reports what really happened.** It says "the phone cancelled" only when the phone cancelled, and gives the real reason for any other failure, such as the phone failing to save its keys. This is research gap C3.
- **The relay answers a health check** that says it is up, so a self-hoster can see their relay works. This is gap C7.
- **The docs have an "Add your iPhone" guide**: install from TestFlight, then the Mac config, then pairing nearby or off-network, then the first approval, with nothing left out. It is linked from both READMEs. ADR 0003 is updated to record the change and why. This is gap A1.

A push that leaks anything about the request doesn't count. Neither does an off-network path that skips the 6 digits, or one that changes the nearby flow.

## Context
- **The research:** see `docs/research/README.md`, section "After round one" (gaps C2, C3, C7, A1, I2).
- **ADR 0003** (`docs/adr/0003-bogo-owns-its-relay-and-push.md`) records why the Mac sends pushes today and how pairing reached its current form. Read its updates.
- **Pairing history**, as the owner asked to have it explained in the docs:
  - Before T21, waiting Macs were listed in a public lobby on the relay, which anyone could read.
  - T21 removed the lobby for safety and briefly used a code.
  - T26 restored the owner's 6-digit flow, finding the Mac with Bonjour instead.

  The off-network fallback is the code path again, with the digits kept.
- **The app's push topic** is `com.johansardar.applebridge.ios`. The owner's push key is the `apns_p8` secret in the owner's live BOGO home. Never read, copy or print it. The coordinator installs it on the relay VM at deploy time, so tell the coordinator exactly what the relay needs: which file, which settings and which permissions.
- **The live relay** at `bogo.eilmer.si:8443` runs on the VM `ab-relay` next to Apple Bridge's service on port 443. The owner approved redeploying BOGO's relay for this. The coordinator does it; you prepare the steps.
- **TestFlight builds** attest in App Attest's production environment. The setup guide must say what a Mac needs for them.
- **The repos** are `/Users/johansardar/.t3/projects/bogo` and `/Users/johansardar/apple-bridge-iphone`. Make your own worktree and branch from `main` in each.
- **Other tickets running at the same time:**
  - T29 owns daemon logging, `catalog_changed` and small CLI fixes.
  - T30 owns iPhone Simulator signing and compiler warnings.
  - T32 owns the release workflow.
  - T28 owns the other docs.

  Stay out of their files where you can, and say in your PR if you couldn't.

## Constraints
- Don't run `bogo start` or `bogo stop`, and don't load or unload the `si.eilmer.bogo` LaunchAgent. It is one label per Mac account, so doing so stops the owner's live daemon at `~/.bogo`. Run any daemon with `serve` in the foreground, on its own home and port.
- Don't redeploy the live relay, or change the VM or DNS. Test against relays you run on this Mac. For pushes, use a local stand-in for Apple's push service, or Apple's sandbox with a Simulator or test token if you can without the owner's key.
- Never read, print, copy or commit the owner's push key or any secret.
- Don't change what is signed or how the Lean core decides.
- Publish nothing beyond the GitHub repos. Spend nothing.
- Don't touch the Apple Bridge Mac repo or the real iPhone.
- Open one PR against `main` in each repo you change, and don't merge them. Push over SSH (`git@github.com:JohanSardar/<repo>.git`). Commits use the owner's name and noreply email, and end with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.

## Guardrails
- The approval flow works exactly as now: an agent's call, the Review card, slide, Face ID, a signed approval, and then that one exact call runs.
- The Lean proofs build, `#print axioms` stays as pinned, and the signed request bytes match the existing vectors.
- Fail closed: deny, expiry, single use, the flood limit and "unknown means never retry" behave as now. A failed or missing push never leads to a call running.
- Nearby pairing works exactly as now, and wins when the Mac is found nearby.
- Setup gets no longer for anyone. For people without their own push key it gets shorter.
- No telemetry is added. The relay learns nothing new about requests beyond which mailbox has something waiting.
- Phones already paired keep working, and so does the owner's live setup, through the redeploy.
- Apple Bridge's own features in the iPhone app work as now.

## How to verify
1. **Push:** with a Mac that has no `[apns]` section, using a local relay with push turned on, a protected call makes the relay send a push to the paired phone's token. Capture the push: it has no request content.
2. **Off-network pairing:** with Bonjour unavailable to the phone, pair through the relay with the code, compare the 6 digits and confirm on both devices. A mismatch or a "no" registers nothing.
3. **Nearby:** with the Mac found nearby, pairing is exactly today's flow, before and after on the same steps.
4. **Failure messages:** make the phone fail to save its keys. The Mac reports that, not "the phone cancelled".
5. **Health check:** it answers on a local relay.
6. **The guide:** follow "Add your iPhone" from the top, as a newcomer, against a local relay and the Simulator, up to the first approval.
7. **Checks:** `make check` in bogo, and `make lint` and `make test` in the iPhone repo, pass locally and in CI.

## Evidence of done
When you are done, use what you changed the way a user would: run the real app and go through each screen, flow, or caller that runs the code you changed. If it has no user, use it the way its real client would, such as a program calling the API or an agent calling the tool. Check that it works and that it does everything this ticket asks. Go through every step in How to verify and check every guardrail. Skip parts your change can't reach. Match the proof to the change: a text or style change needs one look at the screen it changes, and a new flow needs one walk through it. When the change touches no code that runs for a user or client, such as docs, comments, tests, or CI config, don't run the app: lint, types, and the existing checks are enough. Follow the `agent-testing` skill for all testing. If you changed code, run the `sol-loop` skill on it and give Sol this ticket. In your report, show what you saw at each step: screenshots, a video, or the output. Then list anything you did not check.
