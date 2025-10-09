# Advent of Code 2015 - Problem Summary

This document provides a categorized overview of all Advent of Code 2015 problems, organized by day with problem types for both parts.

## Problem Categories

- **String Processing**: Character manipulation, parsing, pattern matching
- **Mathematical**: Arithmetic calculations, formulas, geometric problems
- **Simulation**: State tracking, following instructions step-by-step
- **Search/Traversal**: Finding positions, tracking states
- **Optimization**: Finding minimum/maximum values
- **Data Structures**: Working with arrays, lists, sets, maps
- **Pattern Matching**: Regular expressions, string validation, substring detection
- **Advanced Pattern Matching**: Complex pattern constraints, non-overlapping patterns
- **Brute Force**: Exhaustive search through solution space
- **Cryptographic**: Hash functions, encryption, cryptographic puzzles
- **Parsing**: Escape sequence parsing, character-level analysis
- **Encoding**: String encoding, character escaping

---

## Day-by-Day Summary

### [[day01.md|Day 1: Not Quite Lisp]]
**Title**: Not Quite Lisp  
**Part 1 Type**: String Processing + Mathematical  
**Part 1 Description**: Count opening/closing parentheses to determine final floor  
**Part 2 Type**: Search/Traversal + Simulation  
**Part 2 Description**: Find first position that causes Santa to enter basement (floor -1)  
**Key Concepts**: Character iteration, running sum, early termination condition

---

### [[day02.md|Day 2: I Was Told There Would Be No Math]]
**Title**: I Was Told There Would Be No Math  
**Part 1 Type**: Mathematical + String Processing  
**Part 1 Description**: Calculate surface area of boxes plus slack (smallest side area)  
**Part 2 Type**: Mathematical + Optimization  
**Part 2 Description**: Calculate ribbon length (smallest perimeter + volume for bow)  
**Key Concepts**: Geometric calculations, parsing dimensions, finding minimum values

---

### [[day03.md|Day 3: Perfectly Spherical Houses in a Vacuum]]
**Title**: Perfectly Spherical Houses in a Vacuum  
**Part 1 Type**: Simulation + Data Structures  
**Part 1 Description**: Track Santa's movement on 2D grid, count unique houses visited  
**Part 2 Type**: Simulation + String Processing + Data Structures  
**Part 2 Description**: Santa and Robo-Santa alternate moves (even/odd indices), count combined unique houses  
**Key Concepts**: 2D coordinate tracking, HashSet for uniqueness, character splitting by index, alternating processing

---

### [[day04.md|Day 4: The Ideal Stocking Stuffer]]
**Title**: The Ideal Stocking Stuffer  
**Part 1 Type**: Brute Force + Cryptographic  
**Part 1 Description**: Find lowest number that creates MD5 hash starting with 5 zeros  
**Part 2 Type**: Brute Force + Cryptographic  
**Part 2 Description**: Find lowest number that creates MD5 hash starting with 6 zeros  
**Key Concepts**: MD5 hashing, brute force search, cryptographic hardness, computational complexity, prefix matching

---

### [[day05.md|Day 5: Doesn't He Have Intern-Elves For This?]]
**Title**: Doesn't He Have Intern-Elves For This?  
**Part 1 Type**: String Processing + Pattern Matching  
**Part 1 Description**: Validate "nice" strings with 3+ vowels, consecutive letters, no forbidden substrings (ab/cd/pq/xy)  
**Part 2 Type**: String Processing + Advanced Pattern Matching  
**Part 2 Description**: New rules - non-overlapping letter pairs and letters with exactly one character between  
**Key Concepts**: Regular expressions, string validation, pattern matching, non-overlapping substring detection, character iteration with lookahead

---

### [[day06.md|Day 6: Probably a Fire Hazard]]
**Title**: Probably a Fire Hazard  
**Part 1 Type**: Simulation + Data Structures  
**Part 1 Description**: Control 1000x1000 grid of lights with turn on/off/toggle commands on rectangular regions  
**Part 2 Type**: Simulation + Mathematical  
**Part 2 Description**: Same commands but control brightness levels (+1/-1/+2) instead of boolean states  
**Key Concepts**: 2D grid operations, coordinate parsing, rectangular region processing, string command parsing, grid state management

