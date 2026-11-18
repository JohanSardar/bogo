# Make BOGO's relay work like croc and Happy

## Goal
Make the relay between the BOGO Mac daemon and the iPhone approver work the way croc (github.com/schollz/croc) and Happy (github.com/slopus/happy) do. Study both closely first, and record what you took from each in the PR. The owner is making both repos public and wants anyone to be able to use BOGO: with nothing to set up by default, and with their own relay if they prefer.

When this is done, each of these is true on its own:
- **Public default.** A fresh install pairs and sends approvals through a free public relay with no relay setting at all.
- **Self-hosting.** Anyone can run their own relay with one command, on a Mac, a VPS or a cloud service, and point BOGO at it with one setting. The docs show each case, and the command works when copied.
- **Encrypted.** The relay only ever sees encrypted bytes: never the request, the arguments, the decision or a key.
- **No mix-ups.** Two pairings started at the same moment never get mixed up.
- **No impostors.** Nobody without the pairing code or key can pose as the Mac or the phone.
- **Rooms safe.** A stranger can't read, fill or take over anyone else's mailbox or room.
- **Unused code goes.** Relay code, config and docs that are dead, duplicated or misleading are removed, in both repos.
- **Nearby wins.** Connecting on the same Wi-Fi or nearby keeps working exactly as it does now. If that path and the relay pull against each other, nearby wins.

A relay that is encrypted but needs setting up before first use doesn't count as a public default. A "one command" that needs extra manual steps first doesn't count either.

## Context
- The relay is in the BOGO repo `/Users/johansardar/.t3/projects/bogo`: the relay crates, `docs/RELAY-PROTOCOL.md`, ADR `docs/adr/0003-bogo-owns-its-relay-and-push.md` and `deploy/relay/`.
- The iPhone side is in `/Users/johansardar/apple-bridge-iphone` under `ios/BogoKit/BogoRelay/`.
- Both repos are public on GitHub. Make your own worktree and branch from `main` in each.
- A relay is live at `https://bogo.eilmer.si:8443` on the Google Cloud VM `ab-relay`. Apple Bridge's own relay runs on the same VM, port 443. The owner's real daemon at `~/.bogo` uses the live relay.
- The owner decided to keep BOGO's iPhone code inside the Apple Bridge iPhone app. There is no standalone app.
- Another ticket is writing the TestFlight release steps in the iPhone repo at the same time. Don't edit `packaging/release/`.

## Constraints
- Don't redeploy or restart the live relay, change the VM, or change any DNS. Prepare what a redeploy needs and write it in your report; the owner approves the redeploy separately. Test against relays you run locally.
- Don't stop or change the owner's live daemon or `~/.bogo`.
- Publish nothing beyond the GitHub repos: no Docker image, package, tag or release. A Dockerfile or script in the repo is fine.
- Spend nothing, and never print or commit a key, token or password.
- Display and transport only on the phone side. Don't change what is signed or how the Lean core decides.
- Don't touch the Apple Bridge Mac repo or the real iPhone.
- Open one PR against `main` in each repo you change, and don't merge them. Say in each PR how they depend on each other. Commits use the owner's name and noreply email, and end with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.

## Guardrails
- The approval flow works exactly as now: an agent's call, the phone's Review card, slide, Face ID, a signed approval, and then that one exact call runs.
- The Lean proofs build, `#print axioms` stays as pinned, and the signed request bytes match the existing vectors.
- Fail closed: deny, expiry, single use, the flood limit and "unknown means never retry" behave as now. An unreachable relay never leads to a call running.
- Nearby and local pairing work exactly as now.
- Setup doesn't get longer than it is now, for the daemon or the phone.
- No telemetry is added, and nothing new reaches the relay or the logs.
- Apple Bridge's own features in the iPhone app work as now.

## How to verify
1. **Public default.** With no relay setting, a fresh daemon home pairs with the iPhone app in the Simulator, and an approval goes end to end. Use a local stand-in for the public relay; the live one stays untouched.
2. **Self-hosting.** Copy the self-host command from the docs onto a clean machine or container, set the one setting, and pair and approve through it.
3. **Encrypted.** Capture what the relay stores and forwards during a real pairing and approval. None of it is plaintext request content or key material.
4. **No mix-ups.** Start two pairings at the same moment against one relay. Each phone ends up paired with its own Mac only.
5. **No impostors and rooms safe.** As an attacker, with only the relay's address and whatever a passive observer sees, try to pair, pose as either side, read a mailbox, write into one and take a room over. Every attempt fails.
6. **Nearby.** Pair and approve on the nearby or local path, before and after your change, on the same steps. It behaves the same.
7. **Before and after.** Setup steps for the daemon and the phone, counted before and after, are no longer.
8. `cargo test`, the Lean proof build, and `make lint` and `make test` in the iPhone repo pass.

## Evidence of done
When you are done, use what you changed the way a user would: run the real app and go through each screen, flow, or caller that runs the code you changed. If it has no user, use it the way its real client would, such as a program calling the API or an agent calling the tool. Check that it works and that it does everything this ticket asks. Go through every step in How to verify and check every guardrail. Skip parts your change can't reach. Match the proof to the change: a text or style change needs one look at the screen it changes, and a new flow needs one walk through it. When the change touches no code that runs for a user or client, such as docs, comments, tests, or CI config, don't run the app: lint, types, and the existing checks are enough. Follow the `agent-testing` skill for all testing. If you changed code, run the `sol-loop` skill on it and give Sol this ticket. In your report, show what you saw at each step: screenshots, a video, or the output. Then list anything you did not check.
