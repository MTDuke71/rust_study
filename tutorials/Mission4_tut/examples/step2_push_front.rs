// Step 2: Adding Elements with push_front
// Run with: cargo run --example step2_push_front

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
            next: self.head.take(), // Move current head to new node's next
        });
        self.head = Some(new_node);
        self.length += 1;
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

fn main() {
    println!("=== Step 2: Adding Elements ===");

    let mut list = SimpleLinkedList::new();
    println!("Initial state: {:?}", list);

    // Add first element
    list.push_front(42);
    println!("After push_front(42): {:?}", list);
    println!("Length: {}", list.len());

    // Add second element
    list.push_front(24);
    println!("After push_front(24): {:?}", list);
    println!("Length: {}", list.len());

    // Add third element
    list.push_front(13);
    println!("After push_front(13): {:?}", list);
    println!("Length: {}", list.len());

    // Demonstrate that order is LIFO (Last In, First Out)
    println!();
    println!("📝 Note: Elements are added to the FRONT");
    println!("Order added: 42, 24, 13");
    println!("Current order in list: 13 -> 24 -> 42");

    println!("✅ push_front works correctly!");
}
