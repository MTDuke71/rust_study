// Examples from The Rust Book Chapter 17-03: More Futures
// https://doc.rust-lang.org/stable/book/ch17-03-more-futures.html

use std::pin::{pin, Pin};
use std::time::{Duration, Instant};

// Example 1: Working with Any Number of Futures - join! macro
// Listing 17-14
#[allow(dead_code)]
fn example_join_macro() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        trpl::join!(tx1_fut, rx_fut, tx_fut);
    });
}

// Example 2: Attempting to use Vec with futures (doesn't compile - demonstration only)
// Listing 17-15 - This is shown for educational purposes
/*
#[allow(dead_code)]
fn example_vec_futures_fails() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            // --snip--
        };

        let rx_fut = async {
            // --snip--
        };

        let tx_fut = async move {
            // --snip--
        };

        let futures = vec![tx1_fut, rx_fut, tx_fut];
        trpl::join_all(futures).await;
    });
}
*/

// Example 3: Using Box::new (still doesn't compile - demonstration only)
// Listing 17-16
/*
#[allow(dead_code)]
fn example_box_new_fails() {
    trpl::run(async {
        // ... futures ...
        let futures = vec![Box::new(tx1_fut), Box::new(rx_fut), Box::new(tx_fut)];
        trpl::join_all(futures).await;
    });
}
*/

// Example 4: Using Box with type annotation (still doesn't compile - demonstration only)
// Listing 17-17
/*
#[allow(dead_code)]
fn example_box_with_type_fails() {
    trpl::run(async {
        // ... futures ...
        let futures: Vec<Box<dyn Future<Output = ()>>> =
            vec![Box::new(tx1_fut), Box::new(rx_fut), Box::new(tx_fut)];
        trpl::join_all(futures).await;
    });
}
*/

// Example 5: Using Pin and Box::pin - WORKING VERSION
// Listing 17-18
#[allow(dead_code)]
fn example_pin_box() {
    use std::future::Future;

    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
            vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];

        trpl::join_all(futures).await;
    });
}

// Example 6: Using Pin directly with pin! macro - WORKING VERSION (more efficient)
// Listing 17-19
#[allow(dead_code)]
fn example_pin_macro() {
    use std::future::Future;

    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = pin!(async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        });

        let tx_fut = pin!(async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        });

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx1_fut, rx_fut, tx_fut];

        trpl::join_all(futures).await;
    });
}

// Example 7: Different output types with join!
// Listing 17-20
#[allow(dead_code)]
fn example_different_types() {
    trpl::run(async {
        let a = async { 1u32 };
        let b = async { "Hello!" };
        let c = async { true };

        let (a_result, b_result, c_result) = trpl::join!(a, b, c);
        println!("{a_result}, {b_result}, {c_result}");
    });
}

// Example 8: Racing Futures
// Listing 17-21
#[allow(dead_code)]
fn example_race() {
    trpl::run(async {
        let slow = async {
            println!("'slow' started.");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("'slow' finished.");
        };

        let fast = async {
            println!("'fast' started.");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'fast' finished.");
        };

        trpl::race(slow, fast).await;
    });
}

// Example 9: Yielding Control to the Runtime - thread::sleep version
// Listing 17-22 and 17-23
#[allow(dead_code)]
fn example_slow_blocking() {
    use std::thread;

    fn slow(name: &str, ms: u64) {
        thread::sleep(Duration::from_millis(ms));
        println!("'{name}' ran for {ms}ms");
    }

    trpl::run(async {
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            slow("a", 10);
            slow("a", 20);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            slow("b", 10);
            slow("b", 15);
            slow("b", 350);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'b' finished.");
        };

        trpl::race(a, b).await;
    });
}

// Example 10: Using sleep to yield control
// Listing 17-24
#[allow(dead_code)]
fn example_sleep_yield() {
    use std::thread;

    fn slow(name: &str, ms: u64) {
        thread::sleep(Duration::from_millis(ms));
        println!("'{name}' ran for {ms}ms");
    }

    trpl::run(async {
        let one_ms = Duration::from_millis(1);

        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::sleep(one_ms).await;
            slow("a", 10);
            trpl::sleep(one_ms).await;
            slow("a", 20);
            trpl::sleep(one_ms).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::sleep(one_ms).await;
            slow("b", 10);
            trpl::sleep(one_ms).await;
            slow("b", 15);
            trpl::sleep(one_ms).await;
            slow("b", 350);
            trpl::sleep(one_ms).await;
            println!("'b' finished.");
        };

        trpl::race(a, b).await;
    });
}

// Example 11: Using yield_now for better performance
// Listing 17-25
#[allow(dead_code)]
fn example_yield_now() {
    use std::thread;

    fn slow(name: &str, ms: u64) {
        thread::sleep(Duration::from_millis(ms));
        println!("'{name}' ran for {ms}ms");
    }

    trpl::run(async {
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("'b' finished.");
        };

        trpl::race(a, b).await;
    });
}

// Example 12: Performance comparison - sleep vs yield_now
// Listing 17-26
#[allow(dead_code)]
fn example_performance_comparison() {
    trpl::run(async {
        println!("Running performance comparison...");

        let one_ns = Duration::from_nanos(1);
        let start = Instant::now();
        async {
            for _ in 1..100 {
                trpl::sleep(one_ns).await;
            }
        }
        .await;
        let time = Instant::now() - start;
        println!(
            "'sleep' version finished after {} seconds ({} microseconds).",
            time.as_secs_f32(),
            time.as_micros()
        );

        let start = Instant::now();
        async {
            for _ in 1..1000000 {
                trpl::yield_now().await;
            }
        }
        .await;
        let time = Instant::now() - start;
        println!(
            "'yield' version finished after {} seconds ({} microseconds).",
            time.as_secs_f32(),
            time.as_micros()
        );
    });
}

