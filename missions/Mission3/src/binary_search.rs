/// Binary search implementation for slices and trait-based containers
/// 
/// # Requirements Satisfied
/// - REQ-1: Slice-based binary search with O(log n) complexity
/// - REQ-4: Custom ordering support via key extraction
/// - REQ-5: Lifetime-safe borrowing patterns

use std::cmp::Ordering;

/// Performs binary search on a sorted slice.
///
/// # Requirements Satisfied: REQ-1, REQ-5
/// 
/// Returns `Ok(index)` if the element is found, `Err(insertion_index)` otherwise.
/// The insertion index indicates where the element should be inserted to maintain ordering.
///
/// # Time Complexity: O(log n)
/// # Space Complexity: O(1)
///
/// # Examples
/// ```
/// use mission3::binary_search::search_slice;
/// 
/// let data = [1, 3, 5, 7, 9];
/// assert_eq!(search_slice(&data, &5), Ok(2));
/// assert_eq!(search_slice(&data, &6), Err(3));
/// ```
pub fn search_slice<T: Ord>(slice: &[T], target: &T) -> Result<usize, usize> {
    let mut left = 0;
    let mut right = slice.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        
        // SAFETY: mid is always within bounds due to our loop invariants
        match slice[mid].cmp(target) {
            Ordering::Equal => return Ok(mid),
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid,
        }
    }
    
    Err(left)
}

/// Performs binary search using a custom key extraction function.
///
/// # Requirements Satisfied: REQ-4, REQ-5
///
/// This allows searching collections by a specific field or transformed value.
///
/// # Examples
/// ```
/// use mission3::binary_search::search_by_key;
/// 
/// let people = [(1, "Alice"), (2, "Bob"), (3, "Charlie")];
/// assert_eq!(search_by_key(&people, &2, |person| person.0), Ok(1));
/// ```
pub fn search_by_key<T, K, F>(slice: &[T], target: &K, key_fn: F) -> Result<usize, usize>
where
    K: Ord,
    F: Fn(&T) -> K,
{
    let mut left = 0;
    let mut right = slice.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        
        match key_fn(&slice[mid]).cmp(target) {
            Ordering::Equal => return Ok(mid),
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid,
        }
    }
    
    Err(left)
}

/// Performs binary search with a custom comparator function.
///
/// # Requirements Satisfied: REQ-4
///
/// Provides maximum flexibility for custom ordering logic.
///
/// # Examples
/// ```
/// use mission3::binary_search::search_by;
/// use std::cmp::Ordering;
/// 
/// let data = [5, 4, 3, 2, 1]; // reverse sorted
/// let result = search_by(&data, &3, |a, b| b.cmp(a)); // reverse comparison
/// assert_eq!(result, Ok(2));
/// ```
pub fn search_by<T, F>(slice: &[T], target: &T, compare: F) -> Result<usize, usize>
where
    F: Fn(&T, &T) -> Ordering,
{
    let mut left = 0;
    let mut right = slice.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        
        match compare(&slice[mid], target) {
            Ordering::Equal => return Ok(mid),
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid,
        }
    }
    
    Err(left)
}

/// Finds the leftmost occurrence of target in a sorted slice.
///
/// # Requirements Satisfied: REQ-1, REQ-6
///
/// Useful for finding range boundaries in AoC problems.
pub fn search_left_bound<T: Ord>(slice: &[T], target: &T) -> usize {
    let mut left = 0;
    let mut right = slice.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        
        if slice[mid] < *target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    
    left
}

/// Finds the rightmost occurrence of target in a sorted slice.
///
/// # Requirements Satisfied: REQ-1, REQ-6
///
/// Returns the index after the last occurrence, useful for range operations.
pub fn search_right_bound<T: Ord>(slice: &[T], target: &T) -> usize {
    let mut left = 0;
    let mut right = slice.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        
        if slice[mid] <= *target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    
    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_empty_slice() {
        let empty: &[i32] = &[];
        assert_eq!(search_slice(empty, &5), Err(0));
    }

    #[test]
    fn search_single_element() {
        let single = [42];
        assert_eq!(search_slice(&single, &42), Ok(0));
        assert_eq!(search_slice(&single, &41), Err(0));
        assert_eq!(search_slice(&single, &43), Err(1));
    }

    #[test]
    fn search_basic_cases() {
        let data = [1, 3, 5, 7, 9, 11, 13];
        
        // Found cases
        assert_eq!(search_slice(&data, &1), Ok(0));
        assert_eq!(search_slice(&data, &7), Ok(3));
        assert_eq!(search_slice(&data, &13), Ok(6));
        
        // Not found cases
        assert_eq!(search_slice(&data, &0), Err(0));
        assert_eq!(search_slice(&data, &4), Err(2));
        assert_eq!(search_slice(&data, &14), Err(7));
    }
}