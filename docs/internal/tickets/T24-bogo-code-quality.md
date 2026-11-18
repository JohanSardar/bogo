# Bring the bogo daemon, CLI and MCP code up to the standard of the best projects like it

## Goal
Make the code in `JohanSardar/bogo` as well kept as the best comparable projects (Happy, ntfy, Secretive, Paseo), without changing what it does. The repo just went public. Today nothing checks a pull request, so a contributor can break the build or the proofs without anyone noticing. The research in `docs/research/` (start with its `README.md`) lists the gaps. This ticket owns the code gaps: CI and its badge (5), dependency updates and advisory checks (13), test files named after review rounds and no single command for every check (14), split logging (15), and dead code.

When this is done, each of these is true on its own:
- **Every pull request runs the checks on GitHub's free hosted runners**: the Rust build, format, lint and tests, and the Lean proof build with the pinned `#print axioms`. A failing check shows on the PR.
- **One documented command runs every check locally**, the same ones CI runs.
- **Dependencies get update PRs**, and known advisories fail a check.
- **Dead code is removed**, and test files and modules have names that say what they test.
- **Logging goes one way**, and the structure is clearer where it was confusing.
- **Behaviour is unchanged.** The daemon, CLI, MCP front and shim do exactly what they do now.

A CI that is green because it skips the proofs or the slow tests does not count.

## Context
- The repo is `/Users/johansardar/.t3/projects/bogo`. Make your own worktree and branch from `main`.
- **Files other tickets own, running at the same time:**
  - T21 is changing the relay: `crates/bogo-relay*`, `deploy/relay/`, `docs/RELAY-PROTOCOL.md` and how the daemon picks its relay. Don't edit those files; leave relay dead code to T21.
  - T23 owns the README, community files and `docs/`. The CI badge belongs in the README, so tell T23's coordinator in your PR which badge URL to add rather than editing the README.
- The research lists what we already do better: the Lean-proved core, the threat model and ADRs, the verification records, and pinned dependencies with written reasons. Keep all of them.

## Constraints
- Use GitHub-hosted runners only. Never register this Mac or any of the owner's machines as a runner.
- If the GitHub token can't push workflow files, push over SSH (`git@github.com:JohanSardar/bogo.git`).
- Publish nothing beyond the GitHub repo: no crates, tags, releases or images.
- Spend nothing: free runner minutes only.
- Don't stop or change the owner's live daemon at `~/.bogo` or the live relay.
- Open a PR against `main`, and don't merge it. Commits use the owner's name and noreply email, and end with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.

## Guardrails
- Behaviour is unchanged: the README quick start gives the same results before and after, step for step.
- The Lean proofs build, `#print axioms` stays as pinned, and the signed request bytes match the existing vectors.
- Fail closed: deny, expiry, single use, the flood limit and "unknown means never retry" behave as now.
- No telemetry is added, and nothing new reaches the logs. In particular, no secret or argument that isn't logged today.

## How to verify
1. Open a PR and watch the checks run on GitHub's runners and pass. Push a deliberately broken commit to a scratch branch (a format error, and a failing proof) and show the checks fail, then remove it.
2. Run the one local command. It runs the same checks and passes.
3. Run the README quick start before and after, on the same steps. The outputs match.
4. `cargo test` passes, and the proof build shows the same pinned axioms.
5. Show what was removed as dead, and why each piece was unused.

## Evidence of done
When you are done, use what you changed the way a user would: run the real app and go through each screen, flow, or caller that runs the code you changed. If it has no user, use it the way its real client would, such as a program calling the API or an agent calling the tool. Check that it works and that it does everything this ticket asks. Go through every step in How to verify and check every guardrail. Skip parts your change can't reach. Match the proof to the change: a text or style change needs one look at the screen it changes, and a new flow needs one walk through it. When the change touches no code that runs for a user or client, such as docs, comments, tests, or CI config, don't run the app: lint, types, and the existing checks are enough. Follow the `agent-testing` skill for all testing. If you changed code, run the `sol-loop` skill on it and give Sol this ticket. In your report, show what you saw at each step: screenshots, a video, or the output. Then list anything you did not check.
