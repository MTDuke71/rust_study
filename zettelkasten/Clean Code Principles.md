# Clean Code Principles

A comprehensive guide to writing clean, maintainable, and professional code based on Robert C. Martin's "Clean Code" principles, adapted for Rust development.

## 🎯 **Core Philosophy**

**"Any fool can write code that a computer can understand. Good programmers write code that humans can understand."** - Martin Fowler

Clean code is:
- **Readable** - Easy to understand by any developer
- **Simple** - Does one thing well
- **Testable** - Easy to verify correctness
- **Maintainable** - Easy to modify and extend
- **Expressive** - Clearly communicates intent

## 📚 **Fundamental Principles**

### **1. Meaningful Names**

Names should reveal intent, avoid disinformation, and make distinctions clear.

#### **✅ Good Examples**

```rust
// ✅ GOOD: Intention-revealing names
fn calculate_total_price(items: &[Item], tax_rate: f64) -> f64 {
    let subtotal = items.iter().map(|item| item.price).sum::<f64>();
    subtotal * (1.0 + tax_rate)
}

struct User {
    email_address: String,
    created_at: DateTime<Utc>,
    is_email_verified: bool,
}

// ✅ GOOD: Searchable names for important concepts
const MAX_RETRY_ATTEMPTS: u32 = 3;
const DATABASE_CONNECTION_TIMEOUT_MS: u64 = 5000;

// ✅ GOOD: Clear distinction between similar concepts
fn validate_user_input(raw_input: &str) -> Result<String, ValidationError> {
    // Validates and sanitizes user input
}

fn validate_system_config(config: &Config) -> Result<(), ConfigError> {
    // Validates system configuration
}
```

#### **❌ Bad Examples**

```rust
// ❌ BAD: Cryptic abbreviations and meaningless names
fn calc(x: &[Item], t: f64) -> f64 {
    let s = x.iter().map(|i| i.p).sum::<f64>();
    s * (1.0 + t)
}

struct U {
    e: String,      // What is 'e'?
    d: DateTime<Utc>, // What is 'd'? 
    f: bool,        // What is 'f'?
}

// ❌ BAD: Magic numbers and unclear names
const MAX: u32 = 3;  // Max what?
const TIMEOUT: u64 = 5000; // Timeout for what? What units?

// ❌ BAD: Misleading names
fn validate_user(input: &str) -> String {
    // Actually sanitizes input, doesn't just validate
    input.trim().to_lowercase()
}
```

#### **Rust-Specific Naming Conventions**

```rust
// ✅ Follow Rust naming conventions
mod user_service;           // snake_case for modules
struct UserAccount;         // PascalCase for types
const MAX_CONNECTIONS: u32; // SCREAMING_SNAKE_CASE for constants
fn process_payment();       // snake_case for functions
let user_count: usize;      // snake_case for variables

// ✅ Use descriptive Iterator method chains
let active_users: Vec<User> = users
    .into_iter()
    .filter(|user| user.is_active)
    .filter(|user| user.last_login_within_days(30))
    .collect();

// ✅ Clear Result/Option handling
match user_repository.find_by_email(&email) {
    Ok(Some(user)) => handle_existing_user(user),
    Ok(None) => handle_user_not_found(),
    Err(error) => handle_database_error(error),
}
```

### **2. Functions Should Be Small**

Functions should do one thing and do it well. They should be small, focused, and have a single level of abstraction.

#### **✅ Good Examples**

```rust
// ✅ GOOD: Small, focused functions
fn validate_email(email: &str) -> Result<(), ValidationError> {
    if email.is_empty() {
        return Err(ValidationError::EmptyEmail);
    }
    
    if !email.contains('@') {
        return Err(ValidationError::InvalidFormat);
    }
    
    Ok(())
}

fn hash_password(password: &str) -> Result<String, HashError> {
    let salt = generate_salt()?;
    let hash = bcrypt::hash_with_salt(password, &salt)?;
    Ok(hash.to_string())
}

fn send_welcome_email(user: &User) -> Result<(), EmailError> {
    let template = load_welcome_template()?;
    let personalized_content = personalize_template(&template, user)?;
    email_service::send(&user.email, "Welcome!", &personalized_content)
}

// ✅ GOOD: Single level of abstraction
fn register_new_user(registration_data: UserRegistration) -> Result<User, RegistrationError> {
    validate_registration_data(&registration_data)?;
    
    let user = create_user_from_registration(registration_data)?;
    let saved_user = save_user_to_database(user)?;
    
    send_welcome_email(&saved_user)?;
    log_user_registration(&saved_user);
    
    Ok(saved_user)
}
```

#### **❌ Bad Examples**

```rust
// ❌ BAD: Function doing too many things
fn process_user_registration(
    email: &str, 
    password: &str, 
    name: &str
) -> Result<User, Box<dyn std::error::Error>> {
    // Validation (first responsibility)
    if email.is_empty() || !email.contains('@') {
        return Err("Invalid email".into());
    }
    if password.len() < 8 {
        return Err("Password too short".into());
    }
    if name.trim().is_empty() {
        return Err("Name required".into());
    }
    
    // Password hashing (second responsibility)
    let mut hasher = DefaultHasher::new();
    password.hash(&mut hasher);
    let password_hash = hasher.finish().to_string();
    
    // Database operations (third responsibility)
    let conn = establish_connection()?;
    let user_id = insert_user(&conn, email, &password_hash, name)?;
    
    // Email sending (fourth responsibility)
    let smtp_server = SmtpTransport::relay("smtp.example.com")?
        .credentials(Credentials::new("user", "pass"))
        .build();
    let email_body = format!("Welcome {}! Thanks for registering.", name);
    let message = Message::builder()
        .from("noreply@example.com".parse()?)
        .to(email.parse()?)
        .subject("Welcome!")
        .body(email_body)?;
    smtp_server.send(&message)?;
    
    // Logging (fifth responsibility) 
    println!("User {} registered successfully", email);
    
    Ok(User {
        id: user_id,
        email: email.to_string(),
        name: name.to_string(),
    })
}
```

