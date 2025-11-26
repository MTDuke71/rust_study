# Quality Assurance Pipeline - PowerShell Scripts
# Daily automated code quality pipeline for Windows/PowerShell environment

# File: scripts/quality-pipeline.ps1
param(
    [Parameter(HelpMessage="Run quick checks only (skip coverage and heavy analysis)")]
    [switch]$Quick,
    
    [Parameter(HelpMessage="Skip interactive prompts")]
    [switch]$NonInteractive,
    
    [Parameter(HelpMessage="Output results to file")]
    [string]$OutputFile,
    
    [Parameter(HelpMessage="Fail fast on first error")]
    [switch]$FailFast,
    
    [Parameter(HelpMessage="Run missions-only coverage (default: workspace)")]
    [switch]$MissionsOnly,
    
    [Parameter(HelpMessage="Skip security audit check")]
    [switch]$SkipSecurityAudit,
    
    [Parameter(HelpMessage="Skip dependency checks")]
    [switch]$SkipDependencyChecks,
    
    [Parameter(HelpMessage="Show interactive menu to select which checks to run")]
    [switch]$Menu,
    
    [Parameter(HelpMessage="Run specific check(s) by name. Valid values: Compilation, Formatting, Clippy, Tests, Documentation, Coverage, Security, Outdated, Unused, DeadCode")]
    [ValidateSet("Compilation", "Formatting", "Clippy", "Tests", "Documentation", "Coverage", "Security", "Outdated", "Unused", "DeadCode", "All")]
    [string[]]$Run
)

# Quality Pipeline Configuration
$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"

# Create reports directory if it doesn't exist
$ReportsDir = "reports"
if (-not (Test-Path $ReportsDir)) {
    New-Item -ItemType Directory -Path $ReportsDir | Out-Null
}

# Generate timestamp for file naming
$Timestamp = Get-Date -Format "yyyyMMdd_HHmmss"

# Color output functions
function Write-QAInfo { param([string]$Message) Write-Host "🔍 $Message" -ForegroundColor Cyan }
function Write-QASuccess { param([string]$Message) Write-Host "✅ $Message" -ForegroundColor Green }
function Write-QAWarning { param([string]$Message) Write-Host "⚠️ $Message" -ForegroundColor Yellow }
function Write-QAError { param([string]$Message) Write-Host "❌ $Message" -ForegroundColor Red }

# Check definition mapping
$script:CheckDefinitions = @{
    "Compilation"   = @{ Name = "Compilation Check"; Function = "Test-Compilation"; Quick = $true }
    "Formatting"    = @{ Name = "Code Formatting"; Function = "Format-Code"; Quick = $true }
    "Clippy"        = @{ Name = "Clippy Analysis"; Function = "Test-Clippy"; Quick = $true }
    "Tests"         = @{ Name = "Test Suite"; Function = "Test-Suite"; Quick = $true }
    "Documentation" = @{ Name = "Documentation"; Function = "Generate-Documentation"; Quick = $true }
    "Coverage"      = @{ Name = "Coverage Analysis"; Function = "Test-Coverage"; Quick = $false }
    "Security"      = @{ Name = "Security Audit"; Function = "Test-SecurityAudit"; Quick = $false }
    "Outdated"      = @{ Name = "Outdated Dependencies"; Function = "Test-OutdatedDependencies"; Quick = $false }
    "Unused"        = @{ Name = "Unused Dependencies"; Function = "Test-UnusedDependencies"; Quick = $false }
    "DeadCode"      = @{ Name = "Dead Code Detection"; Function = "Test-DeadCode"; Quick = $false }
}

# Interactive menu function
function Show-QAMenu {
    $selectedChecks = @()
    
    Write-Host @"

🎯 QUALITY ASSURANCE PIPELINE - INTERACTIVE MENU
================================================

Select which checks to run:

  QUICK CHECKS (included in -Quick mode):
    [1] Compilation    - Verify workspace compiles without errors
    [2] Formatting     - Check/apply code formatting (cargo fmt)
    [3] Clippy         - Run Clippy linter with warnings as errors
    [4] Tests          - Run full test suite
    [5] Documentation  - Generate and check documentation

  EXTENDED CHECKS (skipped in -Quick mode):
    [6] Coverage       - Calculate test coverage (requires cargo-tarpaulin)
    [7] Security       - Security vulnerability audit (requires cargo-audit)
    [8] Outdated       - Check for outdated dependencies (requires cargo-outdated)
    [9] Unused         - Find unused dependencies (requires cargo-udeps + nightly)
    [0] DeadCode       - Detect dead/unused code

  PRESETS:
    [A] All Checks     - Run everything
    [Q] Quick Checks   - Run checks 1-5 only
    [E] Extended Only  - Run checks 6-0 only

  [X] Exit without running

"@ -ForegroundColor Blue

    $validInput = $false
    while (-not $validInput) {
        $input = Read-Host "Enter your selection (e.g., '1,2,3' or 'A' for all)"
        $input = $input.Trim().ToUpper()
        
        if ($input -eq 'X') {
            Write-Host "Exiting without running checks." -ForegroundColor Yellow
            return @()
        }
        
        if ($input -eq 'A') {
            $selectedChecks = @("Compilation", "Formatting", "Clippy", "Tests", "Documentation", "Coverage", "Security", "Outdated", "Unused", "DeadCode")
            $validInput = $true
        }
        elseif ($input -eq 'Q') {
            $selectedChecks = @("Compilation", "Formatting", "Clippy", "Tests", "Documentation")
            $validInput = $true
        }
        elseif ($input -eq 'E') {
            $selectedChecks = @("Coverage", "Security", "Outdated", "Unused", "DeadCode")
            $validInput = $true
        }
        else {
            # Parse comma-separated numbers
            $numbers = $input -split '[,\s]+' | Where-Object { $_ -match '^\d$' }
            
            $numberMapping = @{
                '1' = "Compilation"
                '2' = "Formatting"
                '3' = "Clippy"
                '4' = "Tests"
                '5' = "Documentation"
                '6' = "Coverage"
                '7' = "Security"
                '8' = "Outdated"
                '9' = "Unused"
                '0' = "DeadCode"
            }
            
            foreach ($num in $numbers) {
                if ($numberMapping.ContainsKey($num)) {
                    $selectedChecks += $numberMapping[$num]
                }
            }
            
            if ($selectedChecks.Count -gt 0) {
                $validInput = $true
            } else {
                Write-Host "Invalid selection. Please try again." -ForegroundColor Red
            }
        }
    }
    
    # Remove duplicates while preserving order
    $selectedChecks = $selectedChecks | Select-Object -Unique
    
    Write-Host "`nSelected checks: $($selectedChecks -join ', ')" -ForegroundColor Green
    return $selectedChecks
}

