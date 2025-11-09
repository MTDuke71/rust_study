# install_demo.ps1 - Demonstrates the complete custom cargo command lifecycle

Write-Host "🦀 Custom Cargo Command Installation Demo" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host

Write-Host "📋 Step 1: Testing during development" -ForegroundColor Yellow
Write-Host "cargo run -p cargo-example --bin cargo-example" -ForegroundColor Gray
cargo run -p cargo-example --bin cargo-example

Write-Host
Write-Host "📋 Step 2: Testing with arguments during development" -ForegroundColor Yellow
Write-Host "cargo run -p cargo-example --bin cargo-example -- --verbose demo --count 2" -ForegroundColor Gray
cargo run -p cargo-example --bin cargo-example -- --verbose demo --count 2

Write-Host
Write-Host "📋 Step 3: Attempting to use as cargo subcommand (should fail)" -ForegroundColor Yellow
Write-Host "cargo example" -ForegroundColor Gray
cargo example 2>$null
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Already installed!" -ForegroundColor Green
} else {
    Write-Host "❌ Not installed yet - this is expected" -ForegroundColor Red
}

Write-Host
Write-Host "📋 Step 4: Installing the custom command" -ForegroundColor Yellow
Write-Host "cd rust_book/Ch14/custom_command && cargo install --path ." -ForegroundColor Gray
Set-Location rust_book/Ch14/custom_command
cargo install --path .

Write-Host
Write-Host "📋 Step 5: Using the installed cargo subcommand" -ForegroundColor Yellow
Write-Host "cargo example --verbose" -ForegroundColor Gray
cargo example --verbose

Write-Host
Write-Host "📋 Step 6: Testing subcommands" -ForegroundColor Yellow
Write-Host "cargo example demo --count 3" -ForegroundColor Gray
cargo example demo --count 3

Write-Host
Write-Host "🎉 Installation demo complete!" -ForegroundColor Green
Write-Host "To uninstall: cargo uninstall cargo-example" -ForegroundColor Gray