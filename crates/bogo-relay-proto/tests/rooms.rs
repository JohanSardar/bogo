use bogo_relay_proto::{Challenge, SigningKey, Token, auth_bytes};
use ed25519_dalek::Signer;
use serde_json::json;
use std::{
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
};

struct Relay {
    child: Child,
    base: String,
    service: String,
}

impl Relay {
    fn start() -> Self {
        let socket = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let address = socket.local_addr().unwrap();
        drop(socket);
        // `make test` builds every binary first. This test runs from somewhere under
        // the profile directory (target/debug/deps, or target/debug/build/… with newer
        // Cargo), so look upward for the relay.
        let binary = std::env::var_os("BOGO_RELAY_BIN")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| {
                let exe = std::env::current_exe().unwrap();
                exe.ancestors()
                    .map(|dir| dir.join("bogo-relay"))
                    .find(|path| path.is_file())
                    .expect("build bogo-relay first (`make test` does)")
            });
        let mut child = Command::new(binary)
            .args(["--listen", &address.to_string(), "--database", ":memory:"])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let mut ready = String::new();
        BufReader::new(child.stdout.take().unwrap())
            .read_line(&mut ready)
            .unwrap();
        assert!(ready.contains("listening"));
        Self {
            child,
            base: format!("http://{address}"),
            service: address.to_string(),
        }
    }

    async fn challenge(&self, http: &reqwest::Client, key: &SigningKey) -> Challenge {
        http.post(format!("{}/v1/auth/challenge", self.base))
            .json(&json!({"pub": hex::encode(key.verifying_key().as_bytes())}))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }

    async fn login(&self, http: &reqwest::Client) -> String {
        let key = SigningKey::generate(&mut rand::rngs::OsRng);
        let challenge = self.challenge(http, &key).await;
        let sig = key.sign(
            &auth_bytes(
                &self.service,
                &key.verifying_key().to_bytes(),
                &challenge.challenge,
            )
            .unwrap(),
        );
        let token: Token = http.post(format!("{}/v1/auth/verify", self.base))
            .json(&json!({"pub": hex::encode(key.verifying_key().as_bytes()), "sig": hex::encode(sig.to_bytes())}))
            .send().await.unwrap().json().await.unwrap();
        token.token
    }
}

impl Drop for Relay {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[tokio::test]
async fn refused_room_close_does_not_steal_the_phone_seat() {
    let relay = Relay::start();
    let http = reqwest::Client::new();
    let mac = relay.login(&http).await;
    let stranger = relay.login(&http).await;
    let phone = relay.login(&http).await;
    let room = json!({"room": "0123456789abcdef0123456789abcdef"});
    let response = http
        .post(format!("{}/v1/room/open", relay.base))
        .bearer_auth(mac)
        .json(&room)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);
    let response = http
        .post(format!("{}/v1/room/close", relay.base))
        .bearer_auth(stranger)
        .json(&room)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 403);
    // Only fetch/send can claim a seat. A forbidden close must leave it free.
    let response = http
        .post(format!("{}/v1/room/fetch", relay.base))
        .bearer_auth(phone)
        .json(&room)
        .send()
        .await
        .unwrap();
    assert_eq!(
        response.status().as_u16(),
        200,
        "a forbidden close claimed the seat and locked out the real phone"
    );
}
