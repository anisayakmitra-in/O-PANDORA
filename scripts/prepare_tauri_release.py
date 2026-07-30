import json
import os
import sys
from pathlib import Path

if len(sys.argv) != 2:
    raise SystemExit("usage: prepare_tauri_release.py OUTPUT")
public_key = os.environ.get("TAURI_UPDATER_PUBKEY", "").strip()
endpoint = os.environ.get("TAURI_UPDATER_ENDPOINT", "").strip()
private_key = os.environ.get("TAURI_SIGNING_PRIVATE_KEY", "").strip()
if not public_key or not endpoint or not private_key:
    raise SystemExit("TAURI_UPDATER_PUBKEY, TAURI_UPDATER_ENDPOINT, and TAURI_SIGNING_PRIVATE_KEY are required")
if not endpoint.startswith("https://"):
    raise SystemExit("TAURI_UPDATER_ENDPOINT must use HTTPS")
output = Path(sys.argv[1])
output.parent.mkdir(parents=True, exist_ok=True)
output.write_text(json.dumps({"plugins": {"updater": {"pubkey": public_key, "endpoints": [endpoint]}}}, indent=2) + "\n", encoding="utf-8")
