// Visual breakdown of head: Option<Box<Node<T>>>
// Run with: cargo run --example type_breakdown_demo

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,  // ← This line broken down!
    length: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,  // ← Option::None = empty list
            length: 0,
        }
    }
    
    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node {  // ← Box<Node<T>> created
            data,
            next: self.head.take(),     // ← Option<Box<Node<T>>> moved
        });
        self.head = Some(new_node);     // ← Option::Some wraps the Box
        self.length += 1;
    }
}

fn main() {
    println!("=== Type Breakdown: head: Option<Box<Node<T>>> ===\n");
    
    // Step 1: Empty list
    let mut list: SimpleLinkedList<i32> = SimpleLinkedList::new();
    println!("1. Empty list:");
    println!("   head: {:?}", list.head);  // None
    println!("   Memory: Stack only, no heap allocation\n");
    
    // Step 2: Single node
    list.push(42);
    println!("2. After push(42):");
    println!("   head: {:?}", list.head);
    println!("   Memory layout:");
    println!("   Stack: list.head = Some(Box<Node<i32>>)");
    println!("          └─ Box pointer ───┐");
    println!("   Heap:                    └─ Node {{ data: 42, next: None }}\n");
    
    // Step 3: Two nodes
    list.push(24);
    println!("3. After push(24):");
    println!("   head: {:?}", list.head);
    println!("   Memory layout:");
    println!("   Stack: list.head = Some(Box<Node<i32>>)");
    println!("          └─ Box pointer ───┐");
    println!("   Heap:                    ├─ Node {{ data: 24, next: Some(...) }}");
    println!("                           └─ Box pointer ───┐");
    println!("                           Heap:             └─ Node {{ data: 42, next: None }}\n");
    
    // Type analysis
    println!("=== Type Analysis ===");
    println!("T = i32                           (generic type parameter)");
    println!("Node<T> = Node<i32>              (struct containing data + next)");
    println!("Box<Node<T>> = Box<Node<i32>>    (owned heap pointer)");
    println!("Option<Box<Node<T>>>             (nullable pointer)");
    println!("head: Option<Box<Node<T>>>       (first node or None)");
    
    // Ownership demonstration
    println!("\n=== Ownership Properties ===");
    println!("✅ Automatic memory management (no manual free)");
    println!("✅ No null pointer dereferences (Option prevents)");
    println!("✅ No memory leaks (Box cleans up on drop)");
    println!("✅ No dangling pointers (ownership prevents)");
    println!("✅ Compile-time safety (borrow checker enforces)");
}