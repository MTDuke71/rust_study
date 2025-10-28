# Day 36 - Module Basics

**Learning Focus**: Understanding Rust's module system fundamentals - `mod`, `pub`, visibility rules

**Date**: November 2, 2025 (Sunday)

**Mission Alignment**: Mission 10 Setup & Planning (Union-Find Disjoint Sets)

---

## 🎯 Learning Objectives

By the end of this day, you should understand:
- How to declare and organize modules with `mod`
- Visibility rules: `pub`, `pub(crate)`, `pub(super)`, `pub(in path)`
- Module paths and the `use` statement
- Inline vs file-based modules
- Module privacy and encapsulation
- Best practices for module organization

---

## 📚 Core Concepts

### **1. Module Declaration**

Modules are Rust's way of organizing code into logical namespaces and controlling visibility.

```rust
// Inline module declaration
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    // Private by default
    fn internal_helper() {
        println!("This is private to the math module");
    }
}

// Using the module
fn main() {
    let result = math::add(5, 3);
    println!("5 + 3 = {}", result);
    
    // This would error: `internal_helper` is private
    // math::internal_helper();
}
```

**Key Points**:
- Everything is private by default
- Use `pub` to make items public
- Modules create namespaces

---

### **2. Visibility Rules**

Rust has sophisticated visibility controls:

```rust
mod outer {
    pub mod public_nested {
        pub fn public_function() {
            println!("I'm accessible from anywhere!");
        }
        
        pub(crate) fn crate_visible() {
            println!("I'm visible within this crate only");
        }
        
        pub(super) fn parent_visible() {
            println!("I'm visible to my parent module");
        }
        
        pub(in crate::outer) fn custom_path_visible() {
            println!("I'm visible to crate::outer and its descendants");
        }
        
        fn private_function() {
            println!("I'm private to this module");
        }
    }
    
    mod private_nested {
        // This module is private to `outer`
        pub fn function() {
            println!("Module is private, but this fn is pub within it");
        }
    }
    
    pub fn access_nested() {
        // Can access pub(super) items
        public_nested::parent_visible();
        
        // Can access items from private nested module
        private_nested::function();
    }
}

fn main() {
    outer::public_nested::public_function(); // ✅ Works
    outer::public_nested::crate_visible();   // ✅ Works (same crate)
    outer::access_nested();                  // ✅ Works
    
    // These would error:
    // outer::public_nested::parent_visible();  // ❌ Not visible here
    // outer::private_nested::function();       // ❌ Module is private
}
```

**Visibility Levels**:
- `pub` - Public to all
- `pub(crate)` - Visible within the current crate
- `pub(super)` - Visible to the parent module
- `pub(in path)` - Visible to a specific path
- (no modifier) - Private to current module

---

### **3. Module Paths and `use`**

```rust
mod shapes {
    pub mod circle {
        pub struct Circle {
            pub radius: f64,
        }
        
        impl Circle {
            pub fn new(radius: f64) -> Self {
                Self { radius }
            }
            
            pub fn area(&self) -> f64 {
                std::f64::consts::PI * self.radius * self.radius
            }
        }
    }
    
    pub mod rectangle {
        pub struct Rectangle {
            pub width: f64,
            pub height: f64,
        }
        
        impl Rectangle {
            pub fn new(width: f64, height: f64) -> Self {
                Self { width, height }
            }
            
            pub fn area(&self) -> f64 {
                self.width * self.height
            }
        }
    }
}

// Absolute path
fn example_absolute() {
    let c = crate::shapes::circle::Circle::new(5.0);
    println!("Circle area: {}", c.area());
}

// Using `use` for convenience
use shapes::circle::Circle;
use shapes::rectangle::Rectangle;

fn example_with_use() {
    let c = Circle::new(5.0);
    let r = Rectangle::new(4.0, 3.0);
    
    println!("Circle area: {}", c.area());
    println!("Rectangle area: {}", r.area());
}

// Re-exporting
pub use shapes::circle::Circle as Circ;

fn main() {
    example_absolute();
    example_with_use();
    
    // Can now use re-export
    let c = Circ::new(7.0);
    println!("Re-exported circle: {}", c.area());
}
```

