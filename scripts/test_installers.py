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

    def test_powershell_checks_download_before_replacement(self):
        script = (ROOT / "scripts" / "install-cli.ps1").read_text(encoding="utf-8")
        self.assertLess(
            script.index('& $binary --version'),
            script.index('Copy-Item $binary (Join-Path $installDir "pandora.exe") -Force'),
        )


    def test_wrapper_does_not_start_setup_or_import(self):
        script = (ROOT / "scripts" / "install.sh").read_text(encoding="utf-8")
        self.assertNotIn("pandora setup 2>/dev/null", script)
        self.assertNotIn("pandora import", script)
        self.assertIn('"$PANDORA_BIN" doctor', script)
if __name__ == "__main__":
    unittest.main()