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
- **Real-time Analysis**: Temporal scoring, moment-by-moment leader tracking, time-dependent calculations
- **Conditional Logic**: Property-based filtering, range-based matching, rule-based comparisons
- **Combinatorial Optimization**: Subset sum, container packing, constrained combination enumeration

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

### [[advent_of_code/aoc2015/Problem_Statements/day12|Day 12: JSAbacusFramework.io]]
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

### [[day13.md|Day 13: Knights of the Dinner Table]]
**Title**: Knights of the Dinner Table  
**Part 1 Type**: Brute Force + Optimization + Graph Algorithms  
**Part 1 Description**: Find optimal circular seating arrangement that maximizes total happiness (TSP variant)  
**Part 2 Type**: Brute Force + Optimization + Graph Algorithms  
**Part 2 Description**: Add yourself (neutral happiness) and find new optimal arrangement  
**Key Concepts**: Traveling Salesman Problem, weighted directed complete adjacency graph, Heap's algorithm for permutations, circular seating constraints, symmetry exploitation for optimization, global vs. local optimization strategies

**📖 Complete Analysis**: [[../examples/day13_analysis|Day 13 Implementation Analysis]] - Comprehensive TSP algorithms, graph theory, optimization techniques, and mathematical proofs

---

### [[day14.md|Day 14: Reindeer Olympics]]
**Title**: Reindeer Olympics  
**Part 1 Type**: Simulation + Mathematical + Optimization  
**Part 1 Description**: Calculate maximum distance traveled by reindeer with cyclic flight/rest patterns over 2503 seconds  
**Part 2 Type**: Simulation + Data Structures + Real-time Analysis  
**Part 2 Description**: Award points each second to leading reindeer(s), find highest point total after 2503 seconds  
**Key Concepts**: Cyclic behavior simulation, state machine implementation, mathematical optimization vs brute force, real-time leader tracking, different scoring systems producing different winners

**Cyclic Flight/Rest Pattern**:
- Reindeer alternate between flying (constant speed) and resting (stationary)
- Each reindeer has fixed flight duration, rest duration, and flight speed
- Pattern repeats: fly → rest → fly → rest (predictable cycles)
- Example: Comet flies 14 km/s for 10s, then rests 127s (137s total cycle)

**Algorithm Approaches**:
- **Part 1 - Cycle-Based Calculation**: O(n × c) where c = race_duration / cycle_length
  - Calculate complete cycles + partial cycle remainder
  - Mathematical optimization using modular arithmetic
  - Much faster than second-by-second simulation for distance-only calculation
- **Part 2 - Real-time Simulation**: O(n × m) where m = race_duration
  - Track position of each reindeer at every second
  - Award points to current leader(s) each second
  - Handle tie-breaking (multiple leaders get points simultaneously)
  - Cannot optimize since scoring depends on moment-by-moment leadership

**Key Insights**:
1. **Different Winners**: Distance winner ≠ Points winner (different scoring systems)
2. **Lead Changes**: Frequent leadership changes create complex point accumulation
3. **Cyclic Optimization**: Part 1 benefits from mathematical cycle analysis
4. **State Tracking**: Part 2 requires full simulation due to temporal scoring rules

**Rust-Specific Implementation Details**:
- **Struct Design**: `Reindeer` with `speed`, `flight_time`, `rest_time` fields
- **Cycle Methods**: `cycle_length()`, `distance_per_cycle()` for mathematical optimization
- **State Simulation**: while loop with alternating flight/rest phases
- **Vector Tracking**: Points array updated each second for all reindeer
- **Iterator Usage**: `iter().enumerate().max_by()` for finding leaders
- **Performance Analysis**: Comparing O(n×c) vs O(n×m) algorithmic approaches

**Performance Characteristics**:
- **Part 1 Optimization**: ~2500× faster using cycles vs simulation (2503s → 18 cycles for example data)
- **Part 2 Required Simulation**: Cannot optimize due to temporal scoring dependency
- **Memory Efficiency**: O(n) space for storing reindeer data and point tracking
- **Real-world Applications**: Resource scheduling, traffic optimization, manufacturing cycles

**Example Race Results**:
- **Part 1 (Distance)**: Comet wins with 1120 km after 1000s
- **Part 2 (Points)**: Dancer wins with 689 points (had more leading moments)
- **Lesson**: Consistent performers can accumulate more points than fastest finishers

