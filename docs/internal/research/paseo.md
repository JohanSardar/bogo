# Paseo (getpaseo/paseo)

Runs coding agents from a desktop app, phone, web and CLI. A daemon on your
machine runs the agents, and clients reach it directly or through an
end-to-end encrypted relay. It is a busy one-maintainer project built with
heavy agent help, the same way BOGO is built, so its process docs are the most
directly reusable.

Studied at [`c8e3d67`][pa-tree] on 2026-11-17. BOGO is judged at
[`bogo@8500392`][bogo-tree] and [`apple-bridge-iphone@1f25c54`][iphone-tree].

## 1. GitHub front page, badges and app-download links

**What we do now.** The [bogo README][bogo-readme]: an icon, a paragraph, three
doc links, then a long walkthrough. No badges, no screenshot, no install
command. The [iPhone README][iphone-readme] has none of these either.

**What they do.** The [README][pa-readme] has a centred logo and name, links to
README translations (English, 简体中文, 日本語, 한국어), and badges for stars,
latest release, X, Discord and Reddit. Then a one-line pitch, a desktop
screenshot and a mobile screenshot, an eight-bullet feature list, and
**Getting Started**: prerequisites, a desktop download, `npm install -g
@getpaseo/cli && paseo` for servers, and a Docker one-liner. After that: CLI
examples, an SDK snippet, a [package map][pa-readme-dev] under
**Development**, and [Related projects][pa-readme-related] (the separate
`paseo-relay` repo and a VS Code extension).

**Gap.**
- BOGO has no install command. Paseo offers three routes in the first screen
  (app download, `npm -g`, Docker). BOGO's only route is a source build that
  needs Rust, Lean, Node, Python and Claude Code. Even one route, such as a
  release tarball or `brew install`, would change the first impression.
- No package map. Paseo's **Development** section lists each package in one
  line. The bogo README should list its crates grouped by role and say which
  binaries a user runs (`bogo`, `bogo-daemon`, `bogo-shim`, `bogo-relay`).
- No "Related projects" section. That is where the iPhone repo link belongs,
  and the iPhone README needs the mirror.

## 2. Docs and help pages

**What we do now.** A [`docs/`][bogo-docs] folder mixing user pages,
architecture, 22 tickets, 15 design and critique rounds, evidence and
verification logs. There is a good [glossary][bogo-glossary] at the root.

**What they do.** Two doc trees. [`public-docs/`][pa-public-docs] feeds the
website (paseo.sh/docs) and is translated. [`docs/`][pa-docs] is the
contributor reference: [architecture.md][pa-arch] (with an ASCII system
diagram), [development.md][pa-dev] (including a
[Daemon logs][pa-dev-logs] section), [coding-standards.md][pa-standards],
[glossary.md][pa-glossary] ("Authoritative terminology. UI label wins."),
[release.md][pa-release] and others. QA screenshots go in `docs/qa-evidence/`
and refactor notes in `docs/refactors/`, so process files don't crowd the
reference pages. A [CHANGELOG.md][pa-changelog] lists every release with PR
links and contributor credit.

**Gap.**
- No split between user docs and contributor docs. Copy Paseo's two trees:
  BOGO's user pages (configuration, daemon, threat model, proved/assumed) in
  one place, and ARCHITECTURE, SPEC, RELAY-PROTOCOL, ADRs, tickets and
  verification in another. Tickets and design rounds go in their own
  subfolder, as Paseo does with `qa-evidence/` and `refactors/`.
- No changelog. Paseo's CHANGELOG is grouped Added/Fixed with PR links. BOGO
  has none, and the iPhone repo has none for its BOGO tab. Start a
  CHANGELOG.md in bogo at the first tagged version.
- No development guide. The bogo README quick start doubles as the dev
  setup. A `docs/development.md` should cover building, running the checks, the
  Lean toolchain, and where logs go, as Paseo's does.

## 3. How the app is separated from the server, MCP or CLI part

**What we do now.** Mac side in bogo, phone side in apple-bridge-iphone
(`ios/BogoKit`), relay in bogo (`crates/bogo-relay`) with a runbook in
`deploy/relay`. Neither README links to the other repo.

**What they do.** A monorepo for everything a user runs (`packages/server`,
`packages/app`, `packages/cli`, `packages/desktop`, `packages/relay`), with
the shared wire contract in `packages/protocol` and a typed client SDK in
`packages/client`. The production relay is a **separate repo**, linked from
the README under Related projects. [SECURITY.md][pa-security] explains the
daemon/client/relay split and the relay threat model in user terms:
what the relay can see (IPs, timing, sizes, handshake public keys) and what it
can't.

**Gap.**
- BOGO's relay threat model is spread across [ARCHITECTURE.md][bogo-arch],
  [RELAY-PROTOCOL.md][bogo-relay-proto] and [threat-model.md][bogo-threat].
  Paseo puts "what the relay sees, and why it can't attack you" in one short,
  plain section. BOGO should have the same in its threat model or SECURITY.md:
  what bogo.eilmer.si sees, what a compromised relay can and can't do, and
  what the pairing code is trusted for.
