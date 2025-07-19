param([switch]$Release)
Set-Location $PSScriptRoot/..
cargo install cross
if ($IsWindows) { rustup toolchain add stable-x86_64-unknown-linux-gnu --profile minimal --force-non-host }
if ($Release) {
    cross build --target aarch64-unknown-linux-gnu --all-features --release --bins --env PKG_CONFIG_PATH=./.pixi/envs/default/proj/arm64-linux-release/lib/pkgconfig
    Copy-Item ./target/aarch64-unknown-linux-gnu/release/term-nmea ./deploy/linux-aarch64/bin/
}
else {
    cross build --target aarch64-unknown-linux-gnu --all-features --env PKG_CONFIG_PATH=/path/in/container --env PKG_CONFIG_PATH=./.pixi/envs/default/proj/arm64-linux-release/lib/pkgconfig
    cross build --target aarch64-unknown-linux-gnu --all-features --examples --env PKG_CONFIG_PATH=/path/in/container --env PKG_CONFIG_PATH=./.pixi/envs/default/proj/arm64-linux-release/lib/pkgconfig
}
