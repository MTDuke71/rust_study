//! # Chapter 7.3: Paths for Referring to an Item in the Module Tree
//!
//! Demonstrates paths and module navigation from The Rust Book.
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
        #[allow(dead_code)] // Used internally in summer() method
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

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn example1_absolute_paths() {
    println!("📍 Example 1: Absolute Paths");
    println!("============================");

    // Absolute paths start with crate
    crate::front_of_house::hosting::add_to_waitlist();

    println!("Absolute path: crate::front_of_house::hosting::add_to_waitlist()");
    println!("Always starts with 'crate' keyword");
    println!();
}

fn example2_relative_paths() {
    println!("🔄 Example 2: Relative Paths");
    println!("============================");

    // Relative paths start with module name
    front_of_house::hosting::add_to_waitlist();

    println!("Relative path: front_of_house::hosting::add_to_waitlist()");
    println!("Starts with current module or parent module");
    println!();
}

fn example3_using_structs() {
    println!("🏗️  Example 3: Using Structs from Modules");
    println!("=========================================");

    // Create a breakfast using the public constructor
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Can access public fields
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // Cannot access private fields
    // meal.seasonal_fruit = String::from("blueberries"); // Error!

    println!("Can access public fields: meal.toast");
    println!("Cannot access private fields: meal.seasonal_fruit");
    println!();
}

fn example4_using_enums() {
    println!("🎯 Example 4: Using Enums from Modules");
    println!("=====================================");

    // All enum variants are public if the enum is public
    let order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;

    match order1 {
        back_of_house::Appetizer::Soup => println!("Ordered soup"),
        back_of_house::Appetizer::Salad => println!("Ordered salad"),
    }

    println!("All enum variants are public when enum is public");
    println!();
}

// Module for demonstrating super keyword
mod super_example {
    fn serve_order() {
        println!("Serving order");
    }

    pub mod back_of_house {
        pub fn fix_incorrect_order() {
            // super refers to the parent module (super_example)
            super::serve_order();
            println!("Fixing incorrect order");
        }
    }
}

fn example5_super_keyword() {
    println!("⬆️  Example 5: Using super Keyword");
    println!("=================================");

    // super refers to the parent module
    // Useful for relative paths that go up the module tree

    super_example::back_of_house::fix_incorrect_order();

    println!("super keyword refers to parent module");
    println!("Useful for going up the module tree");
    println!();
}

fn example6_self_keyword() {
    println!("🔄 Example 6: Using self Keyword");
    println!("================================");

    // self refers to the current module
    // Useful for relative paths within the same module

    mod example_module {
        pub fn public_function() {
            println!("This is a public function");
        }

        fn private_function() {
            println!("This is a private function");
        }

        pub fn call_both() {
            // self refers to current module
            self::public_function();
            self::private_function();
        }
    }

    example_module::call_both();
    println!("self keyword refers to current module");
    println!();
}

fn main() {
    println!("📚 Chapter 7.3: Paths for Referring to an Item in the Module Tree\n");

    example1_absolute_paths();
    example2_relative_paths();
    example3_using_structs();
    example4_using_enums();
    example5_super_keyword();
    example6_self_keyword();

    println!("✅ All examples completed!");
    println!("📖 Next: Run examples in ../paths/");
}
