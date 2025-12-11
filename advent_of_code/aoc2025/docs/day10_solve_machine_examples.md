# Day 10: `solve_machine` Function - Detailed Examples

This document walks through two complete examples of how the `solve_machine` function uses Gaussian elimination over GF(2) to find the minimum button presses needed.

## Mathematical Foundation

**GF(2) Field Operations:**
- Addition: XOR (`^` in Rust)
- `0 + 0 = 0`
- `0 + 1 = 1`
- `1 + 0 = 1`
- `1 + 1 = 0` (no carrying!)

**Key Insight:** Pressing a button twice is the same as not pressing it at all (toggles cancel out), so we only need to consider pressing each button 0 or 1 times.

---

## Example 1: Simple Case - Unique Solution

### Problem Setup
```
[.##] (2) (1,2) (0,1)
```

**Interpretation:**
- **Target:** 3 lights: `[OFF, ON, ON]` = `[0, 1, 1]`
- **Button 0:** `(2)` - affects light 2
- **Button 1:** `(1,2)` - affects lights 1 and 2
- **Button 2:** `(0,1)` - affects lights 0 and 1

**Goal:** Find minimum button presses to achieve target state.

### Step 1: Build Augmented Matrix

The matrix `[A | b]` where:
- `A[i][j] = 1` if button j affects light i
- `b[i] = 1` if light i should be ON in target

```
           btn0 btn1 btn2 | target
light 0:    0    0    1   |   0
light 1:    0    1    1   |   1
light 2:    1    1    0   |   1
```

**Reading the matrix:**
- Row 0: `0*btn0 + 0*btn1 + 1*btn2 = 0` (light 0 should be OFF)
- Row 1: `0*btn0 + 1*btn1 + 1*btn2 = 1` (light 1 should be ON)
- Row 2: `1*btn0 + 1*btn1 + 0*btn2 = 1` (light 2 should be ON)

### Step 2: Gaussian Elimination - Column 0

**Find pivot in column 0:**
```rust
// Looking for any row with 1 in column 0
// Row 0: matrix[0][0] = 0 ❌
// Row 1: matrix[1][0] = 0 ❌
// Row 2: matrix[2][0] = 1 ✓ Found pivot!
```

**Swap row 0 and row 2:**
```
           btn0 btn1 btn2 | target
light 2:    1    1    0   |   1  ← Now at row 0 (pivot)
light 1:    0    1    1   |   1
light 0:    0    0    1   |   0
```

**Eliminate other 1s in column 0:**
```rust
// Check each other row:
// Row 1: matrix[1][0] = 0, skip (already 0)
// Row 2: matrix[2][0] = 0, skip (already 0)
// No elimination needed!
```

**Record pivot:** `pivot_cols.push((0, 0))` - row 0, column 0

**Matrix after column 0:**
```
           btn0 btn1 btn2 | target
row 0:      1    1    0   |   1  [PIVOT col 0]
row 1:      0    1    1   |   1
row 2:      0    0    1   |   0
```

### Step 3: Gaussian Elimination - Column 1

**Current row = 1** (moved to next row after finding pivot)

**Find pivot in column 1:**
```rust
// Looking for 1 in column 1, starting from row 1
// Row 1: matrix[1][1] = 1 ✓ Found pivot!
```

**No swap needed** (pivot already in position)

**Record pivot:** `pivot_cols.push((1, 1))` - row 1, column 1

**Eliminate other 1s in column 1:**
```rust
// Check each other row for 1s in column 1:
// Row 0: matrix[0][1] = 1 → Need to eliminate!
//   XOR row 0 with row 1:
//   [1,1,0|1] XOR [0,1,1|1] = [1,0,1|0]
// Row 2: matrix[2][1] = 0, skip (already 0)
```

**Matrix after column 1:**
```
           btn0 btn1 btn2 | target
row 0:      1    0    1   |   0  [PIVOT col 0] (modified!)
row 1:      0    1    1   |   1  [PIVOT col 1]
row 2:      0    0    1   |   0
```

### Step 4: Gaussian Elimination - Column 2

**Current row = 2**

**Find pivot in column 2:**
```rust
// Looking for 1 in column 2, starting from row 2
// Row 2: matrix[2][2] = 1 ✓ Found pivot!
```

**Record pivot:** `pivot_cols.push((2, 2))` - row 2, column 2

**Eliminate other 1s in column 2:**
```rust
// Check each other row for 1s in column 2:
// Row 0: matrix[0][2] = 1 → Need to eliminate!
//   XOR row 0 with row 2:
//   [1,0,1|0] XOR [0,0,1|0] = [1,0,0|0]
// Row 1: matrix[1][2] = 1 → Need to eliminate!
//   XOR row 1 with row 2:
//   [0,1,1|1] XOR [0,0,1|0] = [0,1,0|1]
```