- Paseo shows that putting a relay in its own repo is fine as long as the main
  README links it. BOGO's mirror case is the iPhone part living in another
  product's repo, which needs the link even more.

## 4. Server, MCP or CLI code: layout, lint, types, tests, logging, errors, dependencies

**What we do now.** Clippy at warn (two crates not opted in), `unsafe`
forbidden, `thiserror` errors, 17 Rust test functions plus examples, Lean
proofs and vectors. Logging split between `tracing` and `eprintln!`. No
formatter check, no pre-commit hook, no CI, no lockfile or advisory check.

**What they do.**
- **[CI][pa-ci] on every PR**: format check, lockfile lint, `npm audit
  signatures`, oxlint (plus tests for their own lint rules), relay, plugin,
  protocol and client tests, and a typecheck job across all packages.
- **[Pre-commit hooks][pa-lefthook]** (lefthook): format, lint and typecheck on
  staged files.
- **Lint config** in [.oxlintrc.json][pa-oxlint], with correctness, suspicious
  and perf rules set to `error`, and a custom plugin for project rules.
- **One logger**, pino, configured in
  [server/logger.ts][pa-logger], with levels and pretty or JSON output.
- **Written coding standards** with sections on types, errors, naming and
  structure ([coding-standards.md][pa-standards]).
- 650 `*.test.ts` files under `packages/server/src`.

**Gap.**
- No pre-commit or CI gate. Add `cargo fmt --check`, `cargo clippy -D
  warnings` and the tests to CI, and optionally a lefthook or `pre-commit`
  config that runs the fast ones locally.
- Bring the two crates outside workspace lints (`bogo-core-sys`,
  `bogo-verify`) in, or note why they stay out.
- One logging story with levels and a JSON option, documented in a "logs"
  section of `docs/daemon.md`, as Paseo's development.md does.
- No written coding standards for the Rust side. The iPhone repo has
  [PRINCIPLES.md][iphone-principles], but bogo has nothing equivalent for
  contributors.

## 5. The iOS app: project layout, lint, tests, CI, signing, versioning, release

**What we do now.** XcodeGen [project.yml][iphone-projectyml] with version 0.1,
build 1, manual signing. `make lint` checks for comments and that sources
parse. `make test` checks Apple Bridge models but no BogoKit file. No CI.
[testflight.py][iphone-testflight] uploads from the owner's Mac when given
`--version` and `--build`.

**What they do.**
- **One version for every package**, kept in sync by script
  ([package.json `version` scripts][pa-pkg-version]). The native build number
  is derived from that version by
  [native-release-version.js][pa-native-version], which has its own test file.
- **TestFlight from a tag.** Pushing `v*-beta.*` runs an EAS workflow that
  builds iOS and sends it to the external "Paseo Beta" TestFlight group
  ([release-ios-beta.yml][pa-eas-beta]).
- **A written two-step release** ([release.md][pa-release]): the agent prepares
  (checks green, version chosen, changelog drafted and shown), and the user
  says "go ahead" before anything is published.
- A weak spot: [packages/app/README.md][pa-app-readme] is still the Expo
  template's boilerplate.

**Gap.**
- The TestFlight build number and version are typed by hand each run. Derive
  them from one source (a tag, or one version file), as Paseo does, so a build
  always maps back to a commit.
- BOGO's release steps for the phone aren't written down for an outside
  reader. Paseo's release.md (agent prepares, human approves, tag publishes)
  fits BOGO's agent-driven workflow almost exactly. T20 is changing TestFlight
  right now, so record the result as a short `docs/release.md` in the iPhone
  repo.

## 6. Contributing, security, license and issue and PR templates

**What we do now.** MIT license only, in both repos. No CONTRIBUTING, SECURITY,
templates or code of conduct. Private vulnerability reporting is off.

**What they do.**
- [CONTRIBUTING.md][pa-contributing]: philosophy, build a plugin first, how to
  report bugs, discuss features in Discussions, and
  ["Pull requests are closed by default"][pa-contrib-closed], with a
  required [QA evidence][pa-contrib-qa] section ("I need to see that *you*
  tested this").
- [PR template][pa-pr-template]: linked issue, type, reasoning, goals,
  non-goals, QA, and a checklist (typecheck, lint, format, tests).
- [Bug report form][pa-bug] (YAML issue form with required fields), and a
  [config.yml][pa-issue-config] that turns off blank issues and sends feature
  requests to Discussions or Discord.
- [SECURITY.md][pa-security]: architecture, relay threat model, local daemon
  trust boundary, and [a private email route][pa-security-report] for
  vulnerabilities.
- Apache-2.0 [LICENSE][pa-license].

**Gap.**
- BOGO already works the way Paseo's PR template asks: every change carries
  verification evidence (see `docs/verification/`). But nothing tells an
  outside contributor that. Add a PR template that asks for the same evidence
  BOGO's own tickets require (what you ran, what you saw).
- Add a bug report form with a component dropdown (daemon, CLI, MCP front,
  relay, iPhone tab), and turn off blank issues.
- Paseo's SECURITY.md is a threat model plus a reporting route. BOGO has the
  threat model already ([threat-model.md][bogo-threat]) but no SECURITY.md and
  no route. Add a SECURITY.md that links the threat model and gives the route.

## 7. Repo description and topics

**What we do now.** bogo: empty description, no topics, no website. iPhone:
Apple Bridge description, no topics.

**What they do.** "Orchestrate multiple coding agents from desktop and
mobile", website paseo.sh, 15 topics, including `agents`, `claude-code`,
`codex`, `mobile`, `ios` and `orchestration` ([repo page][pa-repo]).

**Gap.** Paseo's topics name the agents it works with (`claude-code`, `codex`,
`opencode`) so people searching for those find it. BOGO works with any MCP
client and documents Claude Code, so `claude-code`, `mcp` and
`model-context-protocol` belong in its topics.

