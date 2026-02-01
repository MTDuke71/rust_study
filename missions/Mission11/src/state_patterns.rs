//! REQ-5: State Space Design Patterns for Dynamic Programming
//!
//! This module demonstrates common state representations for DP problems with
//! Rust-specific considerations: lifetimes, zero-copy, Hash/Eq requirements.
//!
//! # State Pattern Catalog
//!
//! 1. **String Suffixes** - `&'a str` borrowed keys for pattern matching
//! 2. **Index Positions** - `usize` for array/sequence problems
//! 3. **Coordinate Pairs** - `(usize, usize)` for grid problems
//! 4. **Composite State** - Multi-dimensional tuples for complex state
//! 5. **Custom Structs** - `#[derive(Hash, Eq)]` for problem-specific state
//!
//! # Design Principles
//!
//! - **Zero-copy when possible**: Prefer `&'a str` over `String`
//! - **Minimal cloning**: Use references in HashMap keys
//! - **Hash + Eq + PartialEq**: Required for HashMap keys
//! - **Lifetime propagation**: `<'a>` parameters for borrowed data
//!
//! # Performance Characteristics
//!
//! | Pattern | Key Type | Hash Cost | Clone Cost | Use Case |
//! |---------|----------|-----------|------------|----------|
//! | String Suffix | `&'a str` | O(L) | O(1) ref | Pattern matching |
//! | Index Position | `usize` | O(1) | O(1) copy | Sequences |
//! | Coordinate | `(usize, usize)` | O(1) | O(1) copy | Grids |
//! | Composite | `(T1, T2, T3)` | O(k) | O(k) | Multi-dim |
//! | Custom Struct | `MyState` | O(fields) | Varies | Complex |

use std::collections::HashMap;

// ============================================================================
// Pattern 1: String Suffixes with Lifetime Parameters
// ============================================================================

/// **Pattern 1: String Suffixes**
///
/// State: `remaining: &'a str` - Zero-copy borrowed string slices
///
/// # Use Cases
/// - Pattern matching (AoC 2024 Day 19)
/// - String decomposition problems
/// - Parsing with backtracking
///
/// # Rust Considerations
/// - Requires lifetime parameter `<'a>` on function
/// - HashMap key: `&'a str` not `String` (zero allocation)
/// - Use `strip_prefix()` for zero-copy transitions
///
/// # Example
/// ```rust
/// use mission11::state_patterns::string_suffix_pattern;
/// use std::collections::HashMap;
///
/// let patterns = vec!["ab", "abc"];
/// let mut memo = HashMap::new();
/// assert!(string_suffix_pattern("abcabc", &patterns, &mut memo));
/// ```
pub fn string_suffix_pattern<'a>(
    remaining: &'a str,
    patterns: &[&str],
    memo: &mut HashMap<&'a str, bool>,
) -> bool {
    // Base case: empty string is always constructible
    if remaining.is_empty() {
        return true;
    }

    // Memoization check
    if let Some(&cached) = memo.get(remaining) {
        return cached;
    }

    // Try all patterns - zero-copy with strip_prefix
    for &pattern in patterns {
        if let Some(suffix) = remaining.strip_prefix(pattern) {
            if string_suffix_pattern(suffix, patterns, memo) {
                memo.insert(remaining, true);
                return true;
            }
        }
    }

    memo.insert(remaining, false);
    false
}

// ============================================================================
// Pattern 2: Index Positions
// ============================================================================

/// **Pattern 2: Index Positions**
///
/// State: `pos: usize` - Array/sequence position
///
/// # Use Cases
/// - Fibonacci sequence
/// - Longest Increasing Subsequence
/// - Array partitioning problems
///
/// # Rust Considerations
/// - `usize` implements `Hash + Eq + PartialEq + Copy`
/// - No lifetime parameters needed
/// - Cache keys are cheap to copy (8 bytes)
///
/// # Example
/// ```rust
/// use mission11::state_patterns::index_position_pattern;
/// use std::collections::HashMap;
///
/// let arr = vec![1, 2, 3, 4, 5];
/// let mut memo = HashMap::new();
/// let result = index_position_pattern(0, &arr, &mut memo);
/// assert_eq!(result, 15); // Sum of array
/// ```
pub fn index_position_pattern(pos: usize, arr: &[i32], memo: &mut HashMap<usize, i32>) -> i32 {
    // Base case: reached end
    if pos >= arr.len() {
        return 0;
    }

    // Memoization check
    if let Some(&cached) = memo.get(&pos) {
        return cached;
    }

    // Recursive computation: sum from current position
    let result = arr[pos] + index_position_pattern(pos + 1, arr, memo);

    memo.insert(pos, result);
    result
}

