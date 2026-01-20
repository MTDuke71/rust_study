//! # Custom Error Types - Building Production-Ready Errors
//!
//! **Date**: Friday (1/23) - Week 4 Day 5
//! **Topic**: Designing Custom Error Types
//!
//! ## Key Concepts
//! - Comprehensive error type design
//! - Error variants for different failure modes
//! - Context preservation and error chains
//! - Thread safety (`Send + Sync + 'static`)
//! - Downcasting support

use std::error::Error;
use std::fmt;
use std::io;
use std::net::AddrParseError;

// ============================================================================
// EXAMPLE 1: Web Server Error (Comprehensive)
// ============================================================================

#[derive(Debug)]
pub enum ServerError {
    /// Failed to bind to address
    Bind {
        addr: String,
        source: io::Error,
    },
    /// Configuration file error
    Config {
        path: String,
        reason: ConfigErrorKind,
    },
    /// Request handling error
    Request {
        method: String,
        path: String,
        reason: String,
    },
    /// Database connection error
    Database {
        operation: String,
        source: Box<dyn Error + Send + Sync>,
    },
    /// Authentication failure
    Auth {
        user: String,
        reason: AuthError,
    },
}

#[derive(Debug)]
pub enum ConfigErrorKind {
    NotFound,
    ParseError(String),
    InvalidValue { field: String, value: String },
}

#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,
    Expired,
    InsufficientPermissions,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::Bind { addr, .. } => {
                write!(f, "failed to bind to address {}", addr)
            }
            ServerError::Config { path, reason } => {
                write!(f, "config error in {}: {:?}", path, reason)
            }
            ServerError::Request { method, path, reason } => {
                write!(f, "{} {} failed: {}", method, path, reason)
            }
            ServerError::Database { operation, .. } => {
                write!(f, "database operation '{}' failed", operation)
            }
            ServerError::Auth { user, reason } => {
                write!(f, "authentication failed for user {}: {:?}", user, reason)
            }
        }
    }
}

impl Error for ServerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ServerError::Bind { source, .. } => Some(source),
            ServerError::Database { source, .. } => Some(source.as_ref()),
            _ => None,
        }
    }
}

// ============================================================================
// EXAMPLE 2: Parser Error with Position Information
// ============================================================================

#[derive(Debug)]
pub struct ParseError {
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
    /// Source code snippet
    pub snippet: String,
    /// Error kind
    pub kind: ParseErrorKind,
}

#[derive(Debug)]
pub enum ParseErrorKind {
    UnexpectedToken { expected: String, found: String },
    UnterminatedString,
    InvalidEscape { sequence: String },
    UnknownIdentifier { name: String },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "parse error at {}:{}", self.line, self.column)?;
        writeln!(f, "  {}", self.snippet)?;
        write!(f, "  ")?;
        for _ in 0..self.column - 1 {
            write!(f, " ")?;
        }
        writeln!(f, "^")?;
        
        match &self.kind {
            ParseErrorKind::UnexpectedToken { expected, found } => {
                write!(f, "  expected {}, found {}", expected, found)
            }
            ParseErrorKind::UnterminatedString => {
                write!(f, "  unterminated string literal")
            }
            ParseErrorKind::InvalidEscape { sequence } => {
                write!(f, "  invalid escape sequence: {}", sequence)
            }
            ParseErrorKind::UnknownIdentifier { name } => {
                write!(f, "  unknown identifier: {}", name)
            }
        }
    }
}

impl Error for ParseError {}

// ============================================================================
// EXAMPLE 3: Network Client Error
// ============================================================================

#[derive(Debug)]
pub enum NetworkError {
    /// Connection failed
    Connect {
        host: String,
        port: u16,
        source: io::Error,
    },
    /// Invalid address format
    InvalidAddress {
        address: String,
        source: AddrParseError,
    },
    /// Timeout occurred
    Timeout {
        operation: String,
        duration_ms: u64,
    },
    /// Protocol error
    Protocol {
        message: String,
        received: Vec<u8>,
    },
    /// TLS/SSL error
    Tls {
        reason: String,
    },
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkError::Connect { host, port, .. } => {
                write!(f, "failed to connect to {}:{}", host, port)
            }
            NetworkError::InvalidAddress { address, .. } => {
                write!(f, "invalid address: {}", address)
            }
            NetworkError::Timeout { operation, duration_ms } => {
                write!(f, "{} timed out after {}ms", operation, duration_ms)
            }
            NetworkError::Protocol { message, received } => {
                write!(f, "protocol error: {} (received {} bytes)", message, received.len())
            }
            NetworkError::Tls { reason } => {
                write!(f, "TLS error: {}", reason)
            }
        }
    }
}

impl Error for NetworkError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            NetworkError::Connect { source, .. } => Some(source),
            NetworkError::InvalidAddress { source, .. } => Some(source),
            _ => None,
        }
    }
}

