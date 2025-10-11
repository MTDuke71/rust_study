# Quality Assurance Pipeline - PowerShell Scripts
# Daily automated code quality pipeline for Windows/PowerShell environment

# File: scripts/quality-pipeline.ps1
param(
    [Parameter(HelpMessage="Run quick checks only (skip coverage)")]
    [switch]$Quick,
    
    [Parameter(HelpMessage="Skip interactive prompts")]
    [switch]$NonInteractive,
    
    [Parameter(HelpMessage="Output results to file")]
    [string]$OutputFile,
    
    [Parameter(HelpMessage="Fail fast on first error")]
    [switch]$FailFast
)

# Quality Pipeline Configuration
$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"

# Color output functions
function Write-QAInfo { param([string]$Message) Write-Host "🔍 $Message" -ForegroundColor Cyan }
function Write-QASuccess { param([string]$Message) Write-Host "✅ $Message" -ForegroundColor Green }
function Write-QAWarning { param([string]$Message) Write-Host "⚠️ $Message" -ForegroundColor Yellow }
function Write-QAError { param([string]$Message) Write-Host "❌ $Message" -ForegroundColor Red }

# Quality metrics tracking
$QualityReport = @{
    StartTime = Get-Date
    CompilationStatus = $null
    ClippyIssues = 0
    TestResults = @{ Passed = 0; Failed = 0; Total = 0 }
    Coverage = $null
    DocumentationWarnings = 0
    Formatting = $null
}

function Start-QualityPipeline {
    Write-Host @"
🎯 RUST QUALITY ASSURANCE PIPELINE
==================================
Environment: Windows PowerShell
Date: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')
Working Directory: $PWD
"@ -ForegroundColor Blue

    # Check prerequisites
    Test-Prerequisites
    
    # Run quality checks
    $checks = @(
        @{ Name = "Compilation Check"; Function = "Test-Compilation" },
        @{ Name = "Code Formatting"; Function = "Format-Code" },
        @{ Name = "Clippy Analysis"; Function = "Test-Clippy" },
        @{ Name = "Test Suite"; Function = "Test-Suite" },
        @{ Name = "Documentation"; Function = "Generate-Documentation" }
    )
    
    if (-not $Quick) {
        $checks += @{ Name = "Coverage Analysis"; Function = "Test-Coverage" }
    }
    
    foreach ($check in $checks) {
        Write-QAInfo "Running $($check.Name)..."
        try {
            & $check.Function
            Write-QASuccess "$($check.Name) completed successfully"
        }
        catch {
            Write-QAError "$($check.Name) failed: $_"
            if ($FailFast) { 
                Generate-QualityReport
                exit 1 
            }
        }
    }
    
    Generate-QualityReport
}

function Test-Prerequisites {
    Write-QAInfo "Checking prerequisites..."
    
    # Check Rust toolchain
    if (-not (Get-Command "cargo" -ErrorAction SilentlyContinue)) {
        throw "Cargo not found. Please install Rust toolchain."
    }
    
    # Check workspace structure
    if (-not (Test-Path "Cargo.toml")) {
        throw "Cargo.toml not found. Please run from workspace root."
    }
    
    # Verify git repository
    if (-not (Test-Path ".git")) {
        Write-QAWarning "Not in a git repository - some features may be limited"
    }
    
    Write-QASuccess "Prerequisites verified"
}

function Test-Compilation {
    Write-QAInfo "📦 Checking compilation..."
    
    $result = & cargo check --workspace --message-format=json 2>&1
    $exitCode = $LASTEXITCODE
    
    if ($exitCode -eq 0) {
        $QualityReport.CompilationStatus = "✅ Clean"
        Write-QASuccess "Compilation successful - no errors"
    } else {
        $QualityReport.CompilationStatus = "❌ Failed"
        Write-Host $result -ForegroundColor Red
        throw "Compilation failed with exit code $exitCode"
    }
}

function Format-Code {
    Write-QAInfo "🎨 Applying consistent formatting..."
    
    # Check if formatting is needed
    $formatCheck = & cargo fmt --all -- --check 2>&1
    $needsFormatting = $LASTEXITCODE -ne 0
    
    if ($needsFormatting) {
        Write-QAInfo "Code formatting required - applying changes..."
        & cargo fmt --all
        
        if ($LASTEXITCODE -eq 0) {
            $QualityReport.Formatting = "✅ Applied"
            Write-QASuccess "Code formatting applied successfully"
        } else {
            $QualityReport.Formatting = "❌ Failed"
            throw "Code formatting failed"
        }
    } else {
        $QualityReport.Formatting = "✅ Already formatted"
        Write-QASuccess "Code already properly formatted"
    }
}

function Test-Clippy {
    Write-QAInfo "📋 Running Clippy analysis..."
    
    # Run clippy with warnings as errors
    $clippyOutput = & cargo clippy --workspace -- -D warnings 2>&1
    $exitCode = $LASTEXITCODE
    
    if ($exitCode -eq 0) {
        $QualityReport.ClippyIssues = 0
        Write-QASuccess "Clippy analysis passed - no issues found"
    } else {
        # Count issues in output
        $issueCount = ($clippyOutput | Select-String "warning:" | Measure-Object).Count
        $QualityReport.ClippyIssues = $issueCount
        
        Write-Host $clippyOutput -ForegroundColor Yellow
        throw "Clippy found $issueCount issues"
    }
}

