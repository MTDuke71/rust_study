//! # Chapter 10.1: Generic Data Types
//!
//! Demonstrates generic programming concepts from The Rust Book.
//! Shows how to write reusable code that works with multiple types.
//!
//! Run with: `cargo run`

fn example1_basic_generics() {
    println!("🦀 Example 1: Basic Generic Functions");
    println!("=====================================");

    // Generic function that works with any comparable type
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let string_list = vec!["hello", "world", "rust", "programming"];
    let result = largest(&string_list);
    println!("The largest string is {}", result);

    println!();
}

fn example2_generic_structs() {
    println!("🏗️  Example 2: Generic Structs");
    println!("===============================");

    // Generic struct with single type parameter
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    // Specific implementation for f32
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    println!("Integer point: x={}, y={}", integer_point.x(), integer_point.y);
    println!("Float point: x={}, y={}", float_point.x(), float_point.y);
    println!("Distance from origin: {}", float_point.distance_from_origin());

    // Generic struct with multiple type parameters
    struct Point2<T, U> {
        x: T,
        y: U,
    }

    let mixed_point = Point2 { x: 5, y: 4.0 };
    println!("Mixed point: x={}, y={}", mixed_point.x, mixed_point.y);

    println!();
}

fn example3_generic_enums() {
    println!("📦 Example 3: Generic Enums");
    println!("============================");

    // Custom generic enum example (avoiding std library conflicts)
    #[derive(Debug)]
    enum Maybe<T> {
        Just(T),
        Nothing,
    }

    #[derive(Debug)]
    enum MyResult<T, E> {
        Ok(T),
        Err(E),
    }

    // Using our custom Maybe enum
    let some_number = Maybe::Just(5);
    let some_string = Maybe::Just("a string");
    let absent_number: Maybe<i32> = Maybe::Nothing;

    println!("Some number: {:?}", some_number);
    println!("Some string: {:?}", some_string);
    println!("Absent number: {:?}", absent_number);

    // Using our custom Result enum
    let success: MyResult<i32, &str> = MyResult::Ok(42);
    let failure: MyResult<i32, &str> = MyResult::Err("Something went wrong");

    println!("Success: {:?}", success);
    println!("Failure: {:?}", failure);

    // Using standard library Option and Result
    let std_option: Option<i32> = Some(42);
    let std_result: Result<i32, &str> = Ok(42);

    println!("Standard Option: {:?}", std_option);
    println!("Standard Result: {:?}", std_result);

    println!();
}

fn example4_generic_methods() {
    println!("🔧 Example 4: Generic Methods and Impl Blocks");
    println!("==============================================");

    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("Mixed point: x={}, y={}", p3.x, p3.y);

    println!();
}

fn example5_performance_considerations() {
    println!("⚡ Example 5: Performance and Monomorphization");
    println!("===============================================");

    // Generic function that gets specialized at compile time
    fn identity<T>(x: T) -> T {
        println!("Identity function called with type: {}", std::any::type_name::<T>());
        x
    }

    // At compile time, Rust generates specific versions:
    // fn identity_i32(x: i32) -> i32 { x }
    // fn identity_f64(x: f64) -> f64 { x }
    // fn identity_string(x: String) -> String { x }
    // etc.

    let int_result = identity(5i32);
    let float_result = identity(42.0f64);
    let string_result = identity(String::from("hello"));

    println!("Integer identity: {} (type: {})", int_result, std::any::type_name::<i32>());
    println!("Float identity: {} (type: {}) - Note: displays as 42 but is actually f64!", 
             float_result, std::any::type_name::<f64>());
    println!("String identity: {} (type: {})", string_result, std::any::type_name::<String>());

    // Show that 42.0 and 42 are different types even though they display the same
    println!("\n🔍 Float Display vs Type:");
    println!("42.0 displays as: {}", 42.0);
    println!("42.0 debug format: {:?}", 42.0);
    println!("42.0 type: {}", std::any::type_name::<f64>());
    
    // This is zero-cost abstraction - no runtime overhead!
    println!("\nNo runtime cost for generics!");

    println!();
}

