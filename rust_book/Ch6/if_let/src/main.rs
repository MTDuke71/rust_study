//! # Chapter 6.3: Concise Control Flow with if let
//!
//! Demonstrates if let syntax for single-pattern matching:
//! - Basic if let usage
//! - if let with else
//! - Comparison with match
//! - Practical use cases
//!
//! Run with: `cargo run`

fn example1_basic_if_let() {
    println!("🦀 Example 1: Basic if let Syntax");
    println!("==================================");

    let some_value = Some(3);

    // Using match (verbose for one case)
    println!("Using match:");
    match some_value {
        Some(3) => println!("  Found three!"),
        _ => (),
    }

    // Using if let (concise)
    println!("\nUsing if let:");
    if let Some(3) = some_value {
        println!("  Found three!");
    }

    // Another example
    let config_max = Some(3u8);

    println!("\nChecking config_max with match:");
    match config_max {
        Some(max) => println!("  The maximum is configured to be {}", max),
        _ => (),
    }

    println!("\nChecking config_max with if let:");
    if let Some(max) = config_max {
        println!("  The maximum is configured to be {}", max);
    }
    println!();
}

fn example2_if_let_with_else() {
    println!("🔧 Example 2: if let with else");
    println!("===============================");

    #[derive(Debug)]
    enum Coin {
        #[allow(dead_code)]
        Penny,
        #[allow(dead_code)]
        Nickel,
        #[allow(dead_code)]
        Dime,
        Quarter(String),
    }

    let coin = Coin::Quarter(String::from("Alaska"));
    let mut count = 0;

    // Using match
    println!("Using match:");
    match &coin {
        Coin::Quarter(state) => println!("  State quarter from {}!", state),
        _ => count += 1,
    }

    // Using if let with else
    println!("\nUsing if let with else:");
    if let Coin::Quarter(state) = &coin {
        println!("  State quarter from {}!", state);
    } else {
        count += 1;
    }

    println!("Non-quarter coins counted: {}", count);
    println!();
}

fn example3_option_processing() {
    println!("🎯 Example 3: Processing Option Values");
    println!("=======================================");

    fn find_user(id: u32) -> Option<String> {
        match id {
            1 => Some(String::from("Alice")),
            2 => Some(String::from("Bob")),
            _ => None,
        }
    }

    let user_ids = vec![1, 2, 3, 4];

    println!("Looking up users:");
    for id in user_ids {
        // Concise single-case handling with if let
        if let Some(name) = find_user(id) {
            println!("  User {}: {}", id, name);
        } else {
            println!("  User {} not found", id);
        }
    }
    println!();
}

fn example4_nested_options() {
    println!("💡 Example 4: Nested Option Handling");
    println!("=====================================");

    #[derive(Debug)]
    struct User {
        name: String,
        email: Option<String>,
    }

    fn get_user(id: u32) -> Option<User> {
        match id {
            1 => Some(User {
                name: String::from("Alice"),
                email: Some(String::from("alice@example.com")),
            }),
            2 => Some(User {
                name: String::from("Bob"),
                email: None,
            }),
            _ => None,
        }
    }

    println!("Checking user emails:");

    // User 1 - has email
    if let Some(user) = get_user(1) {
        println!("\nUser: {}", user.name);
        if let Some(email) = user.email {
            println!("  Email: {}", email);
        } else {
            println!("  No email on file");
        }
    }

    // User 2 - no email
    if let Some(user) = get_user(2) {
        println!("\nUser: {}", user.name);
        if let Some(email) = user.email {
            println!("  Email: {}", email);
        } else {
            println!("  No email on file");
        }
    }

    // User 3 - doesn't exist
    if let Some(user) = get_user(3) {
        println!("\nUser: {}", user.name);
    } else {
        println!("\nUser 3 not found");
    }
    println!();
}

fn example5_result_handling() {
    println!("⚠️  Example 5: Result Handling with if let");
    println!("==========================================");

    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err(String::from("division by zero"))
        } else {
            Ok(a / b)
        }
    }

    let operations = vec![(10, 2), (15, 3), (20, 0), (100, 5)];

    println!("Division operations:");
    for (a, b) in operations {
        print!("{} / {} = ", a, b);

        if let Ok(result) = divide(a, b) {
            println!("{}", result);
        } else {
            println!("ERROR");
        }
    }

    println!("\nDetailed error handling:");
    if let Err(error) = divide(10, 0) {
        println!("  Division failed: {}", error);
    }
    println!();
}

