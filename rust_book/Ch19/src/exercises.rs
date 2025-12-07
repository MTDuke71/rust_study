//! Chapter 19 Exercises - Pattern Matching Mastery
//!
//! These exercises test understanding of all Chapter 19 concepts through
//! practical implementation challenges with increasing complexity.

#![allow(dead_code)]

use ch19::*;

/// Exercise 1: Multiple Pattern Matching
///
/// **Objective**: Demonstrate matching multiple patterns with |
/// **Skills**: Pattern alternatives, ranges
#[cfg(test)]
mod exercise_1_multiple_patterns {
    #[test]
    fn test_weekday_classification() {
        fn classify_day(day: u8) -> &'static str {
            match day {
                1..=5 => "weekday",
                6 | 7 => "weekend",
                _ => "invalid day",
            }
        }
        
        assert_eq!(classify_day(1), "weekday");
        assert_eq!(classify_day(3), "weekday");
        assert_eq!(classify_day(6), "weekend");
        assert_eq!(classify_day(7), "weekend");
        assert_eq!(classify_day(8), "invalid day");
        
        println!("✅ Multiple pattern matching works correctly");
    }
    
    #[test]
    fn test_character_classification() {
        fn classify_char(c: char) -> &'static str {
            match c {
                'a'..='z' => "lowercase",
                'A'..='Z' => "uppercase",
                '0'..='9' => "digit",
                ' ' | '\t' | '\n' => "whitespace",
                _ => "special",
            }
        }
        
        assert_eq!(classify_char('a'), "lowercase");
        assert_eq!(classify_char('Z'), "uppercase");
        assert_eq!(classify_char('5'), "digit");
        assert_eq!(classify_char(' '), "whitespace");
        assert_eq!(classify_char('@'), "special");
        
        println!("✅ Character classification with patterns works");
    }
}

/// Exercise 2: Refutability Patterns
///
/// **Objective**: Correctly use refutable and irrefutable patterns
/// **Integration**: Prepares for [[mission-3]] Option handling
#[cfg(test)]
mod exercise_2_refutability {
    #[test]
    fn test_irrefutable_patterns() {
        // Tuple destructuring - always succeeds
        let (x, y) = (1, 2);
        assert_eq!(x, 1);
        assert_eq!(y, 2);
        
        // Struct destructuring - always succeeds
        #[derive(Debug)]
        struct Config {
            width: u32,
            height: u32,
        }
        
        let Config { width, height } = Config { width: 800, height: 600 };
        assert_eq!(width, 800);
        assert_eq!(height, 600);
        
        println!("✅ Irrefutable patterns work in let bindings");
    }
    
    #[test]
    fn test_refutable_patterns() {
        let numbers = vec![1, 2, 3, 4, 5];
        
        // if let with refutable pattern
        if let Some(first) = numbers.first() {
            assert_eq!(*first, 1);
        } else {
            panic!("Should have found first element");
        }
        
        // while let with refutable pattern
        let mut stack = vec![1, 2, 3];
        let mut sum = 0;
        while let Some(val) = stack.pop() {
            sum += val;
        }
        assert_eq!(sum, 6);
        
        // match with refutable patterns
        let result = match numbers.get(10) {
            Some(n) => format!("Found: {}", n),
            None => "Not found".to_string(),
        };
        assert_eq!(result, "Not found");
        
        println!("✅ Refutable patterns work in conditional contexts");
    }
}

/// Exercise 3: Destructuring Mastery
///
/// **Objective**: Combine multiple destructuring techniques
/// **Mission Connection**: Applicable to [[mission-8]] graph traversal
#[cfg(test)]
mod exercise_3_destructuring {
    use super::*;
    
    #[test]
    fn test_nested_struct_destructuring() {
        #[derive(Debug)]
        struct Person {
            name: String,
            location: Point,
        }
        
        let person = Person {
            name: "Alice".to_string(),
            location: Point::new(10, 20),
        };
        
        // Nested destructuring
        let Person {
            name,
            location: Point { x, y },
        } = person;
        
        assert_eq!(name, "Alice");
        assert_eq!(x, 10);
        assert_eq!(y, 20);
        
        println!("✅ Nested struct destructuring works");
    }
    
    #[test]
    fn test_complex_enum_destructuring() {
        // Complex nested enum
        let event = Event::ChangeColor(Color::Rgb(255, 128, 0));
        
        let result = match event {
            Event::ChangeColor(Color::Rgb(r, g, b)) => {
                format!("RGB: ({}, {}, {})", r, g, b)
            }
            Event::ChangeColor(Color::Hsv(h, s, v)) => {
                format!("HSV: ({}, {}, {})", h, s, v)
            }
            Event::Move(Point { x, y }) => {
                format!("Move: ({}, {})", x, y)
            }
            Event::Quit => "Quit".to_string(),
        };
        
        assert_eq!(result, "RGB: (255, 128, 0)");
        println!("✅ Complex enum destructuring works");
    }
}

/// Exercise 4: Match Guards
///
/// **Objective**: Use match guards for conditional matching
/// **Skills**: Combining patterns with boolean conditions
#[cfg(test)]
mod exercise_4_match_guards {
    use super::*;
    
    #[test]
    fn test_conditional_matching() {
        fn categorize_point(p: Point) -> &'static str {
            match p {
                Point { x: 0, y: 0 } => "origin",
                Point { x, y } if x == y => "diagonal",
                Point { x, .. } if x > 0 => "right half",
                Point { x, .. } if x < 0 => "left half",
                _ => "other",
            }
        }
        
