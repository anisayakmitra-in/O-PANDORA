# Android support

Pandora supports Android through two deliberately separate paths.

## Termux CLI

Termux is the supported path for local headless execution. Install the F-Droid or GitHub Termux build, then run from Termux:

```bash
pkg install wget
wget -O- https://raw.githubusercontent.com/anisayakmitra-in/O-PANDORA/main/scripts/install-termux.sh | bash
pandora setup
pandora doctor
pandora run "inspect this project"
```

The installer compiles the Rust CLI locally for the device. It requires network access and enough storage for Rust dependencies. Provider credentials stay in Pandora's user data directory; do not paste secrets into shell history.

## Android app / Play Store

The Android GUI is planned as a Tauri mobile client over an authenticated Pandora application/runtime node. It must not expose arbitrary local shell execution or place provider keys in WebView storage.

Before Play Store release, the repository must pass all of these gates:

- Tauri Android project initialized and reproducible from a clean checkout.
- Debug APK built on a GitHub Android runner and installed on an emulator.
- Signed release AAB built from protected CI secrets.
- Target API 36 or newer for submissions after August 31, 2026.
- Runtime permissions, privacy policy, data safety form, and account/data deletion behavior reviewed.
- Authenticated pairing and reconnect tests pass.

Until those gates pass, Pandora does not claim Play Store availability. Desktop and Termux remain the supported Android-adjacent paths.