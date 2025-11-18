// Example: Producer-Consumer Pattern
// Classic concurrent pattern using channels

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Producer-Consumer Pattern ===\n");

    #[derive(Debug, Clone)]
    struct WorkItem {
        id: usize,
        data: String,
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    struct ProcessedItem {
        id: usize,
        result: String,
    }

    // Create two channels: raw -> processed
    let (tx_raw, rx_raw) = mpsc::channel();
    let (tx_processed, rx_processed) = mpsc::channel();

    // Wrap receiver in Arc<Mutex<>> for sharing across workers
    let rx_raw = Arc::new(Mutex::new(rx_raw));

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
        // tx_raw is dropped here, closing the channel for workers
    });

    // Consumer threads (workers)
    let num_workers = 3;
    let mut workers = vec![];

    for worker_id in 0..num_workers {
        let rx_raw_clone = Arc::clone(&rx_raw);
        let tx_processed_clone = tx_processed.clone();

        let worker = thread::spawn(move || {
            println!("Worker {} starting...", worker_id);
            
            // Loop until the channel is empty and disconnected
            while let Ok(item) = rx_raw_clone.lock().unwrap().recv() {
                println!("  Worker {} processing item {}", worker_id, item.id);
                thread::sleep(Duration::from_millis(200)); // Simulate work
                
                let processed = ProcessedItem {
                    id: item.id,
                    result: format!("{}_processed_by_worker_{}", item.data, worker_id),
                };
                
                tx_processed_clone.send(processed).unwrap();
            }
            
            println!("Worker {} finished!", worker_id);
        });
        
        workers.push(worker);
    }

    // Drop the main thread's tx_processed so the collector's channel can close
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
    results.iter().for_each(|r| println!("- {:?}", r));
}