        assert_eq!(categorize_point(Point::origin()), "origin");
        assert_eq!(categorize_point(Point::new(5, 5)), "diagonal");
        assert_eq!(categorize_point(Point::new(10, 3)), "right half");
        assert_eq!(categorize_point(Point::new(-5, 3)), "left half");
        
        println!("✅ Match guards work correctly");
    }
    
    #[test]
    fn test_range_with_guards() {
        fn classify_score(score: i32) -> &'static str {
            match score {
                s if s >= 90 => "A",
                s if s >= 80 => "B",
                s if s >= 70 => "C",
                s if s >= 60 => "D",
                _ => "F",
            }
        }
        
        assert_eq!(classify_score(95), "A");
        assert_eq!(classify_score(85), "B");
        assert_eq!(classify_score(75), "C");
        assert_eq!(classify_score(65), "D");
        assert_eq!(classify_score(55), "F");
        
        println!("✅ Range matching with guards works");
    }
}

/// Exercise 5: @ Bindings
///
/// **Objective**: Use @ to bind and test simultaneously
/// **Integration**: Advanced pattern matching for complex data
#[cfg(test)]
mod exercise_5_at_bindings {
    #[test]
    fn test_at_binding_ranges() {
        enum Priority {
            Low,
            Medium,
            High { id: i32 },
        }
        
        fn process_priority(priority: Priority) -> String {
            match priority {
                Priority::High { id: id_var @ 1..=10 } => {
                    format!("Critical high priority: {}", id_var)
                }
                Priority::High { id: id_var @ 11..=100 } => {
                    format!("High priority: {}", id_var)
                }
                Priority::High { id } => {
                    format!("Standard high priority: {}", id)
                }
                Priority::Medium => "Medium priority".to_string(),
                Priority::Low => "Low priority".to_string(),
            }
        }
        
        assert_eq!(
            process_priority(Priority::High { id: 5 }),
            "Critical high priority: 5"
        );
        assert_eq!(
            process_priority(Priority::High { id: 50 }),
            "High priority: 50"
        );
        assert_eq!(
            process_priority(Priority::High { id: 500 }),
            "Standard high priority: 500"
        );
        
        println!("✅ @ bindings work with ranges");
    }
}

/// Exercise 6: Ignoring Values
///
/// **Objective**: Use _ and .. to ignore unneeded values
/// **Skills**: Selective destructuring
#[cfg(test)]
mod exercise_6_ignoring_values {
    use super::*;
    
    #[test]
    fn test_ignore_with_underscore() {
        // Ignore function parameter
        fn process_pair(_x: i32, y: i32) -> i32 {
            y * 2
        }
        
        assert_eq!(process_pair(10, 5), 10);
        
        // Ignore tuple elements
        let (x, _, z) = (1, 2, 3);
        assert_eq!(x, 1);
        assert_eq!(z, 3);
        
        println!("✅ Ignoring with _ works");
    }
    
    #[test]
    fn test_ignore_with_rest() {
        // Ignore remaining struct fields
        let point = Point::new(10, 20);
        let Point { x, .. } = point;
        assert_eq!(x, 10);
        
        // Ignore middle elements in slice
        let numbers = [1, 2, 3, 4, 5];
        if let [first, .., last] = numbers {
            assert_eq!(first, 1);
            assert_eq!(last, 5);
        }
        
        println!("✅ Ignoring with .. works");
    }
}

/// Exercise 7: Integration Challenge
///
/// **Objective**: Combine multiple pattern techniques in realistic scenario
/// **Mission Connection**: Directly applicable to [[mission-8]] graph traversal
#[cfg(test)]
mod exercise_7_integration_challenge {
    use super::*;
    
    #[derive(Debug)]
    struct SearchResult {
        found: bool,
        path: Vec<Point>,
        cost: Option<i32>,
    }
    
    #[test]
    fn test_complex_result_processing() {
        fn analyze_result(result: SearchResult) -> String {
            match result {
                // Pattern 1: Success with specific cost range
                SearchResult {
                    found: true,
                    path,
                    cost: Some(c @ 0..=10),
                } if path.len() <= 5 => {
                    format!("Optimal path (cost {}): {} steps", c, path.len())
                }
                
                // Pattern 2: Success with higher cost
                SearchResult {
                    found: true,
                    path,
                    cost: Some(cost),
                } => {
                    format!("Path found (cost {}): {} steps", cost, path.len())
                }
                
                // Pattern 3: Success without cost
                SearchResult {
                    found: true,
                    path,
                    ..
                } => {
                    format!("Path found: {} steps", path.len())
                }
                
                // Pattern 4: Failure
                SearchResult { found: false, .. } => {
                    "No path found".to_string()
                }
            }
        }
        
        // Test optimal path
        let optimal = SearchResult {
            found: true,
            path: vec![Point::new(0, 0), Point::new(1, 1)],
            cost: Some(5),
        };
        assert!(analyze_result(optimal).contains("Optimal"));
        
        // Test expensive path
        let expensive = SearchResult {
            found: true,
            path: vec![Point::new(0, 0); 10],
            cost: Some(100),
        };
        assert!(analyze_result(expensive).contains("Path found"));
        
        // Test failure
        let failed = SearchResult {
            found: false,
            path: vec![],
            cost: None,
        };
        assert_eq!(analyze_result(failed), "No path found");
        
        println!("✅ Complex integration pattern matching works");
        println!("🎯 Ready for [[mission-8]] graph traversal patterns");
    }
}
