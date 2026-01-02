# AoC 2023 - Patterns Catalog

Reusable patterns extracted from daily solutions. Patterns are added when used in 3+ days.

---

## 📊 Pattern Usage Summary

| Pattern | Days Used | Location |
|---------|-----------|----------|
| Forward/Reverse Search | Day 1 | `day01.rs` |
| Delimiter-Based Parsing | Day 1, Day 2 | `day01.rs`, `day02.rs` |

---

## 🔤 Parsing Patterns

### Pattern: Position-by-Position Scanning
**Used**: Day 1  
**When to use**: When overlapping patterns need to be detected (e.g., `eightwothree`)  
**Code**: `src/solver/day01.rs::digit_at_position()`

```rust
fn digit_at_position(s: &str, pos: usize) -> Option<u32> {
    let remainder = &s[pos..];
    // Check patterns at this position
    for (pattern, value) in PATTERNS {
        if remainder.starts_with(pattern) {
            return Some(value);
        }
    }
    None
}
```

**Note**: Will promote to `src/patterns/parsing.rs` if used 3+ times.

### Pattern: Delimiter-Based Hierarchical Parsing
**Used**: Day 1 (lines), Day 2 (structured data)  
**When to use**: Input has nested structure with consistent delimiters  
**Code**: `src/solver/day02.rs::parse_game()`

```rust
// Hierarchical split: lines → records → fields → values
let parts: Vec<&str> = line.split(':').collect();  // "Game 1: data"
let game_id = parts[0].strip_prefix("Game ")?.parse()?;

for reveal in parts[1].split(';') {  // Multiple reveals per game
    for cube in reveal.split(',') {   // Multiple cubes per reveal
        let parts: Vec<&str> = cube.trim().split_whitespace().collect();
        let count: u32 = parts[0].parse()?;
        let color = parts[1];
    }
}
```

**Pattern**: 
1. Split on primary delimiter (`:` for record structure)
2. Split on secondary delimiter (`;` for sequences)
3. Split on tertiary delimiter (`,` for elements)
4. Parse individual values

**Error handling**: Use `?` operator with `anyhow::Result` for clean propagation.

**Note**: Pattern used 2+ times, monitor for extraction.

### Pattern: Forward/Reverse Search with Early Termination
**Used**: Day 1  
**When to use**: Finding first and last occurrence of something in a sequence  
**Code**: `src/solver/day01.rs`

```rust
// For simple char matching:
let first = line.chars().find_map(|c| c.to_digit(10))?;
let last = line.chars().rev().find_map(|c| c.to_digit(10))?;

// For position-based matching (when patterns look forward):
let first = (0..line.len()).find_map(|pos| check_at_position(line, pos))?;
let last = (0..line.len()).rev().find_map(|pos| check_at_position(line, pos))?;
```

**Key insight**: For Part 2, we iterate *positions* in reverse, not the string itself. The check function still looks forward from each position.

---

## 🗺️ Grid Processing Patterns

*Patterns to be extracted as grid problems are solved.*

### Pattern: BFS Neighbor Exploration
**Used**: TBD  
**Code**: `src/patterns/grid_search.rs` (when created)

---

## 🔢 Mathematical Patterns

*Patterns to be extracted as math-heavy problems are solved.*

### Pattern: GCD/LCM for Cycle Detection
**Used**: TBD  
**Code**: `src/patterns/number_theory.rs` (when created)

---

## 🌐 Graph Patterns

*Patterns to be extracted as graph problems are solved.*

### Pattern: Dijkstra with Priority Queue
**Used**: TBD  
**Code**: `src/patterns/pathfinding.rs` (when created)

---

## 📝 Pattern Extraction Criteria

A pattern is extracted to `src/patterns/` when:
1. Used in **3+ different days**
2. Has **consistent interface** across uses
3. Is **generic enough** to be reusable
4. Provides **significant code reduction**

---

## 🔗 Related Resources

- [Algorithms Reference](algorithms-reference.md) - Deep dives on complex algorithms
- [Performance Analysis](performance-analysis.md) - Optimization techniques
- [[aoc-parsing-patterns]] - Zettelkasten note on parsing
- [[iterator-patterns]] - Zettelkasten note on iterators
