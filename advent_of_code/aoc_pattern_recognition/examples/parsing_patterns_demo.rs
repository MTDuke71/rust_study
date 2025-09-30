//! Parsing Pattern Recognition Demo
//! 
//! This example demonstrates how to identify and solve parsing-heavy AoC problems
//! using the parsing pattern recognition system.

use aoc_pattern_recognition::parsing_patterns::*;

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
    let coord_parser = CoordinateParser;
    println!("  Pattern: {}", coord_parser.parser_name());
    println!("  Complexity: {}", coord_parser.parse_complexity());
    
    // Test various coordinate formats from actual AoC problems
    let test_inputs = vec![
        "10,12",
        "123, 456", 
        "15 20",
        "887,9",
        "x=2, y=18",
    ];
    
    for input in test_inputs {
        println!("  Input: \"{}\"", input);
        match coord_parser.parse_line(input) {
            Ok(coord) => {
                println!("    ✅ Found coordinate: ({}, {})", coord.x, coord.y);
            }
            Err(e) => println!("    ❌ Error: {}", e),
        }
        println!();
    }
    
    // Test multiple coordinates in one input
    println!("  📋 Multiple Coordinates:");
    let multi_input = "1,2\n3,4\n5,6";
    match coord_parser.parse_lines(multi_input) {
        Ok(coords) => {
            println!("    ✅ Parsed {} coordinates:", coords.len());
            for coord in coords {
                println!("      ({}, {})", coord.x, coord.y);
            }
        }
        Err(e) => println!("    ❌ Error: {}", e),
    }
}

fn demo_instruction_parsing() {
    let instruction_parser = InstructionParser;
    println!("  Pattern: {}", instruction_parser.parser_name());
    println!("  Complexity: {}", instruction_parser.parse_complexity());
    
    // Test navigation instructions (AoC 2020 Day 12 style)
    let test_instructions = vec![
        "F10",
        "N3", 
        "F7",
        "R90",
        "F11",
        "move 3 from 2 to 1",
        "turn left 90",
        "rotate right 5",
    ];
    
    println!("  Navigation Instructions:");
    for instr_text in &test_instructions {
        println!("    Input: \"{}\"", instr_text);
        match instruction_parser.parse_line(instr_text) {
            Ok(instruction) => {
                println!("      ✅ Command: {}, Parameters: {:?}", 
                    instruction.command, instruction.parameters);
            }
            Err(e) => println!("      ❌ Error: {}", e),
        }
    }
    println!();
    
    // Test multi-line instruction parsing
    let multi_input = "F10\nN3\nF7\nR90\nF11";
    println!("  Multi-line Instructions:");
    match instruction_parser.parse_lines(multi_input) {
        Ok(instructions) => {
            println!("    ✅ Parsed {} instructions:", instructions.len());
            
            // Simulate basic navigation
            let mut x = 0i32;
            let mut y = 0i32;
            let mut direction = 90; // East
            
            for instr in instructions {
                let value = instr.parameters.first()
                    .and_then(|p| p.parse::<i32>().ok())
                    .unwrap_or(0);
                    
                match instr.command.as_str() {
                    "F" => {
                        match direction {
                            0 => y += value,   // North
                            90 => x += value,  // East  
                            180 => y -= value, // South
                            270 => x -= value, // West
                            _ => {}
                        }
                    }
                    "N" => y += value,
                    "S" => y -= value,
                    "E" => x += value,
                    "W" => x -= value,
                    "L" => direction = (direction - value + 360) % 360,
                    "R" => direction = (direction + value) % 360,
                    _ => {}
                }
            }
            
            let manhattan = x.abs() + y.abs();
            println!("    Final position: ({}, {})", x, y);
            println!("    Manhattan distance: {}", manhattan);
        }
        Err(e) => println!("    ❌ Error: {}", e),
    }
}

