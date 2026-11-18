# Compare BOGO's two repos with the best projects like them

## Goal
Study five open-source projects in depth and write down, item by item, where BOGO's repos fall short of them. The owner wants BOGO's repos to be as clear, polished and well kept as these. Someone who opens one of their repos and then ours should find ours just as good. This write-up is what the next tickets will be cut from, so each gap must be concrete enough to act on.

When this is done, `docs/research/` in the BOGO repo has one file per project plus a summary. For each item below, each file says what we do now, what they do (with links to their actual files), and the gap. "We" means both repos together:
- `JohanSardar/bogo`: the Mac daemon, CLI, MCP front, relay and Lean core.
- `JohanSardar/apple-bridge-iphone`: the iPhone app, whose BOGO section is the approver.

The summary ranks the gaps by how much they matter to a newcomer. It groups them by who would fix them: front page and docs, the daemon/CLI/relay code, and the iPhone app. A claim about another project without a link to the file or page that shows it doesn't count. Neither does a gap that just says "improve X".

## Context
The owner confirmed these projects:
- slopus/happy (github.com/slopus/happy): a phone app for coding agents, with an encrypted relay you can host yourself.
- binwiederhier/ntfy (github.com/binwiederhier/ntfy): an iPhone app plus a server, with a free public default and one-command self-hosting.
- Finb/Bark (github.com/Finb/Bark): an iOS app with a separate self-hostable server.
- maxgoedjen/secretive (github.com/maxgoedjen/secretive): approvals backed by the Secure Enclave, and a small, polished Apple repo.
- getpaseo/paseo (github.com/getpaseo/paseo): runs coding agents from desktop and phone.

The items to cover for each:
- the GitHub front page, badges and app-download links
- docs and help pages
- how the app is separated from the server, MCP or CLI part
- the server, MCP or CLI code: layout, lint, types, tests, logging, errors, dependencies
- the iOS app: project layout, lint, tests, CI, signing, versioning, release
- contributing, security, license, and issue and PR templates
- the repo description and topics

The owner decided to keep BOGO's iPhone code inside the Apple Bridge iPhone app. Both repos are now public under MIT. The two READMEs must make sense together, so note how each project links its own parts across repos (for example, Bark's app and server).

Other tickets are changing the relay and the TestFlight release at the same time. Judge both repos as they are on `main` now.

## Constraints
- Research and writing only. Change nothing outside `docs/research/`.
- Open a PR against `main` on `JohanSardar/bogo`, and don't merge it. Commits use the owner's name and noreply email, and end with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.

## How to verify
1. Every item above is covered for every project, with what we do now, what they do and the gap.
2. Spot-check ten links to their files: each opens and shows what the write-up claims.
3. The summary's top gaps are things a newcomer opening both repos would notice first.

## Evidence of done
This is research, so there is no app to run. Before you report, check every link you cite actually opens and shows what you say. Check every "we do now" claim against the current `main` of both repos. In your report, give the PR link, the summary's ranked gaps, and anything you could not check.
