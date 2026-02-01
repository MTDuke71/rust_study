//! Chapter 3: Review - Designing Interfaces
//!
//! Comprehensive review of all four principles: Unsurprising, Flexible, Obvious, Constrained

use std::borrow::Cow;
use std::marker::PhantomData;
use std::ops::Deref;

fn main() {
    println!("=== Chapter 3 Review: Designing Interfaces ===\n");

    println!("Four Principles:");
    println!("  1. Unsurprising - Familiar, predictable interfaces");
    println!("  2. Flexible - Accept wide inputs, make minimal promises");
    println!("  3. Obvious - Hard to misuse, well-documented");
    println!("  4. Constrained - Evolution without breaking changes\n");

    comprehensive_example();
}

fn comprehensive_example() {
    println!("--- Comprehensive Example: Connection Library ---\n");

    // UNSURPRISING: Familiar naming and common traits
    let mut conn = Connection::new("example.com", Port(443));
    println!("✓ Unsurprising: new(), common traits (Debug, Clone)");
    println!("  Connection: {:?}\n", conn);

    // FLEXIBLE: Generic arguments accept &str or String
    conn.set_host("newhost.com");
    println!("✓ Flexible: Accepts impl AsRef<str>, not just String");

    // OBVIOUS: Type-state prevents sending before connect
    let conn = conn.connect();
    println!("✓ Obvious: Must connect() before send() (type-state)");

    // FLEXIBLE: Cow returns borrowed or owned based on need
    let status = conn.get_status();
    println!("✓ Flexible: Returns Cow<str> - borrowed when possible");
    println!("  Status: {}\n", status);

    // OBVIOUS: must_use on Result forces error handling
    conn.send("data").expect("send failed");
    println!("✓ Obvious: #[must_use] on Result forces handling");

    // FLEXIBLE: Explicit close() for error handling, Drop as fallback
    conn.close().expect("close failed");
    println!("✓ Flexible: Explicit close() + Drop fallback\n");

    // CONSTRAINED: Sealed trait prevents downstream implementations
    println!("✓ Constrained: Sealed trait allows future methods");
    println!("✓ Constrained: #[non_exhaustive] allows adding fields\n");
}

// === Comprehensive Example Implementation ===

// UNSURPRISING: Wrapper type with Deref
#[derive(Debug, Clone)]
struct Port(u16);

impl Deref for Port {
    type Target = u16;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// OBVIOUS: Type-state pattern
#[derive(Debug)]
struct Disconnected;
#[derive(Debug)]
struct Connected;

// CONSTRAINED: #[non_exhaustive] allows evolution
#[non_exhaustive]
#[derive(Debug, Clone)]
struct Connection<State = Disconnected> {
    host: String,
    port: Port,
    _state: PhantomData<State>,
}

impl Connection<Disconnected> {
    // UNSURPRISING: Familiar naming (new)
    pub fn new(host: impl AsRef<str>, port: Port) -> Self {
        Self {
            host: host.as_ref().to_string(),
            port,
            _state: PhantomData,
        }
    }

    // FLEXIBLE: Generic argument accepts &str, String, etc.
    pub fn set_host(&mut self, host: impl AsRef<str>) {
        self.host = host.as_ref().to_string();
    }

    // OBVIOUS: Type-state transition
    pub fn connect(self) -> Connection<Connected> {
        Connection {
            host: self.host.clone(),
            port: self.port.clone(),
            _state: PhantomData,
        }
    }
}

impl Connection<Connected> {
    // OBVIOUS: #[must_use] forces error handling
    #[must_use = "send result must be checked"]
    pub fn send(&self, _data: &str) -> Result<(), String> {
        Ok(())
    }

    // FLEXIBLE: Cow returns borrowed or owned
    pub fn get_status(&self) -> Cow<'_, str> {
        if self.host == "localhost" {
            Cow::Borrowed("local") // Cheap
        } else {
            Cow::Owned(format!("connected to {}", self.host)) // Allocated when needed
        }
    }

    // FLEXIBLE: Explicit close for error handling
    pub fn close(self) -> Result<(), String> {
        // Graceful shutdown with error handling
        Ok(())
    }
}

// FLEXIBLE: Drop as fallback
impl<State> Drop for Connection<State> {
    fn drop(&mut self) {
        // Best-effort cleanup, ignores errors
    }
}

// CONSTRAINED: Sealed trait pattern
mod sealed {
    pub trait Sealed {}
}

pub trait Protocol: sealed::Sealed {
    fn handshake(&self) -> String;
    // Can add methods later without breaking changes
}

#[allow(dead_code)] // Demonstration struct
struct HttpProtocol;
impl sealed::Sealed for HttpProtocol {}
impl Protocol for HttpProtocol {
    fn handshake(&self) -> String {
        "HTTP/1.1".to_string()
    }
}

// === Design Checklist ===

#[cfg(test)]
mod design_checklist {
    // Design Checklist:
    //
    // Unsurprising
    // - [ ] Use familiar names from std
    // - [ ] Implement Debug, Clone, Default
    // - [ ] Implement comparison traits for keys
    // - [ ] Implement IntoIterator for collections
    // - [ ] Use Deref for transparent wrappers
    //
    // Flexible
    // - [ ] Use generics (impl AsRef) not concrete types
    // - [ ] Make traits object-safe when possible
    // - [ ] Accept borrowed when ownership not needed
    // - [ ] Use Cow for conditional ownership
    // - [ ] Provide explicit close() + Drop fallback
    //
    // Obvious
    // - [ ] Document panics, errors, safety
    // - [ ] Include usage examples
    // - [ ] Use semantic types not primitives
    // - [ ] Use type-state for illegal states
    // - [ ] Add #[must_use] to Results
    //
    // Constrained
    // - [ ] Use #[non_exhaustive] for evolution
    // - [ ] Use sealed traits when needed
    // - [ ] Avoid exposing dependency types
    // - [ ] Test Send/Sync preservation
    // - [ ] Minimize public API surface
}