// ============================================================================
// Pattern 3: Coordinate Pairs (Grid Problems)
// ============================================================================

/// **Pattern 3: Coordinate Pairs**
///
/// State: `(x, y): (usize, usize)` - 2D grid position
///
/// # Connection to Pathfinding Algorithms
///
/// **This is a simplified Dijkstra-style state-space search!**
///
/// ## Core Similarities with Dijkstra (e.g., AoC 2023 Day 17):
/// - **State space**: Coordinates defining current position
/// - **Goal checking**: Base case when reaching target
/// - **State exploration**: Try valid moves (right/down here, all directions in Dijkstra)
/// - **Visited tracking**: Memoization cache (HashMap here, HashSet + BinaryHeap in Dijkstra)
///
/// ## Simplifications (Why This Pattern is Easier to Learn):
/// - ❌ **No priority queue** - Counting paths, not finding optimal cost
/// - ❌ **No distance tracking** - All paths have equal "weight"
/// - ❌ **Simpler state** - Just `(x, y)`, no direction/consecutive move constraints
/// - ✅ **Pure memoization** - HashMap instead of Heap + Set
/// - ✅ **Counting problem** - "How many ways?" vs "What's the shortest path?"
///
/// ## Progression to Full Dijkstra:
/// 1. **This pattern**: Count paths with memoization
/// 2. **Add costs**: Track minimum distance to each state
/// 3. **Add priority queue**: Process lowest-cost states first (BinaryHeap)
/// 4. **Add constraints**: Direction limits, consecutive moves (Day 17 crucible)
///
/// # Use Cases
/// - Grid pathfinding (unweighted)
/// - 2D dynamic programming (edit distance, path counting)
/// - Image processing problems
///
/// # Rust Considerations
/// - Tuples implement `Hash + Eq + PartialEq + Copy`
/// - Order matters: `(x, y) != (y, x)`
/// - Can extend to 3D: `(x, y, z)`
///
/// # Example
/// ```rust
/// use mission11::state_patterns::coordinate_pair_pattern;
/// use std::collections::HashMap;
///
/// let grid = vec![vec![1, 2], vec![3, 4]];
/// let mut memo = HashMap::new();
/// let paths = coordinate_pair_pattern((0, 0), (1, 1), &grid, &mut memo);
/// assert_eq!(paths, 2); // Right-Down or Down-Right
/// ```
pub fn coordinate_pair_pattern(
    current: (usize, usize),
    target: (usize, usize),
    grid: &[Vec<i32>],
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    // Base case: reached target
    if current == target {
        return 1;
    }

    // Out of bounds check
    let (x, y) = current;
    if y >= grid.len() || x >= grid[0].len() {
        return 0;
    }

    // Memoization check
    if let Some(&cached) = memo.get(&current) {
        return cached;
    }

    // Try moving right or down
    let mut paths = 0;

    // Right
    if x < target.0 {
        paths += coordinate_pair_pattern((x + 1, y), target, grid, memo);
    }

    // Down
    if y < target.1 {
        paths += coordinate_pair_pattern((x, y + 1), target, grid, memo);
    }

    memo.insert(current, paths);
    paths
}

// ============================================================================
// Pattern 4: Composite State (Multi-Dimensional)
// ============================================================================

/// **Pattern 4: Composite State**
///
/// State: `(value, count, depth): (u64, usize, usize)` - Multi-dimensional state
///
/// # Use Cases
/// - Knapsack variants (value, weight, items)
/// - Game states (score, turn, resources)
/// - AoC 2024 Day 11 (stone value, blink count)
///
/// # Rust Considerations
/// - Tuples with 2-6 elements implement `Hash + Eq`
/// - Each component must be hashable
/// - Order matters in tuple comparison
///
/// # Example
/// ```rust
/// use mission11::state_patterns::composite_state_pattern;
/// use std::collections::HashMap;
///
/// let mut memo = HashMap::new();
/// let stones = composite_state_pattern((125, 6), &mut memo);
/// assert!(stones > 0);
/// ```
pub fn composite_state_pattern(state: (u64, usize), memo: &mut HashMap<(u64, usize), u64>) -> u64 {
    let (stone_value, blinks_remaining) = state;

    // Base case: no more blinks
    if blinks_remaining == 0 {
        return 1; // One stone remains
    }

    // Memoization check
    if let Some(&cached) = memo.get(&state) {
        return cached;
    }

    // Recursive computation based on stone transformation rules
    let result = if stone_value == 0 {
        // Rule 1: 0 → 1
        composite_state_pattern((1, blinks_remaining - 1), memo)
    } else {
        let digits = stone_value.to_string();
        if digits.len() % 2 == 0 {
            // Rule 2: Even digits → split
            let mid = digits.len() / 2;
            let left: u64 = digits[..mid].parse().unwrap();
            let right: u64 = digits[mid..].parse().unwrap();

            composite_state_pattern((left, blinks_remaining - 1), memo)
                + composite_state_pattern((right, blinks_remaining - 1), memo)
        } else {
            // Rule 3: Odd digits → multiply by 2024
            composite_state_pattern((stone_value * 2024, blinks_remaining - 1), memo)
        }
    };

    memo.insert(state, result);
    result
}

