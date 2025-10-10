# Weekly Learning Quality Assessment - PowerShell Script
# File: scripts/weekly-quality-review.ps1

param(
    [Parameter(HelpMessage="Generate detailed report")]
    [switch]$Detailed,
    
    [Parameter(HelpMessage="Output directory for reports")]
    [string]$OutputDir = "reports",
    
    [Parameter(HelpMessage="Number of weeks to analyze")]
    [int]$WeeksBack = 1
)

$ErrorActionPreference = "Stop"

function Write-ReviewInfo { param([string]$Message) Write-Host "📈 $Message" -ForegroundColor Cyan }
function Write-ReviewMetric { param([string]$Label, [string]$Value) Write-Host "  $Label`: $Value" -ForegroundColor White }

function Start-WeeklyReview {
    Write-Host @"
📈 WEEKLY LEARNING QUALITY ASSESSMENT
====================================
Analysis Period: Last $WeeksBack week$(if ($WeeksBack -gt 1) { "s" })
Generated: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')
"@ -ForegroundColor Blue

    # Ensure output directory exists
    if (-not (Test-Path $OutputDir)) {
        New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null
    }

    $metrics = @{
        CommitActivity = Get-CommitMetrics
        TestGrowth = Get-TestMetrics
        DocumentationQuality = Get-DocumentationMetrics
        KnowledgeRetention = Get-RetentionMetrics
        CodeQuality = Get-CodeQualityTrends
    }

    Generate-WeeklyReport $metrics
}

function Get-CommitMetrics {
    Write-ReviewInfo "Analyzing commit activity..."
    
    $daysBack = $WeeksBack * 7
    $since = (Get-Date).AddDays(-$daysBack).ToString("yyyy-MM-dd")
    
    try {
        $commits = & git log --oneline --since="$since" 2>$null
        $commitCount = ($commits | Measure-Object).Count
        
        # Analyze commit messages for learning indicators
        $learningCommits = $commits | Where-Object { 
            $_ -match "(REQ-\d+|Day \d+|Mission \d+|implement|test|fix|refactor)" 
        }
        $learningCount = ($learningCommits | Measure-Object).Count
        
        $dailyAverage = [math]::Round($commitCount / $daysBack, 1)
        
        return @{
            TotalCommits = $commitCount
            LearningCommits = $learningCount
            DailyAverage = $dailyAverage
            QualityScore = if ($commitCount -gt 0) { [math]::Round(($learningCount / $commitCount) * 10, 1) } else { 0 }
        }
    }
    catch {
        Write-Warning "Git analysis failed: $_"
        return @{ TotalCommits = 0; LearningCommits = 0; DailyAverage = 0; QualityScore = 0 }
    }
}

function Get-TestMetrics {
    Write-ReviewInfo "Analyzing test coverage growth..."
    
    try {
        # Count test files and lines
        $testFiles = Get-ChildItem -Path . -Recurse -Name "*test*.rs" -ErrorAction SilentlyContinue
        $testCount = ($testFiles | Measure-Object).Count
        
        if ($testCount -gt 0) {
            $testLines = 0
            foreach ($file in $testFiles) {
                $lines = (Get-Content $file -ErrorAction SilentlyContinue | Measure-Object -Line).Lines
                $testLines += $lines
            }
            
            # Run tests to get pass/fail counts
            $testOutput = & cargo test --workspace 2>$null
            if ($LASTEXITCODE -eq 0) {
                $testResults = $testOutput | Select-String "(\d+) passed"
                if ($testResults) {
                    $passedTests = [int]($testResults.Matches[0].Groups[1].Value)
                } else {
                    $passedTests = 0
                }
            } else {
                $passedTests = 0
            }
            
            return @{
                TestFiles = $testCount
                TestLines = $testLines
                PassingTests = $passedTests
                AverageTestsPerFile = if ($testCount -gt 0) { [math]::Round($passedTests / $testCount, 1) } else { 0 }
            }
        } else {
            return @{ TestFiles = 0; TestLines = 0; PassingTests = 0; AverageTestsPerFile = 0 }
        }
    }
    catch {
        Write-Warning "Test analysis failed: $_"
        return @{ TestFiles = 0; TestLines = 0; PassingTests = 0; AverageTestsPerFile = 0 }
    }
}

