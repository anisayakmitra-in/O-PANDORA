#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PALACE_DIR="${PANDORA_PALACE_DIR:-$REPO_ROOT/../k-o-palace}"
PORT="${PANDORA_PALACE_PORT:-3000}"

if [[ ! -f "$PALACE_DIR/Cargo.toml" ]]; then
  echo "K-O Palace lives in the separate repository: https://github.com/anisayakmitra-in/k-o-palace"
  echo "Clone it next to O-PANDORA or set PANDORA_PALACE_DIR to its path."
  exit 1
fi

cd "$PALACE_DIR"
echo "Pandora Palace starting on :$PORT"
cargo run --release -p k-o-palace &
PID=$!
echo "Palace PID: $PID"
echo "  http://localhost:$PORT/health"
echo "  http://localhost:$PORT/api/packages"
trap 'kill "$PID" 2>/dev/null' EXIT
wait "$PID"
