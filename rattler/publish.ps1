param($pkg)
Set-Location $PSScriptRoot
Set-Location ..

foreach ($pkg_file in Get-ChildItem "./$pkg/output/*/*.conda" -Recurse -ErrorAction Continue) {
    Write-Output "::group:: upload $pkg"
    if(Test-Path "./packages/$pkg/PUBLIC") {
        Write-Output "$pkg is a public package"
        pixi run rattler-build upload prefix -s -c glatzel $pkg_file
    }
    else {
        Write-Output "$pkg is a private package"
        pixi run rattler-build upload prefix -s -c glatzel-private $pkg_file
    }

    $LASTEXITCODE = 0
    Write-Output "::endgroup::"
}
