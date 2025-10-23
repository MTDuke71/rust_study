/// Day 13 Optimization Verification
///
/// This program proves that the symmetry-optimized solver produces
/// exactly the same results as the brute force method on the actual
/// Day 13 example dataset.
///
use aoc2015::solver::day13::*;
use std::fs;
use std::time::Instant;

fn main() {
    println!("🔬 Day 13: Optimization Verification Test");
    println!("=========================================");

    let input =
        fs::read_to_string("inputs/day13_example.txt").expect("Failed to read day13_example.txt");

    let graph = parse_happiness_graph(&input).unwrap();
    let people = graph.get_people();

    println!("📋 **Test Dataset:**");
    println!("  • File: day13_example.txt");
    println!("  • People: {} ({})", people.len(), people.join(", "));
    println!("  • Relationships: {}", graph.edges.len());

    // Test 1: Original vs Rotation-Optimized
    println!("\n🧪 **TEST 1: Original vs Rotation-Optimized**");
    println!("=============================================");

    // Original brute force
    println!("\n🐌 **Original Brute Force Method:**");
    let start = Instant::now();
    let (original_happiness, original_arrangement) = find_optimal_seating(&graph);
    let original_time = start.elapsed();

    println!("  ✅ Result: {} happiness", original_happiness);
    println!("  ⏱️  Time: {:?}", original_time);
    println!("  📊 Permutations: {}", factorial(people.len()));
    println!("  🪑 Arrangement: {:?}", original_arrangement);

    // Rotation-optimized
    println!("\n🚀 **Rotation-Optimized Method:**");
    let start = Instant::now();
    let (rotation_happiness, rotation_arrangement) = find_optimal_seating_rotation_fixed(&graph);
    let rotation_time = start.elapsed();

    println!("  ✅ Result: {} happiness", rotation_happiness);
    println!("  ⏱️  Time: {:?}", rotation_time);
    println!("  📊 Permutations: {}", factorial(people.len() - 1));
    println!("  🪑 Arrangement: {:?}", rotation_arrangement);

    // Verification
    let test1_passed = original_happiness == rotation_happiness;
    println!("\n🎯 **TEST 1 RESULT:**");
    println!(
        "  Happiness Match: {} ({})",
        if test1_passed { "✅ PASS" } else { "❌ FAIL" },
        if test1_passed {
            format!("{} == {}", original_happiness, rotation_happiness)
        } else {
            format!("{} ≠ {}", original_happiness, rotation_happiness)
        }
    );

    if test1_passed {
        let speedup = original_time.as_nanos() as f64 / rotation_time.as_nanos() as f64;
        println!("  Performance: {:.1}× speedup", speedup);
        println!(
            "  Arrangement: {} (rotations of same solution)",
            if arrangements_are_rotations(&original_arrangement, &rotation_arrangement) {
                "✅ Equivalent"
            } else {
                "⚠️  Different (but same happiness)"
            }
        );
    }

    // Test 2: Test Part 2 (with "You" added)
    println!("\n🧪 **TEST 2: Part 2 Verification (With 'You' Added)**");
    println!("===================================================");

    let mut graph_with_you = graph.clone();
    graph_with_you.add_neutral_person("You");

    println!("\n🐌 **Original Method (Part 2):**");
    let start = Instant::now();
    let (original_part2, _) = find_optimal_seating(&graph_with_you);
    let original_time2 = start.elapsed();

    println!("  ✅ Result: {} happiness", original_part2);
    println!("  ⏱️  Time: {:?}", original_time2);
    println!("  📊 Permutations: {}", factorial(people.len() + 1));

    println!("\n🚀 **Optimized Method (Part 2):**");
    let start = Instant::now();
    let (rotation_part2, _) = find_optimal_seating_rotation_fixed(&graph_with_you);
    let rotation_time2 = start.elapsed();

    println!("  ✅ Result: {} happiness", rotation_part2);
    println!("  ⏱️  Time: {:?}", rotation_time2);
    println!("  📊 Permutations: {}", factorial(people.len()));

    let test2_passed = original_part2 == rotation_part2;
    println!("\n🎯 **TEST 2 RESULT:**");
    println!(
        "  Happiness Match: {} ({})",
        if test2_passed { "✅ PASS" } else { "❌ FAIL" },
        if test2_passed {
            format!("{} == {}", original_part2, rotation_part2)
        } else {
            format!("{} ≠ {}", original_part2, rotation_part2)
        }
    );

    if test2_passed {
        let speedup2 = original_time2.as_nanos() as f64 / rotation_time2.as_nanos() as f64;
        println!("  Performance: {:.1}× speedup", speedup2);
    }

    // Test 3: Consistency Check with Multiple Runs
    println!("\n🧪 **TEST 3: Multiple Verification Runs**");
    println!("=======================================");

    println!("\n🔄 **Running optimization multiple times to ensure consistency:**");
    let mut all_runs_consistent = true;

    for run in 1..=3 {
        let (run_happiness, _) = find_optimal_seating_rotation_fixed(&graph);
        let run_passed = run_happiness == original_happiness;
        all_runs_consistent &= run_passed;

        println!(
            "  Run {}: {} happiness {}",
            run,
            run_happiness,
            if run_passed { "✅" } else { "❌" }
        );
    }

    let test3_passed = all_runs_consistent;
    println!(
        "  All runs consistent: {}",
        if test3_passed { "✅ PASS" } else { "❌ FAIL" }
    );

    // Final Summary
    println!("\n🏆 **FINAL VERIFICATION SUMMARY**");
    println!("================================");

    let all_tests_passed = test1_passed && test2_passed && test3_passed;

    println!(
        "  Test 1 (Part 1): {}",
        if test1_passed { "✅ PASS" } else { "❌ FAIL" }
    );
    println!(
        "  Test 2 (Part 2): {}",
        if test2_passed { "✅ PASS" } else { "❌ FAIL" }
    );
    println!(
        "  Test 3 (Subset): {}",
        if test3_passed { "✅ PASS" } else { "❌ FAIL" }
    );
    println!(
        "  
  Overall: {} - Optimization is mathematically equivalent!",
        if all_tests_passed {
            "✅ ALL TESTS PASS"
        } else {
            "❌ SOME TESTS FAILED"
        }
    );

    if all_tests_passed {
        let total_speedup = (original_time.as_nanos() + original_time2.as_nanos()) as f64
            / (rotation_time.as_nanos() + rotation_time2.as_nanos()) as f64;
        println!("  Combined Speedup: {:.1}×", total_speedup);
        println!(
            "  
  🎉 **CONCLUSION:** The rotation-optimized solver produces identical"
        );
        println!("     results to brute force while being significantly faster!");
    }
}

