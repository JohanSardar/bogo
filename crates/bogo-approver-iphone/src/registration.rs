//! Registration on the Mac: a pairing code the phone scans or types, its
//! sealed relay room, then proof of possession and App Attest (ARCHITECTURE
//! §3.11). Nothing becomes trusted here; the hub commits only what
//! [`MacRegistration`] hands it.
use crate::{IphoneRoute, wire::*};
use async_trait::async_trait;
use base64::Engine;
use bogo_approver::*;
use bogo_relay_proto::RoomSend;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// How long each side waits for the other during registration.
const STEP_TIMEOUT: Duration = Duration::from_secs(600);
const POLL: Duration = Duration::from_millis(300);
const REFRESH: Duration = Duration::from_secs(30);
/// How long a closing room stays up so the phone can fetch the last phase.
const LINGER: Duration = Duration::from_secs(3);

/// True only in a test build (`test-registration`), which accepts the
/// simulator-only path: a registration without App Attest evidence.
pub const fn accepts_simulator_registration() -> bool {
    cfg!(feature = "test-registration")
}

fn transport(e: impl std::fmt::Display) -> RegError {
    RegError::Transport(e.to_string())
}

/// The Mac's seat in the pairing code's room. Every message in it is sealed
/// with the code's room key, so only the phone holding the code can read or
/// write it, and the relay sees ciphertext only.
struct Room<'a> {
    route: &'a IphoneRoute,
    room: [u8; 16],
    room_hex: String,
    key: [u8; 32],
    peer: Option<String>,
    seen: std::collections::HashSet<String>,
    refresher: tokio::task::JoinHandle<()>,
}

impl Room<'_> {
    async fn send(&self, phase: u8, payload: &[u8]) -> Result<(), RegError> {
        room_send(
            &self.route.relay,
            &self.room_hex,
            &self.room,
            &self.key,
            phase,
            payload,
        )
        .await
    }

    /// Wait for the phone's next phase, accepting only `want` or an abort,
    /// and only from the one phone that joined this room.
    async fn next(&mut self, want: u8) -> Result<Vec<u8>, RegError> {
        let deadline = Instant::now() + STEP_TIMEOUT;
        loop {
            let id = self.room_hex.clone();
            let msgs = self
                .route
                .relay
                .call(|c| {
                    let id = id.clone();
                    async move { c.room_fetch(&id).await }
                })
                .await
                .map_err(transport)?;
            for m in msgs {
                if self.seen.contains(&m.msg_id) {
                    continue;
                }
                self.seen.insert(m.msg_id.clone());
                if self.peer.as_ref().is_some_and(|p| p != &m.from) {
                    continue;
                }
                let Ok(body) = hex::decode(&m.body) else {
                    continue;
                };
                let Some((phase, payload)) = open_room(&self.key, &self.room, &body) else {
                    continue;
                };
                if m.msg_id != hex::encode(phase_msg_id(&self.room, phase)) {
                    continue;
                }
                if phase == PHASE_PHONE_ABORT {
                    return Err(RegError::Aborted("the phone cancelled".into()));
                }
                if phase == want {
                    self.peer.get_or_insert(m.from.clone());
                    return Ok(payload);
                }
            }
            if Instant::now() > deadline {
                return Err(RegError::Aborted("timed out waiting for the phone".into()));
            }
            tokio::time::sleep(POLL).await;
        }
    }

    async fn close(&self) {
        self.refresher.abort();
        room_close(&self.route.relay, &self.room_hex).await;
    }

    /// Tell the phone, give it a moment to fetch the abort, then close the room.
    async fn abort(&self) {
        if self.send(PHASE_MAC_ABORT, &[]).await.is_ok() {
            tokio::time::sleep(LINGER).await;
        }
        self.close().await;
    }
}

async fn room_send(
    relay: &crate::relay::Relay,
    room_hex: &str,
    room: &[u8; 16],
    key: &[u8; 32],
    phase: u8,
    payload: &[u8],
) -> Result<(), RegError> {
    let msg = RoomSend {
        room: room_hex.into(),
        msg_id: hex::encode(phase_msg_id(room, phase)),
        body: hex::encode(seal_room(key, room, phase, payload)),
    };
    relay
        .call(|c| {
            let msg = msg.clone();
            async move { c.room_send(&msg).await }
        })
        .await
        .map_err(transport)
}

async fn room_close(relay: &crate::relay::Relay, room_hex: &str) {
    let _ = relay
        .call(|c| {
            let id = room_hex.to_owned();
            async move { c.room_close(&id).await }
        })
        .await;
}

