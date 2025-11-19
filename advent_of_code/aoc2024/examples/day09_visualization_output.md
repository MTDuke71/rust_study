# Day 09 Visualization Output

## Command
```powershell
cargo run -p aoc2024 --example day09_visualization advent_of_code/aoc2024/inputs/day09_small_example.txt
```

## Input
```
2333133121414131402
```

## Output

```
Day 09 Visualization using input length 19

=== Part 1 block compaction (13 frames captured) ===
Step 00: 00...111...2...333.44.5555.6666.777.888899
Step 01: 009..111...2...333.44.5555.6666.777.88889.
Step 02: 0099.111...2...333.44.5555.6666.777.8888..
Step 03: 00998111...2...333.44.5555.6666.777.888...
Step 04: 009981118..2...333.44.5555.6666.777.88....
Step 05: 0099811188.2...333.44.5555.6666.777.8.....
Step 06: 009981118882...333.44.5555.6666.777.......
Step 07: 0099811188827..333.44.5555.6666.77........
Step 08: 00998111888277.333.44.5555.6666.7.........
Step 09: 009981118882777333.44.5555.6666...........
Step 10: 009981118882777333644.5555.666............
Step 11: 00998111888277733364465555.66.............
Step 12: 0099811188827773336446555566..............

=== Part 2 whole-file compaction (5 frames captured) ===
Step 00: 00...111...2...333.44.5555.6666.777.888899
Step 01: 0099.111...2...333.44.5555.6666.777.8888..
Step 02: 0099.1117772...333.44.5555.6666.....8888..
Step 03: 0099.111777244.333....5555.6666.....8888..
Step 04: 00992111777.44.333....5555.6666.....8888..
```

## Analysis

### Part 1: Block-by-Block Compaction
- **Algorithm**: Two-pointer approach moving individual blocks from right to left
- **Steps**: 13 iterations to eliminate all gaps
- **Pattern**: Rightmost file blocks progressively fill leftmost gaps
- **Final state**: All files compacted with no internal gaps

### Part 2: Whole-File Compaction
- **Algorithm**: Greedy first-fit from highest file ID downward
- **Steps**: 5 file relocations
- **Pattern**: 
  - File 9 moves first (entire block)
  - File 2 finds gap created by file 9's move
  - File 1 relocates to fill available space
  - Files 5, 6 cannot fit to their left, remain in place
- **Final state**: More fragmented than Part 1 but files stay contiguous

### Key Differences
- Part 1 creates fully compacted disk (no gaps between files)
- Part 2 preserves file integrity but may leave gaps if files don't fit
- Part 2 processes fewer moves (5 vs 13) but considers entire file constraints

---
*Links: [[../Problem_Statements/day09]] [[../../../../zettelkasten/AoC 2024 Overview]]*
