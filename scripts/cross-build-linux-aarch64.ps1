param([switch]$Release)
Set-Location $PSScriptRoot/..
rustup target add aarch64-unknown-linux-gnu
sudo dpkg --add-architecture arm64
    sudo apt-get update
    sudo apt-get install -y `
      gcc-aarch64-linux-gnu `
      g++-aarch64-linux-gnu `
      qemu-user-static `
      libudev:arm64
$env:PKG_CONFIG_PATH='/usr/lib/aarch64-linux-gnu/pkgconfig'
$env:PKG_CONFIG_ALLOW_CROSS='1'
$env:PKG_CONFIG_LIBDIR='/usr/lib/aarch64-linux-gnu/pkgconfig'
if ($IsWindows) { rustup toolchain add stable-x86_64-unknown-linux-gnu --profile minimal --force-non-host }
if ($Release) {
    cross build --target aarch64-unknown-linux-gnu --all-features --release --bins
    Copy-Item ./target/aarch64-unknown-linux-gnu/release/term-nmea ./deploy/linux-aarch64/bin/
}
else {
    cargo build --target aarch64-unknown-linux-gnu --all-features
    cargo build --target aarch64-unknown-linux-gnu --all-features --examples
}
