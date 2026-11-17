# Bark (Finb/Bark and Finb/bark-server)

An iOS app with a separate, self-hostable Go server that forwards push
requests to APNs. Like BOGO, the app and the server are in two repos. Unlike
BOGO, the **app** repo is the front door and hosts the docs for both. The
server also has an MCP endpoint, so it is the nearest example of an MCP part
kept apart from an iPhone app.

Studied at [`Bark@a5c1cf5`][bark-tree] and [`bark-server@a365aa9`][srv-tree] on
2026-11-17. BOGO is judged at [`bogo@8500392`][bogo-tree] and
[`apple-bridge-iphone@1f25c54`][iphone-tree].

## 1. GitHub front page, badges and app-download links

**What we do now.** The [bogo README][bogo-readme] has no download link, no
screenshot and no badges. The [iPhone README][iphone-readme] has no App Store
or TestFlight link.

**What they do.** The app [README][bark-readme] starts with a language switch
(English, 中文, 日本語), a three-sentence pitch, then a **Download** section with
the App Store badge, a **Documentation** link and a feedback channel. Next
comes **Usage**, which shows the whole product in two steps with screenshots:
copy your URL from the app, then `curl` it. The
[server README][srv-readme] carries Docker automated-build, image-size and
license badges right above its `docker run` line.

**Gap.**
- BOGO has no "see it work in two steps" section. Bark shows the user's path
  (copy, then curl) before any setup. BOGO's equivalent would be: the agent
  asks to write a file, the phone shows the exact call, you slide to approve.
  Put that, with screenshots, above the build instructions.
- The README never sends people to the phone app. Add a Download/TestFlight
  section once T20 ships.

## 2. Docs and help pages

**What we do now.** Markdown in [`docs/`][bogo-docs], no site, no sidebar,
with user docs mixed in among internal tickets and design rounds.

**What they do.** A docsify site at bark.day.app, served from the app repo's
[`docs/`][bark-docs]. Its [English sidebar][bark-sidebar] splits pages into
**App** (tutorial, parameters, encryption, FAQ) and **Server** (deploy, batch
push, build, certificate), with privacy last. The [docs home][bark-docs-home]
links both repos in its first lines. The server repo keeps only API-level docs
([API_V2.md][srv-apiv2], [MCP.md][srv-mcp]).

**Gap.**
- BOGO's docs aren't split by audience. Copy Bark's split in a
  `docs/README.md`: **Use BOGO** (quick start, configure, approve from your
  phone, FAQ), **Run BOGO** (daemon, relay self-host, upgrades), **How it
  works** (threat model, proved/assumed, architecture, relay protocol).
  Internal tickets and design rounds go in a separate folder.
- There is no FAQ. Bark's [FAQ][bark-faq] answers the questions people actually
  ask. BOGO's verification records already hold the answers to several (why
  the soft approver isn't safe, why the shell can bypass MCP, what happens if
  the phone is offline). Lift them into a FAQ.

## 3. How the app is separated from the server, MCP or CLI part

**What we do now.** The iPhone approver lives inside a host app
(`ios/BogoKit` in apple-bridge-iphone) and the server side lives in bogo.
Neither README links to the other. The MCP front is a crate (`bogo-front`)
inside the daemon.

**What they do.**
- The app README doesn't link bark-server directly. The **docs home** does,
  with one line per repo: "[Bark] is a fully open source iOS APP…",
  "[bark-server] is a fully open source Bark service backend…"
  ([docs/en-us/README.md][bark-docs-home]).
- The [server README][srv-readme] opens by linking the app: "[Bark] is an iOS
  App which allows you to push customed notifications to your iPhone."
- The server's [deploy page][bark-deploy] (in the app repo's docs) links the
  server's releases page and its Docker image.
- The server's MCP endpoint is one file, [`route_mcp.go`][srv-mcp-route], with
  its own doc ([docs/MCP.md][srv-mcp]).

**Gap.**
- Each README should say in its first lines what the other repo is and link
  it. The bogo README describes the iPhone part without a link, and the iPhone
  README doesn't mention BOGO.
- BOGO's MCP surface (which tools the agent sees, what a denied or pending
  call returns) is documented only inside ARCHITECTURE.md. Bark gives its MCP
  endpoint a short page of its own. BOGO should have a `docs/mcp.md` written for
  someone wiring up an agent.