**Path Types**:
- Absolute: `crate::module::item`
- Relative: `super::sibling_module::item`
- `self`: Current module
- `super`: Parent module
- `crate`: Crate root

---

### **4. File-Based Modules**

In real projects, modules live in separate files:

**Project Structure**:
```
src/
├── main.rs
├── utils.rs
└── math/
    ├── mod.rs
    ├── addition.rs
    └── multiplication.rs
```

**src/main.rs**:
```rust
// Declare file-based modules
mod utils;
mod math;

use math::{addition, multiplication};

fn main() {
    println!("5 + 3 = {}", addition::add(5, 3));
    println!("5 * 3 = {}", multiplication::multiply(5, 3));
    utils::print_separator();
}
```

**src/utils.rs**:
```rust
pub fn print_separator() {
    println!("====================");
}
```

**src/math/mod.rs**:
```rust
// Declare submodules
pub mod addition;
pub mod multiplication;

// Re-export commonly used items
pub use addition::add;
pub use multiplication::multiply;
```

**src/math/addition.rs**:
```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

**src/math/multiplication.rs**:
```rust
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
```

---

## 🔧 Implementation Patterns

### **Pattern 1: Nested Modules for Organization**

```rust
mod data_structures {
    pub mod linear {
        pub struct Stack<T> {
            items: Vec<T>,
        }
        
        impl<T> Stack<T> {
            pub fn new() -> Self {
                Self { items: Vec::new() }
            }
            
            pub fn push(&mut self, item: T) {
                self.items.push(item);
            }
            
            pub fn pop(&mut self) -> Option<T> {
                self.items.pop()
            }
        }
        
        pub struct Queue<T> {
            items: Vec<T>,
        }
        
        impl<T> Queue<T> {
            pub fn new() -> Self {
                Self { items: Vec::new() }
            }
            
            pub fn enqueue(&mut self, item: T) {
                self.items.push(item);
            }
            
            pub fn dequeue(&mut self) -> Option<T> {
                if self.items.is_empty() {
                    None
                } else {
                    Some(self.items.remove(0))
                }
            }
        }
    }
    
    pub mod tree {
        pub struct BinaryTree<T> {
            root: Option<Box<Node<T>>>,
        }
        
        struct Node<T> {
            value: T,
            left: Option<Box<Node<T>>>,
            right: Option<Box<Node<T>>>,
        }
        
        impl<T> BinaryTree<T> {
            pub fn new() -> Self {
                Self { root: None }
            }
            
            // Tree operations would go here
        }
    }
}

// Clean API usage
use data_structures::linear::{Stack, Queue};

fn main() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    println!("Popped: {:?}", stack.pop());
    
    let mut queue = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    println!("Dequeued: {:?}", queue.dequeue());
}
```

---

### **Pattern 2: Private Implementation Details**

```rust
mod database {
    // Public interface
    pub struct Database {
        connection: Connection, // Private field
    }
    
    // Private implementation
    struct Connection {
        url: String,
        connected: bool,
    }
    
    impl Connection {
        fn new(url: String) -> Self {
            Self {
                url,
                connected: false,
            }
        }
        
        fn connect(&mut self) -> Result<(), String> {
            if self.url.is_empty() {
                return Err("Invalid URL".to_string());
            }
            self.connected = true;
            Ok(())
        }
    }
    
    impl Database {
        pub fn new(url: String) -> Result<Self, String> {
            let mut connection = Connection::new(url);
            connection.connect()?;
            Ok(Self { connection })
        }
        
        pub fn query(&self, sql: &str) -> Result<Vec<String>, String> {
            if !self.connection.connected {
                return Err("Not connected".to_string());
            }
            
            // Simulate query
            Ok(vec![format!("Result for: {}", sql)])
        }
    }
}

