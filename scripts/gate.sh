#!/usr/bin/env bash
# ABOUTME: Runs scripts/verify.sh on a machine with no Rust or Node installed, by first fetching
# ABOUTME: the pinned toolchains into a per-user cache. This is the command hecaton gates on.
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
cd "$root"

# Pins. Rust itself is pinned by rust-toolchain.toml; these pin only how it is fetched, plus
# Node for the lesson-extraction stage. Each archive is checked against a hash recorded here
# rather than one fetched from the same server, so a tampered download fails the gate.
RUSTUP_VERSION=1.29.1
RUSTUP_SHA256=dda7234360b7f578ca8b0ddcb80145646fa61a67c1720a5abc7051b35c9fcb71
NODE_VERSION=v24.22.0
NODE_SHA256=cc2f601cee82d9a5a233302fe307abc600727035fd7d2c3a3571755989e95008

if [[ "$(uname -sm)" != "Linux x86_64" ]]; then
    echo "gate.sh: the pinned hashes are for Linux x86_64, not $(uname -sm)." >&2
    echo "On this machine install the toolchains yourself and run scripts/verify.sh." >&2
    exit 2
fi

if [[ -z "${XDG_CACHE_HOME:-}" && -z "${HOME:-}" ]]; then
    echo "gate.sh: set HOME or XDG_CACHE_HOME, which hold the toolchain cache." >&2
    exit 2
fi
cache="${XDG_CACHE_HOME:-$HOME/.cache}/course-natural-gate"
mkdir -p "$cache"

# Two gates starting at once would otherwise install into the same directory together.
exec 9>"$cache/.lock"
flock 9

fetch() {
    local url="$1" sha="$2" out="$3"
    printf 'gate.sh: downloading %s\n' "$url" >&2
    curl --fail --location --progress-bar --output "$out" "$url"
    if ! printf '%s  %s\n' "$sha" "$out" | sha256sum --check --status; then
        echo "gate.sh: checksum mismatch for $url" >&2
        exit 1
    fi
}

export RUSTUP_HOME="$cache/rustup"
export CARGO_HOME="$cache/cargo"
if [[ ! -x "$CARGO_HOME/bin/rustup" ]]; then
    work="$(mktemp -d "$cache/rustup-init.XXXXXX")"
    fetch "https://static.rust-lang.org/rustup/archive/$RUSTUP_VERSION/x86_64-unknown-linux-gnu/rustup-init" \
        "$RUSTUP_SHA256" "$work/rustup-init"
    chmod +x "$work/rustup-init"
    # Its stdout is a shell-setup banner (plus build-script noise, despite --quiet) that
    # does not apply here. Errors arrive on stderr, which stays visible.
    "$work/rustup-init" -y --quiet --no-modify-path --profile minimal --default-toolchain none >/dev/null
    rm -rf "$work"
    # rustup stays at its pin, and a warm cache needs no network.
    "$CARGO_HOME/bin/rustup" set auto-self-update disable
fi
export PATH="$CARGO_HOME/bin:$PATH"
# Installs the channel and targets named in rust-toolchain.toml, then the two components the
# gate's format and lint stages call. Both are no-ops once present.
rustup toolchain install
rustup component add rustfmt clippy

node_dir="$cache/node-$NODE_VERSION-linux-x64"
if [[ ! -x "$node_dir/bin/node" ]]; then
    work="$(mktemp -d "$cache/node.XXXXXX")"
    fetch "https://nodejs.org/dist/$NODE_VERSION/node-$NODE_VERSION-linux-x64.tar.xz" \
        "$NODE_SHA256" "$work/node.tar.xz"
    tar -xJf "$work/node.tar.xz" -C "$work"
    mv "$work/node-$NODE_VERSION-linux-x64" "$node_dir"
    rm -rf "$work"
fi
export PATH="$node_dir/bin:$PATH"

flock --unlock 9
exec bash "$root/scripts/verify.sh"
