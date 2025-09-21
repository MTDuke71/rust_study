//! Parsing Pattern Recognition Demo
//! 
//! This example demonstrates how to identify and solve parsing-heavy AoC problems
//! using the parsing pattern recognition system.

use aoc_pattern_recognition::parsing_patterns::*;
use aoc_pattern_recognition::{AocPattern, PatternComplexity};

fn main() {
    println!("=== Parsing Pattern Recognition Demo ===\n");

    // Demo 1: Coordinate Parsing (AoC 2018 Day 6 style)
    println!("📍 Demo 1: Coordinate Parsing");
    demo_coordinate_parsing();
    println!();

    // Demo 2: Instruction Parsing (AoC 2020 Day 12 style)
    println!("🎮 Demo 2: Instruction Parsing");
    demo_instruction_parsing();
    println!();

    // Demo 3: Number Extraction (AoC 2015 Day 9 style)
    println!("🔢 Demo 3: Number Extraction");
    demo_number_extraction();
    println!();

    // Demo 4: Advanced Parsing Patterns
    println!("⚡ Demo 4: Advanced Parsing Patterns");
    demo_advanced_parsing();
    println!();

    // Demo 5: Pattern Recognition Guide
    println!("🧠 Demo 5: Pattern Recognition Guide");
    demo_pattern_recognition();
    println!();

    println!("All parsing pattern demos completed! ✅");
}

fn demo_coordinate_parsing() {
    let coord_parser = CoordinateParser::new();
    println!("  Pattern: {}", coord_parser.pattern_name());
    println!("  Complexity: {}", coord_parser.complexity());
    
    // Test various coordinate formats from actual AoC problems
    let test_inputs = vec![
        "position=<10,12> velocity=<1,-1>",
        "123, 456",
        "x=15, y=20",
        "Turn on 887,9 through 959,629",
        "fold along x=655",
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
    ];
    
    for input in test_inputs {
        println!("  Input: \"{}\"", input);
        match coord_parser.solve(input.to_string()) {
            Ok(coords) => {
                println!("    ✅ Found {} coordinates: {:?}", coords.len(), coords);
            }
            Err(e) => println!("    ❌ Error: {}", e),
        }
        println!();
    }
    
    // Demonstrate coordinate validation
    println!("  📋 Coordinate Validation:");
    let valid_inputs = vec!["1,2", "10, 20", "x=5, y=10"];
    let invalid_inputs = vec!["not coords", "1,", "x=abc"];
    
    for input in valid_inputs {
        let is_valid = coord_parser.validate_format(input);
        println!("    \"{}\" → {}", input, if is_valid { "✅ Valid" } else { "❌ Invalid" });
    }
    
    for input in invalid_inputs {
        let is_valid = coord_parser.validate_format(input);
        println!("    \"{}\" → {}", input, if is_valid { "✅ Valid" } else { "❌ Invalid" });
    }
}

fn demo_instruction_parsing() {
    let instruction_parser = InstructionParser::new();
    println!("  Pattern: {}", instruction_parser.pattern_name());
    println!("  Complexity: {}", instruction_parser.complexity());
    
    // Test navigation instructions (AoC 2020 Day 12)
    let navigation_input = "F10\nN3\nF7\nR90\nF11";
    println!("  Navigation Instructions:");
    println!("    {}", navigation_input.replace('\n', ", "));
    
    match instruction_parser.solve(navigation_input.to_string()) {
        Ok(instructions) => {
            println!("    ✅ Parsed {} instructions:", instructions.len());
            for instr in &instructions {
                println!("      {} {}", instr.command, instr.value);
            }
            
            // Simulate basic navigation
            let mut x = 0i32;
            let mut y = 0i32;
            let mut direction = 90; // East
            
            for instr in instructions {
                match instr.command.as_str() {
                    "F" => {
                        match direction {
                            0 => y += instr.value,   // North
                            90 => x += instr.value,  // East  
                            180 => y -= instr.value, // South
                            270 => x -= instr.value, // West
                            _ => {}
                        }
                    }
                    "N" => y += instr.value,
                    "S" => y -= instr.value,
                    "E" => x += instr.value,
                    "W" => x -= instr.value,
                    "L" => direction = (direction - instr.value + 360) % 360,
                    "R" => direction = (direction + instr.value) % 360,
                    _ => {}
                }
            }
            
            let manhattan = x.abs() + y.abs();
            println!("    Final position: ({}, {})", x, y);
            println!("    Manhattan distance: {}", manhattan);
        }
        Err(e) => println!("    ❌ Error: {}", e),
    }
    println!();
    
    // Test other instruction types
    let other_inputs = vec![
        ("Assembly", "inc a\ndec b\njnz a 2\njnz 1 -2"),
        ("Recipes", "turn on 0,0 through 999,999\ntoggle 0,0 through 999,0"),
        ("Memory", "b inc 5 if a > 1\na inc 1 if b < 5"),
    ];
    
    for (name, input) in other_inputs {
        println!("  {} Instructions:", name);
        match instruction_parser.solve(input.to_string()) {
            Ok(instructions) => {
                println!("    ✅ Parsed {} instructions", instructions.len());
                for instr in instructions.iter().take(3) {
                    println!("      {} {}", instr.command, instr.value);
                }
                if instructions.len() > 3 {
                    println!("      ... and {} more", instructions.len() - 3);
                }
            }
            Err(e) => println!("    ❌ Error: {}", e),
        }
        println!();
    }
}

