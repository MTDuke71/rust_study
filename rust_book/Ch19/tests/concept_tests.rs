//! Chapter 19: Pattern Matching - Concept Tests
//!
//! These tests validate understanding of pattern matching concepts.

use ch19::*;

/// Test: Pattern locations work correctly
#[test]
fn test_pattern_locations() {
    // match expression
    let x = 1;
    let result = match x {
        1 => "one",
        _ => "other",
    };
    assert_eq!(result, "one");

    // if let
    let option = Some(42);
    if let Some(value) = option {
        assert_eq!(value, 42);
    } else {
        panic!("Should match Some");
    }

    // while let
    let mut stack = vec![1, 2, 3];
    let mut sum = 0;
    while let Some(top) = stack.pop() {
        sum += top;
    }
    assert_eq!(sum, 6);

    // for loop
    let mut results = vec![];
    for (index, value) in ['a', 'b', 'c'].iter().enumerate() {
        results.push((index, *value));
    }
    assert_eq!(results, vec![(0, 'a'), (1, 'b'), (2, 'c')]);

    // let statement
    let (a, b, c) = (1, 2, 3);
    assert_eq!(a, 1);
    assert_eq!(b, 2);
    assert_eq!(c, 3);

    // function parameter
    fn print_coords(&(x, y): &(i32, i32)) -> i32 {
        x + y
    }
    assert_eq!(print_coords(&(3, 5)), 8);
}

/// Test: Refutability rules are enforced
#[test]
fn test_refutability() {
    // Irrefutable pattern in let
    let _x = 5; // Always matches
    let (_a, _b) = (1, 2); // Always matches

    // Refutable pattern requires if let / while let / match
    let option = Some(3);
    if let Some(x) = option {
        assert_eq!(x, 3);
    }

    // Direct access when we know it's Some
    let result = 3;
    assert_eq!(result, 3);
}

/// Test: Literal pattern matching
#[test]
fn test_literal_patterns() {
    fn classify_number(x: i32) -> &'static str {
        match x {
            1 => "one",
            2 => "two",
            3 => "three",
            _ => "other",
        }
    }

    assert_eq!(classify_number(1), "one");
    assert_eq!(classify_number(2), "two");
    assert_eq!(classify_number(99), "other");
}

/// Test: Named variable shadowing in patterns
#[test]
fn test_named_variables() {
    let x = Some(5);
    let y = 10;

    let result = match x {
        Some(50) => 50,
        Some(y) => y, // This y shadows outer y
        _ => 0,
    };

    assert_eq!(result, 5); // Inner y was 5
    assert_eq!(y, 10); // Outer y unchanged
}

/// Test: Multiple patterns with |
#[test]
fn test_multiple_patterns() {
    fn is_primary_color(c: Color) -> bool {
        matches!(c, Color::Rgb(255, 0, 0) | Color::Rgb(0, 255, 0) | Color::Rgb(0, 0, 255))
    }

    assert!(is_primary_color(Color::Rgb(255, 0, 0)));
    assert!(is_primary_color(Color::Rgb(0, 255, 0)));
    assert!(!is_primary_color(Color::Rgb(128, 128, 128)));
}

/// Test: Range patterns with ..=
#[test]
fn test_ranges() {
    fn classify_score(score: u32) -> &'static str {
        match score {
            0..=59 => "F",
            60..=69 => "D",
            70..=79 => "C",
            80..=89 => "B",
            90..=100 => "A",
            _ => "Invalid",
        }
    }

    assert_eq!(classify_score(45), "F");
    assert_eq!(classify_score(75), "C");
    assert_eq!(classify_score(95), "A");

    // Character ranges
    fn is_lowercase(c: char) -> bool {
        c.is_ascii_lowercase()
    }

    assert!(is_lowercase('m'));
    assert!(!is_lowercase('M'));
}

/// Test: Struct destructuring
#[test]
fn test_struct_destructuring() {
    let p = Point::new(3, 5);

    let result = match p {
        Point { x: 0, y } => format!("y-axis: {}", y),
        Point { x, y: 0 } => format!("x-axis: {}", x),
        Point { x, y } => format!("({}, {})", x, y),
    };

    assert_eq!(result, "(3, 5)");

    // Partial destructuring
    let Point { x, .. } = p;
    assert_eq!(x, 3);
}

