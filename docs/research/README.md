# How BOGO's repos compare with five similar projects

This folder compares BOGO's two public repos with five open-source projects
the owner picked, item by item, and lists where ours fall short. Next tickets
should be cut from it.

- [happy](happy.md): phone app for coding agents, encrypted relay you can host
  yourself.
- [ntfy](ntfy.md): server plus a separate iPhone app repo, free public
  instance, one-command self-hosting.
- [Bark](bark.md): iOS app plus a separate self-hostable server (with an MCP
  endpoint).
- [Secretive](secretive.md): Secure Enclave approvals, a small polished Apple
  repo.
- [Paseo](paseo.md): runs coding agents from desktop and phone, built with
  heavy agent help.

Each file covers the same seven items: front page, docs, how the app is
separated from the server, server/CLI code, the iOS app, community files, and
description and topics. Each item says what we do now, what they do (linked to
the file at the commit studied), and the gap.

**Scope.** "We" means both repos as they are on `main`:
[`JohanSardar/bogo@8500392`][bogo-tree] and
[`JohanSardar/apple-bridge-iphone@1f25c54`][iphone-tree]. T20 (TestFlight) and
T21 (relay) are changing those areas at the same time, so a few gaps here may
close as they land. Every other project was read at the commit named at the top
of its file, on 2026-11-17.

## Where we stand today

| | bogo | apple-bridge-iphone |
|---|---|---|
| Description | empty | Apple Bridge only, no mention of BOGO |
| Topics, website | none | none |
| Links to the other repo | none: the README says the approver "exists inside the Apple Bridge iPhone app" without a link; SPEC and ARCHITECTURE link to `/Users/johansardar/...` paths | none: BOGO is not mentioned |
| Badges, screenshot, download link | none | none |
| Install without building from source | no: needs Rust, Lean, Node, Python and Claude Code | no TestFlight link yet |
| Releases, tags, changelog | none | none |
| CI | none (no `.github/`) | none |
| Lint | Clippy at warn, `unsafe` forbidden; 2 of 23 crates not opted in | custom script: no comments, sources parse |
| Tests | 17 Rust test functions in 14 files, plus runnable examples, Lean proofs and JSON vectors | `make test` checks 11 Apple Bridge models; no BogoKit file |
| Logging | `tracing` in 4 files; relay uses `eprintln!` | n/a |
| SECURITY, CONTRIBUTING, CoC, templates | none; private vulnerability reporting off | none; reporting off |
| GitHub community profile | 28% | 42% |
| License | MIT | MIT |

For comparison, community profiles: ntfy 100%, Secretive 85%, Paseo 75%,
happy 50%, Bark 42%.

## Ranked gaps

Ranked by what a newcomer who opens both repos notices first. "Fixer" is who
would do the work: **Docs** (front page and docs), **Code** (daemon, CLI and
relay code and its tooling), **iPhone** (the iPhone app repo).

| # | Gap | Fixer | Best model |
|---|---|---|---|
| 1 | The two repos don't link to each other, and the iPhone README never says BOGO | Docs, iPhone | ntfy, happy, Bark |
| 2 | bogo has no description or topics; the iPhone repo's description omits BOGO | Docs | Secretive, ntfy |
| 3 | First screen has no picture, no "why", no download link; it opens on a 135-line source-build walkthrough with a demo-only approver | Docs | Secretive, Bark, happy |
| 4 | No way to install without building from source; no releases, tags or binaries | Code | Secretive, ntfy, Paseo |
| 5 | No CI or status badge on either repo | Code, iPhone | ntfy, Secretive, Bark |
| 6 | A security product with no SECURITY.md and no private reporting route | Docs | Secretive, ntfy, Paseo |
| 7 | `docs/` mixes user pages with 22 tickets, 15 design rounds and verification logs; no index | Docs | Paseo, ntfy, Bark |
| 8 | The iPhone README is written for the owner (the Air, `~/bin/ab-local`), and PRINCIPLES.md cites files that aren't in the repo | iPhone | ntfy-ios, Secretive |
| 9 | No CONTRIBUTING, no AI policy, no issue or PR templates | Docs | Paseo, ntfy, Secretive |
| 10 | The iPhone tests check no BOGO code | iPhone | Secretive, happy |
| 11 | No written versioning or release rule; TestFlight builds don't map to tags | iPhone | Paseo, happy, Secretive |
| 12 | Building the iPhone app requires editing the owner's team ID | iPhone | Secretive |
| 13 | No dependency updates or advisory checks | Code | ntfy, Secretive |
| 14 | Rust tests are named after review rounds, and there's no single "check everything" command | Code | ntfy, Paseo |
| 15 | Logging is split between `tracing` and `eprintln!` | Code | ntfy, Paseo, happy |
| 16 | No user-facing page on what the relay sees, or on self-hosting it | Docs, Code | Paseo, ntfy |
| 17 | No changelog | Docs | Paseo |

### Front page and docs

