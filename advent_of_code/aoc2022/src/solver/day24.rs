//! Day 24: Blizzard Basin
//!
//! Navigate through a valley of moving blizzards.
//! Part 1: Fewest minutes from start to goal.
//! Part 2: Start→goal→start→goal total minutes.

use std::collections::VecDeque;

use rustc_hash::FxHashSet;

type Pos = (usize, usize);

struct Valley {
    rows: usize,
    cols: usize,
    /// Blocked positions for each time step 0..period
    blocked: Vec<FxHashSet<Pos>>,
    period: usize,
    start: Pos,
    goal: Pos,
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn parse(input: &str) -> Valley {
    let grid: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let inner_h = rows - 2;
    let inner_w = cols - 2;
    let period = lcm(inner_h, inner_w);

    // Collect blizzards: (row, col, direction)
    let mut blizzards: Vec<(usize, usize, u8)> = Vec::new();
    for (r, row) in grid.iter().enumerate().take(rows - 1).skip(1) {
        for (c, &cell) in row.iter().enumerate().take(cols - 1).skip(1) {
            if matches!(cell, b'^' | b'v' | b'<' | b'>') {
                blizzards.push((r, c, cell));
            }
        }
    }

    // Precompute blocked positions for each time step
    let mut blocked = Vec::with_capacity(period);
    for t in 0..period {
        let mut set = FxHashSet::default();
        for &(r, c, dir) in &blizzards {
            let (br, bc) = match dir {
                b'^' => (((r - 1) + inner_h - (t % inner_h)) % inner_h + 1, c),
                b'v' => (((r - 1) + t) % inner_h + 1, c),
                b'<' => (r, ((c - 1) + inner_w - (t % inner_w)) % inner_w + 1),
                b'>' => (r, ((c - 1) + t) % inner_w + 1),
                _ => unreachable!(),
            };
            set.insert((br, bc));
        }
        blocked.push(set);
    }

    let start_col = grid[0].iter().position(|&b| b == b'.').unwrap();
    let goal_col = grid[rows - 1].iter().position(|&b| b == b'.').unwrap();

    Valley {
        rows,
        cols,
        blocked,
        period,
        start: (0, start_col),
        goal: (rows - 1, goal_col),
    }
}

/// BFS from start to goal, beginning at start_time. Returns the arrival time.
fn bfs(valley: &Valley, start: Pos, goal: Pos, start_time: usize) -> usize {
    let mut visited = FxHashSet::default();
    let mut queue = VecDeque::new();

    let t0_mod = start_time % valley.period;
    queue.push_back((start.0, start.1, start_time));
    visited.insert((start.0, start.1, t0_mod));

    const MOVES: [(isize, isize); 5] = [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)];

    while let Some((r, c, t)) = queue.pop_front() {
        let nt = t + 1;
        let nt_mod = nt % valley.period;
        let blocked = &valley.blocked[nt_mod];

        for (dr, dc) in MOVES {
            let nr = r as isize + dr;
            let nc = c as isize + dc;

            if nr < 0 || nc < 0 {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;

            if nr >= valley.rows || nc >= valley.cols {
                continue;
            }

            if (nr, nc) == goal {
                return nt;
            }

            // Valid positions: start, goal, or inner cells (not walls)
            let is_start_or_goal = (nr, nc) == valley.start || (nr, nc) == valley.goal;
            let is_inner = nr >= 1 && nr < valley.rows - 1 && nc >= 1 && nc < valley.cols - 1;

            if !is_start_or_goal && !is_inner {
                continue;
            }

            if !is_start_or_goal && blocked.contains(&(nr, nc)) {
                continue;
            }

            if visited.insert((nr, nc, nt_mod)) {
                queue.push_back((nr, nc, nt));
            }
        }
    }

    unreachable!("No path found")
}

/// BFS that also reconstructs the path. Returns (arrival_time, path).
/// Path is a Vec of positions from start to goal (inclusive).
#[cfg(test)]
fn bfs_with_path(
    valley: &Valley,
    start: Pos,
    goal: Pos,
    start_time: usize,
) -> (usize, Vec<Pos>) {
    use rustc_hash::FxHashMap;

    let mut visited = FxHashSet::default();
    let mut queue = VecDeque::new();
    // parent maps (row, col, absolute_time) → (row, col, absolute_time)
    let mut parent: FxHashMap<(usize, usize, usize), (usize, usize, usize)> = FxHashMap::default();

    queue.push_back((start.0, start.1, start_time));
    visited.insert((start.0, start.1, start_time % valley.period));

    const MOVES: [(isize, isize); 5] = [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)];

