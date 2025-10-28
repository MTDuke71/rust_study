# Monotonic Stack Pattern

*Tags: #data-structures #stack #algorithms #pattern #competitive-programming #optimization*  
*Links: [[zettel-index]] | [[10-common-interview-problems]] | [[Stack & Queue]] | [[Largest Rectangle in Histogram]] | [[pattern-matching]] | [[Algorithm Analysis]]*

---

## 🎯 Overview

A **Monotonic Stack** is a stack where elements are maintained in a strictly increasing or decreasing order. It's a powerful technique for solving problems that require finding the next/previous greater/smaller element, or identifying spanning areas efficiently.

**Key Property**: Elements in the stack form a monotonic sequence (increasing or decreasing order).

**Common Use Cases**:
- Next/Previous Greater/Smaller Element
- Largest Rectangle in Histogram
- Trapping Rain Water
- Daily Temperatures
- Stock Span Problem
- Remove K Digits to Make Smallest Number

---

## 🔍 Core Concepts

### **Monotonic Increasing Stack**
Maintains elements in **strictly increasing** order from bottom to top.

```
Initial: Stack = []

Add 1: Stack = [1]
Add 2: Stack = [1, 2]
Add 1: Stack = [1]        // Pop 2 because 1 < 2, then pop 1 because 1 ≤ 1
```

### **Monotonic Decreasing Stack**
Maintains elements in **strictly decreasing** order from bottom to top.

```
Initial: Stack = []

Add 5: Stack = [5]
Add 3: Stack = [5, 3]
Add 4: Stack = [5, 4]     // Pop 3 because 4 > 3, then add 4
```

### **Why Monotonic Stacks?**

| Benefit | Impact | Example |
|---------|--------|---------|
| **O(n) Complexity** | Each element pushed/popped once | Linear vs O(n²) brute force |
| **Pattern Recognition** | Identify boundaries efficiently | Next greater element |
| **Space Efficiency** | Only store relevant elements | Don't need all comparisons |
| **Single Pass** | Process input once | Time-optimal for streaming data |

---

## 💡 Common Patterns

### **Pattern 1: Next Greater Element**

**Problem**: For each element, find the next element to the right that's greater.

```
Input:  [1, 3, 2, 4]
Output: [3, 4, 4, -1]
                    ↑
            next greater for each
```

**Algorithm**:
1. Use **decreasing monotonic stack** to find next greater
2. When you find a larger element, it's the "next greater" for all smaller elements you pop
3. Elements popped from stack found their answer

**Rust Implementation**:
```rust
fn next_greater_element(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![-1; n];
    let mut stack: Vec<usize> = Vec::new();
    
    for i in 0..n {
        // Pop elements while current is greater
        while !stack.is_empty() && nums[*stack.last().unwrap()] < nums[i] {
            let idx = stack.pop().unwrap();
            result[idx] = nums[i];  // Found next greater
        }
        stack.push(i);
    }
    
    result
}

// Example:
// Input: [1, 3, 2, 4]
// Output: [3, 4, 4, -1] ✓
```

**Complexity**: 
- **Time**: O(n) - each element pushed/popped once
- **Space**: O(n) - stack storage

---

### **Pattern 2: Largest Rectangle in Histogram**

**Problem**: Given histogram bars with varying heights, find the largest rectangular area.

```
Heights: [2, 1, 5, 6, 2, 3]

  █
  █ █
  █ █
█ █ █ █
█ █ █ █ █ █

Answer: 10 (width 2, height 5, or other combinations)
```

**Algorithm**:
1. Use **increasing monotonic stack** to store indices
2. When height decreases, pop from stack and calculate areas
3. For each popped bar, it becomes the height; width is determined by boundary indices

**Rust Implementation**:
```rust
fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut stack: Vec<usize> = Vec::new();
    let mut max_area = 0;
    let mut i = 0;
    let heights = [vec![0], heights, vec![0]].concat();  // Add sentinels
    
    while i < heights.len() {
        if stack.is_empty() || heights[i] >= heights[*stack.last().unwrap()] {
            stack.push(i);
            i += 1;
        } else {
            let h_idx = stack.pop().unwrap();
            let height = heights[h_idx] as i32;
            let width = if stack.is_empty() {
                i as i32
            } else {
                (i - stack.last().unwrap() - 1) as i32
            };
            max_area = max_area.max(height * width);
        }
    }
    
    max_area
}

// Example:
// Input: [2, 1, 5, 6, 2, 3]
// Output: 10 ✓
```

