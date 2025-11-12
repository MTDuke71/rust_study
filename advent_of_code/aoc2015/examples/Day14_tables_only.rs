//! # Day 14 Actual Problem Tables
//!
//! This example shows just the comprehensive analysis tables for the actual
//! AoC 2015 Day 14 problem input, with results every 200 seconds through 2503.

use aoc2015::solver::day14::*;

/// Actual puzzle input reindeer
fn get_actual_reindeer() -> Vec<Reindeer> {
    vec![
        Reindeer::new("Vixen".to_string(), 8, 8, 53),
        Reindeer::new("Blitzen".to_string(), 13, 4, 49),
        Reindeer::new("Rudolph".to_string(), 20, 7, 132),
        Reindeer::new("Cupid".to_string(), 12, 4, 43),
        Reindeer::new("Donner".to_string(), 9, 5, 38),
        Reindeer::new("Dasher".to_string(), 10, 4, 37),
        Reindeer::new("Comet".to_string(), 3, 37, 76),
        Reindeer::new("Prancer".to_string(), 9, 12, 97),
        Reindeer::new("Dancer".to_string(), 37, 1, 36),
    ]
}

/// Optimized distance calculation (O(1) vs O(n) simulation)
fn calculate_distance_optimized(reindeer: &Reindeer, duration: u32) -> u32 {
    let cycle_length = reindeer.cycle_length();
    let distance_per_cycle = reindeer.distance_per_cycle();

    let complete_cycles = duration / cycle_length;
    let remaining_time = duration % cycle_length;

    let complete_cycle_distance = complete_cycles * distance_per_cycle;

    let final_distance = if remaining_time <= reindeer.flight_time() {
        remaining_time * reindeer.speed()
    } else {
        reindeer.flight_time() * reindeer.speed()
    };

    complete_cycle_distance + final_distance
}

fn main() {
    println!("🦌 AoC 2015 Day 14: Actual Problem Analysis Tables\n");

    let reindeer = get_actual_reindeer();
    let time_points = vec![
        200, 400, 600, 800, 1000, 1200, 1400, 1600, 1800, 2000, 2200, 2503,
    ];

    // Part 1: Distance Analysis
    println!("🏁 PART 1: DISTANCE RACE ANALYSIS (Every 200s + Final)");
    println!("{}", "=".repeat(105));

    // Header row
    print!("{:<12}", "Reindeer");
    for &time in &time_points {
        print!("{:>8}s", time);
    }
    println!();

    // Separator
    print!("{}", "-".repeat(12));
    for _ in &time_points {
        print!("{}", "-".repeat(9));
    }
    println!();

    // Data rows for each reindeer
    let mut reindeer_distances: Vec<Vec<u32>> = Vec::new();

    for r in &reindeer {
        print!("{:<12}", r.name());
        let mut distances = Vec::new();

        for &time in &time_points {
            let distance = calculate_distance_optimized(r, time);
            distances.push(distance);
            print!("{:>8}", distance);
        }
        println!();
        reindeer_distances.push(distances);
    }

    // Winner row
    print!("{}", "-".repeat(12));
    for _ in &time_points {
        print!("{}", "-".repeat(9));
    }
    println!();

    print!("{:<12}", "WINNER");
    for (i, &_time) in time_points.iter().enumerate() {
        let max_distance = reindeer_distances
            .iter()
            .map(|distances| distances[i])
            .max()
            .unwrap_or(0);
        print!("{:>8}", max_distance);
    }
    println!("\n");

    // Part 2: Points Analysis
    println!("🏆 PART 2: POINTS RACE ANALYSIS (Every 200s + Final)");
    println!("{}", "=".repeat(105));

    // Calculate points at each time interval
    let mut points_data: Vec<Vec<u32>> = vec![vec![0; time_points.len()]; reindeer.len()];

    for (time_idx, &time) in time_points.iter().enumerate() {
        let mut current_points = vec![0u32; reindeer.len()];

        for second in 1..=time {
            let distances: Vec<u32> = reindeer
                .iter()
                .map(|r| calculate_distance_optimized(r, second))
                .collect();

            let max_distance = distances.iter().max().unwrap_or(&0);

            for (reindeer_idx, distance) in distances.iter().enumerate() {
                if distance == max_distance {
                    current_points[reindeer_idx] += 1;
                }
            }
        }

        for (reindeer_idx, &points) in current_points.iter().enumerate() {
            points_data[reindeer_idx][time_idx] = points;
        }
    }

    // Header row
    print!("{:<12}", "Reindeer");
    for &time in &time_points {
        print!("{:>8}s", time);
    }
    println!();

    // Separator
    print!("{}", "-".repeat(12));
    for _ in &time_points {
        print!("{}", "-".repeat(9));
    }
    println!();

    // Data rows for points
    for (idx, r) in reindeer.iter().enumerate() {
        print!("{:<12}", r.name());
        for (time_idx, _) in time_points.iter().enumerate() {
            print!("{:>8}", points_data[idx][time_idx]);
        }
        println!();
    }

    // Winner row for points
    print!("{}", "-".repeat(12));
    for _ in &time_points {
        print!("{}", "-".repeat(9));
    }
    println!();

    print!("{:<12}", "WINNER");
    for time_idx in 0..time_points.len() {
        let max_points = points_data
            .iter()
            .map(|points| points[time_idx])
            .max()
            .unwrap_or(0);
        print!("{:>8}", max_points);
    }
    println!();

    // Summary
    println!("\n📊 FINAL RESULTS:");
    let final_distances: Vec<(String, u32)> = reindeer
        .iter()
        .map(|r| (r.name().to_string(), calculate_distance_optimized(r, 2503)))
        .collect();

    let final_points = simulate_race_with_points(&reindeer, 2503);
    let distance_winner = final_distances.iter().max_by_key(|(_, d)| d).unwrap();

    let points_winner_idx = points_data
        .iter()
        .enumerate()
        .max_by_key(|(_, points)| points[time_points.len() - 1])
        .map(|(idx, _)| idx)
        .unwrap_or(0);

    println!(
        "🥇 Part 1 Winner: {} with {} km",
        distance_winner.0, distance_winner.1
    );
    println!(
        "🏆 Part 2 Winner: {} with {} points",
        reindeer[points_winner_idx].name(),
        final_points
    );

    // Show the key insight
    println!("\n💡 Key Insight:");
    if distance_winner.0 != reindeer[points_winner_idx].name() {
        println!(
            "   Different winners! {} wins by distance, {} wins by points.",
            distance_winner.0,
            reindeer[points_winner_idx].name()
        );
        println!("   This shows why Part 2's second-by-second scoring creates different results!");
    } else {
        println!(
            "   Same winner for both parts - {} dominates both distance and points!",
            distance_winner.0
        );
    }
}
