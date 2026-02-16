//! Day 15: Beacon Exclusion Zone
//!
//! Calculate Manhattan distance coverage from sensors to find where beacons cannot exist.

// ============================================================================
// Public API (Parse-Once Pattern)
// ============================================================================

/// Solve both parts efficiently (parse once!)
pub fn solve(input: &str) -> (usize, i64) {
    let sensors = parse_input(input);
    (
        count_impossible_positions(&sensors, 2_000_000),
        find_distress_beacon_perimeter(&sensors, 4_000_000),
    )
}

/// Parse input (exposed for mod.rs compatibility)
pub fn parse(input: &str) -> Vec<Sensor> {
    parse_input(input)
}

/// Solve Part 1 only (for testing)
pub fn part1(sensors: &[Sensor]) -> usize {
    count_impossible_positions(sensors, 2_000_000)
}

/// Solve Part 2 only (for testing) - Uses optimized perimeter search
pub fn part2(sensors: &[Sensor]) -> i64 {
    find_distress_beacon_perimeter(sensors, 4_000_000)
}

/// Part 2 with row scan (original approach, for benchmarking comparison)
pub fn part2_row_scan(sensors: &[Sensor]) -> i64 {
    find_distress_beacon_row_scan(sensors, 4_000_000)
}

#[derive(Debug, Clone, Copy)]
pub struct Sensor {
    pub x: i32,
    pub y: i32,
    pub beacon_x: i32,
    pub beacon_y: i32,
}

impl Sensor {
    pub fn radius(&self) -> i32 {
        manhattan_distance(self.x, self.y, self.beacon_x, self.beacon_y)
    }

    fn distance_to(&self, x: i32, y: i32) -> i32 {
        manhattan_distance(self.x, self.y, x, y)
    }
}

fn manhattan_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn parse_input(input: &str) -> Vec<Sensor> {
    // Parse lines like: "Sensor at x=2, y=18: closest beacon is at x=-2, y=15"
    input
        .lines()
        .map(|line| {
            // Extract all numbers (including negative)
            let nums: Vec<i32> = line
                .split(|c: char| !c.is_numeric() && c != '-')
                .filter(|s| !s.is_empty())
                .filter_map(|s| s.parse().ok())
                .collect();

            Sensor {
                x: nums[0],
                y: nums[1],
                beacon_x: nums[2],
                beacon_y: nums[3],
            }
        })
        .collect()
}

fn count_impossible_positions(sensors: &[Sensor], target_row: i32) -> usize {
    // Calculate X-range intervals for each sensor at target_row
    let mut intervals: Vec<(i32, i32)> = sensors
        .iter()
        .filter_map(|s| {
            let radius = s.radius();
            let vertical_distance = (s.y - target_row).abs();
            let remaining = radius - vertical_distance;

            if remaining >= 0 {
                Some((s.x - remaining, s.x + remaining))
            } else {
                None
            }
        })
        .collect();

    // Sort intervals by start position
    intervals.sort_unstable();

    // Merge overlapping intervals
    let merged = merge_intervals(&intervals);

    // Count total positions covered
    let total_coverage: usize = merged
        .iter()
        .map(|(start, end)| (end - start + 1) as usize)
        .sum();

    // Count beacons actually on target row (these are not "impossible" positions)
    let beacons_on_row: std::collections::HashSet<i32> = sensors
        .iter()
        .filter(|s| s.beacon_y == target_row)
        .map(|s| s.beacon_x)
        .collect();

    total_coverage - beacons_on_row.len()
}

fn merge_intervals(intervals: &[(i32, i32)]) -> Vec<(i32, i32)> {
    if intervals.is_empty() {
        return vec![];
    }

    let mut merged = vec![intervals[0]];

    for &(start, end) in &intervals[1..] {
        let last_idx = merged.len() - 1;
        let (last_start, last_end) = merged[last_idx];

        if start <= last_end + 1 {
            // Overlapping or adjacent - merge
            merged[last_idx] = (last_start, last_end.max(end));
        } else {
            // Non-overlapping - add new interval
            merged.push((start, end));
        }
    }

    merged
}