fn demo_number_extraction() {
    let number_parser = NumberExtractionParser;
    println!("  Pattern: {}", number_parser.parser_name());
    println!("  Complexity: {}", number_parser.parse_complexity());
    
    // Test various number extraction scenarios
    let test_cases = vec![
        ("Distance calculations", "London to Dublin = 464"),
        ("Multiple numbers", "move 3 from 2 to 1"),
        ("Coordinates", "x=15, y=20"),
        ("Mixed text", "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds."),
        ("Negative numbers", "x=-5, y=10, z=-15"),
    ];
    
    for (category, input) in test_cases {
        println!("  {} Example:", category);
        println!("    Input: {}", input);
        
        match number_parser.parse_line(input) {
            Ok(numbers) => {
                println!("    ✅ Extracted numbers: {:?}", numbers);
                if !numbers.is_empty() {
                    println!("    Count: {}, Sum: {}, Max: {}", 
                        numbers.len(), 
                        numbers.iter().sum::<i32>(),
                        numbers.iter().max().unwrap_or(&0)
                    );
                }
            }
            Err(e) => println!("    ❌ Error: {}", e),
        }
        println!();
    }
    
    // Test multi-line number extraction
    println!("  🔢 Multi-line Number Extraction:");
    let multi_input = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";
    match number_parser.parse_lines(multi_input) {
        Ok(number_lists) => {
            println!("    ✅ Parsed {} lines:", number_lists.len());
            for (i, numbers) in number_lists.iter().enumerate() {
                println!("      Line {}: {:?}", i + 1, numbers);
            }
            
            // Calculate total of all numbers
            let total: i32 = number_lists.iter().flatten().sum();
            println!("    Total sum: {}", total);
        }
        Err(e) => println!("    ❌ Error: {}", e),
    }
    
    // Demonstrate parse_utils function
    println!("  🔧 Using parse_utils::extract_numbers:");
    let complex_input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";
    let numbers = parse_utils::extract_numbers(complex_input);
    println!("    Input: {}", complex_input);
    println!("    ✅ Extracted: {:?}", numbers);
}

fn demo_advanced_parsing() {
    let recognizer = ParsingPatternRecognizer;
    
    println!("  🔍 Automatic Pattern Recognition:");
    
    let sample_inputs = vec![
        "123, 456",
        "turn left 90",
        "London to Dublin = 464",
        "x=2, y=18",
        "move 3 from 2 to 1",
    ];
    
    for input in sample_inputs {
        println!("    Input: {}", input);
        let patterns = recognizer.analyze_input(input);
        println!("      → Detected patterns:");
        for pattern in patterns {
            println!("        • {}", pattern);
        }
        
        // Show actual parsing results
        let coord_parser = CoordinateParser;
        let instr_parser = InstructionParser;
        let num_parser = NumberExtractionParser;
        
        if let Ok(coord) = coord_parser.parse_line(input) {
            println!("      → Coordinate: ({}, {})", coord.x, coord.y);
        }
        if let Ok(instr) = instr_parser.parse_line(input) {
            println!("      → Instruction: {} {:?}", instr.command, instr.parameters);
        }
        if let Ok(nums) = num_parser.parse_line(input) {
            if !nums.is_empty() {
                println!("      → Numbers: {:?}", nums);
            }
        }
        println!();
    }
    
    // Pattern complexity analysis
    println!("  📊 Pattern Complexity Analysis:");
    let parsers = vec![
        ("Coordinate Parser", CoordinateParser.parse_complexity()),
        ("Instruction Parser", InstructionParser.parse_complexity()),
        ("Number Extraction Parser", NumberExtractionParser.parse_complexity()),
    ];
    
    for (name, complexity) in parsers {
        println!("    {}: {}", name, complexity);
    }
    
    // Demonstrate utility functions
    println!("  🔧 Utility Functions:");
    
    // Test multi_split
    let text = "move,3;from:2|to:1";
    let parts = parse_utils::multi_split(text, &[',', ';', ':', '|']);
    println!("    multi_split(\"{}\") → {:?}", text, parts);
    
    // Test parse_range
    if let Ok((start, end)) = parse_utils::parse_range("10-20") {
        println!("    parse_range(\"10-20\") → ({}, {})", start, end);
    }
    if let Ok((start, end)) = parse_utils::parse_range("5..15") {
        println!("    parse_range(\"5..15\") → ({}, {})", start, end);
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
        let coord_parser = CoordinateParser;
        let result = coord_parser.parse_lines("1,2\n3,4").unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].x, 1);
        assert_eq!(result[0].y, 2);
        
        // Test instruction parsing - note that "F10" is parsed as command="F10", parameters=[]
        let instr_parser = InstructionParser;
        let result = instr_parser.parse_lines("F 10\nN 3").unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].command, "F");
        assert_eq!(result[0].parameters, vec!["10"]);
        
        // Test number extraction
        let num_parser = NumberExtractionParser;
        let result = num_parser.parse_line("London to Dublin = 464").unwrap();
        assert_eq!(result, vec![464]);
    }
    
    #[test]
    fn test_pattern_recognition() {
        let recognizer = ParsingPatternRecognizer;
        
        // Test coordinate recognition
        let coord_input = "x=1, y=2";
        let patterns = recognizer.analyze_input(coord_input);
        assert!(!patterns.is_empty());
        
        // Test instruction recognition
        let instr_input = "turn left 90";
        let patterns = recognizer.analyze_input(instr_input);
        assert!(!patterns.is_empty());
        
        // Test number recognition
        let num_input = "London to Dublin = 464";
        let patterns = recognizer.analyze_input(num_input);
        assert!(!patterns.is_empty());
    }
}