fn demo_number_extraction() {
    let number_parser = NumberExtractionParser::new();
    println!("  Pattern: {}", number_parser.pattern_name());
    println!("  Complexity: {}", number_parser.complexity());
    
    // Test various number extraction scenarios
    let test_cases = vec![
        ("Distance calculations", "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141"),
        ("Reindeer stats", "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds."),
        ("Ingredient properties", "Sugar: capacity 3, durability 0, flavor 0, texture -3, calories 2"),
        ("Light grid", "turn on 887,9 through 959,629"),
        ("Sensor data", "Sensor at x=2, y=18: closest beacon is at x=-2, y=15"),
    ];
    
    for (category, input) in test_cases {
        println!("  {} Example:", category);
        println!("    Input: {}", input.lines().next().unwrap());
        if input.lines().count() > 1 {
            println!("           {}", input.lines().nth(1).unwrap_or(""));
        }
        
        match number_parser.solve(input.to_string()) {
            Ok(numbers) => {
                println!("    ✅ Extracted numbers: {:?}", numbers);
                println!("    Count: {}, Sum: {}, Max: {}", 
                    numbers.len(), 
                    numbers.iter().sum::<i32>(),
                    numbers.iter().max().unwrap_or(&0)
                );
            }
            Err(e) => println!("    ❌ Error: {}", e),
        }
        println!();
    }
    
    // Demonstrate negative number handling
    println!("  🔢 Negative Number Handling:");
    let negative_input = "x=-5, y=10, z=-15";
    match number_parser.solve(negative_input.to_string()) {
        Ok(numbers) => {
            println!("    Input: {}", negative_input);
            println!("    ✅ Extracted: {:?}", numbers);
            println!("    Handles negatives: {}", numbers.iter().any(|&n| n < 0));
        }
        Err(e) => println!("    ❌ Error: {}", e),
    }
}

