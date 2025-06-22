param($pkg)
Set-Location $PSScriptRoot
Set-Location ..

foreach ($pkg_file in Get-ChildItem "./$pkg/output/*/*.conda" -Recurse -ErrorAction Continue) {
    Write-Output "::group:: upload $pkg"
        Write-Output "$pkg is a public package"
        pixi run rattler-build upload prefix -s -c glatzel $pkg_file
    $LASTEXITCODE = 0
    Write-Output "::endgroup::"
}
