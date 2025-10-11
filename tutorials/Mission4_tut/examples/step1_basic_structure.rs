// Step 1: Basic Box<T> Structure
// Run with: cargo run --example step1_basic_structure

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

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

fn main() {
    println!("=== Step 1: Basic Structure ===");

    // Create an empty list
    let list: SimpleLinkedList<i32> = SimpleLinkedList::new();
    println!("Created empty list: {:?}", list);
    println!("Length: {}", list.len());
    println!("Is empty: {}", list.is_empty());

    // Create a list with a specific type
    let string_list: SimpleLinkedList<String> = SimpleLinkedList::new();
    println!("Created empty string list: {:?}", string_list);

    println!("✅ Basic structure works!");
}
