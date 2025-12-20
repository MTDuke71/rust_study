# Binary Search

*Foundational divide-and-conquer algorithm for efficient searching in sorted data with O(log n) time complexity.*

---

## 🎯 **Core Concept**

Binary search is a **divide-and-conquer** algorithm that finds an element in a sorted array by repeatedly dividing the search space in half. Instead of checking every element (linear search O(n)), it eliminates half the remaining elements with each comparison.

**Key Insight:** If data is sorted, you can determine which half contains your target by comparing to the middle element.

### 📖 **The Phone Book Analogy (CS50)**

Imagine you're looking up "Smith, John" in a physical phone book:

**❌ Linear Search (Slow):**
- Start at page 1
- Flip through one page at a time
- Eventually find Smith... after checking hundreds of pages
- Time: O(n) - proportional to number of pages

**🤔 Slightly Better:**
- Flip through 2 pages at a time
- Still slow, just 2x faster
- Still O(n) - just with a smaller constant

**✅ Binary Search (Fast):**
1. Open to the **middle** of the book
2. You land on "M" - Smith comes **after** M
3. **Tear the book in half**, throw away the first half (A-M)
4. Open to the middle of remaining half (N-Z)
5. You land on "T" - Smith comes **before** T
6. **Tear again**, throw away T-Z section
7. Open to middle of N-S section
8. Repeat until you find Smith!

**Result:** Found in ~log₂(1000) ≈ 10 steps instead of 500+ steps!

**Why It Works:** Each step eliminates half the remaining pages. You can't do this with an unsorted list (imagine a phone book with random names - you'd have to check every page).

## 📐 **Algorithm Overview**

### **Basic Strategy**

1. **Compare** target with middle element
2. **If equal**: Found! Return the index
3. **If target < middle**: Search left half (discard right)
4. **If target > middle**: Search right half (discard left)
5. **Repeat** until found or search space is empty

### **Classic Implementation (Rust)**

```rust
fn binary_search<T: Ord>(arr: &[T], target: &T) -> Result<usize, usize> {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        // Avoid overflow: left + (right - left) / 2
        let mid = left + (right - left) / 2;
        
        match arr[mid].cmp(target) {
            std::cmp::Ordering::Equal => return Ok(mid),
            std::cmp::Ordering::Greater => right = mid,  // Search left half
            std::cmp::Ordering::Less => left = mid + 1,  // Search right half
        }
    }
    
    Err(left)  // Not found; left is insertion point
}
```

**Example Execution:**

```rust
let data = [1, 3, 5, 7, 9, 11, 13, 15];
// Search for 7:
// Step 1: mid=4 (9), 7 < 9 → search left [1,3,5,7]
// Step 2: mid=2 (5), 7 > 5 → search right [7]
// Step 3: mid=3 (7), 7 == 7 → found at index 3!
```

## ⏱️ **Complexity Analysis**

### **Time Complexity: O(log n)**

Each iteration cuts the search space in half:
- n elements → n/2 → n/4 → n/8 → ... → 1
- Number of steps: log₂(n)

**Concrete Examples:**
- 1,000 elements: ~10 comparisons
- 1,000,000 elements: ~20 comparisons
- 1,000,000,000 elements: ~30 comparisons

**Comparison to Linear Search:**
| Array Size | Linear (O(n)) | Binary (O(log n)) | Speedup |
|-----------|---------------|-------------------|---------|
| 100       | 50 avg        | 7                 | 7x      |
| 10,000    | 5,000 avg     | 14                | 357x    |
| 1,000,000 | 500,000 avg   | 20                | 25,000x |

### **Space Complexity: O(1)**

Iterative implementation uses constant extra space (just `left`, `right`, `mid` variables).

**Note:** Recursive implementations use O(log n) stack space.

## ✅ **Preconditions (CRITICAL)**

Binary search **ONLY works on sorted data**. Using it on unsorted data produces undefined results.

```rust
let sorted = [1, 3, 5, 7, 9];      // ✅ Works
let unsorted = [3, 1, 9, 5, 7];    // ❌ WRONG - undefined behavior

sorted.binary_search(&5);     // Ok(2) - correct
unsorted.binary_search(&5);   // Err(_) - unpredictable!
```

**Cost Analysis:**
- Sorting: O(n log n) one-time cost
- Many searches: O(log n) each
- **Breakeven:** ~2+ searches make sorting worthwhile

## 🔄 **Algorithm Variants**

### **1. Standard Binary Search (Exact Match)**

```rust
// Returns Ok(index) if found, Err(insertion_point) if not
arr.binary_search(&target)
```

### **2. Left Bound (Lower Bound)**

Find **first position** where element >= target:

```rust
fn search_left_bound<T: Ord>(arr: &[T], target: &T) -> usize {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] < *target {
            left = mid + 1;
        } else {
            right = mid;  // Could be answer, keep looking left
        }
    }
    left
}
```

**Use Case:** Find start of duplicates `[1,2,2,2,3]` → left_bound(2) = index 1

