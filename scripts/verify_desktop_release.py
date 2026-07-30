import argparse
import hashlib
import re
from pathlib import Path


def safe_artifact(root: Path, relative_name: str) -> Path:
    relative = Path(relative_name.lstrip("*"))
    if relative.is_absolute() or ".." in relative.parts:
        raise SystemExit(f"unsafe checksum path: {relative_name}")
    artifact = (root / relative).resolve()
    try:
        artifact.relative_to(root.resolve())
    except ValueError as error:
        raise SystemExit(f"checksum path escapes release directory: {relative_name}") from error
    return artifact


def verify_checksum_manifest(root: Path, manifest: Path) -> set[Path]:
    entries = set()
    lines = [line.strip() for line in manifest.read_text(encoding="utf-8").splitlines() if line.strip()]
    if not lines:
        raise SystemExit(f"empty checksum manifest: {manifest.name}")
    for line in lines:
        fields = line.split()
        if len(fields) < 2 or not re.fullmatch(r"[0-9a-fA-F]{64}", fields[0]):
            raise SystemExit(f"invalid checksum entry: {manifest.name}")
        artifact = safe_artifact(root, fields[1])
        if not artifact.is_file():
            raise SystemExit(f"checksum target is missing: {fields[1]}")
        actual = hashlib.sha256(artifact.read_bytes()).hexdigest()
        if actual.lower() != fields[0].lower():
            raise SystemExit(f"desktop checksum mismatch: {artifact.name}")
        entries.add(artifact)
    return entries


def main() -> None:
    parser = argparse.ArgumentParser(description="Verify Pandora desktop release artifacts")
    parser.add_argument("directory", type=Path)
    parser.add_argument("--allow-unsigned", action="store_true", help="Allow unsigned release-candidate artifacts")
    args = parser.parse_args()
    root = args.directory
    if not root.is_dir():
        raise SystemExit(f"release directory does not exist: {root}")

    signatures = [path for path in root.rglob("*.sig") if path.is_file() and path.stat().st_size]
    if not signatures and not args.allow_unsigned:
        raise SystemExit("no non-empty Tauri signatures found")
    for signature in signatures:
        artifact = signature.with_suffix("")
        if not artifact.is_file():
            raise SystemExit(f"signature has no matching artifact: {signature.name}")

    checksums = list(root.glob("pandora-desktop-*.sha256"))
    if not checksums:
        raise SystemExit("no desktop checksum manifests found")
    checked_artifacts = set()
    for manifest in checksums:
        checked_artifacts.update(verify_checksum_manifest(root, manifest))
    for signature in signatures:
        if signature.with_suffix("").resolve() not in checked_artifacts:
            raise SystemExit(f"signed artifact is absent from checksum manifests: {signature.name}")

    build_commits = list(root.glob("pandora-desktop-*-build-commit.txt"))
    if not build_commits:
        raise SystemExit("no desktop build commit metadata found")
    for metadata in build_commits:
        if not re.fullmatch(r"[0-9a-f]{40}\n?", metadata.read_text(encoding="utf-8")):
            raise SystemExit(f"invalid desktop build commit metadata: {metadata.name}")

    mode = "signed" if signatures else "unsigned RC"
    print(
        f"verified {mode} desktop artifacts, "
        f"{len(checksums)} checksum manifests, and "
        f"{len(build_commits)} build metadata files"
    )


if __name__ == "__main__":
    main()
