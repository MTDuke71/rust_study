// Day 35 - Error Handling Practice
// Runnable examples demonstrating robust parsers with comprehensive error handling

#[allow(dead_code, unused_variables, unused_assignments, unused_mut)]
use std::collections::HashMap;

// Custom error types and structures
#[derive(Debug)]
pub struct ParserConfig {
    pub strict_mode: bool,
    pub max_errors: usize,
    pub continue_on_error: bool,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            strict_mode: false,
            max_errors: 10,
            continue_on_error: true,
        }
    }
}

#[derive(Debug)]
pub struct ParseResult<T> {
    pub data: T,
    pub warnings: Vec<ParseWarning>,
    pub errors: Vec<ParseError>,
}

#[derive(Debug, Clone)]
pub enum ParseError {
    UnexpectedToken { expected: String, found: String, line: usize, column: usize },
    InvalidFormat { field: String, value: String, line: usize },
    MissingField { field: String, line: usize },
    DuplicateField { field: String, line: usize },
    RangeError { field: String, value: String, min: String, max: String },
    Custom { message: String, line: Option<usize> },
}

#[derive(Debug)]
pub enum ParseWarning {
    DeprecatedField { field: String, line: usize },
    UnknownField { field: String, line: usize },
    FormatSuggestion { field: String, suggestion: String, line: usize },
}

#[derive(Debug)]
pub enum ValidationError {
    InvalidType { expected: String, found: String },
    OutOfRange { min: String, max: String, found: String },
    InvalidFormat { message: String },
}

#[derive(Debug)]
pub enum ProcessingError {
    ProcessorError { processor: String, error: String },
    ValidationError { field: String, message: String },
}

#[derive(Debug, Clone)]
pub enum ConfigValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

impl std::fmt::Display for ConfigValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigValue::String(s) => write!(f, "{}", s),
            ConfigValue::Integer(i) => write!(f, "{}", i),
            ConfigValue::Float(fl) => write!(f, "{}", fl),
            ConfigValue::Boolean(b) => write!(f, "{}", b),
        }
    }
}

// Error recovery trait and implementation
pub trait ErrorRecovery {
    fn can_recover(&self, error: &ParseError) -> bool;
    fn recover(&self, error: ParseError, context: &ParseContext) -> Result<(), ParseError>;
}

#[derive(Debug)]
pub struct ParseContext {
    pub line_number: usize,
    pub column: usize,
    pub field_name: Option<String>,
    pub previous_values: HashMap<String, String>,
}

pub struct StandardErrorRecovery;

impl ErrorRecovery for StandardErrorRecovery {
    fn can_recover(&self, error: &ParseError) -> bool {
        match error {
            ParseError::InvalidFormat { .. } => true,
            ParseError::MissingField { .. } => false,
            ParseError::DuplicateField { .. } => false,
            _ => false,
        }
    }
    
    fn recover(&self, error: ParseError, _context: &ParseContext) -> Result<(), ParseError> {
        match &error {
            ParseError::InvalidFormat { field: _, value, line: _ } => {
                // Try to clean the value
                let cleaned_value = value.trim().to_lowercase();
                if !cleaned_value.is_empty() {
                    return Ok(());
                }
                Err(error)
            }
            _ => Err(error),
        }
    }
}

// CSV Parser implementation
pub struct CsvParser {
    config: ParserConfig,
    error_recovery: Box<dyn ErrorRecovery>,
}

impl CsvParser {
    pub fn new(config: ParserConfig, error_recovery: Box<dyn ErrorRecovery>) -> Self {
        Self {
            config,
            error_recovery,
        }
    }
    
    pub fn parse<T>(&self, content: &str) -> ParseResult<Vec<T>>
    where
        T: FromCsvRow,
    {
        let mut results = Vec::new();
        let mut warnings = Vec::new();
        let mut errors = Vec::new();
        
        let lines: Vec<&str> = content.lines().collect();
        let headers = if !lines.is_empty() {
            match self.parse_headers(lines[0], &mut warnings, &mut errors) {
                Ok(headers) => headers,
                Err(_) => {
                    return ParseResult {
                        data: results,
                        warnings,
                        errors,
                    };
                }
            }
        } else {
            return ParseResult {
                data: results,
                warnings,
                errors,
            };
        };
        
        for (index, line) in lines.iter().enumerate().skip(1) {
            let line_number = index + 1;
            
            if line.trim().is_empty() {
                continue;
            }
            
            match self.parse_row(line, &headers, line_number) {
                Ok(row) => results.push(row),
                Err(mut error) => {
                    if self.error_recovery.can_recover(&error) {
                        match self.error_recovery.recover(error.clone(), &ParseContext {
                            line_number,
                            column: 0,
                            field_name: None,
                            previous_values: HashMap::new(),
                        }) {
                            Ok(()) => {
                                // Try parsing again with recovered state
                                if let Ok(row) = self.parse_row(line, &headers, line_number) {
                                    results.push(row);
                                    continue;
                                }
                            }
                            Err(recovery_error) => error = recovery_error,
                        }
                    }
                    
                    errors.push(error);
                    
                    if !self.config.continue_on_error || errors.len() >= self.config.max_errors {
                        break;
                    }
                }
            }
        }
        
        ParseResult {
            data: results,
            warnings,
            errors,
        }
    }
    
