Set-Location $PSScriptRoot/..
cargo install cross
if ($IsWindows) { rustup toolchain add stable-x86_64-unknown-linux-gnu --profile minimal --force-non-host }
cross build --target aarch64-unknown-linux-gnu
cross build --target aarch64-unknown-linux-gnu --examples
