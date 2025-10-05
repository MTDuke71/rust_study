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

### Day 1: Not Quite Lisp
**Title**: Not Quite Lisp  
**Part 1 Type**: String Processing + Mathematical  
**Part 1 Description**: Count opening/closing parentheses to determine final floor  
**Part 2 Type**: Search/Traversal + Simulation  
**Part 2 Description**: Find first position that causes Santa to enter basement (floor -1)  
**Key Concepts**: Character iteration, running sum, early termination condition

---

### Day 2: I Was Told There Would Be No Math
**Title**: I Was Told There Would Be No Math  
**Part 1 Type**: Mathematical + String Processing  
**Part 1 Description**: Calculate surface area of boxes plus slack (smallest side area)  
**Part 2 Type**: Mathematical + Optimization  
**Part 2 Description**: Calculate ribbon length (smallest perimeter + volume for bow)  
**Key Concepts**: Geometric calculations, parsing dimensions, finding minimum values

---

### Day 3: Perfectly Spherical Houses in a Vacuum
**Title**: Perfectly Spherical Houses in a Vacuum  
**Part 1 Type**: Simulation + Data Structures  
**Part 1 Description**: Track Santa's movement on 2D grid, count unique houses visited  
**Part 2 Type**: Simulation + String Processing + Data Structures  
**Part 2 Description**: Santa and Robo-Santa alternate moves (even/odd indices), count combined unique houses  
**Key Concepts**: 2D coordinate tracking, HashSet for uniqueness, character splitting by index, alternating processing

---

### Day 4: The Ideal Stocking Stuffer
**Title**: The Ideal Stocking Stuffer  
**Part 1 Type**: Brute Force + Cryptographic  
**Part 1 Description**: Find lowest number that creates MD5 hash starting with 5 zeros  
**Part 2 Type**: Brute Force + Cryptographic  
**Part 2 Description**: Find lowest number that creates MD5 hash starting with 6 zeros  
**Key Concepts**: MD5 hashing, brute force search, cryptographic hardness, computational complexity, prefix matching

---

### Day 5: Doesn't He Have Intern-Elves For This?
**Title**: Doesn't He Have Intern-Elves For This?  
**Part 1 Type**: String Processing + Pattern Matching  
**Part 1 Description**: Validate "nice" strings with 3+ vowels, consecutive letters, no forbidden substrings (ab/cd/pq/xy)  
**Part 2 Type**: String Processing + Advanced Pattern Matching  
**Part 2 Description**: New rules - non-overlapping letter pairs and letters with exactly one character between  
**Key Concepts**: Regular expressions, string validation, pattern matching, non-overlapping substring detection, character iteration with lookahead

---

### Day 6: Probably a Fire Hazard
**Title**: Probably a Fire Hazard  
**Part 1 Type**: Simulation + Data Structures  
**Part 1 Description**: Control 1000x1000 grid of lights with turn on/off/toggle commands on rectangular regions  
**Part 2 Type**: Simulation + Mathematical  
**Part 2 Description**: Same commands but control brightness levels (+1/-1/+2) instead of boolean states  
**Key Concepts**: 2D grid operations, coordinate parsing, rectangular region processing, string command parsing, grid state management

---

### Day 7: Some Assembly Required
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

### Day 8: Matchsticks
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

### Day 9: All in a Single Night
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

## Problem Type Distribution (Available Days)

| Category | Part 1 Count | Part 2 Count |
|----------|--------------|--------------|
| String Processing | 4 | 4 |
| Mathematical | 2 | 3 |
| Simulation | 4 | 4 |
| Search/Traversal | 1 | 1 |
| Optimization | 1 | 2 |
| Data Structures | 3 | 1 |
| Brute Force | 2 | 2 |
| Cryptographic | 1 | 1 |
| Pattern Matching | 1 | 0 |
| Advanced Pattern Matching | 0 | 1 |
| Graph Algorithms | 2 | 3 |
| Parsing | 1 | 0 |
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
*Days Available: 1, 2, 3, 4, 5, 6, 7, 8, 9*

---

*Tags: #aoc #2015 #problem-analysis #patterns #string-processing #simulation #mathematical #data-structures #graph-algorithms #memoization #dag #circuit-simulation #competitive-programming #rust-learning #traveling-salesman #permutations*
*Links: [[../../zettelkasten/AoC Patterns MOC]] | [[../../zettelkasten/AoC Collection Problems]] | [[../README]] | [[../../Mission5/README]] | [[../../daily_study/rust_learning_week2_notes/Day10]] | [[../../zettelkasten/HashMap Internals]] | [[../../zettelkasten/Memory Address Analysis]] | [[../../zettelkasten/Heap's Algorithm Deep Dive]]*