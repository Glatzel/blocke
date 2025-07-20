param([switch]$Release)

$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $true

if ($IsLinux) {
    # Define version and download URL
    $ZIG_VERSION = "0.11.0"
    $ZIG_TAR = "zig-linux-x86_64-$ZIG_VERSION.tar.xz"
    $ZIG_DIR = "zig-linux-x86_64-$ZIG_VERSION"
    $ZIG_URL = "https://ziglang.org/download/$ZIG_VERSION/$ZIG_TAR"

    # Download Zig
    if (-not (Test-Path $ZIG_TAR)) {
        Write-Host "Downloading Zig $ZIG_VERSION..."
        Invoke-WebRequest -Uri $ZIG_URL -OutFile $ZIG_TAR
    }

    # Extract Zig
    if (-not (Test-Path $ZIG_DIR)) {
        Write-Host "Extracting Zig..."
        tar -xf $ZIG_TAR
    }

    # Add to PATH temporarily (for this session)
    $zigFullPath = Resolve-Path "./$ZIG_DIR"
    $env:PATH = "$zigFullPath" + ":" + $env:PATH

    # Confirm installation
    zig version

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