# Function to save check results to individual files
function Save-CheckResult {
    param(
        [string]$CheckName,
        [string]$Output,
        [int]$ExitCode,
        [hashtable]$AdditionalData = @{}
    )
    
    $safeName = $CheckName -replace '[^\w\-]', '_'
    $fileName = "$ReportsDir\${safeName}_$Timestamp.txt"
    
    $report = @"
$CheckName Report
================
Timestamp: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')
Exit Code: $ExitCode
Status: $(if ($ExitCode -eq 0) { "SUCCESS" } else { "FAILED" })

$Output

"@
    
    # Add additional structured data if provided
    if ($AdditionalData.Count -gt 0) {
        $report += "`nADDITIONAL DATA:`n"
        $report += ($AdditionalData.GetEnumerator() | ForEach-Object { "$($_.Key): $($_.Value)" }) -join "`n"
        $report += "`n"
    }
    
    $report | Out-File -FilePath $fileName -Encoding UTF8
    Write-QAInfo "Results saved to $fileName"
}

# Quality metrics tracking
$QualityReport = @{
    StartTime = Get-Date
    CompilationStatus = $null
    CompilationErrors = @()
    ClippyIssues = 0
    ClippyDetails = @()
    TestResults = @{ Passed = 0; Failed = 0; Total = 0 }
    FailedTests = @{
        TestNames = @()
        Details = @()
        FailuresList = @()
    }
    Coverage = "Skipped"  # Default to "Skipped" instead of null
    MissionCoverage = @()
    DocumentationWarnings = 0
    DocumentationDetails = @()
    Formatting = $null
    FormattingDetails = @()
    SecurityVulnerabilities = "Not Available"  # Default for optional checks
    SecurityDetails = @()
    OutdatedDependencies = "Not Available"
    OutdatedDetails = @()
    UnusedDependencies = "Not Available"
    UnusedDetails = @()
    DeadCodeWarnings = "Skipped"
    DeadCodeDetails = @()
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
    
    # Determine which checks to run
    $checksToRun = @()
    
    # Handle -Menu parameter
    if ($Menu) {
        $selectedChecks = Show-QAMenu
        if ($selectedChecks.Count -eq 0) {
            Write-Host "No checks selected. Exiting." -ForegroundColor Yellow
            return
        }
        foreach ($checkName in $selectedChecks) {
            $checkDef = $script:CheckDefinitions[$checkName]
            $checksToRun += @{ Name = $checkDef.Name; Function = $checkDef.Function }
        }
    }
    # Handle -Run parameter for specific checks
    elseif ($Run -and $Run.Count -gt 0) {
        if ($Run -contains "All") {
            # Run all checks
            foreach ($checkName in @("Compilation", "Formatting", "Clippy", "Tests", "Documentation", "Coverage", "Security", "Outdated", "Unused", "DeadCode")) {
                $checkDef = $script:CheckDefinitions[$checkName]
                $checksToRun += @{ Name = $checkDef.Name; Function = $checkDef.Function }
            }
        } else {
            foreach ($checkName in $Run) {
                if ($script:CheckDefinitions.ContainsKey($checkName)) {
                    $checkDef = $script:CheckDefinitions[$checkName]
                    $checksToRun += @{ Name = $checkDef.Name; Function = $checkDef.Function }
                } else {
                    Write-QAWarning "Unknown check: $checkName"
                }
            }
        }
        
        if ($checksToRun.Count -eq 0) {
            Write-QAError "No valid checks specified."
            return
        }
        
        Write-QAInfo "Running selected checks: $($Run -join ', ')"
    }
    # Default behavior (original logic)
    else {
        # Build default check list
        $checksToRun = @(
            @{ Name = "Compilation Check"; Function = "Test-Compilation" },
            @{ Name = "Code Formatting"; Function = "Format-Code" },
            @{ Name = "Clippy Analysis"; Function = "Test-Clippy" },
            @{ Name = "Test Suite"; Function = "Test-Suite" },
            @{ Name = "Documentation"; Function = "Generate-Documentation" }
        )
        
        if (-not $Quick) {
            $checksToRun += @{ Name = "Coverage Analysis"; Function = "Test-Coverage" }
            
            if (-not $SkipSecurityAudit) {
                $checksToRun += @{ Name = "Security Audit"; Function = "Test-SecurityAudit" }
            }
            
            if (-not $SkipDependencyChecks) {
                $checksToRun += @{ Name = "Outdated Dependencies"; Function = "Test-OutdatedDependencies" }
                $checksToRun += @{ Name = "Unused Dependencies"; Function = "Test-UnusedDependencies" }
            }
            
            $checksToRun += @{ Name = "Dead Code Detection"; Function = "Test-DeadCode" }
        }
    }
    
    # Run the selected checks
    foreach ($check in $checksToRun) {
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
    
    # Save compilation results to file
    $additionalData = @{
        "Workspace" = (Get-Location).Path
        "Command" = "cargo check --workspace --message-format=json"
    }
    Save-CheckResult -CheckName "Compilation" -Output ($result -join "`n") -ExitCode $exitCode -AdditionalData $additionalData
    
    if ($exitCode -eq 0) {
        $QualityReport.CompilationStatus = "✅ Clean"
        Write-QASuccess "Compilation successful - no errors"
    } else {
        $QualityReport.CompilationStatus = "❌ Failed"
        
        # Capture error details
        $errorLines = $result | Where-Object { $_ -match "error\[|error:" }
        $QualityReport.CompilationErrors = $errorLines
        
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
        
        # Capture files that need formatting
        $formattingNeeded = & cargo fmt --all -- --check 2>&1 | Select-String "Diff in" | ForEach-Object { $_.Line }
        $QualityReport.FormattingDetails = $formattingNeeded
        
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
    
    # Count issues in output
    $issueLines = $clippyOutput | Select-String "warning:|error:" 
    $issueCount = ($issueLines | Measure-Object).Count
    
    # Save clippy results to file
    $additionalData = @{
        "Issue Count" = $issueCount
        "Command" = "cargo clippy --workspace -- -D warnings"
        "Status" = if ($exitCode -eq 0) { "Clean" } else { "Issues Found" }
    }
    Save-CheckResult -CheckName "Clippy" -Output ($clippyOutput -join "`n") -ExitCode $exitCode -AdditionalData $additionalData
    
    if ($exitCode -eq 0) {
        $QualityReport.ClippyIssues = 0
        Write-QASuccess "Clippy analysis passed - no issues found"
    } else {
        $QualityReport.ClippyIssues = $issueCount
        
        # Capture detailed issue information including file paths and line numbers
        # Include lines with: error/warning messages, file locations (-->), code context (|), and help/note
        $QualityReport.ClippyDetails = $clippyOutput | Where-Object { 
            $_ -match "warning:|error:|note:|help:|^\s*-->|^\s*\d+\s*\|"
        }
        
        Write-Host $clippyOutput -ForegroundColor Yellow
        throw "Clippy found $issueCount issues"
    }
}

function Test-Suite {
    Write-QAInfo "🧪 Running test suite..."
    
    # Run tests for all packages with detailed output
    $testOutput = & cargo test --workspace 2>&1
    $exitCode = $LASTEXITCODE
    
    # Parse test results from the human-readable output
    $testResultLines = $testOutput | Select-String "test result:"
    $totalPassed = 0
    $totalFailed = 0
    $totalIgnored = 0
    
    foreach ($line in $testResultLines) {
        if ($line.Line -match "test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored") {
            $totalPassed += [int]$matches[1]
            $totalFailed += [int]$matches[2] 
            $totalIgnored += [int]$matches[3]
        } elseif ($line.Line -match "test result: FAILED\. (\d+) passed; (\d+) failed; (\d+) ignored") {
            $totalPassed += [int]$matches[1]
            $totalFailed += [int]$matches[2]
            $totalIgnored += [int]$matches[3]
        }
    }
    
    $QualityReport.TestResults = @{
        Passed = $totalPassed
        Failed = $totalFailed  
        Ignored = $totalIgnored
        Total = $totalPassed + $totalFailed
    }
    
    # Save test results to file
    $additionalData = @{
        "Passed" = $totalPassed
        "Failed" = $totalFailed
        "Ignored" = $totalIgnored
        "Total" = $totalPassed + $totalFailed
        "Command" = "cargo test --workspace"
        "Success Rate" = if (($totalPassed + $totalFailed) -gt 0) { [math]::Round(($totalPassed / ($totalPassed + $totalFailed)) * 100, 2) } else { 0 }
    }
    Save-CheckResult -CheckName "Tests" -Output ($testOutput -join "`n") -ExitCode $exitCode -AdditionalData $additionalData
    
    if ($exitCode -ne 0 -or $totalFailed -gt 0) {
        # Capture failed test names - look for "test <name> ... FAILED"
        $failedTestNames = $testOutput | Select-String "test .+ \.\.\. FAILED" | ForEach-Object { 
            if ($_.Line -match "test (.+) \.\.\. FAILED") {
                $matches[1]
            }
        }
        
        # Capture panic messages and assertion failures with context
        $failureDetails = @()
        $inFailure = $false
        $failureBlock = @()
        
        foreach ($line in $testOutput) {
            # Start capturing on panic or assertion
            if ($line -match "thread '(.+)' panicked at" -or $line -match "assertion `left == right` failed") {
                $inFailure = $true
                $failureBlock = @($line)
            }
            # Continue capturing failure context
            elseif ($inFailure) {
                $failureBlock += $line
                # End capture after stack trace or empty line
                if ($line -match "^$" -or $line -match "note: run with" -or $failureBlock.Count -gt 20) {
                    $failureDetails += $failureBlock -join "`n"
                    $failureDetails += "---"
                    $inFailure = $false
                    $failureBlock = @()
                }
            }
        }
        
        # Also capture "failures:" section which lists all failed tests
        $failuresSection = @()
        $inFailuresSection = $false
        foreach ($line in $testOutput) {
            if ($line -match "^failures:$") {
                $inFailuresSection = $true
            }
            elseif ($inFailuresSection) {
                if ($line -match "^test result:" -or $line -match "^$" -and $failuresSection.Count -gt 0) {
                    break
                }
                if ($line.Trim()) {
                    $failuresSection += $line.Trim()
                }
            }
        }
        
        $QualityReport.FailedTests = @{
            TestNames = $failedTestNames
            Details = $failureDetails
            FailuresList = $failuresSection
        }
    }
    
    if ($exitCode -eq 0 -and $totalFailed -eq 0) {
        Write-QASuccess "All $($totalPassed) tests passed ($($totalIgnored) ignored)"
    } else {
        Write-Host $testOutput -ForegroundColor Red
        throw "$totalFailed tests failed out of $($totalPassed + $totalFailed) total"
    }
}

function Generate-Documentation {
    Write-QAInfo "📚 Generating documentation..."
    
    $docOutput = & cargo doc --workspace --no-deps 2>&1
    $exitCode = $LASTEXITCODE
    
    # Count documentation warnings
    $warningLines = $docOutput | Select-String "warning:"
    $warningCount = ($warningLines | Measure-Object).Count
    $QualityReport.DocumentationWarnings = $warningCount
    
    # Save detailed documentation output
    $docDetailedFile = Join-Path $ReportsDir "Documentation_Detailed_$Timestamp.txt"
    $docOutput | Out-File -FilePath $docDetailedFile -Encoding UTF8
    
    # Create detailed documentation warnings report
    $detailedDocReport = @()
    $detailedDocReport += "DOCUMENTATION WARNINGS REPORT"
    $detailedDocReport += "============================="
    $detailedDocReport += ""
    $detailedDocReport += "Total Warnings: $warningCount"
    $detailedDocReport += ""
    
    if ($warningCount -gt 0) {
        $detailedDocReport += "WARNING DETAILS:"
        $detailedDocReport += "================"
        
        # Group warnings by file
        $warningsByFile = @{}
        foreach ($warning in $warningLines) {
            $line = $warning.Line
            if ($line -match '(.+\.rs):(\d+):(\d+):') {
                $file = $matches[1]
                $lineNum = $matches[2]
                $colNum = $matches[3]
                
                if (-not $warningsByFile.ContainsKey($file)) {
                    $warningsByFile[$file] = @()
                }
                $warningsByFile[$file] += "  Line $lineNum`:$colNum - $line"
            } else {
                # Warning without file context
                if (-not $warningsByFile.ContainsKey("General")) {
                    $warningsByFile["General"] = @()
                }
                $warningsByFile["General"] += "  $line"
            }
        }
        
        # Add warnings grouped by file
        foreach ($file in ($warningsByFile.Keys | Sort-Object)) {
            $detailedDocReport += ""
            $detailedDocReport += "File: $file"
            $detailedDocReport += "----------------------------------------"
            foreach ($warning in $warningsByFile[$file]) {
                $detailedDocReport += $warning
            }
        }
        
        $QualityReport.DocumentationDetails = $warningLines | ForEach-Object { $_.Line }
    } else {
        $detailedDocReport += "✅ No documentation warnings found!"
    }
    
    # Save detailed documentation warnings report
    $docWarningsFile = Join-Path $ReportsDir "Documentation_Warnings_$Timestamp.txt"
    $detailedDocReport | Out-File -FilePath $docWarningsFile -Encoding UTF8
    
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
    
    # Determine coverage scope
    if ($MissionsOnly) {
        Write-QAInfo "Running missions-only coverage analysis..."
        Test-MissionsCoverage
        return
    }
    
    try {
        # Run tarpaulin and capture output
        $coverageOutput = & cargo tarpaulin --workspace --timeout 120 --out Json 2>&1
        $exitCode = $LASTEXITCODE
        
        if ($exitCode -eq 0) {
            # Save raw output to file
            $coverageDetailedFile = Join-Path $ReportsDir "Coverage_Detailed_$Timestamp.txt"
            $coverageOutput | Out-File -FilePath $coverageDetailedFile -Encoding UTF8
            
            # Parse coverage from JSON file (tarpaulin creates tarpaulin-report.json)
            try {
                $jsonFile = "tarpaulin-report.json"
                if (Test-Path $jsonFile) {
                    $coverageData = Get-Content $jsonFile -Raw | ConvertFrom-Json
                    
                    # Calculate overall coverage
                    $totalLines = 0
                    $coveredLines = 0
                
                $detailedCoverage = @()
                $detailedCoverage += "DETAILED COVERAGE REPORT"
                $detailedCoverage += "========================"
                $detailedCoverage += ""
                
                foreach ($file in $coverageData.files) {
                    $fileCoverage = if ($file.coverable -gt 0) { [math]::Round(($file.covered / $file.coverable) * 100, 2) } else { 0 }
                    $fileName = ($file.path -join "/")
                    $detailedCoverage += "File: $fileName"
                    $detailedCoverage += "  Lines: $($file.coverable) | Covered: $($file.covered) | Coverage: $fileCoverage%"
                    $detailedCoverage += ""
                    
                    $totalLines += $file.coverable
                    $coveredLines += $file.covered
                }
                
                $overallCoverage = if ($totalLines -gt 0) { [math]::Round(($coveredLines / $totalLines) * 100, 2) } else { 0 }
                $QualityReport.Coverage = "$overallCoverage%"
                
                # Save detailed coverage report
                $detailedCoverage += ""
                $detailedCoverage += "OVERALL SUMMARY"
                $detailedCoverage += "==============="
                $detailedCoverage += "Total Lines: $totalLines"
                $detailedCoverage += "Covered Lines: $coveredLines"
                    $detailedCoverage += "Overall Coverage: $overallCoverage%"
                    
                    $detailedCoverageFile = Join-Path $ReportsDir "Coverage_ByFile_$Timestamp.txt"
                    $detailedCoverage | Out-File -FilePath $detailedCoverageFile -Encoding UTF8
                    
                    if ($overallCoverage -ge 85) {
                        Write-QASuccess "Test coverage: $overallCoverage% (target: ≥85%)"
                    } else {
                        Write-QAWarning "Test coverage: $overallCoverage% (below target of 85%)"
                    }
                } else {
                    Write-QAWarning "JSON coverage file not found, falling back to text parsing"
                    throw "JSON file not found"
                }
            }
            catch {
                # Fallback to parsing human-readable output if JSON parsing fails
                Write-QAWarning "JSON parsing failed, falling back to text parsing: $_"
                $coverageOutput = & cargo tarpaulin --workspace --timeout 120 2>&1
                $coverageLine = $coverageOutput | Where-Object { $_ -match 'coverage, \d+/\d+ lines covered' }
                if ($coverageLine) {
                    $match = [regex]::Match($coverageLine, '(\d+\.\d+)% coverage')
                    if ($match.Success) {
                        $coveragePercent = [math]::Round([double]$match.Groups[1].Value, 2)
                        $QualityReport.Coverage = "$coveragePercent%"
                        
                        if ($coveragePercent -ge 85) {
                            Write-QASuccess "Test coverage: $coveragePercent% (target: ≥85%)"
                        } else {
                            Write-QAWarning "Test coverage: $coveragePercent% (below target of 85%)"
                        }
                    }
                }
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

function Test-MissionsCoverage {
    Write-QAInfo "📊 Analyzing coverage for missions only..."
    
    $missionDirs = Get-ChildItem -Path "missions" -Directory -Filter "Mission*" | Sort-Object Name
    
    if ($missionDirs.Count -eq 0) {
        Write-QAWarning "No mission directories found"
        return
    }
    
    $missionReports = @()
    $totalMissionCoverage = 0
    $successfulMissions = 0
    
    foreach ($missionDir in $missionDirs) {
        $missionName = $missionDir.Name
        Write-QAInfo "  Analyzing $missionName..."
        
        try {
            # Run tarpaulin for this specific mission
            Push-Location $missionDir.FullName
            
            $coverageOutput = & cargo tarpaulin --out Json --timeout 120 2>&1
            $exitCode = $LASTEXITCODE
            
            if ($exitCode -eq 0 -and (Test-Path "tarpaulin-report.json")) {
                $coverageData = Get-Content "tarpaulin-report.json" -Raw | ConvertFrom-Json
                
                $totalLines = 0
                $coveredLines = 0
                
                foreach ($file in $coverageData.files.PSObject.Properties) {
                    $totalLines += $file.Value.coverable
                    $coveredLines += $file.Value.covered
                }
                
                $coverage = if ($totalLines -gt 0) { [math]::Round(($coveredLines / $totalLines) * 100, 2) } else { 0 }
                
                $missionReports += @{
                    Mission = $missionName
                    Coverage = $coverage
                    Lines = $totalLines
                    Covered = $coveredLines
                }
                
                $totalMissionCoverage += $coverage
                $successfulMissions++
                
                Write-QASuccess "  ${missionName}: ${coverage}% coverage"
                
                # Clean up JSON file
                Remove-Item "tarpaulin-report.json" -ErrorAction SilentlyContinue
            } else {
                Write-QAWarning "  ${missionName}: Coverage analysis failed"
            }
        }
        catch {
            Write-QAWarning "  ${missionName}: Error - $_"
        }
        finally {
            Pop-Location
        }
    }
    
    if ($successfulMissions -gt 0) {
        $avgCoverage = [math]::Round($totalMissionCoverage / $successfulMissions, 2)
        $QualityReport.Coverage = "$avgCoverage% (avg across $successfulMissions missions)"
        $QualityReport.MissionCoverage = $missionReports
        
        Write-QASuccess "Average mission coverage: $avgCoverage%"
        
        # Save detailed mission coverage report
        $missionCoverageReport = @()
        $missionCoverageReport += "MISSION COVERAGE REPORT"
        $missionCoverageReport += "======================="
        $missionCoverageReport += ""
        $missionCoverageReport += "Average Coverage: $avgCoverage%"
        $missionCoverageReport += "Missions Analyzed: $successfulMissions"
        $missionCoverageReport += ""
        $missionCoverageReport += "INDIVIDUAL MISSION RESULTS:"
        $missionCoverageReport += "---------------------------"
        
        foreach ($report in $missionReports | Sort-Object -Property Coverage -Descending) {
            $status = if ($report.Coverage -ge 85) { "✅" } elseif ($report.Coverage -ge 70) { "⚠️" } else { "❌" }
            $missionCoverageReport += "$status $($report.Mission): $($report.Coverage)% ($($report.Covered)/$($report.Lines) lines)"
        }
        
        $missionCoverageFile = Join-Path $ReportsDir "Mission_Coverage_$Timestamp.txt"
        $missionCoverageReport | Out-File -FilePath $missionCoverageFile -Encoding UTF8
        Write-QAInfo "Mission coverage report saved to $missionCoverageFile"
    }
}

function Test-SecurityAudit {
    Write-QAInfo "🔒 Running security audit..."
    
    # Check if cargo-audit is installed
    if (-not (Get-Command "cargo-audit" -ErrorAction SilentlyContinue)) {
        Write-QAWarning "cargo-audit not installed. Run: cargo install cargo-audit"
        $QualityReport.SecurityVulnerabilities = "Not Available"
        return
    }
    
    try {
        # Run cargo audit without --json first to get human-readable output for parsing
        $auditOutput = & cargo audit 2>&1
        $exitCode = $LASTEXITCODE
        
        # Save audit results
        Save-CheckResult -CheckName "Security_Audit" -Output ($auditOutput -join "`n") -ExitCode $exitCode
        
        # Parse the output to count actual vulnerabilities vs warnings
        # Vulnerabilities are serious (CVE with security impact)
        # Warnings are unmaintained/yanked packages (less critical)
        $vulnerabilityLines = $auditOutput | Where-Object { $_ -match 'Vulnerability:|vulnerability' -and $_ -notmatch 'vulnerabilities' }
        $warningLines = $auditOutput | Where-Object { $_ -match 'Warning:\s+unmaintained|Warning:\s+yanked' }
        
        $vulnCount = ($vulnerabilityLines | Measure-Object).Count
        $warningCount = ($warningLines | Measure-Object).Count
        
        # Also check for the summary line
        $summaryMatch = $auditOutput | Select-String '(\d+) (vulnerabilities|vulnerability) found'
        if ($summaryMatch) {
            $vulnCount = [int]$summaryMatch.Matches[0].Groups[1].Value
        }
        
        $allowedWarningsMatch = $auditOutput | Select-String '(\d+) allowed warnings? found'
        if ($allowedWarningsMatch) {
            $warningCount = [int]$allowedWarningsMatch.Matches[0].Groups[1].Value
        }
        
        $QualityReport.SecurityVulnerabilities = $vulnCount
        
        # Capture details about issues found
        $securityDetails = @()
        
        # Parse crate warnings/vulnerabilities
        $currentCrate = $null
        foreach ($line in $auditOutput) {
            if ($line -match '^Crate:\s+(.+)$') {
                $currentCrate = $matches[1]
            }
            elseif ($line -match '^Title:\s+(.+)$' -and $currentCrate) {
                $title = $matches[1]
                $securityDetails += "$currentCrate - $title"
            }
        }
        
        $QualityReport.SecurityDetails = $securityDetails
        
        if ($vulnCount -eq 0 -and $warningCount -eq 0) {
            Write-QASuccess "No known security vulnerabilities or warnings found"
        } elseif ($vulnCount -eq 0) {
            Write-QASuccess "No security vulnerabilities found ($warningCount unmaintained package warnings)"
            if ($securityDetails.Count -gt 0) {
                Write-QAInfo "Unmaintained packages:"
                $securityDetails | ForEach-Object { Write-Host "  ⚠️ $_" -ForegroundColor Yellow }
            }
        } else {
            Write-QAWarning "Found $vulnCount security vulnerabilities and $warningCount warnings"
            $securityDetails | ForEach-Object { Write-Host "  $_" -ForegroundColor Yellow }
        }
    }
    catch {
        Write-QAWarning "Security audit failed: $_"
        $QualityReport.SecurityVulnerabilities = "Failed"
    }
}

function Test-OutdatedDependencies {
    Write-QAInfo "📦 Checking for outdated dependencies..."
    
    # Check if cargo-outdated is installed
    if (-not (Get-Command "cargo-outdated" -ErrorAction SilentlyContinue)) {
        Write-QAWarning "cargo-outdated not installed. Run: cargo install cargo-outdated"
        $QualityReport.OutdatedDependencies = "Not Available"
        return
    }
    
    try {
        $outdatedOutput = & cargo outdated --root-deps-only 2>&1
        $exitCode = $LASTEXITCODE
        
        # Save outdated dependencies results
        Save-CheckResult -CheckName "Outdated_Dependencies" -Output ($outdatedOutput -join "`n") -ExitCode $exitCode
        
        # Check if all dependencies are up to date (special message from cargo-outdated)
        $allUpToDate = $outdatedOutput | Where-Object { $_ -match 'All dependencies are up to date' }
        
        if ($allUpToDate) {
            $QualityReport.OutdatedDependencies = 0
            Write-QASuccess "All dependencies are up to date"
        } else {
            # Count outdated dependencies from output - exclude header rows and empty lines
            $outdatedLines = $outdatedOutput | Where-Object { 
                $_ -match '^\s*\w+' -and 
                $_ -notmatch '^Name\s+' -and 
                $_ -notmatch '^---' -and
                $_ -notmatch 'Fetching|Parsing|Checking'
            }
            $outdatedCount = ($outdatedLines | Measure-Object).Count
            
            $QualityReport.OutdatedDependencies = $outdatedCount
            
            if ($outdatedCount -eq 0) {
                Write-QASuccess "All dependencies are up to date"
            } else {
                $QualityReport.OutdatedDetails = $outdatedLines
                Write-QAWarning "Found $outdatedCount outdated dependencies"
                $outdatedLines | Select-Object -First 10 | ForEach-Object { Write-Host "  $_" -ForegroundColor Yellow }
                if ($outdatedCount -gt 10) {
                    Write-Host "  ... and $($outdatedCount - 10) more" -ForegroundColor Gray
                }
            }
        }
    }
    catch {
        Write-QAWarning "Outdated dependency check failed: $_"
        $QualityReport.OutdatedDependencies = "Failed"
    }
}

function Test-UnusedDependencies {
    Write-QAInfo "🧹 Checking for unused dependencies..."
    
    # Check if cargo-udeps is installed
    if (-not (Get-Command "cargo-udeps" -ErrorAction SilentlyContinue)) {
        Write-QAWarning "cargo-udeps not installed. Run: cargo install cargo-udeps --locked"
        Write-QAWarning "Note: cargo-udeps requires nightly toolchain"
        $QualityReport.UnusedDependencies = "Not Available"
        return
    }
    
    try {
        # cargo-udeps requires nightly
        $udepsOutput = & cargo +nightly udeps --all-targets 2>&1
        $exitCode = $LASTEXITCODE
        
        # Save unused dependencies results
        Save-CheckResult -CheckName "Unused_Dependencies" -Output ($udepsOutput -join "`n") -ExitCode $exitCode
        
        # Parse output for unused dependencies
        $unusedLines = $udepsOutput | Where-Object { $_ -match 'unused' }
        $unusedCount = ($unusedLines | Measure-Object).Count
        
        $QualityReport.UnusedDependencies = $unusedCount
        
        if ($unusedCount -eq 0) {
            Write-QASuccess "No unused dependencies found"
        } else {
            $QualityReport.UnusedDetails = $unusedLines
            Write-QAWarning "Found $unusedCount unused dependencies"
            $unusedLines | ForEach-Object { Write-Host "  $_" -ForegroundColor Yellow }
        }
    }
    catch {
        Write-QAWarning "Unused dependency check failed: $_"
        $QualityReport.UnusedDependencies = "Failed"
    }
}

function Test-DeadCode {
    Write-QAInfo "💀 Detecting dead code..."
    
    try {
        # Run clippy with dead_code lint
        $deadCodeOutput = & cargo clippy --workspace -- -W dead_code 2>&1
        $exitCode = $LASTEXITCODE
        
        # Save dead code results
        Save-CheckResult -CheckName "Dead_Code" -Output ($deadCodeOutput -join "`n") -ExitCode $exitCode
        
        # Count dead code warnings
        $deadCodeLines = $deadCodeOutput | Where-Object { $_ -match 'dead_code|never used|never constructed' }
        $deadCodeCount = ($deadCodeLines | Measure-Object).Count
        
        $QualityReport.DeadCodeWarnings = $deadCodeCount
        
        if ($deadCodeCount -eq 0) {
            Write-QASuccess "No dead code detected"
        } else {
            $QualityReport.DeadCodeDetails = $deadCodeLines
            Write-QAWarning "Found $deadCodeCount potential dead code instances"
            $deadCodeLines | Select-Object -First 15 | ForEach-Object { Write-Host "  $_" -ForegroundColor Yellow }
            if ($deadCodeCount -gt 15) {
                Write-Host "  ... and $($deadCodeCount - 15) more" -ForegroundColor Gray
            }
        }
    }
    catch {
        Write-QAWarning "Dead code detection failed: $_"
        $QualityReport.DeadCodeWarnings = "Failed"
    }
}

function Generate-QualityReport {
    $endTime = Get-Date
    $duration = $endTime - $QualityReport.StartTime
    
    # Build detailed report sections
    $detailSections = @()
    
    # Compilation errors
    if ($QualityReport.CompilationErrors.Count -gt 0) {
        $detailSections += @"

❌ COMPILATION ERRORS ($($QualityReport.CompilationErrors.Count)):
$("-" * 50)
$($QualityReport.CompilationErrors | ForEach-Object { "  $_" } | Out-String)
"@
    }
    
    # Formatting details
    if ($QualityReport.FormattingDetails.Count -gt 0) {
        $detailSections += @"

⚠️  FORMATTING ISSUES ($($QualityReport.FormattingDetails.Count) files):
$("-" * 50)
$($QualityReport.FormattingDetails | ForEach-Object { "  $_" } | Out-String)
"@
    }
    
    # Clippy issues
    if ($QualityReport.ClippyIssues -gt 0) {
        $detailSections += @"

⚠️  CLIPPY ISSUES ($($QualityReport.ClippyIssues)):
$("-" * 50)
$($QualityReport.ClippyDetails | Select-Object -First 50 | ForEach-Object { "  $_" } | Out-String)
$(if ($QualityReport.ClippyDetails.Count -gt 50) { "  ... (showing first 50 of $($QualityReport.ClippyDetails.Count) lines)" })
"@
    }
    
    # Failed tests - enhanced with detailed capture
    if ($QualityReport.FailedTests -and ($QualityReport.FailedTests.TestNames.Count -gt 0 -or $QualityReport.FailedTests.FailuresList.Count -gt 0)) {
        $detailSections += @"

❌ FAILED TESTS:
$("-" * 50)
"@
        # Show failed test names
        if ($QualityReport.FailedTests.TestNames.Count -gt 0) {
            $detailSections += "`nFailed Test Names:`n"
            foreach ($testName in $QualityReport.FailedTests.TestNames) {
                $detailSections += "  - $testName`n"
            }
        }
        
        # Show failures list from "failures:" section
        if ($QualityReport.FailedTests.FailuresList.Count -gt 0) {
            $detailSections += "`nFailure Summary:`n"
            foreach ($failure in $QualityReport.FailedTests.FailuresList) {
                $detailSections += "  $failure`n"
            }
        }
        
        # Show detailed failure messages (panic info, assertion details)
        if ($QualityReport.FailedTests.Details.Count -gt 0) {
            $detailSections += "`nFailure Details:`n"
            foreach ($detail in $QualityReport.FailedTests.Details) {
                $detailSections += "  $detail`n"
            }
        }
    }
    # Legacy fallback for old format
    elseif ($QualityReport.FailedTests -and $QualityReport.FailedTests -is [array] -and $QualityReport.FailedTests.Count -gt 0) {
        $detailSections += @"

❌ FAILED TESTS:
$("-" * 50)
$($QualityReport.FailedTests | ForEach-Object { "  $_" } | Out-String)
"@
    }
    
    # Documentation warnings
    if ($QualityReport.DocumentationWarnings -gt 0) {
        $detailSections += @"

⚠️  DOCUMENTATION WARNINGS ($($QualityReport.DocumentationWarnings)):
$("-" * 50)
$($QualityReport.DocumentationDetails | Select-Object -First 30 | ForEach-Object { "  $_" } | Out-String)
$(if ($QualityReport.DocumentationDetails.Count -gt 30) { "  ... (showing first 30 of $($QualityReport.DocumentationDetails.Count) warnings)" })
"@
    }
    
    # Mission coverage breakdown
    if ($QualityReport.MissionCoverage.Count -gt 0) {
        $detailSections += @"

📊 MISSION COVERAGE BREAKDOWN:
$("-" * 50)
$($QualityReport.MissionCoverage | ForEach-Object { "  $($_.Mission): $($_.Coverage)% ($($_.Covered)/$($_.Lines) lines)" } | Out-String)
"@
    }
    
    # Security vulnerabilities
    if ($QualityReport.SecurityVulnerabilities -gt 0 -and $QualityReport.SecurityDetails.Count -gt 0) {
        $detailSections += @"

🔒 SECURITY VULNERABILITIES ($($QualityReport.SecurityVulnerabilities)):
$("-" * 50)
$($QualityReport.SecurityDetails | ForEach-Object { "  $_" } | Out-String)
"@
    }
    
    # Outdated dependencies
    if ($QualityReport.OutdatedDependencies -gt 0 -and $QualityReport.OutdatedDetails.Count -gt 0) {
        $detailSections += @"

📦 OUTDATED DEPENDENCIES ($($QualityReport.OutdatedDependencies)):
$("-" * 50)
$($QualityReport.OutdatedDetails | Select-Object -First 20 | ForEach-Object { "  $_" } | Out-String)
$(if ($QualityReport.OutdatedDetails.Count -gt 20) { "  ... (showing first 20 of $($QualityReport.OutdatedDetails.Count) outdated deps)" })
"@
    }
    
    # Unused dependencies
    if ($QualityReport.UnusedDependencies -gt 0 -and $QualityReport.UnusedDetails.Count -gt 0) {
        $detailSections += @"

🧹 UNUSED DEPENDENCIES ($($QualityReport.UnusedDependencies)):
$("-" * 50)
$($QualityReport.UnusedDetails | ForEach-Object { "  $_" } | Out-String)
"@
    }
    
    # Dead code
    if ($QualityReport.DeadCodeWarnings -gt 0 -and $QualityReport.DeadCodeDetails.Count -gt 0) {
        $detailSections += @"

💀 DEAD CODE WARNINGS ($($QualityReport.DeadCodeWarnings)):
$("-" * 50)
$($QualityReport.DeadCodeDetails | Select-Object -First 20 | ForEach-Object { "  $_" } | Out-String)
$(if ($QualityReport.DeadCodeDetails.Count -gt 20) { "  ... (showing first 20 of $($QualityReport.DeadCodeDetails.Count) warnings)" })
"@
    }
    
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

🔒 SECURITY & DEPENDENCIES:
  Security Vulnerabilities: $($QualityReport.SecurityVulnerabilities)
  Outdated Dependencies: $($QualityReport.OutdatedDependencies)
  Unused Dependencies: $($QualityReport.UnusedDependencies)
  Dead Code Warnings: $($QualityReport.DeadCodeWarnings)

🎉 OVERALL STATUS: $(if ($QualityReport.TestResults.Failed -eq 0 -and $QualityReport.ClippyIssues -eq 0 -and $QualityReport.CompilationErrors.Count -eq 0 -and $QualityReport.SecurityVulnerabilities -eq 0) { "✅ PASSED" } else { "❌ ISSUES FOUND" })
$($detailSections -join "`n")

"@
    
    Write-Host $report -ForegroundColor $(if ($QualityReport.TestResults.Failed -eq 0 -and $QualityReport.ClippyIssues -eq 0 -and $QualityReport.CompilationErrors.Count -eq 0) { "Green" } else { "Yellow" })
    
    # Always save comprehensive report to timestamped file
    $comprehensiveReportFile = "$ReportsDir\Quality_Report_$Timestamp.txt"
    $report | Out-File -FilePath $comprehensiveReportFile -Encoding UTF8
    Write-QAInfo "Comprehensive report saved to $comprehensiveReportFile"
    
    # Output to file if requested (additional file)
    if ($OutputFile) {
        $report | Out-File -FilePath $OutputFile -Encoding UTF8
        Write-QAInfo "Additional report saved to $OutputFile"
    }
    
    # Create a latest symlink for easy access
    $latestReportFile = "$ReportsDir\latest_quality_report.txt"
    Copy-Item -Path $comprehensiveReportFile -Destination $latestReportFile -Force
    Write-QAInfo "Latest report also saved as $latestReportFile"
    
    # Show summary of generated files
    Write-Host "`n📁 Generated Report Files:" -ForegroundColor Blue
    Get-ChildItem -Path $ReportsDir -Filter "*$Timestamp*" | ForEach-Object {
        Write-Host "  📄 $($_.Name)" -ForegroundColor White
    }
    Write-Host "  📄 latest_quality_report.txt (symlink to latest)" -ForegroundColor Gray
    
    # Return exit code based on results
    if ($QualityReport.TestResults.Failed -gt 0 -or $QualityReport.ClippyIssues -gt 0 -or $QualityReport.CompilationErrors.Count -gt 0) {
        exit 1
    } else {
        exit 0
    }
}

# Main execution
if ($MyInvocation.InvocationName -ne '.') {
    Start-QualityPipeline
}
