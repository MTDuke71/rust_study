//! Chapter 19.1: All the Places Patterns Can Be Used
//!
//! This example demonstrates all six locations where patterns can be used in Rust:
//! 1. match expressions
//! 2. if let expressions
//! 3. while let loops
//! 4. for loops
//! 5. let statements
//! 6. function parameters

use ch19::*;

fn main() {
    println!("=== Chapter 19.1: Pattern Locations ===\n");

    demonstrate_match_expressions();
    demonstrate_if_let();
    demonstrate_while_let();
    demonstrate_for_loops();
    demonstrate_let_statements();
    demonstrate_function_parameters();
    demonstrate_integration();

    println!("\n✅ All pattern locations demonstrated!");
}

/// Location 1: match expressions (most common)
///
/// **Book Reference**: Section 19.1 - match Arms
/// **Key Learning**: match requires exhaustive patterns
fn demonstrate_match_expressions() {
    println!("--- Location 1: match Expressions ---");

    let point = Point::new(3, 0);
    let description = describe_point(point);
    println!("Point {:?} is: {}", point, description);

    // Multiple match patterns
    let number = 5;
    let category = match number {
        1 | 2 => "one or two",
        3..=10 => "small",
        11..=100 => "medium",
        _ => "large",
    };
    println!("Number {} is {}", number, category);

    // Enum matching
    let msg = Message::Move { x: 10, y: 20 };
    match msg {
        Message::Quit => println!("Quitting"),
        Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
        Message::Write(ref text) => println!("Writing: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: RGB({}, {}, {})", r, g, b),
    }

    println!("💡 match expressions are exhaustive and type-safe");
    println!();
}

/// Location 2: if let expressions
///
/// **Book Reference**: Section 19.1 - Conditional if let
/// **Integration**: Cleaner than match for single pattern
fn demonstrate_if_let() {
    println!("--- Location 2: if let Expressions ---");

    let config_max = Some(3u8);

    // if let - cleaner than match for single case
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // if let with else
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using favorite color: {}", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as color for age {}", age);
        } else {
            println!("Using orange as color");
        }
    } else {
        println!("Using blue as default");
    }

    println!("💡 if let is perfect for single-pattern matching");
    println!();
}

/// Location 3: while let loops
///
/// **Book Reference**: Section 19.1 - while let Conditional Loops
/// **Key Learning**: Loop while pattern continues to match
fn demonstrate_while_let() {
    println!("--- Location 3: while let Loops ---");

    let mut stack = vec![1, 2, 3];
    println!("Draining stack: {:?}", stack);

    // while let - continues until pattern fails to match
    while let Some(top) = stack.pop() {
        println!("  Popped: {}", top);
    }

    println!("Stack is now empty: {:?}", stack);

    // Another example: processing iterator
    let chars = "hello".chars();
    let mut count = 0;
    for _c in chars {
        count += 1;
    }
    println!("Counted {} characters", count);

    println!("💡 while let loops until pattern stops matching");
    println!();
}

/// Location 4: for loops
///
/// **Book Reference**: Section 19.1 - for Loops
/// **Key Learning**: for loops always use patterns
fn demonstrate_for_loops() {
    println!("--- Location 4: for Loops ---");

    // Tuple destructuring in for loop
    let points = vec![Point::new(0, 0), Point::new(1, 1), Point::new(2, 4)];

    println!("Iterating over points with destructuring:");
    for Point { x, y } in points {
        println!("  Point: ({}, {})", x, y);
    }

    // Enumerate with tuple destructuring
    let v = ['a', 'b', 'c'];
    println!("\nEnumerate with destructuring:");
    for (index, value) in v.iter().enumerate() {
        println!("  {} is at index {}", value, index);
    }

    println!("💡 for loops always destructure - even `x` in `for x` is a pattern!");
    println!();
}

/// Location 5: let statements
///
/// **Book Reference**: Section 19.1 - let Statements
/// **Key Learning**: let ALWAYS uses a pattern
fn demonstrate_let_statements() {
    println!("--- Location 5: let Statements ---");

    // Simple pattern
    let x = 5;
    println!("Simple pattern: x = {}", x);

    // Tuple destructuring
    let (a, b, c) = (1, 2, 3);
    println!("Tuple destructuring: a={}, b={}, c={}", a, b, c);

    // Nested destructuring
    let ((x1, y1), (x2, y2)) = ((1, 2), (3, 4));
    println!("Nested tuple: ({}, {}), ({}, {})", x1, y1, x2, y2);

    // Struct destructuring
    let Point { x: px, y: py } = Point::new(10, 20);
    println!("Struct destructuring: px={}, py={}", px, py);

    // Shorthand struct destructuring
    let Point { x, y } = Point::new(30, 40);
    println!("Shorthand destructuring: x={}, y={}", x, y);

    println!("💡 Every let statement uses a pattern - `let x = 5` has pattern `x`!");
    println!();
}

/// Location 6: function parameters
///
/// **Book Reference**: Section 19.1 - Function Parameters
/// **Key Learning**: Function parameters are patterns too
fn demonstrate_function_parameters() {
    println!("--- Location 6: Function Parameters ---");

    // Function with tuple destructuring parameter
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("  Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    println!("Using destructuring in function parameter:");
    print_coordinates(&point);

    // Function with struct destructuring
    fn describe_point_param(Point { x, y }: Point) -> String {
        format!("Point at ({}, {})", x, y)
    }

    let p = Point::new(7, 9);
    println!(
        "Struct destructuring in parameter: {}",
        describe_point_param(p)
    );

    println!("💡 Function parameters are patterns - enables concise destructuring");
    println!();
}

/// Integration example: Combining pattern locations
fn demonstrate_integration() {
    println!("--- Integration: Multiple Pattern Locations ---");

    fn process_events(events: Vec<Event>) {
        // Location 4: for loop with destructuring
        for event in events {
            // Location 2: if let
            if let Event::ChangeColor(color) = event {
                // Location 1: match
                match color {
                    Color::Rgb(r, g, b) => println!("RGB color: {}, {}, {}", r, g, b),
                    Color::Hsv(h, s, v) => println!("HSV color: {}, {}, {}", h, s, v),
                }
            }
        }
    }

    let events = vec![
        Event::ChangeColor(Color::Rgb(255, 0, 0)),
        Event::Move(Point::new(10, 20)),
        Event::Quit,
    ];

    process_events(events);

    println!("💡 Patterns work together seamlessly across all locations");
}
