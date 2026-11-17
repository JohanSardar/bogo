# Make both repos' front pages and docs as clear as the best projects like them

## Goal
Make the front page and docs of `JohanSardar/bogo` and `JohanSardar/apple-bridge-iphone` as clear and well kept as Happy, ntfy, Bark, Secretive and Paseo. Both repos were just made public. The owner wants someone who opens one of those projects and then ours to call ours just as clear and polished. The research in `docs/research/` (start with its `README.md`) lists the gaps, ranked. This ticket owns the front-page and docs gaps: 1, 2, 3, 6, 7 and 9, the docs half of 8, and the changelog from 17.

When this is done, each of these is true on its own:
- **The bogo README's first screen** says what BOGO is and why someone would want it, in plain words. It shows the logo, a tagline and real screenshots of the phone's Review card. It has a TestFlight button (see Context), a quick start that works when copied, a short "how it works and why it's safe", a map of the repo, and links to the deeper docs.
- **The iPhone README** says in its first lines what the app is and that its BOGO section approves agent tool calls for the BOGO daemon, with a link to bogo.
- **The two READMEs make sense together.** The owner asked for this explicitly. Each repo states in its first line what it is. Both carry the same small "Which repo do I want?" table, covering the BOGO daemon and relay, the Apple Bridge iPhone app, and the Apple Bridge Mac repo. Each topic is explained in one repo and linked from the other, never copied into both. Both use the same words, from `GLOSSARY.md`.
- **No link in either repo points to a local path** such as `/Users/...`, and every link in both repos opens.
- **`docs/` has an index page** that separates the pages a user needs from internal records (tickets, design rounds, verification logs). A newcomer is not dropped into internal files first.
- **Both repos have** a LICENSE (MIT; both have one now), CONTRIBUTING, SECURITY (with private vulnerability reporting turned on), a CHANGELOG, issue and PR templates, a GitHub description and topics.

A README that looks polished but whose quick start fails when copied does not count. Neither does a pair of READMEs that each read well alone while a reader can't tell how the two repos relate.

## Context
- The research is in `docs/research/` in the bogo repo: one file per project plus a ranked summary with links to their files.
- The owner decided to keep BOGO's iPhone code inside the Apple Bridge iPhone app. There is no standalone app.
- The Apple Bridge Mac repo (`JohanSardar/apple-bridge`) is public, but nothing in it may be changed. The READMEs may link to it.
- **TestFlight:** another ticket, T20, is getting the app onto TestFlight with a public link right now. If you don't have the link when you finish, leave the button out and say so in the PR; the coordinator adds it when T20 lands. Don't invent a link.
- **Screenshots:** real Simulator screenshots of the current app are in `/Users/johansardar/.t3/worktrees/apple-bridge-iphone/t19-evidence/`. Fresh ones from the app on `main` are better. Debug builds have Settings → Demo, which sends a realistic Link purchase to the Review card.
- **Files other tickets own, running at the same time:**
  - T21 owns the relay: `docs/RELAY-PROTOCOL.md`, `deploy/relay/`, the relay crates, and `ios/BogoKit/BogoRelay/`. Link to the relay docs, but don't edit them.
  - T24 owns the bogo Rust code and CI.
  - T25 owns the iPhone app code, CI and `ios/README.md`.
  - T20 owns `packaging/release/` and the release steps.

  Your files are the top-level README, the community files, `docs/` (except the relay files) and `GLOSSARY.md` in each repo.

## Constraints
- Docs, community files and GitHub repo settings (description, topics, vulnerability reporting) only. No code changes.
- Publish nothing beyond the GitHub repos: no tags, releases or packages.
- Don't move or delete the internal records. Indexing them or grouping them under a clear folder is fine, as long as every link to them still works.
- Open one PR against `main` in each repo, and don't merge them. Commits use the owner's name and noreply email, and end with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.

## Guardrails
- The quick start stays no longer than it is now, in steps and in what it needs installed.
- The docs stay honest about what is and isn't verified, such as the software test approver, the real-device checks and Link. Polish must not turn a limitation into a claim.
- The threat model, ADRs, proved-and-assumed page and verification records stay reachable from the docs index. The research lists them as where we are ahead.

## How to verify
1. Open both repos on GitHub as a newcomer, one after the other. From either README you can tell within the first screen what each repo is and which one you want.
2. Copy the bogo quick start into a fresh terminal and run it to the end. It works as written.
3. A link checker over both repos finds no broken link and no local path.
4. Compare each front page with Happy's, ntfy's and Secretive's, item by item from the research. Nothing in the research's front-page and docs gaps is left without either a fix or a stated reason.
5. On GitHub, both repos show the description, topics, license, security policy and templates.

## Evidence of done
When you are done, use what you changed the way a user would: run the real app and go through each screen, flow, or caller that runs the code you changed. If it has no user, use it the way its real client would, such as a program calling the API or an agent calling the tool. Check that it works and that it does everything this ticket asks. Go through every step in How to verify and check every guardrail. Skip parts your change can't reach. Match the proof to the change: a text or style change needs one look at the screen it changes, and a new flow needs one walk through it. When the change touches no code that runs for a user or client, such as docs, comments, tests, or CI config, don't run the app: lint, types, and the existing checks are enough. Follow the `agent-testing` skill for all testing. If you changed code, run the `sol-loop` skill on it and give Sol this ticket. In your report, show what you saw at each step: screenshots, a video, or the output. Then list anything you did not check.
