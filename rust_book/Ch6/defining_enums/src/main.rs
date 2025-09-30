//! # Chapter 6.1: Defining Enums
//!
//! Demonstrates how to define and use enums in Rust, including:
//! - Basic enum definitions
//! - Enums with associated data
//! - Enum methods
//! - The Option<T> enum
//!
//! Run with: `cargo run`

fn example1_basic_enums() {
    println!("🦀 Example 1: Basic Enum Definitions");
    println!("====================================");

    // Simple enum with variants
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("IP Address kinds: {:?} and {:?}", four, six);

    // Enum in a struct
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    println!("Home address: {} ({:?})", home.address, home.kind);
    println!();
}

fn example2_enums_with_data() {
    println!("🔧 Example 2: Enums with Associated Data");
    println!("=========================================");

    // Enum variants can hold data directly
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home IP: {:?}", home);
    println!("Loopback IP: {:?}", loopback);

    // Different variants can have different types
    #[derive(Debug)]
    enum Message {
        Quit,                       // No data
        Move { x: i32, y: i32 },   // Named fields (like struct)
        Write(String),              // Single value
        ChangeColor(i32, i32, i32), // Multiple values (tuple)
    }

    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello"));
    let msg4 = Message::ChangeColor(255, 0, 128);

    println!("\nMessages:");
    println!("  {:?}", msg1);
    println!("  {:?}", msg2);
    println!("  {:?}", msg3);
    println!("  {:?}", msg4);
    println!();
}

fn example3_enum_methods() {
    println!("🎯 Example 3: Enum Methods");
    println!("==========================");

    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // Enums can have methods just like structs!
    impl Message {
        fn call(&self) {
            match self {
                Message::Quit => println!("  Quitting application..."),
                Message::Move { x, y } => println!("  Moving to position ({}, {})", x, y),
                Message::Write(text) => println!("  Writing: {}", text),
                Message::ChangeColor(r, g, b) => {
                    println!("  Changing color to RGB({}, {}, {})", r, g, b)
                }
            }
        }

        fn description(&self) -> &str {
            match self {
                Message::Quit => "quit message",
                Message::Move { .. } => "move message",
                Message::Write(_) => "write message",
                Message::ChangeColor(_, _, _) => "color change message",
            }
        }
    }

    let messages = vec![
        Message::Write(String::from("Hello, Rust!")),
        Message::Move { x: 100, y: 200 },
        Message::ChangeColor(0, 128, 255),
        Message::Quit,
    ];

    println!("Processing messages:");
    for msg in messages {
        println!("\n{}: ", msg.description());
        msg.call();
    }
    println!();
}

fn example4_option_enum() {
    println!("💎 Example 4: The Option<T> Enum");
    println!("=================================");

    // Option<T> is defined in the standard library as:
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);

    // Option forces you to handle the None case
    fn print_value(opt: Option<i32>) {
        match opt {
            Some(value) => println!("  Value: {}", value),
            None => println!("  No value present"),
        }
    }

    println!("\nHandling Option values:");
    print_value(Some(42));
    print_value(None);

    // Common Option methods
    let x: Option<i32> = Some(2);
    println!("\nOption methods:");
    println!("  x.is_some() = {}", x.is_some());
    println!("  x.is_none() = {}", x.is_none());
    println!("  x.unwrap_or(0) = {}", x.unwrap_or(0));

    let y: Option<i32> = None;
    println!("  y.unwrap_or(0) = {}", y.unwrap_or(0));
    println!();
}

fn example5_real_world_usage() {
    println!("🌍 Example 5: Real-World Enum Usage");
    println!("====================================");

    // Modeling HTTP response status
    #[derive(Debug)]
    enum HttpStatus {
        Ok,
        Created,
        NotFound,
        ServerError(String),
    }

    impl HttpStatus {
        fn is_success(&self) -> bool {
            matches!(self, HttpStatus::Ok | HttpStatus::Created)
        }

        fn status_code(&self) -> u16 {
            match self {
                HttpStatus::Ok => 200,
                HttpStatus::Created => 201,
                HttpStatus::NotFound => 404,
                HttpStatus::ServerError(_) => 500,
            }
        }
    }

    let responses = vec![
        HttpStatus::Ok,
        HttpStatus::Created,
        HttpStatus::NotFound,
        HttpStatus::ServerError(String::from("Database connection failed")),
    ];

    println!("Processing HTTP responses:");
    for response in responses {
        println!("\n  Status: {:?}", response);
        println!("  Code: {}", response.status_code());
        println!("  Success: {}", response.is_success());

        if let HttpStatus::ServerError(msg) = response {
            println!("  Error message: {}", msg);
        }
    }
    println!();
}

fn example6_aoc_pattern() {
    println!("🎄 Example 6: AoC Pattern - Direction Enum");
    println!("===========================================");

    #[derive(Debug, Clone, Copy)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    impl Direction {
        fn to_vector(&self) -> (i32, i32) {
            match self {
                Direction::North => (0, -1),
                Direction::South => (0, 1),
                Direction::East => (1, 0),
                Direction::West => (-1, 0),
            }
        }

        fn turn_right(&self) -> Direction {
            match self {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            }
        }

        fn opposite(&self) -> Direction {
            match self {
                Direction::North => Direction::South,
                Direction::South => Direction::North,
                Direction::East => Direction::West,
                Direction::West => Direction::East,
            }
        }
    }

    let mut position = (0, 0);
    let mut facing = Direction::North;

    println!("Starting position: {:?}, facing: {:?}", position, facing);

    // Simulate movement
    let moves = [
        ("Move forward", false),
        ("Turn right", true),
        ("Move forward", false),
        ("Turn right", true),
        ("Move forward", false),
    ];

    for (action, is_turn) in moves {
        if is_turn {
            facing = facing.turn_right();
            println!("{}: now facing {:?}", action, facing);
        } else {
            let (dx, dy) = facing.to_vector();
            position.0 += dx;
            position.1 += dy;
            println!("{}: moved to {:?}", action, position);
        }
    }

    println!("\nFinal position: {:?}", position);
    println!("Opposite direction: {:?}", facing.opposite());
    println!();
}

fn main() {
    println!("📚 Chapter 6.1: Defining Enums\n");

    example1_basic_enums();
    example2_enums_with_data();
    example3_enum_methods();
    example4_option_enum();
    example5_real_world_usage();
    example6_aoc_pattern();

    println!("✅ All examples completed!");
    println!("📖 Next: Read Chapter 6.2 or run examples in ../match_operator/");
}