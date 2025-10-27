# 🧮 Computational Complexity Classes

**Theoretical classification of problems based on their inherent computational difficulty**

---

## 📖 Core Concept

**Computational complexity classes** categorize problems based on the resources (time, space, non-determinism) required to solve them, not specific algorithms. This is distinct from [[Big-O Analysis]], which measures how fast a particular algorithm runs.

**Key Distinction:**
- **Big-O (Algorithm Complexity)** → *"How fast does THIS algorithm run?"* 
- **Complexity Classes (Problem Complexity)** → *"How hard is THIS problem to solve by ANY algorithm?"*

```rust
// Example: Sorting
// Big-O: "Merge sort runs in O(n log n)"
fn merge_sort<T: Ord>(arr: &mut [T]) { /* ... */ }

// Complexity Class: "The sorting problem is in P"
// (can be solved in polynomial time)
```

---

## 🎯 **Major Complexity Classes**

### **P (Polynomial Time)**
*Problems that can be solved efficiently*

**Definition:** Problems solvable by a deterministic Turing machine in polynomial time: O(n^k) for some constant k.

**Characteristic:** If you can write an algorithm with polynomial time complexity, the problem is in P.

```rust
// ✅ P Problems - Efficiently solvable

// 1. Sorting - O(n log n)
fn sort_problem(arr: &mut [i32]) {
    arr.sort();  // P: Can solve in O(n log n)
}

// 2. Binary Search - O(log n)
fn search_problem(arr: &[i32], target: i32) -> Option<usize> {
    arr.binary_search(&target).ok()  // P: Can solve in O(log n)
}

// 3. Shortest Path (Dijkstra) - O((V + E) log V)
fn shortest_path_problem(graph: &Graph, start: NodeId, end: NodeId) -> Option<Vec<NodeId>> {
    dijkstra(graph, start, end)  // P: Polynomial time
}

// 4. Finding GCD - O(log n)
fn gcd_problem(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd_problem(b, a % b) }  // P: Euclidean algorithm
}
```

**P Problems in Your Missions:**
- **Mission1 (Stack):** All operations O(1) → In P
- **Mission3 (Binary Search):** O(log n) → In P
- **Mission5 (HashMap):** O(1) average insert/lookup → In P
- **Mission9 (Dijkstra):** O((V + E) log V) → In P

---

### **NP (Nondeterministic Polynomial Time)**
*Problems where solutions can be verified efficiently*

**Definition:** Problems where a proposed solution can be verified in polynomial time.

**Key Insight:** Easy to check if a solution is correct, but may be hard to find the solution.

```rust
// ✅ NP Problems - Easy to verify, possibly hard to solve

// 1. Subset Sum Problem
// Given: numbers [3, 34, 4, 12, 5, 2], target 9
// Question: Is there a subset that sums to 9?
fn verify_subset_sum(numbers: &[i32], subset_indices: &[usize], target: i32) -> bool {
    // VERIFICATION: O(n) - polynomial time ✓
    let sum: i32 = subset_indices.iter()
        .map(|&i| numbers[i])
        .sum();
    sum == target
}

// But FINDING the subset can be exponential:
fn find_subset_sum_naive(numbers: &[i32], target: i32) -> Option<Vec<usize>> {
    // SOLVING: O(2^n) - exponential time!
    // Must try all 2^n possible subsets
    generate_all_subsets(numbers)
        .find(|subset| verify_subset_sum(numbers, subset, target))
}

// 2. Hamiltonian Path Problem
// Given: a graph
// Question: Is there a path visiting each vertex exactly once?
fn verify_hamiltonian_path(graph: &Graph, path: &[NodeId]) -> bool {
    // VERIFICATION: O(n) - check path visits all nodes once ✓
    path.len() == graph.node_count() 
        && path.iter().all_unique()
        && path.windows(2).all(|w| graph.has_edge(w[0], w[1]))
}

// But FINDING the path requires checking all n! permutations

// 3. Boolean Satisfiability (SAT)
// Given: (A OR B) AND (NOT A OR C) AND (NOT B OR NOT C)
// Question: Can we assign true/false to make this true?
fn verify_sat_assignment(formula: &Formula, assignment: &HashMap<Var, bool>) -> bool {
    // VERIFICATION: O(n) - just evaluate the formula ✓
    formula.evaluate(assignment)
}

// But FINDING the assignment may require trying 2^n combinations
```

