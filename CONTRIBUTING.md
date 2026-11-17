# Contributing to BOGO

Thanks for helping. BOGO is a security tool, so every change has to show that
it works, not just say so. This page covers both of BOGO's repos; the iPhone
app's own build steps are in its
[CONTRIBUTING](https://github.com/JohanSardar/apple-bridge-iphone/blob/main/CONTRIBUTING.md).

## Where things go

| Change | Repo |
|---|---|
| Daemon, CLI, shim, relay, Lean core and proofs, sandbox, BOGO docs | this repo |
| The BOGO tab and anything else in the iPhone app | [apple-bridge-iphone](https://github.com/JohanSardar/apple-bridge-iphone) |
| A bug you saw in BOGO, even on the phone | [this repo's issues](https://github.com/JohanSardar/bogo/issues/new/choose) |
| A vulnerability | [private report](SECURITY.md), never a public issue |

For anything bigger than a small fix, open an issue first so we can agree on
the approach before you write it.

## How this project uses AI

Most of BOGO's code and docs are written by AI coding agents, working from
written tickets (in [`docs/tickets/`](docs/tickets/)) under the owner's review.
Every change is checked by running it, and the results are recorded (in
[`docs/verification/`](docs/verification/)). Agents don't merge their own work.

You're welcome to use AI tools too. You are responsible for every line you
submit: understand it, run it, and show what you saw. A pull request that
can't explain or demonstrate its own change will be closed.

## Set up

You need the same tools as the [quick start](README.md#quick-start-on-macos):
Xcode Command Line Tools, Rust via rustup, Lean via elan (first on your
`PATH`), and Node.js for the demo upstream.

```sh
export PATH="$HOME/.elan/bin:$PATH"
cargo build --workspace
```

The build compiles the Lean decision core in `lean/` into the daemon; the
toolchain is pinned in `lean/lean-toolchain`.

## Checks

Run these before you open a pull request:

```sh
cargo build -p bogo-relay        # one stand-in test starts the real relay binary
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
(cd lean && lake build BogoProofs BogoVectors && lake env lean BogoProofs/Axioms.lean)
```

The last line builds the proofs and checks that `#print axioms` still reports
only the pinned axioms. If you change the core, the request encoding or the
vectors, say so in the pull request: the iPhone app decodes the same bytes.

## What a pull request needs

- **What changed and why**, in plain words.
- **Evidence.** What you ran and what you saw: command output, a log excerpt,
  or screenshots for anything on the phone. Say what you could not check.
- **Docs.** If behaviour a user can see changed, update the README or
  [`docs/`](docs/README.md) in the same pull request. Use the words in the
  [glossary](GLOSSARY.md).
- **Honesty about limits.** Don't describe something as working, verified or
  safe unless you ran it. [What is verified](docs/status.md) must stay true.

The [pull request template](.github/pull_request_template.md) asks for each of
these.

## Changelog

Add a line under **Unreleased** in [CHANGELOG.md](CHANGELOG.md) for any change
a user would notice.

## License

By contributing, you agree that your contribution is licensed under the
[MIT license](LICENSE).
