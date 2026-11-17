# Ship the iPhone app to TestFlight with a public link

## Goal
Get the Apple Bridge iPhone app, which holds BOGO's approver, onto TestFlight with a public link anyone can open to join. The owner wants a link to share and to put behind a "TestFlight" button in both READMEs, so people can try BOGO without building the app. When this is done, a build from the current `main` of `JohanSardar/apple-bridge-iphone` is uploaded and processed. Its export compliance is answered. It is in an external testing group with a public link turned on, and it is submitted for Beta App Review. The report gives the public link and the review state shown in App Store Connect. A build that is uploaded but not in a public group, or a group with no build submitted for review, does not count. The steps must also be written down in the iPhone repo, with no secret in them, so the next release can be done by following them. That includes the final TestFlight build at the end of this project. The app's existing features and its signing settings must not change. If getting a public link and leaving something untouched ever pull against each other, stop and report: don't work around it.

## Context
- The repo is `/Users/johansardar/apple-bridge-iphone`, which is public on GitHub. Make your own worktree and branch from `main`. Don't work in the main checkout.
- The release script is `packaging/release/testflight.py`. The bundle ID is `com.johansardar.applebridge.ios`, with extensions.
- The release settings are in `~/.config/apple-bridge-release/release.env`: paths to the App Store Connect key, the key ID, the issuer, the dedicated release keychain and the distribution identity.
  - Source that file. Never print, copy or commit its values or the key.
- App Store distribution profiles for the app and all four extensions are installed on this Mac. They include Family Controls and expire 2027-10-31.
- The app was never on TestFlight before. It is not yet known whether an App Store Connect app record exists for this bundle ID.
- The BOGO Mac daemon's config has `app_attest = ["development"]`. A TestFlight build uses App Attest's production environment, so a daemon pairing with a TestFlight build needs `"production"` in that list.
  - Write this into the release steps and the user-facing setup notes.
  - Don't change the owner's live `~/.bogo` config.
- Beta App Review reviewers won't have a Mac running the BOGO daemon or the Apple Bridge gateway. The review notes must explain what the app does and what a reviewer can see without one.

## Constraints
- If no App Store Connect app record exists for the bundle ID, stop and report before creating one. The owner wants to be asked first.
- Ask the same way before anything else that is public or hard to undo beyond what this ticket asks for. A public external group, its public link and submitting for Beta App Review are what this ticket asks for.
- Never accept a legal agreement, sign up, pay or start a trial. If App Store Connect blocks on one, stop and report it.
- Don't put the owner's personal email or other personal details anywhere new. Use contact details already in the account. If a required field would need new personal details, stop and report which field.
- Don't change bundle IDs, entitlements or signing settings.
- Don't touch the BOGO Rust repo, the Apple Bridge Mac repo, the live relay or the real iPhone.
- Open a PR against `main` on `JohanSardar/apple-bridge-iphone` for any repo change, and don't merge it. Commits use the owner's name and noreply email, and end with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.

## Guardrails
- The app's features (Apple Bridge and the BOGO section) work as they do now. They ship in the same app.
- No secret, key or token is printed, logged or committed.
- `make lint` and `make test` in the iPhone repo still pass.

## How to verify
1. App Store Connect, read back through its API, shows the build as processed, with export compliance answered.
2. The build is in an external group with the public link turned on, and its Beta App Review state is "Waiting for Review" or later.
3. The public link opens the TestFlight join page for the app (fetch it and show the page title).
4. Following the written release steps from the top would reproduce this release, and a search of the PR finds no secret value.
5. `make lint` and `make test` pass.

## Evidence of done
When you are done, use what you changed the way a user would: run the real app and go through each screen, flow, or caller that runs the code you changed. If it has no user, use it the way its real client would, such as a program calling the API or an agent calling the tool. Check that it works and that it does everything this ticket asks. Go through every step in How to verify and check every guardrail. Skip parts your change can't reach. Match the proof to the change: a text or style change needs one look at the screen it changes, and a new flow needs one walk through it. When the change touches no code that runs for a user or client, such as docs, comments, tests, or CI config, don't run the app: lint, types, and the existing checks are enough. Follow the `agent-testing` skill for all testing. If you changed code, run the `sol-loop` skill on it and give Sol this ticket. In your report, show what you saw at each step: screenshots, a video, or the output. Then list anything you did not check.
