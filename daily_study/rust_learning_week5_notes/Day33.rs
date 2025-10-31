// Day 33 - Panic Recovery
// Runnable examples demonstrating panic handling with catch_unwind and panic hooks

#![allow(dead_code, clippy::useless_vec)]

use std::panic;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// Custom error types for examples
#[derive(Debug)]
#[allow(dead_code)]  // Demo types for learning purposes
enum FfiError {
    Panic,
    InvalidInput,
    ResourceUnavailable,
}

#[derive(Debug)]
enum RecoveryError {
    PanicOccurred,
    TooManyPanics,
}

#[derive(Debug)]
#[allow(dead_code)]  // Demo types for learning purposes
enum ProcessResult {
    Success(ProcessedData),
    Fallback(ProcessedData),
}

#[derive(Debug)]
#[allow(dead_code)]  // Demo types for learning purposes
struct ProcessedData {
    content: String,
    metadata: String,
}

#[derive(Debug)]
#[allow(dead_code)]  // Demo types for learning purposes
struct Request {
    path: String,
    method: String,
}

#[derive(Debug)]
#[allow(dead_code)]  // Demo types for learning purposes
struct Response {
    status: u16,
    body: String,
}

#[derive(Debug)]
#[allow(dead_code)]  // Demo types for learning purposes
enum BatchResult {
    Success {
        index: usize,
        item: String,
        processed: ProcessedItem,
    },
    Panic {
        index: usize,
        item: String,
    },
}

#[derive(Debug)]
#[allow(dead_code)]  // Demo types for learning purposes
struct ProcessedItem {
    original: String,
    processed: String,
    status: String,
}

#[derive(Debug)]
#[allow(dead_code)]  // Demo types for learning purposes
struct BatchSummary {
    total: usize,
    success_count: usize,
    panic_count: usize,
    errors: Vec<String>,
}

#[derive(Debug)]
enum PluginResult {
    Success(String),
    PluginPanic,
    PluginNotFound,
}

// Trait for plugins
trait Plugin {
    fn execute(&self, input: &str) -> String;
}

// Example functions demonstrating panic recovery

fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero");
    }
    a / b
}

fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn recover_from_panic() {
    let result = panic::catch_unwind(|| {
        // This code might panic
        let x = [1, 2, 3];
        // Use a dynamic value that the compiler can't optimize away
        let index = std::hint::black_box(10);
        println!("{}", x[index]); // This will panic (index out of bounds)
    });
    
    match result {
        Ok(value) => println!("Operation succeeded: {:?}", value),
        Err(_) => println!("Operation panicked, but we caught it!"),
    }
}

#[allow(dead_code)]  // Demo function for learning purposes
fn setup_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        println!("Custom panic handler:");
        if let Some(location) = panic_info.location() {
            println!("Panic occurred at {}:{}:{}", 
                     location.file(), 
                     location.line(), 
                     location.column());
        }
        if let Some(message) = panic_info.payload().downcast_ref::<&str>() {
            println!("Panic message: {}", message);
        }
    }));
}

fn safe_foreign_call() -> Result<String, &'static str> {
    let result = panic::catch_unwind(|| {
        // Simulate foreign code that might panic
        simulate_foreign_library_call()
    });
    
    match result {
        Ok(value) => Ok(value),
        Err(_) => Err("Foreign library call panicked"),
    }
}

fn simulate_foreign_library_call() -> String {
    // Simulate a panic in foreign code
    panic!("Foreign library encountered an error!");
}

#[allow(dead_code)]  // Demo function for learning purposes
fn safe_ffi_operation<F, R>(operation: F) -> Result<R, FfiError>
where
    F: FnOnce() -> R + panic::UnwindSafe,
{
    panic::catch_unwind(operation)
        .map_err(|_| FfiError::Panic)
}

