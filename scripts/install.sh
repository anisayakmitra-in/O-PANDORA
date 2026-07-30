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

if command -v pandora >/dev/null 2>&1; then
  HERMES_DIR="${HOME}/.hermes"
  CLAUDE_DIR="${HOME}/.claude"
  OPENCODE_DIR="${HOME}/.config/opencode"
  [[ -d "$HERMES_DIR" ]] && pandora import hermes 2>/dev/null || true
  [[ -d "$CLAUDE_DIR" ]] && pandora import claude-code 2>/dev/null || true
  [[ -d "$OPENCODE_DIR" ]] && pandora import opencode 2>/dev/null || true
  pandora setup 2>/dev/null || pandora doctor 2>/dev/null || true
  printf '\nPandora is ready. Try: pandora run "say hello"\n'
fi