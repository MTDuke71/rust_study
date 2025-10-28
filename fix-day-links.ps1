# Fix Day links in all markdown files

$files = Get-ChildItem -Path "d:\repos\rust_study" -Include "*.md" -Recurse | Where-Object { $_.FullName -notlike "*\target\*" -and $_.FullName -notlike "*\broken links output.md" }

$totalFiles = 0
$totalReplacements = 0

foreach ($file in $files) {
    $content = Get-Content $file.FullName -Raw
    $originalContent = $content
    
    # Replace [[Day XX - Topic]] with [[daily-study/DayXX]]
    $content = $content -replace '\[\[Day (\d+) - [^\]]+\]\]', '[[daily-study/Day$1]]'
    
    # Replace [[Day XX]] with [[daily-study/DayXX]] (only standalone ones)
    $content = $content -replace '\[\[Day (\d{1,2})\]\](?!\s*-)', '[[daily-study/Day$1]]'
    
    if ($content -ne $originalContent) {
        $content | Set-Content $file.FullName -NoNewline
        $totalFiles++
        
        # Count replacements in this file
        $matches = ([regex]::Matches($originalContent, '\[\[Day \d+[^\]]*\]\]')).Count
        $totalReplacements += $matches
        
        Write-Host "Fixed: $($file.FullName)" -ForegroundColor Green
    }
}

Write-Host "`nSummary:" -ForegroundColor Cyan
Write-Host "Files modified: $totalFiles" -ForegroundColor Yellow
Write-Host "Total links fixed: $totalReplacements" -ForegroundColor Yellow
