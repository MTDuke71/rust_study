use anyhow::Result;
use md5;

// --- Day 4: The Ideal Stocking Stuffer ---
// Santa needs help mining some AdventCoins (very similar to bitcoins) to use as gifts for all the economically forward-thinking little girls and boys.

// To do this, he needs to find MD5 hashes which, in hexadecimal, start with at least five zeroes.
// The input to the MD5 hash is some secret key (your puzzle input, given below) followed by a number in decimal.
// To mine AdventCoins, you must find Santa the lowest positive number (no leading zeroes: 1, 2, 3, ...) that produces such a hash.

// For example:

// If your secret key is abcdef, the answer is 609043, because the MD5 hash of abcdef609043 starts with five zeroes (000001dbbfa...), and it is the lowest such number to do so.
// If your secret key is pqrstuv, the lowest number it combines with to make an MD5 hash starting with five zeroes is 1048970;
// that is, the MD5 hash of pqrstuv1048970 looks like 000006136ef....
// Your puzzle input is yzbqklnj.
// Answer:
// You can also [Share] this puzzle.

/// Day 04: The Ideal Stocking Stuffer
/// Part 1: Find the lowest positive number that produces an MD5 hash starting with five zeroes
/// Part 2: Find the lowest positive number that produces an MD5 hash starting with six zeroes
/// Key Concepts: MD5 hashing, brute force search, cryptographic hardness, computational complexity, prefix matching
pub fn solve_part1(input: &str) -> Result<String> {
    let secret_key = input.trim();
    let mut count = 0;
    let mut hash = String::new();
    while !hash.starts_with("00000") {
        count += 1;
        let data = format!("{secret_key}{count}");
        hash = format!("{:x}", md5::compute(data));
        // println!("{data} -> {hash}");
    }
    Ok(count.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let secret_key = input.trim();
    let mut count = 0;
    let mut hash = String::new();
    while !hash.starts_with("000000") {
        count += 1;
        let data = format!("{secret_key}{count}");
        hash = format!("{:x}", md5::compute(data));
        // println!("{data} -> {hash}");
    }
    Ok(count.to_string())
}
