# Releasing BOGO

A release is a version tag on `main`. Pushing the tag makes CI
([`.github/workflows/release.yml`](../.github/workflows/release.yml)) build
`bogo` and `bogo-shim` for Apple Silicon Macs, with default features only, and
publish them as a GitHub release. [`install.sh`](../install.sh) installs from
that release.

## Versions

- The version is `MAJOR.MINOR.PATCH`, kept once for the whole workspace in
  `Cargo.toml` (`[workspace.package] version`). MAJOR stays 0 while BOGO is
  early, matching the iPhone app, which stays at 0 while it is only on
  TestFlight.
- Raise MINOR, and reset PATCH to 0, when a release adds or changes something a
  user can see: a command, a config key, a log field, what is protected or how.
  A release that only fixes things raises PATCH.
- Any change to the request encoding, the vectors or the relay protocol also
  needs a matching iPhone app build. Say so in the changelog entry.
- Tag the release commit `v<version>`, for example `v0.1.0`. The tag must equal
  the `Cargo.toml` version; CI refuses one that doesn't.
- A tag with a suffix, such as `v0.2.0-rc.1`, is a pre-release. CI publishes it
  as a GitHub pre-release, which `install.sh` installs only when it is named in
  `BOGO_VERSION`. Pre-releases don't need to match `Cargo.toml`.
- Never move or reuse a tag. A release with a mistake gets a new PATCH.

`bogo --version` prints the version and the commit it was built from, for
example `bogo 0.1.0 (commit 1a2b3c4d5e6f)`. A source build prints the
`Cargo.toml` version and the checkout's commit.

## Cut a release

1. On a branch, set the version in `Cargo.toml` and run `cargo build` so
   `Cargo.lock` follows.
2. In `CHANGELOG.md`, rename **Unreleased** to `## [<version>] - <YYYY-MM-DD>`
   and start a new, empty **Unreleased** above it. CI uses that entry as the
   release notes and refuses a release tag without one.
3. Open a pull request, let CI pass, and merge it.
4. Tag the merge commit and push the tag:

   ```sh
   git switch main && git pull
   git tag -a v0.1.0 -m "BOGO v0.1.0"
   git push origin v0.1.0
   ```

5. Watch the Release workflow. When it finishes, the release has
   `bogo-aarch64-apple-darwin.tar.gz` and `SHA256SUMS`. Check the install line
   on a Mac:

   ```sh
   curl -fsSL https://bogo.eilmer.si/install.sh | sh
   bogo --version
   ```

## What the release build checks

Before it publishes, the workflow builds the Lean proofs and checks that
`#print axioms` is still as pinned (`make proofs`) and that the vectors match
(`make vectors`). After the build it checks that each binary is arm64, carries
the linker's ad-hoc code signature, links only system libraries, and that
`bogo` has no software test approver.

## Signing and Gatekeeper

The binaries are ad-hoc signed by the linker, which Apple Silicon requires to
run any code. They are not signed with a Developer ID and not notarized.

`curl` does not add the quarantine flag that browsers add, so binaries
installed by `install.sh` run without a Gatekeeper prompt. If you download the
archive with a browser instead, macOS blocks the programs until you remove the
flag:

```sh
xattr -d com.apple.quarantine ~/.local/bin/bogo ~/.local/bin/bogo-shim
```

## The short install URL

`https://bogo.eilmer.si/install.sh` is an HTTP redirect to
`https://raw.githubusercontent.com/JohanSardar/bogo/main/install.sh`, served by
the relay. The script and the binaries live only on GitHub.
