# Sorting Algorithms

A comprehensive overview of sorting algorithms with implementations, analysis, and practical applications in Rust.

## 🎯 **Quick Reference**

| Algorithm | Time Complexity | Space | Stable | In-Place | Best Use Case |
|-----------|-----------------|-------|--------|----------|---------------|
| **Bubble Sort** | O(n²) | O(1) | Yes | Yes | Educational, small datasets |
| **Selection Sort** | O(n²) | O(1) | No | Yes | Memory-constrained environments |
| **Insertion Sort** | O(n²) | O(1) | Yes | Yes | Nearly sorted data, small arrays |
| **Merge Sort** | O(n log n) | O(n) | Yes | No | Guaranteed O(n log n), stable sorting |
| **Quick Sort** | O(n log n) avg | O(log n) | No | Yes | General purpose, cache-friendly |
| **Heap Sort** | O(n log n) | O(1) | No | Yes | Guaranteed O(n log n), in-place |
| **Counting Sort** | O(n + k) | O(k) | Yes | No | Integer keys, small range |
| **Radix Sort** | O(d(n + k)) | O(n + k) | Yes | No | Integer/string keys |
| **Bucket Sort** | O(n + k) avg | O(n + k) | Yes | No | Uniformly distributed data |
| **Circle Sort** | O(n log n) avg | O(log n) | No | Yes | Curiosity-driven, recursive mirror pairs |

## 🦀 **Rust Standard Library Sorting**

### **slice::sort() - Stable Sort**

```rust
let mut data = vec![5, 2, 8, 1, 9];
data.sort();  // [1, 2, 5, 8, 9]

// Custom comparison
data.sort_by(|a, b| b.cmp(a));  // Reverse sort

// Sort by key
struct Person { name: String, age: u32 }
let mut people = vec![
    Person { name: "Alice".into(), age: 30 },
    Person { name: "Bob".into(), age: 25 },
];
people.sort_by_key(|p| p.age);  // Sort by age
```

**Implementation:** Adaptive merge sort (Timsort variant)
- **Stable:** Equal elements maintain original order
- **Adaptive:** O(n) on already sorted data
- **Time:** O(n log n) worst case
- **Space:** O(n)

### **slice::sort_unstable() - Fast Sort**

```rust
let mut data = vec![5, 2, 8, 1, 9];
data.sort_unstable();  // Faster but not stable

// Custom comparison
data.sort_unstable_by(|a, b| b.cmp(a));

// Sort by key
people.sort_unstable_by_key(|p| p.age);
```

