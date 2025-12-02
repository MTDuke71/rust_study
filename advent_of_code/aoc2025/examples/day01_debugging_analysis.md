# AoC 2025 Day 1 - Debugging Analysis: Safe Dial Problem

## Problem Summary

AoC 2025 Day 1 involves a safe dial with positions 0-99 that rotates based on L/R instructions. Part 2 requires counting how many times the dial points to 0 during rotations (not just final positions).

## Initial Implementation Issues

### Issue 1: Overcounting from Position 0

**Problem**: Initial answer was 6037 (too high)

**Root Cause**: When starting at position 0, the algorithm was counting zeros during rotation cycles incorrectly. For example:
- Starting at 0 with L50: Should not count any "during rotation" zeros
- Starting at 0 with L200: Was counting 2 zeros for completing 2 cycles, but this was overcounting

**Feedback Received**: 
```python
if operation == "L":
    sign = -1
    if (dial_position % 100 != 0) and dial_position % 100 < operand % 100:
        password_accum += 1
```

The key insight: "If the dial is already at 0, and you rotate it left by a distance of 10, you shouldn't count that."

### Issue 2: Overcorrection 

**Problem**: After applying the feedback too broadly, answer became 5015 (too low)

**Root Cause**: Completely eliminated counting any zeros during rotation when starting from position 0, which was too aggressive.

### Issue 3: Complex Logic vs Simple Arithmetic

**Problem**: Multiple attempts with complex cycle counting logic led to answers between 5015-6037

**Root Cause**: The original approach tried to manually calculate how many times the dial passes through 0 during rotation using complex modular arithmetic and cycle detection.

## Final Working Solution

The correct approach uses simple signed integer arithmetic:

```rust
pub fn solve_part2(input: &str) -> Result<String> {
    let mut zero_count = 0i32;
    let mut dial = 50i32; // Start at 50

    for rotation in rotations {
        // Special case: Left rotation from 0
        if dial == 0 && rotation.direction == Direction::Left {
            dial = 100; // Convert 0 to 100 before left rotation
        }
        
        // Apply rotation with signed arithmetic
        dial = match rotation.direction {
            Direction::Left => dial - rotation.distance as i32,
            Direction::Right => dial + rotation.distance as i32,
        };
        
        // Count boundary crossings (100s place = times through 0)
        zero_count += (dial / 100).abs();
        
        // Handle wrap-around below 1
        if dial < 1 {
            zero_count += 1;
        }
        
        // Normalize to 0-99 range
        dial = dial % 100;
        if dial < 0 {
            dial = dial + 100;
        }
    }

    Ok(zero_count.to_string())
}
```

## Key Insights from Working Solution

### 1. Special Case for Left from 0
```rust
if dial == 0 && rotation.direction == Direction::Left {
    dial = 100;
}
```
This prevents the edge case where starting at 0 and going left would incorrectly handle the boundary crossing.

### 2. Boundary Crossing Detection
```rust
zero_count += (dial / 100).abs();
```
Instead of complex cycle detection, simply count how many 100s we've crossed. This naturally handles:
- Right rotations that cross 100+ (e.g., 50 + 75 = 125 → crosses once)
- Left rotations that go negative (e.g., 25 - 50 = -25 → crosses once)

### 3. Wrap-Around Handling
```rust
if dial < 1 {
    zero_count += 1;
}
```
Handles the specific case where we go below position 1, which represents wrapping from 0 to 99.

### 4. Signed Arithmetic
Using `i32` instead of `u32` allows negative values during calculation, making boundary detection natural rather than requiring complex modular arithmetic.

## Results Progression

| Approach | Answer | Status | Issue |
|----------|--------|--------|-------|
| Initial complex logic | 6037 | Too high | Overcounting from position 0 |
| No counting from 0 | 5015 | Too low | Overcorrection |
| Conservative cycles | 5830 | Too low | Still undercounting |
| Simple arithmetic | 5941 | ✅ Correct | Proper boundary detection |

## Lessons Learned

1. **Simple arithmetic often beats complex logic** - The boundary crossing detection using division is much simpler than manual cycle counting
2. **Edge cases matter** - The special handling for Left rotation from position 0 was crucial
3. **Signed arithmetic is your friend** - Using signed integers made negative handling natural
4. **Test incrementally** - The provided test sequence (R50, R50, L50, L50, R75, L50 → 1,1,2,2,3,4) was invaluable for debugging

## Final Answer: 5941 ✅

The working solution correctly handles all edge cases and provides the right answer through elegant signed arithmetic rather than complex boundary detection logic.

---

*Links: [[../Problem_Statements/day01]] | [[../Problem_Statements/summary]] | [[../../aoc_pattern_recognition/README]]*

*Tags: #aoc #2025 #day01 #debugging #signed-arithmetic #edge-cases #lessons-learned*