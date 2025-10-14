# Text Parsing Patterns

**Comprehensive guide to parsing and processing text input in Advent of Code problems**

## 🎯 Overview

Text parsing is the foundation of most Advent of Code problems. Efficient and robust parsing strategies can make the difference between a clean solution and a debugging nightmare. This guide covers the essential patterns for handling AoC's diverse input formats.

**Key Philosophy:** Parse early, parse correctly, parse efficiently. Most AoC problems follow predictable input patterns that can be handled with well-established techniques.

---

## 📝 Basic Line-by-Line Parsing

### **Pattern 1: Simple Line Processing**

```rust
use std::fs;

fn parse_simple_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

// Usage: AoC Day 1 - Sum all numbers
fn day1_example() -> i32 {
    let lines = parse_simple_lines("input.txt");
    lines.iter()
        .map(|line| line.parse::<i32>().expect("Invalid number"))
        .sum()
}
```

### **Pattern 2: Filtering and Transformation**

```rust
fn parse_filtered_lines(filename: &str) -> Vec<i32> {
    fs::read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .filter(|line| !line.is_empty())  // Skip empty lines
        .map(|line| line.trim())          // Remove whitespace
        .filter_map(|line| line.parse::<i32>().ok())  // Parse with error handling
        .collect()
}
```

### **Pattern 3: Split and Parse**

```rust
// AoC Day 2 - "forward 5", "down 3", "up 2"
#[derive(Debug)]
enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn parse_directions(filename: &str) -> Vec<Direction> {
    fs::read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 2 {
                return None;
            }
            
            let value = parts[1].parse::<i32>().ok()?;
            match parts[0] {
                "forward" => Some(Direction::Forward(value)),
                "down" => Some(Direction::Down(value)),
                "up" => Some(Direction::Up(value)),
                _ => None,
            }
        })
        .collect()
}
```

---

## 🔢 Number Parsing Patterns

### **Pattern 4: Integer Extraction from Mixed Text**

```rust
use regex::Regex;

fn extract_numbers(text: &str) -> Vec<i32> {
    let re = Regex::new(r"-?\d+").unwrap();
    re.find_iter(text)
        .filter_map(|m| m.as_str().parse::<i32>().ok())
        .collect()
}

// Usage: "x=12, y=-5, z=3" -> [12, -5, 3]
fn parse_coordinates(line: &str) -> Vec<i32> {
    extract_numbers(line)
}
```

### **Pattern 5: Range Parsing**

```rust
#[derive(Debug)]
struct Range {
    start: i32,
    end: i32,
}

fn parse_range(range_str: &str) -> Option<Range> {
    let parts: Vec<&str> = range_str.split('-').collect();
    if parts.len() != 2 {
        return None;
    }
    
    let start = parts[0].parse::<i32>().ok()?;
    let end = parts[1].parse::<i32>().ok()?;
    
    Some(Range { start, end })
}

// Usage: "1-5,7-10" -> [(1,5), (7,10)]
fn parse_range_pairs(line: &str) -> Vec<(Range, Range)> {
    line.split(',')
        .collect::<Vec<&str>>()
        .chunks_exact(2)
        .filter_map(|chunk| {
            let range1 = parse_range(chunk[0])?;
            let range2 = parse_range(chunk[1])?;
            Some((range1, range2))
        })
        .collect()
}
```

### **Pattern 6: Multi-Format Number Parsing**

```rust
fn parse_flexible_numbers(text: &str) -> Vec<i64> {
    // Handle various formats: "123", "-456", "+789"
    text.split_whitespace()
        .filter_map(|s| {
            // Remove common prefixes/suffixes
            let cleaned = s.trim_matches(|c: char| !c.is_ascii_digit() && c != '-' && c != '+');
            cleaned.parse::<i64>().ok()
        })
        .collect()
}
```

---

## 🔍 Regex-Based Parsing

### **Pattern 7: Structured Data Extraction**

```rust
use regex::Regex;

#[derive(Debug)]
struct GameResult {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

fn parse_game_line(line: &str) -> Option<GameResult> {
    // Pattern: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    let re = Regex::new(r"Game (\d+): (.+)").unwrap();
    let caps = re.captures(line)?;
    
    let id = caps[1].parse::<u32>().ok()?;
    let game_data = &caps[2];
    
    // Parse color counts
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    
    let color_re = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    for cap in color_re.captures_iter(game_data) {
        let count = cap[1].parse::<u32>().unwrap();
        match &cap[2] {
            "red" => red = red.max(count),
            "green" => green = green.max(count),
            "blue" => blue = blue.max(count),
            _ => {}
        }
    }
    
    Some(GameResult { id, red, green, blue })
}
```

### **Pattern 8: Complex Pattern Matching**

```rust
use regex::Regex;

#[derive(Debug)]
struct Instruction {
    op: String,
    args: Vec<String>,
}

fn parse_instructions(filename: &str) -> Vec<Instruction> {
    // Pattern: "addx 15", "noop", "cpy x y"
    let instruction_re = Regex::new(r"(\w+)(?:\s+(.+))?").unwrap();
    
    fs::read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .filter_map(|line| {
            let caps = instruction_re.captures(line)?;
            let op = caps[1].to_string();
            let args = caps.get(2)
                .map(|m| m.as_str().split_whitespace().map(String::from).collect())
                .unwrap_or_default();
            
            Some(Instruction { op, args })
        })
        .collect()
}
```

