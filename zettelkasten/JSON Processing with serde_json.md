# 🔄 JSON Processing with serde_json in Rust

*Comprehensive guide to JSON handling in Rust using the serde_json crate*

---

## 🎯 **Overview**

JSON (JavaScript Object Notation) is a lightweight, text-based data interchange format. In Rust, the `serde_json` crate provides powerful, type-safe JSON processing capabilities with excellent performance and ergonomics.

**Key Benefits:**

- **Zero-copy parsing** for string slices where possible
- **Type safety** with Rust's type system integration
- **Flexible APIs** supporting both dynamic and strongly-typed approaches
- **Excellent error handling** with detailed error messages

---

## 📦 **Core serde_json Types**

### **serde_json::Value - The Universal JSON Type**

The `Value` enum represents any JSON value:

```rust
use serde_json::Value;

pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(Map<String, Value>),
}
```

**Pattern Matching Example:**

```rust
use serde_json::Value;

fn process_json_value(value: &Value) -> i64 {
    match value {
        Value::Number(n) => n.as_i64().unwrap_or(0),
        Value::Array(arr) => arr.iter().map(process_json_value).sum(),
        Value::Object(obj) => obj.values().map(process_json_value).sum(),
        _ => 0, // String, Bool, Null contribute nothing
    }
}
```

### **serde_json::Map - JSON Object Representation**

Represents JSON objects as `Map<String, Value>`:

```rust
use serde_json::{Map, Value};

fn has_property_value(obj: &Map<String, Value>, key: &str, expected: &str) -> bool {
    obj.get(key)
        .and_then(|v| v.as_str())
        .map(|s| s == expected)
        .unwrap_or(false)
}
```

---

## 🛠️ **Essential Operations**

### **1. Parsing JSON from String**

```rust
use serde_json::{Value, Result};

// Parse any JSON into dynamic Value
fn parse_dynamic_json(json_str: &str) -> Result<Value> {
    serde_json::from_str(json_str)
}

// Usage examples
let simple_array = r#"[1, 2, 3]"#;
let json_value = serde_json::from_str::<Value>(simple_array)?;

let complex_object = r#"{"users": [{"name": "Alice", "age": 30}]}"#;
let parsed = serde_json::from_str::<Value>(complex_object)?;
```

### **2. Creating JSON Values Programmatically**

```rust
use serde_json::{json, Value};

// json! macro for convenient JSON construction
let data = json!({
    "name": "John Doe",
    "age": 35,
    "hobbies": ["reading", "coding", "hiking"],
    "address": {
        "street": "123 Main St",
        "city": "Springfield"
    }
});

// Manual construction
let mut user_data = serde_json::Map::new();
user_data.insert("id".to_string(), Value::Number(123.into()));
user_data.insert("active".to_string(), Value::Bool(true));
let user_obj = Value::Object(user_data);
```

### **3. Extracting Values Safely**

```rust
use serde_json::Value;

fn extract_user_info(json: &Value) -> Option<(String, u32)> {
    let obj = json.as_object()?;
    let name = obj.get("name")?.as_str()?.to_string();
    let age = obj.get("age")?.as_u64()? as u32;
    Some((name, age))
}

// Safe number extraction
fn get_number_value(value: &Value) -> Option<i64> {
    value.as_i64()
        .or_else(|| value.as_u64().map(|n| n as i64))
        .or_else(|| value.as_f64().map(|n| n as i64))
}
```

---

## 🎄 **Real-World Example: AoC 2015 Day 12**

### **Problem Context**

**AoC 2015 Day 12** demonstrates advanced JSON processing:

- **Part 1**: Sum all numbers in JSON document
- **Part 2**: Same, but ignore objects containing "red" string values

### **Core Implementation**

