// Rust for Rustaceans - Chapter 2: Types
// Library module providing common utilities for examples

pub mod utils {
    use std::mem;

    /// Display size and alignment information for a type
    pub fn show_type_info<T>(type_name: &str) {
        println!(
            "{:<25} size: {:>3} bytes, align: {:>2} bytes",
            type_name,
            mem::size_of::<T>(),
            mem::align_of::<T>()
        );
    }

    /// Print a section header
    pub fn print_section(title: &str) {
        println!("\n{}", "=".repeat(70));
        println!("{}", title);
        println!("{}", "=".repeat(70));
    }

    /// Print a subsection header
    pub fn print_subsection(title: &str) {
        println!("\n--- {} ---", title);
    }
}