    fn parse_headers(&self, line: &str, _warnings: &mut [ParseWarning], errors: &mut Vec<ParseError>) -> Result<Vec<String>, ParseError> {
        let headers: Vec<String> = line.split(',').map(|s| s.trim().to_string()).collect();
        
        if headers.is_empty() {
            return Err(ParseError::Custom {
                message: "Empty header row".to_string(),
                line: Some(1),
            });
        }
        
        // Check for duplicate headers
        let mut seen = HashMap::new();
        for (index, header) in headers.iter().enumerate() {
            if seen.contains_key(header) {
                errors.push(ParseError::DuplicateField {
                    field: header.clone(),
                    line: 1,
                });
            } else {
                seen.insert(header, index);
            }
        }
        
        Ok(headers)
    }
    
    fn parse_row<T>(&self, line: &str, headers: &[String], line_number: usize) -> Result<T, ParseError>
    where
        T: FromCsvRow,
    {
        let values: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        
        if values.len() != headers.len() {
            return Err(ParseError::InvalidFormat {
                field: "row".to_string(),
                value: format!("Expected {} fields, got {}", headers.len(), values.len()),
                line: line_number,
            });
        }
        
        let mut row_data = HashMap::new();
        for (header, value) in headers.iter().zip(values.iter()) {
            row_data.insert(header.clone(), value.to_string());
        }
        
        T::from_csv_row(row_data, line_number)
    }
}

pub trait FromCsvRow {
    fn from_csv_row(data: HashMap<String, String>, line_number: usize) -> Result<Self, ParseError>
    where
        Self: Sized;
}

// Example implementation
#[derive(Debug)]
pub struct UserRecord {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub age: Option<u32>,
}

impl FromCsvRow for UserRecord {
    fn from_csv_row(data: HashMap<String, String>, line_number: usize) -> Result<Self, ParseError> {
        let id = data.get("id")
            .ok_or_else(|| ParseError::MissingField {
                field: "id".to_string(),
                line: line_number,
            })?
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidFormat {
                field: "id".to_string(),
                value: data.get("id").unwrap().clone(),
                line: line_number,
            })?;
        
        let name = data.get("name")
            .ok_or_else(|| ParseError::MissingField {
                field: "name".to_string(),
                line: line_number,
            })?
            .clone();
        
        let email = data.get("email")
            .ok_or_else(|| ParseError::MissingField {
                field: "email".to_string(),
                line: line_number,
            })?
            .clone();
        
        if !email.contains('@') {
            return Err(ParseError::InvalidFormat {
                field: "email".to_string(),
                value: email,
                line: line_number,
            });
        }
        
        let age = data.get("age")
            .and_then(|age_str| {
                if age_str.is_empty() {
                    None
                } else {
                    age_str.parse::<u32>().ok()
                }
            });
        
        Ok(UserRecord {
            id,
            name,
            email,
            age,
        })
    }
}

// Configuration Parser
pub struct ConfigParser {
    config: ParserConfig,
    validators: HashMap<String, Box<dyn FieldValidator>>,
}

impl ConfigParser {
    pub fn new(config: ParserConfig) -> Self {
        Self {
            config,
            validators: HashMap::new(),
        }
    }
    
    pub fn add_validator(&mut self, field_name: String, validator: Box<dyn FieldValidator>) {
        self.validators.insert(field_name, validator);
    }
    
    pub fn parse_config(&self, content: &str) -> ParseResult<HashMap<String, ConfigValue>> {
        let mut config = HashMap::new();
        let warnings = Vec::new();
        let mut errors = Vec::new();
        
        for (line_number, line) in content.lines().enumerate() {
            let line_number = line_number + 1;
            let line = line.trim();
            
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            match self.parse_config_line(line, line_number) {
                Ok((key, value)) => {
                    // Validate the field if validator exists
                    if let Some(validator) = self.validators.get(&key) {
                        match validator.validate(&value) {
                            Ok(validated_value) => {
                                config.insert(key, validated_value);
                            }
                            Err(_) => {
                                errors.push(ParseError::InvalidFormat {
                                    field: key,
                                    value: value.to_string(),
                                    line: line_number,
                                });
                            }
                        }
                    } else {
                        config.insert(key, value);
                    }
                }
                Err(parse_error) => {
                    errors.push(parse_error);
                    if !self.config.continue_on_error || errors.len() >= self.config.max_errors {
                        break;
                    }
                }
            }
        }
        
        ParseResult {
            data: config,
            warnings,
            errors,
        }
    }
    
