# Day 15: Beacon Exclusion Zone - Function Guide

**Problem**: Find positions where beacons cannot exist based on Manhattan distance coverage from sensors to their nearest beacons.

**Navigation**: [← Day 14](day14_function_guide.md) | [Problem](day15.md) | [Code](../../../src/solver/day15.rs) | [Summary](../summary_2022.md) | [Day 16 →](day16_function_guide.md)

---

## Overview

### Problem Summary
- **Part 1**: Count positions in row y=2,000,000 where a beacon cannot exist
- **Part 2**: Find the ONE position in [0, 4M] × [0, 4M] where a beacon could exist, calculate tuning frequency = x × 4,000,000 + y

### Performance
- **Parse**: 4.59µs (extract 4 numbers per line)
- **Part 1**: 237ns (single row interval merging)
- **Part 2**: 446.74ms (scan 4 million rows)
- **Combined runtime**: 432.52ms ✅ Parse-once verified
- **Complexity**: Part 1 O(n), Part 2 O(n × m) where n=sensors, m=4M rows

### Key Insight
**Interval merging instead of grid storage**: With coordinates spanning millions, we can't materialize a grid. Instead:
1. Calculate X-coverage intervals for each row
2. Merge overlapping intervals
3. Count positions or find gaps

**Sparse representation wins**: Parse (4.6µs) + interval merging beats any dense grid approach.

---

## Algorithm Analysis

### Manhattan Distance Coverage

**Definition**: Distance between points `(x₁, y₁)` and `(x₂, y₂)`:
```
d = |x₁ - x₂| + |y₁ - y₂|
```

**Coverage Diamond**: Sensor at `(sx, sy)` with radius `R` covers all points within Manhattan distance `R`:
```
        #           ← R=2
      #####
    #####S#####     ← Sensor at center
      #####
        #
```

**Key Property**: At target row `y`:
- Vertical distance: `|sy - y|`
- Remaining horizontal radius: `R - |sy - y|`
- X-range covered: `[sx - remaining, sx + remaining]` (if remaining ≥ 0)

---

### Part 1: Single Row Coverage

**Goal**: Count impossible positions in row y=2,000,000.

**Algorithm**:
```rust
fn count_impossible_positions(sensors: &[Sensor], target_row: i32) -> usize {
    // 1. Calculate X-intervals for each sensor at target_row
    intervals = sensors.filter_map(|s| {
        radius = manhattan_distance(s.pos, s.beacon);
        vertical_dist = |s.y - target_row|;
        remaining = radius - vertical_dist;

        if remaining >= 0 {
            Some((s.x - remaining, s.x + remaining))
        } else {
            None  // Sensor doesn't reach target_row
        }
    });

    // 2. Sort intervals by start position
    intervals.sort();

    // 3. Merge overlapping intervals
    merged = merge_intervals(intervals);

    // 4. Count total positions covered
    total = merged.sum(|interval| interval.end - interval.start + 1);

    // 5. Subtract beacons actually on target_row
    total - beacons_on_row.count()
}
```

**Example** (from problem, y=10):
```
Sensor at (8, 7), beacon at (2, 10) → radius = 9
Vertical distance to y=10: |7 - 10| = 3
Remaining horizontal: 9 - 3 = 6
X-coverage: [8-6, 8+6] = [2, 14]

After merging all sensors: 26 positions cannot have beacon
```

**Result**: 4,876,693 positions

---

### Part 2: Find the Gap

**Goal**: Find ONE uncovered position in [0, 4M] × [0, 4M].

**Insight**: Only one gap exists, at the boundary of sensor coverage.

**Algorithm**:
```rust
fn find_distress_beacon(sensors: &[Sensor], max_coord: i32) -> i64 {
    for y in 0..=max_coord {
        // Get coverage intervals for this row (clamped to [0, max_coord])
        intervals = calculate_intervals_at_row(sensors, y);
        intervals.clamp_to(0, max_coord);

        // Sort and merge
        intervals.sort();
        merged = merge_intervals(intervals);

        // Check for gaps:
        // 1. First interval doesn't start at 0?
        if merged[0].start > 0 {
            return y as i64  // Gap at x=0
        }

        // 2. Gap between intervals?
        for adjacent_pair in merged.windows(2) {
            if adjacent_pair[0].end + 1 < adjacent_pair[1].start {
                x = adjacent_pair[0].end + 1;
                return x * 4_000_000 + y;  // Tuning frequency!
            }
        }

        // 3. Last interval doesn't reach max_coord?
        if merged.last().end < max_coord {
            return max_coord * 4_000_000 + y;
        }
    }
}
```