**Important:** All P problems are in NP (if you can solve it quickly, you can verify it quickly). The big question: **Is P = NP?** (One of the Millennium Prize Problems!)

---

### **Co-NP (Complement of NP)**
*Problems where "NO" answers can be verified efficiently*

**Definition:** Problems where you can efficiently verify that something is **NOT** a solution.

```rust
// Co-NP Example: Tautology Problem
// Given: (A OR NOT A) - is this always true?

// NP: Can verify "YES" instances quickly
// Co-NP: Can verify "NO" instances quickly

fn verify_not_tautology(formula: &Formula, counterexample: &HashMap<Var, bool>) -> bool {
    // VERIFICATION: O(n) - check this assignment makes formula false ✓
    !formula.evaluate(counterexample)
}

// If formula is NOT a tautology, we can prove it with one counterexample
// But proving it IS a tautology requires checking all 2^n assignments

// Another Co-NP Example: "Is this graph NOT 3-colorable?"
fn verify_not_3_colorable(graph: &Graph, proof: &ImpossibilityProof) -> bool {
    // Can verify NO answer efficiently
    proof.demonstrates_impossibility(graph)
}
```

**Relationship:** It's unknown if NP = Co-NP, but if P = NP, then NP = Co-NP = P.

---

### **NP-Hard**
*At least as hard as the hardest NP problems*

**Definition:** A problem H is NP-hard if every problem in NP can be reduced to H in polynomial time.

**Key Insight:** NP-hard problems are **at least as hard** as the hardest problems in NP. They may or may not be in NP themselves.

```rust
// Example: Halting Problem (NP-hard but NOT in NP!)
// Question: Given a program, will it halt or run forever?

// This is undecidable - no algorithm exists to solve it!
// But it's NP-hard because you can reduce any NP problem to it

// Another Example: Optimization version of Traveling Salesman
// Question: What's the SHORTEST tour visiting all cities?
fn find_shortest_tsp_tour(cities: &[City]) -> Vec<City> {
    // This is NP-hard
    // Not in NP because we can't verify optimality efficiently
    // (verifying "this is the shortest" requires knowing no shorter tour exists)
    
    // O(n!) naive solution
    all_permutations(cities)
        .min_by_key(|tour| tour_distance(tour))
        .unwrap()
}

// Decision version (in NP): "Is there a tour of length ≤ K?"
fn has_tour_within_distance(cities: &[City], max_distance: f64) -> bool {
    // This IS in NP - can verify a tour in polynomial time
    all_permutations(cities)
        .any(|tour| tour_distance(&tour) <= max_distance)
}
```

**NP-Hard Problems (Rust Contexts):**
- Optimal register allocation in compiler
- Package dependency resolution (general case)
- Optimal branch prediction

---

### **NP-Complete**
*The hardest problems in NP*

**Definition:** A problem is NP-complete if:
1. It is in NP (solutions verifiable in polynomial time)
2. It is NP-hard (all NP problems reduce to it)

**Key Insight:** NP-complete problems are the "hardest" problems in NP. If you find a polynomial-time algorithm for ANY NP-complete problem, you've proven P = NP (and won a million dollars)!

