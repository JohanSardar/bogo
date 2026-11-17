## What changed and why

<!-- Plain words. Link the issue if there is one. -->

## Evidence

<!-- What you ran and what you saw: command output, log excerpts, screenshots for anything on the phone.
     A change without evidence can't be merged. See CONTRIBUTING.md. -->

## Not checked

<!-- What you could not run or verify, and why. "Nothing" is a fine answer if it's true. -->

## Checklist

- [ ] `cargo test --workspace` and `cargo clippy --workspace --all-targets -- -D warnings` pass
- [ ] If the Lean core, encoding or vectors changed: the proofs build and the pinned axioms are unchanged, and the iPhone app still decodes the vectors
- [ ] Docs and [What is verified](../docs/status.md) still tell the truth
- [ ] CHANGELOG.md has a line under Unreleased, if a user would notice
