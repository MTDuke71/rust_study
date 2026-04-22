//! Day 21: Fractal Art
//!
//! Grow a pixel grid by repeatedly splitting it into 2×2 or 3×3 blocks and
//! replacing each block via an enhancement rule. Rules match under any of
//! 8 orientations (4 rotations × 2 flips = the dihedral group D₄), so at
//! parse time we expand every rule LHS into all 8 and key on its string
//! form. Lookup is then just `HashMap::get` on the tile's serialized form.

use std::collections::HashMap;

type Tile = Vec<Vec<bool>>;

/// Parse "##./.../..#" into a Tile. Row separator is '/'; '#' = on.
fn parse_tile(s: &str) -> Tile {
    s.split('/')
        .map(|row| row.chars().map(|c| c == '#').collect())
        .collect()
}

/// Serialize a Tile back to "##./.../..#" form for HashMap keys.
fn tile_to_string(t: &Tile) -> String {
    let rows: Vec<String> = t
        .iter()
        .map(|row| row.iter().map(|&b| if b { '#' } else { '.' }).collect())
        .collect();
    rows.join("/")
}

/// Rotate 90° clockwise: new[r][c] = old[N-1-c][r].
fn rotate(t: &Tile) -> Tile {
    let n = t.len();
    (0..n)
        .map(|r| (0..n).map(|c| t[n - 1 - c][r]).collect())
        .collect()
}

/// Horizontal flip (mirror across the vertical axis): reverse each row.
fn flip(t: &Tile) -> Tile {
    t.iter()
        .map(|row| row.iter().rev().copied().collect())
        .collect()
}

/// All 8 orientations of a tile. For symmetric tiles some will coincide;
/// duplicate HashMap inserts are harmless since they map to the same RHS.
fn orientations(t: &Tile) -> Vec<Tile> {
    let mut out = Vec::with_capacity(8);
    let mut cur = t.clone();
    for _ in 0..4 {
        out.push(flip(&cur));
        out.push(cur.clone());
        cur = rotate(&cur);
    }
    out
}

/// Build the rule table: every orientation of each LHS → that rule's RHS.
fn parse_rules(input: &str) -> HashMap<String, Tile> {
    let mut rules = HashMap::new();
    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let (lhs, rhs) = line.split_once(" => ").expect("malformed rule");
        let output = parse_tile(rhs);
        for variant in orientations(&parse_tile(lhs)) {
            rules.insert(tile_to_string(&variant), output.clone());
        }
    }
    rules
}

/// The fixed starting pattern the puzzle specifies.
fn initial_grid() -> Tile {
    parse_tile(".#./..#/###")
}

/// One enhancement step. Split N×N into blocks of side `b` (2 if N is even,
/// 3 otherwise), replace each via `rules`, then stitch back. Output side
/// is `(N/b) * (b+1)`.
fn step(grid: &Tile, rules: &HashMap<String, Tile>) -> Tile {
    let n = grid.len();
    let b = if n.is_multiple_of(2) { 2 } else { 3 };
    let out_b = b + 1;
    let blocks_per_side = n / b;
    let new_n = blocks_per_side * out_b;

    let mut out: Tile = vec![vec![false; new_n]; new_n];

    for br in 0..blocks_per_side {
        for bc in 0..blocks_per_side {
            // Extract the source block.
            let block: Tile = (0..b)
                .map(|r| (0..b).map(|c| grid[br * b + r][bc * b + c]).collect())
                .collect();

            // Look up its enhancement.
            let enhanced = rules
                .get(&tile_to_string(&block))
                .unwrap_or_else(|| panic!("no rule for block {}", tile_to_string(&block)));

            // Copy the enhanced block into the output at the scaled offset.
            for r in 0..out_b {
                for c in 0..out_b {
                    out[br * out_b + r][bc * out_b + c] = enhanced[r][c];
                }
            }
        }
    }

    out
}

/// Count pixels currently on.
fn count_on(grid: &Tile) -> usize {
    grid.iter().flatten().filter(|&&b| b).count()
}

fn iterate(input: &str, iterations: usize) -> usize {
    let rules = parse_rules(input);
    let mut grid = initial_grid();
    for _ in 0..iterations {
        grid = step(&grid, &rules);
    }
    count_on(&grid)
}

pub fn solve_part1(input: &str) -> usize {
    iterate(input, 5)
}

pub fn solve_part2(input: &str) -> usize {
    iterate(input, 18)
}

pub fn solve(input: &str) -> (usize, usize) {
    // Part 2 subsumes part 1, but the puzzle reports both, so run both.
    (solve_part1(input), solve_part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_serialize_roundtrip() {
        let s = ".#./..#/###";
        assert_eq!(tile_to_string(&parse_tile(s)), s);
    }

    #[test]
    fn test_rotate_matches_problem_statement() {
        // From the puzzle: these four are all the same rule pattern.
        //   .#.    #..    ###    .#.
        //   ..#    #.#    ..#    #..
        //   ###    ##.    .#.    ###
        let start = parse_tile(".#./..#/###");
        let r1 = rotate(&start);
        assert_eq!(tile_to_string(&r1), "#../#.#/##.");
        let r2 = rotate(&r1);
        assert_eq!(tile_to_string(&r2), "###/#../.#.");
        let r3 = rotate(&r2);
        assert_eq!(tile_to_string(&r3), ".##/#.#/..#");
        let r4 = rotate(&r3);
        assert_eq!(tile_to_string(&r4), ".#./..#/###"); // back to start
    }

    #[test]
    fn test_flip() {
        // .#.    .#.       #.    .#
        // ..#    #..   ;   ..    ..
        // ###    ###
        assert_eq!(tile_to_string(&flip(&parse_tile(".#./..#/###"))), ".#./#../###");
        assert_eq!(tile_to_string(&flip(&parse_tile("#./.."))), ".#/..");
    }

    #[test]
    fn test_rule_lookup_hits_all_orientations() {
        // From the puzzle example: rotations/flips of `../.#` should all
        // map to the same RHS.
        let rules = parse_rules("../.# => ##./#../...\n");
        let expected = parse_tile("##./#../...");
        for variant in ["../.#", ".#/..", "#./..", "../#."] {
            assert_eq!(rules.get(variant), Some(&expected), "missed variant {variant}");
        }
    }

    #[test]
    fn test_two_iterations_matches_puzzle_example() {
        // Straight from the puzzle: after 2 iterations with this 2-rule
        // book, 12 pixels are on.
        let rules_text = "\
            ../.# => ##./#../...\n\
            .#./..#/### => #..#/..../..../#..#\n";
        let rules = parse_rules(rules_text);
        let mut grid = initial_grid();
        grid = step(&grid, &rules);
        assert_eq!(grid.len(), 4, "3→4 after /3 rule");
        grid = step(&grid, &rules);
        assert_eq!(grid.len(), 6, "4→6 after /2 rule");
        assert_eq!(count_on(&grid), 12);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day21.txt");
        assert_eq!(solve_part1(input), 167);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day21.txt");
        assert_eq!(solve_part2(input), 2_425_195);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day21.txt");
        assert_eq!(solve(input), (167, 2_425_195));
    }

    #[test]
    fn test_parse_real_ruleset_size() {
        // Sanity: every rule expands to up to 8 keys. With 108 input rules
        // we expect somewhere in [108, 864] entries depending on symmetry.
        let input = include_str!("../../inputs/day21.txt");
        let rules = parse_rules(input);
        assert!(rules.len() >= 108);
        assert!(rules.len() <= 108 * 8);
    }
}
