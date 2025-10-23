// Mission1_tut Step 2: Generic Stack Implementation
// Transform concrete i32 stack into flexible Stack<T> that works with any type

fn main() {
    println!("=== Mission1_tut Step 2: Generic Stack Implementation ===\n");

    // 1. Understanding Generic Types
    println!("1. Understanding Generic Types:");
    explain_generics_concept();

    // 2. Generic Stack Implementation
    println!("\n2. Generic Stack Implementation:");
    demonstrate_generic_stack();

    // 3. Multiple Type Examples
    println!("\n3. Using Stack with Different Types:");
    demonstrate_multiple_types();

    // 4. Type Parameter Constraints
    println!("\n4. Understanding Type Parameters:");
    explain_type_parameters();

    self_assessment();
}

/// Generic stack implementation that works with any type T
#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    /// Create a new empty stack for any type T
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    /// Add an item to the top of the stack
    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    /// Remove and return the top item from the stack
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Look at the top item without removing it
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    /// Check if the stack is empty
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get the number of items in the stack
    fn len(&self) -> usize {
        self.items.len()
    }
}

fn explain_generics_concept() {
    println!("   What are Generic Types?");
    println!("   • Generic types allow code reuse across different data types");
    println!("   • Stack<T> means 'Stack of some type T'");
    println!("   • T is a 'type parameter' - a placeholder for any actual type");
    println!("   • One implementation works for i32, String, bool, custom structs, etc.");

    println!("\n   Benefits of Generic Programming:");
    println!("   ✅ Code Reuse - Write once, use with many types");
    println!("   ✅ Type Safety - Compiler ensures type consistency");
    println!("   ✅ Performance - No runtime overhead (monomorphization)");
    println!("   ✅ Expressiveness - Clear intent about what types are supported");

    println!("\n   Syntax Explanation:");
    println!("   • struct Stack<T>     - Define generic struct with type parameter T");
    println!("   • impl<T> Stack<T>    - Implement methods for any type T");
    println!("   • Stack<i32>          - Concrete instantiation with specific type");
}

fn demonstrate_generic_stack() {
    println!("   Creating Stack<i32> (integer stack):");
    let mut int_stack: Stack<i32> = Stack::new();

    int_stack.push(42);
    int_stack.push(100);
    int_stack.push(7);

    println!("   Stack contents: {:?}", int_stack);
    println!("   Length: {}", int_stack.len());

    if let Some(top) = int_stack.peek() {
        println!("   Top item: {}", top);
    }

    while let Some(item) = int_stack.pop() {
        println!("   Popped: {}", item);
    }

    println!("   Is empty: {}", int_stack.is_empty());
}

fn demonstrate_multiple_types() {
    // Stack of strings
    println!("   String Stack Example:");
    let mut string_stack: Stack<String> = Stack::new();
    string_stack.push("Hello".to_string());
    string_stack.push("World".to_string());
    string_stack.push("Rust".to_string());

    println!("   String stack: {:?}", string_stack);

    // Stack of booleans
    println!("\n   Boolean Stack Example:");
    let mut bool_stack: Stack<bool> = Stack::new();
    bool_stack.push(true);
    bool_stack.push(false);
    bool_stack.push(true);

    println!("   Boolean stack: {:?}", bool_stack);

    // Stack of custom struct
    println!("\n   Custom Type Stack Example:");

    #[derive(Debug)]
    struct Point {
        #[allow(dead_code)]
        x: f64,
        #[allow(dead_code)]
        y: f64,
    }

    let mut point_stack: Stack<Point> = Stack::new();
    point_stack.push(Point { x: 1.0, y: 2.0 });
    point_stack.push(Point { x: 3.0, y: 4.0 });

    println!("   Point stack: {:?}", point_stack);

    // Stack of Vec (stack of vectors!)
    println!("\n   Nested Generic Example - Stack<Vec<i32>>:");
    let mut vector_stack: Stack<Vec<i32>> = Stack::new();
    vector_stack.push(vec![1, 2, 3]);
    vector_stack.push(vec![4, 5]);
    vector_stack.push(vec![6, 7, 8, 9]);

    println!("   Vector stack: {:?}", vector_stack);
}

