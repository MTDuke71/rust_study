//! # Mission 4: Singly Linked List - Interior Mutability and Ownership
//!
//! This library implements singly linked lists in Rust, demonstrating why this fundamental
//! data structure presents unique challenges in Rust's ownership system. We explore multiple
//! implementation approaches, from simple Box-based ownership to advanced patterns using
//! reference counting and interior mutability.
//!
//! ## Requirements Fulfilled
//!
//! - **REQ-1**: Basic linked list structure with owned pointers
//! - **REQ-2**: Interior mutability with `Rc<RefCell<T>>` patterns
//! - **REQ-3**: Core list operations (push, pop, peek)
//! - **REQ-4**: Safe iteration patterns
//! - **REQ-5**: Memory management comparisons
//! - **REQ-6**: Weak references for cycle prevention
//!
//! ## Quick Start
//!
//! ```rust
//! use mission4::{SimpleLinkedList, RcLinkedList};
//!
//! // Box-based simple implementation
//! let mut simple_list = SimpleLinkedList::new();
//! simple_list.push_front(42);
//! simple_list.push_front(24);
//! assert_eq!(simple_list.pop_front(), Some(24));
//!
//! // Rc-based flexible implementation
//! let mut rc_list = RcLinkedList::new();
//! rc_list.push_front("Hello");
//! rc_list.push_front("World");
//! // Use try_peek_front for RcLinkedList
//! match rc_list.try_peek_front() {
//!     Ok(Some(value)) => assert_eq!(*value, "World"),
//!     _ => panic!("Expected value"),
//! };
//! ```
//!
//! ## Why Linked Lists are Challenging in Rust
//!
//! Linked lists demonstrate several core Rust concepts:
//!
//! 1. **Ownership Conflicts**: Traditional linked lists require aliasing
//! 2. **Interior Mutability**: Modifying through shared references
//! 3. **Reference Cycles**: Potential memory leaks in cyclic structures
//! 4. **Iterator Safety**: Preventing use-after-free in traversal
//!
//! ## Performance Characteristics
//!
//! | Operation | Box Implementation | Rc Implementation |
//! |-----------|-------------------|-------------------|
//! | `push_front` | O(1) | O(1) |
//! | `pop_front` | O(1) | O(1) |
//! | `peek_front` | O(1) | O(1) |
//! | Memory overhead | Minimal | Higher (ref counting) |
//!
//! ## Use Cases
//!
//! - **Educational**: Understanding Rust ownership patterns
//! - **Competitive Programming**: Custom data structures for specific algorithms
//! - **Graph Algorithms**: Node-based representations
//! - **Parser Implementation**: Recursive data structures

use std::rc::{Rc, Weak};
use std::cell::{RefCell, Ref, RefMut};
use std::fmt;

/// Simple linked list using Box for owned pointers
/// 
/// This implementation demonstrates the most straightforward approach to linked lists
/// in Rust, using `Box<T>` for heap allocation and owned pointers.
/// 
/// # Requirements Satisfied: REQ-1, REQ-3, REQ-5
/// 
/// # Examples
/// 
/// ```
/// use mission4::SimpleLinkedList;
/// 
/// let mut list = SimpleLinkedList::new();
/// list.push_front(1);
/// list.push_front(2);
/// assert_eq!(list.pop_front(), Some(2));
/// assert_eq!(list.len(), 1);
/// ```
#[derive(Debug)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

/// Node for the simple linked list implementation
#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

/// Linked list using Rc<RefCell<T>> for shared ownership and interior mutability
/// 
/// This implementation demonstrates advanced Rust patterns for scenarios requiring
/// shared ownership of nodes. Uses reference counting and interior mutability.
/// 
/// # Requirements Satisfied: REQ-2, REQ-3, REQ-5, REQ-6
/// 
/// # Examples
/// 
/// ```
/// use mission4::RcLinkedList;
/// 
/// let mut list = RcLinkedList::new();
/// list.push_front("hello");
/// list.push_front("world");
/// assert_eq!(list.len(), 2);
/// ```
#[derive(Debug)]
pub struct RcLinkedList<T> {
    head: Option<Rc<RefCell<RcNode<T>>>>,
    length: usize,
}

/// Node for the Rc-based linked list implementation
#[derive(Debug)]
pub struct RcNode<T> {
    pub data: T,
    pub next: Option<Rc<RefCell<RcNode<T>>>>,
    // For advanced patterns, we might add a weak back-reference
    pub prev_weak: Option<Weak<RefCell<RcNode<T>>>>,
}

