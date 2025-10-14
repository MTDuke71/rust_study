// Day 27: String Parsing (Splitting, Regex & Custom Parsers)
// Complete runnable example demonstrating string parsing techniques

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
        .filter(|s| !s.is_empty() && *s != "-")
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
