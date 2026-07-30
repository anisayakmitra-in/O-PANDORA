#!/usr/bin/env bash
set -euo pipefail

INSTALL_DIR="${PANDORA_INSTALL_DIR:-$HOME/.local/bin}"
TARGET="$INSTALL_DIR/pandora"
if [[ -f "$TARGET" ]]; then
  rm -f "$TARGET"
  printf 'Removed Pandora CLI: %s\n' "$TARGET"
else
  printf 'Pandora CLI is not installed at %s\n' "$TARGET"
fi
printf 'User data was preserved. Remove it separately only if intended.\n'
