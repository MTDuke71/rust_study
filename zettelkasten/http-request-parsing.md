# HTTP Request Parsing - Reading Web Requests

*Understanding HTTP request structure and extracting information from TCP streams*

---

## 🎯 **Core Concept**

HTTP request parsing is the process of reading raw bytes from a `TcpStream` and extracting structured information: the HTTP method (GET, POST), the requested path, headers, and body. Since HTTP is a **text-based protocol**, parsing involves reading lines of text and splitting them into components.

**Key insight**: HTTP requests are **human-readable text** sent over TCP. The first line contains the method and path, followed by header lines, then a blank line, then an optional body. This simple structure makes basic parsing straightforward but robust production parsing requires careful handling of edge cases.

## 🧠 **Mental Models**

### The Postal Mail Analogy

Think of an HTTP request like a postal letter:
- **Request line** (envelope front): "GET /inbox HTTP/1.1" - who/what/where
- **Headers** (envelope back): Metadata like sender address, return address
- **Blank line**: Seal between envelope and contents
- **Body** (letter inside): Actual message content (empty for GET, data for POST)

The parser is like a mail sorter - it reads the envelope first (request line + headers), then decides whether to open it (read body) based on the metadata.

### State Machine View

```
START → Read Request Line → Read Headers → Check for Body → END
          ↓                    ↓               ↓
      Method, Path, Ver    Key-Value Pairs  Body Bytes
```

HTTP parsing is naturally sequential - you must read the request line before headers, and headers before body. This matches Rust's iterator-based parsing perfectly.

## 🔍 **Detailed Content**

### **HTTP Request Structure**

```
GET /index.html HTTP/1.1              ← Request line
Host: www.example.com                 ← Headers start
User-Agent: Mozilla/5.0               ↓
Accept: text/html                     ↓
Connection: keep-alive                ← Headers end
                                      ← Blank line (separator)
[optional body for POST/PUT]          ← Body (if present)
```

**Components:**
1. **Request line**: `METHOD PATH VERSION`
2. **Headers**: `Key: Value` pairs, one per line
3. **Blank line**: `\r\n` signals end of headers
4. **Body**: Optional data (depends on method and Content-Length header)

### **Basic Parsing Pattern (Rust Book Example)**

```rust
use std::io::{BufReader, BufRead};
use std::net::TcpStream;

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    
    // Read first line - the request line
    let request_line = buf_reader
        .lines()
        .next()                    // Get first line
        .unwrap()                  // Iterator always has at least one line
        .unwrap();                 // Handle potential I/O error
    
    println!("Request: {}", request_line);  // "GET / HTTP/1.1"
    
    // Route based on request
    match &request_line[..] {
        "GET / HTTP/1.1" => serve_index(),
        "GET /about HTTP/1.1" => serve_about(),
        _ => serve_404(),
    }
}
```

**Why BufReader?**
- Raw `TcpStream` only reads bytes, no line boundaries
- `BufReader` buffers data and provides `.lines()` iterator
- Much more efficient than reading one byte at a time

### **Parsing Request Line Components**

```rust
fn parse_request_line(line: &str) -> Option<(String, String, String)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    
    if parts.len() != 3 {
        return None;  // Malformed request
    }
    
    let method = parts[0].to_string();   // "GET"
    let path = parts[1].to_string();     // "/index.html"
    let version = parts[2].to_string();  // "HTTP/1.1"
    
    Some((method, path, version))
}

// Usage
let request_line = "GET /api/users HTTP/1.1";
if let Some((method, path, version)) = parse_request_line(request_line) {
    println!("Method: {}, Path: {}, Version: {}", method, path, version);
}
```

### **Parsing Headers**

```rust
use std::collections::HashMap;

fn parse_headers(buf_reader: &mut BufReader<&TcpStream>) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    
    for line in buf_reader.lines() {
        let line = line.unwrap();
        
        // Blank line signals end of headers
        if line.is_empty() {
            break;
        }
        
        // Split "Key: Value" into components
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_lowercase(), value.to_string());
        }
    }
    
    headers
}

// Usage
let headers = parse_headers(&mut buf_reader);
if let Some(host) = headers.get("host") {
    println!("Client requested host: {}", host);
}
```

**Common headers:**
- `Host`: Target domain/IP
- `User-Agent`: Client software info
- `Content-Length`: Body size in bytes (critical for POST)
- `Content-Type`: Body format (application/json, etc.)

### **Path and Query String Parsing**

```rust
fn parse_path(path: &str) -> (&str, Option<&str>) {
    match path.split_once('?') {
        Some((path, query)) => (path, Some(query)),
        None => (path, None),
    }
}

// Example
let full_path = "/search?q=rust&limit=10";
let (path, query) = parse_path(full_path);

println!("Path: {}", path);           // "/search"
println!("Query: {:?}", query);       // Some("q=rust&limit=10")

// Further parse query parameters
if let Some(query) = query {
    for param in query.split('&') {
        if let Some((key, value)) = param.split_once('=') {
            println!("  {} = {}", key, value);
        }
    }
}
```

### **Request Body Handling (POST/PUT)**

