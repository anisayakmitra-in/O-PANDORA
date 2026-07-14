#!/bin/bash
PORT=9091
echo "KUBER Palace starting on :"
cargo run --release -p pandora-palace &
PID=
echo "Palace PID: "
echo "  http://localhost:/health"
echo "  http://localhost:/api/packages"
trap "kill  2>/dev/null" EXIT
wait 