```rust
// Classic NP-Complete Problems

// 1. Traveling Salesman Problem (Decision Version)
// Given: cities and distances, target K
// Question: Is there a tour visiting all cities with distance ≤ K?
fn tsp_decision(cities: &[City], max_distance: f64) -> bool {
    // In NP: Can verify a tour in O(n) ✓
    // NP-hard: All other NP problems reduce to this ✓
    
    // Best known algorithms: O(n² * 2^n) dynamic programming
    solve_tsp_dp(cities, max_distance)
}

// 2. Knapsack Problem (0/1 variant)
// Given: items with weights and values, capacity
// Question: Can we get total value ≥ V without exceeding capacity?
fn knapsack_decision(items: &[(Weight, Value)], capacity: Weight, target_value: Value) -> bool {
    // In NP: Can verify a selection in O(n) ✓
    // NP-complete ✓
    
    // Pseudo-polynomial: O(n * capacity) dynamic programming
    solve_knapsack_dp(items, capacity) >= target_value
}

// 3. Graph Coloring
// Given: graph G, number k
// Question: Can vertices be colored with k colors so no adjacent vertices share a color?
fn is_k_colorable(graph: &Graph, k: usize) -> bool {
    // In NP: Can verify a coloring in O(E) ✓
    // NP-complete for k ≥ 3 ✓
    
    // Exponential algorithm: try all k^n colorings
    backtrack_coloring(graph, k).is_some()
}

// 4. Boolean Satisfiability (SAT)
// Given: Boolean formula
// Question: Is there an assignment making it true?
fn is_satisfiable(formula: &Formula) -> bool {
    // In NP: Can verify assignment in O(n) ✓
    // NP-complete (first proven NP-complete problem - Cook-Levin theorem) ✓
    
    // Modern SAT solvers use sophisticated heuristics
    // Worst case: O(2^n)
    sat_solver(formula).is_some()
}

// 5. Subset Sum
// Given: set of integers, target sum
// Question: Is there a subset summing to target?
fn has_subset_sum(numbers: &[i32], target: i32) -> bool {
    // In NP: Can verify subset in O(n) ✓
    // NP-complete ✓
    
    // Pseudo-polynomial: O(n * sum) dynamic programming
    dp_subset_sum(numbers, target)
}
```

**Why NP-Complete Matters:**
- If you encounter an NP-complete problem, don't expect to find an efficient exact algorithm
- Use approximation algorithms, heuristics, or constraint solvers
- For small inputs, exponential algorithms may be acceptable

---

## 📊 **Complexity Class Relationships**

```
                  ┌─────────────┐
                  │  Undecidable │  (Halting Problem)
                  └──────┬──────┘
                         │
                  ┌──────▼──────┐
                  │   NP-Hard   │  (At least as hard as NP)
                  │             │
                  │  ┌────────┐ │
                  │  │NP-Comp │ │  (Hardest problems in NP)
                  │  │lete    │ │
                  │  └───┬────┘ │
                  └──────┼──────┘
                         │
                  ┌──────▼──────┐
                  │     NP      │  (Verifiable in poly time)
                  │             │
                  │  ┌────┐     │
                  │  │ P  │     │  (Solvable in poly time)
                  │  └────┘     │
                  └─────────────┘
```

**Known Relationships:**
- P ⊆ NP (if solvable quickly, verifiable quickly)
- P ⊆ Co-NP (same reasoning)
- NP-Complete ⊆ NP ∩ NP-Hard
- If P = NP, then P = NP = Co-NP = NP-Complete

**Unknown Relationships:**
- P ?= NP (Millennium Prize Problem - $1 million reward!)
- NP ?= Co-NP
- Many others in complexity theory

---

## 🎓 **Practical Implications for Rust Development**

### **Recognizing Problem Difficulty**

