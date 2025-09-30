
   use std::io;

    fn main() {
        // Data Types
    
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
    println!("x: {}  y: {}",x,y);


        // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;


    println!("sum: {}  difference: {}  product: {}  quotient: {}  truncated: {}  remainder: {}",sum,difference,product,quotient,truncated,remainder);

    let t = true;

    let f: bool = false; // with explicit type annotation
    println!("t: {}  f: {}",t,f);

    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻'; 
    println!("c: {}  z: {}  heart_eyed_cat: {}",c,z,heart_eyed_cat); 

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
    println!("five_hundred: {}  six_point_four: {}  one: {}",five_hundred,six_point_four,one);

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    let third = a[2];
    let fourth = a[3];
    let fifth = a[4];
    println!("first: {}  second: {}  third: {}  fourth: {}  fifth: {}",first,second,third,fourth,fifth);

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("months: {:?}",months);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a: {:?}",a);

    let a = [3; 5];
    println!("a: {:?}",a);

    
    //Example of array indexing that will cause a runtime error
    let a = [1, 2, 3, 4, 5];
    println!("a: {:?}",a);

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
    //crashes if you enter an invalid index > 4 
    
    }

