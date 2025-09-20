
use anyhow::Result;
use crate::parser::split_blocks;

/// Example Day 01: Given groups of numbers separated by blank lines,
/// part1 = max group sum, part2 = sum of top-3 group sums.
pub fn solve_part1(input: &str) -> Result<String> {
    let mut sums: Vec<i64> = split_blocks(input)
        .into_iter()
        .map(|b| b.lines().filter_map(|s| s.trim().parse::<i64>().ok()).sum())
        .collect();
    let ans = sums.into_iter().max().unwrap_or(0);
    Ok(ans.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let mut sums: Vec<i64> = split_blocks(input)
        .into_iter()
        .map(|b| b.lines().filter_map(|s| s.trim().parse::<i64>().ok()).sum())
        .collect();
    sums.sort_unstable_by(|a,b| b.cmp(a));
    let ans: i64 = sums.into_iter().take(3).sum();
    Ok(ans.to_string())
}
