/// Trait abstraction for searchable containers
/// 
/// # Requirements Satisfied
/// - REQ-2: Generic trait-based search abstraction
/// - REQ-5: Lifetime-safe borrowing patterns

use std::cmp::Ordering;

/// Trait for containers that support binary search operations.
///
/// # Requirements Satisfied: REQ-2
///
/// This trait abstracts over different container types, allowing binary search
/// to work with slices, vectors, arrays, and custom data structures.
pub trait Searchable {
    type Item;
    type Index: Copy + Ord + Into<usize> + From<usize>;
    
    /// Returns the number of elements in the container.
    fn len(&self) -> Self::Index;
    
    /// Gets a reference to the element at the given index.
    /// Returns None if the index is out of bounds.
    fn get(&self, index: Self::Index) -> Option<&Self::Item>;
    
    /// Returns true if the container is empty.
    fn is_empty(&self) -> bool {
        self.len() == Self::Index::from(0)
    }
}

/// Implementation of Searchable for slices.
///
/// # Requirements Satisfied: REQ-2, REQ-5
impl<T> Searchable for [T] {
    type Item = T;
    type Index = usize;
    
    fn len(&self) -> Self::Index {
        <[T]>::len(self)
    }
    
    fn get(&self, index: Self::Index) -> Option<&Self::Item> {
        <[T]>::get(self, index)
    }
}

/// Implementation of Searchable for Vecs.
///
/// # Requirements Satisfied: REQ-2
impl<T> Searchable for Vec<T> {
    type Item = T;
    type Index = usize;
    
    fn len(&self) -> Self::Index {
        Vec::len(self)
    }
    
    fn get(&self, index: Self::Index) -> Option<&Self::Item> {
        self.as_slice().get(index)
    }
}

/// Implementation of Searchable for arrays.
///
/// # Requirements Satisfied: REQ-2
impl<T, const N: usize> Searchable for [T; N] {
    type Item = T;
    type Index = usize;
    
    fn len(&self) -> Self::Index {
        N
    }
    
    fn get(&self, index: Self::Index) -> Option<&Self::Item> {
        <[T]>::get(self, index)
    }
}

/// Generic binary search on any Searchable container.
///
/// # Requirements Satisfied: REQ-2, REQ-5
///
/// # Examples
/// ```
/// use mission3::searchable::{search_in};
/// 
/// let data = vec![1, 3, 5, 7, 9];
/// assert_eq!(search_in(&data, &5), Ok(2));
/// 
/// let slice_data = [1, 3, 5, 7, 9];
/// assert_eq!(search_in(&slice_data, &5), Ok(2));
/// ```
pub fn search_in<S, T>(container: &S, target: &T) -> Result<S::Index, S::Index>
where
    S: Searchable<Item = T> + ?Sized,
    T: Ord,
{
    let mut left = S::Index::from(0);
    let mut right = container.len();
    
    while left < right {
        let mid_usize = (left.into() + right.into()) / 2;
        let mid = S::Index::from(mid_usize);
        
        // SAFETY: mid is always within bounds due to our calculation
        let mid_value = container.get(mid).expect("mid should be within bounds");
        
        match mid_value.cmp(target) {
            Ordering::Equal => return Ok(mid),
            Ordering::Less => left = S::Index::from(mid.into() + 1),
            Ordering::Greater => right = mid,
        }
    }
    
    Err(left)
}

/// Generic binary search with custom key extraction.
///
/// # Requirements Satisfied: REQ-2, REQ-4
pub fn search_in_by_key<S, T, K, F>(
    container: &S, 
    target: &K, 
    key_fn: F
) -> Result<S::Index, S::Index>
where
    S: Searchable<Item = T> + ?Sized,
    K: Ord,
    F: Fn(&T) -> K,
{
    let mut left = S::Index::from(0);
    let mut right = container.len();
    
    while left < right {
        let mid_usize = (left.into() + right.into()) / 2;
        let mid = S::Index::from(mid_usize);
        
        let mid_value = container.get(mid).expect("mid should be within bounds");
        let mid_key = key_fn(mid_value);
        
        match mid_key.cmp(target) {
            Ordering::Equal => return Ok(mid),
            Ordering::Less => left = S::Index::from(mid.into() + 1),
            Ordering::Greater => right = mid,
        }
    }
    
    Err(left)
}

/// Wrapper for searching with a custom comparator on Searchable containers.
///
/// # Requirements Satisfied: REQ-2, REQ-4
pub fn search_in_by<S, T, F>(
    container: &S, 
    target: &T, 
    compare: F
) -> Result<S::Index, S::Index>
where
    S: Searchable<Item = T> + ?Sized,
    F: Fn(&T, &T) -> Ordering,
{
    let mut left = S::Index::from(0);
    let mut right = container.len();
    
    while left < right {
        let mid_usize = (left.into() + right.into()) / 2;
        let mid = S::Index::from(mid_usize);
        
        let mid_value = container.get(mid).expect("mid should be within bounds");
        
        match compare(mid_value, target) {
            Ordering::Equal => return Ok(mid),
            Ordering::Less => left = S::Index::from(mid.into() + 1),
            Ordering::Greater => right = mid,
        }
    }
    
    Err(left)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn searchable_slice_basic() {
        let data = [1, 3, 5, 7, 9];
        let slice: &[i32] = &data;
        assert_eq!(search_in(slice, &5), Ok(2));
        assert_eq!(search_in(slice, &6), Err(3));
    }

    #[test]
    fn searchable_vec_basic() {
        let data = vec![1, 3, 5, 7, 9];
        assert_eq!(search_in(&data, &5), Ok(2));
        assert_eq!(search_in(&data, &6), Err(3));
    }

    #[test]
    fn searchable_array_basic() {
        let data = [1, 3, 5, 7, 9];
        assert_eq!(search_in(&data, &5), Ok(2));
        assert_eq!(search_in(&data, &6), Err(3));
    }

    #[test]
    fn searchable_by_key() {
        let people = vec![(1, "Alice"), (3, "Bob"), (5, "Charlie")];
        assert_eq!(search_in_by_key(&people, &3, |p| p.0), Ok(1));
        assert_eq!(search_in_by_key(&people, &4, |p| p.0), Err(2));
    }
}