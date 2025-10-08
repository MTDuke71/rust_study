//! # Chapter 8.1: Vectors - Storing Lists of Values
//!
//! Complete examples from The Rust Book demonstrating vector operations.
//!
//! Run with: `cargo run`

fn example1_creating_vectors() {
    println!("📦 Example 1: Creating Vectors");
    println!("==============================");

    // Creating an empty vector with type annotation
    let v1: Vec<i32> = Vec::new();
    println!("Empty vector: {:?}", v1);

    // Creating a vector with initial values using vec! macro
    let v2 = vec![1, 2, 3];
    println!("Vector with values: {:?}", v2);

    // Type inference from initial values
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    println!("Vector built with push: {:?}", v3);

    // Creating with capacity
    let v4: Vec<i32> = Vec::with_capacity(10);
    println!("Vector with capacity 10: capacity={}, len={}", v4.capacity(), v4.len());

    println!();
}

fn example2_updating_vectors() {
    println!("✏️  Example 2: Updating Vectors");
    println!("==============================");

    let mut v = Vec::new();
    println!("Initial: {:?}", v);

    // Adding elements
    v.push(5);
    println!("After push(5): {:?}", v);

    v.push(6);
    v.push(7);
    v.push(8);
    println!("After multiple pushes: {:?}", v);

    // Removing elements
    let last = v.pop();
    println!("Popped: {:?}, remaining: {:?}", last, v);

    // Removing specific index
    let removed = v.remove(1);  // Removes element at index 1
    println!("Removed index 1 (value {}): {:?}", removed, v);

    // Inserting at specific position
    v.insert(1, 100);
    println!("After insert(1, 100): {:?}", v);

    // Clearing all elements
    let len_before = v.len();
    v.clear();
    println!("After clear: {:?}, was length {}, now {}", v, len_before, v.len());

    println!();
}

fn example3_reading_elements() {
    println!("🔍 Example 3: Reading Elements");
    println!("==============================");

    let v = vec![1, 2, 3, 4, 5];

    // Method 1: Indexing (panics if out of bounds)
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // Method 2: get method (returns Option)
    match v.get(2) {
        Some(third) => println!("The third element (via get) is {}", third),
        None => println!("There is no third element."),
    }

    // Attempting to access out of bounds
    let index = 10;
    match v.get(index) {
        Some(element) => println!("Element at {}: {}", index, element),
        None => println!("No element at index {} (safe handling)", index),
    }

    // This would panic:
    // let does_not_exist = &v[100];  // ❌ Panics!

    println!();
}

fn example4_ownership_and_borrowing() {
    println!("🔗 Example 4: Ownership and Borrowing");
    println!("=====================================");

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];  // Immutable borrow
    println!("The first element is: {}", first);

    // This would cause a compile error:
    // v.push(6);  // ❌ Can't mutably borrow while immutable borrow exists
    // println!("The first element is: {}", first);

    // Drop the immutable borrow by letting it go out of scope
    // (In practice, the borrow ends when `first` is last used)
    // drop(first); // Not needed for references - they don't own data

    v.push(6);  // Now this works
    println!("After push: {:?}", v);

    // Demonstrating why the borrowing rule matters
    let v2 = vec![1, 2, 3];
    let first_ref = &v2[0];
    println!("First element before potential reallocation: {}", first_ref);
    // If we could push here, v2 might reallocate, invalidating first_ref

    println!();
}

fn example5_iterating() {
    println!("🔄 Example 5: Iterating Over Vectors");
    println!("====================================");

    let v = vec![100, 32, 57];

    // Immutable iteration
    print!("Immutable iteration: ");
    for i in &v {
        print!("{} ", i);
    }
    println!();

    // Mutable iteration
    let mut v2 = vec![100, 32, 57];
    print!("Before mutation: {:?} ", v2);
    for i in &mut v2 {
        *i += 50;  // Dereference to modify the value
    }
    println!("→ After adding 50: {:?}", v2);

    // Consuming iteration (takes ownership)
    let v3 = vec![1, 2, 3];
    print!("Consuming iteration: ");
    for i in v3 {  // v3 is moved here
        print!("{} ", i);
    }
    println!();
    // v3 is no longer valid here

    // Using enumerate for index + value
    let v4 = vec!["a", "b", "c"];
    println!("With enumerate:");
    for (index, value) in v4.iter().enumerate() {
        println!("  Index {}: {}", index, value);
    }

    println!();
}

