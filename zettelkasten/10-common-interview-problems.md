# 10 Common Coding Interview Problems - Patterns and Solutions

*Tags: #coding-interviews #leetcode #algorithms #data-structures #problem-solving #competitive-programming*  
*Links: [[zettel-index]] | [[Algorithm Analysis]] | [[pattern-matching]] | [[Dynamic Programming]] | [[Graph Algorithms]] | [[String Algorithms]] | [[monotonic-stack]]*

---

## 🎯 Overview

This page outlines 10 classic coding interview problems frequently asked at FAANG companies. Each problem is framed with:
- **Problem statement** summary
- **Key algorithm/approach**
- **Time & space complexity**
- **Rust implementation pattern**
- **Key insights**

---

## 1️⃣ Valid Anagram

### **Problem**
Given two strings `s` and `t`, determine if `t` is an anagram of `s`. An anagram uses exactly the same characters, just rearranged.

```
Input: s = "listen", t = "silent"
Output: true

Input: s = "hello", t = "world"
Output: false
```

### **Approach Options**

**Option A: Sorting** (Simple but slower)
```rust
fn is_anagram(s: String, t: String) -> bool {
    let mut s_chars: Vec<char> = s.chars().collect();
    let mut t_chars: Vec<char> = t.chars().collect();
    
    s_chars.sort();
    t_chars.sort();
    
    s_chars == t_chars
}
```

**Option B: Character Frequency** (Better)
```rust
fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() { return false; }
    
    let mut freq = vec![0; 26];
    
    for (s_char, t_char) in s.chars().zip(t.chars()) {
        freq[(s_char as usize - 'a' as usize)] += 1;
        freq[(t_char as usize - 'a' as usize)] -= 1;
    }
    
    freq.iter().all(|&f| f == 0)
}
```

### **Complexity**
- **Time**: O(n) with frequency counting; O(n log n) with sorting
- **Space**: O(1) since max 26 letters

### **Key Insights**
- ✅ Character frequency counting solves this efficiently
- ✅ HashMap (or array for fixed alphabet) is ideal
- ✅ Early length check saves time

---

## 2️⃣ First and Last Index in Sorted Array

### **Problem**
Given a sorted array and a target value, find the first and last position of the target.

```
Input: nums = [5,7,7,8,8,10], target = 8
Output: [3, 4]

Input: nums = [5,7,7,8,8,10], target = 6
Output: [-1, -1]
```

### **Approach**

**Binary Search (Two Passes)**
```rust
fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let left = binary_search_left(nums.clone(), target);
    let right = binary_search_right(nums.clone(), target);
    vec![left, right]
}

fn binary_search_left(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() as i32 - 1;
    let mut result = -1;
    
    while left <= right {
        let mid = (left + right) / 2;
        if nums[mid as usize] == target {
            result = mid;
            right = mid - 1;  // Keep searching left
        } else if nums[mid as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    result
}

fn binary_search_right(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() as i32 - 1;
    let mut result = -1;
    
    while left <= right {
        let mid = (left + right) / 2;
        if nums[mid as usize] == target {
            result = mid;
            left = mid + 1;  // Keep searching right
        } else if nums[mid as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    result
}
```

### **Complexity**
- **Time**: O(log n) - two binary searches
- **Space**: O(1)

### **Key Insights**
- ✅ Binary search, but continue searching after finding target
- ✅ One pass finds first, another finds last
- ✅ Must handle duplicates properly

---

## 3️⃣ Kth Largest Element

### **Problem**
Find the kth largest element in an unsorted array.

```
Input: nums = [3,2,1,5,6,4], k = 2
Output: 5  (second largest)

Input: nums = [3,2,1,5,6,4], k = 5
Output: 2
```

### **Approach Options**

**Option A: Min Heap of Size K**
```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut min_heap = BinaryHeap::new();
    
    for num in nums {
        min_heap.push(Reverse(num));
        if min_heap.len() > k as usize {
            min_heap.pop();
        }
    }
    
    match min_heap.pop() {
        Some(Reverse(val)) => val,
        None => -1,
    }
}
```

