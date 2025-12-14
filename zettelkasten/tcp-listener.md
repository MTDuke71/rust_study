# TCP Listener - Network Connection Basics

*Rust's foundational building block for network servers: accepting client connections over TCP*

---

## 🎯 **Core Concept**

A `TcpListener` in Rust provides the ability to bind to a network address and accept incoming TCP connections. It's the server-side primitive for building network applications, from simple HTTP servers to complex distributed systems.

**Key insight**: The listener acts as a **gatekeeper** - it binds to an address:port, waits for clients to connect, and hands you a `TcpStream` for each connection. The listener itself doesn't read/write data; it creates connections that do.

## 🧠 **Mental Models**

### The Restaurant Host Analogy

Think of `TcpListener` as a restaurant host:
- **Binding**: Claiming your table section (address:port)
- **Incoming connections**: Customers arriving at the door
- **TcpStream**: Seating each customer at their individual table
- **Handling connections**: The waiter serving that specific table

The host doesn't take orders or serve food - they create the seating arrangement. Each waiter (connection handler) works with their assigned table independently.

### Iterator of Connection Attempts

```rust
for stream in listener.incoming() {
    // Each iteration = one client connection attempt
}
```

This is an **infinite iterator** - it never stops listening unless the program exits or you explicitly break. Each item is a `Result<TcpStream, Error>` representing a connection attempt (which might fail).

## 🔍 **Detailed Content**

### **Basic Usage Pattern**

```rust
use std::net::TcpListener;

// Bind to localhost on port 7878
let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

// Listen for incoming connections indefinitely
for stream in listener.incoming() {
    match stream {
        Ok(stream) => {
            println!("New connection: {:?}", stream.peer_addr());
            // Handle the connection
        }
        Err(e) => {
            eprintln!("Connection error: {}", e);
        }
    }
}
```

**Step-by-step execution:**
1. `bind()` claims the address:port from the OS
2. `incoming()` returns an iterator that blocks waiting for connections
3. Each iteration yields one `Result<TcpStream, Error>`
4. If connection succeeds, you get a bidirectional byte stream
5. If connection fails (port conflicts, network issues), you get `Err`

### **Address Binding Options**

```rust
// Localhost only (typical for development)
TcpListener::bind("127.0.0.1:8080")?;

// All network interfaces (public server)
TcpListener::bind("0.0.0.0:8080")?;

// Let OS choose available port
let listener = TcpListener::bind("127.0.0.1:0")?;
println!("Bound to: {}", listener.local_addr()?);  // Shows actual port

// IPv6 support
TcpListener::bind("[::1]:8080")?;  // IPv6 localhost
```

**Security consideration**: `0.0.0.0` exposes your server to the internet. Use `127.0.0.1` for local-only development.

### **Synchronous vs Async Listeners**

**Std library (blocking)**:
```rust
use std::net::TcpListener;

// Blocks the thread waiting for connections
for stream in listener.incoming() {
    handle_connection(stream?);  // Blocks until connection handled
}
```

**Tokio (async)**:
```rust
use tokio::net::TcpListener;

let listener = TcpListener::bind("127.0.0.1:8080").await?;

loop {
    let (stream, _addr) = listener.accept().await?;
    
    // Spawn concurrent task - doesn't block
    tokio::spawn(async move {
        handle_connection(stream).await;
    });
}
```

**Key difference**: Std blocks the thread; Tokio yields to runtime allowing thousands of concurrent connections.

### **Connection Metadata**

```rust
let stream = listener.incoming().next().unwrap()?;

// Client address
println!("Client: {:?}", stream.peer_addr()?);

// Server address (what we're bound to)
println!("Server: {:?}", stream.local_addr()?);

// Socket options
stream.set_nodelay(true)?;  // Disable Nagle's algorithm
stream.set_ttl(64)?;        // Time-to-live for packets
```

### **Error Handling Patterns**

```rust
// Pattern 1: Continue on errors
for stream in listener.incoming() {
    match stream {
        Ok(stream) => handle_connection(stream),
        Err(e) => {
            eprintln!("Failed to accept connection: {}", e);
            continue;  // Keep listening despite error
        }
    }
}

// Pattern 2: Propagate errors
for stream in listener.incoming() {
    let stream = stream?;  // Bubble error up, stop server
    handle_connection(stream);
}
```

**Production recommendation**: Log errors but continue listening. Network glitches shouldn't crash your server.

### **Multithreaded Server Pattern**

```rust
use std::thread;

let listener = TcpListener::bind("127.0.0.1:7878")?;

for stream in listener.incoming() {
    let stream = stream?;
    
    // Spawn thread per connection
    thread::spawn(|| {
        handle_connection(stream);
    });
}
```

**Improvement - Thread Pool:**
```rust
let pool = ThreadPool::new(4);  // Fixed worker threads

for stream in listener.incoming() {
    let stream = stream?;
    
    pool.execute(|| {
        handle_connection(stream);
    });
}
```

See [[rust_book/rust-book-ch21]] for complete thread pool implementation.

## 💡 **Key Takeaways**

1. **`TcpListener::bind()`** claims an address:port from the OS - can fail if port already in use
2. **`listener.incoming()`** returns infinite iterator of connection attempts - blocks waiting for clients
3. **Each `TcpStream`** represents a bidirectional byte channel to one client
4. **Std library is blocking** - thread per connection or thread pool required for concurrency
5. **Async runtimes (Tokio) are non-blocking** - thousands of connections on one thread possible

## 🔗 **Integration Points**

### **Builds On**
- [[rust-book/rust-book-ch16]] - Threading for concurrent connections
- [[Result Type]] - Error handling for network operations
- [[Iterator Patterns]] - Understanding `incoming()` as infinite iterator

### **Enables**
- [[http-request-parsing]] - Reading HTTP requests from TcpStream
- [[rust-book/rust-book-ch21]] - Building complete web servers
- [[async-await-basics]] - Async networking with Tokio

### **Related Concepts**
- [[message-passing-channels]] - Thread communication in multi-threaded servers
- [[shared-state-concurrency]] - Thread pool pattern for workers
- [[async-concurrency]] - Non-blocking alternatives to thread-per-connection

### **Practical Applications**
- **HTTP servers**: Web servers, REST APIs
- **Chat servers**: Real-time communication
- **Game servers**: Multiplayer networking
- **Database proxies**: Connection pooling
- **Monitoring tools**: Network traffic analysis

---

*Tags: #networking #tcp #io #rust-book-ch21 #concept #practical #intermediate*

*Links: [[zettel-index]] | [[rust-concurrency-moc]] | [[rust-book/rust-book-ch21]] | [[http-request-parsing]] | [[async-await-basics]]*
