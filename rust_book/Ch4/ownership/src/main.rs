// Ch4/src/main.rs

fn main() {
    {
        // s is not valid here, since it's not yet declared
        let s = "hello"; // s is valid from this point forward
        println!("{}", s); // do stuff with s
    } // this scope is now over, and s is no longer valid

    let s = String::from("hello");

    println!("{}", s); // this will print `hello`

    let mut s1 = String::from("hello");

    s1.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s1); // this will print `hello, world!`

    {
        let s = String::from("hello"); // s is valid from this point forward
        println!("{}", s); // do stuff with s
    } // this scope is now over, and s is no
      // longer valid

    //copying simple variables, makes a copy of the value (Stack only)
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y); // this will print `x = 5, y = 5`

    //moving complex variables, moves the value and invalidates the original
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1); // this line would cause a compile-time error

    println!("{}, world!", s2);

    let mut s = String::from("hello");
    println!("{s}"); // this will print `hello`

    s = String::from("ahoy"); // Here, the String from "hello" is dropped and replaced with "ahoy"

    println!("{s}"); // this will print `ahoy`

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    main1();

    main2();

    main3();
}

fn main1() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // Because i32 implements the Copy trait,
                   // x does NOT move into the function,
                   // so it's okay to use x afterward.
} // Here, x goes out of scope, then s. However, because s's value was moved,
  // nothing special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn main2() {
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3

    println!("s1 = {s1}, s3 = {s3}");
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn main3() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
