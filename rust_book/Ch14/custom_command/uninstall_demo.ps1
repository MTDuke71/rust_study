# uninstall_demo.ps1 - Demonstrates uninstalling the custom cargo command

Write-Host "🗑️ Custom Cargo Command Uninstall Demo" -ForegroundColor Cyan
Write-Host "=======================================" -ForegroundColor Cyan
Write-Host

Write-Host "📋 Step 1: Verifying current installation" -ForegroundColor Yellow
Write-Host "cargo example --version" -ForegroundColor Gray
cargo example version
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ cargo-example is currently installed" -ForegroundColor Green
} else {
    Write-Host "❌ cargo-example is not installed" -ForegroundColor Red
    Write-Host "Nothing to uninstall. Run install_demo.ps1 first." -ForegroundColor Yellow
    exit 0
}

Write-Host
Write-Host "📋 Step 2: Checking installation location" -ForegroundColor Yellow
$cargoExamplePath = Get-Command cargo-example -ErrorAction SilentlyContinue
if ($cargoExamplePath) {
    Write-Host "📍 Installed at: $($cargoExamplePath.Source)" -ForegroundColor Gray
} else {
    Write-Host "❓ Could not determine installation path" -ForegroundColor Yellow
}

Write-Host
Write-Host "📋 Step 3: Uninstalling cargo-example" -ForegroundColor Yellow
Write-Host "cargo uninstall cargo-example" -ForegroundColor Gray
cargo uninstall cargo-example

Write-Host
Write-Host "📋 Step 4: Verifying uninstallation" -ForegroundColor Yellow
Write-Host "cargo example (should fail now)" -ForegroundColor Gray
cargo example 2>$null
if ($LASTEXITCODE -eq 0) {
    Write-Host "❌ Uninstall failed - command still works" -ForegroundColor Red
} else {
    Write-Host "✅ Successfully uninstalled - command no longer available" -ForegroundColor Green
}

Write-Host
Write-Host "📋 Step 5: Confirming binary removal" -ForegroundColor Yellow
$cargoExamplePath = Get-Command cargo-example -ErrorAction SilentlyContinue
if ($cargoExamplePath) {
    Write-Host "❌ Binary still exists at: $($cargoExamplePath.Source)" -ForegroundColor Red
} else {
    Write-Host "✅ Binary successfully removed from PATH" -ForegroundColor Green
}

Write-Host
Write-Host "📋 Step 6: Development mode still works" -ForegroundColor Yellow
Write-Host "cargo run -p cargo-example --bin cargo-example (should still work)" -ForegroundColor Gray
Set-Location ..\..\..
cargo run -p cargo-example --bin cargo-example
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Development mode unaffected by uninstall" -ForegroundColor Green
} else {
    Write-Host "❌ Development mode broken" -ForegroundColor Red
}

Write-Host
Write-Host "🎉 Uninstall demo complete!" -ForegroundColor Green
Write-Host "Key points:" -ForegroundColor Cyan
Write-Host "  • cargo uninstall removes the installed binary" -ForegroundColor Gray
Write-Host "  • Development mode (cargo run) still works" -ForegroundColor Gray
Write-Host "  • To reinstall: run install_demo.ps1" -ForegroundColor Gray