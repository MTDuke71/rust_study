# Link Validation Script for zettel-index.md
# Checks all [[wiki-links]] and verifies target files exist

$zettelIndexPath = "$PSScriptRoot\..\zettelkasten\zettel-index.md"
$zettelDir = "$PSScriptRoot\..\zettelkasten"
$workspaceRoot = "$PSScriptRoot\.."

Write-Host "🔍 Validating links in zettel-index.md..." -ForegroundColor Cyan
Write-Host ""

# Read the zettel-index file
$content = Get-Content $zettelIndexPath -Raw

# Extract all [[wiki-links]]
$linkPattern = '\[\[([^\]]+)\]\]'
$matches = [regex]::Matches($content, $linkPattern)

$totalLinks = 0
$validLinks = 0
$brokenLinks = @()

foreach ($match in $matches) {
    $totalLinks++
    $linkText = $match.Groups[1].Value
    
    # Handle links with display text: [[file|display text]]
    if ($linkText -match '\|') {
        $linkText = $linkText -split '\|' | Select-Object -First 1
    }
    
    # Trim whitespace
    $linkText = $linkText.Trim()
    
    # Determine search locations
    $possiblePaths = @()
    
    # Check if link has explicit path (../ or similar)
    if ($linkText -match '^\.\.\/') {
        # Relative path from zettelkasten directory
        $relativePath = $linkText -replace '^\.\./', ''
        $possiblePaths += Join-Path $workspaceRoot $relativePath
        $possiblePaths += Join-Path $workspaceRoot "$relativePath.md"
    } else {
        # Check in zettelkasten directory
        $possiblePaths += Join-Path $zettelDir "$linkText.md"
        $possiblePaths += Join-Path $zettelDir $linkText
        
        # Check in rust_book
        $possiblePaths += Join-Path $workspaceRoot "rust_book\$linkText.md"
        $possiblePaths += Join-Path $workspaceRoot "rust_book\$linkText\README.md"
    }
    
    # Check if any of the possible paths exist
    $found = $false
    $foundPath = ""
    foreach ($path in $possiblePaths) {
        if (Test-Path $path) {
            $found = $true
            $foundPath = $path
            break
        }
    }
    
    if ($found) {
        $validLinks++
        Write-Host "  ✅ $linkText" -ForegroundColor Green
    } else {
        $brokenLinks += [PSCustomObject]@{
            LinkText = $linkText
            SearchedPaths = $possiblePaths -join "`n    "
        }
        Write-Host "  ❌ $linkText" -ForegroundColor Red
    }
}

Write-Host ""
Write-Host "═══════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "📊 Summary:" -ForegroundColor Yellow
Write-Host "═══════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  Total Links:  $totalLinks"
Write-Host "  Valid Links:  $validLinks" -ForegroundColor Green
Write-Host "  Broken Links: $($brokenLinks.Count)" -ForegroundColor Red
Write-Host "  Success Rate: $([math]::Round(($validLinks / $totalLinks) * 100, 2))%"
Write-Host ""

if ($brokenLinks.Count -gt 0) {
    Write-Host "❌ Broken Links Details:" -ForegroundColor Red
    Write-Host "═══════════════════════════════════════════════" -ForegroundColor Cyan
    foreach ($broken in $brokenLinks) {
        Write-Host ""
        Write-Host "  Link: $($broken.LinkText)" -ForegroundColor Yellow
        Write-Host "  Searched in:"
        Write-Host "    $($broken.SearchedPaths)" -ForegroundColor DarkGray
    }
    Write-Host ""
    Write-Host "💡 Tip: Create missing files or update links in zettel-index.md" -ForegroundColor Cyan
} else {
    Write-Host "🎉 All links are valid!" -ForegroundColor Green
}

Write-Host ""
