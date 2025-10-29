#Requires -Version 5.1

<#
.SYNOPSIS
    Alphabetizes the broken links output file by link name.

.DESCRIPTION
    Reads the broken links output file, sorts the broken link entries alphabetically
    by the link name (first [[...]] in each line), and writes the sorted output
    to a new file while preserving the header message.

.PARAMETER InputFile
    Path to the input broken links file. Defaults to "broken links output.md"

.PARAMETER OutputFile  
    Path to the output alphabetized file. Defaults to ".system/broken links output - alphabetized.md"

.PARAMETER WorkspaceRoot
    Path to the workspace root directory. Defaults to current directory.

.EXAMPLE
    .\Alphabetize-BrokenLinks.ps1
    
    Alphabetizes the default broken links file and creates an alphabetized version.

.EXAMPLE
    .\Alphabetize-BrokenLinks.ps1 -InputFile "custom-broken-links.md" -OutputFile "custom-sorted.md"
    
    Alphabetizes a custom input file and saves to a custom output file.
#>

param(
    [string]$InputFile = "broken links output.md",
    [string]$OutputFile = ".system/broken links output - alphabetized.md", 
    [string]$WorkspaceRoot = "."
)

# Resolve full paths
$InputPath = Join-Path $WorkspaceRoot $InputFile
$OutputPath = Join-Path $WorkspaceRoot $OutputFile

# Ensure the output directory exists
$OutputDir = Split-Path $OutputPath -Parent
if (-not (Test-Path $OutputDir)) {
    Write-Host "📁 Creating output directory: $OutputDir" -ForegroundColor Yellow
    New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null
}

Write-Host "🔍 Alphabetizing Broken Links Output" -ForegroundColor Cyan
Write-Host "=" * 40

# Verify input file exists
if (-not (Test-Path $InputPath)) {
    Write-Error "❌ Input file not found: $InputPath"
    exit 1
}

Write-Host "📂 Input file: $InputPath" -ForegroundColor Green
Write-Host "📝 Output file: $OutputPath" -ForegroundColor Green

try {
    # Read all lines from the input file
    $lines = Get-Content -Path $InputPath -Encoding UTF8
    
    if ($lines.Count -eq 0) {
        Write-Warning "⚠️  Input file is empty"
        exit 0
    }
    
    Write-Host "📊 Processing $($lines.Count) lines..." -ForegroundColor Yellow
    
    # Find the header (lines before the first broken link entry)
    $headerLines = @()
    $linkLines = @()
    $inHeader = $true
    
    foreach ($line in $lines) {
        if ($inHeader -and $line -match '^- \[\[.+\]\] in \[\[.+\]\]') {
            $inHeader = $false
        }
        
        if ($inHeader) {
            $headerLines += $line
        } else {
            # Only add lines that match the broken link pattern
            if ($line -match '^- \[\[.+\]\] in \[\[.+\]\]') {
                $linkLines += $line
            }
        }
    }
    
    Write-Host "📋 Found $($headerLines.Count) header lines" -ForegroundColor Magenta
    Write-Host "🔗 Found $($linkLines.Count) broken link entries" -ForegroundColor Magenta
    
    # Extract and sort broken links
    # Create custom objects for sorting with the link name as the sort key
    $sortedEntries = $linkLines | ForEach-Object {
        # Extract the first [[...]] which is the broken link name
        if ($_ -match '^- \[\[([^\]]+)\]\] in') {
            [PSCustomObject]@{
                LinkName = $matches[1]
                FullLine = $_
            }
        }
    } | Sort-Object LinkName
    
    Write-Host "✅ Sorted $($sortedEntries.Count) entries alphabetically" -ForegroundColor Green
    
    # Build output content
    $outputContent = @()
    
    # Add header lines
    $outputContent += $headerLines
    
    # Add sorted broken link entries
    $outputContent += $sortedEntries | ForEach-Object { $_.FullLine }
    
    # Write to output file
    $outputContent | Set-Content -Path $OutputPath -Encoding UTF8
    
    Write-Host "🎉 Success! Alphabetized broken links written to:" -ForegroundColor Green
    Write-Host "   $OutputPath" -ForegroundColor White
    
    # Show some statistics
    Write-Host "`n📈 Statistics:" -ForegroundColor Cyan
    Write-Host "   • Total lines processed: $($lines.Count)" -ForegroundColor White
    Write-Host "   • Header lines preserved: $($headerLines.Count)" -ForegroundColor White  
    Write-Host "   • Broken links alphabetized: $($sortedEntries.Count)" -ForegroundColor White
    Write-Host "   • Output file size: $((Get-Item $OutputPath).Length) bytes" -ForegroundColor White
    
    # Show first few and last few entries as preview
    if ($sortedEntries.Count -gt 0) {
        Write-Host "`n🔍 Preview (first 3 alphabetical entries):" -ForegroundColor Cyan
        $sortedEntries | Select-Object -First 3 | ForEach-Object {
            Write-Host "   $($_.FullLine)" -ForegroundColor Gray
        }
        
        if ($sortedEntries.Count -gt 6) {
            Write-Host "   ..." -ForegroundColor Gray
            Write-Host "`n🔍 Preview (last 3 alphabetical entries):" -ForegroundColor Cyan
            $sortedEntries | Select-Object -Last 3 | ForEach-Object {
                Write-Host "   $($_.FullLine)" -ForegroundColor Gray
            }
        }
    }

} catch {
    Write-Error "❌ Error processing files: $($_.Exception.Message)"
    Write-Host "💡 Stack trace:" -ForegroundColor Yellow
    Write-Host $_.ScriptStackTrace -ForegroundColor Gray
    exit 1
}

Write-Host "`n✨ Broken links alphabetization completed successfully!" -ForegroundColor Green