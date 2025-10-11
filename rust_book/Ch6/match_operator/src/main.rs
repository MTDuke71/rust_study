//! # Chapter 6.2: The match Control Flow Operator
//!
//! Demonstrates pattern matching with match expressions including:
//! - Basic match patterns
//! - Pattern binding
//! - Matching Option<T>
//! - Exhaustive matching
//! - Wildcard patterns
//!
//! Run with: `cargo run`

fn example1_basic_match() {
    println!("🦀 Example 1: Basic Match Expressions");
    println!("======================================");

    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("  Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    let coins = vec![Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter];

    println!("Coin values:");
    for coin in coins {
        let value = value_in_cents(coin);
        println!("  Value: {} cents", value);
    }
    println!();
}

fn example2_pattern_binding() {
    println!("🔧 Example 2: Patterns That Bind to Values");
    println!("===========================================");

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        California,
        Texas,
    }

    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("  State quarter from {:?}!", state);
                25
            }
        }
    }

    let coins = vec![
        Coin::Quarter(UsState::Alaska),
        Coin::Penny,
        Coin::Quarter(UsState::Texas),
        Coin::Dime,
    ];

    println!("Processing coins with state quarters:");
    let mut total = 0;
    for coin in coins {
        total += value_in_cents(coin) as u32;
    }
    println!("Total value: {} cents", total);
    println!();
}

fn example3_matching_option() {
    println!("🎯 Example 3: Matching with Option<T>");
    println!("======================================");

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    fn describe_option(x: Option<i32>) -> String {
        match x {
            None => String::from("nothing"),
            Some(0) => String::from("zero"),
            Some(i) if i < 0 => format!("negative number: {}", i),
            Some(i) if i > 100 => format!("large number: {}", i),
            Some(i) => format!("number: {}", i),
        }
    }

    let numbers = vec![Some(5), None, Some(0), Some(-10), Some(150)];

    println!("Processing Option values:");
    for num in numbers {
        println!("  Input: {:?}", num);
        println!("    +1: {:?}", plus_one(num));
        println!("    Description: {}", describe_option(num));
    }
    println!();
}

fn example4_exhaustive_matching() {
    println!("⚠️  Example 4: Exhaustive Matching");
    println!("==================================");

    // match must cover all possible cases
    fn classify_number(x: i32) -> &'static str {
        match x {
            0 => "zero",
            1 | 2 | 3 => "small positive",
            4..=10 => "medium positive",
            11..=100 => "large positive",
            // Must handle negative numbers
            _ if x < 0 => "negative",
            // Must handle numbers > 100
            _ => "very large positive",
        }
    }

    let test_numbers = vec![-5, 0, 2, 7, 50, 200];

    println!("Classifying numbers:");
    for num in test_numbers {
        println!("  {}: {}", num, classify_number(num));
    }

    // ❌ This won't compile - not exhaustive (missing None case)
    // fn bad_match(x: Option<i32>) -> i32 {
    //     match x {
    //         Some(i) => i,
    //     }
    // }

    // ✅ This compiles - all cases covered
    fn good_match(x: Option<i32>) -> i32 {
        match x {
            Some(i) => i,
            None => 0,
        }
    }

    println!("\nGood match with default:");
    println!("  Some(42): {}", good_match(Some(42)));
    println!("  None: {}", good_match(None));
    println!();
}

fn example5_wildcard_patterns() {
    println!("🌟 Example 5: Wildcard and Catch-All Patterns");
    println!("==============================================");

    fn describe_dice_roll(roll: u8) {
        match roll {
            1 => println!("  Critical failure!"),
            2 | 3 => println!("  Low roll"),
            4 | 5 | 6 => println!("  Medium roll"),
            7 | 8 | 9 => println!("  Good roll"),
            10 | 11 | 12 => println!("  Excellent roll!"),
            _ => println!("  Invalid dice roll"),
        }
    }

    println!("Dice roll results:");
    for roll in 1..=13 {
        print!("Roll {}: ", roll);
        describe_dice_roll(roll);
    }

    // Using _ to ignore values we don't care about
    fn process_config(value: Option<i32>) {
        match value {
            Some(3) => println!("  Got three!"),
            Some(_) => println!("  Got some other value"),
            None => println!("  Got nothing"),
        }
    }

    println!("\nProcessing config values:");
    process_config(Some(3));
    process_config(Some(7));
    process_config(None);
    println!();
}

