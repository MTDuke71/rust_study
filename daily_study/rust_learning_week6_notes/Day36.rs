// Day 36 - Module Basics Standalone Example
// Run with: rustc Day36.rs && ./Day36 (Linux/Mac) or .\Day36.exe (Windows)

// Complete example demonstrating Rust module system fundamentals

mod library {
    // Public module with books
    pub mod books {
        #[derive(Debug, Clone)]
        pub struct Book {
            pub title: String,
            pub author: String,
            pages: usize, // Private field
        }

        impl Book {
            pub fn new(title: String, author: String, pages: usize) -> Self {
                Self {
                    title,
                    author,
                    pages,
                }
            }

            pub fn page_count(&self) -> usize {
                self.pages
            }

            pub fn summary(&self) -> String {
                format!("{} by {} ({} pages)", self.title, self.author, self.pages)
            }
        }
    }

    // Private module for internal operations
    mod catalog {
        use super::books::Book;

        pub(super) fn sort_by_title(books: &mut [Book]) {
            books.sort_by(|a, b| a.title.cmp(&b.title));
        }

        pub(super) fn sort_by_author(books: &mut [Book]) {
            books.sort_by(|a, b| a.author.cmp(&b.author));
        }

        pub(super) fn sort_by_pages(books: &mut [Book]) {
            books.sort_by(|a, b| a.page_count().cmp(&b.page_count()));
        }
    }

    // Public statistics module
    pub mod stats {
        use super::books::Book;

        pub fn total_pages(books: &[Book]) -> usize {
            books.iter().map(|b| b.page_count()).sum()
        }

        pub fn average_pages(books: &[Book]) -> f64 {
            if books.is_empty() {
                0.0
            } else {
                total_pages(books) as f64 / books.len() as f64
            }
        }

        #[allow(dead_code)]
        pub(crate) fn count_by_author(books: &[Book], author: &str) -> usize {
            books.iter().filter(|b| b.author == author).count()
        }
    }

    // Public Library interface
    pub struct Library {
        books: Vec<books::Book>,
        name: String,
    }

    impl Library {
        pub fn new(name: String) -> Self {
            Self {
                books: Vec::new(),
                name,
            }
        }

        pub fn add_book(&mut self, book: books::Book) {
            self.books.push(book);
        }

        pub fn sort_by_title(&mut self) {
            catalog::sort_by_title(&mut self.books);
        }

        pub fn sort_by_author(&mut self) {
            catalog::sort_by_author(&mut self.books);
        }

        pub fn sort_by_pages(&mut self) {
            catalog::sort_by_pages(&mut self.books);
        }

        pub fn list_books(&self) {
            println!("\n{} Library:", self.name);
            println!("{}", "=".repeat(50));
            for (i, book) in self.books.iter().enumerate() {
                println!("{}. {}", i + 1, book.summary());
            }
        }

        pub fn statistics(&self) {
            println!("\n{} Library Statistics:", self.name);
            println!("{}", "=".repeat(50));
            println!("Total books: {}", self.books.len());
            println!("Total pages: {}", stats::total_pages(&self.books));
            println!("Average pages: {:.1}", stats::average_pages(&self.books));
        }

        pub fn book_count(&self) -> usize {
            self.books.len()
        }
    }
}

// Clean public API using re-exports
pub use library::{books::Book, Library};

// Demonstration of visibility rules
mod visibility_demo {
    pub mod public_mod {
        pub fn public_fn() {
            println!("✓ public_fn: Accessible everywhere");
        }

        pub(crate) fn crate_fn() {
            println!("✓ crate_fn: Accessible within crate");
        }

        pub(super) fn super_fn() {
            println!("✓ super_fn: Accessible in parent module");
        }

        fn private_fn() {
            println!("✓ private_fn: Only in this module");
        }

        pub fn call_all_internal() {
            public_fn();
            crate_fn();
            super_fn();
            private_fn();
        }
    }

    pub fn demonstrate_visibility() {
        println!("\n=== Visibility Demonstration ===");
        println!("Calling from parent module:");
        public_mod::public_fn();
        public_mod::crate_fn();
        public_mod::super_fn();
        // public_mod::private_fn(); // ❌ Would fail - private

        println!("\nCalling all from within module:");
        public_mod::call_all_internal();
    }
}

// Module path demonstration
mod paths {
    pub mod math {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        pub fn multiply(a: i32, b: i32) -> i32 {
            a * b
        }
    }

    pub mod utils {
        pub fn format_result(operation: &str, result: i32) -> String {
            format!("{} = {}", operation, result)
        }
    }
}

