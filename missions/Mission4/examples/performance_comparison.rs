//! Performance Comparison for Mission 4: Singly Linked List
//!
//! This example compares the performance characteristics of SimpleLinkedList
//! vs RcLinkedList and demonstrates their relative strengths.

use mission4::{RcLinkedList, SimpleLinkedList};
use std::time::Instant;

fn main() {
    println!("⚡ Performance Comparison: SimpleLinkedList vs RcLinkedList");
    println!("===========================================================\n");

    benchmark_push_operations();
    println!();
    benchmark_pop_operations();
    println!();
    benchmark_iteration();
    println!();
    memory_usage_analysis();
}

fn benchmark_push_operations() {
    println!("📊 Push Operations Benchmark");
    println!("----------------------------");

    const SIZES: &[usize] = &[100, 500, 1000, 2000];

    for &size in SIZES {
        println!("Testing with {} elements:", size);

        // Benchmark SimpleLinkedList
        let start = Instant::now();
        let mut simple_list = SimpleLinkedList::new();
        for i in 0..size {
            simple_list.push_front(i);
        }
        let simple_time = start.elapsed();

        // Benchmark RcLinkedList
        let start = Instant::now();
        let mut rc_list = RcLinkedList::new();
        for i in 0..size {
            rc_list.push_front(i);
        }
        let rc_time = start.elapsed();

        println!("  SimpleLinkedList: {:?}", simple_time);
        println!("  RcLinkedList:     {:?}", rc_time);
        println!(
            "  Overhead ratio:   {:.2}x",
            rc_time.as_nanos() as f64 / simple_time.as_nanos() as f64
        );
        println!();
    }
}

fn benchmark_pop_operations() {
    println!("📉 Pop Operations Benchmark");
    println!("---------------------------");

    const SIZE: usize = 1000;

    // Setup SimpleLinkedList
    let mut simple_list = SimpleLinkedList::new();
    for i in 0..SIZE {
        simple_list.push_front(i);
    }

    // Setup RcLinkedList
    let mut rc_list = RcLinkedList::new();
    for i in 0..SIZE {
        rc_list.push_front(i);
    }

    // Benchmark SimpleLinkedList pop
    let start = Instant::now();
    while simple_list.pop_front().is_some() {}
    let simple_time = start.elapsed();

    // Benchmark RcLinkedList pop
    let start = Instant::now();
    while let Ok(Some(_)) = rc_list.pop_front() {}
    let rc_time = start.elapsed();

    println!("Popping {} elements:", SIZE);
    println!("  SimpleLinkedList: {:?}", simple_time);
    println!("  RcLinkedList:     {:?}", rc_time);
    println!(
        "  Overhead ratio:   {:.2}x",
        rc_time.as_nanos() as f64 / simple_time.as_nanos() as f64
    );
}

fn benchmark_iteration() {
    println!("🔄 Iteration Benchmark");
    println!("----------------------");

    const SIZE: usize = 1000;

    // Setup SimpleLinkedList
    let mut simple_list = SimpleLinkedList::new();
    for i in 0..SIZE {
        simple_list.push_front(i);
    }

    // Benchmark iteration
    let start = Instant::now();
    let mut sum = 0;
    for value in &simple_list {
        sum += value;
    }
    let iter_time = start.elapsed();

    // Benchmark into_iter (consuming)
    let start = Instant::now();
    let _collected: Vec<usize> = simple_list.into_iter().collect();
    let consume_time = start.elapsed();

    println!("Iterating over {} elements:", SIZE);
    println!("  Borrowing iteration: {:?} (sum: {})", iter_time, sum);
    println!("  Consuming iteration: {:?}", consume_time);

    // Note: RcLinkedList doesn't have iteration implemented yet
    println!("  RcLinkedList: No iterator implementation (would be more complex)");
}

fn memory_usage_analysis() {
    println!("💾 Memory Usage Analysis");
    println!("-----------------------");

    // This is a conceptual analysis since Rust doesn't provide direct memory introspection
    println!("Memory characteristics per node:");
    println!();

    println!("SimpleLinkedList<T>:");
    println!("  - Node<T>: size_of::<T>() + size_of::<Box<Node<T>>>()");
    println!("  - Box overhead: 8 bytes (pointer)");
    println!(
        "  - Total per node: ~{} bytes + sizeof(T)",
        std::mem::size_of::<Box<()>>()
    );
    println!("  - Memory layout: Excellent cache locality");
    println!();

    println!("RcLinkedList<T>:");
    println!("  - RcNode<T>: size_of::<T>() + size_of::<Rc<RefCell<RcNode<T>>>>()");
    println!(
        "  - Rc overhead: ~{} bytes (ref count + weak count + data)",
        std::mem::size_of::<std::rc::Rc<std::cell::RefCell<i32>>>()
    );
    println!(
        "  - RefCell overhead: ~{} bytes (borrow flag)",
        std::mem::size_of::<std::cell::RefCell<i32>>() - std::mem::size_of::<i32>()
    );
    println!(
        "  - Weak reference: ~{} bytes",
        std::mem::size_of::<std::rc::Weak<std::cell::RefCell<i32>>>()
    );
    println!(
        "  - Total per node: ~{} bytes + sizeof(T)",
        std::mem::size_of::<std::rc::Rc<std::cell::RefCell<i32>>>()
            + std::mem::size_of::<std::rc::Weak<std::cell::RefCell<i32>>>()
    );
    println!("  - Memory layout: Potential cache misses due to indirection");
    println!();

    // Practical demonstration
    const SIZE: usize = 1000;

    let mut simple_list = SimpleLinkedList::new();
    for i in 0..SIZE {
        simple_list.push_front(i);
    }

    let mut rc_list = RcLinkedList::new();
    for i in 0..SIZE {
        rc_list.push_front(i);
    }

    println!("Practical comparison with {} i32 elements:", SIZE);
    println!(
        "  SimpleLinkedList: ~{} bytes estimated",
        SIZE * (std::mem::size_of::<i32>() + std::mem::size_of::<Box<()>>())
    );
    println!(
        "  RcLinkedList: ~{} bytes estimated",
        SIZE * (std::mem::size_of::<i32>()
            + std::mem::size_of::<std::rc::Rc<std::cell::RefCell<i32>>>()
            + std::mem::size_of::<std::rc::Weak<std::cell::RefCell<i32>>>())
    );

    let overhead_ratio = (std::mem::size_of::<std::rc::Rc<std::cell::RefCell<i32>>>()
        + std::mem::size_of::<std::rc::Weak<std::cell::RefCell<i32>>>())
        as f64
        / std::mem::size_of::<Box<()>>() as f64;
    println!("  Memory overhead ratio: {:.2}x", overhead_ratio);

    println!();
    println!("Use case recommendations:");
    println!("  SimpleLinkedList: Fast, memory-efficient, simple ownership");
    println!("  RcLinkedList:     Complex sharing, multiple ownership scenarios");
}
