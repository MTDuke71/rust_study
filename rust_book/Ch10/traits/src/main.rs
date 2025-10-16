//! # Chapter 10.2: Traits - Defining Shared Behavior
//!
//! Demonstrates trait definitions, implementations, and usage patterns.
//! Shows how traits enable polymorphism and code reuse in Rust.
//!
//! Run with: `cargo run`

fn example1_basic_trait_definition() {
    println!("🦀 Example 1: Basic Trait Definition and Implementation");
    println!("======================================================");

    // Define a trait
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    // Define a trait with default implementation
    pub trait SummaryWithDefault {
        fn summarize_default(&self) -> String {
            String::from("(Read more...)")
        }
    }

    // Implement the trait for a struct
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    impl SummaryWithDefault for NewsArticle {
        // Uses default implementation
    }

    // Another struct implementing the same trait
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    impl SummaryWithDefault for Tweet {
        fn summarize_default(&self) -> String {
            format!("Tweet by {}: {}", self.username, self.content)
        }
    }

    // Using the implementations
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("New article available! {}", article.summarize());
    println!("New tweet: {}", tweet.summarize());
    println!("Default summary: {}", article.summarize_default());

    println!();
}

fn example2_trait_bounds() {
    println!("🔧 Example 2: Trait Bounds in Generic Functions");
    println!("===============================================");

    // Define a simple trait
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct NewsArticle {
        pub headline: String,
        pub author: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {}", self.headline, self.author)
        }
    }

    // Generic function with trait bound
    pub fn notify<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    // Multiple trait bounds
    use std::fmt::Display;
    pub fn notify_and_display<T: Summary + Display>(item: &T) {
        println!("Breaking news! {}", item.summarize());
        println!("Item: {}", item);
    }

    // Where clause for cleaner syntax
    pub fn some_function<T, U>(t: &T, u: &U) -> i32 
    where
        T: Display + Clone,
        U: Clone + std::fmt::Debug,
    {
        println!("t: {}", t);
        println!("u: {:?}", u);
        42
    }

    let article = NewsArticle {
        headline: String::from("Rust 1.70 released!"),
        author: String::from("Rust Team"),
    };

    notify(&article);

    println!();
}

fn example3_trait_objects() {
    println!("🎯 Example 3: Trait Objects and Dynamic Dispatch");
    println!("================================================");

    // Define a trait
    pub trait Draw {
        fn draw(&self);
    }

    // Implement for different types
    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            println!("Drawing a button: {}x{} with label '{}'", 
                     self.width, self.height, self.label);
        }
    }

    pub struct SelectBox {
        pub width: u32,
        pub height: u32,
        pub options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            println!("Drawing a select box: {}x{} with {} options", 
                     self.width, self.height, self.options.len());
        }
    }

    // Screen that can hold any type that implements Draw
    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    // Using trait objects
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    println!();
}

fn example4_associated_types() {
    println!("📦 Example 4: Associated Types in Traits");
    println!("========================================");

    // Trait with associated type
    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }

    // Counter that implements Iterator
    pub struct Counter {
        count: u32,
    }

    impl Counter {
        pub fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    // Using the iterator
    let mut counter = Counter::new();
    println!("Counter values:");
    while let Some(value) = counter.next() {
        println!("  {}", value);
    }

    println!();
}

fn example5_default_implementations() {
    println!("⚡ Example 5: Default Implementations");
    println!("=====================================");

    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }

        fn summarize_author(&self) -> String;

        // Default implementation that calls another method
        fn summarize_enhanced(&self) -> String {
            format!("{} Author: {}", self.summarize(), self.summarize_author())
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub author: String,
    }

    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            self.author.clone()
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }

        fn summarize_author(&self) -> String {
            self.username.clone()
        }
    }

    let article = NewsArticle {
        headline: String::from("Breaking News"),
        author: String::from("Jane Doe"),
    };

    let tweet = Tweet {
        username: String::from("johndoe"),
        content: String::from("Hello, world!"),
    };

    println!("Article summary: {}", article.summarize());
    println!("Article enhanced: {}", article.summarize_enhanced());
    println!("Tweet summary: {}", tweet.summarize());
    println!("Tweet enhanced: {}", tweet.summarize_enhanced());

    println!();
}

fn example6_trait_bounds_with_impl() {
    println!("🔗 Example 6: Trait Bounds with impl");
    println!("====================================");

    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct NewsArticle {
        pub headline: String,
        pub author: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {}", self.headline, self.author)
        }
    }

    // Implementing a trait for any type that implements another trait
    impl<T: Summary> Summary for Vec<T> {
        fn summarize(&self) -> String {
            if self.is_empty() {
                String::from("No items to summarize")
            } else {
                format!("{} items: {}", self.len(), self[0].summarize())
            }
        }
    }

    let articles = vec![
        NewsArticle {
            headline: String::from("Article 1"),
            author: String::from("Author 1"),
        },
        NewsArticle {
            headline: String::from("Article 2"),
            author: String::from("Author 2"),
        },
    ];

    println!("Articles summary: {}", articles.summarize());

    println!();
}

fn main() {
    println!("📚 Chapter 10.2: Traits - Defining Shared Behavior\n");

    example1_basic_trait_definition();
    example2_trait_bounds();
    example3_trait_objects();
    example4_associated_types();
    example5_default_implementations();
    example6_trait_bounds_with_impl();

    println!("✅ All examples completed!");
    println!("📖 Next: Read Chapter 10.3 (Lifetimes) or run examples in ../lifetimes/");
}
