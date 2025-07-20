Set-Location $PSScriptRoot
$ROOT = git rev-parse --show-toplevel

Set-Location $ROOT
Cargo install --bins --root ./dist
Set-Location $PSScriptRoot
# pixi run rattler-build build
