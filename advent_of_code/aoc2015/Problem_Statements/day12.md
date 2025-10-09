--- Day 12: JSAbacusFramework.io ---
Santa's Accounting-Elves need help balancing the books after a recent order. Unfortunately, their accounting software uses a peculiar storage format. That's where you come in.

They have a JSON document which contains a variety of things: arrays ([1,2,3]), objects ({"a":1, "b":2}), numbers, and strings. Your first job is to simply find all of the numbers throughout the document and add them together.

For example:

[1,2,3] and {"a":2,"b":4} both have a sum of 6.
[[[3]]] and {"a":{"b":4},"c":-1} both have a sum of 3.
{"a":[-1,1]} and [-1,{"a":1}] both have a sum of 0.
[] and {} both have a sum of 0.
You will not encounter any strings containing numbers.

What is the sum of all numbers in the document?

Your puzzle answer was 191164.

--- Part Two ---
Uh oh - the Accounting-Elves have realized that they double-counted everything red.

Ignore any object (and all of its children) which has any property with the value "red". Do this only for objects ({...}), not arrays ([...]).

[1,2,3] still has a sum of 6.
[1,{"c":"red","b":2},3] now has a sum of 4, because the middle object is ignored.
{"d":"red","e":[1,2,3,4],"f":5} now has a sum of 0, because the entire structure is ignored.
[1,"red",5] has a sum of 6, because "red" in an array has no effect.
Your puzzle answer was 87842.

Both parts of this puzzle are complete! They provide two gold stars: **

At this point, you should return to your Advent calendar and try another puzzle.

If you still want to see it, you can get your puzzle input.

You can also [Share] this puzzle.

**JSON parsing and numeric summation with conditional filtering**

---

## 📋 Problem Statement

Santa's Accounting-Elves need help balancing the books after a recent order. Their accounting software uses JSON format, and you need to find and sum all the numbers in the document.

### Part 1: Sum All Numbers

Find all numbers throughout the JSON document and add them together.

**Examples:**
- `[1,2,3]` → **6**
- `{"a":2,"b":4}` → **6**
- `[[[3]]]` → **3**
- `{"a":{"b":4},"c":-1}` → **3** (4 + -1)
- `{"a":[-1,1]}` → **0** (-1 + 1)
- `[-1,{"a":1}]` → **0** (-1 + 1)
- `[]` and `{}` → **0**

**Note:** You will not encounter any strings containing numbers.

### Part 2: Ignore "Red" Objects

Same as Part 1, BUT ignore any object (and all its children) that has any property with the value `"red"`.

**Important:** Only objects trigger this rule, not arrays.