**Final RREF (Reduced Row Echelon Form):**
```
           btn0 btn1 btn2 | target
row 0:      1    0    0   |   0  [PIVOT col 0]
row 1:      0    1    0   |   1  [PIVOT col 1]
row 2:      0    0    1   |   0  [PIVOT col 2]
```

This is the **identity matrix** - perfect diagonal form!

### Step 5: Identify Free Variables

```rust
pivot_cols = [(0,0), (1,1), (2,2)]
pivot_col_set = {0, 1, 2}
free_vars = [] // All columns are pivots!
```

**No free variables!** This means we have a **unique solution**.

### Step 6: Read Solution

With no free variables, the solution is directly in the augmented column:

```rust
solution[0] = matrix[0][3] = 0  // Button 0: Don't press
solution[1] = matrix[1][3] = 1  // Button 1: Press once
solution[2] = matrix[2][3] = 0  // Button 2: Don't press
```

**Solution:** Press only **button 1** = **1 button press**

### Verification

Let's verify this works:
- Initial state: `[0, 0, 0]` (all lights OFF)
- Press button 1 (affects lights 1 and 2): `[0, 1, 1]`
- Target state: `[0, 1, 1]` ✓ **Match!**

---

## Example 2: Complex Case - Multiple Solutions

### Problem Setup
```
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1)
```

**Interpretation:**
- **Target:** 4 lights: `[OFF, ON, ON, OFF]` = `[0, 1, 1, 0]`
- **Button 0:** `(3)` - affects light 3
- **Button 1:** `(1,3)` - affects lights 1 and 3
- **Button 2:** `(2)` - affects light 2
- **Button 3:** `(2,3)` - affects lights 2 and 3
- **Button 4:** `(0,2)` - affects lights 0 and 2
- **Button 5:** `(0,1)` - affects lights 0 and 1

### Step 1: Build Augmented Matrix

```
           btn0 btn1 btn2 btn3 btn4 btn5 | target
light 0:    0    0    0    0    1    1   |   0
light 1:    0    1    0    0    0    1   |   1
light 2:    0    0    1    1    1    0   |   1
light 3:    1    1    0    1    0    0   |   0
```

### Step 2: Gaussian Elimination - Column 0

**Find pivot in column 0:**
```rust
// Row 0: matrix[0][0] = 0 ❌
// Row 1: matrix[1][0] = 0 ❌
// Row 2: matrix[2][0] = 0 ❌
// Row 3: matrix[3][0] = 1 ✓ Found pivot!
```

**Swap row 0 and row 3:**
```
           btn0 btn1 btn2 btn3 btn4 btn5 | target
row 0:      1    1    0    1    0    0   |   0  [PIVOT col 0]
row 1:      0    1    0    0    0    1   |   1
row 2:      0    0    1    1    1    0   |   1
row 3:      0    0    0    0    1    1   |   0
```

**Eliminate other 1s in column 0:**
```rust
// All other rows already have 0 in column 0
// No elimination needed
```

### Step 3: Gaussian Elimination - Column 1

**Current row = 1**

**Find pivot in column 1:**
```rust
// Row 1: matrix[1][1] = 1 ✓ Found pivot!
```

**Record pivot:** `pivot_cols.push((1, 1))`

**Eliminate other 1s in column 1:**
```rust
// Row 0: matrix[0][1] = 1 → Eliminate!
//   [1,1,0,1,0,0|0] XOR [0,1,0,0,0,1|1] = [1,0,0,1,0,1|1]
// Row 2: matrix[2][1] = 0, skip
// Row 3: matrix[3][1] = 0, skip
```

**Matrix after column 1:**
```
           btn0 btn1 btn2 btn3 btn4 btn5 | target
row 0:      1    0    0    1    0    1   |   1  [PIVOT col 0] (modified!)
row 1:      0    1    0    0    0    1   |   1  [PIVOT col 1]
row 2:      0    0    1    1    1    0   |   1
row 3:      0    0    0    0    1    1   |   0
```

### Step 4: Gaussian Elimination - Column 2

**Current row = 2**

**Find pivot in column 2:**
```rust
// Row 2: matrix[2][2] = 1 ✓ Found pivot!
```

**Record pivot:** `pivot_cols.push((2, 2))`

**Eliminate other 1s in column 2:**
```rust
// Row 0: matrix[0][2] = 0, skip
// Row 1: matrix[1][2] = 0, skip
// Row 3: matrix[3][2] = 0, skip
// No elimination needed!
```

**Matrix stays the same.**

### Step 5: Gaussian Elimination - Column 3

**Current row = 3**

**Find pivot in column 3:**
```rust
// Row 3: matrix[3][3] = 0 ❌
// No more rows to check
// Column 3 has NO PIVOT!
```

**Column 3 is a FREE VARIABLE!**

