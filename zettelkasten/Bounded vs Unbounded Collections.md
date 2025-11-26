# Bounded vs Unbounded Collections

*Comparing fixed-capacity and dynamically-growing data structures in system design*

---

## 🎯 **Core Distinction**

### **Bounded Collections**

- **Fixed maximum capacity** determined at creation
- **Fail or overwrite** when capacity exceeded
- **Predictable memory usage** and performance
- **Suitable for resource-constrained environments**

### **Unbounded Collections**

- **Grow dynamically** as elements are added
- **Limited only by available memory**
- **Unpredictable allocation patterns**
- **Suitable for general-purpose applications**

## 📊 **Comparison Matrix**

| Aspect | Bounded | Unbounded |
|--------|---------|-----------|
| **Memory Usage** | Fixed, predictable | Variable, grows with data |
| **Performance** | O(1) consistent | O(1) amortized, occasional O(n) |
| **Failure Mode** | Capacity errors | Out-of-memory crashes |
| **Real-time Suitability** | ✅ Excellent | ❌ Poor (allocation spikes) |
| **Memory Safety** | ✅ Bounded | ⚠️ Can exhaust system memory |
| **Implementation Complexity** | Higher (capacity handling) | Lower (rely on allocator) |

## 🏗️ **Implementation Patterns**

### **Bounded Queue Example**

```rust
pub struct BoundedQueue<T> {
    buf: Vec<Option<T>>,
    head: usize,
    tail: usize,
    len: usize,
    capacity: usize,
}

impl<T> BoundedQueue<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buf: vec![None; capacity],
            head: 0,
            tail: 0,
            len: 0,
            capacity,
        }
    }
    
    pub fn enqueue(&mut self, value: T) -> Result<(), T> {
        if self.len >= self.capacity {
            Err(value)  // Capacity exceeded
        } else {
            self.buf[self.tail] = Some(value);
            self.tail = (self.tail + 1) % self.capacity;
            self.len += 1;
            Ok(())
        }
    }
}
```

### **Unbounded Queue Example**

```rust
use std::collections::VecDeque;

pub struct UnboundedQueue<T> {
    buf: VecDeque<T>,
}

impl<T> UnboundedQueue<T> {
    pub fn new() -> Self {
        Self {
            buf: VecDeque::new(),
        }
    }
    
    pub fn enqueue(&mut self, value: T) {
        self.buf.push_back(value);  // Never fails (until OOM)
    }
    
    pub fn dequeue(&mut self) -> Option<T> {
        self.buf.pop_front()
    }
}
```

## 🎮 **Use Case Analysis**

### **When to Choose Bounded**

#### **1. Real-Time Systems**

```rust
// Audio processing - must not allocate during playback
struct AudioBuffer {
    samples: BoundedQueue<f32>,  // Fixed-size ring buffer
}

impl AudioBuffer {
    fn process_sample(&mut self, sample: f32) {
        // Always succeeds or overwrites - no allocation
        self.samples.enqueue_overwrite(sample);
    }
}
```

**Requirements**: Deterministic timing, no allocation pauses

#### **2. Embedded Systems**

```rust
// Sensor data collection with limited RAM
struct SensorBuffer {
    readings: BoundedQueue<SensorReading>,
}

impl SensorBuffer {
    fn record_reading(&mut self, reading: SensorReading) {
        match self.readings.enqueue(reading) {
            Ok(()) => log::info!("Reading recorded"),
            Err(_) => log::warn!("Buffer full, dropping reading"),
        }
    }
}
```

**Requirements**: Memory constraints, predictable resource usage

#### **3. Rate Limiting / Back-pressure**

```rust
// Network packet buffer with flow control
struct PacketBuffer {
    packets: BoundedQueue<Packet>,
}

impl PacketBuffer {
    fn receive_packet(&mut self, packet: Packet) -> Result<(), PacketDropped> {
        self.packets.enqueue(packet)
            .map_err(|_| PacketDropped::BufferFull)
    }
}
```

**Requirements**: Flow control, prevent memory exhaustion

#### **4. Resource Pools**

```rust
// Connection pool with maximum connections
struct ConnectionPool {
    connections: BoundedQueue<Connection>,
    max_connections: usize,
}

impl ConnectionPool {
    fn return_connection(&mut self, conn: Connection) {
        if self.connections.enqueue(conn).is_err() {
            // Pool full - close excess connection
            drop(conn);
        }
    }
}
```

**Requirements**: Resource limits, system stability

### **When to Choose Unbounded**

#### **1. General-Purpose Applications**

