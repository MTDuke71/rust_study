use std::collections::HashMap;

pub fn solve_part1(input: &str) -> usize {
    let (patterns, designs) = parse_input(input);
    
    designs
        .iter()
        .filter(|design| can_make_design(&patterns, design))
        .count()
}

pub fn solve_part2(input: &str) -> u64 {
    let (patterns, designs) = parse_input(input);
    
    designs
        .iter()
        .map(|design| count_ways_to_make(&patterns, design))
        .sum()
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    
    // If there's only one part, try splitting by single newline
    let (patterns_str, designs_str) = if parts.len() == 1 {
        // Find first newline and split there
        if let Some(first_newline) = input.find('\n') {
            (&input[..first_newline], &input[first_newline + 1..])
        } else {
            (input, "")
        }
    } else {
        (parts[0], parts[1])
    };
    
    let patterns: Vec<&str> = patterns_str
        .split(", ")
        .map(|s| s.trim())
        .collect();
    
    let designs: Vec<&str> = designs_str
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    
    (patterns, designs)
}

/// Check if a design can be made from available patterns
fn can_make_design(patterns: &[&str], design: &str) -> bool {
    let mut memo = HashMap::new();
    can_make_recursive(patterns, design, &mut memo)
}

fn can_make_recursive<'a>(
    patterns: &[&str],
    remaining: &'a str,
    memo: &mut HashMap<&'a str, bool>,
) -> bool {
    // Base case: empty string is always possible
    if remaining.is_empty() {
        return true;
    }
    
    // Check memo
    if let Some(&result) = memo.get(remaining) {
        return result;
    }
    
    // Try each pattern
    for pattern in patterns {
        if let Some(rest) = remaining.strip_prefix(pattern) {
            if can_make_recursive(patterns, rest, memo) {
                memo.insert(remaining, true);
                return true;
            }
        }
    }
    
    memo.insert(remaining, false);
    false
}

/// Count all possible ways to make a design
fn count_ways_to_make(patterns: &[&str], design: &str) -> u64 {
    let mut memo = HashMap::new();
    count_ways_recursive(patterns, design, &mut memo)
}

fn count_ways_recursive<'a>(
    patterns: &[&str],
    remaining: &'a str,
    memo: &mut HashMap<&'a str, u64>,
) -> u64 {
    // Base case: empty string has exactly one way (do nothing)
    if remaining.is_empty() {
        return 1;
    }
    
    // Check memo
    if let Some(&count) = memo.get(remaining) {
        return count;
    }
    
    let mut total_ways = 0;
    
    // Try each pattern
    for pattern in patterns {
        if let Some(rest) = remaining.strip_prefix(pattern) {
            total_ways += count_ways_recursive(patterns, rest, memo);
        }
    }
    
    memo.insert(remaining, total_ways);
    total_ways
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_parse_input() {
        let (patterns, designs) = parse_input(EXAMPLE);
        assert_eq!(patterns.len(), 8);
        assert_eq!(designs.len(), 8);
        assert_eq!(patterns[0], "r");
        assert_eq!(patterns[7], "br");
        assert_eq!(designs[0], "brwrr");
        assert_eq!(designs[7], "bbrgwb");
    }

    #[test]
    fn test_can_make_design() {
        let (patterns, _designs) = parse_input(EXAMPLE);
        
        // Test known possible designs
        assert!(can_make_design(&patterns, "brwrr"));
        assert!(can_make_design(&patterns, "bggr"));
        assert!(can_make_design(&patterns, "gbbr"));
        assert!(can_make_design(&patterns, "rrbgbr"));
        assert!(can_make_design(&patterns, "bwurrg"));
        assert!(can_make_design(&patterns, "brgr"));
        
        // Test known impossible designs
        assert!(!can_make_design(&patterns, "ubwu"));
        assert!(!can_make_design(&patterns, "bbrgwb"));
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 6);
    }

    #[test]
    fn test_count_ways() {
        let (patterns, _) = parse_input(EXAMPLE);
        
        // brwrr can be made in 2 ways: b,r,wr,r or br,wr,r
        assert_eq!(count_ways_to_make(&patterns, "brwrr"), 2);
        
        // bggr can only be made with b,g,g,r
        assert_eq!(count_ways_to_make(&patterns, "bggr"), 1);
        
        // gbbr can be made in 4 ways
        assert_eq!(count_ways_to_make(&patterns, "gbbr"), 4);
        
        // rrbgbr can be made in 6 ways
        assert_eq!(count_ways_to_make(&patterns, "rrbgbr"), 6);
        
        // bwurrg can only be made with bwu,r,r,g
        assert_eq!(count_ways_to_make(&patterns, "bwurrg"), 1);
        
        // brgr can be made in 2 ways: b,r,g,r or br,g,r
        assert_eq!(count_ways_to_make(&patterns, "brgr"), 2);
        
        // impossible designs have 0 ways
        assert_eq!(count_ways_to_make(&patterns, "ubwu"), 0);
        assert_eq!(count_ways_to_make(&patterns, "bbrgwb"), 0);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 16);
    }
}
