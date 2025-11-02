/// AoC 2015 Day 25: Let It Snow
///
/// Part 1: Compute the code at a given row and column in a diagonally-filled grid
///          using the multiplicative generator:
///          next = (prev * 252533) % 33554393, starting at 20151125 at (1,1).
///
/// There is no real Part 2 for Day 25; we return a celebratory message.
use anyhow::{anyhow, Result};
use regex::Regex;

const MOD: u64 = 33_554_393;
const MUL: u64 = 252_533;
const START: u64 = 20_151_125; // value at (1,1)

/// Parse input like: "... row 2981, column 3075." -> (2981, 3075)
pub fn parse_row_col(input: &str) -> Result<(u64, u64)> {
    let re = Regex::new(r"row\s+(\d+),\s*column\s+(\d+)").unwrap();
    if let Some(caps) = re.captures(input) {
        let row: u64 = caps[1].parse()?;
        let col: u64 = caps[2].parse()?;
        Ok((row, col))
    } else {
        Err(anyhow!("Unable to parse row/column from input"))
    }
}

/// Compute the 1-based sequence index for (row, col) in the diagonal order.
/// Diagonal s = row + col - 1; number of elements before diag s is (s-1)*s/2; position within diag is col.
/// Example (from the puzzle prompt): row=2981, col=3075
///   s = 2981 + 3075 - 1 = 6055
///   before = (6054 * 6055) / 2 = 18,328,485
///   index n = before + col = 18,328,485 + 3,075 = 18,331,560
fn index_for(row: u64, col: u64) -> u64 {
    let s = row + col - 1;
    let before = (s - 1) * s / 2;
    before + col
}

/// Fast modular exponentiation: base^exp mod MOD
fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut acc: u64 = 1;
    while exp > 0 {
        if exp & 1 == 1 {
            acc = ((acc as u128 * base as u128) % m as u128) as u64;
        }
        base = ((base as u128 * base as u128) % m as u128) as u64;
        exp >>= 1;
    }
    acc
}

/// Compute the code at (row, col)
pub fn code_at(row: u64, col: u64) -> u64 {
    let n = index_for(row, col);
    if n == 1 {
        return START;
    }
    let pow = mod_pow(MUL, n - 1, MOD);
    ((START as u128 * pow as u128) % MOD as u128) as u64
}

pub fn solve_part1(input: &str) -> Result<String> {
    let (row, col) = parse_row_col(input)?;
    Ok(code_at(row, col).to_string())
}

pub fn solve_part2(_input: &str) -> Result<String> {
    Ok("Merry Christmas!".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_mapping_small() {
        // From the diagram in the problem statement
        assert_eq!(index_for(1, 1), 1);
        assert_eq!(index_for(2, 1), 2);
        assert_eq!(index_for(1, 2), 3);
        assert_eq!(index_for(3, 1), 4);
        assert_eq!(index_for(2, 2), 5);
        assert_eq!(index_for(1, 3), 6);
        assert_eq!(index_for(4, 2), 12);
        assert_eq!(index_for(1, 5), 15);
    }

    #[test]
    fn test_known_codes_from_table() {
        // Selected values from the provided table
        assert_eq!(code_at(1, 1), 20151125);
        assert_eq!(code_at(2, 1), 31916031);
        assert_eq!(code_at(1, 2), 18749137);
        assert_eq!(code_at(4, 2), 32451966);
        assert_eq!(code_at(1, 5), 10071777);
    }
}
