//! Stamps `bogo --version` with the release version and the commit it was built from.
//! The release workflow sets `BOGO_VERSION` (the tag without its `v`) and `BOGO_COMMIT`;
//! a source build uses the crate version and asks git for the commit.

use std::env;
use std::process::Command;

fn git(args: &[&str]) -> Option<String> {
    let out = Command::new("git").args(args).output().ok()?;
    out.status
        .success()
        .then(|| String::from_utf8(out.stdout).ok())
        .flatten()
        .map(|s| s.trim().to_owned())
        .filter(|s| !s.is_empty())
}

fn main() {
    println!("cargo:rerun-if-env-changed=BOGO_VERSION");
    println!("cargo:rerun-if-env-changed=BOGO_COMMIT");
    // Rebuild when the checkout moves to another commit.
    for path in ["HEAD", "packed-refs"] {
        if let Some(p) = git(&["rev-parse", "--git-path", path]) {
            println!("cargo:rerun-if-changed={p}");
        }
    }
    if let Some(r) = git(&["symbolic-ref", "-q", "HEAD"])
        && let Some(p) = git(&["rev-parse", "--git-path", &r])
    {
        println!("cargo:rerun-if-changed={p}");
    }

    let version = env::var("BOGO_VERSION")
        .ok()
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| env::var("CARGO_PKG_VERSION").unwrap());
    let commit = env::var("BOGO_COMMIT")
        .ok()
        .filter(|c| !c.is_empty())
        .or_else(|| git(&["rev-parse", "--short=12", "HEAD"]))
        .unwrap_or_else(|| "unknown".into());
    println!("cargo:rustc-env=BOGO_VERSION_LINE={version} (commit {commit})");
}
