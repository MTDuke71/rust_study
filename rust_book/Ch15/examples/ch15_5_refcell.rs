//! Chapter 15.5: RefCell<T> and the Interior Mutability Pattern
//!
//! This example demonstrates interior mutability with RefCell<T>,
//! runtime borrow checking, and combining Rc<RefCell<T>>.

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("=== Chapter 15.5: RefCell<T> - Interior Mutability ===\n");

    basic_refcell_usage();
    runtime_borrow_checking();
    interior_mutability_pattern();
    rc_refcell_combination();
    compile_time_vs_runtime();

    println!("\n✅ Chapter 15.5 RefCell<T> concepts demonstrated!");
}

/// Demonstrates basic RefCell<T> borrow and borrow_mut
///
/// **Book Reference**: Section 15.5 - Interior Mutability
/// **Key Learning**: RefCell enforces borrowing rules at runtime
fn basic_refcell_usage() {
    println!("--- Basic RefCell<T> Usage ---");

    let data = RefCell::new(5);

    // Immutable borrow
    {
        let r1 = data.borrow();
        let r2 = data.borrow();
        println!("r1: {}, r2: {}", r1, r2);
        // Multiple immutable borrows OK
    } // Borrows dropped here

    // Mutable borrow
    {
        let mut w = data.borrow_mut();
        *w += 10;
        println!("After mutation: {}", w);
    } // Mutable borrow dropped here

    println!("Final value: {}", data.borrow());
    println!("✓ RefCell allows runtime borrow checking\n");
}

/// Shows runtime borrow checking and panic scenarios
///
/// **Book Reference**: Section 15.5 - Keeping Track of Borrows at Runtime
/// **Integration**: Used in [[mission-4]] for mutable linked list nodes
fn runtime_borrow_checking() {
    println!("--- Runtime Borrow Checking ---");

    let data = RefCell::new(vec![1, 2, 3]);

    // Safe: Multiple immutable borrows
    {
        let r1 = data.borrow();
        let r2 = data.borrow();
        println!(
            "Multiple immutable borrows: r1.len()={}, r2.len()={}",
            r1.len(),
            r2.len()
        );
    }

    // Safe: Single mutable borrow
    {
        let mut w = data.borrow_mut();
        w.push(4);
        println!("Mutable borrow: pushed 4, len={}", w.len());
    }

    // WOULD PANIC: Simultaneous mutable and immutable borrow
    // let r = data.borrow();
    // let mut w = data.borrow_mut();  // PANIC!

    println!("✓ RefCell panics on invalid borrow patterns at runtime\n");
}

/// Demonstrates interior mutability pattern for testing
fn interior_mutability_pattern() {
    println!("--- Interior Mutability Pattern (Mock Testing) ---");

    trait Messenger {
        fn send(&self, msg: &str); // Note: takes &self, not &mut self
    }

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // Interior mutability: mutate through &self
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        fn set_value(&mut self, value: usize) {
            self.value = value;
            let percentage = self.value as f64 / self.max as f64;

            if percentage >= 1.0 {
                self.messenger.send("Error: Over quota!");
            } else if percentage >= 0.9 {
                self.messenger.send("Urgent: 90% quota used!");
            } else if percentage >= 0.75 {
                self.messenger.send("Warning: 75% quota used!");
            }
        }
    }

    // Test the pattern
    let mock = MockMessenger::new();
    let mut tracker = LimitTracker::new(&mock, 100);

    tracker.set_value(80);

    println!("Messages sent: {}", mock.sent_messages.borrow().len());
    println!("Message: {}", mock.sent_messages.borrow()[0]);

    println!("✓ Interior mutability enables mock objects with &self methods\n");
}

/// Shows combining Rc<T> and RefCell<T> for shared mutable data
fn rc_refcell_combination() {
    println!("--- Combining Rc<RefCell<T>> ---");

    #[allow(dead_code)]
    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    println!("Initial state:");
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);

    // Mutate the shared value
    *value.borrow_mut() += 10;

    println!("\nAfter mutating shared value:");
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);

    println!("\n✓ Rc<RefCell<T>> enables multiple owners with mutation");
    println!("  Pattern: Multiple ownership (Rc) + Interior mutability (RefCell)\n");
}

/// Additional pattern: Multiple mutable references (compile-time vs runtime)
#[allow(dead_code)]
fn compile_time_vs_runtime() {
    println!("--- Compile-time vs Runtime Borrowing ---");

    // Compile-time: This doesn't compile
    // let mut x = 5;
    // let r1 = &mut x;
    // let r2 = &mut x;  // ERROR: cannot borrow as mutable more than once

    // Runtime: This compiles but panics if rules violated
    let x = RefCell::new(5);
    let _r1 = x.borrow_mut();
    // let r2 = x.borrow_mut();  // PANIC at runtime!

    println!("✓ RefCell trades compile-time safety for flexibility");
    println!("  Use when you know borrowing is safe but compiler can't verify\n");
}