```rust
use serde_json::Value;

/// Recursively sum all numbers in JSON, with optional "red" filtering
/// 
/// # Examples from AoC Day 12:
/// - `[1,2,3]` → 6
/// - `{"a":2,"b":4}` → 6  
/// - `[1,{"c":"red","b":2},3]` → 4 (with filtering, object ignored)
/// - `{"d":"red","e":[1,2,3,4],"f":5}` → 0 (with filtering, entire object ignored)
pub fn sum_numbers(value: &Value, filter_red: bool) -> i64 {
    match value {
        // Base case: found a number
        Value::Number(n) => n.as_i64().unwrap_or(0),
        
        // Recursive case: array - sum all elements
        Value::Array(arr) => {
            arr.iter()
               .map(|v| sum_numbers(v, filter_red))
               .sum()
        }
        
        // Complex case: object - check for "red" filter
        Value::Object(obj) => {
            // Part 2: Skip objects containing "red" string values
            if filter_red && has_red_value(obj) {
                return 0;
            }
            
            // Sum all values in the object
            obj.values()
               .map(|v| sum_numbers(v, filter_red))
               .sum()
        }
        
        // All other types (String, Bool, Null) contribute 0
        _ => 0,
    }
}

/// Check if object has any direct property with string value "red"
fn has_red_value(obj: &serde_json::Map<String, Value>) -> bool {
    obj.values().any(|v| {
        matches!(v, Value::String(s) if s == "red")
    })
}
```

### **Usage Examples**

```rust
use serde_json::json;

// Part 1: Simple summation
let json1 = json!([1, 2, 3]);
assert_eq!(sum_numbers(&json1, false), 6);

let json2 = json!({"a": {"b": 4}, "c": -1});
assert_eq!(sum_numbers(&json2, false), 3); // 4 + (-1)

// Part 2: Red filtering
let json3 = json!([1, {"c": "red", "b": 2}, 3]);
assert_eq!(sum_numbers(&json3, true), 4);  // 1 + 3, middle object ignored

let json4 = json!({"d": "red", "e": [1, 2, 3, 4], "f": 5});
assert_eq!(sum_numbers(&json4, true), 0);  // Entire object ignored

// Red in array doesn't trigger filtering (only direct object properties)
let json5 = json!([1, "red", 5]);
assert_eq!(sum_numbers(&json5, true), 6);  // 1 + 5, string in array ignored
```

---

## ⚡ **Performance Patterns**

### **1. Efficient Number Extraction**

```rust
use serde_json::Value;

// Handle different number types efficiently
fn extract_integer(value: &Value) -> Option<i64> {
    match value {
        Value::Number(n) => {
            // Try i64 first (most common)
            n.as_i64()
                .or_else(|| n.as_u64().map(|u| u as i64))
                .or_else(|| n.as_f64().map(|f| f as i64))
        }
        _ => None,
    }
}
```

### **2. Memory-Efficient Traversal**

```rust
// Use references to avoid cloning large structures
fn traverse_efficiently(value: &Value, visitor: &mut impl FnMut(&Value)) {
    visitor(value);
    
    match value {
        Value::Array(arr) => {
            for item in arr {
                traverse_efficiently(item, visitor);
            }
        }
        Value::Object(obj) => {
            for (_, val) in obj {
                traverse_efficiently(val, visitor);
            }
        }
        _ => {} // Leaf nodes
    }
}
```

### **3. Conditional Processing**

```rust
// Early termination for performance
fn find_first_number(value: &Value) -> Option<i64> {
    match value {
        Value::Number(n) => n.as_i64(),
        Value::Array(arr) => {
            for item in arr {
                if let Some(num) = find_first_number(item) {
                    return Some(num);
                }
            }
            None
        }
        Value::Object(obj) => {
            for (_, val) in obj {
                if let Some(num) = find_first_number(val) {
                    return Some(num);
                }
            }
            None
        }
        _ => None,
    }
}
```

---

## 🎯 **Common Patterns & Idioms**

### **1. Safe Value Extraction Chain**

