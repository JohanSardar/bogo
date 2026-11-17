# Crypto and relay v1 contract

This implements ARCHITECTURE §3.4, §3.11–3.12 and §6. Nothing connects to an
Apple Bridge Mac service. The relay is a separate binary that clients reach
only over HTTPS. It stores ciphertext and routing metadata and has no E2E,
approval, or APNs private key. Every install uses the public relay unless
`[relay] url` names another; [self-hosting](self-hosting-relay.md) shows how to
run one.

## Envelope

All integers are unsigned big endian. IDs are raw bytes inside the envelope:

```
"BOGOE2E\0" | version:u8=1 | direction:u8 | type:u8 | msgId:16
| encLength:u32=32 | enc:32 | ciphertextLength:u32 | ciphertext
```

Direction 0 is Mac → phone; 1 is phone → Mac. The first 27 bytes are AAD.
The suite is DHKEM(X25519, HKDF-SHA256) / HKDF-SHA256 / ChaCha20-Poly1305,
HPKE authenticated mode, with `bogo/e2e/v1 || bogoId:32 || approverId:32`
as info. Every seal uses a new ephemeral key. Limits are 1 MiB plaintext and
1 MiB + 83 bytes total. Empty plaintext is supported.

`Header.kind` is the route's message discriminator. The route defines its
payload codec and discriminator assignments; these crates never interpret a
request or decision payload. After authentication the route ignores unknown
noncritical types; this version rejects all critical (high-bit) types.
`peek_id` supports checking an endpoint's durable tombstone before decryption.
Only record an ID after authentication and a durable effect; never acknowledge
before that. The route must keep replay records, boot/session binding and
idempotent effects; HPKE itself does not prevent replay. Outbox retries reuse
the same sealed bytes. `request_message_id` hashes the full 40-byte request ID
and 32-byte approver ID. Other IDs use `message_id()` once per outbox item.

## Mailbox transport

JSON binary fields use lowercase hex. Mailboxes are SHA-256(Ed25519 public key).
A client starts with `POST /v1/auth/challenge {"pub": <32-byte hex>}`. The relay
returns a random 32-byte challenge, valid for 60 seconds. The signature input is:

```
"bogo/relay/auth/v1\0" || serviceLength:u32 || service UTF-8
|| mailbox:32 || challenge:32
```

