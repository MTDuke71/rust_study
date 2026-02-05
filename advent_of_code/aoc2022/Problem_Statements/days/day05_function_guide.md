# Day 5: Supply Stacks - Function Guide

**Quick Links**: [← Day 4](day04_function_guide.md) | [Problem Statement](day05.md) | [Summary](../summary_2022.md) | [Day 6 →](day06_function_guide.md)

---

## Problem Overview
Simulate a cargo crane rearranging stacks of crates. Part 1 uses a CrateMover 9000 that moves crates one at a time (reversing order). Part 2 uses a CrateMover 9001 that moves multiple crates at once (preserving order).

**Answer**: Part 1: `CVCWCRTVQ` | Part 2: `CNSCZWLVT`

## Performance Benchmarks
- **Parse**: 59.5µs
- **Part 1**: 66.3µs
- **Part 2**: 78.7µs
- **Combined**: 85.0µs

## Core Algorithm: Stack Simulation

Parse ASCII art stack configuration and move instructions, then simulate crane operations using `Vec<Vec<char>>` as stacks.

**Algorithm**:
1. Parse initial stack configuration from ASCII art
2. Parse move instructions into `Move` structs
3. Part 1: Move crates one at a time (pop/push reverses order)
4. Part 2: Move multiple crates at once (preserves order using `split_off`)

**Time Complexity**: O(n + m*k) where n = input size, m = number of moves, k = crates per move  
**Space Complexity**: O(s*h) where s = number of stacks, h = average height

---

## Function Reference

### Main Entry Points

#### `solve(input: &str) -> (String, String)`
**Purpose**: Parse once, solve both parts

**Algorithm**:
```rust
pub fn solve(input: &str) -> (String, String) {
    let data = parse_input(input);
    (solve_part1_impl(&data), solve_part2_impl(&data))
}
```

**Efficiency**: Single parse reduces overhead from ~85µs to ~80µs compared to separate calls.

---

#### `solve_part1(input: &str) -> String`
**Purpose**: CrateMover 9000 - moves one crate at a time

**Algorithm**:
```rust
pub fn solve_part1(input: &str) -> String {
    let data = parse_input(input);
    let mut stacks = data.stacks.clone();
    
    for movement in &data.moves {
        // Pop from source, push to destination (count times)
        // This reverses the order
    }
    
    // Return top crate from each stack
}
```

**Example**:
```
Input: move 3 from 1 to 3
Stack 1: [D, N, Z] (top to bottom)
Result: D, then N, then Z moved individually
Stack 3: [..., Z, N, D] (reversed!)
```

---

#### `solve_part2(input: &str) -> String`
**Purpose**: CrateMover 9001 - moves multiple crates at once

**Algorithm**:
```rust
pub fn solve_part2(input: &str) -> String {
    let data = parse_input(input);
    let mut stacks = data.stacks.clone();
    
    for movement in &data.moves {
        // Calculate split point
        let split_at = stack_len - movement.count;
        
        // Take entire slice off source
        let crates = stacks[from].split_off(split_at);
        
        // Move to destination (preserves order)
        stacks[to].extend(crates);
    }
}
```

**Key Insight**: `split_off()` preserves order because we take a contiguous slice.

---

### Parsing Functions

#### `parse_input(input: &str) -> CargoArrangement`
**Purpose**: Split input into stack configuration and moves

```rust
pub fn parse_input(input: &str) -> CargoArrangement {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let stacks = parse_stacks(parts[0]);
    let moves = parse_moves(parts[1]);
    CargoArrangement { stacks, moves }
}
```

**Format**: Two sections separated by blank line.

---

#### `parse_stacks(stack_text: &str) -> Vec<Vec<char>>`
**Purpose**: Parse ASCII art stack configuration

```rust
fn parse_stacks(stack_text: &str) -> Vec<Vec<char>> {
    // 1. Find number of stacks from last line
    // 2. Process lines bottom-to-top (rev().skip(1))
    // 3. Extract crates at positions: 1, 5, 9, 13... (1 + 4*n)
    // 4. Push alphabetic characters to stacks
}
```

**Algorithm**: 
- Crate positions follow pattern: column 1, 5, 9, 13... (i.e., 1 + 4*stack_idx)
- Process lines in reverse to build stacks bottom-up
- Skip last line (stack numbers)

