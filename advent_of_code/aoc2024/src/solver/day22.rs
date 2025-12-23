//! Day 22: Monkey Market
//! 
//! Pseudorandom number generation (PRNG) using mix (XOR) and prune (modulo 2^24).
//! 
//! Part 1: Generate 2000th secret number for each buyer, sum all results
//! Part 2: Find best sequence of 4 price changes to maximize banana sales

const PRUNE_MODULO: i64 = 16777216; // 2^24

/// Mix operation: Bitwise XOR
/// 
/// Properties:
/// - Self-inverse: (a ^ b) ^ b = a
/// - Avalanche effect: 1 bit change affects ~50% of output
/// - Uniform distribution: each bit has 50% probability
#[inline]
fn mix(secret: i64, value: i64) -> i64 {
    secret ^ value
}

/// Prune operation: Modulo 2^24
/// 
/// Keeps secret number bounded to 24 bits.
/// Compiler optimizes to: secret & 0xFFFFFF (bitwise AND mask)
#[inline]
fn prune(secret: i64) -> i64 {
    secret % PRUNE_MODULO
}

/// Evolve secret number one step through the PRNG algorithm.
/// 
/// Process:
/// 1. Multiply by 64, mix, prune
/// 2. Divide by 32 (integer division), mix, prune
/// 3. Multiply by 2048, mix, prune
/// 
/// Returns the next secret number in the sequence.
fn evolve_secret(mut secret: i64) -> i64 {
    // Step 1: Multiply by 64, mix, prune
    let step1 = secret * 64;
    secret = mix(secret, step1);
    secret = prune(secret);
    
    // Step 2: Divide by 32, mix, prune
    let step2 = secret / 32;
    secret = mix(secret, step2);
    secret = prune(secret);
    
    // Step 3: Multiply by 2048, mix, prune
    let step3 = secret * 2048;
    secret = mix(secret, step3);
    secret = prune(secret);
    
    secret
}

/// Generate the Nth secret number from an initial secret.
/// 
/// # Arguments
/// * `initial` - Starting secret number
/// * `iterations` - Number of evolution steps (2000 for Part 1)
/// 
/// # Returns
/// The secret number after N iterations
fn generate_nth_secret(initial: i64, iterations: usize) -> i64 {
    let mut secret = initial;
    for _ in 0..iterations {
        secret = evolve_secret(secret);
    }
    secret
}

pub fn part1(input: &str) -> String {
    let sum: i64 = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let initial = line.trim().parse::<i64>().expect("Invalid number");
            generate_nth_secret(initial, 2000)
        })
        .sum();
    
    sum.to_string()
}

