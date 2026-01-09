# Day 8: Haunted Wasteland - Function-by-Function Walkthrough

This guide provides a detailed explanation of every type, function, and implementation in the Day 8 solution. Use this as a reference while reading through the code.

---

## 📋 Table of Contents
1. [Overview](#overview)
2. [Type Definitions](#type-definitions)
3. [Core Implementation](#core-implementation)
4. [Mathematical Algorithms](#mathematical-algorithms)
5. [Public API](#public-api)
6. [Design Patterns](#design-patterns)
7. [Performance Analysis](#performance-analysis)

---

## Overview

### Problem Summary
**Part 1**: Navigate from node `AAA` to `ZZZ` following L/R instructions  
**Part 2**: Start at all nodes ending with 'A', find when all simultaneously reach nodes ending with 'Z'

### Key Insights
- **Part 1**: Simple graph traversal - O(n) where n = steps to destination
- **Part 2**: Cycle detection + LCM optimization - brute force would be intractable (8+ trillion steps!)
- **Data structure**: Directed graph with exactly 2 outgoing edges per node (left, right)
- **Mission integration**: HashMap (Mission 5) for O(1) node lookups

### Mathematical Foundation
- **Graph Theory**: Directed graph traversal with labeled edges
- **Number Theory**: GCD (Euclidean algorithm), LCM (cycle alignment), modular arithmetic
- **Zettelkasten**: 
  - [[graph-theory-fundamentals]]
  - [[number-theory-basics]]

---

## Type Definitions

### `struct Network`
**Location**: Lines 36-42  
**Purpose**: Represents the complete navigation network

```rust
#[derive(Debug, Clone)]
struct Network {
    instructions: Vec<char>,                      // L/R navigation sequence
    nodes: HashMap<String, (String, String)>,     // node -> (left_node, right_node)
}
```

**Design Decisions**:

1. **`Vec<char>` for instructions**:
   - Simple, direct representation of "LLR" → ['L', 'L', 'R']
   - Easy indexing: `instructions[idx]`
   - Modular arithmetic for wrapping: `idx % instructions.len()`
   
2. **`HashMap<String, (String, String)>`** for nodes:
   - **Why HashMap?** O(1) lookup by node name (vs O(n) for Vec)
   - **Why String keys?** Node IDs are variable-length strings ("AAA", "11A", etc.)
   - **Why tuple values?** Each node has exactly 2 neighbors (left, right)
   - **Mission 5 integration**: Using HashMap concepts from Mission 5

3. **`#[derive(Clone)]`**:
   - Enables `network.clone()` for testing/debugging
   - Not strictly necessary but helpful for development

**Alternative approaches**:
```rust
// ❌ Vec-based (only works for numeric node IDs)
struct Network {
    nodes: Vec<(usize, usize)>,  // index = node_id, value = (left, right)
}
// Limitation: Can't handle "AAA" string IDs

// ❌ Separate left/right maps
struct Network {
    left_edges: HashMap<String, String>,
    right_edges: HashMap<String, String>,
}
// Problem: Double the lookups, more memory

// ✅ Tuple in single HashMap - clean and efficient
nodes: HashMap<String, (String, String)>
```

---

## Core Implementation

### `Network::parse(input: &str) -> Result<Self>`
**Location**: Lines 51-103  
**Purpose**: Parse the entire puzzle input into a Network structure

**Algorithm**:
```
Input format:
  LLR                    ← instructions (line 1)
                         ← blank line
  AAA = (BBB, CCC)       ← node mappings (remaining lines)
  BBB = (DDD, EEE)
  ...

Steps:
1. Split lines into iterator
2. Parse first line as instructions (chars)
3. Skip blank line
4. For each remaining line:
   a. Parse "AAA = (BBB, CCC)" format
   b. Extract node name ("AAA")
   c. Extract (left, right) pair ("BBB", "CCC")
   d. Insert into HashMap
5. Return Network
```

**Code Walkthrough**:

```rust
fn parse(input: &str) -> Result<Self> {
    let mut lines = input.lines();
    
    // Step 1: Parse instructions
    let instructions: Vec<char> = lines
        .next()                              // Get first line
        .context("Missing instructions line")?  // Error if no first line
        .chars()                             // Convert to chars
        .collect();                          // Collect into Vec
    
    // Step 2: Skip empty line
    lines.next();  // Consume blank line (don't care about return value)
    
    // Step 3: Parse node mappings
    let mut nodes = HashMap::new();
    
    for line in lines {
        if line.trim().is_empty() {
            continue;  // Skip any extra blank lines
        }
        
        // Step 4a: Split "AAA = (BBB, CCC)" on " = "
        let parts: Vec<&str> = line.split(" = ").collect();
        if parts.len() != 2 {
            anyhow::bail!("Invalid line format: {}", line);
        }
        
        let node = parts[0].to_string();  // "AAA"
        
        // Step 4b: Parse "(BBB, CCC)" - remove parens and split
        let lr = parts[1]
            .trim_start_matches('(')    // "(BBB, CCC)" → "BBB, CCC)"
            .trim_end_matches(')')      // "BBB, CCC)" → "BBB, CCC"
            .split(", ")                // "BBB, CCC" → ["BBB", "CCC"]
            .collect::<Vec<_>>();
        
        if lr.len() != 2 {
            anyhow::bail!("Invalid node pair: {}", parts[1]);
        }
        
        // Step 4c: Insert into HashMap
        nodes.insert(node, (lr[0].to_string(), lr[1].to_string()));
    }
    
    Ok(Network { instructions, nodes })
}
```

**Error Handling**:
- Uses `anyhow::Result` for flexible error reporting
- `.context()` adds descriptive messages to errors
- `anyhow::bail!()` returns early with custom error message
- Validates all input constraints (line count, format, etc.)

**Why `.to_string()`?**
```rust
let node = parts[0].to_string();  // Why not just parts[0]?
```
- `parts[0]` is `&str` (borrowed from input)
- HashMap needs owned `String` (can't store borrowed data)
- `.to_string()` creates owned copy

**Example**:
```rust
// Input:
// "LLR
//
// AAA = (BBB, BBB)
// BBB = (AAA, ZZZ)
// ZZZ = (ZZZ, ZZZ)"

let network = Network::parse(input)?;
// network.instructions = ['L', 'L', 'R']
// network.nodes = {
//     "AAA" -> ("BBB", "BBB"),
//     "BBB" -> ("AAA", "ZZZ"),
//     "ZZZ" -> ("ZZZ", "ZZZ"),
// }
```

---

### `Network::navigate(start: &str, end: &str) -> Result<usize>`
**Location**: Lines 105-125  
**Purpose**: Navigate from start node to end node, count steps

**Algorithm**: Simple graph traversal
```
1. Initialize: current = start, steps = 0, instruction_idx = 0
2. While current != end:
   a. Look up (left, right) neighbors of current node
   b. Follow instruction (L or R) to next node
   c. Increment steps
   d. Advance instruction index (with wrapping)
3. Return steps
```

**Code Walkthrough**:

```rust
fn navigate(&self, start: &str, end: &str) -> Result<usize> {
    let mut current = start.to_string();  // Current position
    let mut steps = 0;                     // Step counter
    let mut instruction_idx = 0;           // Which instruction we're on
    
    while current != end {
        // Step 1: Get neighbors from HashMap - O(1) lookup
        let (left, right) = self
            .nodes
            .get(&current)
            .context(format!("Node {} not found", current))?;
        
        // Step 2: Follow instruction
        let instruction = self.instructions[instruction_idx];
        current = match instruction {
            'L' => left.clone(),   // Go left
            'R' => right.clone(),  // Go right
            _ => anyhow::bail!("Invalid instruction: {}", instruction),
        };
        
        // Step 3: Update counters
        steps += 1;
        
        // Step 4: Wrap instruction index (modular arithmetic!)
        instruction_idx = (instruction_idx + 1) % self.instructions.len();
    }
    
    Ok(steps)
}
```

**Key Concepts**:

1. **Cyclic instructions**: 
   ```rust
   instruction_idx = (instruction_idx + 1) % self.instructions.len()
   ```
   - Instructions repeat: "LLR" → L, L, R, L, L, R, L, L, R, ...
   - Modulo ensures we wrap: `idx % 3` gives [0,1,2,0,1,2,...]
   - See [[number-theory-basics]] for modular arithmetic

2. **Graph traversal**:
   - Deterministic: Always follow same edge for same instruction
   - Not a search: Know exactly which edge to take
   - Termination: Assumes path exists (problem guarantees this)

3. **Why clone?**
   ```rust
   current = left.clone();  // Why not current = left?
   ```
   - `left` is `&String` (borrowed from HashMap)
   - `current` needs owned `String` (outlives the borrow)
   - `.clone()` creates independent owned copy

**Complexity**:
- **Time**: O(n) where n = steps to destination
- **Space**: O(1) - only storing current state

**Example**:
```rust
// Network: AAA=(BBB,CCC), BBB=(DDD,EEE), CCC=(ZZZ,GGG), ZZZ=(ZZZ,ZZZ)
// Instructions: "RL"

navigate("AAA", "ZZZ")
// Step 0: current="AAA", instruction='R' → current="CCC"
// Step 1: current="CCC", instruction='L' → current="ZZZ"
// Result: 2 steps
```

---

### `Network::navigate_until_suffix(start: &str, suffix: char) -> Result<usize>`
**Location**: Lines 127-149  
**Purpose**: Navigate until reaching ANY node ending with suffix

**Difference from `navigate()`**:
```rust
// navigate(): Exact match
while current != end { ... }

// navigate_until_suffix(): Pattern match
while !current.ends_with(suffix) { ... }
```

**Why needed?** Part 2 requirement:
- Don't know exact destination node
- Just know it ends with 'Z'
- Examples: "AAZ", "11Z", "XXZ" all valid

**Use case**: Finding cycle length for ghost paths
```rust
// Ghost starts at "11A"
// Finds cycle length to ANY node ending with 'Z'
let cycle_length = network.navigate_until_suffix("11A", 'Z')?;
```

**Otherwise identical** to `navigate()` - same O(n) traversal logic

---

### `Network::find_nodes_ending_with(suffix: char) -> Vec<String>`
**Location**: Lines 151-157  
**Purpose**: Find all node names ending with specific character

**Algorithm**: Simple filter
```rust
fn find_nodes_ending_with(&self, suffix: char) -> Vec<String> {
    self.nodes              // HashMap<String, (String, String)>
        .keys()             // Iterator over keys (node names)
        .filter(|node| node.ends_with(suffix))  // Keep if ends with suffix
        .cloned()           // Clone String (keys are &String)
        .collect()          // Collect into Vec
}
```

**Example**:
```rust
// Nodes: "AAA", "11A", "22A", "ZZZ", "11Z", "22Z"
find_nodes_ending_with('A')  // ["AAA", "11A", "22A"]
find_nodes_ending_with('Z')  // ["ZZZ", "11Z", "22Z"]
```

**Why needed?** Part 2 start condition:
- "Start at all nodes ending with 'A'"
- Don't know how many ghost start positions
- This function finds them all

**Complexity**: O(n) where n = number of nodes

---

### `Network::navigate_ghosts() -> Result<usize>`
**Location**: Lines 159-179  
**Purpose**: **THE CORE ALGORITHM** - Find when all ghost paths synchronize

**Problem**: Brute force simulation is intractable
```rust
// ❌ What we CAN'T do (8+ trillion steps!)
loop {
    for ghost in &mut ghosts {
        ghost.step();  // Move each ghost one step
    }
    total_steps += 1;
    if ghosts.iter().all(|g| g.at_goal()) {
        return total_steps;  // Never reaches here in reasonable time!
    }
}
```

**Solution**: Cycle detection + LCM
```
Key Insight: Each ghost follows a repeating cycle
- Ghost 1: Reaches goal every N₁ steps
- Ghost 2: Reaches goal every N₂ steps
- Ghost k: Reaches goal every Nₖ steps

Question: When do ALL cycles align?
Answer: LCM(N₁, N₂, ..., Nₖ)
```

**Algorithm**:
```
1. Find all starting nodes (ending with 'A')
2. For each start node:
   a. Find cycle length (steps to reach node ending with 'Z')
   b. Store in cycle_lengths array
3. Compute LCM of all cycle lengths
4. Return result
```

**Code Walkthrough**:

```rust
fn navigate_ghosts(&self) -> Result<usize> {
    // Step 1: Find all ghost starting positions
    let start_nodes = self.find_nodes_ending_with('A');
    
    if start_nodes.is_empty() {
        anyhow::bail!("No starting nodes found (nodes ending with 'A')");
    }
    
    // Step 2: Find cycle length for each ghost
    let mut cycle_lengths = Vec::new();
    for start in &start_nodes {
        let steps = self.navigate_until_suffix(start, 'Z')?;
        cycle_lengths.push(steps);
    }
    
    // Step 3: Compute LCM of all cycle lengths
    let result = cycle_lengths.iter().fold(1, |acc, &x| lcm(acc, x));
    
    Ok(result)
}
```

**Why this works**:

Consider 2 ghosts:
```
Ghost 1: Reaches goal at steps 4, 8, 12, 16, 20, 24, ...
Ghost 2: Reaches goal at steps 6, 12, 18, 24, 30, ...

Both at goal: step 12, step 24, step 36, ...
Pattern: Every LCM(4, 6) = 12 steps
```

Mathematical proof:
- If ghost G reaches goal every Nᵢ steps
- Then G is at goal at steps: Nᵢ, 2Nᵢ, 3Nᵢ, ... (multiples of Nᵢ)
- All ghosts at goal ⟺ step is multiple of all Nᵢ
- Smallest such step = LCM(N₁, N₂, ..., Nₖ)

**Performance**:
```rust
// Part 2 puzzle: 6 ghosts with different cycle lengths
// Cycle lengths: [19637, 20093, 16343, 18113, 12361, 15989]
// LCM = 8,811,050,362,409

// Brute force: Would need 8.8 trillion steps (years of computation!)
// LCM approach: ~6.7ms (instant!)
```

**Edge Cases**:
- No starting nodes → Error (bail early)
- One starting node → Returns that cycle length (LCM of 1 number = itself)
- Cycles of length 1 → Works correctly (node already at goal)

---

## Mathematical Algorithms

### `fn gcd(a: usize, b: usize) -> usize`
**Location**: Lines 182-189  
**Purpose**: Compute Greatest Common Divisor using Euclidean algorithm

**Algorithm**: 3000+ year old algorithm!
```
Euclidean Algorithm:
  gcd(a, b) = gcd(b, a mod b)
  gcd(a, 0) = a

Example: gcd(48, 18)
  gcd(48, 18) = gcd(18, 48 mod 18) = gcd(18, 12)
  gcd(18, 12) = gcd(12, 18 mod 12) = gcd(12, 6)
  gcd(12, 6)  = gcd(6,  12 mod 6)  = gcd(6,  0)
  gcd(6,  0)  = 6  ✓
```

**Code Walkthrough**:

```rust
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;      // Save b
        b = a % b;         // New b = remainder of a/b
        a = temp;          // New a = old b
    }
    a  // When b=0, a is the GCD
}
```

**Why it works**:

Mathematical property:
```
Any common divisor of (a, b) is also a divisor of (b, a mod b)

Proof sketch:
  If d divides both a and b
  Then a = k₁×d and b = k₂×d for some integers k₁, k₂
  Then a mod b = (k₁×d) mod (k₂×d) = (k₁ mod k₂)×d
  Therefore d divides (a mod b)
  
So: gcd(a, b) = gcd(b, a mod b)
```

**Complexity**:
- **Time**: O(log min(a, b)) - very fast!
- **Proof**: Each iteration reduces problem size by at least half
- **Example**: gcd(1000000, 999999) takes only ~40 iterations

**Why `mut`?**
```rust
fn gcd(mut a: usize, mut b: usize) -> usize {
    //     ^^^ keyword
```
- Need to modify `a` and `b` in loop
- Take ownership (passed by value, not reference)
- Doesn't affect caller's variables

**Test Cases**:
```rust
assert_eq!(gcd(48, 18), 6);    // Classic example
assert_eq!(gcd(17, 5), 1);     // Coprime (no common divisor)
assert_eq!(gcd(100, 50), 50);  // One divides the other
assert_eq!(gcd(0, 5), 5);      // Zero case
```

---

### `fn lcm(a: usize, b: usize) -> usize`
**Location**: Lines 191-198  
**Purpose**: Compute Least Common Multiple using GCD

**Mathematical Formula**:
```
lcm(a, b) = (a × b) / gcd(a, b)

Relationship: a × b = gcd(a, b) × lcm(a, b)
```

**Why this formula?**

Visual example with 12 and 18:
```
12 = 2² × 3
18 = 2 × 3²

gcd(12, 18) = 2 × 3 = 6         (minimum powers)
lcm(12, 18) = 2² × 3² = 36      (maximum powers)

Verify: 12 × 18 = 216 = 6 × 36 ✓
```

**Code Walkthrough**:

```rust
fn lcm(a: usize, b: usize) -> usize {
    // Edge case: LCM of 0 is 0
    if a == 0 || b == 0 {
        return 0;
    }
    
    // Formula: lcm = (a × b) / gcd(a, b)
    (a * b) / gcd(a, b)
}
```

**Why check for zero?**
```rust
if a == 0 || b == 0 { return 0; }
```
- Avoid division by zero (if gcd(0,0) called)
- Mathematical: lcm(0, n) = 0 for any n
- Edge case in our problem: No cycles of length 0

**Overflow risk?**
```rust
(a * b) / gcd(a, b)
//  ↑ Could overflow!
```
- `a * b` might overflow `usize` before division
- For our puzzle: Cycle lengths ~20K, product ~400M (safe)
- **Better formula** (for huge numbers):
  ```rust
  a / gcd(a, b) * b  // Divide first to reduce overflow risk
  ```

**Multi-way LCM**:
```rust
// Compute LCM of multiple numbers using fold
let result = values.iter().fold(1, |acc, &x| lcm(acc, x));

// Example: lcm(4, 6, 10)
// Step 1: lcm(1, 4)  = 4
// Step 2: lcm(4, 6)  = 12
// Step 3: lcm(12, 10) = 60
```

**Why start fold with 1?**
```rust
.fold(1, |acc, &x| lcm(acc, x))
//    ↑ identity element
```
- lcm(1, n) = n for any n
- 1 is the identity for LCM (like 0 for addition)

**Test Cases**:
```rust
assert_eq!(lcm(4, 6), 12);      // 4×3 = 6×2 = 12
assert_eq!(lcm(21, 6), 42);     // 21×2 = 6×7 = 42  
assert_eq!(lcm(12, 18), 36);    // From GCD example
```

---

## Public API

### `solve_part1(input: &str) -> Result<String>`
**Location**: Lines 200-204  
**Purpose**: Main entry point for Part 1

**Algorithm**:
```
1. Parse input → Network
2. Navigate from "AAA" to "ZZZ"
3. Return step count as string
```

**Code**:
```rust
pub fn solve_part1(input: &str) -> Result<String> {
    let network = Network::parse(input)?;        // Parse
    let steps = network.navigate("AAA", "ZZZ")?; // Navigate
    Ok(steps.to_string())                        // Format
}
```

**Simple and direct** - no tricks, just straightforward implementation

**Example**:
```
Input: "RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\n..."
Output: "19637"
```

---

### `solve_part2(input: &str) -> Result<String>`
**Location**: Lines 206-210  
**Purpose**: Main entry point for Part 2

**Algorithm**:
```
1. Parse input → Network
2. Find ghost synchronization via LCM
3. Return step count as string
```

**Code**:
```rust
pub fn solve_part2(input: &str) -> Result<String> {
    let network = Network::parse(input)?;     // Parse
    let steps = network.navigate_ghosts()?;   // LCM magic!
    Ok(steps.to_string())                     // Format
}
```

**All complexity hidden** in `navigate_ghosts()` - clean API

**Example**:
```
Input: Same as Part 1
Output: "8811050362409"  (8.8 trillion!)
```

---

## Design Patterns

### Pattern 1: HashMap for Graph Adjacency
**Where**: `Network::nodes`

**Why HashMap over Vec?**

| Aspect | Vec<Vec<usize>> | HashMap<String, (String, String)> |
|--------|-----------------|-----------------------------------|
| Node IDs | Must be 0..n integers | Any string (flexible!) |
| Lookup | O(1) by index | O(1) by key |
| Sparse graphs | Wastes space | Only stores existing nodes |
| Our use case | ❌ IDs are strings | ✅ Perfect fit |

**Pattern**:
```rust
// Build adjacency list
let mut graph: HashMap<String, Vec<String>> = HashMap::new();
graph.insert("A".to_string(), vec!["B".to_string(), "C".to_string()]);

// Our variant: Fixed 2 neighbors
let mut graph: HashMap<String, (String, String)> = HashMap::new();
graph.insert("A".to_string(), ("B".to_string(), "C".to_string()));
```

**Mission 5 Integration**: Uses HashMap concepts from Mission 5
- O(1) insertion and lookup
- Flexible key types (String, not just integers)
- Entry API for efficient updates (not used here, but available)

---

### Pattern 2: Modular Arithmetic for Cyclic Sequences
**Where**: `navigate()` instruction wrapping

**Pattern**:
```rust
// Repeat sequence cyclically
let idx = (idx + 1) % sequence.len();

// Example: "LLR" (length 3)
// idx: 0 → 1 → 2 → 0 → 1 → 2 → 0 → ...
```

**Why this works**:
- `n % m` gives remainder in range [0, m-1]
- Perfect for wrapping array indices
- No conditional logic needed

**Alternative (worse)**:
```rust
// ❌ Manual wrapping
idx += 1;
if idx >= instructions.len() {
    idx = 0;
}

// ✅ Modular arithmetic
idx = (idx + 1) % instructions.len();
```

**Applications**:
- Circular buffers
- Repeating patterns
- Clock arithmetic
- Cyclic iteration

See [[number-theory-basics]] for modular arithmetic theory

---

### Pattern 3: Cycle Detection for Periodic Processes
**Where**: Part 2 ghost navigation

**General Pattern**:
```
Problem: Find when N periodic processes synchronize
  Process 1: Repeats every T₁ steps
  Process 2: Repeats every T₂ steps
  ...
  Process N: Repeats every Tₙ steps

Solution: LCM(T₁, T₂, ..., Tₙ)
```

**Why LCM?**
- Each process at goal at multiples of its period
- All at goal ⟺ step is multiple of ALL periods
- Smallest such step = LCM

**Code Pattern**:
```rust
// 1. Find period for each process
let periods: Vec<usize> = processes
    .iter()
    .map(|p| find_period(p))
    .collect();

// 2. Compute LCM of all periods
let sync_point = periods.iter()
    .fold(1, |acc, &p| lcm(acc, p));
```

**Real-world applications**:
- Traffic light synchronization
- Planetary alignments
- Musical beats/rhythms
- CPU scheduling cycles

---

### Pattern 4: Greedy Optimization via Mathematical Proof
**Where**: Part 2 LCM approach

**Anti-pattern**: Brute force simulation
```rust
// ❌ Simulate every step (intractable)
while !all_at_goal {
    step_all();
    steps += 1;
}
```

**Pattern**: Find mathematical shortcut
```rust
// ✅ Recognize the pattern, compute directly
let answer = mathematical_formula();
```

**How to recognize**:
1. **Huge numbers**: Answer in trillions suggests brute force won't work
2. **Periodic behavior**: Cycles, patterns, repetition
3. **Synchronization**: "When do all X align?"
4. **Mathematical structure**: Graph theory, number theory, geometry

**Day 8 insight**:
- Problem designed to be impossible by brute force
- Forces you to understand the mathematics
- Reward: 8+ trillion steps computed in milliseconds!

---

### Pattern 5: Separation of Concerns
**Where**: Network struct + free functions

**Design**:
```rust
struct Network {
    // Data only
}

impl Network {
    // Network-specific operations
    fn navigate(...) { }
    fn parse(...) { }
}

// Separate: General mathematical functions
fn gcd(...) { }
fn lcm(...) { }
```

**Why separate?**
- `gcd()` and `lcm()` are general-purpose
- Not specific to networks/graphs
- Easier to test independently
- Could be moved to `src/patterns/number_theory.rs` if used elsewhere

**Alternative (worse)**:
```rust
impl Network {
    fn gcd(&self, a: usize, b: usize) -> usize { }
    //           ^^^^ Doesn't use self!
}
```
- Methods should use `self`
- Free functions for pure logic

---

## Performance Analysis

### Time Complexity

**Part 1: navigate()**
```
Complexity: O(n)
  where n = number of steps to destination
  
Our input: n ≈ 19,637 steps
Runtime: ~1.5ms

Operations:
  - HashMap lookup: O(1) × n
  - String clone: O(k) × n  where k = avg string length
  - Total: O(n×k)
```

**Part 2: navigate_ghosts()**
```
Complexity: O(k × m + k × log M)
  where k = number of ghosts
        m = average cycle length
        M = maximum LCM intermediate value
  
Our input: k = 6 ghosts, m ≈ 17,000 steps
Runtime: ~6.7ms

Breakdown:
  1. Find all start nodes: O(n)  where n = total nodes
  2. Find each cycle: O(k × m)
     - 6 ghosts × ~17K steps each ≈ 100K lookups
  3. Compute LCM: O(k × log M)
     - 6 numbers, each GCD is O(log M)
     - Total: 6 × log(8.8 trillion) ≈ 6 × 44 = 264 ops
  
Total: O(100K) dominated by cycle detection
```

**GCD: Euclidean Algorithm**
```
Complexity: O(log min(a, b))

Proof (Fibonacci numbers give worst case):
  - Each step reduces problem by factor ≥ φ ≈ 1.618
  - After k steps: b ≤ min(a,b) / φᵏ
  - When b = 0: k ≈ log_φ(min(a,b)) = O(log min(a,b))

Example: gcd(987, 610)  (consecutive Fibonacci numbers)
  Takes 15 iterations (near worst case)
  log₂(610) ≈ 9.25, so reasonable!
```

### Space Complexity

```
Network storage:
  - Instructions: O(I)  where I = instruction length (~300 chars)
  - Nodes: O(N × L)  where N = nodes, L = avg string length
  - Total: O(N × L)
  
Our input: N ≈ 700 nodes, L ≈ 3 chars
Space: ~2KB for HashMap (plus overhead)

navigate():
  - O(1) variables (current, steps, idx)
  - O(L) for current string clone
  
navigate_ghosts():
  - O(k) for cycle_lengths Vec
  - k = 6 ghosts → 48 bytes
```

**Key insight**: Space efficient!
- No need to store entire path
- Only current state matters
- HashMap size dominates, but reasonable

### Actual Performance

**Criterion Benchmarks**:
```
day08_part1: 1.5292ms ± 0.0078ms
  - 19,637 steps
  - ~78 nanoseconds per step
  
day08_part2: 6.6832ms ± 0.0186ms
  - 6 cycles found
  - LCM of 6 numbers
  - Equivalent to 8.8 trillion simulated steps!
```

**Optimization Opportunities**:
1. **String cloning**: Could use `Cow<str>` or indices
2. **Cycle caching**: Memoize if solving multiple times
3. **SIMD**: Batch ghost updates (but LCM already instant)

**But**: Current performance excellent - no optimization needed!

---

## Testing Strategy

### Unit Tests
**Location**: Lines 212-277

**Coverage**:

1. **Parsing**:
   ```rust
   #[test]
   fn test_parse_network() {
       // Verify instructions parsed correctly
       // Verify all nodes present
       // Verify correct neighbors
   }
   ```

2. **Navigation**:
   ```rust
   #[test]
   fn test_part1_example1() {
       // AAA → ZZZ in 2 steps
   }
   
   #[test]
   fn test_part1_example2() {
       // AAA → ZZZ in 6 steps (longer path)
   }
   ```

3. **Mathematical Functions**:
   ```rust
   #[test]
   fn test_gcd() {
       assert_eq!(gcd(48, 18), 6);
       assert_eq!(gcd(17, 5), 1);   // Coprime
       assert_eq!(gcd(100, 50), 50); // One divides other
   }
   
   #[test]
   fn test_lcm() {
       assert_eq!(lcm(4, 6), 12);
       assert_eq!(lcm(21, 6), 42);
   }
   ```

4. **Ghost Navigation**:
   ```rust
   #[test]
   fn test_part2_example() {
       // 2 ghosts synchronize after 6 steps
   }
   ```

### Test Philosophy

**Example-driven**:
- Use puzzle examples as test cases
- Validates algorithm correctness
- Catches edge cases

**Mathematical validation**:
- Known GCD/LCM values
- Verifies mathematical properties
- Coprime, divisibility cases

**Edge cases**:
- Empty input (should error)
- Invalid format (should error)
- Single node (trivial case)
- All nodes already at goal

---

## Common Pitfalls & Solutions

### Pitfall 1: String Ownership
```rust
// ❌ Borrow checker error
let (left, right) = nodes.get(&current)?;
current = left;  // ERROR: Can't store borrowed value

// ✅ Clone to own
current = left.clone();  // Create owned copy
```

**Why**: HashMap owns the strings, we need independent ownership

---

### Pitfall 2: Integer Overflow
```rust
// ❌ Might overflow
let lcm = (a * b) / gcd(a, b);

// ✅ Better for huge numbers
let lcm = a / gcd(a, b) * b;  // Divide first
```

**Why**: Product might exceed `usize::MAX` before division

Our puzzle is safe (20K × 20K < 2^64), but good practice!

---

### Pitfall 3: Off-by-One Errors
```rust
// ❌ Wrong modulo
instruction_idx = (instruction_idx + 1) % (instructions.len() + 1);

// ✅ Correct
instruction_idx = (instruction_idx + 1) % instructions.len();
```

**Why**: Length 3 → valid indices are [0,1,2], so mod 3, not mod 4

---

### Pitfall 4: Forgetting Edge Cases
```rust
// ❌ What if already at goal?
let steps = navigate_until_suffix(start, 'Z')?;

// ✅ Algorithm handles it correctly
// If start.ends_with('Z'), loop never executes, returns 0
```

**Our implementation**: Naturally handles this case!

---

## Key Takeaways

### 1. **HashMap for Flexible Graphs**
- Use when node IDs aren't 0..n integers
- O(1) lookup by string key
- Mission 5 integration

### 2. **Mathematical Shortcuts > Brute Force**
- Recognize periodic behavior → cycle detection
- Synchronization problems → LCM
- 8 trillion steps in 7ms!

### 3. **Modular Arithmetic for Cycles**
- `idx = (idx + 1) % len` for wrapping
- No conditionals needed
- Clean and efficient

### 4. **GCD via Euclidean Algorithm**
- 3000 year old algorithm, still optimal!
- O(log min(a,b)) complexity
- Foundation for LCM

### 5. **LCM for Cycle Alignment**
- When do periodic processes synchronize?
- Answer: LCM of their periods
- Mathematical proof guarantees correctness

### 6. **Graph Theory Fundamentals**
- Directed graph with labeled edges
- Deterministic traversal (not search)
- [[graph-theory-fundamentals]]

### 7. **Number Theory Fundamentals**
- GCD, LCM, modular arithmetic
- Practical applications in algorithms
- [[number-theory-basics]]

---

## Follow-Up Questions

As you read the code, consider:

1. **What if nodes had 3 outgoing edges?** How would the data structure change?

2. **Could we solve Part 1 using Part 2's algorithm?** (Yes! Single ghost, same logic)

3. **What if cycles had offsets?** (Ghost reaches goal at 10, 25, 40, ... not 0, 15, 30, ...)

4. **How would you detect if a path doesn't exist?** (Cycle detection without reaching goal)

5. **Could you parallelize ghost cycle detection?** (Yes! Each ghost independent)

6. **What if LCM overflows `usize`?** (Use `BigInt` or detect/handle overflow)

7. **Why does the puzzle guarantee cycles exist?** (Graph structure + infinite instructions)

---

## Extensions & Challenges

### Challenge 1: Cycle Detection with Offset
Modify to handle: "Ghost reaches goal at steps 10, 25, 40, 55, ..."
- Cycle length: 15
- Offset: 10
- Formula: offset + k × period
- Synchronization: More complex (Chinese Remainder Theorem!)

### Challenge 2: Shortest Path Variant
Instead of following instructions, find SHORTEST path AAA → ZZZ
- Use BFS (Mission 8!)
- Ignore instructions
- Just graph structure

### Challenge 3: Visualization
Draw the graph structure:
- Nodes as vertices
- Left/right edges labeled
- Highlight path taken
- Animate ghost movement

### Challenge 4: General Cycle Detection
Detect if ANY cycle exists (not just goal-reaching):
- Floyd's cycle detection (tortoise and hare)
- Brent's algorithm
- Store visited states

---

**Next Steps**: 
1. Implement the extensions above
2. Read [[graph-theory-fundamentals]] for deeper understanding
3. Read [[number-theory-basics]] for GCD/LCM proofs
4. Try AoC 2023 Day 10 for more graph practice!
