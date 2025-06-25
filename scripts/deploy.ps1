param (
    [Parameter(Mandatory = $true)][string]$User,
    [Parameter(Mandatory = $true)][string]$IP
)

Set-Location "$PSScriptRoot/.."

# download passmark
aria2c -c -x16 -s16 `
    -d ./temp `
    https://www.passmark.com/downloads/PerformanceTest_Linux_ARM64.zip `
    -o passmark.zip
7z e ./temp/passmark.zip PerformanceTest/pt_linux_arm64 -odeploy/bench

# copy file
Write-Host "Copying files to $User@$IP`:~/"
scp -r ./deploy/* "$User@$IP`:~/"