/// The code as a QR, drawn black on white so it scans on dark terminals too.
fn qr(text: &str) -> String {
    let Ok(code) = qrcode::QrCode::with_error_correction_level(text, qrcode::EcLevel::M) else {
        return String::new();
    };
    let width = code.width();
    let dark = |x: isize, y: isize| {
        x >= 0
            && y >= 0
            && (x as usize) < width
            && (y as usize) < width
            && code[(x as usize, y as usize)] == qrcode::Color::Dark
    };
    let mut out = String::new();
    for y in (-2..width as isize + 2).step_by(2) {
        out.push_str("    \x1b[30;47m");
        for x in -2..width as isize + 2 {
            out.push(match (dark(x, y), dark(x, y + 1)) {
                (true, true) => '█',
                (true, false) => '▀',
                (false, true) => '▄',
                (false, false) => ' ',
            });
        }
        out.push_str("\x1b[0m\n");
    }
    out
}

pub(crate) async fn run(
    route: &IphoneRoute,
    name: &str,
    owner: &dyn OwnerPrompt,
) -> Result<Box<dyn PendingRegistration>, RegError> {
    let code = PairCode::new(&route.cfg.relay_url, &route.cfg.public_relay_url);
    let room = code.room();
    let room_hex = hex::encode(room);
    let e_m = x25519_dalek::StaticSecret::random_from_rng(rand::rngs::OsRng);
    let e_pub_m = x25519_dalek::PublicKey::from(&e_m).to_bytes();
    {
        let id = room_hex.clone();
        route
            .relay
            .call(|c| {
                let id = id.clone();
                async move { c.room_open(&id).await }
            })
            .await
            .map_err(transport)?;
    }
    // Keep the room open while the command runs, on its own connection.
    let refresher = {
        let relay = crate::relay::Relay::new(
            &route.cfg.relay_url,
            &route.cfg.relay_service,
            route.cfg.relay_root_pem.as_deref(),
            route.keys.relay.clone(),
        )
        .map_err(RegError::Transport)?;
        let id = room_hex.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(REFRESH).await;
                let id = id.clone();
                let _ = relay
                    .call(|c| {
                        let id = id.clone();
                        async move { c.room_open(&id).await }
                    })
                    .await;
            }
        })
    };
    let mut seat = Room {
        route,
        room,
        room_hex,
        key: code.room_key(),
        peer: None,
        seen: Default::default(),
        refresher,
    };
    if let Err(e) = seat.send(PHASE_MAC_OFFER, &e_pub_m).await {
        seat.close().await;
        return Err(e);
    }
    let text = code.text();
    owner
        .status(&format!(
            "On your iPhone, open BOGO, tap Add Mac and scan this code, or type it:\n\n    {text}\n\n{}",
            qr(&text)
        ))
        .await;
    match exchange(&mut seat, e_m, e_pub_m, name, owner).await {
        Ok(p) => Ok(p),
        Err(e) => {
            if matches!(e, RegError::Refused(_)) {
                // The sealed REFUSED is already in the room.
                tokio::time::sleep(LINGER).await;
                seat.close().await;
            } else {
                seat.abort().await;
            }
            Err(e)
        }
    }
}