**Educational Value**:
- **Algorithm Selection**: When to optimize mathematically vs simulate directly  
- **Scoring System Design**: How different metrics produce different winners
- **State Machine Patterns**: Cyclic behavior modeling in software
- **Performance Analysis**: Understanding complexity trade-offs between approaches
- **Real-time Systems**: Temporal scoring and leader tracking implementations

**📖 Complete Analysis**: [[../examples/day14_analysis|Day 14 Implementation Analysis]] - Comprehensive cyclic behavior simulation and mathematical optimization techniques
**📋 Complete Summary**: [[../examples/DAY14_COMPLETE_SUMMARY|Day 14 Complete Summary]] - Full problem walkthrough with solution approach
**📚 Documentation Guide**: [[../examples/DOCUMENTATION_ENHANCEMENTS|Documentation Enhancement Guide]] - Best practices for AoC solution documentation  
**🎨 Graphics Guide**: [[../examples/GRAPHICS_GUIDE|Graphics and Visualization Guide]] - Visual representation techniques for algorithm analysis

---

### [[day15.md|Day 15: Science for Hungry People]]
**Title**: Science for Hungry People  
**Part 1 Type**: Brute Force + Optimization + Mathematical  
**Part 1 Description**: Find optimal cookie recipe using 100 teaspoons across 4 ingredients to maximize score (capacity × durability × flavor × texture)  
**Part 2 Type**: Brute Force + Optimization + Mathematical  
**Part 2 Description**: Same optimization but with constraint: recipe must have exactly 500 calories  
**Key Concepts**: Combinatorial optimization, nested loop generation with sum constraints, property calculation with negative value handling, iterator patterns (`.iter().enumerate().map().sum()`), dynamic ingredient handling (2/3/4 ingredients), constrained search space optimization

**Cookie Recipe Optimization**:
- Each ingredient has 5 properties: capacity, durability, flavor, texture, calories
- Properties can be negative (reduce total when multiplied by amount)
- Total score = `max(0, capacity_total) × max(0, durability_total) × max(0, flavor_total) × max(0, texture_total)`
- Must use exactly 100 teaspoons total across all ingredients
- If any property total is negative, overall score becomes 0

**Algorithm Implementation**:
- **Nested Loop Optimization**: Generate all valid combinations that sum to 100
  - 4 ingredients: 3 nested loops + calculated 4th value (not 4 loops!)
  - Reduces from 100^4 = 100 million to ~176,000 combinations
  - Pattern: `for a in 0..=100`, `for b in 0..=(100-a)`, `for c in 0..=(100-a-b)`, `let d = 100-a-b-c`
- **Property Calculation**: Sum each property weighted by amounts, then multiply non-negative totals
- **Calorie Constraint** (Part 2): Filter combinations to only those with exactly 500 calories
- **Dynamic Handling**: Separate functions for 2, 3, and 4 ingredients (future-proof for different inputs)

**Example (2 ingredients)**:
```
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3

Best recipe: 44 Butterscotch + 56 Cinnamon
Capacity: 44×(-1) + 56×2 = -44 + 112 = 68
Durability: 44×(-2) + 56×3 = -88 + 168 = 80
Flavor: 44×6 + 56×(-2) = 264 - 112 = 152
Texture: 44×3 + 56×(-1) = 132 - 56 = 76
Score: 68 × 80 × 152 × 76 = 62,842,880
```

**Rust-Specific Implementation Details**:
- **Iterator Chains**: `.iter().enumerate().map().sum()` for functional calorie calculation
- **Struct Design**: `Ingredient` with 6 fields (name + 5 properties)
- **Dead Code Annotation**: `#[allow(dead_code)]` on `name` field (used in Debug, not direct access)
- **Array Indexing**: `amounts[i]` for ingredient amounts parallel to `ingredients` slice
- **Negative Handling**: Return 0 early if any property total is negative (multiplication short-circuit)
- **Match Statement**: Dynamic dispatch for 2/3/4 ingredients using `match num_ingredients`
- **Loop Constraint**: Inner loop bounds calculated from outer loop variables to guarantee sum

**Iterator Usage Patterns** (3 distinct patterns):
1. **`.lines()` and `.split_whitespace()`**: Parse input into tokens
2. **`.next()` consumption**: Sequential token extraction with manual state tracking
3. **`.iter().enumerate().map().sum()`**: Functional transformation and aggregation

