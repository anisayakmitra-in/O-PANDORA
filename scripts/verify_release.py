import argparse
import hashlib
import json
import re
from pathlib import Path


ASSET_NAMES = {
    "linux": ("pandora-linux-x86_64", "pandora-linux-aarch64"),
    "macos": ("pandora-macos-x86_64", "pandora-macos-arm64"),
    "windows": ("pandora-windows-x86_64.exe", "pandora-windows-arm64.exe"),
}


def verify_checksum(asset: Path) -> None:
    checksum = asset.with_name(asset.name + ".sha256")
    if not checksum.is_file():
        raise SystemExit(f"missing checksum: {checksum.name}")
    fields = checksum.read_text(encoding="utf-8").strip().split()
    if len(fields) < 2:
        raise SystemExit(f"checksum does not name {asset.name}: {checksum.name}")
    if Path(fields[1].lstrip("*")).name != asset.name:
        raise SystemExit(f"checksum target mismatch: {checksum.name}")
    expected = fields[0].lower()
    actual = hashlib.sha256(asset.read_bytes()).hexdigest()
    if expected != actual:
        raise SystemExit(f"checksum mismatch: {asset.name}")


def verify_metadata(asset: Path, build_commit: str) -> None:
    metadata = asset.with_name(asset.name + ".metadata.json")
    if not metadata.is_file():
        raise SystemExit(f"missing metadata: {metadata.name}")
    try:
        values = json.loads(metadata.read_text(encoding="utf-8"))
    except json.JSONDecodeError as error:
        raise SystemExit(f"invalid metadata: {metadata.name}: {error}") from error
    if values.get("target") != asset.name:
        raise SystemExit(f"metadata target mismatch: {metadata.name}")
    if not isinstance(values.get("version"), str) or not values["version"].strip():
        raise SystemExit(f"missing metadata version: {metadata.name}")
    if values.get("commit") != build_commit:
        raise SystemExit(f"metadata commit mismatch: {metadata.name}")

def main() -> None:
    parser = argparse.ArgumentParser(description="Verify Pandora release assets")
    parser.add_argument("directory", type=Path)
    parser.add_argument(
        "--require-cli-platforms",
        default="",
        help="comma-separated platform names: linux,macos,windows",
    )
    args = parser.parse_args()
    root = args.directory
    if not root.is_dir():
        raise SystemExit(f"release directory does not exist: {root}")

    required = [item.strip() for item in args.require_cli_platforms.split(",") if item.strip()]
    unknown = sorted(set(required) - set(ASSET_NAMES))
    if unknown:
        raise SystemExit(f"unknown platforms: {', '.join(unknown)}")
    assets = []
    for platform in required:
        candidates = [root / name for name in ASSET_NAMES[platform]]
        available = next((asset for asset in candidates if asset.is_file()), None)
        if available is None:
            names = ", ".join(asset.name for asset in candidates)
            raise SystemExit(f"missing CLI asset for {platform}; expected one of: {names}")

    build_commit_path = root / "pandora-build-commit.txt"
    sbom_path = root / "pandora-sbom.cdx.json"
    if not build_commit_path.is_file():
        raise SystemExit("missing release metadata: pandora-build-commit.txt")
    if not sbom_path.is_file():
        raise SystemExit("missing release metadata: pandora-sbom.cdx.json")
    build_commit = build_commit_path.read_text(encoding="utf-8").strip()
    if not re.fullmatch(r"[0-9a-f]{40}", build_commit):
        raise SystemExit("invalid build commit metadata")

    known_assets = {name for names in ASSET_NAMES.values() for name in names}
    for asset in root.iterdir():
        if asset.is_file() and asset.name in known_assets:
            verify_checksum(asset)
            verify_metadata(asset, build_commit)
            assets.append(asset.name)
    if not assets:
        raise SystemExit("no CLI release assets found")
    print(f"verified {len(assets)} CLI assets and release metadata")


if __name__ == "__main__":
    main()