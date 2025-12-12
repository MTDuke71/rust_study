# AoC 2025 Day 10 Part 2 - Integer Linear Programming Challenge

## Problem Analysis

Part 2 is an **Integer Linear Programming (ILP)** problem:
- **Objective**: Minimize total button presses
- **Constraints**: Each counter must reach exact target value
- **Variables**: Number of times each button is pressed (non-negative integers)

Mathematical formulation:
```
Minimize: Σ x_i (total button presses)
Subject to:
  For each counter j: Σ (a_ij * x_i) = t_j
  where a_ij = 1 if button i affects counter j, 0 otherwise
  x_i ≥ 0 for all i
```

## Performance Issues

### Test Cases (Working ✅)
- Machine 1: targets [3,5,4,7] → 10 presses ✅
- Machine 2: targets [7,5,12,7,2] → 12 presses ✅  
- Machine 3: targets [10,11,11,5,10,5] → 11 presses ✅

### Actual Input (Timeout ❌)
- Machine 3: targets [168,164,176,171,51,173,194,30,168]
  - 9 counters, 9 buttons
  - Max target value: 194
  - State space: potentially 194^9 ≈ 10^20 states
  - **BFS/A* timeout after 10 seconds**

## Attempted Approaches

### 1. BFS with Pruning ✅ (Tests) ❌ (Large Inputs)
- Explores states level-by-level
- Works for small targets (<20)
- Times out for targets >100

### 2. A* with Heuristic ❌ (Suboptimal)
- Heuristic: `h = Σ (target - current) / buttons_affecting_counter`
- **Problem**: Heuristic is inadmissible (not a true lower bound)
- Gets 12 instead of 11 for test case 3

### 3. Z3 SMT Solver (Recommended) ❌ (Compilation Failed)
- **Issue**: Z3 Rust crate requires libclang (Windows compilation problems)
- **Alternative**: Python z3-solver (not installed)
- **Reddit consensus**: Z3 is the intended solution

## Why ILP is Hard

Integer Linear Programming is **NP-hard** in the general case:
- Pure search approaches have exponential time complexity
- State space grows as O(T^C) where T=max_target, C=num_counters
- For machine 3: 194^9 ≈ 2^67 states

Proper solution requires:
- **Branch and bound** algorithms (Z3, CPLEX, Gurobi)
- **Cutting plane** methods
- **Specialized ILP solvers**

## Recommendations

### Immediate Workarounds:
1. **Install Z3 Python bindings**: `pip install z3-solver`
2. **Create Python helper script** that Rust calls via subprocess
3. **Use online ILP solver** (NEOS, Google OR-Tools)

### Long-term Solution:
```bash
# Install Visual Studio Build Tools with C++ compiler
# Then install Z3 Rust crate will work

# OR use Python helper:
cd advent_of_code/aoc2025/src/solver
python solve_day10_part2.py < ../../inputs/day10.txt
```

## Current Code Status

**File**: `aoc2025/src/solver/day10.rs`
- Part 1: ✅ Working (Gaussian elimination over GF(2))
- Part 2: ⚠️ Partial (passes tests, fails large inputs)

**Implementation**: A* search with admissible heuristic attempt
- Correctly solves 2/3 test cases
- Machine 3 test gives 12 instead of 11 (heuristic issue)
- Times out on actual input machine 3

## Next Steps

1. Fix A* heuristic to be truly admissible
2. Or revert to pure BFS (optimal but slow)
3. Or integrate Z3 via Python subprocess
4. Or submit Part 1 answer only and acknowledge Part 2 needs ILP solver

---

## Related Documentation

- **[[advent_of_code/aoc2025/Problem_Statements/summary]]** - Day 10 complete analysis with all 6 solution attempts and performance comparison
- **[[docs/day10_solve_machine_examples]]** - Part 1 Gaussian elimination walkthrough with complete examples
- **[[docs/day10_z3_setup]]** - Z3 alternative implementation (exact integers vs floating-point)
- **[[examples/day10_solution_analysis]]** - Deep dive on why ILP succeeds where search fails
- **[[advent_of_code/aoc2025/Problem_Statements/day10]]** - Original problem statement

*Tags: #aoc2025 #day10 #ilp #integer-linear-programming #optimization #np-hard #state-space-analysis*
