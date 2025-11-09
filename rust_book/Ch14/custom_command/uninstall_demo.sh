#!/usr/bin/env bash
# uninstall_demo.sh - Demonstrates uninstalling the custom cargo command

echo "🗑️ Custom Cargo Command Uninstall Demo"
echo "======================================="
echo

echo "📋 Step 1: Verifying current installation"
echo "cargo example version"
if cargo example version 2>/dev/null; then
    echo "✅ cargo-example is currently installed"
else
    echo "❌ cargo-example is not installed"
    echo "Nothing to uninstall. Run install_demo.sh first."
    exit 0
fi

echo
echo "📋 Step 2: Checking installation location"
CARGO_EXAMPLE_PATH=$(which cargo-example 2>/dev/null)
if [ -n "$CARGO_EXAMPLE_PATH" ]; then
    echo "📍 Installed at: $CARGO_EXAMPLE_PATH"
else
    echo "❓ Could not determine installation path"
fi

echo
echo "📋 Step 3: Uninstalling cargo-example"
echo "cargo uninstall cargo-example"
cargo uninstall cargo-example

echo
echo "📋 Step 4: Verifying uninstallation"
echo "cargo example (should fail now)"
if cargo example 2>/dev/null; then
    echo "❌ Uninstall failed - command still works"
else
    echo "✅ Successfully uninstalled - command no longer available"
fi

echo
echo "📋 Step 5: Confirming binary removal"
CARGO_EXAMPLE_PATH=$(which cargo-example 2>/dev/null)
if [ -n "$CARGO_EXAMPLE_PATH" ]; then
    echo "❌ Binary still exists at: $CARGO_EXAMPLE_PATH"
else
    echo "✅ Binary successfully removed from PATH"
fi

echo
echo "📋 Step 6: Development mode still works"
echo "cargo run -p cargo-example --bin cargo-example (should still work)"
cd ../../..
if cargo run -p cargo-example --bin cargo-example; then
    echo "✅ Development mode unaffected by uninstall"
else
    echo "❌ Development mode broken"
fi

echo
echo "🎉 Uninstall demo complete!"
echo "Key points:"
echo "  • cargo uninstall removes the installed binary"
echo "  • Development mode (cargo run) still works"
echo "  • To reinstall: run install_demo.sh"