#!/usr/bin/env pwsh
# Error Messages Demo Script - Chapter 12.6
# Demonstrates proper stderr/stdout separation and error handling

Write-Host "🦀 Error Messages Demo - Chapter 12.6" -ForegroundColor Cyan
Write-Host "=======================================" -ForegroundColor Cyan

Write-Host "`n📋 Testing Usage Help:" -ForegroundColor Yellow
Write-Host "Running: cargo run (no arguments)" -ForegroundColor Gray
cargo run | Out-Host

Write-Host "`n📋 Testing Error Types Demo:" -ForegroundColor Yellow  
Write-Host "Running: cargo run --demo-errors" -ForegroundColor Gray
cargo run -- --demo-errors | Out-Host

Write-Host "`n📋 Testing Case-Sensitive Search:" -ForegroundColor Yellow  
Write-Host "Running: cargo run nobody poem.txt" -ForegroundColor Gray
cargo run nobody poem.txt | Out-Host

Write-Host "`n📋 Testing Case-Insensitive Search:" -ForegroundColor Yellow
Write-Host "Running: `$env:CASE_INSENSITIVE='1'; cargo run NOBODY poem.txt" -ForegroundColor Gray
$env:CASE_INSENSITIVE = "1"
cargo run NOBODY poem.txt | Out-Host
$env:CASE_INSENSITIVE = $null

Write-Host "`n📋 Testing No Matches Found:" -ForegroundColor Yellow
Write-Host "Running: cargo run xyz poem.txt" -ForegroundColor Gray  
cargo run xyz poem.txt | Out-Host

Write-Host "`n📋 Testing File Not Found Error:" -ForegroundColor Yellow
Write-Host "Running: cargo run test nonexistent.txt" -ForegroundColor Gray
cargo run test nonexistent.txt 2>&1 | Out-Host

Write-Host "`n📋 Testing Missing Arguments Error:" -ForegroundColor Yellow
Write-Host "Running: cargo run onlyquery" -ForegroundColor Gray
cargo run onlyquery 2>&1 | Out-Host

Write-Host "`n🔄 Testing Stream Separation:" -ForegroundColor Yellow
Write-Host "Running: cargo run nobody poem.txt > stdout_only.txt 2> stderr_only.txt" -ForegroundColor Gray
cargo run nobody poem.txt > stdout_only.txt 2> stderr_only.txt

Write-Host "`nContents of stdout_only.txt (search results):" -ForegroundColor Green
if (Test-Path "stdout_only.txt") {
    Get-Content "stdout_only.txt" | ForEach-Object { Write-Host "  $_" -ForegroundColor White }
    Remove-Item "stdout_only.txt"
} else {
    Write-Host "  (no stdout output)" -ForegroundColor Gray
}

Write-Host "`nContents of stderr_only.txt (debug/error messages):" -ForegroundColor Red
if (Test-Path "stderr_only.txt") {
    Get-Content "stderr_only.txt" | ForEach-Object { Write-Host "  $_" -ForegroundColor White }
    Remove-Item "stderr_only.txt"
} else {
    Write-Host "  (no stderr output)" -ForegroundColor Gray
}

Write-Host "`n🧪 Running Tests:" -ForegroundColor Yellow
Write-Host "Running: cargo test" -ForegroundColor Gray
cargo test | Out-Host

Write-Host "`n✅ Demo Complete!" -ForegroundColor Green
Write-Host "📖 Key Points Demonstrated:" -ForegroundColor Green
Write-Host "  • Error messages go to stderr (eprintln!)" -ForegroundColor White
Write-Host "  • Search results go to stdout (println!)" -ForegroundColor White  
Write-Host "  • Proper exit codes (0 = success, 1 = error)" -ForegroundColor White
Write-Host "  • Stream separation allows independent redirection" -ForegroundColor White
Write-Host "  • Clear, actionable error messages with context" -ForegroundColor White