fn demonstrate_paths() {
    println!("\n=== Module Path Demonstration ===");

    // Absolute path
    let result1 = crate::paths::math::add(5, 3);
    println!("Absolute path: {}", result1);

    // Using `use` for convenience
    use paths::math::{add, multiply};
    use paths::utils::format_result;

    let result2 = add(10, 20);
    let result3 = multiply(4, 5);

    println!("With use: {}", format_result("10 + 20", result2));
    println!("With use: {}", format_result("4 * 5", result3));
}

// Mission 10 preview: Basic Union-Find structure
mod union_find_preview {
    pub struct UnionFind {
        parent: Vec<usize>,
        rank: Vec<usize>,
    }

    impl UnionFind {
        pub fn new(size: usize) -> Self {
            Self {
                parent: (0..size).collect(),
                rank: vec![0; size],
            }
        }

        pub fn find(&mut self, x: usize) -> usize {
            if self.parent[x] != x {
                self.parent[x] = self.find(self.parent[x]); // Path compression
            }
            self.parent[x]
        }

        pub fn union(&mut self, x: usize, y: usize) -> bool {
            let root_x = self.find(x);
            let root_y = self.find(y);

            if root_x == root_y {
                return false; // Already connected
            }

            // Union by rank
            if self.rank[root_x] < self.rank[root_y] {
                self.parent[root_x] = root_y;
            } else if self.rank[root_x] > self.rank[root_y] {
                self.parent[root_y] = root_x;
            } else {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }

            true
        }

        pub fn connected(&mut self, x: usize, y: usize) -> bool {
            self.find(x) == self.find(y)
        }
    }
}

fn demonstrate_union_find() {
    println!("\n=== Union-Find Preview (Mission 10) ===");

    let mut uf = union_find_preview::UnionFind::new(10);

    println!("Connecting elements:");
    uf.union(0, 1);
    uf.union(2, 3);
    uf.union(4, 5);
    println!("✓ Connected: 0-1, 2-3, 4-5");

    uf.union(0, 2);
    println!("✓ Connected: 0-2 (merges {{0,1}} and {{2,3}})");

    println!("\nConnectivity tests:");
    println!("0 and 3 connected: {}", uf.connected(0, 3)); // true
    println!("0 and 4 connected: {}", uf.connected(0, 4)); // false
    println!("4 and 5 connected: {}", uf.connected(4, 5)); // true
}

fn main() {
    println!("╔════════════════════════════════════════════════════╗");
    println!("║     Day 36: Module Basics - Complete Demo         ║");
    println!("╚════════════════════════════════════════════════════╝");

    // Part 1: Library System with Modules
    println!("\n=== Part 1: Library Management System ===");

    let mut lib = Library::new("Central".to_string());

    // Add books
    lib.add_book(Book::new(
        "The Rust Programming Language".to_string(),
        "Steve Klabnik".to_string(),
        550,
    ));
    lib.add_book(Book::new(
        "Programming Rust".to_string(),
        "Jim Blandy".to_string(),
        622,
    ));
    lib.add_book(Book::new(
        "Rust in Action".to_string(),
        "Tim McNamara".to_string(),
        454,
    ));
    lib.add_book(Book::new(
        "Rust for Rustaceans".to_string(),
        "Jon Gjengset".to_string(),
        282,
    ));

    lib.list_books();
    lib.statistics();

    // Sort demonstrations
    println!("\n--- Sorted by Title ---");
    lib.sort_by_title();
    lib.list_books();

    println!("\n--- Sorted by Author ---");
    lib.sort_by_author();
    lib.list_books();

    println!("\n--- Sorted by Pages ---");
    lib.sort_by_pages();
    lib.list_books();

    // Part 2: Visibility Rules
    visibility_demo::demonstrate_visibility();

    // Part 3: Module Paths
    demonstrate_paths();

    // Part 4: Union-Find Preview
    demonstrate_union_find();

    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║            Module System Concepts Covered          ║");
    println!("╠════════════════════════════════════════════════════╣");
    println!("║ ✓ Module declaration with mod                     ║");
    println!("║ ✓ Visibility: pub, pub(crate), pub(super)         ║");
    println!("║ ✓ Nested modules for organization                 ║");
    println!("║ ✓ Private implementation details                  ║");
    println!("║ ✓ Re-exports for clean APIs                       ║");
    println!("║ ✓ Module paths (absolute and relative)            ║");
    println!("║ ✓ Encapsulation and abstraction                   ║");
    println!("╚════════════════════════════════════════════════════╝");
}
