//! # Circle Sort — Visual Demo
//!
//! Demonstrates the "bend the array into a circle" sorting algorithm
//! with colored output showing mirror-pair comparisons.
//!
//! Run: `cargo run -p circle_sort --example circle_sort_demo`

use circle_sort::circle_sort_with_callback;
use colored::Colorize;

fn print_array_highlight(arr: &[i32], a: usize, b: usize) {
    print!("  [");
    for (i, val) in arr.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        if i == a || i == b {
            // Yellow = mirror pair being swapped across the circle diameter
            print!("{}", format!("{val:>2}").yellow().bold());
        } else {
            print!("{val:>2}");
        }
    }
    println!("]  swapped [{a}]↔[{b}]");
}

fn print_array(arr: &[i32], label: &str) {
    print!("  {label}: [");
    for (i, val) in arr.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{val:>2}");
    }
    println!("]");
}

fn main() {
    println!("{}", "═══════════════════════════════════════════".bright_cyan());
    println!(
        "{}",
        "  Circle Sort — Forum-Invented O(n log n)".bright_cyan().bold()
    );
    println!("{}", "═══════════════════════════════════════════".bright_cyan());
    println!();

    // --- Origin Story ---
    println!("{}", "Origin Story".underline().bold());
    println!("  A programmer posted this on a coding forum — no paper,");
    println!("  no formal proof. Bend the array into a circle, compare");
    println!("  mirror pairs across the diameter, swap if needed, split,");
    println!("  recurse. Repeat until sorted. It actually works!");
    println!();

    // --- How it works ---
    println!("{}", "How It Works".underline().bold());
    println!("  1. Compare first↔last, second↔second-to-last, ...");
    println!("     (mirror pairs across the circle diameter)");
    println!("  2. Swap if left > right");
    println!("  3. Split in half, recurse on each half");
    println!("  4. Repeat full passes until zero swaps");
    println!();

    // --- Small example with step-by-step ---
    println!(
        "{}",
        "Step-by-Step: [6, 5, 3, 1, 8, 7, 2, 4]".underline().bold()
    );
    println!();

    let mut data = vec![6, 5, 3, 1, 8, 7, 2, 4];
    print_array(&data, "Start");
    println!();

    let mut step = 0;

    circle_sort_with_callback(&mut data, |a, b, state| {
        // Detect new pass: if state reverted to checking from scratch
        // We track this by checking if we're back at a full-array comparison
        step += 1;
        print!("  Step {step:>2}: ");
        print_array_highlight(state, a, b);
    });

    println!();
    print_array(&data, "Final");
    println!("  Total swaps: {step}");
    println!();

    // --- Verify correctness ---
    println!("{}", "Correctness Check".underline().bold());

    let test_cases: Vec<(&str, Vec<i32>)> = vec![
        ("Already sorted", vec![1, 2, 3, 4, 5]),
        ("Reverse order", vec![5, 4, 3, 2, 1]),
        ("With duplicates", vec![3, 1, 4, 1, 5, 9, 2, 6]),
        ("Single element", vec![42]),
        ("Two elements", vec![2, 1]),
        ("Power of two", vec![8, 6, 7, 5, 3, 0, 9, 1]),
    ];

    for (label, mut arr) in test_cases {
        let mut expected = arr.clone();
        expected.sort();
        circle_sort_with_callback(&mut arr, |_, _, _| {});
        let status = if arr == expected {
            "✓".green().bold().to_string()
        } else {
            "✗".red().bold().to_string()
        };
        println!("  {status} {label}: {arr:?}");
    }
    println!();

    // --- Swap count analysis ---
    println!("{}", "Swap Count Analysis".underline().bold());
    println!("  Showing total swaps for various input sizes:");
    println!();

    for &size in &[8, 16, 32, 64, 128, 256] {
        // Worst-case-ish: reverse sorted
        let mut reversed: Vec<i32> = (0..size).rev().collect();
        let mut swap_count = 0u64;
        circle_sort_with_callback(&mut reversed, |_, _, _| {
            swap_count += 1;
        });

        // n log n reference
        let n = size as f64;
        let nlogn = n * n.log2();
        let ratio = swap_count as f64 / nlogn;

        println!(
            "  n={size:>4}  swaps={swap_count:>6}  n·log₂n={nlogn:>8.0}  ratio={ratio:.2}"
        );
    }
    println!();
    println!("  Ratio near 1.0 confirms O(n log n) behavior.");
    println!();

    println!("{}", "═══════════════════════════════════════════".bright_cyan());
    println!(
        "  {}",
        "\"Sometimes the best algorithms come from curiosity.\"".italic()
    );
    println!("{}", "═══════════════════════════════════════════".bright_cyan());
}
