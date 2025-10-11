//! Interactive Pattern Training Demo
//!
//! This example demonstrates the interactive training system for developing
//! pattern recognition skills for Advent of Code problems.

use aoc_pattern_recognition::pattern_trainer::*;

fn main() {
    println!("=== Interactive Pattern Training Demo ===\n");

    // Demo 1: Pattern Recognition Training
    println!("🎓 Demo 1: Pattern Recognition Training");
    demo_pattern_training();
    println!();

    // Demo 2: Interactive Pattern Quiz
    println!("🧠 Demo 2: Interactive Pattern Quiz");
    demo_pattern_quiz();
    println!();

    // Demo 3: Training Exercises
    println!("💪 Demo 3: Training Exercises");
    demo_training_exercises();
    println!();

    // Demo 4: Performance Benchmarking
    println!("⚡ Demo 4: Performance Benchmarking");
    demo_pattern_benchmarking();
    println!();

    // Demo 5: Comprehensive Training Session
    println!("🏆 Demo 5: Comprehensive Training Session");
    demo_training_session();
    println!();

    println!("Interactive pattern training demo completed! ✅");
}

fn demo_pattern_training() {
    let _trainer = PatternTrainer::new();

    println!("  📚 Pattern Training System");
    println!("    Interactive exercises to master AoC pattern recognition");
    println!();

    // Simulate accessing training exercises
    println!("  🎯 Available Training Categories:");
    println!("    1. Grid Patterns - Navigation, flood fill, pathfinding");
    println!("    2. Parsing Patterns - Input transformation and extraction");
    println!("    3. State Patterns - Memoization, cycle detection, caching");
    println!("    4. Optimization Patterns - Dynamic programming, graph algorithms");
    println!();

    println!("  � Sample Training Exercise:");
    println!("    Title: Basic Grid Navigation");
    println!("    Description: Navigate from point A to B in a 2D grid");
    println!("    Difficulty: Beginner");
    println!("    Pattern Focus: Grid traversal, coordinate systems");
    println!();

    // Simulate training progress
    let grid_exercises = vec![
        (
            "Basic Grid Navigation",
            "Navigate from point A to B in a 2D grid",
            95,
        ),
        (
            "Shortest Path Finding",
            "Find shortest path avoiding obstacles",
            87,
        ),
        (
            "Connected Components",
            "Count separate regions in a map",
            92,
        ),
        ("Grid Transformations", "Rotate and flip grid patterns", 78),
    ];

    println!("  📈 Training Progress Simulation:");
    for (exercise_name, description, success_rate) in grid_exercises {
        println!("    🔍 Exercise: {}", exercise_name);
        println!("      Description: {}", description);
        println!("      Result: {}% success rate", success_rate);

        if success_rate >= 90 {
            println!("      Status: ✅ Mastered");
        } else if success_rate >= 80 {
            println!("      Status: 📈 Good progress");
        } else {
            println!("      Status: 📚 Needs practice");
        }
        println!();
    }

    println!("  📊 Training Summary:");
    println!("    Overall Progress: 88% (Good)");
    println!("    Strengths: Basic navigation, component counting");
    println!("    Focus Areas: Grid transformations, complex pathfinding");
    println!("    Next Steps: Practice with AoC 2020 Day 20, 2021 Day 15");
}

