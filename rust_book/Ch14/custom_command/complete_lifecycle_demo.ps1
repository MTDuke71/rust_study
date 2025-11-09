# complete_lifecycle_demo.ps1 - Full demonstration of custom cargo command lifecycle

Write-Host "🔄 Complete Custom Cargo Command Lifecycle Demo" -ForegroundColor Cyan
Write-Host "===============================================" -ForegroundColor Cyan
Write-Host "This demo shows: Development → Install → Use → Uninstall → Reinstall" -ForegroundColor White
Write-Host

# Ensure we start from a clean state
Write-Host "🧹 Cleaning up any existing installation..." -ForegroundColor Yellow
cargo uninstall cargo-example 2>$null
Write-Host

Write-Host "🔵 Phase 1: Development Testing" -ForegroundColor Blue
Write-Host "===============================" -ForegroundColor Blue
Write-Host "Testing the command during development (before installation)" -ForegroundColor Gray
Write-Host
cargo run -p cargo-example --bin cargo-example -- --verbose

Write-Host
Write-Host "🔵 Phase 2: Installation" -ForegroundColor Blue
Write-Host "========================" -ForegroundColor Blue
Write-Host "Installing the custom cargo command" -ForegroundColor Gray
Write-Host
Set-Location rust_book\Ch14\custom_command
cargo install --path .
Set-Location ..\..\..

Write-Host
Write-Host "🔵 Phase 3: Usage as Cargo Subcommand" -ForegroundColor Blue
Write-Host "=====================================" -ForegroundColor Blue
Write-Host "Now we can use it as 'cargo example'" -ForegroundColor Gray
Write-Host
cargo example --verbose
Write-Host
cargo example demo --count 2

Write-Host
Write-Host "🔵 Phase 4: Uninstallation" -ForegroundColor Blue
Write-Host "==========================" -ForegroundColor Blue
Write-Host "Removing the installed command" -ForegroundColor Gray
Write-Host
cargo uninstall cargo-example

Write-Host
Write-Host "🔵 Phase 5: Verification of Uninstall" -ForegroundColor Blue
Write-Host "=====================================" -ForegroundColor Blue
Write-Host "Confirming cargo example no longer works" -ForegroundColor Gray
Write-Host
cargo example 2>$null
if ($LASTEXITCODE -eq 0) {
    Write-Host "❌ Uninstall failed" -ForegroundColor Red
} else {
    Write-Host "✅ Successfully uninstalled - 'cargo example' no longer works" -ForegroundColor Green
}

Write-Host
Write-Host "🔵 Phase 6: Development Mode Still Works" -ForegroundColor Blue
Write-Host "=======================================" -ForegroundColor Blue
Write-Host "Development mode unaffected by uninstall" -ForegroundColor Gray
Write-Host
cargo run -p cargo-example --bin cargo-example

Write-Host
Write-Host "🔵 Phase 7: Reinstallation" -ForegroundColor Blue
Write-Host "==========================" -ForegroundColor Blue
Write-Host "Installing again to show repeatability" -ForegroundColor Gray
Write-Host
Set-Location rust_book\Ch14\custom_command
cargo install --path .
Set-Location ..\..\..

Write-Host
Write-Host "🔵 Phase 8: Final Verification" -ForegroundColor Blue
Write-Host "==============================" -ForegroundColor Blue
Write-Host "Confirming reinstallation worked" -ForegroundColor Gray
Write-Host
cargo example "🎉 Reinstallation successful! Chapter 14.5 complete!"

Write-Host
Write-Host "🎉 Complete Lifecycle Demo Finished!" -ForegroundColor Green
Write-Host "====================================" -ForegroundColor Green
Write-Host "Summary of what we demonstrated:" -ForegroundColor Cyan
Write-Host "  ✅ Development testing with cargo run" -ForegroundColor Gray
Write-Host "  ✅ Installation with cargo install --path ." -ForegroundColor Gray
Write-Host "  ✅ Usage as cargo subcommand" -ForegroundColor Gray
Write-Host "  ✅ Uninstallation with cargo uninstall" -ForegroundColor Gray
Write-Host "  ✅ Development mode unaffected by install/uninstall" -ForegroundColor Gray
Write-Host "  ✅ Repeatability of installation process" -ForegroundColor Gray
Write-Host
Write-Host "This demonstrates the complete custom cargo command ecosystem!" -ForegroundColor Yellow