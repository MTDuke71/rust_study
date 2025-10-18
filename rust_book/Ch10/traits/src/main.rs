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
            format!("{}, by {} ({}). Content: {}...", 
                     self.headline, self.author, self.location, 
                     &self.content[..self.content.len().min(50)])
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
            let tweet_type = if self.reply {
                "Reply"
            } else if self.retweet {
                "Retweet"
            } else {
                "Tweet"
            };
            format!("{} by {}: {}", tweet_type, self.username, self.content)
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

    // Demonstrate the unused functions
    let test_string = String::from("Hello");
    let test_number = 42;
    let result = some_function(&test_string, &test_number);
    println!("Function result: {}", result);

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
        fn summarize(&self) -> String {
            format!("News: {}", self.headline)
        }

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

fn example_encapsulation() {
    println!("🔒 Example 7: Encapsulation in Rust");
    println!("===================================");

    pub struct BankAccount {
        balance: f64,        // Private field
        account_number: u32, // Private field
    }

    impl BankAccount {
        pub fn new(initial_balance: f64, account_number: u32) -> Self {
            BankAccount { 
                balance: initial_balance,
                account_number,
            }
        }

        pub fn deposit(&mut self, amount: f64) {  // Public method
            if amount > 0.0 {
                self.balance += amount;
            }
        }

        pub fn withdraw(&mut self, amount: f64) -> Result<(), String> {
            if self.validate_transaction(amount) {
                self.balance -= amount;
                Ok(())
            } else {
                Err("Insufficient funds or invalid amount".to_string())
            }
        }

        pub fn get_balance(&self) -> f64 {  // Controlled access
            self.balance
        }

        pub fn get_account_info(&self) -> String {
            format!("Account #{}: Balance ${:.2}", self.account_number, self.balance)
        }

        // Private helper method
        fn validate_transaction(&self, amount: f64) -> bool {
            amount > 0.0 && amount <= self.balance
        }
    }

    let mut account = BankAccount::new(100.0, 12345);
    println!("Created: {}", account.get_account_info());
    
    account.deposit(50.0);
    println!("Balance after deposit: ${:.2}", account.get_balance());
    
    match account.withdraw(25.0) {
        Ok(()) => println!("Withdrawal successful. New balance: ${:.2}", account.get_balance()),
        Err(e) => println!("Withdrawal failed: {}", e),
    }
    
    // account.balance = 1000.0; // ❌ Compile error - private field
    // account.validate_transaction(50.0); // ❌ Compile error - private method

    println!();
}

fn example_polymorphism_enhanced() {
    println!("🎪 Example 8: Enhanced Polymorphism");
    println!("===================================");

    // Like an interface in Java/C#
    pub trait Animal {
        fn make_sound(&self) -> String;
        fn get_species(&self) -> &str;
        
        // Default implementation (like default methods in Java interfaces)
        fn introduce(&self) -> String {
            format!("I am a {} and I go {}", self.get_species(), self.make_sound())
        }
    }

    pub struct Dog { name: String }
    pub struct Cat { name: String }
    pub struct Bird { name: String }

    impl Animal for Dog {
        fn make_sound(&self) -> String { "Woof!".to_string() }
        fn get_species(&self) -> &str { "Dog" }
        
        fn introduce(&self) -> String {
            format!("I'm {} the {} and I go {}", self.name, self.get_species(), self.make_sound())
        }
    }

    impl Animal for Cat {
        fn make_sound(&self) -> String { "Meow!".to_string() }
        fn get_species(&self) -> &str { "Cat" }
        
        fn introduce(&self) -> String {
            format!("I'm {} the {} and I go {}", self.name, self.get_species(), self.make_sound())
        }
    }

    impl Animal for Bird {
        fn make_sound(&self) -> String { "Tweet!".to_string() }
        fn get_species(&self) -> &str { "Bird" }
        
        // Override default implementation
        fn introduce(&self) -> String {
            format!("I'm {} the {} and I sing {}", self.name, self.get_species(), self.make_sound())
        }
    }

    // Polymorphic function - works with any Animal
    fn pet_all_animals(animals: &[Box<dyn Animal>]) {
        for animal in animals {
            println!("{}", animal.introduce());
        }
    }

    let zoo: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog { name: "Buddy".to_string() }),
        Box::new(Cat { name: "Whiskers".to_string() }),
        Box::new(Bird { name: "Tweety".to_string() }),
    ];

    pet_all_animals(&zoo);

    println!();
}

