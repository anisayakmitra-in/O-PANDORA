$ErrorActionPreference = "Stop"
$repo = "anisayakmitra-in/O-PANDORA"
$version = if ($env:PANDORA_VERSION) { $env:PANDORA_VERSION } else { "latest" }
$architecture = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture
$asset = if ($architecture -eq [System.Runtime.InteropServices.Architecture]::Arm64) {
  "pandora-windows-arm64.exe"
} else {
  "pandora-windows-x86_64.exe"
}
$installDir = if ($env:PANDORA_INSTALL_DIR) { $env:PANDORA_INSTALL_DIR } else { Join-Path $HOME ".pandora\bin" }
$base = if ($env:PANDORA_RELEASE_BASE_URL) { $env:PANDORA_RELEASE_BASE_URL.TrimEnd('/') } elseif ($version -eq "latest") { "https://github.com/$repo/releases/latest/download" } else { "https://github.com/$repo/releases/download/v$($version.TrimStart('v'))" }
function Add-InstallDirectoryToPath {
  param([string]$Directory)
  try {
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($null -eq $userPath) { $userPath = "" }
    if (($userPath -split ';') -notcontains $Directory) {
      [Environment]::SetEnvironmentVariable("Path", (($userPath.TrimEnd(';') + ";" + $Directory).Trim(';')), "User")
      Write-Host "Added $Directory to the user PATH. Open a new terminal."
    }
  } catch {
    Write-Warning "Could not update the user PATH automatically. Add $Directory to PATH manually."
  }
}
function Install-FromSource {
  param([string]$TempRoot)
  if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) { throw "Rust and Cargo are required for source builds." }
  $sourceRoot = Join-Path $TempRoot "cargo-root"
  $cargoArgs = @("install", "--git", "https://github.com/$repo.git", "--locked", "--bin", "pandora", "--root", $sourceRoot, "--force")
  if ($version -ne "latest") { $cargoArgs += @("--tag", "v$($version.TrimStart('v'))") }
  & cargo @cargoArgs
  if ($LASTEXITCODE -ne 0) { throw "Pandora source build failed." }
  $sourceBinary = Join-Path $sourceRoot "bin\pandora.exe"
  if (-not (Test-Path $sourceBinary)) { throw "Source build did not produce pandora.exe." }
  New-Item -ItemType Directory -Force -Path $installDir | Out-Null
  & $sourceBinary --version
  if ($LASTEXITCODE -ne 0) { throw "Source-built Pandora failed its health check." }
  Copy-Item $sourceBinary (Join-Path $installDir "pandora.exe") -Force
  Add-InstallDirectoryToPath $installDir
  & (Join-Path $installDir "pandora.exe") --version
  if ($LASTEXITCODE -ne 0) { throw "Source-built Pandora failed its health check." }
}
function Test-ReleaseProvenance {
  param([string]$Binary)
  if (-not (Get-Command gh -ErrorAction SilentlyContinue)) { return $false }
  $workflow = "$repo/.github/workflows/cli-release.yml"
  if ($version -eq "latest") {
    & gh attestation verify $Binary --repo $repo --signer-workflow $workflow --deny-self-hosted-runners | Out-Null
  } else {
    $sourceRef = "refs/tags/v$($version.TrimStart('v'))"
    & gh attestation verify $Binary --repo $repo --signer-workflow $workflow --source-ref $sourceRef --deny-self-hosted-runners | Out-Null
  }
  return $LASTEXITCODE -eq 0
}
$temp = Join-Path ([System.IO.Path]::GetTempPath()) ("pandora-install-" + [Guid]::NewGuid())
New-Item -ItemType Directory -Path $temp | Out-Null
try {
  $binary = Join-Path $temp $asset
  $checksum = Join-Path $temp "$asset.sha256"
  try {
    Invoke-WebRequest "$base/$asset" -OutFile $binary
    Invoke-WebRequest "$base/$asset.sha256" -OutFile $checksum
  } catch {
    if ($env:PANDORA_SOURCE_BUILD -eq "1") {
      Install-FromSource $temp
      return
    }
    throw "Pandora release binary unavailable. Set PANDORA_SOURCE_BUILD=1 to build from source."
  }
  $expected = (Get-Content $checksum -Raw).Trim().Split()[0].ToUpperInvariant()
  $actual = (Get-FileHash $binary -Algorithm SHA256).Hash.ToUpperInvariant()
  if ($expected -ne $actual) { throw "Checksum verification failed." }
  if (-not (Test-ReleaseProvenance $binary)) {
    if ($env:PANDORA_SOURCE_BUILD -eq "1") {
      Install-FromSource $temp
      return
    }
    throw "GitHub CLI authentication and valid Pandora build provenance are required for prebuilt installation."
  }
  New-Item -ItemType Directory -Force -Path $installDir | Out-Null
  & $binary --version
  if ($LASTEXITCODE -ne 0) { throw "Downloaded Pandora failed its health check." }
  Copy-Item $binary (Join-Path $installDir "pandora.exe") -Force
  Add-InstallDirectoryToPath $installDir

  & (Join-Path $installDir "pandora.exe") --version
  if ($LASTEXITCODE -ne 0) { throw "Installed Pandora failed its health check." }
} finally {
  Remove-Item $temp -Recurse -Force -ErrorAction SilentlyContinue
}