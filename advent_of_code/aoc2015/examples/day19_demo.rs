/// Day 19: Medicine for Rudolph - Molecular Replacement Demo
///
/// This example demonstrates the molecular replacement system used in Day 19.
/// It shows how molecules can be transformed using replacement rules.
use aoc2015::solver::day19;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Day 19: Medicine for Rudolph Demo ===\n");

    // Example from the problem description
    let simple_example = "H => HO\nH => OH\nO => HH\n\nHOH";

    println!("🧪 Simple Example:");
    println!("Rules:");
    println!("  H => HO");
    println!("  H => OH");
    println!("  O => HH");
    println!("Starting molecule: HOH\n");

    // Parse and show results
    let part1 = day19::solve_part1(simple_example)?;
    let part2 = match day19::solve_part2(simple_example) {
        Ok(steps) => format!("{} steps", steps),
        Err(_) => "N/A (no path from 'e' to HOH with given rules)".to_string(),
    };

    println!("📊 Results:");
    println!("  Part 1 (distinct molecules after 1 step): {}", part1);
    println!("  Part 2 (min steps to create from 'e'): {}", part2);

    // Show the actual molecules that can be generated
    println!("\n🔬 Detailed Analysis:");
    println!("From HOH, we can generate:");

    // Manually demonstrate the transformations
    let molecules = vec![
        ("HOOH", "H=>HO on first H (position 0)"),
        ("HOHO", "H=>HO on second H (position 2)"),
        ("OHOH", "H=>OH on first H (position 0)"),
        ("HOOH", "H=>OH on second H (position 2) - duplicate"),
        ("HHHH", "O=>HH on O (position 1)"),
    ];

    let mut unique_molecules = HashSet::new();
    for (molecule, description) in molecules {
        unique_molecules.insert(molecule);
        println!("  {} via {}", molecule, description);
    }

    println!(
        "\n🎯 Unique molecules: {} (HOOH appears twice, so 4 distinct)",
        unique_molecules.len()
    );

    // Another example
    println!("\n🧪 Complex Example (HOHOHO):");
    let complex_example = "H => HO\nH => OH\nO => HH\n\nHOHOHO";

    if let Ok(result) = day19::solve_part1(complex_example) {
        println!("HOHOHO can become {} distinct molecules", result);
        println!("(6 replacements possible on H atoms + 3 on O atoms = 9 total,");
        println!(" but some produce duplicate molecules, leaving 7 unique)");
    }

    // Demonstrate replacement without regard for context
    println!("\n🧪 Context-Independent Replacement:");
    let context_example = "H => OO\n\nH2O";
    println!("Rule: H => OO");
    println!("Molecule: H2O");

    if let Ok(result) = day19::solve_part1(context_example) {
        println!("Result: {} distinct molecule(s)", result);
        println!("H2O becomes OO2O (replacement ignores surrounding characters)");
    }

    println!("\n✨ Key Insights:");
    println!("• Replacements can occur at any position where the pattern matches");
    println!("• Multiple replacements of the same rule at different positions count as different");
    println!("• Duplicate results are counted only once");
    println!(
        "• Part 2 typically involves finding the minimum steps to build the molecule from 'e'"
    );

    Ok(())
}