The service is the `host:port` the client addressed, the port always written
(443 when the URL has none). The relay knows its own: `--service` (or
`BOGO_RELAY_SERVICE`), required whenever the relay serves TLS itself or listens
beyond loopback. Only a relay on loopback behind something that routes by the
requested hostname (a Cloudflare quick tunnel, Caddy's site matcher) takes it
from the request's Host; behind a front that forwards other hostnames, such as
Tailscale Funnel, `--service` is needed too. Either
way a relay that forwards another relay's challenge cannot use the answer: the
client signed the host it meant to reach, and that is not this relay's name.
The challenge binds both that service and mailbox. `POST /v1/auth/verify {pub,sig}` consumes it even
on a bad signature and returns a random 32-byte hex bearer token valid for one
hour. All remaining endpoints require that bearer. Tokens and challenges are
bounded in memory (4096 each), never stored or logged, and vanish on restart.
Clients reconnect/authenticate when they get 401. TLS certificates and hostnames
are always checked; redirects are disabled. Keep client/key values out of logs.

- `POST /v1/allow {senders:[mailbox]}` atomically replaces the authenticated
  recipient's allowlist (at most 1000 entries); it starts empty.
- `POST /v1/send {to,msg_id,body}` stores a sealed envelope BLOB. `msg_id` is
  16 bytes and must match its header. The sender must be on the recipient's
  allowlist. Equal `(to,msg_id,sender,body)` retries return 200. A changed
  sender or body for a pending ID returns 409; so does a full mailbox.
- `POST /v1/fetch {wait:bool}` returns up to 32 `{msg_id,from,body}` objects,
  without deleting anything. A long poll waits up to 25 seconds. Subscribe
  before reading so a concurrent send cannot lose its wake-up.
- `POST /v1/ack {msg_ids:[id]}` deletes only from the authenticated recipient.
- `POST /v1/retract {to,msg_ids:[id]}` deletes only that sender's deliveries.
  Ack/retract accept at most 1000 IDs, and absent IDs succeed idempotently.

The relay validates only outer envelope framing, never decrypts or interprets
payloads. A malicious relay may still drop, forge routing fields, repeat or
reorder data: endpoints must authenticate HPKE and signatures and check session
binding. A relay cannot enforce that a malicious client actually encrypted its
bytes; privacy depends on endpoints using `bogo-e2e`.

SQLite uses WAL and synchronous FULL. Allowlist replacement, send, ack and
retract commit before success. Messages have recipient, message ID, sender,
opaque body and creation time only. Retention is explicitly set with
`--max-message-age SECONDS`, and expired messages are removed on send/fetch.
Retention is never approval expiry. Endpoint obligations must be resent if
still live. Endpoint deduplication also covers messages resent after relay ack.

## Pairing rooms

A room carries one registration between its owner (the Mac) and one other
mailbox (the phone). Its 16-byte ID is a hash of the pairing code's secret, so
only the two devices holding the code can name it. Rooms are never listed. The
relay sees each body as opaque bytes: endpoints seal every room message with
the code's room key, and the relay never authorizes registration. Every body is
`{room, …}` JSON with hex fields; IDs never appear in URLs.

- `POST /v1/room/open {room}` creates or refreshes a room. Only its owner can
  refresh it; anyone else gets 403.
- `POST /v1/room/send {room,msg_id,body}` carries up to 64 KiB. Repeated identical
  IDs are idempotent; a changed body for an ID is 409.
- `POST /v1/room/fetch {room}` returns the other side's `{msg_id,from,body}`
  messages; each endpoint deduplicates IDs.
- `POST /v1/room/close {room}` removes a room, owner only.

The first mailbox other than the owner to send or fetch takes the room's one
other seat, as in croc, where a third client gets "room full"; everyone else
gets 403, and an unknown room is 404. At most 1024 rooms, 4 per owner, and 16
messages per room are held in memory. Rooms close 120 seconds after their
owner's last refresh; the Mac refreshes every 30 seconds while `bogo approver
add` runs. Restart removes all rooms. Abort means closing the room and running
the command again, which makes a new code.

## iPhone route messages

`bogo-approver-iphone` implements ARCHITECTURE §3.11 on top of the envelope and
mailbox above. Envelope payloads are UTF-8 JSON objects with a `type` field and
camelCase fields; byte fields are lowercase hex. The JSON `type` must agree with
the header's type byte, or the message is acknowledged and dropped.

| Mac → phone | byte | fields |
|---|---|---|
| `request` | 1 | `reqId` (40), `bootSeq`, `bytes` (exact request bytes), `pauseEpoch`, `armDelayMs` |
| `settled` | 2 | `reqId` |
| `ended` | 3 | `reqId`, `why`: callEnded, sessionClosed, ownerCancel, expired, agentRevoked, approverRevoked, agentPaused, otherBoot, unknown |
| `boot` | 4 | `bootSeq`, `bootId` (32) |
| `outcome` | 5 | `reqId`, `outcome`: live, executed, notSent, denied, ended, unknown, noRecord |
| `registered` | 6 | `txId`, `approverId` |
| `revokedAck` | 7 | none |

| Phone → Mac | byte | fields |
|---|---|---|
| `approval` | 1 | `reqId`, `alg` (1 = P-256), `sig`: raw `r ‖ s` over `"BOGO-APPROVAL\0v1" ‖ bytes` |
| `denial` | 2 | `reqId`, optional `note` |
| `pause` | 3 | `agentId`, `epoch` |
| `seen` | 4 | `reqId` |
| `pushToken` | 5 | `tokenSeq`, `token` (32), `env` (development or production), `encodingVersions` |
| `sync` | 6 | `reqIds` |
| `regResend` | 7 | `txId` |
| `removed` | 8 | none |

Message IDs: a request uses `request_message_id(reqId, approverId)`, so every
resend of it is the same ID and the same sealed bytes. Notices use a fresh ID per
outbox item. The phone's IDs are its own outbox IDs, reused on every resend.

Every message is safe to apply late, early or twice. Both sides record an ID
only after its effect is durable and acknowledge only after that; an ID seen
again is acknowledged without being applied. On the Mac, each decision is
checked in this order before anything is counted: approver revoked, unknown
request (including one from an earlier boot), already settled or ended, approver
not asked, duplicate, bad signature. Each check is logged, so a replay can add an
`ignored` line but never a second effect. `removed` revokes the approver and ends
the requests that can no longer reach their threshold, then `revokedAck`
is sent (again for every repeat). `sync` answers one `outcome` per request ID
from the log. `boot` tells the phone that requests from earlier boots are over.
The Mac allowlists every approver it knows, revoked ones included, so a revoked
phone's late messages arrive and are rejected rather than silently dropped.

The push is only a wake-up; the mailbox is the queue. Each delivery attempt
sends the request to the mailbox and an APNs wake at the same time. Once APNs
accepts a wake for a request, resends of it carry no more wakes. A 410, or a 400
BadDeviceToken or DeviceTokenNotForTopic, stops wake-ups to that token until a
newer `pushToken` arrives.

### Registration in the pairing room

`bogo approver add` shows the pairing code as a QR and as text:
`bogo1-` and 24 characters from `abcdefghijkmnpqrstuvwxyz23456789` (the
15-byte secret `s`, base32, in groups of four), plus `@host[:port]` when the
Mac's relay is not the public one. The phone uses that relay for the pairing
and for every message to that Mac afterwards. From the secret:

- `room = SHA-256("bogo-pair-room/v1" ‖ s)[0..16]`, which also stands as the
  transcript's `offerId`;
- `roomKey = HKDF-SHA256(salt = none, ikm = s, info = "bogo-pair-key/v1")`.

Room bodies are `phase ‖ ChaCha20-Poly1305(roomKey, nonce 0^11 ‖ phase, AAD
"bogo-pair/v1" ‖ room ‖ phase, payload)`; each phase is sent once per room. A
body that does not open is ignored, so only a holder of the code can speak in
the room, and the relay sees no key, nonce, label or name. The message ID of
each phase is `SHA-256("bogo-reg-msg" ‖ room ‖ phase)[0..16]`, so repeats are
idempotent.

| phase | from | payload |
|---|---|---|
| 0x0b MAC_OFFER | Mac | `ePubM` (X25519, 32) |
| 0x01 HELLO | phone | `ePubP` (X25519, 32) |
| 0x02 COMMIT | Mac | `Cm = SHA-256("bogo-reg-commit" ‖ ePubM ‖ ePubP ‖ Nm)` |
| 0x03 PHONE_NONCE | phone | `Np` (16) |
| 0x04 REVEAL | Mac | `Nm` (16) |
| 0x05 MAC_CONFIRMED | Mac | sealed `{macE2e, macRelay, bogoId, macName, challenge}` |
| 0x06 REG_SUBMIT | phone | sealed `{approvalKey, proof, phoneE2e, phoneRelay, attestation, pushToken, pushEnv, encodingVersions}` |
| 0x07 REGISTERED | Mac | sealed `{txId, approverId}` |
| 0x08 REFUSED | Mac | sealed `{why}` |
| 0x09 MAC_ABORT / 0x0a PHONE_ABORT | either | empty |

The room key proves each side holds the code, so no digits are compared. The
phone checks `Cm` against the reveal, and the transcript is unchanged, so the
proof of possession and App Attest hash below are exactly as before. Then:

- `th = SHA-256("bogo-reg-transcript/v1" ‖ offerId ‖ ePubM ‖ ePubP ‖ Cm ‖ Nm ‖ Np)`
- `K = HKDF-SHA256(salt = th, ikm = X25519(eM, eP), info = "bogo-reg-key/v1")`
- Phase `p` is sealed with ChaCha20-Poly1305 under `K`, nonce
  `0^11 ‖ p`, AAD `"bogo-reg/v1" ‖ offerId ‖ p`; each phase is sealed once.
- `txId = SHA-256(th)`, `approverId = SHA-256("bogo-approver-id/v1" ‖ txId)`.
- `proof` is the approval key's signature over `"BOGO-REG\0v1" ‖ th ‖ challenge`.
- App Attest's client data hash is `registration_client_data_hash(th,
  challenge, approvalKey, phoneE2e, phoneRelay)`.

The Mac sends MAC_CONFIRMED once the reveal is out; an interrupted command sends
MAC_ABORT. Before committing, it checks that the room peer's mailbox is `SHA-256(phoneRelay)`, the proof, encoding version 1, and the
App Attest evidence. Evidence that is present is always verified. Missing
evidence is accepted only by a build with the `test-registration` feature
(recorded as `simulator-test`), and every other build refuses it. A duplicate
approver ID, txId or approval key is refused. The Mac logs
`approver_registered` durably, then answers REGISTERED in the room and also
sends `registered` to the phone's mailbox. It re-sends `registered` on every boot
and on `regResend`. A room stays up for 3 seconds after the Mac's last phase
so that the phone can read it.

## Approval verifier and attestation

`bogo_verify::verify` accepts tag 1, SEC1 P-256 public keys and a raw 64-byte
`r || s` signature over SHA-256 of the exact message bytes. DER signatures and
unknown tags fail. The C entry point `bogo_rs_verify` has the architecture's ABI;
its documented unsafe pointer conversion is this crate's sole unsafe exception.
No panic crosses the ABI. Tag 0xff00 is compiled only with `test-approvers`;
the optional Ed25519 dependency is absent from this crate's default graph.
Relay mailbox Ed25519 auth remains present independently of approval verification.

`bogo-attest` embeds Apple's App Attestation Root from
<https://www.apple.com/certificateauthority/Apple_App_Attestation_Root_CA.pem>.
Its DER SHA-256 is
`1cb9823ba28ba6ad2d33a006941de2ae4f513ef1d4e831b9f7e0fa7b6242c932`.
No system trust or network is used. Each public named step returns only its step
name on failure, without input bytes. `verify` executes all steps in order:
CBOR/format, certificate chain (including constraints, signatures and time),
nonce, key ID, RP ID hash, zero counter, AAGUID/environment, credential ID, COSE
key agreement with the certificate (the key must end the authenticator data),
and, only when the ED flag is set, Apple's validation category and bundle version.

Real App Attest authenticator data (seen on a real iPhone on 2026-11-16) is 164
bytes, flags `0x40`, with no extension map, so category and bundle version are
normally absent and recorded as null. The caller supplies trusted time, the exact
App ID prefix + bundle ID, allowed environments and the bundle versions to accept
if a map is present. When present, production accepts categories 2 (TestFlight)
and 4 (App Store) and explicitly allowed development accepts 3; the category is a
four-byte little-endian UInt32 CBOR byte string.
Certificate signing digests (SHA-256/SHA-384) are selected independently of
the P-256/P-384 issuer curve, as in Apple's published sample chain.
The registration driver computes `registration_client_data_hash` from its own
transcript hash, one-use challenge and submitted registration keys; it must
consume the challenge and reject previously registered transactions/keys. A
verified receipt is returned as opaque bytes for future fraud-risk assessment;
this crate does not contact Apple's receipt service or validate later assertions.
A real-device attestation and App ID prefix provisioning-profile check remain
part of the device session.

## APNs

`bogo-apns` reads the referenced `.p8` at runtime, zeroizes the temporary PEM,
uses ES256 JWT provider auth over HTTP/2 and caches tokens for 50 minutes.
The client only emits the fixed generic BOGO wake payload and routing IDs.
Its collapse ID is the SHA-256 of the full request ID (64 ASCII bytes), so all
40 request-ID bytes distinguish notifications. APNs rejection details expose
only status, Apple's reason and request UUID, never auth/key data. The caller
owns retry and device-token/environment updates. Sandbox and production hosts
are fixed and redirects disabled.

## Local use

Generate a short-lived certificate outside the repository, with OpenSSL:

```sh
mkdir -p /tmp/bogo-relay-local
chmod 700 /tmp/bogo-relay-local
openssl req -x509 -newkey ec -pkeyopt ec_paramgen_curve:P-256 -nodes \
  -keyout /tmp/bogo-relay-local/key.pem -out /tmp/bogo-relay-local/cert.pem \
  -days 2 -subj /CN=localhost -addext subjectAltName=DNS:localhost \
  -addext basicConstraints=critical,CA:FALSE
chmod 600 /tmp/bogo-relay-local/key.pem
cargo build -p bogo-relay
cargo run -p bogo-relay-proto --example conversation -- \
  target/debug/bogo-relay /tmp/bogo-relay-local
cargo run -p bogo-attest --example steps -- /tmp/bogo-relay-local/cert.pem
cargo run -p bogo-verify --example exact_bytes
cargo run -p bogo-verify --features test-approvers --example exact_bytes
```

The conversation example starts its own local TLS relay on a free port, uses
three independent in-memory mailbox keys, checks refusals, exchanges envelopes,
inspects SQLite, kills/restarts the relay, checks delivery and pairing rooms,
and stops it on exit. Use a fresh directory/database per run. Run `cargo run -p
bogo-apns --example sandbox` only when deliberately checking the owner's
credentials; it sends one made-up token to Apple's sandbox using the explicitly
authorized key path.

`bogo-relay` alone serves plain HTTP on `127.0.0.1:8080` for an HTTPS proxy,
tunnel or cloud service in front, keeping `bogo-relay.sqlite` in the current
directory, 30 days' retention and 1000 messages per mailbox. To serve HTTPS
itself:

```sh
target/debug/bogo-relay --listen 127.0.0.1:8843 \
  --database /tmp/bogo-relay-local/manual.sqlite \
  --cert /tmp/bogo-relay-local/cert.pem --key /tmp/bogo-relay-local/key.pem
```

SIGHUP reloads the configured certificate and key atomically; a failed reload
keeps the current certificate. Protect the relay's state directory and TLS key
with filesystem permissions.
