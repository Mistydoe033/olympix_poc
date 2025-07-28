# Olympix Pattern Tool - Build Script
param(
    [switch]$SkipBuild,
    [switch]$SkipInstaller,
    [string]$Version = "1.0.0"
)

Write-Host "🔨 Olympix Pattern Tool - Build Script" -ForegroundColor Green
Write-Host "=====================================" -ForegroundColor Green

# Check if Rust is installed
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Rust is not installed. Please install Rust from https://rustup.rs/" -ForegroundColor Red
    exit 1
}

# Set variables
$ProjectName = "olympix-pattern-tool"
$ExeName = "$ProjectName.exe"
$BuildDir = "target\release"
$ExePath = "$BuildDir\$ExeName"
$InstallerDir = "installer"

# Clean previous builds
Write-Host "🧹 Cleaning previous builds..." -ForegroundColor Yellow
if (Test-Path $InstallerDir) {
    Remove-Item -Recurse -Force $InstallerDir
}

# Build the application
if (-not $SkipBuild) {
    Write-Host "🔨 Building application..." -ForegroundColor Yellow
    
    # Update dependencies
    Write-Host "📦 Updating dependencies..." -ForegroundColor Cyan
    cargo update
    
    # Build release version
    Write-Host "⚙️ Compiling release version..." -ForegroundColor Cyan
    cargo build --release
    
    if (-not (Test-Path $ExePath)) {
        Write-Host "❌ Build failed! Executable not found at $ExePath" -ForegroundColor Red
        exit 1
    }
    
    Write-Host "✅ Build completed successfully!" -ForegroundColor Green
    Write-Host "📁 Executable location: $ExePath" -ForegroundColor Cyan
}

# Create installer
if (-not $SkipInstaller) {
    Write-Host "📦 Creating installer..." -ForegroundColor Yellow
    
    # Create installer directory structure
    New-Item -ItemType Directory -Force -Path $InstallerDir | Out-Null
    
    # Copy executable
    Copy-Item $ExePath "$InstallerDir\$ExeName"
    
    # Copy sample contracts
    if (Test-Path "contracts") {
        New-Item -ItemType Directory -Force -Path "$InstallerDir\contracts" | Out-Null
        Copy-Item "contracts\*.sol" "$InstallerDir\contracts\" -ErrorAction SilentlyContinue
    }
    
    # Copy sample patterns
    if (Test-Path "patterns") {
        New-Item -ItemType Directory -Force -Path "$InstallerDir\patterns" | Out-Null
        Copy-Item "patterns\*" "$InstallerDir\patterns\" -Recurse -ErrorAction SilentlyContinue
    }
    
    # Create batch file for installation
    $BatchContent = @"
@echo off
echo Installing Olympix Pattern Tool...
echo.
if not exist "%USERPROFILE%\Documents\Olympix Pattern Tool" mkdir "%USERPROFILE%\Documents\Olympix Pattern Tool"
if not exist "%USERPROFILE%\Documents\Olympix Pattern Tool\contracts" mkdir "%USERPROFILE%\Documents\Olympix Pattern Tool\contracts"
if not exist "%USERPROFILE%\Documents\Olympix Pattern Tool\patterns" mkdir "%USERPROFILE%\Documents\Olympix Pattern Tool\patterns"
copy "%~dp0$ExeName" "%USERPROFILE%\Documents\Olympix Pattern Tool\"
if exist "%~dp0contracts\*.sol" copy "%~dp0contracts\*.sol" "%USERPROFILE%\Documents\Olympix Pattern Tool\contracts\"
if exist "%~dp0patterns\*" copy "%~dp0patterns\*" "%USERPROFILE%\Documents\Olympix Pattern Tool\patterns\" /E /I
echo Installation completed!
echo The application is now available at: %USERPROFILE%\Documents\Olympix Pattern Tool\$ExeName
pause
"@
    
    Set-Content -Path "$InstallerDir\install.bat" -Value $BatchContent
    
    # Create README for installer
    $ReadmeContent = @"
# Olympix Pattern Tool - Installation

## Quick Install
1. Run install.bat as Administrator
2. The application will be installed to your Documents folder

## Manual Install
1. Copy $ExeName to your preferred location
2. Create the following directory structure in your Documents folder:
   - Documents\Olympix Pattern Tool\
   - Documents\Olympix Pattern Tool\contracts\
   - Documents\Olympix Pattern Tool\patterns\

## Usage
- Double-click the executable to start the application
- Place your .sol files in the contracts directory
- The application will automatically scan for vulnerabilities

## Version: $Version
"@
    
    Set-Content -Path "$InstallerDir\README.txt" -Value $ReadmeContent
    
    Write-Host "✅ Installer created successfully!" -ForegroundColor Green
    Write-Host "📁 Installer location: $InstallerDir" -ForegroundColor Cyan
    Write-Host "📋 Run install.bat to install the application" -ForegroundColor Cyan
}

# Display summary
Write-Host "`n🎉 Build Summary:" -ForegroundColor Green
Write-Host "================" -ForegroundColor Green
Write-Host "✅ Application: $ExePath" -ForegroundColor Cyan
Write-Host "✅ Installer: $InstallerDir" -ForegroundColor Cyan
Write-Host "✅ Version: $Version" -ForegroundColor Cyan

if (-not $SkipInstaller) {
    Write-Host "`n📋 Next Steps:" -ForegroundColor Yellow
    Write-Host "1. Test the application: .\$ExePath" -ForegroundColor White
    Write-Host "2. Install: .\$InstallerDir\install.bat" -ForegroundColor White
    Write-Host "3. Distribute the $InstallerDir folder" -ForegroundColor White
}

Write-Host "`n✨ Build completed successfully!" -ForegroundColor Green 