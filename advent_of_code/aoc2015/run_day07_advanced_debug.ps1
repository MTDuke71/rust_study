#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Advanced Day 7 AoC debug runner with filtering options
.DESCRIPTION
    This script provides advanced options for running Day 7 debug output,
    including filtering specific sections and custom input handling.
.PARAMETER InputFile
    Path to input file or direct input content
.PARAMETER OutputFile  
    Output file path (optional)
.PARAMETER FilterSections
    Comma-separated list of sections to include: instructions, memo, evaluation, results
.PARAMETER MaxLines
    Maximum lines to capture (useful for large outputs)
.PARAMETER CreateInput
    Create a custom input file interactively
.EXAMPLE
    .\run_day07_advanced_debug.ps1 -FilterSections "instructions,results"
    .\run_day07_advanced_debug.ps1 -InputFile "custom.txt" -MaxLines 1000
    .\run_day07_advanced_debug.ps1 -CreateInput
#>

param(
    [string]$InputFile = "",
    [string]$OutputFile = "", 
    [string]$FilterSections = "",
    [int]$MaxLines = 0,
    [switch]$CreateInput = $false,
    [switch]$Help = $false
)

function Show-Help {
    Write-Host "🔧 Day 7 AoC Advanced Debug Runner" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Usage Examples:" -ForegroundColor Yellow
    Write-Host "  .\run_day07_advanced_debug.ps1                                    # Run with default input"
    Write-Host "  .\run_day07_advanced_debug.ps1 -InputFile custom.txt             # Use custom input file"
    Write-Host "  .\run_day07_advanced_debug.ps1 -FilterSections 'instructions'    # Show only instructions parsing"
    Write-Host "  .\run_day07_advanced_debug.ps1 -FilterSections 'results'         # Show only final results"
    Write-Host "  .\run_day07_advanced_debug.ps1 -MaxLines 500                     # Limit output to 500 lines"
    Write-Host "  .\run_day07_advanced_debug.ps1 -CreateInput                      # Create custom input interactively"
    Write-Host ""
    Write-Host "Filter Sections:" -ForegroundColor Yellow
    Write-Host "  - instructions: Instruction parsing and setup"
    Write-Host "  - memo: Hash table states and memoization"  
    Write-Host "  - evaluation: Wire evaluation steps"
    Write-Host "  - results: Final answers and summaries"
    Write-Host ""
    Write-Host "Tips:" -ForegroundColor Green
    Write-Host "  - Use quotes around filter sections with spaces"
    Write-Host "  - Combine sections with commas: 'instructions,results'"
    Write-Host "  - MaxLines applies after filtering"
}

if ($Help) {
    Show-Help
    exit 0
}

Write-Host "🔧 Day 7 AoC Advanced Debug Runner" -ForegroundColor Cyan

# Handle input creation
if ($CreateInput) {
    Write-Host ""
    Write-Host "📝 Custom Input Creator" -ForegroundColor Yellow
    Write-Host "Enter your circuit instructions (one per line)."
    Write-Host "Examples: '123 -> x', 'x AND y -> z', 'NOT x -> h'"
    Write-Host "Press Ctrl+Z (Windows) or Ctrl+D (Unix) when finished, or type 'END' on a new line."
    Write-Host ""
    
    $customLines = @()
    do {
        $line = Read-Host "Instruction"
        if ($line -eq "END") { break }
        if (![string]::IsNullOrWhiteSpace($line)) {
            $customLines += $line
        }
    } while ($null -ne $line)
    
    if ($customLines.Count -gt 0) {
        $timestamp = Get-Date -Format "yyyy-MM-dd_HH-mm-ss"
        $InputFile = "custom_input_$timestamp.txt"
        $customLines | Out-File -FilePath $InputFile -Encoding UTF8
        Write-Host "✅ Created custom input file: $InputFile" -ForegroundColor Green
        Write-Host "📊 Instructions: $($customLines.Count)" -ForegroundColor Cyan
    } else {
        Write-Host "❌ No instructions entered. Exiting." -ForegroundColor Red
        exit 1
    }
}

# Set default input file
if ([string]::IsNullOrEmpty($InputFile)) {
    $InputFile = "inputs\day07_example.txt"
}

# Generate default output filename
if ([string]::IsNullOrEmpty($OutputFile)) {
    $timestamp = Get-Date -Format "yyyy-MM-dd_HH-mm-ss"
    $filter_suffix = if ([string]::IsNullOrEmpty($FilterSections)) { "" } else { "_filtered" }
    $OutputFile = "day07_debug_advanced_$timestamp$filter_suffix.txt"
}