/// Error types for linked list operations
#[derive(Debug, Clone, PartialEq)]
pub enum LinkedListError {
    /// Attempted to borrow a node that is already mutably borrowed
    BorrowError,
    /// Node still has multiple references and cannot be unwrapped
    MultipleReferences,
    /// Operation attempted on empty list
    EmptyList,
}

impl fmt::Display for LinkedListError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinkedListError::BorrowError => write!(f, "Node is already mutably borrowed"),
            LinkedListError::MultipleReferences => write!(f, "Node still has multiple references"),
            LinkedListError::EmptyList => write!(f, "Operation attempted on empty list"),
        }
    }
}

impl std::error::Error for LinkedListError {}

// ============================================================================
// SimpleLinkedList Implementation
// ============================================================================

impl<T> SimpleLinkedList<T> {
    /// Creates a new empty linked list
    /// 
    /// # Requirements Satisfied: REQ-1
    /// 
    /// # Time Complexity: O(1)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mission4::SimpleLinkedList;
    /// 
    /// let list: SimpleLinkedList<i32> = SimpleLinkedList::new();
    /// assert!(list.is_empty());
    /// ```
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
            length: 0,
        }
    }

    /// Adds an element to the front of the list
    /// 
    /// # Requirements Satisfied: REQ-3
    /// 
    /// # Time Complexity: O(1)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mission4::SimpleLinkedList;
    /// 
    /// let mut list = SimpleLinkedList::new();
    /// list.push_front(42);
    /// assert_eq!(list.len(), 1);
    /// assert_eq!(list.peek_front(), Some(&42));
    /// ```
    pub fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node {
            data: value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.length += 1;
    }

    /// Removes and returns the front element
    /// 
    /// # Requirements Satisfied: REQ-3
    /// 
    /// # Time Complexity: O(1)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mission4::SimpleLinkedList;
    /// 
    /// let mut list = SimpleLinkedList::new();
    /// list.push_front(42);
    /// assert_eq!(list.pop_front(), Some(42));
    /// assert_eq!(list.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.length -= 1;
            node.data
        })
    }

    /// Returns a reference to the front element without removing it
    /// 
    /// # Requirements Satisfied: REQ-3
    /// 
    /// # Time Complexity: O(1)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mission4::SimpleLinkedList;
    /// 
    /// let mut list = SimpleLinkedList::new();
    /// assert_eq!(list.peek_front(), None);
    /// list.push_front(42);
    /// assert_eq!(list.peek_front(), Some(&42));
    /// ```
    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    /// Returns a mutable reference to the front element without removing it
    /// 
    /// # Requirements Satisfied: REQ-3
    /// 
    /// # Time Complexity: O(1)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mission4::SimpleLinkedList;
    /// 
    /// let mut list = SimpleLinkedList::new();
    /// list.push_front(42);
    /// if let Some(front) = list.peek_front_mut() {
    ///     *front = 100;
    /// }
    /// assert_eq!(list.peek_front(), Some(&100));
    /// ```
    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.data)
    }

    /// Returns true if the list is empty
    /// 
    /// # Requirements Satisfied: REQ-3
    /// 
    /// # Time Complexity: O(1)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mission4::SimpleLinkedList;
    /// 
    /// let mut list = SimpleLinkedList::new();
    /// assert!(list.is_empty());
    /// list.push_front(42);
    /// assert!(!list.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Returns the number of elements in the list
    /// 
    /// # Requirements Satisfied: REQ-3
    /// 
    /// # Time Complexity: O(1) - maintained as invariant
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mission4::SimpleLinkedList;
    /// 
    /// let mut list = SimpleLinkedList::new();
    /// assert_eq!(list.len(), 0);
    /// list.push_front(1);
    /// list.push_front(2);
    /// assert_eq!(list.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.length
    }

    /// Clears all elements from the list
    /// 
    /// # Requirements Satisfied: REQ-3
    /// 
    /// # Time Complexity: O(n) where n is the number of elements
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mission4::SimpleLinkedList;
    /// 
    /// let mut list = SimpleLinkedList::new();
    /// list.push_front(1);
    /// list.push_front(2);
    /// list.clear();
    /// assert!(list.is_empty());
    /// assert_eq!(list.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        while self.pop_front().is_some() {}
    }

    /// Returns an iterator over references to the elements
    /// 
    /// # Requirements Satisfied: REQ-4
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mission4::SimpleLinkedList;
    /// 
    /// let mut list = SimpleLinkedList::new();
    /// list.push_front(3);
    /// list.push_front(2);
    /// list.push_front(1);
    /// 
    /// let collected: Vec<&i32> = list.iter().collect();
    /// assert_eq!(collected, vec![&1, &2, &3]);
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            current: self.head.as_deref(),
        }
    }

    /// Returns an iterator over mutable references to the elements
    /// 
    /// # Requirements Satisfied: REQ-4
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mission4::SimpleLinkedList;
    /// 
    /// let mut list = SimpleLinkedList::new();
    /// list.push_front(1);
    /// list.push_front(2);
    /// 
    /// for item in list.iter_mut() {
    ///     *item *= 2;
    /// }
    /// 
    /// let collected: Vec<i32> = list.into_iter().collect();
    /// assert_eq!(collected, vec![4, 2]);
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            current: self.head.as_deref_mut(),
        }
    }
}

