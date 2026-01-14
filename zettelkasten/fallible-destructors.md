# ⚠️ Fallible Destructors

**Why Drop cannot fail and how to handle resource cleanup errors**

**Tags:** #drop #destructors #error-handling #resource-management #rust-for-rustaceans-ch3 #api-design

**Related:** [[drop-trait]], [[Error Handling Patterns]], [[graceful-shutdown-patterns]], [[API Design Patterns]], [[Resource Management]], [[async-concurrency]]

---

## 🎯 Core Problem

**Drop cannot return a Result or be async**

The `Drop` trait has this signature:

```rust
trait Drop {
    fn drop(&mut self);  // ❌ No return value - cannot propagate errors!
}
```

**Why this is a problem:**
- File flush operations can fail (disk full, I/O errors)
- Network socket shutdown can fail (connection lost)
- Database transactions may need explicit commit/rollback
- Async resources cannot be cleaned up (Drop is not async)

---

## 🚫 Why Drop Cannot Fail

### **Reason 1: Implicit Invocation**

Drop is called implicitly - you can't handle errors at call site:

```rust
{
    let file = FileWriter::new("test.txt")?;
    // ... use file ...
}  // ❌ Drop called here - no way to catch errors!
```

### **Reason 2: Panic During Unwind**

If Drop could panic during another panic, program would abort:

```rust
impl Drop for BadResource {
    fn drop(&mut self) {
        panic!("Drop failed!");  // ❌ Double panic = abort!
    }
}
```

### **Reason 3: Order Dependencies**

Drop order is well-defined but you can't control when it happens:

```rust
struct Container {
    a: ResourceA,
    b: ResourceB,
}  // Drop order: b, then a (reverse declaration order)
```

---

## ✅ Solution Pattern: Explicit close() + Best-Effort Drop

**Two-Phase Resource Cleanup:**

1. **Explicit `close()` method** - Allows error handling
2. **Best-effort `Drop` fallback** - Catches forgotten cleanup

```rust
use std::fs::File;
use std::io::{self, Write};

struct FileWriter {
    file: Option<File>,
}

impl FileWriter {
    fn new(path: &str) -> io::Result<Self> {
        Ok(Self {
            file: Some(File::create(path)?),
        })
    }
    
    /// Explicit close allows error handling
    fn close(&mut self) -> io::Result<()> {
        if let Some(mut file) = self.file.take() {
            file.flush()?;      // ✅ Can fail - caller must handle
            file.sync_all()?;   // ✅ Can fail - caller must handle
        }
        Ok(())
    }
}

impl Drop for FileWriter {
    fn drop(&mut self) {
        // Best-effort cleanup - cannot return errors
        if let Some(ref mut file) = self.file {
            let _ = file.flush();  // ⚠️ Ignore errors in Drop
        }
    }
}
```

### **Usage Pattern**

```rust
// ✅ GOOD: Explicit close with error handling
let mut writer = FileWriter::new("important.txt")?;
writer.write_data("critical data")?;

match writer.close() {
    Ok(()) => println!("Saved successfully"),
    Err(e) => eprintln!("Failed to save: {}", e),
}

// ⚠️ ACCEPTABLE: Forgot to close - Drop provides fallback
let writer2 = FileWriter::new("less_important.txt")?;
// ... forgot to call close() ...
// Drop will try best-effort cleanup (errors ignored)
```

---

## 🎓 Design Patterns

### **Pattern 1: Option<Resource> with take()**

```rust
struct Connection {
    socket: Option<TcpStream>,
}

impl Connection {
    fn disconnect(&mut self) -> io::Result<()> {
        if let Some(mut socket) = self.socket.take() {
            socket.shutdown(Shutdown::Both)?;  // Can fail
        }
        Ok(())
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        if let Some(ref mut socket) = self.socket {
            let _ = socket.shutdown(Shutdown::Both);  // Ignore errors
        }
    }
}
```

