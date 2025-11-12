//! Chapter 15.6: Reference Cycles Can Leak Memory
//!
//! This example demonstrates reference cycles, memory leaks with Rc<T>,
//! and preventing cycles using Weak<T>.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {
    println!("=== Chapter 15.6: Reference Cycles and Weak<T> ===\n");
    
    demonstrate_reference_cycle();
    tree_with_weak_references();
    weak_upgrade_example();
    parent_child_relationships();
    
    println!("\n✅ Chapter 15.6 Reference cycles and Weak<T> demonstrated!");
}

/// WARNING: Demonstrates a reference cycle that leaks memory
/// 
/// **Book Reference**: Section 15.6 - Creating a Reference Cycle
/// **Key Learning**: Rc cycles prevent reference count from reaching zero
fn demonstrate_reference_cycle() {
    println!("--- Reference Cycle (Memory Leak) ---");
    
    #[allow(dead_code)]
    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }
    
    use List::{Cons, Nil};
    
    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }
    
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("\na created, b initial rc count = {}", Rc::strong_count(&b));
    println!("a rc count = {}", Rc::strong_count(&a));
    
    // Create cycle: a -> b -> a
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    
    println!("\nb rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    
    // Uncommenting next line would overflow the stack
    // println!("a next item = {:?}", a.tail());
    
    println!("\n⚠️  MEMORY LEAK: a and b reference each other");
    println!("   Reference counts never reach zero!\n");
}

/// Shows proper tree structure using Weak<T> to prevent cycles
///
/// **Book Reference**: Section 15.6 - Preventing Reference Cycles
/// **Integration**: Critical for [[mission-4]] graph structures
fn tree_with_weak_references() {
    println!("--- Tree with Weak References (No Leak) ---");
    
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,      // Weak to prevent cycle
        children: RefCell<Vec<Rc<Node>>>, // Strong references down
    }
    
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    
    println!("leaf:");
    println!("  strong = {}, weak = {}", 
             Rc::strong_count(&leaf), 
             Rc::weak_count(&leaf));
    
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        
        println!("\nbranch:");
        println!("  strong = {}, weak = {}", 
                 Rc::strong_count(&branch), 
                 Rc::weak_count(&branch));
        
        println!("\nleaf:");
        println!("  strong = {}, weak = {}", 
                 Rc::strong_count(&leaf), 
                 Rc::weak_count(&leaf));
    }  // branch dropped here
    
    println!("\nAfter branch dropped:");
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf:");
    println!("  strong = {}, weak = {}", 
             Rc::strong_count(&leaf), 
             Rc::weak_count(&leaf));
    
    println!("\n✓ Weak<T> prevents parent-child reference cycles\n");
}

/// Demonstrates Weak::upgrade behavior
fn weak_upgrade_example() {
    println!("--- Weak::upgrade Example ---");
    
    let strong = Rc::new(42);
    let weak = Rc::downgrade(&strong);
    
    println!("Strong count: {}", Rc::strong_count(&strong));
    println!("Weak count: {}", Rc::weak_count(&strong));
    
    // Upgrade successful when strong reference exists
    match weak.upgrade() {
        Some(rc) => println!("Upgraded successfully: {}", rc),
        None => println!("Upgrade failed - value was dropped"),
    }
    
    drop(strong);
    
    // Upgrade fails after dropping all strong references
    match weak.upgrade() {
        Some(rc) => println!("Upgraded successfully: {}", rc),
        None => println!("Upgrade failed - value was dropped"),
    }
    
    println!("\n✓ Weak::upgrade returns None after strong references dropped\n");
}

/// Shows parent-child relationship pattern
fn parent_child_relationships() {
    println!("--- Parent-Child Relationship Pattern ---");
    
    #[derive(Debug)]
    struct Parent {
        name: String,
        children: RefCell<Vec<Rc<Child>>>,
    }
    
    #[derive(Debug)]
    struct Child {
        name: String,
        parent: RefCell<Weak<Parent>>,  // Weak to parent
    }
    
    let parent = Rc::new(Parent {
        name: String::from("Parent"),
        children: RefCell::new(vec![]),
    });
    
    let child1 = Rc::new(Child {
        name: String::from("Child1"),
        parent: RefCell::new(Rc::downgrade(&parent)),
    });
    
    let child2 = Rc::new(Child {
        name: String::from("Child2"),
        parent: RefCell::new(Rc::downgrade(&parent)),
    });
    
    parent.children.borrow_mut().push(Rc::clone(&child1));
    parent.children.borrow_mut().push(Rc::clone(&child2));
    
    println!("Parent: {}", parent.name);
    println!("  Children count: {}", parent.children.borrow().len());
    println!("  Strong count: {}", Rc::strong_count(&parent));
    println!("  Weak count: {}", Rc::weak_count(&parent));
    
    println!("\nChild1: {}", child1.name);
    if let Some(p) = child1.parent.borrow().upgrade() {
        println!("  Parent: {}", p.name);
    }
    println!("  Strong count: {}", Rc::strong_count(&child1));
    
    println!("\n✓ Pattern: Strong refs to children, Weak refs to parent");
    println!("  Prevents cycles while maintaining relationships\n");
}

/// Memory leak detection pattern
#[allow(dead_code)]
fn memory_leak_detection() {
    println!("--- Memory Leak Detection ---");
    
    println!("Signs of reference cycles:");
    println!("1. Reference counts never reach zero");
    println!("2. Memory usage grows over time");
    println!("3. Infinite recursion when traversing structure");
    
    println!("\nPrevention strategies:");
    println!("1. Use Weak<T> for back-references (child -> parent)");
    println!("2. Analyze ownership graph for cycles");
    println!("3. Test with memory profilers (Valgrind, heaptrack)");
    println!("4. Consider using arena allocation for graphs");
    
    println!("\n✓ Always analyze parent-child and peer relationships\n");
}
