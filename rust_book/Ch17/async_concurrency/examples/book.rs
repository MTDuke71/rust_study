// Chapter 17.2 - Applying Concurrency with Async
// Examples from: https://doc.rust-lang.org/stable/book/ch17-02-concurrency-with-async.html

use std::time::Duration;

// Listing 17-6: Creating a new task to print one thing while the main task prints something else
#[allow(dead_code)]
fn listing_17_6() {
    println!("\n{}", "=".repeat(60));
    println!("Listing 17-6: Creating a new task with spawn_task");
    println!("{}\n", "=".repeat(60));
    trpl::run(async {
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });
}

// Listing 17-7: Using await with a join handle to run a task to completion
#[allow(dead_code)]
fn listing_17_7() {
    println!("\n{}", "=".repeat(60));
    println!("Listing 17-7: Using await with a join handle");
    println!("{}\n", "=".repeat(60));
    trpl::run(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        handle.await.unwrap();
    });
}

// Listing 17-8: Using trpl::join to await two anonymous futures
#[allow(dead_code)]
fn listing_17_8() {
    println!("\n{}", "=".repeat(60));
    println!("Listing 17-8: Using trpl::join with anonymous futures");
    println!("{}\n", "=".repeat(60));
    trpl::run(async {
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
    });
}

// Listing 17-9: Creating an async channel and assigning the two halves to tx and rx
#[allow(dead_code)]
fn listing_17_9() {
    println!("\n{}", "=".repeat(60));
    println!("Listing 17-9: Creating an async channel");
    println!("{}\n", "=".repeat(60));
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let val = String::from("hi");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("received '{received}'");
    });
}

// Listing 17-10: Sending and receiving multiple messages over the async channel
#[allow(dead_code)]
fn listing_17_10() {
    println!("\n{}", "=".repeat(60));
    println!("Listing 17-10: Multiple messages (WARNING: infinite loop!)");
    println!("{}\n", "=".repeat(60));
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
    });
}

// Listing 17-11: Separating send and recv into their own async blocks
#[allow(dead_code)]
fn listing_17_11() {
    println!("\n{}", "=".repeat(60));
    println!("Listing 17-11: Separate async blocks (WARNING: infinite loop!)");
    println!("{}\n", "=".repeat(60));
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx_fut = async {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
    });
}

// Listing 17-12: Using async move to correctly shut down when complete
#[allow(dead_code)]
fn listing_17_12() {
    println!("\n{}", "=".repeat(60));
    println!("Listing 17-12: Using async move (shuts down correctly)");
    println!("{}\n", "=".repeat(60));
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
    });
}

// Listing 17-13: Using multiple producers with async blocks
#[allow(dead_code)]
fn listing_17_13() {
    println!("\n{}", "=".repeat(60));
    println!("Listing 17-13: Multiple producers with trpl::join3");
    println!("{}\n", "=".repeat(60));
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

        trpl::join3(tx1_fut, tx_fut, rx_fut).await;
    });
}

fn main() {
    // Run whichever listing you want to explore
    // Uncomment the one you want to run:

    listing_17_6();
    listing_17_7();
    listing_17_8();
    listing_17_9();
    //listing_17_10();
    //listing_17_11();
    listing_17_12();
    listing_17_13();
}
