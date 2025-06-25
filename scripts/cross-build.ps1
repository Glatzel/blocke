Set-Location $PSScriptRoot/..
cargo install cross
cross build --target aarch64-unknown-linux-gnu