async fn exchange<'a>(
    seat: &mut Room<'a>,
    e_m: x25519_dalek::StaticSecret,
    e_pub_m: [u8; 32],
    name: &str,
    owner: &dyn OwnerPrompt,
) -> Result<Box<dyn PendingRegistration>, RegError> {
    let route = seat.route;
    let e_pub_p: [u8; 32] = seat
        .next(PHASE_HELLO)
        .await?
        .try_into()
        .map_err(|_| RegError::Aborted("malformed hello".into()))?;
    let n_m: [u8; 16] = rand::random();
    let commit = commitment(&e_pub_m, &e_pub_p, &n_m);
    seat.send(PHASE_COMMIT, &commit).await?;
    let n_p: [u8; 16] = seat
        .next(PHASE_PHONE_NONCE)
        .await?
        .try_into()
        .map_err(|_| RegError::Aborted("malformed phone nonce".into()))?;
    seat.send(PHASE_REVEAL, &n_m).await?;
    let shared = e_m.diffie_hellman(&x25519_dalek::PublicKey::from(e_pub_p));
    if !shared.was_contributory() {
        return Err(RegError::Aborted("the phone sent a low-order key".into()));
    }
    // The room key already proves the phone holds the code, so no digits
    // need comparing. The transcript is unchanged, with the room ID as offer ID.
    let t = Transcript {
        offer_id: seat.room,
        e_pub_m,
        e_pub_p,
        commit,
        n_m,
        n_p,
    };
    let th = t.hash();
    let key = phase_key(shared.as_bytes(), &th);
    let challenge: [u8; 32] = rand::random();
    let confirmed = MacConfirmed {
        mac_e2e: B(route.keys.e2e_public.to_vec()),
        mac_relay: B(route.keys.relay.verifying_key().to_bytes().to_vec()),
        bogo_id: B(route.keys.bogo_id.to_vec()),
        mac_name: route.cfg.mac_name.clone(),
        challenge: B(challenge.to_vec()),
    };
    let plaintext = serde_json::to_vec(&confirmed).map_err(transport)?;
    seat.send(
        PHASE_MAC_CONFIRMED,
        &seal_phase(&key, &seat.room, PHASE_MAC_CONFIRMED, &plaintext),
    )
    .await?;
    owner
        .status("The phone has the code. Confirm on the phone with Face ID.")
        .await;
    let sealed = seat.next(PHASE_REG_SUBMIT).await?;
    let submit: RegSubmit = open_phase(&key, &seat.room, PHASE_REG_SUBMIT, &sealed)
        .and_then(|p| serde_json::from_slice(&p).ok())
        .ok_or_else(|| RegError::Aborted("the phone's submission did not open".into()))?;
    let peer = seat.peer.clone().unwrap_or_default();
    let checked = check(route, &submit, &peer, &th, &challenge);
    let new = match checked {
        Ok(new) => new,
        Err(why) => {
            let refused =
                serde_json::to_vec(&RefusedPhase { why: why.clone() }).unwrap_or_default();
            let _ = seat
                .send(
                    PHASE_REFUSED,
                    &seal_phase(&key, &seat.room, PHASE_REFUSED, &refused),
                )
                .await;
            return Err(RegError::Refused(why));
        }
    };
    owner
        .status(&format!(
            "Phone checks passed for \"{name}\"; recording the registration."
        ))
        .await;
    let relay = crate::relay::Relay::new(
        &route.cfg.relay_url,
        &route.cfg.relay_service,
        route.cfg.relay_root_pem.as_deref(),
        route.keys.relay.clone(),
    )
    .map_err(RegError::Transport)?;
    let phone_e2e = bogo_e2e::public_from(&submit.phone_e2e.0)
        .map_err(|_| RegError::Aborted("bad key".into()))?;
    Ok(Box::new(MacRegistration {
        relay,
        room_hex: seat.room_hex.clone(),
        room: seat.room,
        room_key: seat.key,
        key,
        new,
        e2e: route.keys.e2e.clone(),
        bogo_id: route.keys.bogo_id,
        phone_e2e,
        refresher: std::mem::replace(&mut seat.refresher, tokio::spawn(async {})),
    }))
}

/// Every check before anything is trusted. Any failure refuses.
fn check(
    route: &IphoneRoute,
    s: &RegSubmit,
    room_peer: &str,
    th: &[u8; 32],
    challenge: &[u8; 32],
) -> Result<NewApprover, String> {
    let approval_key: [u8; 65] = s
        .approval_key
        .array()
        .ok_or("approval key must be 65-byte SEC1")?;
    let phone_e2e: [u8; 32] = s
        .phone_e2e
        .array()
        .ok_or("phone E2E key must be 32 bytes")?;
    let phone_relay: [u8; 32] = s
        .phone_relay
        .array()
        .ok_or("phone relay key must be 32 bytes")?;
    bogo_e2e::public_from(&phone_e2e).map_err(|_| "phone E2E key is invalid")?;
    let mailbox = bogo_relay_proto::mailbox(&phone_relay);
    if mailbox != room_peer {
        return Err("the submission's relay key is not the phone that joined the room".into());
    }
    if !bogo_verify::verify(
        bogo_verify::P256,
        &approval_key,
        &pop_message(th, challenge),
        &s.proof.0,
    ) {
        return Err("proof of possession does not verify".into());
    }
    if !s.encoding_versions.contains(&1) {
        return Err("the phone does not support request encoding version 1".into());
    }
    let attest = match &s.attestation {
        Some(a) => attest(
            route,
            a,
            th,
            challenge,
            &approval_key,
            &phone_e2e,
            &phone_relay,
        )?,
        None if accepts_simulator_registration() => serde_json::json!({
            "environment": "simulator-test",
            "note": "accepted without App Attest by a test build"
        }),
        None => return Err("App Attest evidence is missing".into()),
    };
    let push = match (&s.push_token, &s.push_env) {
        (Some(t), Some(env))
            if t.0.len() == 32 && (env == "development" || env == "production") =>
        {
            Some(Push {
                token: t.clone(),
                env: env.clone(),
                seq: 0,
            })
        }
        _ => None,
    };
    let route_data = RouteData {
        phone_e2e: B(phone_e2e.to_vec()),
        phone_relay: B(phone_relay.to_vec()),
        mailbox,
        push,
        encoding_versions: s.encoding_versions.clone(),
    };
    let tx = tx_id(th);
    Ok(NewApprover {
        id: approver_id(&tx),
        alg: bogo_verify::P256,
        sig_pk: approval_key.to_vec(),
        tx_id: tx,
        route: serde_json::to_value(route_data).map_err(|e| e.to_string())?,
        attest,
    })
}

