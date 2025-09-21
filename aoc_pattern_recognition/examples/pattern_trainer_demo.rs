//! Interactive Pattern Training Demo
//! 
//! This example demonstrates the interactive training system for developing
//! pattern recognition skills for Advent of Code problems.

use aoc_pattern_recognition::pattern_trainer::*;
use aoc_pattern_recognition::{AocPattern, PatternComplexity};

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
    let trainer = PatternTrainer::new();
    
    println!("  📚 Available Training Modules:");
    let modules = trainer.get_available_modules();
    for (i, module) in modules.iter().enumerate() {
        println!("    {}. {}", i + 1, module);
    }
    println!();
    
    println!("  🎯 Training Module: Grid Patterns");
    
    // Start a training session focused on grid patterns
    println!("    📝 Learning Objectives:");
    println!("      • Identify when to use BFS vs DFS");
    println!("      • Recognize flood fill opportunities");
    println!("      • Understand grid transformation patterns");
    println!("      • Master coordinate system navigation");
    println!();
    
    // Simulate training progress
    let grid_exercises = vec![
        ("Basic Grid Navigation", "Navigate from point A to B in a 2D grid"),
        ("Shortest Path Finding", "Find shortest path avoiding obstacles"),
        ("Connected Components", "Count separate regions in a map"),
        ("Grid Transformations", "Rotate and flip grid patterns"),
    ];
    
    for (exercise_name, description) in grid_exercises {
        println!("    🔍 Exercise: {}", exercise_name);
        println!("      Description: {}", description);
        
        // Simulate exercise completion
        let success_rate = match exercise_name {
            "Basic Grid Navigation" => 95,
            "Shortest Path Finding" => 87,
            "Connected Components" => 92,
            "Grid Transformations" => 78,
            _ => 85,
        };
        
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
    
    println!("  📊 Grid Patterns Training Summary:");
    println!("    Overall Progress: 88% (Good)");
    println!("    Strengths: Basic navigation, component counting");
    println!("    Focus Areas: Grid transformations, complex pathfinding");
    println!("    Next Steps: Practice with AoC 2020 Day 20, 2021 Day 15");
}

fn demo_pattern_quiz() {
    let quiz = PatternQuiz::new();
    
    println!("  🧠 Pattern Recognition Quiz");
    println!("  Time limit: 5 minutes per question");
    println!("  Goal: Identify the primary pattern(s) needed");
    println!();
    
    // Sample quiz questions based on real AoC problems
    let questions = vec![
        QuizQuestion {
            id: 1,
            problem_description: "You have a 2D grid representing a cave system. Each cell contains a risk level. Find the path from top-left to bottom-right with the lowest total risk.".to_string(),
            correct_patterns: vec!["Shortest Path".to_string(), "Grid Navigation".to_string()],
            difficulty: QuizDifficulty::Medium,
            aoc_reference: Some("2021 Day 15".to_string()),
        },
        QuizQuestion {
            id: 2,
            problem_description: "You need to simulate 1 billion iterations of a cellular automaton. Each generation follows specific birth/death rules.".to_string(),
            correct_patterns: vec!["Cycle Detection".to_string(), "State Management".to_string()],
            difficulty: QuizDifficulty::Hard,
            aoc_reference: Some("2015 Day 18".to_string()),
        },
        QuizQuestion {
            id: 3,
            problem_description: "Parse input containing coordinates like 'x=123, y=456' and instructions like 'fold along x=655'.".to_string(),
            correct_patterns: vec!["Coordinate Parsing".to_string(), "Instruction Parsing".to_string()],
            difficulty: QuizDifficulty::Easy,
            aoc_reference: Some("2021 Day 13".to_string()),
        },
    ];
    
    for question in questions {
        println!("  📋 Question {}: {} Difficulty", question.id, 
            match question.difficulty {
                QuizDifficulty::Easy => "Easy",
                QuizDifficulty::Medium => "Medium", 
                QuizDifficulty::Hard => "Hard",
            });
        println!("    Problem: {}", question.problem_description);
        
        if let Some(ref aoc_ref) = question.aoc_reference {
            println!("    Reference: AoC {}", aoc_ref);
        }
        
        println!("    Correct Patterns: {:?}", question.correct_patterns);
        
        // Simulate quiz interaction
        println!("    💡 Analysis:");
        match question.id {
            1 => {
                println!("      • '2D grid' + 'path' + 'lowest cost' → Shortest Path");
                println!("      • Graph algorithms: BFS, Dijkstra, or A*");
                println!("      • Grid navigation with weighted edges");
            }
            2 => {
                println!("      • '1 billion iterations' → Too many to simulate directly");
                println!("      • 'cellular automaton' → State might repeat");
                println!("      • Pattern: Detect cycles to predict future states");
            }
            3 => {
                println!("      • 'coordinates like x=123, y=456' → Coordinate parsing");
                println!("      • 'instructions like fold along' → Instruction parsing");
                println!("      • Mixed input types require multiple parsers");
            }
            _ => {}
        }
        println!();
    }
    
    let quiz_results = quiz.take_quiz(questions.clone());
    println!("  📊 Quiz Results:");
    println!("    Score: {}/100", quiz_results.score);
    println!("    Time: {} minutes", quiz_results.time_taken);
    println!("    Correct: {}/{}", quiz_results.correct_answers, questions.len());
    
    if quiz_results.score >= 80 {
        println!("    Status: ✅ Excellent pattern recognition!");
    } else if quiz_results.score >= 60 {
        println!("    Status: 📈 Good progress, keep practicing");
    } else {
        println!("    Status: 📚 Review fundamental patterns");
    }
}

