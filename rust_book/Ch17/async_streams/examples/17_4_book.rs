// Examples from The Rust Book Chapter 17-04: Streams
// https://doc.rust-lang.org/stable/book/ch17-04-streams.html

use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};

// Example 1: Basic stream from iterator
// Listing 17-30 and 17-31
#[allow(dead_code)]
fn example_stream_from_iterator() {
    trpl::run(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    });
}

// Example 2: Filtering a stream
// Listing 17-32
#[allow(dead_code)]
fn example_stream_filter() {
    trpl::run(async {
        let values = 1..101;
        let iter = values.map(|n| n * 2);
        let stream = trpl::stream_from_iter(iter);

        let mut filtered = stream.filter(|value| value % 3 == 0 || value % 5 == 0);

        while let Some(value) = filtered.next().await {
            println!("The value was: {value}");
        }
    });
}

// Example 3: Basic message stream from channel
// Listing 17-33
#[allow(dead_code)]
fn example_basic_message_stream() {
    fn get_messages() -> impl Stream<Item = String> {
        let (tx, rx) = trpl::channel();

        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for message in messages {
            tx.send(format!("Message: '{message}'")).unwrap();
        }

        ReceiverStream::new(rx)
    }

    trpl::run(async {
        let mut messages = get_messages();

        while let Some(message) = messages.next().await {
            println!("{message}");
        }
    });
}

// Example 4: Adding timeout to stream
// Listing 17-34
#[allow(dead_code)]
fn example_timeout_stream() {
    fn get_messages() -> impl Stream<Item = String> {
        let (tx, rx) = trpl::channel();

        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for message in messages {
            tx.send(format!("Message: '{message}'")).unwrap();
        }

        ReceiverStream::new(rx)
    }

    trpl::run(async {
        let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));

        while let Some(result) = messages.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    })
}

// Example 5: Adding delays between messages
// Listing 17-35
#[allow(dead_code)]
fn example_delayed_messages() {
    fn get_messages() -> impl Stream<Item = String> {
        let (tx, rx) = trpl::channel();

        trpl::spawn_task(async move {
            let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
            for (index, message) in messages.into_iter().enumerate() {
                let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
                trpl::sleep(Duration::from_millis(time_to_sleep)).await;

                tx.send(format!("Message: '{message}'")).unwrap();
            }
        });

        ReceiverStream::new(rx)
    }

    trpl::run(async {
        let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));

        while let Some(result) = messages.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    })
}

// Example 6: Creating an interval stream
// Listing 17-36
#[allow(dead_code)]
fn example_interval_stream() {
    fn get_intervals() -> impl Stream<Item = u32> {
        let (tx, rx) = trpl::channel();

        trpl::spawn_task(async move {
            let mut count = 0;
            loop {
                trpl::sleep(Duration::from_millis(1)).await;
                count += 1;
                tx.send(count).unwrap();
            }
        });

        ReceiverStream::new(rx)
    }

    trpl::run(async {
        let mut intervals = get_intervals();
        
        // Just print first 10 intervals for demonstration
        for _ in 0..10 {
            if let Some(count) = intervals.next().await {
                println!("Interval: {count}");
            }
        }
    })
}

// Example 7: Merging streams (type mismatch - doesn't compile)
// Listing 17-37 - Demonstrates the type mismatch problem
/*
#[allow(dead_code)]
fn example_merge_type_mismatch() {
    fn get_messages() -> impl Stream<Item = String> {
        let (tx, rx) = trpl::channel();

        trpl::spawn_task(async move {
            let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
            for (index, message) in messages.into_iter().enumerate() {
                let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
                trpl::sleep(Duration::from_millis(time_to_sleep)).await;
                tx.send(format!("Message: '{message}'")).unwrap();
            }
        });

        ReceiverStream::new(rx)
    }

    fn get_intervals() -> impl Stream<Item = u32> {
        let (tx, rx) = trpl::channel();

        trpl::spawn_task(async move {
            let mut count = 0;
            loop {
                trpl::sleep(Duration::from_millis(1)).await;
                count += 1;
                tx.send(count).unwrap();
            }
        });

        ReceiverStream::new(rx)
    }

    trpl::run(async {
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals();
        let merged = messages.merge(intervals);  // Type mismatch!

        while let Some(result) = merged.next().await {
            // ...
        }
    })
}
*/

// Example 8: Merging streams with type alignment
// Listing 17-38 and 17-39
#[allow(dead_code)]
fn example_merge_aligned_types() {
    fn get_messages() -> impl Stream<Item = String> {
        let (tx, rx) = trpl::channel();

        trpl::spawn_task(async move {
            let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
            for (index, message) in messages.into_iter().enumerate() {
                let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
                trpl::sleep(Duration::from_millis(time_to_sleep)).await;

                if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
                    eprintln!("Cannot send message '{message}': {send_error}");
                    break;
                }
            }
        });

        ReceiverStream::new(rx)
    }

    fn get_intervals() -> impl Stream<Item = u32> {
        let (tx, rx) = trpl::channel();

        trpl::spawn_task(async move {
            let mut count = 0;
            loop {
                trpl::sleep(Duration::from_millis(1)).await;
                count += 1;

                if let Err(send_error) = tx.send(count) {
                    eprintln!("Could not send interval {count}: {send_error}");
                    break;
                };
            }
        });

        ReceiverStream::new(rx)
    }

    trpl::run(async {
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals).take(20);
        let mut stream = pin!(merged);

        while let Some(result) = stream.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    })
}

fn main() {
    println!("Running Chapter 17-04 examples...\n");

    // Uncomment the example you want to run:

    println!("Example 1: Basic stream from iterator");
    example_stream_from_iterator();

    println!("\nExample 2: Filtering a stream");
    example_stream_filter();

    println!("\nExample 3: Basic message stream from channel");
    example_basic_message_stream();

    println!("\nExample 4: Adding timeout to stream");
    example_timeout_stream();

    println!("\nExample 5: Delayed messages with timeout");
    example_delayed_messages();

    println!("\nExample 6: Interval stream (first 10 intervals)");
    example_interval_stream();

    println!("\nExample 8: Merging streams with aligned types (throttle + take)");
    example_merge_aligned_types();
}
