# Recent Activity Report for Rust Study Repository
# Shows recently created/modified files with multiple perspectives

param(
    [int]$Hours = 24,
    [int]$MaxFiles = 50,
    [switch]$IncludeGitIgnored
)

Write-Host "🕒 Recent Activity Report - Last $Hours hours" -ForegroundColor Cyan
Write-Host "=" * 60

# 1. Git commits in timeframe
Write-Host "`n📝 Recent Git Commits:" -ForegroundColor Yellow
$sinceTime = (Get-Date).AddHours(-$Hours).ToString("yyyy-MM-dd HH:mm:ss")
git log --since="$sinceTime" --oneline --decorate | ForEach-Object {
    Write-Host "  $($_)" -ForegroundColor Green
}

# 2. Files changed in recent commits (last 10 commits)
Write-Host "`n📋 Files from Recent Commits:" -ForegroundColor Yellow
$recentFiles = git log --name-only --oneline -10 | Where-Object { $_ -and $_ -notmatch "^[a-f0-9]+\s" }
$recentFiles | Group-Object | Sort-Object Count -Descending | Select-Object -First 20 | ForEach-Object {
    $count = $_.Count
    $file = $_.Name
    Write-Host "  [$count] $file" -ForegroundColor White
}

# 3. Currently modified files (working directory)
Write-Host "`n🔄 Currently Modified Files:" -ForegroundColor Yellow
git status --porcelain | ForEach-Object {
    $status = $_.Substring(0, 2)
    $file = $_.Substring(3)
    $color = switch ($status.Trim()) {
        "M" { "Yellow" }
        "A" { "Green" }
        "D" { "Red" }
        "??" { "Magenta" }
        default { "White" }
    }
    Write-Host "  $status $file" -ForegroundColor $color
}

# 4. File system recent activity
Write-Host "`n📁 Recently Modified Files (File System):" -ForegroundColor Yellow
$cutoffTime = (Get-Date).AddHours(-$Hours)

$params = @{
    Recurse = $true
    File = $true
}

if (-not $IncludeGitIgnored) {
    # Exclude common git-ignored patterns
    $excludePatterns = @(
        "target",
        "node_modules", 
        ".git",
        ".obsidian",
        ".smart-env",
        "*.tmp",
        "*.temp"
    )
}

Get-ChildItem @params | 
    Where-Object { 
        $_.LastWriteTime -gt $cutoffTime -and
        (-not $excludePatterns -or ($excludePatterns | ForEach-Object { $_.FullName -notlike "*$_*" }))
    } |
    Sort-Object LastWriteTime -Descending |
    Select-Object -First $MaxFiles |
    ForEach-Object {
        $relPath = $_.FullName.Replace((Get-Location).Path + "\", "")
        $timeAgo = [math]::Round(((Get-Date) - $_.LastWriteTime).TotalMinutes)
        Write-Host "  [$($timeAgo)m ago] $relPath" -ForegroundColor Cyan
    }

# 5. File type analysis of recent changes
Write-Host "`n📊 File Types Recently Changed:" -ForegroundColor Yellow
$recentFiles | ForEach-Object {
    [System.IO.Path]::GetExtension($_)
} | Where-Object { $_ } | Group-Object | Sort-Object Count -Descending | ForEach-Object {
    Write-Host "  $($_.Name): $($_.Count) files" -ForegroundColor White
}

# 6. Directory activity summary  
Write-Host "`n📂 Directory Activity Summary:" -ForegroundColor Yellow
$recentFiles | ForEach-Object {
    $dir = [System.IO.Path]::GetDirectoryName($_)
    if ($dir) { $dir.Split('\')[0] } else { "root" }
} | Group-Object | Sort-Object Count -Descending | Select-Object -First 10 | ForEach-Object {
    Write-Host "  $($_.Name): $($_.Count) changes" -ForegroundColor White
}

Write-Host "`n" + "=" * 60
Write-Host "Report generated at $(Get-Date)" -ForegroundColor Gray