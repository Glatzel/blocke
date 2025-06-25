Set-Location $PSScriptRoot/..
New-Item temp -ItemType Directory -ErrorAction SilentlyContinue
gh release download -R balena-io/etcher -p *-win32-x64-*.zip -O ./temp/balenaEtcher.zip --skip-existing
7z x ./temp/balenaEtcher.zip -obalenaEtcher
