//! Chapter 15 Exercises - Comprehensive Practice
//!
//! These exercises test understanding of all Chapter 15 concepts through
//! practical implementation challenges with increasing complexity.

#[cfg(test)]
mod exercise_tests {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    /// Exercise 1: Implement a cons list using Box<T>
    #[cfg(test)]
    mod exercise_1_cons_list {
    
    #[derive(Debug, PartialEq)]
    enum List<T> {
        Cons(T, Box<List<T>>),
        Nil,
    }
    
    impl<T> List<T> {
        fn new() -> Self {
            List::Nil
        }
        
        fn prepend(self, value: T) -> Self {
            List::Cons(value, Box::new(self))
        }
        
        fn len(&self) -> usize {
            match self {
                List::Cons(_, tail) => 1 + tail.len(),
                List::Nil => 0,
            }
        }
    }
    
    #[test]
    fn test_cons_list() {
        let list = List::new()
            .prepend(3)
            .prepend(2)
            .prepend(1);
        
        assert_eq!(list.len(), 3);
    }
}

/// Exercise 2: Create a custom smart pointer with Deref
#[cfg(test)]
mod exercise_2_custom_smart_pointer {
    use std::ops::Deref;
    
    struct SmartPointer<T> {
        value: T,
        access_count: std::cell::Cell<usize>,
    }
    
    impl<T> SmartPointer<T> {
        fn new(value: T) -> Self {
            SmartPointer {
                value,
                access_count: std::cell::Cell::new(0),
            }
        }
        
        fn access_count(&self) -> usize {
            self.access_count.get()
        }
    }
    
    impl<T> Deref for SmartPointer<T> {
        type Target = T;
        
        fn deref(&self) -> &Self::Target {
            self.access_count.set(self.access_count.get() + 1);
            &self.value
        }
    }
    
    #[test]
    fn test_smart_pointer() {
        let sp = SmartPointer::new(String::from("hello"));
        
        let len = sp.len(); // Deref coercion
        assert_eq!(len, 5);
        assert_eq!(sp.access_count(), 1);
        
        let _chars = sp.chars(); // Another access
        assert_eq!(sp.access_count(), 2);
    }
}

/// Exercise 3: Implement a resource with Drop cleanup
#[cfg(test)]
mod exercise_3_resource_cleanup {
    use std::cell::RefCell;
    use std::rc::Rc;
    
    struct Resource {
        name: String,
        cleanup_log: Rc<RefCell<Vec<String>>>,
    }
    
    impl Resource {
        fn new(name: String, cleanup_log: Rc<RefCell<Vec<String>>>) -> Self {
            Resource { name, cleanup_log }
        }
    }
    
    impl Drop for Resource {
        fn drop(&mut self) {
            self.cleanup_log.borrow_mut().push(format!("Cleaned up: {}", self.name));
        }
    }
    
    #[test]
    fn test_resource_cleanup() {
        let log = Rc::new(RefCell::new(Vec::new()));
        
        {
            let _r1 = Resource::new(String::from("Resource1"), Rc::clone(&log));
            let _r2 = Resource::new(String::from("Resource2"), Rc::clone(&log));
            let _r3 = Resource::new(String::from("Resource3"), Rc::clone(&log));
        }
        
        // Resources dropped in reverse order
        assert_eq!(log.borrow().len(), 3);
        assert_eq!(log.borrow()[0], "Cleaned up: Resource3");
        assert_eq!(log.borrow()[1], "Cleaned up: Resource2");
        assert_eq!(log.borrow()[2], "Cleaned up: Resource1");
    }
}

/// Exercise 4: Build a shared data structure with Rc<T>
#[cfg(test)]
mod exercise_4_shared_data {
    use super::*;
    
    #[derive(Debug)]
    struct SharedConfig {
        settings: Rc<Vec<String>>,
    }
    
    impl SharedConfig {
        fn new(settings: Vec<String>) -> Self {
            SharedConfig {
                settings: Rc::new(settings),
            }
        }
        
        fn clone_settings(&self) -> Rc<Vec<String>> {
            Rc::clone(&self.settings)
        }
        
