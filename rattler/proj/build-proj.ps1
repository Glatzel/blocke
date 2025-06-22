Set-Location $PSScriptRoot
$ROOT = git rev-parse --show-toplevel

New-Item $env:PREFIX/$name -ItemType Directory
Copy-Item "$PSScriptRoot/../proj_build/installed/*" "$env:PREFIX/$name" -Recurse
