# Day 9: Rope Bridge - Function Guide

**Quick Links**: [← Day 8](day08_function_guide.md) | [Problem Statement](day09.md) | [Summary](../summary_2022.md) | [Day 10 →](day10_function_guide.md)

---

## Problem Overview
Simulate rope physics with knots that follow each other based on distance rules:
- **Part 1**: Track tail positions for a 2-knot rope (head + tail)
- **Part 2**: Track tail positions for a 10-knot rope (each knot follows the previous)

**Touching Rule**: Knots must stay adjacent (Manhattan distance ≤ 1 in both x and y, includes diagonal).  
**Movement Rule**: When not touching, tail moves one step toward head (diagonal if needed).

**Answer**: Part 1: `6367` | Part 2: `2536`

## Performance Benchmarks
- **Combined**: ~~756µs~~ **568µs** (0.568ms) — optimized with FxHashSet + parse-once
- **Part 1**: ~180µs (2-knot rope simulation)
- **Part 2**: ~388µs (10-knot rope simulation, 5× more knots but same physics)

**Optimizations**:
1. **Parse once**: 756µs → 726µs (-4%, single parse shared between parts)
2. **FxHashSet**: 726µs → 568µs (-20%, faster hash for integer keys)

**Complexity**: O(moves × knots) where moves ≈ 2000 commands, knots = 2 or 10

---

## Algorithm: Rope Physics Simulation

### Coordinate System
Uses **signed coordinates** (`i32`) to handle negative positions as rope moves:
```rust
struct Pos {
    x: i32,  // Horizontal position (can be negative)
    y: i32,  // Vertical position (can be negative)
}
```

**Why signed?** The rope starts at `(0, 0)` but can move in all directions:
- `L 5` (left) → positions like `(-5, 0)`, `(-4, 0)`, etc.
- `D 3` (down) → positions like `(0, 3)`, `(0, 4)`, etc.

### Part 1: Two-Knot Rope

```
Example: R 4 (move head right 4 steps)

Initial:           After R 1:         After R 2:
  H (covers T)       T H               s T H
  starting at        (0,0) (1,0)       (0,0) (1,0) (2,0)
  (0, 0)

Key: T follows H only when they're NOT touching
```

**Physics Rules**:
1. **Touching**: `|dx| ≤ 1 AND |dy| ≤ 1` where `dx = head.x - tail.x`, `dy = head.y - tail.y`
2. **Not touching**: Move tail one step toward head using `signum()`

```rust
// Example: Head at (5, 3), Tail at (3, 1)
dx = 5 - 3 = 2   // Two steps apart horizontally
dy = 3 - 1 = 2   // Two steps apart vertically

// Move diagonally toward head:
tail.x += dx.signum()  // 2.signum() = 1 → tail.x becomes 4
tail.y += dy.signum()  // 2.signum() = 1 → tail.y becomes 2
// Result: Tail at (4, 2), now touching head at (5, 3)
```

**`signum()` function**: Returns -1, 0, or 1 indicating direction:
- `dx = 3` → `signum() = 1` (move right)
- `dx = -2` → `signum() = -1` (move left)  
- `dx = 0` → `signum() = 0` (no horizontal movement)

### Part 2: Ten-Knot Rope

```
Knots: [0=H, 1, 2, 3, 4, 5, 6, 7, 8, 9=tail]

After head moves, each knot follows the previous:
1. Move head (knot 0) based on input command
2. Knot 1 follows knot 0 (using same physics)
3. Knot 2 follows knot 1
4. ...
9. Knot 9 (tail) follows knot 8
```

**Cascading movement**: One head step can cause multiple knots to move:
```
Initial: H 1 2 3 4 5 6 7 8 9 (all at origin)
After "R 5":
   H→→→→→1 2 3 4 5 (only first few knots moved)
   (0,0) →→→→→(5,0)

After "U 8": Head moves up, each knot follows in chain
```

**Key insight**: Same `follow()` logic applies to every knot pair!

---

## Function Reference

### `parse_moves(input: &str) -> Vec<(Direction, i32)>`
**Purpose**: Convert input commands to structured moves  
**Input Format**: `"R 4\nU 2\nL 1"` → Right 4, Up 2, Left 1