fn process_data_with_fallback(data: &str) -> ProcessResult {
    let result = panic::catch_unwind(|| {
        // Complex processing that might panic
        complex_data_processing(data)
    });
    
    match result {
        Ok(processed) => ProcessResult::Success(processed),
        Err(_) => {
            // Fallback to simpler processing
            ProcessResult::Fallback(simple_data_processing(data))
        }
    }
}

fn complex_data_processing(data: &str) -> ProcessedData {
    if data.contains("ERROR") {
        panic!("Complex processing failed!");
    }
    
    ProcessedData {
        content: data.to_uppercase(),
        metadata: "complex".to_string(),
    }
}

fn simple_data_processing(data: &str) -> ProcessedData {
    ProcessedData {
        content: data.to_string(),
        metadata: "simple".to_string(),
    }
}

// Web server panic handling
struct WebServer {
    panic_handler: PanicHandler,
}

impl WebServer {
    fn new() -> Self {
        Self {
            panic_handler: PanicHandler::new(),
        }
    }
    
    fn handle_request(&self, request: &Request) -> Response {
        let result = panic::catch_unwind(|| {
            self.process_request(request)
        });
        
        match result {
            Ok(response) => response,
            Err(_) => {
                // Log the panic and return error response
                self.panic_handler.log_panic(&format!("Request panicked: {:?}", request));
                Response::internal_server_error()
            }
        }
    }
    
    fn process_request(&self, request: &Request) -> Response {
        // Simulate request processing that might panic
        if request.path.contains("panic") {
            panic!("Simulated panic in request processing!");
        }
        
        Response::ok(format!("Processed request to {}", request.path))
    }
}

struct PanicHandler {
    panic_count: std::sync::atomic::AtomicUsize,
}

impl PanicHandler {
    fn new() -> Self {
        Self {
            panic_count: std::sync::atomic::AtomicUsize::new(0),
        }
    }
    
    fn log_panic(&self, message: &str) {
        let count = self.panic_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        println!("PANIC #{}: {}", count + 1, message);
    }
}

impl Response {
    fn ok(body: String) -> Self {
        Self { status: 200, body }
    }
    
    fn internal_server_error() -> Self {
        Self {
            status: 500,
            body: "Internal Server Error".to_string(),
        }
    }
}

// Batch processing with panic recovery
struct BatchProcessor {
    results: Arc<Mutex<Vec<BatchResult>>>,
}

impl BatchProcessor {
    fn new() -> Self {
        Self {
            results: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    fn process_batch(&self, items: Vec<String>) -> BatchSummary {
        let mut success_count = 0;
        let mut panic_count = 0;
        let mut errors = Vec::new();
        
        for (index, item) in items.into_iter().enumerate() {
            let result = panic::catch_unwind(|| {
                self.process_item(item.clone())
            });
            
            match result {
                Ok(processed) => {
                    success_count += 1;
                    self.results.lock().unwrap().push(BatchResult::Success {
                        index,
                        item,
                        processed,
                    });
                }
                Err(_) => {
                    panic_count += 1;
                    errors.push(format!("Item {} panicked during processing", index));
                    self.results.lock().unwrap().push(BatchResult::Panic {
                        index,
                        item,
                    });
                }
            }
        }
        
        BatchSummary {
            total: success_count + panic_count,
            success_count,
            panic_count,
            errors,
        }
    }
    
    fn process_item(&self, item: String) -> ProcessedItem {
        // Simulate processing that might panic
        if item.contains("PANIC") {
            panic!("Processing item '{}' caused a panic!", item);
        }
        
        if item.contains("ERROR") {
            return ProcessedItem {
                original: item.clone(),
                processed: format!("ERROR: {}", item),
                status: "error".to_string(),
            };
        }
        
        ProcessedItem {
            original: item.clone(),
            processed: item.to_uppercase(),
            status: "success".to_string(),
        }
    }
}

// Plugin system with panic recovery
struct PluginManager {
    plugins: HashMap<String, Box<dyn Plugin>>,
    panic_handler: PluginPanicHandler,
}

impl PluginManager {
    fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            panic_handler: PluginPanicHandler::new(),
        }
    }
    
