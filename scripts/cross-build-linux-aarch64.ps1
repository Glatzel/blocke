param([switch]$Release)
$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $true
if ($IsLinux) {
    Set-Location $PSScriptRoot/..
    sudo apt-get update
    sudo apt-get install -y gcc-aarch64-linux-gnu g++-aarch64-linux-gnu
    rustup target add aarch64-unknown-linux-musl
    pixi global install proj -c https://repo.prefix.dev/glatzel --platform linux-aarch64

    # Set PKG_CONFIG_PATH to vcpkg's pkgconfig directory
    $p = resolve-path ~/.pixi/envs/proj/proj/arm64-linux-release/lib/pkgconfig
    $env:PKG_CONFIG_PATH = "$p" + ":" + "$env:PKG_CONFIG_PATH"
    $env:PKG_CONFIG_ALLOW_CROSS = 1
    if ($Release) {
        cargo build --target aarch64-unknown-linux-musl --all-features --release --bins
    }
    else {
        cargo build --target aarch64-unknown-linux-musl --all-features
        cargo build --target aarch64-unknown-linux-musl --all-features --examples
    }
}
else { exit 1 }
