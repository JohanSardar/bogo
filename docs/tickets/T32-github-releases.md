# Ship BOGO as a GitHub release people can download

## Goal
Make it possible to install BOGO's Mac side without building it. Today the only way is from source, which needs Rust, Lean, Node and Python, and every project we compare with ships downloads. On 2026-11-17 the owner approved publishing GitHub releases: a version tag plus a prebuilt binary for Apple Silicon Macs, with checksums, made by CI when a tag is pushed.

When this is done, each of these is true on its own:
- **CI builds a release when a version tag is pushed.** On GitHub's free hosted runners, it builds production binaries (no test features) and attaches them to a GitHub release with SHA-256 checksums and the changelog entry for that version.
- **The README's install section** gives a copy-paste install from the release: download, check the checksum, put the binaries on the path. It also says the binaries are not notarized, and exactly what macOS will show and how to allow it. Building from source stays documented for those who prefer it.
- **A written release rule** says how versions are chosen and how to cut a release, matching the iPhone repo's rule.

A release whose binaries don't run on a fresh Mac doesn't count. Neither does an install line that fails when copied.

## Context
- The repo is `/Users/johansardar/.t3/projects/bogo`. Make your own worktree and branch from `main`.
- The owner ruled out a Homebrew tap, npm, Docker images and paid notarization for now.
- The coordinator pushes the first real tag after merging, so don't push a version tag to the real repo. Prove the workflow on a fork-free scratch tag, such as a pre-release named for testing, that you delete afterwards along with its release.
- The iPhone repo's versioning rule is in its `ios/README.md`.
- **Other tickets running at the same time:**
  - T29 owns daemon logging and small CLI fixes.
  - T31 owns the relay and pairing.
  - T28 owns the docs other than the README's install section.

## Constraints
- Use GitHub-hosted runners only. Never register any of the owner's machines as a runner.
- Don't run `bogo start` or `bogo stop`, and don't load or unload the `si.eilmer.bogo` LaunchAgent. It is one label per Mac account, so doing so stops the owner's live daemon at `~/.bogo`. Use `serve` in the foreground, on its own home and port.
- Never print or commit a key, token or password. Spend nothing.
- Open a PR against `main`, and don't merge it. Push over SSH (`git@github.com:JohanSardar/bogo.git`). Commits use the owner's name and noreply email, and end with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.

## Guardrails
- Behaviour is unchanged: the released binaries do what a source build does.
- The Lean proofs build, and `#print axioms` stays as pinned, in the release build too.
- The software test approver is never in a release binary.

## How to verify
1. Push a scratch test tag. CI builds and publishes a pre-release with binaries and checksums. Then delete the tag and the release.
2. On a clean macOS environment with no Rust, Lean or Node, follow the README's install from that pre-release. Then run the quick start's first steps (`init`, `status`) with the downloaded binaries.
3. Show that the release binary refuses the software test approver.
4. `make check` passes, and CI passes on the PR.

## Evidence of done
When you are done, use what you changed the way a user would: run the real app and go through each screen, flow, or caller that runs the code you changed. If it has no user, use it the way its real client would, such as a program calling the API or an agent calling the tool. Check that it works and that it does everything this ticket asks. Go through every step in How to verify and check every guardrail. Skip parts your change can't reach. Match the proof to the change: a text or style change needs one look at the screen it changes, and a new flow needs one walk through it. When the change touches no code that runs for a user or client, such as docs, comments, tests, or CI config, don't run the app: lint, types, and the existing checks are enough. Follow the `agent-testing` skill for all testing. If you changed code, run the `sol-loop` skill on it and give Sol this ticket. In your report, show what you saw at each step: screenshots, a video, or the output. Then list anything you did not check.