    fn register_plugin(&mut self, name: String, plugin: Box<dyn Plugin>) {
        self.plugins.insert(name, plugin);
    }
    
    fn execute_plugin(&self, name: &str, input: &str) -> PluginResult {
        if let Some(plugin) = self.plugins.get(name) {
            let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
                plugin.execute(input)
            }));
            
            match result {
                Ok(output) => PluginResult::Success(output),
                Err(_) => {
                    self.panic_handler.handle_plugin_panic(name);
                    PluginResult::PluginPanic
                }
            }
        } else {
            PluginResult::PluginNotFound
        }
    }
}

struct SafePlugin {
    name: String,
}

impl Plugin for SafePlugin {
    fn execute(&self, input: &str) -> String {
        format!("{} processed: {}", self.name, input)
    }
}

struct UnsafePlugin {
    name: String,
}

impl Plugin for UnsafePlugin {
    fn execute(&self, input: &str) -> String {
        if input.contains("PANIC") {
            panic!("Plugin '{}' panicked!", self.name);
        }
        format!("{} processed: {}", self.name, input)
    }
}

struct PluginPanicHandler {
    panic_log: Arc<Mutex<Vec<String>>>,
}

impl PluginPanicHandler {
    fn new() -> Self {
        Self {
            panic_log: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    fn handle_plugin_panic(&self, plugin_name: &str) {
        let message = format!("Plugin '{}' panicked", plugin_name);
        println!("PLUGIN PANIC: {}", message);
        self.panic_log.lock().unwrap().push(message);
    }
}

// Custom panic recovery
struct PanicRecovery {
    panic_count: Arc<std::sync::atomic::AtomicBool>,
}

impl PanicRecovery {
    fn new() -> Self {
        Self {
            panic_count: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }
    
    fn execute_with_recovery<F, R>(&self, operation: F) -> Result<R, RecoveryError>
    where
        F: FnOnce() -> R + panic::UnwindSafe,
    {
        if self.panic_count.load(std::sync::atomic::Ordering::Relaxed) {
            return Err(RecoveryError::TooManyPanics);
        }
        
        let result = panic::catch_unwind(|| {
            operation()
        });
        
        match result {
            Ok(value) => {
                self.panic_count.store(false, std::sync::atomic::Ordering::Relaxed);
                Ok(value)
            }
            Err(_) => {
                self.panic_count.store(true, std::sync::atomic::Ordering::Relaxed);
                Err(RecoveryError::PanicOccurred)
            }
        }
    }
}

fn main() {
    println!("=== Day 33: Panic Recovery Examples ===\n");
    
    // Example 1: Basic panic recovery
    println!("1. Basic Panic Recovery:");
    recover_from_panic();
    println!();
    
    // Example 2: Safe vs unsafe operations
    println!("2. Safe vs Unsafe Operations:");
    
    // Safe operation
    match safe_divide(10, 2) {
        Ok(result) => println!("Safe divide: 10 / 2 = {}", result),
        Err(e) => println!("Safe divide error: {}", e),
    }
    
    match safe_divide(10, 0) {
        Ok(result) => println!("Safe divide: 10 / 0 = {}", result),
        Err(e) => println!("Safe divide error: {}", e),
    }
    
    // Unsafe operation with recovery
    let result = panic::catch_unwind(|| divide(10, 2));
    match result {
        Ok(value) => println!("Panic-safe divide: 10 / 2 = {}", value),
        Err(_) => println!("Panic occurred in divide"),
    }
    
    let result = panic::catch_unwind(|| divide(10, 0));
    match result {
        Ok(value) => println!("Panic-safe divide: 10 / 0 = {}", value),
        Err(_) => println!("Panic occurred in divide (as expected)"),
    }
    println!();
    
    // Example 3: FFI safety
    println!("3. FFI Safety:");
    match safe_foreign_call() {
        Ok(result) => println!("Foreign call succeeded: {}", result),
        Err(e) => println!("Foreign call failed: {}", e),
    }
    println!();
    
    // Example 4: Graceful degradation
    println!("4. Graceful Degradation:");
    let result = process_data_with_fallback("Hello World");
    println!("Processing result: {:?}", result);
    
    let result = process_data_with_fallback("ERROR");
    println!("Processing result: {:?}", result);
    println!();
    
    // Example 5: Web server panic handling
    println!("5. Web Server Panic Handling:");
    let server = WebServer::new();
    
    let normal_request = Request {
        path: "/api/users".to_string(),
        method: "GET".to_string(),
    };
    let response = server.handle_request(&normal_request);
    println!("Normal request response: {:?}", response);
    
    let panic_request = Request {
        path: "/api/panic".to_string(),
        method: "GET".to_string(),
    };
    let response = server.handle_request(&panic_request);
    println!("Panic request response: {:?}", response);
    println!();
    
    // Example 6: Batch processing with panic recovery
    println!("6. Batch Processing with Panic Recovery:");
    let processor = BatchProcessor::new();
    let items = vec![
        "item1".to_string(),
        "item2_PANIC".to_string(),
        "item3".to_string(),
        "item4_ERROR".to_string(),
    ];
    
    let summary = processor.process_batch(items);
    println!("Batch processing summary: {:?}", summary);
    println!();
    
    // Example 7: Plugin system
    println!("7. Plugin System:");
    let mut plugin_manager = PluginManager::new();
    
    plugin_manager.register_plugin(
        "safe_plugin".to_string(),
        Box::new(SafePlugin { name: "SafePlugin".to_string() }),
    );
    
    plugin_manager.register_plugin(
        "unsafe_plugin".to_string(),
        Box::new(UnsafePlugin { name: "UnsafePlugin".to_string() }),
    );
    
    // Safe plugin execution
    match plugin_manager.execute_plugin("safe_plugin", "test input") {
        PluginResult::Success(output) => println!("Safe plugin result: {}", output),
        PluginResult::PluginPanic => println!("Safe plugin panicked (unexpected)"),
        PluginResult::PluginNotFound => println!("Safe plugin not found"),
    }
    
    // Unsafe plugin execution (normal case)
    match plugin_manager.execute_plugin("unsafe_plugin", "test input") {
        PluginResult::Success(output) => println!("Unsafe plugin result: {}", output),
        PluginResult::PluginPanic => println!("Unsafe plugin panicked"),
        PluginResult::PluginNotFound => println!("Unsafe plugin not found"),
    }
    
    // Unsafe plugin execution (panic case)
    match plugin_manager.execute_plugin("unsafe_plugin", "PANIC input") {
        PluginResult::Success(output) => println!("Unsafe plugin result: {}", output),
        PluginResult::PluginPanic => println!("Unsafe plugin panicked (as expected)"),
        PluginResult::PluginNotFound => println!("Unsafe plugin not found"),
    }
    println!();
    
    // Example 8: Custom panic recovery
    println!("8. Custom Panic Recovery:");
    let recovery = PanicRecovery::new();
    
    // First execution (should succeed)
    match recovery.execute_with_recovery(|| "Hello, World!") {
        Ok(value) => println!("Recovery execution succeeded: {:?}", value),
        Err(e) => println!("Recovery execution failed: {:?}", e),
    }
    
    // Second execution with panic
    match recovery.execute_with_recovery(|| {
        panic!("This will panic!");
    }) {
        Ok(value) => println!("Recovery execution succeeded: {:?}", value),
        Err(e) => println!("Recovery execution failed: {:?}", e),
    }
    
    // Third execution (should fail due to previous panic)
    match recovery.execute_with_recovery(|| "Should not execute") {
        Ok(value) => println!("Recovery execution succeeded: {:?}", value),
        Err(e) => println!("Recovery execution failed: {:?}", e),
    }
    println!();
    
    println!("=== End of Day 33 Examples ===");
}
