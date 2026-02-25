# Download Advent of Code 2022 Problem Statements and Inputs
# Usage: .\scripts\download_aoc2022_problems.ps1 -SessionCookie "your_session_cookie_here"

param(
    [Parameter(Mandatory=$false)]
    [string]$SessionCookie = $env:AOC_SESSION,
    
    [Parameter(Mandatory=$false)]
    [int]$StartDay = 1,
    
    [Parameter(Mandatory=$false)]
    [int]$EndDay = 25,
    
    [Parameter(Mandatory=$false)]
    [string]$Year = "2022",
    
    [Parameter(Mandatory=$false)]
    [switch]$ProblemsOnly,
    
    [Parameter(Mandatory=$false)]
    [switch]$InputsOnly
)

# Validate session cookie
if ([string]::IsNullOrWhiteSpace($SessionCookie)) {
    Write-Host "ERROR: Session cookie required!" -ForegroundColor Red
    Write-Host ""
    Write-Host "Option 1: Pass as parameter:"
    Write-Host '  .\scripts\download_aoc2022_problems.ps1 -SessionCookie "your_cookie_here"'
    Write-Host ""
    Write-Host "Option 2: Set environment variable:"
    Write-Host '  $env:AOC_SESSION = "your_cookie_here"'
    Write-Host '  .\scripts\download_aoc2022_problems.ps1'
    Write-Host ""
    Write-Host "To get your session cookie:"
    Write-Host "  1. Login to adventofcode.com"
    Write-Host "  2. Open Developer Tools (F12)"
    Write-Host "  3. Go to Application/Storage tab"
    Write-Host "  4. Find Cookies → https://adventofcode.com"
    Write-Host "  5. Copy the 'session' cookie value"
    exit 1
}

# Setup directories
$BaseDir = "advent_of_code\aoc$Year"
$ProblemsDir = "$BaseDir\Problem_Statements\days"
$InputsDir = "$BaseDir\inputs"

Write-Host "=== Advent of Code $Year Downloader ===" -ForegroundColor Cyan
Write-Host ""

# Create directories if they don't exist
if (-not $InputsOnly) {
    New-Item -ItemType Directory -Force -Path $ProblemsDir | Out-Null
    Write-Host "Problems directory: $ProblemsDir" -ForegroundColor Gray
}

if (-not $ProblemsOnly) {
    New-Item -ItemType Directory -Force -Path $InputsDir | Out-Null
    Write-Host "Inputs directory: $InputsDir" -ForegroundColor Gray
}

Write-Host ""

# Setup HTTP headers
$Headers = @{
    "Cookie" = "session=$SessionCookie"
}
$UserAgent = "rust_study/1.0"

# Download loop
for ($Day = $StartDay; $Day -le $EndDay; $Day++) {
    $DayPadded = "{0:D2}" -f $Day
    
    Write-Host "Day ${DayPadded}: " -NoNewline -ForegroundColor Yellow
    
    # Download problem statement
    if (-not $InputsOnly) {
        $ProblemUrl = "https://adventofcode.com/$Year/day/$Day"
        $ProblemFile = "$ProblemsDir\day$DayPadded.html"
        
        Write-Host "Problem..." -NoNewline
        
        try {
            Invoke-WebRequest -Uri $ProblemUrl -Headers $Headers -UserAgent $UserAgent -OutFile $ProblemFile -ErrorAction Stop
            Write-Host "✓ " -NoNewline -ForegroundColor Green
        }
        catch {
            Write-Host "✗ " -NoNewline -ForegroundColor Red
            Write-Host "($($_.Exception.Message)) " -NoNewline -ForegroundColor DarkRed
        }
        
        Start-Sleep -Milliseconds 300
    }
    
    # Download puzzle input
    if (-not $ProblemsOnly) {
        $InputUrl = "https://adventofcode.com/$Year/day/$Day/input"
        $InputFile = "$InputsDir\day$DayPadded.txt"
        
        Write-Host "Input..." -NoNewline
        
        try {
            Invoke-WebRequest -Uri $InputUrl -Headers $Headers -UserAgent $UserAgent -OutFile $InputFile -ErrorAction Stop
            Write-Host "✓" -ForegroundColor Green
        }
        catch {
            Write-Host "✗ ($($_.Exception.Message))" -ForegroundColor Red
        }
        
        Start-Sleep -Milliseconds 300
    }
    else {
        Write-Host ""  # Newline if only downloading problems
    }
}

Write-Host ""
Write-Host "=== Download Complete ===" -ForegroundColor Cyan
Write-Host ""

# Summary
if (-not $InputsOnly) {
    $ProblemCount = (Get-ChildItem "$ProblemsDir\*.html" -ErrorAction SilentlyContinue).Count
    Write-Host "Problems downloaded: $ProblemCount/25" -ForegroundColor Gray
}

if (-not $ProblemsOnly) {
    $InputCount = (Get-ChildItem "$InputsDir\*.txt" -ErrorAction SilentlyContinue).Count
    Write-Host "Inputs downloaded: $InputCount/25" -ForegroundColor Gray
}

Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "  1. Extract example inputs from problem statements"
Write-Host "  2. Save as inputs/dayXX_example.txt"
Write-Host "  3. Start solving!"
