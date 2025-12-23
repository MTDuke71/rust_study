# Karp's 21 NP-Complete Problems

**Created**: 2024-12-22  
**Status**: #computer-science #complexity-theory #np-complete #historical

## Overview

In 1972, **Richard Karp** published the seminal paper *"Reducibility Among Combinatorial Problems"* proving that **21 fundamental computational problems are NP-complete**. This groundbreaking work built on Stephen Cook's 1971 proof that SAT (Boolean satisfiability) is NP-complete, establishing the foundation of computational complexity theory.

**Significance**: Karp's paper demonstrated that many practical problems from diverse domains (graph theory, scheduling, logic, arithmetic) are **all equivalent in computational difficulty**—if you can solve one efficiently, you can solve them all efficiently.

## The 21 Problems

### 1. **SATISFIABILITY (SAT)**
- **Problem**: Given a Boolean formula in CNF (conjunctive normal form), is there an assignment making it true?
- **Example**: `(A ∨ B) ∧ (¬A ∨ C) ∧ (¬B ∨ ¬C)` → Solution: A=true, B=false, C=false
- **Cook's Original**: The first proven NP-complete problem (1971)
- **AoC Connection**: Logic puzzle constraints

### 2. **0-1 INTEGER PROGRAMMING**
- **Problem**: Maximize `c·x` subject to `Ax ≤ b` where `x ∈ {0,1}ⁿ`
- **Example**: Knapsack optimization with binary decisions
- **Applications**: Resource allocation, scheduling

### 3. **CLIQUE**
- **Problem**: Does graph G contain a clique (fully connected subgraph) of size k?
- **Example**: Find if 5 people all know each other in social network
- **AoC Connection**: **[[aoc2024-day23]]** - Maximum clique problem (LAN Party password)
- **Related**: [[bron-kerbosch-algorithm]] for finding maximum cliques

### 4. **SET PACKING**
- **Problem**: Given sets, find maximum collection of disjoint sets
- **Example**: Schedule maximum non-conflicting meetings
- **Dual**: Set Covering (minimization version)

### 5. **VERTEX COVER**
- **Problem**: Find minimum vertices touching all edges in graph
- **Example**: Minimum cameras to monitor all hallways
- **Complement**: Independent Set (vertices with no edges between them)

### 6. **SET COVERING**
- **Problem**: Find minimum sets whose union covers universe
- **Example**: Minimum warehouses to serve all cities
- **Applications**: Facility location, network design

### 7. **FEEDBACK ARC SET**
- **Problem**: Minimum edges to remove to make directed graph acyclic
- **Example**: Break circular dependencies in build system
- **Related**: [[topological-sorting]] when graph is already acyclic

### 8. **FEEDBACK VERTEX SET**
- **Problem**: Minimum vertices to remove to make graph acyclic
- **Example**: Minimum points to eliminate all cycles
- **Dual**: Feedback Arc Set (edges vs vertices)

