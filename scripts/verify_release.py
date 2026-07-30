import argparse
import hashlib
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
    if not fields:
        raise SystemExit(f"empty checksum: {checksum.name}")
    expected = fields[0].lower()
    actual = hashlib.sha256(asset.read_bytes()).hexdigest()
    if expected != actual:
        raise SystemExit(f"checksum mismatch: {asset.name}")


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

    known_assets = {name for names in ASSET_NAMES.values() for name in names}
    for asset in root.iterdir():
        if asset.is_file() and asset.name in known_assets:
            verify_checksum(asset)
            assets.append(asset.name)
    if not assets:
        raise SystemExit("no CLI release assets found")
    for metadata in ("pandora-sbom.cdx.json", "pandora-build-commit.txt"):
        if not (root / metadata).is_file():
            raise SystemExit(f"missing release metadata: {metadata}")
    print(f"verified {len(assets)} CLI assets and release metadata")


if __name__ == "__main__":
    main()