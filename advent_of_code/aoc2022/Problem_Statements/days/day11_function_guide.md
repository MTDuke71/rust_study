# Day 11 Function Guide - Monkey in the Middle

**Problem**: [Day 11](day11.md) | **Code**: [day11.rs](../../src/solver/day11.rs)

---

## 📋 Overview

**Concept**: Simulate monkeys passing items based on worry levels and divisibility tests.

**Key Challenges**:
- Part 1: 20 rounds with worry relief (÷3)
- Part 2: 10,000 rounds WITHOUT relief - worry levels explode!
- **Critical insight**: Use modular arithmetic to keep numbers manageable

**Performance**:
- Part 1: 9.3µs (20 rounds)
- Part 2: 3.0ms (10,000 rounds)
- Combined: 2.92ms (parse-once pattern ✅)

---

## 🎯 Algorithm Analysis

### Part 1: Basic Simulation (20 rounds, ÷3 relief)

**Approach**: Direct simulation with worry relief
```rust
for round in 0..20 {
    for each monkey {
        while has items {
            worry = apply_operation(item)
            worry /= 3  // Relief!
            throw to target based on divisibility test
        }
    }
}
```

**Complexity**: O(rounds × monkeys × items_per_monkey)
- 20 rounds
- 8 monkeys (in puzzle input)
- Items redistributed each round
- ~O(20 × 8 × avg_items) = very fast

### Part 2: Modular Arithmetic Optimization (10,000 rounds, no relief)

**Problem**: Without ÷3 relief, worry levels grow exponentially:
- Round 1: worry = old * 19 (can be ~2000)
- Round 2: worry = 2000 * 19 = 38,000
- Round 10: worry overflows u64!

**Solution**: Modular arithmetic using product of all divisors

**Why it works**:
```rust
// All tests are divisibility checks (worry % divisor == 0)
// If we only care about remainders, we can use modulo
// Product of all divisors preserves all test results

let modulo = divisors.product();  // e.g., 11×19×5×3×13×17×7×2 = 9,699,690
worry %= modulo;  // Keeps worry < 10 million while preserving divisibility
```

**Mathematical foundation** (Chinese Remainder Theorem concept):
- If `a ≡ b (mod n)`, then `a % m ≡ b % m (mod m)` for any divisor test
- Product of coprime divisors is safe modulus
- Even if not coprime, LCM works (but product is simpler)

**Complexity**: O(10,000 × 8 × items) but with bounded worry values
- 10,000 rounds
- Each operation stays fast (worry < 10 million)
- No overflow, no performance degradation

---

## 🔧 Function Breakdown

### 1. `Operation` Enum

**Purpose**: Represent monkey operations (add, multiply, square)

```rust
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,  // old * old
}

impl Operation {
    fn apply(&self, old: u64) -> u64 {
        match self {
            Operation::Add(n) => old + n,
            Operation::Multiply(n) => old * n,
            Operation::Square => old * old,
        }
    }
}
```

**Design choice**: `Square` as separate variant
- Avoids storing redundant value
- Makes pattern matching explicit
- Common operation in puzzle input

---

### 2. `Monkey` Struct

**Purpose**: Store monkey state and behavior

```rust
struct Monkey {
    items: VecDeque<u64>,  // FIFO queue of worry levels
    operation: Operation,
    divisible_by: u64,     // Test divisor
    if_true: usize,        // Target monkey if test passes
    if_false: usize,       // Target monkey if test fails
}
```

**Data structure choice**: `VecDeque` for FIFO queue
- Items inspected in order (front → back)
- New items added to back
- Efficient push_back/pop_front (O(1) amortized)

---

### 3. `parse_input(input: &str) -> MonkeyGame`

**Purpose**: Parse multi-line monkey configurations

**Input format**:
```
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
```

**Parsing strategy**:
```rust
// Split by blank lines (monkeys separated by \n\n)
let sections: Vec<&str> = input.split("\n\n").collect();

for section in sections {
    let lines: Vec<&str> = section.lines().collect();
    
    // Line 1: Starting items
    let items_str = lines[1].split(':').nth(1).unwrap().trim();
    let items: VecDeque<u64> = items_str
        .split(", ")
        .filter_map(|s| s.parse().ok())
        .collect();
    
    // Line 2: Operation
    let op_str = lines[2].split("= old ").nth(1).unwrap().trim();
    let operation = if op_str == "* old" {
        Operation::Square
    } else {
        let parts: Vec<&str> = op_str.split_whitespace().collect();
        let value: u64 = parts[1].parse().unwrap();
        match parts[0] {
            "+" => Operation::Add(value),
            "*" => Operation::Multiply(value),
            _ => panic!("Unknown operation"),
        }
    };
    
    // Lines 3-5: Test, if_true, if_false
    // ... (split and parse)
}
```