**Implementation:** Pattern-defeating quicksort (pdqsort) - **See detailed section below**
- **Not stable:** May reorder equal elements
- **Faster:** Better cache performance, less allocation
- **Time:** O(n log n) average and worst case (unlike classic quicksort)
- **Space:** O(log n)
- **Use when:** Stability not needed, performance critical
- **Pattern detection:** Adapts to sorted, reverse, and duplicate patterns
- **Details:** See [[#pdqsort-pattern-defeating-quicksort-rusts-sort_unstable]] section

### **Choosing Between sort() and sort_unstable()**

```rust
// ✅ Use sort() when:
// - Stability matters (maintaining relative order)
// - Sorting complex types where order of equals matters
struct Event { time: u64, id: u32 }
events.sort_by_key(|e| e.time);  // Events at same time keep ID order

// ✅ Use sort_unstable() when:
// - Sorting primitives (i32, f64, etc.)
// - Performance critical
// - Don't care about relative order of equals
let mut numbers = vec![5, 2, 8, 1, 9];
numbers.sort_unstable();  // Faster for primitives
```

**Performance Comparison:**
```rust
// Benchmark results (typical):
// sort():          ~100 µs for 10,000 elements
// sort_unstable(): ~70 µs for 10,000 elements
// ~30% faster for primitives, may vary by data
```

## 📚 **Fundamental Concepts**

### **Stability**

A sorting algorithm is **stable** if it preserves the relative order of equal elements.

```rust
// Stable sort: equal elements maintain original order
let mut data = vec![(1, 'a'), (2, 'b'), (1, 'c')];
// After stable sort: [(1, 'a'), (1, 'c'), (2, 'b')]
// Note: (1, 'a') still comes before (1, 'c')
```

### **In-Place Sorting**

**In-place** algorithms sort with O(1) extra space (excluding input).

```rust
// In-place: modifies the original array
fn bubble_sort<T: Ord>(arr: &mut [T]) {
    // Only uses a few variables, O(1) space
}

// Not in-place: creates new arrays
fn merge_sort<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
    // Creates temporary vectors, O(n) space
}
```

### **Adaptive Algorithms**

**Adaptive** algorithms perform better on partially sorted data.

```rust
// Insertion sort is adaptive - O(n) on nearly sorted data
// Quick sort can be adaptive with good pivot selection
// Merge sort is not adaptive - always O(n log n)
```

## 🔄 **Comparison-Based Sorts**

### **1. Bubble Sort**

**Concept**: Repeatedly swap adjacent elements that are out of order.

```rust
/// Bubble Sort - O(n²) time, O(1) space, stable
fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        let mut swapped = false;
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break; // Early termination for sorted arrays
        }
    }
}

#[cfg(test)]
mod bubble_tests {
    use super::*;
    
    #[test]
    fn test_bubble_sort() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }
}
```

**Analysis**:

- **Time**: O(n²) worst/average, O(n) best (with early termination)
- **Space**: O(1)
- **Stable**: Yes
- **Use Cases**: Educational purposes, very small datasets

### **2. Selection Sort**

**Concept**: Find the minimum element and place it at the beginning.

```rust
/// Selection Sort - O(n²) time, O(1) space, not stable
fn selection_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        let mut min_idx = i;
        for j in i + 1..n {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        if min_idx != i {
            arr.swap(i, min_idx);
        }
    }
}

#[cfg(test)]
mod selection_tests {
    use super::*;
    
    #[test]
    fn test_selection_sort() {
        let mut arr = vec![64, 25, 12, 22, 11];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 64]);
    }
}
```

**Analysis**:

- **Time**: O(n²) always
- **Space**: O(1)
- **Stable**: No (due to long-distance swaps)
- **Use Cases**: Memory is severely limited

### **3. Insertion Sort**

**Concept**: Build sorted array one element at a time by inserting each element in its correct position.

```rust
/// Insertion Sort - O(n²) time, O(1) space, stable, adaptive
fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// Optimized insertion sort with binary search
fn insertion_sort_binary<T: Ord + Clone>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let key = arr[i].clone();
        let mut left = 0;
        let mut right = i;
        
        // Binary search for insertion position
        while left < right {
            let mid = left + (right - left) / 2;
            if arr[mid] <= key {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        
        // Shift elements and insert
        for j in (left..=i - 1).rev() {
            arr.swap(j, j + 1);
        }
        arr[left] = key;
    }
}

#[cfg(test)]
mod insertion_tests {
    use super::*;
    
    #[test]
    fn test_insertion_sort() {
        let mut arr = vec![12, 11, 13, 5, 6];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![5, 6, 11, 12, 13]);
    }
    
    #[test]
    fn test_nearly_sorted() {
        let mut arr = vec![1, 2, 3, 5, 4]; // Nearly sorted
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
}
```

**Analysis**:

- **Time**: O(n²) worst, O(n) best (nearly sorted)
- **Space**: O(1)
- **Stable**: Yes
- **Adaptive**: Yes
- **Use Cases**: Small arrays, nearly sorted data, online algorithms

### **4. Merge Sort**

**Concept**: Divide array into halves, recursively sort, then merge sorted halves.

```rust
/// Merge Sort - O(n log n) time, O(n) space, stable
fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    
    let mid = len / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();
    
    merge_sort(&mut left);
    merge_sort(&mut right);
    
    merge(arr, &left, &right);
}

fn merge<T: Ord + Clone>(arr: &mut [T], left: &[T], right: &[T]) {
    let mut i = 0; // Left index
    let mut j = 0; // Right index
    let mut k = 0; // Merged array index
    
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }
    
    // Copy remaining elements
    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }
    
    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod merge_tests {
    use super::*;
    
    #[test]
    fn test_merge_sort() {
        let mut arr = vec![38, 27, 43, 3, 9, 82, 10];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![3, 9, 10, 27, 38, 43, 82]);
    }
}
```

**Analysis**:

- **Time**: O(n log n) always (guaranteed)
- **Space**: O(n)
- **Stable**: Yes
- **Use Cases**: When stability is required, external sorting, linked lists

### **5. Quick Sort**

**Concept**: Choose a pivot, partition around it, recursively sort partitions.

```rust
/// Quick Sort - O(n log n) average, O(n²) worst, in-place
fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    
    let pivot_index = partition(arr);
    quick_sort(&mut arr[..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot_index = len - 1; // Choose last element as pivot
    let mut i = 0;
    
    for j in 0..pivot_index {
        if arr[j] <= arr[pivot_index] {
            arr.swap(i, j);
            i += 1;
        }
    }
    
    arr.swap(i, pivot_index);
    i
}

/// Quick Sort with random pivot (better worst-case behavior)
use rand::Rng;

fn quick_sort_random<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    
    // Random pivot selection
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..arr.len());
    let last_index = arr.len() - 1;
    arr.swap(random_index, last_index);
    
    let pivot_index = partition(arr);
    quick_sort_random(&mut arr[..pivot_index]);
    quick_sort_random(&mut arr[pivot_index + 1..]);
}

#[cfg(test)]
mod quick_tests {
    use super::*;
    
    #[test]
    fn test_quick_sort() {
        let mut arr = vec![10, 7, 8, 9, 1, 5];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 5, 7, 8, 9, 10]);
    }
}
```

**Analysis**:

- **Time**: O(n log n) average, O(n²) worst case
- **Space**: O(log n) (recursion stack)
- **Stable**: No
- **In-Place**: Yes
- **Use Cases**: General-purpose sorting, when average performance matters

### **6. Heap Sort**

**Concept**: Build a max heap, repeatedly extract maximum and place at end.

```rust
/// Heap Sort - O(n log n) time, O(1) space, not stable
fn heap_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    
    // Build max heap
    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }
    
    // Extract elements from heap one by one
    for i in (1..n).rev() {
        arr.swap(0, i); // Move current root to end
        heapify(arr, i, 0); // Heapify reduced heap
    }
}

fn heapify<T: Ord>(arr: &mut [T], n: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;
    
    if left < n && arr[left] > arr[largest] {
        largest = left;
    }
    
    if right < n && arr[right] > arr[largest] {
        largest = right;
    }
    
    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}

#[cfg(test)]
mod heap_tests {
    use super::*;
    
    #[test]
    fn test_heap_sort() {
        let mut arr = vec![12, 11, 13, 5, 6, 7];
        heap_sort(&mut arr);
        assert_eq!(arr, vec![5, 6, 7, 11, 12, 13]);
    }
}
```

**Analysis**:

- **Time**: O(n log n) guaranteed
- **Space**: O(1)
- **Stable**: No
- **Use Cases**: When guaranteed O(n log n) and O(1) space is required

### **7. Circle Sort** ⚡

**Concept**: Bend the array into a circle. Compare mirror pairs across the diameter (first↔last, second↔second-to-last), swap if out of order, split in half, recurse. Repeat full passes until zero swaps.

**Origin**: Not published in a journal or taught at MIT — a programmer posted it on a coding forum and the community verified it actually works at O(n log n) average. One of the few sorting algorithms discovered through grassroots experimentation.

```rust
/// Circle Sort - O(n log n) avg, O(log n) space, not stable, in-place
fn circle_sort_pass<T: Ord>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    if lo >= hi { return 0; }
    let mut swaps = 0;
    let (mut left, mut right) = (lo, hi);

    // Compare mirror pairs: outside-in across the "diameter"
    while left < right {
        if arr[left] > arr[right] {
            arr.swap(left, right);
            swaps += 1;
        }
        left += 1;
        right -= 1;
    }
    // Odd-length: check middle pair
    if left == right && left < hi && arr[left] > arr[left + 1] {
        arr.swap(left, left + 1);
        swaps += 1;
    }
    // Recurse on each half
    let mid = lo + (hi - lo) / 2;
    swaps += circle_sort_pass(arr, lo, mid);
    swaps += circle_sort_pass(arr, mid + 1, hi);
    swaps
}

fn circle_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 { return; }
    while circle_sort_pass(arr, 0, arr.len() - 1) > 0 {}
}
```

**Analysis**:

- **Time**: O(n log n) average, O(n log² n) worst case
- **Space**: O(log n) recursion depth only — fully in-place
- **Stable**: No
- **Adaptive**: Efficiently handles reverse-sorted input (one pass suffices)
- **Use Cases**: Educational (elegant recursive structure), competitive programming curiosity

**Benchmark vs std::sort and Bubble Sort** (see `advanced_examples/circle_sort/`):

| n | std::sort | Circle Sort | Bubble Sort |
|------:|----------:|------------:|------------:|
| 100 | 453 ns | 2.7 µs | 2.6 µs |
| 1,000 | 6.3 µs | 52.6 µs | 286 µs |
| 10,000 | 80.6 µs | 1.52 ms | 26.1 ms |

Key insight: At n=100 bubble and circle are neck-and-neck, but by n=10,000 the O(n²) vs O(n log n) divergence is 17× — complexity class only hurts at scale.

> **Implementation**: Full implementation with visualization callback and benchmarks in `advanced_examples/circle_sort/`

## 🔢 **Non-Comparison Sorts**

### **7. Counting Sort**

**Concept**: Count occurrences of each element, then reconstruct sorted array.

```rust
/// Counting Sort - O(n + k) time, works for integers in known range
fn counting_sort(arr: &mut [usize], max_value: usize) {
    let mut count = vec![0; max_value + 1];
    
    // Count occurrences
    for &value in arr.iter() {
        count[value] += 1;
    }
    
    // Reconstruct sorted array
    let mut index = 0;
    for (value, &freq) in count.iter().enumerate() {
        for _ in 0..freq {
            arr[index] = value;
            index += 1;
        }
    }
}

/// Stable counting sort (preserves relative order)
fn counting_sort_stable(arr: &mut [usize], max_value: usize) {
    let n = arr.len();
    let mut count = vec![0; max_value + 1];
    let mut output = vec![0; n];
    
    // Count occurrences
    for &value in arr.iter() {
        count[value] += 1;
    }
    
    // Transform count array to positions
    for i in 1..=max_value {
        count[i] += count[i - 1];
    }
    
    // Build output array (traverse from right to maintain stability)
    for &value in arr.iter().rev() {
        count[value] -= 1;
        output[count[value]] = value;
    }
    
    // Copy back to original array
    arr.copy_from_slice(&output);
}

#[cfg(test)]
mod counting_tests {
    use super::*;
    
    #[test]
    fn test_counting_sort() {
        let mut arr = vec![4, 2, 2, 8, 3, 3, 1];
        counting_sort(&mut arr, 8);
        assert_eq!(arr, vec![1, 2, 2, 3, 3, 4, 8]);
    }
}
```

**Analysis**:

- **Time**: O(n + k) where k is the range of input
- **Space**: O(k)
- **Stable**: Can be made stable
- **Use Cases**: Small integer ranges, frequency counting

### **8. Radix Sort**

**Concept**: Sort by individual digits/characters, starting from least significant.

```rust
/// Radix Sort - sorts integers by processing digits
fn radix_sort(arr: &mut [u32]) {
    if arr.is_empty() {
        return;
    }
    
    let max_value = *arr.iter().max().unwrap();
    let mut exp = 1;
    
    while max_value / exp > 0 {
        counting_sort_by_digit(arr, exp);
        exp *= 10;
    }
}

fn counting_sort_by_digit(arr: &mut [u32], exp: u32) {
    let n = arr.len();
    let mut output = vec![0; n];
    let mut count = vec![0; 10]; // 0-9 digits
    
    // Count occurrences of digits
    for &value in arr.iter() {
        let digit = ((value / exp) % 10) as usize;
        count[digit] += 1;
    }
    
    // Transform count array to positions
    for i in 1..10 {
        count[i] += count[i - 1];
    }
    
    // Build output array
    for &value in arr.iter().rev() {
        let digit = ((value / exp) % 10) as usize;
        count[digit] -= 1;
        output[count[digit]] = value;
    }
    
    // Copy back
    arr.copy_from_slice(&output);
}

#[cfg(test)]
mod radix_tests {
    use super::*;
    
    #[test]
    fn test_radix_sort() {
        let mut arr = vec![170, 45, 75, 90, 2, 802, 24, 66];
        radix_sort(&mut arr);
        assert_eq!(arr, vec![2, 24, 45, 66, 75, 90, 170, 802]);
    }
}
```

**Analysis**:

- **Time**: O(d(n + k)) where d is number of digits
- **Space**: O(n + k)
- **Stable**: Yes
- **Use Cases**: Integers, strings with fixed-length keys

### **9. Bucket Sort**

**Concept**: Distribute elements into buckets, sort buckets individually.

```rust
/// Bucket Sort - for uniformly distributed floating-point numbers [0, 1)
fn bucket_sort(arr: &mut [f64]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    // Create empty buckets
    let mut buckets: Vec<Vec<f64>> = vec![Vec::new(); n];
    
    // Distribute elements into buckets
    for &value in arr.iter() {
        let bucket_index = (value * n as f64).floor() as usize;
        let bucket_index = bucket_index.min(n - 1); // Handle edge case
        buckets[bucket_index].push(value);
    }
    
    // Sort individual buckets and concatenate
    let mut index = 0;
    for bucket in &mut buckets {
        bucket.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for &value in bucket.iter() {
            arr[index] = value;
            index += 1;
        }
    }
}

#[cfg(test)]
mod bucket_tests {
    use super::*;
    
    #[test]
    fn test_bucket_sort() {
        let mut arr = vec![0.897, 0.565, 0.656, 0.1234, 0.665, 0.3434];
        bucket_sort(&mut arr);
        
        // Check if sorted
        for i in 1..arr.len() {
            assert!(arr[i - 1] <= arr[i]);
        }
    }
}
```

**Analysis**:

- **Time**: O(n + k) average, O(n²) worst case
- **Space**: O(n + k)
- **Stable**: Yes
- **Use Cases**: Uniformly distributed data, floating-point numbers

## 🎛️ **Hybrid Algorithms**

### **Introsort (Introspective Sort)**

Rust's standard library uses Introsort - a hybrid of quicksort, heapsort, and insertion sort.

```rust
/// Simplified Introsort implementation
fn introsort<T: Ord>(arr: &mut [T]) {
    let max_depth = (arr.len() as f64).log2().floor() as usize * 2;
    introsort_recursive(arr, max_depth);
}

fn introsort_recursive<T: Ord>(arr: &mut [T], max_depth: usize) {
    if arr.len() <= 16 {
        insertion_sort(arr); // Small arrays: use insertion sort
    } else if max_depth == 0 {
        heap_sort(arr); // Recursion too deep: use heap sort
    } else {
        let pivot = partition(arr);
        introsort_recursive(&mut arr[..pivot], max_depth - 1);
        introsort_recursive(&mut arr[pivot + 1..], max_depth - 1);
    }
}
```

### **Timsort (Used in Python, Java)**

```rust
/// Timsort concepts - detects runs and uses merge sort
fn timsort<T: Ord + Clone>(arr: &mut [T]) {
    let min_run_size = 32; // Typical minimum run size
    let n = arr.len();
    
    // Sort individual runs
    let mut start = 0;
    while start < n {
        let run_size = find_run_size(&arr[start..]).min(min_run_size);
        let end = (start + run_size).min(n);
        insertion_sort(&mut arr[start..end]);
        start = end;
    }
    
    // Merge runs (simplified - real Timsort is much more complex)
    let mut size = min_run_size;
    while size < n {
        for start in (0..n).step_by(size * 2) {
            let mid = (start + size).min(n);
            let end = (start + size * 2).min(n);
            if mid < end {
                merge_in_place(&mut arr[start..end], mid - start);
            }
        }
        size *= 2;
    }
}

fn find_run_size<T: Ord>(arr: &[T]) -> usize {
    if arr.len() < 2 {
        return arr.len();
    }
    
    let mut length = 1;
    let ascending = arr[0] <= arr[1];
    
    for i in 1..arr.len() - 1 {
        if ascending && arr[i] > arr[i + 1] {
            break;
        }
        if !ascending && arr[i] < arr[i + 1] {
            break;
        }
        length += 1;
    }
    
    length
}

fn merge_in_place<T: Ord + Clone>(arr: &mut [T], mid: usize) {
    let left = arr[..mid].to_vec();
    let right = arr[mid..].to_vec();
    merge(arr, &left, &right);
}
```

### **pdqsort (Pattern-Defeating Quicksort) - Rust's sort_unstable()**

**pdqsort** is the sophisticated hybrid algorithm that powers Rust's `sort_unstable()` and `sort_unstable_by()`. It's designed to be fast in practice while avoiding worst-case behaviors that plague traditional quicksort.

#### **Why "Pattern-Defeating"?**

Traditional quicksort has O(n²) worst-case performance on certain patterns:
- Already sorted data (ascending/descending)
- Data with many duplicates
- Adversarial pivot patterns
- Organ-pipe distributions (sorted in middle)

**pdqsort detects these patterns** during execution and switches strategies to maintain O(n log n) performance even on "bad" inputs.

#### **Three-Algorithm Hybrid Strategy**

```rust
/// Conceptual flow of pdqsort
fn pdqsort<T: Ord>(arr: &mut [T]) {
    let max_depth = 2 * (arr.len() as f64).log2().floor() as usize;
    pdqsort_recursive(arr, max_depth, true);
}

fn pdqsort_recursive<T: Ord>(arr: &mut [T], depth_limit: usize, leftmost: bool) {
    // 1. SMALL ARRAYS: Use insertion sort (≤ 24 elements)
    if arr.len() <= 24 {
        if leftmost {
            insertion_sort(arr);  // Unbounded insertion sort
        } else {
            insertion_sort_with_sentinel(arr);  // Faster variant with sentinel
        }
        return;
    }
    
    // 2. DEPTH LIMIT EXCEEDED: Switch to heapsort
    if depth_limit == 0 {
        heapsort(arr);  // Guarantee O(n log n), prevent stack overflow
        return;
    }
    
    // 3. MAIN STRATEGY: Quicksort with pattern detection
    let (pivot_pos, already_partitioned) = partition_with_block_strategy(arr);
    
    // Pattern detection: check if partition was balanced
    let left_size = pivot_pos;
    let right_size = arr.len() - pivot_pos - 1;
    let highly_unbalanced = left_size < arr.len() / 8 || right_size < arr.len() / 8;
    
    if highly_unbalanced {
        // Bad pattern detected! Reduce depth limit faster
        pdqsort_recursive(&mut arr[..pivot_pos], depth_limit - 1, leftmost);
        pdqsort_recursive(&mut arr[pivot_pos + 1..], depth_limit - 1, false);
    } else {
        // Normal recursion
        pdqsort_recursive(&mut arr[..pivot_pos], depth_limit - 1, leftmost);
        pdqsort_recursive(&mut arr[pivot_pos + 1..], depth_limit - 1, false);
    }
}
```

#### **Key Optimizations**

**1. Block Partitioning**
```rust
// Instead of element-by-element partitioning:
// ❌ Traditional: many unpredictable branches, poor CPU pipelining
for i in 0..arr.len() {
    if arr[i] < pivot { /* swap */ }
}

// ✅ pdqsort: processes elements in blocks
// - Fewer branch mispredictions
// - Better CPU instruction pipelining
// - Improved cache utilization
const BLOCK_SIZE: usize = 128;
// Process BLOCK_SIZE elements at a time, batch swaps
```

**2. Pivot Selection Strategy**
```rust
// Median-of-3 with pseudorandom fallback
fn choose_pivot<T: Ord>(arr: &[T]) -> usize {
    let len = arr.len();
    let mut a = len / 4;
    let mut b = len / 2;
    let mut c = a + b;
    
    // Sort a, b, c to get median
    if arr[a] > arr[b] { std::mem::swap(&mut a, &mut b); }
    if arr[b] > arr[c] { std::mem::swap(&mut b, &mut c); }
    if arr[a] > arr[b] { std::mem::swap(&mut a, &mut b); }
    
    b  // Return median position
}
```

**3. Partitioned Heuristic Detection**
```rust
// If data is already partitioned, skip recursion
fn partition_with_detection<T: Ord>(arr: &mut [T]) -> (usize, bool) {
    let pivot_pos = partition(arr);
    
    // Check if already partitioned (all elements in correct half)
    let already_partitioned = 
        arr[..pivot_pos].iter().all(|x| x <= &arr[pivot_pos]) &&
        arr[pivot_pos + 1..].iter().all(|x| x >= &arr[pivot_pos]);
    
    (pivot_pos, already_partitioned)
}
```

#### **Performance Characteristics**

| **Scenario** | **Time Complexity** | **Notes** |
|--------------|---------------------|-----------|
| Random data | O(n log n) | ~20-40% faster than classic quicksort |
| Sorted data | O(n log n) | **Detects pattern**, stays optimal |
| Reverse sorted | O(n log n) | Pattern detection prevents O(n²) |
| Many duplicates | O(n log n) | Optimized 3-way partitioning variant |
| Worst case | O(n log n) | **Guaranteed** (heapsort fallback) |
| Best case | O(n) | Already partitioned data |
| Space | O(log n) | Stack space for recursion |

#### **Comparison: pdqsort vs Alternatives**

```rust
// Benchmark results (typical, 100,000 elements):

// Random data:
pdqsort (sort_unstable):  ~2.5 ms   // Fastest
introsort:                ~3.0 ms   // Older Rust default
quicksort (classic):      ~3.5 ms   // No pattern detection
timsort (sort):          ~4.5 ms   // Stable but slower

// Sorted data:
pdqsort:                  ~0.3 ms   // Pattern-defeating wins!
introsort:               ~3.0 ms   // No degradation
quicksort (classic):     ~5000 ms  // O(n²) disaster!
timsort:                 ~0.2 ms   // Adaptive, best here

// Many duplicates (50% same value):
pdqsort:                  ~1.8 ms   // Efficient partitioning
quicksort:               ~4.0 ms   // Many equal comparisons
timsort:                 ~4.0 ms   
```

#### **Why Rust Chose pdqsort (2018)**

Before Rust 1.20 (2018), `sort_unstable()` used **introsort** (quicksort + heapsort fallback). pdqsort replaced it because:

1. **Better pattern detection**: Recognizes adversarial patterns earlier
2. **Cache efficiency**: Block partitioning minimizes cache misses
3. **Branch prediction**: Fewer unpredictable branches, better CPU pipelining
4. **Real-world speed**: 20-40% faster on typical data
5. **Robustness**: Handles edge cases (sorted, reverse, duplicates) gracefully

**Benchmark results from Rust RFC #1884:**
- Random data: **1.2-1.5x faster** than introsort
- Sorted data: **2-3x faster** (pattern detection)
- Many duplicates: **1.5-2x faster** (efficient partitioning)

#### **When to Use sort_unstable() (pdqsort)**

```rust
// ✅ PERFECT USE CASES:
let mut numbers = vec![5, 2, 8, 1, 9, 3];
numbers.sort_unstable();  // Primitives: i32, f64, char, etc.

let mut points = vec![(1, 2), (3, 1), (2, 3)];
points.sort_unstable_by_key(|p| p.0);  // Don't need stability

// Custom descending order
let mut scores = vec![95, 87, 92, 88];
scores.sort_unstable_by(|a, b| b.cmp(a));

// ❌ DON'T USE when stability matters:
struct Event { time: u64, id: u32 }
let mut events = vec![
    Event { time: 100, id: 1 },
    Event { time: 100, id: 2 },  // Want to keep id=1 before id=2
];
// ❌ BAD: sort_unstable_by_key might reorder equal times
events.sort_unstable_by_key(|e| e.time);

// ✅ GOOD: Use stable sort
events.sort_by_key(|e| e.time);  // Preserves relative order
```

#### **Implementation Details**

**Partition Strategy:**
```rust
// pdqsort uses "block quicksort" partitioning:
// 1. Scan blocks of elements (typically 128 at a time)
// 2. Identify elements that need swapping
// 3. Perform swaps in batch
// Result: Better cache usage, fewer branches
```

**Insertion Sort Threshold:**
```rust
// For arrays ≤ 24 elements, insertion sort wins:
const INSERTION_SORT_THRESHOLD: usize = 24;

// Why 24?
// - Insertion sort has low overhead (simple loop)
// - Cache fits entire small array
// - No recursion overhead
// - Empirically tested optimal point
```

**Depth Limit Calculation:**
```rust
// Prevent worst-case O(n²) by limiting recursion depth
let max_depth = 2 * (arr.len() as f64).log2().floor() as usize;

// If depth exceeded:
// → Switch to heapsort (guaranteed O(n log n))
// → Prevents stack overflow
// → Ensures worst-case performance
```

#### **Advanced Features**

**3-Way Partitioning (for duplicates):**
```rust
// When many equal elements, pdqsort can use 3-way partition:
// [< pivot] [== pivot] [> pivot]
// Avoids unnecessary comparisons on equal elements
```

**Adaptive Behavior:**
```rust
// pdqsort adapts to input characteristics:
if almost_sorted(arr) {
    // Use cheaper partitioning strategy
    // Fall back to insertion sort faster
} else if many_duplicates(arr) {
    // Use 3-way partitioning
} else {
    // Standard 2-way partitioning
}
```

#### **References & Further Reading**

- **Original Paper**: ["Pattern-defeating Quicksort"](https://arxiv.org/abs/2106.05123) by Orson Peters
- **Rust Implementation**: [`core::slice::sort`](https://github.com/rust-lang/rust/blob/master/library/core/src/slice/sort.rs)
- **Rust RFC**: [RFC 1884 - "Unstable Sort"](https://github.com/rust-lang/rfcs/blob/master/text/1884-unstable-sort.md)
- **Blog Post**: [Orson Peters' pdqsort Announcement](https://github.com/orlp/pdqsort)

#### **Key Takeaway**

**Good sorting in practice requires adapting to the data**, not just having good average-case complexity. pdqsort embodies this philosophy:
- Fast on random data (quicksort core)
- Safe on adversarial patterns (pattern detection)
- Guaranteed performance (heapsort fallback)
- Optimized for modern CPUs (block partitioning, branch prediction)

This makes it the ideal choice for Rust's performance-critical unstable sort.

---

## 📈 **Sorting Complexity Lower Bound**

**Theoretical Limit:** Comparison-based sorting cannot be faster than O(n log n) in the worst case.

### **Proof Intuition**

- Sorting n distinct elements has **n! possible outcomes**
- Binary decision tree (compare A vs B) has **2^h leaves** for height h
- Need at least **log₂(n!) ≈ n log n** comparisons

**Example:**
```
For 3 elements [a, b, c], there are 3! = 6 possible orderings:
1. a < b < c
2. a < c < b
3. b < a < c
4. b < c < a
5. c < a < b
6. c < b < a

Decision tree needs at least log₂(6) ≈ 2.58 → 3 comparisons minimum
```

### **Non-Comparison Sorts (Can Be Faster)**

These avoid the O(n log n) lower bound by not using comparisons:

- **Counting Sort:** O(n + k) when elements in small range
- **Radix Sort:** O(d × n) for d-digit numbers
- **Bucket Sort:** O(n) average for uniformly distributed data

**Tradeoff:** These have limitations (specific data types, extra space, limited range)

## 🎲 **Algorithm Performance by Input Type**

### **Best Algorithms for Completely Unsorted/Random Data**

When data is **completely random** with no pattern, certain algorithms excel:

#### **🥇 Quick Sort (Best Overall)**

```rust
let mut random_data = vec![9, 3, 7, 1, 5, 2, 8, 4, 6];
quick_sort(&mut random_data);
// Or use Rust's optimized version:
random_data.sort_unstable();  // Uses pdqsort
```

**Why it wins on random data:**
- **Average case:** O(n log n) - very fast in practice
- **Cache-friendly:** Excellent locality of reference (accesses nearby elements)
- **In-place:** Only O(log n) space for recursion
- **Partitioning:** Works equally well regardless of initial order
- **Modern variants:** pdqsort (Rust's sort_unstable) handles all cases well

⚠️ **Pitfall:** Classic quicksort degrades to O(n²) on sorted/reverse-sorted data with poor pivot selection. Use random pivot or Rust's sort_unstable() which handles this.

#### **🥈 Merge Sort (Most Consistent)**

```rust
let mut random_data = vec![9, 3, 7, 1, 5, 2, 8, 4, 6];
merge_sort(&mut random_data);
// Or use Rust's stable sort:
random_data.sort();  // Uses Timsort
```

**Why it's reliable on random data:**
- **Always O(n log n):** Unaffected by initial distribution
- **Stable:** Preserves relative order of equal elements
- **Predictable:** No worst-case degradation like quicksort
- **Divide-and-conquer:** Doesn't care about input patterns

⚠️ **Tradeoff:** Requires O(n) extra space for merging

#### **🥉 Heap Sort (Space-Efficient Guarantee)**

```rust
let mut random_data = vec![9, 3, 7, 1, 5, 2, 8, 4, 6];
heap_sort(&mut random_data);
```

**Why it's solid on random data:**
- **Always O(n log n):** Guaranteed performance
- **In-place:** Only O(1) extra space
- **Consistent:** Initial order doesn't affect heap construction

⚠️ **Tradeoff:** Slower than quicksort in practice (~2x), not cache-friendly

### **❌ Poor Choices for Random/Unsorted Data**

#### **Insertion Sort**
```rust
// ❌ BAD for random data
let mut random_data = vec![9, 3, 7, 1, 5, 2, 8, 4, 6];
insertion_sort(&mut random_data);  // O(n²) - many element shifts!
```
- **Performance:** O(n²) on random data (worst case behavior)
- **Why it fails:** Each element likely needs to shift through many positions
- **Only use for:** Nearly sorted data (then it's O(n))

#### **Bubble Sort**
```rust
// ❌ BAD for random data  
bubble_sort(&mut random_data);  // O(n²) - educational only!
```
- **Performance:** O(n²) on random data
- **No advantages:** Never better than O(n²) on unsorted data
- **Use case:** Educational purposes only

#### **Selection Sort**
```rust
// ❌ BAD for random data
selection_sort(&mut random_data);  // Always O(n²)
```
- **Performance:** Always O(n²) - never adaptive
- **Why it's bad:** Doesn't benefit from any partial ordering
- **Only use for:** Memory severely limited AND minimal swaps needed

### **Performance Comparison on Random Data**

```rust
// Benchmark results (10,000 random elements, typical hardware):

// 🚀 Fast (O(n log n))
quick_sort:       ~2-3 ms   // Fastest
sort_unstable():  ~2 ms     // Rust's optimized quicksort
merge_sort:       ~4-5 ms   // Stable, predictable
sort():           ~5 ms     // Rust's stable sort
heap_sort:        ~6-8 ms   // Guaranteed O(n log n), in-place

// 🐌 Slow (O(n²))
insertion_sort:   ~100-150 ms  // 50x slower!
selection_sort:   ~120-180 ms  // Always slow
bubble_sort:      ~200-300 ms  // Worst of all
```

### **Best Algorithms for Nearly Sorted Data**

When data is **already sorted or nearly sorted**:

#### **🥇 Insertion Sort**
```rust
let mut nearly_sorted = vec![1, 2, 3, 5, 4, 6, 7]; // Only one element out of place
insertion_sort(&mut nearly_sorted);  // O(n) - lightning fast!
```
- **Best case:** O(n) when already sorted
- **Adaptive:** Skips sorted portions
- **Ideal for:** < 20 elements OR very few inversions

#### **🥈 Timsort (Rust's sort())**
```rust
nearly_sorted.sort();  // Detects sorted runs
```
- **Adaptive:** O(n) to O(n log n) based on existing order
- **Detects runs:** Identifies already-sorted subsequences
- **Stable:** Maintains relative order

### **🦀 Rust Standard Library Recommendations**

```rust
let mut unsorted = vec![9, 3, 7, 1, 5, 2, 8, 4, 6];

// ✅ BEST for completely random/unsorted primitives:
unsorted.sort_unstable();  
// - Uses pdqsort (pattern-defeating quicksort)
// - Handles all edge cases (sorted, reverse, duplicates)
// - O(n log n) guaranteed (unlike classic quicksort)
// - ~30% faster than sort()

// ✅ BEST when stability needed:
unsorted.sort();
// - Uses Timsort (adaptive merge sort)
// - Stable (preserves relative order)
// - Adaptive (fast on nearly-sorted data)
// - O(n log n) guaranteed

// ✅ BEST for nearly sorted data (small arrays):
if unsorted.len() < 20 {
    insertion_sort(&mut unsorted);  // Custom implementation
}
// Rust's sort() already does this internally!
```

### **Summary Table: Algorithm vs Input Type**

| Input Type | Best Choice | Why |
|------------|-------------|-----|
| **Random/Unsorted** | sort_unstable() | Fastest O(n log n), no stability needed |
| **Random + Stability** | sort() | Stable O(n log n) |
| **Nearly Sorted** | sort() | Adaptive (detects runs) |
| **Small (< 20)** | Insertion or sort() | Low overhead |
| **Reverse Sorted** | sort_unstable() | Handles edge cases |
| **Many Duplicates** | sort_unstable() | Efficient partitioning |
| **Integers, Small Range** | Counting Sort | O(n + k) non-comparison |

## 🧪 **Performance Testing & Benchmarks**

```rust
use std::time::Instant;
use rand::Rng;

/// Benchmark sorting algorithms
fn benchmark_sorts() {
    let sizes = vec![100, 1000, 10000];
    let algorithms = vec![
        ("Insertion Sort", insertion_sort as fn(&mut [i32])),
        ("Quick Sort", quick_sort as fn(&mut [i32])),
        ("Merge Sort", merge_sort as fn(&mut [i32])),
        ("Heap Sort", heap_sort as fn(&mut [i32])),
    ];
    
    for size in sizes {
        println!("\n=== Array Size: {} ===", size);
        
        for (name, algorithm) in &algorithms {
            let mut data: Vec<i32> = (0..size).map(|_| rand::thread_rng().gen()).collect();
            
            let start = Instant::now();
            algorithm(&mut data);
            let duration = start.elapsed();
            
            // Verify sorting
            assert!(is_sorted(&data));
            
            println!("{}: {:?}", name, duration);
        }
    }
}

fn is_sorted<T: Ord>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

/// Memory usage analysis
fn analyze_memory_usage() {
    println!("Memory Usage Analysis:");
    println!("- Bubble Sort: O(1) - minimal extra variables");
    println!("- Insertion Sort: O(1) - minimal extra variables");  
    println!("- Selection Sort: O(1) - minimal extra variables");
    println!("- Quick Sort: O(log n) - recursion stack");
    println!("- Merge Sort: O(n) - temporary arrays");
    println!("- Heap Sort: O(1) - in-place heap operations");
    println!("- Counting Sort: O(k) - count array");
    println!("- Radix Sort: O(n + k) - output and count arrays");
}
```

## 🎯 **Algorithm Selection Guide**

### **When to Use Which Algorithm**

```rust
/// Smart sort selection based on input characteristics
fn smart_sort<T: Ord + Clone>(arr: &mut [T]) {
    match arr.len() {
        0..=1 => {} // Already sorted
        2..=16 => insertion_sort(arr), // Small arrays
        _ => {
            if is_nearly_sorted(arr) {
                insertion_sort(arr); // O(n) for nearly sorted
            } else if requires_stability() {
                merge_sort(arr); // Stable O(n log n)
            } else {
                quick_sort(arr); // Fast average case
            }
        }
    }
}

fn is_nearly_sorted<T: Ord>(arr: &[T]) -> bool {
    let inversions = count_inversions(arr);
    inversions < arr.len() / 4 // Arbitrary threshold
}

fn count_inversions<T: Ord>(arr: &[T]) -> usize {
    let mut count = 0;
    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            if arr[i] > arr[j] {
                count += 1;
            }
        }
    }
    count
}

fn requires_stability() -> bool {
    // Application-specific logic
    true // For example purposes
}
```

### **Decision Tree**

```
Is data already sorted or nearly sorted?
  └─ Yes → Insertion Sort (O(n) best case)
  └─ No → Continue...

Do you need stable sorting?
  └─ Yes → Merge Sort or Rust's sort()
  └─ No → Continue...

Is memory very limited?
  └─ Yes → Heap Sort (O(1) space)
  └─ No → Quick Sort or Rust's sort_unstable()

Is data very small (< 20 elements)?
  └─ Yes → Insertion Sort (simple, low overhead)
  └─ No → Quick Sort or sort_unstable()

Need guaranteed O(n log n)?
  └─ Yes → Merge Sort or Heap Sort
  └─ No → Quick Sort (faster on average)

Is data integers in small range?
  └─ Yes → Counting Sort (O(n + k))
  └─ No → Standard comparison sorts
```

### **Practical Recommendations for Rust**

```rust
// ✅ DEFAULT: Use standard library
data.sort();           // Stable, optimized
data.sort_unstable();  // Faster when stability not needed

// ⚠️ RARELY implement custom sorting unless:
// - Educational purposes
// - Very specific constraints
// - Embedded systems with no std
```

### **For AoC / Competitive Programming**

```rust
// Simple cases
numbers.sort_unstable();  // Fast and simple

// Custom ordering
points.sort_unstable_by_key(|p| (p.x, p.y));

// Reverse sort
scores.sort_unstable_by(|a, b| b.cmp(a));
```

### **Original Decision Tree
   │  ├─ Yes: Merge Sort
   │  └─ No: Continue
   ├─ Memory Constrained?
   │  ├─ Yes: Heap Sort or In-place Quick Sort
   │  └─ No: Continue
   ├─ Nearly Sorted?
   │  ├─ Yes: Insertion Sort (adaptive)
   │  └─ No: Quick Sort or Introsort
   └─ Special Data Type?
      ├─ Integers (small range): Counting Sort
      ├─ Integers (large range): Radix Sort  
      └─ Uniform Distribution: Bucket Sort
```

## 📊 **Complexity Summary**

### **Time Complexity Comparison**

| Algorithm | Best | Average | Worst | Notes |
|-----------|------|---------|--------|-------|
| Bubble Sort | O(n) | O(n²) | O(n²) | With early termination |
| Selection Sort | O(n²) | O(n²) | O(n²) | Always n² comparisons |
| Insertion Sort | O(n) | O(n²) | O(n²) | Adaptive to input order |
| Merge Sort | O(n log n) | O(n log n) | O(n log n) | Consistent performance |
| Quick Sort | O(n log n) | O(n log n) | O(n²) | Pivot selection matters |
| Heap Sort | O(n log n) | O(n log n) | O(n log n) | Consistent, in-place |
| Counting Sort | O(n + k) | O(n + k) | O(n + k) | k = range of input |
| Radix Sort | O(d(n + k)) | O(d(n + k)) | O(d(n + k)) | d = number of digits |
| Bucket Sort | O(n + k) | O(n + k) | O(n²) | Depends on distribution |

### **Space Complexity Summary**

- **O(1)**: Bubble, Selection, Insertion, Heap Sort (in-place)
- **O(log n)**: Quick Sort (recursion stack)
- **O(n)**: Merge Sort (temporary arrays)
- **O(k)**: Counting Sort (count array)
- **O(n + k)**: Radix Sort, Bucket Sort

## 🔗 **Real-World Applications**

### **Database Systems**

- **External Merge Sort**: For datasets larger than memory
- **Index Sorting**: B-tree maintenance uses various algorithms
- **Query Optimization**: Sort-merge joins

### **Graphics & Games**

- **Z-buffer Sorting**: Depth sorting for rendering
- **Particle Systems**: Sorting by distance/priority
- **Collision Detection**: Spatial sorting

### **Data Processing**

- **ETL Pipelines**: Large dataset sorting
- **Analytics**: Percentile calculations, ranking
- **Search Engines**: Document ranking, index building

### **System Programming**

- **Process Scheduling**: Priority queue sorting
- **Memory Management**: Address sorting
- **Network Protocols**: Packet ordering

## 🧩 **Advanced Topics**

### **Parallel Sorting**

```rust
use rayon::prelude::*;

/// Parallel merge sort using Rayon
fn parallel_merge_sort<T: Ord + Clone + Send>(arr: &mut [T]) {
    if arr.len() <= 1000 {
        merge_sort(arr); // Sequential for small arrays
        return;
    }
    
    let mid = arr.len() / 2;
    let (left, right) = arr.split_at_mut(mid);
    
    // Parallel recursive calls
    rayon::join(
        || parallel_merge_sort(left),
        || parallel_merge_sort(right),
    );
    
    // Merge the sorted halves
    let left_clone = left.to_vec();
    let right_clone = right.to_vec();
    merge(arr, &left_clone, &right_clone);
}
```

### **External Sorting**

```rust
/// External sorting for datasets larger than memory
fn external_sort(input_file: &str, output_file: &str, memory_limit: usize) {
    // Phase 1: Create sorted runs
    let runs = create_sorted_runs(input_file, memory_limit);
    
    // Phase 2: Merge runs
    merge_runs(&runs, output_file);
}

fn create_sorted_runs(input_file: &str, memory_limit: usize) -> Vec<String> {
    // Read chunks of data, sort in memory, write to temporary files
    // Returns list of temporary file names
    todo!("Implementation depends on specific I/O requirements")
}

fn merge_runs(runs: &[String], output_file: &str) {
    // K-way merge using priority queue
    // Read from multiple sorted files simultaneously
    todo!("K-way merge implementation")
}
```

### **Stable vs Unstable Partitioning**

```rust
/// Stable partition (preserves relative order)
fn stable_partition<T, F>(arr: &mut [T], predicate: F) -> usize 
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    let mut true_elements = Vec::new();
    let mut false_elements = Vec::new();
    
    for item in arr.iter() {
        if predicate(item) {
            true_elements.push(item.clone());
        } else {
            false_elements.push(item.clone());
        }
    }
    
    let partition_point = true_elements.len();
    
    let mut index = 0;
    for item in true_elements {
        arr[index] = item;
        index += 1;
    }
    for item in false_elements {
        arr[index] = item;
        index += 1;
    }
    
    partition_point
}
```

## 🔍 **Testing & Verification**

### **Comprehensive Test Suite**

```rust
#[cfg(test)]
mod sorting_tests {
    use super::*;
    use rand::seq::SliceRandom;
    
    fn test_algorithm<F>(mut sort_fn: F, name: &str) 
    where
        F: FnMut(&mut [i32]),
    {
        // Test empty array
        let mut empty: Vec<i32> = vec![];
        sort_fn(&mut empty);
        assert!(empty.is_empty());
        
        // Test single element
        let mut single = vec![42];
        sort_fn(&mut single);
        assert_eq!(single, vec![42]);
        
        // Test already sorted
        let mut sorted = vec![1, 2, 3, 4, 5];
        sort_fn(&mut sorted);
        assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
        
        // Test reverse sorted
        let mut reverse = vec![5, 4, 3, 2, 1];
        sort_fn(&mut reverse);
        assert_eq!(reverse, vec![1, 2, 3, 4, 5]);
        
        // Test duplicates
        let mut duplicates = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        sort_fn(&mut duplicates);
        assert_eq!(duplicates, vec![1, 1, 2, 3, 4, 5, 5, 6, 9]);
        
        // Test random data
        for _ in 0..10 {
            let mut random: Vec<i32> = (0..100).collect();
            random.shuffle(&mut rand::thread_rng());
            let expected: Vec<i32> = (0..100).collect();
            
            sort_fn(&mut random);
            assert_eq!(random, expected, "Failed on random data for {}", name);
        }
    }
    
    #[test]
    fn test_all_sorts() {
        test_algorithm(bubble_sort, "Bubble Sort");
        test_algorithm(selection_sort, "Selection Sort");
        test_algorithm(insertion_sort, "Insertion Sort");
        test_algorithm(merge_sort, "Merge Sort");
        test_algorithm(quick_sort, "Quick Sort");
        test_algorithm(heap_sort, "Heap Sort");
    }
    
    #[test]
    fn test_stability() {
        #[derive(Debug, Clone, PartialEq, Eq)]
        struct Item {
            key: i32,
            id: usize,
        }
        
        impl Ord for Item {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.key.cmp(&other.key)
            }
        }
        
        impl PartialOrd for Item {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        
        let mut items = vec![
            Item { key: 1, id: 0 },
            Item { key: 2, id: 1 },
            Item { key: 1, id: 2 },
            Item { key: 2, id: 3 },
        ];
        
        // Test stable sort (merge sort)
        merge_sort(&mut items);
        
        // Items with same key should maintain relative order
        assert_eq!(items[0].id, 0); // First item with key=1
        assert_eq!(items[1].id, 2); // Second item with key=1
        assert_eq!(items[2].id, 1); // First item with key=2
        assert_eq!(items[3].id, 3); // Second item with key=2
    }
}
```

## 📚 **Further Reading & Resources**

### **Books**

- "Introduction to Algorithms" (CLRS) - Comprehensive algorithm analysis
- "Algorithm Design Manual" (Skiena) - Practical algorithm selection
- "Programming Pearls" (Bentley) - Real-world optimization techniques

### **Research Papers**

- "Introsort or Introspective Sort" - David Musser (1997)
- "Timsort" - Tim Peters (2002)
- "Engineering a Sort Function" - Bentley & McIlroy (1993)

### **Online Resources**

- [Sorting Algorithm Animations](https://www.sorting-algorithms.com/)
- [Big-O Cheat Sheet](https://www.bigocheatsheet.com/)
- [Rust Standard Library Sort](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort)

### **Implementation References**

- [Rust std::slice::sort](https://github.com/rust-lang/rust/blob/master/library/alloc/src/slice.rs)
- [Java Arrays.sort](https://github.com/openjdk/jdk/blob/master/src/java.base/share/classes/java/util/Arrays.java)
- [Python Timsort](https://github.com/python/cpython/blob/main/Objects/listobject.c)

## 🏷️ **Next Steps**

### **Practice Problems**

1. Implement a custom sorting algorithm for specific constraints
2. Optimize sorting for nearly-sorted data
3. Create a stable version of quick sort
4. Implement external sorting for large datasets
5. Build a parallel sorting algorithm

### **Related Topics**

- [[heap-data-structure]] - Binary heaps and priority queues
- [[binary-search-algorithms]] - Searching in sorted arrays  
- [[algorithm-analysis]] - Time and space complexity analysis
- [[data-structures-overview]] - Foundation data structures
- [[advanced-algorithms]] - More complex algorithmic techniques

---

*Tags: #sorting #algorithms #performance #complexity #rust #implementation #comparison #stable #in-place*

*Links: [[zettel-index]] | [[algorithm-analysis]] | [[data-structures-overview]] | [[heap-data-structure]] | [[binary-search-algorithms]] | [[performance-optimization]] | [[rust-collections]] | [[competitive-programming]]*

*Implementations: `advanced_examples/circle_sort/` (Circle Sort with benchmarks, bubble sort baseline, visualization callback)*
