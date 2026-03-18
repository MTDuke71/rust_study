//! Day 16: Dragon Checksum — O(1) Memory, O(checksum_length) Time
//!
//! This is a Rust port of the brilliant C solution by /u/askalski:
//! https://www.reddit.com/r/adventofcode/comments/5ititq/2016_day_16_c_how_to_tame_your_dragon_in_under_a/
//!
//! Key insight: You never need to generate the dragon curve data.
//! The checksum can be computed directly from the input using properties
//! of the dragon curve's structure and Gray codes.
//!
//! Complexity:
//!   Time:   O(checksum_length) = O(17) for our input
//!   Memory: O(1) — a few u64 registers, no allocation
//!
//! Usage: cargo run --example day16_dragon_o1

use std::time::Instant;

fn main() {
    let input = include_str!("../inputs/day16.txt").trim();

    println!("Day 16: Dragon Checksum — O(1) Memory Solution");
    println!("Input: {input}");
    println!();

    let start = Instant::now();

    // Precompute the input parity register once, reuse for both parts
    let input_parity = precompute_input_parity(input);
    let input_len = input.len() as u64;

    let part1 = solve(input_parity, input_len, 272);
    let part2 = solve(input_parity, input_len, 35_651_584);

    let elapsed = start.elapsed();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    println!("Time:   {elapsed:?}");

    // Verify answers
    assert_eq!(part1, "10010010110011010");
    assert_eq!(part2, "01010100101011100");
    println!("\nBoth answers verified!");
}

/// Precompute prefix parities of the input and its reverse complement.
///
/// The dragon curve fill repeats a pattern:
///   input, dragon_bit, reverse_complement(input), dragon_bit, input, ...
///
/// We pack both the input AND its reverse complement into a single u64
/// register as a prefix-parity scan. This lets us look up the parity
/// of any prefix of the repeating pattern with a single bit shift.
///
/// Example for input "10010" (len=5):
///   Forward positions 1-5:  bits of "10010"
///   Reverse complement positions 6-10: bits of "10110" (reversed + flipped)
///
/// Then the prefix-XOR scan makes bit i = parity of positions [0..i],
/// so we can get any prefix parity with a single shift + mask.
fn precompute_input_parity(input: &str) -> u64 {
    let input_bytes: Vec<u8> = input.bytes().collect();
    let input_len = input_bytes.len();
    let mut parity: u64 = 0;

    // Load forward bits and reverse complement bits into the register.
    // Shifted left by 1 so position 0 is unused (simplifies remainder == 0 case).
    //
    // For each position i from the front and j from the back:
    //   Forward:  bit at position (i+1) = input[i]
    //   Reverse:  bit at position j     = !input[i]  (complement of same char)
    let mut i = 0usize;
    let mut j = input_len * 2;
    while i < j {
        // Forward bit: is input[i] a '1'?
        parity ^= u64::from(input_bytes[i] != b'0') << (i + 1);
        // Reverse complement bit: is input[i] a '0'? (flipped)
        parity ^= u64::from(input_bytes[i] == b'0') << j;
        i += 1;
        j -= 1;
    }

    // Convert individual bits into prefix parities:
    // After this, bit i of `parity` = XOR of all original bits at positions [1..=i]
    //
    // This is a parallel prefix XOR scan using doubling:
    //   Step 1: XOR with shift 1 → each bit has parity of 2 consecutive bits
    //   Step 2: XOR with shift 2 → parity of 4 consecutive bits
    //   Step 4: XOR with shift 4 → parity of 8, etc.
    //   After 6 steps (shift up to 32), all 64 bits are prefix parities.
    let mut shift = 1;
    while shift < 64 {
        parity ^= parity << shift;
        shift <<= 1;
    }

    parity
}

