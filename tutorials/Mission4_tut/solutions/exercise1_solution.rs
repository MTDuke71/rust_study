// Solution for Exercise 1: Basic Box Implementation
use std::fmt::Debug;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }
    
    pub fn push_front(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.length += 1;
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.length -= 1;
            node.data
        })
    }
    
    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }
    
    pub fn len(&self) -> usize {
        self.length
    }
    
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

// Bonus implementations
impl<T: std::fmt::Display> SimpleLinkedList<T> {
    pub fn display(&self) {
        let mut current = &self.head;
        print!("List: ");
        
        while let Some(node) = current {
            print!("{}", node.data);
            current = &node.next;
            if current.is_some() {
                print!(" -> ");
            }
        }
        println!();
    }
}

impl<T> SimpleLinkedList<T> {
    // This is significantly more complex than push_front!
    pub fn push_back(&mut self, data: T) {
        if self.head.is_none() {
            self.push_front(data);
            return;
        }
        
        let new_node = Box::new(Node {
            data,
            next: None,
        });
        
        let mut current = self.head.as_mut().unwrap();
        while current.next.is_some() {
            current = current.next.as_mut().unwrap();
        }
        
        current.next = Some(new_node);
        self.length += 1;
    }
}

fn main() {
    let mut list = SimpleLinkedList::new();
    
    println!("=== Testing Basic Operations ===");
    
    // Test empty list
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.peek_front(), None);
    
    // Test push_front
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    
    assert!(!list.is_empty());
    assert_eq!(list.len(), 3);
    assert_eq!(list.peek_front(), Some(&3));
    
    // Test pop_front
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.len(), 2);
    assert_eq!(list.peek_front(), Some(&2));
    
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), None);
    
    assert!(list.is_empty());
    
    println!("✅ All tests passed!");
    
    // Test bonus features
    println!("\n=== Testing Bonus Features ===");
    
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    list.display();
    
    list.push_back(0);
    list.display();
    
    println!("✅ Bonus features work!");
}