fn example_composition_over_inheritance() {
    println!("🏗️ Example 9: Composition Over Inheritance");
    println!("==========================================");

    // Instead of inheritance, use composition
    struct Engine {
        horsepower: u32,
        fuel_type: String,
    }

    struct Vehicle {
        engine: Engine,      // Composition
        wheels: u8,
        make: String,
    }

    // Multiple traits instead of inheritance
    trait Drivable {
        fn drive(&self) -> String;
    }

    trait Maintainable {
        fn service(&self) -> String;
    }

    trait Insurable {
        fn get_insurance_rate(&self) -> f64;
    }

    impl Drivable for Vehicle {
        fn drive(&self) -> String {
            format!("Driving {}-wheel {} with {} HP engine", 
                    self.wheels, self.make, self.engine.horsepower)
        }
    }

    impl Maintainable for Vehicle {
        fn service(&self) -> String {
            format!("Servicing {} engine", self.engine.fuel_type)
        }
    }

    impl Insurable for Vehicle {
        fn get_insurance_rate(&self) -> f64 {
            self.engine.horsepower as f64 * 0.01
        }
    }

    let car = Vehicle {
        engine: Engine {
            horsepower: 250,
            fuel_type: "Gasoline".to_string(),
        },
        wheels: 4,
        make: "Toyota".to_string(),
    };

    println!("{}", car.drive());
    println!("{}", car.service());
    println!("Insurance rate: ${:.2}", car.get_insurance_rate());

    println!();
}

fn example_advanced_oop_patterns() {
    println!("🎨 Example 10: Advanced OOP-Style Patterns (Strategy Pattern)");
    println!("=============================================================");

    // Strategy Pattern with traits
    trait PaymentProcessor {
        fn process_payment(&self, amount: f64) -> String;
        fn get_fee(&self) -> f64;
    }

    struct CreditCardProcessor { card_type: String }
    struct PayPalProcessor { email: String }
    struct BankTransferProcessor { account: String }

    impl PaymentProcessor for CreditCardProcessor {
        fn process_payment(&self, amount: f64) -> String {
            format!("Processing ${:.2} via {} credit card", amount, self.card_type)
        }
        fn get_fee(&self) -> f64 { 0.029 }
    }

    impl PaymentProcessor for PayPalProcessor {
        fn process_payment(&self, amount: f64) -> String {
            format!("Processing ${:.2} via PayPal ({})", amount, self.email)
        }
        fn get_fee(&self) -> f64 { 0.034 }
    }

    impl PaymentProcessor for BankTransferProcessor {
        fn process_payment(&self, amount: f64) -> String {
            format!("Processing ${:.2} via bank transfer ({})", amount, self.account)
        }
        fn get_fee(&self) -> f64 { 0.005 }
    }

    // Context class that uses strategy
    struct PaymentContext {
        processor: Box<dyn PaymentProcessor>,
    }

    impl PaymentContext {
        fn new(processor: Box<dyn PaymentProcessor>) -> Self {
            PaymentContext { processor }
        }

        fn execute_payment(&self, amount: f64) -> String {
            let fee = amount * self.processor.get_fee();
            format!("{}\nProcessing fee: ${:.2}", 
                   self.processor.process_payment(amount), fee)
        }
    }

    // Using different strategies
    let payment_methods: Vec<Box<dyn PaymentProcessor>> = vec![
        Box::new(CreditCardProcessor { card_type: "Visa".to_string() }),
        Box::new(PayPalProcessor { email: "user@example.com".to_string() }),
        Box::new(BankTransferProcessor { account: "12345678".to_string() }),
    ];

    for processor in payment_methods {
        let context = PaymentContext::new(processor);
        println!("{}\n", context.execute_payment(100.0));
    }

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
    
    // New OOP-style examples
    example_encapsulation();
    example_polymorphism_enhanced();
    example_composition_over_inheritance();
    example_advanced_oop_patterns();

    println!("✅ All examples completed!");
    println!("📖 Next: Read Chapter 10.3 (Lifetimes) or run examples in ../lifetimes/");
}
