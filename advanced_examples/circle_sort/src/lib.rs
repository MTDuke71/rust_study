//! # Circle Sort
//!
//! A sorting algorithm invented by a programmer on a coding forum — no academic
//! paper, no formal proof at first. Someone posted a weird idea about bending an
//! array into a circle and comparing mirror pairs, and it turned out to actually
//! work with O(n log n) average-case performance.
//!
//! ## How It Works
//!
//! 1. **Circle comparison**: Imagine the array bent into a circle. Compare element
//!    pairs mirrored across the diameter: first↔last, second↔second-to-last, etc.
//!    Swap if the left element is greater than the right.
//!
//! 2. **Middle element**: If the subarray has odd length, the middle element has
//!    no mirror partner — skip it.
//!
//! 3. **Recursive split**: After comparing all mirror pairs, split into left and
//!    right halves and recursively circle-sort each half.
//!
//! 4. **Repeat passes**: One full recursive pass may not fully sort the array.
//!    Repeat the entire process until a pass produces zero swaps.
//!
//! ## Complexity
//!
//! | Case     | Time           | Space    |
//! |----------|----------------|----------|
//! | Best     | O(n log n)     | O(log n) |
//! | Average  | O(n log n)     | O(log n) |
//! | Worst    | O(n log² n)    | O(log n) |
//!
//! Space is O(log n) for recursion depth only — the sort is in-place.
//!
//! ## Origin Story
//!
//! Circle Sort was posted on a programming forum by an anonymous programmer
//! experimenting with geometric intuitions about sorting. The community verified
//! it works, analyzed its complexity, and found it genuinely performs at
//! O(n log n) on average. It's one of the few sorting algorithms discovered
//! through grassroots experimentation rather than academic research.
//!
//! ## Quick Start
//!
//! ```rust
//! use circle_sort::circle_sort;
//!
//! let mut data = vec![6, 5, 3, 1, 8, 7, 2, 4];
//! circle_sort(&mut data);
//! assert_eq!(data, vec![1, 2, 3, 4, 5, 6, 7, 8]);
//! ```

/// Sorts a slice in-place using the Circle Sort algorithm.
///
/// Repeatedly applies recursive circular comparison passes until
/// the array is fully sorted (zero swaps in a pass).
///
/// # Examples
///
/// ```
/// use circle_sort::circle_sort;
///
/// let mut v = vec![4, 3, 2, 1];
/// circle_sort(&mut v);
/// assert_eq!(v, [1, 2, 3, 4]);
/// ```
pub fn circle_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    // Repeat full passes until no swaps occur
    while circle_sort_pass(arr, 0, arr.len() - 1) > 0 {}
}

/// Sorts with a callback invoked after every swap, useful for visualization.
///
/// The callback receives `(index_a, index_b, current_slice_state)` after each swap.
///
/// # Examples
///
/// ```
/// use circle_sort::circle_sort_with_callback;
///
/// let mut v = vec![3, 1, 2];
/// let mut swaps = Vec::new();
/// circle_sort_with_callback(&mut v, |a, b, _state| {
///     swaps.push((a, b));
/// });
/// assert_eq!(v, [1, 2, 3]);
/// assert!(!swaps.is_empty());
/// ```
pub fn circle_sort_with_callback<T: Ord + Clone>(
    arr: &mut [T],
    mut on_swap: impl FnMut(usize, usize, &[T]),
) {
    if arr.len() <= 1 {
        return;
    }
    loop {
        let swaps = circle_sort_pass_cb(arr, 0, arr.len() - 1, &mut on_swap);
        if swaps == 0 {
            break;
        }
    }
}