        fn ref_count(&self) -> usize {
            Rc::strong_count(&self.settings)
        }
    }
    
    #[test]
    fn test_shared_config() {
        let config = SharedConfig::new(vec![
            String::from("setting1"),
            String::from("setting2"),
        ]);
        
        assert_eq!(config.ref_count(), 1);
        
        let clone1 = config.clone_settings();
        assert_eq!(config.ref_count(), 2);
        
        let clone2 = config.clone_settings();
        assert_eq!(config.ref_count(), 3);
        
        drop(clone1);
        assert_eq!(config.ref_count(), 2);
        
        drop(clone2);
        assert_eq!(config.ref_count(), 1);
    }
}

/// Exercise 5: Implement interior mutability with RefCell<T>
#[cfg(test)]
mod exercise_5_interior_mutability {
    use super::*;
    
    struct Counter {
        count: RefCell<usize>,
    }
    
    impl Counter {
        fn new() -> Self {
            Counter {
                count: RefCell::new(0),
            }
        }
        
        fn increment(&self) {
            *self.count.borrow_mut() += 1;
        }
        
        fn get(&self) -> usize {
            *self.count.borrow()
        }
    }
    
    #[test]
    fn test_interior_mutability() {
        let counter = Counter::new();
        
        counter.increment();
        counter.increment();
        counter.increment();
        
        assert_eq!(counter.get(), 3);
    }
}

/// Exercise 6: Create tree structure with Weak references
#[cfg(test)]
mod exercise_6_tree_structure {
    use super::*;
    
    #[derive(Debug)]
    struct TreeNode {
        value: i32,
        parent: RefCell<Weak<TreeNode>>,
        children: RefCell<Vec<Rc<TreeNode>>>,
    }
    
    impl TreeNode {
        fn new(value: i32) -> Rc<Self> {
            Rc::new(TreeNode {
                value,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![]),
            })
        }
        
        fn add_child(parent: &Rc<TreeNode>, child: Rc<TreeNode>) {
            *child.parent.borrow_mut() = Rc::downgrade(parent);
            parent.children.borrow_mut().push(child);
        }
    }
    
    #[test]
    fn test_tree_structure() {
        let root = TreeNode::new(1);
        let child1 = TreeNode::new(2);
        let child2 = TreeNode::new(3);
        
        TreeNode::add_child(&root, Rc::clone(&child1));
        TreeNode::add_child(&root, Rc::clone(&child2));
        
        assert_eq!(root.children.borrow().len(), 2);
        assert_eq!(Rc::weak_count(&root), 2);
        
        // Child can access parent
        let parent = child1.parent.borrow().upgrade().unwrap();
        assert_eq!(parent.value, 1);
    }
}

/// Exercise 7: Combine Rc<RefCell<T>> for mutable shared data
#[cfg(test)]
mod exercise_7_mutable_shared_data {
    use super::*;
    
    struct SharedBuffer {
        data: Rc<RefCell<Vec<u8>>>,
    }
    
    impl SharedBuffer {
        fn new() -> Self {
            SharedBuffer {
                data: Rc::new(RefCell::new(Vec::new())),
            }
        }
        
        fn clone_buffer(&self) -> Rc<RefCell<Vec<u8>>> {
            Rc::clone(&self.data)
        }
        
        fn append(&self, byte: u8) {
            self.data.borrow_mut().push(byte);
        }
        
        fn len(&self) -> usize {
            self.data.borrow().len()
        }
    }
    
    #[test]
    fn test_mutable_shared_data() {
        let buffer = SharedBuffer::new();
        let shared1 = buffer.clone_buffer();
        let shared2 = buffer.clone_buffer();
        
        buffer.append(1);
        assert_eq!(buffer.len(), 1);
        
        shared1.borrow_mut().push(2);
        assert_eq!(buffer.len(), 2);
        
        shared2.borrow_mut().push(3);
        assert_eq!(buffer.len(), 3);
        
        // All references see the same data
        assert_eq!(*shared1.borrow(), vec![1, 2, 3]);
        assert_eq!(*shared2.borrow(), vec![1, 2, 3]);
    }
}

} // end exercise_tests module
