// Example: Send Trait
// Understanding ownership transfer between threads

use std::thread;

fn main() {
    println!("=== Send Trait Example ===\n");

    // Example 1: Send types can be transferred to threads
    println!("Example 1: Send types (String, Vec, Box)");

    let data = String::from("Hello from main thread!");
    let numbers = vec![1, 2, 3, 4, 5];
    let boxed = Box::new(42);

    let handle = thread::spawn(move || {
        println!("Thread received: {}", data);
        println!("Thread received numbers: {:?}", numbers);
        println!("Thread received boxed value: {}", boxed);
    });

    handle.join().unwrap();

    // Example 2: Primitive types are Send
    println!("\nExample 2: Primitives are Send");

    let number = 100;
    let flag = true;
    let character = 'A';

    let handle = thread::spawn(move || {
        println!("Number: {}", number);
        println!("Flag: {}", flag);
        println!("Character: {}", character);
    });

    handle.join().unwrap();

    // Example 3: Demonstrating what is NOT Send
    println!("\nExample 3: Rc is NOT Send");
    println!("(This code is commented out because it won't compile)");
    println!("Rc<T> is not Send because it's not thread-safe\n");

    // This would fail to compile:
    // let rc = Rc::new(vec![1, 2, 3]);
    // let handle = thread::spawn(move || {
    //     println!("{:?}", rc);  // ERROR: Rc<Vec<i32>> cannot be sent between threads safely
    // });

    println!("Use Arc<T> instead of Rc<T> for thread-safe reference counting");

    // Example 4: Custom types are automatically Send if all fields are Send
    println!("\nExample 4: Custom struct is Send");

    #[allow(dead_code)]
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Person {
        name: String,
        age: u32,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    let handle = thread::spawn(move || {
        println!("Person: {:?}", person);
    });

    handle.join().unwrap();

    // Example 5: Demonstrating Send with channels
    println!("\nExample 5: Send with message passing");
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();

    let messages = vec![
        String::from("Message 1"),
        String::from("Message 2"),
        String::from("Message 3"),
    ];

    thread::spawn(move || {
        for msg in messages {
            tx.send(msg).unwrap(); // String is Send, so can be sent through channel
        }
    });

    for received in rx {
        println!("Received: {}", received);
    }
}