#### **Function Size Guidelines**

```rust
// ✅ GOOD: Functions should fit on one screen (20-30 lines max)
fn calculate_shipping_cost(
    items: &[CartItem], 
    destination: &Address, 
    shipping_method: ShippingMethod
) -> Result<Money, ShippingError> {
    let total_weight = calculate_total_weight(items);
    let base_cost = get_base_shipping_cost(&shipping_method);
    let distance_multiplier = calculate_distance_multiplier(destination)?;
    let weight_adjustment = calculate_weight_adjustment(total_weight);
    
    let final_cost = base_cost
        .multiply(distance_multiplier)?
        .add(weight_adjustment)?;
    
    Ok(final_cost)
}

// ✅ GOOD: Extract complex logic into separate functions
fn calculate_total_weight(items: &[CartItem]) -> Weight {
    items.iter()
        .map(|item| item.weight.multiply(item.quantity))
        .sum()
}

fn calculate_distance_multiplier(destination: &Address) -> Result<f64, ShippingError> {
    let distance = geocoding_service::calculate_distance(destination)?;
    Ok(match distance {
        d if d < 50.0 => 1.0,
        d if d < 200.0 => 1.5,
        d if d < 500.0 => 2.0,
        _ => 3.0,
    })
}
```

### **3. Single Responsibility Principle (SRP)**

Every function, struct, and module should have one reason to change.

#### **✅ Good Examples**

```rust
// ✅ GOOD: Single responsibility - only handles user data
struct User {
    id: UserId,
    email: Email,
    name: String,
    created_at: DateTime<Utc>,
}

// ✅ GOOD: Single responsibility - only handles user persistence
struct UserRepository {
    db_connection: DatabaseConnection,
}

impl UserRepository {
    fn save(&self, user: &User) -> Result<(), DatabaseError> { /* */ }
    fn find_by_id(&self, id: UserId) -> Result<Option<User>, DatabaseError> { /* */ }
    fn find_by_email(&self, email: &Email) -> Result<Option<User>, DatabaseError> { /* */ }
    fn delete(&self, id: UserId) -> Result<(), DatabaseError> { /* */ }
}

// ✅ GOOD: Single responsibility - only handles email sending
struct EmailService {
    smtp_client: SmtpClient,
}

impl EmailService {
    fn send_email(&self, to: &Email, subject: &str, body: &str) -> Result<(), EmailError> { /* */ }
    fn send_template_email(&self, to: &Email, template: &EmailTemplate) -> Result<(), EmailError> { /* */ }
}

// ✅ GOOD: Single responsibility - orchestrates user registration
struct UserRegistrationService {
    user_repository: UserRepository,
    email_service: EmailService,
    password_hasher: PasswordHasher,
}

impl UserRegistrationService {
    fn register_user(&self, registration: UserRegistration) -> Result<User, RegistrationError> {
        let validated_registration = self.validate_registration(registration)?;
        let user = self.create_user(validated_registration)?;
        let saved_user = self.user_repository.save(&user)?;
        self.email_service.send_welcome_email(&saved_user)?;
        Ok(saved_user)
    }
}
```

#### **❌ Bad Examples**

```rust
// ❌ BAD: Multiple responsibilities mixed together
struct UserManager {
    db_connection: DatabaseConnection,
    smtp_client: SmtpClient,
    logger: Logger,
}

impl UserManager {
    // Handles validation, persistence, email, and logging - too many responsibilities!
    fn register_user(&self, email: &str, password: &str) -> Result<User, Box<dyn Error>> {
        // Validation responsibility
        if !self.is_valid_email(email) {
            return Err("Invalid email".into());
        }
        
        // Password hashing responsibility  
        let hashed_password = self.hash_password(password)?;
        
        // Database responsibility
        let user = User::new(email, &hashed_password);
        self.save_to_database(&user)?;
        
        // Email responsibility
        self.send_welcome_email(&user.email)?;
        
        // Logging responsibility
        self.logger.info(&format!("User {} registered", email));
        
        Ok(user)
    }
    
    // This struct has too many reasons to change:
    // - Email validation logic changes
    // - Database schema changes  
    // - Email service changes
    // - Logging format changes
}
```

### **4. Don't Repeat Yourself (DRY)**

Every piece of knowledge should have a single, unambiguous representation.

#### **✅ Good Examples**

```rust
// ✅ GOOD: Extract common validation logic
struct ValidationRules;

impl ValidationRules {
    fn validate_email(email: &str) -> Result<(), ValidationError> {
        if email.is_empty() {
            return Err(ValidationError::Required("email".to_string()));
        }
        
        let email_regex = regex::Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$")?;
        if !email_regex.is_match(email) {
            return Err(ValidationError::InvalidFormat("email".to_string()));
        }
        
        Ok(())
    }
    
    fn validate_required_string(value: &str, field_name: &str) -> Result<(), ValidationError> {
        if value.trim().is_empty() {
            return Err(ValidationError::Required(field_name.to_string()));
        }
        Ok(())
    }
}

// ✅ GOOD: Reuse validation logic across different contexts
fn validate_user_registration(registration: &UserRegistration) -> Result<(), ValidationError> {
    ValidationRules::validate_email(&registration.email)?;
    ValidationRules::validate_required_string(&registration.name, "name")?;
    ValidationRules::validate_required_string(&registration.password, "password")?;
    Ok(())
}

fn validate_user_update(update: &UserUpdate) -> Result<(), ValidationError> {
    if let Some(email) = &update.email {
        ValidationRules::validate_email(email)?;
    }
    if let Some(name) = &update.name {
        ValidationRules::validate_required_string(name, "name")?;
    }
    Ok(())
}

// ✅ GOOD: Generic error handling pattern
trait DatabaseRepository<T, ID> {
    fn find_by_id(&self, id: ID) -> Result<Option<T>, DatabaseError>;
    fn save(&self, entity: &T) -> Result<T, DatabaseError>;
    fn delete(&self, id: ID) -> Result<(), DatabaseError>;
}

// ✅ GOOD: Macro for reducing boilerplate (when appropriate)
macro_rules! impl_from_error {
    ($from:ty, $to:ty, $variant:path) => {
        impl From<$from> for $to {
            fn from(error: $from) -> Self {
                $variant(error)
            }
        }
    };
}

#[derive(Debug)]
enum ServiceError {
    Database(DatabaseError),
    Validation(ValidationError),
    Network(NetworkError),
}

impl_from_error!(DatabaseError, ServiceError, ServiceError::Database);
impl_from_error!(ValidationError, ServiceError, ServiceError::Validation);
impl_from_error!(NetworkError, ServiceError, ServiceError::Network);
```