**Option B: QuickSelect (Faster average case)**
```rust
fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    let target = nums.len() - k as usize;
    quickselect(&mut nums, 0, nums.len() - 1, target)
}

fn quickselect(nums: &mut Vec<i32>, left: usize, right: usize, target: usize) -> i32 {
    if left == right {
        return nums[left];
    }
    
    let pivot_index = partition(nums, left, right);
    
    match pivot_index.cmp(&target) {
        std::cmp::Ordering::Equal => nums[pivot_index],
        std::cmp::Ordering::Greater => quickselect(nums, left, pivot_index - 1, target),
        std::cmp::Ordering::Less => quickselect(nums, pivot_index + 1, right, target),
    }
}

fn partition(nums: &mut Vec<i32>, left: usize, right: usize) -> usize {
    let pivot = nums[right];
    let mut i = left;
    
    for j in left..right {
        if nums[j] > pivot {
            nums.swap(i, j);
            i += 1;
        }
    }
    nums.swap(i, right);
    i
}
```

### **Complexity**
- **Min Heap**: Time O(n log k), Space O(k)
- **QuickSelect**: Time O(n) average, O(n²) worst, Space O(1)

### **Key Insights**
- ✅ Min heap of size k is reliable O(n log k)
- ✅ QuickSelect is faster on average for random data
- ✅ HeapSort concept: maintain heap of k elements

---

## 4️⃣ Symmetric Tree

### **Problem**
Check if a binary tree is symmetric (mirror of itself).

```
    1
   / \
  2   2
 / \ / \
3  4 4  3

Output: true (symmetric)
```

### **Approach**

**Recursive Comparison**
```rust
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn is_symmetric(root: Option<Box<TreeNode>>) -> bool {
    match root {
        None => true,
        Some(node) => is_mirror(&node.left, &node.right),
    }
}

fn is_mirror(left: &Option<Box<TreeNode>>, right: &Option<Box<TreeNode>>) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(l), Some(r)) => {
            l.val == r.val 
                && is_mirror(&l.left, &r.right)
                && is_mirror(&l.right, &r.left)
        }
        _ => false,
    }
}
```

**Iterative with Queue**
```rust
use std::collections::VecDeque;

fn is_symmetric(root: Option<Box<TreeNode>>) -> bool {
    if let Some(node) = root {
        let mut queue = VecDeque::new();
        queue.push_back((node.left.clone(), node.right.clone()));
        
        while let Some((left, right)) = queue.pop_front() {
            match (left, right) {
                (None, None) => continue,
                (Some(l), Some(r)) if l.val == r.val => {
                    queue.push_back((l.left.clone(), r.right.clone()));
                    queue.push_back((l.right.clone(), r.left.clone()));
                }
                _ => return false,
            }
        }
        true
    } else {
        true
    }
}
```

### **Complexity**
- **Time**: O(n) - visit each node once
- **Space**: O(h) recursive, O(n) iterative (h = height)

### **Key Insights**
- ✅ Compare nodes recursively: compare val, cross-compare children
- ✅ Mirror means left.left == right.right AND left.right == right.left
- ✅ Tree traversal problem, good for recursion practice

---

## 5️⃣ Generate Parentheses

### **Problem**
Generate all combinations of well-formed parentheses with n pairs.

```
Input: n = 3
Output: ["((()))","(()())","(())()","()(())","()()()"]

Input: n = 1
Output: ["()"]
```

### **Approach**

**Backtracking**
```rust
fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    
    fn backtrack(
        result: &mut Vec<String>,
        current: &mut String,
        open: i32,
        close: i32,
        max: i32,
    ) {
        if close == max {
            result.push(current.clone());
            return;
        }
        
        if open < max {
            current.push('(');
            backtrack(result, current, open + 1, close, max);
            current.pop();
        }
        
        if close < open {
            current.push(')');
            backtrack(result, current, open, close + 1, max);
            current.pop();
        }
    }
    
    backtrack(&mut result, &mut current, 0, 0, n);
    result
}
```

### **Complexity**
- **Time**: O(4^n / √n) - Catalan number
- **Space**: O(n) recursion depth

### **Key Insights**
- ✅ Backtracking with constraints
- ✅ Only add '(' if open < n
- ✅ Only add ')' if close < open (valid parentheses constraint)
- ✅ Catalan number: C(n) = (2n)! / ((n+1)! * n!)

---

## 6️⃣ Gas Station

### **Problem**
Given gas and cost arrays, find starting gas station index to complete circuit.

```
Input: gas = [1,2,3,4,5], cost = [3,4,5,1,2]
Output: 3  (start at index 3)

Input: gas = [2,3,4], cost = [3,4,3]
Output: -1  (impossible)
```

