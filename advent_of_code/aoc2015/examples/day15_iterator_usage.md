# Day 15: Iterator Usage Explained

## Overview
AoC 2015 Day 15 demonstrates multiple iterator patterns in Rust, from simple iteration to complex iterator chaining with functional programming techniques.

---

## Iterator Usage #1: Line Parsing with `lines()` and `split_whitespace()`

### Location: `parse_input()` function

```rust
for line in _input.lines() {
    let mut line_split = line.split_whitespace();
    // ...
}
```

### What's happening:
1. **`.lines()`** - Creates an iterator over lines in the input string
   - Returns an `Iterator<Item = &str>`
   - Each item is a line without the newline character
   - Lazy evaluation: only processes lines as needed

2. **`.split_whitespace()`** - Creates an iterator over whitespace-separated tokens
   - Returns an `Iterator<Item = &str>`
   - Automatically skips multiple spaces
   - **Important**: Must be `mut` because iterators track their position internally

### Example:
```rust
let input = "Sugar: capacity 3, durability 0";
for line in input.lines() {  // Iterates once (one line)
    let mut tokens = line.split_whitespace();
    // tokens: ["Sugar:", "capacity", "3,", "durability", "0"]
}
```

---

## Iterator Usage #2: Sequential Token Consumption with `.next()`

### Location: `parse_input()` function

```rust
let mut line_split = line.split_whitespace();

let name = line_split.next().unwrap().trim_end_matches(':').to_string();

line_split.next(); // skip "capacity"
let capacity = line_split.next().unwrap().trim_end_matches(',').parse::<i64>()?;

line_split.next(); // skip "durability"
let durability = line_split.next().unwrap().trim_end_matches(',').parse::<i64>()?;
```

### What's happening:
- **`.next()`** - Advances the iterator and returns the next item
  - Returns `Option<&str>` - `Some(value)` or `None` when exhausted
  - **Consumes** the item - you can't go back
  - Each call moves the iterator forward permanently

### Why this pattern?
Input format: `"Sugar: capacity 3, durability 0, flavor 0, texture -3, calories 2"`

| Call | Returns | Action |
|------|---------|--------|
| `.next()` | `"Sugar:"` | Extract and save as name |
| `.next()` | `"capacity"` | Skip the label |
| `.next()` | `"3,"` | Parse as number |
| `.next()` | `"durability"` | Skip the label |
| `.next()` | `"0,"` | Parse as number |
| ... | ... | Continue pattern |

### Common Mistake:
```rust
// ❌ WRONG - .nth(1) is cumulative
let capacity = line_split.nth(1); // Gets 2nd item
let durability = line_split.nth(1); // Gets 4th item (skips 2 more!)

// ✅ CORRECT - .next() advances one at a time
line_split.next(); // skip label
let capacity = line_split.next(); // get value
line_split.next(); // skip label
let durability = line_split.next(); // get value
```

---

## Iterator Usage #3: Functional Iterator Chain with `.enumerate()`, `.map()`, and `.sum()`

### Location: `calculate_calories()` function

```rust
fn calculate_calories(ingredients: &[Ingredient], amounts: &[i64]) -> i64 {
    ingredients
        .iter()
        .enumerate()
        .map(|(i, ingredient)| ingredient.calories * amounts[i])
        .sum()
}
```

### Breaking it down:

#### Step 1: `.iter()` - Create iterator over slice
```rust
ingredients.iter()
// Returns: Iterator<Item = &Ingredient>
// Borrows each ingredient without taking ownership
```

#### Step 2: `.enumerate()` - Add index to each item
```rust
.enumerate()
// Returns: Iterator<Item = (usize, &Ingredient)>
// Transforms: [ingredient1, ingredient2, ...] 
//          -> [(0, ingredient1), (1, ingredient2), ...]
```

#### Step 3: `.map()` - Transform each item
```rust
.map(|(i, ingredient)| ingredient.calories * amounts[i])
// Returns: Iterator<Item = i64>
// Destructures tuple: (index, ingredient reference)
// Calculates: calories × amount for each ingredient
// Example: [(0, Sugar), (1, Cinnamon)] 
//       -> [2*44, 3*56] 
//       -> [88, 168]
```

#### Step 4: `.sum()` - Consume iterator and add all values
```rust
.sum()
// Returns: i64
// Adds all values: 88 + 168 = 256 total calories
```

### Why this pattern is powerful:

**Imperative style (traditional loop):**
```rust
fn calculate_calories_imperative(ingredients: &[Ingredient], amounts: &[i64]) -> i64 {
    let mut total = 0;
    for i in 0..ingredients.len() {
        total += ingredients[i].calories * amounts[i];
    }
    total
}
```

**Functional style (iterator chain):**
```rust
fn calculate_calories(ingredients: &[Ingredient], amounts: &[i64]) -> i64 {
    ingredients
        .iter()
        .enumerate()
        .map(|(i, ingredient)| ingredient.calories * amounts[i])
        .sum()
}
```

**Benefits of functional style:**
- ✅ More concise and readable
- ✅ No mutable state (`mut total`)
- ✅ No manual indexing bounds to check
- ✅ Compiler can optimize better (zero-cost abstraction)
- ✅ Harder to introduce off-by-one errors

---

## Iterator Usage #4: Loop with `.iter()` and `.enumerate()` for Score Calculation

### Location: `calculate_score()` function

