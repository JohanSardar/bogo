# Close round two's daemon and CLI gaps

## Goal
Fix the daemon and CLI gaps that the comparison after round one found. They are in `docs/research/README.md`, section "After round one": C4, C5 and C6. A newcomer hits them as soon as something goes wrong or they look closely. BOGO is a security tool, so it has to be easy to diagnose, behave as its architecture says, and show no rough edges.

When this is done, each of these is true on its own:
- **C4:** when a relay fetch, a push or an upstream fails, the daemon leaves a trace the owner can read, and the docs say where to find it. No secret, token, key, argument, or anything else the log doesn't record today ever appears in it.
- **C5:** connected agents are told when an upstream's tool list changes, as ARCHITECTURE §3.6 says.
- **C6:** each of these leftovers is gone:
  - the out-of-date `deploy/macos/si.eilmer.bogo.plist`;
  - the `/Users/...` path in the Lean vector generator and the vector files, in both repos, which must stay identical;
  - Rust debug output from `bogo log verify`;
  - the jargon `SUN_LEN` error for a long `--home` path, which now says in plain words what to do.

A diagnostic log that includes request arguments or secrets doesn't count. The owner's privacy rule wins over more detail.

## Context
- The repo is `/Users/johansardar/.t3/projects/bogo`; make your own worktree and branch from `main`.
- The vector copy lives in the iPhone repo `/Users/johansardar/apple-bridge-iphone` (its own worktree, from `main`).
- A previous review found that letting the daemon log upstream stderr exposed things it never logged before. A test, `daemon_does_not_log_upstream_stderr`, guards that and must keep passing.
- **Other tickets running at the same time:**
  - T31 owns the relay, pairing, pushes and the "phone cancelled" message.
  - T32 owns the release workflow and the README install section.
  - T28 owns the other docs.

  Stay out of their files where you can.

## Constraints
- Don't run `bogo start` or `bogo stop`, and don't load or unload the `si.eilmer.bogo` LaunchAgent. It is one label per Mac account, so doing so stops the owner's live daemon at `~/.bogo`. Use `serve` in the foreground, on its own home and port.
- Don't change what is signed or how the Lean core decides. The vectors may change only in the example path text, and only identically in both repos.
- Open one PR against `main` in each repo you change, and don't merge them. Push over SSH (`git@github.com:JohanSardar/<repo>.git`). Commits use the owner's name and noreply email, and end with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.

## Guardrails
- The approval flow, fail-closed behaviour and the README quick start's results are unchanged.
- The Lean proofs build, and `#print axioms` stays as pinned. The vectors match between the Lean generator, bogo and the iPhone repo.
- Nothing new reaches the logs beyond the failure traces this ticket asks for, and those carry no secrets or arguments.

## How to verify
1. Make a relay fetch fail and an upstream fail on a foreground daemon. The owner can read each failure where the docs say, and a search of that output for the agent token, keys and the call's arguments finds nothing.
2. Change an upstream's tool list while an agent is connected. The agent is told, and its next tool list shows the change.
3. Each C6 item: show before and after.
4. `make check` passes, the iPhone repo's `make test` passes with the new vectors, and CI passes.

## Evidence of done
When you are done, use what you changed the way a user would: run the real app and go through each screen, flow, or caller that runs the code you changed. If it has no user, use it the way its real client would, such as a program calling the API or an agent calling the tool. Check that it works and that it does everything this ticket asks. Go through every step in How to verify and check every guardrail. Skip parts your change can't reach. Match the proof to the change: a text or style change needs one look at the screen it changes, and a new flow needs one walk through it. When the change touches no code that runs for a user or client, such as docs, comments, tests, or CI config, don't run the app: lint, types, and the existing checks are enough. Follow the `agent-testing` skill for all testing. If you changed code, run the `sol-loop` skill on it and give Sol this ticket. In your report, show what you saw at each step: screenshots, a video, or the output. Then list anything you did not check.