// ============================================================================
// Pattern 5: Custom Structs with Derive Macros
// ============================================================================

/// **Pattern 5: Custom Structs**
///
/// State: Problem-specific struct with `#[derive(Hash, Eq, PartialEq)]`
///
/// # Use Cases
/// - Complex state with many fields
/// - States with semantic meaning
/// - When tuple readability suffers
///
/// # Rust Considerations
/// - **Must derive**: `Hash`, `Eq`, `PartialEq`
/// - Optional: `Clone`, `Debug`, `Copy`
/// - All fields must be hashable
/// - Use `#[derive(Clone)]` only if needed (prefer references)
///
/// # Example
/// ```rust
/// use mission11::state_patterns::{GameState, custom_struct_pattern};
/// use std::collections::HashMap;
///
/// let mut memo = HashMap::new();
/// // Stronger player state that can win
/// let initial = GameState { player_hp: 100, boss_hp: 50, turn: 0, mana: 500 };
/// let can_win = custom_struct_pattern(initial, &mut memo);
/// assert!(can_win); // Player strong enough to win
/// ```
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct GameState {
    pub player_hp: i32,
    pub boss_hp: i32,
    pub turn: usize,
    pub mana: i32,
}

pub fn custom_struct_pattern(state: GameState, memo: &mut HashMap<GameState, bool>) -> bool {
    // Base case: player wins
    if state.boss_hp <= 0 {
        return true;
    }

    // Base case: player loses
    if state.player_hp <= 0 || state.mana < 0 {
        return false;
    }

    // Base case: turn limit
    if state.turn >= 20 {
        return false;
    }

    // Memoization check
    if let Some(&cached) = memo.get(&state) {
        return cached;
    }

    // Try all possible actions (simplified game logic)
    let damage_per_turn = 10;
    let mana_cost = 50;

    let result = if state.turn.is_multiple_of(2) {
        // Player's turn: attack
        let next = GameState {
            player_hp: state.player_hp,
            boss_hp: state.boss_hp - damage_per_turn,
            turn: state.turn + 1,
            mana: state.mana - mana_cost,
        };
        custom_struct_pattern(next, memo)
    } else {
        // Boss's turn: boss attacks
        let next = GameState {
            player_hp: state.player_hp - 5,
            boss_hp: state.boss_hp,
            turn: state.turn + 1,
            mana: state.mana,
        };
        custom_struct_pattern(next, memo)
    };

    memo.insert(state, result);
    result
}

// ============================================================================
// Advanced Pattern: Composite with Borrowed Data
// ============================================================================

/// **Advanced Pattern: Composite with Lifetimes**
///
/// State: `(&'a str, usize)` - String reference + index
///
/// # Use Cases
/// - Multi-source pattern matching (AoC 2023 Day 12)
/// - String + position composite state
/// - When both borrowed and owned state needed
///
/// # Rust Considerations
/// - Requires lifetime `<'a>` on all functions in call chain
/// - HashMap key: `(&'a str, usize)`
/// - Both components must be hashable
///
/// # Example
/// ```rust
/// use mission11::state_patterns::composite_with_borrowed;
/// use std::collections::HashMap;
///
/// let pattern_groups = vec![vec!["a", "b"], vec!["c", "d"]];
/// let mut memo = HashMap::new();
/// // Need both groups: first for 'a' or 'b', second for 'c' or 'd'
/// let can_build_ac = composite_with_borrowed("ac", 0, &pattern_groups, &mut memo);
/// assert!(can_build_ac); // "a" from group 0, "c" from group 1
/// ```
pub fn composite_with_borrowed<'a>(
    remaining: &'a str,
    group_idx: usize,
    pattern_groups: &[Vec<&str>],
    memo: &mut HashMap<(&'a str, usize), bool>,
) -> bool {
    // Base case: consumed all string
    if remaining.is_empty() {
        return group_idx >= pattern_groups.len();
    }

    // Base case: no more pattern groups
    if group_idx >= pattern_groups.len() {
        return remaining.is_empty();
    }

    // Memoization check - composite key
    if let Some(&cached) = memo.get(&(remaining, group_idx)) {
        return cached;
    }

    // Try all patterns from current group
    let current_patterns = &pattern_groups[group_idx];
    for &pattern in current_patterns {
        if let Some(suffix) = remaining.strip_prefix(pattern) {
            if composite_with_borrowed(suffix, group_idx + 1, pattern_groups, memo) {
                memo.insert((remaining, group_idx), true);
                return true;
            }
        }
    }

    memo.insert((remaining, group_idx), false);
    false
}

