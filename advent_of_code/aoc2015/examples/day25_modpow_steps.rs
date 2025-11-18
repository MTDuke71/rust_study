use std::include_str;

// Verbose walkthrough of modular exponentiation steps for AoC 2015 Day 25.
// Prints each iteration of exponentiation-by-squaring for the input row/col.
//
// Run:
//   cargo run -p aoc2015 --example day25_modpow_steps

fn parse_row_col(input: &str) -> (u64, u64) {
    // Input format: "... row 2981, column 3075."
    let mut row = 0u64;
    let mut col = 0u64;
    for part in input.split(|c: char| !c.is_ascii_digit()) {
        if part.is_empty() {
            continue;
        }
        let val: u64 = part.parse().unwrap();
        if row == 0 {
            row = val;
        } else {
            col = val;
            break;
        }
    }
    (row, col)
}

fn index_for(row: u64, col: u64) -> u64 {
    let s = row + col - 1;
    let before = (s - 1) * s / 2;
    before + col
}

const MOD: u64 = 33_554_393;
const MUL: u64 = 252_533;
const START: u64 = 20_151_125;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../inputs/day25_example.txt");
    let (row, col) = parse_row_col(input);
    let n = index_for(row, col);
    let mut exp = n - 1; // we need MUL^(n-1)

    println!("AoC 2015 Day 25 – Verbose mod_pow Steps\n");
    println!(
        "Input: row={}, col={} -> index n={} (need exponent n-1={})",
        row, col, n, exp
    );
    println!("Constants: START={}, MUL={}, MOD={} \n", START, MUL, MOD);

    let mut acc: u64 = 1;
    let mut base: u64 = MUL;
    let mut step: usize = 0;

    println!(
        "{:>4} | {:>10} | {:>10} | {:>10} | {:>10}",
        "step", "exp(bit)", "acc(before)", "base(before)", "acc(after)"
    );
    println!("{}", "-".repeat(62));
    while exp > 0 {
        let bit = exp & 1;
        let acc_before = acc;
        let base_before = base;
        if bit == 1 {
            acc = ((acc as u128 * base as u128) % MOD as u128) as u64;
        }
        println!(
            "{:>4} | {:>6}({}) | {:>10} | {:>10} | {:>10}",
            step, exp, bit, acc_before, base_before, acc
        );
        base = ((base as u128 * base as u128) % MOD as u128) as u64;
        exp >>= 1;
        step += 1;
    }

    let pow = acc;
    let code = ((START as u128 * pow as u128) % MOD as u128) as u64;
    println!("\nResult: MUL^(n-1) mod MOD = {}", pow);
    println!(
        "Final calculation: (START * pow) % MOD = ({} * {}) % {} = {}",
        START, pow, MOD, code
    );
    println!("Final code at (row={}, col={}): {}", row, col, code);

    Ok(())
}
