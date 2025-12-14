/// ThreadPool implementation for concurrent request handling
/// 
/// This is the core abstraction used in Chapter 21 to manage worker threads
/// and distribute work across them using message passing.

use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

/// A thread pool that maintains a fixed number of worker threads
/// 
/// # Examples
/// 
/// ```
/// use rust_book_ch21::ThreadPool;
/// 
/// let pool = ThreadPool::new(4);
/// 
/// pool.execute(|| {
///     println!("Job executed by worker thread");
/// });
/// ```
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Message>>,
}

/// Type alias for a heap-allocated, thread-safe closure
type Job = Box<dyn FnOnce() + Send + 'static>;

/// Messages sent to worker threads
#[allow(dead_code)]
enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        
        // Wrap receiver in Arc<Mutex<>> for shared ownership across threads
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Execute a closure on a worker thread
    /// 
    /// # Arguments
    /// 
    /// * `f` - A closure that takes no arguments and returns nothing
    /// 
    /// # Type Bounds
    /// 
    /// * `FnOnce()` - Closure called once
    /// * `Send` - Can be sent across thread boundaries
    /// * `'static` - No borrowed references (lives for entire program)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_book_ch21::ThreadPool;
    /// 
    /// let pool = ThreadPool::new(2);
    /// 
    /// pool.execute(|| {
    ///     println!("Executing job");
    /// });
    /// ```
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender
            .as_ref()
            .unwrap()
            .send(Message::NewJob(job))
            .unwrap();
    }
}

/// Drop implementation for graceful shutdown
/// 
/// Sends Terminate message to all workers and waits for them to finish
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Drop sender to close channel (workers will exit when receiving None)
        drop(self.sender.take());

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // Take ownership of thread handle and join
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

/// A worker thread that processes jobs from the thread pool
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Create a new worker thread
    /// 
    /// # Arguments
    /// 
    /// * `id` - Unique identifier for this worker
    /// * `receiver` - Shared receiver wrapped in Arc<Mutex<>>
    /// 
    /// # Design Notes
    /// 
    /// - Uses `Arc` for shared ownership across threads
    /// - Uses `Mutex` for exclusive access to receiver
    /// - Loops continuously waiting for jobs
    /// - Breaks loop on Terminate message
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // Lock mutex, receive message, then release lock
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(Message::NewJob(job)) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Ok(Message::Terminate) => {
                    println!("Worker {id} was told to terminate.");
                    break;
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn test_thread_pool_creation() {
        let pool = ThreadPool::new(4);
        assert_eq!(pool.workers.len(), 4);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_thread_pool_zero_size_panics() {
        ThreadPool::new(0);
    }

    #[test]
    fn test_execute_job() {
        let pool = ThreadPool::new(2);
        let counter = Arc::new(AtomicUsize::new(0));
        
        let counter_clone = Arc::clone(&counter);
        pool.execute(move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        // Wait for job to complete
        thread::sleep(Duration::from_millis(100));
        
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_multiple_jobs() {
        let pool = ThreadPool::new(4);
        let counter = Arc::new(AtomicUsize::new(0));

        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            pool.execute(move || {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            });
        }

        // Wait for all jobs to complete
        thread::sleep(Duration::from_millis(200));

        assert_eq!(counter.load(Ordering::SeqCst), 10);
    }

    #[test]
    fn test_graceful_shutdown() {
        let pool = ThreadPool::new(2);
        
        pool.execute(|| {
            thread::sleep(Duration::from_millis(50));
        });
        
        // Drop happens here - should wait for job to finish
        drop(pool);
        
        // If we reach here, shutdown was successful
        assert!(true);
    }
}