/// Test: Enum destructuring
#[test]
fn test_enum_destructuring() {
    let msg = Message::Move { x: 10, y: 20 };

    let (x, y) = match msg {
        Message::Move { x, y } => (x, y),
        _ => (0, 0),
    };

    assert_eq!(x, 10);
    assert_eq!(y, 20);
}

/// Test: Nested destructuring
#[test]
fn test_nested_destructuring() {
    let event = Event::ChangeColor(Color::Rgb(255, 128, 0));

    let color_type = match event {
        Event::ChangeColor(Color::Rgb(r, g, b)) => format!("RGB({},{},{})", r, g, b),
        Event::ChangeColor(Color::Hsv(h, s, v)) => format!("HSV({},{},{})", h, s, v),
        Event::Move(Point { x, y }) => format!("Move({},{})", x, y),
        Event::Quit => "Quit".to_string(),
    };

    assert_eq!(color_type, "RGB(255,128,0)");
}

/// Test: Ignoring values with _
#[test]
fn test_ignoring_with_underscore() {
    let (x, _, z) = (1, 2, 3);
    assert_eq!(x, 1);
    assert_eq!(z, 3);

    // Ignore entire enum variant data
    let msg = Message::Write("Hello".to_string());
    let is_write = matches!(msg, Message::Write(_));
    assert!(is_write);
}

/// Test: Ignoring remaining parts with ..
#[test]
fn test_ignoring_with_rest() {
    let p = Point::new(100, 200);
    let Point { x, .. } = p;
    assert_eq!(x, 100);

    // With tuples
    let tuple = (1, 2, 3, 4, 5);
    let (first, .., last) = tuple;
    assert_eq!(first, 1);
    assert_eq!(last, 5);

    // With slices - irrefutable pattern for fixed-size arrays
    let numbers = [10, 20, 30, 40, 50];
    let [first, .., last] = numbers;
    assert_eq!(first, 10);
    assert_eq!(last, 50);
}

/// Test: Match guards add conditional logic
#[test]
fn test_match_guards() {
    fn describe_number(x: i32) -> &'static str {
        match x {
            n if n < 0 => "negative",
            0 => "zero",
            n if n % 2 == 0 => "positive even",
            _ => "positive odd",
        }
    }

    assert_eq!(describe_number(-5), "negative");
    assert_eq!(describe_number(0), "zero");
    assert_eq!(describe_number(4), "positive even");
    assert_eq!(describe_number(7), "positive odd");
}

/// Test: Match guards with multiple patterns
#[test]
fn test_guards_with_multiple_patterns() {
    fn check(x: i32, flag: bool) -> &'static str {
        match x {
            4..=6 if flag => "yes",
            _ => "no",
        }
    }

    assert_eq!(check(4, true), "yes");
    assert_eq!(check(5, true), "yes");
    assert_eq!(check(6, false), "no");
    assert_eq!(check(7, true), "no");
}

/// Test: @ bindings capture matched values
#[test]
fn test_at_bindings() {
    #[allow(dead_code)]
    enum Priority {
        Low(i32),
        Medium(i32),
        High(i32),
    }

    let priority = Priority::High(7);

    let result = match priority {
        Priority::High(id @ 1..=10) => format!("Critical: {}", id),
        Priority::High(id @ 11..=100) => format!("Standard: {}", id),
        Priority::High(id) => format!("Low: {}", id),
        Priority::Medium(id) => format!("Medium: {}", id),
        Priority::Low(id) => format!("Low: {}", id),
    };

    assert_eq!(result, "Critical: 7");
}

/// Test: @ bindings with structs
#[test]
fn test_at_bindings_structs() {
    let point = Point::new(5, 10);

    let result = match point {
        Point { x: x @ 0..=5, y } => format!("Left zone: ({}, {})", x, y),
        Point { x: x @ 6..=10, y } => format!("Right zone: ({}, {})", x, y),
        Point { x, y } => format!("Far: ({}, {})", x, y),
    };

    assert_eq!(result, "Left zone: (5, 10)");
}

