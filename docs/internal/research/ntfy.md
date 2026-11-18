# ntfy (binwiederhier/ntfy and binwiederhier/ntfy-ios)

A Go server plus Android, iOS and web apps. There is a free public instance at
ntfy.sh and self-hosting takes one command. The server and the iPhone app live
in separate repos, as BOGO's do, which makes ntfy the best model for linking
two repos.

Studied at [`ntfy@a098f52`][ntfy-tree] and [`ntfy-ios@96e659c`][ios-tree] on
2026-11-17. BOGO is judged at [`bogo@8500392`][bogo-tree] and
[`apple-bridge-iphone@1f25c54`][iphone-tree].

## 1. GitHub front page, badges and app-download links

**What we do now.** No badges in either README. The [bogo README][bogo-readme]
has no screenshot and no download link, and its first runnable step is
`cargo build` with test features.

**What they do.** The [README][ntfy-readme] title is the name plus what it does
("Send push notifications to your phone or desktop via PUT/POST"). Below it is
a row of badges: release, Go reference, tests, Go Report Card, Codecov, chat
and an uptime health check. One paragraph says what it is and that ntfy.sh is
free, followed by Google Play, F-Droid and App Store badges, a strip of five
screenshots (curl, web, phone), and a "Documentation" line of five links. A
TestFlight join link for the iOS beta is in the README too.

**Gap.**
- No CI badge, because there is no CI (see section 4). Once CI exists, put a
  test badge first. Add a release badge once there is a release.
- No screenshot strip. ntfy shows the sending side (curl) next to the phone.
  The BOGO equivalent is three images: an agent's tool call in the terminal,
  the phone review screen, and the call log after approval.
- The iPhone README needs a TestFlight join link once T20 produces one, as
  ntfy has.

## 2. Docs and help pages

**What we do now.** Markdown in [`docs/`][bogo-docs] with no index and no
site. Internal tickets and design rounds sit beside user pages. The
[configuration page][bogo-config] is thorough, but nothing tells a newcomer
which page to read second.

**What they do.** A MkDocs site published at ntfy.sh/docs, built from
[`docs/`][ntfy-docs] with a navigation tree in [mkdocs.yml][ntfy-mkdocs]:
Getting started, Publishing, Subscribing (phone, web, desktop, CLI, API),
Self-hosting (install, config), then FAQ, troubleshooting, known issues,
deprecations, development, contributing, AI policy, privacy and terms. The
[install page][ntfy-install] starts self-hosting with one line:
`docker run -p 80:80 -it binwiederhier/ntfy serve`. The iOS app has its own
build section in [develop.md][ntfy-develop-ios].

**Gap.**
- No navigation. Add a `docs/README.md` that mirrors ntfy's order: what BOGO
  is, quick start, configure, run the daemon, pair the phone, self-host the
  relay, threat model, troubleshooting, development.
- No troubleshooting or known-issues page. The README's "remains outstanding"
  paragraph (Face ID, App Attest, push and public relay not yet qualified)
  belongs on a known-issues page linked from the README, not in the README
  body.
- No "build the iPhone part" page in the bogo docs. ntfy keeps iOS build steps
  in the server repo's docs. BOGO should either do the same or link to the
  iPhone repo's build section.

## 3. How the app is separated from the server, MCP or CLI part

**What we do now.** The bogo README mentions the iPhone approver but doesn't
link to it. The [iPhone README][iphone-readme] never says BOGO. Both repos have
issues turned on, so a newcomer doesn't know where to report a phone-side bug.

**What they do.** The server README's second paragraph links the
[Android app repo][ntfy-android] and the [iOS app repo][ntfy-ios-repo], each
next to its store link. The [ntfy-ios README][ios-readme] opens with "This is
the iOS app for ntfy" and links back. **Issues are turned off on ntfy-ios**:
its README sends people to the main repo's issues, so all bug reports land in
one place. The iOS repo is small: app source, a notification extension and
three docs (feature parity, getting started, technical limitations). For
self-hosted servers, the iOS app gets instant pushes because the server
forwards a poll request through ntfy.sh; this is documented in
[config.md § iOS instant notifications][ntfy-ios-upstream].