---

## 🗂️ Multi-Format Parsing

### **Pattern 9: Section-Based Input**

```rust
#[derive(Debug)]
struct PuzzleInput {
    stacks: Vec<Vec<char>>,
    instructions: Vec<(usize, usize, usize)>,
}

fn parse_crate_mover_input(filename: &str) -> PuzzleInput {
    let content = fs::read_to_string(filename).expect("Failed to read file");
    let sections: Vec<&str> = content.split("\n\n").collect();
    
    // Parse crate stacks (first section)
    let stack_lines: Vec<&str> = sections[0].lines().collect();
    let num_stacks = (stack_lines[0].len() + 1) / 4;
    let mut stacks = vec![Vec::new(); num_stacks];
    
    // Parse from bottom up (skip the last line with numbers)
    for line in stack_lines.iter().rev().skip(1) {
        for (i, stack) in stacks.iter_mut().enumerate() {
            let char_pos = 1 + i * 4;
            if char_pos < line.len() {
                let ch = line.chars().nth(char_pos).unwrap();
                if ch != ' ' {
                    stack.push(ch);
                }
            }
        }
    }
    
    // Parse instructions (second section)
    let instructions = sections[1]
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 6 {
                let count = parts[1].parse::<usize>().ok()?;
                let from = parts[3].parse::<usize>().ok()?;
                let to = parts[5].parse::<usize>().ok()?;
                Some((count, from - 1, to - 1))  // Convert to 0-based
            } else {
                None
            }
        })
        .collect();
    
    PuzzleInput { stacks, instructions }
}
```

### **Pattern 10: Key-Value Parsing**

```rust
use std::collections::HashMap;

fn parse_key_value_sections(filename: &str) -> Vec<HashMap<String, String>> {
    fs::read_to_string(filename)
        .expect("Failed to read file")
        .split("\n\n")
        .map(|section| {
            section.lines()
                .flat_map(|line| {
                    line.split_whitespace()
                        .filter_map(|pair| {
                            let parts: Vec<&str> = pair.split(':').collect();
                            if parts.len() == 2 {
                                Some((parts[0].to_string(), parts[1].to_string()))
                            } else {
                                None
                            }
                        })
                })
                .collect()
        })
        .collect()
}
```

---

## 🎲 Advanced Parsing Patterns

### **Pattern 11: Recursive Structure Parsing**

```rust
#[derive(Debug)]
enum JsonValue {
    Number(i64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

fn parse_json_value(input: &str) -> Result<JsonValue, String> {
    let input = input.trim();
    
    if input.starts_with('[') {
        parse_json_array(input)
    } else if input.starts_with('{') {
        parse_json_object(input)
    } else if input.starts_with('"') {
        parse_json_string(input)
    } else {
        parse_json_number(input)
    }
}

fn parse_json_array(input: &str) -> Result<JsonValue, String> {
    if !input.starts_with('[') || !input.ends_with(']') {
        return Err("Invalid array format".to_string());
    }
    
    let content = &input[1..input.len()-1];
    if content.trim().is_empty() {
        return Ok(JsonValue::Array(vec![]));
    }
    
    let mut values = Vec::new();
    let mut depth = 0;
    let mut start = 0;
    
    for (i, ch) in content.char_indices() {
        match ch {
            '[' | '{' => depth += 1,
            ']' | '}' => depth -= 1,
            ',' if depth == 0 => {
                values.push(parse_json_value(&content[start..i])?);
                start = i + 1;
            }
            _ => {}
        }
    }
    
    // Parse the last element
    if start < content.len() {
        values.push(parse_json_value(&content[start..])?);
    }
    
    Ok(JsonValue::Array(values))
}

// Similar implementations for parse_json_object, parse_json_string, parse_json_number...
```

### **Pattern 12: State Machine Parsing**

```rust
#[derive(Debug)]
enum ParseState {
    Initial,
    ReadingCrates,
    ReadingInstructions,
}

#[derive(Debug)]
struct CrateMover {
    state: ParseState,
    stacks: Vec<Vec<char>>,
    instructions: Vec<(usize, usize, usize)>,
}

impl CrateMover {
    fn new() -> Self {
        Self {
            state: ParseState::Initial,
            stacks: Vec::new(),
            instructions: Vec::new(),
        }
    }
    
    fn parse_line(&mut self, line: &str) {
        match self.state {
            ParseState::Initial => {
                if line.trim().is_empty() {
                    self.state = ParseState::ReadingInstructions;
                } else if line.contains('[') {
                    self.state = ParseState::ReadingCrates;
                    self.parse_line(line); // Re-process this line
                }
            }
            ParseState::ReadingCrates => {
                if line.trim().is_empty() {
                    self.state = ParseState::ReadingInstructions;
                } else {
                    self.parse_crate_line(line);
                }
            }
            ParseState::ReadingInstructions => {
                self.parse_instruction_line(line);
            }
        }
    }
    
    fn parse_crate_line(&mut self, line: &str) {
        // Implementation for parsing crate lines
    }
    
    fn parse_instruction_line(&mut self, line: &str) {
        // Implementation for parsing instruction lines
    }
}
```

