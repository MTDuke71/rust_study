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

## Problem Type Distribution (Available Days)

| Category | Part 1 Count | Part 2 Count |
|----------|--------------|--------------|
| String Processing | 3 | 3 |
| Mathematical | 2 | 2 |
| Simulation | 2 | 2 |
| Search/Traversal | 1 | 1 |
| Optimization | 0 | 1 |
| Data Structures | 1 | 1 |
| Brute Force | 1 | 1 |
| Cryptographic | 1 | 1 |
| Pattern Matching | 1 | 0 |
| Advanced Pattern Matching | 0 | 1 |

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
*Days Available: 1, 2, 3, 4, 5*