**Gap.**
- Add the cross-links both ways: the bogo README names and links
  `apple-bridge-iphone` as "the iPhone app (BOGO tab)", and the iPhone README
  says in its first paragraph that the app contains the BOGO approver and links
  back.
- Decide where BOGO issues go. Following ntfy, file all BOGO bugs on
  `JohanSardar/bogo` and say so in the iPhone README. Issues there can't be
  turned off, because the repo also carries Apple Bridge.
- BOGO's relay and APNs ownership ([ADR 0003][bogo-adr3]) is the same problem
  ntfy documents for self-hosters. BOGO has no user-facing page that explains
  who sends the push, what the relay sees, and whether a self-hosted relay can
  still wake the phone. Write one, in the style of ntfy's section.

## 4. Server, MCP or CLI code: layout, lint, types, tests, logging, errors, dependencies

**What we do now.** No CI at all. Workspace Clippy lints, with two crates not
opting in ([Cargo.toml][bogo-cargo]). No `cargo fmt --check`, `cargo deny` or
`cargo audit` step. 17 Rust test functions, plus examples, Lean proofs and
vectors. Logging is split between `tracing` and `eprintln!`. No dependabot.

**What they do.**
- **One `make check`** runs tests, format check, `go vet`, golint, staticcheck,
  web lint and template checks ([Makefile#L294][ntfy-make-check]).
- **CI on every push and PR**, in three parallel parts against a real Postgres
  service ([test.yaml][ntfy-test-yaml]), with actions pinned to SHAs.
- **Dependabot** weekly for Go modules, npm and GitHub Actions, with a 7-day
  cooldown and grouped action updates ([dependabot.yml][ntfy-dependabot]).
- **Its own log package** with levels, structured fields and tests
  ([log/log.go][ntfy-log]).
- **Releases** on tags: [release.yaml][ntfy-release] runs `make release`,
  which uses GoReleaser ([.goreleaser.yml][ntfy-goreleaser]) to build
  binaries, Debian/RPM packages and Docker images.

**Gap.**
- Add one `make check` (or `cargo xtask check`) for bogo that runs
  `cargo fmt --check`, `cargo clippy --workspace --all-targets -D warnings`,
  `cargo test --workspace`, the Lean build and the vector check, and run it in
  CI on every PR.
- Add dependabot (or Renovate) for Cargo and GitHub Actions, keeping the exact
  `rmcp` pin and its reason ([Cargo.toml][bogo-cargo]) as an ignore rule.
- Add `cargo deny` for licenses and advisories. For a security tool, an
  advisory check in CI is expected.
- Release binaries on tags. Today there are no releases or tags, so the only
  way in is building from source with Rust, Lean, Node and Python.

## 5. The iOS app: project layout, lint, tests, CI, signing, versioning, release

**What we do now.** XcodeGen project, custom lint, `make test` that skips
BogoKit, no CI, manual signing with the team ID committed, version 0.1
overridden per run by [testflight.py][iphone-testflight].

**What they do.** ntfy-ios is not a model here. It has an Xcode project with
the team ID and `MARKETING_VERSION` committed in
[project.pbxproj][ios-pbxproj], no CI, no tests and no lint. Its strength is
scope and docs: three short docs say how to build, which features match
Android, and what iOS can't do ([TECHNICAL_LIMITATIONS.md][ios-limits]).

**Gap.**
- BOGO's iPhone side is already ahead of ntfy-ios on tooling (lint, model
  checks, a release script). The gap is the "technical limitations" page: what
  iOS won't let the BOGO tab do (background push timing, Face ID in the
  Simulator, App Attest on Simulator). These facts are scattered through the
  [iPhone app guide][iphone-ios-readme] and BOGO's verification records.
  Collect them on one page.

## 6. Contributing, security, license and issue and PR templates

**What we do now.** MIT license in both repos. Nothing else. Private
vulnerability reporting is off on both.

