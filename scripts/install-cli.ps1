$ErrorActionPreference = "Stop"
$repo = "https://github.com/anisayakmitra-in/O-PANDORA.git"

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
  throw "Rust and Cargo are required. Install rustup from https://rustup.rs/ and reopen PowerShell."
}

cargo install --git $repo --locked --bin pandora
pandora --version