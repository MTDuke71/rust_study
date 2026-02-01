//! Interior Mutability Examples
//!
//! Demonstrates Cell, RefCell, Mutex, and UnsafeCell patterns
//! for mutating through shared references

use std::cell::{Cell, RefCell};
use std::rc::Rc;

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  Interior Mutability                                     ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // === CONCEPT ===
    {
        println!("📚 INTERIOR MUTABILITY CONCEPT");
        println!("   ────────────────────────────");
        println!("   Mutate data through SHARED reference (&T)");
        println!("   Bypasses \"shared XOR mutable\" rule");
        println!("   Uses RUNTIME borrow checking");
        println!("   Built on UnsafeCell<T> primitive\n");
    }

    // === CELL<T> ===
    {
        println!("📚 CELL<T>: Copy Types Only");
        println!("   ──────────────────────────");

        let x = Cell::new(42);
        let y = &x; // Shared reference!

        println!("   Initial: {}", x.get());
        y.set(84); // Mutate through shared reference!
        println!("   After set: {}", x.get());

        // Multiple shared references can mutate
        let z = &x;
        z.set(100);
        println!("   After another set: {}", x.get());

        println!("   ✓ No borrow checking, only Copy types\n");
    }

    {
        println!("📚 CELL<T>: Use Cases");
        println!("   ───────────────────");

        struct Counter {
            count: Cell<i32>,
        }

        impl Counter {
            fn new() -> Self {
                Counter {
                    count: Cell::new(0),
                }
            }

            fn increment(&self) {
                // Takes &self, not &mut self!
                self.count.set(self.count.get() + 1);
            }

            fn get(&self) -> i32 {
                self.count.get()
            }
        }

        let counter = Counter::new();
        counter.increment();
        counter.increment();
        counter.increment();
        println!("   Counter: {}", counter.get());
        println!("   ✓ Mutate through &self method\n");
    }

    // === REFCELL<T> ===
    {
        println!("📚 REFCELL<T>: Runtime Borrow Checking");
        println!("   ─────────────────────────────────────");

        let data = RefCell::new(vec![1, 2, 3]);

        // Immutable borrow
        {
            let borrowed = data.borrow();
            println!("   Borrowed: {:?}", *borrowed);
        } // Borrow ends

        // Mutable borrow
        {
            let mut borrowed_mut = data.borrow_mut();
            borrowed_mut.push(4);
            println!("   Modified: {:?}", *borrowed_mut);
        } // Mutable borrow ends

        println!("   ✓ Borrow rules enforced at runtime\n");
    }

    {
        println!("📚 REFCELL<T>: Panics on Violation");
        println!("   ─────────────────────────────────");

        let data = RefCell::new(5);

        let _r1 = data.borrow(); // Immutable borrow
        let _r2 = data.borrow(); // Another immutable borrow - OK

        // This would panic at runtime:
        // let _r3 = data.borrow_mut();  // ❌ PANIC: already borrowed

        println!("   Multiple immutable borrows: ✓");
        println!("   Would panic on simultaneous mutable borrow\n");
    }

    {
        println!("📚 REFCELL<T>: Interior Mutability Pattern");
        println!("   ─────────────────────────────────────────");

        #[derive(Debug)]
        struct Config {
            cache: RefCell<Vec<String>>,
        }

        impl Config {
            fn new() -> Self {
                Config {
                    cache: RefCell::new(Vec::new()),
                }
            }

            fn add_entry(&self, entry: String) {
                // &self!
                self.cache.borrow_mut().push(entry);
            }

            fn get_cache(&self) -> Vec<String> {
                self.cache.borrow().clone()
            }
        }

        let config = Config::new();
        config.add_entry("item1".to_string());
        config.add_entry("item2".to_string());
        println!("   Cache: {:?}", config.get_cache());
        println!("   ✓ Mutate internal state through &self\n");
    }

    // === RC<REFCELL<T>> ===
    {
        println!("📚 RC<REFCELL<T>>: Shared Ownership + Mutability");
        println!("   ───────────────────────────────────────────────");

        let data = Rc::new(RefCell::new(vec![1, 2, 3]));

        let owner1 = Rc::clone(&data);
        let owner2 = Rc::clone(&data);

        owner1.borrow_mut().push(4);
        owner2.borrow_mut().push(5);

        println!("   owner1 sees: {:?}", *data.borrow());
        println!("   ✓ Multiple owners can mutate shared data\n");
    }

    {
        println!("📚 RC<REFCELL<T>>: Graph Example");
        println!("   ──────────────────────────────");

        type NodeRef = Rc<RefCell<Node>>;

        struct Node {
            value: i32,
            children: Vec<NodeRef>,
        }

        impl Node {
            fn new(value: i32) -> NodeRef {
                Rc::new(RefCell::new(Node {
                    value,
                    children: Vec::new(),
                }))
            }

            fn add_child(&mut self, child: NodeRef) {
                self.children.push(child);
            }
        }

        let root = Node::new(1);
        let child1 = Node::new(2);
        let child2 = Node::new(3);

        root.borrow_mut().add_child(child1);
        root.borrow_mut().add_child(child2);

        println!("   Root: {}", root.borrow().value);
        println!("   Children: {}", root.borrow().children.len());
        println!("   ✓ Graph structures with shared ownership\n");
    }

    // === UNSAFECELL ===
    {
        println!("📚 UNSAFECELL<T>: The Foundation");
        println!("   ──────────────────────────────");
        println!("   All interior mutability built on UnsafeCell<T>");
        println!("   Tells compiler: \"I'll handle aliasing safety\"");
        println!("   Used in Cell, RefCell, Mutex, RwLock, etc.");
        println!("   ✓ Low-level primitive for building abstractions\n");
    }

    // === THREAD-SAFE ALTERNATIVES ===
    {
        println!("📚 THREAD-SAFE: MUTEX<T> / RWLOCK<T>");
        println!("   ────────────────────────────────────");

        use std::sync::Mutex;

        let counter = Mutex::new(0);

        {
            let mut num = counter.lock().unwrap();
            *num += 1;
        } // Lock released

        println!("   Mutex counter: {}", *counter.lock().unwrap());
        println!("   ✓ Thread-safe interior mutability\n");
    }

    // === COMPARISON ===
    {
        println!("╔══════════════════════════════════════════════════════════╗");
        println!("║  Type Comparison                                         ║");
        println!("╠══════════════════════════════════════════════════════════╣");
        println!("║  Cell<T>      | Copy types only, no borrow tracking      ║");
        println!("║  RefCell<T>   | Runtime borrow checking, single-thread   ║");
        println!("║  Mutex<T>     | Thread-safe, locks, blocking             ║");
        println!("║  RwLock<T>    | Multiple readers OR one writer           ║");
        println!("║  UnsafeCell<T>| Low-level primitive, unsafe to use       ║");
        println!("╚══════════════════════════════════════════════════════════╝\n");
    }

    // === WHEN TO USE ===
    {
        println!("📚 WHEN TO USE INTERIOR MUTABILITY");
        println!("   ─────────────────────────────────");
        println!("   ✓ Caching computed values");
        println!("   ✓ Reference counting with mutation (Rc<RefCell<T>>)");
        println!("   ✓ Mock objects in tests");
        println!("   ✓ Shared mutable state (carefully!)");
        println!("   ✓ Graph/tree data structures");
        println!("   ✗ Default pattern (prefer &mut when possible)\n");
    }

    // === SAFETY CONSIDERATIONS ===
    {
        println!("📚 SAFETY CONSIDERATIONS");
        println!("   ──────────────────────");
        println!("   ⚠️  RefCell panics on borrow violations");
        println!("   ⚠️  Can't catch violations at compile time");
        println!("   ⚠️  Rc<RefCell<T>> can create reference cycles");
        println!("   ⚠️  Use Weak<T> to break cycles");
        println!("   ✓ Trade compile-time safety for flexibility\n");
    }

    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  Key Takeaway                                            ║");
    println!("╠══════════════════════════════════════════════════════════╣");
    println!("║  Interior mutability allows mutation through &T          ║");
    println!("║  Moves borrow checking from compile-time to runtime      ║");
    println!("║  Built on UnsafeCell, used by Cell/RefCell/Mutex         ║");
    println!("╚══════════════════════════════════════════════════════════╝");
}