```rust
// ✅ P Problem: Use standard algorithms
fn find_shortest_path(graph: &Graph, start: NodeId, end: NodeId) -> Option<Vec<NodeId>> {
    // Dijkstra's algorithm: O((V + E) log V)
    dijkstra(graph, start, end)
}

// ❌ NP-Complete Problem: Use approximations or heuristics
fn find_best_tour(cities: &[City]) -> Vec<City> {
    // Don't try to find optimal solution for large inputs!
    
    if cities.len() < 12 {
        // Small input: exact algorithm acceptable
        brute_force_tsp(cities)
    } else {
        // Large input: use approximation
        nearest_neighbor_tsp(cities)  // 2-approximation
        // or simulated_annealing_tsp(cities)
        // or genetic_algorithm_tsp(cities)
    }
}
```

### **Dealing with NP-Complete Problems**

**Strategy 1: Approximation Algorithms**
```rust
// Example: Vertex Cover (NP-complete)
// Goal: Find smallest set of vertices covering all edges

// 2-approximation algorithm (runs in polynomial time)
fn approx_vertex_cover(graph: &Graph) -> HashSet<NodeId> {
    let mut cover = HashSet::new();
    let mut uncovered_edges = graph.edges().collect::<Vec<_>>();
    
    while let Some((u, v)) = uncovered_edges.pop() {
        cover.insert(u);
        cover.insert(v);
        uncovered_edges.retain(|(a, b)| *a != u && *b != u && *a != v && *b != v);
    }
    
    cover  // Guaranteed to be ≤ 2 * optimal size
}
```

**Strategy 2: Heuristics and Metaheuristics**
```rust
// Example: TSP with simulated annealing
fn simulated_annealing_tsp(cities: &[City]) -> Vec<City> {
    let mut current_tour = random_tour(cities);
    let mut temperature = 1000.0;
    
    while temperature > 0.1 {
        let new_tour = perturb_tour(&current_tour);
        let delta = tour_distance(&new_tour) - tour_distance(&current_tour);
        
        if delta < 0.0 || random() < (-delta / temperature).exp() {
            current_tour = new_tour;
        }
        
        temperature *= 0.995;  // Cooling schedule
    }
    
    current_tour  // Good solution, not guaranteed optimal
}
```

**Strategy 3: Constraint Solvers**
```rust
// Use existing SAT/SMT solvers for NP-complete problems
// Example with z3 solver (for Boolean satisfiability)

use z3::*;

fn solve_with_sat_solver() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);
    
    // Define variables
    let a = Bool::new_const(&ctx, "a");
    let b = Bool::new_const(&ctx, "b");
    let c = Bool::new_const(&ctx, "c");
    
    // Add constraints: (a OR b) AND (NOT a OR c) AND (NOT b OR NOT c)
    solver.assert(&Bool::or(&ctx, &[&a, &b]));
    solver.assert(&Bool::or(&ctx, &[&a.not(), &c]));
    solver.assert(&Bool::or(&ctx, &[&b.not(), &c.not()]));
    
    // Check satisfiability
    match solver.check() {
        SatResult::Sat => {
            let model = solver.get_model().unwrap();
            println!("SAT: {:?}", model);
        },
        SatResult::Unsat => println!("UNSAT"),
        SatResult::Unknown => println!("Unknown"),
    }
}
```

**Strategy 4: Problem-Specific Optimizations**
```rust
// Example: If problem has special structure, exploit it

// Subset Sum with small target (pseudo-polynomial DP)
fn subset_sum_dp(numbers: &[i32], target: i32) -> bool {
    // O(n * target) - polynomial if target is small
    let mut dp = vec![false; (target + 1) as usize];
    dp[0] = true;
    
    for &num in numbers {
        for i in (num..=target).rev() {
            if dp[(i - num) as usize] {
                dp[i as usize] = true;
            }
        }
    }
    
    dp[target as usize]
}
```

---

## 🔬 **AoC and Complexity Classes**

### **P Problems in AoC**
```rust
// Most AoC problems are in P
// - Parsing and simulation: O(n)
// - Pathfinding: O((V + E) log V)
// - Dynamic programming: O(n²) or O(n³)

// Example: Day 1 - Sum of numbers
fn aoc_day1_p(input: &[i32]) -> i32 {
    input.iter().sum()  // O(n) - clearly in P
}
```

