Set-Location $PSScriptRoot
# install static dependency
Write-Output "::group::static"
./vcpkg/vcpkg install --triplet arm64-linux-release --x-install-root ./installed
Write-Output "::endgroup::"
