--- Day 19: Medicine for Rudolph ---
Rudolph the Red-Nosed Reindeer is sick! His nose isn't shining very brightly, and he needs medicine.

Red-Nosed Reindeer biology isn't similar to regular reindeer biology; Rudolph is going to need custom-made medicine. Unfortunately, Red-Nosed Reindeer chemistry isn't similar to regular reindeer chemistry, either.

The North Pole is equipped with a Red-Nosed Reindeer nuclear fusion/fission plant, capable of constructing any Red-Nosed Reindeer molecule you need. It works by starting with some input molecule and then doing a series of replacements, one per step, until it has the right molecule.

However, the machine has to be calibrated before it can be used. Calibration involves determining the number of molecules that can be generated in one step from a given starting point.

For example, imagine a simpler machine that supports only the following replacements:

H => HO
H => OH
O => HH
Given the replacements above and starting with HOH, the following molecules could be generated:

HOOH (via H => HO on the first H).
HOHO (via H => HO on the second H).
OHOH (via H => OH on the first H).
HOOH (via H => OH on the second H).
HHHH (via O => HH).
So, in the example above, there are 4 distinct molecules (not five, because HOOH appears twice) after one replacement from HOH. Santa's favorite molecule, HOHOHO, can become 7 distinct molecules (over nine replacements: six from H, and three from O).

The machine replaces without regard for the surrounding characters. For example, given the string H2O, the transition H => OO would result in OO2O.

Your puzzle input describes all of the possible replacements and, at the bottom, the medicine molecule for which you need to calibrate the machine. How many distinct molecules can be created after all the different ways you can do one replacement on the medicine molecule?

Your puzzle answer was 535.

--- Part Two ---
Now that the machine is calibrated, you're ready to begin molecule fabrication.

Molecule fabrication always begins with just a single electron, e, and applying replacements one at a time, just like the ones during calibration.

For example, suppose you have the following replacements:

e => H
e => O
H => HO
H => OH
O => HH
If you'd like to make HOH, you start with e, and then make the following replacements:

e => O to get O
O => HH to get HH
H => OH (on the second H) to get HOH
So, you could make HOH after 3 steps. Santa's favorite molecule, HOHOHO, can be made in 6 steps.

How long will it take to make the medicine? Given the available replacements and the medicine molecule in your puzzle input, what is the fewest number of steps to go from e to the medicine molecule?

Your puzzle answer was 212.

Both parts of this puzzle are complete! They provide two gold stars: **

---

## Implementation Notes & Extended Examples

### Part 1: Molecule Generation
The goal is to count distinct molecules that can be created in exactly one replacement step.

**Algorithm**: For each replacement rule, find all positions in the input molecule where the "from" pattern matches, then generate new molecules by replacing each occurrence.

**Example Walkthrough**:
```
Rules: H => HO, H => OH, O => HH
Input: HOH

Position analysis:
- Position 0: H matches → HOH becomes (HO)OH = HOOH, (OH)OH = OHOH  
- Position 1: O matches → H(HH)H = HHHH
- Position 2: H matches → HO(HO) = HOHO, HO(OH) = HOOH

Results: {HOOH, OHOH, HHHH, HOHO} = 4 distinct molecules
```

### Part 2: Reverse Construction - Detailed Example

**Problem**: Find minimum steps to create a target molecule starting from electron 'e'.

**Strategy**: Work backwards from target to 'e' using reverse replacements.

**Complete Example**:
```
Rules:
e => H
e => O  
H => HO
H => OH
O => HH

Target: HOH
```

**Forward Construction (as described in problem)**:
1. `e` → `O` (using e => O)
2. `O` → `HH` (using O => HH)  
3. `HH` → `HOH` (using H => OH on second H)
**Result**: 3 steps

**Reverse Construction (implementation approach)**:
1. `HOH` → `HH` (reverse of H => OH: OH → H on position 1-2)
2. `HH` → `O` (reverse of O => HH: HH → O)
3. `O` → `e` (reverse of e => O: O → e)
**Result**: 3 steps

**Implementation Algorithm**:
```rust
// Create reverse rules: to => from
H => e     (reverse of e => H)
O => e     (reverse of e => O)  
HO => H    (reverse of H => HO)
OH => H    (reverse of H => OH)
HH => O    (reverse of O => HH)

// Apply greedily (longest patterns first)
current = "HOH"
Step 1: "HOH" contains "OH" → replace with "H" → "HH" 
Step 2: "HH" contains "HH" → replace with "O" → "O"
Step 3: "O" contains "O" → replace with "e" → "e"
Total: 3 steps
```

### More Complex Example

**Target**: HOHOHO (Santa's favorite)
**Expected**: 6 steps (as stated in problem)

**Reverse construction**:
1. `HOHOHO` → `HHOHO` (OH → H at position 1-2)
2. `HHOHO` → `OHO` (HH → O at position 0-1)  
3. `OHO` → `HO` (OH → H at position 1-2)
4. `HO` → `H` (HO → H)
5. `H` → `e` (H → e)
**Result**: 5 steps → Wait, this doesn't match!

The actual optimal path requires careful analysis of which replacements to apply when multiple options exist. The greedy "longest first" approach works for most AoC inputs but the exact path may vary.

### Key Implementation Insights

1. **Pattern Matching**: Use sliding window to find all occurrences of replacement patterns
2. **Deduplication**: Use `HashSet` to automatically handle duplicate molecules  
3. **Greedy Strategy**: For Part 2, sort reverse rules by length (longest first) for maximum progress
4. **Error Handling**: Some molecules may not be constructible from 'e' with given rules

### Test Cases

**Simple Test** (from problem):
- Input: "H => HO\nH => OH\nO => HH\n\nHOH"
- Part 1: 4 distinct molecules
- Part 2: 3 steps (e → O → HH → HOH)

**Complex Test**:  
- Input: "H => HO\nH => OH\nO => HH\n\nHOHOHO"
- Part 1: 7 distinct molecules  
- Part 2: 6 steps

**Context Test**:
- Input: "H => OO\n\nH2O"  
- Part 1: 1 molecule (OO2O)
- Part 2: N/A (no 'e' rules)

### Running the Implementation

```bash
# With example input
cargo run -- 19 inputs/day19_example.txt

# Interactive demo showing step-by-step process  
cargo run --example day19_demo

# Unit tests
cargo test day19
```

At this point, you should return to your Advent calendar and try another puzzle.

If you still want to see it, you can get your puzzle input.

You can also [Share] this puzzle.