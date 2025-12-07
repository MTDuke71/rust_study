//! Chapter 19.3: Pattern Syntax
//!
//! This example demonstrates comprehensive pattern syntax including:
//! - Matching literals
//! - Matching named variables
//! - Multiple patterns with |
//! - Matching ranges with ..=
//! - Destructuring structs, enums, tuples, and arrays
//! - Ignoring values with _ and ..
//! - Match guards
//! - @ bindings

use ch19::*;

fn main() {
    println!("=== Chapter 19.3: Pattern Syntax ===\n");
    
    demonstrate_literals();
    demonstrate_named_variables();
    demonstrate_multiple_patterns();
    demonstrate_ranges();
    demonstrate_destructuring();
    demonstrate_ignoring();
    demonstrate_match_guards();
    demonstrate_at_bindings();
    demonstrate_comprehensive_example();
    
    println!("\n✅ All pattern syntax demonstrated!");
}

/// 1. Matching Literals
///
/// **Book Reference**: Section 19.3 - Matching Literals
/// **Key Learning**: Direct value comparison in patterns
fn demonstrate_literals() {
    println!("--- 1. Matching Literals ---");
    
    let x = 1;
    match x {
        1 => println!("  one"),
        2 => println!("  two"),
        3 => println!("  three"),
        _ => println!("  anything else"),
    }
    
    // String literals (must use &str)
    let name = "Alice";
    match name {
        "Alice" => println!("  Hello, Alice!"),
        "Bob" => println!("  Hello, Bob!"),
        _ => println!("  Hello, stranger!"),
    }
    
    println!("💡 Literals provide exact value matching");
    println!();
}

/// 2. Matching Named Variables
///
/// **Book Reference**: Section 19.3 - Named Variables
/// **Key Learning**: Variables in patterns create NEW bindings (shadow outer)
fn demonstrate_named_variables() {
    println!("--- 2. Named Variables (Shadowing) ---");
    
    let x = Some(5);
    let y = 10;
    
    match x {
        Some(50) => println!("  Got 50"),
        Some(y) => println!("  Matched, inner y = {}", y),  // Shadows outer y!
        _ => println!("  Default case"),
    }
    
    println!("  Outer y still = {} (not shadowed outside match)", y);
    
    println!("💡 Pattern variables shadow outer variables within match arm");
    println!();
}

/// 3. Multiple Patterns with |
///
/// **Book Reference**: Section 19.3 - Multiple Patterns
/// **Key Learning**: OR operator for alternative patterns
fn demonstrate_multiple_patterns() {
    println!("--- 3. Multiple Patterns with | ---");
    
    let x = 1;
    match x {
        1 | 2 => println!("  one or two"),
        3 | 4 | 5 => println!("  three, four, or five"),
        _ => println!("  something else"),
    }
    
    // Works with chars too
    let ch = 'c';
    match ch {
        'a' | 'e' | 'i' | 'o' | 'u' => println!("  '{}' is a vowel", ch),
        _ => println!("  '{}' is a consonant", ch),
    }
    
    println!("💡 Use | to match multiple alternatives");
    println!();
}

/// 4. Matching Ranges with ..=
///
/// **Book Reference**: Section 19.3 - Matching Ranges
/// **Key Learning**: Inclusive range matching with ..=
fn demonstrate_ranges() {
    println!("--- 4. Matching Ranges with ..= ---");
    
    let x = 5;
    match x {
        1..=5 => println!("  {} is between 1 and 5 (inclusive)", x),
        6..=10 => println!("  between 6 and 10"),
        _ => println!("  something else"),
    }
    
    // Character ranges
    let c = 'k';
    match c {
        'a'..='j' => println!("  '{}' is early in alphabet", c),
        'k'..='z' => println!("  '{}' is late in alphabet", c),
        _ => println!("  not a lowercase letter"),
    }
    
    println!("💡 Ranges work with numbers and chars (ordered types)");
    println!();
}

/// 5. Destructuring Structs, Enums, Tuples, Arrays
///
/// **Book Reference**: Section 19.3 - Destructuring
/// **Key Learning**: Break apart complex types in patterns
fn demonstrate_destructuring() {
    println!("--- 5. Destructuring ---");
    
    // Struct destructuring
    let p = Point::new(0, 7);
    match p {
        Point { x: 0, y } => println!("  On y-axis at y={}", y),
        Point { x, y: 0 } => println!("  On x-axis at x={}", x),
        Point { x, y } => println!("  At ({}, {})", x, y),
    }
    
    // Enum destructuring
    let msg = Message::ChangeColor(255, 128, 0);
    match msg {
        Message::Quit => println!("  Quit message"),
        Message::Move { x, y } => println!("  Move to ({}, {})", x, y),
        Message::Write(text) => println!("  Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("  Color: RGB({}, {}, {})", r, g, b),
    }
    
    // Nested destructuring
    let event = Event::ChangeColor(Color::Hsv(180, 100, 50));
    match event {
        Event::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("  RGB color: ({}, {}, {})", r, g, b);
        }
        Event::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("  HSV color: ({}, {}, {})", h, s, v);
        }
        Event::Move(Point { x, y }) => {
            println!("  Move to ({}, {})", x, y);
        }
        Event::Quit => println!("  Quit"),
    }
    
    // Tuple destructuring
    let ((a, b), (c, d)) = ((1, 2), (3, 4));
    println!("  Nested tuple: a={}, b={}, c={}, d={}", a, b, c, d);
    
    // Array/slice destructuring
    let numbers = [1, 2, 3, 4, 5];
    match numbers {
        [first, .., last] => println!("  Array: first={}, last={}", first, last),
    }
    
    println!("💡 Destructuring breaks apart complex types elegantly");
    println!();
}

