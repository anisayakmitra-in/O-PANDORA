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
$temp = Join-Path ([System.IO.Path]::GetTempPath()) ("pandora-install-" + [Guid]::NewGuid())
New-Item -ItemType Directory -Path $temp | Out-Null
try {
  $binary = Join-Path $temp $asset
  $checksum = Join-Path $temp "$asset.sha256"
  try {
    Invoke-WebRequest "$base/$asset" -OutFile $binary
    Invoke-WebRequest "$base/$asset.sha256" -OutFile $checksum
  } catch {
    if ($env:PANDORA_SOURCE_BUILD -eq "1" -and (Get-Command cargo -ErrorAction SilentlyContinue)) {
      $sourceRoot = Join-Path $temp "cargo-root"
      $cargoArgs = @("install", "--git", "https://github.com/$repo.git", "--locked", "--bin", "pandora", "--root", $sourceRoot, "--force")
      if ($version -ne "latest") { $cargoArgs += @("--tag", "v$($version.TrimStart('v'))") }
      & cargo @cargoArgs
      $sourceBinary = Join-Path $sourceRoot "bin\pandora.exe"
      if (-not (Test-Path $sourceBinary)) { throw "Source build did not produce pandora.exe." }
      New-Item -ItemType Directory -Force -Path $installDir | Out-Null
      Copy-Item $sourceBinary (Join-Path $installDir "pandora.exe") -Force
      & (Join-Path $installDir "pandora.exe") --version
      if ($LASTEXITCODE -ne 0) { throw "Source-built Pandora failed its health check." }
      return
    }
    throw "Pandora release binary unavailable. Set PANDORA_SOURCE_BUILD=1 to build from source."
  }
  $expected = (Get-Content $checksum -Raw).Trim().Split()[0].ToUpperInvariant()
  $actual = (Get-FileHash $binary -Algorithm SHA256).Hash.ToUpperInvariant()
  if ($expected -ne $actual) { throw "Checksum verification failed." }
  New-Item -ItemType Directory -Force -Path $installDir | Out-Null
  Copy-Item $binary (Join-Path $installDir "pandora.exe") -Force
  $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
  if (($userPath -split ';') -notcontains $installDir) {
    [Environment]::SetEnvironmentVariable("Path", (($userPath.TrimEnd(';') + ";" + $installDir).Trim(';')), "User")
    Write-Host "Added $installDir to the user PATH. Open a new terminal."
  }
  & (Join-Path $installDir "pandora.exe") --version
  if ($LASTEXITCODE -ne 0) { throw "Installed Pandora failed its health check." }
} finally {
  Remove-Item $temp -Recurse -Force -ErrorAction SilentlyContinue
}