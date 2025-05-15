@echo off
echo Starting FreeBonde Marketplace development environment...

:: Setting project path
cd /d %~dp0

:: Checking Node.js environment
where node >nul 2>nul
if %errorlevel% neq 0 (
echo Error: Node.js not detected, please install Node.js first
pause
exit /b 1
)

:: Install dependencies (if node_modules does not exist)
if not exist node_modules (
echo Installing project dependencies...
npm install
)

:: Starting development server
echo Starting development server...
npm start

pause