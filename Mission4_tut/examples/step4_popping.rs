// Step 4: Removing Elements with pop_front
// Run with: cargo run --example step4_popping

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

fn demonstrate_ownership_transfer() {
    println!("=== Demonstrating Ownership Transfer ===");
    
    let mut list = SimpleLinkedList::new();
    list.push_front(String::from("Hello"));
    list.push_front(String::from("World"));
    
    println!("Before pop: {:?}", list.peek_front());
    
    // Pop returns owned data - we can modify it
    if let Some(mut popped) = list.pop_front() {
        println!("Popped (owned): {}", popped);
        popped.push_str("!");
        println!("Modified popped value: {}", popped);
    }
    
    println!("After pop: {:?}", list.peek_front());
}

fn main() {
    println!("=== Step 4: Removing Elements ===");
    
    let mut list = SimpleLinkedList::new();
    
    // Test popping from empty list
    println!("Pop from empty list: {:?}", list.pop_front());
    assert_eq!(list.pop_front(), None);
    
    // Add elements and test popping
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    
    println!("Initial list length: {}", list.len());
    println!("Initial front: {:?}", list.peek_front());
    
    // Pop elements one by one
    let popped1 = list.pop_front();
    println!("Popped: {:?}, remaining length: {}", popped1, list.len());
    println!("New front: {:?}", list.peek_front());
    
    let popped2 = list.pop_front();
    println!("Popped: {:?}, remaining length: {}", popped2, list.len());
    println!("New front: {:?}", list.peek_front());
    
    let popped3 = list.pop_front();
    println!("Popped: {:?}, remaining length: {}", popped3, list.len());
    println!("New front: {:?}", list.peek_front());
    
    // Try to pop from empty list again
    let popped4 = list.pop_front();
    println!("Popped from empty: {:?}", popped4);
    
    assert!(list.is_empty());
    
    demonstrate_ownership_transfer();
    
    println!();
    println!("📝 Key Insight about take():");
    println!("- take() moves the value OUT of the Option");
    println!("- Leaves None behind in the original location");
    println!("- This transfers ownership to us");
    println!("- We can then move parts of the owned value around");
    
    println!("✅ pop_front works correctly!");
}