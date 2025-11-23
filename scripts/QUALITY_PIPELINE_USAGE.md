# Quality Pipeline Usage Guide

**Updated**: 2025-11-23  
**Script**: `scripts/quality-pipeline.ps1`

---

## 🎯 Overview

Comprehensive local quality assurance pipeline for Rust workspace with 10+ quality checks.

## 🚀 Quick Start

### Basic Usage (All Checks)

```powershell
.\scripts\quality-pipeline.ps1
```

Runs all quality checks with full analysis (6-10 minutes for complete workspace).

### Quick Mode (Essential Checks Only)

```powershell
.\scripts\quality-pipeline.ps1 -Quick
```

Skips coverage, security audit, dependency checks, and dead code detection (~2-3 minutes).

### Missions-Only Coverage

```powershell
.\scripts\quality-pipeline.ps1 -MissionsOnly
```

Runs coverage analysis only for Mission1-10 packages instead of entire workspace.

---

## 📋 Available Checks

### Core Quality Checks (Always Run)

1. **✅ Compilation Check** (`cargo check --workspace`)
   - Verifies all packages compile successfully
   - Captures compilation errors with file locations

2. **🎨 Code Formatting** (`cargo fmt --all`)
   - Ensures consistent code style
   - Auto-applies formatting if needed

3. **📎 Clippy Analysis** (`cargo clippy --workspace -- -D warnings`)
   - Linting and best practices enforcement
   - Zero warnings policy

4. **🧪 Test Suite** (`cargo test --workspace`)
   - Runs all unit and integration tests
   - Reports pass/fail counts with details

5. **📚 Documentation** (`cargo doc --workspace --no-deps`)
   - Generates API documentation
   - Tracks documentation warnings

### Extended Checks (Skipped with `-Quick`)

6. **📊 Code Coverage** (`cargo tarpaulin`)
   - Line coverage analysis
   - `-MissionsOnly`: Per-mission coverage reports
   - Default: Workspace-wide coverage

7. **🔒 Security Audit** (`cargo audit`)
   - Scans for known vulnerabilities in dependencies
   - Uses RustSec Advisory Database
   - Skip with: `-SkipSecurityAudit`

8. **📦 Outdated Dependencies** (`cargo outdated`)
   - Identifies dependencies with newer versions
   - Shows current vs latest versions
   - Skip with: `-SkipDependencyChecks`

9. **🧹 Unused Dependencies** (`cargo udeps`)
   - Detects dependencies declared but never used
   - Requires nightly toolchain
   - Skip with: `-SkipDependencyChecks`

10. **💀 Dead Code Detection** (`cargo clippy -- -W dead_code`)
    - Finds unused functions, structs, and modules
    - Helps identify code that can be removed

---

## 🔧 Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `-Quick` | Switch | Skip coverage, security, and dependency checks |
| `-MissionsOnly` | Switch | Run missions-only coverage (faster than workspace) |
| `-SkipSecurityAudit` | Switch | Skip security vulnerability scanning |
| `-SkipDependencyChecks` | Switch | Skip outdated and unused dependency checks |
| `-FailFast` | Switch | Stop on first failure instead of continuing |
| `-NonInteractive` | Switch | Skip interactive prompts (CI mode) |
| `-OutputFile <path>` | String | Additional output file for report |

---

## 💡 Common Usage Patterns

### Pre-Commit Check (Quick)

```powershell
.\scripts\quality-pipeline.ps1 -Quick
```

**Runs**: Compilation, formatting, clippy, tests, docs  
**Skips**: Coverage, security, dependencies, dead code  
**Time**: ~2-3 minutes

### Weekly Quality Review (Full)

```powershell
.\scripts\quality-pipeline.ps1
```

**Runs**: All 10 checks  
**Time**: ~6-10 minutes (workspace-wide coverage)

### Mission Focus (Coverage Analysis)

```powershell
.\scripts\quality-pipeline.ps1 -MissionsOnly
```

**Runs**: All 10 checks with missions-only coverage  
**Time**: ~4-5 minutes (faster than workspace coverage)

### Security & Dependency Audit Only

```powershell
.\scripts\quality-pipeline.ps1 -Quick:$false -SkipDependencyChecks:$false
```

**Focus**: Security vulnerabilities and dependency health

### CI/CD Mode (Non-Interactive)

```powershell
.\scripts\quality-pipeline.ps1 -NonInteractive -FailFast
```

**Behavior**: Stops on first failure, no prompts, exit code indicates status

---

## 📁 Generated Reports

All reports saved to `reports/` directory with timestamps:

### Individual Check Reports

- `Compilation_<timestamp>.txt` - Build errors and warnings
- `Clippy_<timestamp>.txt` - Linting issues with file locations
- `Tests_<timestamp>.txt` - Test execution details
- `Documentation_Detailed_<timestamp>.txt` - Doc generation output
- `Documentation_Warnings_<timestamp>.txt` - Grouped documentation warnings
- `Coverage_Detailed_<timestamp>.txt` - Raw coverage output
- `Coverage_ByFile_<timestamp>.txt` - Per-file coverage breakdown
- `Mission_Coverage_<timestamp>.txt` - Per-mission coverage (with `-MissionsOnly`)
- `Security_Audit_<timestamp>.txt` - Vulnerability scan results
- `Outdated_Dependencies_<timestamp>.txt` - Outdated dependency list
- `Unused_Dependencies_<timestamp>.txt` - Unused dependency list
- `Dead_Code_<timestamp>.txt` - Dead code detection results

