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
set /p answer="Do you want to continue? [Y/n]: "
if /i "%answer%"=="n" (
    echo Aborted.
    exit /b 0
)

winget install --id Gyan.FFmpeg -e --source winget

echo.
echo Successfully installed all dependencies.
pause