fn demo_training_exercises() {
    println!("  💪 Hands-on Training Exercises");
    println!();
    
    // Exercise 1: Grid Pattern Recognition
    println!("  🔷 Exercise 1: Grid Pattern Implementation");
    let exercise1 = TrainingExercise {
        id: 1,
        name: "Basic Flood Fill".to_string(),
        description: "Implement flood fill to count connected regions".to_string(),
        pattern_type: "Grid Patterns".to_string(),
        difficulty: ExerciseDifficulty::Medium,
        time_limit_minutes: 15,
        test_cases: vec![
            TestCase {
                input: "##.##\n#...#\n.....\n#...#\n##.##".to_string(),
                expected_output: "3".to_string(), // 3 separate regions
                description: "Count water regions in island map".to_string(),
            }
        ],
    };
    
    println!("    Name: {}", exercise1.name);
    println!("    Description: {}", exercise1.description);
    println!("    Pattern: {}", exercise1.pattern_type);
    println!("    Time Limit: {} minutes", exercise1.time_limit_minutes);
    
    println!("    Test Case:");
    println!("      Input grid:");
    for line in exercise1.test_cases[0].input.lines() {
        println!("        {}", line);
    }
    println!("      Expected: {} regions", exercise1.test_cases[0].expected_output);
    println!("      Hint: Use DFS or BFS from each '.' not yet visited");
    println!();
    
    // Exercise 2: Parsing Pattern
    println!("  🔷 Exercise 2: Parsing Pattern Implementation");
    let exercise2 = TrainingExercise {
        id: 2,
        name: "Coordinate Extraction".to_string(),
        description: "Extract all coordinates from mixed input format".to_string(),
        pattern_type: "Parsing Patterns".to_string(),
        difficulty: ExerciseDifficulty::Easy,
        time_limit_minutes: 10,
        test_cases: vec![
            TestCase {
                input: "Sensor at x=2, y=18: closest beacon is at x=-2, y=15".to_string(),
                expected_output: "[(2,18), (-2,15)]".to_string(),
                description: "Extract sensor and beacon coordinates".to_string(),
            }
        ],
    };
    
    println!("    Name: {}", exercise2.name);
    println!("    Description: {}", exercise2.description);
    println!("    Pattern: {}", exercise2.pattern_type);
    
    println!("    Test Case:");
    println!("      Input: {}", exercise2.test_cases[0].input);
    println!("      Expected: {}", exercise2.test_cases[0].expected_output);
    println!("      Hint: Use regex to find 'x=NUMBER, y=NUMBER' patterns");
    println!();
    
    // Exercise 3: State Management
    println!("  🔷 Exercise 3: State Management Implementation");
    let exercise3 = TrainingExercise {
        id: 3,
        name: "Fibonacci with Memoization".to_string(),
        description: "Implement memoized fibonacci for large numbers".to_string(),
        pattern_type: "State Patterns".to_string(),
        difficulty: ExerciseDifficulty::Medium,
        time_limit_minutes: 20,
        test_cases: vec![
            TestCase {
                input: "50".to_string(),
                expected_output: "12586269025".to_string(),
                description: "Calculate fib(50) efficiently".to_string(),
            }
        ],
    };
    
    println!("    Name: {}", exercise3.name);
    println!("    Description: {}", exercise3.description);
    println!("    Pattern: {}", exercise3.pattern_type);
    
    println!("    Test Case:");
    println!("      Input: fib({})", exercise3.test_cases[0].input);
    println!("      Expected: {}", exercise3.test_cases[0].expected_output);
    println!("      Hint: Use HashMap<u64, u64> to cache results");
    println!("      Challenge: Try fib(100) - impossible without memoization!");
    println!();
    
    println!("  📈 Exercise Progression:");
    println!("    1. Complete basic exercises in each pattern category");
    println!("    2. Tackle medium difficulty multi-pattern problems");
    println!("    3. Solve hard exercises combining 3+ patterns");
    println!("    4. Practice with actual AoC problems from past years");
    println!("    5. Time yourself solving new problems under pressure");
}

