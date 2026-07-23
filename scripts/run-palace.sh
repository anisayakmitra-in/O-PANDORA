#!/bin/bash
set -e
PORT="${PANDORA_PALACE_PORT:-3000}"
echo "Pandora Palace starting on :$PORT"
cargo run --release -p pandora-palace &
PID=$!
echo "Palace PID: $PID"
echo "  http://localhost:$PORT/health"
echo "  http://localhost:$PORT/api/packages"
trap "kill $PID 2>/dev/null" EXIT
wait $PID
