# 🔍 Rest Patterns in Rust

**Comprehensive guide to Rust's rest pattern syntax and usage patterns for ignoring fields in pattern matching**

---

## 🎯 **Overview**

**Rest Patterns** (written as `{ .. }` or `..`) are Rust's way of saying "match this structure but ignore some/all remaining fields." They're essential for clean, maintainable pattern matching when you only care about specific parts of data structures.

### **Key Syntax Forms**
- **`{ .. }`** - Ignore all fields in struct-like patterns
- **`{ field1, .. }`** - Extract some fields, ignore the rest
- **`(.., last)`** - Ignore leading elements in tuples
- **`(first, .., last)`** - Ignore middle elements in tuples

---

## 📋 **Core Syntax Reference**

### **Struct Rest Patterns**
```rust
enum GameState {
    Playing { lives: u32, score: u32, level: u32 },
    GameOver { final_score: u32, time_played: Duration },
}

// Check if playing, ignore all fields
fn is_playing(state: &GameState) -> bool {
    matches!(state, GameState::Playing { .. })
}

// Extract only score, ignore other fields
match game_state {
    GameState::Playing { score, .. } => println!("Score: {}", score),
    GameState::GameOver { .. } => println!("Game finished"),
}
```

### **Tuple Rest Patterns**
```rust
let coordinates = (10, 20, 30, 40, 50);

match coordinates {
    (x, y, ..) => println!("First two: {}, {}", x, y),        // Ignore last 3
    (first, .., last) => println!("Ends: {}, {}", first, last), // Ignore middle 3
}
```

### **Array/Slice Rest Patterns**
```rust
let numbers = [1, 2, 3, 4, 5];

match numbers {
    [first, .., last] => println!("First: {}, Last: {}", first, last),
    [only] => println!("Single element: {}", only),
    [] => println!("Empty array"),
}
```

---

## 🔧 **Common Use Cases**

### **1. State Machine Pattern Matching**
```rust
// From Day 14 Reindeer Olympics - State Machine
enum ReindeerState {
    Flying { time_remaining: u32 },
    Resting { time_remaining: u32 },
}

fn is_flying(&self) -> bool {
    matches!(self.state, ReindeerState::Flying { .. })
    //                                           ^^^^
    //                                      Rest Pattern
}
```

### **2. Configuration Structs**
```rust
struct ServerConfig {
    host: String,
    port: u16,
    database_url: String,
    max_connections: u32,
    timeout_seconds: u32,
    retry_attempts: u32,
    // ... many more fields
}

// Only care about host for logging
match config {
    ServerConfig { host, .. } => {
        println!("Connecting to: {}", host);
        // Ignore all other configuration details
    }
}
```

### **3. Error Handling with Complex Error Types**
```rust
enum DatabaseError {
    ConnectionFailed { host: String, port: u16, reason: String },
    QueryTimeout { query: String, duration: Duration },
    PermissionDenied { user: String, table: String, operation: String },
}

// Handle errors by type, ignore specific details in logs
match error {
    DatabaseError::ConnectionFailed { .. } => log::error!("Database connection failed"),
    DatabaseError::QueryTimeout { .. } => log::warn!("Query timed out"),
    DatabaseError::PermissionDenied { .. } => log::error!("Access denied"),
}
```

### **4. HTTP Response Processing**
```rust
struct HttpResponse {
    status: u16,
    headers: HashMap<String, String>,
    body: String,
    timestamp: SystemTime,
    request_id: String,
}

// Route by status code, ignore response details
match response {
    HttpResponse { status: 200, .. } => handle_success(),
    HttpResponse { status: 404, .. } => handle_not_found(),
    HttpResponse { status: 500..=599, .. } => handle_server_error(),
    HttpResponse { status, .. } => handle_other_status(status),
}
```

---

## 🌟 **Advanced Patterns**

### **1. Nested Rest Patterns**
```rust
struct Player {
    name: String,
    stats: PlayerStats,
    inventory: Vec<Item>,
}

struct PlayerStats {
    health: u32,
    mana: u32,
    experience: u64,
    level: u32,
}

// Extract player name and health, ignore everything else
match player {
    Player { 
        name, 
        stats: PlayerStats { health, .. }, 
        .. 
    } => {
        println!("{} has {} health", name, health);
    }
}
```

### **2. Rest Patterns with Guards**
```rust
enum Temperature {
    Celsius { degrees: f32, humidity: f32, pressure: f32 },
    Fahrenheit { degrees: f32, humidity: f32, pressure: f32 },
}

match temp {
    Temperature::Celsius { degrees, .. } if degrees > 30.0 => {
        println!("Hot day in Celsius!");
    }
    Temperature::Fahrenheit { degrees, .. } if degrees > 86.0 => {
        println!("Hot day in Fahrenheit!");
    }
    _ => println!("Moderate temperature"),
}
```

### **3. Rest Patterns in Function Parameters**
```rust
// Extract first and last elements from slice
fn process_endpoints(data: &[i32]) {
    match data {
        [first, .., last] => {
            println!("Processing from {} to {}", first, last);
            // Ignore middle elements
        }
        [single] => println!("Single element: {}", single),
        [] => println!("Empty data"),
    }
}
```

---

## ⚡ **Performance Considerations**

### **Zero-Cost Abstraction**
```rust
// Both compile to identical assembly
fn check_state_verbose(state: &ReindeerState) -> bool {
    match state {
        ReindeerState::Flying { time_remaining: _ } => true,
        ReindeerState::Resting { time_remaining: _ } => false,
    }
}

fn check_state_rest(state: &ReindeerState) -> bool {
    matches!(state, ReindeerState::Flying { .. })
}
```