### **Approach**

**Greedy Algorithm**
```rust
fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut tank = 0;
    let mut start = 0;
    let mut total = 0;
    
    for i in 0..gas.len() {
        tank += gas[i] - cost[i];
        total += gas[i] - cost[i];
        
        if tank < 0 {
            start = i + 1;
            tank = 0;
        }
    }
    
    if total >= 0 { start as i32 } else { -1 }
}
```

### **Complexity**
- **Time**: O(n) - single pass
- **Space**: O(1)

### **Key Insights**
- ✅ Greedy approach: if tank goes negative, restart from next station
- ✅ Track total fuel difference to check if solution exists
- ✅ If no solution exists, total will be negative
- ✅ Only one valid starting point (if solution exists)

---

## 7️⃣ Course Schedule

### **Problem**
Determine if you can finish all courses given prerequisites. Prerequisites form edges in a graph.

```
Input: numCourses = 2, prerequisites = [[1,0]]
Output: true  (take course 0 first, then 1)

Input: numCourses = 2, prerequisites = [[1,0],[0,1]]
Output: false  (circular dependency)
```

### **Approach**

**Topological Sort with DFS**
```rust
fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut graph = vec![vec![]; num_courses as usize];
    
    for prereq in prerequisites {
        graph[prereq[0] as usize].push(prereq[1] as usize);
    }
    
    let mut state = vec![0; num_courses as usize]; // 0: unvisited, 1: visiting, 2: visited
    
    fn has_cycle(node: usize, graph: &[Vec<usize>], state: &mut [i32]) -> bool {
        if state[node] == 1 {
            return true; // Cycle detected
        }
        if state[node] == 2 {
            return false; // Already processed
        }
        
        state[node] = 1; // Mark as visiting
        
        for &neighbor in &graph[node] {
            if has_cycle(neighbor, graph, state) {
                return true;
            }
        }
        
        state[node] = 2; // Mark as visited
        false
    }
    
    for i in 0..num_courses {
        if state[i as usize] == 0 && has_cycle(i as usize, &graph, &mut state) {
            return false;
        }
    }
    true
}
```

