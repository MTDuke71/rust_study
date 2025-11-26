# Rust Threading Basics - Concurrent Execution Fundamentals

*Foundation of concurrent programming in Rust using `std::thread` for spawning threads, `JoinHandle` for synchronization, and thread safety guarantees through the ownership system - with practical Advent of Code parallelization examples.*

---

## 🎯 **Core Concept**

**Threading in Rust** enables concurrent execution by spawning multiple threads of execution that can run simultaneously on multi-core processors. Unlike many languages where threading introduces data race vulnerabilities, Rust's ownership system **guarantees thread safety at compile time** through the `Send` and `Sync` traits.

**Key Components**:

- **`std::thread::spawn`**: Creates new OS threads that execute closures concurrently
- **`JoinHandle<T>`**: Handle for waiting on thread completion and retrieving results
- **`Send` trait**: Types safe to transfer ownership between threads
- **`Sync` trait**: Types safe to share references between threads
- **Thread safety**: Compiler prevents data races through borrow checker

**Why Threading Matters for AoC**:

1. **Computational parallelism**: Divide-and-conquer on multi-core systems (prime checking, search spaces)
2. **Independent subtasks**: Parse input while processing previous data
3. **Performance scaling**: 4-core CPU = potential 4x speedup for CPU-bound problems
4. **Real-world skill**: Threading fundamentals apply to production systems

---

## 🧠 **Mental Models**

### **The Threading Lifecycle**

```
Main Thread                    Spawned Thread
    │                               │
    │  spawn(|| { ... })           │
    ├──────────────────────────────>│  Thread created
    │                               │
    │                               │  Executes closure
    │  Other work...                │  Independent execution
    │                               │
    │  join_handle.join()           │
    ├──────────────────────────────>│  Wait for completion
    │                               │
    │<────────────────────────────┤  Return result
    │  Continue with result         X  Thread terminates
    ▼
```

### **Ownership Transfer vs Shared State**

```
Ownership Transfer (move):
Main Thread          Worker Thread
    │                     │
    │  move || { data }   │
    ├────[data]──────────>│  Ownership transferred
    │  (no access)        │  Exclusive ownership
                          │  Mutate freely
                          │
                          ▼

Shared State (Arc<Mutex<T>>):
Thread 1            Shared Memory         Thread 2
    │                     │                   │
    │  lock()            │                   │
    ├──────────────────> │ <──────────────┤
    │  Exclusive access  │  Waiting...       │
    │  unlock()          │                   │
    │ <──────────────────┼──────────────────┤  lock()
    │                     │ <──────────────┤  Exclusive access
```

### **Thread Safety Spectrum**

```
i32, String, Vec<T>         Arc<T>              Arc<Mutex<T>>
      ↓                       ↓                       ↓
 Send (transfer)      Send + Sync (share)    Send + Sync + Interior Mutability
 Move between         Read-only sharing      Shared mutable state
 threads              Reference counting     Mutual exclusion
```

---

## 🔍 **Basic Thread Operations**

### **1. Spawning Threads**

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // Spawn a new thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    // Main thread continues independently
    for i in 1..5 {
        println!("main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    // Wait for spawned thread to finish
    handle.join().unwrap();
}
```

**Output** (interleaved execution):

```
main thread: 1
spawned thread: 1
main thread: 2
spawned thread: 2
...
```

### **2. Moving Data into Threads**

```rust
use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    
    // WRONG: Cannot borrow `data` because it might outlive the thread
    // let handle = thread::spawn(|| {
    //     println!("length: {}", data.len());  // ❌ Compile error
    // });
    
    // CORRECT: Transfer ownership with `move`
    let handle = thread::spawn(move || {
        println!("length: {}", data.len());  // ✅ Owns data
        data.iter().sum::<i32>()
    });
    
    // data is no longer accessible here
    // println!("{:?}", data);  // ❌ Compile error: value moved
    
    let sum = handle.join().unwrap();
    println!("sum: {}", sum);  // sum: 15
}
```

### **3. Returning Values from Threads**

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        // Perform expensive computation
        let result = (0..1_000_000).sum::<u64>();
        result  // Return value through JoinHandle
    });
    
    // Do other work while thread computes
    println!("Waiting for result...");
    
    // join() returns Result<T, E> where T is the closure's return type
    match handle.join() {
        Ok(result) => println!("Thread result: {}", result),
        Err(e) => eprintln!("Thread panicked: {:?}", e),
    }
}
```