fn example6_complex_patterns() {
    println!("💡 Example 6: Complex Pattern Matching");
    println!("=======================================");

    #[derive(Debug)]
    enum Message {
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    fn process_message(msg: Message) {
        match msg {
            Message::Move { x: 0, y: 0 } => {
                println!("  No movement needed");
            }
            Message::Move { x, y } if x == y => {
                println!("  Moving diagonally to ({}, {})", x, y);
            }
            Message::Move { x, y } => {
                println!("  Moving to ({}, {})", x, y);
            }
            Message::Write(text) if text.len() > 10 => {
                println!("  Writing long message: {}...", &text[..10]);
            }
            Message::Write(text) => {
                println!("  Writing: {}", text);
            }
            Message::ChangeColor(r, g, b) if r == g && g == b => {
                println!("  Changing to grayscale: RGB({}, {}, {})", r, g, b);
            }
            Message::ChangeColor(255, 0, 0) => {
                println!("  Changing to pure red!");
            }
            Message::ChangeColor(r, g, b) => {
                println!("  Changing to RGB({}, {}, {})", r, g, b);
            }
        }
    }

    let messages = vec![
        Message::Move { x: 0, y: 0 },
        Message::Move { x: 5, y: 5 },
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello, world! This is a long message")),
        Message::Write(String::from("Hi")),
        Message::ChangeColor(128, 128, 128),
        Message::ChangeColor(255, 0, 0),
        Message::ChangeColor(100, 150, 200),
    ];

    println!("Processing complex messages:");
    for msg in messages {
        process_message(msg);
    }
    println!();
}

fn example7_aoc_pattern() {
    println!("🎄 Example 7: AoC Pattern - Instruction Parsing");
    println!("================================================");

    #[derive(Debug)]
    enum Instruction {
        Forward(i32),
        Down(i32),
        Up(i32),
        TurnLeft,
        TurnRight,
    }

    fn parse_instruction(line: &str) -> Option<Instruction> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts.as_slice() {
            ["forward", n] => n.parse().ok().map(Instruction::Forward),
            ["down", n] => n.parse().ok().map(Instruction::Down),
            ["up", n] => n.parse().ok().map(Instruction::Up),
            ["left"] => Some(Instruction::TurnLeft),
            ["right"] => Some(Instruction::TurnRight),
            _ => None,
        }
    }

    fn execute_instruction(inst: &Instruction, pos: &mut (i32, i32), depth: &mut i32) {
        match inst {
            Instruction::Forward(n) => {
                pos.0 += n;
                println!("  Moved forward {} units to x={}", n, pos.0);
            }
            Instruction::Down(n) => {
                *depth += n;
                println!("  Increased depth by {} to {}", n, depth);
            }
            Instruction::Up(n) => {
                *depth -= n;
                println!("  Decreased depth by {} to {}", n, depth);
            }
            Instruction::TurnLeft => println!("  Turned left"),
            Instruction::TurnRight => println!("  Turned right"),
        }
    }

    let input = vec![
        "forward 5",
        "down 3",
        "forward 8",
        "up 1",
        "left",
        "forward 2",
        "invalid instruction",
    ];

    let mut position = (0, 0);
    let mut depth = 0;

    println!("Executing navigation instructions:");
    for line in input {
        match parse_instruction(line) {
            Some(instruction) => {
                println!("\nInstruction: {:?}", instruction);
                execute_instruction(&instruction, &mut position, &mut depth);
            }
            None => println!("\n❌ Failed to parse: '{}'", line),
        }
    }

    println!("\n📍 Final position: {:?}", position);
    println!("🌊 Final depth: {}", depth);
    println!();
}

fn main() {
    println!("📚 Chapter 6.2: The match Control Flow Operator\n");

    example1_basic_match();
    example2_pattern_binding();
    example3_matching_option();
    example4_exhaustive_matching();
    example5_wildcard_patterns();
    example6_complex_patterns();
    example7_aoc_pattern();

    println!("✅ All examples completed!");
    println!("📖 Next: Read Chapter 6.3 or run examples in ../if_let/");
}