fn demo_advanced_parsing() {
    let recognizer = ParsingPatternRecognizer::new();
    
    println!("  🔍 Automatic Pattern Recognition:");
    
    let sample_inputs = vec![
        "123, 456\n789, 012",
        "turn left 90\nmove forward 10\nturn right 45",
        "London to Dublin = 464\nLondon to Belfast = 518",
        "toggle 461,550 through 564,900",
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
    ];
    
    for input in sample_inputs {
        println!("    Input: {}", input.lines().next().unwrap());
        let pattern_type = recognizer.identify_pattern(input);
        println!("      → Detected: {:?}", pattern_type);
        
        // Show parsing results
        match pattern_type {
            ParseType::Coordinates => {
                let parser = CoordinateParser::new();
                if let Ok(coords) = parser.solve(input.to_string()) {
                    println!("      → Coordinates: {:?}", coords);
                }
            }
            ParseType::Instructions => {
                let parser = InstructionParser::new();
                if let Ok(instrs) = parser.solve(input.to_string()) {
                    println!("      → Instructions: {} commands", instrs.len());
                }
            }
            ParseType::Numbers => {
                let parser = NumberExtractionParser::new();
                if let Ok(nums) = parser.solve(input.to_string()) {
                    println!("      → Numbers: {:?}", nums);
                }
            }
            ParseType::Mixed => {
                println!("      → Complex input requiring multiple parsers");
            }
        }
        println!();
    }
    
    // Pattern complexity analysis
    println!("  📊 Pattern Complexity Analysis:");
    let parsers: Vec<Box<dyn AocPattern<String, _>>> = vec![
        Box::new(CoordinateParser::new()),
        Box::new(InstructionParser::new()),
        Box::new(NumberExtractionParser::new()),
    ];
    
    for parser in parsers {
        println!("    {}: {} - {}", 
            parser.pattern_name(),
            match parser.complexity() {
                PatternComplexity::Linear => "O(n)",
                PatternComplexity::Quadratic => "O(n²)",
                PatternComplexity::Exponential => "O(2ⁿ)",
                PatternComplexity::Logarithmic => "O(log n)",
            },
            "Good for large inputs"
        );
    }
}

fn demo_pattern_recognition() {
    println!("  🧠 How to recognize parsing patterns in AoC problems:");
    println!();
    
    let pattern_signals = vec![
        (
            "Coordinate-based problems",
            vec![
                "\"x=123, y=456\" format",
                "Grid positions or maps", 
                "Distance calculations",
                "Position tracking",
                "Geometric problems",
            ]
        ),
        (
            "Instruction-based problems",
            vec![
                "\"turn left 90\" or \"move forward 10\"",
                "Assembly-like commands",
                "Navigation or movement",
                "State machine transitions",
                "Robot or turtle graphics",
            ]
        ),
        (
            "Number extraction problems",
            vec![
                "Distance/weight tables",
                "Mixed text with embedded numbers",
                "Statistics or measurements",
                "Configuration parameters",
                "Resource calculations",
            ]
        ),
    ];
    
    for (pattern_name, signals) in pattern_signals {
        println!("  🎯 {}:", pattern_name);
        for signal in signals {
            println!("    • {}", signal);
        }
        println!();
    }
    
    println!("  💡 Pro Tips:");
    println!("    • Look for consistent format patterns in input");
    println!("    • Check if numbers have semantic meaning vs just data");
    println!("    • Consider if order of operations matters (instructions)"); 
    println!("    • Watch for coordinate systems (2D grids, 3D space)");
    println!("    • Test parser with edge cases (negative numbers, missing data)");
    println!();
    
    println!("  ⚡ Common AoC Parsing Challenges:");
    println!("    • Multi-line vs single-line inputs");
    println!("    • Mixed delimiters (commas, spaces, equals)");
    println!("    • Optional fields or variable formats");
    println!("    • Numbers vs identifiers (\"a\" vs \"1\")");
    println!("    • Nested structures (groups of instructions)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_demo_examples() {
        // Test coordinate parsing
        let coord_parser = CoordinateParser::new();
        let result = coord_parser.solve("1,2\n3,4".to_string()).unwrap();
        assert_eq!(result.len(), 2);
        
        // Test instruction parsing
        let instr_parser = InstructionParser::new();
        let result = instr_parser.solve("F10\nN3".to_string()).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].command, "F");
        assert_eq!(result[0].value, 10);
        
        // Test number extraction
        let num_parser = NumberExtractionParser::new();
        let result = num_parser.solve("London to Dublin = 464".to_string()).unwrap();
        assert_eq!(result, vec![464]);
    }
    
    #[test]
    fn test_pattern_recognition() {
        let recognizer = ParsingPatternRecognizer::new();
        
        // Test coordinate recognition
        let coord_input = "x=1, y=2";
        assert!(matches!(recognizer.identify_pattern(coord_input), ParseType::Coordinates));
        
        // Test instruction recognition
        let instr_input = "turn left 90";
        assert!(matches!(recognizer.identify_pattern(instr_input), ParseType::Instructions));
        
        // Test number recognition
        let num_input = "London to Dublin = 464";
        assert!(matches!(recognizer.identify_pattern(num_input), ParseType::Numbers));
    }
}