```rust
use serde_json::Value;

fn get_nested_string(json: &Value, keys: &[&str]) -> Option<String> {
    let mut current = json;
    
    // Navigate through object hierarchy
    for &key in &keys[..keys.len()-1] {
        current = current.as_object()?.get(key)?;
    }
    
    // Extract final string value
    current.as_object()?
           .get(keys.last()?)?
           .as_str()
           .map(|s| s.to_string())
}

// Usage: get_nested_string(&json, &["user", "profile", "name"])
```

### **2. Type-Safe Array Processing**

```rust
fn process_number_array(value: &Value) -> Vec<i64> {
    value.as_array()
         .map(|arr| {
             arr.iter()
                .filter_map(|v| v.as_i64())
                .collect()
         })
         .unwrap_or_default()
}
```

### **3. Object Key Filtering**

```rust
use std::collections::HashMap;

fn extract_string_properties(obj: &Value) -> HashMap<String, String> {
    obj.as_object()
       .map(|map| {
           map.iter()
              .filter_map(|(k, v)| {
                  v.as_str().map(|s| (k.clone(), s.to_string()))
              })
              .collect()
       })
       .unwrap_or_default()
}
```

---

## 🚨 **Error Handling Best Practices**

### **1. Comprehensive Error Types**

```rust
use serde_json::{Error as JsonError, Value};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JsonProcessingError {
    #[error("JSON parsing failed: {0}")]
    ParseError(#[from] JsonError),
    
    #[error("Missing required field: {field}")]
    MissingField { field: String },
    
    #[error("Invalid value type: expected {expected}, found {found}")]
    InvalidType { expected: String, found: String },
    
    #[error("Value out of range: {value}")]
    OutOfRange { value: String },
}
```

### **2. Safe Value Conversion**

```rust
fn safe_extract_user(json: &Value) -> Result<User, JsonProcessingError> {
    let obj = json.as_object()
        .ok_or_else(|| JsonProcessingError::InvalidType {
            expected: "object".to_string(),
            found: json_type_name(json).to_string(),
        })?;
    
    let name = obj.get("name")
        .ok_or_else(|| JsonProcessingError::MissingField { 
            field: "name".to_string() 
        })?
        .as_str()
        .ok_or_else(|| JsonProcessingError::InvalidType {
            expected: "string".to_string(),
            found: json_type_name(obj.get("name").unwrap()).to_string(),
        })?;
    
    let age = obj.get("age")
        .ok_or_else(|| JsonProcessingError::MissingField { 
            field: "age".to_string() 
        })?
        .as_u64()
        .ok_or_else(|| JsonProcessingError::InvalidType {
            expected: "number".to_string(),
            found: json_type_name(obj.get("age").unwrap()).to_string(),
        })?;
    
    if age > 150 {
        return Err(JsonProcessingError::OutOfRange {
            value: age.to_string(),
        });
    }
    
    Ok(User {
        name: name.to_string(),
        age: age as u32,
    })
}

fn json_type_name(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}
```

---

## 🔧 **Advanced Techniques**

### **1. Custom JSON Traversal**

```rust
use serde_json::Value;

pub struct JsonVisitor<'a> {
    pub path: Vec<String>,
    pub callback: Box<dyn FnMut(&Vec<String>, &Value) + 'a>,
}

impl<'a> JsonVisitor<'a> {
    pub fn new<F>(callback: F) -> Self 
    where F: FnMut(&Vec<String>, &Value) + 'a 
    {
        JsonVisitor {
            path: Vec::new(),
            callback: Box::new(callback),
        }
    }
    
    pub fn visit(&mut self, value: &Value) {
        (self.callback)(&self.path, value);
        
        match value {
            Value::Array(arr) => {
                for (i, item) in arr.iter().enumerate() {
                    self.path.push(i.to_string());
                    self.visit(item);
                    self.path.pop();
                }
            }
            Value::Object(obj) => {
                for (key, val) in obj {
                    self.path.push(key.clone());
                    self.visit(val);
                    self.path.pop();
                }
            }
            _ => {} // Leaf values
        }
    }
}

// Usage: Find all numbers and their paths
let mut visitor = JsonVisitor::new(|path, value| {
    if let Value::Number(n) = value {
        println!("Found number {} at path: {}", n, path.join("."));
    }
});
visitor.visit(&json_data);
```

