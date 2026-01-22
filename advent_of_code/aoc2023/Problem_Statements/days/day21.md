## --- Day 21: Step Counter ---

**Zettelkasten**: [[aoc2023]] - AoC 2023 knowledge hub


You manage to catch the airship right as it's dropping someone else off on their all-expenses-paid trip to Desert Island! It even helpfully drops you off near the gardener and his massive farm.

"You got the sand flowing again! Great work! Now we just need to wait until we have enough sand to filter the water for Snow Island and we'll have snow again in no time."

While you wait, one of the Elves that works with the gardener heard how good you are at solving problems and would like your help. He needs to get his steps in for the day, and so he'd like to know which garden plots he can reach with exactly his remaining 64 steps.

He gives you an up-to-date map (your puzzle input) of his starting position (S), garden plots (.), and rocks (#). For example:

...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
The Elf starts at the starting position (S) which also counts as a garden plot. Then, he can take one step north, south, east, or west, but only onto tiles that are garden plots. This would allow him to reach any of the tiles marked O:

...........
.....###.#.
.###.##..#.
..#.#...#..
....#O#....
.##.OS####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
Then, he takes a second step. Since at this point he could be at either tile marked O, his second step would allow him to reach any garden plot that is one step north, south, east, or west of any tile that he could have reached after the first step:

...........
.....###.#.
.###.##..#.
..#.#O..#..
....#.#....
.##O.O####.
.##.O#...#.
.......##..
.##.#.####.
.##..##.##.
...........
After two steps, he could be at any of the tiles marked O above, including the starting position (either by going north-then-south or by going west-then-east).

A single third step leads to even more possibilities:

...........
.....###.#.
.###.##..#.
..#.#.O.#..
...O#O#....
.##.OS####.
.##O.#...#.
....O..##..
.##.#.####.
.##..##.##.
...........
He will continue like this until his steps for the day have been exhausted. After a total of 6 steps, he could reach any of the garden plots marked O:

...........
.....###.#.
.###.##.O#.
.O#O#O.O#..
O.O.#.#.O..
.##O.O####.
.##.O#O..#.
.O.O.O.##..
.##.#.####.
.##O.##.##.
...........
In this example, if the Elf's goal was to get exactly 6 more steps today, he could use them to reach any of 16 garden plots.

However, the Elf actually needs to get 64 steps today, and the map he's handed you is much larger than the example map.

Starting from the garden plot marked S on your map, how many garden plots could the Elf reach in exactly 64 steps?

Your puzzle answer was 3716.

--- Part Two ---
The Elf seems confused by your answer until he realizes his mistake: he was reading from a list of his favorite numbers that are both perfect squares and perfect cubes, not his step counter.

The actual number of steps he needs to get today is exactly 26501365.

He also points out that the garden plots and rocks are set up so that the map repeats infinitely in every direction.

So, if you were to look one additional map-width or map-height out from the edge of the example map above, you would find that it keeps repeating:

.................................
.....###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#....
.##...####..##...####..##...####.
.##..#...#..##..#...#..##..#...#.
.......##.........##.........##..
.##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##.
.................................
.................................
.....###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#....
.##...####..##..S####..##...####.
.##..#...#..##..#...#..##..#...#.
.......##.........##.........##..
.##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##.
.................................
.................................
.....###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#....
.##...####..##...####..##...####.
.##..#...#..##..#...#..##..#...#.
.......##.........##.........##..
.##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##.
.................................
This is just a tiny three-map-by-three-map slice of the inexplicably-infinite farm layout; garden plots and rocks repeat as far as you can see. The Elf still starts on the one middle tile marked S, though - every other repeated S is replaced with a normal garden plot (.).

Here are the number of reachable garden plots in this new infinite version of the example map for different numbers of steps:

In exactly 6 steps, he can still reach 16 garden plots.
In exactly 10 steps, he can reach any of 50 garden plots.
In exactly 50 steps, he can reach 1594 garden plots.
In exactly 100 steps, he can reach 6536 garden plots.
In exactly 500 steps, he can reach 167004 garden plots.
In exactly 1000 steps, he can reach 668697 garden plots.
In exactly 5000 steps, he can reach 16733044 garden plots.
However, the step count the Elf needs is much larger! Starting from the garden plot marked S on your infinite map, how many garden plots could the Elf reach in exactly 26501365 steps?

Your puzzle answer was 616583483179597.

Both parts of this puzzle are complete! They provide two gold stars: **

---

## Solution Analysis

### Part 1: BFS Step Counter

**Algorithm**: Breadth-first search tracking `(row, col, step)` states.

**Example - After 2 steps**:
```
...........
.....###.#.
.###.##..#.
..#.#O..#..   ← (3,5) reachable from (4,5)
....#.#....
.##O.O####.   ← (5,3), (5,5) reachable
.##.O#...#.   ← (6,4) reachable from (5,4)
.......##..
.##.#.####.
.##..##.##.
...........
```

**Key Insight**: Same position can be visited at different steps via different paths. State = (position, step_count) prevents duplicate exploration while allowing revisits.

**Result**: 3,716 plots reachable in 64 steps (7.34ms).

### Part 2: Quadratic Pattern Detection

**The Challenge**: Cannot brute-force 26,501,365 steps!

**Key Observation**: The step count is NOT arbitrary:
```
26,501,365 = 65 + (131 × 202,300)
             ↑     ↑      ↑
        edge_dist  grid   periods
                   size
```

Where:
- Grid: 131×131 (square)
- Start: (65, 65) - exact center
- 65 steps reaches grid edge
- Pattern repeats every 131 steps

**Why Quadratic?** On an infinite 2D grid, reachable area forms a diamond growing quadratically:

**Diamond Growth Pattern**:
```
After 1 step:    After 2 steps:    After 3 steps:
    O                O                  O
   OSO              OOO                OOO
    O              OOOOO              OOOOO
                    OOO              OOOOOOO
                     O                OOOOO
                                       OOO
                                        O
Area = 5         Area = 13          Area = 25
```

**Mathematical Pattern**: f(n) = an² + bn + c (quadratic)

**Solution Strategy**:
1. **Sample 3 points** (minimum for quadratic):
   - f(0) = 3,797 plots at **65 steps** (n=0)
   - f(1) = 34,009 plots at **196 steps** (n=1, 65+131)
   - f(2) = 94,353 plots at **327 steps** (n=2, 65+262)

2. **Fit quadratic** using Lagrange interpolation:
   ```
   a = (n0 - 2n1 + n2) / 2 = (3797 - 68018 + 94353) / 2 = 15,066
   b = (-3n0 + 4n1 - n2) / 2 = (-11391 + 136036 - 94353) / 2 = 15,146
   c = n0 = 3,797
   
   f(n) = 15,066n² + 15,146n + 3,797
   ```

3. **Extrapolate** to n=202,300:
   ```
   f(202,300) = 15,066 × (202,300)² + 15,146 × 202,300 + 3,797
              = 616,583,483,179,597 plots
   ```

**Infinite Grid Wrapping** (modulo arithmetic):
```rust
// Map infinite coordinates to grid cells
let grid_row = infinite_row.rem_euclid(rows);
let grid_col = infinite_col.rem_euclid(cols);
```

**Why `rem_euclid()` not `%`?**
- Standard modulo: `-3 % 11 = -3` ❌
- Euclidean modulo: `-3.rem_euclid(11) = 8` ✓

**Performance**: 1.89s (3 BFS runs vs 26M iterations brute-force)
- **Speedup**: ~800,000× faster!

**Mathematical Validation** (finite differences):
```
Δ₀ = f(1) - f(0) = 34,009 - 3,797 = 30,212
Δ₁ = f(2) - f(1) = 94,353 - 34,009 = 60,344
Δ²₀ = Δ₁ - Δ₀ = 30,132 = 2a
a = 15,066 ✓
```

---

**See Also**:
- **Complete Analysis**: [day21_function_guide.md](day21_function_guide.md) - 800+ line deep dive
- **Implementation**: `advent_of_code/aoc2023/src/solver/day21.rs`
- **Math Foundations**: `zettelkasten/math-foundations/polynomial-interpolation-lagrange.md`