//! # Chapter 11.1: How to Write Tests
//!
//! Demonstrates the fundamentals of writing tests in Rust, including
//! test functions, assertion macros, and testing patterns.
//!
//! Run with: `cargo test`

fn main() {
    println!("📚 Chapter 11.1: How to Write Tests");
    println!("====================================");
    
    example1_basic_assertions();
    example2_assertion_macros();
    example3_custom_messages();
    example4_testing_results();
    example5_testing_panics();
    
    println!("✅ All examples completed!");
    println!("📖 Next: Run `cargo test` to see the actual tests in action");
}

fn example1_basic_assertions() {
    println!("🧪 Example 1: Basic Assertion Macros");
    println!("====================================");
    
    // These are examples of what assertions look like
    // The actual tests are in the #[cfg(test)] module below
    
    let result = add_two(2);
    println!("add_two(2) = {}", result);
    
    let greeting = greeting("World");
    println!("greeting(\"World\") = \"{}\"", greeting);
    
    println!();
}

fn example2_assertion_macros() {
    println!("🔍 Example 2: Different Assertion Types");
    println!("======================================");
    
    println!("assert!()     - Tests boolean conditions");
    println!("assert_eq!()  - Tests equality");
    println!("assert_ne!()  - Tests inequality");
    println!("panic!()      - Forces a panic for testing");
    
    println!();
}

fn example3_custom_messages() {
    println!("💬 Example 3: Custom Error Messages");
    println!("===================================");
    
    println!("Tests can include custom messages for better debugging:");
    println!("assert_eq!(result, expected, \"Operation {{}} failed\", operation);");
    
    println!();
}

fn example4_testing_results() {
    println!("📦 Example 4: Testing with Result Types");
    println!("=======================================");
    
    let result = divide(10.0, 2.0);
    println!("divide(10.0, 2.0) = {:?}", result);
    
    let error_result = divide(10.0, 0.0);
    println!("divide(10.0, 0.0) = {:?}", error_result);
    
    println!();
}

fn example5_testing_panics() {
    println!("💥 Example 5: Testing Expected Panics");
    println!("=====================================");
    
    println!("Some functions are expected to panic under certain conditions.");
    println!("We can test these with #[should_panic] attribute.");
    
    // Demonstrate the panic function with safe input
    let safe_result = panics_on_zero(5);
    println!("panics_on_zero(5) = {}", safe_result);
    
    // Demonstrate Rectangle usage
    let rect = Rectangle::new(10, 5);
    println!("Rectangle {{width: {}, height: {}}} has area: {}", 
             rect.width, rect.height, rect.area());
    
    let smaller_rect = Rectangle::new(3, 2);
    println!("Can larger rectangle hold smaller? {}", rect.can_hold(&smaller_rect));
    
    println!();
}

// Functions to test
fn add_two(a: i32) -> i32 {
    a + 2
}

fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn panics_on_zero(x: u32) -> u32 {
    if x == 0 {
        panic!("Cannot process zero value!");
    }
    x * 2
}

// Rectangle struct for testing
#[derive(Debug, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Test module - this is where the actual tests live
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_basic() {
        assert_eq!(add_two(2), 4);
    }

    #[test]
    fn test_add_two_multiple_cases() {
        assert_eq!(add_two(0), 2);
        assert_eq!(add_two(-2), 0);
        assert_eq!(add_two(100), 102);
    }

    #[test]
    fn test_greeting_basic() {
        let result = greeting("World");
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    #[test]
    fn test_greeting_starts_with_hello() {
        let result = greeting("Anyone");
        assert!(result.starts_with("Hello"));
    }

    #[test]
    fn test_assertion_equality() {
        let left = 2 + 2;
        let right = 4;
        assert_eq!(left, right);
    }

    #[test]
    fn test_assertion_inequality() {
        let a = 5;
        let b = 3;
        assert_ne!(a, b);
    }

    #[test]
    fn test_boolean_assertion() {
        assert!(true);
        assert!(!false);
        assert!(2 + 2 == 4);
    }

    #[test]
    fn test_custom_message() {
        let result = add_two(10);
        assert_eq!(
            result, 
            12, 
            "add_two(10) should equal 12, but got {}", 
            result
        );
    }

    #[test]
    fn test_divide_success() -> Result<(), String> {
        let result = divide(10.0, 2.0)?;
        assert_eq!(result, 5.0);
        Ok(())
    }

    #[test]
    fn test_divide_by_zero() {
        let result = divide(10.0, 0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Cannot divide by zero");
    }

    #[test]
    #[should_panic]
    fn test_panics_on_zero_should_panic() {
        panics_on_zero(0);
    }

    #[test]
    #[should_panic(expected = "Cannot process zero value!")]
    fn test_panics_with_expected_message() {
        panics_on_zero(0);
    }

    #[test]
    fn test_panics_on_zero_non_zero_input() {
        let result = panics_on_zero(5);
        assert_eq!(result, 10);
    }

    // Rectangle tests - practical example
    #[test]
    fn test_rectangle_creation() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10);
        assert_eq!(rect.height, 20);
    }

    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle::new(5, 8);
        assert_eq!(rect.area(), 40);
    }

    #[test]
    fn test_rectangle_equality() {
        let rect1 = Rectangle::new(10, 20);
        let rect2 = Rectangle::new(10, 20);
        let rect3 = Rectangle::new(15, 25);
        
        assert_eq!(rect1, rect2);
        assert_ne!(rect1, rect3);
    }

    #[test]
    fn test_larger_can_hold_smaller() {
        let larger = Rectangle::new(8, 7);
        let smaller = Rectangle::new(5, 1);

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn test_smaller_cannot_hold_larger() {
        let larger = Rectangle::new(8, 7);
        let smaller = Rectangle::new(5, 1);

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn test_rectangle_cannot_hold_itself() {
        let rect = Rectangle::new(5, 5);
        assert!(!rect.can_hold(&rect));
    }

    // Testing edge cases
    #[test]
    fn test_empty_values() {
        let result = greeting("");
        assert_eq!(result, "Hello, !");
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(add_two(-10), -8);
        assert_eq!(add_two(-2), 0);
    }

    #[test]
    fn test_large_numbers() {
        assert_eq!(add_two(i32::MAX - 3), i32::MAX - 1);
    }

    // Multiple assertions in one test
    #[test]
    fn test_multiple_operations() {
        let rect = Rectangle::new(10, 5);
        
        assert_eq!(rect.width, 10);
        assert_eq!(rect.height, 5);
        assert_eq!(rect.area(), 50);
        assert!(rect.width > rect.height);
    }

    // Test with vector operations
    #[test]
    fn test_vector_operations() {
        let mut vec = Vec::new();
        assert!(vec.is_empty());
        
        vec.push(1);
        vec.push(2);
        vec.push(3);
        
        assert_eq!(vec.len(), 3);
        assert_eq!(vec[0], 1);
        assert_eq!(vec.pop(), Some(3));
        assert_eq!(vec.len(), 2);
    }

    // Testing with iterators
    #[test]
    fn test_iterator_behavior() {
        let numbers = vec![1, 2, 3, 4, 5];
        let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
        let expected = vec![2, 4, 6, 8, 10];
        
        assert_eq!(doubled, expected);
    }

    // Testing string operations
    #[test]
    fn test_string_operations() {
        let mut s = String::from("Hello");
        s.push_str(", World!");
        
        assert_eq!(s, "Hello, World!");
        assert!(s.contains("World"));
        assert!(s.starts_with("Hello"));
        assert!(s.ends_with("World!"));
    }
}