**What they do.**
- [SECURITY.md][ntfy-security]: supported versions (latest only) and a private
  email address for reports.
- [CODE_OF_CONDUCT.md][ntfy-coc] (community profile 100%).
- [contributing.md][ntfy-contributing] in the docs site: code, translations,
  docs, bugs.
- [AI policy][ntfy-ai-policy]: the maintainer writes code with Claude and Codex
  and reviews every line. AI-assisted PRs are welcome, but "don't make me the
  first human to read your code".
- Four issue templates (bug, enhancement, tech support, question). The bug
  template asks which component (server, Android, iOS, web) and how to get logs
  from each ([1_bug_report.md][ntfy-bug]).
- Dual licensed, Apache-2.0 [LICENSE][ntfy-license] and GPLv2. ntfy-ios is
  MIT.

**Gap.**
- No SECURITY.md in a security product. Add one to bogo with a private route
  (turn on GitHub private vulnerability reporting, or give an address) and
  "latest main only" as the supported version. The iPhone repo's SECURITY.md
  should point to the same route for BOGO-tab issues.
- No AI policy. Both BOGO repos are written mostly by agents, under human
  review and with recorded evidence. Say so plainly, as ntfy does, so readers
  don't have to guess.
- A bug template that asks "daemon, CLI, relay or iPhone tab?" and says how to
  get logs from each (`$BOGO_HOME` daemon log, `bogo log verify`, the app's
  Diagnostics tab).

## 7. Repo description and topics

**What we do now.** bogo has no description and no topics. The iPhone repo's
description doesn't mention BOGO, and it has no topics.

**What they do.** ntfy: "Send push notifications to your phone or desktop
using PUT/POST", website ntfy.sh, seven topics (`push-notifications`,
`rest-api`, `pubsub`, `curl` and others). ntfy-ios: "iOS App for ntfy". Each
description names its sibling ([ntfy repo][ntfy-repo],
[ntfy-ios repo][ntfy-ios-repo]).

**Gap.** The iPhone repo's description should name both of its jobs, for
example "Apple Bridge for iPhone, and the BOGO approver for AI agent tool
calls". bogo needs the description and topics listed in the summary.