**Examples:**
- `[1,2,3]` → **6** (no objects)
- `[1,{"c":"red","b":2},3]` → **4** (1 + 3, object with "red" ignored)
- `{"d":"red","e":[1,2,3,4],"f":5}` → **0** (entire object ignored)
- `[1,"red",5]` → **6** (string in array doesn't trigger filtering)

---

## 🧩 Algorithm: Recursive JSON Traversal

### Core Approach

Use recursive tree traversal with pattern matching on JSON value types:

```rust
fn sum_numbers(value: &Value, filter_red: bool) -> i64 {
    match value {
        Value::Number(n) => n.as_i64().unwrap_or(0),
        Value::Array(arr) => arr.iter().map(|v| sum_numbers(v, filter_red)).sum(),
        Value::Object(obj) => {
            if filter_red && has_red_value(obj) {
                return 0;  // Ignore this object and all children
            }
            obj.values().map(|v| sum_numbers(v, filter_red)).sum()
        }
        _ => 0,  // Strings, bools, null contribute 0
    }
}
```

### Algorithm Steps

1. **Base Case (Number)**: Return the numeric value
2. **Recursive Case (Array)**: Sum all elements recursively
3. **Recursive Case (Object)**:
   - Check if any value is the string `"red"` (Part 2 only)
   - If "red" found, return 0 (ignore entire object)
   - Otherwise, sum all values recursively
4. **Other Types**: Return 0 for strings, booleans, null

---

## 🔍 "Red" Filtering Logic (Part 2)

### Rules

- **Objects**: Check if any property has string value `"red"`
- **Arrays**: Never trigger filtering (even if they contain "red")
- **Nested Structures**: If object contains "red", ignore it and all nested values

### Examples Breakdown

#### Example 1: Object with "red"
```json
[1, {"c": "red", "b": 2}, 3]
```
- Array: `[1, {...}, 3]`
  - `1` → sum: 1
  - Object `{"c": "red", "b": 2}` → has "red" → **ignored** → sum: 0
  - `3` → sum: 3
- **Total: 4**

#### Example 2: Top-level "red" object
```json
{"d": "red", "e": [1,2,3,4], "f": 5}
```
- Object has property `"d": "red"` → **entire object ignored**
- **Total: 0**

#### Example 3: "red" in array (not filtered)
```json
[1, "red", 5]
```
- Array contains string "red", but arrays don't trigger filtering
- `1` → 1, `"red"` → 0, `5` → 5
- **Total: 6**

#### Example 4: Nested "red" object
```json
{"a": 1, "b": {"c": "red", "d": 2}, "e": 3}
```
- Object `{"a": 1, "b": {...}, "e": 3}`
  - `"a": 1` → sum: 1
  - `"b": {"c": "red", "d": 2}` → nested object has "red" → **ignored** → sum: 0
  - `"e": 3` → sum: 3
- **Total: 4**

---

## 🛠️ Implementation Details

### Using `serde_json`

```rust
use serde_json::Value;

pub fn solve_part1(input: &str) -> Result<String> {
    let json: Value = serde_json::from_str(input.trim())?;
    let sum = sum_numbers(&json, false);
    Ok(sum.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let json: Value = serde_json::from_str(input.trim())?;
    let sum = sum_numbers(&json, true);
    Ok(sum.to_string())
}
```

### Checking for "Red" Value

```rust
fn has_red_value(obj: &serde_json::Map<String, Value>) -> bool {
    obj.values().any(|v| {
        if let Value::String(s) = v {
            s == "red"
        } else {
            false
        }
    })
}
```

**Key Points:**
- Only check direct property values
- `"red"` nested in arrays/objects doesn't count
- Must be exact string match: `"red"`, not `"Red"` or `"RED"`

---

## ⚡ Alternative Approach: Regex (Part 1 Only)

For Part 1, you could use regex to extract all numbers:

```rust
use regex::Regex;

fn sum_with_regex(json: &str) -> i64 {
    let re = Regex::new(r"-?\d+").unwrap();
    re.find_iter(json)
        .filter_map(|m| m.as_str().parse::<i64>().ok())
        .sum()
}
```

### Regex vs JSON Parsing

| Aspect | Regex | JSON Parsing |
|--------|-------|--------------|
| **Speed** | Faster (no parsing overhead) | Slower (tree building) |
| **Simplicity** | Very simple for Part 1 | More complex setup |
| **Part 2** | ❌ Cannot handle filtering | ✅ Supports filtering |
| **Use Case** | Part 1 only | Both parts |
| **Dependencies** | `regex` crate | `serde_json` crate |

**Recommendation:** Use JSON parsing for both parts (consistent approach).

---

## 🧪 Test Cases

### Part 1 Tests
```rust
assert_eq!(sum_numbers(&json!([1, 2, 3]), false), 6);
assert_eq!(sum_numbers(&json!({"a": 2, "b": 4}), false), 6);
assert_eq!(sum_numbers(&json!([[[3]]]), false), 3);
assert_eq!(sum_numbers(&json!({"a": {"b": 4}, "c": -1}), false), 3);
assert_eq!(sum_numbers(&json!({"a": [-1, 1]}), false), 0);
assert_eq!(sum_numbers(&json!([-1, {"a": 1}]), false), 0);
```

### Part 2 Tests
```rust
assert_eq!(sum_numbers(&json!([1, 2, 3]), true), 6);
assert_eq!(sum_numbers(&json!([1, {"c": "red", "b": 2}, 3]), true), 4);
assert_eq!(sum_numbers(&json!({"d": "red", "e": [1, 2, 3, 4], "f": 5}), true), 0);
assert_eq!(sum_numbers(&json!([1, "red", 5]), true), 6);
```

---

## 🎯 Key Concepts

### Rust Patterns
- **`serde_json::Value` enum**: Represents JSON types
- **Pattern matching**: `match` on `Value` variants
- **Recursive functions**: Natural fit for tree structures
- **Iterator methods**: `.iter().map().sum()` for aggregation
- **Error handling**: `as_i64()` with `unwrap_or(0)`

### Algorithm Patterns
- **Tree traversal**: Post-order recursive traversal
- **Conditional filtering**: Prune tree branches based on content
- **Accumulation**: Sum values while traversing
- **Type-based dispatch**: Different handling for different JSON types

### Data Structures
- **JSON as tree**: Nested arrays and objects form a tree
- **Recursive structure**: Arrays/objects contain other values
- **Type safety**: Rust's enum ensures all cases handled

---

## 📊 Complexity Analysis

- **Time Complexity**: O(n) where n = number of JSON values
  - Visit each value exactly once
- **Space Complexity**: O(d) where d = maximum nesting depth
  - Recursion stack depth proportional to nesting
- **Parsing Complexity**: O(n) to build JSON tree from string

---

## 🚀 Running the Solution

```bash
# Run with example input
cargo run -- --day 12

# Run tests
cargo test day12

# Run with custom input
echo '[1,2,3]' | cargo run -- --day 12
```

---

## 💡 Learning Takeaways

1. **JSON Parsing**: Using `serde_json` for structured data
2. **Recursive Traversal**: Natural pattern for tree structures
3. **Pattern Matching**: Rust's enum matching for type safety
4. **Conditional Logic**: Filtering tree branches based on content
5. **Algorithm Selection**: When to use regex vs structured parsing
6. **External Crates**: Integrating `serde_json` into Rust projects

---

*Tags: #aoc2015 #day12 #json #parsing #recursion #tree-traversal #serde-json #pattern-matching*
*Difficulty: ⭐⭐ Medium*
*Concepts: JSON, Recursion, Tree Traversal, Conditional Filtering*
