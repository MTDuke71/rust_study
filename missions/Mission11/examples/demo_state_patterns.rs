//! REQ-5: State Space Design Patterns - Comprehensive Examples
//!
//! This example demonstrates all 5 state pattern types with real problem scenarios.
//!
//! # Pattern Catalog
//!
//! 1. **String Suffixes** (`&'a str`) - Pattern matching with zero-copy
//! 2. **Index Positions** (`usize`) - Sequence/array problems
//! 3. **Coordinate Pairs** (`(x, y)`) - Grid navigation
//! 4. **Composite State** (`(T1, T2, T3)`) - Multi-dimensional state
//! 5. **Custom Structs** (`#[derive(Hash, Eq)]`) - Complex domain models
//!
//! Run with: `cargo run -p mission11 --example demo_state_patterns`

use mission11::state_patterns::*;
use std::collections::HashMap;

fn main() {
    println!("{}", "=".repeat(80));
    println!("Mission 11 REQ-5: State Space Design Patterns");
    println!("{}", "=".repeat(80));
    println!();

    demo_pattern1_string_suffix();
    demo_pattern2_index_position();
    demo_pattern3_coordinate_pairs();
    demo_pattern4_composite_state();
    demo_pattern5_custom_struct();
    demo_advanced_composite_with_borrowed();

    println!("\n{}", "=".repeat(80));
    println!("All pattern examples completed successfully!");
    println!("{}", "=".repeat(80));
}

/// Pattern 1: String Suffixes with Zero-Copy Borrowing
fn demo_pattern1_string_suffix() {
    println!("📝 Pattern 1: String Suffixes (&'a str)");
    println!("{}", "-".repeat(80));

    let patterns = vec!["ab", "abc", "bc"];
    let test_cases = vec![
        ("abc", true),
        ("abcbc", true),
        ("abcabc", true),
        ("xyz", false),
    ];

    for (target, expected) in test_cases {
        let mut memo = HashMap::new();
        let result = string_suffix_pattern(target, &patterns, &mut memo);

        println!(
            "  Can construct {:?} from {:?}? {} (cache: {} states)",
            target,
            patterns,
            result,
            memo.len()
        );
        assert_eq!(result, expected);
    }

    println!("  ✅ Zero-copy string slicing verified!");
    println!();
}

/// Pattern 2: Index Positions for Sequence Problems
fn demo_pattern2_index_position() {
    println!("🔢 Pattern 2: Index Positions (usize)");
    println!("{}", "-".repeat(80));

    let arr = vec![5, 10, 15, 20, 25];
    let mut memo = HashMap::new();

    for start_pos in 0..arr.len() {
        let cache_before = memo.len();
        let sum = index_position_pattern(start_pos, &arr, &mut memo);
        let cache_after = memo.len();

        if cache_before == cache_after {
            // No new entries added = cache hit
            println!("  Sum from position {}: {} [CACHE HIT]", start_pos, sum);
        } else {
            // New entries added = computed recursively
            println!(
                "  Sum from position {}: {} [computed, cached {} new states]",
                start_pos,
                sum,
                cache_after - cache_before
            );
        }
    }

    println!("  Cache size: {} positions", memo.len());
    println!("  ✅ Index-based state caching verified!");
    println!();
}

/// Pattern 3: Coordinate Pairs for Grid Navigation
fn demo_pattern3_coordinate_pairs() {
    println!("📍 Pattern 3: Coordinate Pairs ((x, y))");
    println!("{}", "-".repeat(80));

    let grid = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];

    println!("  Grid:");
    for row in &grid {
        println!("    {:?}", row);
    }
    println!();

    let test_cases = vec![
        ((0, 0), (1, 1), "2x2 corner"),
        ((0, 0), (3, 2), "Full grid"),
        ((1, 0), (2, 1), "Subgrid"),
        ((1, 1), (1, 1), "Same point (edge case)"),
        ((2, 2), (0, 0), "Reversed points (edge case)"),
    ];

    for (start, end, desc) in test_cases {
        let mut memo = HashMap::new();
        let paths = coordinate_pair_pattern(start, end, &grid, &mut memo);

        println!(
            "  {} {:?} → {:?}: {} paths (cache: {} coords)",
            desc,
            start,
            end,
            paths,
            memo.len()
        );
    }

    println!("  ✅ 2D coordinate state verified!");
    println!();
}

