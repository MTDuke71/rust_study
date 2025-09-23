// Linked List Practice: Demonstrating Why RefCell is Necessary
// This file shows compile errors that occur when trying to implement
// linked lists with regular references, and how RefCell solves them.

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    println!("=== Linked List Practice: Compile Error Demonstrations ===");
    
    demonstrate_why_regular_references_fail();
    demonstrate_refcell_solution();
    
    println!("✅ See the commented examples below to understand why RefCell is essential!");
}

// ============================================================================
// WORKING EXAMPLES WITH RefCell
// ============================================================================

type NodeRef<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<NodeRef<T>>,
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<NodeRef<T>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }
    
    fn push_front(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data,
            next: self.head.take(),
        }));
        self.head = Some(new_node);
    }
    
    fn get_head_ref(&self) -> Option<NodeRef<T>> {
        self.head.clone()
    }
}

fn demonstrate_refcell_solution() {
    println!("\n=== ✅ RefCell Solution: This Works! ===");
    
    let mut list = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    
    // Get multiple references to the same node
    if let Some(head1) = list.get_head_ref() {
        let head2 = head1.clone();
        
        // Read through both references
        println!("Head1 data: {}", head1.borrow().data);
        println!("Head2 data: {}", head2.borrow().data);
        
        // Mutate through one reference
        head1.borrow_mut().data = 42;
        
        // Read the updated value through the other reference
        println!("Updated data via head2: {}", head2.borrow().data);
        
        println!("✅ RefCell allows shared ownership with controlled mutation!");
    }
}

fn demonstrate_why_regular_references_fail() {
    println!("=== ❌ Why Regular References Don't Work ===");
    println!("See the commented code below for examples that would cause compile errors.");
    println!("Uncomment any section and run 'cargo check' to see the specific errors.");
}

// ============================================================================
// COMPILE ERROR EXAMPLES - UNCOMMENT TO SEE FAILURES!
// ============================================================================

/*
// ❌ ATTEMPT 1: Using Box<T> for single ownership
// Problem: Can't have multiple references to the same node

struct BoxNode<T> {
    data: T,
    next: Option<Box<BoxNode<T>>>,
}

struct BoxLinkedList<T> {
    head: Option<Box<BoxNode<T>>>,
}

fn compile_error_box_single_ownership() {
    let mut list = BoxLinkedList { head: None };
    
    // Add a node
    list.head = Some(Box::new(BoxNode {
        data: 42,
        next: None,
    }));
    
    // Try to get two references to the same node
    let ref1 = &list.head;
    let ref2 = &list.head;
    
    // This works for reading...
    if let (Some(node1), Some(node2)) = (ref1, ref2) {
        println!("{}, {}", node1.data, node2.data);
    }
    
    // But we can't mutate through one while the other exists
    if let Some(ref mut node) = list.head {
        node.data = 100;  // ❌ ERROR: cannot borrow as mutable while borrowed as immutable
    }
}

// ❌ ATTEMPT 2: Using regular references
// Problem: Lifetime and borrowing conflicts

struct RefNode<'a, T> {
    data: T,
    next: Option<&'a RefNode<'a, T>>,  // ❌ This creates impossible lifetime constraints
}

// This won't even compile because of self-referential structs
fn compile_error_self_referential() {
    // let node1 = RefNode { data: 1, next: None };
    // let node2 = RefNode { data: 2, next: Some(&node1) }; // ❌ ERROR: lifetime issues
}

// ❌ ATTEMPT 3: Trying to implement insertion with regular references
fn compile_error_insertion_with_references() {
    let mut data1 = 1;
    let mut data2 = 2;
    
    // Can't create a linked structure with regular references
    // let node1 = Node { data: &mut data1, next: None };
    // let node2 = Node { data: &mut data2, next: Some(&node1) }; // ❌ ERROR: multiple borrows
}

// ❌ ATTEMPT 4: Trying to modify a node while traversing
fn compile_error_modify_during_traversal() {
    let mut list = BoxLinkedList { head: None };
    
    // Add some nodes
    list.head = Some(Box::new(BoxNode {
        data: 1,
        next: Some(Box::new(BoxNode {
            data: 2,
            next: None,
        })),
    }));
    
    // Try to modify while holding a reference for traversal
    if let Some(ref head) = list.head {
        // We're borrowing list.head immutably here
        
        // This would fail:
        // list.head = None; // ❌ ERROR: cannot assign while borrowed
        
        println!("Head data: {}", head.data);
    }
}

// ❌ ATTEMPT 5: The fundamental aliasing + mutation problem
fn compile_error_aliasing_mutation() {
    let mut vec = vec![1, 2, 3];
    
    // Get two mutable references to the same data
    let ptr1 = &mut vec[0];
    let ptr2 = &mut vec[0]; // ❌ ERROR: cannot borrow as mutable more than once
    
    *ptr1 = 42;
    *ptr2 = 24; // Which value should vec[0] have? Undefined!
}

// ❌ ATTEMPT 6: Iterator invalidation during modification
fn compile_error_iterator_invalidation() {
    let mut vec = vec![1, 2, 3, 4, 5];
    
    // Try to modify the collection while iterating
    for item in &vec {  // Immutable borrow for iteration
        if *item > 3 {
            vec.push(*item * 2); // ❌ ERROR: cannot borrow vec as mutable
        }
    }
}
*/

// ============================================================================
// WHY THESE PROBLEMS MATTER FOR LINKED LISTS
// ============================================================================

/*
🎯 MISSION 4 V-CYCLE CONTEXT: Why RefCell is Essential

REQ-1: Basic linked list structure with owned pointers
  - Box<T> provides ownership but prevents sharing
  - We need multiple references to traverse and modify

REQ-2: Interior mutability with Rc<RefCell<T>>
  - RefCell moves borrow checking from compile-time to runtime
  - Allows controlled mutation through shared references

REQ-3: Core list operations (push, pop, peek)
  - Operations need to modify structure while maintaining references
  - RefCell enables safe modification patterns

REQ-4: Safe iteration patterns
  - Iterators need stable references during traversal
  - RefCell prevents iterator invalidation at runtime

The fundamental issue: Linked lists require ALIASING + MUTATION
- Multiple pointers to the same node (aliasing)
- Ability to modify through those pointers (mutation)
- Rust's compile-time borrow checker prevents this by default
- RefCell provides runtime-checked interior mutability as the solution

🔥 TRY IT YOURSELF:
1. Uncomment any of the functions above
2. Add a call to it in main()
3. Run `cargo check` to see the specific compile error
4. Compare with the working RefCell implementation

This demonstrates why RefCell isn't just convenient—it's NECESSARY
for implementing linked data structures in safe Rust!
*/