# Bring the iPhone app repo up to the standard of the best projects like it

## Goal
Make `JohanSardar/apple-bridge-iphone` as easy to build and as well checked as the best comparable iOS repos (Secretive, Bark, ntfy's iOS app, Happy), without changing what the app does. The repo was just made public, and it holds BOGO's approver. Today a newcomer can't build it from the README: it is written for the owner's own machines ("the Air", `~/bin/ab-local`) and needs the owner's team ID edited in. Nothing checks a pull request, and `make test` checks no BOGO code. The research in the bogo repo's `docs/research/` (start with its `README.md`) lists the gaps. This ticket owns the iPhone gaps: CI and its badge (5), the newcomer README for building (the code half of 8), `make test` covering BOGO (10), a versioning rule (11) and building without editing the team ID (12).

When this is done, each of these is true on its own:
- **A newcomer with a Mac and Xcode** can clone the repo, follow `ios/README.md` (or a build doc it links), and build and run the app in the Simulator. They never edit a tracked file or need the owner's machines or team.
- **Every pull request builds the app and runs the checks** on GitHub's free hosted runners, and a failure shows on the PR.
- **`make test` exercises BOGO's own code** (the review model, request decoding and the relay client's logic), not only Apple Bridge's.
- **A written versioning rule** says how version and build numbers are chosen.
- **Nothing in the repo refers to the owner's private machines, scripts or paths.** PRINCIPLES.md cites no file the repo doesn't have.
- **Behaviour is unchanged.** The app, its BOGO section and the Apple Bridge features do exactly what they do now, and signed release builds still sign the same way.

A CI that only lints, without building the app, does not count.

## Context
- The repo is `/Users/johansardar/apple-bridge-iphone`. Make your own worktree and branch from `main`; don't use the main checkout.
- The owner decided to keep BOGO's iPhone code in this app. There is no standalone app.
- **Files other tickets own, running at the same time:**
  - T20 owns `packaging/release/` and the written release steps. Don't edit them; link to them.
  - T21 is changing `ios/BogoKit/BogoRelay/`. Don't edit it; you may add tests that call it.
  - T23 owns the top-level README and the community files. The CI badge belongs in the top-level README, so say in your PR which badge URL to add rather than editing the README.
- Drive the Simulator with the T3 device tools (load them with ToolSearch `select:mcp__t3-code__device_list,mcp__t3-code__device_open,mcp__t3-code__device_screenshot,mcp__t3-code__device_close`). If a Simulator is in use by another session, boot a different one.

## Constraints
- Use GitHub-hosted runners only. Never register this Mac or any of the owner's machines as a runner.
- If the GitHub token can't push workflow files, push over SSH (`git@github.com:JohanSardar/apple-bridge-iphone.git`).
- Don't change bundle IDs, entitlements or the release signing settings. A local override for development signing is fine.
- No new third-party packages. Publish nothing beyond the GitHub repo. Spend nothing.
- Don't touch the BOGO Rust repo, the Apple Bridge Mac repo or the real iPhone.
- Open a PR against `main`, and don't merge it. Commits use the owner's name and noreply email, and end with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.

## Guardrails
- The app's features (Apple Bridge and the BOGO section) work exactly as now. The approval flow is the Review card, slide, then Face ID.
- Signed release builds (`packaging/release/testflight.py`) still produce the same signed app.
- App size and launch time don't get noticeably worse. The owner treats this limit as soft.

## How to verify
1. **As a newcomer:** make a fresh clone in a new directory, with none of the owner's environment variables, scripts or profiles. Follow the build doc to a running app in the Simulator.
2. **CI:** open a PR and show the checks build the app and pass on GitHub's runners. Push a deliberately broken commit to a scratch branch and show the checks fail, then remove it.
3. **Tests:** `make test` runs BOGO code. Show a BOGO test failing when you break that code, then restore it.
4. **Before and after:** on `main` and on your branch, run the BOGO demo flow in the Simulator (Settings → Demo, Review card, slide, Face ID, success). It behaves the same.
5. A search of the repo finds no owner-specific machine names, private script paths or missing PRINCIPLES.md references.
6. `make lint` and `make test` pass.

## Evidence of done
When you are done, use what you changed the way a user would: run the real app and go through each screen, flow, or caller that runs the code you changed. If it has no user, use it the way its real client would, such as a program calling the API or an agent calling the tool. Check that it works and that it does everything this ticket asks. Go through every step in How to verify and check every guardrail. Skip parts your change can't reach. Match the proof to the change: a text or style change needs one look at the screen it changes, and a new flow needs one walk through it. When the change touches no code that runs for a user or client, such as docs, comments, tests, or CI config, don't run the app: lint, types, and the existing checks are enough. Follow the `agent-testing` skill for all testing. If you changed code, run the `sol-loop` skill on it and give Sol this ticket. In your report, show what you saw at each step: screenshots, a video, or the output. Then list anything you did not check.
