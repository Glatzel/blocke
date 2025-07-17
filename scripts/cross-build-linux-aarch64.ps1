param([switch]$Release)
Set-Location $PSScriptRoot/..
cargo install cross
if ($IsWindows) { rustup toolchain add stable-x86_64-unknown-linux-gnu --profile minimal --force-non-host }
if ($Release) {
    cross build --target aarch64-unknown-linux-gnu --all-features --release --bins --artifact-dir ./deploy/bin
}
else {
    cross build --target aarch64-unknown-linux-gnu --all-features
    cross build --target aarch64-unknown-linux-gnu --all-features --examples
}
