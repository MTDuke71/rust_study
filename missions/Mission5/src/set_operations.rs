use std::collections::HashSet;
use std::hash::Hash;

/// Set operations wrapper around `HashSet<T>` with mathematical operations
///
/// Provides convenient methods for set theory operations including
/// union, intersection, difference, and symmetric difference.
/// Optimized for membership testing and deduplication.
///
/// # Requirements Satisfied: REQ-2
///
/// # Examples
///
/// ```rust
/// use mission5::SetOperations;
///
/// let set1 = SetOperations::from_iter([1, 2, 3, 4]);
/// let set2 = SetOperations::from_iter([3, 4, 5, 6]);
///
/// let union = set1.union(&set2);
/// assert_eq!(union.len(), 6); // {1, 2, 3, 4, 5, 6}
///
/// let intersection = set1.intersection(&set2);
/// assert_eq!(intersection.len(), 2); // {3, 4}
/// ```
#[derive(Debug, Clone)]
pub struct SetOperations<T>
where
    T: Eq + Hash,
{
    inner: HashSet<T>,
}

impl<T> SetOperations<T>
where
    T: Eq + Hash,
{
    /// Create a new empty set
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission5::SetOperations;
    ///
    /// let set: SetOperations<i32> = SetOperations::new();
    /// assert!(set.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            inner: HashSet::new(),
        }
    }

    /// Create a set with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: HashSet::with_capacity(capacity),
        }
    }

    /// Insert an element into the set
    ///
    /// Returns `true` if the element was newly inserted
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission5::SetOperations;
    ///
    /// let mut set = SetOperations::new();
    /// assert!(set.insert(1));  // New element
    /// assert!(!set.insert(1)); // Already exists
    /// ```
    pub fn insert(&mut self, value: T) -> bool {
        self.inner.insert(value)
    }

    /// Check if set contains an element
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission5::SetOperations;
    ///
    /// let mut set = SetOperations::new();
    /// set.insert(42);
    ///
    /// assert!(set.contains(&42));
    /// assert!(!set.contains(&24));
    /// ```
    pub fn contains(&self, value: &T) -> bool {
        self.inner.contains(value)
    }

    /// Remove an element from the set
    ///
    /// Returns `true` if the element was present
    pub fn remove(&mut self, value: &T) -> bool {
        self.inner.remove(value)
    }

    /// Get the number of elements in the set
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if the set is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Clear all elements from the set
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    /// Get an iterator over the elements
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }
}

impl<T> SetOperations<T>
where
    T: Eq + Hash + Clone,
{
    /// Create the union of two sets (all elements from both)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission5::SetOperations;
    ///
    /// let set1 = SetOperations::from_iter([1, 2, 3]);
    /// let set2 = SetOperations::from_iter([3, 4, 5]);
    ///
    /// let union = set1.union(&set2);
    /// assert_eq!(union.len(), 5); // {1, 2, 3, 4, 5}
    /// ```
    pub fn union(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for item in other.iter() {
            result.insert(item.clone());
        }
        result
    }

    /// Create the intersection of two sets (common elements)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission5::SetOperations;
    ///
    /// let set1 = SetOperations::from_iter([1, 2, 3, 4]);
    /// let set2 = SetOperations::from_iter([3, 4, 5, 6]);
    ///
    /// let intersection = set1.intersection(&set2);
    /// assert_eq!(intersection.len(), 2); // {3, 4}
    /// assert!(intersection.contains(&3));
    /// assert!(intersection.contains(&4));
    /// ```
    pub fn intersection(&self, other: &Self) -> Self {
        let mut result = SetOperations::new();
        for item in self.iter() {
            if other.contains(item) {
                result.insert(item.clone());
            }
        }
        result
    }

    /// Create the difference of two sets (elements in first but not second)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission5::SetOperations;
    ///
    /// let set1 = SetOperations::from_iter([1, 2, 3, 4]);
    /// let set2 = SetOperations::from_iter([3, 4, 5, 6]);
    ///
    /// let difference = set1.difference(&set2);
    /// assert_eq!(difference.len(), 2); // {1, 2}
    /// assert!(difference.contains(&1));
    /// assert!(difference.contains(&2));
    /// ```
    pub fn difference(&self, other: &Self) -> Self {
        let mut result = SetOperations::new();
        for item in self.iter() {
            if !other.contains(item) {
                result.insert(item.clone());
            }
        }
        result
    }

    /// Create the symmetric difference of two sets (elements in either but not both)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission5::SetOperations;
    ///
    /// let set1 = SetOperations::from_iter([1, 2, 3]);
    /// let set2 = SetOperations::from_iter([3, 4, 5]);
    ///
    /// let sym_diff = set1.symmetric_difference(&set2);
    /// assert_eq!(sym_diff.len(), 4); // {1, 2, 4, 5}
    /// ```
    pub fn symmetric_difference(&self, other: &Self) -> Self {
        let mut result = SetOperations::new();

        // Add elements from self that are not in other
        for item in self.iter() {
            if !other.contains(item) {
                result.insert(item.clone());
            }
        }

        // Add elements from other that are not in self
        for item in other.iter() {
            if !self.contains(item) {
                result.insert(item.clone());
            }
        }

        result
    }

    /// Check if this set is a subset of another
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission5::SetOperations;
    ///
    /// let set1 = SetOperations::from_iter([1, 2]);
    /// let set2 = SetOperations::from_iter([1, 2, 3, 4]);
    ///
    /// assert!(set1.is_subset(&set2));
    /// assert!(!set2.is_subset(&set1));
    /// ```
    pub fn is_subset(&self, other: &Self) -> bool {
        self.iter().all(|item| other.contains(item))
    }

    /// Check if this set is a superset of another
    pub fn is_superset(&self, other: &Self) -> bool {
        other.is_subset(self)
    }

    /// Check if two sets are disjoint (no common elements)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission5::SetOperations;
    ///
    /// let set1 = SetOperations::from_iter([1, 2]);
    /// let set2 = SetOperations::from_iter([3, 4]);
    /// let set3 = SetOperations::from_iter([2, 3]);
    ///
    /// assert!(set1.is_disjoint(&set2));
    /// assert!(!set1.is_disjoint(&set3));
    /// ```
    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.iter().all(|item| !other.contains(item))
    }

    /// Insert multiple elements from an iterator
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission5::SetOperations;
    ///
    /// let mut set = SetOperations::new();
    /// set.insert_many([1, 2, 3, 2, 1]); // Duplicates ignored
    ///
    /// assert_eq!(set.len(), 3);
    /// assert!(set.contains(&1));
    /// assert!(set.contains(&2));
    /// assert!(set.contains(&3));
    /// ```
    pub fn insert_many<I>(&mut self, items: I)
    where
        I: IntoIterator<Item = T>,
    {
        for item in items {
            self.insert(item);
        }
    }

    /// Get all elements as a vector
    pub fn to_vec(&self) -> Vec<T> {
        self.iter().cloned().collect()
    }
}