    let goal_time;

    'outer: loop {
        let (r, c, t) = queue.pop_front().expect("No path found");
        let nt = t + 1;
        let nt_mod = nt % valley.period;
        let blocked = &valley.blocked[nt_mod];

        for (dr, dc) in MOVES {
            let nr = r as isize + dr;
            let nc = c as isize + dc;

            if nr < 0 || nc < 0 {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;

            if nr >= valley.rows || nc >= valley.cols {
                continue;
            }

            if (nr, nc) == goal {
                parent.insert((nr, nc, nt), (r, c, t));
                goal_time = nt;
                break 'outer;
            }

            let is_start_or_goal = (nr, nc) == valley.start || (nr, nc) == valley.goal;
            let is_inner =
                nr >= 1 && nr < valley.rows - 1 && nc >= 1 && nc < valley.cols - 1;

            if !is_start_or_goal && !is_inner {
                continue;
            }

            if !is_start_or_goal && blocked.contains(&(nr, nc)) {
                continue;
            }

            if visited.insert((nr, nc, nt_mod)) {
                parent.insert((nr, nc, nt), (r, c, t));
                queue.push_back((nr, nc, nt));
            }
        }
    }

    // Reconstruct path by walking parent pointers backwards using absolute time
    let mut path = vec![(goal.0, goal.1)];
    let mut current = (goal.0, goal.1, goal_time);
    while current.2 > start_time {
        let prev = parent[&current];
        path.push((prev.0, prev.1));
        current = prev;
    }
    path.reverse();

    (goal_time, path)
}

