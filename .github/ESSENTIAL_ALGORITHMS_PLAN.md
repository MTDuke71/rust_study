# Essential Algorithms — Coverage & Study Plan

**Source**: *Essential Algorithms* by Rod Stephens (2nd ed.) — library copy, table of contents captured 2026-07-18.
**Goal**: Cross-reference the book's 19 chapters against what already exists in `rust_study` (Missions, AoC, Project Euler, Rust Book/RfR, zettelkasten) and turn the gaps into a prioritized build plan that follows this repo's existing conventions (Mission V-Cycle, tutorial scaffolds, zettelkasten notes).

**Method**: This is a living tracking document, not a one-shot task list. Update the status column and checkboxes as work lands — same spirit as the per-tutorial "Progress Tracking" checklists (see `tutorials/Mission12_tut/README.md`).

**Status legend**: ✅ Covered · 🟡 Partial · ❌ Gap

---

## Quick-Reference Table

| Ch | Chapter | Status | Primary Evidence |
|----|---------|--------|-------------------|
| 1 | Algorithm Basics | ✅ | `zettelkasten/Big-O Notation.md`, `Algorithm Analysis.md`, `Amortized Analysis.md` |
| 2 | Numerical Algorithms | 🟡 | Project Euler (`utils/primes.rs`, `number_theory.rs`, `combinatorics.rs`) covers primes/GCD/modular arith well; integration, Gaussian elimination, least squares, random walks are gaps |
| 3 | Linked Lists | 🟡 | Mission 4 (Box vs Rc\<RefCell\>) covers the core; self-organizing lists, loop detection/marking, multithreaded lists are gaps |
| 4 | Arrays | 🟡 | Mission 6 (Grids/2D) covers the spatial case; sparse arrays, triangular arrays, median/mode-finding are gaps |
| 5 | Stacks and Queues | 🟡 | Mission 1 (Stack), Mission 2 (Queue) complete; binomial heaps are a gap |
| 6 | Sorting | ❌ | Only a conceptual zettelkasten note (`sorting-algorithms.md`) — **no from-scratch implementation anywhere** despite using `.sort()` everywhere |
| 7 | Searching | 🟡 | Mission 3 (Binary Search) complete; interpolation search, majority voting are gaps |
| 8 | Hash Tables | 🟡 | Strong *conceptual* notes (`HashMap Internals.md`, `Collision Resolution.md`) but no from-scratch chaining/open-addressing implementation — Mission 5 uses `std::HashMap`, doesn't build one |
| 9 | Recursion | 🟡 | DP/memoization strongly covered (Mission 11); backtracking (N-Queens, Knight's Tour), fractals, round-robin scheduling, tail-call removal are gaps |
| 10 | Trees | 🟡 | `tutorials/Mission12_tut` in progress (Steps 1-4 done as of 2026-07-18); no real `Mission12/` yet |
| 11 | Balanced Trees | 🟡 | AVL is Step 5 of the Mission 12 tutorial (planned); 2-3 trees / B-trees are a lower-priority gap (std `BTreeMap` already solves the practical need) |
| 12 | Decision Trees | ❌ | Total gap — minimax, branch & bound, simulated annealing, knapsack/bin-packing/TSP |
| 13 | Basic Network Algorithms | ✅/🟡 | Very strong: Missions 7 (graph), 8 (BFS/DFS), 9 (Dijkstra/A\*), 10 (Union-Find). Gap: MST (Kruskal's, Prim's) |
| 14 | More Network Algorithms | 🟡 | Topological sort has zettelkasten notes; max-flow/min-cut, map coloring, cliques, Eulerian paths are gaps |
| 15 | String Algorithms | ❌ | `tutorials/Mission15_tut` planned (KMP, Rabin-Karp, tries, suffix trees) but not started |
| 16 | Cryptography | ❌ | Total gap — but reuses number-theory infra already built for Project Euler |
| 17 | Complexity Theory | 🟡 | Big-O and NP-completeness (`karps-21-np-complete-problems.md`) strong; reductions/3SAT proofs are a theory-only gap |
| 18 | Distributed Algorithms | ❌ | Total gap — good fit for the AUTOSAR/RTE analogy already used in `CLAUDE.md` |
| 19 | Interview Puzzles | 🟡 | Loosely covered by `zettelkasten/10-common-interview-problems/` |

---

## Detailed Chapter Breakdown

### Ch 1 — Algorithm Basics ✅
Pseudocode, algorithm features, Big O (rules 1-5), common runtime functions (1, log N, √N, N, N log N, N², 2^N, N!), visualizing functions.
**Status**: Covered conceptually via zettelkasten. No action needed — revisit only if a specific runtime-function visualization would help (e.g. a small Rust program plotting N vs N² vs 2^N growth, ASCII-chart style).

### Ch 2 — Numerical Algorithms 🟡
- [x] GCD / extended GCD, modular exponentiation — `utils/number_theory.rs`, math-foundations notes
- [x] Prime factorization, sieve, primality testing — `utils/primes.rs`
- [x] Combinatorics — `utils/combinatorics.rs`
- [ ] Random walks / self-avoiding walks
- [ ] Numerical integration (rectangle rule, trapezoid rule, adaptive quadrature, Monte Carlo)
- [ ] Root finding (bisection/Newton)
- [ ] Gaussian elimination (forward elimination + back substitution)
- [ ] Least squares fits (linear + polynomial)
- [ ] Fairness from biased random sources
**Plan**: The unbuilt half is linear-algebra/numerical-methods flavored — a natural fit as a short `project_euler` side-track or a small standalone tutorial (`tutorials/NumericalMethods_tut`), since several Project Euler problems ahead will want Gaussian elimination and Monte Carlo anyway.

### Ch 3 — Linked Lists 🟡
- [x] Singly/doubly linked lists, `Box<T>` vs `Rc<RefCell<T>>` — Mission 4
- [x] Sorted linked lists, insertion/selection sort on lists — Mission 4
- [ ] Self-organizing lists (Move-To-Front, Swap, Count, hybrid)
- [ ] Lists with loops: marking cells, hash-table detection, retracing, reversal, tortoise & hare
- [ ] Multithreaded linked lists
**Plan**: Cycle detection (tortoise & hare) likely already shows up informally in AoC solutions (`cycle-detection.md` note exists) — worth checking whether it's actually *implemented* on a linked list vs. just used conceptually on iteration state. Self-organizing lists are a cheap, satisfying Mission 4 follow-on exercise. Multithreaded lists tie into `rust_book/Ch16` (concurrency) — low priority.

### Ch 4 — Arrays 🟡
- [x] 1D/2D/N-D arrays, grid neighbor/distance operations — Mission 6
- [ ] Median/mode finding as explicit array algorithms (vs. incidental use)
- [ ] Triangular arrays
- [ ] Sparse arrays (row/column/get/set/delete)
- [ ] Matrices as a distinct topic from grids
**Plan**: Sparse arrays and triangular arrays are small, self-contained — good as Mission 6 exercises rather than a new mission.

### Ch 5 — Stacks and Queues 🟡
- [x] Array & linked-list stacks, array & linked-list queues — Missions 1, 2
- [x] Priority queues — used inside Mission 9 (Dijkstra/A\*)
- [ ] Binomial heaps (binomial trees, merging, enqueue/dequeue)
- [ ] Classic stack algorithms as explicit exercises: Tower of Hanoi, train sorting, stack-based insertion/selection sort
**Plan**: `tutorials/Mission13_tut` (Heaps & Priority Queues) is the natural home for binomial heaps as a stretch step.

### Ch 6 — Sorting ❌ (biggest surprising gap)
- [ ] O(N²): insertion sort, selection sort, bubble sort
- [ ] O(N log N): heapsort (+ complete-tree-as-array storage), quicksort (pivot selection, in-place, stack-based), mergesort
- [ ] Sub-O(N log N): counting sort, pigeonhole sort, bucket sort
**Plan**: This is core CS with zero hands-on implementation in the repo despite being used constantly via `.sort()`. **Recommend a new Mission** (`missions/Mission_Sorting` or folded into a "Mission 13: Sorting & Heaps" alongside the heap tutorial) implementing each algorithm generically over `Ord`, with Criterion benchmarks proving the O(N²) vs O(N log N) crossover — same pattern Mission 10 used to prove Union-Find's O(α(n)).

### Ch 7 — Searching 🟡
- [x] Linear search, binary search (recursive + iterative + trait-based) — Mission 3
- [ ] Interpolation search
- [ ] Majority voting (Boyer-Moore)
**Plan**: Small — good as two new exercises appended to Mission 3 rather than a new mission.

### Ch 8 — Hash Tables 🟡
- [x] Conceptual coverage — `Hash Function Design.md`, `HashMap Internals.md`, `Collision Resolution.md`
- [ ] From-scratch chaining implementation
- [ ] From-scratch open addressing: linear probing, quadratic probing, pseudorandom probing, double hashing, ordered hashing
- [ ] Deletion under open addressing (tombstones)
**Plan**: Mission 5 teaches *using* `std::HashMap`; this chapter is about *building* one. Good candidate for a **"Mission 5b" or dedicated tutorial** building a generic hash table with pluggable probing strategy, benchmarking load factor vs. probe count — directly extends the existing zettelkasten theory notes into working code (closes the "documented but not implemented" gap the Explore survey flagged).

### Ch 9 — Recursion 🟡
- [x] Factorial, Fibonacci, divide & conquer, dynamic programming, bottom-up/recursion removal — Mission 11 + general AoC recursion
- [ ] Rod-cutting (brute force vs. recursive vs. DP)
- [ ] Graphical recursion: Koch curve, Hilbert curve, Sierpiński curve/gasket, skyline problem
- [ ] Backtracking: Eight Queens, Knight's Tour
- [ ] Selections/permutations (with/without duplicates)
- [ ] Round-robin scheduling
- [ ] Tail-recursion removal
**Plan**: The backtracking + fractal-curve material is visual and fun — good for a standalone tutorial (`tutorials/Recursion_tut` or folded into Mission 12 as bonus content, since tree recursion is adjacent). N-Queens/Knight's Tour also double as classic interview-prep content, reinforcing `10-common-interview-problems/`.

### Ch 10 — Trees 🟡 (active)
- [x] Terminology, binary tree properties, tree representations — `tutorials/Mission12_tut` Step 1
- [x] BST insert/search — Step 2
- [x] Traversals (recursive + iterative, all 4 orders) — Step 3
- [x] Delete (3 cases), successor/predecessor, LCA, path sums, tree equality — **Step 4 (built 2026-07-18)**
- [ ] Parent pointers, Euler tours, all-pairs LCA — Step 4 follow-on / Step 5 prep
- [ ] Threaded trees
- [ ] Specialized trees: expression evaluation, interval trees, quadtrees, tries
**Plan**: Continue the tutorial in order — Step 5 (AVL/balanced), Step 6 (binary heap internals), Step 7 (tries, segment/Fenwick trees) — then promote to a real, V-Cycle `Mission12`.

### Ch 11 — Balanced Trees 🟡
- [ ] AVL: rotations, rebalancing on insert/delete — planned as Mission 12 tutorial Step 5
- [ ] 2-3 trees
- [ ] B-trees, top-down B-trees, B+trees
**Plan**: AVL is already on the roadmap. 2-3/B-trees are lower priority — the practical need is already met by `std::collections::BTreeMap`, and per the Integrator philosophy (recognize validated components, don't reimplement blindly) a short zettelkasten note comparing "why BTreeMap uses B-trees, not AVL, for disk/cache locality" may be higher-value than a full from-scratch B-tree implementation.

### Ch 12 — Decision Trees ❌
- [ ] Game trees / minimax, heuristics
- [ ] Exhaustive search, branch & bound
- [ ] Simulated annealing, hill climbing (+ sorted hill climbing)
- [ ] Classic optimization problems: subset sum, bin packing, cutting stock, knapsack, TSP, satisfiability
- [ ] Swarm intelligence (ant colony, bees, boids)
**Plan**: Large chapter, lowest near-term priority. Several of these (subset sum, knapsack, TSP) overlap with **Project Euler** problems that are coming up anyway — best absorbed opportunistically as those problems arrive rather than built as a standalone mission up front.

### Ch 13 — Basic Network Algorithms ✅/🟡 (strongest chapter overall)
- [x] Network representations, DFS/BFS traversal — Missions 7, 8
- [x] Connectivity testing, strongly connected components (Kosaraju's) — likely via Mission 8/AoC, verify
- [x] Shortest paths (label-setting = Dijkstra, label-correcting = Bellman-Ford-ish) — Mission 9
- [ ] Spanning trees: Kruskal's MST, Prim's MST, Euclidean MST, maze building
- [ ] All-pairs shortest paths (Floyd-Warshall)
- [ ] Transitive closure/reduction
**Plan**: **Quick win** — Kruskal's MST is almost free given Mission 10's Union-Find already exists (sort edges, union-find to detect cycles). Good candidate for a short follow-on to Mission 10 or a Mission 9/10 integration exercise. Floyd-Warshall (all-pairs shortest paths) is a natural Mission 9 extension too.

### Ch 14 — More Network Algorithms 🟡
- [x] Topological sort — zettelkasten notes exist (`topological-sort.md`, `kahns-topological-sort.md`); confirm whether it's implemented anywhere or documentation-only
- [ ] Cycle detection on directed graphs as an explicit algorithm (vs. incidental)
- [ ] Map coloring (2/3/4/5-coloring)
- [ ] Maximal flow (Ford-Fulkerson-style), min flow cut, network cloning
- [ ] Cliques (Bron-Kerbosch), triangle finding, community detection (Girvan-Newman, clique percolation)
- [ ] Eulerian paths/cycles (Fleury's, Hierholzer's)
**Plan**: Lower priority, richer chapter. If tackled, max-flow/min-cut pairs naturally with the existing `graph-minimum-cut.md` math note (currently theory-only from a Project Euler problem).

### Ch 15 — String Algorithms ❌
- [ ] Matching parentheses, arithmetic expression evaluation + parse trees
- [ ] Pattern matching: DFA/NFA construction, regex-to-DFA
- [ ] String searching (KMP, Boyer-Moore, Rabin-Karp)
- [ ] Edit distance
- [ ] Phonetic algorithms (Soundex, Metaphone)
**Plan**: Already scoped as `tutorials/Mission15_tut`. Sequence after Trees/Heaps (Mission 12/13) per the existing tutorial numbering. Edit distance overlaps with the DP work already done in Mission 11 — good cross-reference opportunity.

### Ch 16 — Cryptography ❌
- [ ] Transposition ciphers (row/column, route)
- [ ] Substitution ciphers (Caesar, Vigenère, simple substitution, one-time pad)
- [ ] Block ciphers (substitution-permutation networks, Feistel)
- [ ] Public-key crypto: RSA, Euler's totient, multiplicative inverses
**Plan**: **Quick win with strong reuse** — RSA needs exactly the modular-exponentiation/Euler's-totient/GCD infrastructure already built and documented for Project Euler (`utils/number_theory.rs`, `modular arithmetic & exponentiation` and `Chinese Remainder Theorem` math notes). A short cryptography tutorial (Caesar → Vigenère → RSA) would be one of the cheapest, highest-payoff items on this whole list because most of the hard math is already done.

### Ch 17 — Complexity Theory 🟡
- [x] Big-O notation, complexity classes, NP-completeness, Karp's 21 NP-complete problems — strong zettelkasten coverage
- [ ] Reductions (3SAT, bipartite matching as reduction targets)
- [ ] Detection ≤p Reporting ≤p Optimization equivalence proofs
- [ ] Approximate optimization
**Plan**: Mostly theory/reading, not implementation — lowest code-investment priority. A single zettelkasten note on reduction proofs would close most of the remaining gap.

### Ch 18 — Distributed Algorithms ❌
- [ ] Parallelism types, systolic arrays
- [ ] Race conditions, deadlock (relevant to `rust_book` Ch16 concurrency already)
- [ ] Embarrassingly parallel algorithms — **already touched**: AoC 2023 Day 16 used Rayon for exactly this pattern (see `CLAUDE.md` "Rayon Parallelization" insight)
- [ ] Dining philosophers (resource hierarchy, waiter, Chandy/Misra)
- [ ] Two Generals, Byzantine Generals, consensus, leader election, snapshot, clock synchronization
**Plan**: Advanced but high-motivation given the AUTOSAR/RTE background already referenced throughout `CLAUDE.md` ("async/await = RTE scheduling"). A good capstone using Tokio (already proven in Missions 9/10's REST API work) to simulate leader election or the Two Generals Problem. Lowest near-term priority, but flagged as a strong long-term fit rather than a generic gap.

### Ch 19 — Interview Puzzles 🟡
Logic puzzles, estimation questions. Loosely covered by `zettelkasten/10-common-interview-problems/`.
**Plan**: Not a coding track — skip as a formal mission/tutorial target. Revisit only if interview prep becomes an active goal.

---

## Recommended Sequencing

This is a suggested order, not a rigid schedule — it interleaves with whatever AoC/Euler/RfR work is already active rather than blocking it.

**Tier 1 — Finish what's in flight**
1. `tutorials/Mission12_tut` Steps 5-7 (AVL → binary heap internals → tries/segment trees), then promote to a real V-Cycle `Mission12`
2. `tutorials/Mission13_tut` (Heaps & Priority Queues), with binomial heaps as a stretch step

**Tier 2 — Quick wins (small scope, reuse existing infrastructure)**
3. Kruskal's MST (reuses Mission 10's Union-Find directly)
4. Cryptography mini-tutorial: Caesar → Vigenère → RSA (reuses Project Euler's `number_theory.rs`)
5. Mission 3 follow-on exercises: interpolation search, Boyer-Moore majority vote

**Tier 3 — Foundational gaps (currently zero hands-on coverage)**
6. **Sorting algorithms mission** — the single biggest surprise gap; implement + benchmark all of Ch6
7. **Hash table internals** — chaining vs. open addressing from scratch, turning the existing theory notes into code

**Tier 4 — Already-planned tutorials**
8. `tutorials/Mission15_tut` (String Algorithms: KMP, Rabin-Karp, tries, suffix trees, edit distance)

**Tier 5 — Larger / lower near-term priority**
9. Recursion deep-dive: backtracking (N-Queens, Knight's Tour) + fractal curves + round-robin scheduling
10. Decision Trees & Optimization — absorb opportunistically via Project Euler problems as they come up, rather than a standalone mission
11. More Network Algorithms: max-flow/min-cut, map coloring, cliques
12. Balanced tree internals beyond AVL (2-3/B-trees) — likely a zettelkasten note (why `BTreeMap` uses B-trees) rather than a full implementation

**Tier 6 — Theory-only, minimal code investment**
13. Complexity theory: reduction proofs, 3SAT (one zettelkasten note)
14. Distributed algorithms capstone (Two Generals / leader election via Tokio) — high motivation given the AUTOSAR background, but advanced; treat as a stretch goal, not a near-term commitment

---

*Created: 2026-07-18*
*Companion to: `.github/MATH_INTEGRATION_PLAN.md` (numerical/math side of this same gap analysis), `zettelkasten/2026_LEARNING_PLAN.md` (broader five-pillar plan)*
