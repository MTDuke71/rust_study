//! # Chapter 4: References and Borrowing
//! 
//! **Learning Objective**: Understand Rust's borrowing system and reference types
//! 
//! **Key Concepts**:
//! - References (&T) for borrowing without taking ownership
//! - Mutable references (&mut T) for borrowing with modification rights
//! - Borrowing rules: multiple immutable OR one mutable reference
//! - Dangling reference prevention
//! - Reference lifetime basics

fn main() {
    println!("🦀 Chapter 4: References and Borrowing");
    println!("======================================\n");
    
    // Example 1: Basic references
    example_basic_references();
    
    // Example 2: Mutable references
    example_mutable_references();
    
    // Example 3: Reference rules
    example_reference_rules();
    
    // Example 4: Functions with references
    example_functions_with_references();
    
    // Example 5: Printing and ownership
    example_printing_ownership();
}

/// Example 1: Basic References
/// Shows how to create and use immutable references
fn example_basic_references() {
    println!("📚 Example 1: Basic References");
    println!("==============================");
    
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // Pass reference, not ownership
    
    println!("The length of '{}' is {}.", s1, len);  // s1 still valid!
    
    // Multiple immutable references are allowed
    let r1 = &s1;
    let r2 = &s1;
    println!("References r1: '{}', r2: '{}'", r1, r2);
    
    println!();
}

/// Example 2: Mutable References
/// Shows how to modify data through mutable references
fn example_mutable_references() {
    println!("✏️  Example 2: Mutable References");
    println!("=================================");
    
    let mut s = String::from("hello");
    change_string(&mut s);  // Pass mutable reference
    
    println!("Modified string: '{}'", s);
    println!();
}

/// Example 3: Reference Rules
/// Demonstrates Rust's borrowing rules
fn example_reference_rules() {
    println!("📋 Example 3: Reference Rules");
    println!("=============================");
    
    let mut s = String::from("hello");
    
    // Rule 1: Multiple immutable references OK
    {
        let r1 = &s;
        let r2 = &s;
        println!("Immutable refs: '{}' and '{}'", r1, r2);
    } // r1 and r2 go out of scope here
    
    // Rule 2: One mutable reference OR multiple immutable references
    {
        let r3 = &mut s;
        r3.push_str(", world!");
        println!("Mutable ref result: '{}'", r3);
    } // r3 goes out of scope here
    
    println!("Final string: '{}'", s);
    println!();
}

/// Example 4: Functions with References
/// Shows common patterns for functions using references
fn example_functions_with_references() {
    println!("🔧 Example 4: Functions with References");
    println!("=======================================");
    
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    // Read-only access
    let sum = sum_vector(&numbers);
    println!("Sum of {:?} = {}", numbers, sum);
    
    // Modify through mutable reference
    double_vector(&mut numbers);
    println!("Doubled: {:?}", numbers);
    
    // String slice example
    let text = String::from("hello world");
    let first_word = get_first_word(&text);
    println!("First word of '{}' is '{}'", text, first_word);
    
    println!();

    main5();
}

// Helper functions

/// Calculate the length of a string without taking ownership
fn calculate_length(s: &String) -> usize {
    s.len()  // s is a reference, so we don't drop the value when it goes out of scope
}

/// Modify a string through a mutable reference
fn change_string(s: &mut String) {
    s.push_str(", world!");
}

/// Calculate sum of vector elements using immutable reference
fn sum_vector(numbers: &Vec<i32>) -> i32 {
    numbers.iter().sum()
}

/// Double all elements in a vector using mutable reference
fn double_vector(numbers: &mut Vec<i32>) {
    for num in numbers.iter_mut() {
        *num *= 2;
    }
}

/// Get the first word from a string (returns string slice)
fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]  // Return entire string if no space found
}

