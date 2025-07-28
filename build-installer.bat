@echo off
echo Building Olympix Pattern Tool Installer...
echo.

echo Step 1: Installing frontend dependencies...
cd src
call npm install
if errorlevel 1 (
    echo Error: Failed to install frontend dependencies
    pause
    exit /b 1
)

echo Step 2: Building frontend...
call npm run build
if errorlevel 1 (
    echo Error: Failed to build frontend
    pause
    exit /b 1
)

echo Step 3: Building Tauri application...
cd ..
call cargo tauri build
if errorlevel 1 (
    echo Error: Failed to build Tauri application
    pause
    exit /b 1
)

echo.
echo Build completed successfully!
echo.
echo Installer location: src-tauri\target\release\bundle\msi\
echo.
echo The installer will:
echo - Install the application to Program Files
echo - Create desktop and start menu shortcuts
echo - Set up the contracts and patterns folders
echo - Make the app ready to use immediately
echo.
pause 