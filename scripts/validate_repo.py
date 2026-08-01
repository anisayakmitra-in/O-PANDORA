#!/usr/bin/env python3
"""
O-PANDORA Repository Validator

Validates repository invariants for version consistency, identity,
workspace integrity, documentation, and license alignment.

Exit code 0 = all checks pass, non-zero = failures found.
"""

import os
import re
import sys
import tomllib
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent
ERRORS = []
WARNINGS = []


def check(condition: bool, message: str, errors_list: list = ERRORS):
    """Record a check result."""
    if not condition:
        errors_list.append(message)
        print(f"  FAIL: {message}")
    return condition


def check_version():
    """Validate version consistency across the repository."""
    print("\n[A] VERSION CHECKS")

    # Workspace version
    cargo_toml = REPO_ROOT / "Cargo.toml"
    if not cargo_toml.exists():
        check(False, "Root Cargo.toml not found")
        return

    content = cargo_toml.read_text()
    m = re.search(r'version\s*=\s*"(\d+\.\d+\.\d+)"', content)
    if not m:
        check(False, "workspace.package.version not found in Cargo.toml")
        return

    workspace_ver = m.group(1)
    print(f"  workspace.package.version = {workspace_ver}")

    desktop_manifest = REPO_ROOT / "pandora-desktop" / "package.json"
    if desktop_manifest.exists():
        import json
        desktop = json.loads(desktop_manifest.read_text(encoding="utf-8"))
        desktop_ver = desktop.get("version")
        check(desktop_ver == workspace_ver,
              f"Desktop package version ({desktop_ver}) != workspace ({workspace_ver})")
        print(f"  Desktop package version = {desktop_ver}")

    tauri_config = REPO_ROOT / "pandora-desktop" / "src-tauri" / "tauri.conf.json"
    if tauri_config.exists():
        import json
        tauri = json.loads(tauri_config.read_text(encoding="utf-8"))
        tauri_ver = tauri.get("version")
        check(tauri_ver == workspace_ver,
              f"Tauri application version ({tauri_ver}) != workspace ({workspace_ver})")
        print(f"  Tauri application version = {tauri_ver}")

    # README badge version
    readme = REPO_ROOT / "README.md"
    if readme.exists():
        readme_text = readme.read_text()
        badge_m = re.search(r'version-(\d+\.\d+\.\d+)', readme_text)
        if badge_m:
            badge_ver = badge_m.group(1)
            check(badge_ver == workspace_ver,
                  f"README badge version ({badge_ver}) != workspace ({workspace_ver})")
            print(f"  README badge version = {badge_ver}")
        else:
            check(False, "README version badge not found")

        # Badge link destination
        link_m = re.search(r'releases/tag/v(\d+\.\d+\.\d+)', readme_text)
        if link_m:
            link_ver = link_m.group(1)
            check(link_ver == workspace_ver,
                  f"README badge links to v{link_ver}, expected v{workspace_ver}")
            print(f"  README badge link = v{link_ver}")

    # CHANGELOG entry
    changelog = REPO_ROOT / "CHANGELOG.md"
    if changelog.exists():
        cl_text = changelog.read_text()
        changelog_entry = re.search(
            rf"^##\s+\[{re.escape(workspace_ver)}(?:-[^\]]+)?\]",
            cl_text,
            re.MULTILINE,
        )
        check(changelog_entry is not None,
              f"CHANGELOG missing entry for [{workspace_ver}]")
        print(f"  CHANGELOG has [{workspace_ver}] entry: {'YES' if changelog_entry else 'NO'}")
    else:
        check(False, "CHANGELOG.md not found")


def check_identity():
    """Validate project identity — no stale PANDORA-SYSTEMS references."""
    print("\n[B] IDENTITY CHECKS")

    stale_refs = []
    # Files that may legitimately contain PANDORA-SYSTEMS
    historical_files = {"CHANGELOG.md", "TAG_HISTORY.md", "VERSION_ARCHAEOLOGY.md", "VERSION_ALIGNMENT_REPORT.md", "OSS_POLISH.md",
                        "OSS_POLISH.md", "FINAL_RELEASE_CHECKLIST.md"}

    for md_file in REPO_ROOT.rglob("*.md"):
        if md_file.name in historical_files:
            continue
        if ".git" in str(md_file) or any(part.startswith((".mobile-hold-", ".app-hold-")) for part in md_file.parts):
            continue
        try:
            text = md_file.read_text(encoding="utf-8", errors="ignore")
            for i, line in enumerate(text.splitlines(), 1):
                if "PANDORA-SYSTEMS" in line:
                    stale_refs.append(f"{md_file.relative_to(REPO_ROOT)}:{i}")
        except Exception:
            pass

    # Check .yml/.yaml files too
    for yml_file in list(REPO_ROOT.rglob("*.yml")) + list(REPO_ROOT.rglob("*.yaml")):
        if ".git" in str(yml_file):
            continue
        try:
            text = yml_file.read_text(encoding="utf-8", errors="ignore")
            for i, line in enumerate(text.splitlines(), 1):
                if "PANDORA-SYSTEMS" in line:
                    stale_refs.append(f"{yml_file.relative_to(REPO_ROOT)}:{i}")
        except Exception:
            pass

    check(len(stale_refs) == 0,
          f"Stale PANDORA-SYSTEMS references found: {stale_refs}")
    if not stale_refs:
        print("  No stale PANDORA-SYSTEMS references")
    print(f"  Stale references: {len(stale_refs)}")


