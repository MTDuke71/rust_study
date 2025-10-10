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

    // 🧠 Memory Layout Explanation
    println!("\n🧠 Memory Layout Analysis:");
    println!("==========================");
    
    println!("Vec<SpreadsheetCell> works because:");
    println!("1. The vector stores the SAME TYPE: SpreadsheetCell enum values");
    println!("2. Each enum has the SAME SIZE in memory (largest variant + discriminant)");
    
    use std::mem;
    println!("\nSize analysis:");
    println!("  i32 size: {} bytes", mem::size_of::<i32>());
    println!("  f64 size: {} bytes", mem::size_of::<f64>());
    println!("  String size: {} bytes", mem::size_of::<String>());
    println!("  SpreadsheetCell size: {} bytes", mem::size_of::<SpreadsheetCell>());
    
    println!("\n📦 What's actually stored in the vector:");
    println!("  Not pointers, but the actual enum values!");
    println!("  Each enum value = discriminant tag + largest possible data");
    
    // Memory layout visualization
    println!("\n🎯 Memory Layout Visualization:");
    println!("Vector memory: [SpreadsheetCell][SpreadsheetCell][SpreadsheetCell]");
    println!("Each cell:     [tag|   data (largest variant size)    ]");
    println!("Cell 0:        [ 0 |   3 (i32) + padding              ]");
    println!("Cell 1:        [ 2 |   String struct (ptr+cap+len)    ]");
    println!("Cell 2:        [ 1 |   10.12 (f64) + padding          ]");
    
    // The vector is homogeneous - same type everywhere
    println!("\n✅ This is why it works:");
    println!("• Vector is homogeneous: Vec<SpreadsheetCell>");
    println!("• All elements are the same type: SpreadsheetCell");
    println!("• Each element has the same size in memory");
    println!("• Rust can calculate offsets: base_ptr + (index × size_of::<SpreadsheetCell>())");

    println!();
}

fn example6a_memory_efficiency_analysis() {
    println!("💾 Example 6a: Memory Efficiency Analysis");
    println!("=========================================");
    
    #[derive(Debug)]
    #[allow(dead_code)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    use std::mem;
    
    println!("📊 Memory Usage Analysis:");
    println!("========================");
    
    let actual_sizes = (
        mem::size_of::<i32>(),
        mem::size_of::<f64>(),
        mem::size_of::<String>(),
    );
    let enum_size = mem::size_of::<SpreadsheetCell>();
    
    println!("Actual data sizes:");
    println!("  i32: {} bytes", actual_sizes.0);
    println!("  f64: {} bytes", actual_sizes.1);
    println!("  String: {} bytes", actual_sizes.2);
    println!("  SpreadsheetCell enum: {} bytes", enum_size);
    
    // Calculate waste for each type
    let int_waste = enum_size - actual_sizes.0;
    let float_waste = enum_size - actual_sizes.1;
    let text_waste = enum_size - actual_sizes.2;
    
    println!("\n💸 Memory waste per element:");
    println!("  Int variant: {} bytes wasted ({:.1}% waste)", 
             int_waste, (int_waste as f64 / enum_size as f64) * 100.0);
    println!("  Float variant: {} bytes wasted ({:.1}% waste)", 
             float_waste, (float_waste as f64 / enum_size as f64) * 100.0);
    println!("  Text variant: {} bytes wasted ({:.1}% waste)", 
             text_waste, (text_waste as f64 / enum_size as f64) * 100.0);
    
    // Demonstrate with actual data
    let mixed_row = vec![
        SpreadsheetCell::Int(42),
        SpreadsheetCell::Int(100),
        SpreadsheetCell::Float(3.14159),
        SpreadsheetCell::Text(String::from("Hello")),
        SpreadsheetCell::Int(7),
    ];
    
    let total_enum_memory = mixed_row.len() * enum_size;
    let actual_memory_needed = 
        3 * actual_sizes.0 +  // 3 ints
        1 * actual_sizes.1 +  // 1 float  
        1 * actual_sizes.2;   // 1 string
    
    println!("\n📈 Real example with {} elements:", mixed_row.len());
    println!("  Enum approach: {} bytes", total_enum_memory);
    println!("  Actual data: {} bytes", actual_memory_needed);
    println!("  Waste: {} bytes ({:.1}% overhead)", 
             total_enum_memory - actual_memory_needed,
             ((total_enum_memory - actual_memory_needed) as f64 / actual_memory_needed as f64) * 100.0);
    
    println!("\n🎯 When is this waste acceptable?");
    println!("✅ Small datasets (< 1000 elements)");
    println!("✅ Simplicity more important than memory");
    println!("✅ Need to maintain order across different types");
    println!("✅ Frequent mixed-type operations");
    println!("✅ Unknown type distribution at compile time");
    
    println!();
}

