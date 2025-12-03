//! Practice exercises for Chapter 18: Object-Oriented Programming

/// Exercise 1: Create an encapsulated Bank Account
/// 
/// Implement a BankAccount struct with:
/// - Private balance field
/// - deposit() method
/// - withdraw() method (returns error if insufficient funds)
/// - balance() method to check current balance
/// 
/// Test that:
/// - Cannot access balance directly
/// - Deposit increases balance
/// - Withdraw with sufficient funds works
/// - Withdraw with insufficient funds fails
#[allow(dead_code)]
pub struct BankAccount {
    // TODO: Add private balance field
}

#[allow(dead_code)]
impl BankAccount {
    pub fn new(_initial_balance: f64) -> Self {
        todo!("Create new account with initial balance")
    }

    pub fn deposit(&mut self, _amount: f64) {
        todo!("Add amount to balance")
    }

    pub fn withdraw(&mut self, _amount: f64) -> Result<(), String> {
        todo!("Remove amount from balance if sufficient funds")
    }

    pub fn balance(&self) -> f64 {
        todo!("Return current balance")
    }
}

/// Exercise 2: Create a Shape trait with trait objects
/// 
/// Define a Shape trait with:
/// - area() method
/// 
/// Implement Shape for:
/// - Circle (radius)
/// - Rectangle (width, height)
/// - Triangle (base, height)
/// 
/// Create a Canvas struct that can hold heterogeneous shapes
/// and calculate total area
#[allow(dead_code)]
pub trait Shape {
    fn area(&self) -> f64;
}

#[allow(dead_code)]
pub struct Circle {
    pub radius: f64,
}

#[allow(dead_code)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

#[allow(dead_code)]
pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

// TODO: Implement Shape for Circle, Rectangle, Triangle

#[allow(dead_code)]
pub struct Canvas {
    pub shapes: Vec<Box<dyn Shape>>,
}

#[allow(dead_code)]
impl Canvas {
    pub fn new() -> Self {
        todo!("Create empty canvas")
    }

    pub fn add_shape(&mut self, _shape: Box<dyn Shape>) {
        todo!("Add shape to canvas")
    }

    pub fn total_area(&self) -> f64 {
        todo!("Calculate sum of all shape areas")
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}

/// Exercise 3: Traffic Light State Machine (OOP Pattern)
/// 
/// Implement a traffic light using the OOP state pattern:
/// - States: Red, Yellow, Green
/// - Red -> Green (after timer)
/// - Green -> Yellow -> Red
/// - Each state has different duration
/// 
/// Use trait objects for state
#[allow(dead_code)]
pub trait TrafficLightState {
    fn next(self: Box<Self>) -> Box<dyn TrafficLightState>;
    fn color(&self) -> &str;
    fn duration(&self) -> u32; // seconds
}

#[allow(dead_code)]
pub struct RedLight {}

#[allow(dead_code)]
pub struct YellowLight {}

#[allow(dead_code)]
pub struct GreenLight {}

// TODO: Implement TrafficLightState for Red, Yellow, Green

#[allow(dead_code)]
pub struct TrafficLight {
    state: Option<Box<dyn TrafficLightState>>,
}

#[allow(dead_code)]
impl TrafficLight {
    pub fn new() -> Self {
        todo!("Create traffic light starting at Red")
    }

    pub fn current_color(&self) -> &str {
        todo!("Return current light color")
    }

