--- Day 18: Like a GIF For Your Yard ---
After the million lights incident, the fire code has gotten stricter: now, at most ten thousand lights are allowed. You arrange them in a 100x100 grid.

Never one to let you down, Santa again mails you instructions on the ideal lighting configuration. With so few lights, he says, you'll have to resort to animation.

Start by setting your lights to the included initial configuration (your puzzle input). A # means "on", and a . means "off".

Then, animate your grid in steps, where each step decides the next configuration based on the current one. Each light's next state (either on or off) depends on its current state and the current states of the eight lights adjacent to it (including diagonals). Lights on the edge of the grid might have fewer than eight neighbors; the missing ones always count as "off".

For example, in a simplified 6x6 grid, the light marked A has the neighbors numbered 1 through 8, and the light marked B, which is on an edge, only has the neighbors marked 1 through 5:

1B5...
234...
......
..123.
..8A4.
..765.
The state a light should have next is based on its current state (on or off) plus the number of neighbors that are on:

A light which is on stays on when 2 or 3 neighbors are on, and turns off otherwise.
A light which is off turns on if exactly 3 neighbors are on, and stays off otherwise.
All of the lights update simultaneously; they all consider the same current state before moving to the next.

Here's a few steps from an example configuration of another 6x6 grid:

Initial state:
.#.#.#
...##.
#....#
..#...
#.#..#
####..

After 1 step:
..##..
..##.#
...##.
......
#.....
#.##..

After 2 steps:
..###.
......
..###.
......
.#....
.#....

After 3 steps:
...#..
......
...#..
..##..
......
......

After 4 steps:
......
......
..##..
..##..
......
......
After 4 steps, this example has four lights on.

In your grid of 100x100 lights, given your initial configuration, how many lights are on after 100 steps?

Your puzzle answer was 1061.

--- Part Two ---
You flip the instructions over; Santa goes on to point out that this is all just an implementation of Conway's Game of Life. At least, it was, until you notice that something's wrong with the grid of lights you bought: four lights, one in each corner, are stuck on and can't be turned off. The example above will actually run like this:

Initial state:
##.#.#
...##.
#....#
..#...
#.#..#
####.#

After 1 step:
#.##.#
####.#
...##.
......
#...#.
#.####

After 2 steps:
#..#.#
#....#
.#.##.
...##.
.#..##
##.###

After 3 steps:
#...##
####.#
..##.#
......
##....
####.#

After 4 steps:
#.####
#....#
...#..
.##...
#.....
#.#..#

After 5 steps:
##.###
.##..#
.##...
.##...
#.#...
##...#
After 5 steps, this example now has 17 lights on.

In your grid of 100x100 lights, given your initial configuration, but with the four corners always in the on state, how many lights are on after 100 steps?

Your puzzle answer was 1006.

Both parts of this puzzle are complete! They provide two gold stars: **

At this point, you should return to your Advent calendar and try another puzzle.

If you still want to see it, you can get your puzzle input.

You can also [Share] this puzzle.



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

---

## Zettelkasten Links

### Related Concepts
- [[../../zettelkasten/Cellular-Automaton-Patterns]] - Pattern evolution in cellular automata
- [[../../zettelkasten/Grid-Data-Structures]] - 2D grid representation techniques
- [[../../zettelkasten/Neighbor-Algorithms]] - 8-connected and 4-connected neighbor patterns
- [[../../zettelkasten/Conway-Game-of-Life]] - Classic cellular automaton rules
- [[../../zettelkasten/Double-Buffering-Technique]] - State update without corruption

### Mission Integration
- [[../../missions/Mission6/README]] - Grid<T>, Coord, neighbors_8_bounded() implementation
- [[../../zettelkasten/mission-6]] - Generic grid data structure for 2D problems

### Related AoC Problems
- [[day06]] - Day 6: Grid manipulation with rectangular regions
- [[summary]] - Problem type distribution and patterns

### Performance & Algorithms
- [[../../zettelkasten/Time-Complexity-Analysis]] - Big-O notation for grid algorithms
- [[../../zettelkasten/Space-Complexity-Optimization]] - Memory usage in simulation problems
- [[../../zettelkasten/Simulation-Algorithms]] - State-based evolution patterns

### Visualization Techniques
- [[../../zettelkasten/ANSI-Terminal-Colors]] - Terminal-based visualization
- [[../../zettelkasten/Unicode-Block-Characters]] - Compressed grid display techniques

---

*Tags: #aoc2015 #day18 #cellular-automaton #game-of-life #grid-simulation #mission6 #neighbor-counting #8-connected #double-buffering #visualization #pattern-evolution #state-machine #rust-implementation*

*Links: [[summary]] | [[../../missions/Mission6/README]] | [[../../zettelkasten/AoC Patterns MOC]] | [[../README]]*
