$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $true
$ROOT = git rev-parse --show-toplevel
if (Test-Path $PSScriptRoot/setup.ps1) {
    &$PSScriptRoot/setup.ps1
}
Set-Location $PSScriptRoot/..
cargo doc --no-deps --all-features

Set-Location $ROOT
