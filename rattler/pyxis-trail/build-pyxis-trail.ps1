Set-Location $PSScriptRoot
$ROOT = git rev-parse --show-toplevel
New-Item $env:PREFIX/bin -ItemType Directory
if ($IsWindows) {
    Copy-Item "$ROOT/target/release/pyxis-trail.exe" "$env:PREFIX/bin/"
}
else {
    Copy-Item "$ROOT/target/release/pyxis-trail" "$env:PREFIX/bin/"
}
