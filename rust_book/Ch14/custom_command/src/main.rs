//! 14.5 - Extending Cargo with Custom Commands
//!
//! This example demonstrates how to create a custom Cargo command.
//!
//! Custom Cargo commands follow the naming convention: cargo-<name>
//! When installed, they become available as: cargo <name>
//!
//! ## Testing during development:
//!   cargo run -p cargo-example --bin cargo-example
//!   cargo run -p cargo-example --bin cargo-example -- --help
//!   cargo run -p cargo-example --bin cargo-example -- demo --count 3
//!
//! ## To install locally and use as `cargo example`:
//!   cd rust_book/Ch14/custom_command
//!   cargo install --path .
//!   cargo example
//!   cargo example --help
//!
//! ## To uninstall:
//!   cargo uninstall cargo-example
//!
//! Note: The `cargo example` command only works AFTER installation.
//! During development, use `cargo run` as shown above.

use clap::{Parser, Subcommand};
use std::env;

/// A custom Cargo command example
///
/// This command demonstrates how to extend Cargo's functionality.
/// After installation, use it with: cargo example
#[derive(Parser)]
#[command(about = "An example custom Cargo command")]
struct ExampleArgs {
    /// Print verbose output
    #[arg(short, long)]
    verbose: bool,

    /// The message to print
    #[arg(default_value = "Hello from custom Cargo command!")]
    message: String,

    #[command(subcommand)]
    command: Option<ExampleCommand>,
}

#[derive(Subcommand)]
enum ExampleCommand {
    /// Run a demo
    Demo {
        /// Number of times to run
        #[arg(short, long, default_value_t = 1)]
        count: u32,
    },
    /// Show version info
    Version,
}

fn main() {
    // Detect how we're being called
    let args: Vec<String> = env::args().collect();
    let called_via_cargo = args.len() > 1 && args[1] == "example";
    
    // Skip the first argument if it's "example" (when called via cargo)
    let args_to_parse = if called_via_cargo {
        let mut modified_args = args;
        modified_args.remove(1);
        modified_args
    } else {
        args
    };
    
    let parsed_args = ExampleArgs::parse_from(args_to_parse);

    // Show how we were invoked
    if parsed_args.verbose {
        println!("Verbose mode enabled");
        if called_via_cargo {
            println!("📦 Called via: cargo example");
            println!("🎉 Custom Cargo command is working!");
        } else {
            println!("🔧 Called directly as: cargo-example");
            println!("💡 To use as 'cargo example', install first:");
            println!("   cd rust_book/Ch14/custom_command");
            println!("   cargo install --path .");
        }
        println!();
    }

    match &parsed_args.command {
        Some(ExampleCommand::Demo { count }) => {
            for i in 0..*count {
                println!("Demo run {}: {}", i + 1, parsed_args.message);
            }
        }
        Some(ExampleCommand::Version) => {
            println!("cargo-example version 0.1.0");
            println!("Custom Cargo command example from Rust Book Chapter 14.5");
            if !called_via_cargo {
                println!("\n💡 Install with: cargo install --path .");
                println!("   Then use as: cargo example version");
            }
        }
        None => {
            println!("{}", parsed_args.message);
            if called_via_cargo {
                println!("\n🎉 Success! Custom cargo command is working.");
                println!("Try: cargo example demo --count 3");
            } else {
                println!("\n🔧 Running in development mode.");
                println!("Try: cargo run -p cargo-example --bin cargo-example -- demo --count 3");
                println!("Or install first, then use: cargo example");
            }
        }
    }
}

