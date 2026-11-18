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

The [After round one](#after-round-one-2026-11-17) section below is the
current state. Everything after it is the original comparison, kept as the
baseline it measures against.

## After round one (2026-11-17)

Round one merged into `main` of both repos: front page and docs (T23), CI and
code quality (T24, T25), TestFlight release steps (T20), and the new relay
with 6-digit pairing (T21, T26). This section checks both repos again against
the five projects, at [`bogo@f770f57`][r1-bogo] and
[`apple-bridge-iphone@ad149c6`][r1-iphone].

**How it was checked.** A gap counts as closed only if it worked when used the
way a newcomer would use it. Here is what was run, on fresh `git clone`s from
GitHub:

- bogo `make check`: passed in 6 min 47 s (fmt, both Clippy passes, 20 Rust
  test functions, the Lean build, the vector diff, `cargo deny`).
- The README quick start, copied block by block and run as written, with real
  Claude Code: the read ran, the write waited, `bogo soft` approved it, the
  file was written, and `bogo log verify` said `chain intact`.
- iPhone `make lint`, `make test` (7 BOGO proofs, 11 model proofs, release and
  lab tests) and `make simulator`: all passed.
- A real pairing over the public relay. The unsigned `make simulator` app ran
  in a new Simulator. A foreground `bogo serve` ran on its own home and port,
  built with `test-registration`. `bogo approver add` advertised the Mac over
  Bonjour, the phone listed "BOGO Mac", both screens showed the same 6
  digits, `y` confirmed on the Mac, and Face ID passed. Then the phone failed
  to save the channel keys (below).
- The self-hosting page's "On a Mac" route: `bogo-relay` behind a Cloudflare
  quick tunnel, with a daemon whose `[relay] url` named the tunnel. The Mac's
  Bonjour advert carried the tunnel's address, and no error came back.
- `gh api` for each repo's description, topics, community profile and private
  vulnerability reporting, plus CI history. The TestFlight link, the relay
  address and every link below were opened.

### Status of the original ranked gaps

| # | Gap | Status | Evidence on `main` |
|---|---|---|---|
| 1 | Repos don't link each other | **Closed** | Both READMEs open by naming and linking the other, and share one "Which repo do I want?" table ([bogo][r1-bogo-readme], [iPhone][r1-iphone-readme]). Bug routing is the same in both: BOGO-tab bugs go to bogo. `/Users/johansardar` links are gone from SPEC and ARCHITECTURE. Only `/Users/you/...` placeholders remain. |
| 2 | No description or topics | **Closed** | `gh api`: bogo "Approve your AI agent's MCP tool calls on your iPhone: Mac daemon, encrypted relay, Lean-verified core", 10 topics. iPhone repo: "iPhone app for Apple Bridge and BOGO…", 10 topics. Neither repo sets a website. |
| 3 | First screen has no picture, why or download | **Closed** | The rendered page has the logo, a one-line pitch, badges, light and dark screenshots, "Why BOGO", "See it work", "Get it" and a status note. All 8 screenshots return 200 from `main`. See new gap D1 for the red badge. |
| 4 | No install without building | **Open, owner's decision** | No releases or tags in either repo (`gh release list`, `gh api …/tags` are empty). "Get it" says so honestly. |
| 5 | No CI or badge | **iPhone closed, bogo partly** | Both have CI and a badge. The iPhone repo's runs on `main` pass. bogo's local `make check` passes, but its CI badge reads **failing** right now: one flaky test failed 2 of the last 4 runs on `main` (new gap C1). |
| 6 | No SECURITY.md or private route | **Closed** | `SECURITY.md` in both repos. `gh api …/private-vulnerability-reporting` returns `{"enabled":true}` for both. |
| 7 | `docs/` mixes user pages and records, no index | **Partly** | [`docs/README.md`][r1-docs-index] has Use, Run and How it works sections, plus an "Internal records" section with a warning. Tickets, design rounds and verification logs still sit beside the user pages in `docs/`, and the index's relay link goes to the operator runbook, not the self-hosting page (A4). |
| 8 | iPhone README written for the owner | **Partly** | The README and PRINCIPLES.md are fixed: no `ab-local`, no "Johan's rules win", no missing files cited. The [app guide][r1-iphone-guide] it links still reads as owner notes: `~/.config/apple-bridge/ios/*.mobileprovision`, an ssh install host, 13 `APP-*-SIM` lab codes. [real-qualification.md][r1-iphone-qual] names the owner and `~/I1-NEEDS-JOHAN.txt` (I3). |
| 9 | No CONTRIBUTING, AI policy or templates | **Closed** | Both repos have CONTRIBUTING with an AI section, a PR template asking for evidence, and a bug form. bogo's form has a component dropdown (daemon, CLI, front or shim, relay, …). Community profile is 85% for both, missing only a Code of Conduct (A7). |
| 10 | iPhone tests check no BOGO code | **Partly** | `make test` now builds BogoModel and BogoCrypto and runs 7 BOGO proofs (vectors, review, reason picture, closed room, sync, reconciliation). All passed. Nothing covers saving keys or finishing a registration, which is exactly where the Simulator pairing failed (I1). It uses no XCTest or `swift test`. |
| 11 | No versioning rule; builds not tagged | **Partly; tags are the owner's** | The rule is in the [app guide][r1-iphone-version]: `MAJOR.MINOR.PATCH`, one build number, and a `v<version>+<build>` tag for every upload. A build was uploaded for review, but no tag exists. |
| 12 | Build needs the owner's team ID | **Partly** | `ios/Local.xcconfig` overrides the team, as the guide says. Bundle IDs, the app group and keychain groups are still `com.johansardar.…` in [`project.yml`][r1-iphone-projectyml]. An unsigned `make simulator` build can't finish pairing (I1). |
| 13 | No dependency updates or advisories | **Closed** | [`dependabot.yml`][r1-dependabot] covers Cargo and Actions, with `rmcp` ignored. `cargo deny --locked check` runs in CI and `make check` (ran: advisories, bans, licences, sources ok). SECURITY.md has no "Dependencies" section naming the crates that touch keys (small; A7). |
| 14 | Review-round test names, no check command | **Closed** | `make check` ran clean on a fresh clone. Test files are named for what they protect (`refused_room_close_does_not_steal_the_phone_seat`, …). Every crate opts into workspace lints. |
| 15 | Logging split | **Partly** | The daemon now drops all `tracing` events on purpose ([owner.rs][r1-nosub]), and says so only in a code comment. The relay still uses `eprintln!`. `docs/daemon.md` doesn't say that failed relay fetches, refused APNs pushes or failed deliveries leave no trace (C4). |
| 16 | No page on what the relay sees or self-hosting | **Partly** | [self-hosting-relay.md][r1-selfhost] is clear, and its Mac route worked (above). The FAQ answers "What does the relay see?". Nothing user-facing says who sends the push, or that only the owner's APNs key can wake the TestFlight app (C2). The index and FAQ link the operator runbook, with its VM and IP, as "deployment" (A4). |
| 17 | No changelog | **Partly** | `CHANGELOG.md` in both repos. Neither lists round one's user-visible relay change: public relay by default, Add Mac finds the Mac nearby, 6-digit check over a relay room (A5). |

### The coordinator's leftovers

| Leftover | Verdict | Evidence |
|---|---|---|
| `Front::catalog_changed` is never called | **Confirmed** (read, not run) | Defined at [bogo-front/src/lib.rs:264][r1-catalog]. No caller exists in the workspace. The upstream manager wakes the daemon on `ToolListChangedNotification`, and `Job::Catalog` re-runs the core's configure, but nothing tells connected agent sessions. [ARCHITECTURE:366][r1-arch366] says "Legacy sessions are notified when an upstream sends `list_changed`." |
| `deploy/macos/si.eilmer.bogo.plist` is out of date | **Confirmed** | The [reference plist][r1-plist] has no `EnvironmentVariables`. The plist the CLI writes ([owner.rs:138][r1-plistfn]) adds `PATH`, `BOGO_RELAY_CA`, `BOGO_MAC_NAME` and `BOGO_APP_BUNDLE_VERSIONS`. |
| Test vectors contain a `/Users/...` path | **Confirmed** | `/Users/johansardar/bogo-test/notes.md` is in [lean/BogoVectors/Main.lean:309][r1-vectorpath], so it is in `vectors/requests.json` and the iPhone copy `ios/BogoKit/BogoRelay/StandInVectors/requests.json`. Changing it changes the vector bytes in both repos. |
| Unsigned `make simulator` build can't pair over the relay | **Confirmed by running it** | After the 6 digits matched and Face ID passed, the Simulator log showed `SecItemAdd` failing with `-34018 "Client has neither application-identifier nor keychain-access-groups entitlements"` at [RelayRegistration.swift:223][r1-keyssave]. The phone said "This iPhone couldn't finish adding the Mac. Nothing was registered." The Mac said `registration aborted: the phone cancelled`, which is wrong. Nothing in the README or app guide warns about this. |
| Pairing needs the phone on the Mac's network | **Confirmed** | The phone finds Macs only by browsing `_bogo-pair._tcp` with Bonjour ([NearbyMacs.swift][r1-nearby]). Add Mac has no code entry or scan fallback ([AddMacScreen.swift:57][r1-addmac]). A phone on cellular, a guest Wi-Fi with client isolation or a different subnet can't add the Mac. On one network, discovery worked at once. |
| TestFlight link admits nobody until Beta App Review passes | **Confirmed** | The link shows "This beta isn't accepting any new testers right now." Both READMEs and `status.md` say so plainly. |

### New gaps, ranked by how much they matter to a newcomer

Round one fixed what a newcomer *sees*. What is left is mostly what a
newcomer *hits* when they go past the quick start. Top: no one but the owner
can get a working phone approver today, even after the TestFlight link opens.

| Rank | Gap | Group | Best model |
|---|---|---|---|
| 1 | Only the owner can wake the TestFlight app; no guide to pairing an iPhone (C2, A1) | Code, Docs | Bark, ntfy, happy |
| 2 | bogo's CI badge reads "failing" on the front page (C1) | Code | ntfy, Secretive |
| 3 | An unsigned Simulator build can't pair, and both sides report it wrongly (I1, C3) | iPhone, Code | Secretive |
| 4 | Pairing works only on the same network (I2) | iPhone, Code | croc |
| 5 | Tool-list changes never reach connected agents, though ARCHITECTURE says they do (C5) | Code | — |
| 6 | The docs index and FAQ send relay readers to the operator runbook (A4) | Docs | ntfy, Paseo |
| 7 | "Why is BOGO inside Apple Bridge?" has no reason (A2) | Docs, iPhone | ntfy-ios |
| 8 | No diagnostics when phone delivery fails (C4) | Code | ntfy, Paseo |
| 9 | Changelogs miss round one's relay change (A5) | Docs | Paseo |
| 10 | The iPhone app guide and qualification record read as owner notes (I3) | iPhone | ntfy-ios, Secretive |
| 11 | Leftover rough edges: stale plist, `/Users` path in vectors, Debug output from `log verify`, build warnings (C6, I4) | Code, iPhone | Secretive |
| 12 | The relay address answers an empty 404 and has no health check (C7) | Code | ntfy |
| 13 | Thin Rust test suite (C8) | Code | ntfy, Paseo |
| 14 | Code of Conduct, homepage, Dependencies section (A7) | Docs | ntfy, Secretive |

#### Front page and docs

- **A1. Write "Add your iPhone".** After the quick start, a newcomer's next
  step is pairing a phone, and the docs index has no page for it. The only
  instructions are in [daemon.md:71][r1-daemon71]: "Configure `[apns]` and
  `[approvers.iphone]` following the architecture", which points at a
  1,072-line design document. Write one page, linked from the README and the
  index, under "Use BOGO". It should give the exact `bogo.toml` for the
  TestFlight app (`app_id`, `app_attest = ["production"]`,
  `approve_arm_delay`), the `[apns]` story once C2 settles it, `bogo approver
  add` with what each screen shows, and what to do when the Mac doesn't
  appear. It depends on C2: today the page would have to say that only the
  owner can do this.
- **A2. Give a reason for the shared app.** The FAQ's answer
  ([faq.md:75][r1-faq75]) is "The owner chose…". A newcomer installing a
  security approver will want the reason in one sentence, and to know what
  else the app can do. The app also carries Apple Bridge's Health, location,
  Screen Time and alarm features. State that those stay off until turned on,
  and that the BOGO tab needs only Face ID, notifications and local network
  access. Put the same two lines in the iPhone README's BOGO bullet.
- **A3. Separate the internal records.** Move `docs/tickets/`,
  `docs/architecture/`, `docs/evidence/`, `docs/verification/` and
  `docs/research/` under one folder (for example `docs/internal/`), as the
  original #7 asked. The index already groups them. On GitHub, the `docs/`
  folder view still shows 5 record folders next to 12 user pages.
- **A4. Point relay readers at the user page.** In the [docs index][r1-docs-index]
  ("Relay deployment") and [faq.md:57][r1-faq57], link
  `self-hosting-relay.md`, not `deploy/relay/README.md`. Add "Running your own
  relay" to "Run BOGO". In the FAQ, answer "Who sends the push, and what does
  Apple see?" in the style of ntfy's
  [iOS instant notifications][ntfy-ios-upstream] section.
- **A5. Bring the changelogs up to date.** Add the round-one relay change
  under **Unreleased** in both changelogs: public relay by default with no
  setting, Add Mac lists Macs nearby, the 6-digit check runs over a private
  relay room, and `[relay] max_message_age` and `mailbox_capacity` are now
  ignored. Also update `status.md`: the phone and Mac have now met through
  the public relay up to key saving, which is more than "Mac clients only".
- **A6. Use neutral names in examples.** `docs/daemon.md` registers
  `'Johan iPhone'`. Use `'My iPhone'`.
- **A7. Community profile and small items.** Add a Code of Conduct to reach
  100% (ntfy's profile is 100%, Secretive 85%). Set a homepage (for example
  the docs index) on both repos. Add a short "Dependencies" section to
  SECURITY.md naming the crates that hold keys or speak the wire, as the
  original #13 asked.

#### Daemon, CLI and relay code

- **C1. Fix the flaky test that turns the badge red.**
  `verify_rejects_forward_without_threshold` in
  [`bogo-daemon/tests/log_verify_and_query.rs:37`][r1-flaky] calls
  `home.lock().unwrap()` on a fresh home and sometimes gets
  `Locked(".../run/lock")`. It failed `main` at `bd0d11b` and `f770f57`, so the
  CI badge on the front page reads "failing". Commit `14e91d0` already gave the
  sibling megabyte test a short wait for the lock. Give this test the same
  fix, then re-run until green.
- **C2. Let someone other than the owner wake the phone.** Production config
  refuses `[approvers.iphone]` without `[apns]`
  ([bogo-config/src/lib.rs:400][r1-apnsreq]). The push must be signed with an
  APNs key from the team that owns `com.johansardar.applebridge.ios`, and
  [ADR 0003][r1-adr3] says the Mac sends it "with the owner's existing key".
  So a newcomer with the TestFlight app can't register it, and nothing tells
  them why. Every comparable project avoids this. Bark's server sends the
  pushes. A self-hosted ntfy forwards a poll request through ntfy.sh, which
  holds the key. happy's server sends them. Options to decide between:
  1. The public relay sends the wake-up. The Mac asks it to push to an opaque
     token, and the push has no content, as now.
  2. The iPhone route works without `[apns]`: the phone fetches on launch and
     on foreground, and the docs say requests arrive only when the app is
     opened.
  3. Both: (2) as the fallback, (1) for instant delivery.

  This changes ADR 0003, so it needs the owner's sign-off. The code and docs
  work is then a normal ticket.
- **C3. Report a phone-side failure truthfully.** When the phone fails after
  the codes match, the Mac prints `registration aborted: the phone cancelled`
  ([registration.rs:88][r1-cancelled]). Tell the two apart, or say "the
  iPhone couldn't finish; see the phone".
- **C4. Leave a trace when delivery fails.** The daemon drops every `tracing`
  event on purpose ([owner.rs:895][r1-nosub]), so failed relay fetches,
  refused APNs pushes and failed deliveries leave nothing in
  `run/stderr.log`. Review those few events (the hub, the iPhone route and the
  APNs client, about 10 call sites) and let them through with secrets
  removed, or add a `bogo doctor` that tests the relay and APNs. Say in
  `docs/daemon.md` where logs go. Switch the relay's three `eprintln!` calls
  to the same scheme.
- **C5. Tell sessions when the tool list changes.** Call
  `Front::catalog_changed` after a new catalog is applied, or remove the
  method and the ARCHITECTURE sentence. Calling it is the fix: an agent that
  keeps a stale tool list sees a renamed or added tool only after it
  reconnects.
- **C6. Small leftovers.**
  - Bring `deploy/macos/si.eilmer.bogo.plist` in line with `plist()`.
  - Replace `/Users/johansardar/bogo-test/notes.md` in the Lean vector
    generator with a neutral path. Regenerate `vectors/`, then copy them to the
    iPhone repo's `StandInVectors` in the same round.
  - Make `bogo log verify` print plain words, not the Rust `Report { entries:
    21, … }` debug form.
  - Explain `path must be shorter than SUN_LEN` (a long `--home` path) in
    words.
- **C7. Give the relay a front door.** `https://bogo.eilmer.si:8443/` answers
  an empty 404, and the relay has no health route, so a self-hoster can't
  check their relay before pointing BOGO at it. The relay also logs nothing
  per request. Add `GET /` with one line naming BOGO and linking the repo,
  plus `GET /healthz`. Add a "check it works" step (`curl …/healthz`) to
  self-hosting-relay.md.
- **C8. Grow the Rust tests where the risk is.** The workspace has 20 test
  functions for 23 crates, leaning on examples, Lean proofs and vectors.
  Pairing over a relay room has one Rust test. Add tests for registration
  (match, mismatch, a phone failing after the match) and for the catalog
  notice (C5).

#### iPhone app

- **I1. Let a Simulator build pair with a real Mac.** `make simulator` builds
  with `CODE_SIGNING_ALLOWED=NO`, so the app has no keychain entitlement. Every
  `BogoKeychain` call fails with `-34018`, and Add Mac fails after Face ID.
  Either sign Simulator builds ad hoc with the entitlements
  (`CODE_SIGN_IDENTITY=-`), or keep channel keys in the same file store
  `SimulatorApprovalKey` uses in Simulator Debug builds. Add a proof that
  saves and loads channel keys, and show a precise message on the phone.
  Until it's fixed, say in the README that `make simulator` builds can pair
  only with the stand-in Mac.
- **I2. Pair without the same network.** Keep Bonjour as the easy path. Add
  a fallback: `bogo approver add` also prints its `bogo1-…` code (and a QR in
  the terminal), and Add Mac takes it typed, pasted or scanned. The 6 digits
  still keep impostors out, so the security argument in ADR 0003's T26 update
  holds. croc, which T21 modelled the relay on, pairs with a typed code.
- **I3. Make the app guide readable by outsiders.** Split
  [`ios/README.md`][r1-iphone-guide] into a short "Build, run and pair" guide
  and a separate lab and qualification page. Replace the 13 `APP-*` scenario
  codes with plain names. Move owner-only material (provisioning profile
  paths, the ssh install host, `pymobiledevice3`) under a clearly labelled
  maintainer section. In [real-qualification.md][r1-iphone-qual], drop the
  owner's name, `~/I1-NEEDS-JOHAN.txt` and the bare "PR #453" from another
  repo.
- **I4. Build without warnings.** `make simulator` prints three kinds of
  warning:
  - Xcode calls the project "malformed" because `Capabilities/Alarms` is in
    two groups.
  - A HealthKit initializer is deprecated in iOS 17.
  - `nonisolated(unsafe)` is unnecessary in `MismatchRule.swift:12`.

  `make test` prints that last warning five times. A newcomer's first build
  shouldn't scroll warnings.

### Needs the owner's decision

These are blocked by the owner's rules on publishing, or are product calls
only the owner can make:

- **Releases and binaries** (original #4): tag a version, attach `bogo`,
  `bogo-daemon`, `bogo-shim` and `bogo-relay`, then possibly Homebrew or a
  relay image. Until then, "Get it" says build from source, which is honest.
- **Tags for TestFlight builds** (original #11): the written rule asks for a
  `v<version>+<build>` tag per upload, and the uploaded build has none.
- **Opening the TestFlight link**: it waits on Beta App Review of the first
  build.
- **Who sends the push** (C2): changing ADR 0003 so a newcomer's phone can be
  woken. This is the decision that most limits who can use BOGO at all.

### Two questions a newcomer asks

**Does it make sense that BOGO's phone side lives inside the Apple Bridge
app?** After reading, yes. Before, no. Both READMEs explain the split in the
same words and the same table, so nobody is lost. But nothing gives a reason,
the TestFlight page and home screen say "Apple Bridge", and a security
approver arrives inside an app with Health, location and Screen Time code. A
newcomer accepts that only with a sentence of why, and one saying those
features stay off (A2). The real cost is C2: the shared bundle ties every
phone to the owner's push key.

**Do the two READMEs read as one story?** Mostly. The openings, status notes,
"Which repo do I want?" table, badges and bug routing match and link each
other. The story breaks in two places. The iPhone README's second half
("Apple Bridge gateway", `go run ./cmd/bridge-lab`) and its app guide read
like a different project's internal notes (I3). Neither README tells you how
to connect the phone you just installed to the Mac you just set up (A1, C2).
That handoff is the one place a reader goes from one repo to the other.

### Item by item against the five

| Item | Now | Still short of |
|---|---|---|
| Front page | Logo, pitch, badges, screenshots, Why, See it work, Get it, honest status. Matches Secretive and Bark in shape. | The red CI badge (C1). No download that works yet (owner). |
| Docs | An index, FAQ, agent setup, status, self-hosting page. | Bark's and ntfy's docs take a user from install to a first notification. Ours stop before the phone (A1). Records still sit in `docs/` (A3). |
| Separation of app and server | Two repos linked both ways with one table, like happy and ntfy. Bugs routed to one repo, like ntfy. | ntfy and Bark explain who sends the push and keep self-hosting working with the phone. Ours can't wake a newcomer's phone (C2). |
| Server and CLI code | `make check`, CI on four jobs, Dependabot, `cargo deny`, pinned actions. On a par with ntfy's checks. | A flaky test on `main` (C1), no logs from the daemon (C4), 20 tests (C8). |
| iOS app | Lint, model and BOGO proofs, an unsigned Simulator build, CI, a written versioning rule, a team override. | Simulator can't pair (I1). Warnings (I4). No tags (owner). Bundle IDs are the owner's, unlike Secretive's one-script setup. |
| Community files | SECURITY, CONTRIBUTING with an AI policy, PR template, bug forms, private reporting on. 85% profile. | No Code of Conduct (ntfy 100%). |
| Description and topics | Both set, 10 topics each. | No homepage or website; all five projects have one. |

### What could not be checked

- A real iPhone, real Face ID, App Attest on hardware or APNs delivery. The
  pairing run used the Simulator, `test-registration` and no push.
- The rendered GitHub pages in a browser. Chrome's debugging port wasn't
  available, so the front page was checked from GitHub's served HTML and raw
  asset URLs instead.
- `catalog_changed` was checked by reading the code and searching for
  callers, not by running an upstream that sends `list_changed`.
- The self-hosting Docker and cloud routes were not run. Only the "On a Mac"
  route was, and only as far as the Mac's advert; no phone joined it.
- Whether a locally signed Xcode Simulator run (`make project`, then Run) can
  pair. The README says Xcode signs Simulator builds locally, which would
  likely avoid I1, but this wasn't tried.

[r1-bogo]: https://github.com/JohanSardar/bogo/tree/f770f573835902c6e1d818b69827008df505944e
[r1-iphone]: https://github.com/JohanSardar/apple-bridge-iphone/tree/ad149c6fb31c0964413e23ec5f4ef93fada2b4a9
[r1-bogo-readme]: https://github.com/JohanSardar/bogo/blob/f770f573835902c6e1d818b69827008df505944e/README.md#which-repo-do-i-want
[r1-iphone-readme]: https://github.com/JohanSardar/apple-bridge-iphone/blob/ad149c6fb31c0964413e23ec5f4ef93fada2b4a9/README.md#which-repo-do-i-want
[r1-docs-index]: https://github.com/JohanSardar/bogo/blob/f770f573835902c6e1d818b69827008df505944e/docs/README.md?plain=1#L28
[r1-iphone-guide]: https://github.com/JohanSardar/apple-bridge-iphone/blob/ad149c6fb31c0964413e23ec5f4ef93fada2b4a9/ios/README.md?plain=1#L65
[r1-iphone-qual]: https://github.com/JohanSardar/apple-bridge-iphone/blob/ad149c6fb31c0964413e23ec5f4ef93fada2b4a9/docs/real-qualification.md?plain=1#L66
[r1-iphone-version]: https://github.com/JohanSardar/apple-bridge-iphone/blob/ad149c6fb31c0964413e23ec5f4ef93fada2b4a9/ios/README.md?plain=1#L56
[r1-iphone-projectyml]: https://github.com/JohanSardar/apple-bridge-iphone/blob/ad149c6fb31c0964413e23ec5f4ef93fada2b4a9/ios/project.yml#L3
[r1-dependabot]: https://github.com/JohanSardar/bogo/blob/f770f573835902c6e1d818b69827008df505944e/.github/dependabot.yml
[r1-nosub]: https://github.com/JohanSardar/bogo/blob/f770f573835902c6e1d818b69827008df505944e/crates/bogo-daemon/src/owner.rs#L895-L897
[r1-selfhost]: https://github.com/JohanSardar/bogo/blob/f770f573835902c6e1d818b69827008df505944e/docs/self-hosting-relay.md
[r1-catalog]: https://github.com/JohanSardar/bogo/blob/f770f573835902c6e1d818b69827008df505944e/crates/bogo-front/src/lib.rs#L264
[r1-arch366]: https://github.com/JohanSardar/bogo/blob/f770f573835902c6e1d818b69827008df505944e/docs/ARCHITECTURE.md?plain=1#L366
[r1-plist]: https://github.com/JohanSardar/bogo/blob/f770f573835902c6e1d818b69827008df505944e/deploy/macos/si.eilmer.bogo.plist
[r1-plistfn]: https://github.com/JohanSardar/bogo/blob/f770f573835902c6e1d818b69827008df505944e/crates/bogo-daemon/src/owner.rs#L138-L171
[r1-vectorpath]: https://github.com/JohanSardar/bogo/blob/f770f573835902c6e1d818b69827008df505944e/lean/BogoVectors/Main.lean#L309
[r1-keyssave]: https://github.com/JohanSardar/apple-bridge-iphone/blob/ad149c6fb31c0964413e23ec5f4ef93fada2b4a9/ios/BogoKit/BogoRelay/RelayRegistration.swift#L223
[r1-nearby]: https://github.com/JohanSardar/apple-bridge-iphone/blob/ad149c6fb31c0964413e23ec5f4ef93fada2b4a9/ios/BogoKit/BogoRelay/NearbyMacs.swift#L14
[r1-addmac]: https://github.com/JohanSardar/apple-bridge-iphone/blob/ad149c6fb31c0964413e23ec5f4ef93fada2b4a9/ios/BogoKit/BogoSection/AddMacScreen.swift#L57
[r1-daemon71]: https://github.com/JohanSardar/bogo/blob/f770f573835902c6e1d818b69827008df505944e/docs/daemon.md?plain=1#L71
[r1-faq75]: https://github.com/JohanSardar/bogo/blob/f770f573835902c6e1d818b69827008df505944e/docs/faq.md?plain=1#L75
[r1-faq57]: https://github.com/JohanSardar/bogo/blob/f770f573835902c6e1d818b69827008df505944e/docs/faq.md?plain=1#L57
[r1-flaky]: https://github.com/JohanSardar/bogo/blob/f770f573835902c6e1d818b69827008df505944e/crates/bogo-daemon/tests/log_verify_and_query.rs#L37
[r1-apnsreq]: https://github.com/JohanSardar/bogo/blob/f770f573835902c6e1d818b69827008df505944e/crates/bogo-config/src/lib.rs#L400-L402
[r1-adr3]: https://github.com/JohanSardar/bogo/blob/f770f573835902c6e1d818b69827008df505944e/docs/adr/0003-bogo-owns-its-relay-and-push.md
[r1-cancelled]: https://github.com/JohanSardar/bogo/blob/f770f573835902c6e1d818b69827008df505944e/crates/bogo-approver-iphone/src/registration.rs#L88
[ntfy-ios-upstream]: https://github.com/binwiederhier/ntfy/blob/a098f52f6b643e8986f261f7cd9796e6bd1405d1/docs/config.md?plain=1#L1645

## Where we stand today (before round one)

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

## Ranked gaps (before round one)

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
  [proved-and-assumed.md](../../proved-and-assumed.md) saying exactly what is
  proved. Of the others, only happy has Lean proofs, for its iOS sync logic.
- **Threat model and ADRs.** [threat-model.md](../../threat-model.md) and three
  ADRs go further than any of the five, except Paseo's SECURITY.md, which is
  comparable for the relay.
- **Evidence culture.** `docs/verification/` records what was actually run.
  Paseo asks contributors for this in its PR template; BOGO already does it.
- **Pinned dependencies with reasons.** `rmcp = "=3.10.0"` with the reason in
  [Cargo.toml](../../../Cargo.toml).
- **Glossary.** [GLOSSARY.md](../../../GLOSSARY.md); only Paseo has one too.

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
