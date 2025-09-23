// Ch4/src/main.rs

fn main() {

      {                     // s is not valid here, since it's not yet declared
        let s = "hello";    // s is valid from this point forward
        println!("{}", s);  // do stuff with s
    }                       // this scope is now over, and s is no longer valid

    let s = String::from("hello");

    println!("{}", s); // this will print `hello`

    let mut s1 = String::from("hello");

    s1.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s1); // this will print `hello, world!`

    {
        let s = String::from("hello"); // s is valid from this point forward
        println!("{}", s);  // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid



}