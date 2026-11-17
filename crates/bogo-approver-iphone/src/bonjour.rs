//! Showing a waiting Mac on the local network with macOS's own Bonjour
//! responder, through `/usr/bin/dns-sd`. The advert holds nothing the digits
//! don't already guard: anyone on the network may read it (ARCHITECTURE
//! §3.11).
use bogo_approver::Nearby;
use std::process::Stdio;

/// The longest Bonjour instance name, in bytes.
const MAX_NAME: usize = 63;

/// `dns-sd` runs until it is killed, so a small shell holds it: the shell
/// kills it when its stdin closes, which happens however this process ends,
/// or when it is signalled.
const HOLDER: &str = "/usr/bin/dns-sd \"$@\" >/dev/null 2>&1 & p=$!
trap 'kill $p; exit' INT TERM HUP
read _
kill $p";

/// Starts advertising `n` until the returned child is dropped.
pub fn advertise(n: &Nearby) -> std::io::Result<tokio::process::Child> {
    let mut name = n.name.trim().to_string();
    while name.len() > MAX_NAME {
        name.pop();
    }
    if name.is_empty() {
        name = "Mac".into();
    }
    // Port 9 (discard): the phone reads the TXT record and never connects.
    tokio::process::Command::new("/bin/sh")
        .args(["-c", HOLDER, "sh", "-R", &name, &n.service, "local", "9"])
        .args(&n.txt)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
}
