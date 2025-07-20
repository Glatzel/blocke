param([switch]$Release)

$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $true

if ($IsLinux) {
    Set-Location "$PSScriptRoot/.."

    sudo apt update
    sudo apt install zig

    cargo install cargo-zigbuild --locked
    rustup target add aarch64-unknown-linux-musl

    # Install proj via Pixi
    pixi install proj -c https://repo.prefix.dev/glatzel --platform linux-aarch64

    # Set PKG_CONFIG_PATH
    $p = Resolve-Path "$HOME/.pixi/envs/proj/proj/arm64-linux-release/lib/pkgconfig"
    $env:PKG_CONFIG_PATH = "$p" + ":" + "/usr/lib/aarch64-linux-gnu/pkgconfig" + ":" + "$env:PKG_CONFIG_PATH"
    $env:PKG_CONFIG_ALLOW_CROSS = 1

    if ($Release) {
        cargo zigbuild --target aarch64-unknown-linux-musl --all-features --release --bins
    }
    else {
        cargo zigbuild --target aarch64-unknown-linux-musl --all-features
        cargo zigbuild --target aarch64-unknown-linux-musl --all-features --examples
    }
}
else {
    Write-Error "This script must be run on Linux."
    exit 1
}
