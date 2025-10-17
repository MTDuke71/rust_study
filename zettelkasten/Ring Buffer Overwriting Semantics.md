# Ring Buffer Overwriting Semantics

*How circular buffers handle capacity constraints through overwrite strategies*

---

## 🎯 **Two Buffer Strategies**

### **1. Reject-on-Full (Standard)**
```rust
queue.enqueue(10)?; // Returns Err(10) if full
```

### **2. Overwrite-Oldest (Circular)**
```rust
queue.enqueue_overwrite(10); // Always succeeds, overwrites oldest
```

## 🔄 **Behavioral Comparison**

### **Reject-on-Full Semantics**
```rust
let mut queue = RingBufferQueue::with_capacity(3);
queue.enqueue(1)?; // Ok(())
queue.enqueue(2)?; // Ok(())
queue.enqueue(3)?; // Ok(()) - Now full
queue.enqueue(4); // Err(4) - Rejected!
```

**Queue state**: `[1, 2, 3]` - Fourth value rejected

### **Overwrite-Oldest Semantics**
```rust
let mut queue = RingBufferQueue::with_capacity(3);
queue.enqueue_overwrite(1);
queue.enqueue_overwrite(2);
queue.enqueue_overwrite(3); // Now full
queue.enqueue_overwrite(4); // Overwrites 1!
```

**Queue state**: `[2, 3, 4]` - Oldest value (1) replaced

## 🔧 **Implementation Pattern**

```rust
pub fn enqueue_overwrite(&mut self, value: T) {
    if self.is_full() {
        // Overwrite: advance head to "forget" oldest value
        self.head = (self.head + 1) % self.capacity();
        self.len -= 1; // Adjust length since we're replacing
    }
    
    // Standard enqueue logic
    self.buf[self.tail] = Some(value);
    self.tail = (self.tail + 1) % self.capacity();
    self.len += 1;
}
```

### **Key Insight**: 
When full, we **artificially dequeue** by moving head forward, then enqueue normally.

## 📊 **Memory Layout During Overwrite**

### **Before Overwrite** (Capacity = 3, Full)
```
buf:  [Some(1), Some(2), Some(3)]
head:  0
tail:  0 (wrapped around)
len:   3
```

### **After `enqueue_overwrite(4)`**
```
buf:  [Some(4), Some(2), Some(3)]
head:  1 (advanced - "forgot" value 1)
tail:  1 (wrapped and placed at old head position)
len:   3
```

### **Next dequeue returns**: `Some(2)` - FIFO order maintained!

## 🎮 **Real-World Use Cases**

### **1. Recent History Tracking**
```rust
struct CommandHistory {
    buffer: RingBufferQueue<String>,
}

impl CommandHistory {
    fn record_command(&mut self, cmd: String) {
        // Keep only last N commands
        self.buffer.enqueue_overwrite(cmd);
    }
}
```

**Application**: Shells, game replay systems, undo buffers

### **2. Sensor Data Buffering**
```rust
struct SensorBuffer {
    readings: RingBufferQueue<Reading>,
}

impl SensorBuffer {
    fn add_reading(&mut self, reading: Reading) {
        // Always accept latest data, discard oldest if needed
        self.readings.enqueue_overwrite(reading);
    }
}
```

**Application**: Real-time monitoring, data acquisition systems

### **3. Game Frame History**
```rust
struct FrameHistory {
    frames: RingBufferQueue<GameState>,
}

impl FrameHistory {
    fn record_frame(&mut self, state: GameState) {
        // Maintain sliding window of recent frames
        self.frames.enqueue_overwrite(state);
    }
}
```

**Application**: Netcode rollback, replay systems, debugging

### **4. Log Rotation**
```rust
struct CircularLog<T> {
    log: RingBufferQueue<T>,
}

impl<T> CircularLog<T> {
    fn log_event(&mut self, event: T) {
        // Maintain fixed-size recent log
        self.log.enqueue_overwrite(event);
    }
}
```