---

### [[day07.md|Day 7: Some Assembly Required]]
**Title**: Some Assembly Required  
**Part 1 Type**: Data Structures + Simulation + Graph Algorithms  
**Part 1 Description**: Simulate a bitwise logic circuit with 339 wires, evaluate signal on wire 'a'  
**Part 2 Type**: Simulation + Graph Algorithms  
**Part 2 Description**: Override wire 'b' with Part 1 result, recalculate entire circuit (tests dynamic modification)  
**Key Concepts**: HashMap memoization, recursive dependency resolution, DAG traversal, bitwise operations (AND/OR/NOT/LSHIFT/RSHIFT), instruction parsing, cycle detection, dependency depth analysis (208 levels), circuit architecture understanding, zero-cost abstractions validation

**Circuit Architecture**:
- 339 wire instructions forming 208-level dependency DAG
- Sequential logic where each variable updates exactly once
- Wire 'b' at depth 0 (foundation), wire 'a' at depth 208 (apex)
- 336 gates required for wire 'a' computation
- Part 2 forces complete tree recalculation (1.46 TB virtual address space separation)

**Advanced Analysis Tools Created**:
- Debug mode with emoji-decorated logging
- PowerShell circuit analysis suite (6 specialized tools)
- Dependency tree visualization
- Maximum depth calculation with memoization
- Gate extraction and path analysis
- Performance benchmarking

---

### [[day08.md|Day 8: Matchsticks]]
**Title**: Matchsticks  
**Part 1 Type**: String Processing + Parsing  
**Part 1 Description**: Calculate difference between code representation length and in-memory string length after processing escape sequences  
**Part 2 Type**: String Processing + Encoding  
**Part 2 Description**: Calculate difference between encoded representation length and original code length  
**Key Concepts**: Escape sequence parsing (`\\`, `\"`, `\xHH`), character vs byte counting, string encoding

