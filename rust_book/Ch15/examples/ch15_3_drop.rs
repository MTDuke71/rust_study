//! Chapter 15.3: Running Code on Cleanup with the Drop Trait
//!
//! This example demonstrates automatic cleanup with Drop trait,
//! drop order, and early dropping with std::mem::drop.

fn main() {
    println!("=== Chapter 15.3: Drop Trait - Automatic Cleanup ===\n");

    basic_drop_example();
    drop_order_demonstration();
    early_drop_with_std_mem();
    nested_drop_order();
    dependent_resources_example();

    println!("\n✅ Chapter 15.3 Drop trait concepts demonstrated!");
}

/// Demonstrates basic Drop trait implementation
///
/// **Book Reference**: Section 15.3 - Running Code on Cleanup
/// **Key Learning**: Drop::drop called automatically when value goes out of scope
fn basic_drop_example() {
    println!("--- Basic Drop Implementation ---");

    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("  Dropping CustomSmartPointer with data: `{}`", self.data);
        }
    }

    {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        println!("CustomSmartPointer created with: `{}`", c.data);
    } // c.drop() called here automatically

    println!("✓ Drop called automatically at end of scope\n");
}

/// Shows drop order (reverse of creation order)
///
/// **Book Reference**: Section 15.3 - Dropping a Value Early
/// **Integration**: Critical for [[mission-4]] linked list cleanup
fn drop_order_demonstration() {
    println!("--- Drop Order (LIFO) ---");

    struct OrderedDrop {
        name: String,
    }

    impl Drop for OrderedDrop {
        fn drop(&mut self) {
            println!("  Dropping: {}", self.name);
        }
    }

    println!("Creating values in order:");
    let _first = OrderedDrop {
        name: String::from("first"),
    };
    println!("  Created: first");

    let _second = OrderedDrop {
        name: String::from("second"),
    };
    println!("  Created: second");

    let _third = OrderedDrop {
        name: String::from("third"),
    };
    println!("  Created: third");

    println!("\nDropping values in reverse order:");
    // Dropped in reverse: third, second, first
    println!("✓ Values dropped in LIFO order (stack-like)\n");
}

/// Demonstrates early dropping with std::mem::drop
fn early_drop_with_std_mem() {
    println!("--- Early Drop with std::mem::drop ---");

    struct Resource {
        name: String,
    }

    impl Drop for Resource {
        fn drop(&mut self) {
            println!("  Releasing resource: {}", self.name);
        }
    }

    let resource = Resource {
        name: String::from("database connection"),
    };
    println!("Resource acquired: {}", resource.name);

    println!("Doing some work...");

    // Early drop before end of scope
    drop(resource); // Can't use resource.drop() - must use std::mem::drop
    println!("Resource released early");

    // resource is no longer available here
    println!("✓ std::mem::drop enables manual early cleanup\n");
}

/// Shows drop order with nested structures
fn nested_drop_order() {
    println!("--- Nested Structure Drop Order ---");

    struct Inner {
        id: u32,
    }

    impl Drop for Inner {
        fn drop(&mut self) {
            println!("    Dropping Inner {}", self.id);
        }
    }

    struct Outer {
        inner: Inner,
        id: u32,
    }

    impl Drop for Outer {
        fn drop(&mut self) {
            println!("  Dropping Outer {} (before inner)", self.id);
        }
    }

    println!("Creating nested structure:");
    {
        let outer = Outer {
            id: 1,
            inner: Inner { id: 100 },
        };
        println!("  Created Outer {} with Inner {}", outer.id, outer.inner.id);
    }

    println!("✓ Outer dropped first, then its fields (inner)\n");
}

/// Example of Drop with dependent resources
fn dependent_resources_example() {
    println!("--- Dependent Resources Drop Order ---");

    struct Connection {
        name: String,
    }

    impl Drop for Connection {
        fn drop(&mut self) {
            println!("  Closing connection: {}", self.name);
        }
    }

    struct Transaction {
        #[allow(dead_code)]
        connection: Connection,
        id: u32,
    }

    impl Drop for Transaction {
        fn drop(&mut self) {
            println!("  Committing transaction {}", self.id);
            // Transaction logic runs before connection is closed
        }
    }

    {
        let _txn = Transaction {
            connection: Connection {
                name: String::from("DB-1"),
            },
            id: 42,
        };
    }
    // Order: Transaction::drop(), then Connection::drop()

    println!("✓ Dependent resources dropped in correct order\n");
}
