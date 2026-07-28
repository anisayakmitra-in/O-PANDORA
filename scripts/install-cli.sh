#!/usr/bin/env bash
set -euo pipefail

REPO="https://github.com/anisayakmitra-in/O-PANDORA.git"

command -v cargo >/dev/null || {
  echo "Rust and Cargo are required. Install rustup from https://rustup.rs/ and rerun." >&2
  exit 1
}

cargo install --git "$REPO" --locked --bin pandora
pandora --version