**Mapping**:
| Char | Direction | Mission 6 Enum |
|------|-----------|----------------|
| `U` | Up | `Direction::North` |
| `D` | Down | `Direction::South` |
| `L` | Left | `Direction::West` |
| `R` | Right | `Direction::East` |

```rust
let mut parts = line.split_whitespace();
let dir_char = parts.next()?;
let count: i32 = parts.next()?.parse().ok()?;

let direction = match dir_char {
    "U" => Direction::North,
    "D" => Direction::South,
    "L" => Direction::West,
    "R" => Direction::East,
    _ => return None,
};
```

**Output**: `Vec<(Direction, i32)>` — list of (direction, step_count) pairs

**Mission 6 Integration**: Uses `Direction` enum for type safety and consistency

---

### `Pos::step(&mut self, dir: Direction)`
**Purpose**: Move position one step in specified direction  
**Strategy**: Modify `x` or `y` based on direction (cardinals only)

```rust
match dir {
    Direction::North => self.y -= 1,  // Up (North decreases y)
    Direction::South => self.y += 1,  // Down
    Direction::East  => self.x += 1,  // Right
    Direction::West  => self.x -= 1,  // Left
    _ => {}  // Diagonals not used for head movement
}
```

**Coordinate Convention**:
- **North (Up)**: Decreases y (screen coordinates)
- **South (Down)**: Increases y
- **East (Right)**: Increases x
- **West (Left)**: Decreases x

**Why no bounds checking?** Rope moves in unbounded space (can have negative coords).

---

### `Pos::is_touching(&self, other: &Pos) -> bool`
**Purpose**: Check if two positions are adjacent (within 1 step in all directions)  
**Strategy**: Chebyshev distance ≤ 1

```rust
(self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
```

**Truth table**:
| dx | dy | Touching? | Example |
|----|----|-----------|---------|
| 0 | 0 | ✓ Yes | Overlapping |
| 1 | 0 | ✓ Yes | Horizontally adjacent |
| 0 | 1 | ✓ Yes | Vertically adjacent |
| 1 | 1 | ✓ Yes | Diagonally adjacent |
| 2 | 0 | ✗ No | Two steps apart horizontally |
| 1 | 2 | ✗ No | Knight's move (not touching) |

**Complexity**: O(1) — just two subtractions and comparisons

---

### `Pos::follow(&mut self, head: &Pos)`
**Purpose**: Update tail position to follow head using rope physics  
**Strategy**: Move one step toward head if not touching

```rust
// Early exit: already touching
if self.is_touching(head) {
    return;
}

// Calculate deltas
let dx = head.x - self.x;
let dy = head.y - self.y;

// Move one step in each direction (using signum for -1/0/1)
self.x += dx.signum();
self.y += dy.signum();
```

**Movement examples**:

| Scenario | Head | Tail | dx | dy | Movement | New Tail |
|----------|------|------|----|----|----|-------|
| Horizontal gap | (3, 0) | (0, 0) | 3 | 0 | x += 1, y += 0 | (1, 0) |
| Vertical gap | (0, 3) | (0, 0) | 0 | 3 | x += 0, y += 1 | (0, 1) |
| Diagonal gap | (2, 2) | (0, 0) | 2 | 2 | x += 1, y += 1 | (1, 1) |
| Knight's move | (2, 1) | (0, 0) | 2 | 1 | x += 1, y += 1 | (1, 1) |

**Diagonal movement**: `signum()` naturally handles diagonal motion when both dx and dy are non-zero.

**Why it works**: 
- If `dx = 2`, `signum() = 1` → move 1 step right
- If `dy = -3`, `signum() = -1` → move 1 step up
- Combined: moves diagonally toward head

**Complexity**: O(1) — constant time calculation and update

---

### `solve_part1(input: &str) -> usize`
**Purpose**: Count unique positions visited by tail of 2-knot rope  
**Strategy**: Simulate head movements, track tail positions in HashSet

```rust
let mut head = Pos::origin();  // (0, 0)
let mut tail = Pos::origin();
let mut visited = HashSet::new();

visited.insert(tail);  // Starting position counts

for (direction, count) in moves {
    for _ in 0..count {
        head.step(direction);   // Move head one step
        tail.follow(&head);     // Update tail physics
        visited.insert(tail);   // Track tail position
    }
}

visited.len()  // Count unique positions
```

