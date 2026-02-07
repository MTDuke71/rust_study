//! Day 7: No Space Left On Device
//!
//! **Problem**: Parse terminal output (cd/ls) to reconstruct a filesystem,
//! then compute directory sizes.
//!
//! **Approach**: Stack-based size accumulation — no tree or HashMap needed.
//!
//! **Key Insight**: We only need directory *sizes*, not names or structure.
//! A stack of running totals mirrors the directory depth. When we `cd ..`,
//! the popped size rolls up into the parent and gets collected into a flat
//! list. One pass, zero allocations beyond the stack and results Vec.

// ============================================================================
// Core Algorithm
// ============================================================================

/// Parse terminal output and collect the total size of every directory.
///
/// Strategy: Maintain a stack of cumulative sizes — one entry per directory
/// depth level. When we encounter a file, add its size to the current
/// (top-of-stack) directory. When we `cd ..`, pop the current directory's
/// total, add it to the parent (new top), and record it in the results.
///
/// Returns a Vec of all directory total sizes (unordered).
fn compute_dir_sizes(input: &str) -> Vec<u64> {
    let mut stack: Vec<u64> = Vec::new(); // running size at each depth
    let mut all_sizes: Vec<u64> = Vec::new(); // completed directory sizes

    for line in input.lines() {
        if line == "$ cd /" {
            // Reset to root — in practice only appears once at the start
            stack.clear();
            stack.push(0);
        } else if line == "$ cd .." {
            // Pop current directory: record its size, roll up to parent
            let completed = stack.pop().unwrap();
            all_sizes.push(completed);
            *stack.last_mut().unwrap() += completed;
        } else if line.starts_with("$ cd ") {
            // Enter subdirectory: push a fresh accumulator
            stack.push(0);
        } else if line == "$ ls" || line.starts_with("dir ") {
            // Ignore: ls command and directory listings don't carry size info
        } else {
            // File entry: "<size> <name>" — add size to current directory
            if let Some(size_str) = line.split_ascii_whitespace().next() {
                if let Ok(size) = size_str.parse::<u64>() {
                    *stack.last_mut().unwrap() += size;
                }
            }
        }
    }

    // Unwind remaining stack (directories never explicitly cd'd out of)
    while stack.len() > 1 {
        let completed = stack.pop().unwrap();
        all_sizes.push(completed);
        *stack.last_mut().unwrap() += completed;
    }
    // Root directory
    if let Some(&root) = stack.last() {
        all_sizes.push(root);
    }

    all_sizes
}

// ============================================================================
// Part 1 Logic
// ============================================================================

/// Find all directories with total size ≤ 100,000 and sum their sizes.
fn solve_part1_impl(input: &str) -> u64 {
    let sizes = compute_dir_sizes(input);
    sizes.iter().filter(|&&size| size <= 100_000).sum()
}

// ============================================================================
// Part 2 Logic
// ============================================================================

/// Find the smallest directory to delete that frees enough space.
///
/// Total disk: 70,000,000. Update needs: 30,000,000 free.
/// Need to free: used - (70,000,000 - 30,000,000) = used - 40,000,000
fn solve_part2_impl(input: &str) -> u64 {
    let sizes = compute_dir_sizes(input);
    // Root is always the last entry pushed
    let total_used = *sizes.last().unwrap();
    let need_to_free = total_used.saturating_sub(40_000_000);

    sizes
        .iter()
        .filter(|&&size| size >= need_to_free)
        .copied()
        .min()
        .expect("At least root directory should qualify")
}

// ============================================================================
// Public Interface
// ============================================================================

pub fn solve(input: &str) -> (u64, u64) {
    (solve_part1_impl(input), solve_part2_impl(input))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_dir_sizes() {
        let sizes = compute_dir_sizes(EXAMPLE);
        // Stack-based: sizes collected in cd-out order, root last
        // e (584), a (94853), d (24933642), / (48381165)
        assert!(sizes.contains(&584), "dir e");
        assert!(sizes.contains(&94_853), "dir a");
        assert!(sizes.contains(&24_933_642), "dir d");
        assert!(sizes.contains(&48_381_165), "root");
        assert_eq!(sizes.len(), 4, "4 directories total");
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1_impl(EXAMPLE), 95_437);
    }

    #[test]
    fn test_part2_example() {
        // Total used: 48,381,165. Need to free: 48,381,165 - 40,000,000 = 8,381,165
        // Smallest dir >= 8,381,165 is d (24,933,642)
        assert_eq!(solve_part2_impl(EXAMPLE), 24_933_642);
    }
}
