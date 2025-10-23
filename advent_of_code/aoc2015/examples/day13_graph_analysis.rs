/// Day 13 Graph Theory Analysis
/// 
/// This demonstrates that our HappinessGraph is indeed an adjacency graph
/// and explores optimization strategies for the TSP variant.

use aoc2015::solver::day13::*;
use std::fs;

fn main() {
    println!("🕸️ Day 13: Graph Theory Analysis");
    println!("================================");
    
    let input = fs::read_to_string("inputs/day13_example.txt")
        .expect("Failed to read day13_example.txt");
    
    let graph = parse_happiness_graph(&input).unwrap();
    
    // 1. Demonstrate this is an adjacency graph
    println!("\n📊 **ADJACENCY GRAPH STRUCTURE:**");
    println!("================================");
    
    let people: Vec<String> = graph.get_people();
    println!("🔹 Vertices (V): {} people", people.len());
    println!("🔹 Edges (E): {} relationships", graph.edges.len());
    println!("🔹 Graph Type: Weighted, Directed, Complete");
    
    // Show adjacency representation
    println!("\n📋 **Adjacency List Representation:**");
    for person in &people {
        let mut neighbors = Vec::new();
        for other in &people {
            if person != other {
                if let Some(&weight) = graph.edges.get(&(person.clone(), other.clone())) {
                    neighbors.push(format!("{}({})", other, weight));
                }
            }
        }
        println!("  {}: [{}]", person, neighbors.join(", "));
    }
    
    // 2. Analyze why we can't just find "weakest link"
    println!("\n🔗 **WHY 'WEAKEST LINK' STRATEGY FAILS:**");
    println!("========================================");
    
    // Find all edge weights and analyze
    let mut all_edges: Vec<((String, String), i32)> = graph.edges.iter()
        .map(|((from, to), &weight)| ((from.clone(), to.clone()), weight))
        .collect();
    
    all_edges.sort_by_key(|(_, weight)| *weight);
    
    println!("🔸 Weakest edges (lowest happiness):");
    for (((from, to), weight), i) in all_edges.iter().zip(0..) {
        if i < 5 {  // Show top 5 weakest
            println!("    {} → {}: {}", from, to, weight);
        }
    }
    
    println!("\n🔸 Strongest edges (highest happiness):");
    all_edges.reverse();
    for (((from, to), weight), i) in all_edges.iter().zip(0..) {
        if i < 5 {  // Show top 5 strongest
            println!("    {} → {}: {}", from, to, weight);
        }
    }
    
    // Demonstrate why Alice-George isn't the insertion point
    let alice_george = graph.get_happiness("Alice", "George");
    let george_alice = graph.get_happiness("George", "Alice");
    let connection_strength = alice_george + george_alice;
    
    println!("\n🎯 **ALICE-GEORGE CONNECTION ANALYSIS:**");
    println!("  Alice → George: {}", alice_george);
    println!("  George → Alice: {}", george_alice);
    println!("  Combined strength: {}", connection_strength);
    println!("  🚨 This is the STRONGEST connection! Breaking it would lose {} happiness", connection_strength);
    
    // Show what actually happened
    let david_frank = graph.get_happiness("David", "Frank");
    let frank_david = graph.get_happiness("Frank", "David");
    let broken_connection = david_frank + frank_david;
    
    println!("\n🔧 **ACTUAL INSERTION POINT (David-Frank):**");
    println!("  David → Frank: {}", david_frank);
    println!("  Frank → David: {}", frank_david);  
    println!("  Combined strength: {}", broken_connection);
    println!("  💡 This connection was broken instead (only {} happiness lost)", broken_connection);
    
    // 3. Explore circular symmetry optimization
    println!("\n🔄 **CIRCULAR SYMMETRY OPTIMIZATION:**");
    println!("====================================");
    
    println!("🔸 Current approach: Check all {} permutations", factorial(people.len()));
    println!("🔸 With rotation optimization: Check {} unique arrangements", factorial(people.len()) / people.len());
    println!("🔸 With reflection + rotation: Check {} unique arrangements", factorial(people.len()) / (people.len() * 2));
    
    let original_perms = factorial(people.len());
    let rotation_only = original_perms / people.len();  
    let both_optimizations = original_perms / (people.len() * 2);
    
    println!("\n📊 **OPTIMIZATION IMPACT:**");
    println!("  Original permutations: {:>8}", original_perms);
    println!("  Rotation-optimized:    {:>8} ({:.1}× speedup)", rotation_only, original_perms as f64 / rotation_only as f64);
    println!("  Rotation + Reflection: {:>8} ({:.1}× speedup)", both_optimizations, original_perms as f64 / both_optimizations as f64);
    
    // 4. Demonstrate the optimization
    demonstrate_symmetry_optimization(&graph);
}

fn demonstrate_symmetry_optimization(graph: &HappinessGraph) {
    println!("\n🧪 **SYMMETRY OPTIMIZATION DEMO:**");
    println!("=================================");
    
    // Generate a few permutations and show their equivalents
    let people: Vec<String> = graph.get_people();
    let base_arrangement = people.clone();
    
    println!("🔸 Base arrangement: {:?}", base_arrangement);
    
    // Show rotations
    println!("\n🔄 Rotational equivalents:");
    for i in 0..std::cmp::min(3, people.len()) {
        let mut rotated = base_arrangement.clone();
        rotated.rotate_left(i);
        let happiness = calculate_circular_happiness(graph, &rotated);
        println!("  Rotation {}: {:?} → happiness: {}", i, rotated, happiness);
    }
    
    // Show reflections  
    println!("\n🪞 Reflections (clockwise ↔ counter-clockwise):");
    let mut reflected = base_arrangement.clone();
    reflected.reverse();
    reflected.rotate_right(1); // Adjust to maintain starting point
    let original_happiness = calculate_circular_happiness(graph, &base_arrangement);
    let reflected_happiness = calculate_circular_happiness(graph, &reflected);
    
    println!("  Original:  {:?} → happiness: {}", base_arrangement, original_happiness);
    println!("  Reflected: {:?} → happiness: {}", reflected, reflected_happiness);
    println!("  Equal? {}", original_happiness == reflected_happiness);
}

fn factorial(n: usize) -> usize {
    match n {
        0 | 1 => 1,
        _ => (2..=n).product()
    }
}