## 4. Server, MCP or CLI code: layout, lint, types, tests, logging, errors, dependencies

**What we do now.** A 23-crate workspace with typed errors (`thiserror`),
Clippy lints, and runnable examples, Lean proofs and vectors as the main
checks. 17 Rust test functions. No CI. No release artifacts, so the only
install is building from source.

**What they do.** bark-server is a flat Go module: one file per route
(`route_push.go`, `route_register.go`, `route_mcp.go`…), with packages for
`apns` and `database`. It has one test file (`push_test.go`). Its
[CI][srv-ci] runs on tags only and builds multi-arch Docker images plus
release binaries with go-task. Install is one line:
`docker run -dt --name bark -p 8080:8080 -v ./bark-data:/data finab/bark-server`
([README][srv-readme]), or download a binary from Releases.

**Gap.**
- bark-server is not a code-quality model; BOGO's code is more rigorous. The
  gap is **distribution**. Bark gives a self-hoster a Docker image and binaries
  per tag. BOGO's relay has a systemd unit and a runbook for one specific VM
  ([deploy/relay/README.md][bogo-relay-readme]), but no image and no release
  binary. If self-hosting the relay is meant to be supported, publish a
  container image and binaries on tags. If it isn't, say so in the README.
- The relay runbook names the operator's cloud project, VM and IP at the top of
  `deploy/relay/README.md`. Bark keeps operator-specific detail out of user
  docs. Split the runbook (operator-only) from a generic "run your own relay"
  page.

## 5. The iOS app: project layout, lint, tests, CI, signing, versioning, release

**What we do now.** XcodeGen [project.yml][iphone-projectyml] (manual signing,
team ID committed, version 0.1). Custom no-comments lint. `make test` covers
none of BogoKit. No CI. TestFlight upload is a local script
([testflight.py][iphone-testflight]) that takes version and build as
arguments.

**What they do.**
- A [Tests workflow][bark-tests] runs on every push and PR to master: macOS
  runner, latest stable Xcode, CocoaPods cache, then `fastlane tests`.
- A [TestFlight workflow][bark-testflight] runs by manual dispatch, with store
  version and build number as inputs. It calls the fastlane `beta` lane, which
  sets the version, uses **match** for signing (read-only on CI) and uploads
  with an App Store Connect API key ([Fastfile][bark-fastfile]). dSYMs are
  uploaded as an artifact.
- The Xcode project keeps the team ID committed, like ours.
- [BarkTests][bark-testsdir] is small (two test files), but CI runs it.

**Gap.**
- No CI on the iPhone repo. Bark shows the minimum: one macOS workflow that
  builds and tests on every PR.
- Releases happen only from the owner's Mac. Bark's release runs in CI with
  signing from `match` and the version as an input, so anyone can see which
  commit became which build. Moving `testflight.py` into a manual-dispatch
  workflow would give the same audit trail. If credentials must stay local,
  write down which commit each TestFlight build came from (tags).

## 6. Contributing, security, license and issue and PR templates

**What we do now.** MIT in both repos, nothing else.

**What they do.** Both repos are MIT ([app LICENSE][bark-license],
[server LICENSE][srv-license]). Neither has CONTRIBUTING, SECURITY or
templates (community profile 42% each). The docs have a
[privacy page][bark-privacy].

**Gap.** Bark is no better than BOGO here, so it isn't the model. Its privacy
page is worth copying: one page saying what the server stores and logs. BOGO
should say the same about bogo.eilmer.si.

## 7. Repo description and topics

**What we do now.** bogo: none. iPhone: an Apple Bridge description, no topics.

**What they do.** Bark: "Bark is an iOS App which allows you to push custom
notifications to your iPhone", website bark.day.app, nine topics (`ios`,
`ios-app`, `ios-swift`, `push-notifications` and others). bark-server:
"Backend of Bark" ([Bark][bark-repo], [bark-server][srv-repo]).

**Gap.** "Backend of Bark" shows how short a sibling repo's description can be
while still naming its partner. The iPhone repo's description should name
BOGO, and bogo's should name the iPhone app.

