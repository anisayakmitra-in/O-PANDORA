#!/usr/bin/env python3
"""Generate a deterministic CycloneDX SBOM from Cargo.lock."""

from __future__ import annotations

import argparse
import json
import tomllib
from pathlib import Path


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("output", type=Path)
    args = parser.parse_args()

    lock = tomllib.loads(Path("Cargo.lock").read_text(encoding="utf-8"))
    components = []
    for package in lock.get("package", []):
        name = package["name"]
        version = package["version"]
        purl = f"pkg:cargo/{name}@{version}"
        components.append(
            {
                "type": "library",
                "bom-ref": purl,
                "name": name,
                "version": version,
                "purl": purl,
            }
        )

    document = {
        "bomFormat": "CycloneDX",
        "specVersion": "1.5",
        "serialNumber": "urn:uuid:pandora-cargo-lock",
        "version": 1,
        "metadata": {"tools": [{"vendor": "Pandora", "name": "generate_sbom.py"}]},
        "components": components,
    }
    args.output.write_text(json.dumps(document, indent=2) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