### **4. Multiple Threads with Collected Results**

```rust
use std::thread;

fn main() {
    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                i * i  // Each thread computes square
            })
        })
        .collect();
    
    // Collect results from all threads
    let results: Vec<_> = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect();
    
    println!("Squares: {:?}", results);
    // [0, 1, 4, 9, 16, 25, 36, 49, 64, 81]
}
```

---

## 🎯 **AoC Parallelization Patterns**

### **Pattern 1: Divide-and-Conquer (Range Splitting)**

**Problem**: Check how many numbers in range 1-1,000,000 are prime.

```rust
use std::thread;

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    
    let sqrt = (n as f64).sqrt() as u64;
    (3..=sqrt).step_by(2).all(|i| n % i != 0)
}

fn count_primes_sequential(start: u64, end: u64) -> usize {
    (start..=end).filter(|&n| is_prime(n)).count()
}

fn count_primes_parallel(start: u64, end: u64, num_threads: usize) -> usize {
    let range_size = (end - start + 1) / num_threads as u64;
    
    let handles: Vec<_> = (0..num_threads)
        .map(|i| {
            let thread_start = start + i as u64 * range_size;
            let thread_end = if i == num_threads - 1 {
                end  // Last thread takes remainder
            } else {
                thread_start + range_size - 1
            };
            
            thread::spawn(move || {
                count_primes_sequential(thread_start, thread_end)
            })
        })
        .collect();
    
    handles.into_iter()
        .map(|h| h.join().unwrap())
        .sum()
}

fn main() {
    use std::time::Instant;
    
    let start = Instant::now();
    let count = count_primes_parallel(1, 100_000, 4);
    let duration = start.elapsed();
    
    println!("Found {} primes in {:?}", count, duration);
    // 4-core speedup: ~3.5x faster than sequential
}
```

**AoC Application**: Day 4 (2015) - MD5 hash mining, Day 17 (2015) - Container combinations

### **Pattern 2: Independent Task Processing**

**Problem**: Process multiple independent inputs (like checking password validity rules).

```rust
use std::thread;

#[derive(Debug)]
struct PasswordPolicy {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl PasswordPolicy {
    fn is_valid(&self) -> bool {
        let count = self.password.chars()
            .filter(|&c| c == self.letter)
            .count();
        count >= self.min && count <= self.max
    }
}

fn validate_passwords_parallel(policies: Vec<PasswordPolicy>, num_threads: usize) -> usize {
    let chunk_size = (policies.len() + num_threads - 1) / num_threads;
    
    let handles: Vec<_> = policies
        .chunks(chunk_size)
        .map(|chunk| {
            let chunk = chunk.to_vec();  // Clone for move
            thread::spawn(move || {
                chunk.iter().filter(|p| p.is_valid()).count()
            })
        })
        .collect();
    
    handles.into_iter()
        .map(|h| h.join().unwrap())
        .sum()
}

fn main() {
    let policies = vec![
        PasswordPolicy { min: 1, max: 3, letter: 'a', password: "abcde".to_string() },
        PasswordPolicy { min: 1, max: 3, letter: 'b', password: "cdefg".to_string() },
        PasswordPolicy { min: 2, max: 9, letter: 'c', password: "ccccccccc".to_string() },
    ];
    
    let valid = validate_passwords_parallel(policies, 2);
    println!("Valid passwords: {}", valid);  // 2
}
```

**AoC Application**: Day 2 (2020) - Password validation, Day 4 (2020) - Passport processing

### **Pattern 3: Search Space Exploration**

**Problem**: Find all permutations or combinations that satisfy constraints.