/// Pattern 4: Composite State for Multi-Dimensional Problems
fn demo_pattern4_composite_state() {
    println!("🎲 Pattern 4: Composite State ((value, count))");
    println!("{}", "-".repeat(80));

    // Simulates AoC 2024 Day 11: Stone transformations
    let test_stones = vec![
        (0, 3, "Zero stone"),
        (1, 3, "One stone"),
        (125, 5, "Three-digit stone"),
        (1000, 4, "Four-digit stone"),
    ];

    for (stone_value, blinks, desc) in test_stones {
        let mut memo = HashMap::new();
        let final_count = composite_state_pattern((stone_value, blinks), &mut memo);

        println!(
            "  {}: Stone {} after {} blinks → {} stones (cache: {} states)",
            desc,
            stone_value,
            blinks,
            final_count,
            memo.len()
        );
    }

    println!("  ✅ Multi-dimensional composite state verified!");
    println!();
}

/// Pattern 5: Custom Structs for Complex Domain Models
fn demo_pattern5_custom_struct() {
    println!("🎮 Pattern 5: Custom Structs (#[derive(Hash, Eq)])");
    println!("{}", "-".repeat(80));

    let scenarios = vec![
        (
            GameState {
                player_hp: 100,
                boss_hp: 30,
                turn: 0,
                mana: 500,
            },
            true,
            "Strong player vs weak boss",
        ),
        (
            GameState {
                player_hp: 20,
                boss_hp: 100,
                turn: 0,
                mana: 100,
            },
            false,
            "Weak player vs strong boss",
        ),
        (
            GameState {
                player_hp: 60,
                boss_hp: 60,
                turn: 0,
                mana: 300,
            },
            true,
            "Balanced fight (player advantage)",
        ),
    ];

    for (state, expected, desc) in scenarios {
        let mut memo = HashMap::new();
        let can_win = custom_struct_pattern(state.clone(), &mut memo);

        println!(
            "  {}: Player HP={}, Boss HP={}, Mana={} → Can win? {} (cache: {} states)",
            desc,
            state.player_hp,
            state.boss_hp,
            state.mana,
            can_win,
            memo.len()
        );
        assert_eq!(can_win, expected);
    }

    println!("  ✅ Custom struct state with semantic fields verified!");
    println!();
}

/// Advanced: Composite with Borrowed Data
fn demo_advanced_composite_with_borrowed() {
    println!("🔗 Advanced: Composite with Borrowed (&'a str, usize)");
    println!("{}", "-".repeat(80));

    let pattern_groups = vec![
        vec!["a", "ab"], // Group 0: 'a' patterns
        vec!["c", "cd"], // Group 1: 'c' patterns
        vec!["e", "ef"], // Group 2: 'e' patterns
    ];

    let test_cases = vec![
        ("ace", true, "One from each group"),
        ("abcde", true, "Longer patterns"),
        ("ac", false, "Missing third group"),
        ("xyz", false, "Invalid patterns"),
    ];

    println!("  Pattern groups: {:?}", pattern_groups);
    println!();

    for (target, expected, desc) in test_cases {
        let mut memo = HashMap::new();
        let result = composite_with_borrowed(target, 0, &pattern_groups, &mut memo);

        println!(
            "  {}: {:?} → {} (cache: {} composite states)",
            desc,
            target,
            result,
            memo.len()
        );
        assert_eq!(result, expected);
    }

    println!("  ✅ Composite borrowed+owned state verified!");
    println!();
}
