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
    balance: f64,
}

#[allow(dead_code)]
impl BankAccount {
    pub fn new(initial_balance: f64) -> Self {
        Self {
            balance: if initial_balance.is_sign_negative() {
                0.0
            } else {
                initial_balance
            },
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        if amount.is_sign_positive() {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount.is_sign_negative() || amount == 0.0 {
            return Err("Withdrawal amount must be positive".into());
        }

        if self.balance >= amount {
            self.balance -= amount;
            Ok(())
        } else {
            Err("Insufficient funds".into())
        }
    }

    pub fn balance(&self) -> f64 {
        self.balance
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

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

#[allow(dead_code)]
pub struct Canvas {
    pub shapes: Vec<Box<dyn Shape>>,
}

#[allow(dead_code)]
impl Canvas {
    pub fn new() -> Self {
        Self { shapes: Vec::new() }
    }

    pub fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape);
    }

    pub fn total_area(&self) -> f64 {
        self.shapes.iter().map(|shape| shape.area()).sum()
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

impl TrafficLightState for RedLight {
    fn next(self: Box<Self>) -> Box<dyn TrafficLightState> {
        Box::new(GreenLight {})
    }

    fn color(&self) -> &str {
        "Red"
    }

    fn duration(&self) -> u32 {
        60
    }
}

impl TrafficLightState for YellowLight {
    fn next(self: Box<Self>) -> Box<dyn TrafficLightState> {
        Box::new(RedLight {})
    }

    fn color(&self) -> &str {
        "Yellow"
    }

    fn duration(&self) -> u32 {
        5
    }
}

impl TrafficLightState for GreenLight {
    fn next(self: Box<Self>) -> Box<dyn TrafficLightState> {
        Box::new(YellowLight {})
    }

    fn color(&self) -> &str {
        "Green"
    }

    fn duration(&self) -> u32 {
        45
    }
}

#[allow(dead_code)]
pub struct TrafficLight {
    state: Option<Box<dyn TrafficLightState>>,
}

#[allow(dead_code)]
impl TrafficLight {
    pub fn new() -> Self {
        Self {
            state: Some(Box::new(RedLight {})),
        }
    }

    pub fn current_color(&self) -> &str {
        self.state
            .as_ref()
            .map(|state| state.color())
            .unwrap_or("Unknown")
    }

    pub fn advance(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.next());
        }
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

#[allow(dead_code)]
impl EmptyDocument {
    pub fn new() -> Self {
        Self {}
    }

    pub fn add_text(self, text: String) -> DraftDocument {
        DraftDocument { content: text }
    }
}

impl Default for EmptyDocument {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl DraftDocument {
    pub fn edit(&mut self, text: String) {
        self.content = text;
    }

    pub fn save(self, filename: String) -> SavedDocument {
        SavedDocument {
            content: self.content,
            filename,
        }
    }
}

#[allow(dead_code)]
impl SavedDocument {
    pub fn open(self) -> DraftDocument {
        DraftDocument {
            content: self.content,
        }
    }
}

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

impl Plugin for LoggingPlugin {
    fn name(&self) -> &str {
        "LoggingPlugin"
    }

    fn execute(&self, data: &str) -> String {
        format!("[LOG] {}", data)
    }
}

impl Plugin for ValidationPlugin {
    fn name(&self) -> &str {
        "ValidationPlugin"
    }

    fn execute(&self, data: &str) -> String {
        if data.trim().is_empty() {
            "Validation failed: empty input".into()
        } else {
            "Validation passed".into()
        }
    }
}

impl Plugin for CachePlugin {
    fn name(&self) -> &str {
        "CachePlugin"
    }

    fn execute(&self, data: &str) -> String {
        format!("Cache lookup for '{}'", data)
    }
}

#[allow(dead_code)]
pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

#[allow(dead_code)]
impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn register(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    pub fn execute_all(&self, data: &str) -> Vec<String> {
        self.plugins
            .iter()
            .map(|plugin| format!("{}: {}", plugin.name(), plugin.execute(data)))
            .collect()
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
/// # OOP VERSION (Runtime Flexibility)
///
/// Trade-offs:
/// + Can store different states in same collection
/// + State can be changed at runtime based on conditions
/// + More familiar to developers from OOP languages
/// - State transitions checked at runtime (can panic/error)
/// - Requires Box allocation for each state
/// - Less compile-time safety
#[allow(dead_code)]
pub trait UserState {
    fn state_name(&self) -> &str;
    fn can_verify_email(&self) -> bool {
        false
    }
    fn can_verify_phone(&self) -> bool {
        false
    }
    fn is_fully_verified(&self) -> bool {
        false
    }
}

#[allow(dead_code)]
pub struct UnverifiedUser {
    pub email: String,
    pub phone: String,
}

#[allow(dead_code)]
pub struct EmailVerifiedUser {
    pub email: String,
    pub phone: String,
}

#[allow(dead_code)]
pub struct PhoneVerifiedUser {
    pub email: String,
    pub phone: String,
}

#[allow(dead_code)]
pub struct FullyVerifiedUser {
    pub email: String,
    pub phone: String,
}

impl UserState for UnverifiedUser {
    fn state_name(&self) -> &str {
        "Unverified"
    }
    fn can_verify_email(&self) -> bool {
        true
    }
    fn can_verify_phone(&self) -> bool {
        true
    }
}

impl UserState for EmailVerifiedUser {
    fn state_name(&self) -> &str {
        "EmailVerified"
    }
    fn can_verify_phone(&self) -> bool {
        true
    }
}

impl UserState for PhoneVerifiedUser {
    fn state_name(&self) -> &str {
        "PhoneVerified"
    }
    fn can_verify_email(&self) -> bool {
        true
    }
}

impl UserState for FullyVerifiedUser {
    fn state_name(&self) -> &str {
        "FullyVerified"
    }
    fn is_fully_verified(&self) -> bool {
        true
    }
}

/// OOP User Registration - runtime state transitions
#[allow(dead_code)]
pub struct OopUserRegistration {
    state: Box<dyn UserState>,
    email: String,
    phone: String,
}

#[allow(dead_code)]
impl OopUserRegistration {
    pub fn new(email: String, phone: String) -> Self {
        Self {
            state: Box::new(UnverifiedUser {
                email: email.clone(),
                phone: phone.clone(),
            }),
            email,
            phone,
        }
    }

    pub fn state_name(&self) -> &str {
        self.state.state_name()
    }

    pub fn verify_email(&mut self) -> Result<(), &'static str> {
        if !self.state.can_verify_email() {
            return Err("Cannot verify email in current state");
        }

        if self.state.state_name() == "PhoneVerified" {
            self.state = Box::new(FullyVerifiedUser {
                email: self.email.clone(),
                phone: self.phone.clone(),
            });
        } else {
            self.state = Box::new(EmailVerifiedUser {
                email: self.email.clone(),
                phone: self.phone.clone(),
            });
        }
        Ok(())
    }

    pub fn verify_phone(&mut self) -> Result<(), &'static str> {
        if !self.state.can_verify_phone() {
            return Err("Cannot verify phone in current state");
        }

        if self.state.state_name() == "EmailVerified" {
            self.state = Box::new(FullyVerifiedUser {
                email: self.email.clone(),
                phone: self.phone.clone(),
            });
        } else {
            self.state = Box::new(PhoneVerifiedUser {
                email: self.email.clone(),
                phone: self.phone.clone(),
            });
        }
        Ok(())
    }

    pub fn is_fully_verified(&self) -> bool {
        self.state.is_fully_verified()
    }
}

/// TYPE STATE VERSION (Compile-Time Safety)
///
/// Trade-offs:
/// + Invalid state transitions are compile errors
/// + Zero runtime overhead for state checking
/// + Self-documenting API via type signatures
/// - Cannot store different states in same collection
/// - More complex type signatures
/// - State must be known at compile time
#[allow(dead_code)]
pub struct Unverified;
#[allow(dead_code)]
pub struct EmailVerified;
#[allow(dead_code)]
pub struct PhoneVerified;
#[allow(dead_code)]
pub struct FullyVerified;

#[allow(dead_code)]
pub struct TypeStateUser<State> {
    email: String,
    phone: String,
    _state: std::marker::PhantomData<State>,
}

#[allow(dead_code)]
impl TypeStateUser<Unverified> {
    pub fn new(email: String, phone: String) -> Self {
        Self {
            email,
            phone,
            _state: std::marker::PhantomData,
        }
    }

    pub fn verify_email(self) -> TypeStateUser<EmailVerified> {
        TypeStateUser {
            email: self.email,
            phone: self.phone,
            _state: std::marker::PhantomData,
        }
    }

    pub fn verify_phone(self) -> TypeStateUser<PhoneVerified> {
        TypeStateUser {
            email: self.email,
            phone: self.phone,
            _state: std::marker::PhantomData,
        }
    }
}

#[allow(dead_code)]
impl TypeStateUser<EmailVerified> {
    pub fn verify_phone(self) -> TypeStateUser<FullyVerified> {
        TypeStateUser {
            email: self.email,
            phone: self.phone,
            _state: std::marker::PhantomData,
        }
    }
}

#[allow(dead_code)]
impl TypeStateUser<PhoneVerified> {
    pub fn verify_email(self) -> TypeStateUser<FullyVerified> {
        TypeStateUser {
            email: self.email,
            phone: self.phone,
            _state: std::marker::PhantomData,
        }
    }
}

#[allow(dead_code)]
impl TypeStateUser<FullyVerified> {
    pub fn get_verified_email(&self) -> &str {
        &self.email
    }

    pub fn get_verified_phone(&self) -> &str {
        &self.phone
    }
}
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
/// HTTP Method enum for builder
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

/// HTTP Response (simplified)
#[derive(Debug)]
#[allow(dead_code)]
pub struct HttpResponse {
    pub status: u16,
    pub body: String,
}

/// Marker types for builder states
#[allow(dead_code)]
pub struct NoMethod;
#[allow(dead_code)]
pub struct HasMethod;
#[allow(dead_code)]
pub struct HasUrl;
#[allow(dead_code)]
pub struct ReadyToSend;

/// Type-safe HTTP Request Builder
///
/// State transitions:
/// NoMethod -> HasMethod (after method())
/// HasMethod -> HasUrl (after url())
/// HasUrl -> ReadyToSend (after build())
/// ReadyToSend -> execute() returns HttpResponse
#[allow(dead_code)]
pub struct HttpRequestBuilder<State> {
    method: Option<HttpMethod>,
    url: Option<String>,
    headers: Vec<(String, String)>,
    body: Option<String>,
    _state: std::marker::PhantomData<State>,
}

#[allow(dead_code)]
impl HttpRequestBuilder<NoMethod> {
    pub fn new() -> Self {
        Self {
            method: None,
            url: None,
            headers: Vec::new(),
            body: None,
            _state: std::marker::PhantomData,
        }
    }

    pub fn method(self, method: HttpMethod) -> HttpRequestBuilder<HasMethod> {
        HttpRequestBuilder {
            method: Some(method),
            url: self.url,
            headers: self.headers,
            body: self.body,
            _state: std::marker::PhantomData,
        }
    }
}

impl Default for HttpRequestBuilder<NoMethod> {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl HttpRequestBuilder<HasMethod> {
    pub fn url(self, url: impl Into<String>) -> HttpRequestBuilder<HasUrl> {
        HttpRequestBuilder {
            method: self.method,
            url: Some(url.into()),
            headers: self.headers,
            body: self.body,
            _state: std::marker::PhantomData,
        }
    }
}

#[allow(dead_code)]
impl HttpRequestBuilder<HasUrl> {
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn build(self) -> HttpRequestBuilder<ReadyToSend> {
        HttpRequestBuilder {
            method: self.method,
            url: self.url,
            headers: self.headers,
            body: self.body,
            _state: std::marker::PhantomData,
        }
    }
}

#[allow(dead_code)]
impl HttpRequestBuilder<ReadyToSend> {
    /// Execute the HTTP request (simulated)
    pub fn execute(self) -> HttpResponse {
        // In real implementation, this would make actual HTTP call
        HttpResponse {
            status: 200,
            body: format!(
                "Executed {:?} request to {} with {} headers",
                self.method.unwrap(),
                self.url.unwrap(),
                self.headers.len()
            ),
        }
    }

    pub fn get_method(&self) -> &HttpMethod {
        self.method.as_ref().unwrap()
    }

    pub fn get_url(&self) -> &str {
        self.url.as_ref().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
    fn test_canvas_shapes() {
        let mut canvas = Canvas::new();

        canvas.add_shape(Box::new(Circle { radius: 5.0 }));
        canvas.add_shape(Box::new(Rectangle {
            width: 4.0,
            height: 6.0,
        }));
        canvas.add_shape(Box::new(Triangle {
            base: 3.0,
            height: 4.0,
        }));

        let total = canvas.total_area();
        let expected = std::f64::consts::PI * 25.0 + 24.0 + 6.0;
        assert!((total - expected).abs() < 0.001);
    }

    #[test]
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

    #[test]
    fn test_document_editor_type_states() {
        // EmptyDocument -> DraftDocument -> SavedDocument -> DraftDocument
        let empty = EmptyDocument::new();

        // Add text transitions to Draft
        let mut draft = empty.add_text("Hello, world!".into());

        // Edit the draft
        draft.edit("Updated content".into());

        // Save transitions to Saved
        let saved = draft.save("document.txt".into());

        // Open transitions back to Draft
        let reopened = saved.open();

        // Verify we can still edit after reopening
        let mut final_draft = reopened;
        final_draft.edit("Final version".into());
    }

    #[test]
    fn test_plugin_system() {
        let mut manager = PluginManager::new();

        manager.register(Box::new(LoggingPlugin {}));
        manager.register(Box::new(ValidationPlugin {}));
        manager.register(Box::new(CachePlugin {}));

        let results = manager.execute_all("test data");

        assert_eq!(results.len(), 3);
        assert!(results[0].contains("LoggingPlugin"));
        assert!(results[0].contains("[LOG]"));
        assert!(results[1].contains("ValidationPlugin"));
        assert!(results[1].contains("passed"));
        assert!(results[2].contains("CachePlugin"));
        assert!(results[2].contains("Cache lookup"));
    }

    #[test]
    fn test_plugin_validation_empty_input() {
        let mut manager = PluginManager::new();
        manager.register(Box::new(ValidationPlugin {}));

        let results = manager.execute_all("   ");
        assert!(results[0].contains("failed"));
    }

    #[test]
    fn test_user_registration_oop() {
        // OOP version - runtime state transitions
        let mut user = OopUserRegistration::new("user@example.com".into(), "555-1234".into());

        assert_eq!(user.state_name(), "Unverified");
        assert!(!user.is_fully_verified());

        // Verify email first
        assert!(user.verify_email().is_ok());
        assert_eq!(user.state_name(), "EmailVerified");

        // Verify phone second -> fully verified
        assert!(user.verify_phone().is_ok());
        assert_eq!(user.state_name(), "FullyVerified");
        assert!(user.is_fully_verified());
    }

    #[test]
    fn test_user_registration_oop_phone_first() {
        // Alternative path: phone first, then email
        let mut user = OopUserRegistration::new("user@example.com".into(), "555-1234".into());

        assert!(user.verify_phone().is_ok());
        assert_eq!(user.state_name(), "PhoneVerified");

        assert!(user.verify_email().is_ok());
        assert_eq!(user.state_name(), "FullyVerified");
    }

    #[test]
    fn test_user_registration_type_state() {
        // Type state version - compile-time safety
        let user = TypeStateUser::<Unverified>::new("user@example.com".into(), "555-1234".into());

        // Each transition consumes the previous state and returns new type
        let email_verified = user.verify_email();
        let fully_verified = email_verified.verify_phone();

        // Only FullyVerified has access to verified getters
        assert_eq!(fully_verified.get_verified_email(), "user@example.com");
        assert_eq!(fully_verified.get_verified_phone(), "555-1234");
    }

    #[test]
    fn test_user_registration_type_state_phone_first() {
        // Alternative path: phone first, then email
        let user = TypeStateUser::<Unverified>::new("user@example.com".into(), "555-1234".into());

        let phone_verified = user.verify_phone();
        let fully_verified = phone_verified.verify_email();

        assert_eq!(fully_verified.get_verified_email(), "user@example.com");
    }

    #[test]
    fn test_http_builder_type_safe() {
        // Type-safe builder pattern - invalid states are compile errors
        let request = HttpRequestBuilder::new()
            .method(HttpMethod::Get)
            .url("https://api.example.com/data")
            .header("Authorization", "Bearer token123")
            .header("Content-Type", "application/json")
            .build();

        assert_eq!(*request.get_method(), HttpMethod::Get);
        assert_eq!(request.get_url(), "https://api.example.com/data");

        let response = request.execute();
        assert_eq!(response.status, 200);
        assert!(response.body.contains("Get"));
        assert!(response.body.contains("2 headers"));
    }

    #[test]
    fn test_http_builder_post_with_body() {
        let request = HttpRequestBuilder::new()
            .method(HttpMethod::Post)
            .url("https://api.example.com/users")
            .header("Content-Type", "application/json")
            .body(r#"{"name": "John"}"#)
            .build();

        let response = request.execute();
        assert!(response.body.contains("Post"));
    }

    // Note: The following code would NOT compile with type state pattern:
    // HttpRequestBuilder::new().url("...")  // Error: url() not available on NoMethod
    // HttpRequestBuilder::new().method(Get).build()  // Error: build() not available on HasMethod
    // This demonstrates compile-time safety!
}
