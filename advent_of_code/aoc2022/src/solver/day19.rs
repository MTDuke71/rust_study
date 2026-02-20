
//! Day 19: Not Enough Minerals
//!
//! Resource optimization: given robot blueprints, maximize geode production.
//! Part 1: 24 minutes, sum quality levels (id × max_geodes) for all 30 blueprints.
//! Part 2: 32 minutes, first 3 blueprints only, product of max geodes.

use rayon::prelude::*;

// ============================================================================
// Data Types
// ============================================================================

#[derive(Debug, Clone)]
pub struct Blueprint {
    pub id: u32,
    pub ore_robot_ore: u32,
    pub clay_robot_ore: u32,
    pub obsidian_robot_ore: u32,
    pub obsidian_robot_clay: u32,
    pub geode_robot_ore: u32,
    pub geode_robot_obsidian: u32,
}

pub type ParsedData = Vec<Blueprint>;

// ============================================================================
// Parsing
// ============================================================================

pub fn parse_input(input: &str) -> ParsedData {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let nums: Vec<u32> = line
                .split(|c: char| !c.is_ascii_digit())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();
            Blueprint {
                id: nums[0],
                ore_robot_ore: nums[1],
                clay_robot_ore: nums[2],
                obsidian_robot_ore: nums[3],
                obsidian_robot_clay: nums[4],
                geode_robot_ore: nums[5],
                geode_robot_obsidian: nums[6],
            }
        })
        .collect()
}

// ============================================================================
// DFS Search with Pruning
// ============================================================================

/// DFS state: (time_left, ore, clay, obsidian, geodes, ore_bots, clay_bots, obs_bots, geo_bots)
type SearchState = (u32, u32, u32, u32, u32, u32, u32, u32, u32);

