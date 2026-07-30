param([string]$OutputDirectory = "dist/releases")
$ErrorActionPreference = "Stop"
New-Item -ItemType Directory -Force -Path $OutputDirectory | Out-Null
cargo build --release -p pandora

$architecture = if ([System.Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture -eq [System.Runtime.InteropServices.Architecture]::Arm64) { "arm64" } else { "x86_64" }
$assetName = "pandora-windows-$architecture.exe"
$asset = Join-Path $OutputDirectory $assetName
Copy-Item "target/release/pandora.exe" $asset -Force
$hash = (Get-FileHash $asset -Algorithm SHA256).Hash
"$hash  $assetName" | Set-Content ("$asset.sha256") -NoNewline
$version = (Select-String -Path "Cargo.toml" -Pattern '^version = "([^"]+)"' | Select-Object -First 1).Matches.Groups[1].Value
$commit = (git rev-parse HEAD).Trim()
@{ version = $version; target = $assetName; commit = $commit } | ConvertTo-Json | Set-Content ("$asset.metadata.json")
$commit | Set-Content (Join-Path $OutputDirectory "pandora-build-commit.txt") -NoNewline
Write-Output "Created $asset, checksum, and metadata."