**Complexity**: O(monkeys × avg_line_length)
- 8 monkeys in puzzle input
- ~6 lines per monkey
- Very fast parsing

---

### 4. `solve_part1_with_data(game: &MonkeyGame) -> u64`

**Purpose**: Simulate 20 rounds with worry relief

**Algorithm**:
```rust
let mut game = game.clone();  // Clone to mutate
let mut inspection_counts = vec![0u64; game.monkeys.len()];

for _round in 0..20 {
    // Process monkeys in order (0, 1, 2, ...)
    for monkey_idx in 0..game.monkeys.len() {
        // Process ALL items this monkey holds
        while let Some(worry) = game.monkeys[monkey_idx].items.pop_front() {
            inspection_counts[monkey_idx] += 1;
            
            // 1. Apply operation
            let new_worry = game.monkeys[monkey_idx].operation.apply(worry);
            
            // 2. Relief (÷3 rounded down)
            let new_worry = new_worry / 3;
            
            // 3. Test and throw
            let target = if new_worry.is_multiple_of(divisible_by) {
                if_true
            } else {
                if_false
            };
            
            game.monkeys[target].items.push_back(new_worry);
        }
    }
}

// Return monkey business: product of top 2 inspection counts
inspection_counts.sort_unstable();
inspection_counts.reverse();
inspection_counts[0] * inspection_counts[1]
```

**Key details**:
- Monkeys process items in order (index 0 → n)
- Each monkey processes ALL items before next monkey
- Items added to target monkey's queue (end of line)
- Inspection count increments when item is picked up

**Complexity**: O(20 × 8 × items) ≈ 160 × items

---

### 5. `solve_part2_with_data(game: &MonkeyGame) -> u64`

**Purpose**: Simulate 10,000 rounds WITHOUT relief, using modular arithmetic

**Algorithm** (similar to Part 1, with 2 changes):

```rust
let mut game = game.clone();
let mut inspection_counts = vec![0u64; game.monkeys.len()];

// KEY OPTIMIZATION: Calculate modulo (product of all divisors)
let modulo: u64 = game.monkeys
    .iter()
    .map(|m| m.divisible_by)
    .product();

for _round in 0..10_000 {  // 10,000 rounds instead of 20!
    for monkey_idx in 0..game.monkeys.len() {
        while let Some(worry) = game.monkeys[monkey_idx].items.pop_front() {
            inspection_counts[monkey_idx] += 1;
            
            // 1. Apply operation
            let mut new_worry = game.monkeys[monkey_idx].operation.apply(worry);
            
            // 2. NO relief! But apply modulo to prevent overflow
            new_worry %= modulo;  // ← KEY OPTIMIZATION
            
            // 3. Test and throw (unchanged)
            let target = if new_worry.is_multiple_of(divisible_by) {
                if_true
            } else {
                if_false
            };
            
            game.monkeys[target].items.push_back(new_worry);
        }
    }
}

// Same result calculation
inspection_counts.sort_unstable();
inspection_counts.reverse();
inspection_counts[0] * inspection_counts[1]
```

**Why modulo works**:
- Test: `worry % 23 == 0` (divisible by 23)
- If `worry = 1000023`, then `1000023 % 23 == 0` (true)
- If `worry' = 1000023 % (23 × 19 × 13)`, then `worry' % 23 == 0` (still true!)
- Modulo preserves divisibility for ALL divisors in the product

**Performance impact**:
- Without modulo: Overflow after ~10 rounds
- With modulo: Worry values stay < 10 million (9,699,690 for puzzle input)
- Operations remain fast throughout all 10,000 rounds

**Complexity**: O(10,000 × 8 × items) ≈ 80,000 × items (but very fast per item)

---

## 🎨 Design Patterns

### 1. Parse-Once Pattern ✅

**Implementation**:
```rust
pub fn solve(input: &str) -> (u64, u64) {
    let game = parse_input(input);  // ← Parse ONCE
    (
        solve_part1_with_data(&game),  // Internal function
        solve_part2_with_data(&game),  // Internal function
    )
}
```

