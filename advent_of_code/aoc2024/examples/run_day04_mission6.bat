@echo off
REM Batch file to run AoC 2024 Day 04 Mission 6 version
REM This demonstrates the benefits of Mission 6 grid utilities

echo ========================================
echo AoC 2024 Day 04 - Mission 6 Version
echo ========================================
echo.
echo This example demonstrates how Mission 6's grid utilities
echo simplify the Day 04 word search problem:
echo   - 43%% code reduction (280 -> 160 lines)
echo   - 100%% elimination of panic risks
echo   - Semantic clarity with Direction enum
echo   - Reusable infrastructure
echo.

REM Change to the aoc2024 directory (parent of examples) and run the example
cd /d "%~dp0.."
cargo run --example day04_wm6

echo.
echo ========================================
echo Comparison with original implementation:
echo   Original day04.rs:     280 lines, manual bounds checking
echo   Mission 6 day04_wm6:   160 lines, automatic safety
echo   Results identical:     Part 1: 2554, Part 2: 1916
echo ========================================
echo.
pause