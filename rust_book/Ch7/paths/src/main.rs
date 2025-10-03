//! # Chapter 7.4: Bringing Paths into Scope with the use Keyword
//!
//! Demonstrates the use keyword and path management from The Rust Book.
//!
//! Run with: `cargo run`

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Adding customer to waitlist");
        }
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

fn example1_basic_use() {
    println!("📥 Example 1: Basic use Keyword");
    println!("===============================");

    // Bring a path into scope with use
    use crate::front_of_house::hosting;

    // Now we can call it directly
    hosting::add_to_waitlist();

    println!("use brings paths into scope for shorter calls");
    println!("Before: crate::front_of_house::hosting::add_to_waitlist()");
    println!("After:  hosting::add_to_waitlist()");
    println!();
}

fn example2_use_with_as() {
    println!("🏷️  Example 2: use with as Keyword");
    println!("=================================");

    // Use 'as' to create an alias
    use crate::front_of_house::hosting as restaurant_hosting;

    restaurant_hosting::add_to_waitlist();

    println!("'as' keyword creates aliases for paths");
    println!("Useful for avoiding naming conflicts");
    println!();
}

fn example3_re_exporting() {
    println!("📤 Example 3: Re-exporting with pub use");
    println!("======================================");

    // Re-export makes the path available to external users
    pub use crate::front_of_house::hosting;

    // External code can now use: my_crate::hosting::add_to_waitlist()
    hosting::add_to_waitlist();

    println!("pub use re-exports paths for external users");
    println!("Makes internal structure more convenient for users");
    println!();
}

fn example4_nested_paths() {
    println!("🌳 Example 4: Nested Paths");
    println!("=========================");

    // Bring multiple items from the same module
    use std::collections::{HashMap, HashSet};

    let mut map = HashMap::new();
    map.insert("key", "value");

    let mut set = HashSet::new();
    set.insert("item");

    println!("Nested paths: use std::collections::{{HashMap, HashSet}}");
    println!("Brings multiple items from same module");
    println!();
}

fn example5_glob_operator() {
    println!("🌟 Example 5: Glob Operator");
    println!("==========================");

    // Glob operator brings all public items into scope
    use std::collections::*;

    // Now all public items from std::collections are available
    let mut map = HashMap::new();
    let mut set = HashSet::new();
    let mut vec_deque = VecDeque::new();

    println!("Glob operator (*) brings all public items into scope");
    println!("Use sparingly - can cause naming conflicts");
    println!();
}

fn example6_use_best_practices() {
    println!("✅ Example 6: use Best Practices");
    println!("===============================");

    // Best practices for use statements:
    // 1. Group use statements by source
    // 2. Use relative paths when possible
    // 3. Avoid glob operator in most cases
    // 4. Place use statements at the top of the file

    use std::collections::HashMap;
    use crate::front_of_house::hosting;

    println!("Best practices:");
    println!("  1. Group by source (std, external, local)");
    println!("  2. Use relative paths when possible");
    println!("  3. Avoid glob operator (*)");
    println!("  4. Place at top of file");
    println!();
}

fn example7_use_in_modules() {
    println!("🏠 Example 7: use in Modules");
    println!("===========================");

    mod restaurant {
        use crate::front_of_house::hosting;

        pub fn take_reservation() {
            hosting::add_to_waitlist();
        }
    }

    restaurant::take_reservation();

    println!("use statements work inside modules too");
    println!("Each module can have its own use statements");
    println!();
}

fn main() {
    println!("📚 Chapter 7.4: Bringing Paths into Scope with the use Keyword\n");

    example1_basic_use();
    example2_use_with_as();
    example3_re_exporting();
    example4_nested_paths();
    example5_glob_operator();
    example6_use_best_practices();
    example7_use_in_modules();

    println!("✅ All examples completed!");
    println!("📖 Next: Run examples in ../visibility/");
}