#### **❌ Bad Examples**

```rust
// ❌ BAD: Repeated validation logic
fn validate_user_email(email: &str) -> bool {
    if email.is_empty() {
        return false;
    }
    if !email.contains('@') {
        return false;
    }
    if !email.contains('.') {
        return false;
    }
    true
}

fn validate_admin_email(email: &str) -> bool {
    if email.is_empty() {
        return false;
    }
    if !email.contains('@') {
        return false;
    }
    if !email.contains('.') {
        return false;
    }
    // Duplicated validation logic!
    true
}

// ❌ BAD: Repeated error handling patterns
fn get_user(id: u32) -> Result<User, String> {
    match database::find_user(id) {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err(format!("User with id {} not found", id)),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

fn get_order(id: u32) -> Result<Order, String> {
    match database::find_order(id) {
        Ok(Some(order)) => Ok(order),
        Ok(None) => Err(format!("Order with id {} not found", id)),
        Err(e) => Err(format!("Database error: {}", e)),
    }
    // Same error handling pattern repeated!
}
```

### **5. Functions Should Have No Side Effects**

Functions should do what their name says and nothing more. Avoid hidden behaviors.

#### **✅ Good Examples**

```rust
// ✅ GOOD: Pure function - no side effects
fn calculate_tax(amount: f64, tax_rate: f64) -> f64 {
    amount * tax_rate
}

// ✅ GOOD: Side effects are explicit in the name and return type
fn save_user_to_database(user: &User) -> Result<UserId, DatabaseError> {
    // Clearly indicates it will modify external state (database)
    database::insert_user(user)
}

fn log_and_calculate_total(items: &[Item]) -> f64 {
    // Name explicitly mentions logging side effect
    log::info!("Calculating total for {} items", items.len());
    items.iter().map(|item| item.price).sum()
}

// ✅ GOOD: Separate query from command
fn find_user_by_email(email: &str) -> Result<Option<User>, DatabaseError> {
    // Pure query - no side effects
    database::query_user_by_email(email)
}

fn update_user_last_login(user_id: UserId) -> Result<(), DatabaseError> {
    // Clear command - explicit about modifying state
    database::update_user_last_login(user_id, Utc::now())
}

// ✅ GOOD: Immutable operations
fn add_item_to_cart(cart: Cart, item: Item) -> Cart {
    let mut new_cart = cart;
    new_cart.items.push(item);
    new_cart
}
```

#### **❌ Bad Examples**

```rust
// ❌ BAD: Hidden side effects
static mut GLOBAL_COUNTER: u32 = 0;

fn calculate_user_age(birth_year: u32) -> u32 {
    let current_year = 2025;
    let age = current_year - birth_year;
    
    // Hidden side effect! Function name doesn't suggest this
    unsafe {
        GLOBAL_COUNTER += 1;
        println!("Age calculation #{}", GLOBAL_COUNTER);
    }
    
    age
}

// ❌ BAD: Modifying input parameters unexpectedly
fn get_cart_total(mut cart: Cart) -> f64 {
    // Hidden side effect - modifying the cart while "getting" total
    cart.items.retain(|item| item.price > 0.0); // Removing items!
    cart.apply_discounts(); // Applying discounts!
    
    cart.items.iter().map(|item| item.price).sum()
}

// ❌ BAD: Functions that do more than their name suggests
fn validate_email(email: &str) -> bool {
    let is_valid = email.contains('@') && email.contains('.');
    
    // Hidden side effects!
    if !is_valid {
        log::error!("Invalid email attempted: {}", email);
        send_security_alert(&format!("Invalid email: {}", email)); // Network call!
        increment_failure_counter(); // Global state modification!
    }
    
    is_valid
}
```

### **6. Command Query Separation**

Methods should either do something (command) or return something (query), but not both.

#### **✅ Good Examples**

```rust
// ✅ GOOD: Clear separation of commands and queries
struct UserService {
    repository: UserRepository,
}

impl UserService {
    // QUERY: Returns information, no side effects
    fn find_user_by_id(&self, id: UserId) -> Result<Option<User>, ServiceError> {
        self.repository.find_by_id(id)
    }
    
    // QUERY: Returns information, no side effects
    fn is_email_taken(&self, email: &Email) -> Result<bool, ServiceError> {
        Ok(self.repository.find_by_email(email)?.is_some())
    }
    
    // COMMAND: Performs action, returns success/failure
    fn create_user(&self, user_data: CreateUserRequest) -> Result<(), ServiceError> {
        let user = User::new(user_data.email, user_data.name);
        self.repository.save(&user)?;
        Ok(())
    }
    
    // COMMAND: Performs action, returns success/failure
    fn delete_user(&self, id: UserId) -> Result<(), ServiceError> {
        self.repository.delete(id)?;
        Ok(())
    }
    
    // COMMAND: Clear that this modifies state
    fn update_user_email(&self, id: UserId, new_email: Email) -> Result<(), ServiceError> {
        let mut user = self.repository.find_by_id(id)?
            .ok_or(ServiceError::UserNotFound)?;
        user.email = new_email;
        self.repository.save(&user)?;
        Ok(())
    }
}

// ✅ GOOD: When you need both, make it explicit
impl UserService {
    fn create_user_and_return_id(&self, user_data: CreateUserRequest) -> Result<UserId, ServiceError> {
        // Name explicitly indicates both command and query
        let user = User::new(user_data.email, user_data.name);
        let saved_user = self.repository.save(&user)?;
        Ok(saved_user.id)
    }
}
```

