# AoC 2025 Day 10 - Solution Analysis

## Problem Overview

Day 10 presented two distinct optimization problems:

### Part 1: Binary Toggle Problem (Lights On/Off)
- **Problem Type**: System of linear equations over GF(2) (binary field)
- **Goal**: Find minimum button presses to reach target light configuration
- **Constraint**: Each button toggles specific lights (XOR operation)
- **Answer**: 385 button presses

### Part 2: Integer Counter Problem (Joltage Levels)
- **Problem Type**: Integer Linear Programming (ILP)
- **Goal**: Find minimum button presses to reach exact counter values
- **Constraint**: Each button press adds +1 to affected counters
- **Answer**: 16757 button presses

## Solution Approaches Attempted

### Part 1: Gaussian Elimination ✅ SUCCESS
**Approach**: Gaussian elimination over GF(2)
- Built augmented matrix [A|b] where A[i][j] = 1 if button j affects light i
- Row-reduced to RREF using XOR operations
- Identified pivot/free variables
- Enumerated all 2^f combinations for f free variables
- Selected solution with minimum button presses

**Result**: Fast and optimal - completed instantly

**Code**:
```rust
fn solve_machine(machine: &Machine) -> usize {
    // Gaussian elimination with XOR operations
    // Enumerate free variable combinations
    // Return minimum solution
}
```

---

### Part 2: Multiple Attempts

## ❌ Attempt 1: Backtracking DFS
**Rationale**: Try all combinations of button presses with pruning

**Implementation**:
```rust
fn backtrack(
    machine: &Machine,
    targets: &[i64],
    button_presses: &mut Vec<usize>,
    current_idx: usize,
    current_total: usize,
    best_so_far: &mut usize,
) -> bool {
    // For each button, try 0..max_target presses
    // Prune if total exceeds best
}
```

**Result**: ❌ **FAILED**
- **Time Complexity**: O(T^B) where T=max_target, B=num_buttons
- **Issue**: Exponential explosion - timed out on actual input
- Machine 3: targets [168, 164, 176, ...] → 194^9 ≈ 10^20 states

---

## ❌ Attempt 2: Greedy Heuristic
**Rationale**: Select button that affects most unfulfilled counters

**Implementation**:
```rust
// For each step, choose button that maximizes:
// - Number of counters below target that it affects
// - Weighted by how far below target they are
```

**Result**: ❌ **FAILED**
- **Issue**: Not optimal - returned 19 presses instead of 10 for test case
- Greedy choices led to suboptimal paths
- Missing global optimization

---

## ❌ Attempt 3: BFS (Breadth-First Search)
**Rationale**: Explore state space level-by-level (guaranteed optimal)

**Implementation**:
```rust
fn solve_machine_joltage(machine: &Machine, targets: &[i64]) -> usize {
    let mut queue = VecDeque::new();
    let mut visited: HashMap<Vec<i64>, usize> = HashMap::new();
    
    queue.push_back((vec![0; num_counters], 0));
    
    while let Some((counters, presses)) = queue.pop_front() {
        if counters == targets {
            return presses;
        }
        
        // Try each button
        for button_idx in 0..num_buttons {
            let mut next = counters.clone();
            // Apply button...
            // Prune if exceeds targets
            queue.push_back((next, presses + 1));
        }
    }
}
```

**Result**: ❌ **FAILED on large inputs**
- **Pros**: Guaranteed optimal solution
- **Test Cases**: ✅ All passed (10, 12, 11 → 33 total)
- **Actual Input**: ❌ Timed out after 10 seconds
- **Bottleneck**: Machine 3 with targets [168, 164, 176, 171, 51, 173, 194, 30, 168]
- **Issue**: State space O(∏ target_i) - visited HashMap grew to millions of entries

---

## ❌ Attempt 4: A* Search with Heuristic
**Rationale**: Use heuristic to guide search toward goal faster

**Heuristic**: 
```rust
fn heuristic(counters: &[i64], targets: &[i64], machine: &Machine) -> usize {
    let mut h = 0;
    for (i, &target) in targets.iter().enumerate() {
        let remaining = target - counters[i];
        if remaining > 0 {
            let affecting_count = buttons_affecting_counter(i).count().max(1);
            h += (remaining as usize + affecting_count - 1) / affecting_count;
        }
    }
    h
}
```

**Result**: ❌ **FAILED**
- **Issue**: Heuristic was inadmissible (not a true lower bound)
- Test case 3: Got 12 presses instead of optimal 11
- A* only guarantees optimality with admissible heuristics