### **3. Right Bound (Upper Bound)**

Find **first position** where element > target:

```rust
fn search_right_bound<T: Ord>(arr: &[T], target: &T) -> usize {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] <= *target {
            left = mid + 1;  // Move past target
        } else {
            right = mid;
        }
    }
    left
}
```

**Use Case:** Find end of duplicates `[1,2,2,2,3]` → right_bound(2) = index 4

### **4. Binary Search by Key**

Search using a custom comparison key:

```rust
struct Event { time: i32, data: String }

// Find event by timestamp without implementing Ord on Event
events.binary_search_by_key(&150, |e| e.time)
```

## 🎯 **When to Use Binary Search**

### **✅ Use When:**

- Data is **sorted** (or can be sorted once, searched many times)
- Need O(log n) lookup performance
- Working with large datasets (n > 100)
- Memory is constrained (no space for hash table)

### **❌ Don't Use When:**

- Data is **unsorted** and won't be sorted
- Only searching once (O(n) linear is simpler)
- Small datasets (n < 20, linear search is faster due to cache)
- Need O(1) lookups (use HashMap instead)

### **🤔 Consider Alternatives:**

- **HashMap/HashSet**: O(1) average lookup, but requires extra memory
- **BTreeMap/BTreeSet**: O(log n) + maintains sorted order
- **Linear Search**: Simpler for small n, works on unsorted data

## ⚠️ **Common Pitfalls**

### **1. Integer Overflow**

```rust
// ❌ WRONG: Can overflow if left + right > usize::MAX
let mid = (left + right) / 2;

// ✅ CORRECT: Overflow-safe
let mid = left + (right - left) / 2;
```

### **2. Off-By-One Errors**

```rust
// Exclusive right bound (Rust convention)
let mut right = arr.len();  // NOT arr.len() - 1
while left < right {        // NOT left <= right
    // ...
}
```

**Why:** Using exclusive bounds makes the invariant cleaner:
- `arr[left..right]` is the search space
- When `left == right`, search space is empty

### **3. Unsorted Data**

```rust
let data = [5, 2, 8, 1, 9];
// This compiles but gives WRONG results!
data.binary_search(&8);  // Unpredictable behavior
```

**Solution:** Always verify data is sorted first, or sort it:

```rust
let mut data = vec![5, 2, 8, 1, 9];
data.sort();  // Now binary search is valid
data.binary_search(&8);
```

### **4. Floating Point Comparisons**

```rust
// ❌ Floats don't implement Ord (due to NaN)
let floats = [1.0, 2.5, 3.7];
floats.binary_search(&2.5);  // Compile error!

// ✅ Use binary_search_by with custom comparison
floats.binary_search_by(|x| x.partial_cmp(&2.5).unwrap())
```

## 🔗 **Rust Standard Library**

```rust
// Built-in binary search on slices
let data = [1, 3, 5, 7, 9];

// Exact match
match data.binary_search(&5) {
    Ok(index) => println!("Found at {}", index),
    Err(insertion_point) => println!("Not found, insert at {}", insertion_point),
}

// Custom comparison
data.binary_search_by(|x| x.cmp(&5));

// Search by key extraction
struct Person { age: u32, name: &'static str }
let people = [
    Person { age: 20, name: "Alice" },
    Person { age: 30, name: "Bob" },
];
people.binary_search_by_key(&25, |p| p.age);
```

## 🎓 **Learning Progression**

1. **Foundation** (this page) - Understand core algorithm and complexity
2. **[[Binary Search Iterator Patterns]]** - Rust-specific patterns with iterators
3. **[[AoC Binary Search Applications]]** - Competitive programming patterns
4. **[[mission-3]]** - Production implementation with trait-based design

## 📊 **Visual Example**

```
Search for 7 in [1, 3, 5, 7, 9, 11, 13, 15]:

Iteration 1:
[1, 3, 5, 7, 9, 11, 13, 15]
           ^
         mid=9 (index 4)
7 < 9 → search left

Iteration 2:
[1, 3, 5, 7]
     ^
   mid=5 (index 2)
7 > 5 → search right

Iteration 3:
[7]
 ^
mid=7 (index 3)
7 == 7 → FOUND!
```

## 🔗 **Related Concepts**

- [[Divide and Conquer]] - Algorithm paradigm family
- [[sorting-algorithms]] - Prerequisite for binary search (data must be sorted)
- [[Binary Search Iterator Patterns]] - Advanced Rust patterns combining search + iteration
- [[AoC Binary Search Applications]] - Competitive programming use cases
- [[mission-3]] - Production-quality trait-based implementation
- [[Algorithm Analysis]] - Big-O complexity analysis
- [[Big-O Notation]] - Understanding time complexity

---

*Tags: #algorithms #binary-search #divide-and-conquer #searching #complexity-analysis #mission3 #fundamentals*

*Links: [[zettel-index]] | [[Algorithms MOC]] | [[Binary Search Iterator Patterns]] | [[AoC Binary Search Applications]] | [[mission-3]]*
