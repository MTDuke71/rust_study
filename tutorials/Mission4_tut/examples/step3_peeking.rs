// Step 3: Safe Peeking at Elements
// Run with: cargo run --example step3_peeking

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

impl<T> Default for SimpleLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.data)
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

fn main() {
    println!("=== Step 3: Safe Peeking ===");

    let mut list = SimpleLinkedList::new();

    // Test peeking at empty list
    println!("Empty list peek: {:?}", list.peek_front());
    assert_eq!(list.peek_front(), None);

    // Add some elements
    list.push_front(100);
    list.push_front(200);
    list.push_front(300);

    // Test immutable peeking
    println!("Peek front (immutable): {:?}", list.peek_front());
    assert_eq!(list.peek_front(), Some(&300));

    // Test multiple peeks (should be allowed)
    let peek1 = list.peek_front();
    let peek2 = list.peek_front();
    println!("Multiple peeks work: {:?} and {:?}", peek1, peek2);

    // Test mutable peeking
    if let Some(front_mut) = list.peek_front_mut() {
        println!("Original value: {}", front_mut);
        *front_mut = 999;
        println!("Modified value: {}", front_mut);
    }

    println!("After modification, peek shows: {:?}", list.peek_front());
    assert_eq!(list.peek_front(), Some(&999));

    // Demonstrate the key insight about borrowing
    println!();
    println!("📝 Key Insight:");
    println!("- as_ref() converts Option<Box<T>> to Option<&Box<T>>");
    println!("- map() transforms Option<&Box<T>> to Option<&T>");
    println!("- This gives us a reference WITHOUT taking ownership");

    println!("✅ Safe peeking works correctly!");
}
