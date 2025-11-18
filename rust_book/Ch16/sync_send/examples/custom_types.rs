// Example: Building Thread-Safe Custom Types
// Understanding how to implement Send and Sync for custom types

use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("=== Thread-Safe Custom Types ===\n");

    // Example 1: Automatically Send+Sync custom type
    println!("Example 1: Auto-derived Send+Sync");

    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    struct Config {
        timeout: u64,
        retries: u32,
        host: String,
    }

    // Config is automatically Send+Sync because all fields are Send+Sync

    let config = Arc::new(Config {
        timeout: 5000,
        retries: 3,
        host: String::from("localhost"),
    });

    let mut handles = vec![];

    for i in 0..3 {
        let config = Arc::clone(&config);
        let handle = thread::spawn(move || {
            println!("Thread {}: {:?}", i, config);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Example 2: Thread-safe shared state with interior mutability
    println!("\nExample 2: Thread-safe counter");

    struct ThreadSafeCounter {
        count: Mutex<u64>,
    }

    impl ThreadSafeCounter {
        fn new() -> Self {
            Self {
                count: Mutex::new(0),
            }
        }

        fn increment(&self) {
            let mut count = self.count.lock().unwrap();
            *count += 1;
        }

        fn get(&self) -> u64 {
            *self.count.lock().unwrap()
        }
    }

    let counter = Arc::new(ThreadSafeCounter::new());
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                counter.increment();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", counter.get());

    // Example 3: Type that is Send but not Sync
    println!("\nExample 3: Send but not Sync type");

    use std::cell::Cell;

    struct SendNotSync {
        // Cell is Send but not Sync
        value: Cell<i32>,
    }

    let send_not_sync = SendNotSync {
        value: Cell::new(42),
    };

    // Can send to another thread (Send)
    let handle = thread::spawn(move || {
        send_not_sync.value.set(100);
        println!("Value updated to: {}", send_not_sync.value.get());
    });

    handle.join().unwrap();

    // But cannot share reference across threads (not Sync)
    // This would fail to compile:
    // let sns = SendNotSync { value: Cell::new(42) };
    // thread::spawn(|| {
    //     println!("{}", sns.value.get());  // ERROR
    // });

    // Example 4: Explicitly opting out of Send/Sync
    println!("\nExample 4: Explicitly not Send/Sync");

    struct NotThreadSafe {
        data: i32,
        _not_send_sync: PhantomData<*const ()>, // Raw pointer is not Send/Sync
    }

    let not_safe = NotThreadSafe {
        data: 42,
        _not_send_sync: PhantomData,
    };

    // This would fail to compile:
    // thread::spawn(move || {
    //     println!("{}", not_safe.data);  // ERROR: NotThreadSafe is not Send
    // });

    println!("NotThreadSafe cannot be sent to threads");
    println!("Data: {}", not_safe.data);

    // Example 5: Wrapper type for thread-safe access
    println!("\nExample 5: Thread-safe wrapper");

    struct SharedState<T> {
        data: Arc<Mutex<T>>,
    }

    impl<T> SharedState<T> {
        fn new(initial: T) -> Self {
            Self {
                data: Arc::new(Mutex::new(initial)),
            }
        }

        fn with<F, R>(&self, f: F) -> R
        where
            F: FnOnce(&mut T) -> R,
        {
            let mut guard = self.data.lock().unwrap();
            f(&mut *guard)
        }

        fn clone_handle(&self) -> Self {
            Self {
                data: Arc::clone(&self.data),
            }
        }
    }

    let state = SharedState::new(vec![1, 2, 3]);
    let mut handles = vec![];

    for i in 0..3 {
        let state = state.clone_handle();
        let handle = thread::spawn(move || {
            state.with(|vec| {
                vec.push(i * 10);
                println!("Thread {} modified vector", i);
            });
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    state.with(|vec| {
        println!("Final state: {:?}", vec);
    });

    println!("\nKey takeaways:");
    println!("- Most types are automatically Send+Sync if all fields are");
    println!("- Use Mutex/RwLock for interior mutability in multi-threaded context");
    println!("- PhantomData can opt out of Send/Sync when needed");
    println!("- Wrapper types can provide ergonomic thread-safe APIs");
}
