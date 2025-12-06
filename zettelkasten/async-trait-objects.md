# Async Trait Objects: Dynamic Dispatch with Futures

*Combining async/await with trait objects for runtime polymorphism in asynchronous Rust*

---

## 🎯 Core Concept

**Async trait objects** enable runtime polymorphism with futures—allowing heterogeneous collections of async operations and dynamic dispatch to async methods. This intersection of Ch17 (async) and Ch18 (OOP/trait objects) requires understanding both `dyn Trait` mechanics and `Pin` requirements.

**Key Challenge**: Async methods return opaque `impl Future` types with different sizes, making them incompatible with trait objects by default.

---

## 🧠 Mental Model

### The Integration Problem

```
Traditional Trait Object          Async Trait Challenge
═════════════════════════        ═══════════════════════════════════
trait Draw {                     trait AsyncService {
    fn draw(&self);                  async fn call(&self) -> Response;
}                                }
         ↓                                    ↓
Box<dyn Draw> ✅                 Box<dyn AsyncService> ❌
(fixed vtable pointer)           (return type varies by impl!)
```

**Why it fails**: Each `async fn` becomes a different state machine type. `impl Future` types have different sizes per implementation, breaking the trait object model.

---

## 🔍 Solutions

### **Solution 1: Return Boxed Futures (Manual)**

Manually box the return type to erase the concrete Future type:

```rust
use std::future::Future;
use std::pin::Pin;

// Define async behavior with boxed return
trait AsyncHandler {
    fn handle(&self, request: Request) 
        -> Pin<Box<dyn Future<Output = Response> + Send + '_>>;
}

struct LoggingHandler;

impl AsyncHandler for LoggingHandler {
    fn handle(&self, request: Request) 
        -> Pin<Box<dyn Future<Output = Response> + Send + '_>> 
    {
        Box::pin(async move {
            println!("Handling: {:?}", request);
            Response::ok()
        })
    }
}

// Now we can use trait objects
async fn dispatch(handlers: &[Box<dyn AsyncHandler>], request: Request) {
    for handler in handlers {
        handler.handle(request.clone()).await;
    }
}
```

**Type Breakdown**:
- `Pin<...>` - Required for self-referential futures
- `Box<...>` - Heap allocation erases concrete size
- `dyn Future<Output = Response>` - Type-erased future
- `+ Send` - Required if crossing thread boundaries
- `+ '_` - Lifetime bound to `&self`

### **Solution 2: async_trait Crate (Recommended)**

The `async_trait` crate automates the boxing:

```rust
use async_trait::async_trait;

#[async_trait]
trait AsyncService {
    async fn process(&self, data: &str) -> Result<String, Error>;
}

#[async_trait]
impl AsyncService for MyService {
    async fn process(&self, data: &str) -> Result<String, Error> {
        // Write natural async code
        let result = fetch_from_db(data).await?;
        Ok(result)
    }
}

// Trait object works seamlessly
let services: Vec<Box<dyn AsyncService>> = vec![
    Box::new(ServiceA),
    Box::new(ServiceB),
];

for service in &services {
    service.process("input").await?;
}
```

**What async_trait does**: Transforms `async fn` into returning `Pin<Box<dyn Future + Send + '_>>` automatically.

### **Solution 3: Return Type Impl Trait (Static Dispatch Alternative)**

When trait objects aren't required, use generics for zero-cost:

```rust
trait AsyncProcessor {
    // Each implementation can have different Future type
    fn process(&self) -> impl Future<Output = i32> + Send;
}

// Static dispatch - no boxing overhead
async fn run_processor<P: AsyncProcessor>(p: &P) -> i32 {
    p.process().await
}
```

**Trade-off**: Requires generics, can't use `dyn AsyncProcessor`.

---

## 📊 Comparison: Static vs Dynamic Async Dispatch

| Aspect | Static (Generics) | Dynamic (Trait Objects) |
|--------|-------------------|------------------------|
| **Syntax** | `impl Future` return | `Pin<Box<dyn Future>>` |
| **Performance** | Zero-cost, inlined | Heap allocation + vtable |
| **Flexibility** | Homogeneous only | Heterogeneous collections |
| **Code Size** | Monomorphized (larger) | Single implementation |
| **Use Case** | Hot paths, known types | Plugin systems, handlers |

---

## 🛠️ Common Patterns

### **Pattern 1: Heterogeneous Task Queue**

```rust
use std::pin::Pin;
use std::future::Future;

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

struct TaskQueue {
    tasks: Vec<BoxFuture<'static, ()>>,
}

impl TaskQueue {
    fn add<F>(&mut self, future: F) 
    where 
        F: Future<Output = ()> + Send + 'static 
    {
        self.tasks.push(Box::pin(future));
    }
    
    async fn run_all(&mut self) {
        for task in self.tasks.drain(..) {
            task.await;
        }
    }
}

// Usage: different async operations in same queue
let mut queue = TaskQueue { tasks: vec![] };
queue.add(async { fetch_users().await; });
queue.add(async { update_cache().await; });
queue.add(async { send_notifications().await; });
queue.run_all().await;
```

