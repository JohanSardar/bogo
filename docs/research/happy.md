# happy (slopus/happy)

A phone app for coding agents, with an end-to-end encrypted relay you can host
yourself. It is the closest match to BOGO's shape: a CLI on the computer, a
native iPhone app, and a relay in the middle that only carries ciphertext.

Studied at [`1d6d88c`][happy-tree] on 2026-11-17. BOGO is judged at
[`JohanSardar/bogo@8500392`][bogo-tree] and
[`JohanSardar/apple-bridge-iphone@1f25c54`][iphone-tree].

## 1. GitHub front page, badges and app-download links

**What we do now.** The [bogo README][bogo-readme] opens with a small icon,
one paragraph, three doc links, then a 135-line build-from-source walkthrough
that uses a software test approver. It has no badges, no screenshot and no
download or TestFlight link. The [iPhone README][iphone-readme] is about Apple
Bridge and never mentions BOGO.

**What they do.** The [README][happy-readme] opens with a centred title, an
embedded demo video, then dark/light-aware download buttons for desktop, the
App Store and Google Play, each showing the store rating. Next come five
numbered reasons to use it, one row of resource links (website, docs, Discord,
development guide, license), and then the install command (`npm install -g
happy`).

**Gap.**
- No picture of the product. A newcomer never sees the phone approval screen,
  the one thing BOGO adds. Add a light/dark screenshot or short clip of the
  review and slide-to-approve screens near the top of the bogo README.
- No App Store or TestFlight button. Once T20 has a public TestFlight link, put
  it as a badge in the first screen of both READMEs.
- No short "why" list ahead of the build steps. Move the walkthrough below a
  four-to-five-item "What you get" list and a one-line install.

## 2. Docs and help pages

**What we do now.** User docs are Markdown files in [`docs/`][bogo-docs]:
configuration, daemon, threat model, proved/assumed, spec and architecture.
They sit next to 22 internal tickets, 15 architecture design and critique
rounds, and verification logs, with no index page and no website.
[SPEC.md][bogo-spec-319] and [ARCHITECTURE.md][bogo-arch-687] link to
`/Users/johansardar/...` paths, which go nowhere on GitHub.

**What they do.** User docs live on a separate website (happy.engineering,
built from [slopus/slopus.github.io][happy-site-repo]). The repo's
[`docs/`][happy-docs] holds developer reference: [encryption.md][happy-encryption]
(blob layout, with a diagram, tied to the source file it describes),
[deployment.md][happy-deployment], `protocol.md` and `api.md`. Plans and
research sit in their own subfolders (`docs/plans`, `docs/research`). A
[PRIVACY.md][happy-privacy] sits at the root.

**Gap.**
- Internal process files (tickets, design rounds, verification logs) are mixed
  in with user docs. Move them under one clearly named folder such as
  `docs/internal/`, and add a `docs/README.md` index that lists the user pages
  first.
- Fix every `/Users/johansardar/...` link in SPEC.md and ARCHITECTURE.md to
  point at the public iPhone repo.
- No privacy statement. The phone app sends request contents through a relay
  BOGO runs. Add a PRIVACY.md that says what the relay at bogo.eilmer.si can and
  cannot see.

## 3. How the app is separated from the server, MCP or CLI part

**What we do now.** Two repos. BOGO's Rust workspace holds the daemon, CLI, MCP
front, shim, relay and Lean core. The iPhone approver is `ios/BogoKit` inside
the Apple Bridge iPhone app. Neither README links to the other: the bogo
README says only that "the iPhone approver code exists inside the Apple Bridge
iPhone app", with no link.

**What they do.** The README has a ["How Happy fits together"][happy-readme-fits]
table that names every repository, says what each is, and marks "← this
repository". In the monorepo, the wire format is its own package
([happy-wire][happy-wire]) shared by CLI, server and apps. The server is a
private package, and a thin publishing shell
([happy-server-self-host][happy-selfhost]) packages it for `happy server`.