fn attest(
    route: &IphoneRoute,
    a: &AttestationEvidence,
    th: &[u8; 32],
    challenge: &[u8; 32],
    approval_key: &[u8; 65],
    phone_e2e: &[u8; 32],
    phone_relay: &[u8; 32],
) -> Result<serde_json::Value, String> {
    let b64 = base64::engine::general_purpose::STANDARD;
    let key_id: [u8; 32] = b64
        .decode(&a.key_id)
        .ok()
        .and_then(|k| k.try_into().ok())
        .ok_or("App Attest key ID must be 32 bytes")?;
    let object = b64
        .decode(&a.object)
        .map_err(|_| "App Attest object is not base64")?;
    let cdh = bogo_attest::registration_client_data_hash(
        th,
        challenge,
        approval_key,
        phone_e2e,
        phone_relay,
    );
    let versions: Vec<&str> = route
        .cfg
        .bundle_versions
        .iter()
        .map(String::as_str)
        .collect();
    let policy = bogo_attest::Policy {
        app_id: &route.cfg.app_id,
        environments: &route.cfg.environments,
        bundle_versions: &versions,
    };
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "clock")?
        .as_secs();
    let v = bogo_attest::verify(&object, &cdh, &key_id, &policy, now).map_err(|e| e.to_string())?;
    use sha2::Digest;
    Ok(serde_json::json!({
        "keyId": a.key_id,
        "environment": match v.environment {
            bogo_attest::Environment::Production => "production",
            bogo_attest::Environment::Development => "development",
        },
        "validationCategory": v.validation_category,
        "bundleVersion": v.bundle_version,
        "attestationSha256": hex::encode(sha2::Sha256::digest(&object)),
    }))
}

struct MacRegistration {
    relay: crate::relay::Relay,
    room_hex: String,
    room: [u8; 16],
    room_key: [u8; 32],
    key: [u8; 32],
    new: NewApprover,
    e2e: bogo_e2e::SecretKey,
    bogo_id: [u8; 32],
    phone_e2e: bogo_e2e::PublicKey,
    refresher: tokio::task::JoinHandle<()>,
}

impl MacRegistration {
    async fn send_sealed(&self, phase: u8, plaintext: &[u8]) -> Result<(), RegError> {
        let sealed = seal_phase(&self.key, &self.room, phase, plaintext);
        room_send(
            &self.relay,
            &self.room_hex,
            &self.room,
            &self.room_key,
            phase,
            &sealed,
        )
        .await
    }

    async fn close(&self) {
        self.refresher.abort();
        room_close(&self.relay, &self.room_hex).await;
    }
}

#[async_trait]
impl PendingRegistration for MacRegistration {
    fn approver(&self) -> &NewApprover {
        &self.new
    }

    /// Answer in the room and as an envelope in the phone's mailbox,
    /// which survives if the room answer is lost.
    async fn answer(self: Box<Self>, committed: &ApproverRecord) -> Result<(), RegError> {
        let registered = RegisteredPhase {
            tx_id: B(committed.tx_id.to_vec()),
            approver_id: B(committed.id.to_vec()),
        };
        let in_room = self
            .send_sealed(
                PHASE_REGISTERED,
                &serde_json::to_vec(&registered).map_err(transport)?,
            )
            .await;
        let route: RouteData =
            serde_json::from_value(committed.route.clone()).map_err(transport)?;
        let msg = MacMsg::Registered {
            tx_id: registered.tx_id.clone(),
            approver_id: registered.approver_id.clone(),
        };
        let id = bogo_e2e::message_id();
        let envelope = bogo_e2e::seal(
            &self.e2e,
            &self.phone_e2e,
            &self.bogo_id,
            &committed.id,
            bogo_e2e::Header {
                direction: bogo_e2e::Direction::MacToPhone,
                kind: msg.kind(),
                id,
            },
            &serde_json::to_vec(&msg).map_err(transport)?,
        )
        .map_err(transport)?;
        let mailbox = self
            .relay
            .call(|c| {
                let (to, envelope) = (route.mailbox.clone(), envelope.clone());
                async move { c.send(&to, id, &envelope).await }
            })
            .await;
        // Give the phone a moment to read the room answer, then close the
        // room. A phone that misses both gets `registered` again on every
        // boot, or by asking with `regResend`.
        tokio::time::sleep(LINGER).await;
        self.close().await;
        match (in_room, mailbox) {
            (Err(e), Err(_)) => Err(e),
            _ => Ok(()),
        }
    }

    async fn refuse(self: Box<Self>, why: &str) {
        let refused = serde_json::to_vec(&RefusedPhase { why: why.into() }).unwrap_or_default();
        if self.send_sealed(PHASE_REFUSED, &refused).await.is_ok() {
            tokio::time::sleep(LINGER).await;
        }
        self.close().await;
    }
}