### Comprehensive Reports

- `Quality_Report_<timestamp>.txt` - Complete quality assessment
- `latest_quality_report.txt` - Symlink to most recent report

**Report Structure**:
```
🎯 QUALITY ASSURANCE REPORT
===========================
Timestamp: 2025-11-23 14:30:45
Duration: 05:42

📊 CODE QUALITY RESULTS:
  Compilation: ✅ Clean
  Formatting: ✅ Already formatted
  Clippy Issues: 0
  Tests: 458/458 passed
  Coverage: 78.5% (avg across 10 missions)
  Doc Warnings: 3

🔒 SECURITY & DEPENDENCIES:
  Security Vulnerabilities: 0
  Outdated Dependencies: 5
  Unused Dependencies: 0
  Dead Code Warnings: 12

🎉 OVERALL STATUS: ✅ PASSED
```

---

## 🛠️ Prerequisites

### Required Tools

**Always Required**:
- `cargo` (Rust toolchain)
- `cargo fmt` (part of rustup)
- `cargo clippy` (part of rustup)

**Optional Tools** (for extended checks):

Install all optional tools:
```powershell
# Code coverage
cargo install cargo-tarpaulin

# Security audit
cargo install cargo-audit

# Outdated dependencies
cargo install cargo-outdated

# Unused dependencies (requires nightly)
cargo install cargo-udeps --locked
rustup toolchain install nightly
```

### Tool Installation Check

Pipeline automatically detects missing tools and provides installation instructions:

```
⚠️  cargo-audit not installed. Run: cargo install cargo-audit
```

Missing tools are gracefully skipped with warnings.

---

## 📊 Coverage Thresholds

### Interpretation

| Coverage | Status | Recommendation |
|----------|--------|----------------|
| **< 70%** | ⚠️ Low | Add fundamental tests |
| **70-85%** | ✅ Good | Focus on edge cases |
| **> 85%** | ✅ Excellent | Maintain quality |

### Missions-Only Coverage

When using `-MissionsOnly`, each mission gets individual coverage report:

```
MISSION COVERAGE BREAKDOWN:
--------------------------------------------------
  Mission8: 92.5% (148/160 lines)
  Mission6: 88.3% (142/161 lines)
  Mission5: 85.7% (120/140 lines)
  Mission1: 78.9% (90/114 lines)
  ...
```

---

## 🔍 Understanding Results

### Exit Codes

- **0**: All checks passed
- **1**: At least one check failed

### Status Indicators

- ✅ **Clean/Passed** - No issues found
- ⚠️ **Warning** - Issues found but not critical
- ❌ **Failed** - Critical issues requiring attention

### Common Issues

#### Compilation Errors

**Action**: Fix syntax errors and type issues before running other checks.

```
❌ COMPILATION ERRORS (3):
  error[E0425]: cannot find value `x` in this scope
  --> src/main.rs:10:5
```

#### Clippy Warnings

**Action**: Address linting suggestions to improve code quality.

```
⚠️  CLIPPY ISSUES (5):
  warning: unused variable: `result`
  --> src/solver/day11.rs:45:9
```

#### Failed Tests

**Action**: Fix test failures immediately - indicates broken functionality.

```
❌ FAILED TESTS:
  test solver::day11::tests::test_part2_sample ... FAILED
  thread 'test_part2_sample' panicked at 'assertion failed'
```

#### Security Vulnerabilities

**Action**: Update dependencies with known security issues.

```
🔒 SECURITY VULNERABILITIES (2):
  RUSTSEC-2024-001: Vulnerability in regex 1.5.0
  RUSTSEC-2024-002: Buffer overflow in xml-rs 0.8.3
```

#### Outdated Dependencies

**Action**: Review and update dependencies periodically (not urgent).

```
📦 OUTDATED DEPENDENCIES (5):
  serde: 1.0.150 -> 1.0.160 (latest)
  tokio: 1.25.0 -> 1.28.0 (latest)
```

---

## ⚡ Performance Tips

### Speed Optimization

1. **Use `-Quick` for pre-commit**: Skips heavy analysis (~2-3 min vs 6-10 min)
2. **Use `-MissionsOnly` for coverage**: Faster than workspace-wide coverage
3. **Run full pipeline weekly**: Not every commit

### Parallel Execution

Pipeline runs checks sequentially for clarity. For parallel execution, run individual checks:

```powershell
# Terminal 1
cargo test --workspace

# Terminal 2  
cargo clippy --workspace -- -D warnings

# Terminal 3
cargo tarpaulin --workspace
```

### Incremental Analysis

For focused checks on specific package:

```powershell
# Single package
cd missions/Mission5
cargo test
cargo clippy -- -D warnings
cargo tarpaulin
```

