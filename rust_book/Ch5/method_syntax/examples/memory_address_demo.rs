// Memory Address Analysis in Rust
// Demonstrates when and how to print memory addresses like C pointers

#[derive(Debug)]
struct HashMapBucket<T> {
    data: Vec<T>,
    capacity: usize,
}

impl<T> HashMapBucket<T> {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            capacity: 0,
        }
    }
    
    fn with_capacity(cap: usize) -> Self {
        Self {
            data: Vec::with_capacity(cap),
            capacity: cap,
        }
    }
}

fn main() {
    println!("=== Memory Address Analysis in Rust ===\n");
    
    // =======================================================================
    // 1. BASIC ADDRESS PRINTING (equivalent to C's &variable and printf("%p"))
    // =======================================================================
    println!("1. Basic Address Printing:");
    
    let x = 42i32;
    let y = 100i32;
    
    println!("   x = {} at address {:p}", x, &x);
    println!("   y = {} at address {:p}", y, &y);
    println!("   Address difference: {} bytes", 
             (&y as *const i32 as usize) - (&x as *const i32 as usize));
    
    // =======================================================================
    // 2. STACK vs HEAP ALLOCATION VISUALIZATION
    // =======================================================================
    println!("\n2. Stack vs Heap Allocation:");
    
    let stack_array = [1, 2, 3, 4, 5];
    let heap_vec = vec![1, 2, 3, 4, 5];
    
    println!("   Stack array at: {:p}", &stack_array);
    println!("   Stack array[0] at: {:p}", &stack_array[0]);
    
    println!("   Vec struct at: {:p}", &heap_vec);
    println!("   Vec data pointer: {:p}", heap_vec.as_ptr());
    println!("   Vec[0] (on heap) at: {:p}", &heap_vec[0]);
    
    // Calculate distance from stack to heap
    let stack_addr = &stack_array as *const _ as usize;
    let heap_addr = heap_vec.as_ptr() as usize;
    println!("   Stack-to-heap distance: {} bytes", 
             if heap_addr > stack_addr { heap_addr - stack_addr } 
             else { stack_addr - heap_addr });
    
    // =======================================================================
    // 3. OWNERSHIP & BORROWING DEBUGGING
    // =======================================================================
    println!("\n3. Ownership & Borrowing Analysis:");
    
    let owner = String::from("Hello, Rust!");
    println!("   Owner string at: {:p}", &owner);
    println!("   String data at: {:p}", owner.as_ptr());
    
    let borrower = &owner;
    println!("   Borrower reference at: {:p}", &borrower);
    println!("   Borrower points to: {:p}", borrower.as_ptr());
    
    // Multiple references to same data
    let ref1 = &owner;
    let ref2 = &owner;
    println!("   ref1 at {:p}, points to {:p}", &ref1, ref1.as_ptr());
    println!("   ref2 at {:p}, points to {:p}", &ref2, ref2.as_ptr());
    println!("   Same data? {}", std::ptr::eq(ref1.as_ptr(), ref2.as_ptr()));
    
    // =======================================================================
    // 4. DATA STRUCTURE INTERNAL LAYOUT (Mission5 HashMap style)
    // =======================================================================
    println!("\n4. Data Structure Layout Analysis:");
    
    let bucket1 = HashMapBucket::<i32>::new();
    let bucket2 = HashMapBucket::<i32>::with_capacity(10);
    
    println!("   bucket1 struct at: {:p}", &bucket1);
    println!("   bucket1.data Vec at: {:p}", &bucket1.data);
    println!("   bucket1.capacity at: {:p}", &bucket1.capacity);
    
    println!("   bucket2 struct at: {:p}", &bucket2);
    println!("   bucket2.data Vec at: {:p}", &bucket2.data);
    println!("   bucket2 allocated buffer: {:p}", bucket2.data.as_ptr());
    
    // =======================================================================
    // 5. PERFORMANCE ANALYSIS - CACHE LOCALITY
    // =======================================================================
    println!("\n5. Cache Locality Analysis:");
    
    // Contiguous array (cache-friendly)
    let contiguous = [0u32; 8];
    println!("   Contiguous array:");
    for (i, item) in contiguous.iter().enumerate() {
        println!("     [{}] at {:p} (offset: {})", 
                 i, item, (item as *const u32 as usize) - (&contiguous[0] as *const u32 as usize));
    }
    
    // Scattered pointers (cache-unfriendly)
    let scattered: Vec<Box<u32>> = (0..4).map(|i| Box::new(i * 10)).collect();
    println!("   Scattered Box allocations:");
    for (i, boxed) in scattered.iter().enumerate() {
        println!("     Box[{}] content at {:p}", i, boxed.as_ref());
    }
    
    // =======================================================================
    // 6. UNSAFE POINTER OPERATIONS (Advanced - use with caution!)
    // =======================================================================
    println!("\n6. Unsafe Pointer Operations:");
    
    let mut value = 100i32;
    let ptr = &mut value as *mut i32;
    
    println!("   Raw pointer: {:p}", ptr);
    
    unsafe {
        println!("   Dereferenced value: {}", *ptr);
        *ptr = 200;  // Modify through raw pointer
        println!("   After modification: {}", *ptr);
    }
    println!("   Original variable: {}", value);  // Shows the change
    
    // =======================================================================
    // 7. REAL-WORLD USE CASES
    // =======================================================================
    println!("\n7. Real-World Applications:");
    
    // A. Debugging memory leaks
    let large_vec: Vec<u32> = (0..1000).collect();
    println!("   Large allocation at: {:p} (size: {} bytes)", 
             large_vec.as_ptr(), large_vec.len() * std::mem::size_of::<u32>());
    
    // B. Verifying data structure integrity
    let original_data = vec![1, 2, 3];
    let cloned_data = original_data.clone();
    println!("   Original data: {:p}", original_data.as_ptr());
    println!("   Cloned data: {:p}", cloned_data.as_ptr());
    println!("   Independent copies? {}", !std::ptr::eq(original_data.as_ptr(), cloned_data.as_ptr()));
    
    // C. Performance profiling memory access patterns
    println!("\n   Memory layout summary:");
    println!("   - Stack variables are typically close together");
    println!("   - Heap allocations can be anywhere in memory");
    println!("   - Cache performance depends on access patterns");
    println!("   - Use addresses to verify optimization assumptions");
    
    println!("\n🎓 When to use address printing:");
    println!("   ✅ Debugging ownership and borrowing issues");
    println!("   ✅ Performance analysis and cache optimization");
    println!("   ✅ Verifying data structure layout and integrity");
    println!("   ✅ Understanding memory allocation patterns");
    println!("   ✅ Educational purposes (learning how Rust works)");
    println!("   ❌ Production logging (addresses change between runs)");
    println!("   ❌ Serialization or persistence (addresses are temporary)");
}