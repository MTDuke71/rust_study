//! Day 21: Fractal Art — Bit-packed alternate implementation
//!
//! Same algorithm as [`super::day21`], different tile representation:
//!
//! | | Reference ([`super::day21`]) | This module |
//! |---|---|---|
//! | Tile key | `String` like `"##./.../..#"` | `u16` row-major bitmask |
//! | Rule map | `HashMap<String, Vec<Vec<bool>>>` | `[u16; 16]` + `[u16; 512]` direct-indexed LUTs |
//! | Grid storage | `Vec<Vec<bool>>` (row-of-Vec) | flat `Vec<bool>` with `r * n + c` indexing |
//! | Rotation/flip | index math on 2D `Vec` | bit permutation on `u16` |
//!
//! Lookup drops from `HashMap::get` + `tile_to_string` to one array index.
//! Grid storage drops one level of indirection. No algorithmic change —
//! still splits/enhances/stitches per step, still runs 18 iterations for
//! Part 2.

/// Flat row-major grid. Cells are `cells[r * n + c]`.
pub(crate) struct Grid {
    pub(crate) cells: Vec<bool>,
    pub(crate) n: usize,
}

/// Pack a b×b block of `grid` starting at (r0, c0) into row-major bit form.
/// Bit `r * b + c` of the result is cell (r0+r, c0+c).
pub(crate) fn pack_block(grid: &Grid, r0: usize, c0: usize, b: usize) -> u16 {
    let mut t = 0u16;
    for r in 0..b {
        for c in 0..b {
            if grid.cells[(r0 + r) * grid.n + (c0 + c)] {
                t |= 1 << (r * b + c);
            }
        }
    }
    t
}

/// Inverse of `pack_block`: write a b×b tile into `cells` at (r0, c0),
/// where `cells` has side length `n`.
fn unpack_block(tile: u16, cells: &mut [bool], n: usize, r0: usize, c0: usize, b: usize) {
    for r in 0..b {
        for c in 0..b {
            cells[(r0 + r) * n + (c0 + c)] = ((tile >> (r * b + c)) & 1) == 1;
        }
    }
}

/// Rotate a b×b bit-packed tile 90° CW. Bit `(r, c)` in the rotated tile
/// comes from bit `(b-1-c, r)` in the original — the same coordinate
/// transform as the reference implementation, applied to packed bits.
fn rotate_tile(t: u16, b: usize) -> u16 {
    let mut out = 0u16;
    for r in 0..b {
        for c in 0..b {
            let src_bit = (b - 1 - c) * b + r;
            if (t >> src_bit) & 1 == 1 {
                out |= 1 << (r * b + c);
            }
        }
    }
    out
}

/// Horizontal flip of a b×b bit-packed tile.
fn flip_tile(t: u16, b: usize) -> u16 {
    let mut out = 0u16;
    for r in 0..b {
        for c in 0..b {
            let src_bit = r * b + (b - 1 - c);
            if (t >> src_bit) & 1 == 1 {
                out |= 1 << (r * b + c);
            }
        }
    }
    out
}

/// All 8 orientations of a tile (D₄ orbit). May contain duplicates for
/// symmetric tiles — harmless for LUT writes.
fn orientations_u16(t: u16, b: usize) -> [u16; 8] {
    let mut out = [0u16; 8];
    let mut cur = t;
    for i in 0..4 {
        out[i * 2] = flip_tile(cur, b);
        out[i * 2 + 1] = cur;
        cur = rotate_tile(cur, b);
    }
    out
}

/// Parse a puzzle-format tile string (e.g. `"##./.../..#"`) into
/// `(packed, side)`.
fn parse_tile_u16(s: &str) -> (u16, usize) {
    let rows: Vec<&str> = s.split('/').collect();
    let b = rows.len();
    let mut t = 0u16;
    for (r, row) in rows.iter().enumerate() {
        for (c, ch) in row.chars().enumerate() {
            if ch == '#' {
                t |= 1 << (r * b + c);
            }
        }
    }
    (t, b)
}

