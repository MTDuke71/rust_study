// Example: Producer-Consumer Pattern
// Classic concurrent pattern using channels

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Producer-Consumer Pattern ===\n");

    #[derive(Debug, Clone)]
    struct WorkItem {
        id: usize,
        data: String,
    }

    #[derive(Debug)]
    struct ProcessedItem {
        id: usize,
        result: String,
    }

    // Create two channels: raw -> processed
    let (tx_raw, rx_raw) = mpsc::channel();
    let (tx_processed, rx_processed) = mpsc::channel();

    // Producer thread
    let producer = thread::spawn(move || {
        println!("Producer starting...");
        for i in 1..=10 {
            let item = WorkItem {
                id: i,
                data: format!("raw_data_{}", i),
            };
            println!("  Producing: {:?}", item);
            tx_raw.send(item).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
        println!("Producer finished!");
    });

    // Consumer threads (workers)
    let num_workers = 3;
    let mut workers = vec![];

    for worker_id in 0..num_workers {
        let rx_raw = rx_raw.clone();
        let tx_processed = tx_processed.clone();

        let worker = thread::spawn(move || {
            println!("Worker {} starting...", worker_id);
            
            // Process items until channel closes
            for item in rx_raw {
                println!("  Worker {} processing item {}", worker_id, item.id);
                thread::sleep(Duration::from_millis(200)); // Simulate work
                
                let processed = ProcessedItem {
                    id: item.id,
                    result: format!("{}_processed_by_worker_{}", item.data, worker_id),
                };
                
                tx_processed.send(processed).unwrap();
            }
            
            println!("Worker {} finished!", worker_id);
        });
        
        workers.push(worker);
    }

    // Drop the extra tx_processed so channel can close
    drop(tx_processed);

    // Result collector thread
    let collector = thread::spawn(move || {
        println!("Collector starting...");
        let mut results = vec![];
        
        for processed in rx_processed {
            println!("  Collected: {:?}", processed);
            results.push(processed);
        }
        
        println!("Collector finished!");
        results
    });

    // Wait for all threads
    producer.join().unwrap();
    for worker in workers {
        worker.join().unwrap();
    }
    let results = collector.join().unwrap();

    println!("\n=== Summary ===");
    println!("Total processed items: {}", results.len());
    println!("Results: {:#?}", results);
}
