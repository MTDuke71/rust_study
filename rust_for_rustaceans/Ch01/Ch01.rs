use std::mem;

// --- Helper Functions and Structs defined in the Chapter ---

// Listing 1-4: Rust assumes that shared references are immutable. [1]
fn cache(input: &i32, sum: &mut i32) {
    *sum = *input + *input;
    assert_eq!(*sum, 2 * *input);
}

// Listing 1-5: Rust assumes that mutable references are exclusive. [2]
fn noalias(input: &i32, output: &mut i32) {
    if *input == 1 {
        *output = 2;
    }
    if *input != 1 {
        *output = 3;
    }
}

// Listing 1-7: Access through a mutable reference must leave a value behind. [3]
fn replace_with_84(s: &mut Box<i32>) {
    // this is not okay, as *s would be empty:
    // let was = *s;
    
    // but this is:
    let was = std::mem::take(s); // [3]
    // so is this:
    *s = was; // [3]
    
    // we can exchange values behind &mut:
    let mut r = Box::new(84);
    std::mem::swap(s, &mut r); // [3]
    assert_ne!(*r, 84);
}

// Listing 1-10: A type that needs to be generic over multiple lifetimes. [4]
struct StrSplit<'s, 'p> {
    delimiter: &'p str,
    document: &'s str,
}

impl<'s, 'p> Iterator for StrSplit<'s, 'p> {
    type Item = &'s str;
    
    fn next(&self) -> Option<Self::Item> {
        // Source uses todo!(), returning None here to make the example runnable.
        None 
    }
}

// Listing 1-10 Helper function [4]
fn str_before(s: &str, c: char) -> Option<&str> {
    StrSplit { document: s, delimiter: &c.to_string() }.next()
}

// Listing 1-11: A type that needs to be generic over multiple lifetimes. [5]
struct MutStr<'a, 'b> {
    s: &'a mut &'b str
}

// Mock function for Listing 1-8 to simulate randomness
fn rand() -> f64 {
    0.6
}

fn main() {
    println!("--- Chapter 1 Examples ---");

    // Listing 1-1: Values, variables, and pointers [6]
    {
        println!("\nListing 1-1:");
        let x = 42;
        let y = 43;
        let var1 = &x;
        let mut var2 = &x;
        var2 = &y;
        println!("x: {}, y: {}, var1: {}, var2: {}", x, y, var1, var2);
    }

    // String example [7]
    {
        let string = "Hello world";
        println!("String variable points to: {}", string);
    }

    // Listing 1-2: Illegal flows that the borrow checker will catch [8]
    // This code is commented out because it causes a compile-time error.
    {
        /* 
        let mut x;
        // this access would be illegal, nowhere to draw the flow from:
        // assert_eq!(x, 42);
        x = 42;
        // this is okay, can draw a flow from the value assigned above:
        let y = &x;
        // this establishes a second, mutable flow from x:
        x = 43; 
        // this continues the flow from y, which in turn draws from x.
        // but that flow conflicts with the assignment to x!
        assert_eq!(*y, 42); 
        */
    }

    // Low-Level Model Example [9]
    {
        let x: usize;
        x = 6;
        println!("Low-level model x: {}", x);
    }

    // Listing 1-3: Moving and copying semantics [10]
    {
        println!("\nListing 1-3:");
        let x1 = 42;
        let y1 = Box::new(84);
        { 
            // starts a new scope
            let z = (x1, y1); 
            // z goes out of scope, and is dropped;
            // it in turn drops the values from x1 and y1
            println!("Inside scope, z: {:?}", z);
        }
        // x1's value is Copy, so it was not moved into z
        let x2 = x1;
        println!("x1 is still valid: {}", x2);
        // y1's value is not Copy, so it was moved into z
        // let y2 = y1; // This would cause a compile error
    }

    // Listing 1-4: Rust assumes that shared references are immutable [1]
    {
        println!("\nListing 1-4:");
        let x = 42;
        let mut sum = 0;
        cache(&x, &mut sum);
        println!("Cache result: {}", sum);
    }

    // Listing 1-5: Rust assumes that mutable references are exclusive [2]
    {
        println!("\nListing 1-5:");
        let input = 1;
        let mut output = 0;
        noalias(&input, &mut output);
        println!("Noalias result: {}", output);
    }

    // Listing 1-6: Mutability applies only to the immediately referenced memory [11]
    {
        let x = 42;
        let mut y = &x; // y is of type &i32
        let z = &mut y; // z is of type &mut &i32
        println!("z points to y which points to x: {}", **z);
    }

    // Listing 1-7: Access through a mutable reference must leave a value behind [3]
    {
        println!("\nListing 1-7:");
        let mut s = Box::new(42);
        replace_with_84(&mut s);
        println!("s is now: {}", s);
    }

    // Listing 1-8: Lifetimes do not need to be contiguous [12]
    {
        println!("\nListing 1-8:");
        let mut x = Box::new(42);
        let r = &x; // 'a
        
        if rand() > 0.5 {
            *x = 84;
        } else {
            println!("{}", r); // 'a
        }
    }

    // Listing 1-9: Lifetimes can have holes [13, 14]
    {
        println!("\nListing 1-9:");
        let mut x = Box::new(42);
        let mut z = &x; // 'a
        
        for i in 0..100 {
            // println!("{}", z); // 'a (Commented out to avoid spamming output)
            x = Box::new(i);
            z = &x; // 'a
        }
        println!("Final z: {}", z); // 'a
    }

    // Listing 1-10: Generic Lifetimes Usage [4]
    {
        println!("\nListing 1-10:");
        let s = "hello world";
        let c = ' ';
        let result = str_before(s, c);
        println!("str_before result: {:?}", result);
    }

    // Listing 1-11: A type that needs to be generic over multiple lifetimes [5]
    {
        println!("\nListing 1-11:");
        let mut s = "hello";
        // Equivalent to defining a variable x holding MutStr and writing *x.s = "world"
        *MutStr { s: &mut s }.s = "world";
        println!("{}", s);
    }
}