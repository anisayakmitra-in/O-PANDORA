#!/usr/bin/env bash
set -euo pipefail

REPO="anisayakmitra-in/O-PANDORA"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [[ -f "$SCRIPT_DIR/install-cli.sh" ]]; then
  bash "$SCRIPT_DIR/install-cli.sh"
else
  TMP_INSTALLER="$(mktemp)"
  trap 'rm -f "$TMP_INSTALLER"' EXIT
  curl --fail --location --silent --show-error "https://raw.githubusercontent.com/$REPO/main/scripts/install-cli.sh" -o "$TMP_INSTALLER"
  bash "$TMP_INSTALLER"
fi

INSTALL_DIR="${PANDORA_INSTALL_DIR:-$HOME/.local/bin}"
PANDORA_BIN="$INSTALL_DIR/pandora"
if [[ ! -x "$PANDORA_BIN" ]]; then
  PANDORA_BIN="$(command -v pandora || true)"
fi
if [[ -z "$PANDORA_BIN" || ! -x "$PANDORA_BIN" ]]; then
  echo "Pandora was installed, but the binary could not be located." >&2
  exit 1
fi

"$PANDORA_BIN" --version
if ! "$PANDORA_BIN" doctor; then
  printf '\nPandora installed, but doctor reported issues. Run `pandora doctor --strict` for details.\n' >&2
  exit 1
fi

printf '\nPandora is installed at %s.\n' "$PANDORA_BIN"
printf 'Next steps:\n  pandora setup\n  pandora run "inspect this project"\n'