#### **❌ Bad Examples**

```rust
// ❌ BAD: Query that also performs commands
impl UserService {
    fn get_user_by_email(&self, email: &Email) -> Result<Option<User>, ServiceError> {
        // This looks like a query but has hidden side effects!
        
        let user = self.repository.find_by_email(email)?;
        
        if let Some(ref u) = user {
            // Hidden command: updating last accessed time
            self.repository.update_last_accessed(u.id, Utc::now())?;
            
            // Hidden command: logging access
            self.audit_logger.log_user_access(u.id)?;
            
            // Hidden command: incrementing access counter
            self.metrics.increment_user_access_count()?;
        }
        
        Ok(user)
    }
    
    // ❌ BAD: Command that returns query data
    fn delete_user_and_return_name(&self, id: UserId) -> Result<String, ServiceError> {
        // Mixing command (delete) with query (return name)
        let user = self.repository.find_by_id(id)?
            .ok_or(ServiceError::UserNotFound)?;
        
        let name = user.name.clone();
        self.repository.delete(id)?;
        
        Ok(name) // Returns data from deleted entity - confusing!
    }
}
```

### **7. Error Handling as First-Class Citizens**

Handle errors explicitly and meaningfully. Don't ignore or hide errors.

#### **✅ Good Examples**

```rust
// ✅ GOOD: Comprehensive error types
#[derive(Debug, thiserror::Error)]
enum UserServiceError {
    #[error("User not found with id: {id}")]
    UserNotFound { id: UserId },
    
    #[error("Email already exists: {email}")]
    EmailAlreadyExists { email: String },
    
    #[error("Invalid email format: {email}")]
    InvalidEmail { email: String },
    
    #[error("Database error: {source}")]
    Database { #[from] source: DatabaseError },
    
    #[error("Validation error: {message}")]
    Validation { message: String },
}

// ✅ GOOD: Explicit error propagation
fn register_user(request: RegisterUserRequest) -> Result<User, UserServiceError> {
    // Validate input
    validate_email(&request.email)
        .map_err(|_| UserServiceError::InvalidEmail { 
            email: request.email.clone() 
        })?;
    
    // Check for existing user
    if user_exists(&request.email)? {
        return Err(UserServiceError::EmailAlreadyExists { 
            email: request.email 
        });
    }
    
    // Create and save user
    let user = User::new(request.email, request.name);
    save_user(&user)?; // Database error automatically converted
    
    Ok(user)
}

// ✅ GOOD: Graceful error recovery
fn load_user_preferences(user_id: UserId) -> UserPreferences {
    match preferences_repository.load(user_id) {
        Ok(prefs) => prefs,
        Err(PreferencesError::NotFound) => {
            log::info!("No preferences found for user {}, using defaults", user_id);
            UserPreferences::default()
        },
        Err(PreferencesError::Database(db_error)) => {
            log::error!("Database error loading preferences: {}", db_error);
            UserPreferences::default() // Graceful degradation
        },
        Err(other_error) => {
            log::error!("Unexpected error loading preferences: {}", other_error);
            UserPreferences::default()
        }
    }
}

// ✅ GOOD: Context-rich error handling
fn process_payment(payment_request: PaymentRequest) -> Result<PaymentResult, PaymentError> {
    let validated_request = validate_payment_request(payment_request)
        .with_context(|| format!("Invalid payment request for user {}", payment_request.user_id))?;
    
    let payment_method = load_payment_method(validated_request.payment_method_id)
        .with_context(|| "Failed to load payment method")?;
    
    let result = payment_processor.process(validated_request, payment_method)
        .with_context(|| "Payment processor failed")?;
    
    Ok(result)
}
```

#### **❌ Bad Examples**

```rust
// ❌ BAD: Ignoring errors
fn load_config() -> Config {
    let config_result = std::fs::read_to_string("config.toml");
    let config_str = config_result.unwrap(); // Panics on error!
    
    let config: Config = toml::from_str(&config_str).unwrap(); // Panics on error!
    config
}

// ❌ BAD: Swallowing errors silently
fn send_notification(user_id: UserId, message: &str) {
    match notification_service.send(user_id, message) {
        Ok(_) => {} // Success case handled
        Err(_) => {} // Error silently ignored!
    }
}

// ❌ BAD: Vague error types
fn process_user_data(data: &str) -> Result<User, String> {
    if data.is_empty() {
        return Err("bad data".to_string()); // Vague error message
    }
    
    let parsed = serde_json::from_str(data)
        .map_err(|_| "parse failed".to_string())?; // Lost error context
    
    Ok(parsed)
}

// ❌ BAD: Using Result for non-error conditions
fn find_user_by_email(email: &str) -> Result<User, String> {
    match database.query_user(email) {
        Some(user) => Ok(user),
        None => Err("User not found".to_string()), // Not an error, just absence!
    }
}

// Better approach:
fn find_user_by_email(email: &str) -> Result<Option<User>, DatabaseError> {
    database.query_user(email) // None is not an error condition
}
```

### **8. Consistent Formatting and Style**

Code should look like it was written by a single, careful programmer.

#### **✅ Good Examples**

```rust
// ✅ GOOD: Consistent formatting with rustfmt
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ✅ GOOD: Consistent struct formatting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ✅ GOOD: Consistent impl block formatting
impl User {
    pub fn new(email: String, name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            email,
            name,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn update_email(&mut self, new_email: String) {
        self.email = new_email;
        self.updated_at = Utc::now();
    }
    
    pub fn is_email_verified(&self) -> bool {
        // Consistent method formatting
        self.email_verified_at.is_some()
    }
}

// ✅ GOOD: Consistent error handling style
#[derive(Debug)]
pub enum UserError {
    InvalidEmail(String),
    DatabaseError(Box<dyn Error + Send + Sync>),
    NotFound(Uuid),
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserError::InvalidEmail(email) => write!(f, "Invalid email: {}", email),
            UserError::DatabaseError(err) => write!(f, "Database error: {}", err),
            UserError::NotFound(id) => write!(f, "User not found: {}", id),
        }
    }
}

impl Error for UserError {}
```