/// Example 5: Printing and Ownership - Debug vs Move
/// Shows how printing works with ownership and when you can reuse values
fn example_printing_ownership() {
    println!("🖨️  Example 5: Printing and Ownership");
    println!("====================================");
    
    // MYTH BUSTING: println! does NOT take ownership!
    let s1 = String::from("Hello, Rust!");
    println!("Debug print 1: {}", s1);  // This borrows &s1, doesn't move
    println!("Debug print 2: {}", s1);  // Can use s1 again!
    println!("Debug print 3: {}", s1);  // Still works!
    
    println!("✅ String s1 is still valid: '{}'", s1);
    
    // What actually happens under the hood:
    println!("\n🔍 What println! really does:");
    let s2 = String::from("Behind the scenes");
    println!("This: println!(\"Debug: {{}}\", s2)");
    println!("Is really: println!(\"Debug: {{}}\", &s2)");  // Automatic referencing!
    println!("So s2 is still valid: '{}'", s2);
    
    // Different ways to print the same string
    println!("\n📝 Different ways to print (all keep ownership):");
    let my_string = String::from("I can be printed many ways!");
    
    println!("Direct: {}", my_string);                    // Borrows automatically
    println!("Reference: {}", &my_string);                // Explicit reference  
    println!("Debug: {:?}", my_string);                   // Debug formatting
    println!("Display again: {}", my_string);             // Still works!
    
    // WHEN YOU ACTUALLY LOSE OWNERSHIP:
    println!("\n⚠️  When you DO lose ownership:");
    let s3 = String::from("This will move");
    
    // These operations MOVE the string:
    let s4 = s3;  // Move ownership to s4
    // println!("s3: {}", s3);  // ❌ Would error - s3 no longer valid
    println!("s4: {}", s4);     // ✅ s4 owns the string now
    
    // Function that takes ownership vs borrows
    let s5 = String::from("Function test");
    takes_ownership_bad(s5.clone());  // Clone to keep original
    println!("s5 after clone: {}", s5);  // Still works because we cloned
    
    takes_ownership_good(&s5);  // Borrow instead of move
    println!("s5 after borrow: {}", s5);  // Still works!
    
    // DEBUGGING PATTERNS:
    println!("\n🐛 Common debug patterns:");
    let debug_string = String::from("Debug me!");
    
    // Pattern 1: Debug print without consuming
    println!("Before processing: {:?}", debug_string);
    let processed = debug_string.to_uppercase();
    println!("After processing: {:?}", processed);
    
    // Pattern 2: Multiple debug prints during development  
    let mut data = vec!["apple", "banana", "cherry"];
    println!("Initial data: {:?}", data);  // Borrow for printing
    
    data.push("date");
    println!("After adding: {:?}", data);  // Can print again
    
    data.sort();
    println!("After sorting: {:?}", data); // And again!
    
    // Pattern 3: Print and continue using in chain
    let result = "hello world"
        .to_string()
        .chars()
        .filter(|&c| c != ' ')
        .collect::<String>();
    
    println!("Filtered result: {}", result);
    println!("Length: {}", result.len());  // Can use result again
    
    println!();
}

// Helper functions for ownership examples
fn takes_ownership_bad(s: String) {
    println!("Function received: {}", s);
    // s goes out of scope here and is dropped
}

fn takes_ownership_good(s: &String) {
    println!("Function borrowed: {}", s);
    // s is a reference, original owner keeps the value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_length() {
        let s = String::from("hello");
        assert_eq!(calculate_length(&s), 5);
        assert_eq!(s, "hello");  // s still valid after borrowing
    }

    #[test]
    fn test_change_string() {
        let mut s = String::from("hello");
        change_string(&mut s);
        assert_eq!(s, "hello, world!");
    }

    #[test]
    fn test_sum_vector() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(sum_vector(&numbers), 15);
    }

    #[test]
    fn test_get_first_word() {
        assert_eq!(get_first_word("hello world"), "hello");
        assert_eq!(get_first_word("rust"), "rust");
        assert_eq!(get_first_word(""), "");
    }
}

//fn main2() {
//    let s = String::from("hello");
//
//    change(&s);
//}

//fn change(some_string: &String) {
//    some_string.push_str(", world");
//cannot borrow `*some_string` as mutable, as it is behind a `&` reference
//`some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
//}

//fn main3() {
//    let mut s = String::from("hello");
//
//   let r1 = &mut s;
//    let r2 = &mut s;
//cannot borrow `s` as mutable more than once at a time
//second mutable borrow occurs here
//    println!("{r1}, {r2}");
//}

//fn main4() {
//    let reference_to_nothing = dangle();
//}


//missing lifetime specifier
//this function's return type contains a borrowed value, but there is no value for it to be borrowed from
//fn dangle() -> &String {
//    let s = String::from("hello");
//
//    &s
// }

fn main5() {
    let reference_to_nothing = no_dangle();
    println!("reference_to_nothing = {}", reference_to_nothing);
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}


// fn main6() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     let r3 = &mut s; // BIG PROBLEM
// cannot borrow `s` as mutable because it is also borrowed as immutable
// mutable borrow occurs here
//     println!("{r1}, {r2}, and {r3}");
// }