@echo off
REM Simple batch wrapper for running markdown code
REM Usage: run_md.bat Day11.md

if "%~1"=="" (
    echo Usage: run_md.bat ^<markdown_file^>
    echo Example: run_md.bat daily_study\rust_learning_week2_notes\Day11.md
    exit /b 1
)

powershell -ExecutionPolicy Bypass -File "run_markdown_code.ps1" "%~1"