---

## ❌ Attempt 5: Z3 SMT Solver (Rust bindings)
**Rationale**: Use industry-standard solver for constraint optimization

**Implementation Attempt**:
```toml
[dependencies]
z3 = "0.12"
```

```rust
use z3::ast::{Ast, Int};
use z3::{Config, Context, Optimize};

let button_vars: Vec<Int> = create_variables();
opt.assert(button_var >= 0);
opt.assert(sum_equals_target);
opt.minimize(total_presses);
```

**Result**: ❌ **COMPILATION FAILED**
- **Issue**: Z3 Rust crate requires `libclang` for binding generation
- Windows compilation error: "couldn't find libclang.dll"
- Tried static linking: requires CMake and Visual Studio Build Tools
- CMake error: "Visual studio version detected but this crate doesn't know how to generate cmake files for it"
- **Conclusion**: Native dependency hell on Windows

---

## ✅ Attempt 6: Integer Linear Programming with good_lp (SUCCESS!)
**Rationale**: Use pure-Rust ILP solver without native dependencies

**Mathematical Formulation**:
```
Minimize: Σ x_i (total button presses)

Subject to:
  For each counter j: Σ (a_ij * x_i) = t_j
  where:
    a_ij = 1 if button i affects counter j, 0 otherwise
    t_j = target value for counter j
    x_i ≥ 0 for all i (non-negative button presses)
    x_i ∈ ℤ (integer constraint)
```

**Implementation**:
```toml
[dependencies]
good_lp = { version = "1.8", features = ["minilp"], default-features = false }
```

```rust
use good_lp::*;

fn solve_machine_joltage(machine: &Machine, targets: &[i64]) -> usize {
    let mut vars = ProblemVariables::new();
    
    // Create integer variable for each button
    let mut button_presses = Vec::new();
    for _ in 0..num_buttons {
        button_presses.push(vars.add(variable().min(0).integer()));
    }
    
    // Objective: minimize total presses
    let mut problem = vars
        .minimise(button_presses.iter().sum::<Expression>())
        .using(default_solver);
    
    // Build constraint expressions
    let mut expressions = vec![Expression::with_capacity(num_buttons); targets.len()];
    for (button_idx, button_affects) in buttons.iter().enumerate() {
        for &counter_idx in button_affects {
            expressions[counter_idx] += button_presses[button_idx];
        }
    }
    
    // Add constraints: each counter equals target
    for (expression, &target) in expressions.into_iter().zip(targets.iter()) {
        problem.add_constraint(expression.eq(target as f64));
    }
    
    // Solve and extract result
    match problem.solve() {
        Ok(solution) => {
            let values: Vec<f64> = button_presses.iter()
                .map(|&v| solution.value(v))
                .collect();
            values.iter().sum::<f64>().round() as usize
        }
        Err(_) => 0
    }
}
```

**Result**: ✅ **SUCCESS**
- **Test Cases**: All passed (10, 12, 11 → 33)
- **Actual Input**: Completed instantly - all 157 machines solved
- **Final Answer**: 16757 button presses

### Critical Fix: Rounding
**Issue Discovered**: Initial implementation used `.sum::<f64>() as usize`
- This truncates fractional parts
- `minilp` solver returned values like 16756.999...
- Truncation gave wrong answer: 16754

**Fix**: Changed to `.round() as usize`
- Properly rounds to nearest integer
- Correct answer: 16757

---

## Why ILP Worked

### Advantages of Linear Programming Approach:
1. **Polynomial-time LP relaxation**: Solving the continuous relaxation is fast
2. **Branch-and-bound efficiency**: ILP solvers use sophisticated techniques
3. **Constraint propagation**: Reduces search space intelligently
4. **Dual bounds**: Provides upper/lower bounds for pruning

### Why All Search Approaches Failed: NP-Hard Complexity

**The fundamental issue**: Part 2 is an **Integer Linear Programming problem**, which is **NP-hard**.

All search-based approaches (DFS, Greedy, BFS, A*) failed because they tried to **explore state space** instead of **solving mathematically**:

| Approach | Why It Failed | Root Cause |
|----------|---------------|------------|
| **DFS/Backtracking** | O(T^B) states - exponential in max target value | Brute force through infinite solution space |
| **Greedy** | Local optimal ≠ global optimal | No guarantee of finding best solution |
| **BFS** | O(∏ T_i) visited states - millions of HashMap entries | State space explosion |
| **A*** | Inadmissible heuristic → wrong answer | Hard to find good lower bound for ILP |

