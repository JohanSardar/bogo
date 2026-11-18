# Keep 6-digit pairing on the new relay

## Goal
Bring back the owner's 6-digit pairing on top of the new relay work in bogo PR #10 and apple-bridge-iphone PR #10. That work replaced it with a QR code and a typed code. The owner reviewed the change on 2026-11-17 and chose to keep 6 digits. Their SPEC already said why: a QR code feels less clean than matching 6 digits, like pairing a Bluetooth device.

When this is done, adding a Mac works like this, with nothing to scan or type:
1. On the Mac, the owner runs `bogo approver add "<name>"`.
2. On the phone, they open BOGO → Macs → **Add Mac** and tap their Mac in the list.
3. Both screens show the same 6 digits.
4. The owner confirms on the Mac, then confirms on the phone with Face ID, which creates the phone's approval key.

Everything else the relay work delivered stays true, each on its own:
- **Public default:** pairing and approvals use the free public relay with no setting.
- **Self-hosting:** one command, plus one setting on the Mac. The phone still needs no setting when the Mac uses its own relay.
- **Encrypted:** the relay sees only encrypted bytes.
- **No mix-ups:** two pairings at the same moment never get mixed up.
- **No impostors:** nobody can pose as either side without the owner matching the digits and confirming on both devices.
- **Rooms safe:** strangers can't read, fill or take over anyone else's mailbox or room.

A design where the digits are shown but a mismatch or a missing confirmation still pairs doesn't count. Neither does one that brings back a lobby any stranger can read.

Then `docs/SPEC.md` describes the 6-digit flow again, and the docs that mention pairing match it.

## Context
- **Build on the two open PR branches:** `t3/t21-relay-rooms` in `JohanSardar/bogo` (PR #10) and `t3/t21-relay` in `JohanSardar/apple-bridge-iphone` (PR #10). Keep everything else they deliver. Read both PR descriptions first: they explain the room design, what was taken from croc and Happy, the attacks tested and the self-host setup.
- The repos are `/Users/johansardar/.t3/projects/bogo` and `/Users/johansardar/apple-bridge-iphone`. Make your own worktree in each, and don't use the iPhone repo's main checkout.
- **`main` has moved since those PRs:**
  - in bogo: CI, `make check`, the new README and docs index, and a fix to the flaky `running_log_query_survives_one_megabyte` test;
  - in the iPhone repo: CI, `ios/Signing.xcconfig` and the new README.

  Bring both branches up to date with `main`.
- The previous worker noted that the unsigned `make simulator` build can't save channel keys to the Keychain, so relay pairing fails right after Face ID there (on `main` too). It used a build with simulator entitlements embedded.
- **The owner approved redeploying the live relay** at `bogo.eilmer.si:8443` once these PRs merge. The coordinator does that. Don't touch the live relay, the VM or DNS yourself.

## Constraints
- Don't redeploy or restart the live relay, or change the VM or DNS. Test against relays you run on this Mac.
- Don't stop or change the owner's live daemon or `~/.bogo`.
- Publish nothing beyond the GitHub repos. Spend nothing, and never print or commit a key, token or password.
- Don't change what is signed or how the Lean core decides.
- Don't touch the Apple Bridge Mac repo or the real iPhone.
- Push to the two existing PR branches over SSH (`git@github.com:JohanSardar/<repo>.git`) and update both PR descriptions. Don't merge. Commits use the owner's name and noreply email, and end with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.

## Guardrails
- The approval flow works exactly as now: an agent's call, the Review card, slide, Face ID, a signed approval, and then that one exact call runs.
- The Lean proofs build, `#print axioms` stays as pinned, and the signed request bytes match the existing vectors.
- Fail closed: deny, expiry, single use, the flood limit and "unknown means never retry" behave as now. An unreachable relay never leads to a call running.
- Setup is no longer than the 6-digit flow above, on the Mac and on the phone.
- No telemetry is added, and nothing new reaches the relay or the logs.
- Apple Bridge's own features in the iPhone app work as now.

## How to verify
1. **Pairing:** with no relay setting, a fresh daemon home pairs with the iPhone app in the Simulator through a local stand-in for the public relay, using the four steps above. Then an agent's protected call goes end to end through slide and Face ID.
2. **Digits:** a mismatch, or a "no" on either device, leaves nothing paired on either side.
3. **Self-hosting:** run the self-host command from the docs and set the one Mac setting. The phone pairs with the 6 digits with no setting of its own.
4. **No mix-ups:** two Macs and two phones pair at the same moment. Each phone gets its own Mac, and each pair shows its own digits.
5. **Attacks:** as an attacker with the relay's address and what a passive observer sees, try to pose as either side, read or write a mailbox, and take a room over. Every attempt fails or is caught by the digits, and nothing is registered.
6. **Encrypted:** capture what the relay stores and forwards. None of it is plaintext request content, names or key material.
7. **Checks:** `make check` in bogo, and `make lint` and `make test` in the iPhone repo, pass locally and in CI on both PRs.

## Evidence of done
When you are done, use what you changed the way a user would: run the real app and go through each screen, flow, or caller that runs the code you changed. If it has no user, use it the way its real client would, such as a program calling the API or an agent calling the tool. Check that it works and that it does everything this ticket asks. Go through every step in How to verify and check every guardrail. Skip parts your change can't reach. Match the proof to the change: a text or style change needs one look at the screen it changes, and a new flow needs one walk through it. When the change touches no code that runs for a user or client, such as docs, comments, tests, or CI config, don't run the app: lint, types, and the existing checks are enough. Follow the `agent-testing` skill for all testing. If you changed code, run the `sol-loop` skill on it and give Sol this ticket. In your report, show what you saw at each step: screenshots, a video, or the output. Then list anything you did not check.