fn demo_pattern_quiz() {
    let quiz = PatternQuiz::new();

    println!("  🧠 Pattern Recognition Quiz");
    println!("  Interactive quiz to test pattern identification skills");
    println!();

    // Sample quiz questions based on real AoC problems - manually created since
    // the actual quiz structure is different
    let sample_questions = vec![
        (
            "2D Grid Pathfinding",
            "You have a 2D grid representing a cave system. Each cell contains a risk level. Find the path from top-left to bottom-right with the lowest total risk.",
            vec!["BFS", "Dijkstra's Algorithm", "A* Search", "DFS"],
            1, // Dijkstra's is correct
            "AoC 2021 Day 15"
        ),
        (
            "Cycle Detection",
            "You need to simulate 1 billion iterations of a cellular automaton. Each generation follows specific birth/death rules.",
            vec!["Brute Force", "Cycle Detection", "Memoization", "Dynamic Programming"],
            1, // Cycle Detection is correct
            "AoC 2015 Day 18"
        ),
        (
            "Parsing Mixed Input",
            "Parse input containing coordinates like 'x=123, y=456' and instructions like 'fold along x=655'.",
            vec!["Single Parser", "Multiple Parsers", "Regex Only", "Manual String Splitting"],
            1, // Multiple Parsers is correct
            "AoC 2021 Day 13"
        ),
    ];

    for (i, (category, problem, options, correct_idx, aoc_ref)) in
        sample_questions.iter().enumerate()
    {
        println!("  📋 Question {}: {}", i + 1, category);
        println!("    Problem: {}", problem);
        println!("    Reference: {}", aoc_ref);
        println!("    Options:");

        for (j, option) in options.iter().enumerate() {
            let marker = if j == *correct_idx { "✅" } else { "  " };
            println!("      {}{}. {}", marker, j + 1, option);
        }

        println!("    💡 Analysis:");
        match i {
            0 => {
                println!("      • '2D grid' + 'path' + 'lowest cost' → Shortest Path");
                println!("      • Dijkstra's algorithm handles weighted edges");
                println!("      • BFS works for unweighted, A* adds heuristics");
            }
            1 => {
                println!("      • '1 billion iterations' → Too many to simulate directly");
                println!("      • 'cellular automaton' → State patterns often repeat");
                println!("      • Cycle detection finds patterns to predict future states");
            }
            2 => {
                println!("      • Mixed input types (coordinates + instructions)");
                println!("      • Each format needs specialized parsing");
                println!("      • Multiple parsers handle different patterns");
            }
            _ => {}
        }
        println!();
    }

    // Simulate quiz results
    println!("  📊 Quiz Results Simulation:");
    println!("    Score: 85/100");
    println!("    Time: 12 minutes");
    println!("    Correct: 3/3");
    println!("    Status: ✅ Excellent pattern recognition!");

    // Show what actual quiz functionality would provide
    if let Some(_current_q) = quiz.current_question() {
        println!("  📝 Real Quiz System Features:");
        println!("    • Interactive questions with multiple choice");
        println!("    • Detailed explanations for each answer");
        println!("    • Progress tracking and scoring");
        println!("    • Adaptive difficulty based on performance");
    }
}