#### **Team Style Guidelines**

```rust
// ✅ Establish team conventions for common patterns

// Pattern 1: Builder pattern formatting
pub struct UserBuilder {
    email: Option<String>,
    name: Option<String>,
    role: Option<UserRole>,
}

impl UserBuilder {
    pub fn new() -> Self {
        Self {
            email: None,
            name: None,
            role: None,
        }
    }
    
    pub fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }
    
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    
    pub fn role(mut self, role: UserRole) -> Self {
        self.role = Some(role);
        self
    }
    
    pub fn build(self) -> Result<User, BuilderError> {
        Ok(User {
            id: Uuid::new_v4(),
            email: self.email.ok_or(BuilderError::MissingField("email"))?,
            name: self.name.ok_or(BuilderError::MissingField("name"))?,
            role: self.role.unwrap_or(UserRole::Standard),
            created_at: Utc::now(),
        })
    }
}

// Pattern 2: Async function formatting
impl UserService {
    pub async fn create_user(
        &self,
        request: CreateUserRequest,
    ) -> Result<User, UserServiceError> {
        self.validate_create_request(&request).await?;
        
        let user = User::builder()
            .email(request.email)
            .name(request.name)
            .role(request.role.unwrap_or_default())
            .build()?;
            
        self.repository.save(&user).await?;
        
        Ok(user)
    }
}

// Pattern 3: Match expression formatting
fn handle_user_action(action: UserAction) -> Result<(), ActionError> {
    match action {
        UserAction::Login { email, password } => {
            validate_credentials(&email, &password)?;
            log_login_attempt(&email);
            Ok(())
        }
        
        UserAction::Logout { session_id } => {
            invalidate_session(session_id)?;
            log_logout(&session_id);
            Ok(())
        }
        
        UserAction::UpdateProfile { user_id, updates } => {
            validate_profile_updates(&updates)?;
            apply_profile_updates(user_id, updates)?;
            Ok(())
        }
    }
}
```

## 🧪 **Testing Clean Code**

### **Test Structure and Organization**

```rust
// ✅ GOOD: Clear test organization
#[cfg(test)]
mod user_service_tests {
    use super::*;
    use mockall::predicate::*;
    
    mod create_user {
        use super::*;
        
        #[test]
        fn should_create_user_with_valid_data() {
            // Given
            let mut mock_repo = MockUserRepository::new();
            mock_repo
                .expect_save()
                .times(1)
                .returning(|_| Ok(()));
            
            let service = UserService::new(mock_repo);
            let request = CreateUserRequest {
                email: "test@example.com".to_string(),
                name: "Test User".to_string(),
            };
            
            // When
            let result = service.create_user(request);
            
            // Then
            assert!(result.is_ok());
        }
        
        #[test]
        fn should_fail_when_email_already_exists() {
            // Given
            let mut mock_repo = MockUserRepository::new();
            mock_repo
                .expect_find_by_email()
                .with(eq("existing@example.com"))
                .returning(|_| Ok(Some(create_test_user())));
            
            let service = UserService::new(mock_repo);
            let request = CreateUserRequest {
                email: "existing@example.com".to_string(),
                name: "Test User".to_string(),
            };
            
            // When
            let result = service.create_user(request);
            
            // Then
            assert!(matches!(result, Err(UserServiceError::EmailAlreadyExists { .. })));
        }
        
        #[test]
        fn should_fail_with_invalid_email_format() {
            // Given
            let service = UserService::new(MockUserRepository::new());
            let request = CreateUserRequest {
                email: "invalid-email".to_string(),
                name: "Test User".to_string(),
            };
            
            // When
            let result = service.create_user(request);
            
            // Then
            assert!(matches!(result, Err(UserServiceError::InvalidEmail { .. })));
        }
    }
    
    mod update_user {
        use super::*;
        
        // More update-specific tests...
    }
    
    // Test utilities
    fn create_test_user() -> User {
        User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
```

### **Property-Based Testing for Clean Code Verification**

```rust
use proptest::prelude::*;

// ✅ Property-based tests for validating clean code principles
proptest! {
    #[test]
    fn email_validation_should_be_consistent(
        email in "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}"
    ) {
        // Property: Valid emails should always pass validation
        let result1 = validate_email(&email);
        let result2 = validate_email(&email);
        
        prop_assert_eq!(result1.is_ok(), result2.is_ok());
    }
    
    #[test]
    fn user_creation_should_be_deterministic(
        email in "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}",
        name in "[A-Za-z ]{1,50}"
    ) {
        // Property: Same input should produce equivalent users (except ID)
        let user1 = User::new(email.clone(), name.clone());
        let user2 = User::new(email.clone(), name.clone());
        
        prop_assert_eq!(user1.email, user2.email);
        prop_assert_eq!(user1.name, user2.name);
    }
}
```

## 📐 **Architecture and Design**

### **Dependency Injection and Inversion of Control**

```rust
// ✅ GOOD: Depend on abstractions, not concretions
trait UserRepository: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), DatabaseError>;
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, DatabaseError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DatabaseError>;
}

trait EmailService: Send + Sync {
    async fn send_welcome_email(&self, user: &User) -> Result<(), EmailError>;
}

// ✅ GOOD: Service depends on abstractions
struct UserService {
    user_repository: Arc<dyn UserRepository>,
    email_service: Arc<dyn EmailService>,
}

impl UserService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        email_service: Arc<dyn EmailService>,
    ) -> Self {
        Self {
            user_repository,
            email_service,
        }
    }
    
    pub async fn register_user(&self, request: RegisterRequest) -> Result<User, UserError> {
        // Business logic here - not dependent on specific implementations
        let user = User::new(request.email, request.name);
        self.user_repository.save(&user).await?;
        self.email_service.send_welcome_email(&user).await?;
        Ok(user)
    }
}

// ✅ GOOD: Concrete implementations
struct PostgresUserRepository {
    pool: sqlx::PgPool,
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn save(&self, user: &User) -> Result<(), DatabaseError> {
        sqlx::query!(
            "INSERT INTO users (id, email, name) VALUES ($1, $2, $3)",
            user.id,
            user.email,
            user.name
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    // ... other methods
}
```