    fn parse_config_line(&self, line: &str, line_number: usize) -> Result<(String, ConfigValue), ParseError> {
        if let Some(pos) = line.find('=') {
            let key = line[..pos].trim().to_string();
            let value_str = line[pos + 1..].trim();
            
            if key.is_empty() {
                return Err(ParseError::MissingField {
                    field: "key".to_string(),
                    line: line_number,
                });
            }
            
            let value = self.parse_config_value(value_str, line_number)?;
            Ok((key, value))
        } else {
            Err(ParseError::InvalidFormat {
                field: "line".to_string(),
                value: line.to_string(),
                line: line_number,
            })
        }
    }
    
    fn parse_config_value(&self, value_str: &str, _line_number: usize) -> Result<ConfigValue, ParseError> {
        if value_str.is_empty() {
            Ok(ConfigValue::String(String::new()))
        } else if value_str == "true" {
            Ok(ConfigValue::Boolean(true))
        } else if value_str == "false" {
            Ok(ConfigValue::Boolean(false))
        } else if let Ok(number) = value_str.parse::<i64>() {
            Ok(ConfigValue::Integer(number))
        } else if let Ok(number) = value_str.parse::<f64>() {
            Ok(ConfigValue::Float(number))
        } else {
            // Remove quotes if present
            let unquoted = if value_str.starts_with('"') && value_str.ends_with('"') && value_str.len() > 1 {
                &value_str[1..value_str.len()-1]
            } else {
                value_str
            };
            Ok(ConfigValue::String(unquoted.to_string()))
        }
    }
}

pub trait FieldValidator {
    fn validate(&self, value: &ConfigValue) -> Result<ConfigValue, ValidationError>;
}

// Example validators
pub struct PortValidator;

impl FieldValidator for PortValidator {
    fn validate(&self, value: &ConfigValue) -> Result<ConfigValue, ValidationError> {
        match value {
            ConfigValue::Integer(port) => {
                if *port >= 1 && *port <= 65535 {
                    Ok(ConfigValue::Integer(*port))
                } else {
                    Err(ValidationError::OutOfRange {
                        min: "1".to_string(),
                        max: "65535".to_string(),
                        found: port.to_string(),
                    })
                }
            }
            _ => Err(ValidationError::InvalidType {
                expected: "integer".to_string(),
                found: format!("{:?}", value),
            }),
        }
    }
}

pub struct EmailValidator;

impl FieldValidator for EmailValidator {
    fn validate(&self, value: &ConfigValue) -> Result<ConfigValue, ValidationError> {
        match value {
            ConfigValue::String(email) => {
                if email.contains('@') && email.contains('.') {
                    Ok(ConfigValue::String(email.clone()))
                } else {
                    Err(ValidationError::InvalidFormat {
                        message: "Email must contain @ and .".to_string(),
                    })
                }
            }
            _ => Err(ValidationError::InvalidType {
                expected: "string".to_string(),
                found: format!("{:?}", value),
            }),
        }
    }
}

// Multi-format parser
#[allow(dead_code)]
pub struct MultiFormatParser {
    config: ParserConfig,
    format_detectors: Vec<Box<dyn FormatDetector>>,
}

impl MultiFormatParser {
    pub fn new(config: ParserConfig) -> Self {
        let mut parser = Self {
            config,
            format_detectors: Vec::new(),
        };
        
        // Register default format detectors
        parser.register_detector(Box::new(CsvFormatDetector));
        parser.register_detector(Box::new(JsonFormatDetector));
        parser.register_detector(Box::new(XmlFormatDetector));
        
        parser
    }
    
    pub fn register_detector(&mut self, detector: Box<dyn FormatDetector>) {
        self.format_detectors.push(detector);
    }
    
    pub fn detect_format(&self, content: &str) -> Option<String> {
        for detector in &self.format_detectors {
            if let Some(format) = detector.detect(content) {
                return Some(format);
            }
        }
        None
    }
}

pub trait FormatDetector {
    fn detect(&self, content: &str) -> Option<String>;
}

// Format detectors
pub struct CsvFormatDetector;

impl FormatDetector for CsvFormatDetector {
    fn detect(&self, content: &str) -> Option<String> {
        let first_line = content.lines().next()?;
        if first_line.contains(',') && first_line.split(',').count() > 1 {
            Some("csv".to_string())
        } else {
            None
        }
    }
}

pub struct JsonFormatDetector;