fn demo_training_exercises() {
    println!("  💪 Hands-on Training Exercises");
    println!();

    // Exercise 1: Grid Pattern Recognition
    println!("  🔷 Exercise 1: Grid Pattern Implementation");
    let exercise1 = TrainingExercise {
        id: 1,
        title: "Basic Flood Fill".to_string(),
        description: "Implement flood fill to count connected regions".to_string(),
        input_sample: "##.##\n#...#\n.....\n#...#\n##.##".to_string(),
        expected_patterns: vec!["Grid Navigation".to_string(), "DFS/BFS".to_string()],
        difficulty: TrainingDifficulty::Intermediate,
        aoc_reference: Some("AoC 2017 Day 12".to_string()),
    };

    println!("    Title: {}", exercise1.title);
    println!("    Description: {}", exercise1.description);
    println!("    Difficulty: {:?}", exercise1.difficulty);
    println!("    Expected Patterns: {:?}", exercise1.expected_patterns);

    println!("    Sample Input:");
    for line in exercise1.input_sample.lines() {
        println!("        {}", line);
    }
    println!("      Expected: 3 regions (water areas marked with '.')");
    println!("      Hint: Use DFS or BFS from each '.' not yet visited");

    if let Some(ref aoc_ref) = exercise1.aoc_reference {
        println!("      AoC Reference: {}", aoc_ref);
    }
    println!();

    // Exercise 2: Parsing Pattern
    println!("  🔷 Exercise 2: Parsing Pattern Implementation");
    let exercise2 = TrainingExercise {
        id: 2,
        title: "Coordinate Extraction".to_string(),
        description: "Extract all coordinates from mixed input format".to_string(),
        input_sample: "Sensor at x=2, y=18: closest beacon is at x=-2, y=15".to_string(),
        expected_patterns: vec!["Coordinate Parsing".to_string(), "Regex".to_string()],
        difficulty: TrainingDifficulty::Beginner,
        aoc_reference: Some("AoC 2022 Day 15".to_string()),
    };

    println!("    Title: {}", exercise2.title);
    println!("    Description: {}", exercise2.description);
    println!("    Difficulty: {:?}", exercise2.difficulty);

    println!("    Sample Input: {}", exercise2.input_sample);
    println!("      Expected: [(2,18), (-2,15)]");
    println!("      Hint: Use regex to find 'x=NUMBER, y=NUMBER' patterns");

    if let Some(ref aoc_ref) = exercise2.aoc_reference {
        println!("      AoC Reference: {}", aoc_ref);
    }
    println!();

    // Exercise 3: State Management
    println!("  🔷 Exercise 3: State Management Implementation");
    let exercise3 = TrainingExercise {
        id: 3,
        title: "Fibonacci with Memoization".to_string(),
        description: "Implement memoized fibonacci for large numbers".to_string(),
        input_sample: "50".to_string(),
        expected_patterns: vec!["Memoization".to_string(), "Dynamic Programming".to_string()],
        difficulty: TrainingDifficulty::Intermediate,
        aoc_reference: Some("AoC Pattern - State Management".to_string()),
    };

    println!("    Title: {}", exercise3.title);
    println!("    Description: {}", exercise3.description);
    println!("    Difficulty: {:?}", exercise3.difficulty);

    println!("    Sample Input: fib({})", exercise3.input_sample);
    println!("      Expected: 12586269025");
    println!("      Hint: Use HashMap<u64, u64> to cache results");
    println!("      Challenge: Try fib(100) - impossible without memoization!");

    if let Some(ref aoc_ref) = exercise3.aoc_reference {
        println!("      Reference: {}", aoc_ref);
    }
    println!();

    println!("  📈 Exercise Progression:");
    println!("    1. Complete basic exercises in each pattern category");
    println!("    2. Tackle medium difficulty multi-pattern problems");
    println!("    3. Solve hard exercises combining 3+ patterns");
    println!("    4. Practice with actual AoC problems from past years");
    println!("    5. Time yourself solving new problems under pressure");

    // Show time limits based on difficulty
    println!("  ⏱️  Time Limits by Difficulty:");
    for difficulty in [
        TrainingDifficulty::Beginner,
        TrainingDifficulty::Intermediate,
        TrainingDifficulty::Advanced,
        TrainingDifficulty::Expert,
    ] {
        println!(
            "    {:?}: {} seconds",
            difficulty,
            difficulty.time_limit_seconds()
        );
    }
}

