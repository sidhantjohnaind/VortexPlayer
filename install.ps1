[CmdletBinding()]
param (
    [string]$Version = "latest",
    [string]$InstallDir = "$env:LOCALAPPDATA\Programs\VortexPlayer",
    [switch]$NoShortcuts,
    [switch]$NoPath,
    [switch]$Launch
)

$ErrorActionPreference = "Stop"

Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host "       VortexPlayer One-Click Windows Installer           " -ForegroundColor Cyan
Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host ""

$Repo = "sidhantjohnaind/VortexPlayer"

# 1. Determine release tag
Write-Host "[1/5] Resolving release version..." -ForegroundColor Yellow
$Tag = $Version
if ($Version -eq "latest") {
    try {
        [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
        $ReleaseUrl = "https://api.github.com/repos/$Repo/releases/latest"
        $ReleaseInfo = Invoke-RestMethod -Uri $ReleaseUrl -Headers @{ "User-Agent" = "VortexPlayer-Installer" }
        $Tag = $ReleaseInfo.tag_name
        Write-Host "  -> Found latest version: $Tag" -ForegroundColor Green
    } catch {
        $Tag = "v1.1.0"
        Write-Host "  -> Defaulting to version: $Tag" -ForegroundColor Yellow
    }
}

# 2. Architecture detection
$Arch = if ([Environment]::Is64BitOperatingSystem) {
    if ($env:PROCESSOR_ARCHITECTURE -eq "ARM64") { "arm64" } else { "x64" }
} else {
    "x64"
}

# Select asset: prefer complete zip bundle
$AssetPattern = if ($Arch -eq "arm64") {
    "VortexPlayer-windows-arm64.zip"
} else {
    "VortexPlayer-$Tag-windows-x64.zip"
}

$DownloadUrl = "https://github.com/$Repo/releases/download/$Tag/$AssetPattern"
$TempZip = Join-Path $env:TEMP "VortexPlayer-$Tag-$Arch.zip"

Write-Host "[2/5] Downloading $AssetPattern..." -ForegroundColor Yellow
Write-Host "      URL: $DownloadUrl" -ForegroundColor DarkGray

try {
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempZip -UseBasicParsing
} catch {
    $FallbackUrl = "https://github.com/$Repo/releases/download/$Tag/VortexPlayer-windows-x64.zip"
    Write-Host "  -> Trying fallback URL: $FallbackUrl" -ForegroundColor Yellow
    Invoke-WebRequest -Uri $FallbackUrl -OutFile $TempZip -UseBasicParsing
}

if (-not (Test-Path $TempZip)) {
    throw "Failed to download VortexPlayer release package."
}

Write-Host "[3/5] Installing to $InstallDir..." -ForegroundColor Yellow
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

$TempExtract = Join-Path $env:TEMP "VortexExtract_$([System.Guid]::NewGuid().ToString('N'))"
Expand-Archive -Path $TempZip -DestinationPath $TempExtract -Force

$SourceDir = $TempExtract
$NestedDir = Get-ChildItem -Path $TempExtract -Directory | Select-Object -First 1
if ($NestedDir -and (Test-Path (Join-Path $NestedDir.FullName "vortex-player-egui.exe"))) {
    $SourceDir = $NestedDir.FullName
} elseif (-not (Test-Path (Join-Path $TempExtract "vortex-player-egui.exe"))) {
    $FoundExe = Get-ChildItem -Path $TempExtract -Filter "vortex*.exe" -Recurse | Select-Object -First 1
    if ($FoundExe) {
        $SourceDir = $FoundExe.DirectoryName
    }
}

Copy-Item -Path "$SourceDir\*" -Destination $InstallDir -Recurse -Force

Remove-Item -Path $TempZip -Force -ErrorAction SilentlyContinue
Remove-Item -Path $TempExtract -Recurse -Force -ErrorAction SilentlyContinue

$ExePath = Join-Path $InstallDir "vortex-player-egui.exe"
if (-not (Test-Path $ExePath)) {
    $AltExe = Get-ChildItem -Path $InstallDir -Filter "vortex*.exe" | Select-Object -First 1
    if ($AltExe) { $ExePath = $AltExe.FullName }
}

Write-Host "  -> Files installed successfully." -ForegroundColor Green

# 3. Create Shortcuts
if (-not $NoShortcuts) {
    Write-Host "[4/5] Creating shortcuts..." -ForegroundColor Yellow
    $WshShell = New-Object -ComObject WScript.Shell

    $StartMenuDir = Join-Path $env:APPDATA "Microsoft\Windows\Start Menu\Programs"
    $StartShortcutPath = Join-Path $StartMenuDir "VortexPlayer.lnk"
    $StartShortcut = $WshShell.CreateShortcut($StartShortcutPath)
    $StartShortcut.TargetPath = $ExePath
    $StartShortcut.WorkingDirectory = $InstallDir
    $StartShortcut.Description = "VortexPlayer - Modern Media Player"
    $StartShortcut.Save()
    Write-Host "  -> Start Menu shortcut created." -ForegroundColor DarkGray

    $DesktopDir = [Environment]::GetFolderPath("Desktop")
    $DesktopShortcutPath = Join-Path $DesktopDir "VortexPlayer.lnk"
    $DesktopShortcut = $WshShell.CreateShortcut($DesktopShortcutPath)
    $DesktopShortcut.TargetPath = $ExePath
    $DesktopShortcut.WorkingDirectory = $InstallDir
    $DesktopShortcut.Description = "VortexPlayer - Modern Media Player"
    $DesktopShortcut.Save()
    Write-Host "  -> Desktop shortcut created." -ForegroundColor DarkGray
} else {
    Write-Host "[4/5] Skipping shortcuts (-NoShortcuts specified)." -ForegroundColor DarkGray
}

# 4. Add to User PATH
if (-not $NoPath) {
    Write-Host "[5/5] Updating user PATH..." -ForegroundColor Yellow
    $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $Paths = $UserPath -split ";"
    if ($Paths -notcontains $InstallDir) {
        $NewPath = ($Paths + $InstallDir) -join ";"
        [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
        $env:Path = "$env:Path;$InstallDir"
        Write-Host "  -> PATH updated." -ForegroundColor DarkGray
    } else {
        Write-Host "  -> Already in PATH." -ForegroundColor DarkGray
    }
} else {
    Write-Host "[5/5] Skipping PATH update (-NoPath specified)." -ForegroundColor DarkGray
}

# Create uninstaller
$UninstallerLines = @(
    'Write-Host "Uninstalling VortexPlayer..." -ForegroundColor Yellow',
    "`$InstallDir = `"$InstallDir`"",
    "`$DesktopShortcut = Join-Path ([Environment]::GetFolderPath('Desktop')) 'VortexPlayer.lnk'",
    "`$StartShortcut = Join-Path `"`$env:APPDATA\Microsoft\Windows\Start Menu\Programs`" 'VortexPlayer.lnk'",
    'if (Test-Path $DesktopShortcut) { Remove-Item $DesktopShortcut -Force }',
    'if (Test-Path $StartShortcut) { Remove-Item $StartShortcut -Force }',
    "`$UserPath = [Environment]::GetEnvironmentVariable('Path', 'User')",
    "`$Paths = `$UserPath -split ';' | Where-Object { `$_ -ne `$InstallDir -and `$_ }",
    "[Environment]::SetEnvironmentVariable('Path', (`$Paths -join ';'), 'User')",
    'Write-Host "Removing installation files..." -ForegroundColor Yellow',
    'Remove-Item -Recurse -Force $InstallDir',
    'Write-Host "VortexPlayer uninstalled successfully." -ForegroundColor Green'
)
$UninstallerContent = $UninstallerLines -join [Environment]::NewLine
Set-Content -Path (Join-Path $InstallDir "uninstall.ps1") -Value $UninstallerContent -Encoding utf8

Write-Host ""
Write-Host "==========================================================" -ForegroundColor Green
Write-Host "         Installation completed successfully!             " -ForegroundColor Green
Write-Host "==========================================================" -ForegroundColor Green
Write-Host " Location:   $InstallDir" -ForegroundColor White
Write-Host " Executable: $ExePath" -ForegroundColor White
Write-Host ""

if ($Launch) {
    Write-Host "Launching VortexPlayer..." -ForegroundColor Cyan
    Start-Process -FilePath $ExePath
}
