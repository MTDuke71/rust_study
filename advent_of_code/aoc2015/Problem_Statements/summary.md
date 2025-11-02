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
- **Cellular Automaton**: Conway's Game of Life, state evolution, neighbor counting, grid simulation
- **Greedy Algorithms**: Optimal greedy strategies, reverse optimization, exploiting problem structure
- **Number Theory**: Divisor sums, highly composite numbers, sieve algorithms, multiplicative functions
- **Search**: Informed search algorithms, A* search, heuristics, state space exploration

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

### [[day18.md|Day 18: Like a GIF For Your Yard]]
**Title**: Like a GIF For Your Yard
**Part 1 Type**: Simulation + Data Structures + Cellular Automaton
**Part 1 Description**: Simulate Conway's Game of Life on 100x100 grid for 100 steps, count lights ON
**Part 2 Type**: Simulation + Data Structures + Cellular Automaton
**Part 2 Description**: Same simulation but 4 corner lights are stuck in ON position
**Key Concepts**: Conway's Game of Life rules, 8-connected neighbor counting, cellular automaton simulation, grid state evolution, Mission 6 Grid integration, double buffering technique, stuck corner constraints, pattern stability analysis

**Game of Life Rules**:
- Light ON with 2-3 neighbors ON → stays ON
- Light ON with 0-1 or 4+ neighbors ON → turns OFF
- Light OFF with exactly 3 neighbors ON → turns ON
- Light OFF otherwise → stays OFF

**Algorithm Implementation**:
- **Mission 6 Integration**: Use `Grid<bool>` for efficient 2D representation, `neighbors_8_bounded()` for safe 8-connected iteration
- **Double Buffering**: Maintain current and next state grids to prevent state corruption during simultaneous updates
- **Neighbor Counting**: `neighbors_8_bounded()` iterator filters and counts ON neighbors
- **Part 2 Constraint**: Force 4 corner cells to ON after each simulation step

**Performance Characteristics**:
- **Time Complexity**: O(steps × width × height × 8) = O(steps × n²) for n×n grid
- **Space Complexity**: O(width × height) for two grids (current + next)
- **Actual Runtime**: ~1ms for 100 steps on 100×100 grid
- **Mission 6 Benefits**: Safe bounds checking, O(1) indexing, iterator-based neighbor access

**Example Evolution**:
```
Start: 5076 lights ON
Step 10: ~1500 lights ON
Step 50: ~1100 lights ON  
Step 100: 1061 lights ON (stable oscillation)
```

**Part 2 Pattern Analysis**:
- Stuck corners act as "light sources" preventing total extinction
- Creates persistent activity near corners
- Different final pattern: 1006 lights (55 fewer than Part 1)
- Corner influence creates stable edge patterns

**Visualization Tools Created**:
1. **Part 1 Animation** (`day18_animation_part1.rs`): Step-by-step 6×6 test grid with ANSI colors
2. **Part 2 Animation** (`day18_animation_part2.rs`): Stuck corners highlighted in red, legend display
3. **Comparison Tool** (`day18_comparison.rs`): Side-by-side Part 1 vs Part 2 evolution
4. **Interactive Simulator** (`day18_interactive.rs`): Full 100×100 grid with:
   - User-selectable mode (Part 1 or Part 2)
   - Configurable steps and animation speed
   - 2×2 Unicode block compression display (16 character variants)
   - Auto-save every 25 steps with metadata
   - Statistics tracking (min/max/stability detection)
   - Adaptive speed adjustment after 100 steps

**Rust-Specific Implementation Details**:
- **Grid Indexing**: `grid[Coord::new(row, col)]` for O(1) access
- **Iterator Filtering**: `.filter(|&neighbor| grid[neighbor]).count()` for neighbor counting
- **Match Expressions**: Pattern matching on (current_state, neighbor_count) tuples for rule application
- **Array Literals**: Corner coordinates stored as `[Coord; 4]` constant array
- **Comprehensive Testing**: 12 tests covering parsing, neighbor counting, simulation steps, corner forcing

