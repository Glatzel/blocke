Set-Location $PSScriptRoot
$ROOT = git rev-parse --show-toplevel

New-Item $env:PREFIX/bin -ItemType Directory
if ($IsWindows) {
    Copy-Item "$ROOT/dist/term-nmea.exe" "$env:PREFIX/bin/"
}
else {
    Copy-Item "$ROOT/dist/term-nmea" "$env:PREFIX/bin/"
}