### **Domain-Driven Design Principles**

```rust
// ✅ GOOD: Rich domain model with behavior
#[derive(Debug, Clone)]
pub struct User {
    id: UserId,
    email: EmailAddress,
    profile: UserProfile,
    account_status: AccountStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    // Business rules are encapsulated in the domain model
    pub fn change_email(&mut self, new_email: EmailAddress) -> Result<(), UserError> {
        if self.account_status == AccountStatus::Suspended {
            return Err(UserError::AccountSuspended);
        }
        
        self.email = new_email;
        self.updated_at = Utc::now();
        Ok(())
    }
    
    pub fn suspend_account(&mut self, reason: SuspensionReason) -> Result<(), UserError> {
        match self.account_status {
            AccountStatus::Active => {
                self.account_status = AccountStatus::Suspended;
                self.updated_at = Utc::now();
                Ok(())
            }
            _ => Err(UserError::InvalidStatusTransition),
        }
    }
    
    pub fn can_perform_action(&self, action: UserAction) -> bool {
        match (self.account_status, action) {
            (AccountStatus::Active, _) => true,
            (AccountStatus::Suspended, UserAction::ViewProfile) => true,
            _ => false,
        }
    }
}

// ✅ GOOD: Value objects for domain concepts
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn new(email: String) -> Result<Self, ValidationError> {
        if Self::is_valid(&email) {
            Ok(Self(email))
        } else {
            Err(ValidationError::InvalidEmail)
        }
    }
    
    fn is_valid(email: &str) -> bool {
        // Email validation logic
        email.contains('@') && email.contains('.') // Simplified
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
```

## 🛠️ **Refactoring Techniques**

### **Extract Method**

```rust
// ❌ BEFORE: Long method with multiple responsibilities
impl OrderService {
    fn process_order(&self, order: Order) -> Result<ProcessedOrder, OrderError> {
        // Validate order items
        for item in &order.items {
            if item.quantity <= 0 {
                return Err(OrderError::InvalidQuantity);
            }
            if !self.inventory.is_available(item.product_id, item.quantity)? {
                return Err(OrderError::InsufficientInventory);
            }
        }
        
        // Calculate totals
        let mut subtotal = 0.0;
        for item in &order.items {
            let product = self.catalog.get_product(item.product_id)?;
            subtotal += product.price * item.quantity as f64;
        }
        
        let tax = subtotal * 0.08;
        let shipping = if subtotal > 100.0 { 0.0 } else { 10.0 };
        let total = subtotal + tax + shipping;
        
        // Process payment
        let payment_request = PaymentRequest {
            amount: total,
            payment_method: order.payment_method,
        };
        
        let payment_result = self.payment_service.process_payment(payment_request)?;
        
        // Update inventory
        for item in &order.items {
            self.inventory.reserve(item.product_id, item.quantity)?;
        }
        
        Ok(ProcessedOrder {
            order_id: order.id,
            total,
            payment_id: payment_result.id,
        })
    }
}

// ✅ AFTER: Extracted methods with single responsibilities
impl OrderService {
    fn process_order(&self, order: Order) -> Result<ProcessedOrder, OrderError> {
        self.validate_order(&order)?;
        
        let order_totals = self.calculate_order_totals(&order)?;
        let payment_result = self.process_payment(&order, order_totals.total)?;
        
        self.reserve_inventory(&order)?;
        
        Ok(ProcessedOrder {
            order_id: order.id,
            total: order_totals.total,
            payment_id: payment_result.id,
        })
    }
    
    fn validate_order(&self, order: &Order) -> Result<(), OrderError> {
        for item in &order.items {
            self.validate_order_item(item)?;
        }
        Ok(())
    }
    
    fn validate_order_item(&self, item: &OrderItem) -> Result<(), OrderError> {
        if item.quantity <= 0 {
            return Err(OrderError::InvalidQuantity);
        }
        
        if !self.inventory.is_available(item.product_id, item.quantity)? {
            return Err(OrderError::InsufficientInventory);
        }
        
        Ok(())
    }
    
    fn calculate_order_totals(&self, order: &Order) -> Result<OrderTotals, OrderError> {
        let subtotal = self.calculate_subtotal(&order.items)?;
        let tax = self.calculate_tax(subtotal);
        let shipping = self.calculate_shipping(subtotal);
        let total = subtotal + tax + shipping;
        
        Ok(OrderTotals {
            subtotal,
            tax,
            shipping,
            total,
        })
    }
    
    fn calculate_subtotal(&self, items: &[OrderItem]) -> Result<f64, OrderError> {
        let mut subtotal = 0.0;
        for item in items {
            let product = self.catalog.get_product(item.product_id)?;
            subtotal += product.price * item.quantity as f64;
        }
        Ok(subtotal)
    }
    
    fn calculate_tax(&self, subtotal: f64) -> f64 {
        subtotal * 0.08
    }
    
    fn calculate_shipping(&self, subtotal: f64) -> f64 {
        if subtotal > 100.0 { 0.0 } else { 10.0 }
    }
    
    fn process_payment(&self, order: &Order, amount: f64) -> Result<PaymentResult, OrderError> {
        let payment_request = PaymentRequest {
            amount,
            payment_method: order.payment_method.clone(),
        };
        
        Ok(self.payment_service.process_payment(payment_request)?)
    }
    
    fn reserve_inventory(&self, order: &Order) -> Result<(), OrderError> {
        for item in &order.items {
            self.inventory.reserve(item.product_id, item.quantity)?;
        }
        Ok(())
    }
}
```