fn demo_pattern_benchmarking() {
    println!("  ⚡ Pattern Performance Benchmarking");
    println!();

    // Demonstrate benchmark functionality
    println!("  🔍 Benchmark: Parsing Pattern Performance");
    println!("    Testing parsing performance on sample input");

    // Sample input for benchmarking
    let input = "1,2,3\n4,5,6\n7,8,9\n10,11,12";

    // Run actual benchmark
    match PatternBenchmark::benchmark_parsing(input, 1000) {
        Ok(results) => {
            println!("  📊 Benchmark Results:");
            for (pattern_name, avg_nanoseconds) in results.results() {
                let avg_microseconds = avg_nanoseconds / 1000.0;
                println!("    {}: {:.2} μs average", pattern_name, avg_microseconds);
            }

            if let Some((fastest_name, fastest_time)) = results.fastest() {
                println!(
                    "    ✅ Fastest: {} ({:.2} μs)",
                    fastest_name,
                    fastest_time / 1000.0
                );
            }

            if let Some((slowest_name, slowest_time)) = results.slowest() {
                println!(
                    "    ⏳ Slowest: {} ({:.2} μs)",
                    slowest_name,
                    slowest_time / 1000.0
                );
            }
        }
        Err(e) => {
            println!("    ❌ Benchmarking failed: {:?}", e);
        }
    }
    println!();

    // Simulated performance analysis for educational purposes
    println!("  📈 Performance Insights (Simulated):");

    println!("  🧠 State Management Patterns:");
    println!("    Naive Recursion (fib 40): 2500ms");
    println!("    Memoized Recursion: 1ms (2500x speedup!)");
    println!("    Bottom-up DP: 1ms, minimal memory");
    println!("    💡 Key Insight: Memoization provides exponential speedup");
    println!();

    println!("  📝 Parsing Pattern Performance:");
    println!("    Regex Parsing: 120ms, 8.5MB (99.8% success)");
    println!("    Manual Parsing: 45ms, 3.2MB (100% success)");
    println!("    State Machine: 35ms, 2.8MB (100% success)");
    println!("    🎯 Recommendation: State machine for complex formats");
    println!();

    println!("  🔍 Pathfinding Algorithms:");
    println!("    BFS (unweighted): 15ms, 2.5MB");
    println!("    Dijkstra (weighted): 45ms, 4.2MB");
    println!("    A* Search: 25ms, 3.1MB");
    println!("    🏆 Winner: BFS for unweighted grids, A* for weighted");
    println!();

    println!("  📊 Benchmarking Best Practices:");
    println!("    • Test with realistic AoC input sizes (1K-100K elements)");
    println!("    • Measure both time and memory usage");
    println!("    • Consider worst-case scenarios");
    println!("    • Benchmark different implementations of same pattern");
    println!("    • Account for cache effects in repeated benchmarks");
}

