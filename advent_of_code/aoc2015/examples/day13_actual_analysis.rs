/// Day 13 Actual Example Analysis
/// 
/// This program analyzes the actual Day 13 example input and shows:
/// 1. Part 1: Optimal seating arrangement for all 8 people
/// 2. Part 2: Optimal arrangement after adding yourself
/// 3. Comparison table showing the difference
/// 
/// Usage: cargo run --example day13_actual_analysis
///
use aoc2015::solver::day13::*;
use std::fs;

fn main() {
    println!("🎄 Day 13: Knights of the Dinner Table - Actual Example Analysis");
    println!("================================================================");
    
    // Read the actual Day 13 example input
    let input = fs::read_to_string("inputs/day13_example.txt")
        .expect("Failed to read day13_example.txt");
    
    println!("\n📋 **Problem Summary:**");
    let lines: Vec<&str> = input.lines().collect();
    println!("  • Input lines: {}", lines.len());
    
    // Parse the graph to get people count
    let graph = parse_happiness_graph(&input).unwrap();
    let people_count = graph.people.len();
    println!("  • People attending: {}", people_count);
    println!("  • Total relationships: {}", graph.edges.len());
    println!("  • Permutations to check: {}", factorial(people_count));
    
    // Show all people
    let mut people: Vec<String> = graph.get_people();
    people.sort();
    println!("  • Attendees: {}", people.join(", "));
    
    // Part 1: Find optimal arrangement without you
    println!("\n🥇 **PART 1: Optimal Arrangement (Without You)**");
    println!("===============================================");
    
    let start_time = std::time::Instant::now();
    let (part1_happiness, part1_arrangement) = find_optimal_seating(&graph);
    let part1_time = start_time.elapsed();
    
    println!("  🎯 Maximum Happiness: {}", part1_happiness);
    println!("  ⏱️  Computation Time: {:?}", part1_time);
    
    print_arrangement_table("Part 1 - Original Guest List", &graph, &part1_arrangement, part1_happiness);
    
    // Part 2: Add yourself and find new optimal arrangement
    println!("\n🥈 **PART 2: Optimal Arrangement (With You)**");
    println!("============================================");
    
    let mut graph_with_you = graph.clone();
    graph_with_you.add_neutral_person("You");
    
    let start_time = std::time::Instant::now();
    let (part2_happiness, part2_arrangement) = find_optimal_seating(&graph_with_you);
    let part2_time = start_time.elapsed();
    
    println!("  🎯 Maximum Happiness: {}", part2_happiness);
    println!("  ⏱️  Computation Time: {:?}", part2_time);
    
    print_arrangement_table("Part 2 - Including You", &graph_with_you, &part2_arrangement, part2_happiness);
    
    // Comparison
    println!("\n📊 **COMPARISON SUMMARY**");
    println!("========================");
    
    let happiness_change = part2_happiness - part1_happiness;
    let change_symbol = if happiness_change >= 0 { "📈" } else { "📉" };
    
    println!("  🥇 Part 1 (8 people):     {:>6} happiness", part1_happiness);
    println!("  🥈 Part 2 (9 people):     {:>6} happiness", part2_happiness);
    println!("  {} Impact of adding you: {:>6} happiness", change_symbol, happiness_change);
    
    let percentage = if part1_happiness != 0 {
        (happiness_change as f64 / part1_happiness as f64) * 100.0
    } else {
        0.0
    };
    
    println!("  📊 Percentage change:     {:>5.1}%", percentage);
    
    // Show where you got seated in Part 2
    if let Some(your_position) = part2_arrangement.iter().position(|p| p == "You") {
        let left_neighbor = &part2_arrangement[(your_position + part2_arrangement.len() - 1) % part2_arrangement.len()];
        let right_neighbor = &part2_arrangement[(your_position + 1) % part2_arrangement.len()];
        
        println!("\n🪑 **Your Seating Position in Part 2:**");
        println!("  • Position: {} of {} ({})", your_position + 1, part2_arrangement.len(), 
                 if your_position < part2_arrangement.len() / 2 { "first half" } else { "second half" });
        println!("  • Left neighbor: {}", left_neighbor);
        println!("  • Right neighbor: {}", right_neighbor);
        println!("  • Your contribution: 0 (neutral toward everyone)");
    }
    
    println!("\n🎉 **Analysis Complete!** The optimal seating arrangements have been found.");
}

fn print_arrangement_table(title: &str, graph: &HappinessGraph, arrangement: &[String], total_happiness: i32) {
    println!("\n┌─{}─┐", "─".repeat(title.len() + 2));
    println!("│ {} │", title);
    println!("└─{}─┘", "─".repeat(title.len() + 2));
    
    // Print seating order
    println!("\n🪑 **Seating Order (Clockwise):**");
    for (i, person) in arrangement.iter().enumerate() {
        let next_person = &arrangement[(i + 1) % arrangement.len()];
        let arrow = if i == arrangement.len() - 1 { "↻" } else { "→" };
        println!("  {}: {} {} {}", i + 1, person, arrow, if i == arrangement.len() - 1 { &arrangement[0] } else { next_person });
    }
    
    // Print happiness contributions table
    println!("\n📋 **Individual Happiness Contributions:**");
    println!("┌─{:─<12}─┬─{:─<12}─┬─{:─<12}─┬─{:─<8}─┬─{:─<8}─┬─{:─<8}─┐", "", "", "", "", "", "");
    println!("│ {:^12} │ {:^12} │ {:^12} │ {:^8} │ {:^8} │ {:^8} │", 
             "Person", "Left Neighbor", "Right Neighbor", "Left ❤️", "Right ❤️", "Total");
    println!("├─{:─<12}─┼─{:─<12}─┼─{:─<12}─┼─{:─<8}─┼─{:─<8}─┼─{:─<8}─┤", "", "", "", "", "", "");
    
    for (i, person) in arrangement.iter().enumerate() {
        let left_neighbor = &arrangement[(i + arrangement.len() - 1) % arrangement.len()];
        let right_neighbor = &arrangement[(i + 1) % arrangement.len()];
        
        let left_happiness = graph.get_happiness(person, left_neighbor);
        let right_happiness = graph.get_happiness(person, right_neighbor);
        let person_total = left_happiness + right_happiness;
        
        // Truncate names if too long
        let person_display = truncate_name(person, 12);
        let left_display = truncate_name(left_neighbor, 12);
        let right_display = truncate_name(right_neighbor, 12);
        
        println!("│ {:^12} │ {:^12} │ {:^12} │ {:>+7} │ {:>+7} │ {:>+7} │",
                 person_display, left_display, right_display, left_happiness, right_happiness, person_total);
    }
    
    println!("├─{:─<12}─┴─{:─<12}─┴─{:─<12}─┴─{:─<8}─┴─{:─<8}─┼─{:─<8}─┤", "", "", "", "", "", "");
    println!("│ {:^57} │ {:>+7} │", "TOTAL HAPPINESS", total_happiness);
    println!("└─{:─<57}─┴─{:─<8}─┘", "", "");
}

fn truncate_name(name: &str, max_len: usize) -> String {
    if name.len() <= max_len {
        name.to_string()
    } else {
        format!("{}...", &name[..max_len-3])
    }
}

fn factorial(n: usize) -> usize {
    match n {
        0 | 1 => 1,
        _ => {
            let mut result = 1;
            for i in 2..=n {
                result *= i;
            }
            result
        }
    }
}