**Complexity**:
- **Time**: O(n) - each bar pushed/popped once
- **Space**: O(n) - stack storage

---

### **Pattern 3: Daily Temperatures**

**Problem**: Given daily temperatures, return how many days to warmer temperature.

```
Input:  [73, 74, 75, 71, 69, 72, 76, 73]
Output: [ 1,  1,  4,  2,  1,  1,  0,  0]
         ↑   ↑   ↑   ↑   ↑
    days until warmer
```

**Algorithm**:
1. Use **decreasing monotonic stack** (store temperatures/indices)
2. For each new temperature, check if it's warmer than stack top
3. When warmer found, calculate days elapsed

**Rust Implementation**:
```rust
fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let n = temperatures.len();
    let mut result = vec![0; n];
    let mut stack: Vec<(i32, usize)> = Vec::new();  // (temperature, index)
    
    for (i, &temp) in temperatures.iter().enumerate() {
        while !stack.is_empty() && stack.last().unwrap().0 < temp {
            let (_, prev_idx) = stack.pop().unwrap();
            result[prev_idx] = (i - prev_idx) as i32;
        }
        stack.push((temp, i));
    }
    
    result
}

// Example:
// Input: [73, 74, 75, 71, 69, 72, 76, 73]
// Output: [1, 1, 4, 2, 1, 1, 0, 0] ✓
```

**Complexity**:
- **Time**: O(n) - each temperature processed once
- **Space**: O(n) - stack storage

---

### **Pattern 4: Trapping Rain Water**

**Problem**: Given elevation map, calculate how much rain water can be trapped.

```
Elevation: [0,1,0,2,1,0,1,3,2,1,2,1]

After rain: 
    |
  | █ | █
  | █ █ █ | █ █ █
█ █ █ █ █ █ █ █ █ █ █ █

Water trapped = 6 units
```

**Algorithm**:
1. Use **decreasing monotonic stack** (store heights/indices)
2. When height increases, calculate trapped water between popped element and current
3. Water depth determined by min of left and right boundaries

**Rust Implementation**:
```rust
fn trap(height: Vec<i32>) -> i32 {
    let mut stack: Vec<usize> = Vec::new();
    let mut water = 0;
    
    for i in 0..height.len() {
        while !stack.is_empty() && height[i] > height[*stack.last().unwrap()] {
            let top = stack.pop().unwrap();
            
            if stack.is_empty() {
                break;
            }
            
            let left = *stack.last().unwrap();
            let h = (height[left].min(height[i]) - height[top]) as i32;
            let w = (i - left - 1) as i32;
            water += h * w;
        }
        
        stack.push(i);
    }
    
    water
}

// Example:
// Input: [0,1,0,2,1,0,1,3,2,1,2,1]
// Output: 6 ✓
```

**Complexity**:
- **Time**: O(n) - each bar processed once
- **Space**: O(n) - stack storage

---

### **Pattern 5: Remove K Digits to Make Smallest Number**

**Problem**: Remove k digits to get the smallest possible number.

```
Input: num = "1432219", k = 3
Output: "1219"

Explanation:
- Remove 4, 3, 2 to get 1219
- Could also remove 3, 2, 2 to get 1419 (larger)
```

**Algorithm**:
1. Use **increasing monotonic stack** 
2. Scan left to right, remove larger digits when smaller digit found
3. If removals remain, remove from end

**Rust Implementation**:
```rust
fn remove_k_digits(num: String, k: i32) -> String {
    let mut stack: Vec<char> = Vec::new();
    let mut k = k;
    
    // Build monotonic stack
    for ch in num.chars() {
        while !stack.is_empty() && k > 0 && stack.last().unwrap() > &ch {
            stack.pop();
            k -= 1;
        }
        stack.push(ch);
    }
    
    // Remove remaining from end if needed
    while k > 0 {
        stack.pop();
        k -= 1;
    }
    
    // Convert to string and remove leading zeros
    let result: String = stack.iter().collect();
    let trimmed = result.trim_start_matches('0');
    
    if trimmed.is_empty() {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}

// Example:
// Input: num = "1432219", k = 3
// Output: "1219" ✓
```

