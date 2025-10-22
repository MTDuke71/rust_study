//! # minigrep - Command-Line Text Search Tool
//!
//! A simple implementation of the `grep` command demonstrating Rust best practices.
//!
//! ## Usage
//! ```bash
//! cargo run -- <query> <filename>
//! CASE_INSENSITIVE=1 cargo run -- <query> <filename>
//! ```
//!
//! ## Examples
//! ```bash
//! cargo run -- "Rust" poem.txt
//! CASE_INSENSITIVE=1 cargo run -- "rust" poem.txt
//! cargo run -- "the" poem.txt 2> errors.txt
//! ```

use std::env;
use std::process;

fn main() {
    let config = minigrep::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
