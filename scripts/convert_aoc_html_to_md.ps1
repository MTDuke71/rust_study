#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Converts AoC HTML problem statements to Markdown format.

.DESCRIPTION
    Converts Advent of Code HTML problem statements to clean Markdown files.
    Tries pandoc first (if available), falls back to basic HTML parsing.

.PARAMETER Year
    The AoC year (default: 2023)

.PARAMETER StartDay
    First day to convert (default: 1)

.PARAMETER EndDay
    Last day to convert (default: 25)

.EXAMPLE
    .\convert_aoc_html_to_md.ps1
    .\convert_aoc_html_to_md.ps1 -Year 2023 -StartDay 1 -EndDay 25
#>

param(
    [int]$Year = 2023,
    [int]$StartDay = 1,
    [int]$EndDay = 25
)

# Setup directories
$BaseDir = "advent_of_code\aoc$Year\Problem_Statements\days"
$InputDir = $BaseDir
$OutputDir = $BaseDir

Write-Host "=== AoC HTML to Markdown Converter ===" -ForegroundColor Cyan
Write-Host ""
Write-Host "Input directory:  $InputDir" -ForegroundColor Gray
Write-Host "Output directory: $OutputDir" -ForegroundColor Gray
Write-Host ""

# Check if pandoc is available
$UsePandoc = $false
try {
    $null = Get-Command pandoc -ErrorAction Stop
    $UsePandoc = $true
    Write-Host "✓ pandoc found - using pandoc for conversion" -ForegroundColor Green
}
catch {
    Write-Host "⚠ pandoc not found - using basic HTML parsing" -ForegroundColor Yellow
    Write-Host "  Install pandoc for better conversion: https://pandoc.org/installing.html" -ForegroundColor Gray
}
Write-Host ""

# Conversion function using pandoc
function Convert-WithPandoc {
    param([string]$HtmlFile, [string]$MdFile)
    
    try {
        # Extract just the article content first
        $html = Get-Content $HtmlFile -Raw
        
        # Find the main article content
        if ($html -match '(?s)<article[^>]*>(.*?)</article>') {
            $articleContent = $matches[1]
            
            # Create a temporary clean HTML file
            $tempHtml = [System.IO.Path]::GetTempFileName() + ".html"
            @"
<!DOCTYPE html>
<html>
<head><meta charset="UTF-8"></head>
<body>
<article>
$articleContent
</article>
</body>
</html>
"@ | Out-File -FilePath $tempHtml -Encoding UTF8
            
            # Convert with pandoc
            pandoc -f html -t gfm --wrap=none $tempHtml -o $MdFile
            
            # Clean up temp file
            Remove-Item $tempHtml
            
            return $true
        }
        else {
            # Fallback: convert entire file
            pandoc -f html -t gfm --wrap=none $HtmlFile -o $MdFile
            return $true
        }
    }
    catch {
        return $false
    }
}

# Conversion function using basic parsing
function Convert-WithBasicParsing {
    param([string]$HtmlFile, [string]$MdFile)
    
    try {
        $html = Get-Content $HtmlFile -Raw
        
        # Extract article content
        if ($html -match '(?s)<article[^>]*>(.*?)</article>') {
            $content = $matches[1]
        }
        else {
            $content = $html
        }
        
        # Basic HTML to Markdown conversion
        $markdown = $content
        
        # Headers
        $markdown = $markdown -replace '<h2[^>]*>(.*?)</h2>', "`n## `$1`n"
        $markdown = $markdown -replace '<h3[^>]*>(.*?)</h3>', "`n### `$1`n"
        $markdown = $markdown -replace '<h4[^>]*>(.*?)</h4>', "`n#### `$1`n"
        
        # Emphasis
        $markdown = $markdown -replace '<em[^>]*>(.*?)</em>', '*$1*'
        $markdown = $markdown -replace '<strong[^>]*>(.*?)</strong>', '**$1**'
        $markdown = $markdown -replace '<code[^>]*>(.*?)</code>', '`$1`'
        
        # Pre/code blocks
        $markdown = $markdown -replace '(?s)<pre><code>(.*?)</code></pre>', "`n`````n`$1`n`````n"
        
        # Lists
        $markdown = $markdown -replace '<ul[^>]*>', "`n"
        $markdown = $markdown -replace '</ul>', "`n"
        $markdown = $markdown -replace '<li[^>]*>', '- '
        $markdown = $markdown -replace '</li>', "`n"
        
        # Paragraphs
        $markdown = $markdown -replace '<p[^>]*>', "`n"
        $markdown = $markdown -replace '</p>', "`n"
        
        # Links
        $markdown = $markdown -replace '<a[^>]*href=[''"]([^''"]*)[''"][^>]*>(.*?)</a>', '[$2]($1)'
        
        # Remove remaining HTML tags
        $markdown = $markdown -replace '<[^>]+>', ''
        
        # Decode HTML entities
        $markdown = [System.Web.HttpUtility]::HtmlDecode($markdown)
        
        # Clean up whitespace
        $markdown = $markdown -replace '(?m)^\s+$', ''
        $markdown = $markdown -replace '(?m)\n{3,}', "`n`n"
        $markdown = $markdown.Trim()
        
        # Save to file
        $markdown | Out-File -FilePath $MdFile -Encoding UTF8
        
        return $true
    }
    catch {
        return $false
    }
}

# Load System.Web for HTML decoding (basic parser only)
if (-not $UsePandoc) {
    Add-Type -AssemblyName System.Web
}

# Conversion loop
$SuccessCount = 0
$FailCount = 0

for ($Day = $StartDay; $Day -le $EndDay; $Day++) {
    $DayPadded = "{0:D2}" -f $Day
    
    $HtmlFile = "$InputDir\day$DayPadded.html"
    $MdFile = "$OutputDir\day$DayPadded.md"
    
    if (-not (Test-Path $HtmlFile)) {
        Write-Host "Day ${DayPadded}: " -NoNewline -ForegroundColor Yellow
        Write-Host "✗ " -NoNewline -ForegroundColor Red
        Write-Host "(HTML file not found)" -ForegroundColor DarkRed
        $FailCount++
        continue
    }
    
    Write-Host "Day ${DayPadded}: " -NoNewline -ForegroundColor Yellow
    
    $success = if ($UsePandoc) {
        Convert-WithPandoc $HtmlFile $MdFile
    }
    else {
        Convert-WithBasicParsing $HtmlFile $MdFile
    }
    
    if ($success -and (Test-Path $MdFile)) {
        Write-Host "✓" -ForegroundColor Green
        $SuccessCount++
    }
    else {
        Write-Host "✗ " -NoNewline -ForegroundColor Red
        Write-Host "(Conversion failed)" -ForegroundColor DarkRed
        $FailCount++
    }
    
    Start-Sleep -Milliseconds 50
}

Write-Host ""
Write-Host "=== Conversion Complete ===" -ForegroundColor Cyan
Write-Host ""
Write-Host "Successfully converted: $SuccessCount/25" -ForegroundColor Green
if ($FailCount -gt 0) {
    Write-Host "Failed: $FailCount/25" -ForegroundColor Red
}
Write-Host ""
Write-Host "Markdown files saved to: $OutputDir" -ForegroundColor Gray