```rust
use std::thread;
use std::sync::{Arc, Mutex};

fn find_combinations(
    target: i32,
    numbers: &[i32],
    num_threads: usize
) -> Vec<Vec<i32>> {
    let results = Arc::new(Mutex::new(Vec::new()));
    let numbers = Arc::new(numbers.to_vec());
    
    let handles: Vec<_> = (0..num_threads)
        .map(|thread_id| {
            let results = Arc::clone(&results);
            let numbers = Arc::clone(&numbers);
            
            thread::spawn(move || {
                // Each thread explores different starting points
                for i in (thread_id..numbers.len()).step_by(num_threads) {
                    for j in (i+1)..numbers.len() {
                        if numbers[i] + numbers[j] == target {
                            let mut res = results.lock().unwrap();
                            res.push(vec![numbers[i], numbers[j]]);
                        }
                    }
                }
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    Arc::try_unwrap(results).unwrap().into_inner().unwrap()
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let combinations = find_combinations(10, &numbers, 4);
    println!("Found {} combinations: {:?}", combinations.len(), combinations);
    // [(1,9), (2,8), (3,7), (4,6)]
}
```

**AoC Application**: Day 1 (2020) - Find two/three numbers that sum to 2020, Day 9 (2020) - Find invalid number in sequence

### **Pattern 4: Map-Reduce Pipeline**

**Problem**: Transform data in parallel, then combine results.

```rust
use std::thread;

fn parallel_map_reduce<T, U, F, R>(
    data: Vec<T>,
    num_threads: usize,
    map_fn: F,
    reduce_fn: R,
) -> U
where
    T: Send + 'static,
    U: Send + 'static,
    F: Fn(Vec<T>) -> U + Send + 'static + Copy,
    R: Fn(U, U) -> U,
{
    let chunk_size = (data.len() + num_threads - 1) / num_threads;
    
    let handles: Vec<_> = data
        .chunks(chunk_size)
        .map(|chunk| {
            let chunk = chunk.to_vec();
            thread::spawn(move || map_fn(chunk))
        })
        .collect();
    
    let results: Vec<U> = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect();
    
    results.into_iter().reduce(reduce_fn).unwrap()
}

fn main() {
    let numbers: Vec<i32> = (1..=1000).collect();
    
    // Map: sum each chunk, Reduce: sum the sums
    let total = parallel_map_reduce(
        numbers,
        4,
        |chunk| chunk.iter().sum::<i32>(),
        |a, b| a + b,
    );
    
    println!("Total sum: {}", total);  // 500500
}
```

**AoC Application**: Day 3 (2021) - Binary diagnostic (count bits), Day 8 (2021) - Seven-segment display (pattern matching)

---

## 🚨 **Common Threading Pitfalls**

### **Pitfall 1: Forgetting to Join Threads**

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // ❌ BAD: Thread may not complete before program exits
    thread::spawn(|| {
        println!("Starting work...");
        thread::sleep(Duration::from_secs(2));
        println!("Work done!");  // May never print!
    });
    
    println!("Main thread exits");
    // Program terminates, spawned thread killed
}

fn main_correct() {
    // ✅ GOOD: Always join to ensure completion
    let handle = thread::spawn(|| {
        println!("Starting work...");
        thread::sleep(Duration::from_secs(2));
        println!("Work done!");
    });
    
    handle.join().unwrap();  // Wait for thread
    println!("Main thread exits");
}
```

### **Pitfall 2: Data Races Through Shared Mutable State**

```rust
use std::thread;

fn main() {
    let mut counter = 0;
    
    // ❌ COMPILE ERROR: Cannot share mutable reference
    // let handle = thread::spawn(|| {
    //     counter += 1;  // Error: cannot borrow as mutable
    // });
    
    // ✅ SOLUTION: Use Arc<Mutex<T>> (covered in Week 8 Day 52)
    // See [[shared-state-concurrency]] for proper patterns
}
```

**Rust prevents data races at compile time!**

### **Pitfall 3: Thread Overhead Exceeding Benefits**

```rust
use std::thread;
use std::time::Instant;

fn small_task(n: u64) -> u64 {
    n * 2  // Trivial computation
}

