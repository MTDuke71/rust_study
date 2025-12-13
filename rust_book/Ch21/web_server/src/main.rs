/// Complete Web Server - Production-Ready Implementation
/// 
/// Features:
/// - Static file serving from static/ directory
/// - Thread pool with configurable size
/// - Request logging
/// - 404 error handling
/// - Graceful shutdown
/// - Multiple routes

mod lib;

use lib::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path::Path,
    thread,
    time::Duration,
};

fn main() {
    let address = "127.0.0.1:7878";
    let pool_size = 4;

    println!("🦀 Complete Web Server");
    println!("══════════════════════════════════════");
    println!("📡 Listening on: http://{}", address);
    println!("🧵 Thread pool size: {}", pool_size);
    println!("📁 Serving static files from: ./static/");
    println!("══════════════════════════════════════");
    println!("\n📝 Available routes:");
    println!("   GET  /              - Home page");
    println!("   GET  /about         - About page");
    println!("   GET  /sleep         - Slow request (5s delay)");
    println!("   GET  /static/*      - Static files");
    println!("\n🛑 Press Ctrl+C to stop the server\n");

    let listener = TcpListener::bind(address).unwrap();
    let pool = ThreadPool::new(pool_size);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("❌ Connection error: {}", e);
            }
        }
    }

    println!("\n🛑 Server shutting down gracefully...");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    
    // Read request line
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        _ => {
            eprintln!("❌ Failed to read request line");
            return;
        }
    };

    let thread_id = thread::current().id();
    println!("📨 [{:?}] {}", thread_id, request_line);

    // Parse request
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        send_error_response(&mut stream, 400, "Bad Request");
        return;
    }

    let method = parts[0];
    let path = parts[1];

    // Route handling
    match (method, path) {
        ("GET", "/") => serve_file(&mut stream, "static/hello.html", thread_id),
        ("GET", "/about") => serve_about(&mut stream, thread_id),
        ("GET", "/sleep") => {
            println!("💤 [{:?}] Simulating slow request...", thread_id);
            thread::sleep(Duration::from_secs(5));
            serve_file(&mut stream, "static/hello.html", thread_id);
        }
        ("GET", path) if path.starts_with("/static/") => {
            serve_static_file(&mut stream, path, thread_id)
        }
        _ => serve_file(&mut stream, "static/404.html", thread_id),
    }
}

fn serve_file(stream: &mut TcpStream, filename: &str, thread_id: thread::ThreadId) {
    match fs::read_to_string(filename) {
        Ok(contents) => {
            let length = contents.len();
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
                length, contents
            );
            stream.write_all(response.as_bytes()).unwrap();
            println!("✅ [{:?}] Served: {} (200 OK)", thread_id, filename);
        }
        Err(_) => {
            serve_file(stream, "static/404.html", thread_id);
        }
    }
}

fn serve_static_file(stream: &mut TcpStream, path: &str, thread_id: thread::ThreadId) {
    // Remove "/static/" prefix
    let file_path = &path[8..];
    let full_path = format!("static/{}", file_path);

    // Security: Prevent directory traversal
    if file_path.contains("..") {
        send_error_response(stream, 403, "Forbidden");
        return;
    }

    // Check if file exists
    if !Path::new(&full_path).exists() {
        serve_file(stream, "static/404.html", thread_id);
        return;
    }

    // Determine content type
    let content_type = match Path::new(file_path).extension().and_then(|s| s.to_str()) {
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "text/javascript",
        Some("json") => "application/json",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        _ => "text/plain",
    };

    match fs::read_to_string(&full_path) {
        Ok(contents) => {
            let length = contents.len();
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",
                length, content_type, contents
            );
            stream.write_all(response.as_bytes()).unwrap();
            println!("✅ [{:?}] Served: {} (200 OK)", thread_id, full_path);
        }
        Err(_) => {
            serve_file(stream, "static/404.html", thread_id);
        }
    }
}

fn serve_about(stream: &mut TcpStream, thread_id: thread::ThreadId) {
    let about_html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>About - Rust Web Server</title>
    <style>
        body { font-family: sans-serif; max-width: 800px; margin: 50px auto; padding: 20px; }
        h1 { color: #333; border-bottom: 3px solid #ff6b35; }
        .info { background-color: #f9f9f9; padding: 15px; border-left: 4px solid #ff6b35; }
    </style>
</head>
<body>
    <h1>🦀 About This Server</h1>
    <div class="info">
        <h3>Implementation Details:</h3>
        <ul>
            <li><strong>Language:</strong> Rust</li>
            <li><strong>Chapter:</strong> The Rust Book - Chapter 21</li>
            <li><strong>Architecture:</strong> Multithreaded with thread pool</li>
            <li><strong>Concurrency:</strong> Message passing (MPSC channels)</li>
        </ul>
    </div>
    <p><a href="/">← Back to Home</a></p>
</body>
</html>"#;

    let length = about_html.len();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        length, about_html
    );
    
    stream.write_all(response.as_bytes()).unwrap();
    println!("✅ [{:?}] Served: /about (200 OK)", thread_id);
}

fn send_error_response(stream: &mut TcpStream, status_code: u16, reason: &str) {
    let error_html = format!(
        r#"<!DOCTYPE html>
<html><head><title>{} {}</title></head>
<body><h1>{} {}</h1></body></html>"#,
        status_code, reason, status_code, reason
    );

    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        status_code,
        reason,
        error_html.len(),
        error_html
    );

    stream.write_all(response.as_bytes()).unwrap();
}
