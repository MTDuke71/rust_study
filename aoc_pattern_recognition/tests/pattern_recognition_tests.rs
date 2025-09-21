//! Comprehensive tests for pattern recognition skills
//! 
//! These tests verify both correctness and pattern identification abilities

use aoc_pattern_recognition::*;
use aoc_pattern_recognition::grid_patterns::*;
use aoc_pattern_recognition::parsing_patterns::*;
use aoc_pattern_recognition::state_patterns::*;
use aoc_pattern_recognition::pattern_trainer::*;

#[cfg(test)]
mod pattern_recognition_tests {
    use super::*;

    #[test] // REQ-P1
    fn test_grid_pattern_recognition() {
        // Test basic grid creation and navigation
        let input = "..#.\n.#..\n..#.";
        let grid = Grid::from_string(input);
        
        assert_eq!(grid.width, 4);
        assert_eq!(grid.height, 3);
        assert_eq!(grid.get(&Coord::new(2, 0)), Some(&'#'));
        
        // Test coordinate neighbors
        let coord = Coord::new(1, 1);
        let neighbors = coord.neighbors_4();
        assert_eq!(neighbors.len(), 4);
        assert!(neighbors.contains(&Coord::new(2, 1)));
        assert!(neighbors.contains(&Coord::new(0, 1)));
        
        // Test Manhattan distance
        let dist = coord.manhattan_distance(&Coord::new(3, 2));
        assert_eq!(dist, 3);
    }

    #[test] // REQ-P1
    fn test_shortest_path_pattern() {
        let input = "....\n.##.\n....\n";
        let grid = Grid::from_string(input);
        let pattern = ShortestPathPattern;
        
        let start = Coord::new(0, 0);
        let end = Coord::new(3, 2);
        let walkable = |ch: char| ch == '.';
        
        let result = pattern.solve((grid, start, end, walkable)).unwrap();
        assert!(result.is_some());
        
        let path = result.unwrap();
        assert_eq!(path.first(), Some(&start));
        assert_eq!(path.last(), Some(&end));
    }

    #[test] // REQ-P1  
    fn test_flood_fill_pattern() {
        let input = "AAA\nABA\nAAA";
        let grid = Grid::from_string(input);
        let pattern = FloodFillPattern;
        
        let result = pattern.solve((grid, Coord::new(0, 0), 'A')).unwrap();
        
        // Should fill all 'A' cells connected to (0,0)
        assert!(result.len() > 1);
        assert!(result.contains(&Coord::new(0, 0)));
        assert!(result.contains(&Coord::new(1, 0)));
        assert!(!result.contains(&Coord::new(1, 1))); // 'B' cell shouldn't be filled
    }

    #[test] // REQ-P2
    fn test_coordinate_parsing_pattern() {
        let parser = CoordinateParser;
        
        // Test various coordinate formats
        let coord1 = parser.parse_line("123,456").unwrap();
        assert_eq!(coord1.x, 123);
        assert_eq!(coord1.y, 456);
        
        let coord2 = parser.parse_line("(10, 20)").unwrap();
        assert_eq!(coord2.x, 10);
        assert_eq!(coord2.y, 20);
        
        let coord3 = parser.parse_line("5 -> 7").unwrap();
        assert_eq!(coord3.x, 5);
        assert_eq!(coord3.y, 7);
    }

    #[test] // REQ-P2
    fn test_number_extraction_pattern() {
        let parser = NumberExtractionParser;
        
        let result = parser.parse_line("move 3 from 2 to 1").unwrap();
        assert_eq!(result, vec![3, 2, 1]);
        
        let result2 = parser.parse_line("position: x=15, y=-23, z=42").unwrap();
        assert_eq!(result2, vec![15, -23, 42]);
    }

    #[test] // REQ-P2
    fn test_instruction_parsing_pattern() {
        let parser = InstructionParser;
        
        let inst = parser.parse_line("move 5 from stack2 to stack1").unwrap();
        assert_eq!(inst.command, "move");
        assert_eq!(inst.parameters, vec!["5", "from", "stack2", "to", "stack1"]);
        
        let inst2 = parser.parse_line("rotate left 3").unwrap();
        assert_eq!(inst2.command, "rotate");
        assert_eq!(inst2.parameters, vec!["left", "3"]);
    }

    #[test] // REQ-P2
    fn test_key_value_parsing_pattern() {
        let parser = KeyValueParser::new(":");
        
        let (key, value) = parser.parse_line("name: Alice").unwrap();
        assert_eq!(key, "name");
        assert_eq!(value, "Alice");
        
        // Test multi-line parsing
        let input = "name: Alice\nage: 30\ncity: Boston";
        let map = parser.parse_to_map(input).unwrap();
        assert_eq!(map.get("name"), Some(&"Alice".to_string()));
        assert_eq!(map.get("age"), Some(&"30".to_string()));
        assert_eq!(map.get("city"), Some(&"Boston".to_string()));
    }