**Why this works**: The distress beacon must be in the ONE position not covered by any sensor. We scan all 4 million rows looking for the gap.

**Example** (search area [0, 20]):
```
At y=11, coverage has gap:
Intervals after merge: [0, 13], [15, 20]
Gap at x=14 → Tuning frequency = 14 × 4,000,000 + 11 = 56,000,011
```

**Result**: 11,645,454,855,041 (x=2,911,363, y=2,855,041)

---

## Implementation Details

### Interval Merging

**Key Function**:
```rust
fn merge_intervals(intervals: &[(i32, i32)]) -> Vec<(i32, i32)> {
    // Assumes intervals are sorted!
    let mut merged = vec![intervals[0]];

    for &(start, end) in &intervals[1..] {
        let last = merged.last_mut().unwrap();

        if start <= last.1 + 1 {
            // Overlapping or adjacent - extend
            last.1 = last.1.max(end);
        } else {
            // Gap - new interval
            merged.push((start, end));
        }
    }

    merged
}
```

**Note**: `start <= last.end + 1` handles both overlapping and **adjacent** intervals:
- `[1, 5]` and `[6, 10]` → merge to `[1, 10]` (adjacent at 5+1=6)
- `[1, 5]` and `[4, 10]` → merge to `[1, 10]` (overlapping at 4-5)
- `[1, 5]` and `[8, 10]` → keep separate (gap at 6-7)

### Parse Pattern

**Extract numbers with sign handling**:
```rust
fn parse_input(input: &str) -> Vec<Sensor> {
    input.lines()
        .map(|line| {
            // Split on non-numeric, non-minus characters
            let nums: Vec<i32> = line
                .split(|c: char| !c.is_numeric() && c != '-')
                .filter(|s| !s.is_empty())
                .filter_map(|s| s.parse().ok())
                .collect();

            Sensor { x: nums[0], y: nums[1], beacon_x: nums[2], beacon_y: nums[3] }
        })
        .collect()
}
```

**Handles negatives**: "x=-2, y=15" → splits to ["", "-2", "", "", "15", ...] → filters to [-2, 15]

---

## Performance Analysis

### Why Part 2 is 446ms

**Work per row**:
- Calculate intervals: O(n) for n sensors
- Sort: O(n log n)
- Merge: O(n)
- Check gaps: O(n)
- **Total per row**: O(n log n)

**4 million rows**: 4,000,001 × O(n log n) ≈ 446ms

**Could we optimize?** Theoretically yes (perimeter checking), but 446ms is acceptable for this problem.

### Parse-Once Verification

**Benchmark proof**:
```
day15_parse:     4.59µs  ← Parsing cost
day15_part1:     237ns   ← Just one row
day15_part2:     446.74ms ← 4M rows
day15_combined:  432.52ms ← Parse + both parts
```

**Verification**: Combined (432ms) ≈ Parse (4.6µs) + Part1 (237ns) + Part2 (446ms) ✅

---

## Edge Cases

### Beacons on Target Row
**Part 1 pitfall**: Coverage includes beacon positions, but beacons CAN exist there!

**Solution**: Count beacons actually on target row, subtract from coverage:
```rust
let beacons_on_row: HashSet<i32> = sensors
    .filter(|s| s.beacon_y == target_row)
    .map(|s| s.beacon_x)
    .collect();

total_coverage - beacons_on_row.len()
```

### Gap at Boundaries
**Part 2 edge cases**:
1. Gap at x=0 (first interval starts > 0)
2. Gap between intervals (most common)
3. Gap at x=max_coord (last interval ends < max_coord)

All three handled explicitly in `find_distress_beacon()`.

### Adjacent vs Overlapping
**Critical distinction**:
- `[1, 5]` and `[6, 10]` are **adjacent** → merge to `[1, 10]`
- `[1, 5]` and `[7, 10]` have **gap at 6** → don't merge

**Implementation**: Use `start <= last_end + 1` to catch both!

---

## Key Takeaways

1. **Sparse coverage → Intervals, not grids**: When coordinates are too large to materialize, use interval arithmetic
2. **Manhattan distance coverage**: Diamond shape, easy to calculate X-range at any Y
3. **Interval merging pattern**: Sort → merge overlapping/adjacent → count or find gaps
4. **Parse-once pays off**: Even 4.6µs matters when baseline is ~446ms
5. **Gap finding**: Check all three locations (start, between, end) to avoid off-by-one errors

---

**Answer**: Part 1: `4876693` | Part 2: `11645454855041`

**Related patterns**: [[interval-merging]], [[manhattan-distance]], [[sparse-coverage]], [[parse-once-pattern]]
