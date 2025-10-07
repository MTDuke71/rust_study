# Dependency Tree for Wire 'a' - AoC Day 7 Circuit Analysis
# Generated on: September 30, 2025

This tree shows the gate-level dependencies for computing wire 'a' in the AoC Day 7 circuit.

## Tree Structure (Limited to 5 levels for readability)

```
a [ASSIGN] ← lx
├── lx [OR] ← lw OR lv  
    ├── lw [LSHIFT] ← lc LSHIFT 1
    │   └── lc [OR] ← lb OR la
    │       ├── lb [LSHIFT] ← kh LSHIFT 1
    │       │   └── kh [OR] ← kg OR kf
    │       │       ├── kg [LSHIFT] ← jm LSHIFT 1 
    │       │       └── kf [AND] ← 1 AND ke
    │       └── la [AND] ← 1 AND kz
    │           └── kz [AND] ← kw AND ky
    │               ├── kw [OR] ← kk OR kv
    │               └── ky [NOT] ← NOT kx
    └── lv [AND] ← 1 AND lu
        └── lu [AND] ← lr AND lt
            ├── lr [OR] ← lf OR lq
            │   ├── lf [OR] ← ld OR le
            │   │   ├── ld [RSHIFT] ← kk RSHIFT 1
            │   │   └── le [LSHIFT] ← la LSHIFT 15
            │   └── lq [AND] ← ln AND lp
            │       ├── ln [OR] ← lg OR lm
            │       └── lp [NOT] ← NOT lo
            └── lt [NOT] ← NOT ls
                └── ls [AND] ← lf AND lq
```

## Key Observations:

1. **Wire 'a' is the root** - Everything flows into computing this value
2. **Deep dependency chains** - Some paths go 10+ levels deep
3. **Shared computations** - Many gates reuse intermediate results (e.g., kk, lf)
4. **Gate type distribution**:
   - AND gates: 33.3% (primary logic operations)
   - OR gates: 23.5% (alternative paths)  
   - RSHIFT: 18.8% (bit shifting right)
   - NOT gates: 14.3% (inversions)
   - LSHIFT: 9.2% (bit shifting left)

## Full Details:

- **Total gates involved**: 336 out of 339 (99.1% efficiency)
- **Maximum dependency depth**: ~15-20 levels
- **Final result for wire 'a'**: 16076 (Part 1)

## How to View Complete Trees:

1. **Simple tree (5 levels)**: `.\extract_gates_for_a.ps1 -OutputFormat "tree"`
2. **Detailed tree (8 levels)**: `.\generate_dependency_tree.ps1 -MaxDepth 8`
3. **Complete gate list**: See `gates_for_wire_a_*.txt`

The tree above shows how wire 'a' depends on 'lx', which depends on 'lw' and 'lv', 
which in turn depend on hundreds of other gates in a complex web of bit manipulations
and logical operations - all to compute the final value 16076! 🎯

---

*Tags: #aoc2015 #day07 #graph-algorithms #dag #dependency-resolution #circuit-simulation #debugging*

*Links: [[../../../../zettelkasten/AoC 2015 MOC]] | [[../../Problem_Statements/summary]] | [[../../src/solver/day07]] | [[../../../../zettelkasten/Collections MOC]] | [[../../../../zettelkasten/zettel-index]]*