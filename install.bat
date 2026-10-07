@echo off
setlocal
title VortexPlayer One-Click Installer

echo ==========================================================
echo        VortexPlayer One-Click Windows Installer
echo ==========================================================
echo.

if exist "%~dp0install.ps1" (
    powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0install.ps1" %*
) else (
    echo Downloading and running latest VortexPlayer installer...
    powershell.exe -NoProfile -ExecutionPolicy Bypass -Command "irm https://raw.githubusercontent.com/sidhantjohnaind/VortexPlayer/main/install.ps1 | iex"
)

echo.
echo ==========================================================
echo Finished. You can close this window.
echo ==========================================================
pause
