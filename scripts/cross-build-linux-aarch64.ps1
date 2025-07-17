param([switch]$Release)
Set-Location $PSScriptRoot/..
cargo install cross
if ($IsWindows) { rustup toolchain add stable-x86_64-unknown-linux-gnu --profile minimal --force-non-host }
if ($Release) {
    cross install --target aarch64-unknown-linux-gnu --all-features --root ./deploy/bin
    cross install --target aarch64-unknown-linux-gnu --all-features --examples --root ./deploy/bin/examples
}
else {
    cross build --target aarch64-unknown-linux-gnu --all-features
    cross build --target aarch64-unknown-linux-gnu --all-features --examples
}