[pa-tree]: https://github.com/getpaseo/paseo/tree/c8e3d67e29ab39300f325b3a230790a716729658
[pa-repo]: https://github.com/getpaseo/paseo
[pa-readme]: https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/README.md
[pa-readme-dev]: https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/README.md?plain=1#L191
[pa-readme-related]: https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/README.md?plain=1#L225
[pa-public-docs]: https://github.com/getpaseo/paseo/tree/c8e3d67e29ab39300f325b3a230790a716729658/public-docs
[pa-docs]: https://github.com/getpaseo/paseo/tree/c8e3d67e29ab39300f325b3a230790a716729658/docs
[pa-arch]: https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/docs/architecture.md
[pa-dev]: https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/docs/development.md
[pa-dev-logs]: https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/docs/development.md?plain=1#L350
[pa-standards]: https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/docs/coding-standards.md
[pa-glossary]: https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/docs/glossary.md
[pa-release]: https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/docs/release.md
[pa-changelog]: https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/CHANGELOG.md
[pa-security]: https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/SECURITY.md
[pa-security-report]: https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/SECURITY.md?plain=1#L95
[pa-ci]: https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/.github/workflows/ci.yml
[pa-lefthook]: https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/lefthook.yml
[pa-oxlint]: https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/.oxlintrc.json
[pa-logger]: https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/packages/server/src/server/logger.ts
[pa-pkg-version]: https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/package.json#L101-L107
[pa-native-version]: https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/packages/app/native-release-version.js
[pa-eas-beta]: https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/packages/app/.eas/workflows/release-ios-beta.yml
[pa-app-readme]: https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/packages/app/README.md
[pa-contributing]: https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/CONTRIBUTING.md
[pa-contrib-closed]: https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/CONTRIBUTING.md?plain=1#L58
[pa-contrib-qa]: https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/CONTRIBUTING.md?plain=1#L77
[pa-pr-template]: https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/.github/PULL_REQUEST_TEMPLATE.md
[pa-bug]: https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/.github/ISSUE_TEMPLATE/bug-report.yml
[pa-issue-config]: https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/.github/ISSUE_TEMPLATE/config.yml
[pa-license]: https://github.com/getpaseo/paseo/blob/c8e3d67e29ab39300f325b3a230790a716729658/LICENSE
[bogo-tree]: https://github.com/JohanSardar/bogo/tree/8500392dc0d5845a2fbb489b595779db41378620
[bogo-readme]: https://github.com/JohanSardar/bogo/blob/8500392dc0d5845a2fbb489b595779db41378620/README.md
[bogo-docs]: https://github.com/JohanSardar/bogo/tree/8500392dc0d5845a2fbb489b595779db41378620/docs
[bogo-glossary]: https://github.com/JohanSardar/bogo/blob/8500392dc0d5845a2fbb489b595779db41378620/GLOSSARY.md
[bogo-arch]: https://github.com/JohanSardar/bogo/blob/8500392dc0d5845a2fbb489b595779db41378620/docs/ARCHITECTURE.md
[bogo-relay-proto]: https://github.com/JohanSardar/bogo/blob/8500392dc0d5845a2fbb489b595779db41378620/docs/RELAY-PROTOCOL.md
[bogo-threat]: https://github.com/JohanSardar/bogo/blob/8500392dc0d5845a2fbb489b595779db41378620/docs/threat-model.md
[iphone-tree]: https://github.com/JohanSardar/apple-bridge-iphone/tree/1f25c547a4e74501f6d7820d92692ba5f19f0f3f
[iphone-readme]: https://github.com/JohanSardar/apple-bridge-iphone/blob/1f25c547a4e74501f6d7820d92692ba5f19f0f3f/README.md
[iphone-principles]: https://github.com/JohanSardar/apple-bridge-iphone/blob/1f25c547a4e74501f6d7820d92692ba5f19f0f3f/PRINCIPLES.md
[iphone-projectyml]: https://github.com/JohanSardar/apple-bridge-iphone/blob/1f25c547a4e74501f6d7820d92692ba5f19f0f3f/ios/project.yml#L10-L13
[iphone-testflight]: https://github.com/JohanSardar/apple-bridge-iphone/blob/1f25c547a4e74501f6d7820d92692ba5f19f0f3f/packaging/release/testflight.py
