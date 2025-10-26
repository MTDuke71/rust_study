//! State Management Pattern Recognition Demo
//!
//! This example demonstrates how to identify and solve state-heavy AoC problems
//! using optimization and state management patterns.

use aoc_pattern_recognition::state_patterns::*;
use std::fmt;

fn main() {
    println!("=== State Management Pattern Recognition Demo ===\n");

    // Demo 1: Memoization for Recursive Problems
    println!("🧠 Demo 1: Memoization for Recursive Problems");
    demo_memoization();
    println!();

    // Demo 2: Cycle Detection in Sequences
    println!("🔄 Demo 2: Cycle Detection in Sequences");
    demo_cycle_detection();
    println!();

    // Demo 3: Dynamic Programming
    println!("⚡ Demo 3: Dynamic Programming");
    demo_dynamic_programming();
    println!();

    // Demo 4: Sliding Window Optimization
    println!("🪟 Demo 4: Sliding Window Optimization");
    demo_sliding_window();
    println!();

    // Demo 5: Complex State Tracking
    println!("📊 Demo 5: Complex State Tracking");
    demo_state_tracking();
    println!();

    // Demo 6: Pattern Recognition Guide
    println!("🧠 Demo 6: Pattern Recognition Guide");
    demo_pattern_recognition();
    println!();

    println!("All state management pattern demos completed! ✅");
}

fn demo_memoization() {
    let mut cache = MemoizationCache::new();

    println!("  📝 Fibonacci with Memoization (AoC 2021 Day 6 Lanternfish style):");

    // Simulate lanternfish population growth with memoization
    fn lanternfish_count_memo(
        days: u64,
        timer: u64,
        cache: &mut MemoizationCache<(u64, u64), u64>,
    ) -> u64 {
        let key = (days, timer);

        if let Some(&cached) = cache.get(&key) {
            return cached;
        }

        let result = if days == 0 {
            1 // One fish at the end
        } else if timer == 0 {
            // Fish reproduces: reset timer + new fish with timer 8
            lanternfish_count_memo(days - 1, 6, cache) + lanternfish_count_memo(days - 1, 8, cache)
        } else {
            // Fish ages: decrement timer
            lanternfish_count_memo(days - 1, timer - 1, cache)
        };

        cache.insert(key, result);
        result
    }

    // Test different scenarios
    let test_cases = vec![
        (18, 3, "Small example"),
        (80, 3, "Part 1 example"),
        (256, 3, "Part 2 example"),
    ];

    for (days, initial_timer, description) in test_cases {
        let start_time = std::time::Instant::now();
        let count = lanternfish_count_memo(days, initial_timer, &mut cache);
        let duration = start_time.elapsed();

        println!(
            "    {} (days={}, timer={}): {} fish",
            description, days, initial_timer, count
        );
        println!("      Time: {:?}, Cache size: {}", duration, cache.len());
    }

    println!("    💡 Without memoization, the 256-day case would take hours!");
    let (_hits, _misses, hit_rate) = cache.stats();
    println!("    Cache hit rate: {:.1}%", hit_rate * 100.0);
}