**Escape Sequences**:
- `\\` - Escaped backslash (represents single `\` in memory)
- `\"` - Escaped quote (represents single `"` in memory)
- `\xHH` - Hex escape (represents single byte/character in memory)

**Part 1 Examples**:
- `""` → 2 code - 0 memory = 2
- `"abc"` → 5 code - 3 memory = 2
- `"aaa\"aaa"` → 10 code - 7 memory = 3
- `"\x27"` → 6 code - 1 memory = 5

**Part 2 Examples** (encoding adds quotes and escapes special chars):
- `""` → `"\"\""` : 2 → 6 (+4)
- `"abc"` → `"\"abc\""` : 5 → 9 (+4)
- `"aaa\"aaa"` → `"\"aaa\\\"aaa\""` : 10 → 16 (+6)
- `"\x27"` → `"\"\\x27\""` : 6 → 11 (+5)
- `"\\zrs\\syur"` → `"\"\\\\zrs\\\\syur\""` : 13 → 21 (+8)

**Rust-Specific Challenge - UTF-8 Encoding Issue**:
- ⚠️ **Critical Bug Found**: Original implementation used `byte as char` for hex escapes
- **Problem**: `\xc4` (byte 196) when cast to `char` becomes Unicode U+00C4 ('Ä')
- **UTF-8 Encoding**: Character 'Ä' encodes as **2 bytes** (0xC3 0x84) in UTF-8
- **Expected**: `\xc4` should count as **1 character** in memory (byte-oriented, C-style strings)
- **Solution**: Created `count_memory_characters()` that counts logical characters, not UTF-8 bytes
- **Lesson**: AoC treats strings as byte arrays (like C), not UTF-8 strings (like Rust)

**Implementation Approach**:
- Part 1: Count logical characters during parsing, treating each escape as single character
- Part 2: Count encoded length by processing each character (`\` → 2, `"` → 2, others → 1) plus 2 for surrounding quotes
- Test Coverage: 18 tests (12 Part 1, 6 Part 2) validating all escape types and edge cases

**Results**:
- Part 1: 1342 (code - memory difference)
- Part 2: 2074 (encoded - code difference)

---

### [[day09.md|Day 9: All in a Single Night]]
**Title**: All in a Single Night  
**Part 1 Type**: Brute Force + Optimization + Graph Algorithms  
**Part 1 Description**: Find shortest route visiting all cities exactly once (Traveling Salesman Problem)  
**Part 2 Type**: Brute Force + Optimization + Graph Algorithms  
**Part 2 Description**: Find longest route visiting all cities exactly once (TSP maximization variant)  
**Key Concepts**: Traveling Salesman Problem, permutation generation, Heap's algorithm, graph traversal, distance matrix, brute force optimization, DRY principle implementation

**Algorithm Implementation**:
- **Heap's Algorithm**: Efficient permutation generation with O(n!) time complexity
- **Distance Matrix**: Bidirectional city-to-city distance lookup using Mission 5 Dictionary
- **Brute Force Search**: Exhaustive exploration of all possible routes
- **DRY Principle**: Single `solve_tsp()` function handles both min/max optimization via boolean parameter

**Rust-Specific Implementation Details**:
- **Lifetime Management**: Explicit lifetime annotations (`'a`) for string slice references in permutations
- **Error Handling**: `anyhow::Result` for robust error propagation with `?` operator
- **Data Structures**: Mission 5 `Dictionary<(&str, &str), usize>` for distance storage
- **Memory Efficiency**: Heap's algorithm minimizes memory allocation compared to recursive approaches
- **Test Coverage**: 11 comprehensive tests including edge cases (single city, two cities, malformed input)

**Performance Characteristics**:
- **Time Complexity**: O(n! × n) where n = number of cities
- **Space Complexity**: O(n! × n) for storing all permutations
- **Practical Limit**: ~10 cities before exponential explosion becomes prohibitive
- **Optimization Opportunities**: Could implement branch-and-bound or dynamic programming for larger instances

**Example Routes** (London-Dublin-Belfast):
- **Shortest**: London → Dublin → Belfast = 464 + 141 = 605
- **Longest**: Dublin → London → Belfast = 464 + 518 = 982

**Educational Value**:
- **Classic CS Problem**: Introduces the fundamental TSP problem and its variants
- **Algorithm Design**: Demonstrates when brute force is acceptable vs. when optimization is needed
- **Rust Patterns**: Lifetime management, error handling, generic function design
- **Competitive Programming**: Common pattern in algorithmic contests and optimization problems

---

### [[day10.md|Day 10: Elves Look, Elves Say]]
**Title**: Elves Look, Elves Say  
**Part 1 Type**: String Processing + Simulation  
**Part 1 Description**: Apply look-and-say sequence transformation 40 times, return final length  
**Part 2 Type**: String Processing + Simulation  
**Part 2 Description**: Apply look-and-say sequence transformation 50 times, return final length  
**Key Concepts**: Run-length encoding, iterative string transformation, consecutive character counting, exponential growth patterns, while loop with manual index control, algorithm optimization analysis

**Look-and-Say Sequence**:
- Read previous term and describe what you see
- "1" → "one 1" → "11"
- "11" → "two 1s" → "21"
- "21" → "one 2, one 1" → "1211"
- "1211" → "one 1, one 2, two 1s" → "111221"

**Algorithm Implementation**:
- **Run-Length Encoding**: Count consecutive identical characters
- **While Loop Pattern**: Manual index control for look-ahead and variable jumps
- **Grouping Operation**: Inner while loop counts consecutive characters, outer while loop processes groups
- **Skip Optimization**: `i += count` (not `i += 1`) to skip entire counted group
- **Bounds Checking**: `i + count < len()` to prevent out-of-bounds access

**Performance Analysis - Iterative vs Memoized**:
- **Iterative Approach**: 340ms for 50 iterations ✅ (simple loop, minimal overhead)
- **Memoized Approach**: 394ms for 50 iterations ❌ (15.7% slower due to overhead)
- **Cache Hit Rate**: 0% (sequences never repeat - linear chain with exponential growth)
- **Memory Comparison**: Iterative uses ~7MB (current string), Memoized uses ~10MB (all 50 intermediates cached)
- **Key Insight**: Memoization hurts performance when subproblems don't repeat!

**Why Memoization Fails Here**:
1. **No Repeated Subproblems**: Each iteration produces unique string never seen again
2. **Linear Sequence**: No branching or overlapping calls (unlike Fibonacci)
3. **Pure Overhead**: String cloning, hashing, HashMap operations add ~54ms with zero cache hits
4. **Exponential Growth**: Strings grow ~30% per iteration, ensuring uniqueness

**String Growth Pattern**:
- Iteration 0: 10 chars
- Iteration 10: 222 chars (22x growth)
- Iteration 20: 4,822 chars (22x growth)
- Iteration 30: 102,814 chars (21x growth)
- Iteration 40: 360,154 chars (3.5x growth)
- Iteration 50: 5,103,798 chars (14x growth)
- **Average**: ~30% growth per iteration

**Rust-Specific Implementation Details**:
- **For Loop vs While Loop**: While loop superior for grouping operations (needs look-ahead, variable jumps)
- **Index Control**: Manual `i += count` provides flexibility for skipping groups
- **String Building**: `push_str()` for count, `push()` for character
- **Redundant Code Elimination**: User discovered `if chars.is_empty()` check unnecessary - while condition already handles it (DRY principle)
- **Clean Code Principle**: Simpler code without redundant checks is more idiomatic Rust

**Comprehensive Documentation Created**:
- **DAY10_LEARNING_GUIDE.md**: Step-by-step implementation guide (200+ lines)
- **DAY10_BENCHMARK_ANALYSIS.md**: Full performance comparison and analysis
- **DAY10_MEMOIZATION_WALKTHROUGH.md**: Detailed explanation of memoized approach
- **DAY10_EXECUTION_TRACE.md**: Visual side-by-side execution comparison
- **DAY10_README.md**: Quick reference and command guide
- **Benchmark Suite**: `benches/day10_comparison.rs` with Criterion integration

**Educational Value**:
- **Algorithm Selection**: Demonstrates when simple iterative beats "clever" memoization
- **Benchmarking Importance**: "Premature optimization is the root of all evil" - measure first!
- **Data Pattern Recognition**: Understanding when caching helps vs hurts
- **Clean Code Principles**: Trust loop conditions, avoid redundant checks
- **Mission 5 Integration**: Shows MemoCache usage even when not beneficial (educational contrast)

**Results**:
- Part 1 (40 iterations): 492,982 characters
- Part 2 (50 iterations): 6,989,950 characters

---

### [[day11.md|Day 11: Corporate Policy]]
**Title**: Corporate Policy  
**Part 1 Type**: String Processing + Pattern Matching + Simulation  
**Part 1 Description**: Find next valid password using base-26 incrementing with validation rules  
**Part 2 Type**: String Processing + Pattern Matching + Simulation  
**Part 2 Description**: Find the next valid password after Part 1 result  
**Key Concepts**: Base-26 counting with carry, sliding window for consecutive sequences, non-overlapping pattern detection, optimization through forbidden character skipping, string validation rules

**Password Requirements**:
1. **8 lowercase letters** (a-z only)
2. **Incrementing wraps**: `xx → xy → xz → ya → yb` (like odometer)
3. **Must include increasing straight**: At least 3 consecutive letters (`abc`, `bcd`, `xyz`)
4. **No forbidden characters**: Cannot contain `i`, `o`, or `l` (too confusing)
5. **Two different pairs**: At least 2 non-overlapping pairs (`aa`, `bb`, not just one pair twice)

**Algorithm Implementation**:
- **Base-26 Incrementing**: Like counting in base-26 (a=0, z=25)
  - Single increment: `xx → xy → xz`
  - Wrap with carry: `xz → ya` (z wraps to a, carry left)
  - Multiple carries: `zz → aa` (both wrap)
- **Sliding Window**: Check 3-character windows for consecutive letters
- **Pair Detection**: Find non-overlapping pairs by tracking last pair position
- **Optimization**: Skip entire ranges when hitting forbidden characters
  - If password contains `i`: jump directly to next `j` prefix
  - Avoids checking thousands of invalid passwords

**Validation Rules**:
1. **`has_increasing_straight()`**: Slide window through string, check if chars are consecutive (`b == a+1 && c == b+1`)
2. **`has_no_forbidden_chars()`**: Simple check for `i`, `o`, `l` presence
3. **`has_two_pairs()`**: Find first pair, then search for different pair after it
4. **`skip_forbidden_char()`**: When forbidden char found, increment that position and reset all following to `a`

**Example Progressions**:
- `hijklmmn` ❌: Has straight (`hij`) but contains forbidden `i` and `l`
- `abbceffg` ❌: Has pairs (`bb`, `ff`) but no increasing straight
- `abbcegjk` ❌: Has only one pair (`bb`)
- `abcdefgh` → `abcdffaa` ✅: First valid password
- `ghijklmn` → `ghjaabcc` ✅: Skips forbidden `i`

**Rust-Specific Implementation Details**:
- **Base-26 Arithmetic**: `((char as u8) + 1) as char` for incrementing
- **Reverse Iteration**: `(0..len).rev()` for right-to-left carry propagation
- **Vec<char> Mutability**: Convert string to char vector for in-place modification
- **Sliding Window Pattern**: Three consecutive index accesses `chars[i], chars[i+1], chars[i+2]`
- **Early Return Optimization**: Exit validation early on first failure
- **Range Skipping**: When forbidden char detected, jump entire range instead of increment-by-increment

**Performance Optimization**:
- **Without Skipping**: Would check every password sequentially (very slow)
- **With Skipping**: Jump past entire invalid ranges
  - Example: `abcdefhi` → skip directly to `abcdefjaa`
  - Saves checking `abcdefhj`, `abcdefhk`, ..., `abcdefiz` (unnecessary checks)
- **Early Validation**: Check forbidden chars first (cheapest check) before expensive pattern matching

**Educational Value**:
- **Base-N Counting**: Generalizes to any base (binary, octal, hex, base-26)
- **Carry Propagation**: Classic algorithm problem (odometer, clock arithmetic)
- **String Validation**: Multiple independent validation rules combined
- **Optimization Techniques**: Skip invalid ranges instead of brute force
- **Pattern Detection**: Sliding windows, non-overlapping constraints
- **Clean Code**: Separate validation functions for each rule (Single Responsibility)

---

### [[day12.md|Day 12: JSAbacusFramework.io]]
**Title**: JSAbacusFramework.io  
**Part 1 Type**: Parsing + Mathematical + Data Structures  
**Part 1 Description**: Parse JSON document and sum all numeric values (arrays, objects, nested structures)  
**Part 2 Type**: Parsing + Mathematical + Data Structures + Pattern Matching  
**Part 2 Description**: Sum all numbers, but ignore any object (and all its children) that has any property with value "red"  
**Key Concepts**: JSON parsing, recursive traversal, tree structures, numeric extraction, conditional filtering, deep object inspection

**JSON Structure Types**:
- **Arrays**: `[1, 2, 3]` - List of values
- **Objects**: `{"a": 1, "b": 2}` - Key-value pairs
- **Numbers**: Integers and negative numbers (`-1`, `42`, `100`)
- **Strings**: Text values (ignored for Part 1, special meaning in Part 2)
- **Nested Structures**: Arrays containing objects containing arrays, etc.

**Examples (Part 1)**:
- `[1,2,3]` → Sum: **6** (simple array)
- `{"a":2,"b":4}` → Sum: **6** (object values)
- `[[[3]]]` → Sum: **3** (deeply nested array)
- `{"a":{"b":4},"c":-1}` → Sum: **3** (nested object: 4 + (-1))
- `{"a":[-1,1]}` → Sum: **0** (array in object: -1 + 1)
- `[-1,{"a":1}]` → Sum: **0** (object in array: -1 + 1)
- `[]` and `{}` → Sum: **0** (empty structures)

**Algorithm Approaches**:

**Approach 1: Regex/Pattern Matching** (Simple but limited)
- Extract all number patterns from raw JSON string
- Use regex: `-?\d+` to find integers (negative and positive)
- Sum all matched numbers
- **Limitation**: Cannot handle Part 2 filtering (no structure awareness)

**Approach 2: JSON Deserialization** (Structured, Part 2-ready)
- Parse JSON into proper data structure using `serde_json`
- Recursively traverse the JSON tree
- Handle different value types: `Number`, `Array`, `Object`, `String`, `Bool`, `Null`
- Sum numbers while respecting filtering rules

**Recursive Traversal Pattern**:
```rust
fn sum_numbers(value: &Value) -> i64 {
    match value {
        Value::Number(n) => n.as_i64().unwrap_or(0),
        Value::Array(arr) => arr.iter().map(sum_numbers).sum(),
        Value::Object(obj) => obj.values().map(sum_numbers).sum(),
        _ => 0,  // Strings, bools, null = 0
    }
}
```

**Part 2 Filtering Logic**:
- Check if object contains property with value `"red"` (string "red")
- If found: ignore entire object and all nested values
- Arrays are never ignored (only objects can trigger filtering)
- Recursive filtering: check object before processing its contents

**Example (Part 2 - Red Filtering)**:
- `[1,2,3]` → Sum: **6** (no objects, no filtering)
- `[1,{"c":"red","b":2},3]` → Sum: **4** (1 + 3, object with "red" ignored)
- `{"d":"red","e":[1,2,3,4],"f":5}` → Sum: **0** (entire object ignored due to "red")
- `[1,"red",5]` → Sum: **6** (string "red" in array doesn't trigger filtering, only in objects)

**Rust-Specific Implementation Details**:
- **`serde_json` crate**: Industry-standard JSON parsing
- **`Value` enum**: Represents JSON types (`Number`, `Array`, `Object`, `String`, `Bool`, `Null`)
- **Pattern Matching**: `match` on `Value` variants for type handling
- **Recursive Functions**: Natural fit for nested JSON structures
- **Iterator Methods**: `.iter().map().sum()` for clean aggregation
- **Error Handling**: `as_i64()` with `unwrap_or(0)` for safe numeric conversion
- **Ownership**: JSON tree owned by parsed `Value`, references used in traversal

**Alternative Regex Approach** (Part 1 only):
```rust
fn sum_with_regex(json: &str) -> i64 {
    let re = Regex::new(r"-?\d+").unwrap();
    re.find_iter(json)
        .filter_map(|m| m.as_str().parse::<i64>().ok())
        .sum()
}
```
- **Pros**: Simple, fast for Part 1, no JSON parsing overhead
- **Cons**: Cannot handle Part 2 filtering (no structure awareness)
- **Use Case**: When you only need numbers, not structure

**Performance Considerations**:
- **Parsing Overhead**: JSON deserialization has initial cost
- **Regex Speed**: Faster for simple extraction (Part 1 only)
- **Memory**: Full JSON tree in memory vs streaming regex
- **Trade-off**: Regex for Part 1 speed, JSON parsing for Part 2 flexibility

**Educational Value**:
- **JSON Parsing**: Real-world data format handling
- **Recursive Algorithms**: Tree traversal patterns
- **Pattern Matching**: Rust's enum matching for type safety
- **External Crates**: Using `serde_json` for serialization/deserialization
- **Algorithm Selection**: Regex vs structured parsing trade-offs
- **Filtering Logic**: Conditional tree traversal
- **Data Structures**: Understanding JSON as a tree structure

---

## Problem Type Distribution (Available Days)

| Category | Part 1 Count | Part 2 Count |
|----------|--------------|--------------|
| String Processing | 6 | 6 |
| Mathematical | 3 | 4 |
| Simulation | 6 | 6 |
| Search/Traversal | 1 | 1 |
| Optimization | 1 | 2 |
| Data Structures | 4 | 2 |
| Brute Force | 2 | 2 |
| Cryptographic | 1 | 1 |
| Pattern Matching | 2 | 2 |
| Advanced Pattern Matching | 0 | 1 |
| Graph Algorithms | 2 | 3 |
| Parsing | 2 | 1 |
| Encoding | 0 | 1 |

## Implementation Notes

### Common Patterns Observed:
1. **Input Parsing**: Most problems require parsing structured input (dimensions, characters)
2. **Iterative Processing**: Process input character-by-character or line-by-line
3. **State Tracking**: Maintain running totals or current positions
4. **Part 2 Extensions**: Often adds complexity or early termination conditions to Part 1
5. **Brute Force Solutions**: Some problems require computational search without mathematical shortcuts

### Rust-Specific Considerations:
- Day 1: Good for practicing iterator methods, `chars()`, `enumerate()`
- Day 2: Excellent for tuple destructuring, `split()`, `parse()`, mathematical operations
- Day 3: Perfect for HashSet usage, coordinate systems, `step_by()` iterators, even/odd index splitting
- Day 4: Demonstrates external crates (`md5`), loop optimization, computational complexity, parallel processing
- Day 5: Ideal for regex crate usage, string pattern matching, `saturating_sub()`, manual string iteration vs regex trade-offs
- Day 6: Excellent for 2D grid data structures, coordinate systems, rectangular iteration, `split_whitespace()`, `saturating_sub()` for brightness bounds
- Day 7: **Advanced HashMap memoization**, recursive algorithms, enum-based instruction modeling, `anyhow` error handling, comprehensive test coverage (36 tests), professional debug tooling, dependency analysis algorithms, architectural understanding of DAG structures, zero-cost abstraction validation
- Day 8: **Critical UTF-8 vs byte array distinction**, escape sequence parsing, character counting vs byte counting, `chars()` iteration, understanding when `byte as char` fails, Rust strings are UTF-8 (not byte arrays like C), custom character counting logic
- Day 9: **Advanced algorithmic problem solving**, Heap's algorithm implementation, lifetime management with string slices, `anyhow::Result` error handling, Mission 5 Dictionary integration, permutation generation, DRY principle in function design, comprehensive test coverage (11 tests), competitive programming patterns
- Day 10: **Run-length encoding**, while loop with manual index control, iterative vs recursive performance comparison, benchmarking with Criterion, understanding when memoization hurts performance (0% cache hit rate), string growth patterns, clean code principles (avoiding redundant checks), comprehensive performance analysis documentation
- Day 11: **Base-26 counting with carry**, string validation with multiple rules, sliding window for pattern detection, non-overlapping pair constraints, optimization through range skipping, forbidden character handling, password incrementing algorithms
- Day 12: **JSON parsing with serde_json**, recursive tree traversal, pattern matching on Value enum, conditional filtering (red objects), regex vs structured parsing trade-offs, external crate integration, data structure selection (string scanning vs tree building)

---

## Adding New Days

To add a new day to this summary:

1. **Read the problem statement**
2. **Identify the core algorithm type** for each part
3. **Add entry following the format above**
4. **Update the distribution table**
5. **Note any new patterns or Rust learning opportunities**

### Template for New Days:
```markdown
### Day X: [Problem Title]
**Title**: [Problem Title]  
**Part 1 Type**: [Category]  
**Part 1 Description**: [Brief description]  
**Part 2 Type**: [Category]  
**Part 2 Description**: [Brief description]  
**Key Concepts**: [Relevant programming concepts]
```

---

*Last Updated: Based on available problem statements as of current date*
*Days Available: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12*

---

*Tags: #aoc #2015 #problem-analysis #patterns #string-processing #simulation #mathematical #data-structures #graph-algorithms #memoization #dag #circuit-simulation #competitive-programming #rust-learning #traveling-salesman #permutations #run-length-encoding #benchmarking #performance-analysis*
*Links: [[../../zettelkasten/AoC Patterns MOC]] | [[../../zettelkasten/AoC Collection Problems]] | [[../README]] | [[../../Mission5/README]] | [[../../daily_study/rust_learning_week2_notes/Day10]] | [[../../zettelkasten/HashMap Internals]] | [[../../zettelkasten/Memory Address Analysis]] | [[../../zettelkasten/Heap's Algorithm Deep Dive]] | [[DAY10_BENCHMARK_ANALYSIS]] | [[DAY10_MEMOIZATION_WALKTHROUGH]]*