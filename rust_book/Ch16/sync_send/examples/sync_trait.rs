// Example: Sync Trait
// Understanding safe reference sharing between threads

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("=== Sync Trait Example ===\n");

    // Example 1: Sync types can be shared via references across threads
    println!("Example 1: Primitives are Sync");

    let shared_number = &42;

    // Can use scoped threads to share references
    thread::scope(|s| {
        s.spawn(|| {
            println!("Thread 1 sees: {}", shared_number);
        });

        s.spawn(|| {
            println!("Thread 2 sees: {}", shared_number);
        });
    });

    // Example 2: Arc allows safe shared ownership (Arc<T> is Sync if T is Send+Sync)
    println!("\nExample 2: Arc provides Sync wrapper");

    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];

    for i in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("Thread {} sees: {:?}", i, data);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Example 3: Mutex<T> is Sync (provides interior mutability safely)
    println!("\nExample 3: Mutex provides Sync with interior mutability");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter: {}", *counter.lock().unwrap());

    // Example 4: What is NOT Sync
    println!("\nExample 4: RefCell is NOT Sync");
    println!("(This code is commented out because it won't compile)\n");

    // This would fail to compile:
    // let cell = RefCell::new(vec![1, 2, 3]);
    // let cell_ref = &cell;
    // thread::spawn(move || {
    //     cell_ref.borrow_mut().push(4);  // ERROR: RefCell<Vec<i32>> cannot be shared between threads
    // });

    println!("RefCell is for single-threaded interior mutability");
    println!("Use Mutex or RwLock for multi-threaded interior mutability");

    // Example 5: Demonstrating Send vs Sync
    println!("\nExample 5: Send vs Sync distinction");

    // Box<T> is Send but not necessarily Sync
    // Arc<T> is both Send and Sync (if T is Send+Sync)

    struct Data {
        value: i32,
    }

    let arc_data = Arc::new(Data { value: 42 });

    thread::scope(|s| {
        // Can share Arc references across threads (Sync)
        let data_ref = &arc_data;

        s.spawn(move || {
            println!("Thread accessing via reference: {}", data_ref.value);
        });
    });

    // Can also send Arc across threads (Send)
    let arc_clone = Arc::clone(&arc_data);
    thread::spawn(move || {
        println!("Thread received ownership: {}", arc_clone.value);
    })
    .join()
    .unwrap();

    println!("\nKey insight:");
    println!("- Send: Type can be TRANSFERRED between threads (move)");
    println!("- Sync: Type can be SHARED between threads (&T can be Send)");
}
