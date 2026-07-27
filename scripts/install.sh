#!/bin/bash
set -e

# Pandora one-line installer
# curl -fsSL https://raw.githubusercontent.com/anisayakmitra-in/O-PANDORA/main/scripts/install.sh | sh

BOLD=$(tput bold 2>/dev/null || echo "")
GREEN=$(tput setaf 2 2>/dev/null || echo "")
RESET=$(tput sgr0 2>/dev/null || echo "")

echo "${BOLD}Pandora installer${RESET}"
echo ""

# ── Check Rust ──
if command -v cargo &>/dev/null; then
    echo "${GREEN}OK${RESET} Rust found: $(rustc --version)"
else
    echo "Rust not found. Install it first:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo "Then re-run this script."
    exit 1
fi

# ── Install Pandora ──
echo ""
echo "Installing Pandora..."

if command -v pandora &>/dev/null; then
    CURRENT=$(pandora --version 2>/dev/null | head -1 || echo "unknown")
    echo "Pandora already installed: $CURRENT"
    echo "Updating to latest..."
    cargo install --git https://github.com/anisayakmitra-in/O-PANDORA.git pandora --force 2>&1 | tail -3
else
    cargo install --git https://github.com/anisayakmitra-in/O-PANDORA.git pandora 2>&1 | tail -3
fi

echo "${GREEN}OK${RESET} Pandora installed: $(pandora --version 2>/dev/null | head -1 || echo 'done')"

# ── Auto-detect and import ──
echo ""
HERMES_DIR="${HOME}/.hermes"
CLAUDE_DIR="${HOME}/.claude"
OPENCODE_DIR="${HOME}/.config/opencode"

if [ -d "$HERMES_DIR" ]; then
    echo "${GREEN}Hermes config found${RESET} at $HERMES_DIR"
    echo "Importing connections and skills..."
    pandora import hermes 2>/dev/null && echo "  + Imported from Hermes" || echo "  (nothing to import)"
fi

if [ -d "$CLAUDE_DIR" ]; then
    pandora import claude-code 2>/dev/null || true
fi

if [ -d "$OPENCODE_DIR" ]; then
    pandora import opencode 2>/dev/null || true
fi

# ── Quick setup ──
echo ""
echo "Running quick setup..."
pandora setup 2>/dev/null || pandora doctor 2>/dev/null

echo ""
echo "${BOLD}${GREEN}Pandora is ready.${RESET}"
echo ""
echo "Try:  pandora run \"say hello\""
echo "      pandora new gene my-tool"
echo "      pandora --help"
echo ""
echo "Docs: https://github.com/anisayakmitra-in/O-PANDORA"
