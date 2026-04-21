# Packages an all-backends krokiet binary together with per-backend launcher
# batch scripts into a zip archive.
#
# Usage: pack_all_backends.ps1 -Binary <path> -OutputZip <path>
param(
    [Parameter(Mandatory)][string]$Binary,
    [Parameter(Mandatory)][string]$OutputZip
)

$pkgDir = Join-Path $env:TEMP ([System.IO.Path]::GetRandomFileName())
New-Item -ItemType Directory -Force $pkgDir | Out-Null

Copy-Item $Binary (Join-Path $pkgDir "krokiet.exe")

$backends = [ordered]@{
    "krokiet_winit_femtovg.bat"     = "winit-femtovg"
    "krokiet_winit_skia_opengl.bat" = "winit-skia-opengl"
    "krokiet_winit_skia_vulkan.bat" = "winit-skia-vulkan"
    "krokiet_winit_software.bat"    = "winit-software"
    "krokiet_femtovg_wgpu.bat"      = "femtovg-wgpu"
}
foreach ($file in $backends.Keys) {
    $backend = $backends[$file]
    "@echo off`r`nset SLINT_BACKEND=$backend`r`n`"%~dp0krokiet.exe`" %*`r`n" |
        Set-Content -Encoding ASCII (Join-Path $pkgDir $file)
}

Compress-Archive -Path (Join-Path $pkgDir "*") -DestinationPath $OutputZip -Force

Remove-Item -Recurse -Force $pkgDir