function Get-DocumentationMetrics {
    Write-ReviewInfo "Analyzing documentation quality..."
    
    try {
        # Generate docs and capture warnings
        $docOutput = & cargo doc --workspace --no-deps 2>&1
        $warningCount = ($docOutput | Select-String "warning:" | Measure-Object).Count
        
        # Count README files
        $readmeFiles = Get-ChildItem -Path . -Recurse -Name "README*.md" -ErrorAction SilentlyContinue
        $readmeCount = ($readmeFiles | Measure-Object).Count
        
        # Count documented functions (approximation)
        $rustFiles = Get-ChildItem -Path . -Recurse -Name "*.rs" -ErrorAction SilentlyContinue
        $docComments = 0
        foreach ($file in $rustFiles) {
            $content = Get-Content $file -ErrorAction SilentlyContinue
            $docLines = ($content | Where-Object { $_ -match "^\s*///|^\s*//!" } | Measure-Object).Count
            $docComments += $docLines
        }
        
        return @{
            DocumentationWarnings = $warningCount
            ReadmeFiles = $readmeCount
            DocumentationLines = $docComments
            QualityScore = if ($warningCount -eq 0) { 10 } else { [math]::Max(0, 10 - $warningCount) }
        }
    }
    catch {
        Write-Warning "Documentation analysis failed: $_"
        return @{ DocumentationWarnings = 0; ReadmeFiles = 0; DocumentationLines = 0; QualityScore = 0 }
    }
}

function Get-RetentionMetrics {
    Write-ReviewInfo "Analyzing knowledge retention..."
    
    $retentionFile = "zettelkasten/spaced-repetition-cards.md"
    
    if (Test-Path $retentionFile) {
        try {
            $content = Get-Content $retentionFile
            
            # Count cards and success indicators
            $totalCards = ($content | Select-String "## [A-Z]+-\d+:" | Measure-Object).Count
            $successMarkers = ($content | Select-String "✅" | Measure-Object).Count
            $dueToday = ($content | Select-String "due today" -CaseSensitive:$false | Measure-Object).Count
            
            $successRate = if ($totalCards -gt 0) { [math]::Round(($successMarkers / $totalCards) * 100, 1) } else { 0 }
            
            return @{
                TotalCards = $totalCards
                SuccessfulReviews = $successMarkers
                DueToday = $dueToday
                SuccessRate = $successRate
            }
        }
        catch {
            Write-Warning "Retention analysis failed: $_"
            return @{ TotalCards = 0; SuccessfulReviews = 0; DueToday = 0; SuccessRate = 0 }
        }
    } else {
        return @{ TotalCards = 0; SuccessfulReviews = 0; DueToday = 0; SuccessRate = 0 }
    }
}

function Get-CodeQualityTrends {
    Write-ReviewInfo "Analyzing code quality trends..."
    
    try {
        # Run clippy for current quality
        $clippyOutput = & cargo clippy --workspace 2>&1
        $clippyIssues = ($clippyOutput | Select-String "warning:" | Measure-Object).Count
        
        # Count Rust files and estimate complexity
        $rustFiles = Get-ChildItem -Path . -Recurse -Name "*.rs" -ErrorAction SilentlyContinue
        $totalLines = 0
        $functionCount = 0
        
        foreach ($file in $rustFiles) {
            $content = Get-Content $file -ErrorAction SilentlyContinue
            $totalLines += ($content | Measure-Object -Line).Lines
            $functionCount += ($content | Select-String "fn\s+\w+" | Measure-Object).Count
        }
        
        $averageFunctionSize = if ($functionCount -gt 0) { [math]::Round($totalLines / $functionCount, 1) } else { 0 }
        
        return @{
            ClippyIssues = $clippyIssues
            TotalLines = $totalLines
            FunctionCount = $functionCount
            AverageFunctionSize = $averageFunctionSize
            QualityScore = [math]::Max(0, 10 - $clippyIssues)
        }
    }
    catch {
        Write-Warning "Code quality analysis failed: $_"
        return @{ ClippyIssues = 0; TotalLines = 0; FunctionCount = 0; AverageFunctionSize = 0; QualityScore = 0 }
    }
}

