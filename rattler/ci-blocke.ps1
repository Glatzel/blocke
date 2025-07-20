Set-Location $PSScriptRoot
$ROOT = git rev-parse --show-toplevel

Set-Location $ROOT
Remove-Item ./dist/ -Recurse -ErrorAction SilentlyContinue
New-Item ./dist -ItemType Directory -ErrorAction SilentlyContinue
&./scripts/setup.ps1
Cargo build -r --bins
if ($IsWindows) {
    Copy-Item "$ROOT/target/release/term-nmea.exe" ./dist
}
else {
    Copy-Item "$ROOT/target/release/term-nmea" ./dist
}
Set-Location $PSScriptRoot
pixi run rattler-build build

# linux-aarch64
if ($IsLinux) {
    Set-Location $ROOT
    Remove-Item ./dist/ -Recurse -ErrorAction SilentlyContinue
    New-Item ./dist -ItemType Directory -ErrorAction SilentlyContinue
    &./scripts/cross-build-linux-aarch64.ps1 -Release
    Copy-Item "$ROOT/target/aarch64-unknown-linux-gnu/release/term-nmea" ./dist
    pixi run rattler-build build --target-platform linux-aarch64
}