/// Build the two lookup tables: `rules_2[k]` is the enhancement of the
/// 2×2 tile with bitmask `k`; `rules_3[k]` the same for 3×3 tiles.
///
/// 2×2 has 16 possible bitmasks, 3×3 has 512. With D₄ expansion at parse
/// time every reachable tile maps to its output — no misses possible at
/// run time if the rulebook is well-formed.
pub(crate) fn parse_rules(input: &str) -> (Box<[u16; 16]>, Box<[u16; 512]>) {
    let mut rules_2 = Box::new([0u16; 16]);
    let mut rules_3 = Box::new([0u16; 512]);
    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let (lhs_s, rhs_s) = line.split_once(" => ").expect("malformed rule");
        let (lhs, b) = parse_tile_u16(lhs_s);
        let (rhs, _) = parse_tile_u16(rhs_s);
        for &variant in orientations_u16(lhs, b).iter() {
            match b {
                2 => rules_2[variant as usize] = rhs,
                3 => rules_3[variant as usize] = rhs,
                other => panic!("unexpected LHS side length {other}"),
            }
        }
    }
    (rules_2, rules_3)
}

pub(crate) fn initial_grid() -> Grid {
    // ".#./..#/###" → bit-by-bit: (0,1)=1, (1,2)=1, (2,0..2)=1 but stored
    // as cells directly for clarity — the bit-packing only kicks in at
    // lookup boundaries.
    let mut cells = vec![false; 9];
    for (r, row) in [".#.", "..#", "###"].iter().enumerate() {
        for (c, ch) in row.chars().enumerate() {
            cells[r * 3 + c] = ch == '#';
        }
    }
    Grid { cells, n: 3 }
}

pub(crate) fn step(grid: &Grid, rules_2: &[u16; 16], rules_3: &[u16; 512]) -> Grid {
    let n = grid.n;
    let b = if n.is_multiple_of(2) { 2 } else { 3 };
    let out_b = b + 1;
    let blocks_per_side = n / b;
    let new_n = blocks_per_side * out_b;
    let mut out = vec![false; new_n * new_n];

    for br in 0..blocks_per_side {
        for bc in 0..blocks_per_side {
            let tile = pack_block(grid, br * b, bc * b, b);
            let enhanced = if b == 2 {
                rules_2[tile as usize]
            } else {
                rules_3[tile as usize]
            };
            unpack_block(enhanced, &mut out, new_n, br * out_b, bc * out_b, out_b);
        }
    }

    Grid { cells: out, n: new_n }
}

fn count_on(grid: &Grid) -> usize {
    grid.cells.iter().filter(|&&b| b).count()
}

fn iterate(input: &str, iterations: usize) -> usize {
    let (rules_2, rules_3) = parse_rules(input);
    let mut grid = initial_grid();
    for _ in 0..iterations {
        grid = step(&grid, &rules_2, &rules_3);
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
    (solve_part1(input), solve_part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tile_roundtrip() {
        let (t, b) = parse_tile_u16(".#./..#/###");
        assert_eq!(b, 3);
        // Bits: (0,1), (1,2), (2,0), (2,1), (2,2)
        // Positions in u16: 1, 5, 6, 7, 8  → 0b1_1110_0010 = 0x1E2 = 482
        assert_eq!(t, 0b1_1110_0010);
    }

    #[test]
    fn test_rotate_matches_reference() {
        // .#./..#/### rotated 90° CW should match reference: #../#.#/##.
        let (start, b) = parse_tile_u16(".#./..#/###");
        let rotated = rotate_tile(start, b);
        let (expected, _) = parse_tile_u16("#../#.#/##.");
        assert_eq!(rotated, expected);
    }

    #[test]
    fn test_flip_matches_reference() {
        let (start, b) = parse_tile_u16(".#./..#/###");
        let flipped = flip_tile(start, b);
        let (expected, _) = parse_tile_u16(".#./#../###");
        assert_eq!(flipped, expected);
    }

    #[test]
    fn test_two_iterations_matches_puzzle_example() {
        let rules_text = "\
            ../.# => ##./#../...\n\
            .#./..#/### => #..#/..../..../#..#\n";
        let (rules_2, rules_3) = parse_rules(rules_text);
        let mut grid = initial_grid();
        grid = step(&grid, &rules_2, &rules_3);
        assert_eq!(grid.n, 4);
        grid = step(&grid, &rules_2, &rules_3);
        assert_eq!(grid.n, 6);
        assert_eq!(count_on(&grid), 12);
    }

    #[test]
    fn test_part1_matches_reference() {
        let input = include_str!("../../inputs/day21.txt");
        assert_eq!(solve_part1(input), 167);
    }

    #[test]
    fn test_part2_matches_reference() {
        let input = include_str!("../../inputs/day21.txt");
        assert_eq!(solve_part2(input), 2_425_195);
    }
}
