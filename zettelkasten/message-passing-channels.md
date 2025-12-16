# Message Passing Channels - Producer/Consumer Coordination

*Rust's `std::sync::mpsc` channels move Advent of Code workloads through predictable producer/consumer pipelines without sharing mutable state.*

---

## 🎯 **Core Concept**

Message passing channels provide a **single-consumer queue** where any number of producers (`Sender<T>`) can transmit ownership of values to one receiving thread (`Receiver<T>`). The channel enforces ordering, backpressure, and ownership transfer automatically—when the last sender is dropped, the receiver observes `RecvError`, signaling pipeline completion. This pattern is the Rust Book Chapter 16.2 answer to "How do I connect independent threads without locks?"

## 🧠 **Mental Models**

- **Factory Conveyor Belt**: Producers drop fully prepared workpieces onto a belt (channel), and a consumer pulls them off in order without touching upstream machinery.
- **Mailboxes per Stage**: Each stage owns an inbox; only the owner reads it, but many coworkers can drop envelopes in.
- **Waterfall Pipelines**: Each stage transforms data and hands it downstream, keeping ownership flow obvious and preventing accidental reuse.

## 🔍 **Detailed Content**

### **Rust mpsc Essentials**

- `let (tx, rx) = mpsc::channel::<T>();` creates an unbounded FIFO queue.
- Cloning the `Sender<T>` spreads production across threads; the `Receiver<T>` is **not** `Clone`, so share it via `Arc<Mutex<_>>` if multiple consumers must pull from the same queue.
- Dropping every `Sender<T>` automatically closes the channel; the receiver sees `Err(RecvError)` and can exit cleanly.
- `recv()` blocks, `try_recv()` polls, and `recv_timeout()` adds bounded waiting—handy for AoC problems where a stage should break out if work dries up.
- `sync_channel(capacity)` adds bounded buffering to create natural backpressure for CPU-heavy parsers.

```rust
use std::sync::mpsc;
use std::thread;

fn simple_example() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send("parsed line #42").unwrap();
        // Sender drops here, receiver eventually sees channel closure
    });

    match rx.recv() {
        Ok(value) => println!("consumer got {value}"),
        Err(_) => println!("channel closed"),
    }
}
```

### **AoC Pipeline Pattern**

1. **Parser Stage** (single producer): Streams gigantic AoC input files line-by-line and converts them into typed work items.
2. **Worker Pool** (multi-threaded consumers): Splits heavy calculations (hashing, geometry, graph traversals) across CPU cores without touching shared data.
3. **Reducer Stage** (single consumer): Aggregates answers, emits metrics, or forwards data to subsequent puzzle parts.

```
┌────────┐   tx.send()   ┌──────────────┐   tx.send()   ┌─────────────┐
│ Parser │ ───────────▶ │ Worker Pool  │ ───────────▶ │ Aggregator  │
└────────┘   WorkItem    └──────────────┘   Partial     └─────────────┘
                   channel                   channel
```

This pattern keeps AoC solutions responsive: parsing never waits for computation, and workers can scale with `num_cpus` to chew through massive datasets like 2015 Day 4 hash mining or 2023 Day 20 pulse routing.

### **Complete Runnable Example: Concurrent Calibration Aggregator**

The snippet below shows a full AoC-style pipeline where a parser thread streams input, three worker threads compute calibration scores, and the main thread aggregates results.