pub fn solve(input: &str) -> (usize, usize) {
    let input = input.replace("\r\n", "\n");
    let valley = parse(&input);

    // Part 1: start → goal
    let t1 = bfs(&valley, valley.start, valley.goal, 0);

    // Part 2: start → goal → start → goal
    let t2 = bfs(&valley, valley.goal, valley.start, t1);
    let t3 = bfs(&valley, valley.start, valley.goal, t2);

    (t1, t3)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn test_example_part1() {
        let (p1, _) = solve(EXAMPLE);
        assert_eq!(p1, 18);
    }

    #[test]
    fn test_example_part2() {
        let (_, p2) = solve(EXAMPLE);
        assert_eq!(p2, 54);
    }

    #[test]
    fn test_actual_input() {
        let input = include_str!("../../inputs/day24.txt");
        let (p1, p2) = solve(input);
        assert_eq!(p1, 242);
        assert_eq!(p2, 720);
    }

    struct PathStats {
        productive: usize,
        waits: usize,
        backtracks: usize,
    }

    /// Classify each step: productive (S/E), wait, or backtrack (N/W)
    fn analyze_path(path: &[Pos], goal: Pos) -> PathStats {
        let manhattan = |p: Pos| -> isize {
            (p.0 as isize - goal.0 as isize).abs() + (p.1 as isize - goal.1 as isize).abs()
        };

        let mut stats = PathStats { productive: 0, waits: 0, backtracks: 0 };

        for window in path.windows(2) {
            let from = window[0];
            let to = window[1];

            if from == to {
                stats.waits += 1;
            } else if manhattan(to) < manhattan(from) {
                stats.productive += 1;
            } else {
                stats.backtracks += 1;
            }
        }

        stats
    }

    fn print_leg(label: &str, steps: usize, s: &PathStats, min_dist: isize) {
        let pct = |n: usize| n as f64 / steps as f64 * 100.0;
        println!("{}: {} steps", label, steps);
        println!("  Productive: {:>3} ({:.0}%)", s.productive, pct(s.productive));
        println!("  Wait:       {:>3} ({:.0}%)", s.waits, pct(s.waits));
        println!("  Backtrack:  {:>3} ({:.0}%)", s.backtracks, pct(s.backtracks));
        println!("  Overhead: {} extra steps ({:.1}x minimum)\n",
            steps as isize - min_dist, steps as f64 / min_dist as f64);
    }

    #[test]
    fn test_path_analysis_example() {
        let valley = parse(EXAMPLE);
        let min_dist = (valley.goal.0 as isize - valley.start.0 as isize).abs()
            + (valley.goal.1 as isize - valley.start.1 as isize).abs();

        println!("\n=== PATH ANALYSIS: Example Input (Manhattan distance: {}) ===\n", min_dist);

        let (t1, path1) = bfs_with_path(&valley, valley.start, valley.goal, 0);
        let s1 = analyze_path(&path1, valley.goal);
        print_leg("Leg 1 (start→goal)", t1, &s1, min_dist);

        let (t2, path2) = bfs_with_path(&valley, valley.goal, valley.start, t1);
        let s2 = analyze_path(&path2, valley.start);
        print_leg("Leg 2 (goal→start)", t2 - t1, &s2, min_dist);

        let (t3, path3) = bfs_with_path(&valley, valley.start, valley.goal, t2);
        let s3 = analyze_path(&path3, valley.goal);
        print_leg("Leg 3 (start→goal)", t3 - t2, &s3, min_dist);

        let total = t3;
        let tp = s1.productive + s2.productive + s3.productive;
        let tw = s1.waits + s2.waits + s3.waits;
        let tb = s1.backtracks + s2.backtracks + s3.backtracks;
        let pct = |n: usize| n as f64 / total as f64 * 100.0;
        println!("=== TOTALS ({} steps) ===", total);
        println!("  Productive: {:>3} ({:.0}%)", tp, pct(tp));
        println!("  Wait:       {:>3} ({:.0}%)", tw, pct(tw));
        println!("  Backtrack:  {:>3} ({:.0}%)", tb, pct(tb));
        println!("  Minimum possible: {} (3 × {})", min_dist * 3, min_dist);
        println!("  Overall efficiency: {:.0}%\n", (min_dist * 3) as f64 / total as f64 * 100.0);
    }

    #[test]
    fn test_path_analysis_actual() {
        let input = include_str!("../../inputs/day24.txt");
        let input = input.replace("\r\n", "\n");
        let valley = parse(&input);
        let min_dist = (valley.goal.0 as isize - valley.start.0 as isize).abs()
            + (valley.goal.1 as isize - valley.start.1 as isize).abs();

        println!("\n=== PATH ANALYSIS: Actual Input ({}×{}, Manhattan distance: {}) ===\n",
            valley.rows, valley.cols, min_dist);

        let (t1, path1) = bfs_with_path(&valley, valley.start, valley.goal, 0);
        let s1 = analyze_path(&path1, valley.goal);
        print_leg("Leg 1 (start→goal)", t1, &s1, min_dist);

        let (t2, path2) = bfs_with_path(&valley, valley.goal, valley.start, t1);
        let s2 = analyze_path(&path2, valley.start);
        print_leg("Leg 2 (goal→start)", t2 - t1, &s2, min_dist);

        let (t3, path3) = bfs_with_path(&valley, valley.start, valley.goal, t2);
        let s3 = analyze_path(&path3, valley.goal);
        print_leg("Leg 3 (start→goal)", t3 - t2, &s3, min_dist);

        let total = t3;
        let tp = s1.productive + s2.productive + s3.productive;
        let tw = s1.waits + s2.waits + s3.waits;
        let tb = s1.backtracks + s2.backtracks + s3.backtracks;
        let pct = |n: usize| n as f64 / total as f64 * 100.0;
        println!("=== TOTALS ({} steps) ===", total);
        println!("  Productive: {:>3} ({:.0}%)", tp, pct(tp));
        println!("  Wait:       {:>3} ({:.0}%)", tw, pct(tw));
        println!("  Backtrack:  {:>3} ({:.0}%)", tb, pct(tb));
        println!("  Minimum possible: {} (3 × {})", min_dist * 3, min_dist);
        println!("  Overall efficiency: {:.0}%\n", (min_dist * 3) as f64 / total as f64 * 100.0);

        assert_eq!(t1, 242);
        assert_eq!(t3, 720);
    }
}
