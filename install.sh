#!/bin/bash
# Pandora one-line installer
set -e
REPO="anisayakmitra-in/PANDORA-SYSTEMS"
BIN_DIR="${HOME}/.local/bin"
mkdir -p "$BIN_DIR"

case "$(uname -s)" in
    Linux*)   TARGET="x86_64-unknown-linux-gnu" ;;
    Darwin*)  TARGET="x86_64-apple-darwin" ;;
    *)
        echo "Unsupported platform for install.sh. Use WSL on Windows or build manually."
        exit 1
        ;;
esac

LATEST=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" 2>/dev/null | grep '"tag_name"' | head -1 | sed 's/.*"tag_name": "//;s/".*//')
if [ -n "$LATEST" ]; then
    URL="https://github.com/${REPO}/releases/download/${LATEST}/pandora-${TARGET}"
    echo "Downloading pandora ${LATEST}..."
    curl -fsSL "$URL" -o "$BIN_DIR/pandora" && chmod +x "$BIN_DIR/pandora" && echo "pandora installed to $BIN_DIR/pandora" && exit 0
fi

echo "Building from source..."
git clone "https://github.com/${REPO}.git" /tmp/pandora-install
cd /tmp/pandora-install && cargo build --release -p pandora && cp target/release/pandora "$BIN_DIR/pandora"
rm -rf /tmp/pandora-install
echo "pandora installed to $BIN_DIR/pandora"
