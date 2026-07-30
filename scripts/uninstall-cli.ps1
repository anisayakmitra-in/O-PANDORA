$ErrorActionPreference = "Stop"
$installDir = if ($env:PANDORA_INSTALL_DIR) { $env:PANDORA_INSTALL_DIR } else { Join-Path $HOME ".pandora\bin" }
$target = Join-Path $installDir "pandora.exe"
if (Test-Path -LiteralPath $target) {
  Remove-Item -LiteralPath $target -Force
  Write-Host "Removed Pandora CLI: $target"
} else {
  Write-Host "Pandora CLI is not installed at $target"
}
Write-Host "User data was preserved. Remove it separately only if intended."