impl FormatDetector for JsonFormatDetector {
    fn detect(&self, content: &str) -> Option<String> {
        let trimmed = content.trim();
        if trimmed.starts_with('{') || trimmed.starts_with('[') {
            Some("json".to_string())
        } else {
            None
        }
    }
}

pub struct XmlFormatDetector;

impl FormatDetector for XmlFormatDetector {
    fn detect(&self, content: &str) -> Option<String> {
        let trimmed = content.trim();
        if trimmed.starts_with('<') && trimmed.contains('>') {
            Some("xml".to_string())
        } else {
            None
        }
    }
}

fn main() {
    println!("=== Day 35: Error Handling Practice Examples ===\n");
    
    // Example 1: CSV Parser with Error Recovery
    println!("1. CSV Parser with Error Recovery:");
    let config = ParserConfig::default();
    let error_recovery = Box::new(StandardErrorRecovery);
    let parser = CsvParser::new(config, error_recovery);
    
    let csv_content = "id,name,email,age\n1,John,john@example.com,25\n2,Jane,jane@example.com,30";
    let result: ParseResult<Vec<UserRecord>> = parser.parse(csv_content);
    
    println!("Valid CSV parsing:");
    println!("  Records parsed: {}", result.data.len());
    println!("  Errors: {}", result.errors.len());
    println!("  Warnings: {}", result.warnings.len());
    
    for user in &result.data {
        println!("  User: {:?}", user);
    }
    println!();
    
    // Example 2: CSV Parser with Invalid Data
    println!("2. CSV Parser with Invalid Data:");
    let csv_content_invalid = "id,name,email,age\n1,John,invalid-email,25\n2,Jane,jane@example.com\n3,Bob,bob@example.com,abc";
    let result: ParseResult<Vec<UserRecord>> = parser.parse(csv_content_invalid);
    
    println!("Invalid CSV parsing:");
    println!("  Records parsed: {}", result.data.len());
    println!("  Errors: {}", result.errors.len());
    
    for error in &result.errors {
        println!("  Error: {:?}", error);
    }
    println!();
    
    // Example 3: Configuration Parser with Validation
    println!("3. Configuration Parser with Validation:");
    let config = ParserConfig::default();
    let mut parser = ConfigParser::new(config);
    
    parser.add_validator("port".to_string(), Box::new(PortValidator));
    parser.add_validator("email".to_string(), Box::new(EmailValidator));
    
    let config_content = "port=8080\nemail=admin@example.com\ninvalid_port=99999\ninvalid_email=not-an-email\nname=MyApp";
    let result = parser.parse_config(config_content);
    
    println!("Config parsing with validation:");
    println!("  Valid entries: {}", result.data.len());
    println!("  Errors: {}", result.errors.len());
    
    for (key, value) in &result.data {
        println!("  {}: {:?}", key, value);
    }
    
    for error in &result.errors {
        println!("  Error: {:?}", error);
    }
    println!();
    
    // Example 4: Multi-Format Detection
    println!("4. Multi-Format Detection:");
    let config = ParserConfig::default();
    let parser = MultiFormatParser::new(config);
    
    // Test CSV detection
    let csv_content = "name,age\nJohn,25";
    let detected = parser.detect_format(csv_content);
    println!("CSV content detected as: {:?}", detected);
    
    // Test JSON detection
    let json_content = r#"{"name": "John", "age": 25}"#;
    let detected = parser.detect_format(json_content);
    println!("JSON content detected as: {:?}", detected);
    
    // Test XML detection
    let xml_content = "<root><name>John</name><age>25</age></root>";
    let detected = parser.detect_format(xml_content);
    println!("XML content detected as: {:?}", detected);
    
    // Test unknown format
    let unknown_content = "This is just plain text";
    let detected = parser.detect_format(unknown_content);
    println!("Unknown content detected as: {:?}", detected);
    println!();
    
    // Example 5: Error Recovery Demonstration
    println!("5. Error Recovery Demonstration:");
    let config = ParserConfig {
        strict_mode: false,
        max_errors: 5,
        continue_on_error: true,
    };
    let error_recovery = Box::new(StandardErrorRecovery);
    let parser = CsvParser::new(config, error_recovery);
    
    let csv_content_with_issues = "id,name,email,age\n1,John,  john@example.com  ,25\n2,Jane,jane@example.com,30\n3,Bob,invalid-email,abc";
    let result: ParseResult<Vec<UserRecord>> = parser.parse(csv_content_with_issues);
    
    println!("CSV parsing with recovery:");
    println!("  Records parsed: {}", result.data.len());
    println!("  Errors: {}", result.errors.len());
    
    for user in &result.data {
        println!("  User: {:?}", user);
    }
    
    for error in &result.errors {
        println!("  Error: {:?}", error);
    }
    println!();
    
    println!("=== End of Day 35 Examples ===");
}
