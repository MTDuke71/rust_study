#!/usr/bin/env bash
# install_demo.sh - Demonstrates the complete custom cargo command lifecycle

echo "🦀 Custom Cargo Command Installation Demo"
echo "========================================"
echo

echo "📋 Step 1: Testing during development"
echo "cargo run -p cargo-example --bin cargo-example"
cargo run -p cargo-example --bin cargo-example

echo
echo "📋 Step 2: Testing with arguments during development"
echo "cargo run -p cargo-example --bin cargo-example -- --verbose demo --count 2"
cargo run -p cargo-example --bin cargo-example -- --verbose demo --count 2

echo
echo "📋 Step 3: Attempting to use as cargo subcommand (should fail)"
echo "cargo example"
if cargo example 2>/dev/null; then
    echo "✅ Already installed!"
else
    echo "❌ Not installed yet - this is expected"
fi

echo
echo "📋 Step 4: Installing the custom command"
echo "cd rust_book/Ch14/custom_command && cargo install --path ."
cd rust_book/Ch14/custom_command
cargo install --path .

echo
echo "📋 Step 5: Using the installed cargo subcommand"
echo "cargo example --verbose"
cargo example --verbose

echo
echo "📋 Step 6: Testing subcommands"
echo "cargo example demo --count 3"
cargo example demo --count 3

echo
echo "🎉 Installation demo complete!"
echo "To uninstall: cargo uninstall cargo-example"