fn main() {
    match database::Database::new("localhost:5432".to_string()) {
        Ok(db) => {
            match db.query("SELECT * FROM users") {
                Ok(results) => {
                    for result in results {
                        println!("{}", result);
                    }
                }
                Err(e) => eprintln!("Query error: {}", e),
            }
        }
        Err(e) => eprintln!("Connection error: {}", e),
    }
    
    // Can't access Connection directly - it's private
    // let conn = database::Connection::new("url".to_string()); // ❌ Error
}
```

---

### **Pattern 3: Module Re-exports for Clean APIs**

```rust
mod internal {
    pub mod parsers {
        pub fn parse_json(input: &str) -> Result<String, String> {
            // Simplified parser
            if input.is_empty() {
                Err("Empty input".to_string())
            } else {
                Ok(format!("Parsed: {}", input))
            }
        }
        
        pub fn parse_xml(input: &str) -> Result<String, String> {
            // Simplified parser
            if input.is_empty() {
                Err("Empty input".to_string())
            } else {
                Ok(format!("Parsed: {}", input))
            }
        }
    }
    
    pub mod validators {
        pub fn validate_email(email: &str) -> bool {
            email.contains('@') && email.contains('.')
        }
        
        pub fn validate_url(url: &str) -> bool {
            url.starts_with("http://") || url.starts_with("https://")
        }
    }
}

// Public API - re-export selected items
pub use internal::parsers::{parse_json, parse_xml};
pub use internal::validators::{validate_email, validate_url};

