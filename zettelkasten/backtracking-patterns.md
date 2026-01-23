# Backtracking Patterns in Rust

**Type**: Algorithm Pattern  
**Domain**: Search Algorithms, Recursion  
**Created**: 2026-01-23  
**Context**: AoC 2023 Day 23 - DFS with visited set for longest path

---

## Core Concept

**Backtracking** is a systematic way to explore all possible solutions by:
1. **Making a choice** (explore one possibility)
2. **Recursing** (solve smaller subproblem)
3. **Undoing the choice** (restore state for other possibilities)

Also known as "exhaustive search" or "brute force with pruning."

---

## The Pattern

### Generic Template

```rust
fn backtrack(state: &mut State, choices: Vec<Choice>) -> Solution {
    // Base case: valid solution found
    if is_solution(state) {
        return extract_solution(state);
    }
    
    // Base case: invalid state
    if is_invalid(state) {
        return None;
    }
    
    let mut best = None;
    
    for choice in choices {
        // Skip invalid choices
        if !is_valid_choice(state, choice) {
            continue;
        }
        
        // 1. MAKE CHOICE
        apply_choice(state, choice);
        
        // 2. RECURSE
        let result = backtrack(state, get_next_choices(state));
        
        // Update best solution
        best = better_of(best, result);
        
        // 3. UNDO CHOICE (backtrack!)
        undo_choice(state, choice);
    }
    
    best
}
```

### Key Principles

1. **State mutation**: Modify state in-place for efficiency
2. **Undo operations**: MUST restore state after recursion
3. **Pruning**: Skip invalid choices early
4. **Complete search**: Eventually tries all valid combinations

---

## DFS with Visited Set Pattern

**Use case**: Graph/tree traversal where nodes can't be revisited.

### Implementation

```rust
fn dfs(&self, current: Node, visited: &mut HashSet<Node>) -> usize {
    // Base case: reached goal
    if current == self.goal {
        return 0;
    }
    
    let mut best_length = 0;
    
    // Try all neighbors
    for neighbor in self.neighbors(current) {
        // Pruning: skip visited nodes
        if visited.contains(&neighbor) {
            continue;
        }
        
        // 1. MAKE CHOICE
        visited.insert(neighbor);
        
        // 2. RECURSE
        let length = self.dfs(neighbor, visited);
        
        // Update best
        if length > 0 || neighbor == self.goal {
            best_length = best_length.max(length + 1);
        }
        
        // 3. UNDO CHOICE (CRITICAL!)
        visited.remove(&neighbor);
    }
    
    best_length
}

// Initial call
let mut visited = HashSet::new();
visited.insert(start);
let result = dfs(start, &mut visited);
```

**Why HashSet?**
- O(1) insert/remove/contains
- Efficient for sparse visits (not all nodes visited)
- No need for ordered traversal

**Common Bug**: Forgetting `visited.remove(&neighbor)` causes wrong results!

---

## AoC 2023 Day 23 Case Study

### Problem: Longest Path in Hiking Trail

Find longest simple path from start to goal through grid.

### Implementation

```rust
impl HikingMap {
    fn dfs(&self, current: Coord, visited: &mut HashSet<Coord>, 
           ignore_slopes: bool) -> usize {
        // Base case: reached goal
        if current == self.goal {
            return 0;
        }
        
        let mut max_length = 0;
        
        // Try all 4 cardinal directions
        for (next, direction) in current.neighbors(self.rows, self.cols) {
            // Pruning conditions:
            // 1. Already visited (prevent cycles)
            // 2. Forest tile (impassable)
            // 3. Slope constraint violated (Part 1 only)
            if visited.contains(&next) || 
               !self.can_move(current, next, direction, ignore_slopes) {
                continue;
            }
            
            // Make choice
            visited.insert(next);
            
            // Recurse
            let length = self.dfs(next, visited, ignore_slopes);
            
            // Update maximum
            if length > 0 || next == self.goal {
                max_length = max_length.max(length + 1);
            }
            
            // Backtrack (restore state for other paths)
            visited.remove(&next);
        }
        
        max_length
    }
}

// Usage
pub fn solve_part1(input: &str) -> Result<String> {
    let map = HikingMap::parse(input)?;
    let mut visited = HashSet::new();
    visited.insert(map.start);
    let longest = map.dfs(map.start, &mut visited, false);
    Ok(longest.to_string())
}
```

### Performance Characteristics

**Part 1** (with slope constraints):
- Slopes create directed edges → natural pruning
- Branching factor: ~2-3 (restricted moves)
- Max depth: ~2,200
- Result: 22.9ms ✓

**Part 2** (without slope constraints):
- Bidirectional movement → exponential explosion
- Branching factor: ~3-4
- Max depth: ~7,000
- Result: Stack overflow ✗

**Solution**: Graph contraction reduces depth to 35 → 2.38s ✓

---

## Common Backtracking Problems

### 1. N-Queens

Place N queens on N×N chessboard so none attack each other.

