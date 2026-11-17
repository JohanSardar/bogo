# Secretive (maxgoedjen/secretive)

A Mac app that keeps SSH keys in the Secure Enclave and can require Touch ID
or Apple Watch before each use. It is the closest match to BOGO's security
promise (a hardware-held key, a biometric check per use, a notification on
every access). It is also the model for a small, carefully kept Apple repo
whose README earns trust.

Studied at [`56c2f84`][sec-tree] on 2026-11-17. BOGO is judged at
[`bogo@8500392`][bogo-tree] and [`apple-bridge-iphone@1f25c54`][iphone-tree].

## 1. GitHub front page, badges and app-download links

**What we do now.** The [bogo README][bogo-readme] opens with "BOGO puts
approval between a named AI agent and protected MCP tools", then has a
paragraph of doc links and caveats and a 135-line quick start of
prerequisites and shell. It warns twice, correctly, that the software approver
isn't safe for real use. It has no badge, no
screenshot and no install path other than building from source.

**What they do.** The [README][sec-readme] has two badges (Test, Release) on
the title line, one sentence of purpose, and a light/dark `<picture>`
screenshot. A **Why?** section follows with one short paragraph and one image
each for Safer Storage, Access Control, Notifications and Smart Cards. Then
**Getting Started**: download from Releases or `brew install secretive`.
After that come the trust sections: **Auditable Build Process**, a note on
code signing and keychains, **Backups and Transfers** ("not able to be backed
up… just create a new set"), and **Security** with a link to the private
reporting route.

**Gap.**
- BOGO has no "Why?" section, which is the part that sells a security tool.
  Secretive explains each property in two sentences with a picture. BOGO's
  properties map one to one: the key is in the Secure Enclave and needs Face
  ID; you approve the exact call; every decision is in a verifiable log; a
  Lean proof covers the core. Each needs two sentences and, where possible, an
  image.
- BOGO says what it does not protect against, but only in the middle of the
  quick start. Secretive gives a limit like "keys can't be backed up" its own
  headed section. BOGO should do the same for "doesn't stop an agent using its
  own shell" (with the sandbox link) and "software approver is for demos only".
- No light/dark screenshot. Secretive's `<picture>` with
  `prefers-color-scheme` is the standard pattern, and happy uses it too.

## 2. Docs and help pages

**What we do now.** A strong set of technical pages: [threat model][bogo-threat],
[What is proved / What is assumed][bogo-proved], [configuration][bogo-config]
and [daemon operations][bogo-daemon], plus [ADRs][bogo-adr] and a
[glossary][bogo-glossary]. No FAQ, and nothing that answers "why should I trust
this?" in plain words.

**What they do.** Few docs, each with one clear job, at the repo root:
[FAQ.md][sec-faq] (15 questions, including
["Why should I trust you?"][sec-faq-trust] and
["I want to build Secretive from source"][sec-faq-build]),
[APP_CONFIG.md][sec-appconfig] (setting up SSH clients),
[LOCALIZING.md][sec-localizing], plus SECURITY and CONTRIBUTING.

**Gap.**
- BOGO's proof and threat docs go deeper than anything Secretive has, so
  depth isn't the gap. The gap is the plain-words layer above them. Add an FAQ
  with "Why should I trust BOGO?" that answers in three sentences and links to
  proved-and-assumed.md for the detail.
- No "set up your agent" page. Secretive's APP_CONFIG.md shows the exact
  snippet for each SSH client. BOGO's README embeds a Claude Code MCP config
  inside a Python heredoc. Pull that out into a `docs/agents.md` with a
  copy-paste config for Claude Code, Codex and other MCP clients.

## 3. How the app is separated from the server, MCP or CLI part

**What we do now.** Mac side: daemon, CLI, shim and relay as separate binaries
in one Rust workspace. Phone side: `ios/BogoKit` split into four targets
(`BogoModel`, `BogoCrypto`, `BogoRelay`, `BogoSection`) inside the host app's
[project.yml][iphone-projectyml-bogo]. That split is good, but no README in
either repo mentions it.

**What they do.** One repo, split by privilege. `Sources/Secretive` is the UI
app, `Sources/SecretAgent` is the background agent that signs, and parsing
runs in separate XPC services (`SecretAgentInputParser`,
`SecretiveCertificateParser`). Shared logic is in Swift packages under
[`Sources/Packages/Sources`][sec-packages] (`SecretKit`,
`SecureEnclaveSecretKit`, `SSHProtocolKit` and others), and the root
[Package.swift][sec-package-swift] exposes the testable libraries so
`swift test` runs on them.

**Gap.**
- BOGO's split is as clean as Secretive's but invisible. Add a short "Layout"
  section to each README: the bogo crates grouped by role (front, core, log,
  approvers, relay, tools), and in the iPhone repo the four BogoKit targets and
  what each owns.
- BogoKit isn't a Swift package with its own tests. Secretive puts its core
  logic in packages that `swift test` can run without the app. BogoModel and
  BogoCrypto could be built and tested the same way, which would close the
  "`make test` checks no BOGO code" gap described in the happy notes.

## 4. Server, MCP or CLI code: layout, lint, types, tests, logging, errors, dependencies

**What we do now.** Rust with `thiserror`, `unsafe` forbidden, Clippy at warn,
exact pins with written reasons (`rmcp = "=3.10.0"`, [Cargo.toml][bogo-cargo]).
A large dependency tree (Tokio, rmcp, serde, crypto crates) and no automated
audit. No CI.

**What they do.**
- **No third-party dependencies in the app**, as policy, written in
  [SECURITY.md § Dependencies][sec-security] and enforced in
  [CONTRIBUTING.md][sec-contributing-deps].
- **Tests on every push and PR** ([test.yml][sec-test]): the Xcode package
  tests and `swift test` on the root package, with a 10-minute timeout and
  read-only permissions.
- **CodeQL** on pushes and PRs to main and weekly, for Swift and for the
  workflows themselves
  ([codeql.yml][sec-codeql]).
- **A nightly build** ([nightly.yml][sec-nightly]).

**Gap.**
- BOGO can't have zero dependencies, but it can say which ones matter and why.
  Add a "Dependencies" section to SECURITY.md that lists the crates that touch
  keys, signatures and the wire (and the pinned `rmcp`), and run
  `cargo deny check advisories licenses` in CI.
- No static analysis in CI. At minimum run `cargo clippy -D warnings` and
  `cargo audit` on every PR, and add CodeQL if its Rust support fits. Today
  none of these runs automatically.

## 5. The iOS app: project layout, lint, tests, CI, signing, versioning, release

**What we do now.** Manual signing with the owner's team ID and bundle prefix
committed in [project.yml][iphone-projectyml]. Anyone else who builds it must
edit the project. Version 0.1, build 1, with per-run overrides in
[testflight.py][iphone-testflight]. No CI. No tags or GitHub releases.

**What they do.**
- **An OSS build path for outsiders.** [configure_team_id.sh][sec-configure]
  writes your team ID into a git-ignored `OpenSource.xcconfig`, and
  [Config.xcconfig][sec-config] falls back to the maintainer's IDs when that
  file is absent. Building from source never means editing tracked files.
- **Release on tags** ([release.yml][sec-release]): CI imports signing from
  secrets ([signing.sh][sec-signing]), runs the tests, archives, notarizes with
  `notarytool`, creates a **GitHub Artifact Attestation**, and fills a release
  template that links the exact build run and the attestation
  ([release.md template][sec-release-template]).
- **Version from the tag.** The release workflow strips the `v` from the tag
  and passes it to the build as the version.

**Gap.**
- An outsider can't build the iPhone app without editing `project.yml`.
  Copy Secretive's pattern: move `DEVELOPMENT_TEAM` and `bundleIdPrefix` into
  an xcconfig that includes an optional, git-ignored local override, and add a
  one-command script to create it.
- No link between a shipped build and its source. Secretive's release notes
  link the CI run and the attestation. BOGO asks people to trust a phone app
  with approvals, so each TestFlight build should map to a tag. Ideally it
  comes from a CI run with an attestation; if not, at least record the tag and
  build number.

## 6. Contributing, security, license and issue and PR templates

**What we do now.** MIT license only. Private vulnerability reporting is off.
No statement of principles a contributor can read (the iPhone repo's
[PRINCIPLES.md][iphone-principles] is written for the owner's agents: it opens
"Johan's rules win" and cites files that aren't in the repo).

**What they do.**
- [SECURITY.md][sec-security] opens with **Security Principles** (can't leak
  what it can't read, simplicity and auditability, no dependencies), then
  supported versions, then reporting through **GitHub private vulnerability
  reporting**, also linked from the README.
- [CONTRIBUTING.md][sec-contributing] is short and firm: no new dependencies,
  **no AI- or LLM-generated code** ([§ AI/LLM Policy][sec-contributing-ai]),
  no collaborator access because collaborators could read the signing secrets
  ([§ Collaborator Status][sec-contributing-collab]), and an opinionated scope.
- [CODE_OF_CONDUCT.md][sec-coc]. MIT [LICENSE][sec-license]. Community profile
  85%.

**Gap.**
- Add a SECURITY.md to bogo that opens with BOGO's own principles, as
  Secretive's does: the core decides; approvals bind to the exact bytes; the
  log is append-only and verifiable; what is proved and what is assumed. Then
  give supported versions and the reporting route, and turn on GitHub private
  vulnerability reporting on both repos.
- State an AI policy either way. Secretive bans AI-written code and ntfy
  welcomes it with review. BOGO is written mostly by agents, and a security
  reviewer will want to know that up front, along with how changes are
  reviewed and verified.
- Rewrite PRINCIPLES.md (or add a CONTRIBUTING.md beside it) for outside
  readers: drop the owner's name and the references to files that aren't in
  the repo.

## 7. Repo description and topics

**What we do now.** bogo has no description and no topics.

**What they do.** "Protect your SSH keys with your Mac's Secure Enclave",
website secretive.dev, four topics: `ssh`, `mac`, `secure-enclave`,
`security` ([repo page][sec-repo]).

**Gap.** Secretive shows a description can be eight words that name the asset
and the protection. A BOGO equivalent: "Approve your AI agent's tool calls on
your iPhone, with Face ID". Topics should include `secure-enclave` and
`security`, as here.

[sec-tree]: https://github.com/maxgoedjen/secretive/tree/56c2f841ed6400b43b60c4c94c36e172e3f7407d
[sec-repo]: https://github.com/maxgoedjen/secretive
[sec-readme]: https://github.com/maxgoedjen/secretive/blob/56c2f841ed6400b43b60c4c94c36e172e3f7407d/README.md
[sec-faq]: https://github.com/maxgoedjen/secretive/blob/56c2f841ed6400b43b60c4c94c36e172e3f7407d/FAQ.md
[sec-faq-trust]: https://github.com/maxgoedjen/secretive/blob/56c2f841ed6400b43b60c4c94c36e172e3f7407d/FAQ.md?plain=1#L43
[sec-faq-build]: https://github.com/maxgoedjen/secretive/blob/56c2f841ed6400b43b60c4c94c36e172e3f7407d/FAQ.md?plain=1#L47
[sec-appconfig]: https://github.com/maxgoedjen/secretive/blob/56c2f841ed6400b43b60c4c94c36e172e3f7407d/APP_CONFIG.md
[sec-localizing]: https://github.com/maxgoedjen/secretive/blob/56c2f841ed6400b43b60c4c94c36e172e3f7407d/LOCALIZING.md
[sec-packages]: https://github.com/maxgoedjen/secretive/tree/56c2f841ed6400b43b60c4c94c36e172e3f7407d/Sources/Packages/Sources
[sec-package-swift]: https://github.com/maxgoedjen/secretive/blob/56c2f841ed6400b43b60c4c94c36e172e3f7407d/Package.swift
[sec-security]: https://github.com/maxgoedjen/secretive/blob/56c2f841ed6400b43b60c4c94c36e172e3f7407d/SECURITY.md
[sec-contributing]: https://github.com/maxgoedjen/secretive/blob/56c2f841ed6400b43b60c4c94c36e172e3f7407d/CONTRIBUTING.md
[sec-contributing-deps]: https://github.com/maxgoedjen/secretive/blob/56c2f841ed6400b43b60c4c94c36e172e3f7407d/CONTRIBUTING.md?plain=1#L9
[sec-contributing-ai]: https://github.com/maxgoedjen/secretive/blob/56c2f841ed6400b43b60c4c94c36e172e3f7407d/CONTRIBUTING.md?plain=1#L13
[sec-contributing-collab]: https://github.com/maxgoedjen/secretive/blob/56c2f841ed6400b43b60c4c94c36e172e3f7407d/CONTRIBUTING.md?plain=1#L29
[sec-coc]: https://github.com/maxgoedjen/secretive/blob/56c2f841ed6400b43b60c4c94c36e172e3f7407d/CODE_OF_CONDUCT.md
[sec-license]: https://github.com/maxgoedjen/secretive/blob/56c2f841ed6400b43b60c4c94c36e172e3f7407d/LICENSE
[sec-test]: https://github.com/maxgoedjen/secretive/blob/56c2f841ed6400b43b60c4c94c36e172e3f7407d/.github/workflows/test.yml
[sec-codeql]: https://github.com/maxgoedjen/secretive/blob/56c2f841ed6400b43b60c4c94c36e172e3f7407d/.github/workflows/codeql.yml
[sec-nightly]: https://github.com/maxgoedjen/secretive/blob/56c2f841ed6400b43b60c4c94c36e172e3f7407d/.github/workflows/nightly.yml
[sec-release]: https://github.com/maxgoedjen/secretive/blob/56c2f841ed6400b43b60c4c94c36e172e3f7407d/.github/workflows/release.yml
[sec-signing]: https://github.com/maxgoedjen/secretive/blob/56c2f841ed6400b43b60c4c94c36e172e3f7407d/.github/scripts/signing.sh
[sec-release-template]: https://github.com/maxgoedjen/secretive/blob/56c2f841ed6400b43b60c4c94c36e172e3f7407d/.github/templates/release.md
[sec-configure]: https://github.com/maxgoedjen/secretive/blob/56c2f841ed6400b43b60c4c94c36e172e3f7407d/configure_team_id.sh
[sec-config]: https://github.com/maxgoedjen/secretive/blob/56c2f841ed6400b43b60c4c94c36e172e3f7407d/Sources/Config/Config.xcconfig
[bogo-tree]: https://github.com/JohanSardar/bogo/tree/8500392dc0d5845a2fbb489b595779db41378620
[bogo-readme]: https://github.com/JohanSardar/bogo/blob/8500392dc0d5845a2fbb489b595779db41378620/README.md
[bogo-threat]: https://github.com/JohanSardar/bogo/blob/8500392dc0d5845a2fbb489b595779db41378620/docs/threat-model.md
[bogo-proved]: https://github.com/JohanSardar/bogo/blob/8500392dc0d5845a2fbb489b595779db41378620/docs/proved-and-assumed.md
[bogo-config]: https://github.com/JohanSardar/bogo/blob/8500392dc0d5845a2fbb489b595779db41378620/docs/configuration.md
[bogo-daemon]: https://github.com/JohanSardar/bogo/blob/8500392dc0d5845a2fbb489b595779db41378620/docs/daemon.md
[bogo-adr]: https://github.com/JohanSardar/bogo/tree/8500392dc0d5845a2fbb489b595779db41378620/docs/adr
[bogo-glossary]: https://github.com/JohanSardar/bogo/blob/8500392dc0d5845a2fbb489b595779db41378620/GLOSSARY.md
[bogo-cargo]: https://github.com/JohanSardar/bogo/blob/8500392dc0d5845a2fbb489b595779db41378620/Cargo.toml
[iphone-tree]: https://github.com/JohanSardar/apple-bridge-iphone/tree/1f25c547a4e74501f6d7820d92692ba5f19f0f3f
[iphone-projectyml]: https://github.com/JohanSardar/apple-bridge-iphone/blob/1f25c547a4e74501f6d7820d92692ba5f19f0f3f/ios/project.yml#L3-L13
[iphone-projectyml-bogo]: https://github.com/JohanSardar/apple-bridge-iphone/blob/1f25c547a4e74501f6d7820d92692ba5f19f0f3f/ios/project.yml#L93-L129
[iphone-testflight]: https://github.com/JohanSardar/apple-bridge-iphone/blob/1f25c547a4e74501f6d7820d92692ba5f19f0f3f/packaging/release/testflight.py
[iphone-principles]: https://github.com/JohanSardar/apple-bridge-iphone/blob/1f25c547a4e74501f6d7820d92692ba5f19f0f3f/PRINCIPLES.md