/// One recursive pass of circle sort. Returns the number of swaps performed.
///
/// This is the core algorithm: compare mirror pairs across the "circle diameter",
/// then recurse on each half.
fn circle_sort_pass<T: Ord>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    if lo >= hi {
        return 0;
    }

    let mut swaps = 0;
    let mut left = lo;
    let mut right = hi;

    // Compare mirror pairs: outside-in
    while left < right {
        if arr[left] > arr[right] {
            arr.swap(left, right);
            swaps += 1;
        }
        left += 1;
        right -= 1;
    }

    // Odd-length subarray: middle element compared with itself
    // Special case — if left == right and it equals the next element boundary,
    // we still need to check (handles some edge cases in the original algorithm)
    if left == right && left < hi && arr[left] > arr[left + 1] {
        arr.swap(left, left + 1);
        swaps += 1;
    }

    // Recursive split: circle-sort each half
    let mid = lo + (hi - lo) / 2;
    swaps += circle_sort_pass(arr, lo, mid);
    swaps += circle_sort_pass(arr, mid + 1, hi);

    swaps
}

/// One recursive pass with swap callback. Returns number of swaps.
fn circle_sort_pass_cb<T: Ord + Clone>(
    arr: &mut [T],
    lo: usize,
    hi: usize,
    on_swap: &mut impl FnMut(usize, usize, &[T]),
) -> usize {
    if lo >= hi {
        return 0;
    }

    let mut swaps = 0;
    let mut left = lo;
    let mut right = hi;

    while left < right {
        if arr[left] > arr[right] {
            arr.swap(left, right);
            swaps += 1;
            on_swap(left, right, arr);
        }
        left += 1;
        right -= 1;
    }

    if left == right && left < hi && arr[left] > arr[left + 1] {
        arr.swap(left, left + 1);
        swaps += 1;
        on_swap(left, left + 1, arr);
    }

    let mid = lo + (hi - lo) / 2;
    swaps += circle_sort_pass_cb(arr, lo, mid, on_swap);
    swaps += circle_sort_pass_cb(arr, mid + 1, hi, on_swap);

    swaps
}

/// Bubble Sort — O(n²) baseline for benchmarking comparison.
///
/// Classic quadratic sort: repeatedly walk the array, swapping adjacent
/// elements that are out of order, until a full pass produces no swaps.
///
/// Included here as a concrete demonstration of how O(n²) diverges
/// from O(n log n) as input size grows.
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    for i in 0..n {
        let mut swapped = false;
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_empty() {
        let mut v: Vec<i32> = vec![];
        circle_sort(&mut v);
        assert_eq!(v, vec![]);
    }

    #[test]
    fn sorts_single() {
        let mut v = vec![42];
        circle_sort(&mut v);
        assert_eq!(v, vec![42]);
    }

    #[test]
    fn sorts_two_elements() {
        let mut v = vec![2, 1];
        circle_sort(&mut v);
        assert_eq!(v, vec![1, 2]);
    }

    #[test]
    fn sorts_already_sorted() {
        let mut v = vec![1, 2, 3, 4, 5];
        circle_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn sorts_reverse() {
        let mut v = vec![5, 4, 3, 2, 1];
        circle_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn sorts_with_duplicates() {
        let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        circle_sort(&mut v);
        assert_eq!(v, vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
    }

    #[test]
    fn sorts_power_of_two_length() {
        let mut v = vec![8, 6, 7, 5, 3, 0, 9, 1];
        circle_sort(&mut v);
        assert_eq!(v, vec![0, 1, 3, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn sorts_odd_length() {
        let mut v = vec![9, 7, 5, 3, 1, 2, 4, 6, 8];
        circle_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn sorts_all_same() {
        let mut v = vec![7, 7, 7, 7, 7];
        circle_sort(&mut v);
        assert_eq!(v, vec![7, 7, 7, 7, 7]);
    }

    #[test]
    fn callback_fires_on_swaps() {
        let mut v = vec![4, 3, 2, 1];
        let mut swap_count = 0;
        circle_sort_with_callback(&mut v, |_a, _b, _state| {
            swap_count += 1;
        });
        assert_eq!(v, [1, 2, 3, 4]);
        assert!(swap_count > 0);
    }

    #[test]
    fn sorts_large_random() {
        // Deterministic "random" sequence
        let mut v: Vec<i32> = (0..100).rev().collect();
        // Interleave to create disorder
        for i in (0..100).step_by(3) {
            if i + 2 < 100 {
                v.swap(i, i + 2);
            }
        }
        circle_sort(&mut v);
        let expected: Vec<i32> = (0..100).collect();
        assert_eq!(v, expected);
    }
}
