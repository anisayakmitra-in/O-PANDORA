#!/usr/bin/env bash
set -euo pipefail

OUT_DIR="${1:-dist/releases}"
mkdir -p "$OUT_DIR"
cargo build --release -p pandora

case "$(uname -s):$(uname -m)" in
  Linux:x86_64) NAME="pandora-linux-x86_64" ;;
  Linux:aarch64|Linux:arm64) NAME="pandora-linux-aarch64" ;;
  Darwin:x86_64) NAME="pandora-macos-x86_64" ;;
  Darwin:arm64) NAME="pandora-macos-arm64" ;;
  *) echo "Unsupported release target: $(uname -s) $(uname -m)" >&2; exit 1 ;;
esac

ASSET="$OUT_DIR/$NAME"
cp target/release/pandora "$ASSET"
if command -v sha256sum >/dev/null 2>&1; then
  sha256sum "$ASSET" > "$ASSET.sha256"
else
  shasum -a 256 "$ASSET" > "$ASSET.sha256"
fi
VERSION="$(awk -F'"' '/^version = / { print $2; exit }' Cargo.toml)"
COMMIT="$(git rev-parse HEAD)"
printf '{"version":"%s","target":"%s","commit":"%s"}\n' "$VERSION" "$NAME" "$COMMIT" > "$ASSET.metadata.json"
printf '%s\n' "$COMMIT" > "$OUT_DIR/pandora-build-commit.txt"
printf 'Created %s, checksum, and metadata.\n' "$ASSET"