/// Day 13 Seating Visualization Demo
/// 
/// This example demonstrates the seating optimization algorithm by:
/// 1. Parsing happiness relationships between people
/// 2. Finding the optimal (maximum happiness) arrangement  
/// 3. Finding the worst (minimum happiness) arrangement
/// 4. Displaying both arrangements visually around a circular table
/// 
/// Usage: cargo run --example day13_seating_visualization
use aoc2015::solver::day13::*;

fn main() {
    println!("🎄 Day 13: Knights of the Dinner Table - Seating Visualization");
    println!("================================================================");
    
    // Example 1: Simple 4-person example
    demonstrate_simple_example();
    
    // Example 2: Complex example with more people
    demonstrate_complex_example();
    
    // Example 3: Adding yourself (Part 2)
    demonstrate_part2_example();
}

fn demonstrate_simple_example() {
    println!("\n🪑 **Example 1: Simple 4-Person Dinner Party**");
    println!("==============================================");
    
    let input = r#"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."#;

    let graph = parse_happiness_graph(input).unwrap();
    
    // Show the happiness matrix
    print_happiness_matrix(&graph);
    
    // Find optimal and worst arrangements
    let (best_happiness, best_arrangement) = find_optimal_seating(&graph);
    let (worst_happiness, worst_arrangement) = find_worst_seating(&graph);
    
    println!("\n📊 **Results:**");
    println!("  🏆 Best arrangement happiness: {}", best_happiness);
    println!("  💀 Worst arrangement happiness: {}", worst_happiness);
    println!("  📈 Difference: {} happiness units", best_happiness - worst_happiness);
    
    // Visualize both arrangements
    println!("\n🏆 **OPTIMAL SEATING ARRANGEMENT**");
    visualize_seating_arrangement(&graph, &best_arrangement, best_happiness);
    
    println!("\n💀 **WORST SEATING ARRANGEMENT**");
    visualize_seating_arrangement(&graph, &worst_arrangement, worst_happiness);
}

fn demonstrate_complex_example() {
    println!("\n\n🍽️ **Example 2: Larger Dinner Party (6 People)**");
    println!("===============================================");
    
    let input = r#"Alice would gain 2 happiness units by sitting next to Bob.
Alice would gain 26 happiness units by sitting next to Carol.
Alice would lose 82 happiness units by sitting next to David.
Alice would lose 75 happiness units by sitting next to Eric.
Alice would gain 42 happiness units by sitting next to Frank.
Bob would gain 40 happiness units by sitting next to Alice.
Bob would lose 61 happiness units by sitting next to Carol.
Bob would lose 15 happiness units by sitting next to David.
Bob would gain 63 happiness units by sitting next to Eric.
Bob would gain 41 happiness units by sitting next to Frank.
Carol would lose 35 happiness units by sitting next to Alice.
Carol would lose 99 happiness units by sitting next to Bob.
Carol would gain 82 happiness units by sitting next to David.
Carol would lose 46 happiness units by sitting next to Eric.
Carol would gain 27 happiness units by sitting next to Frank.
David would gain 92 happiness units by sitting next to Alice.
David would gain 49 happiness units by sitting next to Bob.
David would lose 37 happiness units by sitting next to Carol.
David would lose 46 happiness units by sitting next to Eric.
David would lose 93 happiness units by sitting next to Frank.
Eric would lose 98 happiness units by sitting next to Alice.
Eric would lose 6 happiness units by sitting next to Bob.
Eric would gain 95 happiness units by sitting next to Carol.
Eric would gain 40 happiness units by sitting next to David.
Eric would gain 18 happiness units by sitting next to Frank.
Frank would lose 69 happiness units by sitting next to Alice.
Frank would gain 57 happiness units by sitting next to Bob.
Frank would lose 85 happiness units by sitting next to Carol.
Frank would gain 45 happiness units by sitting next to David.
Frank would lose 15 happiness units by sitting next to Eric."#;

    let graph = parse_happiness_graph(input).unwrap();
    
    let (best_happiness, best_arrangement) = find_optimal_seating(&graph);
    let (worst_happiness, _worst_arrangement) = find_worst_seating(&graph);
    
    println!("📊 **Results for {} people:**", graph.people.len());
    println!("  🏆 Best arrangement happiness: {}", best_happiness);
    println!("  💀 Worst arrangement happiness: {}", worst_happiness);
    println!("  📈 Difference: {} happiness units", best_happiness - worst_happiness);
    println!("  🔢 Total permutations analyzed: {}", factorial(graph.people.len()));
    
    println!("\n🏆 **OPTIMAL ARRANGEMENT (Top 6 People)**");
    visualize_seating_arrangement(&graph, &best_arrangement, best_happiness);
}