### **Replace Magic Numbers with Named Constants**

```rust
// ❌ BEFORE: Magic numbers scattered throughout code
impl TaxCalculator {
    fn calculate_tax(&self, amount: f64, region: &str) -> f64 {
        match region {
            "CA" => amount * 0.0875,  // What is 0.0875?
            "NY" => amount * 0.08,    // What is 0.08?
            "TX" => amount * 0.0625,  // What is 0.0625?
            _ => amount * 0.05,       // What is 0.05?
        }
    }
    
    fn calculate_shipping(&self, weight: f64) -> f64 {
        if weight <= 1.0 {        // Magic number: 1.0
            5.99                  // Magic number: 5.99
        } else if weight <= 5.0 { // Magic number: 5.0
            9.99                  // Magic number: 9.99
        } else {
            15.99                 // Magic number: 15.99
        }
    }
}

// ✅ AFTER: Named constants with clear meaning
impl TaxCalculator {
    // Tax rates by region
    const CALIFORNIA_TAX_RATE: f64 = 0.0875;
    const NEW_YORK_TAX_RATE: f64 = 0.08;
    const TEXAS_TAX_RATE: f64 = 0.0625;
    const DEFAULT_TAX_RATE: f64 = 0.05;
    
    // Shipping constants
    const LIGHT_PACKAGE_WEIGHT_LBS: f64 = 1.0;
    const MEDIUM_PACKAGE_WEIGHT_LBS: f64 = 5.0;
    const LIGHT_PACKAGE_SHIPPING_COST: f64 = 5.99;
    const MEDIUM_PACKAGE_SHIPPING_COST: f64 = 9.99;
    const HEAVY_PACKAGE_SHIPPING_COST: f64 = 15.99;
    
    fn calculate_tax(&self, amount: f64, region: &str) -> f64 {
        let tax_rate = match region {
            "CA" => Self::CALIFORNIA_TAX_RATE,
            "NY" => Self::NEW_YORK_TAX_RATE,
            "TX" => Self::TEXAS_TAX_RATE,
            _ => Self::DEFAULT_TAX_RATE,
        };
        
        amount * tax_rate
    }
    
    fn calculate_shipping(&self, weight: f64) -> f64 {
        if weight <= Self::LIGHT_PACKAGE_WEIGHT_LBS {
            Self::LIGHT_PACKAGE_SHIPPING_COST
        } else if weight <= Self::MEDIUM_PACKAGE_WEIGHT_LBS {
            Self::MEDIUM_PACKAGE_SHIPPING_COST
        } else {
            Self::HEAVY_PACKAGE_SHIPPING_COST
        }
    }
}
```

## 🔧 **Rust-Specific Clean Code Practices**

### **Leverage Type System for Correctness**

```rust
// ✅ GOOD: Use types to prevent errors
use std::num::NonZeroU32;

#[derive(Debug, Clone, Copy)]
pub struct UserId(NonZeroU32);

impl UserId {
    pub fn new(id: u32) -> Option<Self> {
        NonZeroU32::new(id).map(Self)
    }
}

#[derive(Debug, Clone)]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn new(email: String) -> Result<Self, ValidationError> {
        if Self::is_valid(&email) {
            Ok(Self(email))
        } else {
            Err(ValidationError::InvalidEmail)
        }
    }
    
    fn is_valid(email: &str) -> bool {
        email.contains('@') && !email.is_empty()
    }
}

// ✅ Functions can only be called with valid data
fn send_email_to_user(user_id: UserId, email: EmailAddress, message: &str) -> Result<(), EmailError> {
    // user_id is guaranteed to be non-zero
    // email is guaranteed to be valid format
    email_service.send(user_id, email, message)
}

// ✅ Impossible to pass invalid data at compile time
// send_email_to_user(0, "invalid-email", "message"); // Won't compile!
```

### **Use Iterators Idiomatically**

```rust
// ✅ GOOD: Functional iterator style
impl OrderService {
    fn calculate_total_for_active_orders(&self, orders: &[Order]) -> f64 {
        orders
            .iter()
            .filter(|order| order.status == OrderStatus::Active)
            .filter(|order| order.items.len() > 0)
            .map(|order| self.calculate_order_total(order))
            .sum()
    }
    
    fn find_expensive_items(&self, orders: &[Order], threshold: f64) -> Vec<&OrderItem> {
        orders
            .iter()
            .flat_map(|order| &order.items)
            .filter(|item| item.price > threshold)
            .collect()
    }
    
    fn group_orders_by_status(&self, orders: Vec<Order>) -> HashMap<OrderStatus, Vec<Order>> {
        orders
            .into_iter()
            .into_group_map_by(|order| order.status)
    }
}

// ❌ BAD: Imperative style with manual loops
impl OrderService {
    fn calculate_total_for_active_orders(&self, orders: &[Order]) -> f64 {
        let mut total = 0.0;
        for order in orders {
            if order.status == OrderStatus::Active && order.items.len() > 0 {
                total += self.calculate_order_total(order);
            }
        }
        total
    }
}
```

### **Error Handling with Context**

```rust
use anyhow::{Context, Result};

// ✅ GOOD: Rich error context
async fn load_user_profile(user_id: UserId) -> Result<UserProfile> {
    let user = user_repository
        .find_by_id(user_id)
        .await
        .with_context(|| format!("Failed to query user with id: {}", user_id))?
        .ok_or_else(|| anyhow::anyhow!("User not found: {}", user_id))?;
    
    let preferences = preferences_repository
        .load_for_user(user_id)
        .await
        .with_context(|| format!("Failed to load preferences for user: {}", user_id))?;
    
    let profile_data = external_service
        .get_profile_data(&user.email)
        .await
        .with_context(|| format!("Failed to fetch external profile for: {}", user.email))?;
    
    Ok(UserProfile::new(user, preferences, profile_data))
}
```

## 📊 **Metrics and Measurement**

