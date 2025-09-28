// Chapter 5.3 - Method Syntax

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method - takes &self as first parameter
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Method that takes another Rectangle as parameter
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // Associated function (like static method) - no self parameter
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    println!("Chapter 5.3 - Method Syntax");
    
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    // Using method syntax
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    
    // Multiple rectangles for can_hold example
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    // Using associated function
    let sq = Rectangle::square(3);
    println!("Created square: {:?}", sq);
    println!("Square area: {}", sq.area());
    
    // Demonstrate multiple impl blocks
    let rect4 = Rectangle {
        width: 20,
        height: 30,
    };
    
    println!("Rectangle perimeter: {}", rect4.perimeter());
}

// Multiple impl blocks are allowed
impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}