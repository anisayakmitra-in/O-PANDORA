#!/usr/bin/env python3
"""Validate docs for dead internal links and known contradictions."""

import os
import re
import sys

ROOT = "/home/user/pandora-systems"

EXCLUDED_DIRS = {"target", ".git", "llama.cpp", "legacy"}

errors = []

md_files = set()
for dirpath, _, filenames in os.walk(ROOT):
    parts = set(dirpath.split(os.sep))
    if parts & EXCLUDED_DIRS:
        continue
    for fn in filenames:
        if fn.endswith(".md"):
            md_files.add(os.path.relpath(os.path.join(dirpath, fn), ROOT))

link_re = re.compile(r"\[([^\]]+)\]\(([^)]+)\)")

for path in sorted(md_files):
    full = os.path.join(ROOT, path)
    with open(full, encoding="utf-8", errors="replace") as f:
        content = f.read()
    for line_no, line in enumerate(content.splitlines(), 1):
        for text, target in link_re.findall(line):
            if target.startswith(("http://", "https://", "mailto:")):
                continue
            if target.startswith("#"):
                continue
            base = os.path.dirname(path)
            resolved = os.path.normpath(os.path.join(base, target.split("#")[0]))
            if resolved not in md_files:
                if not os.path.exists(os.path.join(ROOT, resolved)):
                    errors.append(f"{path}:{line_no}: dead link '{target}' (resolved: {resolved})")

readme_path = os.path.join(ROOT, "README.md")
with open(readme_path, encoding="utf-8", errors="replace") as f:
    readme = f.read()

if "PANDORA-SYSTEMS" in readme:
    errors.append("README.md: still references old PANDORA-SYSTEMS repo URL")
if "21 built-in" in readme and "22 built-in" not in readme:
    errors.append("README.md: claims 21 built-in genes but `pandora genes` shows 22")
if "pandora-palace" in readme:
    errors.append("README.md: lists pandora-palace crate which is not in workspace")
if "MIT" in readme:
    errors.append("README.md: claims MIT license but LICENSE is Apache-2.0")
if "sample-apps/" in readme:
    errors.append("README.md: links to sample-apps/ which does not exist")
if "docs/SECURITY.md" in readme:
    errors.append("README.md: links to docs/SECURITY.md but canonical file is root SECURITY.md")
if "docs/tutorials/BUILD_A_GENE.md" in readme or "docs/tutorials/BUILD_DOMAIN_HARNESS.md" in readme:
    errors.append("README.md: links to tutorial files that do not exist")

arch_path = os.path.join(ROOT, "docs", "ARCHITECTURE.md")
with open(arch_path, encoding="utf-8", errors="replace") as f:
    arch = f.read()
if "Currently empty on startup" in arch:
    errors.append("docs/ARCHITECTURE.md: claims Shadow Council is empty on startup, but 13 harnesses register")
if "PANDORA SYSTEMS" in arch:
    errors.append("docs/ARCHITECTURE.md: still uses PANDORA SYSTEMS branding")

if errors:
    print("Documentation errors found:")
    for e in errors:
        print(f"  - {e}")
    sys.exit(1)
else:
    print("Documentation validation passed.")
    sys.exit(0)
