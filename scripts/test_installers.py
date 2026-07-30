import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


class InstallerContractTests(unittest.TestCase):
    def test_bash_checks_download_before_replacement(self):
        script = (ROOT / "scripts" / "install-cli.sh").read_text(encoding="utf-8")
        self.assertLess(
            script.index('"$BINARY" --version'),
            script.index('install -m 0755 "$BINARY" "$INSTALL_DIR/pandora"'),
        )