def check_workspace():
    """Validate workspace integrity."""
    print("\n[C] WORKSPACE CHECKS")

    cargo_toml = REPO_ROOT / "Cargo.toml"
    content = cargo_toml.read_text()

    # Parse the manifest so every workspace member, is covered.
    with open(cargo_toml, "rb") as handle:
        manifest = tomllib.load(handle)
    members = manifest.get("workspace", {}).get("members", [])
    actual_count = len(members)
    print(f"  Declared workspace members: {actual_count}")

    for member in members:
        member_path = REPO_ROOT / member
        check(member_path.exists() and (member_path / "Cargo.toml").exists(),
              f"Workspace member path missing: {member}")

    # Check version.workspace = true in each Rust workspace crate.
    for member in members:
        crate_toml = REPO_ROOT / member / "Cargo.toml"
        if crate_toml.exists():
            crate_content = crate_toml.read_text()
            if "version.workspace = true" not in crate_content:
                WARNINGS.append(f"{member}/Cargo.toml does not use version.workspace = true")
                print(f"  WARNING: {member} does not inherit workspace version")


def check_workflow_pins():
    """Require immutable action references in checked-in workflows."""
    print("\n[D] WORKFLOW SUPPLY-CHAIN CHECKS")
    workflow_dir = REPO_ROOT / ".github" / "workflows"
    if not workflow_dir.exists():
        check(False, ".github/workflows directory not found")
        return

    mutable_refs = []
    for workflow in list(workflow_dir.glob("*.yml")) + list(workflow_dir.glob("*.yaml")):
        for line_number, line in enumerate(workflow.read_text(encoding="utf-8").splitlines(), 1):
            match = re.search(r"^\s*-?\s*uses:\s+([^\s#]+)", line)
            if not match:
                continue
            action = match.group(1)
            if action.startswith("./"):
                continue
            if "@" not in action or not re.search(r"@[0-9a-f]{40}$", action):
                mutable_refs.append(f"{workflow.relative_to(REPO_ROOT)}:{line_number}:{action}")

    check(not mutable_refs, f"Mutable GitHub Action references found: {mutable_refs}")
    if not mutable_refs:
        print("  All external actions are pinned to immutable commits")

def check_license():
    """Validate license consistency."""
    print("\n[E] LICENSE CHECKS")

    license_file = REPO_ROOT / "LICENSE"
    check(license_file.exists(), "LICENSE file not found")
    if license_file.exists():
        text = license_file.read_text()
        check("Apache License" in text or "Apache-2.0" in text,
              "LICENSE file does not appear to be Apache 2.0")
        print(f"  LICENSE file: {'Apache 2.0' if 'Apache' in text else 'UNKNOWN'}")

    # Check workspace license
    cargo_toml = REPO_ROOT / "Cargo.toml"
    content = cargo_toml.read_text()
    if "license" in content:
        license_m = re.search(r'license\s*=\s*"([^"]+)"', content)
        if license_m:
            declared = license_m.group(1)
            check(declared == "Apache-2.0",
                  f"Workspace license declaration ({declared}) != Apache-2.0")
            print(f"  Workspace license: {declared}")


def check_architecture_claims():
    """Check if README claims specific counts that should be derived."""
    print("\n[F] ARCHITECTURE CLAIMS")

    readme = REPO_ROOT / "README.md"
    if not readme.exists():
        return

    text = readme.read_text()

    # Check architecture claims against source
    count_patterns = [
        (r'(\d+)\s+built-in\s+harness', "harness count", None),
        (r'(\d+)\s+built-in\s+gene', "gene count", None),
        (r'(\d+)\s+crate', "crate count", "workspace.members"),
    ]

    for pattern, desc, source_key in count_patterns:
        m = re.search(pattern, text)
        if m:
            count = int(m.group(1))
            print(f"  README claims {desc}: {count}")
            if source_key == "workspace.members":
                import tomllib
                cargo_toml = REPO_ROOT / "Cargo.toml"
                with open(cargo_toml, "rb") as f:
                    manifest = tomllib.load(f)
                actual = len(manifest.get("workspace", {}).get("members", []))
                if count != actual:
                    WARNINGS.append(f"README claims {count} crates but workspace has {actual}")
                else:
                    print(f"  OK: matches workspace member count ({actual})")
            else:
                WARNINGS.append(f"README hardcodes {desc} = {count} — should be derived from source")


def main():
    print("=" * 60)
    print("O-PANDORA Repository Validator")
    print("=" * 60)
    os.chdir(REPO_ROOT)

    check_version()
    check_identity()
    check_workspace()
    check_workflow_pins()
    check_license()
    check_architecture_claims()

    print("\n" + "=" * 60)
    if ERRORS:
        print(f"RESULT: FAIL — {len(ERRORS)} error(s)")
        for e in ERRORS:
            print(f"  - {e}")
        sys.exit(1)
    else:
        print("RESULT: PASS")
        if WARNINGS:
            print(f"  ({len(WARNINGS)} warning(s))")
            for w in WARNINGS:
                print(f"  - {w}")
        sys.exit(0)


if __name__ == "__main__":
    main()