**Performance Characteristics**:
- **Search Space**: ~176,000 combinations for 4 ingredients (vs 100^4 = 100M brute force)
- **Time Complexity**: O(n^(k-1)) where k = number of ingredients, n = total teaspoons
- **Space Complexity**: O(k) for storing ingredient data and current amounts
- **Optimization**: Nested loop bounds prevent generating invalid combinations (sum ≠ 100)

**Educational Value**:
- **Combinatorics**: Generating combinations with sum constraints  
- **Optimization Problems**: Constrained search space exploration
- **Functional Programming**: Iterator methods for cleaner code (`.enumerate().map().sum()`)
- **Code Organization**: Separate validation functions, dynamic dispatch by input size
- **Negative Values**: Handling properties that reduce totals (not just positive contributions)
- **Multiple Constraints**: Distance optimization (Part 1) vs constrained optimization (Part 2)

**Results**:
- Part 1: 18,965,440 (best cookie without calorie constraint)
- Part 2: 15,862,900 (best cookie with exactly 500 calories)

**📖 Iterator Analysis**: [[../examples/day15_iterator_usage|Day 15 Iterator Usage Explained]] - Comprehensive breakdown of all iterator patterns used

---

### [[day16.md|Day 16: Aunt Sue]]
**Title**: Aunt Sue  
**Part 1 Type**: Data Structures + Pattern Matching + String Processing  
**Part 1 Description**: Find which of 500 Aunt Sues gave you a gift by matching known properties against MFCSAM analysis results  
**Part 2 Type**: Data Structures + Pattern Matching + Conditional Logic  
**Part 2 Description**: Same problem but with range-based comparisons (cats/trees > target, pomeranians/goldfish < target)  
**Key Concepts**: HashMap sparse storage, partial information matching, linear search with early termination, conditional comparison logic, pattern matching on string keys, range-based filtering

**The Gift Identification Problem**:
- 500 aunts all named "Sue" (numbered 1-500)
- Each aunt: remember exactly 3 out of 10 possible properties
- MFCSAM analysis provides target values for all 10 properties
- Unknown properties ≠ zero (just not remembered)
- Find the aunt where all known properties match

**MFCSAM Target Properties**:
```
children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0
vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1
```

**Algorithm Implementation**:
- **Part 1 - Exact Matching**:
  - Parse 500 aunt records with 3 properties each
  - Linear search: check if all aunt's known properties == target values
  - Early termination when match found
  - Result: Sue #40

- **Part 2 - Range-Based Matching** (Retroencabulator effect):
  - **cats, trees**: Aunt's value must be **> target** (nuclear decay → lower bounds)
  - **pomeranians, goldfish**: Aunt's value must be **< target** (magnetoreluctance → upper bounds)
  - **All others**: Aunt's value must be **== target** (exact measurements)
  - Result: Sue #241

**Data Structure Design**:
```rust
struct AuntSue {
    number: usize,
    properties: HashMap<String, usize>,  // Sparse storage: only 3/10 properties
}
```

**Why HashMap for Properties?**
- Each aunt knows only 3 properties (not all 10)
- HashMap provides sparse representation (O(1) lookup)
- Avoids storing zeros for unknown properties
- Efficiently checks "Does aunt have this property?" and "What's its value?"

**Rust-Specific Implementation Details**:
- **Pattern Matching on Strings**: `match key.as_str()` for property-specific logic
- **Conditional Comparison**: Different operators (>, <, ==) based on property name
- **Early Return Optimization**: `return false` on first mismatch (short-circuit)
- **String Parsing Chain**: `split(':')` → `strip_prefix("Sue ")` → `parse::<usize>()`
- **HashMap Iteration**: `for (key, value) in &self.properties` with conditional checks
- **Option Handling**: `if let Some(&target_value) = target.get(key)` for safe lookup

**Parsing Pattern**:
```rust
"Sue 1: goldfish: 9, cars: 0, samoyeds: 9"
     ↓
Split by ':' → ["Sue 1", " goldfish", " 9, cars", " 0, samoyeds", " 9"]
     ↓
Extract number: 1
     ↓
Parse properties: goldfish→9, cars→0, samoyeds→9
```

**Performance Characteristics**:
- **Time Complexity**: O(n × m) where n=500 aunts, m=3 properties per aunt
- **Space Complexity**: O(n × m) for storing all aunt data
- **Search**: Linear scan with early termination
  - Part 1: Stops at Sue #40 (checked ~8% of data)
  - Part 2: Stops at Sue #241 (checked ~48% of data)

