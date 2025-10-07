# Day 27 · String Parsing (Splitting, Regex & Custom Parsers)

> **Learning Context**: Day 27 covers essential string parsing techniques for handling AoC inputs and real-world data processing - from simple splitting to powerful regex patterns and custom parser implementations that transform raw text into structured data.

**Cross-Track Integration:**
- **Mission Focus**: Parsing utilities support all missions' input handling; regex patterns crucial for AoC problems
- **Daily Study**: Week 4, Day 6 - Data processing techniques completing the algorithmic foundations
- **Rust Book**: Chapter 8 (Strings), Chapter 9 (Error Handling with Result), Chapter 18 (Patterns)

**Related Zettelkasten Notes:**
- [[../missions/Mission3/README|Mission3 - Search & Sort]]
- [[Regex Patterns]] - Pattern matching strategies
- [[Error Handling Patterns]] - Result and Option in parsing
- [[zettel-index]] - Main learning hub

---

## 🎯 Core Concepts

### Why String Parsing Matters

**String parsing** is the process of extracting structured data from unstructured text. It's one of the most common tasks in programming, especially in competitive programming.

**Real-World Applications:**
- **AoC Inputs**: Parsing puzzle inputs into usable data structures
- **Configuration Files**: Reading .ini, .toml, .yaml files
- **Log Analysis**: Extracting patterns from log files
- **Data Import**: Converting CSV, TSV, JSON to structs
- **Command Parsing**: Interpreting user commands
- **Natural Language**: Extracting information from text

**Why It's Essential for AoC:**
```
Raw Input:                     Parsed Structure:
"3,4,5"          →             vec![3, 4, 5]
"x=5, y=10"      →             Point { x: 5, y: 10 }
"move 3 from 1 to 2"  →        Move { count: 3, from: 1, to: 2 }
```

---

## ✂️ Basic String Splitting

### The `split()` Family of Methods

Rust's `str` type provides several powerful splitting methods:

```rust
fn main() {
    let text = "apple,banana,cherry";
    
    // 1. split() - Returns iterator over substrings
    let fruits: Vec<&str> = text.split(',').collect();
    println!("{:?}", fruits);
    // ["apple", "banana", "cherry"]
    
    // 2. split_whitespace() - Splits on any whitespace
    let sentence = "hello   world\t\nrust";
    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("{:?}", words);
    // ["hello", "world", "rust"]
    
    // 3. split_once() - Splits into exactly two parts
    let pair = "key=value";
    if let Some((key, value)) = pair.split_once('=') {
        println!("Key: {}, Value: {}", key, value);
    }
    
    // 4. split_terminator() - Doesn't include empty string at end
    let data = "a,b,c,";
    let items: Vec<&str> = data.split_terminator(',').collect();
    println!("{:?}", items);
    // ["a", "b", "c"] (no empty string)
    
    // 5. splitn() - Limit number of splits
    let text = "1:2:3:4:5";
    let parts: Vec<&str> = text.splitn(3, ':').collect();
    println!("{:?}", parts);
    // ["1", "2", "3:4:5"] (splits into at most 3 parts)
}
```

**Key Differences:**

| Method | Separator | Empty Strings | Use Case |
|--------|-----------|---------------|----------|
| `split(pat)` | Any pattern | Includes | General splitting |
| `split_whitespace()` | Any whitespace | Excludes | Word tokenization |
| `split_once(pat)` | First occurrence | N/A | Key-value pairs |
| `split_terminator(pat)` | Pattern | Excludes trailing | CSV with trailing delimiter |
| `splitn(n, pat)` | Pattern | Includes | Limit splits |
| `lines()` | `\n` or `\r\n` | Excludes | Line-by-line processing |

### Parsing Numbers from Strings

```rust
/// Parse comma-separated integers
fn parse_integers(s: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
    s.split(',')
        .map(|num| num.trim().parse::<i32>())
        .collect()
}

fn main() {
    let input = "10, 20, 30, 40";
    
    match parse_integers(input) {
        Ok(numbers) => println!("Parsed: {:?}", numbers),
        Err(e) => println!("Parse error: {}", e),
    }
    
    // Alternative: filter_map to skip errors
    let numbers: Vec<i32> = "1,abc,2,3,xyz"
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();
    
    println!("Valid numbers: {:?}", numbers);
    // [1, 2, 3]
}
```

**Why `collect()` Works with Result:**

