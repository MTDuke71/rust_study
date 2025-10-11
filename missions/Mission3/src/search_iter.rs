/// Iterator-based search operations
///
/// # Requirements Satisfied
/// - REQ-3: Iterator integration for zero-cost search operations
/// - REQ-6: Range queries for AoC-style problems
use crate::binary_search::{search_left_bound, search_right_bound};

/// Iterator that yields elements within a specified range.
///
/// # Requirements Satisfied: REQ-3, REQ-6
pub struct RangeIter<'a, T> {
    slice: &'a [T],
    end: usize,
    current: usize,
}

impl<'a, T> RangeIter<'a, T> {
    pub fn new(slice: &'a [T], start: usize, end: usize) -> Self {
        Self {
            slice,
            end,
            current: start,
        }
    }
}

impl<'a, T> Iterator for RangeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end && self.current < self.slice.len() {
            let item = &self.slice[self.current];
            self.current += 1;
            Some(item)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = if self.current < self.end {
            self.end - self.current
        } else {
            0
        };
        (remaining, Some(remaining))
    }
}

impl<'a, T> ExactSizeIterator for RangeIter<'a, T> {}

/// Finds all elements in a sorted slice that are equal to the target.
///
/// # Requirements Satisfied: REQ-3, REQ-6
///
/// Returns an iterator over all matching elements. Uses binary search to find
/// the range efficiently in O(log n) time.
///
/// # Examples
/// ```
/// use mission3::search_iter::find_all_equal;
///
/// let data = [1, 2, 2, 2, 3, 4];
/// let twos: Vec<_> = find_all_equal(&data, &2).collect();
/// assert_eq!(twos, [&2, &2, &2]);
/// ```
pub fn find_all_equal<'a, T: Ord>(slice: &'a [T], target: &T) -> RangeIter<'a, T> {
    let left = search_left_bound(slice, target);
    let right = search_right_bound(slice, target);
    RangeIter::new(slice, left, right)
}

/// Finds all elements in a sorted slice within the specified range [min, max].
///
/// # Requirements Satisfied: REQ-3, REQ-6
///
/// Returns an iterator over all elements where min <= element <= max.
/// Uses binary search for efficient range location.
///
/// # Examples
/// ```
/// use mission3::search_iter::find_range;
///
/// let data = [1, 3, 5, 7, 9, 11, 13];
/// let middle: Vec<_> = find_range(&data, &4, &10).collect();
/// assert_eq!(middle, [&5, &7, &9]);
/// ```
pub fn find_range<'a, T: Ord>(slice: &'a [T], min: &T, max: &T) -> RangeIter<'a, T> {
    let left = search_left_bound(slice, min);
    let right = search_right_bound(slice, max);
    RangeIter::new(slice, left, right)
}

/// Finds the first element matching a predicate using binary search principles.
///
/// # Requirements Satisfied: REQ-3
///
/// The predicate must be monotonic - if it returns true for an element,
/// it must return true for all elements to the right.
///
/// # Examples
/// ```
/// use mission3::search_iter::find_first_matching;
///
/// let data = [1, 3, 5, 7, 9, 11];
/// let first_big = find_first_matching(&data, |&x| x >= 6);
/// assert_eq!(first_big, Some(&7));
/// ```
pub fn find_first_matching<T, P>(slice: &[T], predicate: P) -> Option<&T>
where
    P: Fn(&T) -> bool,
{
    let mut left = 0;
    let mut right = slice.len();

    while left < right {
        let mid = left + (right - left) / 2;

        if predicate(&slice[mid]) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    if left < slice.len() && predicate(&slice[left]) {
        Some(&slice[left])
    } else {
        None
    }
}

/// Extension trait to add search methods to slices.
///
/// # Requirements Satisfied: REQ-3
pub trait SearchExt<T> {
    /// Find all equal elements as an iterator.
    fn find_all_equal(&self, target: &T) -> RangeIter<'_, T>
    where
        T: Ord;

    /// Find elements in range [min, max] as an iterator.
    fn find_range(&self, min: &T, max: &T) -> RangeIter<'_, T>
    where
        T: Ord;

    /// Find first element matching predicate.
    fn find_first_matching<P>(&self, predicate: P) -> Option<&T>
    where
        P: Fn(&T) -> bool;
}

impl<T> SearchExt<T> for [T] {
    fn find_all_equal(&self, target: &T) -> RangeIter<'_, T>
    where
        T: Ord,
    {
        find_all_equal(self, target)
    }

    fn find_range(&self, min: &T, max: &T) -> RangeIter<'_, T>
    where
        T: Ord,
    {
        find_range(self, min, max)
    }

    fn find_first_matching<P>(&self, predicate: P) -> Option<&T>
    where
        P: Fn(&T) -> bool,
    {
        find_first_matching(self, predicate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_iter_basic() {
        let data = [1, 2, 3, 4, 5];
        let mut iter = RangeIter::new(&data, 1, 4);

        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn find_all_equal_works() {
        let data = [1, 2, 2, 2, 3, 4];
        let result: Vec<_> = find_all_equal(&data, &2).collect();
        assert_eq!(result, [&2, &2, &2]);
    }

    #[test]
    fn find_range_works() {
        let data = [1, 3, 5, 7, 9, 11, 13];
        let result: Vec<_> = find_range(&data, &4, &10).collect();
        assert_eq!(result, [&5, &7, &9]);
    }

    #[test]
    fn find_first_matching_works() {
        let data = [1, 3, 5, 7, 9, 11];
        assert_eq!(find_first_matching(&data, |&x| x >= 6), Some(&7));
        assert_eq!(find_first_matching(&data, |&x| x >= 20), None);
    }

    #[test]
    fn search_ext_trait_works() {
        let data = [1, 2, 2, 2, 3, 4];
        let twos: Vec<_> = data.find_all_equal(&2).collect();
        assert_eq!(twos, [&2, &2, &2]);
    }
}