fn demonstrate_part2_example() {
    println!("\n\n👤 **Example 3: Adding Yourself (Part 2)**");
    println!("========================================");
    
    let input = r#"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob."#;

    // Part 1: Original arrangement
    let graph1 = parse_happiness_graph(input).unwrap();
    let (best1, arrangement1) = find_optimal_seating(&graph1);
    
    // Part 2: Add yourself
    let mut graph2 = parse_happiness_graph(input).unwrap();
    graph2.add_neutral_person("You");
    let (best2, arrangement2) = find_optimal_seating(&graph2);
    
    println!("📊 **Comparison:**");
    println!("  🥇 Part 1 (without you): {} happiness", best1);
    println!("  🥈 Part 2 (with you):    {} happiness", best2);
    println!("  📉 Impact of adding you: {} happiness", best2 - best1);
    
    println!("\n🥇 **PART 1: Original Optimal Arrangement**");
    visualize_seating_arrangement(&graph1, &arrangement1, best1);
    
    println!("\n🥈 **PART 2: Optimal Arrangement (With You)**");
    visualize_seating_arrangement(&graph2, &arrangement2, best2);
}

// ============================================================================
// Visualization Functions
// ============================================================================

/// Prints a happiness matrix showing relationships between all people
fn print_happiness_matrix(graph: &HappinessGraph) {
    let people: Vec<String> = graph.get_people();
    
    println!("\n💝 **Happiness Matrix** (row = person, column = sitting next to):");
    
    // Header row
    print!("        ");
    for person in &people {
        print!("{:>8}", person);
    }
    println!();
    
    // Data rows
    for person1 in &people {
        print!("{:>8}", person1);
        for person2 in &people {
            if person1 == person2 {
                print!("      --");
            } else {
                let happiness = graph.get_happiness(person1, person2);
                print!("{:>8}", happiness);
            }
        }
        println!();
    }
}

/// Visualizes a seating arrangement around a circular table
fn visualize_seating_arrangement(graph: &HappinessGraph, arrangement: &[String], total_happiness: i32) {
    if arrangement.is_empty() {
        println!("  (No people to seat)");
        return;
    }
    
    println!("  🎯 Total Happiness: {}", total_happiness);
    println!("  👥 Seating Order: {}", arrangement.join(" → "));
    
    // Draw the circular table
    draw_circular_table(arrangement);
    
    // Show individual happiness contributions
    println!("\n  📋 **Individual Happiness Contributions:**");
    for (i, person) in arrangement.iter().enumerate() {
        let left_neighbor = &arrangement[(i + arrangement.len() - 1) % arrangement.len()];
        let right_neighbor = &arrangement[(i + 1) % arrangement.len()];
        
        let left_happiness = graph.get_happiness(person, left_neighbor);
        let right_happiness = graph.get_happiness(person, right_neighbor);
        let person_total = left_happiness + right_happiness;
        
        println!("    {} → {} (left: {:+3}) + {} (right: {:+3}) = {:+3}", 
                 person, left_neighbor, left_happiness, right_neighbor, right_happiness, person_total);
    }
}

