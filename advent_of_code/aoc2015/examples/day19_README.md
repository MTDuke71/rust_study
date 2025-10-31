# Day 19: Medicine for Rudolph - Implementation Notes

## Problem Summary

Day 19 involves a molecular replacement system where:
- **Part 1**: Count distinct molecules that can be generated with one replacement step
- **Part 2**: Find minimum steps to create a target molecule starting from 'e'

## Algorithm Details

### Part 1: Molecule Generation
- Parse replacement rules (e.g., "H => HO")
- For each rule, find all positions where the pattern matches in the input molecule
- Generate new molecules by applying each possible replacement
- Return count of unique molecules

**Example**: From "HOH" with rules "H => HO", "H => OH", "O => HH":
- Position 0 H: "HOH" → "HOOH" (H=>HO), "OHOH" (H=>OH)  
- Position 2 H: "HOH" → "HOHO" (H=>HO), "HOOH" (H=>OH)
- Position 1 O: "HOH" → "HHHH" (O=>HH)
- Unique results: {HOOH, HOHO, OHOH, HHHH} = 4 molecules

### Part 2: Reverse Construction  
- Use greedy reverse approach: start from target, work backwards to 'e'
- Sort replacement rules by length (longest first) for maximum progress per step
- Apply first applicable reverse replacement until reaching 'e'

**Note**: This greedy approach works for AoC 2015 Day 19 due to the specific structure of the rules, but may not be optimal for all possible rule sets.

## Key Implementation Features

### Robust Input Parsing
```rust
fn parse_input(input: &str) -> Result<(Vec<Replacement>, String)>
```
- Handles blank line separation between rules and molecule
- Validates rule format ("from => to")
- Provides detailed error messages with line numbers

### Efficient Pattern Matching
```rust
fn generate_molecules(replacements: &[Replacement], molecule: &str) -> HashSet<String>
```
- Uses `String::find()` with sliding window to find all pattern occurrences
- Handles overlapping patterns correctly
- Uses `HashSet` to automatically deduplicate results

### Context-Independent Replacement
The algorithm correctly handles replacements without regard for surrounding characters:
- "H2O" with rule "H => OO" becomes "OO2O"
- Numbers and other characters are preserved as-is

## Test Coverage

Comprehensive test suite covering:
- ✅ Input parsing validation
- ✅ Simple molecule generation (HOH example)
- ✅ Complex cases (HOHOHO → 7 molecules)
- ✅ Context-independent replacements
- ✅ Edge cases and error handling

## Performance Characteristics

- **Time Complexity**: O(n × m × k) where:
  - n = length of input molecule
  - m = number of replacement rules  
  - k = average length of replacement results
- **Space Complexity**: O(r) where r = number of unique result molecules
- **Optimizations**: 
  - HashSet for O(1) duplicate detection
  - Greedy longest-first ordering for Part 2

## Usage Examples

### Basic Usage
```rust
use aoc2015::solver::day19;

let input = "H => HO\nH => OH\nO => HH\n\nHOH";
let part1 = day19::solve_part1(input)?; // "4"
let part2 = day19::solve_part2(input)?; // May fail if no path from 'e'
```

### Running the Demo
```bash
cd advent_of_code/aoc2015
cargo run --example day19_demo
```

## Files Structure

```
src/solver/day19.rs        # Main implementation
examples/day19_demo.rs      # Interactive demonstration  
inputs/day19_example.txt    # Example input data
```

## Error Handling

The implementation provides detailed error context for common issues:
- Invalid rule format: "Invalid replacement rule format at line X"
- Missing molecule: "No starting molecule found after blank line"  
- Unreachable target: "Cannot reduce molecule 'X' further"
- Input format issues with line numbers and context

## Algorithm Insights

1. **Replacement Order**: The order of applying replacements doesn't affect Part 1 count
2. **Pattern Overlap**: Multiple occurrences of the same pattern are handled correctly
3. **Greedy Strategy**: Part 2 uses longest-replacement-first greedy approach
4. **Termination**: Built-in safeguards prevent infinite loops (max 1000 steps)

This implementation successfully handles the AoC 2015 Day 19 problem with robust error handling, comprehensive testing, and clear documentation of the algorithmic approach.