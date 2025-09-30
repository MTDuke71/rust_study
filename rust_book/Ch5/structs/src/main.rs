// Chapter 5.1 - Defining and Instantiating Structs

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    println!("Chapter 5.1 - Defining and Instantiating Structs");
    
    // Creating an instance of User
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    
    println!("User: {} ({}) - Active: {}, Sign-ins: {}", 
             user1.username, user1.email, user1.active, user1.sign_in_count);
    
    // Mutable instance
    let mut user2 = User {
        active: true,
        username: String::from("anotherusername567"),
        email: String::from("another@example.com"),
        sign_in_count: 1,
    };
    
    user2.email = String::from("anotheremail@example.com");
    println!("Updated user email: {}", user2.email);
    
    // Using functions
    let user3 = build_user(String::from("third@example.com"), String::from("thirduser"));
    println!("Built user: {} ({})", user3.username, user3.email);
    
    // Struct update syntax
    let user4 = User {
        email: String::from("fourth@example.com"),
        ..user3
    };
    println!("User with update syntax: {} ({})", user4.username, user4.email);
    
    // Tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("Black color: ({}, {}, {})", black.0, black.1, black.2);
    println!("Origin point: ({}, {}, {})", origin.0, origin.1, origin.2);
    
    // Unit-like structs
    #[derive(Debug)]
    struct AlwaysEqual;
    let subject = AlwaysEqual;
    println!("Created unit-like struct: {:?}", subject);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // Field init shorthand
        email,    // Field init shorthand
        sign_in_count: 1,
    }
}

// Invalid Code Snippet Below - For Reference Only
//
// struct UserX {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }
// // missing lifetime specifier
// //expected named lifetime parameter

// fn main2() {
//     let user1 = UserX {
//         active: true,
//         username: "someusername123",
//         email: "someone@example.com",
//         sign_in_count: 1,
//     };
// }