param([switch]$Release)
$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $true
if ($IsLinux) {
    Set-Location $PSScriptRoot/..
    sudo apt-get update
    sudo dpkg --add-architecture arm64
    sudo apt-get install -y `
        g++-aarch64-linux-gnu `
        librust-libudev-dev
    rustup target add aarch64-unknown-linux-gnu
    pixi global install proj -c https://repo.prefix.dev/glatzel --platform linux-aarch64

    # Set PKG_CONFIG_PATH to vcpkg's pkgconfig directory
    $p = resolve-path ~/.pixi/envs/proj/proj/arm64-linux-release/lib/pkgconfig
    $env:PKG_CONFIG_PATH = "$p" + ":" + "$env:PKG_CONFIG_PATH"
    $env:PKG_CONFIG_ALLOW_CROSS = 1
    if ($Release) {
        cargo build --target aarch64-unknown-linux-gnu --all-features --release --bins
        Copy-Item ./target/aarch64-unknown-linux-gnu/release/term-nmea ./deploy/linux-aarch64/bin/
    }
    else {
        cargo build --target aarch64-unknown-linux-gnu --all-features
        cargo build --target aarch64-unknown-linux-gnu --all-features --examples
    }
}
else { exit 1 }
