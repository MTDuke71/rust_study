// Chapter 5.2 - An Example Program Using Structs
// Rectangle area calculation program


// Chapter 5.2 of The Rust Programming Language (“An Example Program Using Structs”) walks through building a small program to demonstrate how structs can organize related data and methods in Rust. Here’s a concise summary:
// Goal of the Example
// Build a program that calculates the area of a rectangle using structs instead of separate variables.
// Key Steps and Concepts
// Starting with Separate Variables
// Initially, the area is calculated using two separate variables (width and height).
// This approach works but doesn’t group related data together.
// Using Tuples
// Next, the example uses a tuple (width, height) to pass dimensions to a function.
// Tuples group values but lack clarity because the fields have no names.
// Introducing Structs
// Define a Rectangle struct with named fields:
// Ruststruct Rectangle {    width: u32,    height: u32,}Show more lines
// This improves readability and makes the code self-documenting.
// Implementing Methods
// Use impl to define methods for the struct:
// Rustimpl Rectangle {    fn area(&self) -> u32 {        self.width * self.height    }}Show more lines
// &self is a shorthand for borrowing the instance immutably.
// Adding More Functionality
// Additional methods like can_hold(&self, other: &Rectangle) demonstrate how structs can encapsulate logic related to their data.
// Using dbg! and #[derive(Debug)]
// The dbg! macro and Debug trait help print struct values for debugging.
// Takeaways
// Structs provide clarity, organization, and encapsulation.
// Methods in impl blocks allow behavior to be associated with data.
// Rust encourages ownership and borrowing principles even in method definitions.


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    println!("Chapter 5.2 - An Example Program Using Structs");
    
    // Version 1: Using separate variables
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area_separate(width1, height1)
    );
    
    // Version 2: Using tuples
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );
    
    // Version 3: Using structs
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect2)
    );
    
    // Debug printing
    println!("rect2 is {:?}", rect2);
    println!("rect2 is {:#?}", rect2);
    
    // Using dbg! macro
    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect3);
}

fn area_separate(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}