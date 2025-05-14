@echo off
echo 正在启动 FreeBonde Marketplace 开发环境...

:: 设置项目路径
cd /d %~dp0

:: 检查 Node.js 环境
where node >nul 2>nul
if %errorlevel% neq 0 (
    echo 错误：未检测到 Node.js，请先安装 Node.js
    pause
    exit /b 1
)

:: 安装依赖（如果 node_modules 不存在）
if not exist node_modules (
    echo 正在安装项目依赖...
    npm install
)

:: 启动开发服务器
echo 正在启动开发服务器...
npm start

pause