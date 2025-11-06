//! 14.5 - Extending Cargo with Custom Commands
//!
//! This example demonstrates how to create a custom Cargo command.
//!
//! Custom Cargo commands follow the naming convention: cargo-<name>
//! When installed, they become available as: cargo <name>
//!
//! Installation:
//!   cargo install --path .
//!
//! Usage:
//!   cargo example
//!   cargo example --help

use clap::{Parser, Subcommand};

/// A custom Cargo command example
///
/// This command demonstrates how to extend Cargo's functionality.
/// After installation, use it with: cargo example
#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
#[command(about = "A custom Cargo command example", long_about = None)]
enum Cargo {
    /// Custom example command
    Example(ExampleArgs),
}

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
    let Cargo::Example(args) = Cargo::parse();

    if args.verbose {
        println!("Verbose mode enabled");
    }

    match &args.command {
        Some(ExampleCommand::Demo { count }) => {
            for i in 0..*count {
                println!("Demo run {}: {}", i + 1, args.message);
            }
        }
        Some(ExampleCommand::Version) => {
            println!("cargo-example version 0.1.0");
            println!("Custom Cargo command example");
        }
        None => {
            println!("{}", args.message);
            println!("\nTry: cargo example --help for more options");
        }
    }
}

