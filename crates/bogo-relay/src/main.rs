//! Standalone ciphertext mailbox. No daemon or E2E dependencies. With a push
//! key it also wakes phones: a push that says only that something is waiting.
use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, State},
    http::{HeaderMap, StatusCode, Uri, header},
    response::Redirect,
    routing::{get, post},
};
use bogo_relay_proto::*;
use clap::Parser;
use rusqlite::{Connection, OptionalExtension, params};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use tokio::sync::watch;

/// Every flag has a default, so `bogo-relay` alone serves plain HTTP on
/// 127.0.0.1:8080 for an HTTPS proxy, tunnel or cloud service in front.
/// With `--cert` and `--key` it serves HTTPS itself.
#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(long, default_value = "127.0.0.1:8080")]
    listen: std::net::SocketAddr,
    #[arg(long, default_value = "bogo-relay.sqlite")]
    database: PathBuf,
    /// TLS certificate chain (PEM). Give it with --key to serve HTTPS directly.
    #[arg(long, requires = "key")]
    cert: Option<PathBuf>,
    #[arg(long, requires = "cert")]
    key: Option<PathBuf>,
    /// The name clients reach this relay by (host:port); every sign-in is bound
    /// to it. Required unless the relay listens on loopback behind a tunnel or
    /// proxy that routes by the requested hostname (a Cloudflare quick tunnel);
    /// then it is the request's Host.
    #[arg(long, env = "BOGO_RELAY_SERVICE")]
    service: Option<String>,
    /// Seconds an undelivered message is kept; never an approval expiry.
    #[arg(long, default_value_t = 30 * 24 * 3600)]
    max_message_age: u64,
    /// Messages one mailbox may hold.
    #[arg(long, default_value_t = 1000)]
    mailbox_capacity: u64,
    /// An APNs key (`.p8`) from the team that owns the app's bundle ID. With
    /// it, the relay wakes phones that a Mac asks it to wake. Read once at
    /// start; never logged.
    #[arg(long, env = "BOGO_RELAY_APNS_KEY", requires_all = ["apns_key_id", "apns_team_id"])]
    apns_key: Option<PathBuf>,
    #[arg(long, env = "BOGO_RELAY_APNS_KEY_ID")]
    apns_key_id: Option<String>,
    #[arg(long, env = "BOGO_RELAY_APNS_TEAM_ID")]
    apns_team_id: Option<String>,
    /// The app's bundle ID.
    #[arg(
        long,
        env = "BOGO_RELAY_APNS_TOPIC",
        default_value = "com.johansardar.applebridge.ios"
    )]
    apns_topic: String,
    /// Tests only: send pushes to this stand-in for Apple (`https://host:port`).
    #[arg(
        long,
        hide = true,
        env = "BOGO_RELAY_APNS_STAND_IN",
        requires = "apns_key"
    )]
    apns_stand_in: Option<String>,
    /// Tests only: an extra root certificate (PEM) for the stand-in.
    #[arg(
        long,
        hide = true,
        env = "BOGO_RELAY_APNS_STAND_IN_CA",
        requires = "apns_stand_in"
    )]
    apns_stand_in_ca: Option<PathBuf>,
}
/// Wakes phones with the relay's push key. It learns a phone's push token and
/// which mailbox has something waiting, and nothing else.
struct Pusher {
    apns: tokio::sync::Mutex<bogo_apns::Client>,
    /// Tokens Apple called invalid: never pushed to again by this process.
    dead: Mutex<std::collections::HashSet<String>>,
    /// Pushes per token in the current minute, so a sender can't flood a phone.
    recent: Mutex<HashMap<String, (Instant, u32)>>,
}
const PUSHES_PER_MINUTE: u32 = 30;
const PUSH_TOKENS: usize = 10_000;
/// A pairing room: its owner (the Mac) and the one other seat (the phone).
struct PairRoom {
    owner: String,
    peer: Option<String>,
    refreshed: Instant,
    messages: Vec<RoomMessage>,
}
struct Memory {
    challenges: HashMap<String, (String, Instant)>,
    sessions: HashMap<String, (String, Instant)>,
    rooms: HashMap<String, PairRoom>,
}
struct App {
    db: Mutex<Connection>,
    memory: Mutex<Memory>,
    service: Option<String>,
    max_age: u64,
    capacity: u64,
    changed: watch::Sender<u64>,
    pusher: Option<Pusher>,
}
type Shared = Arc<App>;
type Failure = StatusCode;
type Result<T> = std::result::Result<T, Failure>;
fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}
fn internal<T>(r: rusqlite::Result<T>) -> Result<T> {
    r.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
fn owner(app: &App, headers: &HeaderMap) -> Result<String> {
    let token = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;
    let mut m = app
        .memory
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    m.sessions.retain(|_, (_, until)| *until > Instant::now());
    m.sessions
        .get(token)
        .map(|(id, _)| id.clone())
        .ok_or(StatusCode::UNAUTHORIZED)
}
fn valid_ids(ids: &[String]) -> bool {
    ids.len() <= 1000 && ids.iter().all(|s| valid_hex(s, 16))
}
/// Rooms close 120 s after their owner's last refresh.
const ROOM_LIFE: Duration = Duration::from_secs(120);
const ROOMS: usize = 1024;
const ROOMS_PER_OWNER: usize = 4;
const ROOM_MESSAGES: usize = 16;
fn prune(m: &mut Memory) {
    let now = Instant::now();
    m.rooms
        .retain(|_, r| now.duration_since(r.refreshed) < ROOM_LIFE);
}
/// The identity a sign-in is bound to: `--service`, or on loopback the host the
/// tunnel or proxy in front routed here. A relay that forwards someone else's
/// challenge cannot use the answer, because the client signs the host it meant
/// to reach and nothing in front of this relay routes that host here.
fn service(app: &App, headers: &HeaderMap, uri: &Uri) -> Option<String> {
    if let Some(s) = &app.service {
        return Some(s.clone());
    }
    let host = headers
        .get(header::HOST)
        .and_then(|h| h.to_str().ok())
        .or_else(|| uri.authority().map(|a| a.as_str()))?
        .to_ascii_lowercase();
    if host.is_empty() || host.contains(['/', '@', ' ']) {
        return None;
    }
    // Clients only speak HTTPS, so a host without a port means 443.
    let has_port = host.rsplit_once(':').is_some_and(|(h, p)| {
        !p.is_empty()
            && p.bytes().all(|b| b.is_ascii_digit())
            && (!h.starts_with('[') || h.ends_with(']'))
    });
    Some(if has_port {
        host
    } else {
        format!("{host}:443")
    })
}

async fn challenge(
    State(app): State<Shared>,
    Json(req): Json<ChallengeRequest>,
) -> Result<Json<Challenge>> {
    if !valid_hex(&req.r#pub, 32) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let mut m = app
        .memory
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let now = Instant::now();
    m.challenges.retain(|_, (_, until)| *until > now);
    if m.challenges.len() >= 4096 {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
    let challenge = random_hex();
    m.challenges.insert(
        req.r#pub,
        (challenge.clone(), now + Duration::from_secs(60)),
    );
    Ok(Json(Challenge { challenge }))
}
async fn verify(
    State(app): State<Shared>,
    headers: HeaderMap,
    uri: Uri,
    Json(req): Json<VerifyRequest>,
) -> Result<Json<Token>> {
    let service = service(&app, &headers, &uri).ok_or(StatusCode::BAD_REQUEST)?;
    let mut m = app
        .memory
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // Remove before verifying: even a bad signature consumes the challenge.
    let (challenge, until) = m
        .challenges
        .remove(&req.r#pub)
        .ok_or(StatusCode::UNAUTHORIZED)?;
    if Instant::now() >= until {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let id =
        verify_auth(&service, &req.r#pub, &challenge, &req.sig).ok_or(StatusCode::UNAUTHORIZED)?;
    m.sessions.retain(|_, (_, until)| *until > Instant::now());
    if m.sessions.len() >= 4096 {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
    let token = random_hex();
    m.sessions.insert(
        token.clone(),
        (id, Instant::now() + Duration::from_secs(3600)),
    );
    Ok(Json(Token { token }))
}
async fn allow(
    State(app): State<Shared>,
    headers: HeaderMap,
    Json(req): Json<Allow>,
) -> Result<StatusCode> {
    let id = owner(&app, &headers)?;
    if req.senders.len() > 1000 || !req.senders.iter().all(|s| valid_hex(s, 32)) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let mut db = app
        .db
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let tx = internal(db.transaction())?;
    internal(tx.execute("DELETE FROM allowlist WHERE recipient=?", [&id]))?;
    for sender in req.senders {
        internal(tx.execute(
            "INSERT OR IGNORE INTO allowlist VALUES (?,?)",
            params![id, sender],
        ))?;
    }
    internal(tx.commit())?;
    Ok(StatusCode::OK)
}
async fn send(
    State(app): State<Shared>,
    headers: HeaderMap,
    Json(req): Json<Send>,
) -> Result<StatusCode> {
    let id = owner(&app, &headers)?;
    if !valid_hex(&req.to, 32)
        || !valid_hex(&req.msg_id, 16)
        || req.body.len() < 166
        || req.body.len() > 2 * (1024 * 1024 + 83)
        || !valid_hex(&req.body, req.body.len() / 2)
    {
        return Err(StatusCode::BAD_REQUEST);
    }
    let body = hex::decode(&req.body).map_err(|_| StatusCode::BAD_REQUEST)?;
    // Only validate outer framing. Never open, decode, or log application data.
    if &body[..8] != b"BOGOE2E\0"
        || body[8] != 1
        || hex::encode(&body[11..27]) != req.msg_id
        || body[27..31] != 32u32.to_be_bytes()
        || u32::from_be_bytes(body[63..67].try_into().unwrap()) as usize != body.len() - 67
    {
        return Err(StatusCode::BAD_REQUEST);
    }
    let mut db = app
        .db
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let tx = internal(db.transaction())?;
    let allowed: bool = internal(tx.query_row(
        "SELECT EXISTS(SELECT 1 FROM allowlist WHERE recipient=? AND sender=?)",
        params![req.to, id],
        |r| r.get(0),
    ))?;
    if !allowed {
        return Err(StatusCode::FORBIDDEN);
    }
    internal(tx.execute(
        "DELETE FROM messages WHERE created < ?",
        [now().saturating_sub(app.max_age as i64)],
    ))?;
    let existing: Option<(String, Vec<u8>)> = internal(
        tx.query_row(
            "SELECT sender,body FROM messages WHERE recipient=? AND msg_id=?",
            params![req.to, req.msg_id],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .optional(),
    )?;
    if let Some((sender, stored)) = existing {
        return if sender == id && stored == body {
            Ok(StatusCode::OK)
        } else {
            Err(StatusCode::CONFLICT)
        };
    }
    let count: u64 = internal(tx.query_row(
        "SELECT COUNT(*) FROM messages WHERE recipient=?",
        [&req.to],
        |r| r.get(0),
    ))?;
    if count >= app.capacity {
        return Err(StatusCode::CONFLICT);
    }
    internal(tx.execute(
        "INSERT INTO messages VALUES (?,?,?,?,?)",
        params![req.to, req.msg_id, id, body, now()],
    ))?;
    internal(tx.commit())?;
    drop(db);
    app.changed.send_modify(|n| *n = n.wrapping_add(1));
    // Only a newly stored message wakes the phone; a resend of it doesn't.
    if let Some(wake) = req.wake {
        let app = app.clone();
        tokio::spawn(async move { push(&app, wake, &req.msg_id).await });
    }
    Ok(StatusCode::OK)
}
async fn push(app: &App, wake: Wake, msg_id: &str) {
    let Some(p) = &app.pusher else { return };
    let env = match wake.env.as_str() {
        "development" => bogo_apns::Environment::Sandbox,
        "production" => bogo_apns::Environment::Production,
        _ => return,
    };
    if !valid_hex(&wake.token, 32) || hold(&p.dead).contains(&wake.token) {
        return;
    }
    {
        let mut recent = hold(&p.recent);
        let now = Instant::now();
        recent.retain(|_, (start, _)| now.duration_since(*start) < Duration::from_secs(60));
        if recent.len() >= PUSH_TOKENS && !recent.contains_key(&wake.token) {
            return;
        }
        let (_, count) = recent.entry(wake.token.clone()).or_insert((now, 0));
        if *count >= PUSHES_PER_MINUTE {
            return;
        }
        *count += 1;
    }
    match p.apns.lock().await.nudge(env, &wake.token, msg_id).await {
        Ok(r) if r.status == 200 => {}
        Ok(r) => {
            if r.status == 410
                || (r.status == 400
                    && matches!(
                        r.reason.as_deref(),
                        Some("BadDeviceToken" | "DeviceTokenNotForTopic")
                    ))
            {
                let mut dead = hold(&p.dead);
                if dead.len() < PUSH_TOKENS {
                    dead.insert(wake.token);
                }
            }
            eprintln!(
                "bogo-relay: Apple refused a push: {} {}",
                r.status,
                r.reason.as_deref().unwrap_or("")
            );
        }
        Err(e) => eprintln!("bogo-relay: push failed: {e}"),
    }
}
fn hold<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|p| p.into_inner())
}
/// Says the relay is up, and whether it wakes phones.
async fn healthz(State(app): State<Shared>) -> Json<Health> {
    Json(Health {
        status: "ok".into(),
        push: app.pusher.is_some(),
    })
}
async fn front() -> &'static str {
    "BOGO relay. It passes sealed messages between a Mac and a phone. https://github.com/JohanSardar/bogo\n"
}
/// Optional install redirect. Whoever controls the relay controls its destination.
/// Install directly from GitHub; see docs/setup.md.
async fn install() -> Redirect {
    Redirect::temporary("https://raw.githubusercontent.com/JohanSardar/bogo/main/install.sh")
}
fn read_messages(app: &App, id: &str) -> Result<Vec<Message>> {
    let db = app
        .db
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    internal(db.execute(
        "DELETE FROM messages WHERE created < ?",
        [now().saturating_sub(app.max_age as i64)],
    ))?;
    let mut stmt = internal(db.prepare(
        "SELECT msg_id,sender,body FROM messages WHERE recipient=? ORDER BY created,rowid LIMIT 32",
    ))?;
    let rows = internal(stmt.query_map([id], |r| {
        Ok(Message {
            msg_id: r.get(0)?,
            from: r.get(1)?,
            body: hex::encode(r.get::<_, Vec<u8>>(2)?),
        })
    }))?;
    internal(rows.collect())
}
async fn fetch(
    State(app): State<Shared>,
    headers: HeaderMap,
    Json(req): Json<Fetch>,
) -> Result<Json<Vec<Message>>> {
    let id = owner(&app, &headers)?;
    let mut change = app.changed.subscribe();
    let deadline = tokio::time::Instant::now() + Duration::from_secs(25);
    loop {
        change.borrow_and_update();
        let messages = read_messages(&app, &id)?;
        if !messages.is_empty() || !req.wait {
            return Ok(Json(messages));
        }
        if tokio::time::timeout_at(deadline, change.changed())
            .await
            .is_err()
        {
            return Ok(Json(Vec::new()));
        }
    }
}
async fn ack(
    State(app): State<Shared>,
    headers: HeaderMap,
    Json(req): Json<Ack>,
) -> Result<StatusCode> {
    let id = owner(&app, &headers)?;
    if !valid_ids(&req.msg_ids) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let mut db = app
        .db
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let tx = internal(db.transaction())?;
    for msg in req.msg_ids {
        internal(tx.execute(
            "DELETE FROM messages WHERE recipient=? AND msg_id=?",
            params![id, msg],
        ))?;
    }
    internal(tx.commit())?;
    Ok(StatusCode::OK)
}
async fn retract(
    State(app): State<Shared>,
    headers: HeaderMap,
    Json(req): Json<Retract>,
) -> Result<StatusCode> {
    let id = owner(&app, &headers)?;
    if !valid_hex(&req.to, 32) || !valid_ids(&req.msg_ids) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let mut db = app
        .db
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let tx = internal(db.transaction())?;
    for msg in req.msg_ids {
        internal(tx.execute(
            "DELETE FROM messages WHERE recipient=? AND sender=? AND msg_id=?",
            params![req.to, id, msg],
        ))?;
    }
    internal(tx.commit())?;
    Ok(StatusCode::OK)
}
async fn room_open(
    State(app): State<Shared>,
    headers: HeaderMap,
    Json(req): Json<Room>,
) -> Result<StatusCode> {
    let id = owner(&app, &headers)?;
    if !valid_hex(&req.room, 16) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let mut m = app
        .memory
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    prune(&mut m);
    if let Some(r) = m.rooms.get_mut(&req.room) {
        if r.owner != id {
            return Err(StatusCode::FORBIDDEN);
        }
        r.refreshed = Instant::now();
        return Ok(StatusCode::OK);
    }
    if m.rooms.len() >= ROOMS
        || m.rooms.values().filter(|r| r.owner == id).count() >= ROOMS_PER_OWNER
    {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
    m.rooms.insert(
        req.room,
        PairRoom {
            owner: id,
            peer: None,
            refreshed: Instant::now(),
            messages: Vec::new(),
        },
    );
    Ok(StatusCode::OK)
}
/// Look a room up for `id`: its owner, or the one other seat, which the first
/// other mailbox to use the room takes. Anyone else is refused.
fn seat<'a>(m: &'a mut Memory, room: &str, id: &str) -> Result<&'a mut PairRoom> {
    if !valid_hex(room, 16) {
        return Err(StatusCode::BAD_REQUEST);
    }
    prune(m);
    let r = m.rooms.get_mut(room).ok_or(StatusCode::NOT_FOUND)?;
    if r.owner != id {
        match &r.peer {
            Some(p) if p != id => return Err(StatusCode::FORBIDDEN),
            Some(_) => {}
            None => r.peer = Some(id.into()),
        }
    }
    Ok(r)
}
async fn room_send(
    State(app): State<Shared>,
    headers: HeaderMap,
    Json(req): Json<RoomSend>,
) -> Result<StatusCode> {
    let id = owner(&app, &headers)?;
    if !valid_hex(&req.msg_id, 16)
        || req.body.len() > 128 * 1024
        || !valid_hex(&req.body, req.body.len() / 2)
    {
        return Err(StatusCode::BAD_REQUEST);
    }
    let mut m = app
        .memory
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let r = seat(&mut m, &req.room, &id)?;
    if let Some(old) = r.messages.iter().find(|m| m.msg_id == req.msg_id) {
        return if old.from == id && old.body == req.body {
            Ok(StatusCode::OK)
        } else {
            Err(StatusCode::CONFLICT)
        };
    }
    if r.messages.len() >= ROOM_MESSAGES {
        return Err(StatusCode::CONFLICT);
    }
    r.messages.push(RoomMessage {
        msg_id: req.msg_id,
        from: id,
        body: req.body,
    });
    Ok(StatusCode::OK)
}
async fn room_fetch(
    State(app): State<Shared>,
    headers: HeaderMap,
    Json(req): Json<Room>,
) -> Result<Json<Vec<RoomMessage>>> {
    let id = owner(&app, &headers)?;
    let mut m = app
        .memory
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let r = seat(&mut m, &req.room, &id)?;
    Ok(Json(
        r.messages
            .iter()
            .filter(|m| m.from != id)
            .cloned()
            .collect(),
    ))
}
async fn room_close(
    State(app): State<Shared>,
    headers: HeaderMap,
    Json(req): Json<Room>,
) -> Result<StatusCode> {
    let id = owner(&app, &headers)?;
    let mut m = app
        .memory
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if !valid_hex(&req.room, 16) {
        return Err(StatusCode::BAD_REQUEST);
    }
    match m.rooms.get(&req.room) {
        None => return Err(StatusCode::NOT_FOUND),
        Some(r) if r.owner != id => return Err(StatusCode::FORBIDDEN),
        Some(_) => {}
    }
    m.rooms.remove(&req.room);
    Ok(StatusCode::OK)
}

#[tokio::main]
async fn main() {
    if run().await.is_err() {
        eprintln!("bogo-relay: startup or service failed");
        std::process::exit(1)
    }
}
async fn run() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if args.service.as_ref().is_some_and(String::is_empty)
        || args.max_message_age == 0
        || args.max_message_age > i64::MAX as u64
        || args.mailbox_capacity == 0
    {
        return Err("invalid limits".into());
    }
    if args.service.is_none() && (args.cert.is_some() || !args.listen.ip().is_loopback()) {
        eprintln!(
            "bogo-relay: give --service host:port (or BOGO_RELAY_SERVICE), the name clients reach this relay by; only a relay on loopback behind a tunnel or proxy can go without it"
        );
        return Err("no service".into());
    }
    let pusher = match &args.apns_key {
        None => None,
        Some(key) => {
            let mut apns = bogo_apns::Client::from_file(
                key,
                args.apns_key_id.as_deref().unwrap_or_default(),
                args.apns_team_id.as_deref().unwrap_or_default(),
                &args.apns_topic,
            )
            .map_err(|e| {
                eprintln!("bogo-relay: {e}");
                e
            })?;
            if let Some(base) = &args.apns_stand_in {
                let ca = args
                    .apns_stand_in_ca
                    .as_ref()
                    .map(std::fs::read)
                    .transpose()?;
                apns = apns.stand_in(base, ca.as_deref())?;
            }
            Some(Pusher {
                apns: tokio::sync::Mutex::new(apns),
                dead: Mutex::new(Default::default()),
                recent: Mutex::new(HashMap::new()),
            })
        }
    };
    let db = Connection::open(&args.database)?;
    db.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=FULL;
        CREATE TABLE IF NOT EXISTS allowlist(recipient TEXT NOT NULL,sender TEXT NOT NULL,PRIMARY KEY(recipient,sender));
        CREATE TABLE IF NOT EXISTS messages(recipient TEXT NOT NULL,msg_id TEXT NOT NULL,sender TEXT NOT NULL,body BLOB NOT NULL,created INTEGER NOT NULL,PRIMARY KEY(recipient,msg_id));")?;
    let app = Arc::new(App {
        db: Mutex::new(db),
        memory: Mutex::new(Memory {
            challenges: HashMap::new(),
            sessions: HashMap::new(),
            rooms: HashMap::new(),
        }),
        service: args.service,
        max_age: args.max_message_age,
        capacity: args.mailbox_capacity,
        changed: watch::channel(0).0,
        pusher,
    });
    let pushing = if app.pusher.is_some() {
        "; it wakes phones with its push key"
    } else {
        ""
    };
    let router = Router::new()
        .route("/", get(front))
        .route("/healthz", get(healthz))
        .route("/install.sh", get(install))
        .route("/v1/auth/challenge", post(challenge))
        .route("/v1/auth/verify", post(verify))
        .route("/v1/allow", post(allow))
        .route("/v1/send", post(send))
        .route("/v1/fetch", post(fetch))
        .route("/v1/ack", post(ack))
        .route("/v1/retract", post(retract))
        .route("/v1/room/open", post(room_open))
        .route("/v1/room/send", post(room_send))
        .route("/v1/room/fetch", post(room_fetch))
        .route("/v1/room/close", post(room_close))
        .layer(DefaultBodyLimit::max(2 * 1024 * 1024 + 4096))
        .with_state(app);
    let (Some(cert), Some(key)) = (args.cert, args.key) else {
        println!(
            "bogo-relay listening with plain HTTP on {}; put HTTPS in front of it (a proxy, tunnel or cloud service){pushing}",
            args.listen
        );
        let listener = tokio::net::TcpListener::bind(args.listen).await?;
        axum::serve(listener, router).await?;
        return Ok(());
    };
    rustls::crypto::ring::default_provider()
        .install_default()
        .map_err(|_| "TLS provider")?;
    let tls = axum_server::tls_rustls::RustlsConfig::from_pem_file(&cert, &key).await?;
    // SIGHUP atomically reloads the configured cert/key after an external renew hook.
    #[cfg(unix)]
    {
        let tls = tls.clone();
        tokio::spawn(async move {
            let Ok(mut signal) =
                tokio::signal::unix::signal(tokio::signal::unix::SignalKind::hangup())
            else {
                return;
            };
            while signal.recv().await.is_some() {
                if tls.reload_from_pem_file(&cert, &key).await.is_err() {
                    eprintln!("bogo-relay: certificate reload failed");
                }
            }
        });
    }
    println!("bogo-relay listening with TLS on {}{pushing}", args.listen);
    axum_server::bind_rustls(args.listen, tls)
        .serve(router.into_make_service())
        .await?;
    Ok(())
}