fn explain_type_parameters() {
    println!("   How Type Parameters Work:");
    println!("   • T is a placeholder that gets replaced at compile time");
    println!("   • Stack<i32> creates a version where T = i32");
    println!("   • Stack<String> creates a version where T = String");
    println!("   • Each type gets its own optimized machine code");

    println!("\n   Monomorphization Process:");
    println!("   1. Compiler sees Stack<i32> and Stack<String>");
    println!("   2. Generates separate implementations for each type");
    println!("   3. No runtime type checking - everything resolved at compile time");
    println!("   4. Zero-cost abstraction - generic code is as fast as hand-written code");

    println!("\n   Type Safety Benefits:");
    println!("   ✅ Can't push String into Stack<i32> - compile error");
    println!("   ✅ Can't mix types accidentally - compiler catches mistakes");
    println!("   ✅ Method calls are type-checked - peek() returns Option<&T>");
    println!("   ✅ Memory layout optimized per type - no boxing or indirection");

    println!("\n   Common Generic Patterns in Rust:");
    println!("   • Vec<T>           - Dynamic array of T");
    println!("   • Option<T>        - Maybe contains a T");
    println!("   • Result<T, E>     - Success T or Error E");
    println!("   • HashMap<K, V>    - Key-value pairs");
    println!("   • Box<T>           - Heap-allocated T");
}

#[allow(dead_code)]
fn demonstrate_compilation_differences() {
    println!("\n   What the Compiler Generates:");
    println!("   Original generic code:");
    println!("   ```rust");
    println!("   struct Stack<T> {{ items: Vec<T> }}");
    println!("   impl<T> Stack<T> {{ fn push(&mut self, item: T) {{ ... }} }}");
    println!("   ```");

    println!("\n   After monomorphization (conceptual):");
    println!("   ```rust");
    println!("   // For Stack<i32>");
    println!("   struct StackI32 {{ items: Vec<i32> }}");
    println!("   impl StackI32 {{ fn push(&mut self, item: i32) {{ ... }} }}");
    println!();
    println!("   // For Stack<String>");
    println!("   struct StackString {{ items: Vec<String> }}");
    println!("   impl StackString {{ fn push(&mut self, item: String) {{ ... }} }}");
    println!("   ```");

    println!("   This is why Rust generics have zero runtime cost!");
}

fn self_assessment() {
    println!("\n=== Self-Assessment Questions ===");
    println!("After completing Step 2, you should understand:");
    println!("1. What does Stack<T> mean and why is it useful?");
    println!("2. How do you create stacks for different types?");
    println!("3. What is monomorphization and why does it matter?");
    println!("4. Can you explain why Stack<i32> and Stack<String> are different types?");
    println!("5. What happens at compile time vs runtime with generics?");

    println!("\n=== Hands-On Challenge ===");
    println!("Try creating these stacks and experiment:");
    println!("• Stack<char> - for individual characters");
    println!("• Stack<Option<i32>> - stack of optional integers");
    println!("• Stack<(f64, f64)> - stack of coordinate tuples");

    println!("\n=== Next Step Preview ===");
    println!("Step 3 will explore Rust ownership patterns in stack operations:");
    println!("• When does push() take ownership of items?");
    println!("• Why does pop() return Option<T> instead of T?");
    println!("• How does borrowing work with peek() and peek_mut()?");
    println!("Run: cargo run --example step3_ownership_patterns");
}

// Bonus: Demonstrate advanced generic usage
#[allow(dead_code)]
fn bonus_advanced_generics() {
    println!("\n=== Bonus: Advanced Generic Concepts ===");
    println!("   Advanced patterns you'll see later:");
    println!("   • Trait bounds: Stack<T> where T: Clone");
    println!("   • Associated types: Iterator<Item = T>");
    println!("   • Higher-ranked trait bounds: for<'a> ...");
    println!("   • Generic methods: fn map<U, F>(...) where F: FnOnce(T) -> U");
}

// Generic methods for Stack<T>
impl<T> Stack<T> {
    /// Generic method that works with any closure
    #[allow(dead_code)]
    fn _map_top<U, F>(&self, f: F) -> Option<U>
    where
        F: FnOnce(&T) -> U,
    {
        self.peek().map(f)
    }
}
