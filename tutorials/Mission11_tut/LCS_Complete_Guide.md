# Longest Common Subsequence (LCS) - Complete Guide

## Problem Definition

**Input**: Two strings `s1` and `s2`  
**Output**: Length of the longest common subsequence

**Important**: A **subsequence** can have letters in between (non-contiguous), as long as the order is preserved.

### Example
```
s1 = "ABC"
s2 = "AC"
LCS = "AC" (length 2)

s1 = "AGGTAB"  
s2 = "GXTXAYB"
LCS = "GTAB" (length 4)
```

In the second example, "GTAB" appears in both strings with gaps:
- In s1: **G**--**T**-**AB**
- In s2: **G**-X**T**-**A**-Y**B**

---

## Method 1: Naive Recursion

### Core Idea
Try all possible subsequences recursively. At each position, make a choice:
- If characters match → include them and move both pointers
- If they don't match → try skipping from either string

### Code
```rust
fn lcs_naive(s1: &str, s2: &str) -> usize {
    lcs_naive_helper(s1.as_bytes(), s2.as_bytes(), 0, 0)
}

fn lcs_naive_helper(s1: &[u8], s2: &[u8], i: usize, j: usize) -> usize {
    // Base case: reached end of either string
    if i >= s1.len() || j >= s2.len() {
        return 0;
    }

    if s1[i] == s2[j] {
        // Characters match - include and move both pointers
        1 + lcs_naive_helper(s1, s2, i + 1, j + 1)
    } else {
        // No match - try both options and take the max
        let skip_s1 = lcs_naive_helper(s1, s2, i + 1, j);  // Skip s1[i]
        let skip_s2 = lcs_naive_helper(s1, s2, i, j + 1);  // Skip s2[j]
        skip_s1.max(skip_s2)
    }
}
```

### Step-by-Step Example: "ABC" vs "AC"

```
lcs(0,0): 'A' == 'A' → MATCH!
  ├─ 1 + lcs(1,1)
  │
  └─ lcs(1,1): 'B' != 'C' → NO MATCH
       ├─ Path A: skip 'B' → lcs(2,1)
       │    └─ lcs(2,1): 'C' == 'C' → MATCH!
       │         └─ 1 + lcs(3,2) = 1 + 0 = 1
       │
       └─ Path B: skip 'C' → lcs(1,2)
            └─ lcs(1,2): 'B' vs END → 0
       
       max(1, 0) = 1
  
  Final: 1 + 1 = 2 ✓
```

### Characteristics
- **Time Complexity**: O(2^(m+n)) - exponential! Creates binary tree of calls
- **Space Complexity**: O(m+n) - recursion stack depth
- **Pros**: Simple to understand, directly maps to problem definition
- **Cons**: Extremely slow, recalculates same subproblems many times

### When to Use
- Only for very small strings (< 10 characters)
- Educational purposes to understand the problem
- Proof of correctness for other approaches

---

## Method 2: Memoization (Top-Down DP)

### Core Idea
Same as naive recursion, but **cache results** to avoid recalculating the same `(i, j)` position multiple times.

### Code
```rust
fn lcs_memoized(s1: &str, s2: &str) -> usize {
    let mut memo = HashMap::new();
    lcs_memo_helper(s1.as_bytes(), s2.as_bytes(), 0, 0, &mut memo)
}

fn lcs_memo_helper(
    s1: &[u8],
    s2: &[u8],
    i: usize,
    j: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    // Base case
    if i >= s1.len() || j >= s2.len() {
        return 0;
    }

    // Check cache first
    if let Some(&result) = memo.get(&(i, j)) {
        return result;  // Already computed!
    }

    // Compute result
    let result = if s1[i] == s2[j] {
        1 + lcs_memo_helper(s1, s2, i + 1, j + 1, memo)
    } else {
        let skip_s1 = lcs_memo_helper(s1, s2, i + 1, j, memo);
        let skip_s2 = lcs_memo_helper(s1, s2, i, j + 1, memo);
        skip_s1.max(skip_s2)
    };

    // Cache the result
    memo.insert((i, j), result);
    result
}
```

### Why the Key is (i, j) Not Characters

**Consider**: `s1 = "AAA"`, `s2 = "AA"`

If we used characters as the key:
```rust
memo['A']['A'] = ???  // Which 'A' pair? There are 6 different positions!
```

