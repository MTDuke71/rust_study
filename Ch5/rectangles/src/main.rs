// Chapter 5.2 - An Example Program Using Structs
// Rectangle area calculation program

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