Rust's `collect()` has a special implementation for `Result`:
```rust
// If all Ok, returns Ok(Vec<T>)
// If any Err, returns first Err encountered
let result: Result<Vec<i32>, _> = vec!["1", "2", "3"]
    .into_iter()
    .map(|s| s.parse())
    .collect();
```

---

## 🔍 Advanced Splitting Techniques

### Multi-Character Delimiters

```rust
fn main() {
    // Split by multi-character pattern
    let text = "one::two::three::four";
    let parts: Vec<&str> = text.split("::").collect();
    println!("{:?}", parts);
    // ["one", "two", "three", "four"]
    
    // Split by any of several characters
    let text = "one,two;three:four";
    let parts: Vec<&str> = text
        .split(|c| c == ',' || c == ';' || c == ':')
        .collect();
    println!("{:?}", parts);
    // ["one", "two", "three", "four"]
    
    // Split with closure for complex logic
    let text = "aAbBcCdD";
    let parts: Vec<&str> = text
        .split(|c: char| c.is_uppercase())
        .filter(|s| !s.is_empty())
        .collect();
    println!("{:?}", parts);
    // ["a", "b", "c", "d"]
}
```

### Parsing Structured Lines

**Problem**: Parse lines like `"x=5, y=10, z=15"` into a struct.

```rust
#[derive(Debug)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3D {
    /// Parse from format "x=5, y=10, z=15"
    fn from_str(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split(", ").collect();
        if parts.len() != 3 {
            return None;
        }
        
        let x = parts[0].strip_prefix("x=")?.parse().ok()?;
        let y = parts[1].strip_prefix("y=")?.parse().ok()?;
        let z = parts[2].strip_prefix("z=")?.parse().ok()?;
        
        Some(Point3D { x, y, z })
    }
}

fn main() {
    let input = "x=5, y=10, z=15";
    
    match Point3D::from_str(input) {
        Some(point) => println!("Parsed: {:?}", point),
        None => println!("Failed to parse"),
    }
}
```

---

## 🎨 Regular Expressions with `regex` Crate

### Why Use Regex?

**Simple splits** work for simple patterns, but regex handles:
- Complex patterns (e.g., "match any number followed by a letter")
- Capturing groups (extract specific parts)
- Optional elements
- Alternation (match A or B or C)
- Repetition with constraints

### Getting Started with Regex

First, add to `Cargo.toml`:
```toml
[dependencies]
regex = "1"
```

### Basic Regex Patterns

```rust
use regex::Regex;

fn main() {
    // 1. Simple matching
    let re = Regex::new(r"\d+").unwrap(); // Match one or more digits
    let text = "There are 123 apples and 456 oranges";
    
    for mat in re.find_iter(text) {
        println!("Found number: {}", mat.as_str());
    }
    // Found number: 123
    // Found number: 456
    
    // 2. Capturing groups
    let re = Regex::new(r"(\d+) (\w+)").unwrap(); // Number + word
    
    for cap in re.captures_iter(text) {
        let count = &cap[1];
        let item = &cap[2];
        println!("{} {}", count, item);
    }
    // 123 apples
    // 456 oranges
    
    // 3. Named captures (more readable!)
    let re = Regex::new(r"(?P<count>\d+) (?P<item>\w+)").unwrap();
    
    for cap in re.captures_iter(text) {
        println!("{} {}", &cap["count"], &cap["item"]);
    }
}
```

### Common Regex Patterns for AoC

```rust
use regex::Regex;

fn main() {
    // Parse "move 3 from 1 to 2"
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let text = "move 3 from 1 to 2";
    
    if let Some(caps) = re.captures(text) {
        let count: usize = caps[1].parse().unwrap();
        let from: usize = caps[2].parse().unwrap();
        let to: usize = caps[3].parse().unwrap();
        println!("Move {} from {} to {}", count, from, to);
    }
    
    // Parse coordinates "x=5, y=-10"
    let re = Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();
    let text = "x=5, y=-10";
    
    if let Some(caps) = re.captures(text) {
        let x: i32 = caps[1].parse().unwrap();
        let y: i32 = caps[2].parse().unwrap();
        println!("Coordinates: ({}, {})", x, y);
    }
    
    // Extract all numbers (positive and negative)
    let re = Regex::new(r"-?\d+").unwrap();
    let text = "Position: x=10, y=-5, z=20";
    let numbers: Vec<i32> = re
        .find_iter(text)
        .filter_map(|m| m.as_str().parse().ok())
        .collect();
    println!("Numbers: {:?}", numbers);
    // [10, -5, 20]
}
```

### Regex Cheat Sheet