[bark-tree]: https://github.com/Finb/Bark/tree/a5c1cf5e0ff273c04d252d71c78b58f149e7c274
[srv-tree]: https://github.com/Finb/bark-server/tree/a365aa9ddc9362495efffb3db4358b7c12beb963
[bark-repo]: https://github.com/Finb/Bark
[srv-repo]: https://github.com/Finb/bark-server
[bark-readme]: https://github.com/Finb/Bark/blob/a5c1cf5e0ff273c04d252d71c78b58f149e7c274/README.md
[bark-docs]: https://github.com/Finb/Bark/tree/a5c1cf5e0ff273c04d252d71c78b58f149e7c274/docs
[bark-sidebar]: https://github.com/Finb/Bark/blob/a5c1cf5e0ff273c04d252d71c78b58f149e7c274/docs/en-us/_sidebar.md
[bark-docs-home]: https://github.com/Finb/Bark/blob/a5c1cf5e0ff273c04d252d71c78b58f149e7c274/docs/en-us/README.md
[bark-faq]: https://github.com/Finb/Bark/blob/a5c1cf5e0ff273c04d252d71c78b58f149e7c274/docs/en-us/faq.md
[bark-deploy]: https://github.com/Finb/Bark/blob/a5c1cf5e0ff273c04d252d71c78b58f149e7c274/docs/en-us/deploy.md
[bark-privacy]: https://github.com/Finb/Bark/blob/a5c1cf5e0ff273c04d252d71c78b58f149e7c274/docs/en-us/privacy.md
[bark-tests]: https://github.com/Finb/Bark/blob/a5c1cf5e0ff273c04d252d71c78b58f149e7c274/.github/workflows/tests.yaml
[bark-testflight]: https://github.com/Finb/Bark/blob/a5c1cf5e0ff273c04d252d71c78b58f149e7c274/.github/workflows/testflight.yaml
[bark-fastfile]: https://github.com/Finb/Bark/blob/a5c1cf5e0ff273c04d252d71c78b58f149e7c274/fastlane/Fastfile#L20-L55
[bark-testsdir]: https://github.com/Finb/Bark/tree/a5c1cf5e0ff273c04d252d71c78b58f149e7c274/BarkTests
[bark-license]: https://github.com/Finb/Bark/blob/a5c1cf5e0ff273c04d252d71c78b58f149e7c274/LICENSE
[srv-readme]: https://github.com/Finb/bark-server/blob/a365aa9ddc9362495efffb3db4358b7c12beb963/README.md
[srv-apiv2]: https://github.com/Finb/bark-server/blob/a365aa9ddc9362495efffb3db4358b7c12beb963/docs/API_V2.md
[srv-mcp]: https://github.com/Finb/bark-server/blob/a365aa9ddc9362495efffb3db4358b7c12beb963/docs/MCP.md
[srv-mcp-route]: https://github.com/Finb/bark-server/blob/a365aa9ddc9362495efffb3db4358b7c12beb963/route_mcp.go
[srv-ci]: https://github.com/Finb/bark-server/blob/a365aa9ddc9362495efffb3db4358b7c12beb963/.github/workflows/ci.yaml
[srv-license]: https://github.com/Finb/bark-server/blob/a365aa9ddc9362495efffb3db4358b7c12beb963/LICENSE
[bogo-tree]: https://github.com/JohanSardar/bogo/tree/8500392dc0d5845a2fbb489b595779db41378620
[bogo-readme]: https://github.com/JohanSardar/bogo/blob/8500392dc0d5845a2fbb489b595779db41378620/README.md
[bogo-docs]: https://github.com/JohanSardar/bogo/tree/8500392dc0d5845a2fbb489b595779db41378620/docs
[bogo-relay-readme]: https://github.com/JohanSardar/bogo/blob/8500392dc0d5845a2fbb489b595779db41378620/deploy/relay/README.md
[iphone-tree]: https://github.com/JohanSardar/apple-bridge-iphone/tree/1f25c547a4e74501f6d7820d92692ba5f19f0f3f
[iphone-readme]: https://github.com/JohanSardar/apple-bridge-iphone/blob/1f25c547a4e74501f6d7820d92692ba5f19f0f3f/README.md
[iphone-projectyml]: https://github.com/JohanSardar/apple-bridge-iphone/blob/1f25c547a4e74501f6d7820d92692ba5f19f0f3f/ios/project.yml#L10-L13
[iphone-testflight]: https://github.com/JohanSardar/apple-bridge-iphone/blob/1f25c547a4e74501f6d7820d92692ba5f19f0f3f/packaging/release/testflight.py