### **NP-Complete-Like Problems in AoC**
```rust
// Some AoC problems resemble NP-complete problems
// but have exploitable structure or small inputs

// Example: Day with TSP-like structure
fn aoc_tsp_like(cities: &[City]) -> i32 {
    if cities.len() <= 15 {
        // Small enough for brute force: O(n!)
        brute_force_tsp(cities)
    } else {
        // Use problem-specific constraints to prune search space
        // AoC problems often have hidden structure!
        optimized_search_with_pruning(cities)
    }
}
```

**Key Insight:** AoC problems that look NP-complete usually have:
1. Small input sizes (brute force feasible)
2. Special structure (dynamic programming works)
3. Greedy solution (problem is actually in P)

---

## 📊 **Decision Problems vs Optimization Problems**

### **Decision Problem (NP)**
```rust
// Question: "Does a solution exist?"
// Answer: Yes/No

fn has_solution(instance: &Problem) -> bool {
    // Can verify solution in polynomial time
    find_any_solution(instance).is_some()
}
```

### **Optimization Problem (NP-Hard)**
```rust
// Question: "What's the BEST solution?"
// Answer: Actual solution value

fn find_best_solution(instance: &Problem) -> Solution {
    // May need to verify optimality (harder!)
    find_optimal_solution(instance)
}
```

**Relationship:**
- Optimization version is at least as hard as decision version
- If optimization is in P, decision is in P
- Optimization TSP is NP-hard (not in NP)
- Decision TSP is NP-complete (in NP)

---

## 🎯 **Summary Table**

| Class | Definition | Verification | Examples | Solving |
|-------|-----------|--------------|----------|---------|
| **P** | Solvable in poly time | Poly time | Sorting, shortest path, binary search | Poly time algorithm exists |
| **NP** | Verifiable in poly time | Poly time | Subset sum, SAT, graph coloring | May require exponential time |
| **Co-NP** | "No" answers verifiable | Poly time (for "no") | Tautology, non-3-colorability | May require exponential time |
| **NP-Hard** | At least as hard as NP | May not be in NP | Halting problem, optimization TSP | No known poly algorithm |
| **NP-Complete** | Hardest problems in NP | Poly time | TSP (decision), SAT, knapsack | If P≠NP, requires exponential |

---

## 🔗 **Related Concepts**

- **[[Algorithm Analysis]]** - Analyzing algorithm performance
- **[[Big-O Analysis]]** - Algorithm time/space complexity (different from problem complexity!)
- **[[Performance Optimization]]** - Making algorithms faster
- **[[Dynamic Programming]]** - Solving some NP problems efficiently with special structure
- **[[AoC Patterns MOC]]** - Recognizing problem patterns in competitive programming
- **[[Rust Collections MOC]]** - Data structure complexity
- **[[Mission9 Overview]]** - Graph algorithms and their complexity classes

---

## 📚 **Further Reading**

### **Classic Papers**
- Cook's Theorem (1971) - Proving SAT is NP-complete
- Karp's 21 NP-complete problems (1972)

### **Books**
- *Introduction to the Theory of Computation* by Sipser
- *Computers and Intractability: A Guide to NP-Completeness* by Garey & Johnson

### **Online Resources**
- Complexity Zoo - Comprehensive list of complexity classes
- Scott Aaronson's blog - Quantum Computing and Complexity

---

*Tags: #complexity-theory #np-complete #computational-complexity #p-vs-np #theoretical-cs #algorithm-theory #np-hard #decidability #problem-hardness*

*Links: [[Algorithm Analysis]] | [[Big-O Analysis]] | [[Performance Optimization]] | [[Dynamic Programming]] | [[AoC Patterns MOC]] | [[Mission9 Overview]] | [[zettel-index]]*
