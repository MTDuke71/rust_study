//! Chapter 3: Obvious Interfaces
//!
//! Topics: Documentation, Type System Guidance, Semantic Typing, must_use

use std::marker::PhantomData;

fn main() {
    println!("=== Obvious Interfaces ===\n");

    documentation_demo();
    semantic_typing();
    type_state_pattern();
    must_use_demo();
}

/// Documentation: Make behavior explicit
fn documentation_demo() {
    println!("--- Documentation Best Practices ---");
    println!("  ✅ Document panics, errors, safety requirements");
    println!("  ✅ Include usage examples");
    println!("  ✅ Use #[doc(cfg(...))] for feature gates");
    println!("  ✅ Use #[doc(alias = \"...\")] for discoverability\n");
}

/// Semantic Typing: Use specific types instead of primitives
fn semantic_typing() {
    println!("--- Semantic Typing ---");

    // ❌ BAD: Primitives are ambiguous
    // connect(true, false, 8080);  // Which bool means what?

    // ✅ GOOD: Enums make intent obvious
    let config = ConnectionConfig {
        encryption: Encryption::Enabled,
        compression: Compression::Disabled,
        port: Port(8080),
    };

    println!("  Encryption: {:?}", config.encryption);
    println!("  Compression: {:?}", config.compression);
    println!("  Port: {}", config.port.0);
    println!();
}

/// Type-State Pattern: Make illegal states unrepresentable
fn type_state_pattern() {
    println!("--- Type-State Pattern ---");

    let connection = Connection::<Disconnected>::new();
    println!("  Created connection (Disconnected state)");

    let connection = connection.connect();
    println!("  Connected (Connected state)");

    connection.send_data("Hello!");
    println!("  Sent data (only possible in Connected state)");

    let _connection = connection.disconnect();
    println!("  Disconnected");

    // ❌ COMPILER ERROR: Cannot send in Disconnected state
    // connection.send_data("Nope!");

    println!();
}

/// must_use: Warn when ignoring important return values
fn must_use_demo() {
    println!("--- #[must_use] Annotation ---");

    let _result = important_operation(); // ✅ Used
                                         // important_operation();  // ⚠️ Warning: unused must_use value

    println!("  #[must_use] prevents accidentally ignoring Results\n");
}

// === Semantic Types ===

#[derive(Debug)]
#[allow(dead_code)] // Demonstration enum - not all variants used
enum Encryption {
    Enabled,
    Disabled,
}

#[derive(Debug)]
#[allow(dead_code)] // Demonstration enum - not all variants used
enum Compression {
    Enabled,
    Disabled,
}

struct Port(u16);

struct ConnectionConfig {
    encryption: Encryption,
    compression: Compression,
    port: Port,
}

// === Type-State Pattern ===

struct Disconnected;
struct Connected;

struct Connection<State> {
    _state: PhantomData<State>,
}

impl Connection<Disconnected> {
    fn new() -> Self {
        Self {
            _state: PhantomData,
        }
    }

    fn connect(self) -> Connection<Connected> {
        Connection {
            _state: PhantomData,
        }
    }
}

impl Connection<Connected> {
    fn send_data(&self, data: &str) {
        println!("  Sending: {}", data);
    }

    fn disconnect(self) -> Connection<Disconnected> {
        Connection {
            _state: PhantomData,
        }
    }
}

// === must_use Example ===

#[must_use = "This operation's result must be checked"]
fn important_operation() -> Result<(), String> {
    Ok(())
}

/// Example of well-documented function
///
/// # Arguments
///
/// * `data` - The data to process
///
/// # Panics
///
/// Panics if data is empty.
///
/// # Examples
///
/// ```
/// # use ch03_examples::process_data;
/// let result = process_data("test");
/// assert!(result.is_ok());
/// ```
pub fn process_data(_data: &str) -> Result<(), String> {
    Ok(())
}
