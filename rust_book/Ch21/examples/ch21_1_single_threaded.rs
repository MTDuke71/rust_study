/// Chapter 21.1: Building a Single-Threaded Web Server
/// 
/// This example demonstrates:
/// - TCP listener basics
/// - HTTP request/response format
/// - Serving HTML files
/// - The blocking problem with slow requests

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    // Bind to localhost on port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("🦀 Single-threaded server listening on http://127.0.0.1:7878");
    println!("📝 Try these URLs:");
    println!("   http://127.0.0.1:7878/        - Normal response");
    println!("   http://127.0.0.1:7878/sleep   - Slow response (5 seconds)");
    println!("\n⚠️  WARNING: This server blocks on slow requests!");
    println!("   Open two tabs: one to /sleep, one to /");
    println!("   You'll see the / request waits for /sleep to complete.\n");

    // Listen for incoming connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Create buffered reader for efficiency
    let buf_reader = BufReader::new(&mut stream);
    
    // Read HTTP request line (first line)
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("📨 Request: {}", request_line);

    // Match on request path
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            println!("💤 Sleeping for 5 seconds (simulating slow request)...");
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    // Read HTML file
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    // Construct HTTP response
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    // Send response
    stream.write_all(response.as_bytes()).unwrap();
    
    println!("✅ Response sent: {}\n", status_line);
}

/* 
=== Key Learning Points ===

1. TCP Listener:
   - Binds to address:port combination
   - incoming() returns iterator of connection attempts
   - Each connection is a TcpStream

2. HTTP Request Format:
   GET / HTTP/1.1
   Host: 127.0.0.1:7878
   User-Agent: ...
   [blank line]

3. HTTP Response Format:
   HTTP/1.1 200 OK
   Content-Length: 140
   [blank line]
   <html>...</html>

4. The Blocking Problem:
   - handle_connection() is called synchronously
   - If one request sleeps for 5 seconds, ALL other requests wait
   - No concurrency = poor user experience

5. Why This Matters:
   - Real servers need to handle multiple requests simultaneously
   - Slow/expensive operations shouldn't block other clients
   - Solution: Multithreading (see ch21_2_multithreaded.rs)

=== Testing the Blocking Issue ===

Terminal 1:
$ cargo run --example ch21_1_single_threaded

Browser:
1. Open Tab 1: http://127.0.0.1:7878/sleep (starts 5-second delay)
2. Immediately open Tab 2: http://127.0.0.1:7878
3. Observe: Tab 2 doesn't load until Tab 1 completes!

This demonstrates why we need concurrency.

=== HTTP Status Codes ===

200 OK           - Request succeeded
404 NOT FOUND    - Resource doesn't exist
500 SERVER ERROR - Server encountered error

Our simple server only uses 200 and 404.

=== File Structure ===

Ch21/
├── hello.html - Success page
├── 404.html   - Error page
└── examples/
    └── ch21_1_single_threaded.rs (this file)

=== Next Steps ===

Run ch21_2_multithreaded.rs to see how ThreadPool solves the blocking problem!
*/