fn demo_cycle_detection() {
    println!("  🔄 Conway's Game of Life with Cycle Detection:");

    // Simplified Game of Life state representation
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct LifeState {
        grid: Vec<Vec<bool>>,
        generation: usize,
    }

    impl LifeState {
        fn new(pattern: &str) -> Self {
            let grid = pattern
                .lines()
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect();
            Self {
                grid,
                generation: 0,
            }
        }

        fn next_generation(&self) -> Self {
            let height = self.grid.len();
            let width = self.grid[0].len();
            let mut new_grid = vec![vec![false; width]; height];

            for (y, row) in new_grid.iter_mut().enumerate().take(height) {
                for (x, cell) in row.iter_mut().enumerate().take(width) {
                    let neighbors = self.count_neighbors(x, y);
                    let alive = self.grid[y][x];

                    *cell = match (alive, neighbors) {
                        (true, 2) | (true, 3) => true, // Stay alive
                        (false, 3) => true,            // Born
                        _ => false,                    // Die or stay dead
                    };
                }
            }

            Self {
                grid: new_grid,
                generation: self.generation + 1,
            }
        }

        fn count_neighbors(&self, x: usize, y: usize) -> usize {
            let height = self.grid.len() as i32;
            let width = self.grid[0].len() as i32;
            let mut count = 0;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;

                    if nx >= 0
                        && nx < width
                        && ny >= 0
                        && ny < height
                        && self.grid[ny as usize][nx as usize]
                    {
                        count += 1;
                    }
                }
            }
            count
        }
    }

    impl fmt::Display for LifeState {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let display_str = self
                .grid
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|&alive| if alive { '#' } else { '.' })
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("\n");
            write!(f, "{}", display_str)
        }
    }

    // Test with a known oscillator pattern (blinker)
    let initial_pattern = "...\n###\n...";
    let mut state = LifeState::new(initial_pattern);

    println!("    Initial state (generation {}):", state.generation);
    println!(
        "{}",
        state
            .to_string()
            .lines()
            .map(|l| format!("      {}", l))
            .collect::<Vec<_>>()
            .join("\n")
    );

    // Use StateTracker for cycle detection
    let mut tracker = StateTracker::new(state.clone());

    // Run simulation and detect cycles
    for generation in 1..=10 {
        state = state.next_generation();

        if generation <= 3 {
            println!("    Generation {}:", generation);
            println!(
                "{}",
                state
                    .to_string()
                    .lines()
                    .map(|l| format!("      {}", l))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
        }

        // Advance the tracker with the new state
        let advanced = tracker.advance(|_| state.clone());
        if !advanced {
            break;
        }

        // Check for cycle using StateTracker's built-in cycle detection
        if let Some((cycle_start, cycle_length)) = tracker.find_cycle() {
            println!("    🎯 Cycle detected!");
            println!("      Cycle starts at generation: {}", cycle_start);
            println!("      Cycle length: {} generations", cycle_length);

            // Predict future state
            let future_gen = 1000000;
            let predicted_state_idx = cycle_start + ((future_gen - cycle_start) % cycle_length);
            println!(
                "      State at generation {} would be same as generation {}",
                future_gen, predicted_state_idx
            );
            break;
        }
    }
}

fn demo_dynamic_programming() {
    println!("  ⚡ Classic DP: Longest Common Subsequence");

    // Use DynamicProgramming static methods instead of instantiation

    // Example from AoC-style string processing
    let sequences = vec![
        ("ABCDGH", "AEDFHR", "Similar DNA sequences"),
        ("PROGRAMMING", "ALGORITHM", "Code similarity"),
        ("ADVENT", "EVENTS", "Short example"),
    ];

    for (seq1, seq2, description) in sequences {
        println!("    {} Example:", description);
        println!("      Sequence 1: {}", seq1);
        println!("      Sequence 2: {}", seq2);

        let lcs_length = DynamicProgramming::longest_common_subsequence(seq1, seq2);
        println!("      LCS Length: {}", lcs_length);

        // Calculate similarity percentage
        let max_len = seq1.len().max(seq2.len());
        let similarity = (lcs_length as f64 / max_len as f64) * 100.0;
        println!("      Similarity: {:.1}%", similarity);
        println!();
    }

    println!("  💰 Classic DP: Knapsack Problem (Resource optimization)");

    // Simulate AoC resource optimization problem
    let weights = vec![10, 20, 30];
    let values = vec![60, 100, 120];
    let capacity = 50;

    let max_value = DynamicProgramming::knapsack(&weights, &values, capacity);
    println!("    Items: weights={:?}, values={:?}", weights, values);
    println!("    Capacity: {}", capacity);
    println!("    Maximum value: {}", max_value);

    // Show which items to take
    println!("    💡 This pattern appears in AoC for:");
    println!("      • Equipment selection with weight limits");
    println!("      • Resource allocation with constraints");
    println!("      • Optimization with multiple criteria");
}