**Complexity**:
- **Time**: O(n) - each digit processed once
- **Space**: O(n) - stack storage

---

## 🎓 Implementation Template

### **Generic Monotonic Stack Template**

```rust
/// Generic template for monotonic stack problems
/// Stack maintains indices for accessing original values
fn monotonic_stack_template(values: Vec<i32>, stack_type: &str) -> Vec<i32> {
    let mut result = vec![-1; values.len()];
    let mut stack: Vec<usize> = Vec::new();
    
    for i in 0..values.len() {
        // Pop based on monotonic property
        match stack_type {
            "increasing" => {
                // Maintain increasing: pop if current >= stack top
                while !stack.is_empty() && values[i] > values[*stack.last().unwrap()] {
                    let idx = stack.pop().unwrap();
                    result[idx] = values[i];  // Found answer
                }
            }
            "decreasing" => {
                // Maintain decreasing: pop if current <= stack top
                while !stack.is_empty() && values[i] < values[*stack.last().unwrap()] {
                    let idx = stack.pop().unwrap();
                    result[idx] = values[i];  // Found answer
                }
            }
            _ => {}
        }
        
        stack.push(i);
    }
    
    result
}
```

### **Decision Tree: When to Use Monotonic Stack**

```
Is your problem about finding:
│
├─ Next/Previous Greater Element?        → Decreasing Stack
│
├─ Next/Previous Smaller Element?        → Increasing Stack
│
├─ Largest Rectangle/Area?               → Increasing Stack (boundaries)
│
├─ Trapping Water/Fill?                  → Decreasing Stack (depth)
│
├─ Remove K Elements to Optimize?        → Increasing Stack (remove larger)
│
└─ Span Problem (consecutive equal)?     → Decreasing Stack (equal allowed)
```

---

## 🔗 Real-World Applications

### **1. Stock Market Analysis**
- **Problem**: When will a stock price go higher than today?
- **Solution**: Monotonic stack to find next higher closing price
- **Impact**: O(n) instead of O(n²) brute force

### **2. Building Skyline**
- **Problem**: Given buildings with height and position, trace the skyline
- **Solution**: Monotonic stack maintains active building heights
- **Impact**: Efficient boundary detection

### **3. Data Stream Analysis**
- **Problem**: Process streaming sensor data for anomalies
- **Solution**: Monotonic stack finds spikes/drops in real-time
- **Impact**: Single pass, constant space per decision

### **4. Graphics Rendering**
- **Problem**: Rasterize polygon edges efficiently
- **Solution**: Monotonic stack for edge scanning
- **Impact**: Used in OpenGL and game engines

### **5. Compiler Design**
- **Problem**: Expression parsing and optimization
- **Solution**: Monotonic stack for operator precedence
- **Impact**: Efficient AST construction

---

## 📊 Complexity Analysis

| Pattern | Time | Space | Use When |
|---------|------|-------|----------|
| Next Greater | O(n) | O(n) | Finding next/prev extremes |
| Largest Rectangle | O(n) | O(n) | Spanning area problems |
| Trapping Water | O(n) | O(n) | Gap-filling scenarios |
| Daily Temperatures | O(n) | O(n) | Sequence analysis |
| Remove K Digits | O(n) | O(n) | Greedy optimization |

**vs. Brute Force Alternatives**:
- Brute force scanning: **O(n²)** → Monotonic stack: **O(n)** ✅
- Space trade-off: Small constant space overhead for massive time gain

---

## ⚠️ Common Pitfalls

### **Pitfall 1: Wrong Monotonic Direction**
```rust
// ❌ WRONG - Using increasing stack for "next greater"
while !stack.is_empty() && nums[i] < nums[*stack.last().unwrap()] {
    // This pops when current is SMALLER, not when we find greater
}

// ✅ CORRECT - Using decreasing stack
while !stack.is_empty() && nums[i] > nums[*stack.last().unwrap()] {
    let idx = stack.pop().unwrap();
    result[idx] = nums[i];  // Now we found next greater
}
```

