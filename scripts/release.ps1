param(
  [string]$OutputDirectory = "dist/releases",
  [string]$Target = $env:PANDORA_TARGET
)
$ErrorActionPreference = "Stop"
New-Item -ItemType Directory -Force -Path $OutputDirectory | Out-Null

$cargoArgs = @("build", "--release", "-p", "pandora")
if ($Target) {
  rustup target add $Target
  $cargoArgs += @("--target", $Target)
}
& cargo @cargoArgs
if ($LASTEXITCODE -ne 0) { throw "cargo build failed with exit code $LASTEXITCODE" }

if ($Target) {
  $targetInfo = @{
    "x86_64-pc-windows-msvc" = @{ Name = "pandora-windows-x86_64.exe"; Binary = "target\x86_64-pc-windows-msvc\release\pandora.exe" }
    "aarch64-pc-windows-msvc" = @{ Name = "pandora-windows-arm64.exe"; Binary = "target\aarch64-pc-windows-msvc\release\pandora.exe" }
  }
  if (-not $targetInfo.ContainsKey($Target)) { throw "Unsupported Windows release target: $Target" }
  $assetName = $targetInfo[$Target].Name
  $binary = $targetInfo[$Target].Binary
} else {
  $architecture = if ([System.Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture -eq [System.Runtime.InteropServices.Architecture]::Arm64) { "arm64" } else { "x86_64" }
  $assetName = "pandora-windows-$architecture.exe"
  $binary = "target\release\pandora.exe"
}

$asset = Join-Path $OutputDirectory $assetName
if (-not (Test-Path -LiteralPath $binary)) { throw "Build did not produce $binary" }
Copy-Item $binary $asset -Force
$hash = (Get-FileHash $asset -Algorithm SHA256).Hash
"$hash  $assetName" | Set-Content ("$asset.sha256") -NoNewline
$version = (Select-String -Path "Cargo.toml" -Pattern '^version = "([^"]+)"' | Select-Object -First 1).Matches.Groups[1].Value
$commit = (git rev-parse HEAD).Trim()
@{ version = $version; target = $assetName; commit = $commit } | ConvertTo-Json | Set-Content ("$asset.metadata.json")
$commit | Set-Content (Join-Path $OutputDirectory "pandora-build-commit.txt") -NoNewline
Write-Output "Created $asset, checksum, and metadata."