| Pattern | Matches | Example |
|---------|---------|---------|
| `\d` | Any digit | `\d+` matches "123" |
| `\w` | Word character (a-z, A-Z, 0-9, _) | `\w+` matches "hello" |
| `\s` | Whitespace | `\s+` matches "   " |
| `.` | Any character | `a.c` matches "abc", "a9c" |
| `*` | Zero or more | `\d*` matches "", "1", "123" |
| `+` | One or more | `\d+` matches "1", "123" (not "") |
| `?` | Zero or one | `\d?` matches "", "1" (not "12") |
| `{n}` | Exactly n | `\d{3}` matches "123" (not "12") |
| `{n,m}` | Between n and m | `\d{2,4}` matches "12", "123", "1234" |
| `[abc]` | Any of a, b, or c | `[0-9]` same as `\d` |
| `[^abc]` | NOT a, b, or c | `[^0-9]` matches non-digits |
| `^` | Start of line | `^\d` matches digit at start |
| `$` | End of line | `\d$` matches digit at end |
| `\|` | Alternation (or) | `cat\|dog` matches "cat" or "dog" |
| `()` | Capturing group | `(\d+)` captures numbers |
| `(?P<name>)` | Named capture | `(?P<num>\d+)` |

---

## 🏗️ Custom Parser Implementations

### Building a Simple Parser

Sometimes regex isn't enough - you need a **state machine** or **recursive parser**.

```rust
/// Parses expressions like "add(5, 10)" or "mul(3, 4)"
#[derive(Debug, PartialEq)]
enum Operation {
    Add(i32, i32),
    Mul(i32, i32),
}

struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }
    
    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }
    
    fn advance(&mut self) {
        self.pos += 1;
    }
    
    fn skip_whitespace(&mut self) {
        while self.peek().map_or(false, |c| c.is_whitespace()) {
            self.advance();
        }
    }
    
    fn parse_identifier(&mut self) -> Option<String> {
        let start = self.pos;
        
        while self.peek().map_or(false, |c| c.is_alphanumeric()) {
            self.advance();
        }
        
        if self.pos > start {
            Some(self.input[start..self.pos].to_string())
        } else {
            None
        }
    }
    
    fn parse_number(&mut self) -> Option<i32> {
        self.skip_whitespace();
        let start = self.pos;
        
        // Handle negative numbers
        if self.peek() == Some('-') {
            self.advance();
        }
        
        while self.peek().map_or(false, |c| c.is_ascii_digit()) {
            self.advance();
        }
        
        if self.pos > start {
            self.input[start..self.pos].parse().ok()
        } else {
            None
        }
    }
    
    fn expect(&mut self, ch: char) -> bool {
        self.skip_whitespace();
        if self.peek() == Some(ch) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    fn parse_operation(&mut self) -> Option<Operation> {
        self.skip_whitespace();
        let op_name = self.parse_identifier()?;
        
        if !self.expect('(') {
            return None;
        }
        
        let a = self.parse_number()?;
        
        if !self.expect(',') {
            return None;
        }
        
        let b = self.parse_number()?;
        
        if !self.expect(')') {
            return None;
        }
        
        match op_name.as_str() {
            "add" => Some(Operation::Add(a, b)),
            "mul" => Some(Operation::Mul(a, b)),
            _ => None,
        }
    }
}

fn main() {
    let input = "add(5, 10)";
    let mut parser = Parser::new(input);
    
    if let Some(op) = parser.parse_operation() {
        println!("Parsed: {:?}", op);
    }
    
    // Test multiple operations
    let tests = vec![
        "add(5, 10)",
        "mul(3, 4)",
        "add(-5, 20)",
        "mul(  100  ,  200  )", // whitespace tolerant
    ];
    
    for test in tests {
        let mut parser = Parser::new(test);
        match parser.parse_operation() {
            Some(op) => println!("{} → {:?}", test, op),
            None => println!("{} → Parse error", test),
        }
    }
}
```

### State Machine Parser

For more complex grammars:

```rust
#[derive(Debug, PartialEq)]
enum Token {
    Number(i32),
    Operator(char),
    OpenParen,
    CloseParen,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    
    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' => {
                chars.next();
            }
            '0'..='9' => {
                let mut num = 0;
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_digit() {
                        num = num * 10 + (ch as i32 - '0' as i32);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(num));
            }
            '+' | '-' | '*' | '/' => {
                tokens.push(Token::Operator(ch));
                chars.next();
            }
            '(' => {
                tokens.push(Token::OpenParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::CloseParen);
                chars.next();
            }
            _ => {
                chars.next(); // Skip unknown characters
            }
        }
    }
    
    tokens
}

fn main() {
    let input = "3 + (4 * 5)";
    let tokens = tokenize(input);
    
    println!("Input: {}", input);
    println!("Tokens: {:?}", tokens);
}
```

