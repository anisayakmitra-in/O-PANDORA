#!/usr/bin/env bash
set -euo pipefail

OUT_DIR="${1:-dist/releases}"
TARGET="${PANDORA_TARGET:-}"
mkdir -p "$OUT_DIR"

cargo_args=(build --release -p pandora)
if [[ -n "$TARGET" ]]; then
  rustup target add "$TARGET"
  cargo_args+=(--target "$TARGET")
fi
cargo "${cargo_args[@]}"

if [[ -n "$TARGET" ]]; then
  case "$TARGET" in
    x86_64-unknown-linux-gnu) NAME="pandora-linux-x86_64" ;;
    aarch64-unknown-linux-gnu) NAME="pandora-linux-aarch64" ;;
    x86_64-apple-darwin) NAME="pandora-macos-x86_64" ;;
    aarch64-apple-darwin) NAME="pandora-macos-arm64" ;;
    *) echo "Unsupported Unix release target: $TARGET" >&2; exit 1 ;;
  esac
  BINARY="target/$TARGET/release/pandora"
else
  case "$(uname -s):$(uname -m)" in
    Linux:x86_64) NAME="pandora-linux-x86_64" ;;
    Linux:aarch64|Linux:arm64) NAME="pandora-linux-aarch64" ;;
    Darwin:x86_64) NAME="pandora-macos-x86_64" ;;
    Darwin:arm64) NAME="pandora-macos-arm64" ;;
    *) echo "Unsupported release target: $(uname -s) $(uname -m)" >&2; exit 1 ;;
  esac
  BINARY="target/release/pandora"
fi

ASSET="$OUT_DIR/$NAME"
cp "$BINARY" "$ASSET"
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