// Example 13: Building Our Own Async Abstractions - timeout function
// Listing 17-27, 17-28, and 17-29
#[allow(dead_code)]
fn example_timeout() {
    use std::future::Future;
    use trpl::Either;

    async fn timeout<F: Future>(
        future_to_try: F,
        max_time: Duration,
    ) -> Result<F::Output, Duration> {
        match trpl::race(future_to_try, trpl::sleep(max_time)).await {
            Either::Left(output) => Ok(output),
            Either::Right(_) => Err(max_time),
        }
    }

    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(4)).await;
            "Finally finished"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}

// Example 14: Demonstrating Windows Timer Resolution with timeBeginPeriod
#[allow(dead_code)]
fn example_timer_resolution() {
    #[cfg(windows)]
    {
        use std::thread;

        // Windows FFI for timer resolution control
        #[link(name = "winmm")]
        extern "system" {
            fn timeBeginPeriod(period: u32) -> u32;
            fn timeEndPeriod(period: u32) -> u32;
        }

        println!("=== Windows Timer Resolution Demonstration ===\n");

        // Test 1: Default Windows timer resolution (~15.6ms)
        println!("Test 1: Default timer resolution (no timeBeginPeriod) - 100 iterations");
        let start = Instant::now();
        for _ in 0..100 {
            thread::sleep(Duration::from_millis(1));
        }
        let total = Instant::now() - start;
        println!(
            "  Total: {}ms (average {:.2}ms per sleep)\n",
            total.as_millis(),
            total.as_secs_f64() * 1000.0 / 100.0
        );

        // Test 2: Request 1ms timer resolution
        println!("Test 2: With timeBeginPeriod(1) - requesting 1ms resolution - 100 iterations");
        unsafe {
            let result = timeBeginPeriod(1);
            if result == 0 {
                println!("  ✓ Successfully set timer resolution to 1ms");
            } else {
                println!("  ✗ Failed to set timer resolution (error: {})", result);
            }
        }

        let start = Instant::now();
        for _ in 0..100 {
            thread::sleep(Duration::from_millis(1));
        }
        let total = Instant::now() - start;
        println!(
            "  Total: {}ms (average {:.2}ms per sleep)\n",
            total.as_millis(),
            total.as_secs_f64() * 1000.0 / 100.0
        );

        // Clean up: restore default timer resolution
        unsafe {
            timeEndPeriod(1);
            println!("  ✓ Restored default timer resolution");
        }

        println!("\n=== Async Version Comparison ===\n");

        // Test 3: Async sleep with default resolution
        println!("Test 3: trpl::sleep with default timer resolution - 100 iterations");
        let start = Instant::now();
        trpl::run(async {
            for _ in 0..100 {
                trpl::sleep(Duration::from_millis(1)).await;
            }
        });
        let total = Instant::now() - start;
        println!(
            "  Total: {}ms (average {:.2}ms per sleep)\n",
            total.as_millis(),
            total.as_secs_f64() * 1000.0 / 100.0
        );

        // Test 4: Async sleep with 1ms resolution
        println!("Test 4: trpl::sleep with timeBeginPeriod(1) - 100 iterations");
        unsafe {
            timeBeginPeriod(1);
        }

        let start = Instant::now();
        trpl::run(async {
            for _ in 0..100 {
                trpl::sleep(Duration::from_millis(1)).await;
            }
        });
        let total = Instant::now() - start;
        println!(
            "  Total: {}ms (average {:.2}ms per sleep)\n",
            total.as_millis(),
            total.as_secs_f64() * 1000.0 / 100.0
        );

        unsafe {
            timeEndPeriod(1);
        }

        println!("\n=== Key Takeaway ===");
        println!("Without timeBeginPeriod: ~15.6ms minimum sleep");
        println!("With timeBeginPeriod(1): ~1-2ms minimum sleep");
        println!("This is why your 100 sleeps took 1.45 seconds!");
    }

    #[cfg(not(windows))]
    {
        println!("This example only works on Windows.");
        println!("Linux/macOS have different timer mechanisms (typically 1-4ms by default).");
    }
}

fn main() {
    println!("Running Chapter 17-03 examples...\n");

    // Uncomment the example you want to run:

    // println!("Example 1: join! macro");
    // example_join_macro();

    // println!("\nExample 5: Pin and Box::pin");
    // example_pin_box();

    // println!("\nExample 6: Pin with pin! macro (more efficient)");
    // example_pin_macro();

    // println!("\nExample 7: Different output types with join!");
    // example_different_types();

    // println!("\nExample 8: Racing futures");
    // example_race();

    // println!("\nExample 9: Slow blocking operations");
    // example_slow_blocking();

    // println!("\nExample 10: Using sleep to yield control");
    // example_sleep_yield();

    // println!("\nExample 11: Using yield_now for better performance");
    // example_yield_now();

    // println!("\nExample 12: Performance comparison - sleep vs yield_now");
    // example_performance_comparison();

    // println!("\nExample 13: Building custom timeout abstraction");
    // example_timeout();

    println!("\nExample 14: Windows Timer Resolution Demonstration");
    example_timer_resolution();
}
