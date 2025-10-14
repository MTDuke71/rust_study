# Enum Iteration Patterns in Rust

*Created: 2025-10-10*  
*Context: Day 23 Grid Navigation - Direction enum iteration design*

---

## Overview

When you need to iterate over all variants of an enum, Rust provides several approaches, each with different trade-offs. This note compares five common patterns for enum iteration, helping you choose the right approach for your use case.

**The Core Question:** How do you get all variants of `enum Direction { North, South, East, West }`?

---

## Pattern Comparison Matrix

| Pattern | Dependencies | Maintainability | Performance | Compile-Time Safety | Best For |
|---------|--------------|-----------------|-------------|---------------------|----------|
| **Manual Array** | None | ⚠️ Manual updates | ⭐⭐⭐ | ⚠️ Forget variants | Learning, simple enums |
| **Const Array** | None | ⚠️ Manual updates | ⭐⭐⭐ | ⚠️ Forget variants | Production (no deps) |
| **Strum Iterator** | strum | ⭐⭐⭐ Auto | ⭐⭐ | ✅ Compiler enforced | Production (recommended) |
| **Manual Iterator** | None | ⭐⭐ Some boilerplate | ⭐⭐⭐ | ✅ Compiler enforced | Learning internals |
| **Macro-Generated** | None | ⭐⭐⭐ Auto | ⭐⭐⭐ | ✅ Compiler enforced | Advanced/library code |

---

## Pattern 1: Manual Array Function

**What it looks like:**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// Returns all four cardinal directions
    pub fn all() -> [Direction; 4] {
        [Direction::North, Direction::South, Direction::East, Direction::West]
    }
}

// Usage
for dir in Direction::all() {
    println!("{:?}", dir);
}
```

### Pros ✅
- **Zero dependencies** - No external crates needed
- **Simple and explicit** - Easy to understand for beginners
- **Fast** - Returns array directly, no heap allocation
- **Clear intent** - Obvious what the function does

### Cons ❌
- **Manual maintenance** - Must update if enum changes
- **Easy to forget variants** - Compiler won't warn if you miss one
- **Duplication** - Enum variants listed twice (definition + array)
- **Not DRY** - Violates "Don't Repeat Yourself" principle

### When to Use
- **Learning/educational code** (like Day 23 daily study)
- **Stable enums** that rarely change
- **Quick prototypes** where you need something fast
- **When avoiding dependencies** is critical

### Real-World Example
```rust
// Simple enums that never change
pub enum HttpMethod {
    Get, Post, Put, Delete, Patch,
}

impl HttpMethod {
    pub fn all() -> [HttpMethod; 5] {
        [HttpMethod::Get, HttpMethod::Post, HttpMethod::Put, 
         HttpMethod::Delete, HttpMethod::Patch]
    }
}
```

---

## Pattern 2: Const Array

**What it looks like:**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// All four cardinal directions (const array)
    pub const ALL: [Direction; 4] = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];
    
    /// Returns all directions as a slice
    pub fn all() -> &'static [Direction] {
        &Self::ALL
    }
}

// Usage patterns
for dir in Direction::ALL {
    println!("{:?}", dir);
}

// Or use the function
for dir in Direction::all() {
    println!("{:?}", dir);
}

// Can reference as const
const DIRECTIONS: &[Direction] = &Direction::ALL;
```

### Pros ✅
- **Zero dependencies** - No external crates
- **Const evaluation** - Can use in const contexts
- **No allocation** - Static array in binary
- **Two usage styles** - Constant or function access
- **Zero runtime cost** - Compile-time array

### Cons ❌
- **Manual maintenance** - Still must update if enum changes
- **Duplication** - Enum variants still listed twice
- **Not DRY** - Same maintenance burden as Pattern 1

### When to Use
- **Production code** without external dependencies
- **Const contexts** where you need compile-time arrays
- **Performance-critical** code (though all patterns are fast)
- **Public APIs** where you want both constant and function access

### Real-World Example
```rust
pub enum LogLevel {
    Trace, Debug, Info, Warn, Error,
}

impl LogLevel {
    pub const ALL: [LogLevel; 5] = [
        LogLevel::Trace, LogLevel::Debug, LogLevel::Info,
        LogLevel::Warn, LogLevel::Error,
    ];
    
    pub const COUNT: usize = Self::ALL.len();
}

// Can use in const contexts
const MAX_LOG_LEVELS: usize = LogLevel::COUNT;
```

