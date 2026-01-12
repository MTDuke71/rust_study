use std::collections::HashMap;

type Memo = HashMap<(usize, usize, usize), usize>;

/// Parse a single line into springs pattern and damaged group sizes
fn parse_line(line: &str) -> (&str, Vec<usize>) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let springs = parts[0];
    let groups: Vec<usize> = parts[1].split(',').map(|n| n.parse().unwrap()).collect();
    (springs, groups)
}

/// Count valid arrangements using dynamic programming with memoization
/// 
/// **DEBUG VERSION** with detailed tracing of recursion flow
/// 
/// Parameters:
/// - springs: byte slice of the spring pattern
/// - groups: slice of damaged group sizes
/// - pos: current position in springs
/// - group_idx: current group we're trying to place
/// - current_run: length of current contiguous '#' run
/// - memo: memoization cache
fn count_arrangements(
    springs: &[u8],
    groups: &[usize],
    pos: usize,
    group_idx: usize,
    current_run: usize,
    memo: &mut Memo,
) -> usize {
    let indent = "  ".repeat(pos);
    let _springs_str = String::from_utf8_lossy(springs);
    println!("{}→ count_arrangements(pos={}, group_idx={}, run={}) | springs[{}]='{}'", 
             indent, pos, group_idx, current_run, pos, 
             if pos < springs.len() { springs[pos] as char } else { '✓' });
    
    // Base case: reached end of springs
    if pos == springs.len() {
        // Valid if: all groups placed AND no incomplete run
        // OR: last group being completed right now
        if group_idx == groups.len() && current_run == 0 {
            println!("{}  ✓ BASE CASE: All groups placed, no incomplete run → VALID (1)", indent);
            return 1; // All groups placed, no incomplete run
        }
        if group_idx == groups.len() - 1 && current_run == groups[group_idx] {
            println!("{}  ✓ BASE CASE: Last group completed exactly → VALID (1)", indent);
            return 1; // Last group completed exactly
        }
        println!("{}  ✗ BASE CASE: Invalid (group_idx={}, groups.len()={}, run={}) → INVALID (0)", 
                 indent, group_idx, groups.len(), current_run);
        return 0; // Invalid arrangement
    }

    // Check memo cache
    let key = (pos, group_idx, current_run);
    if let Some(&cached) = memo.get(&key) {
        println!("{}  💾 CACHE HIT! → {}", indent, cached);
        return cached;
    }

    let mut count = 0;
    let ch = springs[pos];

    // Try placing '.' (operational spring)
    if ch == b'.' || ch == b'?' {
        if current_run == 0 {
            // Not in a run, just continue
            println!("{}  Try '.': Not in run, continue", indent);
            count += count_arrangements(springs, groups, pos + 1, group_idx, 0, memo);
        } else if group_idx < groups.len() && current_run == groups[group_idx] {
            // Complete current group and move to next
            println!("{}  Try '.': Complete group {} (size={}), move to next", indent, group_idx, groups[group_idx]);
            count += count_arrangements(springs, groups, pos + 1, group_idx + 1, 0, memo);
        } else {
            println!("{}  Try '.': SKIP (incomplete group: run={}, need={})", 
                     indent, current_run, if group_idx < groups.len() { groups[group_idx] } else { 0 });
        }
        // Else: incomplete group, invalid path (count += 0)
    }

    // Try placing '#' (damaged spring)
    if (ch == b'#' || ch == b'?') && group_idx < groups.len() && current_run < groups[group_idx] {
        // Continue or start current group
        println!("{}  Try '#': Extend group {} (run {} → {})", indent, group_idx, current_run, current_run + 1);
        count += count_arrangements(springs, groups, pos + 1, group_idx, current_run + 1, memo);
    } else if ch == b'#' || ch == b'?' {
        println!("{}  Try '#': SKIP (no room: group_idx={}, run={}, limit={})", 
                 indent, group_idx, current_run, if group_idx < groups.len() { groups[group_idx] } else { 0 });
    }
    // Else: run too long or no more groups, invalid path (count += 0)

    memo.insert(key, count);
    println!("{}  ← Returning {} (cached)", indent, count);
    count
}

/// Solve Part 1: Count arrangements for each row and sum
/// 
/// **DEBUG VERSION** - Only processes first line with detailed output
pub fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .take(1)  // DEBUG: Only process first line
        .map(|line| {
            println!("\n═══════════════════════════════════════════");
            println!("Processing: {}", line);
            let (springs, groups) = parse_line(line);
            println!("Springs: {}", springs);
            println!("Groups: {:?}", groups);
            println!("═══════════════════════════════════════════\n");
            let mut memo = HashMap::new();
            let result = count_arrangements(springs.as_bytes(), &groups, 0, 0, 0, &mut memo);
            println!("\n═══════════════════════════════════════════");
            println!("FINAL RESULT: {} arrangements", result);
            println!("Memo size: {} states cached", memo.len());
            println!("═══════════════════════════════════════════\n");
            result
        })
        .sum()
}

/// Unfold the springs pattern and groups for Part 2
/// 
/// Example: `???.### 1,1,3` becomes `???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3`
fn unfold(springs: &str, groups: &[usize]) -> (String, Vec<usize>) {
    let unfolded_springs = [springs; 5].join("?");
    let unfolded_groups = groups.repeat(5);
    (unfolded_springs, unfolded_groups)
}

/// Solve Part 2: Unfold each row 5x and count arrangements
pub fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (springs, groups) = parse_line(line);
            let (unfolded_springs, unfolded_groups) = unfold(springs, &groups);
            let mut memo = HashMap::new();
            count_arrangements(unfolded_springs.as_bytes(), &unfolded_groups, 0, 0, 0, &mut memo)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"#;

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 21);
    }

    #[test]
    fn test_individual_lines() {
        assert_eq!(solve_part1("???.### 1,1,3"), 1);
        assert_eq!(solve_part1(".??..??...?##. 1,1,3"), 4);
        assert_eq!(solve_part1("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(solve_part1("????.#...#... 4,1,1"), 1);
        assert_eq!(solve_part1("????.######..#####. 1,6,5"), 4);
        assert_eq!(solve_part1("?###???????? 3,2,1"), 10);
    }

    #[test]
    fn test_unfold() {
        let (springs, groups) = unfold(".#", &[1]);
        assert_eq!(springs, ".#?.#?.#?.#?.#");
        assert_eq!(groups, vec![1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_part2_first_line() {
        // First example line should give 1 arrangement when unfolded
        assert_eq!(solve_part2("???.### 1,1,3"), 1);
    }
}