fn example6b_why_enums_are_needed() {
    println!("❌ Example 6b: Why We Need Enums (What Doesn't Work)");
    println!("====================================================");
    
    println!("This code would NOT compile:");
    println!("```rust");
    println!("// ❌ ERROR: Vec can only store ONE type");
    println!("let mixed = vec![");
    println!("    42,                    // i32");
    println!("    3.14,                  // f64"); 
    println!("    String::from(\"text\"),   // String");
    println!("]; // ERROR: expected integer, found floating-point number");
    println!("```");
    
    println!("\n🤔 Why doesn't this work?");
    println!("• Vectors need to know the exact size of each element");
    println!("• i32 = 4 bytes, f64 = 8 bytes, String = 24 bytes");
    println!("• Different sizes = can't calculate memory offsets");
    println!("• No way to know which type is at each index");
    
    println!("\n✅ Solutions:");
    println!("1. Enum (like SpreadsheetCell) - same type, different variants");
    println!("2. Trait objects: Vec<Box<dyn Display>> - same size pointers");
    println!("3. Separate vectors: Vec<i32>, Vec<f64>, Vec<String>");
    
    // Show trait object approach
    use std::fmt::Display;
    
    let trait_objects: Vec<Box<dyn Display>> = vec![
        Box::new(42),
        Box::new(3.14),
        Box::new(String::from("text")),
    ];
    
    println!("\n📦 Trait object approach (Vec<Box<dyn Display>>):");
    for (i, item) in trait_objects.iter().enumerate() {
        println!("  Item {}: {}", i, item);
    }
    
    println!("\n🧠 Trait objects work because:");
    println!("• Vec stores Box<dyn Display> - all same size (pointer + vtable)");
    println!("• Each Box points to heap-allocated data");
    println!("• Runtime dispatch through virtual function table");
    
    use std::mem;
    println!("\nSize comparison:");
    println!("  Box<dyn Display> size: {} bytes (fat pointer)", mem::size_of::<Box<dyn Display>>());
    println!("  Box<i32> size: {} bytes (thin pointer)", mem::size_of::<Box<i32>>());
    
    println!();
}