### **Pitfall 2: Forgetting Boundary Checks**
```rust
// ❌ WRONG - Doesn't handle non-existent answers
fn next_greater(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; nums.len()];  // Wrong default!
    // ...
}

// ✅ CORRECT - Initialize to -1 or default answer
fn next_greater(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![-1; nums.len()];
    // ...
}
```

### **Pitfall 3: Index vs Value Confusion**
```rust
// ❌ WRONG - Storing values when you need indices
let mut stack: Vec<i32> = Vec::new();  // Values only
stack.push(nums[i]);
// Now you can't track "which element found its answer"

// ✅ CORRECT - Store indices to access values and track positions
let mut stack: Vec<usize> = Vec::new();
stack.push(i);
result[stack.pop().unwrap()] = nums[i];  // Know which position found answer
```

### **Pitfall 4: Sentinel Values**
```rust
// ❌ WRONG - Histogram problem without sentinels
fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    // Risky when stack becomes empty
    // ...
}

// ✅ CORRECT - Add sentinels to prevent boundary errors
let heights = [vec![0], heights, vec![0]].concat();
// Now stack never fully empties prematurely
```

---

## 🧪 Testing Strategy

### **Test Case Categories**

**1. Basic Cases**
```rust
#[test]
fn test_next_greater_basic() {
    assert_eq!(next_greater_element(vec![1, 3, 2, 4]), vec![3, 4, 4, -1]);
}

#[test]
fn test_histogram_basic() {
    assert_eq!(largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
}
```

**2. Edge Cases**
```rust
#[test]
fn test_next_greater_empty() {
    assert_eq!(next_greater_element(vec![]), vec![]);
}

#[test]
fn test_histogram_single() {
    assert_eq!(largest_rectangle_area(vec![5]), 5);
}

#[test]
fn test_histogram_increasing() {
    assert_eq!(largest_rectangle_area(vec![1, 2, 3, 4, 5]), 5);
}

#[test]
fn test_histogram_decreasing() {
    assert_eq!(largest_rectangle_area(vec![5, 4, 3, 2, 1]), 5);
}
```

**3. Complex Cases**
```rust
#[test]
fn test_next_greater_all_decreasing() {
    assert_eq!(next_greater_element(vec![5, 4, 3, 2, 1]), vec![-1, -1, -1, -1, -1]);
}

#[test]
fn test_histogram_mixed_pattern() {
    assert_eq!(largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
}
```

---

## 🎯 Interview Tips

### **When Asked About Monotonic Stack**

1. **Clarify the Problem**:
   - "Are we looking for next greater OR next smaller?"
   - "Do we need indices OR just values?"
   - "Are there duplicate values?"

2. **Communicate Your Approach**:
   - "I'll use a monotonic stack to maintain elements in order"
   - "When we find the triggering element, we can determine answers for all popped elements"
   - "This gives us O(n) time because each element is pushed and popped once"

3. **Implementation Order**:
   - Start with the comparison logic (greater vs smaller)
   - Add the result collection
   - Handle boundary cases (empty input, no answer)
   - Add sentinels if needed

4. **Verify Your Solution**:
   - Walk through example while building stack
   - Show which element found its answer at each pop
   - Verify time complexity claim (linear pass, each element touched once)

---

## 🔗 Cross-References

### **Related Zettelkasten Pages**
- **[[Stack & Queue]]** - Fundamental data structure theory
- **[[pattern-matching]]** - Pattern recognition in algorithms
- **[[Algorithm Analysis]]** - Complexity analysis framework
- **[[Largest Rectangle in Histogram]]** - Detailed explanation of Problem #10
- **[[Daily Temperatures]]** - Temperature sequence problem
- **[[Trapping Rain Water]]** - Water accumulation pattern

### **Workspace Integration**
- **[[10-common-interview-problems]]** - Problem #10 uses monotonic stack
- **[[missions/Mission1 Overview|Mission1]]** - Stack fundamentals
- **[[mission-2]]** - Queue and sequence problems
- **[[daily-study/Day-??]]** - Stack pattern practice

### **LeetCode Problems**
- LeetCode 496: Next Greater Element I
- LeetCode 503: Next Greater Element II (circular)
- LeetCode 84: Largest Rectangle in Histogram
- LeetCode 739: Daily Temperatures
- LeetCode 42: Trapping Rain Water
- LeetCode 402: Remove K Digits to Make Smallest Number