fn max_geodes(bp: &Blueprint, time_limit: u32) -> u32 {
    // Max ore we could spend per minute — no point building more ore robots than this
    let max_ore_cost = bp.ore_robot_ore
        .max(bp.clay_robot_ore)
        .max(bp.obsidian_robot_ore)
        .max(bp.geode_robot_ore);
    let max_clay_cost = bp.obsidian_robot_clay;
    let max_obsidian_cost = bp.geode_robot_obsidian;

    let mut best = 0u32;

    let mut stack: Vec<SearchState> = Vec::new();
    stack.push((time_limit, 0, 0, 0, 0, 1, 0, 0, 0));

    while let Some((time, ore, clay, obsidian, geodes, r_ore, r_clay, r_obs, r_geo)) = stack.pop()
    {
        // Update best
        best = best.max(geodes + r_geo * time);

        if time == 0 {
            continue;
        }

        // Upper bound: even if we build a geode robot every remaining minute
        // Current geodes + current_bots * time + (time-1 + time-2 + ... + 0) = time*(time-1)/2
        let upper = geodes + r_geo * time + time * (time - 1) / 2;
        if upper <= best {
            continue;
        }

        // Try building each robot type (skip-to-build: compute how long to wait)
        // PUSH ORDER REVERSED: Ore -> Clay -> Obsidian -> Geode
        // This ensures Geode branches are popped and explored FIRST, establishing a high `best` score early.

        // Option 1: Build geode robot
        if r_obs > 0 {
            let wait_ore = if ore >= bp.geode_robot_ore {
                0
            } else {
                (bp.geode_robot_ore - ore).div_ceil(r_ore)
            };
            let wait_obs = if obsidian >= bp.geode_robot_obsidian {
                0
            } else {
                (bp.geode_robot_obsidian - obsidian).div_ceil(r_obs)
            };
            let wait = wait_ore.max(wait_obs) + 1; // +1 for build time
            if wait < time {
                stack.push((
                    time - wait,
                    ore + r_ore * wait - bp.geode_robot_ore,
                    clay + r_clay * wait,
                    obsidian + r_obs * wait - bp.geode_robot_obsidian,
                    geodes + r_geo * wait,
                    r_ore,
                    r_clay,
                    r_obs,
                    r_geo + 1,
                ));
            }
        }

        // Option 2: Build obsidian robot (only if we need more)
        // End-game pruning: Obsidian robot must be ready by time=3 to produce obsidian for a Geode robot starting at time=2
        if r_obs < max_obsidian_cost && r_clay > 0 {
            let wait_ore = if ore >= bp.obsidian_robot_ore {
                0
            } else {
                (bp.obsidian_robot_ore - ore).div_ceil(r_ore)
            };
            let wait_clay = if clay >= bp.obsidian_robot_clay {
                0
            } else {
                (bp.obsidian_robot_clay - clay).div_ceil(r_clay)
            };
            let wait = wait_ore.max(wait_clay) + 1;
            if wait < time.saturating_sub(2) {
                stack.push((
                    time - wait,
                    ore + r_ore * wait - bp.obsidian_robot_ore,
                    clay + r_clay * wait - bp.obsidian_robot_clay,
                    obsidian + r_obs * wait,
                    geodes + r_geo * wait,
                    r_ore,
                    r_clay,
                    r_obs + 1,
                    r_geo,
                ));
            }
        }

        // Option 3: Build clay robot (only if we need more)
        // End-game pruning: Clay robot must be ready by time=5 to produce clay for an Obsidian robot (ready at 3), which produces obsidian for a Geode robot (ready at 1)
        if r_clay < max_clay_cost {
            let wait = if ore >= bp.clay_robot_ore {
                0
            } else {
                (bp.clay_robot_ore - ore).div_ceil(r_ore)
            } + 1;
            if wait < time.saturating_sub(4) {
                stack.push((
                    time - wait,
                    ore + r_ore * wait - bp.clay_robot_ore,
                    clay + r_clay * wait,
                    obsidian + r_obs * wait,
                    geodes + r_geo * wait,
                    r_ore,
                    r_clay + 1,
                    r_obs,
                    r_geo,
                ));
            }
        }

        // Option 4: Build ore robot (only if we need more)
        // End-game pruning: Ore robot must be ready by time=3 to produce ore for a Geode robot starting at time=2
        if r_ore < max_ore_cost {
            let wait = if ore >= bp.ore_robot_ore {
                0
            } else {
                (bp.ore_robot_ore - ore).div_ceil(r_ore)
            } + 1;
            if wait < time.saturating_sub(2) {
                stack.push((
                    time - wait,
                    ore + r_ore * wait - bp.ore_robot_ore,
                    clay + r_clay * wait,
                    obsidian + r_obs * wait,
                    geodes + r_geo * wait,
                    r_ore + 1,
                    r_clay,
                    r_obs,
                    r_geo,
                ));
            }
        }
    }

    best
}

// ============================================================================
// Part 1
// ============================================================================

pub fn solve_part1_with_data(blueprints: &ParsedData) -> u32 {
    blueprints
        .par_iter()
        .map(|bp| bp.id * max_geodes(bp, 24))
        .sum()
}

// ============================================================================
// Part 2: 32 minutes, first 3 blueprints, product of geodes
// ============================================================================

pub fn solve_part2_with_data(blueprints: &ParsedData) -> u64 {
    let limit = blueprints.len().min(3);
    blueprints[..limit]
        .par_iter()
        .map(|bp| max_geodes(bp, 32) as u64)
        .product()
}

// ============================================================================
// Entry point
// ============================================================================

