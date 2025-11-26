# Ring Buffer Implementation Patterns

*Common architectural patterns and design considerations for circular buffer data structures*

---

## 🏗️ **Core Implementation Strategies**

### **1. Array-Based Ring Buffer**

```rust
pub struct RingBuffer<T> {
    buf: Vec<Option<T>>,
    head: usize,    // Points to oldest element
    tail: usize,    // Points to next insertion position
    len: usize,     // Current number of elements
}
```

**Characteristics**:

- ✅ O(1) all operations
- ✅ Cache-friendly contiguous memory
- ✅ Simple modular arithmetic
- ❌ Fixed capacity (unless reallocated)

### **2. Linked Ring Buffer**

```rust
pub struct LinkedRingBuffer<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
    capacity: usize,
}

struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}
```

**Characteristics**:

- ✅ Dynamic allocation per element
- ✅ No wasted slots
- ❌ Reference counting overhead
- ❌ Poor cache locality

### **3. VecDeque-Based Implementation**

```rust
use std::collections::VecDeque;

pub struct RingBuffer<T> {
    buf: VecDeque<T>,
    capacity: usize,
}
```

**Characteristics**:

- ✅ Leverages std library optimizations
- ✅ Handles growth automatically
- ❌ Less control over memory layout
- ❌ Potential allocation on growth

## 🎯 **Index Management Patterns**

### **Pattern 1: Separate Head/Tail Pointers**

```rust
impl<T> RingBuffer<T> {
    fn enqueue(&mut self, value: T) -> Result<(), T> {
        if self.is_full() {
            return Err(value);
        }
        
        self.buf[self.tail] = Some(value);
        self.tail = (self.tail + 1) % self.capacity();
        self.len += 1;
        Ok(())
    }
    
    fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        
        let value = self.buf[self.head].take();
        self.head = (self.head + 1) % self.capacity();
        self.len -= 1;
        value
    }
}
```

**Advantages**:

- Clear separation of concerns
- Easy to understand and debug
- Standard pattern in literature

**Disadvantages**:

- Extra `len` field needed
- Three fields to keep in sync

### **Pattern 2: Count-Based Management**

```rust
impl<T> RingBuffer<T> {
    fn is_full(&self) -> bool {
        self.len == self.capacity()
    }
    
    fn is_empty(&self) -> bool {
        self.len == 0
    }
}
```

**Key Insight**: Using explicit `len` field simplifies full/empty detection

### **Pattern 3: Sentinel-Based (One Slot Wasted)**

```rust
impl<T> RingBuffer<T> {
    fn is_full(&self) -> bool {
        (self.tail + 1) % self.capacity() == self.head
    }
    
    fn is_empty(&self) -> bool {
        self.head == self.tail
    }
}
```

**Trade-off**: Simpler logic, but wastes one slot to distinguish full vs empty

## 🔄 **Wrap-Around Arithmetic Patterns**

### **Modular Arithmetic**

```rust
// Standard approach
next_index = (current_index + 1) % capacity;
```

### **Bitwise AND (Power-of-2 Capacity)**

```rust
// Only works when capacity is power of 2
next_index = (current_index + 1) & (capacity - 1);
```

**Performance**: Bitwise AND is faster than modulo for power-of-2 sizes

### **Conditional Wrap**

```rust
// Explicit branching
next_index = if current_index + 1 == capacity {
    0
} else {
    current_index + 1
};
```

**Trade-off**: May be more readable but potentially slower

## 🛡️ **Safety and Error Handling Patterns**

### **Pattern 1: Fallible Operations**

```rust
pub enum RingBufferError<T> {
    Full(T),    // Returns the value that couldn't be inserted
    Empty,      // Nothing to dequeue
}

impl<T> RingBuffer<T> {
    pub fn enqueue(&mut self, value: T) -> Result<(), RingBufferError<T>> {
        if self.is_full() {
            Err(RingBufferError::Full(value))
        } else {
            // ... insert logic
            Ok(())
        }
    }
}
```

### **Pattern 2: Dual APIs (Fallible + Infallible)**

```rust
impl<T> RingBuffer<T> {
    // Conservative: fails on full
    pub fn try_enqueue(&mut self, value: T) -> Result<(), T> { ... }
    
    // Aggressive: overwrites on full
    pub fn enqueue_overwrite(&mut self, value: T) { ... }
    
    // Query state
    pub fn try_dequeue(&mut self) -> Option<T> { ... }
}
```

### **Pattern 3: Panic-Free Design**

```rust
impl<T> RingBuffer<T> {
    // All operations return Results or Options
    // No panics even in edge cases
    pub fn enqueue(&mut self, value: T) -> Result<(), T> { ... }
    pub fn dequeue(&mut self) -> Option<T> { ... }
    pub fn peek(&self) -> Option<&T> { ... }
}
```

## 🎮 **Capacity Management Patterns**

### **Fixed Capacity**

```rust
impl<T> RingBuffer<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buf: vec![None; capacity],
            head: 0,
            tail: 0,
            len: 0,
        }
    }
}
```

**Best for**: Embedded systems, real-time applications, predictable memory usage

### **Dynamic Resizing**