/// Test: Library utility functions
#[test]
fn test_lib_functions() {
    assert_eq!(classify_number(5), "small");
    assert_eq!(classify_number(50), "medium");
    assert_eq!(classify_number(500), "large");

    assert_eq!(get_quadrant(Point::new(5, 5)), "Q1");
    assert_eq!(get_quadrant(Point::new(-5, 5)), "Q2");
    assert_eq!(get_quadrant(Point::new(0, 5)), "Y-axis");

    assert_eq!(categorize_color(&Color::Rgb(255, 0, 0)), "RGB");
    assert_eq!(categorize_color(&Color::Hsv(180, 50, 50)), "HSV");

    assert_eq!(event_type(&Event::Move(Point::new(1, 1))), "Move");
    assert_eq!(event_type(&Event::Quit), "Quit");
}

/// Test: Complex real-world pattern matching
#[test]
fn test_complex_patterns() {
    #[derive(Debug)]
    #[allow(dead_code)]
    enum Status {
        Active { tasks: Vec<i32>, priority: u8 },
        Idle,
        Error { code: i32, message: String },
    }

    fn process_status(status: Status) -> String {
        match status {
            Status::Active {
                tasks,
                priority: _p @ 1..=3,
            } if !tasks.is_empty() => {
                format!("High priority with {} tasks", tasks.len())
            }
            Status::Active { tasks, .. } if tasks.is_empty() => "Active but no tasks".to_string(),
            Status::Active { tasks, priority } => {
                format!("{} tasks, priority {}", tasks.len(), priority)
            }
            Status::Idle => "Waiting".to_string(),
            Status::Error { code: 404, .. } => "Not found".to_string(),
            Status::Error { code, message } => {
                format!("Error {}: {}", code, message)
            }
        }
    }

    let status1 = Status::Active {
        tasks: vec![1, 2, 3],
        priority: 2,
    };
    assert_eq!(process_status(status1), "High priority with 3 tasks");

    let status2 = Status::Active {
        tasks: vec![],
        priority: 5,
    };
    assert_eq!(process_status(status2), "Active but no tasks");

    let status3 = Status::Error {
        code: 404,
        message: "Page not found".to_string(),
    };
    assert_eq!(process_status(status3), "Not found");
}

/// Test: Tuple destructuring in various contexts
#[test]
fn test_tuple_patterns() {
    // Nested tuples
    let ((a, b), (c, d)) = ((1, 2), (3, 4));
    assert_eq!((a, b, c, d), (1, 2, 3, 4));

    // In match
    let pair = (5, 10);
    let (x, y) = pair;
    let sum = x + y;
    assert_eq!(sum, 15);

    // In for loop
    let pairs = vec![(1, 2), (3, 4), (5, 6)];
    let mut sums = vec![];
    for (x, y) in pairs {
        sums.push(x + y);
    }
    assert_eq!(sums, vec![3, 7, 11]);
}

/// Test: Array pattern matching
#[test]
fn test_array_patterns() {
    let numbers = [1, 2, 3, 4, 5];

    match numbers {
        [first, .., last] => {
            assert_eq!(first, 1);
            assert_eq!(last, 5);
        }
    }

    // Fixed-size array
    let triple = [10, 20, 30];
    let [a, b, c] = triple;
    assert_eq!(a + b + c, 60);
}

/// Test: Combining multiple pattern techniques
#[test]
fn test_combined_patterns() {
    fn analyze(point: Point, threshold: i32) -> String {
        match point {
            // @ binding + guard + destructuring
            Point { x: x @ 0..=5, y } if y > threshold => {
                format!("Near origin high: ({}, {})", x, y)
            }
            // Range + guard
            Point { x: 0..=5, y: 0..=5 } => "Near origin".to_string(),
            // Named variables
            Point { x, y } if x == y => {
                format!("Diagonal at {}", x)
            }
            // Rest
            Point { x, .. } if x > 10 => "Far right".to_string(),
            // Catch-all
            _ => "Other".to_string(),
        }
    }

    assert_eq!(analyze(Point::new(3, 20), 10), "Near origin high: (3, 20)");
    assert_eq!(analyze(Point::new(2, 2), 10), "Near origin");
    assert_eq!(analyze(Point::new(7, 7), 10), "Diagonal at 7");
    assert_eq!(analyze(Point::new(15, 3), 10), "Far right");
}