**Complexity**: O(rows * stacks)  
**Used By**: `parse_input()`

**Example**:
```
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

Result: [[Z, N], [M, C, D], [P]]
```

---

#### `parse_moves(moves_text: &str) -> Vec<Move>`
**Purpose**: Parse move instructions

```rust
fn parse_moves(moves_text: &str) -> Vec<Move> {
    moves_text.lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 6 && parts[0] == "move" {
                Some(Move {
                    count: parts[1].parse().ok()?,
                    from: parts[3].parse().ok()?,
                    to: parts[5].parse().ok()?,
                })
            } else {
                None
            }
        })
        .collect()
}
```

**Format**: "move N from X to Y"  
**Complexity**: O(lines)  
**Used By**: `parse_input()`

---

## Type Definitions

#### `Move`
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
    pub count: usize,  // How many crates to move
    pub from: usize,   // Source stack (1-indexed in input)
    pub to: usize,     // Destination stack (1-indexed in input)
}
```

**Purpose**: Represent a single crane operation  
**Design Decision**: Keep 1-indexed values from input; convert to 0-indexed when accessing arrays

---

#### `CargoArrangement`
```rust
#[derive(Debug, Clone)]
pub struct CargoArrangement {
    pub stacks: Vec<Vec<char>>,  // Bottom of stack = index 0
    pub moves: Vec<Move>,
}
```

**Purpose**: Complete parsed input state  
**Design Decision**: 
- `Vec<Vec<char>>` natural for stack operations
- Bottom-to-top storage matches Vec's push/pop semantics
- Clone-able to allow both parts to work with fresh state

---

## Algorithm Deep Dive

### Why This Approach Works

**Stack Representation**: `Vec<char>` is perfect for stacks:
- `push()` and `pop()` are O(1)
- End of vector = top of stack (natural mapping)

**Part 1 vs Part 2 Difference**:
- **Part 1**: Individual pop/push operations reverse order
  - Moving [D, N, Z] one-at-a-time: pop D → push D, pop N → push N, pop Z → push Z
  - Result: [Z, N, D] (reversed)
  
- **Part 2**: Bulk operation preserves order
  - Moving [D, N, Z] together: `split_off()` keeps them as [D, N, Z]
  - `extend()` appends in same order: [D, N, Z]

### Complexity Analysis

**Parsing**: O(rows * cols) for ASCII art + O(moves) for instructions  
**Part 1**: O(sum of all move counts) - each individual crate move  
**Part 2**: O(sum of all move counts) - same, but `split_off` + `extend` still iterate

**Overall**: O(n) where n = total crates moved across all operations

### Alternative Approaches

**`VecDeque`**: Could use for double-ended access, but Vec is simpler  
**Linked List**: Poor cache locality, unnecessary overhead  
**String Manipulation**: Could store stacks as strings, but char indexing more natural

---

## Test Cases

### Example Input
```
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
```

**Expected Output**:
- Part 1: `CMZ`
- Part 2: `MCD`

### Edge Cases Handled
- **Empty stacks**: `filter_map(|stack| stack.last())` safely skips
- **Whitespace in ASCII art**: Character position indexing handles variable spacing
- **Bottom-to-top processing**: `rev().skip(1)` correctly builds stacks

---

## Key Insights

1. **ASCII Art Parsing**: Crate positions follow predictable pattern (1, 5, 9, 13...), making column extraction straightforward

2. **Parse-Once Efficiency**: Cloning parsed data for both parts is cheaper than parsing twice (~85µs vs potential 120µs+)

3. **Vec Operations**: `split_off()` is key to preserving order in Part 2 - it removes and returns elements from split point onward as a contiguous slice

4. **1-Indexed Input**: Problem uses 1-indexed stacks; converting to 0-indexed when accessing arrays prevents off-by-one errors

---

## Zettelkasten Links

*Mathematics & Algorithms*:
- [[stack-data-structure]] - LIFO operations, Vec as stack implementation
- [[parsing-patterns]] - ASCII art parsing with character positions
- [[aoc-parsing-patterns]] - Two-section input format (config + instructions)

*Related Problems*:
- This problem demonstrates fundamental stack operations in context of simulation

---

**Navigation**: [← Day 4](day04_function_guide.md) | [Problem](day05.md) | [Summary](../summary_2022.md) | [Day 6 →](day06_function_guide.md)
