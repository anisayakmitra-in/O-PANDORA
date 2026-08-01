import re
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

    def test_bash_prebuilt_assets_are_published(self):
        script = (ROOT / "scripts" / "install-cli.sh").read_text(encoding="utf-8")
        workflow = (ROOT / ".github" / "workflows" / "cli-release.yml").read_text(
            encoding="utf-8"
        )

        for asset in re.findall(r'ASSET="(pandora-[^"]+)"', script):
            self.assertIn(f"asset: {asset}", workflow)

    def test_powershell_checks_download_before_replacement(self):
        script = (ROOT / "scripts" / "install-cli.ps1").read_text(encoding="utf-8")
        self.assertLess(
            script.index('& $binary --version'),
            script.index('Copy-Item $binary (Join-Path $installDir "pandora.exe") -Force'),
        )


    def test_powershell_path_update_is_best_effort(self):
        script = (ROOT / "scripts" / "install-cli.ps1").read_text(encoding="utf-8")
        self.assertIn("function Add-InstallDirectoryToPath", script)
        self.assertEqual(script.count("Add-InstallDirectoryToPath $installDir"), 2)
        self.assertIn("Could not update the user PATH automatically", script)


    def test_powershell_uses_os_architecture_for_asset_selection(self):
        script = (ROOT / "scripts" / "install-cli.ps1").read_text(encoding="utf-8")
        self.assertIn("RuntimeInformation]::OSArchitecture", script)
        self.assertNotIn("RuntimeInformation]::ProcessArchitecture", script)

    def test_release_workflow_smoke_tests_installer_after_publication(self):
        workflow = (ROOT / ".github" / "workflows" / "cli-release.yml").read_text(encoding="utf-8")
        self.assertIn("installer-smoke:", workflow)
        self.assertIn("needs: publish", workflow)
        self.assertIn("ref: ${{ github.ref_name }}", workflow)
        self.assertIn("PANDORA_VERSION: ${{ github.ref_name }}", workflow)
        self.assertIn("bash scripts/install-cli.sh", workflow)
        self.assertIn("& .\\scripts\\install-cli.ps1", workflow)

    def test_manual_smoke_workflow_uses_requested_tagged_scripts(self):
        workflow = (ROOT / ".github" / "workflows" / "installer-smoke.yml").read_text(encoding="utf-8")
        self.assertIn("ref: v${{ inputs.version }}", workflow)
        self.assertIn("PANDORA_VERSION: ${{ inputs.version }}", workflow)
        self.assertIn("bash scripts/install-cli.sh", workflow)
        self.assertIn("& .\\scripts\\install-cli.ps1", workflow)
        self.assertNotIn("raw.githubusercontent.com", workflow)

    def test_wrapper_does_not_start_setup_or_import(self):
        script = (ROOT / "scripts" / "install.sh").read_text(encoding="utf-8")
        self.assertNotIn("pandora setup 2>/dev/null", script)
        self.assertNotIn("pandora import", script)
        self.assertIn('"$PANDORA_BIN" doctor', script)
if __name__ == "__main__":
    unittest.main()