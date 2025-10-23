# Quick Recent Files Report
param([int]$Hours = 2)

Write-Host "🕒 Quick Activity Report - Last $Hours hours" -ForegroundColor Cyan

# Recently modified files
$cutoff = (Get-Date).AddHours(-$Hours)
Write-Host "`n📁 Recently Modified:" -ForegroundColor Yellow

Get-ChildItem -Recurse -File | 
    Where-Object { 
        $_.LastWriteTime -gt $cutoff -and 
        $_.FullName -notlike "*\target\*" -and
        $_.FullName -notlike "*\.git\*" -and
        $_.FullName -notlike "*\.obsidian\*" -and
        $_.FullName -notlike "*\.smart-env\*"
    } |
    Sort-Object LastWriteTime -Descending |
    Select-Object -First 15 |
    ForEach-Object {
        $relPath = $_.FullName.Replace((Get-Location).Path + "\", "")
        $ago = [math]::Round(((Get-Date) - $_.LastWriteTime).TotalMinutes)
        Write-Host "  [$($ago)m] $relPath" -ForegroundColor White
    }

# Git status
Write-Host "`n🔄 Git Status:" -ForegroundColor Yellow  
git status --porcelain | ForEach-Object {
    Write-Host "  $_" -ForegroundColor Cyan
}

# Recent commits
Write-Host "`n📝 Recent Commits:" -ForegroundColor Yellow
git log --oneline -5 | ForEach-Object {
    Write-Host "  $_" -ForegroundColor Green
}