Each position has a **different answer** based on remaining string length:
```
lcs(0, 0) with "AAA" vs "AA" → 2
lcs(1, 0) with "AA"  vs "AA" → 2
lcs(2, 0) with "A"   vs "AA" → 1  ← Different result, same character!
```

**Position = state of the problem** (how much work remains)  
**Character = data we look up** at that position

### Memo Contents for "ABC" vs "AC"

After running, the memo would contain:
```rust
{
    (0, 0): 2,  // 'A' == 'A', matched, result = 2
    (1, 1): 1,  // 'B' != 'C', no match, result = 1
    (2, 1): 1,  // 'C' == 'C', matched, result = 1
    (1, 2): 0,  // 'B' vs END, base case
}
```

**Every position gets cached**, whether it matched or not!

### Characteristics
- **Time Complexity**: O(m×n) - each (i,j) computed once
- **Space Complexity**: O(m×n) HashMap + O(m+n) stack
- **Pros**: Much faster than naive, intuitive recursion structure
- **Cons**: HashMap overhead, still has recursion stack

### When to Use
- Medium-sized strings (< 1000 characters)
- When you want recursive structure but need speed
- Debugging (easy to inspect memo contents)

---

## Method 3: Bottom-Up DP (Iterative)

### Core Idea
Build a 2D table **iteratively** from end to start. No recursion - just nested loops!

### Code
```rust
fn lcs_bottom_up(s1: &str, s2: &str) -> usize {
    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();
    let m = s1_bytes.len();
    let n = s2_bytes.len();

    if m == 0 || n == 0 {
        return 0;
    }

    // Create (m+1) × (n+1) table, initialized to 0
    let mut dp = vec![vec![0; n + 1]; m + 1];

    // Fill from bottom-right to top-left
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if s1_bytes[i] == s2_bytes[j] {
                dp[i][j] = 1 + dp[i + 1][j + 1];
            } else {
                dp[i][j] = dp[i + 1][j].max(dp[i][j + 1]);
            }
        }
    }

    dp[0][0]  // Answer at top-left
}
```

### Why Fill Backwards?

Each cell `dp[i][j]` depends on:
- `dp[i+1][j+1]` - diagonal (next row, next column)
- `dp[i+1][j]` - below (next row)
- `dp[i][j+1]` - right (same row)

By filling **from bottom-right to top-left**, these dependencies are already computed!

### Building the Table for "ABC" vs "AC"

**Step 1**: Initialize table with zeros
```
       A    C   END
A :    0    0    0
B :    0    0    0
C :    0    0    0
END:   0    0    0
```

**Step 2**: Fill backwards (reverse order)

Position (2,1): `s1[2]='C'`, `s2[1]='C'` → MATCH!
```
dp[2][1] = 1 + dp[3][2] = 1 + 0 = 1
```

Position (2,0): `s1[2]='C'`, `s2[0]='A'` → No match
```
dp[2][0] = max(dp[3][0], dp[2][1]) = max(0, 1) = 1
```

Position (1,1): `s1[1]='B'`, `s2[1]='C'` → No match
```
dp[1][1] = max(dp[2][1], dp[1][2]) = max(1, 0) = 1
```

Position (1,0): `s1[1]='B'`, `s2[0]='A'` → No match
```
dp[1][0] = max(dp[2][0], dp[1][1]) = max(1, 1) = 1
```

Position (0,1): `s1[0]='A'`, `s2[1]='C'` → No match
```
dp[0][1] = max(dp[1][1], dp[0][2]) = max(1, 0) = 1
```

Position (0,0): `s1[0]='A'`, `s2[0]='A'` → MATCH!
```
dp[0][0] = 1 + dp[1][1] = 1 + 1 = 2
```

**Final Table**:
```
       A    C   END
A :    2    1    0
B :    1    1    0
C :    1    1    0
END:   0    0    0
```

Answer: `dp[0][0] = 2` ✓

### Characteristics
- **Time Complexity**: O(m×n) - fills every cell once
- **Space Complexity**: O(m×n) - full 2D table
- **Pros**: No recursion (no stack overflow), faster than memoization (array access vs HashMap)
- **Cons**: Uses more space than necessary (see Method 4)

### When to Use
- Production code for LCS
- When you need to reconstruct the subsequence (see Method 5)
- When recursion depth might be an issue

---

## Method 4: Space-Optimized DP

### Core Idea
Notice that `dp[i][j]` only depends on the **next row** and **current row**. We don't need the entire table - just 2 rows!

