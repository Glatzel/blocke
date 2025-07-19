param([switch]$Release)
Set-Location $PSScriptRoot/..
# download proj
aria2c -c -x16 -s16 `
    -d ./temp `
    https://repo.prefix.dev/glatzel/linux-aarch64/proj-9.6.2-he8cfe8b_2.conda `
    -o proj.zip
7z x ./temp/proj.zip -otemp
zstd -d ./temp/pkg-proj-9.6.2-he8cfe8b_2.tar.zst -o - | tar -xf - -C ./temp

$env:PKG_CONFIG_PATH = Resolve-Path ./temp/proj/arm64-linux-release/lib/pkgconfig
cargo install cross
if ($IsWindows) { rustup toolchain add stable-x86_64-unknown-linux-gnu --profile minimal --force-non-host }
if ($Release) {
    cross build --target aarch64-unknown-linux-gnu --all-features --release --bins
    Copy-Item ./target/aarch64-unknown-linux-gnu/release/term-nmea ./deploy/linux-aarch64/bin/
}
else {
    cross build --target aarch64-unknown-linux-gnu --all-features
    cross build --target aarch64-unknown-linux-gnu --all-features --examples
}
