// Step 5: Introduction to Rc<RefCell<T>> - Enhanced Version (No Warnings)
// Run with: cargo run --example step5_enhanced_no_warnings

use std::cell::RefCell;
use std::rc::Rc;

type NodeRef<T> = Rc<RefCell<RcNode<T>>>;

#[derive(Debug)]
pub struct RcNode<T> {
    // Made public to fix visibility warning
    pub data: T,                  // Made public for external access
    pub next: Option<NodeRef<T>>, // Made public for external access
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

    pub fn is_empty(&self) -> bool {
        self.length == 0
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

    // Helper method to safely access data
    pub fn get_data(&self, index: usize) -> Option<T>
    where
        T: Clone,
    {
        self.get_node_ref(index)
            .map(|node| node.borrow().data.clone())
    }

    // Method to modify data at a specific index
    pub fn set_data(&mut self, index: usize, new_data: T) -> Result<(), &'static str> {
        if let Some(node) = self.get_node_ref(index) {
            node.borrow_mut().data = new_data;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
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
        "Initial reference count for first node: {}",
        Rc::strong_count(&first_node)
    );
    println!(
        "Initial reference count for second node: {}",
        Rc::strong_count(&second_node)
    );

    // Create additional references and USE them to demonstrate the concept
    let _additional_ref1 = first_node.clone();
    let _additional_ref2 = first_node.clone();

    println!("After creating more references:");
    println!(
        "Reference count for first node: {}",
        Rc::strong_count(&first_node)
    );

    // Actually use the references to show they work
    println!("Data via first reference: {}", first_node.borrow().data);
    println!(
        "Data via additional ref1: {}",
        _additional_ref1.borrow().data
    );
    println!(
        "Data via additional ref2: {}",
        _additional_ref2.borrow().data
    );

    // References automatically dropped at end of scope
    println!(
        "All references point to the same data: {}",
        first_node.borrow().data == _additional_ref1.borrow().data
    );
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

    // Demonstrate that changes through one reference affect the other
    {
        let mut borrowed = node_ref1.borrow_mut();
        borrowed.data = "Modified!";
    }
    println!(
        "After modifying via ref1, ref2 sees: {:?}",
        node_ref2.borrow().data
    );
}

fn demonstrate_mutation() {
    println!("=== Demonstrating Safe Mutation ===");

    let mut list = RcLinkedList::new();
    list.push_front(100);
    list.push_front(200);
    list.push_front(300);

    println!("Original list data:");
    for i in 0..list.len() {
        if let Some(data) = list.get_data(i) {
            println!("  Position {}: {}", i, data);
        }
    }

    // Modify data using our helper method
    if let Err(e) = list.set_data(1, 999) {
        println!("Error: {}", e);
    }

    println!("After modifying position 1:");
    for i in 0..list.len() {
        if let Some(data) = list.get_data(i) {
            println!("  Position {}: {}", i, data);
        }
    }
}

fn main() {
    println!("=== Step 5: Rc<RefCell<T>> Basics (Enhanced) ===");

    let mut list = RcLinkedList::new();

    // Basic operations work the same
    list.push_front(100);
    list.push_front(200);
    list.push_front(300);

    println!("Created list with length: {}", list.len());
    println!("Is empty? {}", list.is_empty());

    // But now we can get shared references!
    if let Some(first_node) = list.get_node_ref(0) {
        println!("First node data: {:?}", first_node.borrow().data);
    }

    if let Some(second_node) = list.get_node_ref(1) {
        println!("Second node data: {:?}", second_node.borrow().data);
    }

    demonstrate_reference_counting();
    demonstrate_shared_access();
    demonstrate_mutation();

    println!();
    println!("📝 Key Differences from Box<T>:");
    println!("- Rc::new() instead of Box::new()");
    println!("- clone() creates a new reference, not a new object");
    println!("- RefCell allows interior mutability");
    println!("- borrow() needed to access data inside RefCell");
    println!("- Can have multiple references to the same node!");
    println!("- Reference counting tracks how many owners exist");

    println!();
    println!("🔧 Warning Fixes Applied:");
    println!("- Made RcNode public to fix visibility warning");
    println!("- Used all variables to demonstrate concepts");
    println!("- Added practical methods that use the references");

    println!("✅ Rc<RefCell<T>> basics work without warnings!");
}
