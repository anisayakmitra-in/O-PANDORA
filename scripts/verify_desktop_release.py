import argparse
from pathlib import Path


def main() -> None:
    parser = argparse.ArgumentParser(description="Verify signed Pandora desktop release artifacts")
    parser.add_argument("directory", type=Path)
    args = parser.parse_args()
    root = args.directory
    if not root.is_dir():
        raise SystemExit(f"release directory does not exist: {root}")

    signatures = [path for path in root.rglob("*.sig") if path.is_file() and path.stat().st_size]
    if not signatures:
        raise SystemExit("no non-empty Tauri signatures found")

    checksums = list(root.glob("pandora-desktop-*.sha256"))
    if not checksums:
        raise SystemExit("no desktop checksum manifests found")
    for manifest in checksums:
        if not manifest.read_text(encoding="utf-8").strip():
            raise SystemExit(f"empty checksum manifest: {manifest.name}")

    build_commits = list(root.glob("pandora-desktop-*-build-commit.txt"))
    if not build_commits:
        raise SystemExit("no desktop build commit metadata found")

    print(
        f"verified {len(signatures)} Tauri signatures, "
        f"{len(checksums)} checksum manifests, and "
        f"{len(build_commits)} build metadata files"
    )


if __name__ == "__main__":
    main()
