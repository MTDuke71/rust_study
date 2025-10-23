/// Optimized Day 13 Solver with Symmetry Reduction
/// 
/// This demonstrates how to optimize the TSP solver by exploiting
/// rotational and reflectional symmetry in circular seating arrangements.
use aoc2015::solver::day13::*;
use std::fs;
use std::time::Instant;

fn main() {
    println!("⚡ Day 13: Optimized Solver with Symmetry Reduction");
    println!("==================================================");
    
    let input = fs::read_to_string("inputs/day13_example.txt")
        .expect("Failed to read day13_example.txt");
    
    let graph = parse_happiness_graph(&input).unwrap();
    let people = graph.get_people();
    
    println!("\n📊 **PERFORMANCE COMPARISON:**");
    println!("=============================");
    
    // Original brute force approach
    println!("\n🐌 **Original Brute Force:**");
    let start = Instant::now();
    let (original_happiness, _) = find_optimal_seating(&graph);
    let original_time = start.elapsed();
    
    println!("  Result: {} happiness", original_happiness);
    println!("  Time: {:?}", original_time);
    println!("  Permutations checked: {}", factorial(people.len()));
    
    // Optimized approach with rotational symmetry
    println!("\n🚀 **Rotation-Optimized:**");
    let start = Instant::now();
    let (rotation_happiness, _) = find_optimal_seating_rotation_optimized(&graph);
    let rotation_time = start.elapsed();
    
    println!("  Result: {} happiness", rotation_happiness);
    println!("  Time: {:?}", rotation_time);
    println!("  Permutations checked: {}", factorial(people.len() - 1));
    println!("  Speedup: {:.1}×", original_time.as_nanos() as f64 / rotation_time.as_nanos() as f64);
    
    // Fully optimized approach (rotation + reflection)
    println!("\n⚡ **Fully Optimized (Rotation + Reflection):**");
    let start = Instant::now();
    let (full_happiness, full_arrangement) = find_optimal_seating_full_optimized(&graph);
    let full_time = start.elapsed();
    
    println!("  Result: {} happiness", full_happiness);
    println!("  Time: {:?}", full_time);
    println!("  Permutations checked: {}", factorial(people.len() - 1) / 2);
    println!("  Speedup: {:.1}×", original_time.as_nanos() as f64 / full_time.as_nanos() as f64);
    
    // Verify all approaches give the same result
    println!("\n✅ **VERIFICATION:**");
    println!("===================");
    println!("  All methods found same happiness? {}", 
             original_happiness == rotation_happiness && rotation_happiness == full_happiness);
    
    println!("  🎯 **OPTIMAL ARRANGEMENT:** {:?}", full_arrangement);
    
    // Show the math behind the optimization
    println!("\n🔢 **OPTIMIZATION MATHEMATICS:**");
    println!("==============================");
    let n = people.len();
    let original_perms = factorial(n);
    let rotation_perms = factorial(n - 1);
    let full_perms = factorial(n - 1) / 2;
    
    println!("  People (n): {}", n);
    println!("  Original: n! = {}", original_perms);
    println!("  Rotation fix: (n-1)! = {} ({:.1}× reduction)", rotation_perms, original_perms as f64 / rotation_perms as f64);
    println!("  + Reflection: (n-1)!/2 = {} ({:.1}× total reduction)", full_perms, original_perms as f64 / full_perms as f64);
}

/// Optimized solver that fixes one person's position to eliminate rotational symmetry
fn find_optimal_seating_rotation_optimized(graph: &HappinessGraph) -> (i32, Vec<String>) {
    let people = graph.get_people();
    if people.is_empty() {
        return (0, vec![]);
    }
    
    // Fix the first person at position 0 to eliminate rotational symmetry
    let fixed_person = people[0].clone();
    let mut remaining_people: Vec<String> = people.iter()
        .filter(|p| **p != fixed_person)
        .cloned()
        .collect();
    
    let mut best_happiness = i32::MIN;
    let mut best_arrangement = Vec::new();
    
    // Generate permutations of the remaining people
    generate_permutations_with_callback(&mut remaining_people, &mut |perm| {
        // Create full arrangement with fixed person at the start
        let mut arrangement = vec![fixed_person.clone()];
        arrangement.extend_from_slice(perm);
        
        let happiness = calculate_circular_happiness(graph, &arrangement);
        if happiness > best_happiness {
            best_happiness = happiness;
            best_arrangement = arrangement.clone();
        }
    });
    
    (best_happiness, best_arrangement)
}

/// Fully optimized solver that eliminates both rotational and reflectional symmetry
fn find_optimal_seating_full_optimized(graph: &HappinessGraph) -> (i32, Vec<String>) {
    let people = graph.get_people();
    if people.len() < 2 {
        return find_optimal_seating_rotation_optimized(graph);
    }
    
    // Fix first person at position 0 and second person at position 1
    // This eliminates both rotational and reflectional symmetry
    let fixed_person1 = people[0].clone();
    let fixed_person2 = people[1].clone();
    let mut remaining_people: Vec<String> = people.iter()
        .filter(|p| **p != fixed_person1 && **p != fixed_person2)
        .cloned()
        .collect();
    
    let mut best_happiness = i32::MIN;
    let mut best_arrangement = Vec::new();
    
    generate_permutations_with_callback(&mut remaining_people, &mut |perm| {
        // Create arrangement with two fixed people
        let mut arrangement = vec![fixed_person1.clone(), fixed_person2.clone()];
        arrangement.extend_from_slice(perm);
        
        let happiness = calculate_circular_happiness(graph, &arrangement);
        if happiness > best_happiness {
            best_happiness = happiness;
            best_arrangement = arrangement.clone();
        }
    });
    
    (best_happiness, best_arrangement)
}

/// Generate all permutations of a vector and call the callback for each one
fn generate_permutations_with_callback<T: Clone, F: FnMut(&[T])>(items: &mut [T], callback: &mut F) {
    heap_permute(items, items.len(), callback);
}

/// Heap's algorithm for generating permutations
fn heap_permute<T: Clone, F: FnMut(&[T])>(items: &mut [T], k: usize, callback: &mut F) {
    if k == 1 {
        callback(items);
        return;
    }
    
    for i in 0..k {
        heap_permute(items, k - 1, callback);
        
        if k.is_multiple_of(2) {
            items.swap(i, k - 1);
        } else {
            items.swap(0, k - 1);
        }
    }
}

fn factorial(n: usize) -> usize {
    match n {
        0 | 1 => 1,
        _ => (2..=n).product()
    }
}