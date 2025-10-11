// Step 7: Performance Comparison
// Run with: cargo run --example step7_performance --release

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

// Box-based implementation
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

    pub fn len(&self) -> usize {
        self.length
    }
}

// Rc-based implementation
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
            next: self.head.clone(),
        }));
        self.head = Some(new_node);
        self.length += 1;
    }

    pub fn try_pop_front(&mut self) -> Option<T> {
        if let Some(head_node) = self.head.take() {
            // Check if we have exclusive access
            match Rc::try_unwrap(head_node) {
                Ok(cell) => {
                    let node = cell.into_inner();
                    self.head = node.next;
                    self.length -= 1;
                    Some(node.data)
                }
                Err(rc_node) => {
                    // Put it back if we can't get exclusive access
                    self.head = Some(rc_node);
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

fn benchmark_box_list(size: usize) -> std::time::Duration {
    let start = Instant::now();
    let mut list = SimpleLinkedList::new();

    // Push elements
    for i in 0..size {
        list.push_front(i);
    }

    // Pop all elements
    while list.pop_front().is_some() {
        // Continue until empty
    }

    start.elapsed()
}

fn benchmark_rc_list(size: usize) -> std::time::Duration {
    let start = Instant::now();
    let mut list = RcLinkedList::new();

    // Push elements
    for i in 0..size {
        list.push_front(i);
    }

    // Pop all elements
    while list.try_pop_front().is_some() {
        // Continue until empty
    }

    start.elapsed()
}

fn memory_usage_comparison() {
    println!("=== Memory Usage Comparison ===");

    // Box<T> memory layout
    println!("Box<T> memory per node:");
    println!("  - Box pointer: {} bytes", std::mem::size_of::<Box<i32>>());
    println!(
        "  - Node struct: {} bytes",
        std::mem::size_of::<Node<i32>>()
    );
    println!(
        "  - Total per node: ~{} bytes + data size",
        std::mem::size_of::<Box<i32>>() + std::mem::size_of::<Node<i32>>()
    );

    println!();

    // Rc<RefCell<T>> memory layout
    println!("Rc<RefCell<T>> memory per node:");
    println!(
        "  - Rc pointer: {} bytes",
        std::mem::size_of::<Rc<RefCell<RcNode<i32>>>>()
    );
    println!(
        "  - RefCell: {} bytes",
        std::mem::size_of::<RefCell<RcNode<i32>>>()
    );
    println!(
        "  - RcNode struct: {} bytes",
        std::mem::size_of::<RcNode<i32>>()
    );
    println!("  - Rc control block: ~16 bytes (estimated)");
    println!(
        "  - Total per node: ~{} bytes + data size",
        std::mem::size_of::<Rc<RefCell<RcNode<i32>>>>()
            + std::mem::size_of::<RefCell<RcNode<i32>>>()
            + 16
    );
}

fn cache_locality_demonstration() {
    println!("=== Cache Locality Demonstration ===");

    const SIZE: usize = 10000;

    // Box-based list (better cache locality)
    let start = Instant::now();
    let mut box_list = SimpleLinkedList::new();
    for i in 0..SIZE {
        box_list.push_front(i);
    }

    // Traverse the list (this tests cache performance)
    let mut sum = 0;
    while let Some(value) = box_list.pop_front() {
        sum += value;
    }
    let box_time = start.elapsed();

    // Rc-based list (more heap allocations, potential cache misses)
    let start = Instant::now();
    let mut rc_list = RcLinkedList::new();
    for i in 0..SIZE {
        rc_list.push_front(i);
    }

    // Traverse the list
    let mut sum2 = 0;
    while let Some(value) = rc_list.try_pop_front() {
        sum2 += value;
    }
    let rc_time = start.elapsed();

    println!("Box list traversal: {:?} (sum: {})", box_time, sum);
    println!("Rc list traversal: {:?} (sum: {})", rc_time, sum2);
    println!(
        "Cache locality difference: {:.2}x",
        rc_time.as_nanos() as f64 / box_time.as_nanos() as f64
    );
}

fn main() {
    println!("=== Step 7: Performance Comparison ===");
    println!("Note: Run with --release flag for meaningful benchmarks!");
    println!();

    memory_usage_comparison();
    println!();

    // Benchmark different sizes
    let sizes = [1000, 5000, 10000];

    for &size in &sizes {
        println!("=== Benchmarking {} elements ===", size);

        let box_time = benchmark_box_list(size);
        let rc_time = benchmark_rc_list(size);

        println!("Box<T> implementation: {:?}", box_time);
        println!("Rc<RefCell<T>> implementation: {:?}", rc_time);

        let overhead_ratio = rc_time.as_nanos() as f64 / box_time.as_nanos() as f64;
        println!("Overhead ratio: {:.2}x", overhead_ratio);

        if overhead_ratio > 3.0 {
            println!("⚠️  Significant overhead detected!");
        } else if overhead_ratio > 2.0 {
            println!("⚠️  Moderate overhead");
        } else {
            println!("✅ Reasonable overhead");
        }

        println!();
    }

    cache_locality_demonstration();

    println!("📝 Performance Insights:");
    println!("- Box<T>: Minimal overhead, excellent cache locality");
    println!("- Rc<RefCell<T>>: Higher memory usage, runtime borrow checking overhead");
    println!("- Reference counting adds CPU cycles for increment/decrement");
    println!("- Multiple heap allocations can hurt cache performance");
    println!("- Choose Box<T> for performance-critical code");
    println!("- Choose Rc<RefCell<T>> when you need shared ownership");

    println!("✅ Performance comparison complete!");
}