    #[test] // REQ-P2
    fn test_parsing_pattern_recognition() {
        let recognizer = ParsingPatternRecognizer;
        
        // Test coordinate input recognition
        let coord_input = "1,2\n3,4\n5,6";
        let patterns = recognizer.analyze_input(coord_input);
        assert!(patterns.iter().any(|p| p.contains("Coordinate")));
        
        // Test instruction input recognition
        let instruction_input = "move 3 from 2 to 1\nrotate left 5";
        let patterns = recognizer.analyze_input(instruction_input);
        assert!(patterns.iter().any(|p| p.contains("Instruction")));
        
        // Test number extraction recognition
        let number_input = "position 10 20 30\nvelocity 5 -3 8";
        let patterns = recognizer.analyze_input(number_input);
        assert!(patterns.iter().any(|p| p.contains("Number")));
    }

    #[test] // REQ-P3
    fn test_memoization_cache() {
        let mut cache = MemoizationCache::new();
        
        // Test fibonacci with memoization
        let result1 = cached_fibonacci(&mut cache, 10);
        let result2 = cached_fibonacci(&mut cache, 10); // Should hit cache
        
        assert_eq!(result1, result2);
        assert_eq!(result1, 55); // 10th Fibonacci number
        
        let (hits, misses, hit_rate) = cache.stats();
        assert!(hits > 0); // Should have cache hits
        assert!(hit_rate > 0.0);
    }

    #[test] // REQ-P3
    fn test_state_tracker() {
        let mut tracker = StateTracker::new(0);
        
        // Simulate simple state transitions
        tracker.advance(|&x| x + 1); // 0 -> 1
        tracker.advance(|&x| x + 1); // 1 -> 2  
        tracker.advance(|&x| x + 1); // 2 -> 3
        
        assert_eq!(*tracker.current(), 3);
        assert_eq!(tracker.generation(), 3);
        
        // Test cycle detection with repeating pattern
        tracker.advance(|&_x| 1); // 3 -> 1 (cycle back)
        tracker.advance(|&x| x + 1); // 1 -> 2
        tracker.advance(|&x| x + 1); // 2 -> 3
        
        if let Some((cycle_start, cycle_length)) = tracker.find_cycle() {
            assert!(cycle_length > 0);
            assert!(cycle_start < tracker.generation());
        }
    }

    #[test] // REQ-P3
    fn test_set_operations() {
        use std::collections::HashSet;
        
        let set1: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
        let set2: HashSet<i32> = [3, 4, 5, 6].iter().cloned().collect();
        let set3: HashSet<i32> = [4, 5, 6, 7].iter().cloned().collect();
        
        let sets = vec![set1, set2, set3];
        
        let intersection = SetOperations::intersection(&sets);
        assert!(intersection.contains(&4)); // Only 4 is in all sets
        assert_eq!(intersection.len(), 1);
        
        let union = SetOperations::union(&sets);
        assert_eq!(union.len(), 7); // 1,2,3,4,5,6,7
        
        let unique_count = SetOperations::count_unique(&sets);
        assert_eq!(unique_count, 7);
    }

    #[test] // REQ-P3
    fn test_dynamic_programming() {
        // Test knapsack problem
        let weights = vec![10, 20, 30];
        let values = vec![60, 100, 120];
        let capacity = 50;
        
        let max_value = DynamicProgramming::knapsack(&weights, &values, capacity);
        assert_eq!(max_value, 220); // items 2 and 3
        
        // Test longest common subsequence
        let lcs_length = DynamicProgramming::longest_common_subsequence("ABCDGH", "AEDFHR");
        assert_eq!(lcs_length, 3); // "ADH"
        
        // Test edit distance
        let edit_dist = DynamicProgramming::edit_distance("kitten", "sitting");
        assert_eq!(edit_dist, 3);
    }

    #[test] // REQ-P3, REQ-P4
    fn test_sliding_window() {
        let arr = vec![1, 4, 2, 9, 5, 10, 23, 8, 14, 6];
        let window_size = 4;
        
        let max_sum = SlidingWindow::max_sum_window(&arr, window_size);
        assert!(max_sum.is_some());
        assert!(max_sum.unwrap() > 0);
        
        // Test anagram finding
        let anagrams = SlidingWindow::find_anagrams("abcab", "ab");
        assert!(anagrams.contains(&0)); // "ab" at position 0
        assert!(anagrams.contains(&3)); // "ab" at position 3
    }

    #[test] // REQ-P3, REQ-P4
    fn test_cycle_detection() {
        // Test simple cycle: 0 -> 1 -> 2 -> 1 (cycle)
        let next_fn = |&x: &i32| match x {
            0 => 1,
            1 => 2,
            2 => 1, // cycle back to 1
            _ => x,
        };
        
        let result = CycleDetection::floyd_cycle_detection(0, next_fn);
        assert!(result.is_some());
        
        let (mu, lambda) = result.unwrap();
        assert_eq!(mu, 1); // cycle starts at position 1
        assert_eq!(lambda, 2); // cycle length is 2 (1 -> 2 -> 1)
    }