---

## 🚀 Performance Optimization

### **Pattern 13: Streaming Parser**

```rust
use std::io::{BufRead, BufReader};
use std::fs::File;

fn parse_large_file_streaming(filename: &str) -> Result<Vec<i32>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    
    let mut numbers = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        if let Ok(num) = line.trim().parse::<i32>() {
            numbers.push(num);
        }
    }
    
    Ok(numbers)
}
```

### **Pattern 14: Pre-allocated Collections**

```rust
fn parse_with_capacity(filename: &str, expected_size: usize) -> Vec<String> {
    let mut lines = Vec::with_capacity(expected_size);
    
    fs::read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .for_each(|line| lines.push(line.to_string()));
    
    lines
}
```

---

## 🧪 Testing and Validation

### **Pattern 15: Parse with Validation**

```rust
fn parse_with_validation<T, F>(
    filename: &str,
    parser: F,
    validator: impl Fn(&T) -> bool,
) -> Result<Vec<T>, String>
where
    F: Fn(&str) -> Option<T>,
{
    let results: Vec<T> = fs::read_to_string(filename)
        .map_err(|e| format!("Failed to read file: {}", e))?
        .lines()
        .enumerate()
        .filter_map(|(line_num, line)| {
            parser(line).or_else(|| {
                eprintln!("Warning: Failed to parse line {}: {}", line_num + 1, line);
                None
            })
        })
        .collect();
    
    // Validate all parsed results
    for (i, result) in results.iter().enumerate() {
        if !validator(result) {
            return Err(format!("Validation failed for item {}: {:?}", i, result));
        }
    }
    
    Ok(results)
}
```

---

## 📚 AoC-Specific Patterns

### **Pattern 16: Coordinate Parsing**

```rust
#[derive(Debug, Clone, Copy)]
struct Point2D {
    x: i32,
    y: i32,
}

fn parse_coordinates_various_formats(input: &str) -> Vec<Point2D> {
    input.lines()
        .filter_map(|line| {
            // Handle formats: "x=123, y=456", "123,456", "123 456"
            let numbers: Vec<i32> = extract_numbers(line);
            if numbers.len() >= 2 {
                Some(Point2D { x: numbers[0], y: numbers[1] })
            } else {
                None
            }
        })
        .collect()
}
```

### **Pattern 17: Grid Parsing**

```rust
fn parse_char_grid(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn parse_int_grid(filename: &str) -> Vec<Vec<i32>> {
    fs::read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as i32)
                .collect()
        })
        .collect()
}
```

---

## 🔗 Related Concepts

### **Core Parsing Techniques**
- **[[Number Processing]]** - Advanced number parsing and validation
- **[[Grid Input Handling]]** - 2D data ingestion patterns
- **[[Multi-Format Parsing]]** - Handling diverse input formats
- **[[JSON Processing with serde_json]]** - Structured data parsing
- **[[unicode-utf8-rust]]** - Understanding UTF-8 encoding for text parsing

### **Algorithm Integration**
- **[[BFS Patterns]]** - Parsing graph structures for BFS
- **[[DFS Patterns]]** - Tree and graph parsing for DFS
- **[[Binary Search]]** - Parsing sorted data for search algorithms
- **[[AoC Collection Problems]]** - Choosing collections for parsed data

### **Performance and Testing**
- **[[Performance Optimization]]** - Efficient parsing strategies
- **[[Testing Patterns]]** - Validating parser correctness
- **[[Error Handling]]** - Robust parsing with error recovery
- **[[Stream Processing]]** - Memory-efficient large file parsing

### **Advanced Patterns**
- **[[Pattern Matching]]** - Regex and string matching techniques
- **[[String Manipulation]]** - Text transformation and cleaning
- **[[Parsing Techniques]]** - Recursive descent and state machines
- **[[Text Processing]]** - Tokenization and normalization

---

## 🎓 Key Takeaways

1. **Start Simple**: Use basic string methods before reaching for regex
2. **Validate Early**: Check input format assumptions early in parsing
3. **Handle Errors Gracefully**: Use `filter_map` and `Result` types for robust parsing
4. **Pre-allocate When Possible**: Use `Vec::with_capacity()` for known sizes
5. **Parse Once**: Avoid re-parsing the same data multiple times
6. **Test Edge Cases**: Empty lines, malformed input, boundary conditions

---

*Tags: #parsing #text-processing #aoc #input-handling #regex #validation #error-handling #performance*

*Links: [[zettel-index]] | [[AoC Patterns MOC]] | [[Number Processing]] | [[Grid Input Handling]] | [[Pattern Matching]] | [[Performance Optimization]] | [[Error Handling]] | [[Testing Patterns]]*