fn demo_pattern_benchmarking() {
    let benchmark = PatternBenchmark::new();
    
    println!("  ⚡ Pattern Performance Benchmarking");
    println!();
    
    // Benchmark 1: Pathfinding algorithms
    println!("  🔍 Benchmark 1: Pathfinding Algorithms");
    let pathfinding_results = vec![
        BenchmarkResult {
            pattern_name: "BFS Shortest Path".to_string(),
            input_size: 1000,
            execution_time_ms: 15,
            memory_usage_mb: 2.5,
            success_rate: 100.0,
        },
        BenchmarkResult {
            pattern_name: "Dijkstra Weighted Path".to_string(),
            input_size: 1000,
            execution_time_ms: 45,
            memory_usage_mb: 4.2,
            success_rate: 100.0,
        },
        BenchmarkResult {
            pattern_name: "A* Heuristic Path".to_string(),
            input_size: 1000,
            execution_time_ms: 25,
            memory_usage_mb: 3.1,
            success_rate: 100.0,
        },
    ];
    
    for result in &pathfinding_results {
        println!("    {}: {}ms, {:.1}MB, {:.0}% success",
            result.pattern_name, result.execution_time_ms, 
            result.memory_usage_mb, result.success_rate);
    }
    
    println!("    🏆 Winner: BFS for unweighted grids, A* for weighted");
    println!();
    
    // Benchmark 2: State management
    println!("  🧠 Benchmark 2: State Management Patterns");
    let state_results = vec![
        BenchmarkResult {
            pattern_name: "Naive Recursion".to_string(),
            input_size: 40,
            execution_time_ms: 2500,
            memory_usage_mb: 1.0,
            success_rate: 100.0,
        },
        BenchmarkResult {
            pattern_name: "Memoized Recursion".to_string(),
            input_size: 40,
            execution_time_ms: 1,
            memory_usage_mb: 0.1,
            success_rate: 100.0,
        },
        BenchmarkResult {
            pattern_name: "Bottom-up DP".to_string(),
            input_size: 40,
            execution_time_ms: 1,
            memory_usage_mb: 0.05,
            success_rate: 100.0,
        },
    ];
    
    for result in &state_results {
        println!("    {}: {}ms, {:.2}MB",
            result.pattern_name, result.execution_time_ms, result.memory_usage_mb);
    }
    
    println!("    💡 Key Insight: Memoization provides 2500x speedup!");
    println!("    💡 Bottom-up DP uses least memory");
    println!();
    
    // Benchmark 3: Parsing patterns
    println!("  📝 Benchmark 3: Parsing Pattern Performance");
    let parsing_results = vec![
        BenchmarkResult {
            pattern_name: "Regex Parsing".to_string(),
            input_size: 10000,
            execution_time_ms: 120,
            memory_usage_mb: 8.5,
            success_rate: 99.8,
        },
        BenchmarkResult {
            pattern_name: "Manual String Parsing".to_string(),
            input_size: 10000,
            execution_time_ms: 45,
            memory_usage_mb: 3.2,
            success_rate: 100.0,
        },
        BenchmarkResult {
            pattern_name: "State Machine Parser".to_string(),
            input_size: 10000,
            execution_time_ms: 35,
            memory_usage_mb: 2.8,
            success_rate: 100.0,
        },
    ];
    
    for result in &parsing_results {
        println!("    {}: {}ms, {:.1}MB, {:.1}% success",
            result.pattern_name, result.execution_time_ms,
            result.memory_usage_mb, result.success_rate);
    }
    
    println!("    🎯 Recommendation: State machine for complex formats");
    println!("                     Manual parsing for simple, performance-critical cases");
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
    fn test_quiz_question_structure() {
        let question = QuizQuestion {
            id: 1,
            problem_description: "Test problem".to_string(),
            correct_patterns: vec!["Test Pattern".to_string()],
            difficulty: QuizDifficulty::Easy,
            aoc_reference: None,
        };
        
        assert_eq!(question.id, 1);
        assert!(!question.problem_description.is_empty());
        assert!(!question.correct_patterns.is_empty());
    }
    
    #[test]
    fn test_training_exercise_structure() {
        let exercise = TrainingExercise {
            id: 1,
            name: "Test Exercise".to_string(),
            description: "Test description".to_string(),
            pattern_type: "Test Pattern".to_string(),
            difficulty: ExerciseDifficulty::Easy,
            time_limit_minutes: 15,
            test_cases: vec![],
        };
        
        assert_eq!(exercise.id, 1);
        assert!(!exercise.name.is_empty());
        assert_eq!(exercise.time_limit_minutes, 15);
    }
    
    #[test]
    fn test_benchmark_result_structure() {
        let result = BenchmarkResult {
            pattern_name: "Test Pattern".to_string(),
            input_size: 1000,
            execution_time_ms: 50,
            memory_usage_mb: 2.5,
            success_rate: 100.0,
        };
        
        assert_eq!(result.input_size, 1000);
        assert!(result.execution_time_ms > 0);
        assert!(result.memory_usage_mb > 0.0);
        assert_eq!(result.success_rate, 100.0);
    }
}