**Application**: Embedded systems, constrained-memory logging

## 🏆 **Key Benefits**

1. **Bounded Memory** - Never exceeds fixed capacity
2. **O(1) Operations** - Constant time regardless of history size
3. **Predictable Behavior** - No allocation surprises
4. **Cache Friendly** - Fixed array backing store
5. **No Failure Cases** - `enqueue_overwrite` always succeeds

## ⚠️ **Trade-offs**

### **Advantages Over Unbounded**
- ✅ Fixed memory footprint
- ✅ No allocation overhead
- ✅ Predictable performance
- ✅ Suitable for real-time systems

### **Disadvantages**
- ❌ Data loss (oldest values discarded)
- ❌ Requires capacity planning
- ❌ Not suitable when all data must be preserved

## 🔀 **Choosing the Right Strategy**

| Scenario | Strategy | Reason |
|----------|----------|---------|
| **All data must be preserved** | Reject-on-full or unlimited | No data loss acceptable |
| **Recent data more important** | Overwrite-oldest | Latest data prioritized |
| **Fixed memory constraint** | Overwrite-oldest | Hard memory limit |
| **Real-time systems** | Overwrite-oldest | Predictable behavior |
| **Logs, metrics, telemetry** | Overwrite-oldest | Sampling recent data |
| **Message processing** | Reject-on-full | All messages important |

## 🧪 **Testing Overwrite Behavior**

```rust
#[test]
fn test_overwrite_behavior() {
    let mut queue = RingBufferQueue::with_capacity(3);
    
    // Fill queue
    queue.enqueue_overwrite(1);
    queue.enqueue_overwrite(2);
    queue.enqueue_overwrite(3);
    assert_eq!(queue.len(), 3);
    
    // Overwrite oldest (1)
    queue.enqueue_overwrite(4);
    assert_eq!(queue.len(), 3); // Still at capacity
    
    // Verify FIFO order: should get 2, 3, 4
    assert_eq!(queue.dequeue(), Some(2)); // 1 was overwritten
    assert_eq!(queue.dequeue(), Some(3));
    assert_eq!(queue.dequeue(), Some(4));
    assert!(queue.is_empty());
}
```

## 📐 **Requirements Integration**

This feature adds to Mission2's requirements:

**REQ-R5**: Overwriting Circular Buffer Support
- When full, `enqueue_overwrite` shall replace the oldest element
- Maintains FIFO ordering of remaining elements
- Operation completes in O(1) time
- Preserves ring buffer invariants

**REQ-R6**: Dual Enqueue Semantics
- `enqueue()` - Returns `Err(T)` when full (reject-on-full)
- `enqueue_overwrite()` - Overwrites oldest when full (always succeeds)
- Users can choose appropriate strategy per use case

## 🔗 **Integration with Mission2 APIs**

```rust
// Conservative approach - handle full condition
match queue.enqueue(value) {
    Ok(()) => println!("Enqueued successfully"),
    Err(returned_value) => println!("Queue full, value rejected: {}", returned_value),
}

// Aggressive approach - always accept latest
queue.enqueue_overwrite(value);
println!("Value recorded (may have replaced oldest)");
```

## 💡 **Design Philosophy**

The dual API design follows Rust's principle of **"explicit over implicit"**:

- `enqueue()` - Makes capacity limits **explicit** through `Result`
- `enqueue_overwrite()` - Makes data loss **explicit** through name
- Users **consciously choose** the trade-off appropriate for their use case

---

*Tags: #ring-buffer #circular-buffer #overwrite-semantics #mission2 #data-structures #capacity-management*

*Links: [[zettel-index]] | [[Missions Overview]] | [[../missions/Mission2/README|Mission2 Queue]] | [[Ring Buffer Implementation Patterns]] | [[Bounded vs Unbounded Collections]]*