```rust
// Task queue that processes work items
struct TaskQueue {
    tasks: UnboundedQueue<Task>,
}

impl TaskQueue {
    fn submit_task(&mut self, task: Task) {
        self.tasks.enqueue(task);  // Simple, always works
    }
}
```

**Benefits**: Simple API, no capacity planning needed

#### **2. Data Collection and Analysis**

```rust
// Log aggregation - collect all events
struct LogCollector {
    events: Vec<LogEvent>,  // Unbounded growth
}

impl LogCollector {
    fn record_event(&mut self, event: LogEvent) {
        self.events.push(event);  // Preserve all data
    }
}
```

**Requirements**: Complete data retention, analysis flexibility

#### **3. User Interface Components**

```rust
// Command history in text editor
struct CommandHistory {
    commands: Vec<Command>,
}

impl CommandHistory {
    fn execute_command(&mut self, cmd: Command) {
        self.commands.push(cmd);  // Unlimited undo history
    }
}
```

**Benefits**: Better user experience, no artificial limits

#### **4. Caching Without Size Limits**

```rust
use std::collections::HashMap;

// Simple cache that grows as needed
struct Cache<K, V> {
    data: HashMap<K, V>,
}

impl<K: Eq + Hash, V> Cache<K, V> {
    fn get_or_insert<F>(&mut self, key: K, f: F) -> &V
    where F: FnOnce() -> V
    {
        self.data.entry(key).or_insert_with(f)
    }
}
```

**Trade-off**: Simplicity vs. memory control

## ⚡ **Performance Characteristics**

### **Bounded Collections**

```rust
// Consistent O(1) operations
impl<T> BoundedQueue<T> {
    pub fn enqueue(&mut self, value: T) -> Result<(), T> {
        // Always O(1) - no allocation, no reallocation
        if self.is_full() {
            Err(value)
        } else {
            // Fixed-time insertion
            self.insert_at_tail(value);
            Ok(())
        }
    }
}
```

**Performance Profile**:

- ✅ **Consistent timing** - no allocation pauses
- ✅ **Predictable memory access** patterns
- ✅ **Cache-friendly** with contiguous storage
- ❌ **Capacity planning** required

### **Unbounded Collections**

```rust
// Amortized O(1) with occasional O(n) spikes
impl<T> UnboundedQueue<T> {
    pub fn enqueue(&mut self, value: T) {
        // Usually O(1), but O(n) when growing
        self.buf.push_back(value);  // May trigger reallocation
    }
}
```

**Performance Profile**:

- ✅ **Simple implementation** - delegate to allocator
- ✅ **No capacity limits** to plan for
- ❌ **Allocation spikes** during growth
- ❌ **Unpredictable latency** due to reallocation

## 🔄 **Hybrid Approaches**

### **1. Bounded with Overflow Strategy**

```rust
pub enum OverflowStrategy {
    DropOldest,     // Ring buffer behavior
    DropNewest,     // Reject new items
    DropRandom,     // Probabilistic dropping
}

pub struct BoundedQueueWithOverflow<T> {
    buf: BoundedQueue<T>,
    strategy: OverflowStrategy,
}

impl<T> BoundedQueueWithOverflow<T> {
    pub fn enqueue(&mut self, value: T) {
        match self.buf.enqueue(value) {
            Ok(()) => {},
            Err(rejected_value) => {
                self.handle_overflow(rejected_value);
            }
        }
    }
}
```

### **2. Growable with Maximum Limit**

```rust
pub struct LimitedGrowthQueue<T> {
    buf: Vec<T>,
    max_capacity: usize,
}

impl<T> LimitedGrowthQueue<T> {
    pub fn enqueue(&mut self, value: T) -> Result<(), T> {
        if self.buf.len() >= self.max_capacity {
            Err(value)  // Hard limit reached
        } else {
            self.buf.push(value);  // Grow until limit
            Ok(())
        }
    }
}
```

### **3. Adaptive Capacity**

```rust
pub struct AdaptiveQueue<T> {
    buf: VecDeque<T>,
    target_capacity: usize,
    max_capacity: usize,
}

impl<T> AdaptiveQueue<T> {
    pub fn enqueue(&mut self, value: T) -> Result<(), T> {
        if self.buf.len() >= self.max_capacity {
            Err(value)
        } else {
            self.buf.push_back(value);
            self.maybe_shrink();  // Adaptive resizing
            Ok(())
        }
    }
    
    fn maybe_shrink(&mut self) {
        if self.buf.len() < self.target_capacity / 4 {
            self.buf.shrink_to(self.target_capacity);
        }
    }
}
```

## 🛡️ **Safety and Error Handling**