### **2. Streaming-Style Processing**

```rust
// For very large JSON documents, process incrementally
use serde_json::{Deserializer, Value};
use std::io::Read;

fn process_json_stream<R: Read>(reader: R) -> Result<i64, Box<dyn std::error::Error>> {
    let deserializer = Deserializer::from_reader(reader);
    let mut sum = 0;
    
    for value in deserializer.into_iter::<Value>() {
        let json_value = value?;
        sum += sum_numbers(&json_value, false);
    }
    
    Ok(sum)
}
```

---

## 🧪 **Testing Patterns**

### **1. Comprehensive Test Cases**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_number_extraction() {
        assert_eq!(sum_numbers(&json!(42), false), 42);
        assert_eq!(sum_numbers(&json!(-15), false), -15);
        assert_eq!(sum_numbers(&json!(0), false), 0);
    }

    #[test]
    fn test_array_processing() {
        assert_eq!(sum_numbers(&json!([1, 2, 3]), false), 6);
        assert_eq!(sum_numbers(&json!([]), false), 0);
        assert_eq!(sum_numbers(&json!([1, "text", 2]), false), 3);
    }

    #[test]
    fn test_object_processing() {
        assert_eq!(sum_numbers(&json!({"a": 1, "b": 2}), false), 3);
        assert_eq!(sum_numbers(&json!({}), false), 0);
    }

    #[test]
    fn test_red_filtering() {
        // Object with red should be ignored
        assert_eq!(sum_numbers(&json!({"x": "red", "y": 5}), true), 0);
        
        // Red in array should not trigger filtering
        assert_eq!(sum_numbers(&json!([1, "red", 2]), true), 3);
        
        // Nested red object should be ignored
        let nested = json!({"a": 1, "b": {"c": "red", "d": 10}, "e": 2});
        assert_eq!(sum_numbers(&nested, true), 3); // 1 + 2
    }

    #[test]
    fn test_complex_nesting() {
        let complex = json!({
            "level1": {
                "level2": {
                    "numbers": [1, 2, 3],
                    "nested": {"value": 10}
                }
            },
            "other": 5
        });
        assert_eq!(sum_numbers(&complex, false), 21); // 1+2+3+10+5
    }
}
```

---

## 📚 **Integration with Other Systems**

### **1. HTTP API Integration**

```rust
use serde_json::Value;
use reqwest;

async fn fetch_and_process_json(url: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let json: Value = response.json().await?;
    Ok(sum_numbers(&json, false))
}
```

### **2. File Processing**

```rust
use std::fs;
use serde_json::Value;

fn process_json_file(path: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let json: Value = serde_json::from_str(&content)?;
    Ok(sum_numbers(&json, false))
}
```

---

## 🎯 **Key Takeaways**

1. **serde_json::Value** provides flexible, type-safe JSON processing
2. **Pattern matching** is the idiomatic way to handle different JSON types
3. **Recursive traversal** enables complex document processing
4. **Error handling** should be comprehensive and user-friendly
5. **Performance** can be optimized through careful reference usage and early termination
6. **Real-world problems** like AoC Day 12 demonstrate practical JSON processing patterns

---

## 🔗 **Related Concepts**

- [[Error Handling Deep Dive]] - Comprehensive error handling patterns
- [[../advent_of_code/aoc2015/src/solver/day12.rs]] - Complete AoC Day 12 implementation
- [[AoC Patterns MOC]] - Competitive programming techniques
- [[rust-concepts-MOC]] - Core language features

---

*Tags: #json #serde #parsing #data-processing #aoc #day12 #pattern-matching #recursion #error-handling*
*Links: [[zettel-index]] | [[AoC Patterns MOC]] | [[rust-concepts-MOC]] | [[Error Handling Deep Dive]]*