**Mission 6 Advantage** (vs manual implementation):
```rust
// With Mission 6: Clean and safe
fn count_neighbors_on(grid: &Grid<bool>, coord: Coord) -> usize {
    neighbors_8_bounded(&coord, grid.width(), grid.height())
        .filter(|&neighbor| grid[neighbor])
        .count()
}

// Without Mission 6: Error-prone manual bounds checking
fn count_neighbors_unsafe(grid: &Vec<Vec<bool>>, row: usize, col: usize) -> usize {
    let mut count = 0;
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 { continue; }
            let nr = row as i32 + dr;
            let nc = col as i32 + dc;
            if nr >= 0 && nr < 100 && nc >= 0 && nc < 100 {
                if grid[nr as usize][nc as usize] { count += 1; }
            }
        }
    }
    count
}
```

**Educational Value**:
- **Cellular Automaton**: Classic computational model for complex systems
- **Grid Data Structures**: Efficient 2D array representation and iteration
- **Neighbor Algorithms**: 8-connected grid traversal patterns
- **State Evolution**: Double buffering to prevent update conflicts
- **Visualization**: ANSI terminal graphics, Unicode block characters
- **Interactive Tools**: Building user-facing simulation interfaces
- **Mission Integration**: Leveraging custom data structures for cleaner code

