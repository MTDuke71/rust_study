# PowerShell script to extract and run Rust code from markdown files
# Usage: .\run_markdown_code.ps1 daily_study\rust_learning_week2_notes\Day11.md

param(
    [Parameter(Mandatory=$true)]
    [string]$MarkdownFile
)

# Check if file exists
if (-not (Test-Path $MarkdownFile)) {
    Write-Error "File $MarkdownFile not found!"
    exit 1
}

# Extract the complete runnable example (between "Complete Runnable Example" and end of code block)
$content = Get-Content $MarkdownFile -Raw
$pattern = '(?s)## 🚀 \*\*Complete Runnable Example\*\*.*?```rust\s*(.*?)```'

if ($content -match $pattern) {
    $rustCode = $matches[1]
    
    # Create temporary .rs file
    $tempFile = "temp_markdown_code.rs"
    Set-Content -Path $tempFile -Value $rustCode
    
    Write-Host "🦀 Running code from $MarkdownFile..." -ForegroundColor Green
    Write-Host "=" * 50 -ForegroundColor Yellow
    
    # Compile and run
    rustc $tempFile -o temp_markdown_code.exe
    if ($LASTEXITCODE -eq 0) {
        ./temp_markdown_code.exe
        
        # Cleanup
        Remove-Item $tempFile -ErrorAction SilentlyContinue
        Remove-Item "temp_markdown_code.exe" -ErrorAction SilentlyContinue
        Remove-Item "temp_markdown_code.pdb" -ErrorAction SilentlyContinue
    } else {
        Write-Error "Compilation failed!"
        Remove-Item $tempFile -ErrorAction SilentlyContinue
    }
} else {
    Write-Warning "No 'Complete Runnable Example' section found in $MarkdownFile"
    Write-Host "Looking for any rust code blocks..." -ForegroundColor Yellow
    
    # Alternative: extract first main function found
    $codePattern = '(?s)```rust\s*(use.*?fn main.*?}.*?)```'
    if ($content -match $codePattern) {
        $rustCode = $matches[1]
        $tempFile = "temp_markdown_code.rs"
        Set-Content -Path $tempFile -Value $rustCode
        
        Write-Host "Found main function, running..." -ForegroundColor Green
        rustc $tempFile -o temp_markdown_code.exe
        if ($LASTEXITCODE -eq 0) {
            ./temp_markdown_code.exe
            Remove-Item $tempFile -ErrorAction SilentlyContinue
            Remove-Item "temp_markdown_code.exe" -ErrorAction SilentlyContinue
            Remove-Item "temp_markdown_code.pdb" -ErrorAction SilentlyContinue
        }
    } else {
        Write-Warning "No runnable Rust code found in $MarkdownFile"
    }
}