fn example6_enum_variants() {
    println!("🌟 Example 6: Checking Specific Enum Variants");
    println!("==============================================");

    #[derive(Debug)]
    enum Event {
        KeyPress(char),
        Click { x: i32, y: i32 },
        #[allow(dead_code)]
        Scroll(i32),
        Quit,
    }

    fn handle_event(event: Event) {
        // Only interested in KeyPress events
        if let Event::KeyPress(c) = event {
            println!("  Key pressed: '{}'", c);

            // Handle special keys
            if c == 'q' || c == 'Q' {
                println!("    Quit key detected!");
            }
        }

        // Only interested in Click events
        if let Event::Click { x, y } = event {
            println!("  Clicked at ({}, {})", x, y);
        }
    }

    let events = vec![
        Event::KeyPress('a'),
        Event::Click { x: 100, y: 200 },
        Event::Scroll(5),
        Event::KeyPress('q'),
        Event::Quit,
    ];

    println!("Processing events:");
    for event in events {
        handle_event(event);
    }
    println!();
}

fn example7_match_vs_if_let() {
    println!("📊 Example 7: When to Use match vs if let");
    println!("==========================================");

    #[derive(Debug)]
    enum Status {
        Active,
        Paused,
        Stopped,
        Error(String),
    }

    let statuses = vec![
        Status::Active,
        Status::Paused,
        Status::Stopped,
        Status::Error(String::from("Connection lost")),
    ];

    println!("Scenario 1: Need to handle all cases → use match");
    for status in &statuses {
        match status {
            Status::Active => println!("  ✅ Running"),
            Status::Paused => println!("  ⏸️  Paused"),
            Status::Stopped => println!("  ⏹️  Stopped"),
            Status::Error(msg) => println!("  ❌ Error: {}", msg),
        }
    }

    println!("\nScenario 2: Only care about errors → use if let");
    for status in &statuses {
        if let Status::Error(msg) = status {
            println!("  Alert: {}", msg);
        }
    }

    println!("\nScenario 3: Two specific cases → if let with else");
    for status in &statuses {
        if let Status::Active = status {
            println!("  System is running normally");
        } else {
            println!("  System is not active");
        }
    }
    println!();
}

fn example8_aoc_pattern() {
    println!("🎄 Example 8: AoC Pattern - Config Parsing");
    println!("===========================================");

    fn parse_config(line: &str) -> Option<(String, i32)> {
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() != 2 {
            return None;
        }

        let key = parts[0].trim().to_string();
        let value = parts[1].trim().parse().ok()?;

        Some((key, value))
    }

    let config_lines = vec![
        "max_players=4",
        "timeout=30",
        "invalid line",
        "port=8080",
        "max_connections=100",
        "=50",
    ];

    let mut max_players = 0;
    let mut timeout = 0;

    println!("Parsing configuration:");
    for line in config_lines {
        if let Some((key, value)) = parse_config(line) {
            println!("  {} = {}", key, value);

            // Extract specific values we care about
            if key == "max_players" {
                max_players = value;
            } else if key == "timeout" {
                timeout = value;
            }
        } else {
            println!("  ❌ Failed to parse: '{}'", line);
        }
    }

    println!("\nExtracted configuration:");
    println!("  Max players: {}", max_players);
    println!("  Timeout: {}", timeout);
    println!();
}