fn example6_multiple_types_with_enum() {
    println!("🎨 Example 6: Storing Multiple Types with Enum");
    println!("==============================================");

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("Spreadsheet row:");
    for (i, cell) in row.iter().enumerate() {
        match cell {
            SpreadsheetCell::Int(value) => println!("  Cell {}: Integer = {}", i, value),
            SpreadsheetCell::Float(value) => println!("  Cell {}: Float = {}", i, value),
            SpreadsheetCell::Text(value) => println!("  Cell {}: Text = '{}'", i, value),
        }
    }

    println!();
}

fn example7_vector_methods() {
    println!("🛠️  Example 7: Useful Vector Methods");
    println!("===================================");

    let mut v = vec![1, 2, 3, 4, 5];

    // Length and capacity
    println!("Length: {}, Capacity: {}", v.len(), v.capacity());

    // Is empty
    println!("Is empty: {}", v.is_empty());

    // Contains
    println!("Contains 3: {}", v.contains(&3));
    println!("Contains 10: {}", v.contains(&10));

    // First and last
    println!("First: {:?}", v.first());
    println!("Last: {:?}", v.last());

    // Slicing
    let slice = &v[1..4];
    println!("Slice [1..4]: {:?}", slice);

    // Splitting
    let (left, right) = v.split_at(3);
    println!("Split at 3: {:?} | {:?}", left, right);

    // Reverse
    v.reverse();
    println!("After reverse: {:?}", v);

    // Sort
    let mut v2 = vec![5, 2, 8, 1, 9];
    v2.sort();
    println!("Sorted: {:?}", v2);

    // Dedup (remove consecutive duplicates)
    let mut v3 = vec![1, 2, 2, 3, 3, 3, 4];
    v3.dedup();
    println!("After dedup: {:?}", v3);

    println!();
}

fn example8_practical_use_case() {
    println!("💼 Example 8: Practical Use Case - Grade Average");
    println!("================================================");

    let grades = vec![95, 87, 92, 88, 100, 78];

    // Calculate average
    let sum: i32 = grades.iter().sum();
    let avg = sum as f64 / grades.len() as f64;

    println!("Grades: {:?}", grades);
    println!("Sum: {}", sum);
    println!("Average: {:.2}", avg);

    // Find min and max
    let min = grades.iter().min().unwrap();
    let max = grades.iter().max().unwrap();
    println!("Min: {}, Max: {}", min, max);

    // Filter passing grades (>= 70)
    let passing: Vec<_> = grades.iter().filter(|&&grade| grade >= 70).collect();
    println!("Passing grades: {:?}", passing);

    // Count grades in ranges
    let a_count = grades.iter().filter(|&&g| g >= 90).count();
    let b_count = grades.iter().filter(|&&g| g >= 80 && g < 90).count();
    let c_count = grades.iter().filter(|&&g| g >= 70 && g < 80).count();

    println!("Grade distribution:");
    println!("  A's (90-100): {}", a_count);
    println!("  B's (80-89): {}", b_count);
    println!("  C's (70-79): {}", c_count);

    println!();
}

fn main() {
    println!("=== Chapter 8.1: Vectors ===\n");

    example1_creating_vectors();
    example2_updating_vectors();
    example3_reading_elements();
    example4_ownership_and_borrowing();
    example5_iterating();
    example6_multiple_types_with_enum();
    example7_vector_methods();
    example8_practical_use_case();

    println!("✅ All vector examples completed!");
}
