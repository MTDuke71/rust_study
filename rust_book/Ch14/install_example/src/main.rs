//! # 14.4 - Installing Binaries from Crates.io with cargo install
//!
//! This example demonstrates how to INSTALL and USE binary crates from crates.io.
//! The focus is on the CONSUMER side - how to install existing tools.
//!
//! ## Installation Process:
//! 1. `cargo install <crate-name>` - downloads, compiles, and installs
//! 2. Binary is installed to `~/.cargo/bin/`
//! 3. Make sure `~/.cargo/bin` is in your PATH
//! 4. Use the tool from anywhere in your terminal
//!
//! ## Popular Binary Crates to Try:
//! - ripgrep: `cargo install ripgrep` → use with `rg pattern files/`
//! - fd-find: `cargo install fd-find` → use with `fd filename`
//! - bat: `cargo install bat` → use with `bat file.rs`
//! - exa: `cargo install exa` → use with `exa -la`

use std::process::Command;

fn main() {
    println!("🦀 Rust Binary Installation Demo - Chapter 14.4");
    println!("================================================");
    
    println!("\n📦 How to install binary crates from crates.io:");
    println!("1. cargo install ripgrep     # Fast text search");
    println!("2. cargo install fd-find     # Fast file finder");
    println!("3. cargo install bat         # Better cat with syntax highlighting");
    println!("4. cargo install exa         # Better ls with colors");
    println!("5. cargo install tokei       # Code statistics");
    
    println!("\n🔧 After installation, use them anywhere:");
    println!("- rg \"pattern\" files/       # Search text with ripgrep");
    println!("- fd filename               # Find files with fd");
    println!("- bat src/main.rs           # Display file with syntax highlighting");
    println!("- exa -la                   # List files with colors and git info");
    
    println!("\n📍 Installation location: ~/.cargo/bin/");
    println!("💡 Make sure ~/.cargo/bin is in your PATH!");
    
    // Check if some popular tools are already installed
    println!("\n🔍 Checking for installed Rust tools...");
    
    let tools = [
        ("ripgrep", "rg"),
        ("fd-find", "fd"), 
        ("bat", "bat"),
        ("exa", "exa"),
        ("cargo-watch", "cargo-watch"),
    ];
    
    for (crate_name, command) in &tools {
        match Command::new(command).arg("--version").output() {
            Ok(output) if output.status.success() => {
                let version = String::from_utf8_lossy(&output.stdout);
                println!("✅ {} is installed: {}", crate_name, version.lines().next().unwrap_or(""));
            }
            _ => {
                println!("❌ {} not found - install with: cargo install {}", crate_name, crate_name);
            }
        }
    }
    
    println!("\n🎯 This demonstrates the cargo install ecosystem!");
    println!("Many popular Rust CLI tools are distributed this way.");
    println!("\nTry installing ripgrep: cargo install ripgrep");
    println!("Then search files: rg \"fn main\" src/");
}