**Topological Sort with BFS (Kahn's Algorithm)**
```rust
fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut graph = vec![vec![]; num_courses as usize];
    let mut in_degree = vec![0; num_courses as usize];
    
    for prereq in prerequisites {
        graph[prereq[1] as usize].push(prereq[0] as usize);
        in_degree[prereq[0] as usize] += 1;
    }
    
    let mut queue: std::collections::VecDeque<usize> = in_degree
        .iter()
        .enumerate()
        .filter(|(_, &d)| d == 0)
        .map(|(i, _)| i)
        .collect();
    
    let mut count = 0;
    
    while let Some(node) = queue.pop_front() {
        count += 1;
        
        for &neighbor in &graph[node] {
            in_degree[neighbor] -= 1;
            if in_degree[neighbor] == 0 {
                queue.push_back(neighbor);
            }
        }
    }
    
    count == num_courses
}
```

### **Complexity**
- **Time**: O(V + E) - graph traversal
- **Space**: O(V + E) - graph storage

### **Key Insights**
- ✅ Detect cycle in directed graph
- ✅ Three state DFS: unvisited, visiting, visited
- ✅ Topological sort: Kahn's algorithm with in-degrees
- ✅ If all nodes processed without cycle, schedule possible

---

## 8️⃣ Kth Permutation

### **Problem**
Find the kth permutation of n numbers (1 to n) in lexicographic order.

```
Input: n = 3, k = 3
Output: "213"

Sequence: "123", "132", "213", "231", "312", "321"
         (1st,  2nd,  3rd,  4th,  5th,  6th)
```

### **Approach**

**Math-Based (No Generation)**
```rust
fn get_permutation(n: i32, k: i32) -> String {
    let mut factorial = vec![1; n as usize];
    for i in 1..n as usize {
        factorial[i] = factorial[i - 1] * i as i32;
    }
    
    let mut numbers: Vec<i32> = (1..=n).collect();
    let mut result = String::new();
    let mut k = k - 1; // Convert to 0-indexed
    
    for i in (0..n as usize).rev() {
        let index = (k / factorial[i]) as usize;
        result.push_str(&numbers[index].to_string());
        numbers.remove(index);
        k %= factorial[i];
    }
    
    result
}
```

### **Complexity**
- **Time**: O(n²) - O(n) digits, each remove is O(n)
- **Space**: O(n)

### **Key Insights**
- ✅ Use factorial number system to calculate directly
- ✅ For each position, determine which number from remaining
- ✅ k is divided by factorial of remaining numbers
- ✅ Much faster than generating all permutations: O(n²) vs O(n! * n)

---

## 9️⃣ Minimum Window Substring

### **Problem**
Find minimum window substring containing all characters of target string.

```
Input: s = "ADOBECODEBANC", t = "ABC"
Output: "BANC"

Input: s = "a", t = "aa"
Output: ""  (impossible)
```

### **Approach**

**Sliding Window with Frequency Maps**
```rust
use std::collections::HashMap;

fn min_window(s: String, t: String) -> String {
    if t.len() > s.len() {
        return String::new();
    }
    
    let mut char_count = HashMap::new();
    for ch in t.chars() {
        *char_count.entry(ch).or_insert(0) += 1;
    }
    
    let mut required = char_count.len();
    let mut formed = 0;
    let mut window_counts = HashMap::new();
    
    let mut left = 0;
    let mut min_len = usize::MAX;
    let mut min_left = 0;
    
    for (right, ch) in s.chars().enumerate() {
        *window_counts.entry(ch).or_insert(0) += 1;
        
        if let Some(&count) = char_count.get(&ch) {
            if window_counts[&ch] == count {
                formed += 1;
            }
        }
        
        while formed == required && left <= right {
            let window_len = right - left + 1;
            
            if window_len < min_len {
                min_len = window_len;
                min_left = left;
            }
            
            let left_char = s.chars().nth(left).unwrap();
            *window_counts.get_mut(&left_char).unwrap() -= 1;
            
            if let Some(&count) = char_count.get(&left_char) {
                if window_counts[&left_char] < count {
                    formed -= 1;
                }
            }
            
            left += 1;
        }
    }
    
    if min_len == usize::MAX {
        String::new()
    } else {
        s.chars().skip(min_left).take(min_len).collect()
    }
}
```

### **Complexity**
- **Time**: O(|s| + |t|) - each character visited at most twice
- **Space**: O(|t|) - character count maps

### **Key Insights**
- ✅ Sliding window with two pointers
- ✅ Expand right pointer, contract left pointer
- ✅ Track character frequency requirements
- ✅ `formed` counter tracks how many unique chars have required count

---

## 🔟 Largest Rectangle in Histogram

### **Problem**
Find the largest rectangle area in histogram where bars have varying heights.

```
Input: heights = [2,1,5,6,2,3]
Output: 10  (rectangle from index 2-3, height 5)

Visual:
  █
  █ █
  █ █
█ █ █ █
█ █ █ █ █ █
[2,1,5,6,2,3]

Rectangle: 2×5 = 10 or 2×6 = 12? No, it's limited by minimum height
```

### **Approach**

**Monotonic Stack (Optimal)** - See [[monotonic-stack]] for detailed pattern explanation
```rust
fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut stack = Vec::new();
    let mut max_area = 0;
    let mut i = 0;
    let heights: Vec<i32> = [vec![0], heights, vec![0]].concat(); // Add sentinels
    
    while i < heights.len() {
        if stack.is_empty() || heights[i] >= heights[*stack.last().unwrap()] {
            stack.push(i);
            i += 1;
        } else {
            let h_index = stack.pop().unwrap();
            let height = heights[h_index] as i32;
            let width = if stack.is_empty() {
                i as i32
            } else {
                (i - stack.last().unwrap() - 1) as i32
            };
            let area = height * width;
            max_area = max_area.max(area);
        }
    }
    
    max_area
}
```

### **Complexity**
- **Time**: O(n) - each bar pushed/popped once
- **Space**: O(n) - stack storage

### **Key Insights**
- ✅ Monotonic stack maintains increasing heights
- ✅ When height drops, calculate area for all bars that can't extend further
- ✅ Add sentinels (0s) at start and end to simplify logic
- ✅ For each bar, width = distance between previous and next smaller bar

---

## 📊 Problem Categories & Patterns

### **By Algorithm Type**

| Problem | Category | Pattern |
|---------|----------|---------|
| 1. Anagram | String/Hash | Frequency counting |
| 2. First/Last Index | Search | Binary search variation |
| 3. Kth Largest | Heap/Selection | Min heap, QuickSelect |
| 4. Symmetric Tree | Tree/Graph | Tree traversal |
| 5. Generate Parentheses | Backtracking | DFS with constraints |
| 6. Gas Station | Greedy | One-pass greedy |
| 7. Course Schedule | Graph | Cycle detection |
| 8. Kth Permutation | Math/Combinatorics | Factorial number system |
| 9. Min Window | Sliding Window | Two pointers |
| 10. Largest Rectangle | Stack | Monotonic stack |

### **By Data Structure**

| Structure | Problems | Key Technique |
|-----------|----------|---------------|
| String | 1, 9 | Frequency, sliding window |
| Array | 2, 3, 10 | Binary search, heap, stack |
| Tree | 4 | Recursion, traversal |
| Graph | 7 | DFS, topological sort |
| Hash Map | 1, 9 | Frequency tracking |

---

## 🎓 Interview Preparation Tips

### **Pattern Recognition**
- ✅ **String/Character**: Think hash map, frequency counting
- ✅ **Array Search**: Think binary search
- ✅ **Top K**: Think heap
- ✅ **Permutations/Combinations**: Think backtracking or math
- ✅ **Intervals/Ranges**: Think greedy or dynamic programming
- ✅ **Dependencies**: Think topological sort
- ✅ **Max/Min Subarrays**: Think stack or dynamic programming

### **Complexity Targets**
- ✅ O(n log n): Sorting-based, binary search
- ✅ O(n): Optimal for most problems, often sliding window
- ✅ O(n²): Nested loops, not ideal but sometimes necessary
- ✅ O(2^n): Exponential, backtracking, only for small n

### **Implementation Tips**
1. **Clarify constraints** before coding
2. **Start with brute force**, then optimize
3. **Test edge cases**: empty, single element, all same
4. **Verify complexity** matches expected
5. **Walk through example** while coding

---

## 🔗 Cross-References

### **In This Workspace**
- **Pattern Matching**: Problem 5 (Generate Parentheses)
- **Algorithm Analysis**: All problems
- **Data Structures**: Specific to each problem
- **Graph Algorithms**: Problem 7 (Course Schedule)
- **Competitive Programming**: AoC integration

### **Related Concepts**
- **[[Dynamic Programming]]** - Course schedule, Kth permutation
- **[[Tree Algorithms]]** - Symmetric tree
- **[[Graph Theory]]** - Course schedule, cycle detection
- **[[Greedy Algorithms]]** - Gas station
- **[[String Algorithms]]** - Anagram, minimum window
- **[[Stack & Queue]]** - Largest rectangle, parentheses
- **[[monotonic-stack]]** - Detailed pattern for Problem #10

---

## ✅ Study Checklist

- [ ] Understand each problem statement clearly
- [ ] Identify the algorithm/pattern for each
- [ ] Code solution in Rust (or your language)
- [ ] Test with provided examples
- [ ] Identify time & space complexity
- [ ] Think of edge cases
- [ ] Practice explaining solution verbally
- [ ] Time yourself (15-20 min per problem target)

---

## 📚 Additional Resources

### **LeetCode Problem Links**
1. Valid Anagram - LeetCode 242
2. First and Last Index - LeetCode 34
3. Kth Largest - LeetCode 215
4. Symmetric Tree - LeetCode 101
5. Generate Parentheses - LeetCode 22
6. Gas Station - LeetCode 134
7. Course Schedule - LeetCode 207
8. Kth Permutation - LeetCode 60
9. Minimum Window Substring - LeetCode 76
10. Largest Rectangle - LeetCode 84

### **Study Resources**
- **NeetCode.io**: Video walkthroughs for all problems
- **Rust Playground**: Test implementations
- **LeetCode Discuss**: Learn from community solutions

---

**Status**: ✅ Active Interview Prep Resource  
**Last Updated**: October 20, 2025  
**Difficulty**: Medium → Hard  
**Interview Level**: FAANG Interviews

*Tags: #coding-interviews #leetcode #algorithms #data-structures #problem-solving #competitive-programming #interview-prep #rust*

*Navigation: [[zettel-index]] | [[Algorithm Analysis]] | [[pattern-matching]] | [[Dynamic Programming]] | [[Graph Algorithms]]*

---

*Based on popular coding interview question video compilation*
*Each problem includes approach, complexity analysis, and Rust implementation patterns*