### Code
```rust
fn lcs_space_optimized(s1: &str, s2: &str) -> usize {
    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();

    // Make sure we iterate over the longer string
    // and store arrays for the shorter one
    let (shorter, longer) = if s1_bytes.len() <= s2_bytes.len() {
        (s1_bytes, s2_bytes)
    } else {
        (s2_bytes, s1_bytes)
    };

    let n = shorter.len();
    let mut prev = vec![0; n + 1];  // Previous row
    let mut curr = vec![0; n + 1];  // Current row

    // Process each row of the longer string
    for i in (0..longer.len()).rev() {
        for j in (0..n).rev() {
            curr[j] = if longer[i] == shorter[j] {
                1 + prev[j + 1]       // Diagonal from previous row
            } else {
                curr[j + 1].max(prev[j])  // Right (curr) or below (prev)
            };
        }
        std::mem::swap(&mut prev, &mut curr);  // Current becomes previous
    }

    prev[0]
}
```

### How the Two-Row System Works

**Mapping from full table to 2 rows**:
- `dp[i+1][j+1]` → `prev[j+1]` (diagonal in previous row)
- `dp[i+1][j]` → `prev[j]` (same column in previous row)
- `dp[i][j+1]` → `curr[j+1]` (already computed in current row)

### Example: "ABC" vs "AC"

**Iteration 1** (computing row 2):
```
prev = [0, 0, 0]  ← Row 3 (END row)
curr = [1, 1, 0]  ← Row 2 being computed
swap → prev becomes [1, 1, 0]
```

**Iteration 2** (computing row 1):
```
prev = [1, 1, 0]  ← Row 2 (just finished)
curr = [1, 1, 0]  ← Row 1 being computed
swap → prev becomes [1, 1, 0]
```

**Iteration 3** (computing row 0):
```
prev = [1, 1, 0]  ← Row 1 (just finished)
curr = [2, 1, 0]  ← Row 0 being computed
swap → prev becomes [2, 1, 0]
```

Answer: `prev[0] = 2` ✓

### Space Savings

| **Strings** | **Method 3** | **Method 4** | **Savings** |
|-------------|--------------|--------------|-------------|
| (100, 10)   | 1,111 cells  | 22 cells     | 98%         |
| (50, 50)    | 2,601 cells  | 102 cells    | 96%         |
| (10, 1000)  | 11,011 cells | 22 cells     | 99.8%       |

### Characteristics
- **Time Complexity**: O(m×n) - same as Method 3
- **Space Complexity**: O(min(m,n)) - massive improvement!
- **Pros**: Very memory efficient, same speed as Method 3
- **Cons**: Cannot reconstruct the actual subsequence (no full table)

### When to Use
- Very long strings (millions of characters)
- Memory-constrained environments
- When you only need the **length**, not the actual characters

---

## Method 5: Reconstruction (Extract the Subsequence)

### Core Idea
Build the full DP table (Method 3), then **backtrack** through it to extract which characters were actually in the LCS.

### Code
```rust
fn lcs_reconstruct(s1: &str, s2: &str) -> String {
    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();
    let m = s1_bytes.len();
    let n = s2_bytes.len();

    if m == 0 || n == 0 {
        return String::new();
    }

    // PHASE 1: Build full DP table (same as Method 3)
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if s1_bytes[i] == s2_bytes[j] {
                dp[i][j] = 1 + dp[i + 1][j + 1];
            } else {
                dp[i][j] = dp[i + 1][j].max(dp[i][j + 1]);
            }
        }
    }

    // PHASE 2: Backtrack to extract the subsequence
    let mut result = String::new();
    let mut i = 0;
    let mut j = 0;

    while i < m && j < n {
        if s1_bytes[i] == s2_bytes[j] {
            // This character is IN the LCS!
            result.push(s1_bytes[i] as char);
            i += 1;  // Move diagonally
            j += 1;
        } else if dp[i + 1][j] > dp[i][j + 1] {
            // LCS came from skipping s1[i]
            i += 1;  // Move down
        } else {
            // LCS came from skipping s2[j]
            j += 1;  // Move right
        }
    }

    result
}
```

### The Three Backtracking Cases

**Case 1: Characters Match**
```rust
if s1_bytes[i] == s2_bytes[j] {
    result.push(s1_bytes[i] as char);  // Add to result
    i += 1;  // Move both pointers
    j += 1;
}
```
This character is DEFINITELY in the LCS.