**Test Coverage** (12 tests):
- Input parsing validation (# → ON, . → OFF)
- Neighbor counting (corners have 3-5 neighbors, interior has 8)
- Game of Life rules (stay ON, turn ON, turn OFF scenarios)
- Multi-step simulation accuracy
- Corner forcing (Part 2 constraint verification)
- Integration tests (full 100-step simulation)

**Results**:
- Part 1: 1061 lights ON after 100 steps
- Part 2: 1006 lights ON after 100 steps (with stuck corners)

**📖 Complete Documentation**: [[../src/solver/day18.md]] - Comprehensive problem analysis, Mission 6 integration details, performance characteristics, visualization examples
**🎨 Interactive Demo**: Run `cargo run --example day18_interactive` for full 100×100 simulation with auto-save and statistics

---

### [[day19.md|Day 19: Medicine for Rudolph]]
**Title**: Medicine for Rudolph
**Part 1 Type**: String Processing + Pattern Matching + Data Structures
**Part 1 Description**: Count distinct molecules generated by applying all possible single replacements to medicine molecule
**Part 2 Type**: Search + Greedy Algorithms + Graph Algorithms
**Part 2 Description**: Find minimum number of steps to synthesize medicine molecule from single electron using reverse greedy approach
**Key Concepts**: String replacement patterns, HashSet for uniqueness, molecular synthesis simulation, reverse engineering optimization, greedy algorithm design, pattern recognition in replacement rules

**The Molecular Replacement Problem**:
- Given a set of replacement rules (e.g., `H => HO`, `O => HH`)
- Part 1: Apply each rule at each position in molecule, count unique results
- Part 2: Find minimum steps to build target molecule from single electron (`e`)

**Part 1 Algorithm - All Single Replacements**:
```rust
// For each replacement rule
for (from, to) in replacements {
    // Find all occurrences of 'from' in molecule
    for position in molecule.match_indices(from) {
        // Replace at this position, add to HashSet for uniqueness
        let new_molecule = replace_at_position(molecule, position, from, to);
        unique_molecules.insert(new_molecule);
    }
}
```
- Time Complexity: O(n × m × k) where n=rules, m=molecule length, k=avg replacements per rule
- HashSet ensures only distinct molecules counted (HOOH counted once, not twice)

**Part 2 Algorithm - Reverse Greedy**:
- **Key Insight**: Working backwards is easier than forward search
- Start with target molecule, apply replacements in reverse until reach `e`
- Greedy strategy: Always apply longest possible replacement first
- Why it works: Replacement grammar has special structure (unambiguous reverse path)

```rust
let mut current = medicine_molecule.clone();
let mut steps = 0;

while current != "e" {
    // Try all reverse replacements, pick longest match
    for (from, to) in replacements {
        if let Some(pos) = current.find(to) {
            current = current.replacen(to, from, 1);  // Reverse: to => from
            steps += 1;
            break;  // Greedy: take first match found
        }
    }
}
```

**Example (Part 1)**:
```
Rules: H => HO, H => OH, O => HH
Molecule: HOH

Replacements:
- H(0) => HO: HOOH
- H(0) => OH: OHOH
- O(1) => HH: HHHH
- H(2) => HO: HOHO
- H(2) => OH: HOOH (duplicate!)

Unique molecules: 4 (HOOH, OHOH, HHHH, HOHO)
```

**Example (Part 2)**:
```
Rules: e => H, e => O, H => HO, H => OH, O => HH
Target: HOH

Forward (hard - exponential search):
e => O => HH => HOH (3 steps, but how to find this path?)

Reverse (easy - greedy):
HOH => HH (reverse H => OH)
HH => O (reverse O => HH)
O => e (reverse e => O)
Total: 3 steps
```

**Why Reverse Greedy Works** (Problem-Specific):
- Replacement grammar has special structure (context-free, unambiguous)
- Each reverse step reduces molecule size monotonically
- No backtracking needed - first valid replacement always works
- This is NOT general (works for this specific AoC problem structure)

**Rust-Specific Implementation Details**:
- **HashSet<String>**: Automatic deduplication for Part 1
- **`match_indices()`**: Find all occurrences of pattern with positions
- **String Slicing**: `&molecule[..pos]` + replacement + `&molecule[pos+len..]`
- **`clone()`**: Necessary for owned strings in HashSet
- **`replacen(to, from, 1)`**: Replace first occurrence only (greedy single step)
- **Iterator Patterns**: `.lines()`, `.split(" => ")`, `.collect::<HashSet>()`

**Performance Characteristics**:
- **Part 1**: O(n × m) where n=rules (~45), m=molecule length (~500)
  - Runtime: ~1ms (simple string operations)
- **Part 2**: O(steps × rules) where steps ≈ 200-300
  - Runtime: <1ms (greedy finds path quickly)
  - Forward BFS would be O(b^d) - exponential explosion!

**Why Forward Search Fails**:
- **Branching Factor**: Each molecule can expand to dozens of variants
- **Depth**: Need 200+ steps to reach target
- **Search Space**: Billions of possible molecular states
- **Memory**: Cannot store all visited states
- **Greedy Reversal**: Collapses this to linear time

**Pattern Recognition in Rules**:
- Most rules expand molecules (H => HO adds characters)
- Reverse application contracts molecules (deterministic shrinking)
- Special structure allows greedy to be optimal
- Similar to CYK parsing for context-free grammars

**Educational Value**:
- **Bidirectional Search**: Sometimes reverse is easier than forward
- **Greedy Algorithms**: When greedy is optimal (vs when it fails)
- **String Manipulation**: Replacement at specific positions
- **HashSet Usage**: Deduplication in combinatorial problems
- **Problem Structure**: Recognizing exploitable patterns in rules
- **Algorithm Selection**: BFS vs greedy based on problem properties

**Test Coverage** (examples created during solving):
- Part 1: Small example (HOH → 4 distinct molecules)
- Part 2: Reverse path finding (HOH from e in 3 steps)
- Edge cases: No replacements possible, already at target
- Large molecule stress test (500+ character target)

**Results**:
- Part 1: 535 distinct molecules from single replacements
- Part 2: 212 steps to synthesize medicine from electron

**Why This Problem is Interesting**:
- **Deceptive Complexity**: Looks like BFS, actually greedy
- **Domain Modeling**: Chemical synthesis as string rewriting
- **Algorithm Insight**: Understanding when to reverse the problem
- **Performance Gap**: Greedy O(n) vs BFS O(b^d) - dramatic difference

**Related Concepts**:
- **Formal Grammars**: Context-free language generation
- **CYK Parsing**: Bottom-up parsing algorithms
- **String Rewriting Systems**: L-systems, term rewriting
- **Molecular Synthesis**: Actual chemistry planning problems

---

### [[day20.md|Day 20: Infinite Elves and Infinite Houses]]
**Title**: Infinite Elves and Infinite Houses
**Part 1 Type**: Mathematical + Simulation + Number Theory
**Part 1 Description**: Find lowest house number receiving at least 33,100,000 presents (sum of divisors × 10)
**Part 2 Type**: Mathematical + Simulation + Number Theory
**Part 2 Description**: Same problem with modified rules - elves visit only 50 houses, deliver 11 presents each
**Key Concepts**: Divisor sum calculation, sieve-like simulation, elf delivery patterns, multiples iteration, cache-friendly algorithms, upper bound estimation, early termination optimization, number theory application

**The Delivery Problem**:
- Elf N delivers to houses: N, 2N, 3N, 4N, ... (all multiples of N)
- Each elf delivers presents equal to their number times multiplier (10 for Part 1, 11 for Part 2)
- House receives presents from all elves whose numbers divide it evenly
- Total presents = sum of (elf number × multiplier) for all visiting elves

**Mathematical Insight**:
- Presents at house H = sum_of_divisors(H) × multiplier
- House 1: divisors [1] → 1×10 = 10 presents
- House 4: divisors [1,2,4] → (1+2+4)×10 = 70 presents
- House 6: divisors [1,2,3,6] → (1+2+3+6)×10 = 120 presents

**Algorithm Implementation**:

**Approach 1: Divisor Calculation** (O(√n) per house):
```rust
fn sum_of_divisors(n: usize) -> usize {
    let mut sum = 0;
    for i in 1..=sqrt(n) {
        if n % i == 0 {
            sum += i;
            if i != n/i { sum += n/i; }  // Add paired divisor
        }
    }
    sum
}
```
- **Limitation**: Must check every house sequentially (slow for large targets)

**Approach 2: Elf Simulation** (Sieve-like, O(n log n)):
```rust
let mut houses = vec![0; max_houses + 1];
for elf in 1..=max_houses {
    let mut house = elf;
    while house <= max_houses {
        houses[house] += elf * 10;  // Elf delivers to multiples
        house += elf;
    }
}
```
- **Advantages**: Cache-friendly, iterates through elves (not houses), more efficient for finding first match
- **Key Insight**: Flip perspective from "find divisors of house" to "mark multiples of elf"

**Part 2 Modifications**:
- Each elf visits only first 50 houses (not infinite)
- Presents per delivery: 11 (instead of 10)
- Creates asymmetry: large house numbers get fewer presents (many elves exhausted their 50 visits)
- Upper bound estimation: target/11 (better than target/10 for Part 1)

**Performance Characteristics**:
- **Time Complexity**: O(n log n) where n = max_houses (harmonic series sum)
- **Space Complexity**: O(n) for houses array
- **Upper Bound Estimation**: target/10 for Part 1, target/11 for Part 2
- **Actual Runtime**: ~100ms for target=33,100,000 (max_houses ≈ 3.3M)
- **Early Termination**: Stop on first house meeting threshold (don't compute all)

**Example Delivery Patterns**:
```
House 1: Elf 1 → 10 presents
House 2: Elf 1, Elf 2 → 10 + 20 = 30 presents
House 4: Elf 1, Elf 2, Elf 4 → 10 + 20 + 40 = 70 presents
House 6: Elf 1, Elf 2, Elf 3, Elf 6 → 10 + 20 + 30 + 60 = 120 presents
House 8: Elf 1, Elf 2, Elf 4, Elf 8 → 10 + 20 + 40 + 80 = 150 presents
```

**Upper Bound Reasoning**:
- **Part 1**: If only Elf 1 visited, house N would get N×10 presents
- To get target T presents, need at least house T/10
- In reality, houses have multiple divisors, so answer is lower
- Conservative estimate: target/10 works well in practice

**Rust-Specific Implementation Details**:
- **Vector Pre-allocation**: `vec![0; max_houses + 1]` for O(1) indexed access
- **Index 0 Placeholder**: Houses numbered 1-N, index 0 unused
- **Stepped Iteration**: `house += elf` to jump to next multiple
- **Visit Limiting** (Part 2): Counter to enforce 50-house limit per elf
- **Early Exit**: Break on first house >= target (`.iter().enumerate().skip(1)`)
- **Dead Code Annotation**: `#[allow(dead_code)]` on `sum_of_divisors()` (educational but unused)

**Why Sieve Approach Wins**:
- **Cache Locality**: Sequential memory access for each elf's multiples
- **One Pass**: Compute all houses simultaneously
- **Natural Termination**: Find answer while simulating (no re-scanning)
- **Simpler Logic**: "Elf delivers to multiples" more intuitive than "find divisors"

**Number Theory Connection**:
- **Divisor Function**: σ(n) = sum of divisors of n
- **Highly Composite Numbers**: Many divisors = high present count
- Numbers like 120, 240, 360 are "highly composite" (many small factors)
- Answer tends to be highly composite number just exceeding threshold

**Educational Value**:
- **Algorithm Perspective**: Same problem, different viewpoint (divisors vs multiples)
- **Performance Analysis**: O(√n) per house vs O(n log n) total sieve
- **Number Theory**: Divisor sums, highly composite numbers, multiplicative functions
- **Cache Optimization**: Memory access patterns impact real-world performance
- **Sieve Techniques**: Pattern similar to Sieve of Eratosthenes (primes)
- **Upper Bound Estimation**: Heuristic reasoning for search space limits

**Test Coverage** (8 tests):
- Divisor sum calculation accuracy
- House present calculation verification (examples from problem)
- Small target searches (10, 30, 70, 120, 150)
- Input parsing with whitespace handling
- Part 1 vs Part 2 constraint differences
- Edge case: Finding house 8 gets exactly 150 presents

**Results**:
- Part 1: 776160 (lowest house with ≥33,100,000 presents, infinite visits, 10 presents/elf)
- Part 2: 786240 (lowest house with ≥33,100,000 presents, 50 visit limit, 11 presents/elf)

**Why Part 2 Answer is Higher**:
- Limited visits (50) means large houses get fewer total presents
- Higher multiplier (11 vs 10) partially compensates but not enough
- Net effect: need higher house number to reach same present threshold

**Related Problems**:
- **Sieve of Eratosthenes**: Similar multiples-marking pattern
- **Perfect Numbers**: Sum of divisors analysis
- **Highly Composite Numbers**: Maximizing divisor count
- **Goldbach's Conjecture**: Number theory exploration

---

### [[day21.md|Day 21: RPG Simulator 20XX]]
**Title**: RPG Simulator 20XX
**Part 1 Type**: Optimization + Simulation + Combinatorial Optimization
**Part 1 Description**: Find minimum cost equipment combination that defeats the boss (1 weapon, 0-1 armor, 0-2 rings)
**Part 2 Type**: Optimization + Simulation + Combinatorial Optimization
**Part 2 Description**: Find maximum cost equipment combination that loses to the boss (guaranteeing defeat)
**Key Concepts**: Equipment combination generation, turn-based combat simulation, cost optimization, brute force equipment search, constraint-based filtering (must buy exactly 1 weapon, optional armor, 0-2 rings), different optimization goals (min cost win vs max cost loss)

---

### [[day22.md|Day 22: Wizard Simulator 20XX]]
**Title**: Wizard Simulator 20XX
**Part 1 Type**: Search + Optimization + Simulation + Graph Algorithms
**Part 1 Description**: Find minimum mana cost spell sequence to defeat boss using A* search through game state space (player: 50 HP, 500 mana; boss stats from input)
**Part 2 Type**: Search + Optimization + Simulation + Graph Algorithms
**Part 2 Description**: Same optimization but with hard mode: lose 1 HP at start of each player turn before effects
**Key Concepts**: A* search algorithm, game state graph modeling, spell effect timers, turn-based combat with effects (Shield/Poison/Recharge), admissible heuristics (boss HP remaining), Mission9 pathfinding integration, state space explosion prevention, effect timer management, mana cost minimization vs equipment cost minimization (Day 21), different search goals (optimal path vs optimal selection)

**The Wizard Combat Problem**:
- Player casts spells instead of attacking: Magic Missile (4 damage, 53 mana), Drain (2 damage +2 heal, 73 mana), Shield (7 armor for 6 turns, 113 mana), Poison (3 damage/turn for 6 turns, 173 mana), Recharge (101 mana/turn for 5 turns, 229 mana)
- Effects apply at start of each turn (both player and boss), then timer decreases
- Cannot cast spell if effect already active, but effects can start on turn they end
- Part 1: Find minimum mana spent sequence to win vs boss
- Part 2: Same but lose 1 HP at start of each player turn

**Algorithm Implementation - A* Search**:
- **Game State Modeling**: `GameState` tracks player HP/mana/effects, boss HP, total mana spent, turn number
- **Weighted Graph**: Each state has neighbors via valid spell casts, edge weight = spell mana cost
- **Admissible Heuristic**: Boss HP remaining (underestimates actual cost to win)
- **Mission9 Integration**: Uses `AstarPathfinder` and `BinaryHeapQueue` for efficient search
- **Effect Timer Management**: `EffectTimers` struct tracks Shield/Poison/Recharge timers
- **State Space**: Millions of potential states from effect combinations and HP values

**Key Differences from Day 21**:
- **Equipment Optimization (Day 21)**: Static combination generation (~8,000 sets), brute force enumeration, cost optimization
- **Spell Sequence Optimization (Day 22)**: Dynamic state evolution through turn-based simulation, A* search through state graph, mana spent minimization
- **Data Structures**: Equipment vectors (Day 21) vs game state graphs with effect timers (Day 22)
- **Algorithmic Challenge**: Combinatorial optimization (Day 21) vs graph search with state explosion (Day 22)
- **Solution Approach**: Brute force enumeration (Day 21) vs informed search with heuristics (Day 22)

**Performance Characteristics**:
- **State Space**: Millions of states vs thousands of equipment combinations
- **Search Algorithm**: A* with O(b^d) worst case vs brute force O(n) enumeration
- **Mission9 Benefits**: Efficient priority queue, reusable pathfinding infrastructure
- **Actual Runtime**: ~1-2 seconds for both parts vs <1ms for Day 21

**Educational Value**:
- **A* Algorithm**: Admissible heuristics, graph search, state space modeling
- **Game State Design**: Complex state representation with timers and effects
- **Problem Transformation**: RPG equipment optimization vs spell sequence optimization
- **Search vs Enumeration**: When to use informed search vs brute force
- **Mission9 Integration**: Leveraging specialized pathfinding libraries
- **Effect Systems**: Timer-based buffs/debuffs in turn-based games

**Results**:
- Part 1: 900 mana (minimum cost winning sequence)
- Part 2: 1216 mana (hard mode with HP loss each turn)

---

### [[day23.md|Day 23: Opening the Turing Lock]]
**Title**: Opening the Turing Lock
**Part 1 Type**: Simulation + Data Structures + Mathematical Computation
**Part 1 Description**: Execute assembly program implementing Collatz conjecture computation starting with register a=0, count steps to reach 1 (184 steps)
**Part 2 Type**: Simulation + Data Structures + Mathematical Computation
**Part 2 Description**: Same Collatz computation but start with register a=1, count steps to reach 1 (231 steps)
**Key Concepts**: Assembly language interpretation, register-based virtual machine, program counter management, Collatz conjecture implementation, conditional execution paths, mathematical algorithm hiding in low-level code, Turing machine principles, famous unsolved mathematical problems in programming

---

## Problem Type Distribution (Available Days)

| Category | Part 1 Count | Part 2 Count |
|----------|--------------|--------------|
| String Processing | 8 | 7 |
| Mathematical | 7 | 7 |
| Simulation | 12 | 12 |
| Search/Traversal | 1 | 2 |
| Optimization | 6 | 6 |
| Data Structures | 9 | 8 |
| Brute Force | 5 | 5 |
| Cryptographic | 1 | 1 |
| Pattern Matching | 4 | 3 |
| Advanced Pattern Matching | 0 | 1 |
| Graph Algorithms | 4 | 6 |
| Parsing | 2 | 1 |
| Encoding | 0 | 1 |
| Real-time Analysis | 0 | 1 |
| Conditional Logic | 0 | 1 |
| Combinatorial Optimization | 2 | 2 |
| Cellular Automaton | 1 | 1 |
| Number Theory | 1 | 1 |
| Greedy Algorithms | 0 | 1 |
| Search | 1 | 1 |

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
- Day 18: **Conway's Game of Life implementation**, Mission 6 Grid integration (`Grid<bool>`, `neighbors_8_bounded()`), 8-connected neighbor counting, cellular automaton rules, double buffering (current + next state), stuck corner constraints, pattern evolution analysis, ANSI terminal visualization, Unicode block character compression (2×2 cells), interactive simulation with auto-save, statistics tracking (min/max/stability), iterator-based neighbor filtering, match expressions for rule application, comprehensive test coverage (12 tests)
- Day 19: **Molecular replacement system**, string replacement patterns, HashSet for uniqueness tracking, `match_indices()` for pattern finding, reverse greedy algorithm, bidirectional search optimization (working backwards from target), understanding when greedy is optimal, string slicing and reconstruction, recognizing exploitable problem structure (unambiguous grammar), performance analysis (greedy O(n) vs BFS O(b^d)), pattern matching in replacement rules, context-free grammar concepts
- Day 20: **Number theory and divisor sums**, sieve-like simulation (marking multiples), cache-friendly algorithms (sequential memory access), upper bound estimation heuristics (target/multiplier), early termination optimization, vector pre-allocation for performance, stepped iteration (`house += elf`), visit limiting with counters, understanding algorithm perspective (divisors vs multiples viewpoint), harmonic series complexity O(n log n), highly composite numbers, educational dead code annotation (`#[allow(dead_code)]`), doc test ignore patterns
- Day 21: **Equipment optimization and turn-based combat simulation**, equipment combination generation with constraints (exactly 1 weapon, optional armor, 0-2 rings with no duplicates), cost calculation and comparison, brute force search through all valid combinations (~8,000 equipment sets), different optimization goals (min cost win vs max cost loss), reusable fight simulation logic, struct-based data modeling for items and characters, damage calculation with armor reduction (max(1, damage - armor)), turn-based combat loop with early termination, Pattern matching for equipment cost and stats calculation
- Day 22: **A* search algorithm implementation**, game state representation with effect timers, weighted graph modeling for spell casting decisions, admissible heuristic design (boss HP remaining), Mission9 pathfinding library integration, state space explosion management, effect timer state transitions, turn-based combat simulation with spell effects (Shield/Poison/Recharge), mana cost minimization through optimal path finding, graph traversal with priority queue optimization, complex state modeling for AI planning problems
- Day 23: **Collatz conjecture implementation through assembly language**, register-based virtual machine executing mathematical algorithm, conditional program paths based on initial register state, Collatz sequence step counting (184 for a=0, 231 for a=1), famous unsolved mathematical problem disguised as assembly programming, demonstrates computational universality of simple instruction sets, Turing machine principles in practice, mathematical computation through low-level operations, [[../examples/day23_collatz_analysis|complete analysis]]

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

*Last Updated: November 2, 2025*
*Days Available: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23*

---

*Tags: #aoc #2015 #problem-analysis #patterns #string-processing #simulation #mathematical #data-structures #graph-algorithms #memoization #dag #circuit-simulation #competitive-programming #rust-learning #traveling-salesman #permutations #run-length-encoding #benchmarking #performance-analysis #adjacency-graph #symmetry-optimization #circular-seating #tsp-variants #cyclic-behavior #state-machines #reindeer-olympics #mathematical-optimization #real-time-analysis #algorithm-complexity #performance-comparison #combinatorial-optimization #nested-loops #iterator-patterns #constrained-search #pattern-matching #conditional-logic #sparse-storage #parsing-patterns #filtering-logic #partial-matching #early-termination #subset-sum #backtracking #exponential-algorithms #np-complete #recursive-algorithms #decision-trees #cellular-automaton #game-of-life #neighbor-counting #grid-simulation #mission6-integration #double-buffering #visualization #molecular-synthesis #string-replacement #greedy-algorithms #bidirectional-search #reverse-optimization #number-theory #divisor-sums #sieve-algorithms #highly-composite-numbers #cache-optimization #harmonic-series #context-free-grammars #pattern-recognition #rpg-simulator #equipment-optimization #turn-based-combat #cost-optimization #brute-force-search #constraint-based-filtering #fight-simulation #collatz-conjecture #assembly-language #virtual-machine #program-counter #instruction-parsing #register-management #conditional-jumps #execution-tracing #turing-machine #mathematical-computation #unsolved-problems #algorithm-hiding #computational-universality*
*Links: [[../../../zettelkasten/AoC Patterns MOC]] | [[../../../zettelkasten/AoC Collection Problems]] | [[../../../zettelkasten/Obsidian Plugin Integration Strategy]] | [[../README]] | [[../../../missions/Mission5/README]] | [[../../../missions/Mission6/README]] | [[../../../daily_study/rust_learning_week2_notes/Day10]] | [[../../../zettelkasten/HashMap Internals]] | [[../../../zettelkasten/Memory Address Analysis]] | [[../../../zettelkasten/Heap's Algorithm Deep Dive]] | [[DAY10_BENCHMARK_ANALYSIS]] | [[DAY10_MEMOIZATION_WALKTHROUGH]] | [[../examples/day13_analysis]] | [[../examples/day14_analysis]] | [[../examples/DAY14_COMPLETE_SUMMARY]] | [[../examples/DOCUMENTATION_ENHANCEMENTS]] | [[../examples/GRAPHICS_GUIDE]] | [[../examples/day15_iterator_usage]] | [[../../../zettelkasten/Graph Theory MOC]] | [[../../../zettelkasten/TSP Algorithms]] | [[../../../zettelkasten/Subset-Sum-Scaling-Analysis]] | [[day18.md]] | [[../../../zettelkasten/Rust Learning Roadmap - The Master Plan]] | [[../../../zettelkasten/Optimization]] | [[../../../zettelkasten/Simulation]] | [[../../../zettelkasten/Brute Force Algorithms]] | [[../examples/day23_collatz_analysis]]*