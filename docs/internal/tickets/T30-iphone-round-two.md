# Let a Simulator build pair with a real Mac, and clean up the iPhone build

## Goal
Fix the iPhone gaps that the comparison after round one found (`docs/research/README.md` in the bogo repo, section "After round one"): I1 and I4.

Today a newcomer who builds the app with `make simulator`, as the README says, can't pair it with a real BOGO Mac. The unsigned build can't save its channel keys to the Keychain (error `-34018`), so pairing fails right after Face ID. That's the first thing someone trying BOGO without TestFlight would do. And `make simulator` and `make test` print compiler warnings, which the comparable repos don't.

When this is done, each of these is true on its own:
- **A newcomer's Simulator build pairs with a real Mac.** The build made by the README's steps pairs with a BOGO Mac daemon through the relay and approves a call, with no Apple account or team.
- **The builds print no warnings.** `make simulator` and `make test` run clean, and CI keeps them that way.
- **Behaviour is unchanged.** Signed and TestFlight builds sign exactly as now.

## Context
- The repo is `/Users/johansardar/apple-bridge-iphone`. Make your own worktree and branch from `main`, and don't use its main checkout.
- A BOGO Mac daemon for testing: build it from `/Users/johansardar/.t3/projects/bogo` `main` in your own worktree, and run `serve` on its own home and port.
- Drive the Simulator with the T3 device tools (load them with ToolSearch `select:mcp__t3-code__device_list,mcp__t3-code__device_open,mcp__t3-code__device_screenshot,mcp__t3-code__device_close`). Boot a new Simulator: other sessions hold the existing ones. Delete it when you're done.
- **Other tickets running at the same time:**
  - T31 changes the iPhone pairing and push code, including Add Mac.
  - T29 changes the test vectors' example path.
  - T28 changes `ios/README.md`'s owner notes.

  Stay out of those where you can.

## Constraints
- Don't run `bogo start` or `bogo stop`, and don't load or unload the `si.eilmer.bogo` LaunchAgent. It is one label per Mac account, so doing so stops the owner's live daemon at `~/.bogo`.
- Don't change bundle IDs, release entitlements or release signing.
- No new third-party packages. Don't touch the real iPhone or the Apple Bridge Mac repo.
- Open a PR against `main`, and don't merge it. Push over SSH (`git@github.com:JohanSardar/apple-bridge-iphone.git`). Commits use the owner's name and noreply email, and end with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.

## Guardrails
- The app's features (Apple Bridge and the BOGO section) work as now. The approval flow is the Review card, slide, then Face ID.
- Release and TestFlight builds produce the same signed app: the build settings for release are unchanged.

## How to verify
1. In a fresh clone with none of the owner's environment, follow the README to a Simulator build. Pair it with a foreground BOGO daemon through a local relay using the 6 digits, then approve an agent's call end to end.
2. `make simulator` and `make test` show no warnings, and CI passes.
3. The release build settings are identical to `main`'s.

## Evidence of done
When you are done, use what you changed the way a user would: run the real app and go through each screen, flow, or caller that runs the code you changed. If it has no user, use it the way its real client would, such as a program calling the API or an agent calling the tool. Check that it works and that it does everything this ticket asks. Go through every step in How to verify and check every guardrail. Skip parts your change can't reach. Match the proof to the change: a text or style change needs one look at the screen it changes, and a new flow needs one walk through it. When the change touches no code that runs for a user or client, such as docs, comments, tests, or CI config, don't run the app: lint, types, and the existing checks are enough. Follow the `agent-testing` skill for all testing. If you changed code, run the `sol-loop` skill on it and give Sol this ticket. In your report, show what you saw at each step: screenshots, a video, or the output. Then list anything you did not check.
