#!/usr/bin/env pwsh
# MiniGrep Demo Script
# Demonstrates all functionality of the Chapter 12.5 complete program

Write-Host "🦀 MiniGrep - Complete Rust CLI Tool Demo" -ForegroundColor Cyan
Write-Host "===========================================" -ForegroundColor Cyan

Write-Host "`n📋 Testing Usage Help:" -ForegroundColor Yellow
Write-Host "Running: cargo run (no arguments)" -ForegroundColor Gray
try { 
    cargo run 2>&1 | Out-Host
} catch {
    Write-Host "Expected: Shows usage information and examples" -ForegroundColor Green
}

Write-Host "`n📋 Testing Case-Sensitive Search:" -ForegroundColor Yellow  
Write-Host "Running: cargo run nobody poem.txt" -ForegroundColor Gray
cargo run nobody poem.txt | Out-Host

Write-Host "`n📋 Testing Case-Insensitive Search:" -ForegroundColor Yellow
Write-Host "Running: CASE_INSENSITIVE=1 cargo run NOBODY poem.txt" -ForegroundColor Gray
$env:CASE_INSENSITIVE = "1"
cargo run NOBODY poem.txt | Out-Host
$env:CASE_INSENSITIVE = $null

Write-Host "`n📋 Testing No Matches Found:" -ForegroundColor Yellow
Write-Host "Running: cargo run xyz poem.txt" -ForegroundColor Gray  
cargo run xyz poem.txt | Out-Host

Write-Host "`n📋 Testing File Not Found:" -ForegroundColor Yellow
Write-Host "Running: cargo run test nonexistent.txt" -ForegroundColor Gray
try {
    cargo run test nonexistent.txt 2>&1 | Out-Host
} catch {
    Write-Host "Expected: File not found error" -ForegroundColor Green
}

Write-Host "`n✅ Demo Complete!" -ForegroundColor Green
Write-Host "📖 This is a fully functional, standalone Rust CLI program." -ForegroundColor Green