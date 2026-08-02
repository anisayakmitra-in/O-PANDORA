#!/usr/bin/env bash
set -euo pipefail

REPO="anisayakmitra-in/O-PANDORA"
VERSION="${PANDORA_VERSION:-latest}"
INSTALL_DIR="${PANDORA_INSTALL_DIR:-$HOME/.local/bin}"

case "$(uname -s):$(uname -m)" in
  Linux:x86_64) ASSET="pandora-linux-x86_64"; HASH_TOOL="sha256sum" ;;
  Darwin:x86_64) ASSET="pandora-macos-x86_64"; HASH_TOOL="shasum -a 256" ;;
  Darwin:arm64) ASSET="pandora-macos-arm64"; HASH_TOOL="shasum -a 256" ;;
  *) ASSET=""; HASH_TOOL="" ;;
esac

if [[ -n "${PANDORA_RELEASE_BASE_URL:-}" ]]; then
  BASE_URL="${PANDORA_RELEASE_BASE_URL%/}"
elif [[ "$VERSION" == "latest" ]]; then
  BASE_URL="https://github.com/$REPO/releases/latest/download"
else
  TAG="v${VERSION#v}"
  BASE_URL="https://github.com/$REPO/releases/download/$TAG"
fi

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT
BINARY="$TMP_DIR/$ASSET"
CHECKSUM="$TMP_DIR/$ASSET.sha256"

install_from_source() {
  command -v cargo >/dev/null || { echo "Rust and Cargo are required for source builds." >&2; return 1; }
  local source_root="$TMP_DIR/cargo-root"
  local cargo_args=(install --git "https://github.com/$REPO.git" --locked --bin pandora --root "$source_root" --force)
  if [[ "$VERSION" != "latest" ]]; then cargo_args+=(--tag "v${VERSION#v}"); fi
  cargo "${cargo_args[@]}"
  "$source_root/bin/pandora" --version
  mkdir -p "$INSTALL_DIR"
  install -m 0755 "$source_root/bin/pandora" "$INSTALL_DIR/pandora"
  "$INSTALL_DIR/pandora" --version
}

if [[ -z "$ASSET" ]] || ! curl --fail --location --silent --show-error "$BASE_URL/$ASSET" -o "$BINARY" || \
   ! curl --fail --location --silent --show-error "$BASE_URL/$ASSET.sha256" -o "$CHECKSUM"; then
  cat >&2 <<EOF
Pandora release binary was not available for this platform.
Use PANDORA_SOURCE_BUILD=1 with Rust installed, or choose a published PANDORA_VERSION.
EOF
  [[ "${PANDORA_SOURCE_BUILD:-0}" == "1" ]] || exit 1
  install_from_source
  exit 0
fi

EXPECTED="$(awk '{print $1}' "$CHECKSUM")"
if [[ "$HASH_TOOL" == "sha256sum" ]]; then
  ACTUAL="$(sha256sum "$BINARY" | awk '{print $1}')"
else
  ACTUAL="$(shasum -a 256 "$BINARY" | awk '{print $1}')"
fi
[[ "$EXPECTED" == "$ACTUAL" ]] || { echo "Checksum verification failed." >&2; exit 1; }

verify_provenance() {
  command -v gh >/dev/null || return 1
  local args=(attestation verify "$BINARY" --repo "$REPO" --signer-workflow "$REPO/.github/workflows/cli-release.yml" --deny-self-hosted-runners)
  if [[ "$VERSION" != "latest" ]]; then args+=(--source-ref "refs/tags/v${VERSION#v}"); fi
  gh "${args[@]}" >/dev/null
}
if ! verify_provenance; then
  echo "GitHub CLI authentication and valid Pandora build provenance are required for prebuilt installation." >&2
  if [[ "${PANDORA_SOURCE_BUILD:-0}" == "1" ]]; then
    install_from_source
    exit 0
  fi
  exit 1
fi

"$BINARY" --version
mkdir -p "$INSTALL_DIR"
install -m 0755 "$BINARY" "$INSTALL_DIR/pandora"
case ":$PATH:" in *":$INSTALL_DIR:"*) ;; *) echo "Add $INSTALL_DIR to PATH to run pandora." ;; esac
"$INSTALL_DIR/pandora" --version