**The Retroencabulator Effect (Part 2)**:
- MFCSAM has "outdated retroencabulator" causing measurement errors
- **Nuclear decay** of cat dander and tree pollen → readings are **minimums**
- **Modial interaction** of magnetoreluctance → pomeranians/goldfish readings are **maximums**
- Must adjust comparison logic to account for measurement uncertainty

**Why Different Sues?**:
- Sue #40: All known properties exactly match target (Part 1 winner)
- Sue #241: Known properties satisfy range constraints (Part 2 winner)
- Example difference: Sue #40 might have cats: 7 (exact), Sue #241 has cats: 8 (>7)

**Test Coverage** (11 tests):
- Basic parsing (multi-line, property extraction)
- Part 1: Exact matching, subset matching, mismatch detection
- Part 2: Greater-than (cats, trees), fewer-than (goldfish, pomeranians)
- Part 2: Exact matching for normal properties
- Edge cases: Equal values fail for range properties

**Educational Value**:
- **Sparse Data Representation**: HashMap for partial information
- **Conditional Logic**: Different rules for different data types
- **Linear Search Optimization**: Early termination strategies
- **Pattern Matching**: String-based dispatch in match expressions
- **Partial Matching**: Working with incomplete datasets
- **Constraint Satisfaction**: Multiple comparison rules simultaneously

**Results**:
- Part 1: 40 (exact matching with complete information)
- Part 2: 241 (range-based matching with measurement uncertainty)

---

### [[day17.md|Day 17: No Such Thing as Too Much]]
**Title**: No Such Thing as Too Much  
**Part 1 Type**: Brute Force + Data Structures + Combinatorial Optimization  
**Part 1 Description**: Count how many different combinations of containers exactly fit 150 liters of eggnog  
**Part 2 Type**: Brute Force + Optimization + Data Structures  
**Part 2 Description**: Find minimum number of containers needed, then count combinations using exactly that minimum  
**Key Concepts**: Subset sum problem (NP-complete), recursive backtracking with include/exclude pattern, exponential time complexity O(2^n), combination enumeration vs counting, two-phase optimization (find minimum then filter), algorithm scaling analysis

**The Container Packing Problem**:
- Given ~20 containers with different capacities (liters)
- Must use combinations that sum to exactly 150 liters
- Classic **subset sum problem** - NP-complete computational complexity
- Part 1: Count all valid combinations (any number of containers)
- Part 2: Among all combinations, find those using fewest containers

**Algorithm Implementation - Recursive Backtracking**:
```rust
// Core pattern: include/exclude decision at each step
fn count_recursive(containers: &[usize], index: usize, remaining: usize) -> usize {
    if remaining == 0 { return 1; }  // Found valid combination
    if index >= containers.len() { return 0; }  // No more options
    
    // Try including current container
    let include = if containers[index] <= remaining {
        count_recursive(containers, index + 1, remaining - containers[index])
    } else { 0 };
    
    // Try excluding current container  
    let exclude = count_recursive(containers, index + 1, remaining);
    
    include + exclude  // Sum of both branches
}
```

**Decision Tree Example** (simplified):
```text
containers = [20, 15, 10, 5, 5], target = 25

                    Start (rem=25)
                   /              \
          Include 20            Exclude 20
          (rem=5)               (rem=25)
           /    \                 /      \
      Inc 15  Exc 15         Inc 15    Exc 15
      (fail)  (rem=5)        (rem=10)  (rem=25)
              /    \           /    \       ...
          Inc 10  Exc 10   Inc 10  Exc 10
          (fail)  (rem=5)  ✓FOUND  (rem=10)
                           [15,10] 
```

**Computational Complexity Analysis**:
- **Search Space**: 2^n combinations (n = number of containers)
- **For n=20**: 2^20 = 1,048,576 combinations (~10-20ms runtime) ✅
- **For n=30**: 2^30 = 1,073,741,824 combinations (~30 seconds) ⚠️
- **For n=40**: 2^40 = 1+ trillion combinations (~12 hours) ❌
- **Threshold**: Brute force reasonable for n ≤ 25-30

**Two Algorithmic Approaches**:

1. **Counting Only** (Part 1 - Memory Efficient):
   - Only tracks count, doesn't store combinations
   - Space: O(n) for recursion stack
   - Fast: No combination cloning/storage overhead
   
