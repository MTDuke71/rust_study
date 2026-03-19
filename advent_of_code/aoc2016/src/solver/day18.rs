//! Day 18: Like a Rogue
//!
//! Generate rows of trap/safe tiles based on the previous row and count safe tiles.

/// Parsed representation of the first row where `true` means trap (`^`).
type Row = Vec<bool>;

fn parse_input(input: &str) -> Row {
    input.trim().bytes().map(|b| b == b'^').collect()
}

#[inline]
fn count_safe_in_row(row: &[bool]) -> usize {
    row.iter().filter(|&&is_trap| !is_trap).count()
}

/// Count safe tiles for `rows` total rows (including the initial row).
///
/// Trap rule simplifies to: next tile is trap iff left and right differ (XOR).
fn count_safe_tiles(first_row: &[bool], rows: usize) -> usize {
    if rows == 0 {
        return 0;
    }

    let width = first_row.len();
    let mut current = first_row.to_vec();
    let mut next = vec![false; width];

    let mut safe_tiles = count_safe_in_row(&current);

    for _ in 1..rows {
        for index in 0..width {
            let left = if index == 0 { false } else { current[index - 1] };
            let right = if index + 1 == width {
                false
            } else {
                current[index + 1]
            };

            next[index] = left ^ right;
        }

        safe_tiles += count_safe_in_row(&next);
        std::mem::swap(&mut current, &mut next);
    }

    safe_tiles
}

fn solve_part1_with_data(first_row: &[bool]) -> usize {
    count_safe_tiles(first_row, 40)
}

fn solve_part2_with_data(first_row: &[bool]) -> usize {
    count_safe_tiles(first_row, 400_000)
}

/// Count safe tiles using bit-parallel operations with u128.
/// Only works if row width <= 128 bits.
/// This is ~1000x faster for large row counts (Part 2) because:
/// - All tiles update in parallel (shifts + XOR in registers)
/// - count_ones() is a single CPU instruction (popcount)
/// - No per-row allocations or branching
fn count_safe_tiles_bitset(first_row: &[bool], rows: usize) -> usize {
    if rows == 0 {
        return 0;
    }

    let width = first_row.len();
    if width > 128 {
        // Fallback to vector approach for very wide rows
        return count_safe_tiles(first_row, rows);
    }

    // Convert boolean row to u128: true (trap) -> 1, false (safe) -> 0
    let mut curr_line: u128 = first_row
        .iter()
        .enumerate()
        .map(|(i, &is_trap)| if is_trap { 1u128 << (width - 1 - i) } else { 0 })
        .sum();

    // Mask to keep only the valid bits (prevents overflow artifacts)
    let valid_bits_mask = (1u128 << width) - 1;

    // Count initial traps
    let mut total_traps = curr_line.count_ones() as usize;

    // Generate rows and count traps
    for _ in 1..rows {
        let left = curr_line >> 1;   // tiles to the left (leftmost gets 0)
        let right = curr_line << 1;  // tiles to the right (rightmost gets 0)
        curr_line = (left ^ right) & valid_bits_mask; // XOR and mask
        total_traps += curr_line.count_ones() as usize;
    }

    // Total tiles minus traps = safe tiles
    let total_tiles = rows * width;
    total_tiles - total_traps
}

pub fn solve_part2_bitset(input: &str) -> String {
    let data = parse_input(input);
    count_safe_tiles_bitset(&data, 400_000).to_string()
}

pub fn solve(input: &str) -> (String, String) {
    let data = parse_input(input);
    (
        solve_part1_with_data(&data).to_string(),
        solve_part2_with_data(&data).to_string(),
    )
}

pub fn solve_part1(input: &str) -> String {
    let data = parse_input(input);
    solve_part1_with_data(&data).to_string()
}

pub fn solve_part2(input: &str) -> String {
    let data = parse_input(input);
    solve_part2_with_data(&data).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse_input("..^^."), vec![false, false, true, true, false]);
    }

    #[test]
    fn test_example_three_rows() {
        assert_eq!(count_safe_tiles(&parse_input("..^^."), 3), 6);
    }

    #[test]
    fn test_example_ten_rows() {
        assert_eq!(count_safe_tiles(&parse_input(".^^.^.^^^^"), 10), 38);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day18.txt");
        let (part1, _) = solve(input);
        assert_eq!(part1, "1978");
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day18.txt");
        let (_, part2) = solve(input);
        assert_eq!(part2, "20003246");
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day18.txt");
        assert_eq!(solve(input), ("1978".to_string(), "20003246".to_string()));
    }

    #[test]
    fn test_bitset_example_three_rows() {
        assert_eq!(count_safe_tiles_bitset(&parse_input("..^^."), 3), 6);
    }

    #[test]
    fn test_bitset_example_ten_rows() {
        assert_eq!(count_safe_tiles_bitset(&parse_input(".^^.^.^^^^"), 10), 38);
    }

    #[test]
    fn test_bitset_part2_actual() {
        let input = include_str!("../../inputs/day18.txt");
        assert_eq!(solve_part2_bitset(input), "20003246");
    }

    #[test]
    fn test_bitset_vs_vector_equivalence() {
        // Verify bitset and vector implementations produce identical results
        let input = include_str!("../../inputs/day18.txt");
        let data = parse_input(input);
        assert_eq!(
            count_safe_tiles(&data, 400_000),
            count_safe_tiles_bitset(&data, 400_000)
        );
    }
}