fn example6_trait_bounds_basics() {
    println!("🎯 Example 6: Basic Trait Bounds");
    println!("=================================");

    use std::fmt::Display;

    // Function that requires T to implement Display
    fn print_generic<T: Display>(item: T) {
        println!("Item: {}", item);
    }

    // Function with multiple trait bounds
    fn compare_and_print<T: PartialOrd + Display>(a: T, b: T) {
        if a > b {
            println!("{} is greater than {}", a, b);
        } else {
            println!("{} is not greater than {}", a, b);
        }
    }

    print_generic(42);
    print_generic("Hello, Rust!");
    print_generic(3.14);

    compare_and_print(10, 5);
    compare_and_print('z', 'a');

    println!();
}

fn example7_user_input_and_generics() {
    println!("👤 Example 7: User Input and Generic Type Resolution");
    println!("===================================================");

    use std::io;

    // Generic function that works with any numeric type
    fn process_number<T: std::str::FromStr + std::fmt::Display + std::fmt::Debug>(value: T) -> T {
        println!("Processing number: {:?}", value);
        value
    }

    // Function to get user input as a specific type
    fn get_user_input<T: std::str::FromStr>() -> Result<T, T::Err> {
        let mut input = String::new();
        println!("Enter a number: ");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().parse::<T>()
    }

    // Example 1: Working with i32 (integer)
    println!("=== Working with i32 ===");
    match get_user_input::<i32>() {
        Ok(num) => {
            let processed = process_number(num);
            println!("You entered integer: {}", processed);
        }
        Err(e) => println!("Invalid integer input: {:?}", e),
    }

    // Example 2: Working with f64 (float)
    println!("\n=== Working with f64 ===");
    match get_user_input::<f64>() {
        Ok(num) => {
            let processed = process_number(num);
            println!("You entered float: {}", processed);
        }
        Err(e) => println!("Invalid float input: {:?}", e),
    }

    // Example 3: The key insight - each call creates a different specialized function
    println!("\n=== Type Specialization at Compile Time ===");
    
    // These create DIFFERENT specialized versions of process_number:
    let int_val: i32 = 42;
    let float_val: f64 = 42.0;
    
    let processed_int = process_number(int_val);
    let processed_float = process_number(float_val);
    
    println!("Processed integer: {}", processed_int);
    println!("Processed float: {}", processed_float);
    
    // ❌ This would NOT work - you can't mix types in the same generic call:
    // process_number(42, 42.0);  // Compile error!
    
    // ✅ But you CAN convert between types:
    let converted_float = process_number(42 as f64);
    println!("Converted and processed: {}", converted_float);

    println!();
}

fn example8_parsing_and_conversion() {
    println!("🔄 Example 8: Parsing and Type Conversion");
    println!("==========================================");

    use std::io;

    // Generic function for parsing user input
    fn parse_input<T: std::str::FromStr + std::fmt::Display>(prompt: &str) -> Result<T, String> {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        input.trim().parse::<T>().map_err(|_| {
            format!("Failed to parse '{}' as the expected type", input.trim())
        })
    }

    // Function that can work with different numeric types
    fn calculate_square<T: std::ops::Mul<Output = T> + Copy>(value: T) -> T {
        value * value
    }

    println!("=== Integer Calculation ===");
    match parse_input::<i32>("Enter an integer:") {
        Ok(num) => {
            let square = calculate_square(num);
            println!("The square of {} is {}", num, square);
        }
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== Float Calculation ===");
    match parse_input::<f64>("Enter a float:") {
        Ok(num) => {
            let square = calculate_square(num);
            println!("The square of {} is {}", num, square);
        }
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== Manual Type Conversion ===");
    // Sometimes you need to convert between types
    match parse_input::<i32>("Enter a number:") {
        Ok(int_val) => {
            // Convert to float for calculation
            let float_val = int_val as f64;
            let square_float = calculate_square(float_val);
            println!("Integer {} converted to float {} has square {}", 
                     int_val, float_val, square_float);
        }
        Err(e) => println!("Error: {}", e),
    }

    println!();
}

fn main() {
    println!("📚 Chapter 10.1: Generic Data Types\n");

    example1_basic_generics();
    example2_generic_structs();
    example3_generic_enums();
    example4_generic_methods();
    example5_performance_considerations();
    example6_trait_bounds_basics();
    example7_user_input_and_generics();
    example8_parsing_and_conversion();

    println!("✅ All examples completed!");
    println!("📖 Next: Read Chapter 10.2 (Traits) or run examples in ../traits/");
}