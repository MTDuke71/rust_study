//! Chapter 15.4: Rc<T>, the Reference Counted Smart Pointer
//!
//! This example demonstrates Rc<T> for multiple ownership,
//! reference counting, and shared immutable data.

use std::rc::Rc;

fn main() {
    println!("=== Chapter 15.4: Rc<T> - Reference Counted Pointers ===\n");
    
    basic_rc_example();
    shared_list_structure();
    reference_counting_demo();
    rc_clone_semantics();
    rc_immutability();
    
    println!("\n✅ Chapter 15.4 Rc<T> concepts demonstrated!");
}

/// Demonstrates basic Rc<T> usage and reference counting
/// 
/// **Book Reference**: Section 15.4 - Using Rc<T> to Share Data
/// **Key Learning**: Rc keeps track of references, deallocates at zero
fn basic_rc_example() {
    println!("--- Basic Rc<T> Usage ---");
    
    let a = Rc::new(5);
    println!("Created a, count = {}", Rc::strong_count(&a));
    
    {
        let _b = Rc::clone(&a);
        println!("Created b, count = {}", Rc::strong_count(&a));
        
        {
            let _c = Rc::clone(&a);
            println!("Created c, count = {}", Rc::strong_count(&a));
        }  // c dropped here
        
        println!("After c dropped, count = {}", Rc::strong_count(&a));
    }  // b dropped here
    
    println!("After b dropped, count = {}", Rc::strong_count(&a));
    println!("✓ Reference count decrements as clones are dropped\n");
}

/// Shows shared ownership in data structures
///
/// **Book Reference**: Section 15.4 - Sharing a List
/// **Integration**: Essential pattern in [[mission-4]] for linked lists
fn shared_list_structure() {
    println!("--- Shared List with Rc<T> ---");
    
    #[allow(dead_code)]
    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }
    
    use List::{Cons, Nil};
    
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Created list a, count = {}", Rc::strong_count(&a));
    println!("a = {:?}", a);
    
    let b = Cons(3, Rc::clone(&a));
    println!("\nCreated list b sharing a's tail, count = {}", Rc::strong_count(&a));
    println!("b = {:?}", b);
    
    let c = Cons(4, Rc::clone(&a));
    println!("\nCreated list c sharing a's tail, count = {}", Rc::strong_count(&a));
    println!("c = {:?}", c);
    
    println!("\n✓ Multiple lists share the same tail through Rc<T>\n");
}

/// Demonstrates reference count changes throughout lifetime
fn reference_counting_demo() {
    println!("--- Reference Counting Lifecycle ---");
    
    let data = Rc::new(vec![1, 2, 3, 4, 5]);
    println!("Initial count: {}", Rc::strong_count(&data));
    
    let owners = {
        let o1 = Rc::clone(&data);
        println!("After o1: count = {}", Rc::strong_count(&data));
        
        let o2 = Rc::clone(&data);
        println!("After o2: count = {}", Rc::strong_count(&data));
        
        let o3 = Rc::clone(&data);
        println!("After o3: count = {}", Rc::strong_count(&data));
        
        vec![o1, o2, o3]
    };
    
    println!("After collecting owners: count = {}", Rc::strong_count(&data));
    
    drop(owners);
    println!("After dropping all owners: count = {}", Rc::strong_count(&data));
    
    println!("✓ Reference count tracks all live clones\n");
}

/// Shows Rc::clone vs value clone semantics
fn rc_clone_semantics() {
    println!("--- Rc::clone Semantics ---");
    
    let expensive_data = Rc::new(vec![0; 1000]);
    println!("Created expensive data (1000 elements)");
    println!("Initial ref count: {}", Rc::strong_count(&expensive_data));
    
    // Rc::clone only increments reference count, doesn't clone data
    let _shared1 = Rc::clone(&expensive_data);
    println!("Rc::clone called, ref count: {}", Rc::strong_count(&expensive_data));
    
    let _shared2 = Rc::clone(&expensive_data);
    println!("Rc::clone called, ref count: {}", Rc::strong_count(&expensive_data));
    
    println!("\n✓ Rc::clone is cheap - only increments counter");
    println!("  Note: Could use expensive_data.clone() but Rc::clone is more explicit\n");
}

/// Demonstrates Rc<T> only allows immutable access
#[allow(dead_code)]
fn rc_immutability() {
    println!("--- Rc<T> Immutability ---");
    
    let data = Rc::new(vec![1, 2, 3]);
    let shared = Rc::clone(&data);
    
    // Can read from multiple owners
    println!("data: {:?}", data);
    println!("shared: {:?}", shared);
    
    // This would NOT compile - Rc<T> only allows shared immutable access:
    // data.push(4);  // Error: cannot borrow as mutable
    
    println!("✓ Rc<T> provides multiple readers, no writers");
    println!("  Use Rc<RefCell<T>> for interior mutability\n");
}