---

## 🚀 Complete Runnable Example

```rust
use std::collections::HashMap;

fn main() {
    println!("=== Day 27: String Parsing Demo ===\n");
    
    // 1. Basic Splitting
    println!("🔷 1. Basic String Splitting");
    println!("=============================");
    
    let csv = "apple,banana,cherry,date";
    let fruits: Vec<&str> = csv.split(',').collect();
    println!("CSV: {}", csv);
    println!("Parsed: {:?}\n", fruits);
    
    // 2. Whitespace Splitting
    println!("🔷 2. Whitespace Splitting");
    println!("==========================");
    
    let text = "  hello   world  \t rust  \n programming  ";
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("Text: {:?}", text);
    println!("Words: {:?}\n", words);
    
    // 3. Key-Value Parsing
    println!("🔷 3. Key-Value Pair Parsing");
    println!("============================");
    
    let config = "debug=true\nport=8080\nhost=localhost";
    let mut settings = HashMap::new();
    
    for line in config.lines() {
        if let Some((key, value)) = line.split_once('=') {
            settings.insert(key, value);
        }
    }
    
    println!("Config:\n{}", config);
    println!("Parsed settings: {:?}\n", settings);
    
    // 4. Parsing Numbers
    println!("🔷 4. Parsing Integer Lists");
    println!("===========================");
    
    let numbers_str = "10, 20, 30, 40, 50";
    let numbers: Result<Vec<i32>, _> = numbers_str
        .split(',')
        .map(|s| s.trim().parse())
        .collect();
    
    match numbers {
        Ok(nums) => println!("Parsed numbers: {:?}", nums),
        Err(e) => println!("Parse error: {}", e),
    }
    
    // With error recovery
    let mixed = "1, abc, 2, xyz, 3";
    let valid_nums: Vec<i32> = mixed
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    println!("Input with errors: {}", mixed);
    println!("Valid numbers only: {:?}\n", valid_nums);
    
    // 5. Multi-Character Delimiters
    println!("🔷 5. Complex Delimiters");
    println!("========================");
    
    let path = "home::user::documents::file.txt";
    let parts: Vec<&str> = path.split("::").collect();
    println!("Path: {}", path);
    println!("Parts: {:?}\n", parts);
    
    // 6. Custom Parser - Coordinates
    println!("🔷 6. Custom Parser (Coordinates)");
    println!("==================================");
    
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    impl Point {
        fn parse(s: &str) -> Option<Self> {
            let s = s.trim_matches(|c| c == '(' || c == ')');
            let parts: Vec<&str> = s.split(',').collect();
            
            if parts.len() == 2 {
                let x = parts[0].trim().parse().ok()?;
                let y = parts[1].trim().parse().ok()?;
                Some(Point { x, y })
            } else {
                None
            }
        }
    }
    
    let coords = vec![
        "(10, 20)",
        "(5,15)",
        "(-10, -20)",
    ];
    
    for coord_str in coords {
        match Point::parse(coord_str) {
            Some(point) => println!("{} → {:?}", coord_str, point),
            None => println!("{} → Parse failed", coord_str),
        }
    }
    println!();
    
    // 7. AoC-Style Input Parsing
    println!("🔷 7. AoC-Style Command Parsing");
    println!("================================");
    
    #[derive(Debug)]
    enum Command {
        Move { direction: String, distance: i32 },
        Turn { angle: i32 },
    }
    
    fn parse_command(line: &str) -> Option<Command> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        match parts.as_slice() {
            ["move", dir, dist] => {
                let distance = dist.parse().ok()?;
                Some(Command::Move {
                    direction: dir.to_string(),
                    distance,
                })
            }
            ["turn", angle] => {
                let angle = angle.parse().ok()?;
                Some(Command::Turn { angle })
            }
            _ => None,
        }
    }
    
    let commands = vec![
        "move north 10",
        "turn 90",
        "move east 5",
        "turn -45",
    ];
    
    for cmd_str in commands {
        match parse_command(cmd_str) {
            Some(cmd) => println!("{} → {:?}", cmd_str, cmd),
            None => println!("{} → Invalid command", cmd_str),
        }
    }
    println!();
    
    // 8. Grid Parsing (Common AoC Pattern)
    println!("🔷 8. Grid Parsing from Text");
    println!("============================");
    
    let grid_input = "\
    ...#...\n\
    .#...#.\n\
    #....#.\n\
    .......";
    
    let grid: Vec<Vec<char>> = grid_input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    
    println!("Input grid:");
    println!("{}", grid_input);
    println!("\nParsed as 2D vector: {}x{}", grid.len(), grid[0].len());
    
    // Count obstacles
    let obstacles = grid.iter()
        .flatten()
        .filter(|&&c| c == '#')
        .count();
    
    println!("Obstacles found: {}\n", obstacles);
    
    // 9. Number Extraction
    println!("🔷 9. Extracting All Numbers");
    println!("============================");
    
    let text = "Sensor at x=2, y=18: closest beacon at x=-2, y=15";
    
    let numbers: Vec<i32> = text
        .split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter(|s| !s.is_empty() && s != "-")
        .filter_map(|s| s.parse().ok())
        .collect();
    
    println!("Text: {}", text);
    println!("Extracted numbers: {:?}\n", numbers);
    
    // 10. Performance Comparison
    println!("🔷 10. Parsing Performance");
    println!("==========================");
    
    use std::time::Instant;
    
    let large_csv = (0..10000)
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(",");
    
    let start = Instant::now();
    let _parsed: Vec<i32> = large_csv
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();
    let duration = start.elapsed();
    
    println!("Parsed 10,000 integers from CSV");
    println!("Time: {:?}", duration);
    println!("Performance: {} ns per number",
        duration.as_nanos() / 10000);
}
```