```rust
fn solve_n_queens(row: usize, board: &mut Vec<usize>, 
                  cols: &mut HashSet<usize>,
                  diag1: &mut HashSet<isize>,
                  diag2: &mut HashSet<usize>) -> Vec<Vec<usize>> {
    if row == board.len() {
        return vec![board.clone()];  // Found solution
    }
    
    let mut solutions = Vec::new();
    
    for col in 0..board.len() {
        let d1 = row as isize - col as isize;
        let d2 = row + col;
        
        if cols.contains(&col) || diag1.contains(&d1) || diag2.contains(&d2) {
            continue;  // Queen would be attacked
        }
        
        // Make choice
        board[row] = col;
        cols.insert(col);
        diag1.insert(d1);
        diag2.insert(d2);
        
        // Recurse
        solutions.extend(solve_n_queens(row + 1, board, cols, diag1, diag2));
        
        // Backtrack
        cols.remove(&col);
        diag1.remove(&d1);
        diag2.remove(&d2);
    }
    
    solutions
}
```

### 2. Sudoku Solver

```rust
fn solve_sudoku(board: &mut [[u8; 9]; 9], row: usize, col: usize) -> bool {
    if row == 9 {
        return true;  // Solved!
    }
    
    let (next_row, next_col) = if col == 8 { (row + 1, 0) } else { (row, col + 1) };
    
    if board[row][col] != 0 {
        return solve_sudoku(board, next_row, next_col);  // Pre-filled
    }
    
    for num in 1..=9 {
        if is_valid_placement(board, row, col, num) {
            // Make choice
            board[row][col] = num;
            
            // Recurse
            if solve_sudoku(board, next_row, next_col) {
                return true;  // Solution found!
            }
            
            // Backtrack
            board[row][col] = 0;
        }
    }
    
    false  // No valid number works
}
```

### 3. Subset Sum

Find subset of numbers that sum to target.

```rust
fn subset_sum(nums: &[i32], target: i32, current_sum: i32, 
              index: usize, subset: &mut Vec<i32>) -> bool {
    if current_sum == target {
        return true;  // Found!
    }
    if index == nums.len() || current_sum > target {
        return false;  // No solution
    }
    
    // Choice 1: Include nums[index]
    subset.push(nums[index]);
    if subset_sum(nums, target, current_sum + nums[index], index + 1, subset) {
        return true;
    }
    subset.pop();  // Backtrack
    
    // Choice 2: Exclude nums[index]
    subset_sum(nums, target, current_sum, index + 1, subset)
}
```

---

## Optimization Techniques

### 1. Pruning

Skip branches that can't lead to solution:
```rust
if current_sum > target {
    return;  // Can't reach target (all nums positive)
}

if remaining_budget < min_cost_to_goal {
    return;  // Insufficient resources
}
```

### 2. Ordering Choices

Try most promising choices first:
```rust
// For subset sum: sort descending to reach target faster
let mut nums = nums.to_vec();
nums.sort_by(|a, b| b.cmp(a));
```

### 3. Memoization

Cache results for overlapping subproblems:
```rust
let mut memo: HashMap<State, Result> = HashMap::new();

fn backtrack_memo(state: State, memo: &mut HashMap<State, Result>) -> Result {
    if let Some(&result) = memo.get(&state) {
        return result;  // Already computed
    }
    
    let result = /* ... compute ... */;
    memo.insert(state, result);
    result
}
```

Note: Only works if subproblems truly independent!

### 4. Iterative Deepening

Limit recursion depth, gradually increase:
```rust
for max_depth in 1.. {
    if let Some(solution) = backtrack_limited(start, max_depth) {
        return solution;
    }
}
```

Prevents stack overflow while still exploring deeply.

---

## Rust-Specific Considerations

### Ownership and Borrowing

**Problem**: Can't pass `&mut visited` and `visited` simultaneously.

**Solution 1**: Clone visited set (expensive)
```rust
for choice in choices {
    let mut new_visited = visited.clone();
    new_visited.insert(choice);
    backtrack(choice, &new_visited);
}
```

**Solution 2**: Insert/remove pattern (efficient)
```rust
for choice in choices {
    visited.insert(choice);
    backtrack(choice, visited);
    visited.remove(&choice);  // Backtrack
}
```

### Stack Overflow Prevention

Rust default stack: ~2-8MB depending on platform.

**Symptoms**:
```
thread 'main' has overflowed its stack
```

**Solutions**:
1. **Graph contraction**: Reduce depth (Day 23 approach)
2. **Iterative with explicit stack**: Convert recursion to loop
3. **Increase stack size**: `RUST_MIN_STACK=16777216` (16MB)
4. **Spawn thread with larger stack**:
   ```rust
   std::thread::Builder::new()
       .stack_size(32 * 1024 * 1024)  // 32MB
       .spawn(|| { /* backtracking */ })
       .unwrap()
       .join()
       .unwrap()
   ```

---

## Related Concepts

- [[graph-theory-fundamentals]] - Graph traversal basics
- [[longest-path-np-hard]] - Why backtracking needed for longest path
- [[graph-contraction-optimization]] - Reducing backtracking depth
- [[memoization-comprehensive-guide]] - Caching for overlapping subproblems
- [[dfs-patterns]] - Depth-first search variations

---

## References

- **AoC 2023 Day 23**: [day23.rs](../advent_of_code/aoc2023/src/solver/day23.rs) - Real-world backtracking example
- **The Rust Book**: Ch 15.1 - Box<T> for recursive types
- **Algorithm Design Manual** (Skiena) - Chapter 7: Combinatorial Search

---

*Tags: #algorithms #backtracking #recursion #dfs #graph-traversal #rust-patterns #aoc2023*