### **Bounded Collection Errors**

```rust
#[derive(Debug, PartialEq)]
pub enum BoundedError<T> {
    CapacityExceeded(T),  // Returns the rejected value
    Empty,                // Nothing to dequeue
}

impl<T> BoundedQueue<T> {
    pub fn try_enqueue(&mut self, value: T) -> Result<(), BoundedError<T>> {
        if self.is_full() {
            Err(BoundedError::CapacityExceeded(value))
        } else {
            self.enqueue_unchecked(value);
            Ok(())
        }
    }
}
```

### **Unbounded Collection Monitoring**

```rust
pub struct MonitoredUnboundedQueue<T> {
    buf: VecDeque<T>,
    max_size_seen: usize,
    size_warnings: Vec<usize>,
}

impl<T> MonitoredUnboundedQueue<T> {
    pub fn enqueue(&mut self, value: T) {
        self.buf.push_back(value);
        
        let current_size = self.buf.len();
        if current_size > self.max_size_seen {
            self.max_size_seen = current_size;
            
            // Warn at exponential thresholds
            if current_size.is_power_of_two() && current_size >= 1024 {
                log::warn!("Queue size reached: {}", current_size);
                self.size_warnings.push(current_size);
            }
        }
    }
}
```

## 🎛️ **Configuration Patterns**

### **Runtime Configuration**

```rust
pub struct ConfigurableQueue<T> {
    inner: QueueImpl<T>,
}

enum QueueImpl<T> {
    Bounded(BoundedQueue<T>),
    Unbounded(VecDeque<T>),
}

impl<T> ConfigurableQueue<T> {
    pub fn new(config: QueueConfig) -> Self {
        let inner = match config {
            QueueConfig::Bounded { capacity } => {
                QueueImpl::Bounded(BoundedQueue::with_capacity(capacity))
            },
            QueueConfig::Unbounded => {
                QueueImpl::Unbounded(VecDeque::new())
            },
        };
        Self { inner }
    }
}
```

### **Feature-Based Selection**

```rust
#[cfg(feature = "bounded-only")]
type DefaultQueue<T> = BoundedQueue<T>;

#[cfg(not(feature = "bounded-only"))]
type DefaultQueue<T> = VecDeque<T>;

pub struct Queue<T> {
    inner: DefaultQueue<T>,
}
```

## 📈 **Memory Usage Patterns**

### **Bounded Memory Profile**

```
Memory Usage
     ▲
     │ ┌─────────────────────────┐  ← Fixed maximum
     │ │                         │
     │ │  Predictable Usage      │
     │ │                         │
     │ └─────────────────────────┘
     │
     └─────────────────────────────► Time
```

### **Unbounded Memory Profile**

```
Memory Usage
     ▲
     │                    /
     │                 /
     │              /     ← Growth spikes
     │           /
     │        /
     │     /
     │  /
     └─────────────────────────────► Time
```

## 🔍 **Choosing the Right Approach**

### **Decision Matrix**

| Requirement | Bounded | Unbounded | Hybrid |
|-------------|---------|-----------|---------|
| **Real-time constraints** | ✅ | ❌ | ⚠️ |
| **Memory-constrained environment** | ✅ | ❌ | ⚠️ |
| **Simple implementation** | ❌ | ✅ | ❌ |
| **Complete data retention** | ❌ | ✅ | ⚠️ |
| **Predictable performance** | ✅ | ❌ | ⚠️ |
| **Unknown data volume** | ❌ | ✅ | ✅ |

### **Recommendation Guidelines**

```rust
// Choose bounded when:
// - Memory usage must be predictable
// - Real-time constraints exist
// - Running in embedded/constrained environment
// - Implementing flow control or rate limiting

let bounded_queue = BoundedQueue::with_capacity(1000);

// Choose unbounded when:
// - Data must be preserved completely
// - Memory constraints are not a concern
// - Implementation simplicity is prioritized
// - Data volume is highly variable

let unbounded_queue = VecDeque::new();

// Choose hybrid when:
// - Need flexibility between bounded/unbounded
// - Want growth with safety limits
// - Adaptive behavior based on runtime conditions

let hybrid_queue = AdaptiveQueue::new(target_cap: 1000, max_cap: 10000);
```

---

*Tags: #bounded-collections #unbounded-collections #memory-management #performance #system-design #capacity-planning*

*Links: [[zettel-index]] | [[Ring Buffer Overwriting Semantics]] | [[Ring Buffer Implementation Patterns]] | [[Memory Layout Optimization]] | [[Performance Analysis Patterns]] | [[mission-2]]*
