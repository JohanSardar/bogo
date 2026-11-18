# `bogo setup`: one guided command from install to a protected agent

## Goal
After installing, a newcomer runs `bogo setup` and ends with an agent whose risky tools wait for their iPhone, without editing any TOML or JSON. Today they must hand-write `bogo.toml`, add each upstream, add an agent and copy its token into a client config. Here is how the projects we compare with do it:
- [Cordon](https://github.com/marras0914/cordon)'s `cordon init` reads the existing Claude config, moves its MCP servers behind the gateway and writes the policy;
- [holdgate](https://github.com/im-amir/holdgate) swaps one line in the client config;
- Happy's first run pairs the phone straight away.

The owner's first real use will be a Gmail agent where sending needs Face ID, so Gmail is the first recipe.

When this is done, each of these is true on its own:
- **`bogo setup` walks through everything, and is safe to run again.** It explains each step in one plain line and skips steps that are already done:
  1. Create the home if it doesn't exist.
  2. Start BOGO in the background.
  3. Add your iPhone, using the same 6-digit pairing `bogo approver add` uses; skip this if an approver exists.
  4. Connect an agent.
  5. Make one protected test call the owner approves on the phone.

  Every step can also run on its own, and a flag runs setup without questions.
- **Import existing MCP servers.** Setup finds the MCP servers already configured in Claude Code, Claude Desktop and Cursor, and offers to move each one behind BOGO:
  - it adds each server as an upstream;
  - it moves any credentials in the server's config into `secrets.toml`, referred to by name;
  - it replaces the client's entries with one `bogo` entry that uses the shim and its own agent.

  It backs up every file it changes, and `bogo setup --undo` restores them exactly.
- **Suggested policy.** For each tool, setup suggests a policy from the MCP tool annotations the server publishes, and the owner confirms or changes each suggestion:
  - `readOnlyHint` suggests **allow**;
  - `destructiveHint`, and anything that sends, pays, writes or deletes, suggests **approve**;
  - a tool with no hint suggests **approve**.

  Nothing is allowed without the owner confirming it. The default policy stays `approve`.
- **Recipes.** `bogo upstream add gmail` sets up a well-maintained, open-source Gmail MCP server. Pick it, and say why in the PR.
  - Sending, and anything else that leaves the mailbox, needs approval. Reading and searching are allowed.
  - It prints the Google sign-in steps the owner must do themselves.
  - Add recipes the same way for GitHub, Slack, Stripe (test mode) and files.
  - Each recipe pins the server's version.
- **`bogo doctor`** checks each thing a working setup needs and says in one plain line what's wrong and the fix for each: daemon running, port, phone paired, approver attestation environment, relay reachable, each upstream starting, each agent's client config pointing at BOGO.
- **TestFlight and App Store builds are accepted.** A new home's config accepts both App Attest environments, `production` for TestFlight and the App Store and `development` for Xcode builds, so the TestFlight app pairs with no config edit. `doctor` flags a home that would reject a TestFlight phone.
- **Round two's daemon gaps are closed.** T29's goal, from `docs/tickets/T29-daemon-round-two.md`, is part of this ticket. That ticket covers C4 diagnostics, which `doctor` should use, C5 `catalog_changed`, and the C6 leftovers. Follow its constraints, guardrails and checks.

## Context
- The repo is `/Users/johansardar/.t3/projects/bogo`. Make your own worktree and branch from `main`. The C6 vectors need the same change in `/Users/johansardar/apple-bridge-iphone`, in its own worktree and PR.
- **What exists to build on:** `bogo init`, `upstream add`, `agent add`, `approver add` (6-digit nearby pairing) and `docs/agents.md`. See `docs/agents.md` for how each client is configured and why the token stays out of files the agent can edit. The config shapes are in `docs/configuration.md`.
- **Agent token placement:** keep the rule that the token is never written where the agent can edit it. If a client can only take it from its config file, pick the safest place it allows, and say in the PR what an agent with a shell could still reach.
- **Other tickets running at the same time:**
  - T33 owns the release workflow, `install.sh` and `--version`;
  - T35 owns the relay, pairing internals, pushes and the iPhone app.

  Setup calls the existing pairing as it is. Where T35 changes it, the coordinator merges both.

## Constraints
- Don't run `bogo start` or `bogo stop`, and don't load or unload the `si.eilmer.bogo` LaunchAgent. It is one label per Mac account, so doing so stops the owner's live daemon at `~/.bogo`.
  - To prove the "start in the background" step, give the LaunchAgent label a test-only override and use a different label.
  - Otherwise use `serve` in the foreground, on its own home and port.
- Never read, change or back up the owner's real client configs: `~/.claude.json`, `~/.claude/`, Claude Desktop's config and `~/.cursor/`. Point setup at scratch copies with a flag or a scratch `HOME`, and only run CLIs under it. Don't launch desktop apps with a changed `HOME`.
- Don't do any Google sign-in, and don't use the owner's accounts. Prove the Gmail recipe up to the point where the owner's sign-in is needed.
- No new third-party Rust crates without saying why in the PR. Don't change what is signed or how the Lean core decides.
- Never print or commit a key, token or password. Spend nothing.
- Open a PR against `main` in each repo you change, and don't merge them. Push over SSH (`git@github.com:JohanSardar/<repo>.git`). Commits use the owner's name and noreply email, and end with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.

## Guardrails
- The approval flow, fail-closed behaviour and the README quick start's results are unchanged.
- The Lean default still can never be `allow`. Setup only writes explicit per-tool entries the owner confirmed.
- No credential or token appears in logs, terminal output after first display, or files the agent can edit.
- `--undo` restores every changed client file byte for byte.
- The Lean proofs build, and `#print axioms` stays as pinned.

## How to verify
1. Use a scratch `HOME` holding a Claude Code config with two MCP servers, one with an API key in its env. Run `bogo setup` with the software test approver standing in for the phone.
   - Both servers become upstreams.
   - The key moves to `secrets.toml`.
   - Claude Code, run under that `HOME` through BOGO, reads freely and waits on a write.
   - `bogo soft` approves it.
   - `--undo` restores the config byte for byte.
2. `bogo upstream add gmail` installs the pinned server and shows the sign-in steps. The policy it writes protects sending.
3. `bogo doctor` on a healthy home, then on homes with each fault: daemon down, no approver, a broken upstream, development-only attestation. Show the output for each.
4. T29's How to verify steps.
5. `make check` passes, the iPhone repo's `make test` passes with the new vectors, and CI passes.

## Evidence of done
When you are done, use what you changed the way a user would: run the real app and go through each screen, flow, or caller that runs the code you changed. If it has no user, use it the way its real client would, such as a program calling the API or an agent calling the tool. Check that it works and that it does everything this ticket asks. Go through every step in How to verify and check every guardrail. Skip parts your change can't reach. Match the proof to the change: a text or style change needs one look at the screen it changes, and a new flow needs one walk through it. When the change touches no code that runs for a user or client, such as docs, comments, tests, or CI config, don't run the app: lint, types, and the existing checks are enough. Follow the `agent-testing` skill for all testing. If you changed code, run the `sol-loop` skill on it and give Sol this ticket. In your report, show what you saw at each step: screenshots, a video, or the output. Then list anything you did not check.