---

## ✅ Learning Checklist

- [ ] Understand monotonic increasing vs decreasing stacks
- [ ] Recognize next greater/smaller element problems
- [ ] Implement next greater element solution
- [ ] Understand largest rectangle in histogram algorithm
- [ ] Walk through histogram example step-by-step
- [ ] Identify when to use sentinels
- [ ] Solve daily temperatures problem
- [ ] Understand trapping rain water concept
- [ ] Practice on LeetCode similar problems
- [ ] Explain monotonic stack concept verbally

---

## 📚 Complete Working Example

```rust
/// Complete monotonic stack demo with multiple patterns
fn main() {
    println!("=== Monotonic Stack Patterns ===\n");
    
    // Pattern 1: Next Greater Element
    let nums = vec![1, 3, 2, 4];
    let result = next_greater_element(nums.clone());
    println!("Next Greater Element:");
    println!("  Input:  {:?}", nums);
    println!("  Output: {:?}\n", result);
    
    // Pattern 2: Largest Rectangle
    let heights = vec![2, 1, 5, 6, 2, 3];
    let area = largest_rectangle_area(heights.clone());
    println!("Largest Rectangle in Histogram:");
    println!("  Input:  {:?}", heights);
    println!("  Output: {}\n", area);
    
    // Pattern 3: Daily Temperatures
    let temps = vec![73, 74, 75, 71, 69, 72, 76, 73];
    let days = daily_temperatures(temps.clone());
    println!("Daily Temperatures:");
    println!("  Input:  {:?}", temps);
    println!("  Output: {:?}\n", days);
}

fn next_greater_element(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![-1; n];
    let mut stack: Vec<usize> = Vec::new();
    
    for i in 0..n {
        while !stack.is_empty() && nums[*stack.last().unwrap()] < nums[i] {
            let idx = stack.pop().unwrap();
            result[idx] = nums[i];
        }
        stack.push(i);
    }
    result
}

fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut stack: Vec<usize> = Vec::new();
    let mut max_area = 0;
    let mut i = 0;
    let heights = [vec![0], heights, vec![0]].concat();
    
    while i < heights.len() {
        if stack.is_empty() || heights[i] >= heights[*stack.last().unwrap()] {
            stack.push(i);
            i += 1;
        } else {
            let h_idx = stack.pop().unwrap();
            let height = heights[h_idx] as i32;
            let width = if stack.is_empty() {
                i as i32
            } else {
                (i - stack.last().unwrap() - 1) as i32
            };
            max_area = max_area.max(height * width);
        }
    }
    max_area
}

fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let n = temperatures.len();
    let mut result = vec![0; n];
    let mut stack: Vec<(i32, usize)> = Vec::new();
    
    for (i, &temp) in temperatures.iter().enumerate() {
        while !stack.is_empty() && stack.last().unwrap().0 < temp {
            let (_, prev_idx) = stack.pop().unwrap();
            result[prev_idx] = (i - prev_idx) as i32;
        }
        stack.push((temp, i));
    }
    result
}
```

**Output**:
```
=== Monotonic Stack Patterns ===

Next Greater Element:
  Input:  [1, 3, 2, 4]
  Output: [3, 4, 4, -1]

Largest Rectangle in Histogram:
  Input:  [2, 1, 5, 6, 2, 3]
  Output: 10

Daily Temperatures:
  Input:  [73, 74, 75, 71, 69, 72, 76, 73]
  Output: [1, 1, 4, 2, 1, 1, 0, 0]
```

---

**Status**: ✅ Comprehensive Learning Resource  
**Last Updated**: October 20, 2025  
**Difficulty**: Medium → Hard  
**Real-World Usage**: High (system design, stream processing, graphics)

*Tags: #data-structures #stack #algorithms #pattern #competitive-programming #optimization #interview-prep #monotonic-stack*

*Navigation: [[zettel-index]] | [[10-common-interview-problems]] | [[Stack & Queue]] | [[Algorithm Analysis]] | [[pattern-matching]]*

---

*Comprehensive guide covering all major monotonic stack patterns with implementations, complexity analysis, and real-world applications.*
