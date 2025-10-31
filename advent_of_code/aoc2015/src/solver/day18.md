# Day 18: Like a GIF For Your Yard

## Problem Description

After the [million lights incident](https://adventofcode.com/2015/day/6), you're gifted an even grander display - animated lights! Your neighbors implement Conway's Game of Life on their 100x100 grid.

### Game of Life Rules

Each light follows these rules based on its 8 surrounding neighbors:

- **Light ON** with 2-3 neighbors ON → **stays ON**
- **Light ON** with 0-1 or 4+ neighbors ON → **turns OFF**  
- **Light OFF** with exactly 3 neighbors ON → **turns ON**
- **Light OFF** otherwise → **stays OFF**

### Part 1

Starting with the given configuration, how many lights are ON after 100 steps?

### Part 2

Your neighbors discover 4 lights are **stuck in the ON position** at the corners:
- (0, 0) - top-left
- (0, 99) - top-right  
- (99, 0) - bottom-left
- (99, 99) - bottom-right

These lights ALWAYS stay ON regardless of neighbor count. How many lights are ON after 100 steps with stuck corners?

## Solution Approach

### Core Algorithm

This is a classic **cellular automaton** problem - Conway's Game of Life variant.

**Key insight**: Use Mission 6's `Grid<T>` for efficient 2D representation and neighbor iteration.

```rust
// Grid from Mission 6 provides:
// - Grid<bool> for light states (true = ON, false = OFF)
// - Coord for (row, col) positions  
// - neighbors_8_bounded() for safe 8-connected neighbor iteration
```

### Implementation Strategy

1. **Parse input** → `Grid<bool>` (# = ON, . = OFF)
2. **Count neighbors** → Use `neighbors_8_bounded()` for 8-connected cells
3. **Apply rules** → Create new grid based on Game of Life rules
4. **Iterate** → Repeat for 100 steps
5. **Part 2 modification** → Force corners to ON after each step

### Data Structures

```rust
use mission6::{Grid, Coord};

// Light grid (100x100)
let grid: Grid<bool> = Grid::new(100, 100, false);

// Count ON neighbors for a cell
fn count_neighbors_on(grid: &Grid<bool>, coord: Coord) -> usize {
    neighbors_8_bounded(&coord, grid.width(), grid.height())
        .filter(|&neighbor| grid[neighbor])
        .count()
}
```

### Performance Analysis

**Time Complexity**: O(steps × width × height × 8)
- 100 steps × 100 × 100 × 8 neighbors = 8 million operations
- Very fast with Rust's zero-cost abstractions

**Space Complexity**: O(width × height)
- Two grids: current state + next state (10,000 bools each)
- ~20KB memory total

### Mission 6 Integration

**Why Mission 6 Grid is perfect for this**:

1. **Safe bounds checking**: `neighbors_8_bounded()` prevents edge panics
2. **Efficient indexing**: `grid[coord]` with O(1) access
3. **Generic storage**: `Grid<bool>` optimized by compiler
4. **Iterator support**: Seamless integration with Rust iterators

```rust
// Clean neighbor counting with Mission 6
fn count_neighbors_on(grid: &Grid<bool>, coord: Coord) -> usize {
    neighbors_8_bounded(&coord, grid.width(), grid.height())
        .filter(|&neighbor| grid[neighbor])
        .count()
}

// Without Mission 6, you'd need manual bounds checking:
fn count_neighbors_unsafe(grid: &Vec<Vec<bool>>, row: usize, col: usize) -> usize {
    let mut count = 0;
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 { continue; }
            let nr = row as i32 + dr;
            let nc = col as i32 + dc;
            if nr >= 0 && nr < 100 && nc >= 0 && nc < 100 {
                if grid[nr as usize][nc as usize] {
                    count += 1;
                }
            }
        }
    }
    count
}
```

## Implementation Details

### Parsing

```rust
fn parse_grid(input: &str) -> Result<Grid<bool>> {
    let lines: Vec<&str> = input.trim().lines().collect();
    let height = lines.len();
    let width = lines[0].len();
    
    let mut grid = Grid::new(width, height, false);
    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                grid[Coord::new(row, col)] = true;
            }
        }
    }
    Ok(grid)
}
```

### Simulation Step

```rust
fn simulate_step(grid: &Grid<bool>) -> Grid<bool> {
    let mut next = Grid::new(grid.width(), grid.height(), false);
    
    for row in 0..grid.height() {
        for col in 0..grid.width() {
            let coord = Coord::new(row, col);
            let on_neighbors = count_neighbors_on(grid, coord);
            let currently_on = grid[coord];
            
            // Game of Life rules
            next[coord] = match (currently_on, on_neighbors) {
                (true, 2..=3) => true,  // Stay ON with 2-3 neighbors
                (false, 3) => true,     // Turn ON with exactly 3 neighbors
                _ => false,             // Otherwise OFF
            };
        }
    }
    next
}
```

### Part 2: Stuck Corners

```rust
fn stick_corners_on(grid: &mut Grid<bool>) {
    let corners = [
        Coord::new(0, 0),                              // Top-left
        Coord::new(0, grid.width() - 1),               // Top-right
        Coord::new(grid.height() - 1, 0),              // Bottom-left
        Coord::new(grid.height() - 1, grid.width() - 1), // Bottom-right
    ];
    
    for corner in &corners {
        grid[*corner] = true;
    }
}
```

## Results

- **Part 1**: `1061` lights ON after 100 steps
- **Part 2**: `1006` lights ON after 100 steps

### Pattern Analysis

**Part 1 Evolution**:
- Start: 5076 lights ON
- After 10 steps: ~1500 lights  
- After 50 steps: ~1100 lights
- After 100 steps: 1061 lights (stable oscillation)

**Part 2 Effect**:
- Stuck corners act as "light sources"
- Prevents total extinction of patterns
- Creates persistent activity near corners
- Final count: 1006 lights (55 fewer than Part 1)

## Visualization Examples

### Part 1 Animation
```bash
cargo run --example day18_animation_part1
```
Shows step-by-step evolution on 6x6 test grid with ANSI colors.

### Part 2 Animation  
```bash
cargo run --example day18_animation_part2
```
Demonstrates stuck corners (red) influencing nearby patterns.

### Side-by-Side Comparison
```bash
cargo run --example day18_comparison
```
Compares Part 1 vs Part 2 evolution on same initial state.

### Interactive 100x100 Simulator
```bash
cargo run --example day18_interactive
```
Full-featured simulator with:
- User-selectable mode (Part 1 or Part 2)
- Configurable steps and animation speed
- 2x2 Unicode block compression for display
- Auto-save every 25 steps
- Statistics tracking (min/max/stability)
- Adaptive speed after 100 steps

## Edge Cases Handled

1. **Grid boundaries**: `neighbors_8_bounded()` prevents out-of-bounds access
2. **Empty grid**: All lights OFF → stays OFF (no spontaneous generation)
3. **Full grid**: All lights ON → most turn OFF (overcrowding)
4. **Corner cells**: Have only 3-5 neighbors instead of 8
5. **Part 2 corners**: Force ON even with 8 ON neighbors

## Testing

```rust
#[test]
fn test_parse_grid() {
    let input = ".#\n#.";
    let grid = parse_grid(input).unwrap();
    assert!(!grid[Coord::new(0, 0)]);  // .
    assert!(grid[Coord::new(0, 1)]);   // #
}

#[test]
fn test_count_neighbors_on() {
    // 3x3 grid: ###
    //           #.#
    //           ###
    let grid = parse_grid("###\n#.#\n###").unwrap();
    assert_eq!(count_neighbors_on(&grid, Coord::new(1, 1)), 8);
}

#[test]
fn test_simulate_step() {
    // Blinker oscillator pattern
    let grid = parse_grid(".#.\n.#.\n.#.").unwrap();
    let next = simulate_step(&grid);
    // Should rotate 90 degrees: ...
    //                           ###
    //                           ...
    assert_eq!(count_lights_on(&next), 3);
}
```

## Lessons Learned

1. **Mission 6 Grid** is perfect for cellular automaton problems
2. **8-connected iteration** much cleaner with `neighbors_8_bounded()`
3. **Double buffering** (current + next grid) prevents state corruption
4. **Stuck lights** (Part 2) create interesting stable patterns
5. **Unicode block characters** enable 4x compression for terminal display

## Related Problems

- Day 6: Million Lights (grid manipulation)
- Mission 6: Generic Grid data structure
- Mission 7: Graph traversal (will use similar grid iteration)

## Optimization Notes

**Current implementation is already optimal**:
- O(1) grid access
- O(8) neighbor iteration  
- No allocations in hot loop
- Compiler inlines `grid[coord]` to direct array access

**Potential enhancements** (not needed for this problem):
- Sparse representation (hashmap of ON cells)
- Parallel simulation with Rayon
- GPU acceleration for massive grids
- Pattern detection for early termination
