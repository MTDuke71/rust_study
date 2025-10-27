# Day 33 - Panic Recovery

**Learning Focus**: `catch_unwind`, panic hooks, and panic handling strategies

---

## 🎯 **Executive Summary**

### **🔑 Core Subject Matter:**
**Panic Recovery** in Rust is about **safely catching and handling panics** using `std::panic::catch_unwind` and panic hooks, enabling graceful degradation and robust error handling in production systems.

### **🎪 Key Learning Areas:**

#### **1. Fundamental Concepts:**
- **Panics vs Errors** - Understanding when code should crash vs return errors
- **`catch_unwind`** - Mechanism to catch panics and prevent program termination
- **Panic Hooks** - Custom handlers that run when panics occur for logging/debugging

#### **2. Practical Patterns:**
- **🛡️ Safe FFI (Foreign Function Interface) Wrappers** - Protecting against panics in foreign/third-party code
- **🔄 Graceful Degradation** - Falling back to simpler processing when complex code panics  
- **🧪 Testing Panic Conditions** - Verifying that code panics when it should
- **⚙️ Custom Recovery Systems** - Building sophisticated panic management

#### **3. Real-World Applications:**
- **🌐 Web Server Resilience** - Handling request processing panics without crashing
- **📦 Batch Processing** - Continuing with remaining items when some panic
- **🔌 Plugin Systems** - Isolating plugin failures from main application

### **⚖️ Critical Decision Framework:**

#### **✅ GOOD Uses for Panic Recovery:**
- **FFI (Foreign Function Interface) calls** that might panic from C libraries
- **Third-party library code** you can't control
- **Plugin/extension systems** to isolate failures
- **Testing scenarios** to verify panic conditions
- **Graceful degradation** with fallback strategies

#### **❌ BAD Uses for Panic Recovery:**
- **Hiding bugs** in your own application logic
- **Regular error handling** (use `Result<T, E>` instead)
- **Making panics invisible** without proper logging
- **Performance-critical paths** (panic recovery has overhead)

### **🧠 Mental Model:**
Think of panic recovery as **"emergency parachutes"** for your code:
- **Not for regular flight** (use proper error handling)
- **Essential for dangerous situations** (FFI, untrusted code)
- **Provides safe landing** (graceful degradation)
- **Includes monitoring** (panic hooks for logging)

### **🎯 Today's Learning Goal:**
Master the **art of selective panic recovery** - knowing when panics should crash your program (bugs) vs when they should be caught (external failures), and building robust systems that can handle both scenarios appropriately.

This is **advanced error handling** that bridges the gap between development (where panics are debugging tools) and production (where system resilience matters most).

### **🏆 The Golden Rule:**
> **"Perfect your own code, protect against everyone else's"**
> 
> - **Your code panicking** = You have a bug to fix
> - **External code panicking** = Tuesday (assume it will happen)

*Note: "Tuesday" is programming humor meaning "just another ordinary day" - external code will fail as routinely as Tuesday arrives each week. Plan for it, don't be surprised by it.*

Write robust, defensive systems that can handle the chaos of the real world while maintaining high standards for your own code quality.

---

## 🎯 Learning Objectives

By the end of this day, you should understand:
- How to catch panics with `std::panic::catch_unwind`
- When and why to use panic recovery
- How to set up panic hooks for debugging
- The difference between panics and regular errors
- Best practices for panic handling in production
- When NOT to catch panics

---

## 📚 Core Concepts

### **Understanding Panics vs Errors**

```rust
// Panics: Unrecoverable conditions that should crash the program
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero!"); // This will crash the program
    }
    a / b
}

// Errors: Recoverable conditions that can be handled
fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}
```

### **Basic Panic Recovery**

```rust
use std::panic;

fn recover_from_panic() {
    let result = panic::catch_unwind(|| {
        // This code might panic
        let x = vec![1, 2, 3];
        println!("{}", x[10]); // This will panic (index out of bounds)
    });
    
    match result {
        Ok(value) => println!("Operation succeeded: {:?}", value),
        Err(_) => println!("Operation panicked, but we caught it!"),
    }
}
```

### **Panic Hooks**

```rust
use std::panic;

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
```

---

## 🔧 Implementation Patterns

### **Pattern 1: Safe FFI Wrappers**

```rust
use std::panic;

// Wrapper for potentially panicking foreign code
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

// More sophisticated FFI wrapper
fn safe_ffi_operation<F, R>(operation: F) -> Result<R, FfiError>
where
    F: FnOnce() -> R + panic::UnwindSafe,
{
    panic::catch_unwind(operation)
        .map_err(|_| FfiError::Panic)
}

#[derive(Debug)]
enum FfiError {
    Panic,
    InvalidInput,
    ResourceUnavailable,
}
```

### **Pattern 2: Graceful Degradation**

```rust
use std::panic;

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

#[derive(Debug)]
enum ProcessResult {
    Success(ProcessedData),
    Fallback(ProcessedData),
}

#[derive(Debug)]
struct ProcessedData {
    content: String,
    metadata: String,
}
```

### **Pattern 3: Testing Panic Conditions**

```rust
use std::panic;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_division_by_zero_panics() {
        let result = panic::catch_unwind(|| {
            divide(10, 0);
        });
        
        assert!(result.is_err(), "Expected panic when dividing by zero");
    }
    
    #[test]
    fn test_valid_division() {
        let result = panic::catch_unwind(|| {
            divide(10, 2)
        });
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5);
    }
    
    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_division_by_zero_should_panic() {
        divide(10, 0);
    }
}

fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero");
    }
    a / b
}
```