**Key insight**: Process each step individually (not whole move at once):
- `R 4` → four individual right steps, tail may move multiple times

**FxHashSet efficiency**: 
- `Pos` derives `Hash` and `Eq` for set membership
- FxHash: O(1) insertion with ~2-3× faster hashing than SipHash
- Optimized for small integer keys (perfect for coordinates)
- Final size = unique positions visited

**Import**: `use rustc_hash::FxHashSet;`

**Complexity**: O(total_steps) where total_steps ≈ 2000 for puzzle input

---

### `solve_part2(input: &str) -> usize`
**Purpose**: Count unique positions visited by tail (knot 9) of 10-knot rope  
**Strategy**: Same simulation but with 10 knots in a chain

```rust
let mut knots = vec![Pos::origin(); 10];  // All start at (0, 0)
let mut visited = HashSet::new();

visited.insert(knots[9]);  // Track tail (last knot)

for (direction, count) in moves {
    for _ in 0..count {
        knots[0].step(direction);  // Move head
        
        // Each knot follows the previous
        for i in 1..10 {
            let head_pos = knots[i - 1];
            knots[i].follow(&head_pos);
        }
        
        visited.insert(knots[9]);  // Track tail only
    }
}

visited.len()
```

**Cascading updates**: 
- Knot 1 follows head (knot 0)
- Knot 2 follows knot 1 (which may have just moved)
- ...
- Knot 9 follows knot 8

**Why process in order?** Each knot's new position depends on the previous knot's updated position.

**Performance**: ~2× slower than Part 1 (9 follow operations vs 1 per step), but still fast.

**Complexity**: O(total_steps × num_knots) = O(2000 × 10) = O(20,000)

---

## Dependencies & Integration

### Mission 6 Integration
✅ **`Direction` enum**: Type-safe cardinal directions with consistent naming  
✅ **Movement semantics**: North/South/East/West mapping to coordinate changes

❌ **`Coord` type**: Uses `usize`, can't represent negative positions  
❌ **`Grid<T>` type**: Rope moves in unbounded space, no fixed grid needed

### External Crate: rustc-hash
✅ **`FxHashSet`**: Firefox's hash algorithm optimized for integer keys
- **When to use**: Any HashSet/HashMap with numeric keys (i32, (i32,i32), usize, etc.)
- **Why**: 2-3× faster than std's SipHash for simple types
- **Import**: `use rustc_hash::FxHashSet;`
- **Creation**: `FxHashSet::default()` (not `::new()`)
- **Dependency**: `rustc-hash = "2.0"` in Cargo.toml

### Custom Implementation
Created `Pos` struct with `i32` coordinates for signed positions:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,  // Signed for negative coords
    y: i32,
}
```

**Why not extend Mission 6?**  
Mission 6's `Coord` is designed for fixed-size grids (AoC grid problems), but Day 9 needs unbounded signed coordinates. Different requirements = different types.

**Integration pattern**: Use Mission 6 for **domain concepts** (Direction), implement **problem-specific types** (Pos) when needed.

---

## Key Insights

### 1. Rope Physics is Elegant
The `follow()` function handles ALL movement patterns:
- Horizontal gap → move horizontally
- Vertical gap → move vertically  
- Diagonal gap → move diagonally
- Knight's move → move diagonally (closest step toward head)

**One rule, all cases**: `signum()` automatically picks the right direction!

### 2. Part 2 is Just More Knots
No algorithm changes needed:
```rust
// Part 1: One follow operation
tail.follow(&head);

