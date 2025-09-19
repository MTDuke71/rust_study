fn main() {

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    
let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("THREE_HOURS_IN_SECONDS = {}",THREE_HOURS_IN_SECONDS);

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces: {}",spaces);

    /// Data Types
    
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
    println!("x: {}  y: {}",x,y);
}