pub fn part2(input: &str) -> String {
    use std::collections::HashMap;
    
    // Track total bananas for each sequence across all buyers
    let mut sequence_totals: HashMap<[i8; 4], i64> = HashMap::new();
    
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        let initial: i64 = line.parse().expect("Invalid number");
        
        // Track sequences this buyer has already seen (first occurrence only)
        let mut buyer_sequences: HashMap<[i8; 4], i64> = HashMap::new();
        
        // Generate prices and changes for this buyer
        let mut secret = initial;
        let mut prev_price = (secret % 10) as i8;
        let mut changes: Vec<i8> = Vec::with_capacity(2000);
        
        // Generate 2000 new secrets and track prices/changes
        for _ in 0..2000 {
            secret = evolve_secret(secret);
            let price = (secret % 10) as i8;
            let change = price - prev_price;
            changes.push(change);
            prev_price = price;
            
            // Once we have 4 changes, start tracking sequences
            if changes.len() >= 4 {
                let sequence = [
                    changes[changes.len() - 4],
                    changes[changes.len() - 3],
                    changes[changes.len() - 2],
                    changes[changes.len() - 1],
                ];
                
                // Only record first occurrence for this buyer
                buyer_sequences.entry(sequence).or_insert(price as i64);
            }
        }
        
        // Add this buyer's prices to the global totals
        for (sequence, price) in buyer_sequences {
            *sequence_totals.entry(sequence).or_insert(0) += price;
        }
    }
    
    // Find the sequence with maximum total bananas
    sequence_totals
        .values()
        .max()
        .copied()
        .unwrap_or(0)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        // Example from problem: 42 ^ 15 = 37
        assert_eq!(mix(42, 15), 37);
        
        // Self-inverse property: (a ^ b) ^ b = a
        let a = 123;
        let b = 456;
        assert_eq!(mix(mix(a, b), b), a);
    }

    #[test]
    fn test_prune() {
        // Example from problem: 100000000 % 16777216 = 16113920
        assert_eq!(prune(100000000), 16113920);
        
        // Numbers smaller than modulo are unchanged
        assert_eq!(prune(1000), 1000);
        
        // Power of 2 modulo behavior
        assert_eq!(prune(16777216), 0);
        assert_eq!(prune(16777217), 1);
    }

    #[test]
    fn test_evolve_secret_first_step() {
        // Verify first evolution: 123 → 15887950
        let initial = 123;
        let next = evolve_secret(initial);
        assert_eq!(next, 15887950);
    }

    #[test]
    fn test_evolve_secret_sequence() {
        // Test the 10-step sequence from the problem
        let expected = vec![
            15887950,
            16495136,
            527345,
            704524,
            1553684,
            12683156,
            11100544,
            12249484,
            7753432,
            5908254,
        ];
        
        let mut secret = 123;
        for (i, &exp) in expected.iter().enumerate() {
            secret = evolve_secret(secret);
            assert_eq!(secret, exp, "Mismatch at iteration {}", i + 1);
        }
    }

    #[test]
    fn test_generate_2000th_secret() {
        // Test cases from problem statement
        let test_cases = vec![
            (1, 8685429),
            (10, 4700978),
            (100, 15273692),
            (2024, 8667524),
        ];
        
        for (initial, expected_2000th) in test_cases {
            let result = generate_nth_secret(initial, 2000);
            assert_eq!(
                result, 
                expected_2000th,
                "Failed for initial secret {}: expected {}, got {}",
                initial,
                expected_2000th,
                result
            );
        }
    }

    #[test]
    fn test_part1_example() {
        let input = "1\n10\n100\n2024";
        let result = part1(input);
        
        // Sum: 8685429 + 4700978 + 15273692 + 8667524 = 37327623
        assert_eq!(result, "37327623");
    }

    #[test]
    fn test_part1_single_buyer() {
        // Test with single buyer
        assert_eq!(part1("1"), "8685429");
        assert_eq!(part1("10"), "4700978");
    }

    #[test]
    fn test_part1_with_whitespace() {
        // Test that we handle whitespace correctly
        let input = "  1  \n  10  \n\n  100  \n  2024  ";
        assert_eq!(part1(input), "37327623");
    }

    #[test]
    fn test_part1_with_file() {
        use std::fs;
        let input = fs::read_to_string("inputs/day22_example.txt").unwrap();
        let result = part1(&input);
        println!("Part 1 result: {}", result);
        // Expected: 17163502021 (from problem statement)
        // Just print for now to see what we get
    }

    #[test]
    fn test_price_extraction() {
        // Test that we correctly extract prices (last digit)
        // First ten prices: initial + 9 evolutions
        let mut secret = 123;
        let mut prices = vec![secret % 10];  // Initial price: 3
        
        for _ in 0..9 {  // Generate 9 more prices
            secret = evolve_secret(secret);
            prices.push(secret % 10);
        }
        
        // From problem: 3, 0, 6, 5, 4, 4, 6, 4, 4, 2
        assert_eq!(prices, vec![3, 0, 6, 5, 4, 4, 6, 4, 4, 2]);
    }

    #[test]
    fn test_price_changes() {
        // Test change calculation
        let mut secret = 123;
        let mut prev_price = (secret % 10) as i8;
        let mut changes = Vec::new();
        
        for _ in 0..9 {
            secret = evolve_secret(secret);
            let price = (secret % 10) as i8;
            changes.push(price - prev_price);
            prev_price = price;
        }
        
        // From problem: -3, 6, -1, -1, 0, 2, -2, 0, -2
        assert_eq!(changes, vec![-3, 6, -1, -1, 0, 2, -2, 0, -2]);
    }

    #[test]
    fn test_part2_example() {
        // From problem: buyers 1, 2, 3, 2024
        // Best sequence [-2, 1, -1, 3] yields: 7 + 7 + 0 + 9 = 23
        let input = "1\n2\n3\n2024\n";
        assert_eq!(part2(input), "23");
    }
}