/// Compute the checksum for a given disk size without generating any data.
///
/// The checksum has `disk_size / chunk_size` digits, where chunk_size = lowest set bit.
/// For disk_size 35,651,584 = 17 * 2^21, that's just 17 digits.
///
/// For each chunk boundary, we compute the CUMULATIVE parity of all bits [0..length].
/// The actual chunk parity = current cumulative XOR previous cumulative.
fn solve(input_parity: u64, input_len: u64, disk_size: u64) -> String {
    // Chunk size = lowest set bit of disk_size
    // Same as 2^trailing_zeros — this determines how many checksum digits we produce.
    //
    // Example: 35,651,584 = 0b10_0001_1111_1111_0000_0000_0000_0000_0
    //          lowest bit = 2^21 = 2,097,152
    //          checksum length = 35,651,584 / 2,097,152 = 17 digits
    let increment = lowest_set_bit(disk_size);

    let mut result = String::new();
    let mut previous_parity: u64 = 0;

    // Walk through chunk boundaries: increment, 2*increment, ..., disk_size
    let mut length = increment;
    while length <= disk_size {
        // === Decompose [0..length] into dragon bits + input bits ===
        //
        // The dragon fill has structure:
        //   [input] [dragon_bit] [rev_complement] [dragon_bit] [input] [dragon_bit] ...
        //
        // Every (input_len + 1) positions, there's one dragon joining bit.
        // So in [0..length]:
        //   dragons       = number of joining bits
        //   input bits    = length - dragons
        //   input_cycles  = how many complete forward+reverse copies
        //   input_remainder = leftover partial copy

        let dragons = length / (input_len + 1);
        let input_bits = length - dragons;
        let input_cycles = input_bits / (input_len * 2);
        let input_remainder = input_bits % (input_len * 2);

        // === Compute cumulative parity of all bits in [0..length] ===

        // 1) Parity of all dragon joining bits (the 0s between copies).
        //    These follow a pattern related to Gray codes — see dragon_parity().
        let mut p = dragon_parity(dragons);

        // 2) Parity of complete input cycles (forward + reverse complement).
        //    One full cycle (forward + reverse) has 2*input_len bits.
        //    Forward has some number of 1s. Reverse complement has
        //    (input_len - count_of_1s) ones. Together: input_len ones total.
        //    So each complete cycle contributes parity = input_len % 2.
        //    For input_cycles cycles: parity = (input_cycles * input_len) % 2.
        //    Bit trick: input_cycles & input_len gives LSB of the product.
        p ^= input_cycles & input_len;

        // 3) Parity of the remaining partial input.
        //    Look up from the precomputed prefix parity register.
        //    Shifting right by input_remainder positions the correct
        //    prefix parity bit into position 0... but we mask with & 1 later.
        p ^= input_parity >> input_remainder;

        // Only the least significant bit matters (it's a parity)
        p &= 1;

        // === Convert to checksum digit ===
        //
        // The checksum digit for this chunk = parity of bits in THIS chunk only.
        // Cumulative parity XOR previous cumulative = chunk parity.
        // Inverted: even parity → '1', odd → '0' (matches the pairing rule).
        let digit = if (p ^ previous_parity) == 0 { '1' } else { '0' };
        result.push(digit);

        previous_parity = p;
        length += increment;
    }

    result
}

/// Returns the lowest set bit of n.
///
/// Example: lowest_set_bit(272) = lowest_set_bit(0b100010000) = 0b10000 = 16
///
/// The classic bit trick: n & (-n) isolates the lowest 1-bit.
/// In two's complement, -n = !n + 1, which flips all bits up to (but not including)
/// the lowest 1-bit.
fn lowest_set_bit(n: u64) -> u64 {
    n & n.wrapping_neg()
}

/// Compute the parity of the first n dragon curve joining bits.
///
/// The dragon curve's joining bits (the 0s inserted between copies)
/// follow a pattern connected to Gray codes:
///
///   Position:  1  2  3  4  5  6  7  8  9 10 11 ...
///   Bit:       1  1  0  1  1  0  0  1  1  1  0 ...
///
/// (These are the "turn" bits of the dragon curve — whether to turn
/// left or right at each step.)
///
/// The parity of the first n bits can be computed using:
///   gray = n XOR (n >> 1)          — the Gray code of n
///   parity = (gray XOR popcount(n AND gray)) AND 1
///
/// This works because the dragon curve's recursive structure maps
/// directly onto the binary reflected Gray code. The Gray code captures
/// the alternating pattern of the dragon curve at each level of recursion.
fn dragon_parity(n: u64) -> u64 {
    let gray = n ^ (n >> 1);
    (gray ^ (n & gray).count_ones() as u64) & 1
}
