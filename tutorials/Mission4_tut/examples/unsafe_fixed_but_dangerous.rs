// Demonstration: Raw pointers "work" with unsafe blocks, but are dangerous!
// This compiles but is prone to memory bugs, data races, and undefined behavior

struct Node<T> {
    data: T,
    next: *mut Node<T>, // Raw pointer - still dangerous even with unsafe!
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
        // ⚠️ "Fixed" with unsafe block - but still dangerous!
        let new_node = unsafe {
            Box::into_raw(Box::new(Node {
                data,
                next: self.head,
            }))
        };
        self.head = new_node;
    }

    fn pop_front(&mut self) -> Option<T> {
        if self.head.is_null() {
            None
        } else {
            // ⚠️ "Fixed" with unsafe block - but could access freed memory!
            unsafe {
                let node = *Box::from_raw(self.head);
                self.head = node.next;
                Some(node.data)
            }
        }
    }

    fn peek_front(&self) -> Option<&T> {
        if self.head.is_null() {
            None
        } else {
            // ⚠️ "Fixed" with unsafe block - but could return dangling reference!
            unsafe { Some(&(*self.head).data) }
        }
    }

    fn modify_front(&mut self, new_data: T) -> bool {
        if self.head.is_null() {
            false
        } else {
            // ⚠️ "Fixed" with unsafe block - but no protection from data races!
            unsafe {
                (*self.head).data = new_data;
                true
            }
        }
    }
}

// ⚠️ This compiles and runs, but has serious safety issues:
// - No automatic memory cleanup (memory leaks)
// - Potential use-after-free bugs
// - Possible data races in concurrent code
// - No protection from dangling pointers

fn main() {
    let mut list = UnsafeLinkedList::new();
    list.push_front(42);
    list.push_front(24);

    println!("Peeked: {:?}", list.peek_front());
    println!("Popped: {:?}", list.pop_front());
    println!("Popped: {:?}", list.pop_front());

    // ⚠️ Memory leak! The nodes aren't properly cleaned up
    println!("✅ Compiles and runs, but unsafe and error-prone!");
    println!("🚨 This is why Rust prefers Box<T> and Rc<RefCell<T>>!");
}