fn example6c_alternative_approaches() {
    println!("🔄 Example 6c: Alternative Approaches to Mixed Data");
    println!("==================================================");
    
    // Approach 1: Separate vectors (most memory efficient)
    println!("📋 Approach 1: Separate Vectors");
    println!("===============================");
    
    #[derive(Debug)]
    struct SeparatedSpreadsheet {
        ints: Vec<(usize, i32)>,      // (row_index, value)  
        floats: Vec<(usize, f64)>,    // (row_index, value)
        texts: Vec<(usize, String)>,  // (row_index, value)
    }
    
    let separated = SeparatedSpreadsheet {
        ints: vec![(0, 42), (1, 100), (4, 7)],
        floats: vec![(2, 3.14159)],
        texts: vec![(3, String::from("Hello"))],
    };
    
    println!("Memory efficient storage:");
    println!("  Ints: {:?}", separated.ints);
    println!("  Floats: {:?}", separated.floats);
    println!("  Texts: {:?}", separated.texts);
    
    // Reconstruction requires sorting/merging
    println!("  Reconstructed row: [42, 100, 3.14159, \"Hello\", 7]");
    
    use std::mem;
    let separated_memory = 
        separated.ints.len() * mem::size_of::<(usize, i32)>() +
        separated.floats.len() * mem::size_of::<(usize, f64)>() +
        separated.texts.len() * mem::size_of::<(usize, String)>();
    
    println!("  Memory usage: {} bytes (no waste!)", separated_memory);
    
    // Approach 2: Tagged Union with Box (heap allocation)
    println!("\n📦 Approach 2: Boxed Values with Type Tags");
    println!("==========================================");
    
    #[derive(Debug)]
    #[allow(dead_code)]
    enum CellType { Int, Float, Text }
    
    #[derive(Debug)]
    #[allow(dead_code)]
    struct TypedCell {
        cell_type: CellType,
        data: Box<dyn std::fmt::Debug>,
    }
    
    // This approach uses heap allocation but uniform size
    println!("Pros: Uniform size, no padding waste");
    println!("Cons: Heap allocation overhead, vtable indirection");
    println!("Size per element: {} bytes (pointer + vtable + discriminant)", 
             mem::size_of::<TypedCell>());
    
    // Approach 3: Hybrid with threshold
    println!("\n⚖️  Approach 3: Hybrid Approach");
    println!("===============================");
    
    #[derive(Debug)]
    #[allow(dead_code)]
    struct HybridSpreadsheet {
        // Small values stored inline
        small_values: Vec<(usize, SmallValue)>,
        // Large values stored separately  
        large_texts: Vec<(usize, String)>,
    }
    
    #[derive(Debug)]
    #[allow(dead_code)]
    enum SmallValue {
        Int(i32),
        Float(f64),
        SmallText([u8; 16]), // Fixed-size strings
    }
    
    println!("Strategy: Store small values together, large values separately");
    println!("SmallValue size: {} bytes", mem::size_of::<SmallValue>());
    println!("Threshold: Strings > 16 bytes go to separate vector");
    
    // Approach 4: Column-oriented storage
    println!("\n📊 Approach 4: Column-Oriented (Database Style)");
    println!("===============================================");
    
    #[allow(dead_code)]
    struct ColumnStore {
        // Each column is homogeneous
        int_columns: Vec<Vec<Option<i32>>>,
        float_columns: Vec<Vec<Option<f64>>>,
        text_columns: Vec<Vec<Option<String>>>,
    }
    
    println!("Strategy: Group by type, use Option<T> for missing values");
    println!("Pros: Cache-friendly, no memory waste, vectorizable");
    println!("Cons: More complex access patterns");
    
    // Use case analysis
    println!("\n🎯 When to Use Each Approach:");
    println!("=============================");
    
    println!("🔹 Enum Approach (Vec<SpreadsheetCell>):");
    println!("  ✅ Simple code, maintain order, small datasets");
    println!("  ✅ Mixed operations, prototyping, readability");
    println!("  ❌ Memory-constrained environments");
    
    println!("\n🔹 Separate Vectors:");
    println!("  ✅ Memory efficiency, type-specific operations");
    println!("  ✅ Large datasets, performance critical");
    println!("  ❌ Complex ordering, frequent mixed access");
    
    println!("\n🔹 Boxed/Trait Objects:");
    println!("  ✅ Dynamic typing, plugin systems");
    println!("  ✅ Uniform interface, polymorphism");
    println!("  ❌ Performance overhead, heap fragmentation");
    
    println!("\n🔹 Column-Oriented:");
    println!("  ✅ Analytics, SIMD operations, compression");
    println!("  ✅ Sparse data, database-like operations");
    println!("  ❌ Row-wise access patterns");
    
    println!();
}