fn main() {
    let data: Vec<u64> = (0..100).collect();
    
    // ❌ BAD: Thread creation overhead >> computation time
    let start = Instant::now();
    let handles: Vec<_> = data.iter()
        .map(|&n| thread::spawn(move || small_task(n)))
        .collect();
    let _results: Vec<_> = handles.into_iter()
        .map(|h| h.join().unwrap())
        .collect();
    println!("Parallel (100 threads): {:?}", start.elapsed());
    // ~5ms (overhead dominates)
    
    // ✅ GOOD: Batch small tasks per thread
    let start = Instant::now();
    let chunk_size = 25;  // 4 threads for 100 items
    let handles: Vec<_> = data.chunks(chunk_size)
        .map(|chunk| {
            let chunk = chunk.to_vec();
            thread::spawn(move || {
                chunk.iter().map(|&n| small_task(n)).collect::<Vec<_>>()
            })
        })
        .collect();
    let _results: Vec<_> = handles.into_iter()
        .flat_map(|h| h.join().unwrap())
        .collect();
    println!("Parallel (4 threads): {:?}", start.elapsed());
    // ~0.2ms (efficient batching)
}
```

**Rule of thumb**: Each thread should do at least 1ms of work to justify overhead.

---

## 💡 **Key Takeaways**

1. **`std::thread::spawn`** creates OS threads that run closures concurrently with the main thread
2. **`JoinHandle::join()`** blocks until thread completes and returns its result or panic
3. **`move` closures** transfer ownership of captured variables to the thread
4. **Rust prevents data races** through `Send`/`Sync` traits enforced by the borrow checker
5. **AoC parallelization** works best for CPU-bound problems with independent subtasks (prime checking, password validation, search spaces)
6. **Thread overhead matters**: Batch small tasks to avoid thread creation cost exceeding computation

---

## 🔗 **Integration Points**

### **Builds On**

- [[ownership-fundamentals]] - Understanding move semantics for thread data transfer
- [[closures-rust]] - Threads execute closures, `move` keyword captures environment
- [[error-handling-patterns]] - `JoinHandle::join()` returns `Result<T, Box<dyn Any>>`
- [[traits]] - `Send` and `Sync` marker traits enable thread safety

### **Enables**

- [[message-passing-channels]] - Week 8 Day 51, mpsc for thread communication (Monday Nov 17)
- [[shared-state-concurrency]] - Week 8 Day 52, Arc<Mutex<T>> for shared mutable state (Tuesday Nov 18)
- [[sync-send-traits]] - Week 8 Day 53, deep dive into thread safety markers (Wednesday Nov 19)
- [[async-await-basics]] - Week 8 Day 54, async as alternative to thread-based concurrency (Thursday Nov 20)
- [[parallel-iterators-rayon]] - Week 8 Day 55, rayon for ergonomic data parallelism (Friday Nov 21)

### **Related Concepts**

- [[handles-resource-abstraction]] - Understanding thread handles and the handle pattern
- [[concurrency-vs-parallelism]] - Concurrency (task switching) vs parallelism (simultaneous execution)
- [[cpu-bound-vs-io-bound]] - Threading best for CPU-bound, async better for I/O-bound
- [[rust-book-ch16-1]] - Official Rust Book coverage of threading
- [[aoc-optimization-strategies]] - When and how to parallelize AoC solutions

### **Mission Applications**

- **Mission 11** (Dynamic Programming): Parallelize independent DP subproblems
- **Mission 12** (Graph Algorithms): Parallel BFS/DFS on disconnected components
- **Future Advanced Missions**: Thread pools, work-stealing schedulers

### **AoC Problem Integration**

- **2015 Day 4**: MD5 hash mining with parallel range searching
- **2015 Day 17**: Container combinations with parallel subset generation
- **2020 Day 1**: Find sum pairs/triplets with parallel chunk processing
- **2020 Day 2**: Password validation with parallel batch processing
- **2021 Day 3**: Binary diagnostic bit counting with map-reduce pattern

---

## 📚 **Learning Path**

### **Beginner (Current - Week 8 Day 50)**

1. ✅ Spawn threads with `thread::spawn(|| { })`
2. ✅ Move data into threads with `move` closures
3. ✅ Wait for threads with `join()` and retrieve results
4. ✅ Parallelize simple AoC problems (independent tasks)

### **Intermediate (Week 8 Days 51-53)**

1. Use `mpsc` channels for thread communication (Monday)
2. Share mutable state with `Arc<Mutex<T>>` (Tuesday)
3. Understand `Send`/`Sync` traits for custom types (Wednesday)

### **Advanced (Week 8 Days 54-56)**

1. Compare threads vs async for different workloads (Thursday)
2. Use `rayon` for ergonomic data parallelism (Friday)
3. Build thread-safe systems with proper synchronization (Saturday)

### **Expert (Future Topics)**

1. Implement custom thread pools with work stealing
2. Lock-free concurrent data structures with atomics
3. Real-time constraints and thread priority management

---

## 🔬 **Performance Benchmarking**

```rust
use std::thread;
use std::time::Instant;