[ntfy-tree]: https://github.com/binwiederhier/ntfy/tree/a098f52f6b643e8986f261f7cd9796e6bd1405d1
[ios-tree]: https://github.com/binwiederhier/ntfy-ios/tree/96e659caf4b14405e201db6f5e4b2c6947c81643
[ntfy-repo]: https://github.com/binwiederhier/ntfy
[ntfy-ios-repo]: https://github.com/binwiederhier/ntfy-ios
[ntfy-android]: https://github.com/binwiederhier/ntfy-android
[ntfy-readme]: https://github.com/binwiederhier/ntfy/blob/a098f52f6b643e8986f261f7cd9796e6bd1405d1/README.md
[ntfy-docs]: https://github.com/binwiederhier/ntfy/tree/a098f52f6b643e8986f261f7cd9796e6bd1405d1/docs
[ntfy-mkdocs]: https://github.com/binwiederhier/ntfy/blob/a098f52f6b643e8986f261f7cd9796e6bd1405d1/mkdocs.yml#L83-L111
[ntfy-install]: https://github.com/binwiederhier/ntfy/blob/a098f52f6b643e8986f261f7cd9796e6bd1405d1/docs/install.md?plain=1#L372
[ntfy-develop-ios]: https://github.com/binwiederhier/ntfy/blob/a098f52f6b643e8986f261f7cd9796e6bd1405d1/docs/develop.md?plain=1#L435
[ntfy-ios-upstream]: https://github.com/binwiederhier/ntfy/blob/a098f52f6b643e8986f261f7cd9796e6bd1405d1/docs/config.md?plain=1#L1645
[ntfy-make-check]: https://github.com/binwiederhier/ntfy/blob/a098f52f6b643e8986f261f7cd9796e6bd1405d1/Makefile#L294
[ntfy-test-yaml]: https://github.com/binwiederhier/ntfy/blob/a098f52f6b643e8986f261f7cd9796e6bd1405d1/.github/workflows/test.yaml
[ntfy-dependabot]: https://github.com/binwiederhier/ntfy/blob/a098f52f6b643e8986f261f7cd9796e6bd1405d1/.github/dependabot.yml
[ntfy-log]: https://github.com/binwiederhier/ntfy/blob/a098f52f6b643e8986f261f7cd9796e6bd1405d1/log/log.go
[ntfy-release]: https://github.com/binwiederhier/ntfy/blob/a098f52f6b643e8986f261f7cd9796e6bd1405d1/.github/workflows/release.yaml
[ntfy-goreleaser]: https://github.com/binwiederhier/ntfy/blob/a098f52f6b643e8986f261f7cd9796e6bd1405d1/.goreleaser.yml
[ntfy-security]: https://github.com/binwiederhier/ntfy/blob/a098f52f6b643e8986f261f7cd9796e6bd1405d1/SECURITY.md
[ntfy-coc]: https://github.com/binwiederhier/ntfy/blob/a098f52f6b643e8986f261f7cd9796e6bd1405d1/CODE_OF_CONDUCT.md
[ntfy-contributing]: https://github.com/binwiederhier/ntfy/blob/a098f52f6b643e8986f261f7cd9796e6bd1405d1/docs/contributing.md
[ntfy-ai-policy]: https://github.com/binwiederhier/ntfy/blob/a098f52f6b643e8986f261f7cd9796e6bd1405d1/docs/ai-policy.md
[ntfy-bug]: https://github.com/binwiederhier/ntfy/blob/a098f52f6b643e8986f261f7cd9796e6bd1405d1/.github/ISSUE_TEMPLATE/1_bug_report.md
[ntfy-license]: https://github.com/binwiederhier/ntfy/blob/a098f52f6b643e8986f261f7cd9796e6bd1405d1/LICENSE
[ios-readme]: https://github.com/binwiederhier/ntfy-ios/blob/96e659caf4b14405e201db6f5e4b2c6947c81643/README.md
[ios-pbxproj]: https://github.com/binwiederhier/ntfy-ios/blob/96e659caf4b14405e201db6f5e4b2c6947c81643/ntfy.xcodeproj/project.pbxproj#L643-L661
[ios-limits]: https://github.com/binwiederhier/ntfy-ios/blob/96e659caf4b14405e201db6f5e4b2c6947c81643/docs/TECHNICAL_LIMITATIONS.md
[bogo-tree]: https://github.com/JohanSardar/bogo/tree/8500392dc0d5845a2fbb489b595779db41378620
[bogo-readme]: https://github.com/JohanSardar/bogo/blob/8500392dc0d5845a2fbb489b595779db41378620/README.md
[bogo-docs]: https://github.com/JohanSardar/bogo/tree/8500392dc0d5845a2fbb489b595779db41378620/docs
[bogo-config]: https://github.com/JohanSardar/bogo/blob/8500392dc0d5845a2fbb489b595779db41378620/docs/configuration.md
[bogo-adr3]: https://github.com/JohanSardar/bogo/blob/8500392dc0d5845a2fbb489b595779db41378620/docs/adr/0003-bogo-owns-its-relay-and-push.md
[bogo-cargo]: https://github.com/JohanSardar/bogo/blob/8500392dc0d5845a2fbb489b595779db41378620/Cargo.toml
[iphone-tree]: https://github.com/JohanSardar/apple-bridge-iphone/tree/1f25c547a4e74501f6d7820d92692ba5f19f0f3f
[iphone-readme]: https://github.com/JohanSardar/apple-bridge-iphone/blob/1f25c547a4e74501f6d7820d92692ba5f19f0f3f/README.md
[iphone-ios-readme]: https://github.com/JohanSardar/apple-bridge-iphone/blob/1f25c547a4e74501f6d7820d92692ba5f19f0f3f/ios/README.md
[iphone-testflight]: https://github.com/JohanSardar/apple-bridge-iphone/blob/1f25c547a4e74501f6d7820d92692ba5f19f0f3f/packaging/release/testflight.py