// ============================================================================
// Pattern Comparison and Guidelines
// ============================================================================

/// # State Pattern Selection Guide
///
/// | Requirement | Recommended Pattern | Rationale |
/// |-------------|-------------------|-----------|
/// | String matching | String Suffix (`&'a str`) | Zero-copy, minimal memory |
/// | Sequence position | Index Position (`usize`) | Simple, fast hash |
/// | Grid navigation | Coordinate Pair (`(x, y)`) | Natural 2D representation |
/// | 2-3 state vars | Composite Tuple | Quick implementation |
/// | 4+ state vars | Custom Struct | Readability, semantics |
/// | Mixed borrowed/owned | Composite with Lifetime | Flexibility |
///
/// # Performance Guidelines
///
/// 1. **Prefer Copy types**: `usize`, tuples of primitives (no allocation)
/// 2. **Use borrowed keys**: `&'a str` over `String` when possible
/// 3. **Derive only needed traits**: `Hash + Eq + PartialEq` are required, `Clone` adds overhead
/// 4. **Benchmark composite vs struct**: Tuples hash faster but structs are clearer
///
/// # Common Pitfalls
///
/// - ❌ Using `String` as key when `&str` suffices
/// - ❌ Deriving `Clone` unnecessarily (cache keys don't need it)
/// - ❌ Forgetting `Hash` or `Eq` on custom structs
/// - ❌ Mixing owned and borrowed without lifetimes
/// - ❌ Using large structs as keys (hash cost scales with size)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_suffix_basic() {
        let patterns = vec!["a", "b", "ab"];
        let mut memo = HashMap::new();
        assert!(string_suffix_pattern("ab", &patterns, &mut memo));
        assert!(string_suffix_pattern("abab", &patterns, &mut memo));
        assert!(!string_suffix_pattern("c", &patterns, &mut memo));
    }

    #[test]
    fn test_index_position_sum() {
        let arr = vec![1, 2, 3, 4];
        let mut memo = HashMap::new();
        assert_eq!(index_position_pattern(0, &arr, &mut memo), 10);
        assert_eq!(index_position_pattern(2, &arr, &mut memo), 7);
    }

    #[test]
    fn test_coordinate_pair_paths() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let mut memo = HashMap::new();
        let paths = coordinate_pair_pattern((0, 0), (2, 1), &grid, &mut memo);
        assert_eq!(paths, 3); // 3 paths in 3x2 grid
    }

    #[test]
    fn test_composite_state_stones() {
        let mut memo = HashMap::new();
        // Stone 0 → 1 → 2024 → ...
        let count = composite_state_pattern((0, 3), &mut memo);
        assert!(count > 0);
    }

    #[test]
    fn test_custom_struct_game() {
        let mut memo = HashMap::new();
        let state = GameState {
            player_hp: 100,
            boss_hp: 50,
            turn: 0,
            mana: 500,
        };
        assert!(custom_struct_pattern(state, &mut memo));
    }

    #[test]
    fn test_composite_with_borrowed() {
        let groups = vec![vec!["a", "ab"], vec!["c"]];
        let mut memo = HashMap::new();
        assert!(composite_with_borrowed("abc", 0, &groups, &mut memo));
        assert!(!composite_with_borrowed("abd", 0, &groups, &mut memo));
    }

    #[test]
    fn test_lifetime_propagation() {
        // Verify that borrowed keys work with lifetime propagation
        let input = String::from("test");
        let patterns = vec!["te", "st"];
        let mut memo = HashMap::new();

        assert!(string_suffix_pattern(&input, &patterns, &mut memo));

        // Cache should contain borrowed slices
        assert!(memo.contains_key("test") || memo.contains_key("st"));
    }
}
