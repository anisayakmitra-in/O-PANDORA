#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

REPO="https://github.com/anisayakmitra-in/O-PANDORA.git"

command -v pkg >/dev/null || { echo "This installer must run inside Termux." >&2; exit 1; }

pkg update -y
pkg install -y rust clang make pkg-config openssl git

export OPENSSL_DIR="${PREFIX}"
export OPENSSL_LIB_DIR="${PREFIX}/lib"
export OPENSSL_INCLUDE_DIR="${PREFIX}/include"

cargo install --git "$REPO" --locked --bin pandora

mkdir -p "$HOME/.local/bin"
case ":${PATH}:" in
  *":$HOME/.local/bin:"*) ;;
  *) echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bashrc" ;;
esac

printf '\nPandora CLI installed. Restart Termux or run: source ~/.bashrc\n'
pandora --version