### **Pattern 4: Custom Panic Recovery**

```rust
use std::panic;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

struct PanicRecovery {
    panic_count: Arc<AtomicBool>,
}

impl PanicRecovery {
    fn new() -> Self {
        Self {
            panic_count: Arc::new(AtomicBool::new(false)),
        }
    }
    
    fn execute_with_recovery<F, R>(&self, operation: F) -> Result<R, RecoveryError>
    where
        F: FnOnce() -> R + panic::UnwindSafe,
    {
        if self.panic_count.load(Ordering::Relaxed) {
            return Err(RecoveryError::TooManyPanics);
        }
        
        let result = panic::catch_unwind(|| {
            operation()
        });
        
        match result {
            Ok(value) => {
                self.panic_count.store(false, Ordering::Relaxed);
                Ok(value)
            }
            Err(_) => {
                self.panic_count.store(true, Ordering::Relaxed);
                Err(RecoveryError::PanicOccurred)
            }
        }
    }
}

#[derive(Debug)]
enum RecoveryError {
    PanicOccurred,
    TooManyPanics,
}
```

---

## 🎮 Practical Applications

### **Application 1: Web Server Panic Handling**

```rust
use std::panic;
use std::thread;

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

#[derive(Debug)]
struct Request {
    path: String,
    method: String,
}

#[derive(Debug)]
struct Response {
    status: u16,
    body: String,
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
```

### **Application 2: Batch Processing with Panic Recovery**

```rust
use std::panic;
use std::sync::{Arc, Mutex};

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

#[derive(Debug)]
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
struct ProcessedItem {
    original: String,
    processed: String,
    status: String,
}

#[derive(Debug)]
struct BatchSummary {
    total: usize,
    success_count: usize,
    panic_count: usize,
    errors: Vec<String>,
}
```

### **Application 3: Safe Plugin System**

```rust
use std::panic;
use std::collections::HashMap;

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
            let result = panic::catch_unwind(|| {
                plugin.execute(input)
            });
            
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

trait Plugin {
    fn execute(&self, input: &str) -> String;
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

#[derive(Debug)]
enum PluginResult {
    Success(String),
    PluginPanic,
    PluginNotFound,
}
```

---

## 🧪 Best Practices

### **When to Use Panic Recovery**

```rust
// ✅ GOOD: Use panic recovery for:
// - FFI calls that might panic
// - Third-party library code
// - Plugin systems
// - Testing panic conditions
// - Graceful degradation scenarios

fn safe_third_party_call() -> Result<String, ThirdPartyError> {
    panic::catch_unwind(|| {
        third_party_library_function()
    }).map_err(|_| ThirdPartyError::Panic)
}

// ❌ BAD: Don't use panic recovery for:
// - Your own application logic (use Result instead)
// - Hiding bugs in your code
// - Making panics "invisible"

fn bad_panic_handling() -> i32 {
    // This hides bugs and makes debugging difficult
    panic::catch_unwind(|| {
        let x = vec![1, 2, 3];
        x[10] // This should be caught by proper bounds checking
    }).unwrap_or(0)
}
```

### **Panic Hook Best Practices**

```rust
use std::panic;
use std::env;

fn setup_production_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        let location = panic_info.location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "unknown location".to_string());
        
        let message = panic_info.payload()
            .downcast_ref::<&str>()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown panic".to_string());
        
        // In production, you might want to:
        // - Log to a file
        // - Send to monitoring service
        // - Trigger alerts
        
        eprintln!("PANIC: {} at {}", message, location);
        
        // Don't call panic! again in the hook - it will cause a double panic
    }));
}

fn setup_development_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        println!("🐛 Development Panic Handler");
        println!("Location: {:?}", panic_info.location());
        println!("Message: {:?}", panic_info.payload());
        println!("Backtrace would be printed in debug mode");
    }));
}
```

---

## 🎯 Key Takeaways

1. **Use `catch_unwind` sparingly** - Only for FFI, plugins, or testing
2. **Don't hide bugs** - Panics usually indicate programming errors
3. **Set up panic hooks** - For logging and monitoring in production
4. **Consider graceful degradation** - Provide fallbacks when panics occur
5. **Test panic conditions** - Ensure your code panics when it should
6. **Use `Result` for recoverable errors** - Don't use panic recovery as error handling

---

## 🔗 Related Concepts

- **[[Error Handling Patterns]]** - When to use Result vs panic
- **[[Error Handling Practice]]** - Building robust error handling systems
- **[[Testing Strategies]]** - Testing panic conditions
- **[[FFI Safety]]** - Safe foreign function interface patterns
- **[[Production Debugging]]** - Debugging panic conditions
- **[[../../zettelkasten/daily-study/Day33]]** - Zettelkasten redirect page

---

## 🔗 Navigation

**Previous**: [[Day32]] | **Next**: [[Day34]]

**Week Overview**: [[README|Week 5 Overview]]

**Zettelkasten**: [[../../zettelkasten/daily-study/Day33|Day33 (Zettelkasten)]]

---

*Tags: #panic-recovery #catch-unwind #panic-hooks #ffi-safety #error-handling #production-debugging*

*Links: [[zettel-index]] | [[Error Handling Patterns]] | [[Error Handling Practice]] | [[Testing Strategies]] | [[FFI Safety]] | [[Week 5 Overview]]*
