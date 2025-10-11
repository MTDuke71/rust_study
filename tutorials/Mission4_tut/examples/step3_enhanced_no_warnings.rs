// Step 3 Enhanced: Safe Peeking with Traversal
// This version reads the 'next' field, eliminating warnings

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

    // ✅ Original peek methods (read 'data' field)
    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.data)
    }

    // ✅ New methods that read 'next' field, eliminating warnings
    pub fn peek_second(&self) -> Option<&T> {
        self.head
            .as_ref()
            .and_then(|first_node| first_node.next.as_ref()) // ← Reads 'next'
            .map(|second_node| &second_node.data)
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    // ✅ Iterator that traverses using 'next' fields
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        LinkedListIter {
            current: self.head.as_ref(),
        }
    }
}

// ✅ Iterator implementation that reads 'next' fields
struct LinkedListIter<'a, T> {
    current: Option<&'a Box<Node<T>>>,
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_ref(); // ← Reads 'next' field!
            &node.data
        })
    }
}

fn main() {
    println!("=== Step 3 Enhanced: No Warnings ===");

    let mut list = SimpleLinkedList::new();
    list.push_front(42);
    list.push_front(24);
    list.push_front(13);

    // ✅ Peek operations
    println!("First element: {:?}", list.peek_front());
    println!("Second element: {:?}", list.peek_second()); // Uses 'next'

    // ✅ Mutable peek
    if let Some(front) = list.peek_front_mut() {
        *front += 100;
        println!("Modified first element: {}", front);
    }

    // ✅ Iteration that uses 'next' fields
    print!("All elements: ");
    for value in list.iter() {
        // Uses 'next' for traversal
        print!("{} ", value);
    }
    println!();

    println!("List: {:?}", list);
}