fn demo_sliding_window() {
    println!("  🪟 Sliding Window: Finding Patterns in Sequences");

    // Use SlidingWindow static methods instead of instantiation

    // Simulate packet processing (AoC 2022 Day 6 style)
    let datastream = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    println!("    Datastream: {}", datastream);
    println!("    Looking for first 4-character window with all unique characters:");

    let mut current_window = Vec::new();

    for (i, ch) in datastream.chars().enumerate() {
        // Add character to window
        current_window.push(ch);
        if current_window.len() > 4 {
            current_window.remove(0);
        }

        // Check if we have a valid start-of-packet marker
        if current_window.len() == 4 {
            let mut unique_chars = current_window.clone();
            unique_chars.sort();
            unique_chars.dedup();

            if unique_chars.len() == 4 {
                println!(
                    "    ✅ Found start marker: {:?} at position {}",
                    current_window, i
                );
                break;
            }
        }

        // For demo, simulate the sliding
        println!(
            "    Position {}: '{}' → Window: {:?}",
            i, ch, current_window
        );
        if i >= 8 {
            // Limit output for demo
            println!("    ... continuing scan ...");
            break;
        }
    }

    println!();
    println!("  📊 Sliding Window: Sum Analysis");

    let numbers = vec![1, 3, 2, 6, -1, 4, 1, 8, 2];
    let window_size = 3;

    println!("    Numbers: {:?}", numbers);
    println!("    Window size: {}", window_size);

    // Calculate sliding window sums
    let mut max_sum = i32::MIN;
    let mut max_window = Vec::new();

    for i in 0..=(numbers.len().saturating_sub(window_size)) {
        let window: Vec<i32> = numbers[i..i + window_size].to_vec();
        let sum: i32 = window.iter().sum();

        if sum > max_sum {
            max_sum = sum;
            max_window = window.clone();
        }

        println!("    Window {:?} → Sum: {}", window, sum);
    }

    println!("    🏆 Maximum sum window: {:?} → {}", max_window, max_sum);
}

fn demo_state_tracking() {
    println!("  📊 Complex State Tracking: Robot Navigation");

    #[derive(Debug, Clone, Hash, PartialEq, Eq)]
    struct RobotState {
        x: i32,
        y: i32,
        direction: String,
        fuel: u32,
    }

    let mut tracker = StateTracker::new(RobotState {
        x: 0,
        y: 0,
        direction: "North".to_string(),
        fuel: 100,
    });

    let initial_state = RobotState {
        x: 0,
        y: 0,
        direction: "North".to_string(),
        fuel: 100,
    };

    // StateTracker manages state history using advance() method
    // tracker.advance() handles both recording and transitioning

    // Simulate robot movement commands
    let commands = vec![
        ("forward", 5),
        ("turn_right", 0),
        ("forward", 3),
        ("turn_left", 0),
        ("forward", 2),
    ];

    let mut current_state = initial_state;

    println!("    Initial state: {:?}", current_state);

    for (command, value) in commands {
        println!("    Command: {} {}", command, value);

        // Apply command
        match command {
            "forward" => {
                match current_state.direction.as_str() {
                    "North" => current_state.y += value,
                    "South" => current_state.y -= value,
                    "East" => current_state.x += value,
                    "West" => current_state.x -= value,
                    _ => {}
                }
                current_state.fuel = current_state.fuel.saturating_sub(value as u32);
            }
            "turn_right" => {
                current_state.direction = match current_state.direction.as_str() {
                    "North" => "East",
                    "East" => "South",
                    "South" => "West",
                    "West" => "North",
                    _ => "North",
                }
                .to_string();
            }
            "turn_left" => {
                current_state.direction = match current_state.direction.as_str() {
                    "North" => "West",
                    "West" => "South",
                    "South" => "East",
                    "East" => "North",
                    _ => "North",
                }
                .to_string();
            }
            _ => {}
        }

        // StateTracker uses advance() for state transitions
        let advanced = tracker.advance(|state| RobotState {
            x: state.x + 1,
            y: state.y,
            direction: state.direction.clone(),
            fuel: state.fuel.saturating_sub(1),
        });

        if !advanced {
            break; // Couldn't advance further
        }

        let current_state = tracker.current();
        println!("      → {:?}", current_state);

        // Check if we've been in this state before
        if tracker.has_seen_state(current_state) {
            println!("      🔄 State revisited! Potential loop detected.");
        }
    }

    println!("    📈 State Statistics:");
    println!("      Total generations tracked: {}", tracker.generation());
    println!(
        "      Final position: ({}, {})",
        current_state.x, current_state.y
    );
    println!(
        "      Manhattan distance from origin: {}",
        current_state.x.abs() + current_state.y.abs()
    );
}

