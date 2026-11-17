# Let the config say where a highlighted value goes on the Review card

## Goal
The iPhone's Review screen is being redesigned as a clean card (T18): a **title**, a big **image**, a **subtitle** (a short description, like the merchant) next to the **price**, the agent's **reason**, and **extra details** at the bottom. Today a tool's `highlight` map only says how to format a value (`"text"` or money like `"usd_cents"`), so the phone can't know which value is the title, which is the picture and which is the description. Let the owner say so in `bogo.toml`, so any tool, such as a purchase, an email or a deploy, can be set up as a clean card.

When this is done, three new highlight formats exist next to `"text"` and the money formats:

- `"title"`: the card's heading (text).
- `"subtitle"`: a short line under the title, such as the merchant or the recipient (text).
- `"image"`: a picture link (`https://`) shown large on the card.

The existing formats keep their meaning on the card: the first money highlight is the **price**, and `"text"` highlights are **extra details**. The agent's justification is already its own field and becomes the **reason**. For example, Link's spend request becomes:

```toml
[tools."stripe__call_write_tool[name=spend-request_create]"]
policy = "approve"
justification = "required"
highlight = { "arguments.lineItem.0.name" = "title", "arguments.lineItem.0.image_url" = "image", "arguments.merchantName" = "subtitle", "arguments.amount" = "usd_cents", "arguments.context" = "text", "arguments.test" = "text" }
```

These are display hints like the ones already in the signed request, so the roles are signed too. They change presentation only: never authorization, forwarding, arguments or logs.

## Context
- The BOGO repo is `/Users/johansardar/.t3/projects/bogo`, branch from `build/bogo`. Highlight formats live in `lean/BogoCore/Encoding.lean` (`inductive Format`, `encFormat`, `Format.wf`, `decFormat`), with proofs in `lean/BogoProofs` (`Encoding.lean`, `Decode.lean`, `Bytes.lean`). The Rust side is the config crate (`crates/bogo-config`: parsing `highlight`, per-action entries, `select`) and whatever builds the request's hints. Read `docs/SPEC.md` §3.6, `docs/configuration.md` (the `highlight` and `select` rows and the Link example) and `docs/proved-and-assumed.md`.
- The iPhone app is `/Users/johansardar/apple-bridge-iphone`, branch from `build/bogo`, in your own new worktree (don't use the main checkout, which is on `main`). The decoder is `ios/BogoKit/BogoModel/RequestBytes.swift` (`hint()`), the model `ApprovalRequest.swift` (`DisplayFormat`), and the stand-in Mac's test vectors are `ios/BogoKit/BogoRelay/StandInVectors/`.
- T18 is redesigning the phone's screens in parallel. It will read `DisplayFormat.title`, `.subtitle` and `.image`. Give the new formats exactly those Swift names, and encode them as format tags 2, 3 and 4 after `text` (0) and `moneyMinor` (1).
- Commit in small commits on your own branches, each ending with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`. Don't merge into `build/bogo`.

## Constraints
- Every Lean proof must still build, and `#print axioms` must stay as pinned. Build the proofs yourself (`lake build` of the proof library, not only `BogoCore:static`).
- Don't change the request encoding of existing requests: a request with only `text` and money hints must encode to the same bytes as before.
- Don't touch the live BOGO at `~/.bogo` or its running daemon, and don't touch the Apple Bridge Mac repo.

## Guardrails
- Config rejects a role on an invalid path or a non-string format as today. Reject more than one `title`, `subtitle` or `image` in one entry, with a clear error.
- The selected argument (`select`) still leads the highlights.
- The phone decoder rejects unknown format tags, as it does now.
- Approve, Deny, the arm delay, the mismatch warning and T14's pictures behave as before.

## How to verify
1. `cargo test` passes, and the Lean proofs build with the same pinned axioms.
2. A config with the Link example above loads, and a spend request built from it carries those hints in its encoded bytes.
3. A request with only the old formats encodes to exactly the same bytes as on `build/bogo`.
4. The stand-in vectors include a request using all three new roles. The iPhone decodes it in a Simulator; `make lint` and `make test` pass in the iPhone repo.
5. `docs/configuration.md` documents the three roles with the Link example, and the Link example in it uses them.

## Evidence of done
When you are done, use it yourself the way a user would: load the config with the daemon's own commands and send a request through the stand-in path, the way the real client would. Check that it works, that it does everything this ticket asks, and that you hit no other problems. Go through every step in How to verify and check every guardrail. Follow the `agent-testing` skill for all testing. If you changed code, run the `sol-loop` skill on it and give Sol this ticket. In your report, show what you saw at each step, give both branch names and head commits, and list anything you did not check.