impl<T> Default for SimpleLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Clone for SimpleLinkedList<T> {
    fn clone(&self) -> Self {
        let mut new_list = SimpleLinkedList::new();
        // Collect items and reverse to maintain order
        let items: Vec<T> = self.iter().cloned().collect();
        for item in items.into_iter().rev() {
            new_list.push_front(item);
        }
        new_list
    }
}

impl<T: PartialEq> PartialEq for SimpleLinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

impl<T: Eq> Eq for SimpleLinkedList<T> {}

impl<T: fmt::Display> fmt::Display for SimpleLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        for item in self.iter() {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
            first = false;
        }
        write!(f, "]")
    }
}

// ============================================================================
// Iterator Implementations (REQ-4)
// ============================================================================

/// Iterator that consumes the SimpleLinkedList
pub struct IntoIter<T> {
    list: SimpleLinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len(), Some(self.list.len()))
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}

impl<T> IntoIterator for SimpleLinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

/// Iterator over references to elements in SimpleLinkedList
pub struct Iter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_deref();
            &node.data
        })
    }
}

/// Iterator over mutable references to elements in SimpleLinkedList
pub struct IterMut<'a, T> {
    current: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            self.current = node.next.as_deref_mut();
            &mut node.data
        })
    }
}

/// Allow iteration over references
impl<'a, T> IntoIterator for &'a SimpleLinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Allow iteration over mutable references
impl<'a, T> IntoIterator for &'a mut SimpleLinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/// Convert from iterator
impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        // Collect and reverse to maintain order when pushing front
        let items: Vec<T> = iter.into_iter().collect();
        for item in items.into_iter().rev() {
            list.push_front(item);
        }
        list
    }
}

/// Extend trait implementation
impl<T> Extend<T> for SimpleLinkedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        // Add each item to the front (LIFO order)
        for item in iter {
            self.push_front(item);
        }
    }
}

// ============================================================================
// RcLinkedList Implementation
// ============================================================================

impl<T> RcLinkedList<T> {
    /// Creates a new empty Rc-based linked list
    /// 
    /// # Requirements Satisfied: REQ-2
    /// 
    /// # Time Complexity: O(1)
    pub fn new() -> Self {
        RcLinkedList {
            head: None,
            length: 0,
        }
    }

    /// Adds an element to the front of the list
    /// 
    /// # Requirements Satisfied: REQ-2, REQ-3
    /// 
    /// # Time Complexity: O(1)
    pub fn push_front(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(RcNode {
            data: value,
            next: self.head.take(),
            prev_weak: None,
        }));
        
        // Set up weak back-reference if there was a previous head
        if let Some(ref next_node) = new_node.borrow().next {
            next_node.borrow_mut().prev_weak = Some(Rc::downgrade(&new_node));
        }
        