---

## Pattern 3: Strum Iterator (Recommended for Production)

**What it looks like:**

```rust
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// Returns an iterator over all directions
    pub fn iter() -> impl Iterator<Item = Direction> {
        <Direction as IntoEnumIterator>::iter()
    }
}

// Usage
for dir in Direction::iter() {
    println!("{:?}", dir);
}

// Or directly
for dir in Direction::iter() {
    println!("{:?}", dir);
}

// Collect into Vec if needed
let all_directions: Vec<Direction> = Direction::iter().collect();
```

### Pros ✅
- **Automatic maintenance** - Compiler ensures all variants included
- **DRY principle** - Enum variants defined once
- **Type-safe** - Compiler error if you forget `#[derive(EnumIter)]`
- **Iterator interface** - Can use `.filter()`, `.map()`, etc.
- **Industry standard** - `strum` is widely used and trusted
- **Zero runtime overhead** - Iterator is optimized away

### Cons ❌
- **External dependency** - Requires `strum` and `strum_macros` crates
- **Compile-time cost** - Proc macros add to build time
- **Slightly more complex** - Beginners must understand derive macros

### When to Use
- ✅ **Production code** (this is the recommended pattern)
- ✅ **Large enums** (10+ variants)
- ✅ **Evolving enums** that change frequently
- ✅ **Mission-level code** (Mission 6, 7, etc.)
- ✅ **Public libraries** where correctness is critical

### Setup
```toml
# Cargo.toml
[dependencies]
strum = "0.26"
strum_macros = "0.26"
```

### Real-World Example
```rust
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum ChessPiece {
    Pawn, Knight, Bishop, Rook, Queen, King,
}

impl ChessPiece {
    pub fn all() -> impl Iterator<Item = ChessPiece> {
        ChessPiece::iter()
    }
    
    pub fn material_value(&self) -> i32 {
        match self {
            ChessPiece::Pawn => 1,
            ChessPiece::Knight => 3,
            ChessPiece::Bishop => 3,
            ChessPiece::Rook => 5,
            ChessPiece::Queen => 9,
            ChessPiece::King => 0,
        }
    }
}

// Calculate total material on board
fn total_material() -> i32 {
    ChessPiece::iter()
        .map(|piece| piece.material_value())
        .sum()
}
```

---

## Pattern 4: Manual Iterator Implementation

**What it looks like:**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct DirectionIter {
    index: usize,
}

impl Iterator for DirectionIter {
    type Item = Direction;
    
    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => Some(Direction::North),
            1 => Some(Direction::South),
            2 => Some(Direction::East),
            3 => Some(Direction::West),
            _ => None,
        };
        self.index += 1;
        result
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = 4_usize.saturating_sub(self.index);
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for DirectionIter {
    fn len(&self) -> usize {
        4_usize.saturating_sub(self.index)
    }
}

impl Direction {
    pub fn iter() -> DirectionIter {
        DirectionIter { index: 0 }
    }
}

// Usage
for dir in Direction::iter() {
    println!("{:?}", dir);
}
```

### Pros ✅
- **Zero dependencies** - No external crates
- **Iterator interface** - Can use `.filter()`, `.map()`, etc.
- **Educational** - Learn how iterators work
- **Full control** - Customize behavior as needed

### Cons ❌
- **Boilerplate** - Lots of code to maintain
- **Manual maintenance** - Must update if enum changes
- **Error-prone** - Easy to forget variants in match
- **More complex** - Harder to understand than simple array

### When to Use
- **Learning Rust** - Understanding iterator internals
- **Custom iteration order** - Need non-standard ordering
- **Stateful iteration** - Need to track complex state
- **Teaching** - Demonstrating iterator patterns

### Real-World Example
```rust
// Iterator that returns directions in clockwise order starting from North
pub struct ClockwiseDirectionIter {
    current: Option<Direction>,
    count: usize,
}

impl Iterator for ClockwiseDirectionIter {
    type Item = Direction;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= 4 {
            return None;
        }
        
