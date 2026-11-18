# Pushes for everyone, pairing off the Mac's network, and Simulator builds that pair

## Goal
This ticket covers everything about the phone and the Mac finding each other, so that anyone who installs BOGO can use it with an iPhone. It combines two earlier tickets and adds one small relay route:
- **T31's goal:** the relay sends pushes, pairing works off the Mac's network, real failure messages, the relay health check, the "Add your iPhone" guide, and ADR 0003. Read `docs/tickets/T31-relay-push-and-off-network-pairing.md`; its Goal, Context, Constraints, Guardrails and How to verify all apply here.
- **T30's goal:** an unsigned `make simulator` build pairs with a real Mac, and the builds print no warnings. Read `docs/tickets/T30-iphone-round-two.md`; all of it applies here.
- **An install redirect.** The relay answers `GET /install.sh` with a redirect to `https://raw.githubusercontent.com/JohanSardar/bogo/main/install.sh`. Install directly with `curl -fsSL https://raw.githubusercontent.com/JohanSardar/bogo/main/install.sh | sh`; this command does not depend on the relay. It serves no script itself, and it doesn't log the request beyond what the relay logs today.

## Context
- The repos are `/Users/johansardar/.t3/projects/bogo` and `/Users/johansardar/apple-bridge-iphone`. Make your own worktree and branch from `main` in each, and don't use the iPhone repo's main checkout.
- **The owner's explanation of this work:**
  - Today only the owner's Mac can push to the app, because pushes need the app publisher's key.
  - Pairing needs the same Wi-Fi, because the phone finds the Mac nearby.
  - Unsigned Simulator builds can't save keys.
- **Deploying is the coordinator's job.** Write down exactly what the live relay needs: the binary, the push key file and its permissions, the settings, and the order of steps that keeps paired phones and Apple Bridge's service on port 443 working.
- **Other tickets running at the same time:**
  - T33 owns the release workflow and `install.sh`;
  - T34 owns `bogo setup`, `bogo doctor`, recipes, daemon logging, `catalog_changed` and the C6 leftovers.

  `bogo setup` calls `approver add` as it is today. Keep its command-line behaviour for nearby pairing, and say in the PR what changed for off-network pairing.

## Constraints
All of T31's and T30's constraints apply. In particular:
- Don't run `bogo start` or `bogo stop`, and don't load or unload `si.eilmer.bogo`.
- Don't redeploy the live relay, or change the VM or DNS.
- Never read, print or copy the owner's push key.
- Don't change bundle IDs, release entitlements, release signing, what is signed, or how the Lean core decides.
- Use the T3 device tools on a new Simulator, and delete it afterwards.
- Open one PR against `main` in each repo, and don't merge them. Push over SSH. Commits use the owner's name and noreply email, and end with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.

## Guardrails
All of T31's and T30's guardrails apply. In addition, the redirect route can't serve or rewrite content, and nothing else on the relay changes for it.

## How to verify
1. All of T31's and T30's How to verify steps.
2. `curl -sI` on a local relay's `/install.sh` shows the redirect to GitHub's raw URL.

## Evidence of done
When you are done, use what you changed the way a user would: run the real app and go through each screen, flow, or caller that runs the code you changed. If it has no user, use it the way its real client would, such as a program calling the API or an agent calling the tool. Check that it works and that it does everything this ticket asks. Go through every step in How to verify and check every guardrail. Skip parts your change can't reach. Match the proof to the change: a text or style change needs one look at the screen it changes, and a new flow needs one walk through it. When the change touches no code that runs for a user or client, such as docs, comments, tests, or CI config, don't run the app: lint, types, and the existing checks are enough. Follow the `agent-testing` skill for all testing. If you changed code, run the `sol-loop` skill on it and give Sol this ticket. In your report, show what you saw at each step: screenshots, a video, or the output. Then list anything you did not check.