### Step 6: Gaussian Elimination - Column 4

**Still at row = 3** (didn't increment, no pivot found)

**Find pivot in column 4:**
```rust
// Row 3: matrix[3][4] = 1 ✓ Found pivot!
```

**Record pivot:** `pivot_cols.push((3, 4))`

**Eliminate other 1s in column 4:**
```rust
// Row 0: matrix[0][4] = 0, skip
// Row 1: matrix[1][4] = 0, skip
// Row 2: matrix[2][4] = 1 → Eliminate!
//   [0,0,1,1,1,0|1] XOR [0,0,0,0,1,1|0] = [0,0,1,1,0,1|1]
```

**Matrix after column 4:**
```
           btn0 btn1 btn2 btn3 btn4 btn5 | target
row 0:      1    0    0    1    0    1   |   1  [PIVOT col 0]
row 1:      0    1    0    0    0    1   |   1  [PIVOT col 1]
row 2:      0    0    1    1    0    1   |   1  [PIVOT col 2] (modified!)
row 3:      0    0    0    0    1    1   |   0  [PIVOT col 4]
```

### Step 7: Gaussian Elimination - Column 5

**Current row = 4** (but we only have 4 rows, indexed 0-3)

**Row >= num_lights, so we're done with Gaussian elimination!**

**Final RREF:**
```
           btn0 btn1 btn2 btn3 btn4 btn5 | target
row 0:      1    0    0    1    0    1   |   1  [PIVOT col 0]
row 1:      0    1    0    0    0    1   |   1  [PIVOT col 1]
row 2:      0    0    1    1    0    1   |   1  [PIVOT col 2]
row 3:      0    0    0    0    1    1   |   0  [PIVOT col 4]
```

### Step 8: Identify Free Variables

```rust
pivot_cols = [(0,0), (1,1), (2,2), (3,4)]
pivot_col_set = {0, 1, 2, 4}
free_vars = [3, 5] // Columns 3 and 5 are NOT pivots!
```

**We have 2 free variables!** This means multiple solutions exist.

### Step 9: Try All Combinations of Free Variables

With 2 free variables, we have **2² = 4** combinations to try:

#### **Combination 1: btn3=0, btn5=0** (mask = 0b00)

```rust
solution = [?, ?, ?, 0, ?, 0]  // Set free variables
```

**Back-substitute for pivot variables (reverse order):**

**Row 3 (pivot col 4):**
```
matrix[3]: [0,0,0,0,1,1|0]
btn4 = 0 XOR (0*btn5) = 0 XOR 0 = 0
solution = [?, ?, ?, 0, 0, 0]
```

**Row 2 (pivot col 2):**
```
matrix[2]: [0,0,1,1,0,1|1]
btn2 = 1 XOR (1*btn3 + 0*btn4 + 1*btn5) = 1 XOR (0 + 0 + 0) = 1
solution = [?, ?, 1, 0, 0, 0]
```

**Row 1 (pivot col 1):**
```
matrix[1]: [0,1,0,0,0,1|1]
btn1 = 1 XOR (0*btn3 + 0*btn4 + 1*btn5) = 1 XOR 0 = 1
solution = [?, 1, 1, 0, 0, 0]
```

**Row 0 (pivot col 0):**
```
matrix[0]: [1,0,0,1,0,1|1]
btn0 = 1 XOR (1*btn3 + 0*btn4 + 1*btn5) = 1 XOR 0 = 1
solution = [1, 1, 1, 0, 0, 0]
```

**Total presses:** 1+1+1+0+0+0 = **3 presses**

#### **Combination 2: btn3=1, btn5=0** (mask = 0b01)

```rust
solution = [?, ?, ?, 1, ?, 0]
```

**Back-substitute:**

**Row 3:** `btn4 = 0 XOR (0) = 0`
```
solution = [?, ?, ?, 1, 0, 0]
```

**Row 2:** `btn2 = 1 XOR (1*1 + 0 + 0) = 1 XOR 1 = 0`
```
solution = [?, ?, 0, 1, 0, 0]
```

**Row 1:** `btn1 = 1 XOR (0) = 1`
```
solution = [?, 1, 0, 1, 0, 0]
```

**Row 0:** `btn0 = 1 XOR (1*1 + 0) = 1 XOR 1 = 0`
```
solution = [0, 1, 0, 1, 0, 0]
```

**Total presses:** 0+1+0+1+0+0 = **2 presses** ← Better!

#### **Combination 3: btn3=0, btn5=1** (mask = 0b10)

```rust
solution = [?, ?, ?, 0, ?, 1]
```

**Back-substitute:**

**Row 3:** `btn4 = 0 XOR (1*1) = 1`
```
solution = [?, ?, ?, 0, 1, 1]
```

**Row 2:** `btn2 = 1 XOR (0 + 0 + 1*1) = 1 XOR 1 = 0`
```
solution = [?, ?, 0, 0, 1, 1]
```

**Row 1:** `btn1 = 1 XOR (1*1) = 1 XOR 1 = 0`
```
solution = [?, 0, 0, 0, 1, 1]
```

**Row 0:** `btn0 = 1 XOR (0 + 0 + 1*1) = 1 XOR 1 = 0`
```
solution = [0, 0, 0, 0, 1, 1]
```

**Total presses:** 0+0+0+0+1+1 = **2 presses** ← Also good!

#### **Combination 4: btn3=1, btn5=1** (mask = 0b11)

```rust
solution = [?, ?, ?, 1, ?, 1]
```

**Back-substitute:**

**Row 3:** `btn4 = 0 XOR (1*1) = 1`
```
solution = [?, ?, ?, 1, 1, 1]
```

**Row 2:** `btn2 = 1 XOR (1*1 + 0 + 1*1) = 1 XOR 2 = 1 XOR 0 = 1` (in GF(2): 1+1=0)
```
solution = [?, ?, 1, 1, 1, 1]
```

**Row 1:** `btn1 = 1 XOR (0 + 0 + 1*1) = 0`
```
solution = [?, 0, 1, 1, 1, 1]
```

**Row 0:** `btn0 = 1 XOR (1*1 + 0 + 1*1) = 1 XOR 0 = 1` (in GF(2): 1+1=0)
```
solution = [1, 0, 1, 1, 1, 1]
```

**Total presses:** 1+0+1+1+1+1 = **5 presses**

### Step 10: Select Minimum

```rust
min_presses = min(3, 2, 2, 5) = 2
```

**Answer:** **2 button presses** (either combination 2 or 3)

**Combination 2 verification:**
- Press buttons 1 and 3 (buttons `(1,3)` and `(2,3)`)
- Button 1 toggles lights 1,3: `[0,0,0,0]` → `[0,1,0,1]`
- Button 3 toggles lights 2,3: `[0,1,0,1]` → `[0,1,1,0]`
- Target: `[0,1,1,0]` ✓ **Match!**

**Combination 3 verification:**
- Press buttons 4 and 5 (buttons `(0,2)` and `(0,1)`)
- Button 4 toggles lights 0,2: `[0,0,0,0]` → `[1,0,1,0]`
- Button 5 toggles lights 0,1: `[1,0,1,0]` → `[0,1,1,0]`
- Target: `[0,1,1,0]` ✓ **Match!**

---

## Summary of Algorithm Phases

### Phase 1: Matrix Construction
- Build augmented matrix `[A | b]`
- Each row = one light's equation
- Each column (except last) = one button's effect
- Last column = target state

### Phase 2: Gaussian Elimination
- Process each column left-to-right
- Find pivot (a `1` in current column)
- Swap to bring pivot to diagonal
- XOR other rows to eliminate their `1`s in pivot column
- Result: Reduced Row Echelon Form (RREF)

### Phase 3: Variable Classification
- **Pivot columns:** Dependent variables (determined by free variables)
- **Non-pivot columns:** Free variables (can be any value)

### Phase 4: Solution Space
- **No free variables:** Unique solution (read from augmented column)
- **k free variables:** 2^k possible solutions (try all combinations)

### Phase 5: Optimization
- For each combination of free variables:
  - Back-substitute to find pivot variables
  - Count total button presses
- Return minimum count

---

## Code Correspondence

**Matrix construction:**
```rust
for (button_idx, button) in machine.buttons.iter().enumerate() {
    for &light_idx in button {
        matrix[light_idx][button_idx] = 1;
    }
}
```

**Gaussian elimination:**
```rust
for col in 0..num_buttons {
    if let Some(pivot_row) = (row..num_lights).find(|&r| matrix[r][col] == 1) {
        matrix.swap(row, pivot_row);
        // Eliminate using XOR
        for other_row in matrix.iter_mut() {
            if other_row[col] == 1 {
                other_row[c] ^= pivot_value;
            }
        }
    }
}
```

**Free variable enumeration:**
```rust
for free_mask in 0..(1 << num_free) {
    // Set free variables from mask bits
    // Back-substitute for pivot variables
    // Count total presses
}
```

---

## Key Takeaways

1. **GF(2) makes it binary:** Only press each button 0 or 1 times
2. **Gaussian elimination finds structure:** Identifies which buttons are independent
3. **Free variables = choices:** Multiple ways to solve = need optimization
4. **XOR is the magic:** All operations are XOR (addition in GF(2))
5. **Brute force over small space:** 2^k is manageable for small k (typically k ≤ 10)

The beauty is that what looks like a combinatorial explosion (6 buttons = 2^6 = 64 combinations to try naively) reduces to just 2^k combinations where k is typically much smaller (in our example, k=2 → only 4 combinations).