2. **Collecting Combinations** (Part 2 - Needs Analysis):
   - Stores all valid combinations in Vec<Vec<usize>>
   - Space: O(k × m) where k=combo count, m=avg size
   - Required: Must analyze combination sizes to find minimum
   - Uses backtracking with push/pop pattern

**Part 2 Two-Phase Strategy**:
```rust
// Phase 1: Collect all combinations
let all_combinations = find_all_combinations(&containers, 150);

// Phase 2: Find minimum size
let min_count = all_combinations.iter().map(|c| c.len()).min().unwrap();

// Phase 3: Count combinations with that minimum
all_combinations.iter().filter(|c| c.len() == min_count).count()
```

**Optimization Considerations** (from [[../../../zettelkasten/Subset-Sum-Scaling-Analysis]]):
- **Sorting**: Descending order helps pruning (2-5x speedup) but not needed for AoC input size
- **Pruning**: Suffix sum checks eliminate impossible branches
- **Dynamic Programming**: O(n × target) but only counts, doesn't enumerate
- **Meet-in-the-Middle**: O(2^(n/2)) for n ≤ 45 but more complex
- **For n=20**: Simple brute force is optimal - don't over-engineer!

**Example (2 containers, target=25)**:
```
Containers: [20, 15, 10, 5, 5]

Valid combinations:
- [15, 10] = 25 ✓ (2 containers)
- [20, 5] = 25 ✓ (2 containers)
- [20, 5] = 25 ✓ (2 containers, different 5)
- [15, 5, 5] = 25 ✓ (3 containers)

Part 1: 4 combinations total
Part 2: Minimum = 2 containers, count = 3 combinations
```

**Rust-Specific Implementation Details**:
- **Recursive Pattern**: Include/exclude branches explore all 2^n subsets
- **Base Cases**: `remaining == 0` (success), `index >= len` (exhausted)
- **Mutable References**: `&mut Vec` for backtracking with push/pop
- **Iterator Methods**: `.filter()`, `.map()`, `.min()`, `.count()` for Part 2 analysis
- **No Memoization**: Each path unique (unlike Fibonacci) - cache hits would be 0%
- **Memory Trade-off**: Counting is O(n) stack, collecting is O(k×m) heap

**Performance Characteristics**:
- **Time**: O(n × 2^n) for counting (n operations per combination)
- **Space**: O(n) for counting, O(k × m) for collecting
- **Actual Runtime**: ~10-20ms for n=20 (AoC input size)
- **Scaling**: Doubles with each additional container (exponential growth)

**When Brute Force Breaks** (from scaling analysis):
- n ≤ 20: ✅ Instant (< 20ms)
- n ≤ 25: ✅ Fast (< 1 second)  
- n ≤ 30: ⚠️ Tolerable (< 1 minute)
- n > 35: ❌ Need better algorithm (minutes to hours)

**Educational Value**:
- **NP-Complete Recognition**: Classic subset sum problem structure
- **Exponential Complexity**: Understanding O(2^n) growth in practice
- **Backtracking Pattern**: Include/exclude decision tree exploration
- **Algorithm Scaling**: When simple solutions break and optimization needed
- **Space-Time Trade-offs**: Counting vs collecting combinations
- **Two-Phase Optimization**: Find constraint, then filter by constraint
- **Recursive Thinking**: Breaking problem into subproblems

**Test Coverage** (12 tests):
- Example validation (4 combinations for 25 liters)
- Part 2 minimum count (3 combinations with 2 containers)
- Edge cases: empty input, single exact container, no valid combinations
- Algorithm consistency: recursive count matches backtracking collection
- Input parsing: whitespace handling, line-by-line parsing

**Results**:
- Part 1: 1304 combinations (for n=20, target=150)
- Part 2: 18 combinations using minimum containers

**📚 Related Analysis**: 
- [[../../../zettelkasten/Subset-Sum-Scaling-Analysis]] - Complete exponential algorithm analysis, optimization strategies (DP, meet-in-the-middle, branch-and-bound), sorting impact on pruning, scaling thresholds
- [[../../../zettelkasten/AoC Collection Problems]] - Pattern recognition for subset problems

---

## Problem Type Distribution (Available Days)