    pub fn advance(&mut self) {
        todo!("Move to next state")
    }
}

impl Default for TrafficLight {
    fn default() -> Self {
        Self::new()
    }
}

/// Exercise 4: Document Editor (Type State Pattern)
/// 
/// Implement a document editor using type states:
/// - EmptyDocument: can add text -> DraftDocument
/// - DraftDocument: can edit, save, or discard
/// - SavedDocument: can only open (back to Draft)
/// 
/// Ensure compile-time safety for state transitions
#[allow(dead_code)]
pub struct EmptyDocument {}

#[allow(dead_code)]
pub struct DraftDocument {
    content: String,
}

#[allow(dead_code)]
pub struct SavedDocument {
    content: String,
    filename: String,
}

// TODO: Implement methods for each document type
// EmptyDocument::new() -> EmptyDocument
// EmptyDocument::add_text(self, text) -> DraftDocument
// DraftDocument::edit(&mut self, text)
// DraftDocument::save(self, filename) -> SavedDocument
// SavedDocument::open(self) -> DraftDocument

/// Exercise 5: Plugin System
/// 
/// Create a plugin system using trait objects:
/// - Plugin trait with execute() method
/// - LoggingPlugin, ValidationPlugin, CachePlugin
/// - PluginManager to register and run plugins
#[allow(dead_code)]
pub trait Plugin {
    fn name(&self) -> &str;
    fn execute(&self, data: &str) -> String;
}

#[allow(dead_code)]
pub struct LoggingPlugin {}

#[allow(dead_code)]
pub struct ValidationPlugin {}

#[allow(dead_code)]
pub struct CachePlugin {}

// TODO: Implement Plugin for each plugin type

#[allow(dead_code)]
pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

#[allow(dead_code)]
impl PluginManager {
    pub fn new() -> Self {
        todo!("Create empty plugin manager")
    }

    pub fn register(&mut self, _plugin: Box<dyn Plugin>) {
        todo!("Add plugin to manager")
    }

    pub fn execute_all(&self, _data: &str) -> Vec<String> {
        todo!("Run all plugins and collect results")
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Exercise 6: Compare OOP vs Type State
/// 
/// Implement the same workflow both ways and compare:
/// - User registration workflow
/// - States: Unverified, EmailVerified, PhoneVerified, FullyVerified
/// 
/// OOP Version: Use trait objects
/// Type State Version: Use type system
/// 
/// Document trade-offs in comments
/// 
/// TODO: Implement both approaches
/// Exercise 7: Advanced - Builder with Type States
/// 
/// Create a type-safe builder for HTTP requests:
/// - UnsetMethod -> SetMethod (after method())
/// - SetMethod -> SetUrl (after url())
/// - SetUrl -> Ready (after headers/body)
/// - Ready -> execute() returns Response
/// 
/// Each state transition should consume the previous state
/// Invalid states should be impossible to create
#[allow(dead_code)]
pub struct HttpRequestBuilder<State> {
    _state: std::marker::PhantomData<State>,
}

#[allow(dead_code)]
pub struct UnsetMethod;

#[allow(dead_code)]
pub struct SetMethod;

#[allow(dead_code)]
pub struct SetUrl;

#[allow(dead_code)]
pub struct Ready;

// TODO: Implement type-safe builder pattern

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_bank_account() {
        let mut account = BankAccount::new(100.0);
        assert_eq!(account.balance(), 100.0);

        account.deposit(50.0);
        assert_eq!(account.balance(), 150.0);

        assert!(account.withdraw(75.0).is_ok());
        assert_eq!(account.balance(), 75.0);

        assert!(account.withdraw(100.0).is_err());
        assert_eq!(account.balance(), 75.0);
    }

    #[test]
    #[ignore]
    fn test_canvas_shapes() {
        // TODO: Implement Shape trait first
        /*
        let mut canvas = Canvas::new();
        
        canvas.add_shape(Box::new(Circle { radius: 5.0 }));
        canvas.add_shape(Box::new(Rectangle { width: 4.0, height: 6.0 }));
        canvas.add_shape(Box::new(Triangle { base: 3.0, height: 4.0 }));

        let total = canvas.total_area();
        let expected = std::f64::consts::PI * 25.0 + 24.0 + 6.0;
        assert!((total - expected).abs() < 0.001);
        */
    }

    #[test]
    #[ignore]
    fn test_traffic_light() {
        let mut light = TrafficLight::new();
        assert_eq!(light.current_color(), "Red");

        light.advance();
        assert_eq!(light.current_color(), "Green");

        light.advance();
        assert_eq!(light.current_color(), "Yellow");

        light.advance();
        assert_eq!(light.current_color(), "Red");
    }

    // Add more tests for other exercises
}
