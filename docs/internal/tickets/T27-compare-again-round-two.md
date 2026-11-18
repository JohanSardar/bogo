# Compare both repos with the five projects again, after round one

## Goal
Round one of the cleanup has merged into `main` of both repos: front page and docs, CI and code quality in both repos, TestFlight release steps, and the new relay with 6-digit pairing. Compare `JohanSardar/bogo` and `JohanSardar/apple-bridge-iphone` with Happy, ntfy, Bark, Secretive and Paseo again, item by item, the same way `docs/research/` did. Write down what still falls short. The owner wants to keep going until someone who opens one of those repos and then ours would call ours just as clear, polished and well kept. This comparison is what round two's tickets will be cut from.

When this is done, `docs/research/README.md` has a dated "After round one" section. For every gap in the original ranked list, it says whether it is closed, partly closed or still open, with the evidence on `main`. It also lists new gaps that round one exposed or created, ranked by how much they matter to a newcomer, and grouped by who would fix them:
- front page and docs;
- the daemon, CLI and relay code;
- the iPhone app.

Each open gap is concrete enough to become a ticket. Gaps the owner's rules block (publishing releases, binaries, images or tags) are listed separately as needing the owner's decision. A gap counts as closed only when you used it the way a newcomer would and it worked, for example ran the copied command or opened the page. A file merely existing doesn't count.

Also judge these as a newcomer would, reading both repos one after the other:
- Does it make sense to a newcomer that BOGO's phone side lives inside the Apple Bridge app?
- Do the two READMEs read as one story?

## Context
- The original research and ranked gaps are in `docs/research/` on bogo `main`.
- The coordinator already knows of these leftovers. Confirm or refute each one:
  - `Front::catalog_changed` is never called, though ARCHITECTURE says sessions hear when an upstream's tool list changes.
  - `deploy/macos/si.eilmer.bogo.plist` is out of date.
  - The test vectors contain a `/Users/...` path.
  - An unsigned `make simulator` build can't pair over the relay, because it can't save channel keys to the Keychain.
  - Pairing needs the phone on the Mac's network (Bonjour), with no fallback.
  - The TestFlight link admits nobody until a build passes Beta App Review.
- The live public relay at `bogo.eilmer.si:8443` runs the round-one relay. You may use it as any newcomer would, but don't administer it.

## Constraints
- Research and writing only. Change nothing outside `docs/research/`.
- Don't run `bogo start` or `bogo stop`, and don't load or unload the `si.eilmer.bogo` LaunchAgent. It is one label per Mac account, so doing so stops the owner's live daemon at `~/.bogo`. Run any daemon you need with `serve` in the foreground, on its own home and port.
- Open a PR against `main` on `JohanSardar/bogo`, and don't merge it. Commits use the owner's name and noreply email, and end with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.

## How to verify
1. Every gap from the original list has a status and evidence.
2. Every "closed" was checked by using it the way a newcomer would. Say what you ran or opened.
3. Every link you cite opens and shows what you say.

## Evidence of done
This is research, so there is no app to run. Before you report, check every link you cite and every claim about our repos against current `main`. In your report, give the PR link, the ranked open gaps by group, the gaps needing the owner's decision, and anything you could not check.