    #[test] // REQ-P5
    fn test_pattern_trainer_basic() {
        let mut trainer = PatternTrainer::new();
        
        // Get first exercise
        let exercise = trainer.next_exercise();
        assert!(exercise.is_some());
        
        let exercise = exercise.unwrap();
        assert!(!exercise.title.is_empty());
        assert!(!exercise.description.is_empty());
        assert!(!exercise.expected_patterns.is_empty());
        
        // Submit correct answer
        let identified = exercise.expected_patterns.clone();
        let result = trainer.submit_answer(identified, 10.0);
        assert!(result.correct);
        
        // Check stats
        let stats = trainer.get_stats();
        assert_eq!(stats.total_attempts, 1);
        assert_eq!(stats.correct_identifications, 1);
        assert_eq!(stats.accuracy(), 1.0);
    }

    #[test] // REQ-P5
    fn test_pattern_quiz() {
        let mut quiz = PatternQuiz::new();
        
        // Get first question
        let question = quiz.current_question();
        assert!(question.is_some());
        
        let question = question.unwrap();
        assert!(!question.question.is_empty());
        assert!(!question.options.is_empty());
        assert!(question.correct_answer < question.options.len());
        
        // Submit correct answer
        let correct = quiz.submit_answer(question.correct_answer);
        assert!(correct);
        
        let (score, total) = quiz.final_score();
        assert_eq!(score, 1);
        assert!(total > 0);
    }

    #[test] // REQ-P5
    fn test_state_pattern_recognition() {
        let recognizer = StatePatternRecognizer;
        
        // Test recursive problem recognition
        let recursive_desc = "Calculate the nth Fibonacci number";
        let patterns = recognizer.analyze_problem(recursive_desc);
        assert!(patterns.iter().any(|p| p.contains("Memoization")));
        
        // Test cycle detection recognition
        let cycle_desc = "Simulate cellular automaton with repeating patterns";
        let patterns = recognizer.analyze_problem(cycle_desc);
        assert!(patterns.iter().any(|p| p.contains("Cycle")));
        
        // Test optimization recognition
        let optimization_desc = "Find the maximum value that fits in the knapsack";
        let patterns = recognizer.analyze_problem(optimization_desc);
        assert!(patterns.iter().any(|p| p.contains("Dynamic programming")));
    }

    #[test] // REQ-P4
    fn test_parse_utils() {
        // Test number extraction
        let numbers = parse_utils::extract_numbers("position x=15, y=-23, z=42");
        assert_eq!(numbers, vec![15, -23, 42]);
        
        // Test multi-split
        let parts = parse_utils::multi_split("a,b;c:d", &[',', ';', ':']);
        assert_eq!(parts, vec!["a", "b", "c", "d"]);
        
        // Test range parsing
        let range1 = parse_utils::parse_range("1-5").unwrap();
        assert_eq!(range1, (1, 5));
        
        let range2 = parse_utils::parse_range("10..20").unwrap();
        assert_eq!(range2, (10, 20));
    }

    #[test] // REQ-P4
    fn test_performance_complexity() {
        let pattern = ShortestPathPattern;
        assert_eq!(pattern.complexity(), PatternComplexity::Linear);
        
        let pattern = FloodFillPattern;
        assert_eq!(pattern.complexity(), PatternComplexity::Linear);
        
        // Test complexity display
        let complexity = PatternComplexity::Polynomial;
        let display = format!("{}", complexity);
        assert!(display.contains("O(n²)"));
    }

    #[test] // REQ-P1, REQ-P2, REQ-P3 Integration
    fn test_aoc_style_integration() {
        // Simulate a complete AoC-style problem using multiple patterns
        
        // Step 1: Parse input (coordinate pattern)
        let input = "1,1\n2,2\n3,3\n1,3\n2,1\n3,1";
        let coord_parser = CoordinateParser;
        let coordinates: Result<Vec<_>, _> = input
            .lines()
            .map(|line| coord_parser.parse_line(line))
            .collect();
        let coordinates = coordinates.unwrap();
        
        assert_eq!(coordinates.len(), 6);
        
        // Step 2: Build grid from coordinates (grid pattern)
        let mut grid = Grid::new(4, 4);
        for coord in &coordinates {
            grid.set(Coord::new(coord.x, coord.y), '#');
        }
        
        // Step 3: Use state tracking to analyze regions (state pattern)
        let flood_pattern = FloodFillPattern;
        let filled_region = flood_pattern.solve((grid.clone(), Coord::new(1, 1), '#')).unwrap();
        
        assert!(!filled_region.is_empty());
        
        // Step 4: Use memoization for expensive calculation (state pattern)
        let mut cache = MemoizationCache::new();
        let result = cached_fibonacci(&mut cache, 20);
        assert_eq!(result, 6765);
        
        // This integration test shows how patterns work together in AoC problems
    }
}