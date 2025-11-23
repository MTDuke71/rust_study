# 🧠 Memory Address Analysis

*Understanding memory layout, pointer arithmetic, and address printing in Rust for debugging and performance analysis*

---

## 🎯 **Core Concepts**

Memory address analysis in Rust provides insights into how data is stored and accessed, crucial for debugging ownership issues, optimizing performance, and understanding system-level behavior.

### **Address Printing Fundamentals**
```rust
let x = 42i32;
println!("x = {} at address {:p}", x, &x);
// Output: x = 42 at address 0x7ffee1234567
```

**Key Format Specifiers:**
- `{:p}` - Pointer/address formatting (hexadecimal)
- `{:x}` - Hexadecimal formatting for numeric addresses
- `{:?}` - Debug formatting (includes addresses for references)

## 🏗️ **Stack vs Heap Analysis**

### **Stack Allocation Patterns**
```rust
let stack_array = [1, 2, 3, 4, 5];
let x = 10;
let y = 20;

println!("Stack array: {:p}", &stack_array);
println!("Variable x: {:p}", &x);
println!("Variable y: {:p}", &y);

// Stack variables are typically close together
// Address difference shows stack growth direction
```

### **Heap Allocation Patterns**
```rust
let heap_vec = vec![1, 2, 3, 4, 5];
let heap_box = Box::new(42);

println!("Vec data pointer: {:p}", heap_vec.as_ptr());
println!("Vec struct address: {:p}", &heap_vec);
println!("Box data address: {:p}", heap_box.as_ref());
println!("Box pointer address: {:p}", &heap_box);
```

**Key Insights:**
- **Vec struct** lives on stack, **data** lives on heap
- **Box pointer** on stack, **contents** on heap
- Heap addresses are unpredictable and scattered

## 🔍 **Method Call Memory Analysis**

### **Method Dispatch and Self**
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        println!("Method called on: {:p}", self);
        self.width * self.height
    }
}

let rect = Rectangle { width: 10, height: 20 };
println!("Rectangle address: {:p}", &rect);
let area = rect.area(); // Automatic referencing: &rect passed as self
```

**Memory Behavior:**
- `rect.area()` automatically converts to `Rectangle::area(&rect)`
- `self` parameter receives same address as original struct
- **Zero-cost abstraction**: No runtime overhead for method syntax

### **Ownership Transfer Analysis**
```rust
impl Rectangle {
    fn consume(self) -> u32 {
        println!("Consumed rectangle at: {:p}", &self);
        self.width * self.height
    }
}

let rect = Rectangle { width: 10, height: 20 };
println!("Before consume: {:p}", &rect);
let result = rect.consume(); // Moves ownership
// rect is no longer accessible
```

## 🚀 **Performance Analysis Applications**

### **Cache Locality Verification**
```rust
let data = vec![1, 2, 3, 4, 5];
for (i, value) in data.iter().enumerate() {
    println!("data[{}] = {} at {:p}", i, value, value);
}
// Sequential addresses indicate good cache performance
```

### **Memory Layout Optimization**
```rust
#[repr(C)]  // C-style layout for predictable addressing
struct OptimizedStruct {
    large_field: u64,    // 8 bytes
    medium_field: u32,   // 4 bytes  
    small_field: u16,    // 2 bytes
    tiny_field: u8,      // 1 byte
}

let opt = OptimizedStruct {
    large_field: 1,
    medium_field: 2,
    small_field: 3,
    tiny_field: 4,
};

println!("Struct base: {:p}", &opt);
println!("large_field: {:p}", &opt.large_field);
println!("medium_field: {:p}", &opt.medium_field);
// Analyze padding and alignment
```

## 🧪 **Debugging Applications**

### **Ownership Issue Diagnosis**
```rust
fn debug_ownership() {
    let original = vec![1, 2, 3];
    println!("Original: {:p}", original.as_ptr());
    
    let reference = &original;
    println!("Reference points to: {:p}", reference.as_ptr());
    
    let cloned = original.clone();
    println!("Clone: {:p}", cloned.as_ptr());
    
    // Verify independent allocations
    assert_ne!(original.as_ptr(), cloned.as_ptr());
}
```

### **Memory Leak Detection**
```rust
fn leak_analysis() {
    let large_allocation = vec![0u32; 1_000_000];
    println!("Large allocation: {:p} (size: {} MB)", 
             large_allocation.as_ptr(),
             large_allocation.len() * 4 / 1_000_000);
    
    // Memory is freed when large_allocation goes out of scope
    // Use tools like Valgrind or AddressSanitizer for production
}
```

## ⚠️ **Raw Pointer Analysis**

### **Unsafe Address Manipulation**
```rust
let mut value = 100;
let ptr = &mut value as *mut i32;