fn example6d_real_world_use_cases() {
    println!("🌍 Example 6d: Real-World Use Cases");
    println!("===================================");
    
    // Use Case 1: Configuration file with mixed types
    println!("📄 Use Case 1: Configuration Parser");
    println!("===================================");
    
    #[derive(Debug)]
    enum ConfigValue {
        Integer(i64),
        Boolean(bool),
        Text(String),
        Float(f64),
    }
    
    // Config file: server_port=8080, debug_mode=true, server_name="MyApp", timeout=30.5
    let config: Vec<(&str, ConfigValue)> = vec![
        ("server_port", ConfigValue::Integer(8080)),
        ("debug_mode", ConfigValue::Boolean(true)),
        ("server_name", ConfigValue::Text("MyApp".to_string())),
        ("timeout", ConfigValue::Float(30.5)),
    ];
    
    println!("✅ Enum approach perfect here because:");
    println!("  • Small dataset (dozens of config values)");
    println!("  • Mixed types naturally occur together");
    println!("  • Need to preserve order");
    println!("  • Simple parsing and validation");
    
    for (key, value) in &config {
        match value {
            ConfigValue::Integer(i) => println!("  {}: {} (integer)", key, i),
            ConfigValue::Boolean(b) => println!("  {}: {} (boolean)", key, b),
            ConfigValue::Text(s) => println!("  {}: '{}' (text)", key, s),
            ConfigValue::Float(f) => println!("  {}: {} (float)", key, f),
        }
    }
    
    // Use Case 2: Analytics data with millions of records
    println!("\n📊 Use Case 2: Analytics Data (Large Scale)");
    println!("===========================================");
    
    #[allow(dead_code)]
    struct AnalyticsData {
        // Separate vectors for memory efficiency
        user_ids: Vec<u64>,           // 8 bytes each
        timestamps: Vec<u64>,         // 8 bytes each  
        event_types: Vec<u8>,         // 1 byte each (enum as u8)
        values: Vec<f32>,             // 4 bytes each
        // String data stored separately and indexed
        string_pool: Vec<String>,
        string_indices: Vec<Option<usize>>,
    }
    
    println!("✅ Separate vectors perfect here because:");
    println!("  • Large dataset (millions of records)");
    println!("  • Memory efficiency critical");
    println!("  • Column-wise operations common");
    println!("  • SIMD/vectorization opportunities");
    
    // Use Case 3: JSON-like data structure
    println!("\n🔗 Use Case 3: JSON-like Nested Data");
    println!("====================================");
    
    #[derive(Debug)]
    #[allow(dead_code)]
    enum JsonValue {
        Null,
        Boolean(bool),
        Number(f64),
        String(String),
        Array(Vec<JsonValue>),       // Recursive!
        Object(Vec<(String, JsonValue)>),
    }
    
    let _json_data = JsonValue::Object(vec![
        ("name".to_string(), JsonValue::String("Alice".to_string())),
        ("age".to_string(), JsonValue::Number(30.0)),
        ("active".to_string(), JsonValue::Boolean(true)),
        ("scores".to_string(), JsonValue::Array(vec![
            JsonValue::Number(95.5),
            JsonValue::Number(87.2),
            JsonValue::Number(92.0),
        ])),
    ]);
    
    println!("✅ Enum approach essential here because:");
    println!("  • Nested, recursive structure");
    println!("  • Unknown type distribution");
    println!("  • Need uniform interface");
    println!("  • Dynamic typing requirements");
    
    // Use Case 4: Game entity component system 
    println!("\n🎮 Use Case 4: Game Components (Performance Critical)");
    println!("=====================================================");
    
    // Bad approach: Mixed component storage
    // enum Component { Position(f32,f32), Health(i32), Sprite(String) }
    // Vec<Component> - cache unfriendly!
    
    // Good approach: Separate component arrays
    #[allow(dead_code)]
    struct EntityComponentSystem {
        positions: Vec<(f32, f32)>,    // Hot data together
        healths: Vec<i32>,             // Cache-friendly
        sprites: Vec<String>,          // Cold data separate
        // Entity ID maps to indices in each array
        entity_to_position: std::collections::HashMap<u32, usize>,
        entity_to_health: std::collections::HashMap<u32, usize>,
        entity_to_sprite: std::collections::HashMap<u32, usize>,
    }
    
    println!("✅ Separate storage critical here because:");
    println!("  • Performance is paramount");
    println!("  • Systems operate on one component type");
    println!("  • Cache locality essential");
    println!("  • SIMD processing opportunities");
    
    // Decision framework
    println!("\n🎯 Decision Framework:");
    println!("======================");
    println!("Choose ENUM approach when:");
    println!("  📝 Mixed types naturally occur together");
    println!("  🔢 Small to medium datasets (< 100K elements)");
    println!("  🔄 Order preservation important");
    println!("  🛠️  Development speed over memory efficiency");
    println!("  🌐 Dynamic/unknown type distribution");
    
    println!("\nChoose SEPARATE VECTORS when:");
    println!("  🚀 Performance is critical");
    println!("  💾 Memory efficiency important");
    println!("  📊 Large datasets (> 100K elements)");
    println!("  🔍 Type-specific operations common");
    println!("  ⚡ SIMD/vectorization opportunities");
        
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
    
    // Converting Option to plain values
    println!("\n🔓 Unwrapping Option Values:");
    println!("============================");
    
    // Method 1: unwrap() - panics if None
    let first_val = v.first().unwrap();  // Returns &i32
    println!("Method 1 (unwrap): first = {}", first_val);
    
    // Method 2: unwrap_or() - provides default if None
    let empty_vec: Vec<i32> = vec![];
    let first_or_default = empty_vec.first().unwrap_or(&0);
    println!("Method 2 (unwrap_or): first of empty vec = {}", first_or_default);
    
    // Method 3: unwrap_or_else() - compute default lazily
    let first_or_computed = empty_vec.first().unwrap_or_else(|| &-1);
    println!("Method 3 (unwrap_or_else): {}", first_or_computed);
    
    // Method 4: match expression (safest, most explicit)
    match v.first() {
        Some(value) => println!("Method 4 (match): first = {}", value),
        None => println!("Method 4 (match): vector is empty"),
    }
    
    // Method 5: if let (concise when you only care about Some case)
    if let Some(first) = v.first() {
        println!("Method 5 (if let): first = {}", first);
    }
    
    // Method 6: expect() - like unwrap but with custom panic message
    let last_val = v.last().expect("Vector should not be empty!");
    println!("Method 6 (expect): last = {}", last_val);
    
    // Method 7: Dereferencing to get owned value (when you need i32, not &i32)
    let first_owned: i32 = *v.first().unwrap();  // Dereference &i32 to i32
    println!("Method 7 (dereference): first_owned = {} (type: i32)", first_owned);
    
    // Method 8: copied() or cloned() - convert Option<&T> to Option<T>
    let first_copied: Option<i32> = v.first().copied();  // Option<&i32> -> Option<i32>
    println!("Method 8 (copied): {:?} can be unwrapped to {}", first_copied, first_copied.unwrap());
    
    println!("\n⚠️  When to use each method:");
    println!("• unwrap(): When you're CERTAIN the Option is Some (or want to panic)");
    println!("• unwrap_or(default): When you want a fallback value");
    println!("• expect(msg): Like unwrap, but with helpful error message");
    println!("• match: When you need to handle both Some and None cases");
    println!("• if let: When you only care about the Some case");
    println!("• copied()/cloned(): When you need Option<T> instead of Option<&T>");

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
    example6a_memory_efficiency_analysis();
    example6b_why_enums_are_needed();
    example6c_alternative_approaches();
    example6d_real_world_use_cases();
    example7_vector_methods();
    example8_practical_use_case();

    println!("✅ All vector examples completed!");
}
