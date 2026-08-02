#!/usr/bin/env bash
set -euo pipefail

REPO="anisayakmitra-in/O-PANDORA"
VERSION="${PANDORA_VERSION:-latest}"
INSTALL_DIR="${PANDORA_INSTALL_DIR:-$HOME/.local/bin}"
case "$(uname -s):$(uname -m)" in
  Linux:x86_64) ASSET="pandora-linux-x86_64"; HASH_TOOL="sha256sum" ;;
  Darwin:x86_64) ASSET="pandora-macos-x86_64"; HASH_TOOL="shasum -a 256" ;;
  Darwin:arm64) ASSET="pandora-macos-arm64"; HASH_TOOL="shasum -a 256" ;;
  *) echo "Unsupported platform." >&2; exit 1 ;;
esac
if [[ -n "${PANDORA_RELEASE_BASE_URL:-}" ]]; then
  BASE_URL="${PANDORA_RELEASE_BASE_URL%/}"
elif [[ "$VERSION" == "latest" ]]; then
  BASE_URL="https://github.com/$REPO/releases/latest/download"
else
  BASE_URL="https://github.com/$REPO/releases/download/v${VERSION#v}"
fi
TMP_DIR="$(mktemp -d)"; trap 'rm -rf "$TMP_DIR"' EXIT
BINARY="$TMP_DIR/$ASSET"; CHECKSUM="$TMP_DIR/$ASSET.sha256"
curl --fail --location --silent --show-error "$BASE_URL/$ASSET" -o "$BINARY"
curl --fail --location --silent --show-error "$BASE_URL/$ASSET.sha256" -o "$CHECKSUM"
EXPECTED="$(awk '{print toupper($1)}' "$CHECKSUM")"
if [[ "$HASH_TOOL" == "sha256sum" ]]; then ACTUAL="$(sha256sum "$BINARY" | awk '{print toupper($1)}')"; else ACTUAL="$(shasum -a 256 "$BINARY" | awk '{print toupper($1)}')"; fi
[[ "$EXPECTED" == "$ACTUAL" ]] || { echo "Checksum verification failed." >&2; exit 1; }
command -v gh >/dev/null || {
  echo "GitHub CLI authentication is required to verify release provenance." >&2
  exit 1
}
ATTESTATION_ARGS=(attestation verify "$BINARY" --repo "$REPO" --signer-workflow "$REPO/.github/workflows/cli-release.yml" --deny-self-hosted-runners)
if [[ "$VERSION" != "latest" ]]; then ATTESTATION_ARGS+=(--source-ref "refs/tags/v${VERSION#v}"); fi
gh "${ATTESTATION_ARGS[@]}" >/dev/null || {
  echo "Release provenance verification failed; existing binary was not changed." >&2
  exit 1
}
mkdir -p "$INSTALL_DIR"
TARGET="$INSTALL_DIR/pandora"
BACKUP="$TARGET.previous"
if [[ -f "$TARGET" ]]; then cp "$TARGET" "$BACKUP"; fi
install -m 0755 "$BINARY" "$TARGET"
if ! "$TARGET" --version >/dev/null 2>&1; then
  [[ -f "$BACKUP" ]] && mv "$BACKUP" "$TARGET"
  echo "Updated binary failed its health check; previous version restored." >&2
  exit 1
fi
rm -f "$BACKUP"
"$TARGET" --version
