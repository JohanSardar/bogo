# Add your iPhone

Your iPhone is where you approve. BOGO on your Mac sends it each protected call,
you check it and slide to approve, and Face ID signs your answer. This page
takes you from nothing to your first approval. You need BOGO running on your
Mac first ([Run BOGO on your Mac](daemon.md)).

## 1. Install the app

The approver is the BOGO tab of the Apple Bridge iPhone app.
[Join the TestFlight beta](https://testflight.apple.com/join/dQu75bv1), install
it, open it and allow notifications when it asks, so it can tell you when a
request is waiting.

## 2. Tell your Mac about the iPhone

Add this to `~/.bogo/bogo.toml`:

```toml
[approvers.iphone]
app_id = "AX3C2B78YD.com.johansardar.applebridge.ios"
app_attest = ["production"]
approve_arm_delay = "1s"
```

- `app_id` is the app's App ID. Leave it as shown.
- `app_attest = ["production"]` accepts the TestFlight and App Store app.
- `approve_arm_delay` is how long the Review card waits before you can slide.
  BOGO has no default; pick one. A second or two stops a slide you didn't mean.

You don't need an `[apns]` section or a push key. The relay wakes your phone
when a request is waiting. That push says only "Approval requested": no Mac,
tool, agent or amount (see [the relay protocol](RELAY-PROTOCOL.md#pushes-from-the-relay)).

Restart the daemon so it reads the file: `bogo stop && bogo start`.

## 3. Pair them

In a terminal on the Mac:

```sh
bogo approver add 'My iPhone'
```

**Same network (the usual way).** On the iPhone, open BOGO → Macs → Add Mac.
Your Mac is listed by name; tap it.

**Another network, or cellular.** If your Mac isn't listed, tap
**Mac not listed? Use a code**. The terminal shows a code that starts with
`bogo1-`, with a QR code above it. Tap **Scan code** and point the camera at
the terminal, or type or paste the code, then tap Continue.

Either way, the terminal and the phone now show the same 6 digits. Check that
they match, answer `y` in the terminal, then confirm on the phone with Face ID.
The terminal prints the new approver. If the digits differ, answer `n`: nothing
is registered. The digits, not the code, keep anyone else out, so always
compare them.

Run `bogo approver list` to see it.

## 4. Your first approval

Mark a tool as needing approval (see [configuration](configuration.md)) and
have your agent call it. Your phone gets a notification; tap it, or open BOGO.
The Review card shows the exact call. Slide to approve and confirm with Face
ID, or tap Deny. The Mac runs the call only after your signed approval arrives.

## When something goes wrong

The terminal says what happened:

| Message | What to do |
|---|---|
| `the phone cancelled` | Someone tapped Cancel on the phone. Run `bogo approver add` again. |
| `the owner said the codes do not match` | You answered `n`. Run it again and compare the digits. |
| `the iPhone couldn't finish: …` | The phone says what failed, for example its Keychain or App Attest. It shows the same reason. |
| `the iPhone stopped without saying why` | An older app version. Update it, and look at the phone. |

- **Mac not listed in Add Mac.** The phone and Mac aren't on the same network,
  or the network blocks Bonjour. Use the code instead.
- **"That isn't a BOGO pairing code."** Check what you typed against the
  terminal; codes start with `bogo1-`.
- **No notification.** Check notifications are allowed for the app in
  Settings. A request still waits on the Mac: open BOGO and it shows up.
- **Using your own relay.** Nothing changes here: the code and the local advert
  name your relay, and the phone uses it for that Mac. Your relay wakes phones
  only if you give it a push key ([self-hosting](self-hosting-relay.md#waking-phones));
  without one, open BOGO to see waiting requests, or give the Mac its own
  `[apns]` section.

## In the Simulator

You can try this without an iPhone. Build the app for the Simulator from the
[iPhone app repo](https://github.com/JohanSardar/apple-bridge-iphone#build-and-check)
with `make simulator`; it needs no Apple developer account. The Simulator has
no App Attest, so pair it only with a Mac daemon built for testing:

```sh
cargo build -p bogo --features test-registration
```

Never use that build for real approvals. The Simulator gets no pushes; open
BOGO to see waiting requests.
