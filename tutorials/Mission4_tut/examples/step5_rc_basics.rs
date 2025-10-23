// Step 5: Introduction to Rc<RefCell<T>>
// Run with: cargo run --example step5_rc_basics

use std::cell::RefCell;
use std::rc::Rc;

type NodeRef<T> = Rc<RefCell<RcNode<T>>>;

#[derive(Debug)]
struct RcNode<T> {
    data: T,
    next: Option<NodeRef<T>>,
}

#[derive(Debug)]
pub struct RcLinkedList<T> {
    head: Option<NodeRef<T>>,
    length: usize,
}

impl<T> Default for RcLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> RcLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    pub fn push_front(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(RcNode {
            data,
            next: self.head.clone(), // Clone the Rc, not the data!
        }));
        self.head = Some(new_node);
        self.length += 1;
    }

    pub fn len(&self) -> usize {
        self.length
    }

    // Safe method to get a shared reference to a node
    pub fn get_node_ref(&self, index: usize) -> Option<NodeRef<T>> {
        if index >= self.length {
            return None;
        }

        let mut current = self.head.clone()?;
        for _ in 0..index {
            let next = current.borrow().next.clone()?;
            current = next;
        }
        Some(current)
    }
}

fn demonstrate_reference_counting() {
    println!("=== Demonstrating Reference Counting ===");

    let mut list = RcLinkedList::new();
    list.push_front(42);
    list.push_front(24);

    // Get references to nodes
    let first_node = list.get_node_ref(0).unwrap();
    let second_node = list.get_node_ref(1).unwrap();

    println!(
        "Reference count for first node: {}",
        Rc::strong_count(&first_node)
    );
    println!(
        "Reference count for second node: {}",
        Rc::strong_count(&second_node)
    );

    // Create additional references
    let another_ref_to_first = first_node.clone();
    let yet_another_ref = first_node.clone();

    println!("After creating more references:");
    println!(
        "Reference count for first node: {}",
        Rc::strong_count(&first_node)
    );

    // References automatically dropped at end of scope
}

fn demonstrate_shared_access() {
    println!("=== Demonstrating Shared Access ===");

    let mut list = RcLinkedList::new();
    list.push_front("Hello");
    list.push_front("World");

    // Get multiple handles to the same node
    let node_ref1 = list.get_node_ref(0).unwrap();
    let node_ref2 = list.get_node_ref(0).unwrap(); // Same node!

    // Both references point to the same data
    println!("Via reference 1: {:?}", node_ref1.borrow().data);
    println!("Via reference 2: {:?}", node_ref2.borrow().data);

    // Prove they're the same by comparing addresses
    let addr1 = node_ref1.as_ptr();
    let addr2 = node_ref2.as_ptr();
    println!("Same memory location? {}", addr1 == addr2);
}

fn main() {
    println!("=== Step 5: Rc<RefCell<T>> Basics ===");

    let mut list = RcLinkedList::new();

    // Basic operations work the same
    list.push_front(100);
    list.push_front(200);
    list.push_front(300);

    println!("Created list with length: {}", list.len());

    // But now we can get shared references!
    if let Some(first_node) = list.get_node_ref(0) {
        println!("First node data: {:?}", first_node.borrow().data);
    }

    if let Some(second_node) = list.get_node_ref(1) {
        println!("Second node data: {:?}", second_node.borrow().data);
    }

    demonstrate_reference_counting();
    demonstrate_shared_access();

    println!();
    println!("📝 Key Differences from Box<T>:");
    println!("- Rc::new() instead of Box::new()");
    println!("- clone() creates a new reference, not a new object");
    println!("- RefCell allows interior mutability");
    println!("- borrow() needed to access data inside RefCell");
    println!("- Can have multiple references to the same node!");

    println!("✅ Rc<RefCell<T>> basics work!");
}