        self.head = Some(new_node);
        self.length += 1;
    }

    /// Removes and returns the front element
    /// 
    /// # Requirements Satisfied: REQ-2, REQ-3
    /// 
    /// # Time Complexity: O(1)
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(T)` if successful, or `Err(LinkedListError)` if the node
    /// still has multiple references.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mission4::RcLinkedList;
    /// 
    /// let mut list = RcLinkedList::new();
    /// list.push_front(42);
    /// assert_eq!(list.pop_front(), Ok(Some(42)));
    /// assert_eq!(list.pop_front(), Ok(None));
    /// ```
    pub fn pop_front(&mut self) -> Result<Option<T>, LinkedListError> {
        match self.head.take() {
            None => Ok(None),
            Some(node) => {
                match Rc::try_unwrap(node) {
                    Ok(cell) => {
                        let node = cell.into_inner();
                        self.head = node.next;
                        // Clear weak reference in new head if exists
                        if let Some(ref new_head) = self.head {
                            new_head.borrow_mut().prev_weak = None;
                        }
                        self.length -= 1;
                        Ok(Some(node.data))
                    },
                    Err(rc_node) => {
                        // Put the node back and return error
                        self.head = Some(rc_node);
                        Err(LinkedListError::MultipleReferences)
                    }
                }
            }
        }
    }

    /// Attempts to get a reference to the front element
    /// 
    /// # Requirements Satisfied: REQ-2, REQ-3
    /// 
    /// # Time Complexity: O(1)
    /// 
    /// Note: This operation can fail if the front node is currently mutably borrowed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mission4::RcLinkedList;
    /// 
    /// let mut list = RcLinkedList::new();
    /// list.push_front(42);
    /// 
    /// match list.try_peek_front() {
    ///     Ok(Some(value)) => println!("Front value: {}", *value),
    ///     Ok(None) => println!("List is empty"),
    ///     Err(_) => println!("Could not borrow front node"),
    /// };
    /// ```
    pub fn try_peek_front(&self) -> Result<Option<Ref<'_, T>>, LinkedListError> {
        match &self.head {
            None => Ok(None),
            Some(node) => {
                match node.try_borrow() {
                    Ok(borrowed) => Ok(Some(Ref::map(borrowed, |n| &n.data))),
                    Err(_) => Err(LinkedListError::BorrowError),
                }
            }
        }
    }

    /// Attempts to get a mutable reference to the front element
    /// 
    /// # Requirements Satisfied: REQ-2, REQ-3
    /// 
    /// # Time Complexity: O(1)
    /// 
    /// Note: This operation can fail if the front node is currently borrowed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mission4::RcLinkedList;
    /// 
    /// let mut list = RcLinkedList::new();
    /// list.push_front(42);
    /// 
    /// match list.try_peek_front_mut() {
    ///     Ok(Some(mut value)) => *value = 100,
    ///     Ok(None) => println!("List is empty"),
    ///     Err(_) => println!("Could not mutably borrow front node"),
    /// };
    /// ```
    pub fn try_peek_front_mut(&self) -> Result<Option<RefMut<'_, T>>, LinkedListError> {
        match &self.head {
            None => Ok(None),
            Some(node) => {
                match node.try_borrow_mut() {
                    Ok(borrowed) => Ok(Some(RefMut::map(borrowed, |n| &mut n.data))),
                    Err(_) => Err(LinkedListError::BorrowError),
                }
            }
        }
    }

    /// Returns true if the list is empty
    /// 
    /// # Requirements Satisfied: REQ-2, REQ-3
    /// 
    /// # Time Complexity: O(1)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Returns the number of elements in the list
    /// 
    /// # Requirements Satisfied: REQ-2, REQ-3
    /// 
    /// # Time Complexity: O(1)
    pub fn len(&self) -> usize {
        self.length
    }

    /// Clears all elements from the list
    /// 
    /// # Requirements Satisfied: REQ-2, REQ-3
    /// 
    /// # Time Complexity: O(n) where n is the number of elements
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mission4::RcLinkedList;
    /// 
    /// let mut list = RcLinkedList::new();
    /// list.push_front(1);
    /// list.push_front(2);
    /// list.clear();
    /// assert!(list.is_empty());
    /// assert_eq!(list.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        while let Ok(Some(_)) = self.pop_front() {}
    }

    /// Gets a cloned reference to the head node for advanced operations
    /// 
    /// # Requirements Satisfied: REQ-6
    /// 
    /// This allows for advanced patterns where you need direct access to the Rc.
    /// Be careful not to create reference cycles.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mission4::RcLinkedList;
    /// 
    /// let mut list = RcLinkedList::new();
    /// list.push_front(42);
    /// 
    /// if let Some(head_ref) = list.get_head_ref() {
    ///     // Can share this reference with other data structures
    ///     println!("Head ref count: {}", std::rc::Rc::strong_count(&head_ref));
    /// }
    /// ```
    pub fn get_head_ref(&self) -> Option<Rc<RefCell<RcNode<T>>>> {
        self.head.clone()
    }
}

impl<T> Default for RcLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}