### 🛠️ How to Run This Code:

1. **Online**: Copy to [Rust Playground](https://play.rust-lang.org/)
2. **Local file**: Save as `day27_demo.rs` and run `rustc day27_demo.rs && ./day27_demo`
3. **In this workspace**: Create a new cargo project and paste into `src/main.rs`

---

## 💡 Key Takeaways

### Parsing Strategy Selection

**Use Simple Splitting when:**
- ✅ Input has consistent delimiters
- ✅ No complex patterns
- ✅ Performance is critical
- ✅ Minimal dependencies desired

**Use Regex when:**
- ✅ Complex patterns (optional parts, alternatives)
- ✅ Need capturing groups
- ✅ Validating format
- ✅ One-time setup, many matches

**Use Custom Parser when:**
- ✅ Complex grammar (nested structures)
- ✅ Need error recovery
- ✅ State-dependent parsing
- ✅ Building AST or tokens

### Performance Characteristics

| Method | Time Complexity | Memory | Flexibility |
|--------|----------------|--------|-------------|
| `split()` | O(n) | Low | Low |
| Regex | O(n) first match, then O(1) per match | Medium | High |
| Custom Parser | O(n) | Low | Very High |

### Common AoC Parsing Patterns

**Pattern 1: Grid from Lines**
```rust
let grid: Vec<Vec<char>> = input
    .lines()
    .map(|line| line.chars().collect())
    .collect();
```

**Pattern 2: Extract All Numbers**
```rust
let numbers: Vec<i32> = input
    .split(|c: char| !c.is_ascii_digit() && c != '-')
    .filter_map(|s| s.parse().ok())
    .collect();
```

**Pattern 3: Parse Commands**
```rust
let commands: Vec<Command> = input
    .lines()
    .filter_map(|line| parse_command(line))
    .collect();
```

**Pattern 4: Key-Value Config**
```rust
let config: HashMap<&str, &str> = input
    .lines()
    .filter_map(|line| line.split_once('='))
    .collect();
```

---

## 🔗 Related Topics

### Tomorrow's Preview: Day 28 - Week 4 Review & Integration
- Comprehensive review of grids, queues, and parsing
- Integrated examples combining all concepts
- Performance optimization strategies
- Real AoC problem solving

### Mission Integration
- **All Missions**: Input parsing for test data
- **Mission 3**: String search and pattern matching
- **Mission 6**: Grid parsing from text files

### AoC Applications
- **Input Parsing**: Converting puzzle inputs to data structures
- **Command Interpretation**: Parsing movement/operation commands
- **Grid Construction**: Building 2D grids from text
- **Data Extraction**: Pulling numbers and patterns from text

---

*Tags: #string-parsing #regex #split #custom-parser #input-handling #mission3 #aoc-patterns #data-processing*
*Links: [[Day26]] ← | [[../missions/Mission3/README|Mission3]] | [[zettel-index]] | [[Day28]] →*
