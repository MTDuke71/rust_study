# Day 1 Part 2: Failed Approach Analysis

## Background

During the development of Day 1 Part 2 (counting zeros during rotations AND at final positions), an initial approach using `count_zeros_during_rotation()` was attempted but produced incorrect results. This document preserves the failed approach and analyzes why it didn't work.

## The Failed Approach

The function `count_zeros_during_rotation` attempted to explicitly count how many times the dial passes through position 0 during a single rotation:

```rust
fn count_zeros_during_rotation(current_position: u32, rotation: Rotation) -> u32 {
    const DIAL_SIZE: u32 = 100;
    let distance = rotation.distance;
    
    if distance == 0 { return 0; }
    
    match rotation.direction {
        Direction::Left => {
            if current_position == 0 {
                // Starting at 0: only count full additional cycles
                if distance >= DIAL_SIZE { distance / DIAL_SIZE } else { 0 }
            } else {
                // Starting at non-zero position
                if current_position < (distance % DIAL_SIZE) && distance > current_position {
                    // We wrap around
                    let remaining_distance = distance - current_position - 1;
                    1 + (remaining_distance / DIAL_SIZE)
                } else { 0 }
            }
        },
        Direction::Right => {
            if current_position == 0 {
                // Starting at 0: only count full additional cycles  
                if distance >= DIAL_SIZE { distance / DIAL_SIZE } else { 0 }
            } else {
                // Starting at non-zero position
                let steps_to_zero = DIAL_SIZE - current_position;
                if distance > steps_to_zero {
                    let remaining_distance = distance - steps_to_zero;
                    1 + (remaining_distance / DIAL_SIZE)
                } else { 0 }
            }
        },
    }
}
```

**Usage in solve_part2:**
```rust
fn solve_part2_old(rotations: &[Rotation]) -> u32 {
    let mut position = 50u32;
    let mut zero_count = 0u32;
    
    for &rotation in rotations {
        // Count zeros DURING rotation
        zero_count += count_zeros_during_rotation(position, rotation);
        // Apply the rotation
        position = apply_rotation(position, rotation);
        // Count if we ENDED at 0
        if position == 0 { zero_count += 1; }
    }
    zero_count
}
```

## Results

| Input | Old Approach | New Approach | Expected |
|-------|--------------|--------------|----------|
| Example (10 rotations) | 6 | 6 | 6 ✓ |
| Actual (4099 rotations) | **5046** | 5941 | 5941 ✓ |

**The old approach was 895 zeros short!**

## Specific Failures Identified

### Bug 1: L819 from position 50 (Large Left Rotations)

| Approach | Result |
|----------|--------|
| Old | **0** |
| New | **8** |
| Expected | 8 (passes through 0 eight times) |

**Analysis:**
```
Position 50, Left 819:
- Go left 50 steps → reach 0 (1st zero)
- Continue left 100 steps → reach 0 (2nd zero) [total: 150]
- Continue left 100 steps → reach 0 (3rd zero) [total: 250]
- ... (continues)
- Continue left 100 steps → reach 0 (8th zero) [total: 750]
- Continue left 69 steps → end at position 31 [total: 819]
```

**The bug:** The condition `current_position < (distance % DIAL_SIZE)` evaluates to:
```
50 < (819 % 100)
50 < 19
FALSE → returns 0!
```

The condition was meant to detect wrap-around, but `distance % 100` loses information about full cycles when `distance % 100 < current_position`.

### Bug 2: L100 from position 0 (Full Rotation from Zero)

| Approach | Result |
|----------|--------|
| Old | **3** (after R50 → 0, then L100) |
| New | **2** |
| Expected | 2 |

**Analysis:**
Starting at 0, going left 100 steps brings you back to 0. But this is:
- 1 pass through 0 during rotation (going 99→0)
- 1 final position at 0

Old approach counted `distance / DIAL_SIZE = 100/100 = 1` for "during rotation" PLUS the final position check, but the logic was inconsistent with how the problem defines "passing through 0."

## Why the New Approach Works

The successful approach uses **signed integer arithmetic** to elegantly handle all cases:

```rust
fn solve_part2_new(rotations: &[Rotation]) -> i32 {
    let mut zero_count = 0i32;
    let mut dial = 50i32;

    for &rotation in rotations {
        // Special: treat 0 as 100 when going left
        if dial == 0 && rotation.direction == Direction::Left {
            dial = 100;
        }
        
        // Apply rotation WITHOUT normalization
        dial = match rotation.direction {
            Direction::Left => dial - rotation.distance as i32,
            Direction::Right => dial + rotation.distance as i32,
        };
        
        // Count boundary crossings (how many times we passed ±100)
        zero_count += (dial / 100).abs();
        
        // Special case: going below 1 means we wrapped through 0
        if dial < 1 {
            zero_count += 1;
        }
        
        // NOW normalize to 0-99
        dial = dial % 100;
        if dial < 0 { dial = dial + 100; }
    }
    zero_count
}
```

**Key insight:** By letting `dial` go negative or above 100, the division `dial / 100` naturally counts how many "hundreds" we've crossed, which corresponds to passing through 0.

**For L819 from 50:**
```
dial = 50 - 819 = -769
(-769 / 100).abs() = 7
-769 < 1 → add 1 more
Total: 8 ✓
```

## Step-by-Step Example Comparison

Using the example input (L68, L30, R48, L5, R60, L55, L1, L99, R14, L82):

| Step | Rotation | Old (during+final) | New (boundary+below) | Both |
|------|----------|-------------------|---------------------|------|
| 1 | L68 | 1+0=1 | 0+1=1 | 1 |
| 2 | L30 | 0+0=0 | 0+0=0 | 1 |
| 3 | R48 | 0+1=1 | 1+0=1 | 2 |
| 4 | L5 | 0+0=0 | 0+0=0 | 2 |
| 5 | R60 | 1+0=1 | 1+0=1 | 3 |
| 6 | L55 | 0+1=1 | 0+1=1 | 4 |
| 7 | L1 | 0+0=0 | 0+0=0 | 4 |
| 8 | L99 | 0+1=1 | 0+1=1 | 5 |
| 9 | R14 | 0+0=0 | 0+0=0 | 5 |
| 10 | L82 | 1+0=1 | 0+1=1 | 6 |

Both get **6** for the example! The approaches give the same result through different counting mechanisms, but they diverge on edge cases with large rotations.

## Lessons Learned

1. **Explicit boundary tracking is error-prone** - The `count_zeros_during_rotation` function had complex conditional logic that failed for certain edge cases.

2. **Signed arithmetic simplifies wrap-around** - Letting values go negative and then fixing them is cleaner than preventing negatives upfront.

3. **Division naturally counts crossings** - `(value / 100).abs()` elegantly counts how many times we've crossed boundaries.

4. **Test with large values** - The example input had no rotations > 100, so the bug wasn't exposed until the actual puzzle input with values like `L819`, `L967`, `R978`, etc.

5. **Keep failed approaches for learning** - This analysis wouldn't be possible if the code was immediately deleted.

## The Dead Code

The `count_zeros_during_rotation` function and its tests remain in `day01.rs` as a historical artifact. It passes its unit tests (which test individual cases correctly) but produces wrong results when composed with the main loop because of the edge cases described above.

---

*Generated: December 2, 2025*
*AoC 2025 Day 1 - Debugging Artifact Preservation*

*Links: [[../Problem_Statements/summary]] | [[../src/solver/day01.rs]]*