```rust
for (i, ingredient) in ingredients.iter().enumerate() {
    capacity_total += ingredient.capacity * amounts[i];
    durability_total += ingredient.durability * amounts[i];
    flavor_total += ingredient.flavor * amounts[i];
    texture_total += ingredient.texture * amounts[i];
}
```

### What's happening:
- **`.iter().enumerate()`** - Same pattern as above, but used in a `for` loop
- **Loop destructuring** - `(i, ingredient)` unpacks the tuple automatically
- **Why not `.map().sum()` here?** - We need to update 4 different totals, not one sum

### Equivalent imperative code:
```rust
for i in 0..ingredients.len() {
    let ingredient = &ingredients[i];
    capacity_total += ingredient.capacity * amounts[i];
    // ...
}
```

### Why `.iter().enumerate()` is better:
```rust
// ❌ Manual indexing - can have bugs
for i in 0..ingredients.len() {
    let ingredient = &ingredients[i];  // Could panic if wrong index
}

// ✅ Iterator - borrows safely, can't go out of bounds
for (i, ingredient) in ingredients.iter().enumerate() {
    // ingredient is guaranteed valid &Ingredient
    // i is guaranteed in range [0, ingredients.len())
}
```

---

## Key Iterator Concepts Demonstrated

### 1. **Iterator State & Consumption**
```rust
let mut iter = vec![1, 2, 3].into_iter();
iter.next(); // Returns Some(1), moves to next position
iter.next(); // Returns Some(2), moves to next position
iter.next(); // Returns Some(3), moves to next position
iter.next(); // Returns None, iterator exhausted
```

### 2. **Borrowing vs Ownership**
```rust
// .iter() - borrows (most common for references)
ingredients.iter()  // Iterator<Item = &Ingredient>

// .into_iter() - takes ownership (consumes collection)
ingredients.into_iter()  // Iterator<Item = Ingredient>

// .iter_mut() - mutable borrows
ingredients.iter_mut()  // Iterator<Item = &mut Ingredient>
```

### 3. **Lazy Evaluation**
```rust
let iter = (0..1_000_000)  // Doesn't create million items in memory
    .map(|x| x * 2)         // Doesn't execute yet
    .filter(|x| x > 100);   // Still lazy

// Only when consumed does computation happen:
let first = iter.next();    // Computes ONE item
let all: Vec<_> = iter.collect();  // Computes ALL remaining items
```

### 4. **Zero-Cost Abstraction**
```rust
// These compile to the SAME machine code:

// Manual loop
let mut sum = 0;
for i in 0..data.len() {
    sum += data[i];
}

// Iterator chain
let sum: i32 = data.iter().sum();
```

---

## Performance Comparison

### Nested Loops (Day 15 Part 1)
```rust
// 4 nested loops generate ~176,000 combinations
for a in 0..=100 {
    for b in 0..=(100 - a) {
        for c in 0..=(100 - a - b) {
            let d = 100 - a - b - c;
            // Process combination
        }
    }
}
```

**Why not use iterators here?**
- Nested loops with constraints are clearer imperatively
- Iterator chains would be more complex: `.flat_map().flat_map().flat_map()`
- Performance is identical (both compile to same loops)

**Rule of thumb:**
- Use iterators for: transforming, filtering, aggregating data
- Use loops for: complex control flow, multiple mutable variables, nested generation

---

## Common Iterator Methods Used in AoC

| Method | Purpose | Example |
|--------|---------|---------|
| `.lines()` | Split string by lines | `input.lines()` |
| `.split_whitespace()` | Split by whitespace | `line.split_whitespace()` |
| `.chars()` | Iterate over characters | `"abc".chars()` |
| `.iter()` | Borrow each element | `vec.iter()` |
| `.enumerate()` | Add index to items | `.iter().enumerate()` |
| `.map()` | Transform each item | `.map(|x| x * 2)` |
| `.filter()` | Keep matching items | `.filter(|x| x > 0)` |
| `.fold()` | Accumulate with state | `.fold(0, |acc, x| acc + x)` |
| `.sum()` | Add all numbers | `.iter().sum()` |
| `.collect()` | Build collection | `.collect::<Vec<_>>()` |
| `.next()` | Get next item | `iter.next()` |
| `.take()` | Take first N items | `.take(5)` |
| `.skip()` | Skip first N items | `.skip(2)` |

---

## Practice Exercise

Try rewriting this imperative code using iterators:

```rust
// Imperative style
fn sum_even_squares(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for &num in numbers {
        if num % 2 == 0 {
            sum += num * num;
        }
    }
    sum
}

// Your turn: write the functional iterator version
fn sum_even_squares_functional(numbers: &[i32]) -> i32 {
    // TODO: Use .iter(), .filter(), .map(), .sum()
    todo!()
}
```

<details>
<summary>Click for solution</summary>

```rust
fn sum_even_squares_functional(numbers: &[i32]) -> i32 {
    numbers
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * n)
        .sum()
}
```

</details>

---

## Further Reading

- **Rust Book Chapter 13**: Iterators and Closures
- **Iterator trait documentation**: `std::iter::Iterator`
- **Common iterator adapters**: `map`, `filter`, `fold`, `scan`, `flat_map`
- **Performance**: Iterators are zero-cost abstractions in Rust

---

## Summary

Day 15 demonstrates three core iterator patterns:

1. **Sequential parsing** - `.lines()` and `.split_whitespace()` for text processing
2. **Token consumption** - `.next()` for state-based parsing
3. **Functional transformations** - `.iter().enumerate().map().sum()` for calculations

Mastering these patterns makes Rust code more concise, safer, and often easier to reason about than equivalent imperative loops.
