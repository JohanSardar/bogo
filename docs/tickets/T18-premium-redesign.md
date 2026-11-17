# Redesign the whole iPhone app to look premium, for the demo video

## Goal
The iPhone app looks like default SwiftUI: grouped lists, system fonts and colors, and a Review screen that dumps every input, output and empty field. The owner wants to film a demo of an agent trying to buy something and the owner approving it on the phone, and the app has to look like a top fintech app made it: clean, polished, calm and luxurious, with smooth animation. The references are Stripe, Mercury and Coinbase. Change the look of the **whole app**, every screen, with BOGO as the hero.

When this is done, the app has one design system used everywhere: color, type, spacing, corner radii, materials, icons, haptics and motion. Every screen uses it, nothing looks like a default system list, and light and dark mode both look finished.

The heart of the demo is the **Review card**. The owner sketched it on 2026-11-17, top to bottom:

1. **Title**, such as the product's name.
2. **Image**, large and beautifully framed. Never a randomly cropped picture splatted in the middle: fit the whole picture inside a calm rounded frame with a fixed aspect, on a soft neutral background, with a shimmer while it loads and a quiet placeholder if it fails. Handle SVG links too (the real Link request uses one).
3. **Description and price**: a short subtitle such as the merchant, and the price in large type.
4. **Reason**: the agent's reason, clearly marked as what the agent says.
5. **Extra details**: a few quiet rows.
6. **Approve and Deny** at the bottom.

T17 adds the config roles that fill the card: `DisplayFormat.title`, `.subtitle` and `.image`, encoded as format tags 2, 3 and 4. The first money highlight is the price, `text` highlights are extra details, and the justification is the reason. Other tools, like an email or a deploy, use the same card with whatever roles their config sets. A request with no roles still looks clean: the action and tool as the title, and highlights as details.

## What to build
- **Research first.** Study the Stripe, Mercury and Coinbase iPhone apps (also Cash App, Revolut, Robinhood and the Apple Pay sheet if useful): App Store screenshots, product pages, design write-ups and videos of their animations and transitions. Write down what makes them feel premium (type scale, spacing, color restraint, number styling, motion curves, haptics, confirm gestures) in `docs/design.md` in the iPhone repo, with the design system you chose. Then build to it.
- **The Review card** above, with smooth transitions in and out. Approving should feel like a premium payment confirm: for example a hold-to-approve or slide control with progressive haptics, then Face ID, then a satisfying success state, such as a checkmark that draws itself. Keep the arm delay. Deny should be easy but quiet.
- **"All details"**: one tap opens the full, exact signed request (every argument, including empty ones, as T16 shows them today) in a clean sheet. Nothing signed may be hidden without that visible way in.
- **Every other screen** in the same style: the BOGO home (pending requests as rich cards, a beautiful empty state), History (like a bank's transaction list, with clear approved, denied and expired states), Macs and Add Mac, pairing, Settings and Diagnostics, and the tab bar or navigation. The Settings and Diagnostics tabs can move behind one gear or profile button if that reads better; keep every feature working.
- **A demo mode** for filming: a Debug-only way to make the stand-in Mac send a realistic purchase (the Link spend request below), so a full approve flow can be filmed in the Simulator with no real Mac. It must not exist in Release builds.
- Your own ideas that make the demo stronger are welcome. Good candidates: a rich notification that shows the product picture and price, a smooth app icon or launch, and small touches like number animations. Ship the core flow first, then add these if time allows, and list what you added.

## Context
- The iPhone app is `/Users/johansardar/apple-bridge-iphone`. Make your own worktree and branch from `build/bogo` (don't use the main checkout, which is on `main`). The BOGO screens are in `ios/BogoKit/BogoSection/`, the model in `ios/BogoKit/BogoModel/` (`Review.swift` builds highlights, warnings and pictures), the shell in `ios/App/` (`AppleBridgeApp.swift`, `ShellModel.swift`), pairing in `ios/Pairing/`. The stand-in Mac is `ios/BogoKit/BogoRelay/StandInMacLink.swift` with vectors in `StandInVectors/`.
- Read `/Users/johansardar/.t3/projects/bogo/docs/SPEC.md` (the review screen), `docs/tickets/T2-iphone-section.md`, `T14-review-images.md`, `T16-review-readability.md` and `T17-display-roles.md`.
- The real purchase to show: Link `call_write_tool`, `name = "spend-request_create"`, amount 1199 USD, merchantName "Amazon", test true, `lineItem` `[{"name":"Classic 3x3 Rubik's Cube","image_url":"https://upload.wikimedia.org/wikipedia/commons/a/a6/Rubik%27s_cube.svg","quantity":1,"unit_amount":1199}]`, a `context` paragraph, and the reason "Buying the desk toy the user picked, under their $20 budget." Also show a non-purchase, such as an email send, using the same card.
- T17 runs in parallel and changes `RequestBytes.swift` and `ApprovalRequest.swift`. If you need the three roles before it lands, add exactly `.title`, `.subtitle`, `.image` with tags 2, 3, 4 yourself, so the two branches agree.
- Drive the Simulator with the T3 device tools (`device_list`, `device_open`, `device_screenshot`), not `simctl` or `idb` for taps and screenshots.
- Commit in small commits on your own branch, each ending with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`. Don't merge into `build/bogo`.

## Constraints
- Display and interaction only: no change to what is signed, forwarded or logged, to the crypto, or to the relay protocol.
- Don't touch the Apple Bridge Mac repo or the BOGO Rust repo, and don't touch the real iPhone. Simulator only.
- Use only system frameworks: no new third-party packages.

## Guardrails
- Approve, Deny, Face ID, the arm delay, expiry, the mismatch warning, T14's pictures and the full exact argument tree still work and are still reachable.
- The agent's reason and picture are clearly the agent's claims, not facts BOGO checked.
- Every other tab's features still work after the restyle.
- Dynamic Type, VoiceOver labels and Reduce Motion are respected.

## How to verify
1. Show every screen in light and dark, before and after, as Simulator screenshots.
2. Film the demo flow in the Simulator: a Link purchase arrives, the card opens, approve with the hold or slide, Face ID, success. Also film a deny.
3. Show the email request on the same card, a request with no roles, and a request with a mismatch warning.
4. Show "All details" opening the full exact request.
5. Approve one request and show the stand-in Mac verifies the signature as before.
6. `make lint` and `make test` pass.

## Evidence of done
When you are done, use it yourself the way a user would: run the real app and go through it like they do. Check that it works, that it does everything this ticket asks, and that you hit no other problems. Go through every step in How to verify and check every guardrail. Follow the `agent-testing` skill for all testing. Iterate the design with screenshots until it is ultra clean and genuinely looks like Stripe, Mercury or Coinbase made it. If you changed code, run the `sol-loop` skill on it and give Sol this ticket. In your report, show what you saw at each step (screenshot and video paths), give your branch name and head commit, and list anything you did not check.
