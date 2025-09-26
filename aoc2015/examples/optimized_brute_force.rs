//! # Optimized Brute Force Approaches for Day 4
//! 
//! While brute force is required, there are ways to make it faster

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("🚀 Optimized Brute Force Strategies");
    println!("===================================\n");
    
    let secret = "test";
    let target = "000"; // 3 zeros for demo speed
    
    compare_sequential_vs_parallel(secret, target);
    show_memory_optimizations();
    discuss_hardware_optimizations();
}

/// Compare sequential vs parallel brute force
fn compare_sequential_vs_parallel(secret: &str, target: &str) {
    println!("🔄 Sequential vs Parallel Processing");
    println!("====================================");
    
    // Sequential approach
    let start = std::time::Instant::now();
    let result1 = sequential_search(secret, target, 50000);
    let time1 = start.elapsed();
    
    // Multi-threaded approach  
    let start = std::time::Instant::now();
    let result2 = parallel_search(secret, target, 50000);
    let time2 = start.elapsed();
    
    println!("Sequential: {:?} in {:?}", result1, time1);
    println!("Parallel:   {:?} in {:?}", result2, time2);
    
    if let (Some(_), Some(_)) = (result1, result2) {
        let speedup = time1.as_nanos() as f64 / time2.as_nanos() as f64;
        println!("Speedup: {:.2}x", speedup);
    }
    
    println!();
}

fn sequential_search(secret: &str, prefix: &str, max: u32) -> Option<u32> {
    for i in 1..=max {
        let hash = format!("{:x}", md5::compute(format!("{secret}{i}")));
        if hash.starts_with(prefix) {
            return Some(i);
        }
    }
    None
}

fn parallel_search(secret: &str, prefix: &str, max: u32) -> Option<u32> {
    let num_threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
    let chunk_size = max / num_threads as u32;
    let result = Arc::new(Mutex::new(None));
    let found = Arc::new(std::sync::atomic::AtomicBool::new(false));
    
    let handles: Vec<_> = (0..num_threads).map(|thread_id| {
        let secret = secret.to_string();
        let prefix = prefix.to_string();
        let result = Arc::clone(&result);
        let found = Arc::clone(&found);
        
        thread::spawn(move || {
            let start = thread_id as u32 * chunk_size + 1;
            let end = if thread_id == num_threads - 1 { max } else { (thread_id as u32 + 1) * chunk_size };
            
            for i in start..=end {
                if found.load(std::sync::atomic::Ordering::Relaxed) {
                    break; // Another thread found it
                }
                
                let hash = format!("{:x}", md5::compute(format!("{secret}{i}")));
                if hash.starts_with(&prefix) {
                    let mut result = result.lock().unwrap();
                    if result.is_none() {
                        *result = Some(i);
                        found.store(true, std::sync::atomic::Ordering::Relaxed);
                    }
                    break;
                }
            }
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let result = result.lock().unwrap();
    *result
}

fn show_memory_optimizations() {
    println!("💾 Memory Optimization Strategies");
    println!("=================================");
    
    println!("1. String allocation optimization:");
    println!("   ❌ Bad:  format!(\"{{secret}}{{i}}\") every time");
    println!("   ✅ Good: Reuse String buffer, use push_str()");
    
    println!("\n2. Hash formatting optimization:");
    println!("   ❌ Bad:  format!(\"{{:x}}\", hash) creates new String");
    println!("   ✅ Good: Write directly to pre-allocated buffer");
    
    println!("\n3. Early termination:");
    println!("   ✅ Good: Check prefix as soon as enough bytes computed");
    
    // Demonstrate optimized version
    let secret = "demo";
    let start = std::time::Instant::now();
    let result = optimized_search(secret, "00", 10000);
    let time = start.elapsed();
    
    println!("\nOptimized search result: {:?} in {:?}", result, time);
    println!();
}

fn optimized_search(secret: &str, prefix: &str, max: u32) -> Option<u32> {
    let mut input_buffer = String::with_capacity(secret.len() + 10);
    input_buffer.push_str(secret);
    let base_len = input_buffer.len();
    
    for i in 1..=max {
        // Reuse buffer instead of allocating new string
        input_buffer.truncate(base_len);
        input_buffer.push_str(&i.to_string());
        
        let hash = format!("{:x}", md5::compute(&input_buffer));
        if hash.starts_with(prefix) {
            return Some(i);
        }
    }
    None
}

fn discuss_hardware_optimizations() {
    println!("⚡ Hardware-Level Optimizations");
    println!("==============================");
    
    println!("1. **CPU Features**:");
    println!("   - Use SIMD instructions for parallel MD5");
    println!("   - Optimize for specific CPU architectures");
    println!("   - Use all available cores efficiently");
    
    println!("\n2. **GPU Computing** (CUDA/OpenCL):");
    println!("   - Thousands of parallel threads");
    println!("   - Specialized MD5 implementations");
    println!("   - Can be 100x+ faster than CPU");
    
    println!("\n3. **Custom Hardware** (ASIC/FPGA):");
    println!("   - Bitcoin miners use this approach");
    println!("   - Dedicated MD5/SHA256 chips");
    println!("   - Ultimate performance but expensive");
    
    println!("\n4. **Memory Hierarchy**:");
    println!("   - Keep data in CPU cache");
    println!("   - Minimize memory allocations");
    println!("   - Use stack allocation where possible");
    
    println!("\n💡 Key Insight: All still brute force, just faster!");
    
    println!("\n📊 Realistic Performance Expectations:");
    println!("   - Modern CPU: ~1M hashes/second");
    println!("   - Multi-core CPU: ~4-8M hashes/second"); 
    println!("   - High-end GPU: ~100M+ hashes/second");
    println!("   - Custom ASIC: ~1B+ hashes/second");
    
    println!("\n🎯 For AoC Day 4:");
    println!("   - Part 1 (5 zeros): ~1M attempts → 1 second on CPU");
    println!("   - Part 2 (6 zeros): ~16M attempts → 16 seconds on CPU");
}

// This would require adding dependencies to Cargo.toml:
// [dependencies]
// num_cpus = "1.0"
// rayon = "1.5"  # For even easier parallelism

#[allow(dead_code)]
fn rayon_example(secret: &str, prefix: &str, max: u32) -> Option<u32> {
    // Using rayon crate for easy parallelism
    // (1..=max).into_par_iter().find_first(|&i| {
    //     let hash = format!("{:x}", md5::compute(format!("{secret}{i}")));
    //     hash.starts_with(prefix)
    // })
    
    // Fallback implementation without rayon
    sequential_search(secret, prefix, max)
}