```rust
impl<T> RingBuffer<T> {
    pub fn grow(&mut self) {
        let new_capacity = self.capacity() * 2;
        let mut new_buf = vec![None; new_capacity];
        
        // Copy existing elements maintaining order
        for i in 0..self.len {
            let old_index = (self.head + i) % self.capacity();
            new_buf[i] = self.buf[old_index].take();
        }
        
        self.buf = new_buf;
        self.head = 0;
        self.tail = self.len;
    }
}
```

**Complexity**: Requires careful element reordering during resize

### **Configurable Strategy**

```rust
pub enum ResizeStrategy {
    Fixed,           // Never resize
    Double,          // Double capacity when full
    Linear(usize),   // Add fixed amount
}

pub struct ConfigurableRingBuffer<T> {
    buf: Vec<Option<T>>,
    strategy: ResizeStrategy,
    // ... other fields
}
```

## 🔍 **Memory Layout Optimization Patterns**

### **Option<T> vs Uninitialized Memory**

```rust
// Safe but potentially wasteful
buf: Vec<Option<T>>,

// More efficient but requires unsafe
buf: Vec<MaybeUninit<T>>,
```

### **Avoiding Option Overhead**

```rust
pub struct RingBuffer<T> {
    buf: Box<[MaybeUninit<T>]>,    // No Option wrapper
    head: usize,
    tail: usize,
    len: usize,
}
```

**Benefit**: Eliminates discriminant overhead for `Option<T>`

### **SIMD-Friendly Alignment**

```rust
#[repr(align(64))]  // Cache line alignment
pub struct AlignedRingBuffer<T> {
    buf: Vec<T>,
    // ... fields
}
```

## 🧪 **Testing Patterns**

### **Comprehensive State Testing**

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_wraparound_behavior() {
        let mut buf = RingBuffer::with_capacity(3);
        
        // Test full cycle
        for i in 0..10 {
            buf.enqueue_overwrite(i);
            verify_invariants(&buf);
        }
        
        // Verify final state
        assert_eq!(buf.dequeue(), Some(7));
        assert_eq!(buf.dequeue(), Some(8));
        assert_eq!(buf.dequeue(), Some(9));
        assert!(buf.is_empty());
    }
    
    fn verify_invariants<T>(buf: &RingBuffer<T>) {
        assert!(buf.len <= buf.capacity());
        assert!(buf.head < buf.capacity());
        assert!(buf.tail < buf.capacity());
    }
}
```

### **Property-Based Testing**

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn ring_buffer_maintains_fifo_order(
        ops: Vec<RingBufferOp>,
        capacity in 1usize..100
    ) {
        let mut buf = RingBuffer::with_capacity(capacity);
        let mut expected = VecDeque::new();
        
        for op in ops {
            match op {
                RingBufferOp::Enqueue(val) => {
                    if !buf.is_full() {
                        buf.enqueue(val).unwrap();
                        expected.push_back(val);
                    }
                },
                RingBufferOp::Dequeue => {
                    assert_eq!(buf.dequeue(), expected.pop_front());
                }
            }
        }
    }
}
```

## 🎨 **API Design Patterns**

### **Builder Pattern for Configuration**

```rust
impl<T> RingBuffer<T> {
    pub fn builder() -> RingBufferBuilder<T> {
        RingBufferBuilder::new()
    }
}

pub struct RingBufferBuilder<T> {
    capacity: Option<usize>,
    resize_strategy: ResizeStrategy,
    // ... other options
}

impl<T> RingBufferBuilder<T> {
    pub fn capacity(mut self, cap: usize) -> Self {
        self.capacity = Some(cap);
        self
    }
    
    pub fn fixed_size(mut self) -> Self {
        self.resize_strategy = ResizeStrategy::Fixed;
        self
    }
    
    pub fn build(self) -> RingBuffer<T> {
        // ... construct with options
    }
}
```

### **Iterator Support**

```rust
impl<T> RingBuffer<T> {
    pub fn iter(&self) -> RingBufferIter<T> {
        RingBufferIter {
            buffer: self,
            current: self.head,
            remaining: self.len,
        }
    }
}

impl<T> Iterator for RingBufferIter<'_, T> {
    type Item = &T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            let item = &self.buffer.buf[self.current];
            self.current = (self.current + 1) % self.buffer.capacity();
            self.remaining -= 1;
            item.as_ref()
        }
    }
}
```

## 🔗 **Integration Patterns**

### **Generic Over Storage**

```rust
pub struct RingBuffer<T, S = Vec<Option<T>>> {
    storage: S,
    head: usize,
    tail: usize,
    len: usize,
}

// Can be backed by Vec, array, or custom storage
type ArrayRingBuffer<T, const N: usize> = RingBuffer<T, [Option<T>; N]>;
type VecRingBuffer<T> = RingBuffer<T, Vec<Option<T>>>;
```

### **Async-Compatible Design**

```rust
pub struct AsyncRingBuffer<T> {
    inner: Arc<Mutex<RingBuffer<T>>>,
    capacity: usize,
}

impl<T> AsyncRingBuffer<T> {
    pub async fn enqueue(&self, value: T) -> Result<(), T> {
        let mut buffer = self.inner.lock().await;
        buffer.enqueue(value)
    }
}
```

---

*Tags: #ring-buffer #implementation-patterns #data-structures #performance #memory-management #api-design*

*Links: [[zettel-index]] | [[Ring Buffer Overwriting Semantics]] | [[Bounded vs Unbounded Collections]] | [[mission-2]] | [[Memory Layout Optimization]] | [[Cache-Friendly Data Structures]]*
