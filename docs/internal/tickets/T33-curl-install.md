# Install BOGO on a Mac with one curl command

## Goal
Make BOGO's Mac side installable with one command and nothing to build. Today the only way is from source, which needs Rust, Lean, Node and Python, and the projects we compare with install in one step. Happy uses `npm i -g`, ntfy and Secretive use `brew install`, and many CLI tools use `curl … | sh`. On 2026-11-17 the owner chose a curl script, with no Homebrew, npm or Docker. This ticket replaces T32.

```sh
curl -fsSL https://raw.githubusercontent.com/JohanSardar/bogo/main/install.sh | sh
```

When this is done, each of these is true on its own:
- **CI builds a release when a version tag is pushed.** On GitHub's free hosted runners, it builds production binaries for Apple Silicon Macs: `bogo`, `bogo-daemon` if it is separate, and `bogo-shim`, with no test features. It attaches them to a GitHub release as one archive, with a SHA-256 checksum file and that version's changelog entry.
- **`install.sh` is at the repo root.** It's short and readable, and it:
  - downloads the latest release (or the one named in `BOGO_VERSION`);
  - checks the checksum and stops on a mismatch;
  - installs into `~/.local/bin`, or `BOGO_INSTALL_DIR`, with no sudo;
  - says how to add that directory to `PATH` if it isn't there;
  - ends by printing the one next step: `bogo setup`.

  It changes nothing outside the install directory, and running it again upgrades in place.
- **It runs on a fresh Mac with no warning.**
  - Find out and show whether a binary downloaded by curl runs without a Gatekeeper prompt. curl doesn't add the quarantine flag that browsers add.
  - Find out whether Apple Silicon's required ad-hoc signature is present.
  - If something does need signing, say so in the PR and stop there. Don't add the owner's Developer ID or any key to CI; that is the owner's decision.
- **The relay’s `/install.sh` route forwards to GitHub’s raw `install.sh` on `main`** with an HTTP redirect, so the script and binaries live only on GitHub. T35 adds the route to the relay and the coordinator deploys it. The install command uses GitHub directly. The optional relay shortcut is described in [setup](../../setup.md#install).
- **A written release rule** says how versions are chosen and how to cut a release, matching the iPhone repo's rule in its `ios/README.md`.
- **`bogo --version`** prints the version and the commit it was built from.

An install line that fails when copied doesn't count, and neither does a binary that won't run on a fresh Mac.

## Context
- The repo is `/Users/johansardar/.t3/projects/bogo`. Make your own worktree and branch from `main`.
- The coordinator pushes the first real tag after merging, so don't push a version tag to the real repo. To prove the workflow, push a scratch pre-release tag such as `v0.0.0-test.1`, then delete it and its release afterwards.
- Until T35's redirect is deployed, test the script by running it straight from your branch's raw URL, or from a local file.
- **Other tickets running at the same time:**
  - T34 adds `bogo setup`, `bogo doctor`, recipes and daemon fixes;
  - T35 owns the relay, pairing, pushes and the iPhone app;
  - T36 (later) rewrites the README top and docs.

  Touch only the README's install lines, and stay out of the other tickets' files.

## Constraints
- Use GitHub-hosted runners only. Never register any of the owner's machines as a runner.
- Don't run `bogo start` or `bogo stop`, and don't load or unload the `si.eilmer.bogo` LaunchAgent. It is one label per Mac account, so doing so stops the owner's live daemon at `~/.bogo`. Use `serve` in the foreground, on its own home and port.
- Don't overwrite `~/.local/bin/bogo`, which is the owner's live binary. Test installs with `BOGO_INSTALL_DIR` set to a scratch directory.
- Never print or commit a key, token or password. Spend nothing. Publish nothing beyond the GitHub repo.
- Open a PR against `main`, and don't merge it. Push over SSH (`git@github.com:JohanSardar/bogo.git`). Commits use the owner's name and noreply email, and end with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.

## Guardrails
- Behaviour is unchanged: the released binaries do what a source build does.
- The Lean proofs build, and `#print axioms` stays as pinned, in the release build too.
- The software test approver is never in a release binary.
- The script never needs sudo, and never runs anything it downloaded other than the binary, after the checksum passes.

## How to verify
1. Push a scratch test tag. CI publishes a pre-release with the archive and checksums. Delete both afterwards.
2. With no Rust, Lean or Node on `PATH`, use a fresh scratch `HOME` and install dir. Run the one-line install against that pre-release, then `bogo --version`, `bogo init` and `bogo status`, using a scratch `--home`. Show any Gatekeeper prompt, or that there was none.
3. Change one byte of the archive. The script refuses it.
4. The release binary refuses the software test approver.
5. `make check` passes, and CI passes on the PR.

## Evidence of done
When you are done, use what you changed the way a user would: run the real app and go through each screen, flow, or caller that runs the code you changed. If it has no user, use it the way its real client would, such as a program calling the API or an agent calling the tool. Check that it works and that it does everything this ticket asks. Go through every step in How to verify and check every guardrail. Skip parts your change can't reach. Match the proof to the change: a text or style change needs one look at the screen it changes, and a new flow needs one walk through it. When the change touches no code that runs for a user or client, such as docs, comments, tests, or CI config, don't run the app: lint, types, and the existing checks are enough. Follow the `agent-testing` skill for all testing. If you changed code, run the `sol-loop` skill on it and give Sol this ticket. In your report, show what you saw at each step: screenshots, a video, or the output. Then list anything you did not check.
