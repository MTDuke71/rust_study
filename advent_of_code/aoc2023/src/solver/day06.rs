use anyhow::Result;

pub fn solve_part1(input: &str) -> Result<String> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.len() < 2 {
        anyhow::bail!("Input must have at least 2 lines");
    }

    // Parse times and distances
    let times = parse_numbers(lines[0])?;
    let distances = parse_numbers(lines[1])?;

    if times.len() != distances.len() {
        anyhow::bail!("Times and distances must have same count");
    }

    // For each race, count ways to win
    let mut product = 1;
    for (time, record) in times.iter().zip(distances.iter()) {
        let ways = count_ways_to_win(*time, *record);
        product *= ways;
    }

    Ok(product.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.len() < 2 {
        anyhow::bail!("Input must have at least 2 lines");
    }

    // Parse as single concatenated numbers (ignore spaces)
    let time = parse_concatenated_number(lines[0])?;
    let distance = parse_concatenated_number(lines[1])?;

    let ways = count_ways_quadratic(time, distance);

    Ok(ways.to_string())
}

/// Parse a line like "Time:      7  15   30" into vec of numbers
fn parse_numbers(line: &str) -> Result<Vec<u64>> {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() != 2 {
        anyhow::bail!("Expected 'Label: numbers' format");
    }

    parts[1]
        .split_whitespace()
        .map(|s| {
            s.parse::<u64>()
                .map_err(|e| anyhow::anyhow!("Parse error: {}", e))
        })
        .collect()
}

/// Parse a line like "Time:      7  15   30" into single concatenated number 71530
fn parse_concatenated_number(line: &str) -> Result<u64> {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() != 2 {
        anyhow::bail!("Expected 'Label: numbers' format");
    }

    let concatenated: String = parts[1].chars().filter(|c| !c.is_whitespace()).collect();

    concatenated
        .parse::<u64>()
        .map_err(|e| anyhow::anyhow!("Parse error: {}", e))
}

/// Count how many hold times beat the record using quadratic formula
///
/// # Mathematical Foundation
///
/// Uses **quadratic equation solving** to find boundaries in O(1) time.
/// See `zettelkasten/math-foundations/quadratic-equations.md` for theory.
///
/// For time T and record distance R:
/// Distance = hold_time * (T - hold_time) = -h² + T*h
/// Need: -h² + T*h > R
/// Or: h² - T*h + R < 0
///
/// Solve h² - T*h + R = 0 using quadratic formula:
/// h = (T ± √(T² - 4R)) / 2
///
/// Winners are integers strictly between the two roots
fn count_ways_quadratic(time: u64, record: u64) -> u64 {
    let t = time as f64;
    let r = record as f64;

    // Discriminant: T² - 4R
    let discriminant = t * t - 4.0 * r;

    if discriminant < 0.0 {
        return 0; // No real solutions
    }

    let sqrt_disc = discriminant.sqrt();

    // Two roots of the quadratic equation
    let root1 = (t - sqrt_disc) / 2.0;
    let root2 = (t + sqrt_disc) / 2.0;

    // We need integers STRICTLY between the roots (> record, not >= record)
    // Adjust if roots are exactly integers (edge case: exact ties with record)
    let min_hold = if (root1.fract() == 0.0) && ((root1 as u64) * (time - (root1 as u64)) == record)
    {
        (root1 as u64) + 1
    } else {
        root1.ceil() as u64
    };

    let max_hold = if (root2.fract() == 0.0) && ((root2 as u64) * (time - (root2 as u64)) == record)
    {
        (root2 as u64) - 1
    } else {
        root2.floor() as u64
    };

    if max_hold >= min_hold {
        max_hold - min_hold + 1
    } else {
        0
    }
}

/// Count how many hold times beat the record (brute force for small inputs)
/// For time T and record distance R:
/// Distance = hold_time * (T - hold_time) = -h² + T*h
/// Need: -h² + T*h > R
/// Or: h² - T*h + R < 0
fn count_ways_to_win(time: u64, record: u64) -> u64 {
    let mut count = 0;

    // Brute force for now - check each possible hold time
    for hold in 0..=time {
        let distance = hold * (time - hold);
        if distance > record {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn test_count_ways_to_win() {
        // Race 1: time=7, record=9
        assert_eq!(count_ways_to_win(7, 9), 4);

        // Race 2: time=15, record=40
        assert_eq!(count_ways_to_win(15, 40), 8);

        // Race 3: time=30, record=200
        assert_eq!(count_ways_to_win(30, 200), 9);
    }

    #[test]
    fn test_example_part1() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result, "288");
    }

    #[test]
    fn test_example_part2() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result, "71503");
    }

    #[test]
    fn test_parse_numbers() {
        let times = parse_numbers("Time:      7  15   30").unwrap();
        assert_eq!(times, vec![7, 15, 30]);

        let distances = parse_numbers("Distance:  9  40  200").unwrap();
        assert_eq!(distances, vec![9, 40, 200]);
    }
}