Write-Host ""
Write-Host "📁 Input File: $InputFile" -ForegroundColor Yellow
Write-Host "📄 Output File: $OutputFile" -ForegroundColor Yellow

if (![string]::IsNullOrEmpty($FilterSections)) {
    Write-Host "🔍 Filter Sections: $FilterSections" -ForegroundColor Yellow
}
if ($MaxLines -gt 0) {
    Write-Host "📏 Max Lines: $MaxLines" -ForegroundColor Yellow
}

# Check input file
if (-not (Test-Path $InputFile)) {
    Write-Host "❌ Error: Input file '$InputFile' not found!" -ForegroundColor Red
    exit 1
}

try {
    Write-Host ""
    Write-Host "🚀 Running Day 7 solution with advanced debug..." -ForegroundColor Green
    
    # Run the command and capture output
    $debugOutput = & cargo run -- 7 $InputFile --debug 2>&1
    
    if ($LASTEXITCODE -eq 0) {
        # Apply filtering if requested
        if (![string]::IsNullOrEmpty($FilterSections)) {
            $sections = $FilterSections.Split(',') | ForEach-Object { $_.Trim().ToLower() }
            $filteredOutput = @()
            
            foreach ($line in $debugOutput) {
                $include = $false
                
                # Check each filter section
                foreach ($section in $sections) {
                    switch ($section) {
                        "instructions" { 
                            if ($line -match "Adding instruction|Parsed as|Instructions loaded") {
                                $include = $true
                            }
                        }
                        "memo" { 
                            if ($line -match "Memo table|Memoizing|Found in memo") {
                                $include = $true
                            }
                        }
                        "evaluation" { 
                            if ($line -match "Evaluating|Computing|Wire.*=") {
                                $include = $true
                            }
                        }
                        "results" { 
                            if ($line -match "Part.*result|Final Results|Day 7 Part") {
                                $include = $true
                            }
                        }
                    }
                }
                
                # Also include section headers and important markers
                if ($line -match "^≡ƒ" -or $line -match "^Day 7" -or $line -match "Debug mode") {
                    $include = $true
                }
                
                if ($include) {
                    $filteredOutput += $line
                }
            }
            
            $debugOutput = $filteredOutput
            Write-Host "🔍 Applied filters: $FilterSections" -ForegroundColor Cyan
            Write-Host "📊 Filtered to $($debugOutput.Count) lines" -ForegroundColor Cyan
        }
        
        # Apply line limit if requested
        if ($MaxLines -gt 0 -and $debugOutput.Count -gt $MaxLines) {
            $debugOutput = $debugOutput | Select-Object -First $MaxLines
            Write-Host "📏 Limited to first $MaxLines lines" -ForegroundColor Cyan
        }
        
        # Save output
        $debugOutput | Out-File -FilePath $OutputFile -Encoding UTF8
        
        $outputInfo = Get-Item $OutputFile
        Write-Host ""
        Write-Host "✅ Advanced debug output saved!" -ForegroundColor Green
        Write-Host "📄 Output file: $OutputFile" -ForegroundColor Yellow
        Write-Host "📊 Output size: $([math]::Round($outputInfo.Length / 1KB, 2)) KB" -ForegroundColor Cyan
        Write-Host "📝 Lines written: $($debugOutput.Count)" -ForegroundColor Cyan
        
        # Show preview
        Write-Host ""
        Write-Host "📋 Preview (first 15 lines):" -ForegroundColor Magenta
        Write-Host "=" * 60 -ForegroundColor Gray
        $debugOutput | Select-Object -First 15 | ForEach-Object { Write-Host $_ }
        Write-Host "=" * 60 -ForegroundColor Gray
        
        if ($debugOutput.Count -gt 15) {
            Write-Host ""
            Write-Host "📋 Preview (last 10 lines):" -ForegroundColor Magenta
            Write-Host "=" * 60 -ForegroundColor Gray
            $debugOutput | Select-Object -Last 10 | ForEach-Object { Write-Host $_ }
            Write-Host "=" * 60 -ForegroundColor Gray
        }
        
    } else {
        Write-Host "❌ Error running solution:" -ForegroundColor Red
        $debugOutput | ForEach-Object { Write-Host $_ -ForegroundColor Red }
        exit 1
    }
    
} catch {
    Write-Host "❌ Unexpected error: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "🎉 Advanced debug complete!" -ForegroundColor Green