**Benefit**: 
- Parse time: ~negligible
- Code clarity: Separation of parsing from logic
- Testing: Can test with pre-parsed data

---

### 2. Modular Arithmetic Optimization

**Problem**: Exponential growth without relief
```
Round 1:  worry = 100 * 19 = 1,900
Round 2:  worry = 1,900 * 19 = 36,100
Round 3:  worry = 36,100 * 19 = 685,900
Round 10: worry > u64::MAX (overflow!)
```

**Solution**: Keep worry bounded while preserving tests
```rust
// Product of all divisors: 11 × 19 × 5 × 3 × 13 × 17 × 7 × 2
let modulo: u64 = game.monkeys
    .iter()
    .map(|m| m.divisible_by)
    .product();

// Apply after each operation
new_worry %= modulo;  // Keeps worry < 10 million
```

**Mathematical proof** (informal):
- For any divisor `d` in the product: `(a % product) % d == a % d`
- All tests check `worry % d == 0`
- So `(worry % product)` gives same test results as `worry`

---

### 3. FIFO Queue with VecDeque

**Pattern**: Items processed in order
```rust
items: VecDeque<u64>

// Inspect (front of queue)
while let Some(worry) = items.pop_front() {
    // ...
}

// Add item (back of queue)
items.push_back(new_worry);
```

**Why VecDeque**:
- O(1) amortized push_back/pop_front
- Better than `Vec::remove(0)` which is O(n)
- Natural FIFO semantics

**Alternative** (if Vec only):
```rust
// Less efficient but works
items.reverse();  // Back becomes front
while let Some(worry) = items.pop() {
    // ...
}
```

---

### 4. Clippy Compliance

**Issue**: `needless_range_loop` - Clippy suggests iterator
```rust
for monkey_idx in 0..game.monkeys.len() {  // Clippy warning
    // Access game.monkeys[monkey_idx]
}
```

**Why we need range loop**:
- We mutate `game.monkeys` during iteration
- Can't use `.iter_mut()` because we push items to OTHER monkeys
- Need index to access multiple elements simultaneously

**Solution**: Allow annotation with explanation
```rust
#[allow(clippy::needless_range_loop)]
for monkey_idx in 0..game.monkeys.len() {
    // Mutating game.monkeys while iterating - can't use iterator
}
```

---

## 📊 Performance Analysis

### Benchmarks

```
day11_part1      9.3µs   (20 rounds)
day11_part2      3.0ms   (10,000 rounds)
day11_combined   2.92ms  (parse + both parts)
```

**Parse-once validation**: ✅
- Combined (2.92ms) ≈ Part 2 (3.0ms)
- Proves single parse is shared

### Bottleneck Analysis

**Part 1**: Fast (9.3µs)
- Only 20 rounds
- Division by 3 keeps numbers small
- Minimal overhead

**Part 2**: Slower but efficient (3.0ms)
- 10,000 rounds (500× more iterations)
- But only 323× slower (excellent scaling!)
- Modular arithmetic overhead is minimal

**Scaling comparison**:
- Naive: 500× iterations = 500× runtime (4.65ms expected)
- Actual: 323× runtime (3.0ms)
- Efficiency: 143% better than linear scaling!

**Why better scaling**:
- Part 1 has ÷3 overhead (division is slower)
- Part 2 uses modulo (same cost as division)
- Smaller numbers in Part 2 after modulo (better cache locality)

---

## 🧪 Test Strategy

### Unit Tests

```rust
#[test]
fn test_parse() {
    let game = parse_input(EXAMPLE);
    assert_eq!(game.monkeys.len(), 4);
    assert_eq!(game.monkeys[0].items.len(), 2);
    assert_eq!(game.monkeys[0].divisible_by, 23);
}

#[test]
fn test_part1_example() {
    assert_eq!(solve_part1(EXAMPLE), 10605);
}

#[test]
fn test_part2_example() {
    assert_eq!(solve_part2(EXAMPLE), 2713310158);
}
```

**Coverage**:
- Parsing correctness (structure, values)
- Part 1 logic (20 rounds, relief)
- Part 2 logic (10,000 rounds, modulo)

---

## 💡 Key Insights

### 1. Modular Arithmetic is Essential

**Without modulo**: Overflow after ~10-15 rounds
**With modulo**: Stable for all 10,000 rounds