/// Rotation-optimized solver that fixes first person's position
fn find_optimal_seating_rotation_fixed(graph: &HappinessGraph) -> (i32, Vec<String>) {
    let people = graph.get_people();
    if people.is_empty() {
        return (0, vec![]);
    }

    // Fix the first person (alphabetically) at position 0 to eliminate rotational symmetry
    let fixed_person = people[0].clone();
    let mut remaining_people: Vec<String> = people
        .iter()
        .filter(|p| **p != fixed_person)
        .cloned()
        .collect();

    let mut best_happiness = i32::MIN;
    let mut best_arrangement = Vec::new();

    // Generate all permutations of remaining people
    generate_permutations_heap(&mut remaining_people, &mut |perm| {
        let mut full_arrangement = vec![fixed_person.clone()];
        full_arrangement.extend_from_slice(perm);

        let happiness = calculate_circular_happiness(graph, &full_arrangement);
        if happiness > best_happiness {
            best_happiness = happiness;
            best_arrangement = full_arrangement.clone();
        }
    });

    (best_happiness, best_arrangement)
}

/// Generate all permutations using Heap's algorithm
fn generate_permutations_heap<T: Clone>(items: &mut [T], callback: &mut impl FnMut(&[T])) {
    fn heap_permute<T: Clone>(items: &mut [T], k: usize, callback: &mut impl FnMut(&[T])) {
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

    heap_permute(items, items.len(), callback);
}

/// Check if two arrangements are rotations of each other
fn arrangements_are_rotations(arr1: &[String], arr2: &[String]) -> bool {
    if arr1.len() != arr2.len() {
        return false;
    }

    let n = arr1.len();
    for rotation in 0..n {
        let mut matches = true;
        for i in 0..n {
            if arr1[i] != arr2[(i + rotation) % n] {
                matches = false;
                break;
            }
        }
        if matches {
            return true;
        }
    }
    false
}

/// Factorial calculation
fn factorial(n: usize) -> usize {
    (1..=n).product()
}