### **Pattern 2: Async Strategy Pattern**

```rust
use async_trait::async_trait;

#[async_trait]
trait DataFetcher {
    async fn fetch(&self, id: u64) -> Result<Data, Error>;
}

struct HttpFetcher { base_url: String }
struct CacheFetcher { cache: HashMap<u64, Data> }
struct FallbackFetcher { 
    primary: Box<dyn DataFetcher + Send + Sync>,
    fallback: Box<dyn DataFetcher + Send + Sync>,
}

#[async_trait]
impl DataFetcher for FallbackFetcher {
    async fn fetch(&self, id: u64) -> Result<Data, Error> {
        match self.primary.fetch(id).await {
            Ok(data) => Ok(data),
            Err(_) => self.fallback.fetch(id).await,
        }
    }
}

// Runtime strategy selection
let fetcher: Box<dyn DataFetcher + Send + Sync> = if use_cache {
    Box::new(CacheFetcher { cache })
} else {
    Box::new(HttpFetcher { base_url })
};
```

### **Pattern 3: Async Streams with Trait Objects**

```rust
use futures::stream::{Stream, BoxStream};
use futures::StreamExt;

trait DataSource {
    fn stream(&self) -> BoxStream<'_, DataItem>;
}

struct DatabaseSource { /* ... */ }

impl DataSource for DatabaseSource {
    fn stream(&self) -> BoxStream<'_, DataItem> {
        // Box the stream for trait object compatibility
        Box::pin(async_stream::stream! {
            for item in self.query().await {
                yield item;
            }
        })
    }
}

// Heterogeneous stream sources
let sources: Vec<Box<dyn DataSource>> = vec![
    Box::new(DatabaseSource { }),
    Box::new(FileSource { }),
    Box::new(ApiSource { }),
];

for source in &sources {
    let mut stream = source.stream();
    while let Some(item) = stream.next().await {
        process(item);
    }
}
```

---

## ⚠️ Pitfalls and Solutions

### **Pitfall 1: Send Bounds**

```rust
// ❌ Won't work across threads (e.g., tokio::spawn)
trait Handler {
    fn handle(&self) -> Pin<Box<dyn Future<Output = ()> + '_>>;
}

// ✅ Add Send bound for thread safety
trait Handler {
    fn handle(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;
}
```

### **Pitfall 2: Lifetime Elision**

```rust
// ❌ Lifetime ambiguity
fn handle(&self) -> Pin<Box<dyn Future<Output = ()>>>;

// ✅ Explicit lifetime tied to &self
fn handle(&self) -> Pin<Box<dyn Future<Output = ()> + '_>>;
// The '_ means: "lives as long as &self"
```

### **Pitfall 3: Object Safety**

```rust
// ❌ Not object-safe: generic method
trait Processor {
    async fn process<T: Serialize>(&self, item: T);
}

// ✅ Object-safe: concrete type or type parameter on trait
trait Processor<T> {
    async fn process(&self, item: T);
}
```

---

## 🔗 Integration with Ch16-18 Concepts

### **From Ch16 (Concurrency)**

- **Send/Sync**: Async trait objects crossing threads need `+ Send + Sync`
- **Arc<dyn Trait>**: Shared ownership of async trait objects

```rust
let handler: Arc<dyn AsyncHandler + Send + Sync> = Arc::new(MyHandler);
let handler_clone = Arc::clone(&handler);
tokio::spawn(async move {
    handler_clone.handle(request).await;
});
```

### **From Ch17 (Async)**

- **Pin**: Required because `dyn Future` might be self-referential
- **Polling**: Runtime polls boxed futures through vtable

### **From Ch18 (OOP)**

- **Trait Objects**: Same `dyn Trait` mechanics, just with async returns
- **Encapsulation**: Hide async implementation details behind trait interface

---

## 📚 When to Use

| Scenario | Recommendation |
|----------|----------------|
| Performance-critical hot path | Static dispatch (generics) |
| Plugin/handler systems | `async_trait` + trait objects |
| Heterogeneous task collections | `BoxFuture` / `BoxStream` |
| Library public API | Consider both, document trade-offs |
| Simple cases | Avoid complexity, use concrete types |

---

## 🔗 Related Notes

**Async Concepts:**
- [[async-await-basics]] - Foundation of async/await
- [[future-trait-deep-dive]] - Future, Pin, Unpin mechanics
- [[async-streams]] - BoxStream patterns

**OOP Concepts:**
- [[rust-oop-characteristics]] - Rust's approach to OOP
- [[trait-objects-polymorphism]] - General trait object guide
- [[state-pattern-rust]] - State pattern with async considerations

**Integration:**
- [[rust-concurrency-moc]] - Concurrency patterns overview
- [[rust-book-ch16-18-review]] - Chapter review connecting these concepts

---

*Tags: #async #trait-objects #dynamic-dispatch #futures #polymorphism #ch17 #ch18*

*Links: [[zettel-index]] | [[rust-concurrency-moc]] | [[rust-oop-characteristics]] | [[future-trait-deep-dive]] | [[async-await-basics]] | [[async-streams]] | [[state-pattern-rust]]*
