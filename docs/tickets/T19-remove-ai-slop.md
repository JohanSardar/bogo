# Strip the AI-generated look out of the iPhone app

## Goal
T18 gave the iPhone app a new design, but the owner looked at the Review card on 2026-11-17 and called it "typical AI slop". The tells are things no real premium app does: a ✦ sparkle stuck on labels, a letter-in-a-circle avatar ("S"), an "Agent's picture" pill badge laid over the photo, an amber-tinted gradient box with spaced-out capitals ("AGENT'S REASON · UNVERIFIED"), boxes nested in boxes, raw code names ("spend-request_create · call_write_tool on stripe"), and a slider that covers content. The owner's screenshot is `/Users/johansardar/.t3/userdata/attachments/e34e37e9-434b-47d7-9b7c-daefe641d21b-9e93a74c-7f42-4973-ae8f-ba8f56b0b6ef.png`.

When this is done, every screen of the app looks like Mercury, Stripe or Coinbase made it. The hierarchy comes from type, spacing and restraint, not from icons, badges and tints. Put any screen next to a real Mercury, Stripe or Coinbase screen and nothing gives it away as AI-made.

## What to change
- **No decorative icons anywhere.** Remove every sparkle (✦, `sparkle`, `sparkles` symbols) and any icon that only decorates a label. Icons stay only where they are controls (back, more, close) or carry a meaning a word can't.
- **No letter avatars.** The agent is a quiet line of text, such as "Shopping agent · 7:10 AM" under the title.
- **The picture stands alone.** No badge or pill on it, and no grey well inside a card. The product sits cleanly on the page, fitted whole and never cropped. Its provenance goes in the details as plain text ("Image from the agent").
- **The reason is plain text.** No tint, gradient, all-caps tracking or icon: a small grey "Reason" label, the agent's words in body text, and a small grey footnote "Not verified by BOGO".
- **One surface.** Stop nesting cards inside cards. Separate sections with spacing and hairline dividers.
- **Plain words, not code names.** "spend-request_create · call_write_tool on stripe" becomes something like "Link · Create spend request" (humanize the action, show the upstream as a name). Raw identifiers live only in All details.
- **A calm price.** One color with tabular figures; drop the grey cents unless the references really do it.
- **Fix the bottom bar.** The approve control must never cover content. Give it its own area, with the scroll content inset above it.
- **Sweep the whole app** for the same tells: sparkles, badges and pills, tinted or gradient boxes, uppercase tracked labels, letter avatars, nested cards, emoji, decorative glows, rainbow accents, and buzzword copy. That covers the home cards, Activity, Settings, Macs, Add Mac, pairing, Diagnostics, the tab bar, the success state and All details.
- Write these rules into `docs/design.md` as a short "Never" list, so later work keeps them.

## Context
- The iPhone app is `/Users/johansardar/apple-bridge-iphone`. Make your own worktree and branch from `build/bogo` (now `2cca38f`, which includes T17 and T18). Don't use the main checkout, which is on `main`.
- Read `docs/design.md` and `/Users/johansardar/.t3/projects/bogo/docs/tickets/T18-premium-redesign.md`. T18's screenshots are in `/Users/johansardar/.t3/worktrees/apple-bridge-iphone/t18-evidence/`.
- Look at real reference screenshots of the Mercury, Stripe and Coinbase iPhone apps (App Store listings, their sites, Mobbin-style galleries, design write-ups) before you change anything, and keep a few of them in your evidence folder to compare against.
- Demo mode (Settings → Demo) sends the Link Rubik's Cube purchase, an email, a deploy and a mismatch request through the stand-in Mac in the Simulator.
- Drive the Simulator with the T3 device tools (load them with ToolSearch `select:mcp__t3-code__device_list,mcp__t3-code__device_open,mcp__t3-code__device_screenshot,mcp__t3-code__device_close`). If a Simulator is in use by another session, boot a different one.
- Commit in small commits on your own branch, each ending with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`. Don't merge into `build/bogo`.

## Constraints
- Display only: no change to what is signed, forwarded or logged, to the crypto, or to the relay protocol.
- Don't touch the BOGO Rust repo, the Apple Bridge Mac repo, or the real iPhone.
- System frameworks only: no new packages.

## Guardrails
- Approve (slide, then Face ID), Deny, the arm delay, expiry, the mismatch warning, pictures and All details all still work and stay reachable. The mismatch warning stays visible, but restyle it to fit: plain text, no tinted box.
- The reason and picture are still clearly the agent's claims, said in plain words.
- Light and dark mode, Dynamic Type, VoiceOver labels and Reduce Motion still work.

## How to verify
1. Show every screen in light and dark, before (T18) and after, side by side with a matching reference screenshot from Mercury, Stripe or Coinbase.
2. Show that a search of the Swift source finds no sparkle symbols, no letter avatars and no uppercase tracked labels.
3. Film the demo again: the Link purchase arrives, you review it, slide, Face ID, success. Film a deny too.
4. Show the email, no-roles and mismatch requests, and All details.
5. `make lint` and `make test` pass.

## Evidence of done
When you are done, use it yourself the way a user would: run the real app and go through it like they do. Check that it works, that it does everything this ticket asks, and that you hit no other problems. Go through every step in How to verify and check every guardrail. Follow the `agent-testing` skill for all testing. Iterate with screenshots until no screen gives away that AI made it. Be your own harshest critic: if a detail would look out of place in Mercury, remove it. If you changed code, run the `sol-loop` skill on it and give Sol this ticket. In your report, show what you saw at each step (screenshot and video paths), give your branch name and head commit, and list anything you did not check.