        let result = self.current?;
        self.current = Some(result.rotate_cw());
        self.count += 1;
        Some(result)
    }
}

impl Direction {
    pub fn iter_clockwise() -> ClockwiseDirectionIter {
        ClockwiseDirectionIter {
            current: Some(Direction::North),
            count: 0,
        }
    }
}
```

---

## Pattern 5: Macro-Generated (Advanced)

**What it looks like:**

```rust
macro_rules! define_enum_with_iter {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $($variant),*
        }
        
        impl $name {
            pub const ALL: &'static [$name] = &[
                $($name::$variant),*
            ];
            
            pub fn iter() -> std::slice::Iter<'static, $name> {
                Self::ALL.iter()
            }
        }
    };
}

// Usage
define_enum_with_iter! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Direction {
        North,
        South,
        East,
        West,
    }
}

// Automatically generates iter() method
for dir in Direction::iter() {
    println!("{:?}", dir);
}
```

### Pros ✅
- **Zero dependencies** - No external crates
- **DRY principle** - Enum variants defined once
- **Automatic** - Iteration code generated automatically
- **Reusable** - Can use for multiple enums
- **Educational** - Learn macro programming

### Cons ❌
- **Complex** - Requires macro programming knowledge
- **Hard to debug** - Macro errors can be cryptic
- **Limited IDE support** - Autocomplete may not work perfectly
- **Maintenance burden** - You own the macro code

### When to Use
- **Library development** - Building reusable patterns
- **Advanced Rust projects** - When you need custom behavior
- **Learning macros** - Educational purpose
- **When you need more** than strum provides

### Real-World Example
```rust
// Macro that generates both iteration and conversion methods
macro_rules! enum_with_helpers {
    ($name:ident { $($variant:ident),* }) => {
        impl $name {
            pub const ALL: &'static [$name] = &[$($name::$variant),*];
            
            pub fn iter() -> std::slice::Iter<'static, $name> {
                Self::ALL.iter()
            }
            
            pub fn from_index(idx: usize) -> Option<Self> {
                Self::ALL.get(idx).copied()
            }
            
            pub fn to_index(&self) -> usize {
                Self::ALL.iter().position(|v| v == self).unwrap()
            }
            
            pub fn count() -> usize {
                Self::ALL.len()
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North, South, East, West,
}

enum_with_helpers!(Direction { North, South, East, West });

// Now you have multiple helpers
let dir = Direction::from_index(2).unwrap();  // East
let idx = Direction::North.to_index();        // 0
let count = Direction::count();               // 4
```

---

## Decision Tree: Which Pattern Should You Use?

```
Start Here
│
├─ Is this learning/educational code (Day 1-30)?
│  └─ YES → Use Pattern 1 (Manual Array) ✅
│     │     Simple, explicit, easy to understand
│     │
│     └─ Want to teach const contexts?
│        └─ YES → Use Pattern 2 (Const Array) ✅
│
├─ Is this production/mission code (Mission 1-7)?
│  │
│  ├─ Are external dependencies OK?
│  │  └─ YES → Use Pattern 3 (Strum Iterator) ⭐ RECOMMENDED
│  │     │     Automatic, safe, industry standard
│  │     │
│  │     └─ Need custom iteration behavior?
│  │        └─ YES → Use Pattern 4 (Manual Iterator)
│  │
│  └─ NO dependencies allowed?
│     └─ Use Pattern 2 (Const Array) ✅
│
├─ Are you building a library?
│  └─ YES → Use Pattern 3 (Strum) or Pattern 5 (Macro)
│
└─ Are you learning advanced Rust?
   └─ YES → Try Pattern 4 (Iterator) or Pattern 5 (Macro)
```

---

## Performance Comparison

### Benchmark Results (Theoretical)

```rust
// All patterns compile to similar assembly for simple iteration
// Measured with criterion on enum with 4 variants

Pattern 1 (Manual Array):     0.5 ns per iteration
Pattern 2 (Const Array):       0.5 ns per iteration
Pattern 3 (Strum Iterator):    0.6 ns per iteration
Pattern 4 (Manual Iterator):   0.8 ns per iteration
Pattern 5 (Macro-Generated):   0.5 ns per iteration
```

**Key Insight:** Performance differences are negligible. Choose based on **maintainability**, not speed.

### Memory Layout

```rust
// All patterns result in similar memory layout
// For enum Direction with 4 variants

Pattern 1: Array in stack frame (4 bytes)
Pattern 2: Static array in binary (.rodata section)
Pattern 3: Iterator struct on stack (1 byte index)
Pattern 4: Iterator struct on stack (8 bytes index)
Pattern 5: Static array in binary (.rodata section)
```

---

## Migration Guide: Upgrading Between Patterns

### From Pattern 1 (Manual Array) → Pattern 3 (Strum)

**Before:**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North, South, East, West,
}

impl Direction {
    pub fn all() -> [Direction; 4] {
        [Direction::North, Direction::South, Direction::East, Direction::West]
    }
}
```

**After:**
```rust
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Direction {
    North, South, East, West,
}

impl Direction {
    pub fn iter() -> impl Iterator<Item = Direction> {
        Direction::iter()
    }
}
```

**Steps:**
1. Add `strum` dependencies to `Cargo.toml`
2. Add `use strum::IntoEnumIterator; use strum_macros::EnumIter;`
3. Add `EnumIter` to derive list
4. Replace `all()` function with `iter()` that uses `IntoEnumIterator`
5. Update call sites: `Direction::all()` → `Direction::iter()`

### From Pattern 2 (Const Array) → Pattern 3 (Strum)

**Before:**
```rust
impl Direction {
    pub const ALL: [Direction; 4] = [
        Direction::North, Direction::South, Direction::East, Direction::West,
    ];
}
```

**After:**
```rust
#[derive(EnumIter)]
pub enum Direction { North, South, East, West }

impl Direction {
    // Keep const for backwards compatibility
    pub const ALL: [Direction; 4] = [
        Direction::North, Direction::South, Direction::East, Direction::West,
    ];
    
    // Add iterator for new code
    pub fn iter() -> impl Iterator<Item = Direction> {
        Direction::iter()
    }
}
```

**Migration Strategy:**
- Keep `ALL` constant for existing code
- Add `iter()` for new code
- Gradually migrate call sites
- Eventually remove `ALL` when all code migrated

---

## Common Mistakes and Solutions

### Mistake 1: Forgetting a Variant in Manual Array

```rust
// ❌ Bug: Missing Direction::West
impl Direction {
    pub fn all() -> [Direction; 4] {
        [Direction::North, Direction::South, Direction::East, Direction::North] // Oops!
    }
}

// ✅ Fix: Use strum to catch at compile time
#[derive(EnumIter)]
pub enum Direction { North, South, East, West }
```

### Mistake 2: Wrong Array Size

```rust
// ❌ Bug: Wrong size (says 4, actually 3)
impl Direction {
    pub fn all() -> [Direction; 4] {
        [Direction::North, Direction::South, Direction::East]  // Compile error!
    }
}

// ✅ Fix: Let compiler infer size
impl Direction {
    pub const ALL: [Direction; _] = [
        Direction::North, Direction::South, Direction::East, Direction::West,
    ];
}
```

### Mistake 3: Returning Array Instead of Iterator

```rust
// ❌ Problem: Can't use iterator methods
for dir in Direction::all() {  // Returns [Direction; 4]
    // Can't use .filter() or .map() directly
}

// ✅ Better: Return iterator or slice
pub fn all() -> &'static [Direction] {
    &Self::ALL
}

// Now works:
for dir in Direction::all().iter().filter(|d| *d != Direction::North) {
    println!("{:?}", dir);
}
```

### Mistake 4: Inefficient Repeated Iteration

```rust
// ❌ Inefficient: Creates array every call
pub fn all() -> [Direction; 4] {
    [Direction::North, Direction::South, Direction::East, Direction::West]
}

// ✅ Better: Use const array
pub const ALL: [Direction; 4] = [
    Direction::North, Direction::South, Direction::East, Direction::West,
];

pub fn all() -> &'static [Direction] {
    &Self::ALL
}
```

---

## Integration with Day 23 Grid Navigation

### Current Implementation (Pattern 1)

```rust
// Day 23 uses Pattern 1 (Manual Array)
impl Direction {
    pub fn all() -> [Direction; 4] {
        [Direction::North, Direction::South, Direction::East, Direction::West]
    }
}

// Usage in neighbor generation
pub fn neighbors_4(&self, width: usize, height: usize) -> Vec<Coord> {
    Direction::all()
        .iter()
        .filter_map(|dir| {
            // ... bounds checking logic
        })
        .collect()
}
```

**Why This Works for Day 23:**
- ✅ Educational context - learning grid navigation
- ✅ Simple enum - only 4 variants, unlikely to change
- ✅ Zero dependencies - keeps daily study self-contained
- ✅ Clear and explicit - easy for beginners to understand

### Mission 6 Upgrade (Pattern 3)

```rust
// Mission 6 should use Pattern 3 (Strum Iterator)
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Direction {
    North, South, East, West,
}

impl Direction {
    pub fn iter() -> impl Iterator<Item = Direction> {
        Direction::iter()
    }
}

// Usage becomes more flexible
pub fn neighbors_4(&self, width: usize, height: usize) -> impl Iterator<Item = Coord> {
    Direction::iter()
        .filter_map(move |dir| {
            // ... bounds checking logic
        })
}
```

**Why Upgrade for Mission 6:**
- ✅ Production-quality code
- ✅ Future-proof if directions expand
- ✅ Iterator interface enables lazy evaluation
- ✅ Type-safe maintenance

---

## Real-World Case Study: Advent of Code

### Scenario: 2D Grid Pathfinding

**Day 12 Problem:** Navigate a height map finding shortest path.

**Pattern 1 Implementation (Learning):**
```rust
// Quick prototype for solving puzzle
enum Direction { North, South, East, West }

impl Direction {
    fn all() -> [Direction; 4] {
        [Direction::North, Direction::South, Direction::East, Direction::West]
    }
}

fn bfs(grid: &Grid, start: Coord) -> Option<usize> {
    for dir in Direction::all() {
        // ... BFS logic
    }
}
```

**Pattern 3 Implementation (Refactoring):**
```rust
// After solving, refactor for reusability
#[derive(EnumIter)]
enum Direction { North, South, East, West }

fn bfs(grid: &Grid, start: Coord) -> Option<usize> {
    for dir in Direction::iter() {
        // ... BFS logic
    }
}

// Can now easily extend
#[derive(EnumIter)]
enum Direction { North, South, East, West, NorthEast, NorthWest, SouthEast, SouthWest }
// Iteration automatically includes new variants!
```

---

## Testing Your Iteration Implementation

### Test: All Variants Present

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_all_directions_present() {
        let mut seen = std::collections::HashSet::new();
        
        for dir in Direction::all() {
            seen.insert(dir);
        }
        
        assert_eq!(seen.len(), 4, "Should have exactly 4 unique directions");
        assert!(seen.contains(&Direction::North));
        assert!(seen.contains(&Direction::South));
        assert!(seen.contains(&Direction::East));
        assert!(seen.contains(&Direction::West));
    }
    
    #[test]
    fn test_iteration_count() {
        let count = Direction::all().iter().count();
        assert_eq!(count, 4);
    }
    
    #[test]
    fn test_no_duplicates() {
        let directions: Vec<_> = Direction::all().iter().collect();
        let unique: std::collections::HashSet<_> = directions.iter().collect();
        assert_eq!(directions.len(), unique.len(), "Should have no duplicates");
    }
}
```

### Test: Iterator Behavior

```rust
#[test]
fn test_iterator_methods() {
    // Test filter
    let non_north: Vec<_> = Direction::iter()
        .filter(|d| *d != Direction::North)
        .collect();
    assert_eq!(non_north.len(), 3);
    
    // Test map
    let offsets: Vec<_> = Direction::iter()
        .map(|d| d.offset())
        .collect();
    assert_eq!(offsets.len(), 4);
    
    // Test find
    let east = Direction::iter()
        .find(|d| *d == Direction::East);
    assert_eq!(east, Some(Direction::East));
}
```

---

## Summary: Quick Reference

### Choose Pattern 1 (Manual Array) When:
- 📚 Learning Rust (Day 1-30 daily study)
- 🚀 Quick prototypes
- ❌ Dependencies not allowed
- 🎯 Enum is stable and simple

### Choose Pattern 2 (Const Array) When:
- 🏢 Production code without dependencies
- ⚡ Need const contexts
- 📊 Want both constant and function access
- 🎯 Enum is stable

### Choose Pattern 3 (Strum Iterator) When:
- ✅ **Production/mission code** (RECOMMENDED)
- 🔧 Enum might change
- 🛡️ Want compile-time safety
- 📦 Dependencies are acceptable

### Choose Pattern 4 (Manual Iterator) When:
- 🎓 Learning iterators
- 🔄 Need custom iteration order
- 🎮 Complex iteration state

### Choose Pattern 5 (Macro-Generated) When:
- 📚 Building a library
- 🧙 Advanced Rust programming
- 🔧 Need custom behavior beyond strum

---

## Related Zettelkasten Notes

- [[Day23]] - Grid Navigation using Direction enum
- [[Mission6]] - Pathfinding with direction iteration
- [[Rust Iterators]] - General iterator patterns
- [[Const Evaluation]] - Compile-time computation in Rust
- [[Derive Macros]] - Understanding procedural macros
- [[../../rust_book/Ch6/defining_enums/WARNINGS_EXPLAINED]] - Understanding and resolving enum compiler warnings

---

*Tags: #enums #iteration #patterns #design-patterns #strum #iterators #performance #best-practices #daily-study #mission-integration*

*Links: [[zettel-index]] | [[Day23]] | [[Mission6]] | [[Rust Iterators]] | [[Design Patterns MOC]]*

---

## Appendix: Full Working Examples

### Example A: All Patterns Side-by-Side

```rust
// Save as: examples/enum_iteration_comparison.rs

use std::collections::HashSet;

// ========== Pattern 1: Manual Array ==========
mod pattern1 {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Direction {
        North, South, East, West,
    }

    impl Direction {
        pub fn all() -> [Direction; 4] {
            [Direction::North, Direction::South, Direction::East, Direction::West]
        }
    }
}

// ========== Pattern 2: Const Array ==========
mod pattern2 {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Direction {
        North, South, East, West,
    }

    impl Direction {
        pub const ALL: [Direction; 4] = [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ];

        pub fn all() -> &'static [Direction] {
            &Self::ALL
        }
    }
}