println!("Raw pointer: {:p}", ptr);
println!("Points to stack? {}", is_stack_address(ptr as usize));

unsafe {
    println!("Dereferenced: {}", *ptr);
    *ptr = 200;
}
println!("Modified through pointer: {}", value);

fn is_stack_address(addr: usize) -> bool {
    // Heuristic: stack addresses are typically high
    addr > 0x7f0000000000
}
```

**Safety Considerations:**
- Raw pointers bypass Rust's safety guarantees
- Address arithmetic can lead to undefined behavior
- Use only for system programming or FFI integration

## 🎯 **Mission Integration Applications**

### **Mission2: Queue Memory Layout**
```rust
// Analyze ring buffer memory access patterns
struct RingBuffer<T> {
    data: Vec<Option<T>>,
    head: usize,
    tail: usize,
}

impl<T> RingBuffer<T> {
    fn analyze_layout(&self) {
        println!("Buffer base: {:p}", self.data.as_ptr());
        println!("Head index: {} -> {:p}", self.head, 
                unsafe { self.data.as_ptr().add(self.head) });
        println!("Tail index: {} -> {:p}", self.tail,
                unsafe { self.data.as_ptr().add(self.tail) });
    }
}
```

### **Mission5: HashMap Bucket Analysis**
```rust
// Verify hash collision distribution
struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
}

impl<K, V> HashMap<K, V> {
    fn analyze_distribution(&self) {
        for (i, bucket) in self.buckets.iter().enumerate() {
            if !bucket.is_empty() {
                println!("Bucket {} at {:p}: {} entries", 
                         i, bucket.as_ptr(), bucket.len());
            }
        }
    }
}
```

## 📊 **Real-World Use Cases**

### **When to Use Address Analysis**
- ✅ **Debugging ownership/borrowing** issues
- ✅ **Performance optimization** and cache analysis  
- ✅ **Data structure verification** (independence, layout)
- ✅ **Memory allocation patterns** understanding
- ✅ **Educational purposes** (learning system behavior)
- ✅ **FFI integration** with C libraries

### **When NOT to Use**
- ❌ **Production logging** (addresses change between runs)
- ❌ **Serialization/persistence** (addresses are temporary)
- ❌ **Comparison operations** (use `std::ptr::eq` instead)
- ❌ **General program logic** (use high-level abstractions)

## 🔗 **Integration with Learning Tracks**

### **Mission Applications**
- **[[mission-2]]**: Ring buffer memory access patterns
- **[[mission-5]]**: HashMap bucket distribution analysis
- **[[mission-6]]**: Grid memory layout optimization
- **[[mission-7]]**: Graph adjacency list memory efficiency

### **Daily Study Connections**
- **Week 1**: Ownership and borrowing visualization
- **Week 2**: Collection memory patterns (Vec, HashMap, BTreeMap)
- **Week 3**: Trait object memory layout and vtables
- **Week 4**: Grid algorithms and spatial locality

### **Rust Book Integration**
- **Chapter 4**: Ownership system memory model
- **Chapter 5**: Struct layout and method dispatch
- **Chapter 8**: Collection internal memory management
- **Chapter 15**: Smart pointer memory indirection

## 🛠️ **Tools and Techniques**

### **Built-in Rust Tools**
```rust
// Memory size analysis
println!("Size of u32: {}", std::mem::size_of::<u32>());
println!("Alignment of u32: {}", std::mem::align_of::<u32>());

// Pointer comparison
let a = [1, 2, 3];
let b = [1, 2, 3];
println!("Same data, different addresses: {}", !std::ptr::eq(&a, &b));
```

### **External Tools**
- **Valgrind**: Memory error detection (Linux/macOS)
- **AddressSanitizer**: Fast memory error detector  
- **Heaptrack**: Heap memory profiler
- **cargo-profiler**: Rust-specific profiling tools

---

*Tags: #memory-analysis #addresses #pointers #performance #debugging #unsafe #method-dispatch #ownership #heap #stack #cache-locality #profiling #mission-integration #rust-book-ch5*

*Links: [[zettel-index]] | [[mission-5]] | [[Ownership and Borrowing]] | [[Performance Optimization]] | [[Collections MOC]] | [[API Design Patterns]] | [[Unsafe Rust - Raw Pointers and Safety Contracts]]*