```rust
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct WorkItem {
    line_no: usize,
    payload: String,
}

fn parse_line(line_no: usize, line: &str) -> Option<WorkItem> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }
    Some(WorkItem {
        line_no,
        payload: trimmed.to_string(),
    })
}

fn calibration_score(payload: &str) -> i64 {
    let digit_sum: i64 = payload
        .bytes()
        .filter(|b| b.is_ascii_digit())
        .map(|b| (b - b'0') as i64)
        .sum();
    digit_sum * payload.len() as i64
}

fn run_pipeline(input: &str) -> i64 {
    let (work_tx, work_rx) = mpsc::channel::<WorkItem>();
    let (result_tx, result_rx) = mpsc::channel::<i64>();

    let input_owned = input.to_string();
    let producer = thread::spawn(move || {
        for (line_no, line) in input_owned.lines().enumerate() {
            if let Some(item) = parse_line(line_no, line) {
                work_tx.send(item).unwrap();
            }
        }
        // Sender drops here → workers eventually see RecvError and exit
    });

    let shared_rx = Arc::new(Mutex::new(work_rx));
    let workers: Vec<_> = (0..3)
        .map(|worker_id| {
            let work_rx = Arc::clone(&shared_rx);
            let result_tx = result_tx.clone();
            thread::spawn(move || loop {
                let next = {
                    // Receiver is not Sync, so guard it behind a Mutex
                    let guard = work_rx.lock().unwrap();
                    guard.recv()
                };

                match next {
                    Ok(work) => {
                        let score = calibration_score(&work.payload);
                        println!(
                            "worker {worker_id} processed line {} → score {score}",
                            work.line_no
                        );
                        result_tx.send(score).unwrap();
                    }
                    Err(_) => break, // channel closed
                }
            })
        })
        .collect();
    drop(result_tx); // allow result_rx.iter() to finish when workers exit

    producer.join().unwrap();
    for handle in workers {
        handle.join().unwrap();
    }

    result_rx.iter().sum()
}

fn main() {
    let input = "\
depth 120
signal 7
noop
valves 3 5 8
acc -42
";

    let total = run_pipeline(input);
    println!("aggregate calibration: {total}");
}
```

This mirrors many AoC tasks (e.g., **2023 Day 20 pulse processing**) where parsing and processing can be decoupled. The receiver is wrapped in `Arc<Mutex<_>>` so multiple workers can pull work safely, and closing the last sender lets the reducers exit naturally—no shared mutable state required.

### **Diagnostics & Extensions**

- **Backpressure**: Swap to `sync_channel(n)` when parser threads should slow down instead of flooding memory with work.
- **Timeouts**: Use `recv_timeout()` for stages that must periodically publish heartbeats or checkpoints.
- **Tracing**: Include `(worker_id, timestamp)` in payloads to feed [[deterministic-debugging]] timelines.
- **Crossbeam for Fan-Out**: When you need multi-consumer semantics without `Arc<Mutex<_>>`, reach for `crossbeam-channel` (drop-in API, multiple receivers).

### **Shared State Comparison**

- When aggregation requires mutation of shared structures (scoreboards, caches), switch to the lock-based patterns cataloged in [[shared-state-concurrency]] while keeping channel boundaries between stages.

### **Trait Safety Checklist**

- Before sending custom AoC structs through the pipeline, run the static assertions captured in [[sync-send-traits]] to confirm every payload remains `Send` and shared references stay `Sync`.

## 💡 **Key Takeaways**

- Message passing eliminates whole classes of data races by **moving** ownership instead of sharing mutable state.
- Dropping every `Sender<T>` is the canonical way to shut down worker pools—no sentinel messages required.
- Channels compose naturally into **parse → process → aggregate** pipelines, perfect for AoC workloads.
- `Arc<Mutex<Receiver<T>>>` is the minimal ergonomic pattern for multi-threaded consumers using the standard library.
- Instrumenting channel stages (counts, latency) surfaces bottlenecks faster than digging through shared-state bugs.

## 🔗 **Integration Points**

### **Builds On**

- [[rust-threading-basics]] - Spawning threads safely and joining for completion
- [[ownership]] - `Sender<T>` transfers ownership instead of cloning data by default
- [[Send and Sync Deep Dive]] - Why `Sender` is `Clone`/`Send` and `Receiver` is `Send` but not `Sync`
- [[rust_book/rust-book-ch16]] - Official resource for Chapter 16.2 message passing

### **Enables**

- [[thread-pool-pattern]] - MPSC channels distribute jobs to fixed worker threads
- [[AoC Pattern Library]] - Adds producer/consumer pipeline recipes to the catalog
- [[AoC Integration]] - Builds the concurrency strand of the three-track workflow
- [[deterministic-debugging]] - Channel logging provides reproducible event streams for tricky bugs

### **Related Concepts**

- [[Divide and Conquer]] - Split workloads before feeding them into channels
- [[Performance Patterns]] - Backpressure, batching, and throughput tuning
- [[string-processing-patterns]] - Typical AoC parsing stage that feeds the channel

---

*Tags: #concurrency #pattern #aoc #rust-book #intermediate*

*Links: [[zettel-index]] | [[AoC Patterns MOC]] | [[rust-threading-basics]] | [[AoC Integration]] | [[deterministic-debugging]] | [[shared-state-concurrency]] | [[sync-send-traits]]*
