// Step 2 Enhanced: Adding peek operations to use the fields
// This eliminates the warnings by actually reading the data

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

    // ✅ These methods READ the fields, eliminating warnings
    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data) // Reads 'data' field
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    // ✅ Method that traverses the list (reads 'next' field)
    pub fn contains(&self, target: &T) -> bool
    where
        T: PartialEq,
    {
        let mut current = &self.head;
        while let Some(node) = current {
            if node.data == *target {
                // Reads 'data' field
                return true;
            }
            current = &node.next; // Reads 'next' field
        }
        false
    }
}

fn main() {
    println!("=== Step 2 Enhanced: No Warnings ===");

    let mut list = SimpleLinkedList::new();
    list.push_front(42);
    list.push_front(24);
    list.push_front(13);

    // ✅ Now we actually USE the fields
    if let Some(value) = list.peek_front() {
        println!("Front value: {}", value); // Uses 'data' field
    }

    println!("Contains 24: {}", list.contains(&24)); // Uses both fields
    println!("List: {:?}", list);
}