/// Draws a visual representation of people sitting around a circular table
fn draw_circular_table(arrangement: &[String]) {
    let n = arrangement.len();
    
    match n {
        2 => draw_table_2_people(arrangement),
        3 => draw_table_3_people(arrangement),
        4 => draw_table_4_people(arrangement),
        5 => draw_table_5_people(arrangement),
        6 => draw_table_6_people(arrangement),
        _ => draw_table_many_people(arrangement),
    }
}

fn draw_table_2_people(arrangement: &[String]) {
    println!("\n      ┌─────────────────┐");
    println!("      │                 │");
    println!("   {:>8}  🪑       🪑  {:<8}", arrangement[0], arrangement[1]);
    println!("      │                 │");
    println!("      └─────────────────┘");
}

fn draw_table_3_people(arrangement: &[String]) {
    println!("\n         {}          ", arrangement[0]);
    println!("           🪑           ");
    println!("       ╭─────────╮      ");
    println!("      ╱           ╲     ");
    println!("    🪑               🪑  ");
    println!("  {}              {}", arrangement[2], arrangement[1]);
    println!("      ╲           ╱     ");
    println!("       ╰─────────╯      ");
}

fn draw_table_4_people(arrangement: &[String]) {
    println!("\n       {}      {}    ", arrangement[0], arrangement[1]);
    println!("         🪑    🪑       ");
    println!("       ┌───────────┐    ");
    println!("       │           │    ");
    println!("    🪑 │           │ 🪑 ");
    println!("  {}   │           │   {}", arrangement[3], arrangement[2]);
    println!("       │           │    ");
    println!("       └───────────┘    ");
}

fn draw_table_5_people(arrangement: &[String]) {
    println!("\n           {}         ", arrangement[0]);
    println!("             🪑         ");
    println!("       {}         {}   ", arrangement[4], arrangement[1]);
    println!("         🪑 ╭───╮ 🪑     ");
    println!("           ╱     ╲      ");
    println!("         🪑         🪑   ");
    println!("       {}         {}   ", arrangement[3], arrangement[2]);
    println!("           ╲     ╱      ");
    println!("             ╰───╯      ");
}

fn draw_table_6_people(arrangement: &[String]) {
    println!("\n     {}       {}    ", arrangement[0], arrangement[1]);
    println!("       🪑     🪑       ");
    println!("   {}   ┌─────┐   {}  ", arrangement[5], arrangement[2]);
    println!("     🪑 │     │ 🪑     ");
    println!("       │       │       ");
    println!("     🪑 │     │ 🪑     ");
    println!("   {}   └─────┘   {}  ", arrangement[4], arrangement[3]);
    println!("       🪑     🪑       ");
}

fn draw_table_many_people(arrangement: &[String]) {
    println!("\n  ⭕ **Circular Table ({} people)**", arrangement.len());
    println!("     Seating clockwise:");
    
    for (i, person) in arrangement.iter().enumerate() {
        let arrow = if i == arrangement.len() - 1 { "↻" } else { "→" };
        println!("     {}: {} {}", i + 1, person, arrow);
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Finds the worst possible seating arrangement (minimum happiness)
fn find_worst_seating(graph: &HappinessGraph) -> (i32, Vec<String>) {
    let people: Vec<String> = graph.get_people();
    
    if people.is_empty() {
        return (0, Vec::new());
    }
    
    if people.len() == 1 {
        return (0, people);
    }
    
    let mut worst_happiness = i32::MAX;
    let mut worst_arrangement = people.clone();
    
    // Generate all permutations and find the one with minimum happiness
    let all_permutations = generate_permutations(&people);
    
    for arrangement in all_permutations {
        let total_happiness = calculate_circular_happiness(graph, &arrangement);
        
        if total_happiness < worst_happiness {
            worst_happiness = total_happiness;
            worst_arrangement = arrangement;
        }
    }
    
    (worst_happiness, worst_arrangement)
}

/// Calculates factorial for showing the number of permutations
fn factorial(n: usize) -> usize {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}