### 9. **DIRECTED HAMILTONIAN CIRCUIT**
- **Problem**: Find cycle visiting each vertex exactly once in directed graph
- **Example**: Traveling salesman on one-way streets
- **Undirected Version**: Also NP-complete (#11 below)

### 10. **UNDIRECTED HAMILTONIAN CIRCUIT**
- **Problem**: Find cycle visiting each vertex exactly once in undirected graph
- **Example**: Classic traveling salesman problem
- **AoC Connection**: Frequently appears in path optimization problems

### 11. **SATISFIABILITY (≤3 CNF)**
- **Problem**: SAT restricted to clauses with ≤3 literals
- **Example**: `(A ∨ B ∨ C) ∧ (¬A ∨ B) ∧ (¬B ∨ ¬C)`
- **Note**: 2-SAT is in P, 3-SAT is NP-complete (phase transition!)

### 12. **CHROMATIC NUMBER**
- **Problem**: Minimum colors to color graph vertices (no adjacent same color)
- **Example**: Minimum exam time slots avoiding conflicts
- **Applications**: Register allocation, scheduling
- **AoC Connection**: Constraint satisfaction problems

### 13. **CLIQUE COVER**
- **Problem**: Partition graph vertices into minimum cliques
- **Example**: Group people into teams where everyone knows each other
- **Dual**: Graph Coloring (independent sets vs cliques)

### 14. **EXACT COVER**
- **Problem**: Find subcollection where each element appears exactly once
- **Example**: Sudoku, pentomino tiling
- **Applications**: [[dancing-links]] algorithm (Knuth's Algorithm X)

### 15. **HITTING SET**
- **Problem**: Find minimum elements hitting all sets
- **Example**: Minimum skills to cover all job requirements
- **Dual**: Set Cover (minimize sets vs minimize elements)

### 16. **STEINER TREE**
- **Problem**: Find minimum-cost tree connecting required vertices
- **Example**: Minimum network connecting offices (can use intermediate nodes)
- **Applications**: Circuit design, network routing

### 17. **3-DIMENSIONAL MATCHING**
- **Problem**: Find disjoint triples from three sets
- **Example**: Match students, dorms, meal plans (all compatible)
- **2D Version**: Bipartite matching is in P (phase transition!)

### 18. **KNAPSACK**
- **Problem**: Maximize value fitting items in capacity constraint
- **Example**: Classic 0-1 knapsack optimization
- **Pseudo-polynomial**: Dynamic programming works when weights are small
- **AoC Connection**: Subset sum variants

### 19. **JOB SEQUENCING**
- **Problem**: Schedule jobs with deadlines to maximize completed jobs
- **Example**: Task scheduling with time constraints
- **Applications**: Real-time systems, project management

### 20. **PARTITION**
- **Problem**: Split set into two equal-sum subsets
- **Example**: Divide items into two balanced groups
- **Special Case**: Subset Sum with target = total/2

### 21. **MAX CUT**
- **Problem**: Partition graph vertices maximizing edges between partitions
- **Example**: Maximize disagreements in opinion graph
- **Applications**: Circuit layout, clustering

## Key Insights

### Computational Equivalence
All 21 problems are **polynomial-time reducible** to each other:
- If you solve one in polynomial time, you solve all
- If one requires exponential time, all do (assuming P ≠ NP)
- Provides evidence that P ≠ NP (no polynomial algorithms found in 50+ years)

### Problem Structure Patterns

**Graph Problems** (10 total):
- Clique, Vertex Cover, Set Packing, Feedback sets, Hamiltonian circuits, Chromatic Number, Clique Cover, Steiner Tree, Max Cut
- **Theme**: Structural constraints on connectivity

**Set/Logic Problems** (7 total):
- SAT variants, Set Covering, Exact Cover, Hitting Set, 3D Matching
- **Theme**: Combinatorial selection with constraints

**Optimization Problems** (4 total):
- Integer Programming, Knapsack, Job Sequencing, Partition
- **Theme**: Resource allocation under constraints

### Phase Transitions (Complexity Boundaries)

**2-SAT vs 3-SAT**:
- 2-SAT: Polynomial (linear time with implication graphs)
- 3-SAT: NP-complete
- **Boundary**: Clause size = 3

**Bipartite Matching vs 3D Matching**:
- 2D (Bipartite): Polynomial (Hungarian algorithm, max-flow)
- 3D (Tripartite): NP-complete
- **Boundary**: Dimension = 3

**Graph Coloring**:
- 2-coloring: Polynomial (BFS bipartiteness check)
- 3-coloring: NP-complete
- **Boundary**: Number of colors = 3

**Lesson**: Small problem changes can cause exponential complexity jumps!

## Historical Impact

### Before Karp (1972)
- Only SAT proven NP-complete (Cook, 1971)
- Unclear if other hard problems were related
- Each problem studied independently

### After Karp (1972)
- Framework for proving NP-completeness via reduction
- Thousands of problems proven NP-complete
- **Garey & Johnson (1979)**: Catalog of 300+ NP-complete problems
- Shifted focus from "find algorithm" to "prove hardness"

### Modern Implications
- **Algorithm Design**: For NP-complete problems, focus on:
  - Approximation algorithms (guaranteed sub-optimal solutions)
  - Heuristics and metaheuristics (practical good-enough solutions)
  - Special case algorithms (exploit problem structure)
  - Parameterized complexity (efficient for small parameters)
- **Cryptography**: Hardness assumptions (e.g., subset sum for public-key systems)
- **Computational Limits**: Understanding what's efficiently computable

## Advent of Code Connections

### Direct Appearances
- **[[aoc2024-day23]]** (LAN Party): **CLIQUE** problem (find maximum clique)
  - Part 2 uses [[bron-kerbosch-algorithm]] for maximum clique finding
  - 520 nodes, 13-node maximum clique found in <100ms with pivoting
  - Educational implementation vs `petgraph` library

### Common Patterns
- **HAMILTONIAN CIRCUIT**: Traveling salesman variants (optimal path problems)
- **KNAPSACK**: Subset sum, container packing (many AoC optimization problems)
- **SET COVER**: Minimum resource problems
- **CHROMATIC NUMBER**: Constraint satisfaction, scheduling problems
- **SATISFIABILITY**: Logic puzzle constraints, constraint propagation

### Typical AoC Approach
When encountering NP-complete problems in AoC:
1. **Check input size**: Often small enough for brute force/backtracking
2. **Exploit structure**: Problem may have special properties enabling polynomial solutions
3. **Use heuristics**: Greedy algorithms, dynamic programming, memoization
4. **Implement classical algorithms**: Educational value (e.g., Bron-Kerbosch for cliques)

**AoC Philosophy**: Problems designed to be solvable, even if theoretically hard
- Small input sizes make exponential algorithms practical
- Problem structure often allows optimizations
- Part 1 usually has polynomial solution, Part 2 escalates to NP-hard

## Practical Algorithm Strategies

### When You Encounter NP-Complete Problems

**1. Exact Algorithms (Small Inputs)**
- **Backtracking**: Depth-first search with pruning
- **Branch and Bound**: Prune branches using bounds
- **Dynamic Programming**: Exploit overlapping subproblems
- **Example**: [[bron-kerbosch-algorithm]] for CLIQUE with pivoting

**2. Approximation Algorithms (Guaranteed Quality)**
- **Greedy Approximation**: Simple heuristics with quality bounds
- **Example**: 2-approximation for Vertex Cover (repeatedly pick edge, remove both vertices)
- **Trade-off**: Fast runtime for bounded sub-optimality

**3. Heuristics (Good Practical Solutions)**
- **Genetic Algorithms**: Evolutionary optimization
- **Simulated Annealing**: Probabilistic hill climbing
- **Tabu Search**: Local search with memory
- **Trade-off**: No guarantees but often works well

**4. Parameterized Algorithms (Efficient for Small Parameters)**
- **Fixed-Parameter Tractable (FPT)**: O(f(k) · n^c) where k is parameter
- **Example**: Vertex Cover is FPT in solution size k
- **Trade-off**: Exponential in k, polynomial in n

**5. Special Cases (Exploit Structure)**
- **Planar Graphs**: Many NP-complete problems become polynomial
- **Bounded Treewidth**: Dynamic programming on tree decompositions
- **Interval Graphs**: Chromatic number polynomial
- **Trade-off**: Only works for specific problem instances

## Complexity Theory Context

### Complexity Classes

```
P ⊆ NP ⊆ PSPACE ⊆ EXP ⊆ NEXP
    ↑
    NP-complete (hardest in NP)
```

**P (Polynomial Time)**: Solvable in O(n^k) for some constant k
- Sorting, shortest paths, bipartite matching, 2-SAT

**NP (Nondeterministic Polynomial)**: Solution verifiable in polynomial time
- Guess solution, check in polynomial time
- SAT, CLIQUE, Hamiltonian Circuit

**NP-Complete**: Hardest problems in NP
- If any NP-complete problem is in P, then P = NP
- All 21 Karp problems (and thousands more)

**NP-Hard**: At least as hard as NP-complete (may not be in NP)
- Optimization versions (e.g., find largest clique vs decide if k-clique exists)

### Open Question: P vs NP

**One of the Millennium Prize Problems** ($1,000,000 prize)

**Consequences if P = NP**:
- Cryptography collapses (factoring becomes easy)
- Optimization problems solvable efficiently
- Many practical problems become tractable

**Consequences if P ≠ NP**:
- Confirms inherent computational limits
- Justifies approximation/heuristic approaches
- Current cryptography remains secure

**Current Belief**: P ≠ NP (strong evidence but no proof)

## References and Further Reading

### Original Paper
- **Karp, Richard M. (1972)**: "Reducibility Among Combinatorial Problems"
- *Complexity of Computer Computations*, Springer
- Defines 21 NP-complete problems via polynomial reductions

### Essential Books
- **Garey & Johnson (1979)**: *Computers and Intractability: A Guide to the Theory of NP-Completeness*
  - Catalog of 300+ NP-complete problems
  - Reduction techniques and proof strategies
  - **The** reference for NP-completeness
- **Cormen et al. (CLRS)**: *Introduction to Algorithms*
  - Chapter 34: NP-Completeness
  - Formal definitions, proof techniques

### Modern Perspectives
- **Arora & Barak (2009)**: *Computational Complexity: A Modern Approach*
- **Wigderson (2019)**: *Mathematics and Computation*
- **Parameterized Complexity**: Downey & Fellows

### Online Resources
- **Complexity Zoo**: Catalog of 500+ complexity classes
- **NP-Completeness Reference**: Michael Garey's maintained list

## Related Zettelkasten Notes

### Algorithm Implementations
- [[bron-kerbosch-algorithm]] - Maximum clique finding (CLIQUE problem)
- [[topological-sorting]] - Related to Feedback Arc Set
- [[dancing-links]] - Exact Cover algorithm (Knuth's Algorithm X)

### Complexity Theory
- [[p-vs-np-problem]] - Fundamental open question
- [[reduction-techniques]] - Proving NP-completeness
- [[approximation-algorithms]] - Handling NP-hard problems
- [[parameterized-complexity]] - Fixed-parameter tractability

### Graph Theory
- [[graph-cliques]] - Fully connected subgraphs
- [[graph-coloring]] - Chromatic number algorithms
- [[hamiltonian-paths]] - Circuit and path variants
- [[vertex-cover-algorithms]] - Covering strategies

### Advent of Code Applications
- [[aoc2024-day23]] - Maximum CLIQUE (LAN Party)
- [[aoc-np-complete-patterns]] - Common NP-hard problems in AoC
- [[aoc-optimization-strategies]] - Practical approaches to hard problems

### Practical Algorithms
- [[backtracking-algorithms]] - Exact solution techniques
- [[greedy-algorithms]] - Approximation strategies
- [[dynamic-programming]] - Optimal substructure exploitation
- [[heuristic-search]] - Practical optimization methods

---

*Tags: #np-complete #complexity-theory #karp #computational-complexity #graph-algorithms #optimization #historical #computer-science #algorithm-design #advent-of-code*

*Links: [[bron-kerbosch-algorithm]] [[aoc2024-day23]] [[graph-cliques]] [[p-vs-np-problem]] [[reduction-techniques]]*
