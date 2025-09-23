// Demonstration of why raw pointers cause compile errors in safe Rust
// This code intentionally contains unsafe patterns to show compiler errors

// ❌ This won't compile in safe Rust!
struct Node<T> {
    data: T,
    next: *mut Node<T>,  // Raw pointer - unsafe!
}

struct UnsafeLinkedList<T> {
    head: *mut Node<T>,
}

impl<T> UnsafeLinkedList<T> {
    fn new() -> Self {
        Self {
            head: std::ptr::null_mut(),
        }
    }
    
    fn push_front(&mut self, data: T) {
        // ❌ ERROR 1: Box::into_raw is unsafe
        let new_node = Box::into_raw(Box::new(Node {
            data,
            next: self.head,
        }));
        self.head = new_node;
    }
    
    fn pop_front(&mut self) -> Option<T> {
        if self.head.is_null() {
            None
        } else {
            // ❌ ERROR 2: Box::from_raw is unsafe - could be accessing freed memory!
            let node = *Box::from_raw(self.head);
            self.head = node.next;
            Some(node.data)
        }
    }
    
    fn peek_front(&self) -> Option<&T> {
        if self.head.is_null() {
            None
        } else {
            // ❌ ERROR 3: Dereferencing raw pointer is unsafe
            Some(&(*self.head).data)
        }
    }
    
    fn modify_front(&mut self, new_data: T) -> bool {
        if self.head.is_null() {
            false
        } else {
            // ❌ ERROR 4: Another unsafe dereference
            (*self.head).data = new_data;
            true
        }
    }
}

fn main() {
    let mut list = UnsafeLinkedList::new();
    list.push_front(42);
    list.push_front(24);
    
    // Try to use the list
    println!("Popped: {:?}", list.pop_front());
    println!("Popped: {:?}", list.pop_front());
}