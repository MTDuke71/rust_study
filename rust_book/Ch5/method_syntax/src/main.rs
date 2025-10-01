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

      fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    println!("Chapter 5.3 - Method Syntax & Memory Address Analysis");
    
    // =============================================================================
    // MEMORY ADDRESS PRINTING IN RUST (like C's pointer printing)
    // =============================================================================
    
    println!("\n🔍 Memory Address Analysis:");
    
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    // Print address of the struct itself
    println!("   rect1 address: {:p}", &rect1);
    
    // Print addresses of individual fields
    println!("   rect1.width address: {:p}", &rect1.width);
    println!("   rect1.height address: {:p}", &rect1.height);
    
    // Stack vs Heap demonstration
    let stack_rect = Rectangle::new(10, 20);
    let heap_rect = Box::new(Rectangle::new(10, 20));
    
    println!("\n📚 Stack vs Heap Addresses:");
    println!("   Stack rect: {:p}", &stack_rect);
    println!("   Heap rect (Box): {:p}", heap_rect.as_ref());
    println!("   Box itself: {:p}", &heap_rect);
    
    // Address changes with moves
    println!("\n🚚 Move Semantics & Address Changes:");
    let original = Rectangle::new(5, 5);
    println!("   Original address: {:p}", &original);
    
    let moved = original;  // Move occurs here
    println!("   After move: {:p}", &moved);
    // Note: original is no longer accessible
    
    // Reference addresses vs value addresses
    println!("\n🔗 Reference vs Value Addresses:");
    let value = 42u32;
    let reference = &value;
    let reference_to_reference = &reference;
    
    println!("   Value address: {:p}", &value);
    println!("   Reference points to: {:p}", reference);
    println!("   Reference itself at: {:p}", &reference);
    println!("   Ref-to-ref points to: {:p}", reference_to_reference);
    println!("   Ref-to-ref itself at: {:p}", &reference_to_reference);
    
    // Using method syntax
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    
       if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    } else {
        println!("The rectangle does not have a nonzero width; it is {}", rect1.width);
    }
    
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