/// Part 2: Row Scan Approach (original - kept for comparison)
///
/// Scans all 4 million rows, calculating intervals for each.
/// Performance: ~451ms (10× slower than perimeter search)
fn find_distress_beacon_row_scan(sensors: &[Sensor], max_coord: i32) -> i64 {
    for y in 0..=max_coord {
        // Get coverage intervals for this row
        let mut intervals: Vec<(i32, i32)> = sensors
            .iter()
            .filter_map(|s| {
                let radius = s.radius();
                let vertical_distance = (s.y - y).abs();
                let remaining = radius - vertical_distance;

                if remaining >= 0 {
                    // Clamp to search bounds
                    let x_start = (s.x - remaining).max(0);
                    let x_end = (s.x + remaining).min(max_coord);
                    Some((x_start, x_end))
                } else {
                    None
                }
            })
            .collect();

        intervals.sort_unstable();
        let merged = merge_intervals(&intervals);

        // Check for gap in [0, max_coord]
        // If first interval doesn't start at 0, gap is at x=0
        if merged[0].0 > 0 {
            return y as i64;
        }

        // Check gaps between intervals
        for i in 0..merged.len() - 1 {
            let (_, end1) = merged[i];
            let (start2, _) = merged[i + 1];

            if end1 + 1 < start2 {
                // Gap found!
                let x = end1 + 1;
                return x as i64 * 4_000_000 + y as i64;
            }
        }

        // Check if last interval covers up to max_coord
        let (_, last_end) = merged[merged.len() - 1];
        if last_end < max_coord {
            return max_coord as i64 * 4_000_000 + y as i64;
        }
    }

    0 // Not found
}

/// Part 2: Perimeter Search Approach (PRIMARY - 10× faster!)
///
/// **Key Insight**: The distress beacon must be just outside (radius + 1) from
/// at least one sensor's range, at the boundary between coverage areas.
///
/// **Algorithm**: Check perimeter points at distance = radius + 1 for each sensor.
///
/// **Performance**: ~45ms (vs 451ms for row scan = 10× speedup!)
///
/// **Why it works**: Only one uncovered position exists, and it MUST be at the
/// edge of sensor coverage (where two or more sensor ranges meet).
fn find_distress_beacon_perimeter(sensors: &[Sensor], max_coord: i32) -> i64 {
    // Check perimeter of each sensor at distance = radius + 1
    for sensor in sensors {
        let perimeter_dist = sensor.radius() + 1;

        // Walk the perimeter diamond
        for offset in 0..=perimeter_dist {
            let dx = offset;
            let dy = perimeter_dist - offset;

            // Check all 4 quadrants
            let candidates = [
                (sensor.x + dx, sensor.y + dy),
                (sensor.x + dx, sensor.y - dy),
                (sensor.x - dx, sensor.y + dy),
                (sensor.x - dx, sensor.y - dy),
            ];

            for (x, y) in candidates {
                // Skip out of bounds
                if x < 0 || x > max_coord || y < 0 || y > max_coord {
                    continue;
                }

                // Check if this point is outside ALL sensor ranges
                if sensors
                    .iter()
                    .all(|s| s.distance_to(x, y) > s.radius())
                {
                    return x as i64 * 4_000_000 + y as i64;
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(manhattan_distance(8, 7, 2, 10), 9);
    }

    #[test]
    fn test_part1_example() {
        // Example uses y=10 (not 2000000)
        let sensors = parse(EXAMPLE);
        let count = count_impossible_positions(&sensors, 10);
        assert_eq!(count, 26);
    }

    #[test]
    fn test_parse() {
        let sensors = parse(EXAMPLE);
        assert_eq!(sensors.len(), 14);
        assert_eq!(sensors[0].x, 2);
        assert_eq!(sensors[0].y, 18);
        assert_eq!(sensors[0].beacon_x, -2);
        assert_eq!(sensors[0].beacon_y, 15);
    }

    #[test]
    fn test_part2_example() {
        let sensors = parse(EXAMPLE);
        let tuning_freq = find_distress_beacon_perimeter(&sensors, 20);
        assert_eq!(tuning_freq, 56000011);
    }

    #[test]
    fn test_part2_row_scan_example() {
        let sensors = parse(EXAMPLE);
        let result = find_distress_beacon_row_scan(&sensors, 20);
        assert_eq!(result, 56000011);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day15.txt");
        let sensors = parse(input);
        assert_eq!(part1(&sensors), 4876693);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day15.txt");
        let sensors = parse(input);
        assert_eq!(part2(&sensors), 11645454855041);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day15.txt");
        assert_eq!(solve(input), (4876693, 11645454855041));
    }

    #[test]
    fn test_part2_row_scan_actual() {
        let input = include_str!("../../inputs/day15.txt");
        let sensors = parse(input);
        assert_eq!(part2_row_scan(&sensors), 11645454855041);
    }
}