| Category | Part 1 Count | Part 2 Count |
|----------|--------------|--------------|
| String Processing | 7 | 7 |
| Mathematical | 5 | 5 |
| Simulation | 7 | 7 |
| Search/Traversal | 1 | 1 |
| Optimization | 5 | 5 |
| Data Structures | 6 | 5 |
| Brute Force | 5 | 5 |
| Cryptographic | 1 | 1 |
| Pattern Matching | 3 | 3 |
| Advanced Pattern Matching | 0 | 1 |
| Graph Algorithms | 3 | 4 |
| Parsing | 2 | 1 |
| Encoding | 0 | 1 |
| Real-time Analysis | 0 | 1 |
| Conditional Logic | 0 | 1 |
| Combinatorial Optimization | 1 | 1 |

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
- Day 13: **Advanced graph theory implementation**, weighted directed complete adjacency graph using HashMap composite keys, Traveling Salesman Problem recognition and solution, Heap's algorithm for efficient permutation generation, circular seating constraint handling, mathematical symmetry exploitation for 9× performance optimization, global vs. local optimization analysis (why greedy "weakest link" fails), comprehensive verification testing proving optimization equivalence, HashMap adjacency list implementation, modular arithmetic for circular indexing
- Day 14: **Cyclic behavior simulation and mathematical optimization**, state machine implementation for flight/rest cycles, algorithmic complexity comparison (O(n×c) vs O(n×m)), struct design with behavior methods (`cycle_length()`, `distance_per_cycle()`), real-time leader tracking with tie handling, different scoring systems analysis, performance optimization through cycle mathematics, while loop state transitions, vector-based point accumulation, iterator methods for leader detection (`max_by()`), temporal vs final scoring trade-offs
- Day 15: **Combinatorial optimization with constraints**, nested loop generation with sum constraints (~176K combinations from 100^4 space), iterator patterns (`.iter().enumerate().map().sum()`), property calculation with negative value handling, dynamic ingredient handling (2/3/4 variations), constrained search space optimization, functional vs imperative iterator usage comparison, dead code annotations for struct fields
- Day 16: **HashMap sparse storage for partial data**, pattern matching on string keys (`match key.as_str()`), conditional comparison logic (different operators per property type), early return optimization, string parsing chain (`strip_prefix()`, `split()`, `parse()`), linear search with early termination, Option handling with `if let Some()`, property-specific range checks
- Day 17: **Subset sum backtracking**, recursive include/exclude pattern, combination enumeration with push/pop, minimum value filtering, Vec<Vec<T>> for storing combinations, O(2^n) complexity awareness, exponential algorithm scaling understanding, two-phase optimization (find constraint then filter), space-time trade-offs (counting vs collecting), NP-complete problem recognition, algorithm selection based on input size (n≤25 brute force acceptable)

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
*Days Available: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17*

---

*Tags: #aoc #2015 #problem-analysis #patterns #string-processing #simulation #mathematical #data-structures #graph-algorithms #memoization #dag #circuit-simulation #competitive-programming #rust-learning #traveling-salesman #permutations #run-length-encoding #benchmarking #performance-analysis #adjacency-graph #symmetry-optimization #circular-seating #tsp-variants #cyclic-behavior #state-machines #reindeer-olympics #mathematical-optimization #real-time-analysis #algorithm-complexity #performance-comparison #combinatorial-optimization #nested-loops #iterator-patterns #constrained-search #pattern-matching #conditional-logic #sparse-storage #parsing-patterns #filtering-logic #partial-matching #early-termination #subset-sum #backtracking #exponential-algorithms #np-complete #recursive-algorithms #decision-trees*
*Links: [[../../../zettelkasten/AoC Patterns MOC]] | [[../../../zettelkasten/AoC Collection Problems]] | [[../../../zettelkasten/Obsidian Plugin Integration Strategy]] | [[../README]] | [[../../../missions/Mission5/README]] | [[../../../daily_study/rust_learning_week2_notes/Day10]] | [[../../../zettelkasten/HashMap Internals]] | [[../../../zettelkasten/Memory Address Analysis]] | [[../../../zettelkasten/Heap's Algorithm Deep Dive]] | [[DAY10_BENCHMARK_ANALYSIS]] | [[DAY10_MEMOIZATION_WALKTHROUGH]] | [[../examples/day13_analysis]] | [[../examples/day14_analysis]] | [[../examples/DAY14_COMPLETE_SUMMARY]] | [[../examples/DOCUMENTATION_ENHANCEMENTS]] | [[../examples/GRAPHICS_GUIDE]] | [[../examples/day15_iterator_usage]] | [[../../../zettelkasten/Graph Theory MOC]] | [[../../../zettelkasten/TSP Algorithms]] | [[../../../zettelkasten/Subset-Sum-Scaling-Analysis]]*