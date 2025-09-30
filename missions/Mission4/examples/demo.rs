//! Demo example for Mission 4: Singly Linked List
//! 
//! This example demonstrates both SimpleLinkedList and RcLinkedList implementations,
//! showing their different use cases and ownership patterns.

use mission4::{SimpleLinkedList, RcLinkedList};

fn main() {
    println!("🦀 Mission 4: Singly Linked List Demo");
    println!("=====================================\n");

    demo_simple_linked_list();
    println!();
    demo_rc_linked_list();
    println!();
    demo_comparison();
}

fn demo_simple_linked_list() {
    println!("📦 SimpleLinkedList (Box-based ownership)");
    println!("------------------------------------------");
    
    let mut list = SimpleLinkedList::new();
    
    // Push some elements
    println!("Pushing elements: 3, 2, 1");
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);
    
    println!("List length: {}", list.len());
    println!("List display: {}", list);
    
    // Peek at front
    if let Some(front) = list.peek_front() {
        println!("Front element: {}", front);
    }
    
    // Modify front element
    if let Some(front) = list.peek_front_mut() {
        *front = 42;
        println!("Modified front element to: {}", front);
    }
    
    // Iterate and collect
    println!("All elements:");
    for (i, value) in list.iter().enumerate() {
        println!("  [{}]: {}", i, value);
    }
    
    // Pop elements
    println!("Popping elements:");
    while let Some(value) = list.pop_front() {
        println!("  Popped: {}", value);
    }
    
    println!("List is now empty: {}", list.is_empty());
}

fn demo_rc_linked_list() {
    println!("🔗 RcLinkedList (Rc<RefCell<>> shared ownership)");
    println!("------------------------------------------------");
    
    let mut list = RcLinkedList::new();
    
    // Push some elements
    println!("Pushing elements: 'C', 'B', 'A'");
    list.push_front('C');
    list.push_front('B');
    list.push_front('A');
    
    println!("List length: {}", list.len());
    
    // Demonstrate shared ownership
    if let Some(head_ref) = list.get_head_ref() {
        println!("Head node reference count: {}", std::rc::Rc::strong_count(&head_ref));
        
        // We can share this reference with other data structures
        let shared_ref = head_ref.clone();
        println!("After cloning reference count: {}", std::rc::Rc::strong_count(&head_ref));
        
        drop(shared_ref);
        println!("After dropping clone: {}", std::rc::Rc::strong_count(&head_ref));
    }
    
    // Try to peek (demonstrating runtime borrow checking)
    match list.try_peek_front() {
        Ok(Some(value_ref)) => {
            println!("Successfully borrowed front element: {}", *value_ref);
            // The borrow is automatically dropped here
        },
        Ok(None) => println!("List is empty"),
        Err(e) => println!("Could not borrow: {}", e),
    }
    
    // Pop with error handling
    println!("Popping elements:");
    loop {
        match list.pop_front() {
            Ok(Some(value)) => println!("  Popped: {}", value),
            Ok(None) => {
                println!("  List is empty");
                break;
            },
            Err(e) => {
                println!("  Error popping: {}", e);
                break;
            }
        }
    }
}

fn demo_comparison() {
    println!("⚖️  Comparison: Simple vs Rc Implementation");
    println!("===========================================");
    
    const N: usize = 10000;
    
    // Benchmark Simple List
    let start = std::time::Instant::now();
    let mut simple_list = SimpleLinkedList::new();
    for i in 0..N {
        simple_list.push_front(i);
    }
    while simple_list.pop_front().is_some() {}
    let simple_time = start.elapsed();
    
    // Benchmark Rc List
    let start = std::time::Instant::now();
    let mut rc_list = RcLinkedList::new();
    for i in 0..N {
        rc_list.push_front(i);
    }
    while let Ok(Some(_)) = rc_list.pop_front() {}
    let rc_time = start.elapsed();
    
    println!("Performance for {} operations:", N * 2);
    println!("  SimpleLinkedList: {:?}", simple_time);
    println!("  RcLinkedList:     {:?}", rc_time);
    println!("  Ratio:            {:.2}x", rc_time.as_nanos() as f64 / simple_time.as_nanos() as f64);
    
    println!("\nMemory characteristics:");
    println!("  SimpleLinkedList: Minimal overhead, exclusive ownership");
    println!("  RcLinkedList:     Reference counting overhead, shared ownership");
    
    println!("\nUse cases:");
    println!("  SimpleLinkedList: Fast, simple scenarios, competitive programming");
    println!("  RcLinkedList:     Complex sharing patterns, graph-like structures");
}