**Case 2: Skip from s1**
```rust
else if dp[i + 1][j] > dp[i][j + 1] {
    i += 1;  // Move down (skip s1[i])
}
```
The optimal path skipped `s1[i]`, so follow that path.

**Case 3: Skip from s2**
```rust
else {
    j += 1;  // Move right (skip s2[j])
}
```
The optimal path skipped `s2[j]`, so follow that path.

### Backtracking Example: "ABC" vs "AC"

**DP Table**:
```
       A    C   END
A :    2    1    0
B :    1    1    0
C :    1    1    0
END:   0    0    0
```

**Path through table**:

1. **Start (0,0)**: 'A' == 'A' → MATCH! ✓
   - Add 'A' to result: `result = "A"`
   - Move to (1,1)

2. **At (1,1)**: 'B' != 'C' → NO MATCH
   - Check: `dp[2][1]=1` > `dp[1][2]=0`
   - Skip 'B', move to (2,1)

3. **At (2,1)**: 'C' == 'C' → MATCH! ✓
   - Add 'C' to result: `result = "AC"`
   - Move to (3,2)

4. **At (3,2)**: Out of bounds, done!

**Result**: `"AC"` ✓

### Why Backtracking Works

The DP table **encodes all the decisions**:
- When we built it, we chose between diagonal (match) or max(down, right) (skip)
- During reconstruction, we **reverse the logic** to find which path gave us the optimal value
- We only follow the **winning path**, ignoring all the dead ends

### Characteristics
- **Time Complexity**: O(m×n) for table + O(m+n) for backtracking
- **Space Complexity**: O(m×n) - needs full table
- **Pros**: Gets both length AND actual subsequence
- **Cons**: Can't use space optimization (Method 4)

### When to Use
- When you need the actual characters, not just the length
- DNA sequence alignment
- Diff tools (finding matching lines between files)
- Any application where you need to show WHAT matched

---

## Method Comparison Summary

| **Method** | **Time** | **Space** | **Gets Length?** | **Gets String?** | **Best For** |
|------------|----------|-----------|------------------|------------------|--------------|
| 1. Naive | O(2^n) | O(n) | ✓ | ✗ | Learning only |
| 2. Memoized | O(m×n) | O(m×n) | ✓ | ✗ | Medium strings, debugging |
| 3. Bottom-Up | O(m×n) | O(m×n) | ✓ | ✗ | Production code |
| 4. Space-Opt | O(m×n) | O(min(m,n)) | ✓ | ✗ | Very long strings |
| 5. Reconstruct | O(m×n) | O(m×n) | ✓ | ✓ | When you need the actual subsequence |

---

## Key Insights

### 1. State vs Data
- **State** = `(i, j)` positions (where we are in the problem)
- **Data** = characters at those positions (what we're comparing)
- Memoization key must be the **state**, not the data

### 2. Top-Down vs Bottom-Up
- **Top-Down** (Method 2): Start at problem, recurse to base cases
- **Bottom-Up** (Method 3): Start at base cases, build up to problem
- Both compute the same values, just in different order

### 3. Space Optimization
- Notice dependencies: only need **previous row** for next computation
- Trade space for inability to reconstruct
- Huge savings for long strings

### 4. Reconstruction
- Need full table to backtrack
- Follow the path that gave optimal values
- Separate "compute" phase from "extract" phase

---

## Practical Recommendations

**For learning**: Start with Method 1 (naive), understand the recursion tree

**For small strings (< 100 chars)**: Use Method 3 (bottom-up)

**For medium strings (< 10,000 chars)**: Use Method 3 or 5 (if you need the string)

**For large strings (> 10,000 chars)**:
- Length only → Method 4 (space-optimized)
- Need string → Method 5, consider memory constraints

**For production**:
- Method 3 (bottom-up) for most cases
- Method 4 (space-optimized) if memory is critical
- Method 5 (reconstruct) for diff tools, alignment, etc.

---

## Related

- [[README]] - Mission 11 Tutorial overview
- [[../../zettelkasten/missions/mission-11]] - Mission 11 zettelkasten note
- [[../../zettelkasten/memoization-comprehensive-guide]] - Memoization patterns in Rust
- [[../../zettelkasten/Memoization MOC]] - Memoization knowledge map
- **Concepts**: Dynamic programming, memoization, space optimization, backtracking
- **Applications**: DNA sequence alignment, diff tools, version control