**The key insight**: These are **not path-finding problems** - they're **optimization problems over continuous domains with integer constraints**.

- **Wrong paradigm**: "Search for solution by exploring states"
- **Right paradigm**: "Solve system of linear constraints with integer variables"

ILP solvers don't search the state space - they:
1. Solve LP relaxation (continuous, polynomial time)
2. Use branch-and-bound with cutting planes (intelligent search)
3. Exploit problem structure (constraint propagation)
4. Prune suboptimal branches early (dual bounds)

**Bottom line**: The exponential search space of NP-hard problems requires **specialized optimization algorithms**, not generic search algorithms.

### Solver Choice: minilp vs highs
- **highs**: State-of-the-art solver, fastest, best integer support
  - ❌ Requires CMake/native compilation
- **minilp**: Pure Rust, no native dependencies
  - ✅ Works out of the box
  - ⚠️ Slightly less accurate (requires rounding)
  - Fast enough for this problem size

---

## Performance Comparison

| Approach | Test Cases | Machine 3 (Large) | Time Complexity |
|----------|------------|-------------------|-----------------|
| Backtracking | ❌ Timeout | ❌ Timeout | O(T^B) exponential |
| Greedy | ❌ Wrong (19) | Fast | O(B*C) but suboptimal |
| BFS | ✅ Optimal | ❌ Timeout | O(∏ T_i) exponential space |
| A* | ❌ Wrong (12) | ❌ Timeout | O(∏ T_i) with bad heuristic |
| Z3 | N/A | N/A | Compilation failed |
| **ILP (minilp)** | ✅ Optimal | ✅ <1s | **Polynomial (LP) + branch-and-bound** |

---

## Key Learnings

### 1. Problem Recognition - The Critical Insight
- **Part 2 is NP-hard ILP**: Requires specialized solvers, not naive search
- **All search attempts failed due to NP-hardness**: DFS, Greedy, BFS, A* all hit exponential complexity
- **Paradigm shift required**: From "search for path" to "solve optimization problem"
- **Mathematical structure matters**: Underdetermined linear system (m < n equations) with integer constraints
- Recognizing problem class early saves days of implementation time

### 2. Solver Selection Matters
- **Native dependencies are painful**: Z3 would have worked but compilation issues
- **Pure Rust alternatives exist**: good_lp with minilp feature
- **Tradeoff**: Speed vs. ease of compilation

### 3. Floating-Point Precision
- ILP solvers may return near-integer solutions (e.g., 16756.999)
- **Always round** when converting to integer counts
- Use `.round()` not truncation (`as usize`)

### 4. Test Early on Large Inputs
- Test cases passed with BFS but actual input timed out
- **State space analysis** crucial for assessing scalability
- Machine 3: 9 counters × 194 max value = massive search space

### 5. Community Resources
- Reddit discussion mentioned Z3
- Tom Wilkinson's solution used good_lp with highs
- **Adapting working solutions** is valid when learning

---

## Final Solution Architecture

```
Day 10 Solution
│
├── Part 1: Binary Toggle (GF(2))
│   ├── Gaussian elimination over binary field
│   ├── RREF with XOR operations
│   ├── Free variable enumeration
│   └── ✅ Answer: 385
│
└── Part 2: Integer Counters (ILP)
    ├── Mathematical formulation as ILP
    ├── good_lp crate (minilp solver)
    ├── Integer constraints + minimization
    ├── Rounding fix for precision
    └── ✅ Answer: 16757
```

---

## Code Credits

- **Part 1**: Original implementation using Gaussian elimination
- **Part 2**: Based on Tom Wilkinson's ILP approach
  - Adapted from Reddit/community solutions
  - Modified to work with our input system and data structures
  - Added rounding fix for minilp solver precision

---

## Conclusion

This problem demonstrates the importance of:
1. **Problem classification**: Recognizing ILP vs. graph search
2. **Algorithmic complexity analysis**: Understanding when search explodes
3. **Tooling choices**: Balancing power vs. ease of use
4. **Precision handling**: Rounding floating-point solver results
5. **Community learning**: Leveraging existing solutions and adapting them

The journey from naive backtracking → BFS → A* → Z3 → ILP shows the iterative problem-solving process and the value of knowing when to switch approaches rather than optimizing a fundamentally flawed algorithm.
