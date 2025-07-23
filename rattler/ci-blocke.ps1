Set-Location $PSScriptRoot
$ROOT = git rev-parse --show-toplevel

Set-Location $ROOT
Remove-Item ./dist/ -Recurse -ErrorAction SilentlyContinue
New-Item ./dist -ItemType Directory -ErrorAction SilentlyContinue
&./scripts/setup.ps1
cargo build -r --bins
if ($IsWindows) {
    Copy-Item "$ROOT/target/release/term-nmea.exe" ./dist
}
else {
    Copy-Item "$ROOT/target/release/term-nmea" ./dist
}
Set-Location $PSScriptRoot
pixi run rattler-build build