```rust
fn read_body(
    buf_reader: &mut BufReader<&TcpStream>, 
    headers: &HashMap<String, String>
) -> Option<String> {
    // Check if body exists
    let content_length: usize = headers
        .get("content-length")?
        .parse()
        .ok()?;
    
    if content_length == 0 {
        return None;
    }
    
    // Read exact number of bytes
    let mut body = vec![0u8; content_length];
    buf_reader.read_exact(&mut body).ok()?;
    
    // Convert to string (assuming UTF-8)
    String::from_utf8(body).ok()
}

// Usage
if let Some(body) = read_body(&mut buf_reader, &headers) {
    println!("Request body: {}", body);
}
```

**Important**: Without `Content-Length`, you can't know when the body ends. HTTP requires this header for requests with bodies.

### **Production-Grade Parsing Considerations**

```rust
// ❌ AVOID: Vulnerable to attack
let request_line = buf_reader.lines().next().unwrap().unwrap();

// ✅ BETTER: Handle errors gracefully
let request_line = buf_reader
    .lines()
    .next()
    .ok_or("No request line")?
    .map_err(|e| format!("Failed to read: {}", e))?;

// ✅ BETTER: Limit line length (prevent DoS)
let mut limited_reader = buf_reader.take(8192);  // 8KB max
let request_line = limited_reader.lines().next()...;

// ✅ BETTER: Timeout for slow clients
stream.set_read_timeout(Some(Duration::from_secs(30)))?;
```

**Security concerns:**
- **Buffer overflow**: Limit header/body sizes
- **Slowloris attacks**: Timeout slow readers
- **Invalid UTF-8**: Handle non-text data gracefully
- **Header injection**: Sanitize header values

### **Complete Example: Simple Router**

```rust
use std::collections::HashMap;
use std::io::{BufReader, BufRead, Write};
use std::net::TcpStream;

struct Request {
    method: String,
    path: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

fn parse_request(mut stream: TcpStream) -> Result<Request, String> {
    let mut buf_reader = BufReader::new(&stream);
    
    // Parse request line
    let request_line = buf_reader
        .lines()
        .next()
        .ok_or("Empty request")??;
    
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() != 3 {
        return Err("Invalid request line".to_string());
    }
    
    let method = parts[0].to_string();
    let path = parts[1].to_string();
    
    // Parse headers
    let mut headers = HashMap::new();
    for line in buf_reader.lines() {
        let line = line?;
        if line.is_empty() { break; }
        
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_lowercase(), value.to_string());
        }
    }
    
    // Parse body if present
    let body = if let Some(len_str) = headers.get("content-length") {
        if let Ok(len) = len_str.parse::<usize>() {
            let mut body_bytes = vec![0u8; len];
            buf_reader.read_exact(&mut body_bytes).ok();
            String::from_utf8(body_bytes).ok()
        } else {
            None
        }
    } else {
        None
    };
    
    Ok(Request { method, path, headers, body })
}

fn route_request(request: Request, mut stream: TcpStream) {
    let response = match (&request.method[..], &request.path[..]) {
        ("GET", "/") => "HTTP/1.1 200 OK\r\n\r\nHome Page",
        ("GET", "/about") => "HTTP/1.1 200 OK\r\n\r\nAbout Page",
        ("POST", "/api/data") => {
            println!("Received data: {:?}", request.body);
            "HTTP/1.1 201 Created\r\n\r\n"
        }
        _ => "HTTP/1.1 404 NOT FOUND\r\n\r\n404 Not Found",
    };
    
    stream.write_all(response.as_bytes()).unwrap();
}
```

## 💡 **Key Takeaways**

1. **HTTP is text-based** - read lines using `BufReader`, not raw bytes
2. **Request line comes first** - contains method, path, and HTTP version
3. **Headers are key-value pairs** - parse until blank line, use `HashMap` for storage
4. **Body requires Content-Length** - can't parse body without knowing its size
5. **Production parsing needs limits** - prevent DoS with max sizes and timeouts

## 🔗 **Integration Points**

### **Builds On**
- [[tcp-listener]] - TcpStream is source of HTTP data
- [[Iterator Patterns]] - `.lines()` provides line-by-line reading
- [[HashMap Deep Dive]] - Storing headers efficiently
- [[Result Type]] - Error handling for malformed requests

### **Enables**
- [[rust-book/rust-book-ch21]] - Building complete web servers
- [[JSON Processing with serde_json]] - Parsing JSON request bodies
- API servers, REST endpoints, web applications

### **Related Concepts**
- [[String Manipulation]] - Parsing text protocols
- [[Text Parsing Patterns]] - General parsing strategies
- [[Error Handling Patterns]] - Graceful failure in network code
- [[async-streams]] - Async HTTP parsing with Tokio

### **Real-World Libraries**
Instead of manual parsing, production code uses:
- **hyper**: High-performance HTTP implementation
- **actix-web**: Full web framework with routing
- **axum**: Ergonomic web framework built on Tower/Tokio
- **warp**: Composable web framework with filters

Manual parsing (as shown here) is educational and useful for:
- Understanding HTTP fundamentals
- Custom protocols
- Lightweight servers without dependencies
- Embedded systems with size constraints

---

*Tags: #networking #http #parsing #rust-book-ch21 #concept #practical #intermediate*

*Links: [[zettel-index]] | [[rust-concurrency-moc]] | [[tcp-listener]] | [[rust-book/rust-book-ch21]] | [[String Manipulation]] | [[Text Parsing Patterns]] | [[../rust_book/Ch21/README]] | [[../rust_book/Ch21/web_server/README]]*