---

## 🔗 Integration

### Git Pre-Commit Hook

Create `.git/hooks/pre-commit`:

```bash
#!/bin/sh
pwsh -Command ".\scripts\quality-pipeline.ps1 -Quick -FailFast"
```

### VSCode Task

Add to `.vscode/tasks.json`:

```json
{
  "label": "Quality Pipeline (Quick)",
  "type": "shell",
  "command": ".\\scripts\\quality-pipeline.ps1 -Quick",
  "problemMatcher": []
}
```

### Scheduled Task (Windows)

Run weekly quality checks automatically:

```powershell
$action = New-ScheduledTaskAction -Execute "pwsh.exe" `
  -Argument "-File D:\repos\rust_study\scripts\quality-pipeline.ps1"
  
$trigger = New-ScheduledTaskTrigger -Weekly -DaysOfWeek Sunday -At 2am

Register-ScheduledTask -TaskName "RustQualityPipeline" `
  -Action $action -Trigger $trigger
```

---

## 🆚 Pipeline vs CI/CD

### Local Pipeline (This Script)

**When**: Before committing, during development  
**Purpose**: Fast feedback, catch issues early  
**Scope**: Configurable (quick vs full)  
**Reports**: Detailed local files

### GitHub Actions CI/CD

**When**: Nightly at 2 AM UTC  
**Purpose**: Comprehensive validation, long-term tracking  
**Scope**: Always full (all checks)  
**Reports**: GitHub issues and artifacts

### Complementary Usage

```
┌─────────────────────────────────────────────────┐
│ Development Workflow                            │
├─────────────────────────────────────────────────┤
│                                                 │
│  Write Code                                     │
│      ↓                                          │
│  Run Local Pipeline (Quick)  ← 2-3 min         │
│      ↓                                          │
│  Fix Issues                                     │
│      ↓                                          │
│  Commit & Push                                  │
│      ↓                                          │
│  Nightly CI/CD (Full)        ← 10-15 min       │
│      ↓                                          │
│  Review GitHub Issue                            │
│                                                 │
└─────────────────────────────────────────────────┘
```

---

## 📚 Related Documentation

- `.github/CODE_COVERAGE_INTEGRATION.md` - Nightly CI/CD coverage details
- `.github/workflows/nightly-comprehensive-tests.yml` - CI/CD workflow
- `.github/RUST_TEST_DOCUMENTATION_STANDARDS.md` - Testing standards
- `.github/RUST_DOCUMENTATION_STANDARDS.md` - Documentation standards

---

## 🐛 Troubleshooting

### "cargo-tarpaulin not found"

**Solution**: Install cargo-tarpaulin

```powershell
cargo install cargo-tarpaulin
```

**Note**: Windows support may require WSL for full functionality.

### "cargo-udeps requires nightly"

**Solution**: Install nightly toolchain

```powershell
rustup toolchain install nightly
```

Then pipeline will use `cargo +nightly udeps`.

### "Coverage analysis failed"

**Common Causes**:
- Compilation errors (run `cargo build` first)
- Tarpaulin timeout (increase `--timeout` value)
- Missing tests (coverage requires tests to run)

**Solution**: Fix compilation first, then retry coverage.

### Slow workspace coverage

**Solution**: Use `-MissionsOnly` to focus on mission packages:

```powershell
.\scripts\quality-pipeline.ps1 -MissionsOnly
```

Analyzes 10 missions (~4-5 min) vs entire workspace (~8-10 min).

---

## 📝 Examples

### Example 1: Pre-Commit Workflow

```powershell
# Quick check before commit
.\scripts\quality-pipeline.ps1 -Quick

# Output:
# ✅ Compilation successful - no errors
# ✅ Code already properly formatted
# ✅ Clippy analysis passed - no issues found
# ✅ All 458 tests passed (12 ignored)
# ✅ Documentation generated without warnings
# 
# 🎉 OVERALL STATUS: ✅ PASSED
```

### Example 2: Weekly Full Analysis

```powershell
# Comprehensive quality review
.\scripts\quality-pipeline.ps1

# Output includes all checks:
# 📊 CODE QUALITY RESULTS:
#   Compilation: ✅ Clean
#   Formatting: ✅ Already formatted
#   Clippy Issues: 0
#   Tests: 458/458 passed
#   Coverage: 78.5% (avg across 10 missions)
#   Doc Warnings: 3
#
# 🔒 SECURITY & DEPENDENCIES:
#   Security Vulnerabilities: 0
#   Outdated Dependencies: 5
#   Unused Dependencies: 0
#   Dead Code Warnings: 12
```

### Example 3: Mission Coverage Focus

```powershell
# Analyze mission coverage in detail
.\scripts\quality-pipeline.ps1 -MissionsOnly

# Generates:
# reports/Mission_Coverage_<timestamp>.txt
# - Per-mission coverage percentages
# - Line coverage details
# - Recommendations based on thresholds
```

---

**Last Updated**: 2025-11-23  
**Script Version**: 2.0 (with extended quality checks)  
**Maintained By**: Rust Study Workspace Quality Team
