#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Run Day 7 AoC solution with debug output and save to file
.DESCRIPTION
    This script runs the Day 7 circuit simulation with full debug output,
    showing all instruction parsing and hash table states, then saves the 
    output to a timestamped file.
.PARAMETER InputFile
    Path to the input file (defaults to day07_example.txt)
.PARAMETER OutputFile
    Path to save debug output (defaults to timestamped file)
.PARAMETER ShowOutput
    Whether to display output to console as well as saving to file
.EXAMPLE
    .\run_day07_debug.ps1
    .\run_day07_debug.ps1 -InputFile "my_input.txt" -OutputFile "debug_output.txt"
    .\run_day07_debug.ps1 -ShowOutput
#>

param(
    [string]$InputFile = "inputs\day07_example.txt",
    [string]$OutputFile = "",
    [switch]$ShowOutput = $false
)

# Generate default output filename with timestamp if not provided
if ([string]::IsNullOrEmpty($OutputFile)) {
    $timestamp = Get-Date -Format "yyyy-MM-dd_HH-mm-ss"
    $OutputFile = "day07_debug_output_$timestamp.txt"
}

Write-Host "🔧 Day 7 AoC Debug Output Generator" -ForegroundColor Cyan
Write-Host "📁 Input File: $InputFile" -ForegroundColor Yellow
Write-Host "📄 Output File: $OutputFile" -ForegroundColor Yellow
Write-Host ""

# Check if input file exists
if (-not (Test-Path $InputFile)) {
    Write-Host "❌ Error: Input file '$InputFile' not found!" -ForegroundColor Red
    Write-Host "Available input files:" -ForegroundColor Yellow
    Get-ChildItem inputs -Filter "day07*.txt" | ForEach-Object { Write-Host "  - $($_.Name)" -ForegroundColor Green }
    exit 1
}

# Get input file info
$inputInfo = Get-Item $InputFile
Write-Host "📊 Input file size: $($inputInfo.Length) bytes ($($inputInfo.Length / 1KB) KB)" -ForegroundColor Cyan

try {
    Write-Host "🚀 Running Day 7 solution with debug output..." -ForegroundColor Green
    
    # Run the command and capture output
    $debugOutput = & cargo run -- 7 $InputFile --debug 2>&1
    
    if ($LASTEXITCODE -eq 0) {
        # Save output to file
        $debugOutput | Out-File -FilePath $OutputFile -Encoding UTF8
        
        $outputInfo = Get-Item $OutputFile
        Write-Host "✅ Debug output saved successfully!" -ForegroundColor Green
        Write-Host "📄 Output file: $OutputFile" -ForegroundColor Yellow
        Write-Host "📊 Output size: $($outputInfo.Length) bytes ($([math]::Round($outputInfo.Length / 1KB, 2)) KB)" -ForegroundColor Cyan
        Write-Host "📝 Lines written: $(($debugOutput | Measure-Object -Line).Lines)" -ForegroundColor Cyan
        
        # Show preview of output
        Write-Host ""
        Write-Host "📋 Preview (first 10 lines):" -ForegroundColor Magenta
        Write-Host "=" * 50 -ForegroundColor Gray
        $debugOutput | Select-Object -First 10 | ForEach-Object { Write-Host $_ }
        Write-Host "=" * 50 -ForegroundColor Gray
        Write-Host ""
        
        Write-Host "📋 Preview (last 10 lines):" -ForegroundColor Magenta
        Write-Host "=" * 50 -ForegroundColor Gray
        $debugOutput | Select-Object -Last 10 | ForEach-Object { Write-Host $_ }
        Write-Host "=" * 50 -ForegroundColor Gray
        
        # Optionally show full output
        if ($ShowOutput) {
            Write-Host ""
            Write-Host "📺 Full Debug Output:" -ForegroundColor Magenta
            Write-Host "=" * 80 -ForegroundColor Gray
            $debugOutput | ForEach-Object { Write-Host $_ }
            Write-Host "=" * 80 -ForegroundColor Gray
        }
        
        Write-Host ""
        Write-Host "💡 Tip: View the full output with: Get-Content '$OutputFile' | More" -ForegroundColor Blue
        Write-Host "💡 Tip: Search in output with: Select-String 'pattern' '$OutputFile'" -ForegroundColor Blue
        
    } else {
        Write-Host "❌ Error running Day 7 solution:" -ForegroundColor Red
        $debugOutput | ForEach-Object { Write-Host $_ -ForegroundColor Red }
        exit 1
    }
    
} catch {
    Write-Host "❌ Unexpected error: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "🎉 Debug output generation complete!" -ForegroundColor Green