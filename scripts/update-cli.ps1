$ErrorActionPreference = "Stop"
$repo = "anisayakmitra-in/O-PANDORA"
$version = if ($env:PANDORA_VERSION) { $env:PANDORA_VERSION } else { "latest" }
$architecture = [System.Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture
$asset = if ($architecture -eq [System.Runtime.InteropServices.Architecture]::Arm64) {
  "pandora-windows-arm64.exe"
} else {
  "pandora-windows-x86_64.exe"
}
$installDir = if ($env:PANDORA_INSTALL_DIR) { $env:PANDORA_INSTALL_DIR } else { Join-Path $HOME ".pandora\bin" }
$base = if ($env:PANDORA_RELEASE_BASE_URL) { $env:PANDORA_RELEASE_BASE_URL.TrimEnd('/') } elseif ($version -eq "latest") { "https://github.com/$repo/releases/latest/download" } else { "https://github.com/$repo/releases/download/v$($version.TrimStart('v'))" }
$temp = Join-Path ([System.IO.Path]::GetTempPath()) ("pandora-update-" + [Guid]::NewGuid())
New-Item -ItemType Directory -Path $temp | Out-Null
try {
  $binary = Join-Path $temp $asset
  $checksum = Join-Path $temp "$asset.sha256"
  Invoke-WebRequest "$base/$asset" -OutFile $binary
  Invoke-WebRequest "$base/$asset.sha256" -OutFile $checksum
  $expected = (Get-Content $checksum -Raw).Trim().Split()[0].ToUpperInvariant()
  $actual = (Get-FileHash $binary -Algorithm SHA256).Hash.ToUpperInvariant()
  if ($expected -ne $actual) { throw "Checksum verification failed." }
  if (-not (Get-Command gh -ErrorAction SilentlyContinue)) {
    throw "GitHub CLI authentication is required to verify release provenance."
  }
  $workflow = "$repo/.github/workflows/cli-release.yml"
  if ($version -eq "latest") {
    & gh attestation verify $binary --repo $repo --signer-workflow $workflow --deny-self-hosted-runners | Out-Null
  } else {
    $sourceRef = "refs/tags/v$($version.TrimStart('v'))"
    & gh attestation verify $binary --repo $repo --signer-workflow $workflow --source-ref $sourceRef --deny-self-hosted-runners | Out-Null
  }
  if ($LASTEXITCODE -ne 0) { throw "Release provenance verification failed; existing binary was not changed." }
  New-Item -ItemType Directory -Force -Path $installDir | Out-Null
  $target = Join-Path $installDir "pandora.exe"
  $backup = "$target.previous"
  if (Test-Path $target) { Copy-Item $target $backup -Force }
  Copy-Item $binary $target -Force
  try {
    & $target --version | Out-Null
    if ($LASTEXITCODE -ne 0) {
      throw "updated binary exited with code $LASTEXITCODE"
    }
  } catch {
    if (Test-Path $backup) { Move-Item $backup $target -Force }
    throw "Updated binary failed its health check; previous version restored."
  }
  Remove-Item $backup -Force -ErrorAction SilentlyContinue
  & $target --version
} finally {
  Remove-Item $temp -Recurse -Force -ErrorAction SilentlyContinue
}