impl<T> Default for SetOperations<T>
where
    T: Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for SetOperations<T>
where
    T: Eq + Hash,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = SetOperations::new();
        for item in iter {
            set.insert(item);
        }
        set
    }
}

impl<T> IntoIterator for SetOperations<T>
where
    T: Eq + Hash,
{
    type Item = T;
    type IntoIter = std::collections::hash_set::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<T> Extend<T> for SetOperations<T>
where
    T: Eq + Hash,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.inner.extend(iter);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut set = SetOperations::new();

        assert!(set.insert(1));
        assert!(set.insert(2));
        assert!(!set.insert(1)); // Already exists

        assert_eq!(set.len(), 2);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(!set.contains(&3));

        assert!(set.remove(&1));
        assert!(!set.remove(&3)); // Doesn't exist
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_union() {
        let set1 = SetOperations::from_iter([1, 2, 3]);
        let set2 = SetOperations::from_iter([3, 4, 5]);

        let union = set1.union(&set2);
        assert_eq!(union.len(), 5);

        for i in 1..=5 {
            assert!(union.contains(&i));
        }
    }

    #[test]
    fn test_intersection() {
        let set1 = SetOperations::from_iter([1, 2, 3, 4]);
        let set2 = SetOperations::from_iter([3, 4, 5, 6]);

        let intersection = set1.intersection(&set2);
        assert_eq!(intersection.len(), 2);
        assert!(intersection.contains(&3));
        assert!(intersection.contains(&4));
        assert!(!intersection.contains(&1));
        assert!(!intersection.contains(&5));
    }

    #[test]
    fn test_difference() {
        let set1 = SetOperations::from_iter([1, 2, 3, 4]);
        let set2 = SetOperations::from_iter([3, 4, 5, 6]);

        let diff = set1.difference(&set2);
        assert_eq!(diff.len(), 2);
        assert!(diff.contains(&1));
        assert!(diff.contains(&2));
        assert!(!diff.contains(&3));
        assert!(!diff.contains(&4));
    }

    #[test]
    fn test_symmetric_difference() {
        let set1 = SetOperations::from_iter([1, 2, 3]);
        let set2 = SetOperations::from_iter([3, 4, 5]);

        let sym_diff = set1.symmetric_difference(&set2);
        assert_eq!(sym_diff.len(), 4);
        assert!(sym_diff.contains(&1));
        assert!(sym_diff.contains(&2));
        assert!(!sym_diff.contains(&3)); // Common element
        assert!(sym_diff.contains(&4));
        assert!(sym_diff.contains(&5));
    }

    #[test]
    fn test_subset_superset() {
        let small = SetOperations::from_iter([1, 2]);
        let large = SetOperations::from_iter([1, 2, 3, 4]);
        let other = SetOperations::from_iter([2, 3]);

        assert!(small.is_subset(&large));
        assert!(!large.is_subset(&small));

        assert!(large.is_superset(&small));
        assert!(!small.is_superset(&large));

        assert!(!small.is_subset(&other)); // {1,2} not subset of {2,3}
    }

    #[test]
    fn test_disjoint() {
        let set1 = SetOperations::from_iter([1, 2]);
        let set2 = SetOperations::from_iter([3, 4]);
        let set3 = SetOperations::from_iter([2, 3]);

        assert!(set1.is_disjoint(&set2));
        assert!(!set1.is_disjoint(&set3)); // Share element 2
    }

    #[test]
    fn test_insert_many() {
        let mut set = SetOperations::new();
        set.insert_many([1, 2, 3, 2, 1]); // Duplicates should be ignored

        assert_eq!(set.len(), 3);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
    }

    #[test]
    fn test_from_iterator() {
        let items = vec![1, 2, 3, 2, 1];
        let set: SetOperations<i32> = items.into_iter().collect();

        assert_eq!(set.len(), 3);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
    }

    #[test]
    fn test_to_vec() {
        let set = SetOperations::from_iter([3, 1, 2]);
        let mut vec = set.to_vec();
        vec.sort(); // HashSet doesn't guarantee order

        assert_eq!(vec, vec![1, 2, 3]);
    }
}