// ============================================================================
// EXAMPLE 4: Validation Error with Multiple Failures
// ============================================================================

#[derive(Debug)]
pub struct ValidationError {
    /// Field that failed validation
    pub field: String,
    /// List of validation failures
    pub violations: Vec<Violation>,
}

#[derive(Debug)]
pub struct Violation {
    pub rule: String,
    pub message: String,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "validation failed for field '{}':", self.field)?;
        for violation in &self.violations {
            writeln!(f, "  - {}: {}", violation.rule, violation.message)?;
        }
        Ok(())
    }
}

impl Error for ValidationError {}

impl ValidationError {
    pub fn new(field: impl Into<String>) -> Self {
        ValidationError {
            field: field.into(),
            violations: Vec::new(),
        }
    }
    
    pub fn add_violation(&mut self, rule: impl Into<String>, message: impl Into<String>) {
        self.violations.push(Violation {
            rule: rule.into(),
            message: message.into(),
        });
    }
}

// ============================================================================
// EXAMPLE 5: Error with Stack Trace (Custom)
// ============================================================================

#[derive(Debug)]
pub struct ErrorWithTrace {
    message: String,
    trace: Vec<String>,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl ErrorWithTrace {
    pub fn new(message: impl Into<String>) -> Self {
        ErrorWithTrace {
            message: message.into(),
            trace: Vec::new(),
            source: None,
        }
    }
    
    pub fn with_source(message: impl Into<String>, source: impl Error + Send + Sync + 'static) -> Self {
        ErrorWithTrace {
            message: message.into(),
            trace: Vec::new(),
            source: Some(Box::new(source)),
        }
    }
    
    pub fn add_context(mut self, context: impl Into<String>) -> Self {
        self.trace.push(context.into());
        self
    }
}

impl fmt::Display for ErrorWithTrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.message)?;
        
        if !self.trace.is_empty() {
            writeln!(f, "Trace:")?;
            for (i, ctx) in self.trace.iter().enumerate() {
                writeln!(f, "  {}: {}", i, ctx)?;
            }
        }
        
        Ok(())
    }
}

impl Error for ErrorWithTrace {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn Error + 'static))
    }
}

// ============================================================================
// EXAMPLE 6: Error Builder Pattern
// ============================================================================

#[derive(Debug)]
pub struct ApiError {
    status_code: u16,
    message: String,
    details: Option<String>,
    request_id: Option<String>,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl ApiError {
    pub fn new(status_code: u16, message: impl Into<String>) -> Self {
        ApiError {
            status_code,
            message: message.into(),
            details: None,
            request_id: None,
            source: None,
        }
    }
    
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }
    
    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }
    
    pub fn with_source(mut self, source: impl Error + Send + Sync + 'static) -> Self {
        self.source = Some(Box::new(source));
        self
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "API Error {}: {}", self.status_code, self.message)?;
        
        if let Some(details) = &self.details {
            write!(f, " ({})", details)?;
        }
        
        if let Some(request_id) = &self.request_id {
            write!(f, " [request_id: {}]", request_id)?;
        }
        
        Ok(())
    }
}

impl Error for ApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn Error + 'static))
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("=== Custom Error Types ===\n");

    // Example 1: Server error
    println!("1. Server Error:");
    let server_err = ServerError::Auth {
        user: "alice".to_string(),
        reason: AuthError::InsufficientPermissions,
    };
    println!("   {}", server_err);
    println!();

    // Example 2: Parse error with position
    println!("2. Parse Error with Position:");
    let parse_err = ParseError {
        line: 5,
        column: 12,
        snippet: "let x = \"unterminated;".to_string(),
        kind: ParseErrorKind::UnterminatedString,
    };
    println!("{}", parse_err);
    println!();

    // Example 3: Network error
    println!("3. Network Error:");
    let net_err = NetworkError::Timeout {
        operation: "HTTP GET".to_string(),
        duration_ms: 5000,
    };
    println!("   {}", net_err);
    println!();

    // Example 4: Validation error
    println!("4. Validation Error:");
    let mut val_err = ValidationError::new("email");
    val_err.add_violation("format", "must be a valid email address");
    val_err.add_violation("length", "must be at least 5 characters");
    println!("{}", val_err);

    // Example 5: Error with trace
    println!("5. Error with Custom Trace:");
    let trace_err = ErrorWithTrace::new("operation failed")
        .add_context("in module: user_service")
        .add_context("during: authentication")
        .add_context("at: 2024-01-19 10:30:00");
    println!("{}", trace_err);

    // Example 6: API error with builder
    println!("6. API Error (Builder Pattern):");
    let api_err = ApiError::new(404, "Resource not found")
        .with_details("User with ID 123 does not exist")
        .with_request_id("req_abc123xyz");
    println!("   {}", api_err);
}
