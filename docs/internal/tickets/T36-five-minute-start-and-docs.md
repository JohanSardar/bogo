# Rewrite the front pages around a five-minute start, and close the docs gaps

Runs after T33, T34 and T35 merge.

## Goal
A newcomer opens either repo and is protected in about five minutes, in three steps:
1. Install with `curl -fsSL https://raw.githubusercontent.com/JohanSardar/bogo/main/install.sh | sh`.
2. Get the app from TestFlight.
3. Run `bogo setup`.

Happy and ntfy open the same way. The build-from-source quick start moves to a contributor page, and the software test approver demo stays as a "try it without an iPhone" option.

When this is done, each of these is true on its own:
- **Both READMEs open with the three steps** and a short screen recording or GIF of a protected call being approved. The "Which repo do I want?" table stays, and the two READMEs still read as one story.
- **Recipes are documented.** There's a page per recipe, starting with Gmail: what is allowed and what needs approval, the Google sign-in steps, and how to check it works. Each is linked from the README.
- **`bogo setup`, `bogo doctor` and the curl install are documented** in the docs index, with what each changes on the Mac.
- **T28's goal is part of this ticket.** Read `docs/tickets/T28-docs-round-two.md`: A2 through A7 and I3. Follow its constraints, guardrails and checks.
- **The status note says what is true when you write it**, including whether the TestFlight link admits testers and whether a real-iPhone session has been recorded. Ask the coordinator if you're unsure.

## Context
- The repos are `/Users/johansardar/.t3/projects/bogo` and `/Users/johansardar/apple-bridge-iphone`. Make your own worktree and branch from `main` in each.
- Read the merged PRs for T33, T34 and T35 for the real commands and their output. Run each command you document, in a scratch home and install dir.

## Constraints
- Docs, community files, the recording, and GitHub repo settings only. No code changes.
- Don't run `bogo start` or `bogo stop`, and don't load or unload `si.eilmer.bogo`. Don't overwrite `~/.local/bin/bogo`.
- Open one PR against `main` in each repo, and don't merge them. Push over SSH. Commits use the owner's name and noreply email, and end with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.

## Guardrails
- The docs stay honest about what is and isn't verified.
- Every link in both repos still opens.

## How to verify
1. As a newcomer, with a scratch `HOME`, follow the README's three steps, using the software test approver in place of the phone. You end with a protected call approved.
2. A link checker over both repos finds no broken link.
3. T28's How to verify steps.

## Evidence of done
When you are done, use what you changed the way a user would: run the real app and go through each screen, flow, or caller that runs the code you changed. If it has no user, use it the way its real client would, such as a program calling the API or an agent calling the tool. Check that it works and that it does everything this ticket asks. Go through every step in How to verify and check every guardrail. Skip parts your change can't reach. Match the proof to the change: a text or style change needs one look at the screen it changes, and a new flow needs one walk through it. When the change touches no code that runs for a user or client, such as docs, comments, tests, or CI config, don't run the app: lint, types, and the existing checks are enough. Follow the `agent-testing` skill for all testing. If you changed code, run the `sol-loop` skill on it and give Sol this ticket. In your report, show what you saw at each step: screenshots, a video, or the output. Then list anything you did not check.
