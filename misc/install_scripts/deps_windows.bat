@echo off
setlocal

:: Check for winget
where winget >nul 2>&1
if errorlevel 1 (
    echo winget not found. Install it from the Microsoft Store ^(App Installer^) and try again.
    pause
    exit /b 1
)

echo The following packages will be installed via winget:
echo   ffmpeg
echo.
echo NOTE: libheif, libraw and libavif are not available in prebuilt binaries.
echo       If you need those, use MSYS2 app builds (https://www.msys2.org/):
echo         pacman -S mingw-w64-x86_64-libheif mingw-w64-x86_64-libraw mingw-w64-x86_64-libavif
echo.
set /p answer="Do you want to continue? [Y/n]: "
if /i "%answer%"=="n" (
    echo Aborted.
    exit /b 0
)

winget install --id Gyan.FFmpeg -e --source winget

echo.
echo Successfully installed all dependencies.
pause