- **#1** **Link the repos both ways.** The bogo README gets a "How BOGO fits
  together" table (this repo; `apple-bridge-iphone`, whose BOGO tab and
  `ios/BogoKit` are the approver), as happy has. The iPhone README's first
  paragraph says it contains the BOGO approver and links back, as ntfy-ios
  and bark-server do. Replace every `/Users/johansardar/...` link in
  `docs/SPEC.md` and `docs/ARCHITECTURE.md` with a GitHub link. Say where to
  file BOGO bugs: ntfy sends all of them to one repo. See
  [ntfy § 3](ntfy.md#3-how-the-app-is-separated-from-the-server-mcp-or-cli-part).
- **#2** **Description and topics.** bogo: one line such as "Approve your AI agent's
  MCP tool calls on your iPhone: Mac daemon, encrypted relay, Lean-verified
  core", with topics `mcp`, `model-context-protocol`, `ai-agents`,
  `claude-code`, `secure-enclave`, `security`, `lean4`, `rust`. iPhone: a
  description that names both Apple Bridge and BOGO, plus topics. See
  [Secretive § 7](secretive.md#7-repo-description-and-topics).
- **#3** **First screen of the README.** In order: name and one sentence, a
  light/dark screenshot of the phone review screen, a "Why" list (key in the
  Secure Enclave behind Face ID; approve the exact call; verifiable log;
  proved core), a "See it work" strip (agent asks, phone shows, you slide),
  install, then links. Move the walkthrough and the "remains outstanding"
  paragraph further down or to their own pages. Give limits like "doesn't
  stop the agent's own shell" a headed section. See
  [Secretive § 1](secretive.md#1-github-front-page-badges-and-app-download-links)
  and [Bark § 1](bark.md#1-github-front-page-badges-and-app-download-links).
- **#6** **SECURITY.md.** Start with BOGO's principles, as Secretive does, link the
  existing threat model and proved/assumed page, state supported versions,
  and give the route. Turn on GitHub private vulnerability reporting on both
  repos. See [Secretive § 6](secretive.md#6-contributing-security-license-and-issue-and-pr-templates).
- **#7** **Split and index the docs.** Add `docs/README.md` with sections for Use
  BOGO, Run BOGO and How it works. Move tickets, architecture rounds, evidence
  and verification under one internal folder. Add an FAQ ("Why should I trust
  BOGO?") and a `docs/agents.md` with copy-paste MCP configs. See
  [Paseo § 2](paseo.md#2-docs-and-help-pages) and
  [Bark § 2](bark.md#2-docs-and-help-pages).
- **#9** **CONTRIBUTING, AI policy, templates.** A CONTRIBUTING.md covering both
  repos: dev setup, the one check command, and that every change carries
  verification evidence. State plainly that the code is written mostly by
  agents under review (ntfy's policy is the closest fit). Add a PR template
  that asks for evidence, and a bug form with a component dropdown (daemon,
  CLI, MCP front, relay, iPhone tab). See
  [Paseo § 6](paseo.md#6-contributing-security-license-and-issue-and-pr-templates).
- **#16** **Relay page.** One plain page: what bogo.eilmer.si sees, what a
  compromised relay can't do, who sends the push, and whether self-hosting is
  supported. Separate the operator runbook (which names the cloud project,
  VM and IP) from that page. See [Paseo § 3](paseo.md#3-how-the-app-is-separated-from-the-server-mcp-or-cli-part)
  and [ntfy § 3](ntfy.md#3-how-the-app-is-separated-from-the-server-mcp-or-cli-part).
- **#17** **CHANGELOG.md**, starting at the first tag.

### Daemon, CLI and relay code

- **#4** **Releases.** Tag a version and attach prebuilt `bogo`, `bogo-daemon`,
  `bogo-shim` and `bogo-relay` binaries for macOS (and Linux for the relay).
  A Homebrew formula or a relay container image is the next step. Secretive
  attests its release builds, which suits a security tool. See
  [ntfy § 4](ntfy.md#4-server-mcp-or-cli-code-layout-lint-types-tests-logging-errors-dependencies).
- **#5** **CI.** One workflow on every PR: `cargo fmt --check`, `cargo clippy
  --workspace --all-targets -D warnings`, `cargo test --workspace`, the Lean
  build, and the vector check. Put the badge at the top of the README.
- **#13** **Dependencies.** Dependabot (or Renovate) for Cargo and Actions, with
  `rmcp` pinned by an ignore rule. Add `cargo deny check advisories licenses`
  in CI, and a "Dependencies" section in SECURITY.md naming the crates that
  touch keys and the wire.
- **#14** **Tests and one check command.** Rename `review_s1.rs`-style files after
  what they protect. Add a `make check` (or `cargo xtask check`) that runs
  everything in item 5 locally. Bring `bogo-core-sys` and `bogo-verify` under
  workspace lints, or say why they stay out.
- **#15** **Logging.** Use `tracing` with `tracing-subscriber` in every binary,
  including the relay, with a JSON option, and document where logs go in
  `docs/daemon.md`.

### iPhone app

- **#1** (shared with docs) The README names BOGO in its first paragraph, links the
  bogo repo, and gives BOGO's code a short layout section (`BogoModel`,
  `BogoCrypto`, `BogoRelay`, `BogoSection`).
- **#5** **CI.** A macOS workflow on every PR running `make lint` and the BogoKit
  tests (item 10). Bark's [tests.yaml](bark.md#5-the-ios-app-project-layout-lint-tests-ci-signing-versioning-release)
  is the minimum.
- **#8** **Write the README and principles for outsiders.** Remove "On the Air, run
  each step through `~/bin/ab-local`" and other references to the owner's
  machines. Rewrite PRINCIPLES.md without "Johan's rules win" and without
  citing `design/DECISIONS.md` and other files that aren't in the repo.
- **#10** **Test BOGO code.** Add a test target, or a Swift package with `swift
  test`, for `BogoModel`, `BogoCrypto` and `BogoRelay/Messages.swift`, and run
  it from `make test`. See
  [Secretive § 3](secretive.md#3-how-the-app-is-separated-from-the-server-mcp-or-cli-part).
- **#11** **Versioning and release.** Write one rule (happy uses CalVer `YYYY.M.C`;
  Paseo derives the build number from a single version). Set it in
  `project.yml` instead of per run, and tag every TestFlight build's commit.
  Add a short `docs/release.md` once T20 settles the flow. See
  [Paseo § 5](paseo.md#5-the-ios-app-project-layout-lint-tests-ci-signing-versioning-release).
- **#12** **Build without the owner's team.** Move `DEVELOPMENT_TEAM` and
  `bundleIdPrefix` into an xcconfig with a git-ignored local override and a
  one-command setup script, as Secretive's `configure_team_id.sh` does.

## How the others link their parts across repos

The owner kept BOGO's phone code inside the Apple Bridge iPhone app, so both
READMEs must explain the split. Patterns seen:

- **happy:** a table in the main README naming each repo and marking "this
  repository" ([README](https://github.com/slopus/happy/blob/1d6d88c5e14cbfb98cad76881acd7ff171edb50c/README.md?plain=1#L41)).
- **ntfy:** the server README links each app repo next to its store badge. The
  iOS repo's README opens with "This is the iOS app for ntfy". Issues are off on
  ntfy-ios, so all bugs go to the server repo
  ([ntfy-ios README](https://github.com/binwiederhier/ntfy-ios/blob/96e659caf4b14405e201db6f5e4b2c6947c81643/README.md)).
- **Bark:** the app repo is the front door and hosts the docs for both. The
  docs home lists both repos in its first lines, and the server README opens
  by linking the app
  ([docs home](https://github.com/Finb/Bark/blob/a5c1cf5e0ff273c04d252d71c78b58f149e7c274/docs/en-us/README.md),
  [server README](https://github.com/Finb/bark-server/blob/a365aa9ddc9362495efffb3db4358b7c12beb963/README.md)).
- **Paseo:** one monorepo, plus a "Related projects" list for the separate
  relay repo ([README](https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/README.md?plain=1#L225)).
- **Secretive:** one repo, so nothing to link.

For BOGO, use happy's table in bogo and ntfy-ios's opening line in the iPhone
repo. Issues can't be turned off on the iPhone repo, because it also carries
Apple Bridge.

## Where we are already ahead

Keep these through any cleanup:

- **Formal core.** A Lean model of the decision core, with
  [proved-and-assumed.md](../proved-and-assumed.md) saying exactly what is
  proved. Of the others, only happy has Lean proofs, for its iOS sync logic.
- **Threat model and ADRs.** [threat-model.md](../threat-model.md) and three
  ADRs go further than any of the five, except Paseo's SECURITY.md, which is
  comparable for the relay.
- **Evidence culture.** `docs/verification/` records what was actually run.
  Paseo asks contributors for this in its PR template; BOGO already does it.
- **Pinned dependencies with reasons.** `rmcp = "=3.10.0"` with the reason in
  [Cargo.toml](../../Cargo.toml).
- **Glossary.** [GLOSSARY.md](../../GLOSSARY.md); only Paseo has one too.

## What could not be checked

- The rendered websites (happy.engineering, ntfy.sh/docs, bark.day.app,
  secretive.dev, paseo.sh) were not reviewed page by page. Claims cite the
  source files in each repo instead.
- App Store listings, ratings and TestFlight groups were taken from README
  badges and workflow files, not from App Store Connect.
- Secretive's "no third-party dependencies" claim is cited from its
  SECURITY.md and CONTRIBUTING.md. The Xcode project's package list was not
  audited.
- How each project's CI behaves in practice (pass rates, run times) was not
  checked; only the workflow files were read.

[bogo-tree]: https://github.com/JohanSardar/bogo/tree/8500392dc0d5845a2fbb489b595779db41378620
[iphone-tree]: https://github.com/JohanSardar/apple-bridge-iphone/tree/1f25c547a4e74501f6d7820d92692ba5f19f0f3f