**Gap.**
- Copy the table. The bogo README needs a "How BOGO fits together" table with
  two rows (this repo; `apple-bridge-iphone`, "the iPhone app; its BOGO tab is
  the approver; code in `ios/BogoKit`"). The iPhone README needs the mirror
  row pointing back.
- The shared wire contract has no single named home a newcomer can find. BOGO
  has [`vectors/`][bogo-vectors] and a copy in the iPhone repo's
  `ios/BogoKit/BogoRelay/StandInVectors`. Name the source of truth in both
  READMEs and say how the copy is kept in sync.

## 4. Server, MCP or CLI code: layout, lint, types, tests, logging, errors, dependencies

**What we do now.** 23 crates in one Cargo workspace. Workspace lints forbid
`unsafe` and warn on all Clippy lints ([Cargo.toml][bogo-cargo]), but two crates
(`bogo-core-sys`, `bogo-verify`) do not opt in. There are 17 `#[test]` functions in
14 files. Most checks are runnable `examples/` plus Lean proofs and JSON
vectors. Test files are named after review rounds (`review_s1.rs`,
`review_s2.rs`). Errors use `thiserror` in 8 crates. Logging is mixed:
`tracing` appears in 4 source files, while the relay prints with `eprintln!`
([main.rs][bogo-relay-main]). There is no CI and no dependency bot.

**What they do.** The server uses one logger, pino, set up once in
[`utils/log.ts`][happy-log]. Request validation uses zod through
`fastify-type-provider-zod`, and the server has vitest plus `tsc --noEmit`
typecheck scripts ([package.json][happy-server-pkg]). Releases run from one
manual workflow with every action pinned to a commit SHA
([release-happy-cli.yml][happy-release]). Happy is not ahead everywhere: its
[AGENTS.md][happy-agents-actions] forbids adding CI triggers, so it has no PR
CI either.

**Gap.**
- Pick one logging route (`tracing` with `tracing-subscriber`) and use it in
  the relay too, so operators get the same log format from every binary.
- Rename `review_s1.rs`-style tests after the behaviour they pin. A newcomer
  can't tell what `review_s2.rs` protects.
- Say in the README or a CONTRIBUTING file that the runnable examples, Lean
  proofs and vectors are the test suite, and give the single command that runs
  them all.

## 5. The iOS app: project layout, lint, tests, CI, signing, versioning, release

**What we do now.** XcodeGen [`project.yml`][iphone-projectyml] with
`MARKETING_VERSION: 0.1`, `CURRENT_PROJECT_VERSION: 1`, manual signing and the
team ID committed. `make lint` runs a custom script that bans comments and
checks that Swift, Python and shell files parse ([lint.py][iphone-lint]).
`make test` runs model proofs for eleven Apple Bridge capabilities
([check-models.py][iphone-checkmodels]); none of them is a BogoKit file. No
CI. Release is [testflight.py][iphone-testflight], which takes `--version` and
`--build` by hand on every run. No tags.

**What they do.** [happy-ios][happy-ios-readme] is its own package in the
monorepo with `Package.swift` and 13 test targets under `Tests/`
(crypto, wire, store, layout and others). The README gives one command per
platform. Versioning is written down: CalVer `YYYY.M.C`, set once in
`project.yml`, build number per variant, and a tag
`native-ios-<version>-build<build>` on the exact commit when a build ships
publicly ([AGENTS.md § Versioning][happy-ios-versioning]). The README states
the next production build number and how signing and upload work.

**Gap.**
- `make test` checks no BOGO code. Add a BogoKit test target (or package
  tests) for the model and crypto files (`BogoModel`, `BogoCrypto`,
  `BogoRelay/Messages.swift`) and run it from `make test`.
- No written versioning rule and no tags. Write one rule (happy's CalVer or
  semver) in the iPhone README, set the version in `project.yml` rather than
  per run, and tag the commit of each TestFlight build.
- No CI on the iPhone repo. At minimum, run `make lint` and the BogoKit tests on
  a macOS runner for every PR.

## 6. Contributing, security, license and issue and PR templates

**What we do now.** Both repos have an MIT `LICENSE` and nothing else. No
CONTRIBUTING, no SECURITY, no code of conduct, no templates, and GitHub's
private vulnerability reporting is off on both. GitHub's community profile
scores bogo 28% and the iPhone repo 42%.

**What they do.** A [CONTRIBUTING guide][happy-contributing] in `docs/` covers
priorities, issue rules, PR rules and dev setup for every package. MIT
[LICENSE][happy-license]. Happy has no SECURITY.md and no issue templates
either (community profile 50%).

**Gap.** Even happy, the least formal project here, has a contributing guide
with dev setup per package. BOGO needs one that covers both repos. Happy is
not a good model for security reporting; see the secretive and ntfy notes.

## 7. Repo description and topics

**What we do now.** bogo: empty description, no topics, no website. iPhone:
"Native iPhone companion for Apple Bridge: app, widgets, extensions and iOS
builds", no topics, no mention of BOGO.

**What they do.** Description "Orchestrate Claude, Codex and Grok in one app.
Open-source (MIT), reuses the subscriptions you already pay for." Website
happy.engineering. Nine topics, including `claude-code`, `codex`,
`coding-agents` and `ai-agents` ([repo page][happy-repo]).

**Gap.** bogo needs a one-line description that names what it does ("Phone
approval for AI agents' MCP tool calls: a Mac daemon, an encrypted relay and a
Lean-verified core"), topics such as `mcp`, `model-context-protocol`,
`ai-agents`, `claude-code`, `approval`, `secure-enclave`, `lean4` and `rust`,
and a website field once one exists. The iPhone repo's description should
mention its BOGO tab.

[happy-tree]: https://github.com/slopus/happy/tree/1d6d88c5e14cbfb98cad76881acd7ff171edb50c
[happy-repo]: https://github.com/slopus/happy
[happy-readme]: https://github.com/slopus/happy/blob/1d6d88c5e14cbfb98cad76881acd7ff171edb50c/README.md
[happy-readme-fits]: https://github.com/slopus/happy/blob/1d6d88c5e14cbfb98cad76881acd7ff171edb50c/README.md?plain=1#L41
[happy-site-repo]: https://github.com/slopus/slopus.github.io
[happy-docs]: https://github.com/slopus/happy/tree/1d6d88c5e14cbfb98cad76881acd7ff171edb50c/docs
[happy-encryption]: https://github.com/slopus/happy/blob/1d6d88c5e14cbfb98cad76881acd7ff171edb50c/docs/encryption.md
[happy-deployment]: https://github.com/slopus/happy/blob/1d6d88c5e14cbfb98cad76881acd7ff171edb50c/docs/deployment.md
[happy-privacy]: https://github.com/slopus/happy/blob/1d6d88c5e14cbfb98cad76881acd7ff171edb50c/PRIVACY.md
[happy-wire]: https://github.com/slopus/happy/tree/1d6d88c5e14cbfb98cad76881acd7ff171edb50c/packages/happy-wire
[happy-selfhost]: https://github.com/slopus/happy/blob/1d6d88c5e14cbfb98cad76881acd7ff171edb50c/packages/happy-server-self-host/README.md
[happy-log]: https://github.com/slopus/happy/blob/1d6d88c5e14cbfb98cad76881acd7ff171edb50c/packages/happy-server/sources/utils/log.ts
[happy-server-pkg]: https://github.com/slopus/happy/blob/1d6d88c5e14cbfb98cad76881acd7ff171edb50c/packages/happy-server/package.json
[happy-release]: https://github.com/slopus/happy/blob/1d6d88c5e14cbfb98cad76881acd7ff171edb50c/.github/workflows/release-happy-cli.yml
[happy-agents-actions]: https://github.com/slopus/happy/blob/1d6d88c5e14cbfb98cad76881acd7ff171edb50c/AGENTS.md?plain=1#L27
[happy-ios-readme]: https://github.com/slopus/happy/blob/1d6d88c5e14cbfb98cad76881acd7ff171edb50c/packages/happy-ios/README.md
[happy-ios-versioning]: https://github.com/slopus/happy/blob/1d6d88c5e14cbfb98cad76881acd7ff171edb50c/packages/happy-ios/AGENTS.md?plain=1#L690
[happy-contributing]: https://github.com/slopus/happy/blob/1d6d88c5e14cbfb98cad76881acd7ff171edb50c/docs/CONTRIBUTING.md
[happy-license]: https://github.com/slopus/happy/blob/1d6d88c5e14cbfb98cad76881acd7ff171edb50c/LICENSE
[bogo-tree]: https://github.com/JohanSardar/bogo/tree/8500392dc0d5845a2fbb489b595779db41378620
[bogo-readme]: https://github.com/JohanSardar/bogo/blob/8500392dc0d5845a2fbb489b595779db41378620/README.md
[bogo-docs]: https://github.com/JohanSardar/bogo/tree/8500392dc0d5845a2fbb489b595779db41378620/docs
[bogo-spec-319]: https://github.com/JohanSardar/bogo/blob/8500392dc0d5845a2fbb489b595779db41378620/docs/SPEC.md?plain=1#L319
[bogo-arch-687]: https://github.com/JohanSardar/bogo/blob/8500392dc0d5845a2fbb489b595779db41378620/docs/ARCHITECTURE.md?plain=1#L687
[bogo-vectors]: https://github.com/JohanSardar/bogo/tree/8500392dc0d5845a2fbb489b595779db41378620/vectors
[bogo-cargo]: https://github.com/JohanSardar/bogo/blob/8500392dc0d5845a2fbb489b595779db41378620/Cargo.toml
[bogo-relay-main]: https://github.com/JohanSardar/bogo/blob/8500392dc0d5845a2fbb489b595779db41378620/crates/bogo-relay/src/main.rs#L481
[iphone-tree]: https://github.com/JohanSardar/apple-bridge-iphone/tree/1f25c547a4e74501f6d7820d92692ba5f19f0f3f
[iphone-readme]: https://github.com/JohanSardar/apple-bridge-iphone/blob/1f25c547a4e74501f6d7820d92692ba5f19f0f3f/README.md
[iphone-projectyml]: https://github.com/JohanSardar/apple-bridge-iphone/blob/1f25c547a4e74501f6d7820d92692ba5f19f0f3f/ios/project.yml#L10-L13
[iphone-lint]: https://github.com/JohanSardar/apple-bridge-iphone/blob/1f25c547a4e74501f6d7820d92692ba5f19f0f3f/packaging/dev/lint.py
[iphone-checkmodels]: https://github.com/JohanSardar/apple-bridge-iphone/blob/1f25c547a4e74501f6d7820d92692ba5f19f0f3f/packaging/dev/check-models.py
[iphone-testflight]: https://github.com/JohanSardar/apple-bridge-iphone/blob/1f25c547a4e74501f6d7820d92692ba5f19f0f3f/packaging/release/testflight.py