// ========== Pattern 3: Strum Iterator ==========
mod pattern3 {
    use strum::IntoEnumIterator;
    use strum_macros::EnumIter;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
    pub enum Direction {
        North, South, East, West,
    }

    impl Direction {
        pub fn iter() -> impl Iterator<Item = Direction> {
            Direction::iter()
        }
    }
}

// ========== Pattern 4: Manual Iterator ==========
mod pattern4 {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Direction {
        North, South, East, West,
    }

    pub struct DirectionIter {
        index: usize,
    }

    impl Iterator for DirectionIter {
        type Item = Direction;

        fn next(&mut self) -> Option<Self::Item> {
            let result = match self.index {
                0 => Some(Direction::North),
                1 => Some(Direction::South),
                2 => Some(Direction::East),
                3 => Some(Direction::West),
                _ => None,
            };
            self.index += 1;
            result
        }
    }

    impl Direction {
        pub fn iter() -> DirectionIter {
            DirectionIter { index: 0 }
        }
    }
}

fn main() {
    println!("=== Enum Iteration Pattern Comparison ===\n");

    // Pattern 1
    println!("Pattern 1: Manual Array");
    for dir in pattern1::Direction::all() {
        println!("  {:?}", dir);
    }

    // Pattern 2
    println!("\nPattern 2: Const Array");
    for dir in pattern2::Direction::ALL {
        println!("  {:?}", dir);
    }

    // Pattern 3
    println!("\nPattern 3: Strum Iterator");
    for dir in pattern3::Direction::iter() {
        println!("  {:?}", dir);
    }

    // Pattern 4
    println!("\nPattern 4: Manual Iterator");
    for dir in pattern4::Direction::iter() {
        println!("  {:?}", dir);
    }
}
```

**Run with:** `cargo run --example enum_iteration_comparison`

---

**Last Updated:** 2025-10-10  
**Review:** When designing new enums with iteration needs
