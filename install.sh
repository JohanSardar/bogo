#!/bin/sh
# Installs BOGO's Mac programs, `bogo` and `bogo-shim`, from a GitHub release.
#
#   curl -fsSL https://bogo.eilmer.si/install.sh | sh
#
# BOGO_VERSION      a release tag such as v0.1.0 (default: the latest release)
# BOGO_INSTALL_DIR  where the programs go (default: ~/.local/bin)
#
# It needs no sudo and changes nothing outside the install directory. It runs nothing
# it downloaded except `bogo --version`, and only after the checksum matches.
# Running it again upgrades in place.
set -eu

REPO="JohanSardar/bogo"
ARCHIVE="bogo-aarch64-apple-darwin.tar.gz"

say() { printf 'bogo-install: %s\n' "$*"; }
fail() { printf 'bogo-install: %s\n' "$*" >&2; exit 1; }

main() {
    [ "$(uname -s)" = Darwin ] || fail "BOGO runs on macOS only."
    # An Apple Silicon Mac, even when this shell runs under Rosetta.
    [ "$(sysctl -n hw.optional.arm64 2>/dev/null || echo 0)" = 1 ] \
        || fail "BOGO's release is built for Apple Silicon Macs only. Build from source instead: https://github.com/$REPO#quick-start-on-macos"

    dir="${BOGO_INSTALL_DIR:-${HOME:?HOME is not set}/.local/bin}"
    version="${BOGO_VERSION:-}"
    case "$version" in
        "") base="https://github.com/$REPO/releases/latest/download" ;;
        v*) base="https://github.com/$REPO/releases/download/$version" ;;
        *) base="https://github.com/$REPO/releases/download/v$version" ;;
    esac

    tmp="$(mktemp -d)"
    trap 'rm -rf "$tmp"' EXIT INT TERM

    say "downloading ${version:-the latest release}"
    curl -fsSL --proto '=https' --tlsv1.2 -o "$tmp/$ARCHIVE" "$base/$ARCHIVE" \
        || fail "could not download $base/$ARCHIVE"
    curl -fsSL --proto '=https' --tlsv1.2 -o "$tmp/SHA256SUMS" "$base/SHA256SUMS" \
        || fail "could not download $base/SHA256SUMS"

    want="$(awk -v f="$ARCHIVE" '$2 == f { print $1 }' "$tmp/SHA256SUMS")"
    [ -n "$want" ] || fail "SHA256SUMS has no line for $ARCHIVE"
    got="$(shasum -a 256 "$tmp/$ARCHIVE" | awk '{ print $1 }')"
    [ "$got" = "$want" ] || fail "checksum mismatch for $ARCHIVE: expected $want, got $got. Nothing was installed."

    mkdir "$tmp/x"
    tar -xzf "$tmp/$ARCHIVE" -C "$tmp/x"
    mkdir -p "$dir"
    for bin in bogo bogo-shim; do
        [ -f "$tmp/x/$bin" ] || fail "the archive has no $bin"
        # Copy next to the old one, then rename over it, so a running copy is never half-written.
        cp "$tmp/x/$bin" "$dir/.$bin.new"
        chmod 755 "$dir/.$bin.new"
        mv -f "$dir/.$bin.new" "$dir/$bin"
    done

    say "installed $("$dir/bogo" --version) into $dir"
    case ":$PATH:" in
        *":$dir:"*) ;;
        *)
            say "$dir is not on your PATH. Add it with:"
            # shellcheck disable=SC2016 # $PATH is meant literally, for the user's shell.
            printf '\n    echo '\''export PATH="%s:$PATH"'\'' >> ~/.zshrc && exec zsh\n\n' "$dir"
            ;;
    esac
    say "next step: bogo setup"
}

# Everything is inside main, so a download cut off halfway runs nothing.
main "$@"
