Set-Location $PSScriptRoot/..

if ($IsWindows) {
}
if ($IsMacOS) {
}
if ($IsLinux -and ($(uname -m) -eq 'x86_64' )) {
    sudo apt install -y libudev-dev libc6-dev
}
if ($IsLinux -and ($(uname -m) -eq 'aarch64' )) {
    sudo apt install -y libudev-dev libc6-dev
}
