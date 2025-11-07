//! 14.3 - Cargo Workspaces
//!
//! This binary crate demonstrates using workspace members.


fn main() {
    let num = 10;
    
    println!("Starting number: {}", num);
    println!("Adding one: {}", add_one::add_one(num));
    println!("Adding two: {}", add_two::add_two(num));
    
    // Chain operations from different workspace members
    let result = add_two::add_two(add_one::add_one(num));
    println!("Adding one then two: {}", result);
}

