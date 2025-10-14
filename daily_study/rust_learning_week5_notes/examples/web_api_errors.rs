// Comprehensive Web API Error Handling Example
// This example demonstrates real-world error handling patterns using anyhow and thiserror

use anyhow::Result;
use thiserror::Error;

// Library error types using thiserror
#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Connection failed: {message}")]
    ConnectionFailed { message: String },

    #[error("Query failed: {sql}")]
    QueryFailed {
        sql: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Transaction failed: {message}")]
    TransactionFailed { message: String },
}

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Field '{field}' is required")]
    RequiredField { field: String },

    #[error("Invalid format for '{field}': {reason}")]
    InvalidFormat { field: String, reason: String },

    #[error("Value '{value}' is out of range: {min} <= value <= {max}")]
    OutOfRange {
        field: String,
        value: String,
        min: String,
        max: String,
    },
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Authentication failed: {reason}")]
    AuthenticationFailed { reason: String },

    #[error("Authorization failed: user {user_id} lacks permission {permission}")]
    AuthorizationFailed { user_id: u32, permission: String },

    #[error("Resource not found: {resource_type} with id {id}")]
    NotFound { resource_type: String, id: String },

    #[error("Validation failed: {field} - {message}")]
    ValidationFailed { field: String, message: String },

    #[error("Internal server error")]
    InternalServerError,

    #[error("External service error: {service}")]
    ExternalServiceError { service: String },
}

impl ApiError {
    #[allow(dead_code)]
    fn status_code(&self) -> u16 {
        match self {
            ApiError::AuthenticationFailed { .. } => 401,
            ApiError::AuthorizationFailed { .. } => 403,
            ApiError::NotFound { .. } => 404,
            ApiError::ValidationFailed { .. } => 400,
            ApiError::InternalServerError => 500,
            ApiError::ExternalServiceError { .. } => 502,
        }
    }
}

// Data structures
#[derive(Debug)]
#[allow(dead_code)]
struct User {
    id: u32,
    name: String,
    email: String,
    age: u32,
}

#[derive(Debug)]
struct CreateUserRequest {
    name: String,
    email: String,
    age: u32,
}

// Application code using anyhow
fn main() -> Result<()> {
    println!("=== Web API Error Handling Example ===\n");

    // Simulate API calls
    let api_handler = ApiHandler::new();

    // Test successful user creation
    let request = CreateUserRequest {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        age: 25,
    };

    match api_handler.create_user(request, "valid_token") {
        Ok(user) => println!("✅ User created: {:?}", user),
        Err(e) => println!("❌ Failed to create user: {}", e),
    }

    // Test validation error
    let invalid_request = CreateUserRequest {
        name: "".to_string(),
        email: "invalid-email".to_string(),
        age: 150,
    };

    match api_handler.create_user(invalid_request, "valid_token") {
        Ok(user) => println!("✅ User created: {:?}", user),
        Err(e) => println!("❌ Failed to create user: {}", e),
    }

    // Test authentication error
    let request = CreateUserRequest {
        name: "Jane Doe".to_string(),
        email: "jane@example.com".to_string(),
        age: 30,
    };

    match api_handler.create_user(request, "invalid_token") {
        Ok(user) => println!("✅ User created: {:?}", user),
        Err(e) => println!("❌ Failed to create user: {}", e),
    }

    Ok(())
}

struct ApiHandler {
    user_service: UserService,
    auth_service: AuthService,
}

impl ApiHandler {
    fn new() -> Self {
        Self {
            user_service: UserService,
            auth_service: AuthService,
        }
    }

    fn create_user(&self, request: CreateUserRequest, token: &str) -> Result<User, ApiError> {
        // Authentication
        let auth_user = self
            .auth_service
            .authenticate(token)
            .map_err(|e| ApiError::AuthenticationFailed { reason: e })?;

        // Authorization
        if !self.auth_service.can_create_user(&auth_user) {
            return Err(ApiError::AuthorizationFailed {
                user_id: auth_user.id,
                permission: "create_user".to_string(),
            });
        }

        // Validation
        validate_user_request(&request)?;

        // Business logic
        let user = self
            .user_service
            .create_user(request)
            .map_err(|e| match e {
                UserServiceError::DuplicateEmail => ApiError::ValidationFailed {
                    field: "email".to_string(),
                    message: "Email already exists".to_string(),
                },
                UserServiceError::DatabaseError(db_err) => match db_err {
                    DatabaseError::ConnectionFailed { .. } => ApiError::ExternalServiceError {
                        service: "database".to_string(),
                    },
                    _ => ApiError::InternalServerError,
                },
            })?;

        Ok(user)
    }
}

fn validate_user_request(request: &CreateUserRequest) -> Result<(), ApiError> {
    if request.name.trim().is_empty() {
        return Err(ApiError::ValidationFailed {
            field: "name".to_string(),
            message: "Name cannot be empty".to_string(),
        });
    }

    if !request.email.contains('@') {
        return Err(ApiError::ValidationFailed {
            field: "email".to_string(),
            message: "Invalid email format".to_string(),
        });
    }

    if request.age < 18 || request.age > 120 {
        return Err(ApiError::ValidationFailed {
            field: "age".to_string(),
            message: "Age must be between 18 and 120".to_string(),
        });
    }

    Ok(())
}

// Simulated services
struct UserService;

impl UserService {
    fn create_user(&self, request: CreateUserRequest) -> Result<User, UserServiceError> {
        // Simulate database operations
        if request.email.contains("duplicate") {
            return Err(UserServiceError::DuplicateEmail);
        }

        if request.email.contains("db_error") {
            return Err(UserServiceError::DatabaseError(
                DatabaseError::ConnectionFailed {
                    message: "Database connection lost".to_string(),
                },
            ));
        }

        Ok(User {
            id: 1,
            name: request.name,
            email: request.email,
            age: request.age,
        })
    }
}

#[derive(Error, Debug)]
enum UserServiceError {
    #[error("Duplicate email address")]
    DuplicateEmail,

    #[error("Database error: {0}")]
    DatabaseError(#[from] DatabaseError),
}

struct AuthService;

impl AuthService {
    fn authenticate(&self, token: &str) -> Result<AuthenticatedUser, String> {
        if token == "invalid" {
            Err("Invalid token".to_string())
        } else {
            Ok(AuthenticatedUser { id: 1 })
        }
    }

    fn can_create_user(&self, _auth_user: &AuthenticatedUser) -> bool {
        true // Simplified
    }
}

#[derive(Debug)]
struct AuthenticatedUser {
    id: u32,
}