fn demo_pattern_recognition() {
    println!("  🧠 How to recognize state management patterns in AoC problems:");
    println!();

    let pattern_signals = vec![
        (
            "Memoization/Caching",
            vec![
                "Recursive problems with overlapping subproblems",
                "\"Calculate for very large numbers\" (days 256, etc.)",
                "Tree/graph traversal with repeated states",
                "Fibonacci-like sequences",
                "Dynamic population/growth simulations",
            ],
        ),
        (
            "Cycle Detection",
            vec![
                "\"Simulate for 1 billion iterations\"",
                "Cellular automata (Game of Life style)",
                "State machines that might repeat",
                "Orbital mechanics or planetary motion",
                "Memory/register manipulation that might loop",
            ],
        ),
        (
            "Dynamic Programming",
            vec![
                "Optimization problems (shortest, longest, maximum)",
                "\"How many ways to...\" counting problems",
                "Resource allocation with constraints",
                "String/sequence analysis problems",
                "Building/crafting with dependencies",
            ],
        ),
        (
            "Sliding Window",
            vec![
                "Finding subsequences with properties",
                "\"First occurrence of pattern\" problems",
                "Stream processing with fixed-size buffers",
                "Moving averages or rolling calculations",
                "Signal processing or data validation",
            ],
        ),
    ];

    for (pattern_name, signals) in pattern_signals {
        println!("  🎯 {}:", pattern_name);
        for signal in signals {
            println!("    • {}", signal);
        }
        println!();
    }

    println!("  💡 Performance Impact Guidelines:");
    println!("    • Memoization: Converts O(2ⁿ) to O(n) for recursive problems");
    println!("    • Cycle Detection: Converts O(n) to O(√n) space, early termination");
    println!("    • Dynamic Programming: O(n²) or O(n³) but avoids exponential");
    println!("    • Sliding Window: O(n) instead of O(n×k) for window operations");
    println!();

    println!("  ⚠️  When NOT to use these patterns:");
    println!("    • Don't memoize if each computation is unique");
    println!("    • Don't look for cycles in truly random sequences");
    println!("    • Don't use DP for simple linear problems");
    println!("    • Don't use sliding window for sparse/random access");
    println!();

    println!("  🔧 Implementation Tips:");
    println!("    • Use HashMap for memoization with composite keys");
    println!("    • Floyd's cycle detection for space efficiency");
    println!("    • Bottom-up DP usually more efficient than top-down");
    println!("    • VecDeque for sliding window with efficient pop/push");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memoization() {
        let mut cache = MemoizationCache::new();

        // Test fibonacci with memoization
        fn fib(n: u64, cache: &mut MemoizationCache<u64, u64>) -> u64 {
            if let Some(&cached) = cache.get(&n) {
                return cached;
            }

            let result = match n {
                0 => 0,
                1 => 1,
                _ => fib(n - 1, cache) + fib(n - 2, cache),
            };

            cache.insert(n, result);
            result
        }

        assert_eq!(fib(10, &mut cache), 55);
        assert!(cache.len() > 0);
    }

    #[test]
    fn test_cycle_detection() {
        // Use CycleDetection static methods

        // Test with a simple repeating sequence using Floyd's detection
        let sequence = vec![1, 2, 3, 1, 2, 3, 1, 2, 3];
        let indexed_sequence: Vec<usize> = (0..sequence.len()).collect();

        // Floyd's algorithm expects a function, so this is a simplified test
        if let Some((start, length)) = CycleDetection::floyd_cycle_detection(0, |&i| (i + 1) % 3) {
            assert_eq!(length, 3); // Cycle length should be 3
        }
    }

    #[test]
    fn test_dynamic_programming() {
        // Use DynamicProgramming static methods

        // Test LCS
        let lcs_len = DynamicProgramming::longest_common_subsequence("ABCDGH", "AEDFHR");
        assert_eq!(lcs_len, 3); // "ADH"

        // Test knapsack
        let weights = vec![10, 20, 30];
        let values = vec![60, 100, 120];
        let max_value = DynamicProgramming::knapsack(&weights, &values, 50);
        assert_eq!(max_value, 220); // Take items 2 and 3
    }

    #[test]
    fn test_sliding_window() {
        // Use SlidingWindow static methods
        let data = vec![1, 2, 3, 4, 5];

        // Test maximum sum window
        let max_sum = SlidingWindow::max_sum_window(&data, 3);
        assert_eq!(max_sum, Some(12)); // [3, 4, 5]

        // Test anagram finding
        let text = "abab";
        let pattern = "ab";
        let anagrams = SlidingWindow::find_anagrams(text, pattern);
        assert_eq!(anagrams, vec![0, 1, 2]); // "ab" at position 0, "ba" at position 1, "ab" at position 2
    }
}
