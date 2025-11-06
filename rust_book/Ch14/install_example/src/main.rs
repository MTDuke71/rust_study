//! 14.4 - Installing Binaries from Crates.io with cargo install
//!
//! This is an example binary crate that can be installed globally
//! using `cargo install install-example`.
//!
//! To make a crate installable:
//! 1. It must have at least one [[bin]] target
//! 2. The binary should be useful as a command-line tool
//! 3. Dependencies should be reasonable (large deps slow installation)

use clap::Parser;

/// A simple example CLI tool that can be installed with cargo install
#[derive(Parser, Debug)]
#[command(name = "install-example")]
#[command(about = "An example installable binary crate", long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value = "World")]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello, {}! (from install-example)", args.name);
    }
    
    println!("\nThis binary was installed using: cargo install install-example");
    println!("You can now use it from anywhere in your terminal!");
}