fn demo_training_session() {
    println!("  🏆 Complete Training Session: AoC Problem Simulation");
    println!();

    // Simulate a complete training session with a mock AoC problem
    println!("  📋 Mock Problem: \"Digital Signage Network\"");
    println!("  (Based on AoC 2023 Day 16 style problem)");
    println!();

    let problem_statement = r#"
    You have a 2D grid representing a digital signage network. Light beams enter 
    the grid and bounce off mirrors. Calculate how many tiles are energized.

    Grid symbols:
    - '.' = empty space (light passes through)
    - '/' = mirror (reflects light 90 degrees)
    - '\' = mirror (reflects light 90 degrees)  
    - '|' = vertical splitter
    - '-' = horizontal splitter

    Light starts at top-left corner moving right.
    "#;

    println!("  Problem Description:");
    for line in problem_statement.trim().lines() {
        println!("    {}", line);
    }
    println!();

    // Pattern recognition phase
    println!("  🧠 Step 1: Pattern Recognition");
    println!("    Keywords: '2D grid', 'light beams', 'bounce', 'tiles energized'");
    println!("    Identified Patterns:");
    println!("      ✅ Grid parsing and navigation");
    println!("      ✅ State tracking (position + direction)");
    println!("      ✅ Cycle detection (prevent infinite loops)");
    println!("      ✅ Simulation with rules");
    println!();

    // Implementation strategy
    println!("  🔧 Step 2: Implementation Strategy");
    println!("    1. Parse grid from input string");
    println!("    2. Track beam state: (x, y, direction)");
    println!("    3. Implement reflection/splitting rules");
    println!("    4. Use HashSet to track visited states");
    println!("    5. Use HashSet to track energized tiles");
    println!("    6. Detect cycles to prevent infinite simulation");
    println!();

    // Complexity analysis
    println!("  📊 Step 3: Complexity Analysis");
    println!("    Time: O(W × H × 4) - worst case visit each cell in each direction");
    println!("    Space: O(W × H) - track energized tiles and visited states");
    println!("    Pattern Complexity: Grid (Linear) + State (Linear)");
    println!();

    // Implementation outline
    println!("  💻 Step 4: Implementation Outline");
    println!("    ```rust");
    println!("    struct BeamState {{ x: i32, y: i32, direction: Direction }}");
    println!("    ");
    println!("    fn simulate_beam(grid: &Grid) -> usize {{");
    println!("        let mut visited_states = HashSet::new();");
    println!("        let mut energized = HashSet::new();");
    println!("        let mut beams = vec![BeamState::new(0, 0, Right)];");
    println!("        ");
    println!("        while let Some(beam) = beams.pop() {{");
    println!("            if visited_states.contains(&beam) {{ continue; }}");
    println!("            visited_states.insert(beam.clone());");
    println!("            energized.insert((beam.x, beam.y));");
    println!("            ");
    println!("            // Apply grid rules and generate new beams");
    println!("            beams.extend(apply_rules(&grid, &beam));");
    println!("        }}");
    println!("        ");
    println!("        energized.len()");
    println!("    }}");
    println!("    ```");
    println!();

    // Testing strategy
    println!("  🧪 Step 5: Testing Strategy");
    println!("    Unit Tests:");
    println!("      • Grid parsing from string");
    println!("      • Beam reflection rules");
    println!("      • Beam splitting rules");
    println!("      • Cycle detection");
    println!("    ");
    println!("    Integration Tests:");
    println!("      • Small example grids");
    println!("      • Edge cases (beam exits grid)");
    println!("      • Performance with large grids");
    println!();

    // Performance optimization
    println!("  ⚡ Step 6: Performance Optimization");
    println!("    Potential Optimizations:");
    println!("      • Use Vec<bool> instead of HashSet for grid-sized data");
    println!("      • Implement efficient beam queue (VecDeque)");
    println!("      • Early termination when all tiles energized");
    println!("      • Parallel processing for multiple starting positions");
    println!();

    // Training evaluation
    println!("  📈 Training Session Evaluation:");
    println!("    Pattern Recognition: ✅ Correctly identified 4 core patterns");
    println!("    Implementation Strategy: ✅ Clear step-by-step approach");
    println!("    Complexity Analysis: ✅ Realistic time/space estimates");
    println!("    Code Structure: ✅ Modular, testable design");
    println!("    Optimization Awareness: ✅ Identified performance considerations");
    println!();
    println!("    🏆 Overall Score: 95/100 - Excellent pattern application!");
    println!("    📝 Next Steps: Practice similar problems (AoC 2022 Day 14, 2021 Day 25)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_trainer_creation() {
        let _trainer = PatternTrainer::new();
        // Test that trainer can be created successfully
        // The trainer should have built-in exercises
    }

    #[test]
    fn test_pattern_quiz_creation() {
        let quiz = PatternQuiz::new();
        // Test that quiz can be created successfully
        if let Some(question) = quiz.current_question() {
            assert!(!question.question.is_empty());
            assert!(!question.options.is_empty());
            assert!(!question.explanation.is_empty());
        }
    }

    #[test]
    fn test_training_exercise_structure() {
        let exercise = TrainingExercise {
            id: 1,
            title: "Test Exercise".to_string(),
            description: "Test description".to_string(),
            input_sample: "test input".to_string(),
            expected_patterns: vec!["Test Pattern".to_string()],
            difficulty: TrainingDifficulty::Beginner,
            aoc_reference: Some("AoC Test".to_string()),
        };

        assert_eq!(exercise.id, 1);
        assert!(!exercise.title.is_empty());
        assert!(!exercise.expected_patterns.is_empty());
        assert_eq!(exercise.difficulty, TrainingDifficulty::Beginner);
    }

    #[test]
    fn test_benchmark_creation() {
        let input = "1,2,3\n4,5,6\n7,8,9";
        let results = PatternBenchmark::benchmark_parsing(input, 10).unwrap();

        // Results should contain at least one benchmark
        assert!(!results.results().is_empty());

        // Should be able to find fastest and slowest
        assert!(results.fastest().is_some());
        assert!(results.slowest().is_some());
    }

    #[test]
    fn test_training_difficulty_time_limits() {
        assert_eq!(TrainingDifficulty::Beginner.time_limit_seconds(), 300);
        assert_eq!(TrainingDifficulty::Intermediate.time_limit_seconds(), 600);
        assert_eq!(TrainingDifficulty::Advanced.time_limit_seconds(), 1200);
        assert_eq!(TrainingDifficulty::Expert.time_limit_seconds(), 1800);
    }
}
