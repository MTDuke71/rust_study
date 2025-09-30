// Chapter 5.3 - Method Syntax
// Demonstrates method syntax patterns used in Mission5 HashMap implementation

#[derive(Debug, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function constructors (Mission5 pattern: HashMap::new())
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    
    // Method - takes &self as first parameter (Mission5: map.get(&key))
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Method that takes another Rectangle as parameter
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // Associated function (Mission5 pattern: HashMap::with_capacity())
    fn square(size: u32) -> Self {
        Self::new(size, size)  // Delegate to constructor
    }
    
    // Mutable method (Mission5 pattern: map.insert())
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
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
    
    // Using associated function (Mission5: HashMap::new() pattern)
    let sq = Rectangle::square(3);
    println!("Created square: {:?}", sq);
    println!("Square area: {}", sq.area());
    
    // Demonstrate mutable methods (Mission5: map.insert() pattern)
    let mut scalable_rect = Rectangle::new(10, 15);
    println!("Before scaling: {:?}", scalable_rect);
    scalable_rect.scale(2);
    println!("After scaling by 2: {:?}", scalable_rect);
    
    // Demonstrate multiple impl blocks
    let rect4 = Rectangle::new(20, 30);
    println!("Rectangle perimeter: {}", rect4.perimeter());
}

// Multiple impl blocks are allowed
impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}