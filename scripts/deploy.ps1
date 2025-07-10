param (
    [Parameter(Mandatory = $true)][string]$User,
    [Parameter(Mandatory = $true)][string]$IP
)
Set-Location $PSScriptRoot/..

# remove file
ssh $User@$IP "rm -rf /path/to/remote/folder"

# copy file
Write-Host "Copying files to $User@$IP`:~/"
scp -r ./deploy/* "$User@$IP`:~/"