pub fn solve(input: &str) -> (u32, u64) {
    let blueprints = parse_input(input);
    (
        solve_part1_with_data(&blueprints),
        solve_part2_with_data(&blueprints),
    )
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test_parse() {
        let bps = parse_input(EXAMPLE);
        assert_eq!(bps.len(), 2);
        assert_eq!(bps[0].id, 1);
        assert_eq!(bps[0].ore_robot_ore, 4);
        assert_eq!(bps[0].geode_robot_obsidian, 7);
        assert_eq!(bps[1].clay_robot_ore, 3);
    }

    #[test]
    fn test_part1_blueprint1() {
        let bps = parse_input(EXAMPLE);
        assert_eq!(max_geodes(&bps[0], 24), 9);
    }

    #[test]
    fn test_part1_blueprint2() {
        let bps = parse_input(EXAMPLE);
        assert_eq!(max_geodes(&bps[1], 24), 12);
    }

    #[test]
    fn test_part1_example() {
        let bps = parse_input(EXAMPLE);
        assert_eq!(solve_part1_with_data(&bps), 33);
    }

    #[test]
    fn test_part2_blueprint1() {
        let bps = parse_input(EXAMPLE);
        assert_eq!(max_geodes(&bps[0], 32), 56);
    }

    #[test]
    fn test_part2_blueprint2() {
        let bps = parse_input(EXAMPLE);
        assert_eq!(max_geodes(&bps[1], 32), 62);
    }

    #[test]
    fn test_actual_answers() {
        let input = include_str!("../../inputs/day19.txt");
        let data = parse_input(input);
        assert_eq!(solve_part1_with_data(&data), 1413);
        assert_eq!(solve_part2_with_data(&data), 21080);
    }

    #[test]
    fn test_debug_trace() {
        let bps = parse_input(EXAMPLE);
        let bp = &bps[0];

        let max_ore_cost = bp.ore_robot_ore
            .max(bp.clay_robot_ore)
            .max(bp.obsidian_robot_ore)
            .max(bp.geode_robot_ore);
        let max_clay_cost = bp.obsidian_robot_clay;
        let max_obsidian_cost = bp.geode_robot_obsidian;

        // Each entry: (state, parent_index, action_label)
        let mut history: Vec<(SearchState, Option<usize>, String)> = Vec::new();
        // Stack holds (state, index_in_history)
        let mut stack: Vec<(SearchState, usize)> = Vec::new();

        let init: SearchState = (24, 0, 0, 0, 0, 1, 0, 0, 0);
        history.push((init, None, "START".to_string()));
        stack.push((init, 0));

        let mut best = 0u32;
        let mut best_idx = 0usize;
        let mut states_explored = 0u32;
        let mut states_pruned = 0u32;

        while let Some(((time, ore, clay, obsidian, geodes, r_ore, r_clay, r_obs, r_geo), my_idx)) =
            stack.pop()
        {
            states_explored += 1;
            let projected = geodes + r_geo * time;
            if projected > best {
                best = projected;
                best_idx = my_idx;
            }

            if time == 0 { continue; }

            let upper = geodes + r_geo * time + time * (time - 1) / 2;
            if upper <= best {
                states_pruned += 1;
                continue;
            }

            // Helper macro to push a child
            macro_rules! push_child {
                ($state:expr, $label:expr) => {{
                    let idx = history.len();
                    history.push(($state, Some(my_idx), $label));
                    stack.push(($state, idx));
                }};
            }

            if r_obs > 0 {
                let wait_ore = if ore >= bp.geode_robot_ore { 0 } else { (bp.geode_robot_ore - ore).div_ceil(r_ore) };
                let wait_obs = if obsidian >= bp.geode_robot_obsidian { 0 } else { (bp.geode_robot_obsidian - obsidian).div_ceil(r_obs) };
                let wait = wait_ore.max(wait_obs) + 1;
                if wait < time {
                    let s = (time - wait, ore + r_ore * wait - bp.geode_robot_ore, clay + r_clay * wait, obsidian + r_obs * wait - bp.geode_robot_obsidian, geodes + r_geo * wait, r_ore, r_clay, r_obs, r_geo + 1);
                    push_child!(s, format!("Build GEODE (wait {wait} min)"));
                }
            }
            if r_obs < max_obsidian_cost && r_clay > 0 {
                let wait_ore = if ore >= bp.obsidian_robot_ore { 0 } else { (bp.obsidian_robot_ore - ore).div_ceil(r_ore) };
                let wait_clay = if clay >= bp.obsidian_robot_clay { 0 } else { (bp.obsidian_robot_clay - clay).div_ceil(r_clay) };
                let wait = wait_ore.max(wait_clay) + 1;
                if wait < time {
                    let s = (time - wait, ore + r_ore * wait - bp.obsidian_robot_ore, clay + r_clay * wait - bp.obsidian_robot_clay, obsidian + r_obs * wait, geodes + r_geo * wait, r_ore, r_clay, r_obs + 1, r_geo);
                    push_child!(s, format!("Build OBS (wait {wait} min)"));
                }
            }
            if r_clay < max_clay_cost {
                let wait = if ore >= bp.clay_robot_ore { 0 } else { (bp.clay_robot_ore - ore).div_ceil(r_ore) } + 1;
                if wait < time {
                    let s = (time - wait, ore + r_ore * wait - bp.clay_robot_ore, clay + r_clay * wait, obsidian + r_obs * wait, geodes + r_geo * wait, r_ore, r_clay + 1, r_obs, r_geo);
                    push_child!(s, format!("Build CLAY (wait {wait} min)"));
                }
            }
            if r_ore < max_ore_cost {
                let wait = if ore >= bp.ore_robot_ore { 0 } else { (bp.ore_robot_ore - ore).div_ceil(r_ore) } + 1;
                if wait < time {
                    let s = (time - wait, ore + r_ore * wait - bp.ore_robot_ore, clay + r_clay * wait, obsidian + r_obs * wait, geodes + r_geo * wait, r_ore + 1, r_clay, r_obs, r_geo);
                    push_child!(s, format!("Build ORE (wait {wait} min)"));
                }
            }
        }

        // Reconstruct the winning path
        let mut path = Vec::new();
        let mut idx = best_idx;
        loop {
            let (state, parent, ref label) = history[idx];
            path.push((state, label.clone()));
            if let Some(p) = parent {
                idx = p;
            } else {
                break;
            }
        }
        path.reverse();

        eprintln!();
        eprintln!("=== Blueprint 1: ore=4ore, clay=2ore, obs=3ore+14clay, geo=2ore+7obs ===");
        eprintln!("Robot caps: ore<={max_ore_cost}, clay<={max_clay_cost}, obs<={max_obsidian_cost}");
        eprintln!();
        eprintln!("--- WINNING PATH ({best} geodes) ---");
        eprintln!();
        eprintln!("{:<30} {:>4} {:>5} {:>5} {:>5} {:>5}  {:>9} {:>10} {:>9} {:>9}",
            "Action", "Time", "Ore", "Clay", "Obs", "Geo", "Ore Bots", "Clay Bots", "Obs Bots", "Geo Bots");
        eprintln!("{}", "-".repeat(120));

        for (state, label) in &path {
            let (time, ore, clay, obs, geo, r_ore, r_clay, r_obs, r_geo) = *state;
            eprintln!("{:<30} {:>4} {:>5} {:>5} {:>5} {:>5}  {:>9} {:>10} {:>9} {:>9}",
                label, time, ore, clay, obs, geo, r_ore, r_clay, r_obs, r_geo);
        }

        let (_, _, _, _, geo, _, _, _, r_geo) = path.last().unwrap().0;
        let final_time = path.last().unwrap().0.0;
        eprintln!();
        eprintln!("Final: {geo} produced + {r_geo} bots × {final_time} remaining = {} geodes", geo + r_geo * final_time);
        eprintln!();
        eprintln!("Stats: {states_explored} states explored, {states_pruned} pruned by upper bound");
        assert_eq!(best, 9);
    }

    /// Finds the winning path for a blueprint, then expands it minute-by-minute as CSV.
    fn trace_minute_by_minute(bp: &Blueprint, time_limit: u32) -> String {
        let max_ore_cost = bp.ore_robot_ore
            .max(bp.clay_robot_ore)
            .max(bp.obsidian_robot_ore)
            .max(bp.geode_robot_ore);
        let max_clay_cost = bp.obsidian_robot_clay;
        let max_obsidian_cost = bp.geode_robot_obsidian;

        // --- Pass 1: find winning path via DFS (same as test_debug_trace) ---
        let mut history: Vec<(SearchState, Option<usize>, String)> = Vec::new();
        let mut stack: Vec<(SearchState, usize)> = Vec::new();
        let init: SearchState = (time_limit, 0, 0, 0, 0, 1, 0, 0, 0);
        history.push((init, None, "START".to_string()));
        stack.push((init, 0));
        let mut best = 0u32;
        let mut best_idx = 0usize;

        while let Some(((time, ore, clay, obsidian, geodes, r_ore, r_clay, r_obs, r_geo), my_idx)) =
            stack.pop()
        {
            let projected = geodes + r_geo * time;
            if projected > best { best = projected; best_idx = my_idx; }
            if time == 0 { continue; }
            let upper = geodes + r_geo * time + time * (time - 1) / 2;
            if upper <= best { continue; }

            macro_rules! push_child {
                ($state:expr, $label:expr) => {{
                    let idx = history.len();
                    history.push(($state, Some(my_idx), $label));
                    stack.push(($state, idx));
                }};
            }
            if r_obs > 0 {
                let w_ore = if ore >= bp.geode_robot_ore { 0 } else { (bp.geode_robot_ore - ore).div_ceil(r_ore) };
                let w_obs = if obsidian >= bp.geode_robot_obsidian { 0 } else { (bp.geode_robot_obsidian - obsidian).div_ceil(r_obs) };
                let w = w_ore.max(w_obs) + 1;
                if w < time {
                    push_child!((time-w, ore+r_ore*w-bp.geode_robot_ore, clay+r_clay*w, obsidian+r_obs*w-bp.geode_robot_obsidian, geodes+r_geo*w, r_ore, r_clay, r_obs, r_geo+1), format!("Build GEODE"));
                }
            }
            if r_obs < max_obsidian_cost && r_clay > 0 {
                let w_ore = if ore >= bp.obsidian_robot_ore { 0 } else { (bp.obsidian_robot_ore - ore).div_ceil(r_ore) };
                let w_clay = if clay >= bp.obsidian_robot_clay { 0 } else { (bp.obsidian_robot_clay - clay).div_ceil(r_clay) };
                let w = w_ore.max(w_clay) + 1;
                if w < time {
                    push_child!((time-w, ore+r_ore*w-bp.obsidian_robot_ore, clay+r_clay*w-bp.obsidian_robot_clay, obsidian+r_obs*w, geodes+r_geo*w, r_ore, r_clay, r_obs+1, r_geo), format!("Build OBS"));
                }
            }
            if r_clay < max_clay_cost {
                let w = if ore >= bp.clay_robot_ore { 0 } else { (bp.clay_robot_ore - ore).div_ceil(r_ore) } + 1;
                if w < time {
                    push_child!((time-w, ore+r_ore*w-bp.clay_robot_ore, clay+r_clay*w, obsidian+r_obs*w, geodes+r_geo*w, r_ore, r_clay+1, r_obs, r_geo), format!("Build CLAY"));
                }
            }
            if r_ore < max_ore_cost {
                let w = if ore >= bp.ore_robot_ore { 0 } else { (bp.ore_robot_ore - ore).div_ceil(r_ore) } + 1;
                if w < time {
                    push_child!((time-w, ore+r_ore*w-bp.ore_robot_ore, clay+r_clay*w, obsidian+r_obs*w, geodes+r_geo*w, r_ore+1, r_clay, r_obs, r_geo), format!("Build ORE"));
                }
            }
        }

        // Reconstruct winning path as build decisions
        let mut path = Vec::new();
        let mut idx = best_idx;
        loop {
            let (state, parent, ref label) = history[idx];
            path.push((state, label.clone()));
            match parent { Some(p) => idx = p, None => break }
        }
        path.reverse();

        // --- Pass 2: expand to minute-by-minute CSV ---
        // The path gives us snapshots AFTER each build completes.
        // We simulate forward, filling in the waiting minutes between builds.
        let mut csv = String::from("Minute,Action,Ore,Clay,Obsidian,Geodes,Ore Bots,Clay Bots,Obs Bots,Geo Bots\n");

        let mut ore = 0u32;
        let mut clay = 0u32;
        let mut obsidian = 0u32;
        let mut geodes = 0u32;
        let mut r_ore = 1u32;
        let mut r_clay = 0u32;
        let mut r_obs = 0u32;
        let mut r_geo = 0u32;

        // Build a schedule: at what minute does each build happen?
        // path[0] is START at time_limit, path[1..] are build steps
        // Each build happens at minute = (time_limit - prev_time_left + 1) through (time_limit - new_time_left)
        // The build completes (robot ready) at the END of minute (time_limit - new_time_left)
        let mut build_at: std::collections::HashMap<u32, String> = std::collections::HashMap::new();
        for i in 1..path.len() {
            let prev_time = path[i - 1].0.0; // time_left before
            let curr_time = path[i].0.0;     // time_left after build
            let build_minute = time_limit - curr_time; // minute when build completes
            // The build was STARTED at the beginning of this minute,
            // robot becomes available at END of this minute
            build_at.insert(build_minute, path[i].1.clone());

            // Also figure out when spending happens: resources are spent at start of build minute
            // but we need to track the "wait" minutes where we just collect
            let _wait = prev_time - curr_time; // includes 1 min build time
        }

        // Simulate minute by minute
        for minute in 1..=time_limit {
            let action = build_at.get(&minute).cloned().unwrap_or_default();

            // Determine resource cost this minute (spent at START of minute)
            let (cost_ore, cost_clay, cost_obs) = match action.as_str() {
                "Build ORE" => (bp.ore_robot_ore, 0, 0),
                "Build CLAY" => (bp.clay_robot_ore, 0, 0),
                "Build OBS" => (bp.obsidian_robot_ore, bp.obsidian_robot_clay, 0),
                "Build GEODE" => (bp.geode_robot_ore, 0, bp.geode_robot_obsidian),
                _ => (0, 0, 0),
            };

            // Spend resources (start of minute)
            ore -= cost_ore;
            clay -= cost_clay;
            obsidian -= cost_obs;

            // Collect resources (robots produce)
            ore += r_ore;
            clay += r_clay;
            obsidian += r_obs;
            geodes += r_geo;

            // New robot ready (end of minute)
            let display_action = if action.is_empty() { "Collect".to_string() } else { action.clone() };
            match action.as_str() {
                "Build ORE" => r_ore += 1,
                "Build CLAY" => r_clay += 1,
                "Build OBS" => r_obs += 1,
                "Build GEODE" => r_geo += 1,
                _ => {}
            }

            csv.push_str(&format!(
                "{minute},{display_action},{ore},{clay},{obsidian},{geodes},{r_ore},{r_clay},{r_obs},{r_geo}\n"
            ));
        }

        csv
    }

    #[test]
    fn test_minute_by_minute_actual() {
        let input = include_str!("../../inputs/day19.txt");
        let bps = parse_input(input);

        // Find first blueprint that actually produces geodes
        eprintln!();
        eprintln!("=== Geode counts per blueprint (24 min) ===");
        for bp in &bps {
            let g = max_geodes(bp, 24);
            if g > 0 {
                eprintln!("  BP {:>2}: {} geodes  (quality = {})", bp.id, g, bp.id * g);
            } else {
                eprintln!("  BP {:>2}: 0 geodes", bp.id);
            }
        }

        // Trace the first productive blueprint
        let bp = bps.iter().find(|bp| max_geodes(bp, 24) > 0).unwrap();
        eprintln!();
        eprintln!("=== Minute-by-minute for Blueprint {} ({} geodes) ===", bp.id, max_geodes(bp, 24));
        eprintln!("Costs: ore={} ore, clay={} ore, obs={} ore + {} clay, geo={} ore + {} obs",
            bp.ore_robot_ore, bp.clay_robot_ore,
            bp.obsidian_robot_ore, bp.obsidian_robot_clay,
            bp.geode_robot_ore, bp.geode_robot_obsidian);
        eprintln!();
        let csv = trace_minute_by_minute(bp, 24);
        eprintln!("{csv}");

        // Also trace example BP1 for comparison (known: 9 geodes)
        let example_bps = parse_input(EXAMPLE);
        eprintln!("=== Minute-by-minute for Example Blueprint 1 (9 geodes) ===");
        eprintln!("Costs: ore=4, clay=2, obs=3+14clay, geo=2+7obs");
        eprintln!();
        let csv2 = trace_minute_by_minute(&example_bps[0], 24);
        eprintln!("{csv2}");
    }
}