**Why `Option`?**
- `take()` moves resource out, replacing with `None`
- Prevents double-cleanup (close + drop won't both try)
- Drop checks `is_some()` before attempting cleanup

### **Pattern 2: ManuallyDrop for Opt-Out**

```rust
use std::mem::ManuallyDrop;

struct CustomCleanup {
    resource: ManuallyDrop<ExpensiveResource>,
}

impl CustomCleanup {
    fn cleanup(mut self) -> Result<(), Error> {
        // Explicit cleanup with error handling
        unsafe {
            ManuallyDrop::drop(&mut self.resource);
        }
        Ok(())
    }
}

// No Drop implementation - user MUST call cleanup()
```

### **Pattern 3: Scoped Cleanup with Guards**

```rust
struct TransactionGuard<'a> {
    tx: &'a mut Transaction,
    committed: bool,
}

impl<'a> TransactionGuard<'a> {
    fn commit(mut self) -> Result<(), Error> {
        self.tx.commit()?;
        self.committed = true;  // Mark as committed
        Ok(())
    }
}

impl Drop for TransactionGuard<'_> {
    fn drop(&mut self) {
        if !self.committed {
            let _ = self.tx.rollback();  // Auto-rollback if not committed
        }
    }
}
```

---

## ⚡ Async Resources Problem

### **The Async Drop Problem**

```rust
struct AsyncResource {
    connection: TcpStream,
}

impl Drop for AsyncResource {
    fn drop(&mut self) {
        // ❌ Cannot use async/await in Drop!
        // self.connection.shutdown().await;  // Won't compile
    }
}
```

### **Solution: Explicit Async Close**

```rust
impl AsyncResource {
    async fn close(self) -> io::Result<()> {
        self.connection.shutdown().await?;  // ✅ Async cleanup
        Ok(())
    }
}

// No Drop implementation - user must call close().await
```

### **Alternative: Blocking Shutdown**

```rust
use tokio::runtime::Handle;

impl Drop for AsyncResource {
    fn drop(&mut self) {
        // Spawn blocking task for cleanup
        if let Ok(handle) = Handle::try_current() {
            handle.spawn(async move {
                let _ = self.connection.shutdown().await;
            });
        }
    }
}
```

**⚠️ Warning:** Spawning in Drop is controversial - can cause subtle bugs

---

## 📋 Real-World Examples

### **File Handles**

```rust
// Standard library pattern
impl Drop for File {
    fn drop(&mut self) {
        let _ = self.flush();  // Best effort - errors ignored
    }
}

// User code with explicit error handling
let mut file = File::create("data.txt")?;
file.write_all(b"important")?;
file.sync_all()?;  // ✅ Explicit - can handle errors
```

### **Database Connections**

```rust
struct DbConnection {
    conn: Option<PgConnection>,
}

impl DbConnection {
    async fn close(mut self) -> Result<(), DbError> {
        if let Some(conn) = self.conn.take() {
            conn.close().await?;  // Async cleanup with error handling
        }
        Ok(())
    }
}

// No Drop - forces user to call close().await
```

### **Mutex Guards**

```rust
// Mutex unlocking is infallible - perfect for Drop
impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        // Unlocking cannot fail - safe in Drop
        unsafe { self.lock.unlock(); }
    }
}
```

---

## 🎯 Decision Matrix: When to Use Each Pattern

| **Resource Type** | **Pattern** | **Rationale** |
|-------------------|-------------|---------------|
| **Infallible cleanup** | Only Drop | Unlocking mutex, decrementing counters |
| **Fallible cleanup (sync)** | close() + Drop | File flush, socket shutdown |
| **Fallible cleanup (async)** | async close() only | Async I/O, network protocols |
| **Must guarantee cleanup** | close() mandatory | Database transactions, critical resources |
| **Optional cleanup** | Drop only | Best-effort logging, non-critical cleanup |

---

## ⚠️ Common Pitfalls

### **❌ Pitfall 1: Panic in Drop**

```rust
impl Drop for BadResource {
    fn drop(&mut self) {
        self.cleanup().unwrap();  // ❌ DANGEROUS - can double panic!
    }
}
```

**Fix:** Ignore errors or log them:

```rust
impl Drop for GoodResource {
    fn drop(&mut self) {
        if let Err(e) = self.cleanup() {
            eprintln!("Cleanup failed: {}", e);  // ✅ Log, don't panic
        }
    }
}
```

### **❌ Pitfall 2: Blocking in Async Drop**

```rust
impl Drop for AsyncResource {
    fn drop(&mut self) {
        // ❌ BAD - blocks async runtime!
        std::thread::sleep(Duration::from_secs(1));
    }
}
```

### **❌ Pitfall 3: Forgetting to Call Explicit Close**

```rust
fn process() -> io::Result<()> {
    let mut writer = FileWriter::new("data.txt")?;
    writer.write_data("important")?;
    // ❌ Forgot writer.close()? - Drop might lose data!
    Ok(())
}
```

**Fix:** Use type system to enforce cleanup:

```rust
struct UnclosedWriter(File);

impl UnclosedWriter {
    fn close(self) -> io::Result<ClosedWriter> {
        self.0.sync_all()?;
        Ok(ClosedWriter)
    }
}

struct ClosedWriter;  // Proof of proper cleanup
```

---

## 🔗 Related Concepts

- [[drop-trait]] - The Drop trait fundamentals
- [[Error Handling Patterns]] - Rust error handling strategies
- [[graceful-shutdown-patterns]] - Proper shutdown sequences
- [[API Design Patterns]] - Designing safe APIs
- [[async-concurrency]] - Async cleanup challenges
- [[rust-for-rustaceans]] - Source material (Ch3)

---

## 📝 Quick Reference

```rust
// ✅ PATTERN: Explicit close() + Best-effort Drop
struct Resource {
    handle: Option<Handle>,
}

impl Resource {
    // Fallible close - user can handle errors
    fn close(&mut self) -> Result<(), Error> {
        if let Some(handle) = self.handle.take() {
            handle.cleanup()?;  // Can fail
        }
        Ok(())
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        // Best-effort - ignore errors
        if let Some(ref mut handle) = self.handle {
            let _ = handle.cleanup();
        }
    }
}

// Usage
let mut res = Resource::new()?;
res.use_resource()?;
res.close()?;  // ✅ Handle errors explicitly
```

---

## 🎓 Learning Resources

### **Rust for Rustaceans Ch3: Designing Interfaces**

Key insights:
- Drop cannot return Result - design around this limitation
- Provide explicit cleanup methods for fallible operations
- Drop should be best-effort fallback, not primary cleanup
- Async resources cannot use Drop for cleanup

### **Implementation Example**

See: [[rust-for-rustaceans]] Ch3, `flexible_ownership.rs` - `FileWriter` implementation

---

**Key Takeaway:** **Drop is for infallible cleanup.** For fallible resource cleanup, provide an explicit `close()` method and use Drop as a best-effort safety net.

*Links:*
- **Outgoing:** [[drop-trait]], [[Error Handling Patterns]], [[graceful-shutdown-patterns]], [[API Design Patterns]], [[async-concurrency]], [[rust-for-rustaceans]]
- **Incoming:** (To be added by related notes)