### **Code Quality Metrics**

```rust
// Use tools to measure clean code principles

// 1. Cyclomatic Complexity - measure with `cargo complexity`
// ✅ GOOD: Low complexity (< 10)
fn calculate_discount(customer_type: CustomerType, order_total: f64) -> f64 {
    match customer_type {
        CustomerType::Regular => 0.0,
        CustomerType::Premium => order_total * 0.05,
        CustomerType::VIP => order_total * 0.10,
    }
}

// ❌ BAD: High complexity (> 15)
fn calculate_complex_discount(
    customer_type: CustomerType,
    order_total: f64,
    is_birthday: bool,
    is_first_order: bool,
    has_coupon: bool,
    season: Season,
) -> f64 {
    // Many nested conditions = high complexity
    if customer_type == CustomerType::VIP {
        if is_birthday {
            if order_total > 100.0 {
                return order_total * 0.20;
            } else {
                return order_total * 0.15;
            }
        } else if is_first_order {
            // ... many more nested conditions
        }
    }
    // ... more complexity
    0.0
}

// 2. Function Length - use `tokei` or `cloc`
// ✅ GOOD: Functions under 20-30 lines
// ❌ BAD: Functions over 50 lines

// 3. Test Coverage - use `cargo tarpaulin`
// ✅ GOOD: >80% test coverage
// ❌ BAD: <60% test coverage
```

### **Automated Quality Gates**

```toml
# Cargo.toml - configure quality tools
[package]
name = "clean-code-example"
version = "0.1.0"
edition = "2021"

[dependencies]
# Production dependencies

[dev-dependencies]
proptest = "1.0"
mockall = "0.11"
criterion = { version = "0.5", features = ["html_reports"] }

# Quality tools (install globally)
# cargo install cargo-audit        # Security vulnerabilities
# cargo install cargo-outdated     # Dependency updates
# cargo install cargo-tarpaulin    # Code coverage
# cargo install cargo-machete      # Unused dependencies
# cargo install cargo-deny         # License and security policies
```

```bash
#!/bin/bash
# quality-check.sh - Automated quality gate script

set -e

echo "🔍 Running quality checks..."

# 1. Compile check
echo "📦 Checking compilation..."
cargo check --all-targets

# 2. Linting
echo "📋 Running clippy..."
cargo clippy --all-targets -- -D warnings

# 3. Formatting
echo "✨ Checking formatting..."
cargo fmt --check

# 4. Tests
echo "🧪 Running tests..."
cargo test --all-features

# 5. Documentation tests
echo "📚 Running doc tests..."
cargo test --doc

# 6. Security audit
echo "🔒 Security audit..."
cargo audit

# 7. Code coverage (if available)
if command -v cargo-tarpaulin &> /dev/null; then
    echo "📊 Measuring coverage..."
    cargo tarpaulin --out xml --output-dir target/tarpaulin
fi

# 8. Dependency check
echo "📦 Checking dependencies..."
cargo outdated --exit-code 1

echo "✅ All quality checks passed!"
```

## 🏆 **Clean Code Checklist**

### **Before Committing Code**

- [ ] **Names are meaningful and reveal intent**
- [ ] **Functions are small (< 30 lines) and do one thing**
- [ ] **No code duplication (DRY principle)**
- [ ] **Functions have no hidden side effects**
- [ ] **Errors are handled explicitly**
- [ ] **Code is properly formatted (rustfmt)**
- [ ] **All tests pass**
- [ ] **No compiler warnings**
- [ ] **Documentation is up to date**

### **Code Review Checklist**

- [ ] **Is the code easy to understand?**
- [ ] **Are the names descriptive and consistent?**
- [ ] **Are functions focused on a single responsibility?**
- [ ] **Is error handling comprehensive?**
- [ ] **Are there adequate tests?**
- [ ] **Is the code maintainable and extensible?**
- [ ] **Does it follow established team conventions?**

### **Refactoring Triggers**

Refactor when you see:
- Functions longer than 30 lines
- More than 3 levels of nesting
- Duplicate code patterns
- Functions with more than 3-4 parameters
- Classes/structs with too many responsibilities
- Hard-to-understand variable names
- Magic numbers or strings
- Long parameter lists
- Comments explaining what (not why)

## 🔗 **Related Resources**

### **Books**
- "Clean Code" by Robert C. Martin
- "Refactoring" by Martin Fowler
- "Code Complete" by Steve McConnell
- "The Pragmatic Programmer" by Hunt & Thomas

### **Rust-Specific Resources**
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [Effective Rust](https://www.lurklurk.org/effective-rust/)

### **Tools for Clean Code**
- `rustfmt` - Automatic code formatting
- `clippy` - Lint tool for catching common mistakes
- `cargo-audit` - Security vulnerability scanner
- `cargo-tarpaulin` - Code coverage tool
- `cargo-machete` - Find unused dependencies

## 🎯 **Next Steps**

### **Practice Exercises**
1. **Refactor Legacy Code**: Take an existing codebase and apply clean code principles
2. **Code Review Practice**: Review others' code with clean code principles in mind
3. **TDD Practice**: Write tests first, then implement clean code to pass them
4. **Extract Method**: Practice breaking large functions into smaller, focused ones
5. **Eliminate Duplication**: Find and remove code duplication in a project

### **Advanced Topics**
- [[software-architecture-patterns]] - Larger-scale design principles
- [[TDD (Test-Driven Development)]] - TDD practices and techniques
- [[refactoring-techniques]] - Systematic code improvement methods
- [[code-review-best-practices]] - Effective peer review processes
- [[technical-debt-management]] - Managing and reducing technical debt

---

*Tags: #clean-code #software-engineering #rust #best-practices #maintainability #readability #testing #refactoring #design-principles*

*Links: [[zettel-index]] | [[software-architecture-patterns]] | [[TDD (Test-Driven Development)]] | [[refactoring-techniques]] | [[rust-best-practices]] | [[code-quality-metrics]] | [[technical-debt-management]]*