function Test-Suite {
    Write-QAInfo "🧪 Running test suite..."
    
    # Run tests with JSON output for parsing
    $testOutput = & cargo test --workspace --message-format=json 2>&1
    $exitCode = $LASTEXITCODE
    
    # Parse test results (simplified - could be more robust)
    $testResults = $testOutput | Where-Object { $_ -match '"type":"test"' } | ConvertFrom-Json
    $passed = ($testResults | Where-Object { $_.event -eq "ok" } | Measure-Object).Count
    $failed = ($testResults | Where-Object { $_.event -eq "failed" } | Measure-Object).Count
    
    $QualityReport.TestResults = @{
        Passed = $passed
        Failed = $failed  
        Total = $passed + $failed
    }
    
    if ($exitCode -eq 0) {
        Write-QASuccess "All $($passed) tests passed"
    } else {
        Write-Host $testOutput -ForegroundColor Red
        throw "$failed tests failed out of $($passed + $failed) total"
    }
}

function Generate-Documentation {
    Write-QAInfo "📚 Generating documentation..."
    
    $docOutput = & cargo doc --workspace --no-deps 2>&1
    $exitCode = $LASTEXITCODE
    
    # Count documentation warnings
    $warningCount = ($docOutput | Select-String "warning:" | Measure-Object).Count
    $QualityReport.DocumentationWarnings = $warningCount
    
    if ($exitCode -eq 0) {
        if ($warningCount -eq 0) {
            Write-QASuccess "Documentation generated without warnings"
        } else {
            Write-QAWarning "Documentation generated with $warningCount warnings"
            $docOutput | Select-String "warning:" | ForEach-Object { Write-Host $_ -ForegroundColor Yellow }
        }
    } else {
        throw "Documentation generation failed"
    }
}

function Test-Coverage {
    Write-QAInfo "📊 Calculating test coverage..."
    
    # Check if cargo-tarpaulin is installed
    if (-not (Get-Command "cargo-tarpaulin" -ErrorAction SilentlyContinue)) {
        Write-QAWarning "cargo-tarpaulin not installed. Run: cargo install cargo-tarpaulin"
        $QualityReport.Coverage = "Not Available"
        return
    }
    
    try {
        $coverageOutput = & cargo tarpaulin --workspace --timeout 120 --out Json 2>&1
        $exitCode = $LASTEXITCODE
        
        if ($exitCode -eq 0) {
            # Parse coverage percentage (simplified)
            $coverageJson = $coverageOutput | Where-Object { $_ -match '"coverage":' } | ConvertFrom-Json
            $coveragePercent = [math]::Round($coverageJson.coverage, 2)
            
            $QualityReport.Coverage = "$coveragePercent%"
            
            if ($coveragePercent -ge 85) {
                Write-QASuccess "Test coverage: $coveragePercent% (target: ≥85%)"
            } else {
                Write-QAWarning "Test coverage: $coveragePercent% (below target of 85%)"
            }
        } else {
            throw "Coverage analysis failed"
        }
    }
    catch {
        Write-QAWarning "Coverage analysis failed: $_"
        $QualityReport.Coverage = "Failed"
    }
}

function Generate-QualityReport {
    $endTime = Get-Date
    $duration = $endTime - $QualityReport.StartTime
    
    $report = @"

🎯 QUALITY ASSURANCE REPORT
===========================
Timestamp: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')
Duration: $($duration.ToString('mm\:ss'))

📊 CODE QUALITY RESULTS:
  Compilation: $($QualityReport.CompilationStatus)
  Formatting: $($QualityReport.Formatting)  
  Clippy Issues: $($QualityReport.ClippyIssues)
  Tests: $($QualityReport.TestResults.Passed)/$($QualityReport.TestResults.Total) passed
  Coverage: $($QualityReport.Coverage)
  Doc Warnings: $($QualityReport.DocumentationWarnings)

🎉 OVERALL STATUS: $(if ($QualityReport.TestResults.Failed -eq 0 -and $QualityReport.ClippyIssues -eq 0) { "✅ PASSED" } else { "❌ ISSUES FOUND" })

"@
    
    Write-Host $report -ForegroundColor $(if ($QualityReport.TestResults.Failed -eq 0 -and $QualityReport.ClippyIssues -eq 0) { "Green" } else { "Yellow" })
    
    # Output to file if requested
    if ($OutputFile) {
        $report | Out-File -FilePath $OutputFile -Encoding UTF8
        Write-QAInfo "Report saved to $OutputFile"
    }
    
    # Return exit code based on results
    if ($QualityReport.TestResults.Failed -gt 0 -or $QualityReport.ClippyIssues -gt 0) {
        exit 1
    } else {
        exit 0
    }
}

# Main execution
if ($MyInvocation.InvocationName -ne '.') {
    Start-QualityPipeline
}
