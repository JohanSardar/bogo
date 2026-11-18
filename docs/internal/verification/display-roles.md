# T17 display roles verification

Implementation branches: BOGO `t3/add-display-roles`; iPhone
`impl/t17-display-roles` in its own worktree at
`/Users/johansardar/.t3/worktrees/iphone-t17-display-roles`, branched from
`build/bogo` (`c047049d3535764a3736313db95ac58873d8c3d9`).
BOGO's base is `17d630e5a094c1580487578a2c86bebae3e97d9a`.
Reviewed behavior commits: BOGO `cac554622af20a98248574f3f2b46b074aa78cfc`;
iPhone `698299e87f83b957f2a39112e73fc6aae13d21c0`.
Neither branch was pushed or merged.

The new signed format tags are title=2, subtitle=3, image=4. Existing tags and
payloads are unchanged. Swift exposes `.title`, `.subtitle`, `.image`. The
current Review model displays these as text until T18 supplies the new card.
Dotted array indices in the ticket's Link example work for the new roles;
text/money retain numeric object keys and use bracket notation for arrays.

## Required checks

- From `lean/`, with elan on PATH: `lake build BogoProofs BogoVectors` passed
  (25 jobs), and `lake env lean BogoProofs/Axioms.lean` exited 0. The pinned
  axiom guards are unchanged; the proof library was built, not only static core.
- `cargo test` passed in full after the final fix: 17 tests, 57 targets including
  zero-test/doc-test targets. The initial run stopped at the existing stand-in
  regression's missing `target/debug/bogo-relay`; building that documented
  prerequisite and rerunning the suite passed.
- `cargo clippy -p bogo-config -p bogo-core -p bogo-daemon -p bogo-standin
  --all-targets -- -D warnings` passed. After the compatibility fix,
  `cargo clippy -p bogo-config --all-targets -- -D warnings` passed again.
- `rustfmt --edition 2024 --check --config skip_children=true` on the five
  changed Rust source files and `git diff --check` passed. Repository-wide
  `cargo fmt --check` reports pre-existing formatting in unrelated files;
  it was not applied to the repository.
- `lake exe BogoVectors` generated and round-tripped the new `display-roles`
  request. Comparing every legacy request's `encoded_hex` to `build/bogo`
  proved all 22 existing requests byte-identical. All three shared stand-in
  vector files are identical between BOGO and iPhone.
- In the iPhone worktree, `make lint`, full `make test`, and `make simulator`
  passed. `make test` includes pairing proof, 11 production-model proofs,
  and 27 Python tests. Existing Xcode group/provisioning-profile warnings and
  the existing `MismatchRule.swift` unnecessary `nonisolated(unsafe)` warning
  remain.
- Compiled the production Swift model sources with `xcrun swiftc
  -parse-as-library -swift-version 6` and exercised the shared vectors:
  all 23 requests decode, all three roles resolve to their actual values,
  and mutated format tags 5 and 255 are rejected. The existing `lie-in-reason`
  vector still produces the amount mismatch warning. The existing
  `BogoReasonPictureProof.swift` (T14, `PASS S1`) and
  `BogoReviewExactKeyProof.swift` both ran successfully against the production
  model sources.

## Use through the real callers

Built `bogo` with `test-approvers`, and used its own `init`, `serve`,
`agent add`, `approver add --kind soft`, `request list`, `soft`, and
`log verify` commands. Everything used an isolated temporary `--home`; no
LaunchAgent was loaded and no live BOGO state or daemon was touched.
A synthetic stdio MCP upstream supplied the real wrapper tool schema; the
real `bogo-shim` initialized a client session and sent the vector's spend call.
The config used the ticket's exact Link highlight map and per-action key,
with `select = "name"` on the base entry.

Observed after the final compatibility fix:

```text
daemon rejects duplicate title
daemon rejects duplicate subtitle
daemon rejects duplicate image
daemon rejects invalid path or non-string format
daemon rejects invalid path or non-string format
daemon loaded Link example and started in isolated home
Protected spend request created; signed bytes saved for decoder inspection
Approved request forwarded exact original arguments, justification removed
client result: spend accepted
daemon audit chain and approval signatures verify
Daemon signed image: https://images.unsplash.com/photo-1500530855697-b586d89ba3ee?w=800
Daemon signed title: Trip to SFO
Daemon signed subtitle: Example Travel
Selector leads all 7 signed highlights
```

The daemon's actual request bytes were decoded by the production Swift model,
not inferred from config alone. The synthetic upstream's received parameters
were compared to the original arguments and matched exactly. The successful
signed approval and log verification also cover the new formats inside signed
request bytes. Role validation applies to each base/per-action entry through
the same validator; inherited maps remain valid and explicit maps replace them.
The selector reordering remains unchanged.

Also built `bogo-standin` with `test-registration`, ran a private local TLS
`bogo-relay` with explicit retention/capacity, and registered `bogo-simphone`
using the real relay protocol. The two registration comparison codes matched.
Sent `display-roles` through the stand-in twice: the software phone approved
one request and denied the other. The stand-in verified the approval signature
over the exact bytes and its audit log verified. All temporary processes were
terminated afterward. This exercised the updated stand-in format reader;
it was a software phone, not native Simulator UI evidence.

## Sol review

Two rounds with Codex `gpt-6.1-sol`, medium effort. Round 1 found S1 (blocker):
the initial dotted-index parser reinterpreted legacy numeric object keys.
Applied Sol's test, observed it fail, scoped dotted indices to the three new
roles, and observed it pass. The retained test is
`crates/bogo-config/tests/review_s1_numeric_key.rs`. Round 2 approved the fix
and marked S1 resolved. No declined findings or open blocker/should findings.
Sol added one regression test; it made no product edits. Usage was not exposed.
BOGO diff grew from 184 insertions/14 deletions to 220 insertions/16 deletions,
including the 32-line regression and its compatibility fix, before this report.
The iPhone diff remained 85 insertions/4 deletions. No push was requested.

## Unchecked or blocked

Native Simulator decoding/UI verification remains blocked. Device tools opened
BOGO iPhone `535D8871-C27D-43D9-8244-CF9EBFEF7897`; app launch returned:

```text
Error (DEVICE_IN_USE): Device is already in use by session "t3-3431a5c0daaf5d52b7c80a35".
```

The advised bounded retry (`open ... --wait 1000`) also failed after 1004 ms.
The other session was not reused or closed; device work stopped as required.
Consequently the ticket's native Simulator decode step and visible Approve,
Deny, arm-delay and picture-rendering checks are not claimed complete. The
Approve/Deny relay protocol, mismatch warning and T14 picture-link extraction
were checked as described above. Arm-delay and decision UI code are unchanged
and were inspected, but were not exercised visually.

No hardware Face ID, App Attest, APNs, real Link purchase, or external image
fetch was checked. No Apple Bridge Mac repository, live `~/.bogo`, live daemon,
password prompt, sudo command, or secret output was used.
