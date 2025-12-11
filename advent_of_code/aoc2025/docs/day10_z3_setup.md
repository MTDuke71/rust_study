# Day 10 Alternative Implementation with Z3

## Overview

This directory contains an alternative implementation of Day 10 Part 2 using the Z3 SMT solver instead of the `good_lp` crate.

## Current Status: ✅ Z3 Successfully Installed and Configured

The `day10_alt.rs` module uses the `z3` Rust crate, which requires:
1. **libclang** (for binding generation) ✅ INSTALLED
2. **Z3 library** (the actual solver) ✅ INSTALLED

## Why Two Implementations?

| Implementation | Solver | Dependencies | Pros | Cons |
|----------------|--------|--------------|------|------|
| **day10.rs** (default) | `good_lp` with `minilp` | Pure Rust | ✅ No native deps<br>✅ Easy compilation<br>✅ Works instantly | ⚠️ Returns f64 (needs rounding)<br>⚠️ Potential FP errors near 0.5 |
| **day10_alt.rs** (Z3) | Z3 SMT solver | Requires Z3 library | ✅ Industry-standard<br>✅ **Exact integers (no FP errors!)**<br>✅ Very powerful | ❌ Native dependency<br>❌ Installation required |

### Key Difference: Floating Point vs Exact Integers

**good_lp (day10.rs)**:
```rust
// Returns floating-point values that need rounding
let values: Vec<f64> = button_presses.iter()
    .map(|&v| solution.value(v))
    .collect();
let total: f64 = values.iter().sum();
let rounded_total = total.round() as usize;  // ⚠️ Must round!
```

**Z3 (day10_alt.rs)**:
```rust
// Returns EXACT integers - no rounding needed!
let presses = value.as_i64()?;  // ✅ Exact integer
result += presses as usize;      // No floating-point errors
```

This is particularly important for ILP problems where the optimal solution should be exactly an integer, but floating-point solvers may return values like `42.000000001` or `16756.999999`, requiring careful rounding logic.

## Installing Z3 (Required for day10_alt.rs)

### Option 1: Official Z3 Release (Recommended)

1. **Download Z3**: https://github.com/Z3Prover/z3/releases
   - Download: `z3-4.13.4-x64-win.zip` (or latest version)
   - Extract to: `C:\Program Files\z3\`

2. **Add to PATH**:
   ```powershell
   $env:Path += ";C:\Program Files\z3\bin"
   # Make permanent:
   [System.Environment]::SetEnvironmentVariable('Path', 
       $env:Path + ';C:\Program Files\z3\bin', 
       'User')
   ```

3. **Set environment variables**:
   ```powershell
   $env:Z3_SYS_Z3_HEADER = "C:\Program Files\z3\include\z3.h"
   # Make permanent:
   [System.Environment]::SetEnvironmentVariable('Z3_SYS_Z3_HEADER', 
       'C:\Program Files\z3\include\z3.h', 
       'User')
   ```

4. **Verify installation**:
   ```powershell
   z3 --version
   # Should output: Z3 version 4.13.4 - 64 bit
   ```

### Option 2: Via Chocolatey

```powershell
choco install z3 -y
```

Then set environment variable as above.

### Option 3: Via vcpkg (for Visual Studio users)

```powershell
vcpkg install z3:x64-windows
```

Then point to vcpkg installation:
```powershell
$env:Z3_SYS_Z3_HEADER = "C:\path\to\vcpkg\installed\x64-windows\include\z3.h"
```

## Testing Z3 Implementation

Once Z3 is installed:

```bash
cd advent_of_code/aoc2025
cargo test --package aoc2025 day10_alt -- --nocapture
```

## Running with Z3

To use the Z3 implementation instead of good_lp, you would need to:

1. Modify `main.rs` to call `day10_alt::solve_part2()` instead of `day10::solve_part2()`
2. Or create a separate binary/example that uses the alt implementation

## Comparison: good_lp vs Z3

### Performance (Expected)
Both should be instant for AoC-sized problems (~157 machines).

### Precision
- **good_lp (minilp)**: Returns floating-point near-integers (e.g., 16756.999) → requires rounding
- **Z3**: Returns exact integers → no rounding needed

### Ease of Use
- **good_lp**: Just works, pure Rust
- **Z3**: Requires external library installation, but more powerful

## Why Keep Both?

Educational purposes:
1. **good_lp** demonstrates that pure Rust solutions exist
2. **Z3** shows how to integrate industrial-strength solvers
3. Illustrates trade-offs: convenience vs. power

## Current Recommendation

**Use `day10.rs` (good_lp)** unless:
- You need exact integer guarantees without rounding
- You're already using Z3 in your workflow
- You want to learn Z3 integration

The good_lp solution works perfectly for AoC and is much easier to set up!

## Future: Optional Z3 Feature Flag

Could make Z3 optional:
```toml
[features]
default = []
z3-solver = ["z3"]

[dependencies]
z3 = { version = "0.12", optional = true }
```

Then conditionally compile day10_alt only when feature is enabled.

---

*Last Updated: December 10, 2025*
*Status: Z3 implementation complete but requires Z3 library installation*

---

## Related Documentation
- **[[../Problem_Statements/day10]]** - Original problem statement
- **[[../Problem_Statements/summary]]** - Complete Day 10 analysis with 6 solution attempts
- **[[../Part2_ILP_NOTES]]** - ILP formulation and state space analysis
- **[[day10_solve_machine_examples]]** - Part 1 Gaussian elimination walkthrough
- **[[../examples/day10_solution_analysis]]** - Deep dive on solver comparison

*Tags: #aoc2025 #day10 #z3 #smt-solver #ilp #exact-integers #floating-point*