function Generate-WeeklyReport {
    param($Metrics)
    
    $report = @"
📊 WEEKLY LEARNING QUALITY REPORT
=================================
Period: Last $WeeksBack week$(if ($WeeksBack -gt 1) { "s" })
Generated: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')

📝 COMMIT ACTIVITY:
  Total Commits: $($Metrics.CommitActivity.TotalCommits)
  Learning-Related: $($Metrics.CommitActivity.LearningCommits)
  Daily Average: $($Metrics.CommitActivity.DailyAverage)
  Quality Score: $($Metrics.CommitActivity.QualityScore)/10

🧪 TEST COVERAGE GROWTH:
  Test Files: $($Metrics.TestGrowth.TestFiles)
  Test Lines: $($Metrics.TestGrowth.TestLines)
  Passing Tests: $($Metrics.TestGrowth.PassingTests)
  Avg Tests/File: $($Metrics.TestGrowth.AverageTestsPerFile)

📚 DOCUMENTATION QUALITY:
  Documentation Lines: $($Metrics.DocumentationQuality.DocumentationLines)
  README Files: $($Metrics.DocumentationQuality.ReadmeFiles)
  Doc Warnings: $($Metrics.DocumentationQuality.DocumentationWarnings)
  Quality Score: $($Metrics.DocumentationQuality.QualityScore)/10

🧠 KNOWLEDGE RETENTION:
  Total Cards: $($Metrics.KnowledgeRetention.TotalCards)
  Successful Reviews: $($Metrics.KnowledgeRetention.SuccessfulReviews)
  Due Today: $($Metrics.KnowledgeRetention.DueToday)
  Success Rate: $($Metrics.KnowledgeRetention.SuccessRate)%

📈 CODE QUALITY TRENDS:
  Clippy Issues: $($Metrics.CodeQuality.ClippyIssues)
  Total Lines: $($Metrics.CodeQuality.TotalLines)
  Functions: $($Metrics.CodeQuality.FunctionCount)
  Avg Function Size: $($Metrics.CodeQuality.AverageFunctionSize) lines
  Quality Score: $($Metrics.CodeQuality.QualityScore)/10

🎯 OVERALL ASSESSMENT:
"@

    # Calculate overall quality score
    $overallScore = [math]::Round((
        $Metrics.CommitActivity.QualityScore + 
        $Metrics.DocumentationQuality.QualityScore + 
        $Metrics.CodeQuality.QualityScore +
        ($Metrics.KnowledgeRetention.SuccessRate / 10)
    ) / 4, 1)
    
    $report += "`n  Overall Quality Score: $overallScore/10"
    
    if ($overallScore -ge 8) {
        $report += "`n  Status: ✅ EXCELLENT - Keep up the great work!"
    } elseif ($overallScore -ge 6) {
        $report += "`n  Status: 🟡 GOOD - Some areas for improvement"
    } else {
        $report += "`n  Status: 🔴 NEEDS ATTENTION - Focus on quality improvements"
    }
    
    Write-Host $report -ForegroundColor $(
        if ($overallScore -ge 8) { "Green" } 
        elseif ($overallScore -ge 6) { "Yellow" } 
        else { "Red" }
    )
    
    # Save to file
    $reportFile = Join-Path $OutputDir "weekly-quality-$(Get-Date -Format 'yyyy-MM-dd').txt"
    $report | Out-File -FilePath $reportFile -Encoding UTF8
    Write-ReviewInfo "Report saved to $reportFile"
    
    # Generate CSV for trend analysis if detailed
    if ($Detailed) {
        Generate-TrendCSV $Metrics $OutputDir
    }
}

function Generate-TrendCSV {
    param($Metrics, $OutputDir)
    
    $csvFile = Join-Path $OutputDir "quality-trends.csv"
    $csvData = @(
        "Date,Commits,LearningCommits,TestFiles,PassingTests,ClippyIssues,DocLines,SuccessRate,OverallScore"
        "$(Get-Date -Format 'yyyy-MM-dd'),$($Metrics.CommitActivity.TotalCommits),$($Metrics.CommitActivity.LearningCommits),$($Metrics.TestGrowth.TestFiles),$($Metrics.TestGrowth.PassingTests),$($Metrics.CodeQuality.ClippyIssues),$($Metrics.DocumentationQuality.DocumentationLines),$($Metrics.KnowledgeRetention.SuccessRate),$(($Metrics.CommitActivity.QualityScore + $Metrics.DocumentationQuality.QualityScore + $Metrics.CodeQuality.QualityScore) / 3)"
    )
    
    # Append to existing file or create new
    if (Test-Path $csvFile) {
        $csvData[1] | Add-Content $csvFile
    } else {
        $csvData | Out-File $csvFile -Encoding UTF8
    }
    
    Write-ReviewInfo "Trend data appended to $csvFile"
}

# Main execution
if ($MyInvocation.InvocationName -eq $MyInvocation.MyCommand.Name) {
    Start-WeeklyReview
}