use anyhow::Result;
use serde_json::Value;

/// AoC 2015 Day 12: JSAbacusFramework.io
/// JSON parsing and numeric summation
///
/// ## Problem Description
/// Santa's Accounting-Elves need help balancing books from a JSON document.
/// The document contains:
/// - Arrays: \[1,2,3\]
/// - Objects: {"a":1, "b":2}
/// - Numbers: integers (positive and negative)
/// - Strings: text values
///
/// ## Part 1
/// Find all numbers in the JSON document and sum them.
///
/// ### Examples:
/// - \[1,2,3\] → 6
/// - {"a":2,"b":4} → 6
/// - \[\[\[3\]\]\] → 3
/// - {"a":{"b":4},"c":-1} → 3
/// - {"a":[-1,1]} → 0
/// - [-1,{"a":1}] → 0
/// - [] and {} → 0
///
/// ## Part 2
/// Same as Part 1, BUT ignore any object (and all its children) that has
/// any property with the value "red".
///
/// ### Examples with "red" filtering:
/// - \[1,2,3\] → 6 (no objects)
/// - \[1,{"c":"red","b":2},3\] → 4 (1+3, object ignored)
/// - {"d":"red","e":\[1,2,3,4\],"f":5} → 0 (entire object ignored)
/// - \[1,"red",5\] → 6 (string in array doesn't trigger filtering)
pub fn solve_part1(input: &str) -> Result<String> {
    println!("🎄 Day 12 Part 1: Sum all numbers in JSON");

    let json: Value = serde_json::from_str(input.trim())?;
    let sum = sum_numbers(&json, false);

    Ok(sum.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    println!("🎄 Day 12 Part 2: Sum all numbers, ignore 'red' objects");

    let json: Value = serde_json::from_str(input.trim())?;
    let sum = sum_numbers(&json, true);

    Ok(sum.to_string())
}

// ============================================================================
// Core Algorithm: Recursive JSON Traversal
// ============================================================================

/// Recursively sum all numbers in a JSON value
///
/// ## Algorithm
/// 1. Match on JSON value type
/// 2. Number: return its i64 value
/// 3. Array: recursively sum all elements
/// 4. Object: check for "red" filter, then sum all values
/// 5. Other types (String, Bool, Null): return 0
///
/// ## Parameters
/// - `value`: The JSON value to process
/// - `filter_red`: If true, ignore objects containing "red" string
///
/// ## Examples
/// ```
/// # use serde_json::json;
/// # use aoc2015::solver::day12::sum_numbers;
///
/// let json = json!([1, 2, 3]);
/// assert_eq!(sum_numbers(&json, false), 6);
///
/// let json = json!({"a": 2, "b": 4});
/// assert_eq!(sum_numbers(&json, false), 6);
///
/// let json = json!([1, {"c": "red", "b": 2}, 3]);
/// assert_eq!(sum_numbers(&json, true), 4);  // Filters out red object
/// ```
pub fn sum_numbers(value: &Value, filter_red: bool) -> i64 {
    match value {
        // Base case: number found
        Value::Number(n) => n.as_i64().unwrap_or(0),

        // Recursive case: array - sum all elements
        Value::Array(arr) => arr.iter().map(|v| sum_numbers(v, filter_red)).sum(),

        // Recursive case: object - check for "red" filter
        Value::Object(obj) => {
            // Part 2: If any value is the string "red", ignore entire object
            if filter_red && has_red_value(obj) {
                return 0;
            }

            // Sum all values in the object
            obj.values().map(|v| sum_numbers(v, filter_red)).sum()
        }

        // All other types contribute 0
        _ => 0,
    }
}

/// Check if an object has any property with the string value "red"
///
/// ## Examples
/// ```
/// # use serde_json::json;
/// # use aoc2015::solver::day12::has_red_value;
///
/// let obj1 = json!({"a": 1, "b": "red"});
/// assert!(has_red_value(obj1.as_object().unwrap()));
///
/// let obj2 = json!({"a": 1, "b": "blue"});
/// assert!(!has_red_value(obj2.as_object().unwrap()));
///
/// let obj3 = json!({"a": [1, "red", 3]});
/// assert!(!has_red_value(obj3.as_object().unwrap()));  // "red" in array, not direct value
/// ```
pub fn has_red_value(obj: &serde_json::Map<String, Value>) -> bool {
    obj.values().any(|v| {
        if let Value::String(s) = v {
            s == "red"
        } else {
            false
        }
    })
}

// ============================================================================
// Alternative Approach: Regex-based (Part 1 Only)
// ============================================================================

// NOTE: This is an alternative approach for Part 1 only.
// To use this, you would need to add the regex dependency and uncomment the code below.

// use regex::Regex;

// Alternative approach using regex to extract all numbers
//
// ## Pros
// - Simple and fast for Part 1
// - No JSON parsing overhead
//
// ## Cons
// - Cannot handle Part 2 filtering (no structure awareness)
// - Only works for simple numeric extraction
//
// ## Examples
// ```ignore
// use aoc2015::solver::day12::sum_with_regex;
//
// assert_eq!(sum_with_regex("[1,2,3]"), 6);
// assert_eq!(sum_with_regex(r#"{"a":2,"b":4}"#), 6);
// assert_eq!(sum_with_regex("[[[3]]]"), 3);
// ```
/*
pub fn sum_with_regex(json: &str) -> i64 {
    let re = Regex::new(r"-?\d+").unwrap();
    re.find_iter(json)
        .filter_map(|m| m.as_str().parse::<i64>().ok())
        .sum()
}
*/
// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // ------------------------------------------------------------------------
    // Part 1 Tests: Basic Number Summation
    // ------------------------------------------------------------------------

    #[test]
    fn test_simple_array() {
        let json = json!([1, 2, 3]);
        assert_eq!(sum_numbers(&json, false), 6);
    }

    #[test]
    fn test_simple_object() {
        let json = json!({"a": 2, "b": 4});
        assert_eq!(sum_numbers(&json, false), 6);
    }

    #[test]
    fn test_deeply_nested_array() {
        let json = json!([[[3]]]);
        assert_eq!(sum_numbers(&json, false), 3);
    }

    #[test]
    fn test_nested_object() {
        let json = json!({"a": {"b": 4}, "c": -1});
        assert_eq!(sum_numbers(&json, false), 3);
    }

    #[test]
    fn test_array_in_object() {
        let json = json!({"a": [-1, 1]});
        assert_eq!(sum_numbers(&json, false), 0);
    }

    #[test]
    fn test_object_in_array() {
        let json = json!([-1, {"a": 1}]);
        assert_eq!(sum_numbers(&json, false), 0);
    }

    #[test]
    fn test_empty_array() {
        let json = json!([]);
        assert_eq!(sum_numbers(&json, false), 0);
    }

    #[test]
    fn test_empty_object() {
        let json = json!({});
        assert_eq!(sum_numbers(&json, false), 0);
    }

    #[test]
    fn test_negative_numbers() {
        let json = json!([-5, -10, 3]);
        assert_eq!(sum_numbers(&json, false), -12);
    }

    #[test]
    fn test_mixed_types() {
        let json = json!([1, "hello", 2, true, 3, null, 4]);
        assert_eq!(sum_numbers(&json, false), 10);
    }

    // ------------------------------------------------------------------------
    // Part 2 Tests: "Red" Object Filtering
    // ------------------------------------------------------------------------

    #[test]
    fn test_no_filtering_without_red() {
        let json = json!([1, 2, 3]);
        assert_eq!(sum_numbers(&json, true), 6);
    }

    #[test]
    fn test_filter_object_with_red() {
        let json = json!([1, {"c": "red", "b": 2}, 3]);
        assert_eq!(sum_numbers(&json, true), 4); // 1 + 3, object ignored
    }

    #[test]
    fn test_filter_top_level_red_object() {
        let json = json!({"d": "red", "e": [1, 2, 3, 4], "f": 5});
        assert_eq!(sum_numbers(&json, true), 0); // Entire object ignored
    }

    #[test]
    fn test_red_in_array_not_filtered() {
        let json = json!([1, "red", 5]);
        assert_eq!(sum_numbers(&json, true), 6); // String in array doesn't filter
    }

    #[test]
    fn test_nested_red_object() {
        let json = json!({"a": 1, "b": {"c": "red", "d": 2}, "e": 3});
        assert_eq!(sum_numbers(&json, true), 4); // 1 + 3, nested object ignored
    }

    #[test]
    fn test_multiple_red_objects() {
        let json = json!([
            1,
            {"a": "red", "b": 10},
            {"c": "red", "d": 20},
            2
        ]);
        assert_eq!(sum_numbers(&json, true), 3); // 1 + 2, both objects ignored
    }

    #[test]
    fn test_red_not_as_string() {
        let json = json!({"a": ["red"], "b": 5});
        assert_eq!(sum_numbers(&json, true), 5); // "red" in array, not direct value
    }

    #[test]
    fn test_complex_nesting_with_red() {
        let json = json!({
            "a": [1, 2, 3],
            "b": {"x": "red", "y": [4, 5, 6]},
            "c": {"z": 10}
        });
        assert_eq!(sum_numbers(&json, true), 16); // 1+2+3+10, b ignored
    }

    // ------------------------------------------------------------------------
    // Helper Function Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_has_red_value_true() {
        let value = json!({"a": 1, "b": "red"});
        let obj = value.as_object().unwrap();
        assert!(has_red_value(obj));
    }

    #[test]
    fn test_has_red_value_false() {
        let value = json!({"a": 1, "b": "blue"});
        let obj = value.as_object().unwrap();
        assert!(!has_red_value(obj));
    }

    #[test]
    fn test_has_red_value_in_nested_structure() {
        let value = json!({"a": [1, "red", 3]});
        let obj = value.as_object().unwrap();
        assert!(!has_red_value(obj)); // "red" in array, not direct property value
    }

    // ------------------------------------------------------------------------
    // Edge Cases
    // ------------------------------------------------------------------------

    #[test]
    fn test_large_numbers() {
        let json = json!([1000000, -1000000]);
        assert_eq!(sum_numbers(&json, false), 0);
    }

    #[test]
    fn test_deeply_nested_structure() {
        let json = json!({
            "a": {
                "b": {
                    "c": {
                        "d": {
                            "e": 42
                        }
                    }
                }
            }
        });
        assert_eq!(sum_numbers(&json, false), 42);
    }
}