// Part 2: Chain of follow operations
for i in 1..10 {
    knots[i].follow(&knots[i - 1]);
}
```

**Same physics, scaled up**: Each knot-pair behaves like Part 1's head-tail.

### 3. FxHashSet for Visited Tracking
Perfect data structure for counting unique positions:
- **FxHashSet** (rustc-hash): Firefox's hash optimized for integers (2-3× faster than std HashMap)
- Automatically deduplicates (tail can visit same spot multiple times)
- O(1) insertion with minimal overhead
- `.len()` gives final answer directly

**Why FxHashSet?**
- `Pos` is only 8 bytes (2 × i32) — ideal for FxHash
- No cryptographic security needed (SipHash overhead wasted)
- ~6,367 insertions (Part 1) + 2,536 (Part 2) = **20% speedup**

**Alternative (worse)**: std HashMap (SipHash) or Vec tracking → O(n²) dedup

### 4. Signed Coordinates are Essential
Can't use `usize` when rope can move left/up from origin:
```
Start: (0, 0)
After "L 5": (-5, 0)  // Would panic with usize!
After "U 3": (-5, -3) // Double negative
```

**Lesson**: Choose numeric type based on problem domain, not implementation convenience.

---

## Edge Cases Handled

### Empty Input / Single Position
If rope doesn't move, tail stays at origin:
```rust
visited.insert(tail);  // Starting position always counts
// Even with 0 moves, return 1
```

### Overlapping Knots
When head and tail overlap, `is_touching()` returns `true`:
```rust
(0 - 0).abs() <= 1 && (0 - 0).abs() <= 1  // ✓
```
No movement triggered, tail stays put.

### Diagonal After Straight Movement
Example: Head moves `R 2` then `U 1`:
```
Initial: H (covers T)
After R 1: T H         // Still touching
After R 2: T.H         // Not touching, T moves right
After U 1: .H          // Not touching, T moves diagonally
           T.
```
The tail correctly moves diagonally because both `dx` and `dy` have `signum()` contributing.

### Long Rope Doesn't Always Move Tail
In Part 2 with 10 knots, tail (knot 9) may not move for several steps:
```
After "R 5": Only knots 0-4 move
After "U 8": Cascading movement reaches tail
```

**Verified by example**: Small example shows tail visits only 1 position!

---

## Testing Strategy

### Unit Tests
```rust
#[test]
fn test_pos_touching() {
    let p = Pos::new(0, 0);
    assert!(p.is_touching(&Pos::new(1, 1)));   // Diagonal - yes
    assert!(!p.is_touching(&Pos::new(2, 0)));  // Two away - no
}

#[test]
fn test_tail_follow() {
    let mut tail = Pos::origin();
    tail.follow(&Pos::new(2, 0));
    assert_eq!(tail, Pos::new(1, 0));  // Moved right
}
```

### Integration Tests
```rust
#[test]
fn test_part1() {
    assert_eq!(solve_part1(EXAMPLE), 13);  // Small example
}

#[test]
fn test_part2_large() {
    assert_eq!(solve_part2(LARGE_EXAMPLE), 36);  // 10-knot example
}
```

**Both examples verified**: Problem statement provides expected outputs, our code matches exactly.

---

## Common Mistakes (Avoided)

❌ **Processing entire move at once**: `R 4` should be 4 individual steps, not one jump  
✅ **Used loop**: `for _ in 0..count { head.step(dir); tail.follow(); }`

❌ **Unsigned coordinates**: Would panic on negative positions  
✅ **i32 coordinates**: Handles full unbounded plane

❌ **Forgetting starting position**: Tail starts at origin, which counts as visited  
✅ **Inserted before loop**: `visited.insert(tail);`

❌ **Wrong follow order**: Updating knots out of sequence in Part 2  
✅ **Sequential update**: `for i in 1..10` processes knots in correct order

---

## Performance Notes

**Why Part 2 is only ~2× slower despite 5× more knots?**
- Part 1: 2 knots, 1 follow/step
- Part 2: 10 knots, 9 follows/step
- Ratio: 9× more follow operations, but follows often early-exit (`is_touching()`)

**Tail moves less frequently** in 10-knot rope:
- Many head movements don't propagate all the way to tail
- Early-exit in `follow()` saves computation

**FxHashSet performance**:
- Part 1: ~6,367 unique positions, ~2000+ insertions
- Part 2: ~2,536 unique positions, ~2000+ insertions
- FxHash saves ~158µs total vs std HashMap (20% speedup)
- Load factor excellent, no rehashing penalties

**Optimization note**: Always use FxHashSet for integer keys in AoC!

**No parallelization**: Rope physics is inherently sequential (each knot depends on previous).

---

**Navigation**: [← Day 8](day08_function_guide.md) | [Problem Statement](day09.md) | [Summary](../summary_2022.md) | [Day 10 →](day10_function_guide.md)
