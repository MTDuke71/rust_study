//! Chapter 19: Patterns and Matching
//!
//! This module contains reusable types and functions for demonstrating
//! pattern matching concepts from the Rust Book Chapter 19.

/// A 2D point used for pattern matching demonstrations
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn origin() -> Self {
        Point { x: 0, y: 0 }
    }
}

/// Message enum for demonstrating enum destructuring
#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

/// Color enum for nested destructuring demonstrations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

/// Event enum with nested Color for complex pattern matching
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    ChangeColor(Color),
    Move(Point),
    Quit,
}

/// Describes a point using pattern matching
///
/// # Requirements Satisfied: Pattern matching basics
///
/// # Examples
/// ```
/// use ch19::*;
/// assert_eq!(describe_point(Point::origin()), "origin");
/// assert_eq!(describe_point(Point::new(5, 0)), "on x-axis at 5");
/// ```
pub fn describe_point(point: Point) -> String {
    match point {
        Point { x: 0, y: 0 } => "origin".to_string(),
        Point { x, y: 0 } => format!("on x-axis at {}", x),
        Point { x: 0, y } => format!("on y-axis at {}", y),
        Point { x, y } => format!("at ({}, {})", x, y),
    }
}

/// Checks if a number is in a specific range using match guards
///
/// # Requirements Satisfied: Match guards
pub fn classify_number(n: i32) -> &'static str {
    match n {
        n if n < 0 => "negative",
        0 => "zero",
        n if n < 10 => "small",
        n if n < 100 => "medium",
        _ => "large",
    }
}

/// Gets the quadrant or axis location of a point
///
/// # Requirements Satisfied: Pattern matching with guards
pub fn get_quadrant(point: Point) -> &'static str {
    match point {
        Point { x: 0, y: 0 } => "Origin",
        Point { x: 0, .. } => "Y-axis",
        Point { y: 0, .. } => "X-axis",
        Point { x, y } if x > 0 && y > 0 => "Q1",
        Point { x, y } if x < 0 && y > 0 => "Q2",
        Point { x, y } if x < 0 && y < 0 => "Q3",
        _ => "Q4",
    }
}

/// Categorizes a color type
///
/// # Requirements Satisfied: Enum pattern matching
pub fn categorize_color(color: &Color) -> &'static str {
    match color {
        Color::Rgb(..) => "RGB",
        Color::Hsv(..) => "HSV",
    }
}

/// Gets the event type as a string
///
/// # Requirements Satisfied: Enum pattern matching
pub fn event_type(event: &Event) -> &'static str {
    match event {
        Event::ChangeColor(_) => "ChangeColor",
        Event::Move(_) => "Move",
        Event::Quit => "Quit",
    }
}

/// Processes a message using comprehensive enum destructuring
///
/// # Requirements Satisfied: Enum destructuring
pub fn process_message(msg: Message) -> String {
    match msg {
        Message::Quit => "Quitting".to_string(),
        Message::Move { x, y } => format!("Moving to ({}, {})", x, y),
        Message::Write(text) => format!("Writing: {}", text),
        Message::ChangeColor(r, g, b) => format!("Changing color to RGB({}, {}, {})", r, g, b),
    }
}

/// Processes an event with nested enum destructuring
///
/// # Requirements Satisfied: Nested destructuring
pub fn process_event(event: Event) -> String {
    match event {
        Event::Quit => "Quitting".to_string(),
        Event::Move(Point { x, y }) => format!("Moving to ({}, {})", x, y),
        Event::ChangeColor(Color::Rgb(r, g, b)) => {
            format!("Changing to RGB({}, {}, {})", r, g, b)
        }
        Event::ChangeColor(Color::Hsv(h, s, v)) => {
            format!("Changing to HSV({}, {}, {})", h, s, v)
        }
    }
}

/// Extracts first and last elements from a slice using pattern matching
///
/// # Requirements Satisfied: Slice patterns
pub fn first_and_last<T: Clone>(slice: &[T]) -> Option<(T, T)> {
    match slice {
        [first, .., last] => Some((first.clone(), last.clone())),
        [single] => Some((single.clone(), single.clone())),
        [] => None,
    }
}

/// Demonstrates @ bindings by categorizing a value and binding it
///
/// # Requirements Satisfied: @ bindings
pub fn categorize_with_binding(id: i32) -> String {
    match id {
        id_var @ 1..=10 => format!("Small ID: {}", id_var),
        id_var @ 11..=100 => format!("Medium ID: {}", id_var),
        id_var @ 101..=1000 => format!("Large ID: {}", id_var),
        id_var => format!("Extra large ID: {}", id_var),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_point() {
        assert_eq!(describe_point(Point::origin()), "origin");
        assert_eq!(describe_point(Point::new(5, 0)), "on x-axis at 5");
        assert_eq!(describe_point(Point::new(0, 3)), "on y-axis at 3");
        assert_eq!(describe_point(Point::new(4, 7)), "at (4, 7)");
    }

    #[test]
    fn test_classify_number() {
        assert_eq!(classify_number(-5), "negative");
        assert_eq!(classify_number(0), "zero");
        assert_eq!(classify_number(5), "small");
        assert_eq!(classify_number(50), "medium");
        assert_eq!(classify_number(500), "large");
    }

    #[test]
    fn test_process_message() {
        assert_eq!(process_message(Message::Quit), "Quitting");
        assert_eq!(
            process_message(Message::Move { x: 10, y: 20 }),
            "Moving to (10, 20)"
        );
        assert_eq!(
            process_message(Message::Write("Hello".to_string())),
            "Writing: Hello"
        );
    }

    #[test]
    fn test_first_and_last() {
        assert_eq!(first_and_last(&[1, 2, 3, 4, 5]), Some((1, 5)));
        assert_eq!(first_and_last(&[42]), Some((42, 42)));
        assert_eq!(first_and_last(&[] as &[i32]), None);
    }

    #[test]
    fn test_categorize_with_binding() {
        assert_eq!(categorize_with_binding(5), "Small ID: 5");
        assert_eq!(categorize_with_binding(50), "Medium ID: 50");
        assert_eq!(categorize_with_binding(500), "Large ID: 500");
        assert_eq!(categorize_with_binding(5000), "Extra large ID: 5000");
    }
}
