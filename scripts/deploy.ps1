param (
    [Parameter(Mandatory=$true)][string]$User,
    [Parameter(Mandatory=$true)][string]$IP
)

Set-Location "$PSScriptRoot/.."

Write-Host "Copying files to $User@$IP`:~/"

# scp -r .\deploy\* "$User@$IP:~/"