fn benchmark_threading() {
    let data: Vec<u64> = (1..=1_000_000).collect();
    
    // Sequential baseline
    let start = Instant::now();
    let sum_seq: u64 = data.iter().map(|&n| n * n).sum();
    let seq_time = start.elapsed();
    
    // Parallel with 4 threads
    let start = Instant::now();
    let chunk_size = data.len() / 4;
    let handles: Vec<_> = data.chunks(chunk_size)
        .map(|chunk| {
            let chunk = chunk.to_vec();
            thread::spawn(move || {
                chunk.iter().map(|&n| n * n).sum::<u64>()
            })
        })
        .collect();
    let sum_par: u64 = handles.into_iter()
        .map(|h| h.join().unwrap())
        .sum();
    let par_time = start.elapsed();
    
    assert_eq!(sum_seq, sum_par);
    println!("Sequential: {:?}", seq_time);
    println!("Parallel (4 threads): {:?}", par_time);
    println!("Speedup: {:.2}x", seq_time.as_secs_f64() / par_time.as_secs_f64());
}
```

**Expected results on 4-core CPU**:

- Sequential: ~8ms
- Parallel: ~2.5ms
- Speedup: ~3.2x (not 4x due to overhead and memory bandwidth)

---

## 🛠️ **Complete Runnable Example: AoC Prime Search**

```rust
use std::thread;
use std::time::Instant;

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    
    let sqrt = (n as f64).sqrt() as u64;
    (3..=sqrt).step_by(2).all(|i| n % i != 0)
}

fn find_primes_sequential(start: u64, end: u64) -> Vec<u64> {
    (start..=end).filter(|&n| is_prime(n)).collect()
}

fn find_primes_parallel(start: u64, end: u64, num_threads: usize) -> Vec<u64> {
    let range_size = (end - start + 1) / num_threads as u64;
    
    let handles: Vec<_> = (0..num_threads)
        .map(|i| {
            let thread_start = start + i as u64 * range_size;
            let thread_end = if i == num_threads - 1 {
                end
            } else {
                thread_start + range_size - 1
            };
            
            thread::spawn(move || {
                find_primes_sequential(thread_start, thread_end)
            })
        })
        .collect();
    
    handles.into_iter()
        .flat_map(|h| h.join().unwrap())
        .collect()
}

fn main() {
    let start = 1;
    let end = 100_000;
    
    // Sequential
    let timer = Instant::now();
    let primes_seq = find_primes_sequential(start, end);
    let seq_time = timer.elapsed();
    
    // Parallel (4 threads)
    let timer = Instant::now();
    let mut primes_par = find_primes_parallel(start, end, 4);
    let par_time = timer.elapsed();
    
    primes_par.sort();  // Threads return out of order
    
    println!("Found {} primes", primes_seq.len());
    println!("Sequential: {:?}", seq_time);
    println!("Parallel:   {:?}", par_time);
    println!("Speedup:    {:.2}x", seq_time.as_secs_f64() / par_time.as_secs_f64());
    
    assert_eq!(primes_seq, primes_par);
}

// Sample Output:
// Found 9592 primes
// Sequential: 145.2ms
// Parallel:   42.7ms
// Speedup:    3.40x
```

---

*Tags: #concurrency #threading #std-thread #parallelism #aoc-optimization #cpu-bound #performance #rust-book-ch16 #week8 #beginner #practical*

*Links: [[zettel-index]] | [[concurrency-fundamentals]] | [[message-passing-channels]] | [[shared-state-concurrency]] | [[sync-send-traits]] | [[ownership-fundamentals]] | [[closures-rust]] | [[aoc-optimization-strategies]] | [[rust-book-ch16-1]]*
