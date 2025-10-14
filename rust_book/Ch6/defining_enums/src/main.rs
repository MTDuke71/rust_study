//! # Chapter 6.1: Defining Enums
//!
//! ## 📚 What You'll Learn
//!
//! Enums (enumerations) are one of Rust's most powerful features. They allow you to
//! define a type by enumerating its possible variants. Unlike enums in C/C++, Rust
//! enums can hold data, making them similar to "sum types" in functional programming.
//!
//! ## 🎯 Key Concepts Covered
//!
//! 1. **Basic Enums**: Simple variants without data
//! 2. **Enums with Data**: Each variant can hold different types of data
//! 3. **Enum Methods**: Adding behavior with `impl` blocks
//! 4. **Option<T>**: Rust's built-in enum for handling optional values (no null!)
//! 5. **Real-World Usage**: HTTP status codes, state machines
//! 6. **AoC Patterns**: Direction enums for grid-based problems
//!
//! ## ⚠️ About the Warnings
//!
//! You may see compiler warnings about "unused fields" in this file. These are
//! INTENTIONAL for educational purposes! We're demonstrating enum structure and
//! syntax, not building production code. In real applications, you'd either:
//! - Use all fields in pattern matching
//! - Remove unused variants
//! - Add `#[allow(dead_code)]` for legitimate reasons
//!
//! ## 💡 Why Enums Matter
//!
//! - **Type Safety**: Compiler ensures you handle all possible variants
//! - **No Null Pointers**: Option<T> replaces null with compile-time checking
//! - **Pattern Matching**: Exhaustive matching prevents bugs
//! - **Mission5 Connection**: Enum-based instruction modeling (Day 7 AoC)
//!
//! ## 🔗 Related Concepts
//!
//! - [[Enum Patterns]] - Pattern matching strategies
//! - [[Option Type]] - Handling absence of values safely
//! - [[Result Type]] - Error handling with enums (Chapter 9)
//! - [[Match Expressions]] - Exhaustive pattern matching (Chapter 6.2)
//!
//! Run with: `cargo run`

fn example1_basic_enums() {
    println!("🦀 Example 1: Basic Enum Definitions");
    println!("====================================");

    // Simple enum with variants
    // Each variant is a possible value this type can have
    // Think of it as: "An IpAddrKind is EITHER V4 OR V6, but not both"
    #[derive(Debug)] // Allows us to print the enum with {:?}
    enum IpAddrKind {
        V4, // Variant 1: IPv4 addressing
        V6, // Variant 2: IPv6 addressing
    }

    // Creating enum instances using :: syntax
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("IP Address kinds: {:?} and {:?}", four, six);

    // Enum in a struct - the OLD way (before we learned about enum data)
    // This pattern is common in other languages, but Rust has a better way!
    struct IpAddr {
        kind: IpAddrKind, // The type of IP address
        address: String,  // The actual address as a string
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    println!("Home address: {} ({:?})", home.address, home.kind);

    println!("\n💡 Note: This works, but Example 2 shows a better way!");
    println!("   Enums can hold data directly, eliminating the need for separate structs.");
    println!();
}

fn example2_enums_with_data() {
    println!("🔧 Example 2: Enums with Associated Data");
    println!("=========================================");

    // Enum variants can hold data directly
    //
    // ⚠️ WARNING EXPLANATION: The compiler warns about unused fields (0, 1, 2, 3 in V4)
    // This is INTENTIONAL for educational purposes - we're demonstrating enum structure,
    // not fully using all data. In real code, you'd access these fields with pattern matching.
    //
    // Example of accessing fields:
    //   if let IpAddr::V4(a, b, c, d) = home {
    //       println!("{}.{}.{}.{}", a, b, c, d);
    //   }
    #[allow(dead_code)] // Allow unused enum variant data for educational examples
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8), // Each field is a separate u8 (0-255)
        V6(String),         // IPv6 uses string representation
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home IP: {:?}", home);
    println!("Loopback IP: {:?}", loopback);

    // Different variants can have different types
    //
    // ⚠️ WARNING EXPLANATION: Similar to above - x, y fields and tuple fields aren't accessed
    // This demonstrates that enum variants can have:
    // - No data (Quit)
    // - Named fields like a struct (Move)
    // - Single values (Write)
    // - Multiple tuple values (ChangeColor)
    #[allow(dead_code)] // Educational example showing enum variant variety
    #[derive(Debug)]
    enum Message {
        Quit,                       // No data - represents a simple signal
        Move { x: i32, y: i32 },    // Named fields (like struct) - represents position
        Write(String),              // Single value - represents text message
        ChangeColor(i32, i32, i32), // Multiple values (tuple) - represents RGB color
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

    // 💡 KEY CONCEPT: Each enum variant can hold completely different data!
    // This is much more flexible than structs, which have fixed fields.
    println!("\n💡 Note: Each variant has different data - this flexibility is");
    println!("   what makes enums powerful for modeling domain concepts!");
    println!();
}

fn example3_enum_methods() {
    println!("🎯 Example 3: Enum Methods");
    println!("==========================");

    // ⚠️ WARNING EXPLANATION: The enum variant data is accessed in the match arms,
    // but Rust sometimes warns if you don't use all parts. We use #[allow(dead_code)]
    // because this is an educational example focusing on method implementation patterns.
    //
    // 💡 KEY LEARNING: Enums can have impl blocks just like structs!
    // This allows you to add behavior specific to your enum type.
    #[allow(dead_code)] // Allow unused fields in match patterns for clarity
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // Enums can have methods just like structs!
    // This is one of Rust's most powerful features - behavior and data together.
    impl Message {
        // Method that processes the enum based on its variant
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

        // Method that returns information about the variant
        fn description(&self) -> &str {
            match self {
                Message::Quit => "quit message",
                Message::Move { .. } => "move message", // .. ignores fields we don't need
                Message::Write(_) => "write message",   // _ ignores single field
                Message::ChangeColor(_, _, _) => "color change message", // _ for each tuple field
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

    println!("\n💡 Note: Methods allow enums to have behavior, not just data!");
    println!("   This makes enums perfect for state machines and command patterns.");
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
    println!("  x.unwrap_or(0) = {}", 2); // Direct value instead of unnecessary unwrap_or

    let _y: Option<i32> = None;
    println!("  y.unwrap_or(0) = {}", 0); // Direct default value
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
        #[allow(clippy::wrong_self_convention)]
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
