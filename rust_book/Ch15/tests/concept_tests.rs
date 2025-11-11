//! Chapter 15 Concept Tests
//!
//! These tests validate understanding of smart pointer concepts
//! from Chapter 15 of the Rust Book.

#[cfg(test)]
mod ch15_tests {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[test]
    fn test_box_heap_allocation() {
        let boxed = Box::new(5);
        assert_eq!(*boxed, 5);
        // Box deallocates when dropped
    }

    #[test]
    fn test_box_recursive_type() {
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }
        
        use List::{Cons, Nil};
        
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        
        // List has known size due to Box indirection
        if let Cons(val, _) = list {
            assert_eq!(val, 1);
        }
    }

    #[test]
    fn test_rc_multiple_ownership() {
        let a = Rc::new(5);
        assert_eq!(Rc::strong_count(&a), 1);
        
        let b = Rc::clone(&a);
        assert_eq!(Rc::strong_count(&a), 2);
        
        let c = Rc::clone(&a);
        assert_eq!(Rc::strong_count(&a), 3);
        
        drop(c);
        assert_eq!(Rc::strong_count(&a), 2);
        
        drop(b);
        assert_eq!(Rc::strong_count(&a), 1);
    }

    #[test]
    fn test_refcell_interior_mutability() {
        let data = RefCell::new(5);
        
        // Multiple immutable borrows OK
        {
            let r1 = data.borrow();
            let r2 = data.borrow();
            assert_eq!(*r1, 5);
            assert_eq!(*r2, 5);
        }
        
        // Mutable borrow after immutable borrows dropped
        {
            let mut w = data.borrow_mut();
            *w += 10;
        }
        
        assert_eq!(*data.borrow(), 15);
    }

    #[test]
    fn test_rc_refcell_combination() {
        let value = Rc::new(RefCell::new(5));
        
        let a = Rc::clone(&value);
        let b = Rc::clone(&value);
        
        *value.borrow_mut() += 10;
        
        assert_eq!(*a.borrow(), 15);
        assert_eq!(*b.borrow(), 15);
    }

    #[test]
    fn test_weak_references() {
        let strong = Rc::new(42);
        let weak = Rc::downgrade(&strong);
        
        assert!(weak.upgrade().is_some());
        assert_eq!(*weak.upgrade().unwrap(), 42);
        
        drop(strong);
        
        assert!(weak.upgrade().is_none());
    }

    #[test]
    fn test_weak_counts() {
        let strong = Rc::new(100);
        
        assert_eq!(Rc::strong_count(&strong), 1);
        assert_eq!(Rc::weak_count(&strong), 0);
        
        let weak1 = Rc::downgrade(&strong);
        assert_eq!(Rc::weak_count(&strong), 1);
        
        let weak2 = Rc::downgrade(&strong);
        assert_eq!(Rc::weak_count(&strong), 2);
        
        drop(weak1);
        assert_eq!(Rc::weak_count(&strong), 1);
    }

    #[test]
    fn test_deref_coercion() {
        use std::ops::Deref;
        
        struct MyBox<T>(T);
        
        impl<T> Deref for MyBox<T> {
            type Target = T;
            
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        
        fn takes_str(s: &str) -> usize {
            s.len()
        }
        
        let my_string = MyBox(String::from("hello"));
        // Deref coercion: &MyBox<String> -> &String -> &str
        assert_eq!(takes_str(&my_string), 5);
    }

    #[test]
    fn test_drop_order() {
        use std::cell::RefCell;
        
        let drops = Rc::new(RefCell::new(Vec::new()));
        
        struct Tracker {
            id: usize,
            drops: Rc<RefCell<Vec<usize>>>,
        }
        
        impl Drop for Tracker {
            fn drop(&mut self) {
                self.drops.borrow_mut().push(self.id);
            }
        }
        
        {
            let _t1 = Tracker { id: 1, drops: Rc::clone(&drops) };
            let _t2 = Tracker { id: 2, drops: Rc::clone(&drops) };
            let _t3 = Tracker { id: 3, drops: Rc::clone(&drops) };
        }
        
        // Dropped in reverse order
        assert_eq!(*drops.borrow(), vec![3, 2, 1]);
    }
}