**Learning**: When simulating growth without bounds, look for modular properties

---

### 2. Queue Semantics Matter

**Wrong data structure**:
```rust
items: Vec<u64>

while !items.is_empty() {
    let worry = items.remove(0);  // O(n) - BAD!
}
```

**Correct data structure**:
```rust
items: VecDeque<u64>

while let Some(worry) = items.pop_front() {  // O(1) - GOOD!
}
```

**Performance impact**: O(n²) → O(n) for item processing

---

### 3. Chinese Remainder Theorem (Informal Application)

**Concept**: When working with multiple moduli, product of moduli preserves remainders

**Application**:
- Multiple divisibility tests (23, 19, 13, 17, ...)
- Product preserves all test results
- Keeps numbers bounded

**Formal CRT**: Works with coprime moduli (likely true for puzzle inputs)
**Informal use**: Product works even if not coprime (might be larger than LCM)

---

## 🎯 Common Mistakes

### 1. ❌ Forgetting to Apply Modulo in Part 2

```rust
// WRONG: Numbers overflow
new_worry = operation.apply(worry);
// NO modulo applied!
```

**Fix**:
```rust
new_worry = operation.apply(worry);
new_worry %= modulo;  // ← Essential!
```

---

### 2. ❌ Using Wrong Modulo Value

```rust
// WRONG: Using single divisor
new_worry %= game.monkeys[0].divisible_by;  // Only preserves one test!
```

**Fix**:
```rust
// CORRECT: Product of ALL divisors
let modulo: u64 = game.monkeys.iter().map(|m| m.divisible_by).product();
new_worry %= modulo;
```

---

### 3. ❌ Processing Items Out of Order

```rust
// WRONG: Reverse iteration
for i in (0..items.len()).rev() {
    // Items inspected in wrong order!
}
```

**Fix**:
```rust
// CORRECT: FIFO queue
while let Some(worry) = items.pop_front() {
    // Items inspected in arrival order
}
```

---

## 🔗 Related Concepts

**AoC Patterns**:
- [[aoc-simulation-patterns]] - Multi-round state evolution
- [[parse-once-pattern]] - Parse ONCE, use many times ✅
- [[aoc-modular-arithmetic]] - Keeping numbers bounded

**Mathematical Concepts**:
- [[chinese-remainder-theorem]] - Multiple moduli preservation
- [[modular-arithmetic]] - Remainder-based calculations
- [[queue-data-structures]] - FIFO semantics with VecDeque

**Zettelkasten Links**:
- [[daily-study/Day11]] - Session notes (if created)
- [[aoc-2022-week2-review]] - Week 2 problem patterns

---

## 📈 Optimization Opportunities

### Current Performance: Excellent ✅

**Benchmarks**:
- Part 1: 9.3µs (very fast)
- Part 2: 3.0ms (efficient for 10,000 rounds)

### Potential Optimizations (likely not needed):

1. **Pre-allocate VecDeque capacity**
   - Avoid reallocation during item passing
   - Minimal benefit (items redistributed anyway)

2. **Use array instead of Vec for inspection_counts**
   - 8 monkeys = fixed size
   - Avoids heap allocation
   - Benefit: ~10-20ns improvement (negligible)

3. **Inline operation.apply()**
   - Already likely inlined by compiler
   - No practical benefit

**Verdict**: Current implementation is near-optimal for this problem size. Optimizations would add complexity with minimal benefit.

---

## 📝 Summary

**Problem**: Simulate monkeys passing items with worry-level dependent behavior

**Challenge**: Part 2's 10,000 rounds cause exponential growth without relief

**Solution**: Modular arithmetic using product of all divisors

**Performance**: 
- Part 1: 9.3µs (20 rounds with ÷3)
- Part 2: 3.0ms (10,000 rounds with modulo)
- Excellent scaling: 500× iterations → 323× runtime

**Key Techniques**:
- Parse-once pattern ✅
- Modular arithmetic optimization (Chinese Remainder Theorem concept)
- FIFO queue with VecDeque
- Careful handling of mutable state during iteration

**Learning**: When simulation involves unbounded growth, look for mathematical properties (modular arithmetic, cycle detection, invariants) to keep values manageable.

---

**Navigation**: [← Day 10](day10_function_guide.md) | [Problem](day11.md) | [Code](../../src/solver/day11.rs) | [Summary](../summary_2022.md)