/// 6. Ignoring Values with _ and ..
///
/// **Book Reference**: Section 19.3 - Ignoring Values
/// **Key Learning**: Skip parts you don't need
fn demonstrate_ignoring() {
    println!("--- 6. Ignoring Values ---");
    
    // Ignore entire value with _
    fn process_pair(_first: i32, second: i32) -> i32 {
        second
    }
    println!("  Ignoring first param: {}", process_pair(10, 20));
    
    // Ignore tuple elements
    let (x, _, z) = (1, 2, 3);
    println!("  Tuple ignore: x={}, z={} (middle ignored)", x, z);
    
    // Ignore struct fields with ..
    let p = Point::new(10, 20);
    let Point { x, .. } = p;
    println!("  Struct ignore: x={} (y ignored)", x);
    
    // Ignore middle of slice with ..
    let numbers = [1, 2, 3, 4, 5];
    match numbers {
        [first, .., last] => {
            println!("  Slice ignore: first={}, last={}", first, last);
        }
    }
    
    // Prefix unused variable with _
    let _unused = 42;  // No warning about unused variable
    
    println!("💡 Use _ for single values, .. for rest");
    println!();
}

/// 7. Match Guards - Extra if Conditions
///
/// **Book Reference**: Section 19.3 - Match Guards
/// **Key Learning**: Add boolean conditions to patterns
fn demonstrate_match_guards() {
    println!("--- 7. Match Guards ---");
    
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("  Even number: {}", x),
        Some(x) => println!("  Odd number: {}", x),
        None => println!("  No number"),
    }
    
    // Guards with multiple patterns
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("  yes"),  // Guard applies to ALL alternatives
        _ => println!("  no (guard failed or different value)"),
    }
    
    // Guards for complex conditions
    let point = Point::new(5, 5);
    match point {
        Point { x, y } if x == y => println!("  On diagonal at ({}, {})", x, y),
        Point { x, .. } if x > 0 => println!("  Right half at x={}", x),
        Point { x, .. } if x < 0 => println!("  Left half at x={}", x),
        _ => println!("  On y-axis"),
    }
    
    println!("💡 Guards add conditional logic to pattern matching");
    println!();
}

/// 8. @ Bindings - Bind and Test Simultaneously
///
/// **Book Reference**: Section 19.3 - @ Bindings
/// **Key Learning**: Capture value while testing it
fn demonstrate_at_bindings() {
    println!("--- 8. @ Bindings ---");
    
    #[allow(dead_code)]
    enum Priority {
        Low { id: i32 },
        Medium { id: i32 },
        High { id: i32 },
    }
    
    let priority = Priority::High { id: 7 };
    
    match priority {
        Priority::High { id: id_var @ 1..=10 } => {
            println!("  Critical high priority with id {}", id_var);
        }
        Priority::High { id: id_var @ 11..=100 } => {
            println!("  Standard high priority with id {}", id_var);
        }
        Priority::High { id } => {
            println!("  Low high priority with id {}", id);
        }
        Priority::Medium { id } => println!("  Medium priority: {}", id),
        Priority::Low { id } => println!("  Low priority: {}", id),
    }
    
    // @ binding with match guard
    let result = categorize_with_binding(50);
    println!("  {}", result);
    
    println!("💡 @ bindings capture matched values in range tests");
    println!();
}

/// 9. Comprehensive Example - Combining All Syntax
///
/// **Integration**: Shows how patterns work together
fn demonstrate_comprehensive_example() {
    println!("--- 9. Comprehensive Pattern Syntax ---");
    
    #[derive(Debug)]
    #[allow(dead_code)]
    enum Task {
        Todo { id: i32, priority: u8, description: String },
        InProgress { id: i32, assigned_to: String },
        Done { id: i32 },
    }
    
    fn process_task(task: Task) -> String {
        match task {
            // @ binding with range and guard
            Task::Todo {
                id: task_id @ 1..=100,
                priority: p @ 1..=3,
                ..
            } if p == 1 => {
                format!("URGENT! Task #{} priority {}", task_id, p)
            }
            
            // Destructuring with ignoring
            Task::Todo { id, priority, .. } => {
                format!("Todo #{} with priority {}", id, priority)
            }
            
            // Named variable pattern
            Task::InProgress { id, assigned_to } => {
                format!("Task #{} assigned to {}", id, assigned_to)
            }
            
            // Simple destructuring
            Task::Done { id } => {
                format!("Task #{} completed", id)
            }
        }
    }
    
    let tasks = vec![
        Task::Todo {
            id: 5,
            priority: 1,
            description: "Fix critical bug".to_string(),
        },
        Task::InProgress {
            id: 42,
            assigned_to: "Alice".to_string(),
        },
        Task::Done { id: 100 },
    ];
    
    println!("Processing tasks:");
    for task in tasks {
        println!("  {}", process_task(task));
    }
    
    println!("\n💡 All pattern syntax works together seamlessly!");
    println!("🎯 Ready to use patterns in [[mission-8]] and AoC problems!");
}
