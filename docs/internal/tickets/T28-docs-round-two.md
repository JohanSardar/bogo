# Close round two's docs gaps in both repos

## Goal
Fix the front-page and docs gaps that the comparison after round one found (`docs/research/README.md` in the bogo repo, section "After round one"). They are A2 through A7, and I3. The owner wants both repos to read as one clear story to a newcomer who opens them one after the other.

When this is done, each of these is true on its own:
- **A2:** the FAQ explains why BOGO's phone side lives inside the Apple Bridge app with a real reason, not "the owner chose". The reason is in Context. It also says what that means for someone who only wants BOGO: the app holds other features they can leave off, and BOGO uses none of Apple Bridge's servers.
- **A3:** a newcomer browsing `docs/` meets the user pages first. Internal records (tickets, design rounds, verification logs) are grouped apart, and every existing link to them still works.
- **A4:** the docs index and FAQ link the self-hosting page, not the operator runbook with its VM and IP address.
- **A5:** both changelogs record round one, including the new relay and 6-digit pairing.
- **A6:** the owner's name is gone from examples; use a neutral name.
- **A7:** both repos have a Code of Conduct, a homepage link in the GitHub settings, and a Dependencies section saying what each part depends on and why.
- **I3:** the iPhone repo's app guide and qualification page read as docs for anyone, not as the owner's notes. No provisioning paths, `APP-*` codes, or files like `~/I1-NEEDS-JOHAN.txt`. Where a historical record must stay, it is clearly marked as a record.

## Context
- **Why BOGO is inside Apple Bridge**, from the owner's decision on 2026-11-17: one app keeps a single TestFlight and App Store listing and a single signing and release path. The Review card shares the app's design. BOGO still uses no Apple Bridge server, relay, pairing or key.
- **Other tickets running at the same time** own these files, so stay out of them:
  - T31 writes the "Add your iPhone" guide, changes ADR 0003 and the relay and pairing docs;
  - T32 changes the README's install section and adds the release rule;
  - T29 changes `docs/daemon.md`'s logging section and the macOS plist;
  - T30 changes the iPhone build.
- The repos are `/Users/johansardar/.t3/projects/bogo` and `/Users/johansardar/apple-bridge-iphone`. Make your own worktree and branch from `main` in each, and don't use the iPhone repo's main checkout.

## Constraints
- Docs, community files and GitHub repo settings only. No code changes.
- Don't run `bogo start` or `bogo stop`, and don't load or unload the `si.eilmer.bogo` LaunchAgent. It stops the owner's live daemon.
- Open one PR against `main` in each repo, and don't merge them. Push over SSH (`git@github.com:JohanSardar/<repo>.git`). Commits use the owner's name and noreply email, and end with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.

## Guardrails
- The docs stay honest about what is and isn't verified.
- The threat model, ADRs, proved-and-assumed page and verification records stay reachable from the docs index.
- Every link in both repos still opens, including links into moved internal records.

## How to verify
1. Open both repos on GitHub as a newcomer, one after the other. The story reads as one, and the FAQ answers "why inside Apple Bridge" with the real reason.
2. A link checker over both repos finds no broken link, including the old paths of anything moved.
3. A search of both repos finds no owner-specific note in the user-facing docs, and no owner's name in examples.
4. GitHub shows the Code of Conduct and homepage on both repos.

## Evidence of done
When you are done, use what you changed the way a user would: run the real app and go through each screen, flow, or caller that runs the code you changed. If it has no user, use it the way its real client would, such as a program calling the API or an agent calling the tool. Check that it works and that it does everything this ticket asks. Go through every step in How to verify and check every guardrail. Skip parts your change can't reach. Match the proof to the change: a text or style change needs one look at the screen it changes, and a new flow needs one walk through it. When the change touches no code that runs for a user or client, such as docs, comments, tests, or CI config, don't run the app: lint, types, and the existing checks are enough. Follow the `agent-testing` skill for all testing. If you changed code, run the `sol-loop` skill on it and give Sol this ticket. In your report, show what you saw at each step: screenshots, a video, or the output. Then list anything you did not check.
