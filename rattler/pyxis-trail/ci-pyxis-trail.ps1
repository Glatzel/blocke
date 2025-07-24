Set-Location $PSScriptRoot
$ROOT = git rev-parse --show-toplevel

Set-Location $ROOT
Remove-Item ./dist/ -Recurse -ErrorAction SilentlyContinue
New-Item ./dist -ItemType Directory -ErrorAction SilentlyContinue
&./scripts/setup.ps1
cargo build -r --bin pyxis-trail
Set-Location $PSScriptRoot
pixi run rattler-build build