### **Memory Efficiency**
- ✅ **No runtime cost** - patterns are compile-time constructs
- ✅ **No field access** - ignored fields aren't read from memory
- ✅ **Optimal assembly** - identical to manual enum discrimination

---

## 🔄 **Alternative Approaches**

### **Without Rest Patterns (Verbose)**
```rust
// Explicit field naming - cluttered and brittle
match config {
    ServerConfig { 
        host, 
        port: _, 
        database_url: _, 
        max_connections: _, 
        timeout_seconds: _, 
        retry_attempts: _ 
    } => {
        println!("Host: {}", host);
    }
}
```

### **With Rest Patterns (Clean)**
```rust
// Clean and forward-compatible
match config {
    ServerConfig { host, .. } => {
        println!("Host: {}", host);
    }
}
```

---

## 🛠️ **Best Practices**

### **1. Forward Compatibility**
```rust
// ✅ Good: Adding new fields won't break existing code
struct User {
    name: String,
    email: String,
    // Future: phone: Option<String>,  // Can add without breaking matches
    // Future: preferences: UserPrefs, // Rest patterns handle this
}

match user {
    User { name, .. } => println!("Hello, {}", name),
}
```

### **2. Clear Intent**
```rust
// ✅ Good: Clear what you care about
match event {
    UserEvent::Login { user_id, .. } => handle_login(user_id),
    UserEvent::Logout { .. } => handle_logout(),
}

// ❌ Avoid: Unclear intent
match event {
    UserEvent::Login { user_id, timestamp: _, session_id: _, ip: _ } => {
        handle_login(user_id)
    }
}
```

### **3. Documentation Value**
```rust
// ✅ Good: Self-documenting - "only care about error type"
match result {
    Err(NetworkError::Timeout { .. }) => retry_request(),
    Err(NetworkError::Unauthorized { .. }) => refresh_auth(),
    Ok(response) => process_response(response),
}
```

---

## 🧪 **Testing Patterns**

### **State Machine Testing**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_state_transitions() {
        let mut reindeer = StateMachineReindeer::new("Comet", 14, 10, 127);
        
        // Use rest patterns for clean state checking
        assert!(matches!(reindeer.state, ReindeerState::Flying { .. }));
        
        reindeer.tick();
        // Still flying after 1 second
        assert!(matches!(reindeer.state, ReindeerState::Flying { .. }));
        
        // Advance to resting state
        for _ in 0..9 { reindeer.tick(); }
        assert!(matches!(reindeer.state, ReindeerState::Resting { .. }));
    }
    
    #[test]
    fn test_specific_field_values() {
        let mut reindeer = StateMachineReindeer::new("Comet", 14, 10, 127);
        
        // When you DO care about specific field values
        match reindeer.state {
            ReindeerState::Flying { time_remaining } => {
                assert_eq!(time_remaining, 10);
            }
            _ => panic!("Should start flying"),
        }
    }
}
```

---

## 🎓 **Learning Progression**

### **Beginner Level**
- Use `{ .. }` to ignore all fields when checking enum variants
- Apply in `matches!` macro for simple boolean checks

### **Intermediate Level**
- Mix field extraction with rest patterns: `{ field1, field2, .. }`
- Use in complex match statements with multiple patterns

### **Advanced Level**
- Nested rest patterns in complex data structures
- Combine with pattern guards and range patterns
- Design APIs that leverage rest patterns for forward compatibility

---

## 🔗 **Related Concepts**

### **Pattern Matching Hierarchy**
- [[Pattern Matching MOC]] - Complete pattern matching guide
- [[Enum Patterns]] - Enum-specific pattern matching techniques
- [[Struct Destructuring]] - Extracting values from structs
- [[Tuple Patterns]] - Working with tuple destructuring

### **Rust Language Features**
- [[Matches Macro]] - `matches!` macro usage patterns
- [[Match Expressions]] - Comprehensive match statement guide
- [[If Let Patterns]] - Alternative pattern matching syntax
- [[While Let Patterns]] - Loop-based pattern matching

### **Real-World Applications**
- [[State Machine Patterns]] - State machine implementation in Rust
- [[Error Handling Patterns]] - Pattern matching for error types
- [[Configuration Parsing]] - Processing complex configuration structs
- [[API Design Patterns]] - Forward-compatible API design

---

## 📚 **Examples from Codebase**

### **Day 14 State Machine** (AoC 2015)
```rust
// From: advent_of_code/aoc2015/examples/Day14_state_machine.rs
fn is_flying(&self) -> bool {
    matches!(self.state, ReindeerState::Flying { .. })
}
```

### **Mission System Integration**
- [[Mission5 HashMap]] - Pattern matching with complex data structures
- [[Mission7 Graph]] - State pattern matching in graph algorithms
- [[Mission8 BFS]] - Rest patterns in algorithm state tracking

---

*Tags: #rust-patterns #pattern-matching #rest-patterns #struct-destructuring #enum-patterns #matches-macro #state-machines #code-clarity #forward-compatibility #best-practices #aoc2015 #day14 #reindeer-olympics*

*Links: [[zettel-index]] | [[Pattern Matching MOC]] | [[Matches Macro]] | [[Enum Patterns]] | [[State Machine Patterns]] | [[AoC 2015 MOC]] | [[daily-study/Day14]] | [[Rust Concepts MOC]] | [[Mission System Architecture]]*