fn example9_let_else_happy_path() {
    println!("😊 Example 9: Staying on the Happy Path with let...else");
    println!("=======================================================");

    // This example follows the official Rust Book's approach
    // https://doc.rust-lang.org/stable/book/ch06-03-if-let.html

    #[derive(Debug, Clone)]
    enum UsState {
        Alabama,
        Alaska,
        California,
        Texas,
    }

    impl UsState {
        fn existed_in(&self, year: u16) -> bool {
            match self {
                UsState::Alabama => year >= 1819,
                UsState::Alaska => year >= 1959,
                UsState::California => year >= 1850,
                UsState::Texas => year >= 1845,
            }
        }
    }

    #[derive(Debug, Clone)]
    enum Coin {
        #[allow(dead_code)]
        Penny,
        #[allow(dead_code)]
        Nickel,
        #[allow(dead_code)]
        Dime,
        Quarter(UsState),
    }

    // Traditional approach with nested if let
    fn describe_state_quarter_traditional(coin: Coin) -> Option<String> {
        if let Coin::Quarter(state) = coin {
            if state.existed_in(1900) {
                Some(format!("{:?} is pretty old, for America!", state))
            } else {
                Some(format!("{:?} is relatively new.", state))
            }
        } else {
            None
        }
    }

    // Using if let to produce a value or return early
    fn describe_state_quarter_if_let(coin: Coin) -> Option<String> {
        let state = if let Coin::Quarter(state) = coin {
            state
        } else {
            return None;
        };

        if state.existed_in(1900) {
            Some(format!("{:?} is pretty old, for America!", state))
        } else {
            Some(format!("{:?} is relatively new.", state))
        }
    }

    // Modern approach with let...else (Rust 1.65+)
    fn describe_state_quarter_let_else(coin: Coin) -> Option<String> {
        let Coin::Quarter(state) = coin else {
            return None;
        };

        if state.existed_in(1900) {
            Some(format!("{:?} is pretty old, for America!", state))
        } else {
            Some(format!("{:?} is relatively new.", state))
        }
    }

    let coins = vec![
        Coin::Quarter(UsState::Alabama),    // 1819 - old
        Coin::Quarter(UsState::Alaska),     // 1959 - new
        Coin::Quarter(UsState::California), // 1850 - old
        Coin::Penny,                        // Not a quarter
        Coin::Quarter(UsState::Texas),      // 1845 - old
    ];

    println!("Traditional approach (nested if let):");
    for coin in &coins {
        if let Some(desc) = describe_state_quarter_traditional(coin.clone()) {
            println!("  ✅ {}", desc);
        } else {
            println!("  ❌ Not a state quarter");
        }
    }

    println!("\nif let approach (produce value or return early):");
    for coin in &coins {
        if let Some(desc) = describe_state_quarter_if_let(coin.clone()) {
            println!("  ✅ {}", desc);
        } else {
            println!("  ❌ Not a state quarter");
        }
    }

    println!("\nlet...else approach (staying on happy path):");
    for coin in &coins {
        if let Some(desc) = describe_state_quarter_let_else(coin.clone()) {
            println!("  ✅ {}", desc);
        } else {
            println!("  ❌ Not a state quarter");
        }
    }

    // Additional example with Option handling
    println!("\nOption handling with let...else:");
    let config_values = vec![Some(3u8), Some(10u8), None, Some(255u8)];

    for config in config_values {
        let Some(max) = config else {
            println!("  ❌ No maximum configured");
            continue;
        };

        // We're on the happy path here - no nesting!
        println!("  ✅ Maximum configured to be {}", max);

        if max > 100 {
            println!("    (That's a high maximum!)");
        }
    }

    println!("\n💡 Key Benefits of let...else:");
    println!("  - Reduces nesting and improves readability");
    println!("  - Keeps the 'happy path' at the top level");
    println!("  - Early returns make error handling cleaner");
    println!("  - Works with both Option and Result types");
    println!("  - Available in Rust 1.65+");
    println!();
}

fn main() {
    println!("📚 Chapter 6.3: Concise Control Flow with if let\n");

    example1_basic_if_let();
    example2_if_let_with_else();
    example3_option_processing();
    example4_nested_options();
    example5_result_handling();
    example6_enum_variants();
    example7_match_vs_if_let();
    example8_aoc_pattern();
    example9_let_else_happy_path();

    println!("✅ All examples completed!");
    println!("📖 Next: Review Chapter 6 concepts or continue to Chapter 7");
    println!("\n💡 Key Insight: Use 'if let' when you only care about one pattern,");
    println!("   use 'match' when you need to handle multiple patterns exhaustively.");
    println!("   Use 'let...else' to stay on the happy path with early returns.");
}