fn main() {
    // Users can access these directly
    match parse_json(r#"{"key": "value"}"#) {
        Ok(result) => println!("{}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    let email = "user@example.com";
    if validate_email(email) {
        println!("{} is valid", email);
    }
}
```

---

## 🎮 Practical Applications

### **Application 1: Union-Find Module Structure (Mission 10)**

```rust
// Mission 10: Union-Find Disjoint Sets module organization
mod union_find {
    /// A disjoint-set data structure with path compression and union by rank
    pub struct UnionFind {
        parent: Vec<usize>,
        rank: Vec<usize>,
        count: usize, // Number of disjoint sets
    }
    
    impl UnionFind {
        /// Creates a new Union-Find structure with n elements
        pub fn new(size: usize) -> Self {
            Self {
                parent: (0..size).collect(),
                rank: vec![0; size],
                count: size,
            }
        }
        
        /// Finds the representative of the set containing x (with path compression)
        pub fn find(&mut self, mut x: usize) -> Result<usize, String> {
            if x >= self.parent.len() {
                return Err(format!("Element {} out of bounds", x));
            }
            
            // Path compression: make every node point directly to root
            let root = self.find_root(x);
            while x != root {
                let next = self.parent[x];
                self.parent[x] = root;
                x = next;
            }
            
            Ok(root)
        }
        
        /// Unites the sets containing x and y (with union by rank)
        pub fn union(&mut self, x: usize, y: usize) -> Result<bool, String> {
            let root_x = self.find(x)?;
            let root_y = self.find(y)?;
            
            if root_x == root_y {
                return Ok(false); // Already in same set
            }
            
            // Union by rank: attach smaller tree under larger tree
            if self.rank[root_x] < self.rank[root_y] {
                self.parent[root_x] = root_y;
            } else if self.rank[root_x] > self.rank[root_y] {
                self.parent[root_y] = root_x;
            } else {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
            
            self.count -= 1;
            Ok(true)
        }
        
        /// Returns true if x and y are in the same set
        pub fn connected(&mut self, x: usize, y: usize) -> Result<bool, String> {
            Ok(self.find(x)? == self.find(y)?)
        }
        
        /// Returns the number of disjoint sets
        pub fn count(&self) -> usize {
            self.count
        }
        
        // Private helper methods
        fn find_root(&self, mut x: usize) -> usize {
            while x != self.parent[x] {
                x = self.parent[x];
            }
            x
        }
    }
}

// Public API
pub use union_find::UnionFind;

fn main() {
    let mut uf = UnionFind::new(10);
    
    println!("Initial sets: {}", uf.count());
    
    // Union some elements
    uf.union(0, 1).unwrap();
    uf.union(2, 3).unwrap();
    uf.union(0, 2).unwrap(); // Joins {0,1} and {2,3}
    
    println!("After unions: {} sets", uf.count());
    println!("0 and 3 connected: {}", uf.connected(0, 3).unwrap());
    println!("0 and 4 connected: {}", uf.connected(0, 4).unwrap());
}
```

---

### **Application 2: Parser Module with Visibility Control**

```rust
mod parser {
    // Internal tokenizer - not exposed
    mod tokenizer {
        #[derive(Debug, Clone)]
        pub(super) enum Token {
            Number(i32),
            Plus,
            Minus,
            Star,
            Slash,
            LeftParen,
            RightParen,
        }
        
        pub(super) fn tokenize(input: &str) -> Result<Vec<Token>, String> {
            let mut tokens = Vec::new();
            let mut chars = input.chars().peekable();
            
            while let Some(&ch) = chars.peek() {
                match ch {
                    ' ' | '\t' | '\n' => {
                        chars.next();
                    }
                    '0'..='9' => {
                        let mut num_str = String::new();
                        while let Some(&digit) = chars.peek() {
                            if digit.is_ascii_digit() {
                                num_str.push(digit);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        if let Ok(num) = num_str.parse() {
                            tokens.push(Token::Number(num));
                        } else {
                            return Err(format!("Invalid number: {}", num_str));
                        }
                    }
                    '+' => {
                        tokens.push(Token::Plus);
                        chars.next();
                    }
                    '-' => {
                        tokens.push(Token::Minus);
                        chars.next();
                    }
                    '*' => {
                        tokens.push(Token::Star);
                        chars.next();
                    }
                    '/' => {
                        tokens.push(Token::Slash);
                        chars.next();
                    }
                    '(' => {
                        tokens.push(Token::LeftParen);
                        chars.next();
                    }
                    ')' => {
                        tokens.push(Token::RightParen);
                        chars.next();
                    }
                    _ => return Err(format!("Unexpected character: {}", ch)),
                }
            }
            
            Ok(tokens)
        }
    }
    
    // Public parser interface
    pub struct Parser {
        expression: String,
    }
    
    impl Parser {
        pub fn new(expression: String) -> Self {
            Self { expression }
        }
        
        pub fn parse(&self) -> Result<i32, String> {
            let tokens = tokenizer::tokenize(&self.expression)?;
            self.evaluate(&tokens)
        }
        
        fn evaluate(&self, tokens: &[tokenizer::Token]) -> Result<i32, String> {
            // Simplified evaluation - just sums numbers for demo
            let mut result = 0;
            let mut operation = tokenizer::Token::Plus;
            
            for token in tokens {
                match token {
                    tokenizer::Token::Number(n) => {
                        match operation {
                            tokenizer::Token::Plus => result += n,
                            tokenizer::Token::Minus => result -= n,
                            _ => {}
                        }
                    }
                    tokenizer::Token::Plus => operation = tokenizer::Token::Plus,
                    tokenizer::Token::Minus => operation = tokenizer::Token::Minus,
                    _ => {}
                }
            }
            
            Ok(result)
        }
    }
}

pub use parser::Parser;

fn main() {
    let expressions = vec![
        "1 + 2 + 3",
        "10 - 5 + 2",
        "100 + 50 - 25",
    ];
    
    for expr in expressions {
        let parser = Parser::new(expr.to_string());
        match parser.parse() {
            Ok(result) => println!("{} = {}", expr, result),
            Err(e) => eprintln!("Error parsing '{}': {}", expr, e),
        }
    }
    
    // Can't access tokenizer::Token directly - it's pub(super)
    // let token = parser::tokenizer::Token::Plus; // ❌ Error
}
```

---

## 🧪 Exercises

### Exercise 1: Create a Module Hierarchy
Create a `geometry` module with nested `two_d` and `three_d` submodules containing appropriate shapes.

<details>
<summary>Solution</summary>

```rust
mod geometry {
    pub mod two_d {
        pub struct Point {
            pub x: f64,
            pub y: f64,
        }
        
        pub struct Circle {
            pub center: Point,
            pub radius: f64,
        }
        
        impl Circle {
            pub fn new(x: f64, y: f64, radius: f64) -> Self {
                Self {
                    center: Point { x, y },
                    radius,
                }
            }
            
            pub fn area(&self) -> f64 {
                std::f64::consts::PI * self.radius * self.radius
            }
        }
    }
    
    pub mod three_d {
        pub struct Point {
            pub x: f64,
            pub y: f64,
            pub z: f64,
        }
        
        pub struct Sphere {
            pub center: Point,
            pub radius: f64,
        }
        
        impl Sphere {
            pub fn new(x: f64, y: f64, z: f64, radius: f64) -> Self {
                Self {
                    center: Point { x, y, z },
                    radius,
                }
            }
            
            pub fn volume(&self) -> f64 {
                (4.0 / 3.0) * std::f64::consts::PI * self.radius.powi(3)
            }
        }
    }
}

fn main() {
    let circle = geometry::two_d::Circle::new(0.0, 0.0, 5.0);
    println!("Circle area: {:.2}", circle.area());
    
    let sphere = geometry::three_d::Sphere::new(0.0, 0.0, 0.0, 5.0);
    println!("Sphere volume: {:.2}", sphere.volume());
}
```
</details>

---

### Exercise 2: Implement Visibility Controls
Create a `cache` module with public API but private implementation details.

<details>
<summary>Solution</summary>

```rust
mod cache {
    use std::collections::HashMap;
    
    // Private storage
    struct Storage {
        data: HashMap<String, String>,
        hits: usize,
        misses: usize,
    }
    
    impl Storage {
        fn new() -> Self {
            Self {
                data: HashMap::new(),
                hits: 0,
                misses: 0,
            }
        }
        
        fn get(&mut self, key: &str) -> Option<&String> {
            if let Some(value) = self.data.get(key) {
                self.hits += 1;
                Some(value)
            } else {
                self.misses += 1;
                None
            }
        }
        
        fn set(&mut self, key: String, value: String) {
            self.data.insert(key, value);
        }
    }
    
    // Public cache interface
    pub struct Cache {
        storage: Storage,
    }
    
    impl Cache {
        pub fn new() -> Self {
            Self {
                storage: Storage::new(),
            }
        }
        
        pub fn get(&mut self, key: &str) -> Option<&String> {
            self.storage.get(key)
        }
        
        pub fn set(&mut self, key: String, value: String) {
            self.storage.set(key, value);
        }
        
        pub fn stats(&self) -> (usize, usize) {
            (self.storage.hits, self.storage.misses)
        }
    }
}

pub use cache::Cache;

fn main() {
    let mut cache = Cache::new();
    
    cache.set("key1".to_string(), "value1".to_string());
    cache.set("key2".to_string(), "value2".to_string());
    
    cache.get("key1");
    cache.get("key1");
    cache.get("key3"); // Miss
    
    let (hits, misses) = cache.stats();
    println!("Cache hits: {}, misses: {}", hits, misses);
}
```
</details>

---

### Exercise 3: Use Re-exports
Create a `prelude` module that re-exports commonly used items.

<details>
<summary>Solution</summary>

```rust
mod data {
    pub struct User {
        pub name: String,
        pub email: String,
    }
    
    pub struct Product {
        pub id: u32,
        pub name: String,
        pub price: f64,
    }
}

mod operations {
    pub fn validate_email(email: &str) -> bool {
        email.contains('@')
    }
    
    pub fn format_price(price: f64) -> String {
        format!("${:.2}", price)
    }
}

// Prelude module for convenient imports
pub mod prelude {
    pub use super::data::{User, Product};
    pub use super::operations::{validate_email, format_price};
}

// Now users can import everything with one line
use prelude::*;

fn main() {
    let user = User {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };
    
    let product = Product {
        id: 1,
        name: "Widget".to_string(),
        price: 29.99,
    };
    
    println!("User: {}", user.name);
    println!("Email valid: {}", validate_email(&user.email));
    println!("Product: {} - {}", product.name, format_price(product.price));
}
```
</details>

---

## 🎯 Key Takeaways

1. **Privacy by default** - Everything is private unless marked `pub`
2. **Granular visibility** - Use `pub(crate)`, `pub(super)`, `pub(in path)` for fine control
3. **Module organization** - Use nested modules for logical code organization
4. **Clean APIs** - Re-export public items, hide implementation details
5. **File structure** - Large modules should use separate files
6. **Path types** - Master absolute (`crate::`), relative (`super::`), and `use` statements

---

## 📝 Complete Runnable Example

```rust
// Complete example demonstrating all Day 36 concepts
mod library {
    // Public module with re-exports
    pub mod books {
        #[derive(Debug)]
        pub struct Book {
            pub title: String,
            pub author: String,
            pages: usize, // Private field
        }
        
        impl Book {
            pub fn new(title: String, author: String, pages: usize) -> Self {
                Self { title, author, pages }
            }
            
            pub fn page_count(&self) -> usize {
                self.pages
            }
        }
    }
    
    // Private module for internal operations
    mod catalog {
        use super::books::Book;
        
        pub(super) fn sort_by_title(books: &mut [Book]) {
            books.sort_by(|a, b| a.title.cmp(&b.title));
        }
    }
    
    // Public interface
    pub struct Library {
        books: Vec<books::Book>,
    }
    
    impl Library {
        pub fn new() -> Self {
            Self { books: Vec::new() }
        }
        
        pub fn add_book(&mut self, book: books::Book) {
            self.books.push(book);
        }
        
        pub fn sort(&mut self) {
            catalog::sort_by_title(&mut self.books);
        }
        
        pub fn list_books(&self) {
            for book in &self.books {
                println!("{} by {} ({} pages)", 
                    book.title, book.author, book.page_count());
            }
        }
    }
}

// Clean public API
pub use library::{Library, books::Book};

fn main() {
    let mut lib = Library::new();
    
    lib.add_book(Book::new("The Rust Book".to_string(), "Steve Klabnik".to_string(), 550));
    lib.add_book(Book::new("Programming Rust".to_string(), "Jim Blandy".to_string(), 622));
    lib.add_book(Book::new("Rust in Action".to_string(), "Tim McNamara".to_string(), 454));
    
    println!("Books before sorting:");
    lib.list_books();
    
    lib.sort();
    
    println!("\nBooks after sorting:");
    lib.list_books();
}
```

**Expected Output**:
```
Books before sorting:
The Rust Book by Steve Klabnik (550 pages)
Programming Rust by Jim Blandy (622 pages)
Rust in Action by Tim McNamara (454 pages)

Books after sorting:
Programming Rust by Jim Blandy (622 pages)
Rust in Action by Tim McNamara (454 pages)
The Rust Book by Steve Klabnik (550 pages)
```

---

## 🔗 Related Concepts

- **[[Crate Organization]]** - Day 37: lib.rs vs main.rs, module trees
- **[[Cargo Features]]** - Day 38: Conditional compilation
- **[[API Design]]** - Best practices for public interfaces
- **[[Encapsulation]]** - Hiding implementation details
- **[[Namespaces]]** - Avoiding name collisions

---

## 🔗 Navigation

**Previous**: [[../rust_learning_week5_notes/Day35|Day 35 - Error Handling Practice]] | **Next**: [[Day37|Day 37 - Crate Organization]]

**Week Overview**: [[README|Week 6 Overview]]

**Mission**: [[../../missions/Mission10/README|Mission 10 - Union-Find Disjoint Sets]]

**Zettelkasten**: [[../../zettelkasten/daily-study/Day36|Day36 (Zettelkasten)]]

---

*Tags: #modules #visibility #organization #privacy #api-design #rust-module-system*

*Links: [[../../zettelkasten/zettel-index|Zettelkasten Index]] | [[README|Week 6 Overview]] | [[Day37|Day 37]]*
