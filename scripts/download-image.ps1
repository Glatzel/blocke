param (
    [ValidateSet("orangepi_zero_2w")][string]$Model
)
Set-Location $PSScriptRoot/..
switch ($Model) {
    "orangepi_zero_2w" { $url = "https://dietpi.com/downloads/images/DietPi_OrangePiZero2W-ARMv8-Bookworm.img.xz" }
    default {
        Write-Error "Unknown SBC model"
        exit
    }
}
$out_file_name = $Model + ".img.xz"
aria2c -c -x16 -s16 `
    -d ./temp `
    $url `
    -o $out_file_name
7z x ./temp/$out_file_name -oimage
