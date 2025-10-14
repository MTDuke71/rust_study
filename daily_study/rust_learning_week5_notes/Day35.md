# Day 35 - Error Handling Practice

**Learning Focus**: Building robust parsers with comprehensive error handling

---

## 🎯 Learning Objectives

By the end of this day, you should understand:
- How to build robust parsers with comprehensive error handling
- Error recovery strategies for parsing failures
- How to provide meaningful error messages to users
- Testing error handling code effectively
- Building error handling into larger systems
- Performance considerations for error handling

---

## 📚 Core Concepts

### **Robust Parser Design**

```rust
use std::collections::HashMap;

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

#[derive(Debug)]
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
```

### **Error Recovery Strategies**

```rust
pub trait ErrorRecovery {
    fn can_recover(&self, error: &ParseError) -> bool;
    fn recover(&self, error: ParseError, context: &ParseContext) -> Result<(), ParseError>;
}

pub struct ParseContext {
    pub line_number: usize,
    pub column: usize,
    pub field_name: Option<String>,
    pub previous_values: HashMap<String, String>,
}
```

---

## 🔧 Implementation Patterns

### **Pattern 1: CSV Parser with Error Recovery**

```rust
use std::collections::HashMap;

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
        let mut line_number = 1;
        
        let lines: Vec<&str> = content.lines().collect();
        let headers = if !lines.is_empty() {
            self.parse_headers(lines[0], &mut warnings, &mut errors)?
        } else {
            return ParseResult {
                data: results,
                warnings,
                errors,
            };
        };
        
        for (index, line) in lines.iter().enumerate().skip(1) {
            line_number = index + 1;
            
            if line.trim().is_empty() {
                continue;
            }
            
            match self.parse_row(line, &headers, line_number) {
                Ok(row) => results.push(row),
                Err(mut error) => {
                    if self.error_recovery.can_recover(&error) {
                        match self.error_recovery.recover(error, &ParseContext {
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
    
    fn parse_headers(&self, line: &str, warnings: &mut Vec<ParseWarning>, errors: &mut Vec<ParseError>) -> Result<Vec<String>, ParseError> {
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
            if let Some(prev_index) = seen.get(header) {
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
```

### **Pattern 2: JSON Parser with Detailed Error Reporting**

```rust
use std::collections::HashMap;

pub struct JsonParser {
    config: ParserConfig,
}

impl JsonParser {
    pub fn new(config: ParserConfig) -> Self {
        Self { config }
    }
    
    pub fn parse<T>(&self, content: &str) -> ParseResult<T>
    where
        T: FromJson,
    {
        let mut warnings = Vec::new();
        let mut errors = Vec::new();
        
        // Tokenize the JSON
        let tokens = match self.tokenize(content, &mut errors) {
            Ok(tokens) => tokens,
            Err(_) => {
                return ParseResult {
                    data: unsafe { std::mem::zeroed() }, // This is unsafe, but for demo purposes
                    warnings,
                    errors,
                };
            }
        };
        
        // Parse the tokens
        let data = match self.parse_tokens::<T>(&tokens, &mut warnings, &mut errors) {
            Ok(data) => data,
            Err(_) => {
                return ParseResult {
                    data: unsafe { std::mem::zeroed() }, // This is unsafe, but for demo purposes
                    warnings,
                    errors,
                };
            }
        };
        
        ParseResult {
            data,
            warnings,
            errors,
        }
    }
    
    fn tokenize(&self, content: &str, errors: &mut Vec<ParseError>) -> Result<Vec<JsonToken>, ParseError> {
        let mut tokens = Vec::new();
        let mut chars = content.chars().enumerate().peekable();
        let mut line = 1;
        let mut column = 1;
        
        while let Some((pos, ch)) = chars.next() {
            match ch {
                '{' => tokens.push(JsonToken::LeftBrace { line, column }),
                '}' => tokens.push(JsonToken::RightBrace { line, column }),
                '[' => tokens.push(JsonToken::LeftBracket { line, column }),
                ']' => tokens.push(JsonToken::RightBracket { line, column }),
                ':' => tokens.push(JsonToken::Colon { line, column }),
                ',' => tokens.push(JsonToken::Comma { line, column }),
                '"' => {
                    // Parse string
                    let mut string_value = String::new();
                    while let Some((_, next_ch)) = chars.peek() {
                        if *next_ch == '"' {
                            chars.next();
                            break;
                        } else if *next_ch == '\\' {
                            chars.next();
                            if let Some((_, escape_ch)) = chars.next() {
                                string_value.push(match escape_ch {
                                    'n' => '\n',
                                    't' => '\t',
                                    'r' => '\r',
                                    _ => escape_ch,
                                });
                            }
                        } else {
                            string_value.push(*next_ch);
                            chars.next();
                        }
                    }
                    tokens.push(JsonToken::String { value: string_value, line, column });
                }
                ' ' | '\t' | '\r' => {
                    // Skip whitespace
                }
                '\n' => {
                    line += 1;
                    column = 1;
                    continue;
                }
                '0'..='9' | '-' => {
                    // Parse number
                    let mut number_str = String::new();
                    number_str.push(ch);
                    
                    while let Some((_, next_ch)) = chars.peek() {
                        match next_ch {
                            '0'..='9' | '.' | 'e' | 'E' | '+' | '-' => {
                                number_str.push(*next_ch);
                                chars.next();
                            }
                            _ => break,
                        }
                    }
                    
                    tokens.push(JsonToken::Number { value: number_str, line, column });
                }
                't' | 'f' | 'n' => {
                    // Parse boolean or null
                    let mut keyword = String::new();
                    keyword.push(ch);
                    
                    while let Some((_, next_ch)) = chars.peek() {
                        if next_ch.is_alphabetic() {
                            keyword.push(*next_ch);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    
                    match keyword.as_str() {
                        "true" => tokens.push(JsonToken::Boolean { value: true, line, column }),
                        "false" => tokens.push(JsonToken::Boolean { value: false, line, column }),
                        "null" => tokens.push(JsonToken::Null { line, column }),
                        _ => {
                            errors.push(ParseError::UnexpectedToken {
                                expected: "true, false, or null".to_string(),
                                found: keyword,
                                line,
                                column,
                            });
                        }
                    }
                }
                _ => {
                    errors.push(ParseError::UnexpectedToken {
                        expected: "valid JSON token".to_string(),
                        found: ch.to_string(),
                        line,
                        column,
                    });
                }
            }
            
            column += 1;
        }
        
        Ok(tokens)
    }
    
    fn parse_tokens<T>(&self, tokens: &[JsonToken], warnings: &mut Vec<ParseWarning>, errors: &mut Vec<ParseError>) -> Result<T, ParseError>
    where
        T: FromJson,
    {
        // Simplified parsing - in real implementation, this would be more complex
        T::from_json(tokens, 0, warnings, errors)
    }
}

#[derive(Debug)]
pub enum JsonToken {
    LeftBrace { line: usize, column: usize },
    RightBrace { line: usize, column: usize },
    LeftBracket { line: usize, column: usize },
    RightBracket { line: usize, column: usize },
    Colon { line: usize, column: usize },
    Comma { line: usize, column: usize },
    String { value: String, line: usize, column: usize },
    Number { value: String, line: usize, column: usize },
    Boolean { value: bool, line: usize, column: usize },
    Null { line: usize, column: usize },
}

pub trait FromJson {
    fn from_json(tokens: &[JsonToken], start: usize, warnings: &mut Vec<ParseWarning>, errors: &mut Vec<ParseError>) -> Result<Self, ParseError>
    where
        Self: Sized;
}
```

### **Pattern 3: Configuration Parser with Validation**

```rust
use std::collections::HashMap;

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
        let mut warnings = Vec::new();
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
                            Err(validation_error) => {
                                errors.push(ParseError::InvalidFormat {
                                    field: key,
                                    value: value.to_string(),
                                    line: line_number,
                                });
                            }
                        }
                    } else {
                        config.insert(key, ConfigValue::String(value));
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
    
    fn parse_config_value(&self, value_str: &str, line_number: usize) -> Result<ConfigValue, ParseError> {
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

#[derive(Debug, Clone)]
pub enum ConfigValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

pub trait FieldValidator {
    fn validate(&self, value: &ConfigValue) -> Result<ConfigValue, ValidationError>;
}

#[derive(Debug)]
pub enum ValidationError {
    InvalidType { expected: String, found: String },
    OutOfRange { min: String, max: String, found: String },
    InvalidFormat { message: String },
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
```

---

## 🎮 Practical Applications

### **Application 1: Robust CSV Processing System**

```rust
use std::collections::HashMap;

pub struct CsvProcessingSystem {
    parser: CsvParser,
    processors: HashMap<String, Box<dyn RowProcessor>>,
}

impl CsvProcessingSystem {
    pub fn new(config: ParserConfig) -> Self {
        let error_recovery = Box::new(StandardErrorRecovery);
        let parser = CsvParser::new(config, error_recovery);
        
        Self {
            parser,
            processors: HashMap::new(),
        }
    }
    
    pub fn add_processor(&mut self, name: String, processor: Box<dyn RowProcessor>) {
        self.processors.insert(name, processor);
    }
    
    pub fn process_file<T>(&self, content: &str) -> ProcessingResult<T>
    where
        T: FromCsvRow,
    {
        let parse_result = self.parser.parse::<T>(content);
        
        let mut processed_rows = Vec::new();
        let mut processing_errors = Vec::new();
        
        for row in parse_result.data {
            match self.process_row(&row) {
                Ok(processed) => processed_rows.push(processed),
                Err(error) => processing_errors.push(error),
            }
        }
        
        ProcessingResult {
            original_count: parse_result.data.len(),
            processed_count: processed_rows.len(),
            parse_warnings: parse_result.warnings,
            parse_errors: parse_result.errors,
            processing_errors,
            data: processed_rows,
        }
    }
    
    fn process_row<T>(&self, row: &T) -> Result<T, ProcessingError>
    where
        T: Clone,
    {
        // Apply all registered processors
        for processor in self.processors.values() {
            if let Err(error) = processor.process(row) {
                return Err(ProcessingError::ProcessorError {
                    processor: "unknown".to_string(),
                    error: error.to_string(),
                });
            }
        }
        
        Ok(row.clone())
    }
}

pub struct ProcessingResult<T> {
    pub original_count: usize,
    pub processed_count: usize,
    pub parse_warnings: Vec<ParseWarning>,
    pub parse_errors: Vec<ParseError>,
    pub processing_errors: Vec<ProcessingError>,
    pub data: Vec<T>,
}

#[derive(Debug)]
pub enum ProcessingError {
    ProcessorError { processor: String, error: String },
    ValidationError { field: String, message: String },
}

pub trait RowProcessor {
    fn process<T>(&self, row: &T) -> Result<(), Box<dyn std::error::Error>>;
}

// Error recovery implementation
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
    
    fn recover(&self, error: ParseError, context: &ParseContext) -> Result<(), ParseError> {
        match error {
            ParseError::InvalidFormat { field, value, line } => {
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
```

### **Application 2: Multi-Format Data Parser**

```rust
pub struct MultiFormatParser {
    config: ParserConfig,
    format_detectors: Vec<Box<dyn FormatDetector>>,
    parsers: HashMap<String, Box<dyn AnyParser>>,
}

impl MultiFormatParser {
    pub fn new(config: ParserConfig) -> Self {
        let mut parser = Self {
            config,
            format_detectors: Vec::new(),
            parsers: HashMap::new(),
        };
        
        // Register default format detectors
        parser.register_detector(Box::new(CsvFormatDetector));
        parser.register_detector(Box::new(JsonFormatDetector));
        parser.register_detector(Box::new(XmlFormatDetector));
        
        // Register default parsers
        parser.register_parser("csv".to_string(), Box::new(CsvParser::new(config.clone(), Box::new(StandardErrorRecovery))));
        parser.register_parser("json".to_string(), Box::new(JsonParser::new(config.clone())));
        
        parser
    }
    
    pub fn register_detector(&mut self, detector: Box<dyn FormatDetector>) {
        self.format_detectors.push(detector);
    }
    
    pub fn register_parser(&mut self, format: String, parser: Box<dyn AnyParser>) {
        self.parsers.insert(format, parser);
    }
    
    pub fn parse_auto<T>(&self, content: &str) -> ParseResult<T>
    where
        T: 'static,
    {
        // Detect format
        let detected_format = self.detect_format(content);
        
        // Parse with appropriate parser
        match detected_format {
            Some(format) => {
                if let Some(parser) = self.parsers.get(&format) {
                    parser.parse(content)
                } else {
                    ParseResult {
                        data: unsafe { std::mem::zeroed() },
                        warnings: vec![],
                        errors: vec![ParseError::Custom {
                            message: format!("No parser available for format: {}", format),
                            line: None,
                        }],
                    }
                }
            }
            None => ParseResult {
                data: unsafe { std::mem::zeroed() },
                warnings: vec![],
                errors: vec![ParseError::Custom {
                    message: "Unable to detect file format".to_string(),
                    line: None,
                }],
            }
        }
    }
    
    fn detect_format(&self, content: &str) -> Option<String> {
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

pub trait AnyParser {
    fn parse<T>(&self, content: &str) -> ParseResult<T>;
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
```

---

## 🧪 Testing Error Handling

### **Comprehensive Test Suite**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_csv_parser_valid_data() {
        let config = ParserConfig::default();
        let error_recovery = Box::new(StandardErrorRecovery);
        let parser = CsvParser::new(config, error_recovery);
        
        let csv_content = "id,name,email,age\n1,John,john@example.com,25\n2,Jane,jane@example.com,30";
        let result: ParseResult<Vec<UserRecord>> = parser.parse(csv_content);
        
        assert!(result.errors.is_empty());
        assert_eq!(result.data.len(), 2);
        assert_eq!(result.data[0].name, "John");
        assert_eq!(result.data[1].name, "Jane");
    }
    
    #[test]
    fn test_csv_parser_invalid_data() {
        let config = ParserConfig::default();
        let error_recovery = Box::new(StandardErrorRecovery);
        let parser = CsvParser::new(config, error_recovery);
        
        let csv_content = "id,name,email,age\n1,John,invalid-email,25\n2,Jane,jane@example.com";
        let result: ParseResult<Vec<UserRecord>> = parser.parse(csv_content);
        
        assert!(!result.errors.is_empty());
        assert!(result.errors.iter().any(|e| matches!(e, ParseError::InvalidFormat { field, .. } if field == "email")));
    }
    
    #[test]
    fn test_config_parser_with_validation() {
        let config = ParserConfig::default();
        let mut parser = ConfigParser::new(config);
        
        parser.add_validator("port".to_string(), Box::new(PortValidator));
        parser.add_validator("email".to_string(), Box::new(EmailValidator));
        
        let config_content = "port=8080\nemail=admin@example.com\ninvalid_port=99999\ninvalid_email=not-an-email";
        let result = parser.parse_config(config_content);
        
        assert_eq!(result.data.len(), 2); // Only valid entries
        assert!(!result.errors.is_empty()); // Some validation errors
    }
    
    #[test]
    fn test_error_recovery() {
        let config = ParserConfig {
            strict_mode: false,
            max_errors: 5,
            continue_on_error: true,
        };
        let error_recovery = Box::new(StandardErrorRecovery);
        let parser = CsvParser::new(config, error_recovery);
        
        let csv_content = "id,name,email,age\n1,John,  john@example.com  ,25\n2,Jane,jane@example.com,30";
        let result: ParseResult<Vec<UserRecord>> = parser.parse(csv_content);
        
        // Should recover from whitespace issues
        assert_eq!(result.data.len(), 2);
        assert_eq!(result.data[0].email, "john@example.com");
    }
    
    #[test]
    fn test_multi_format_detection() {
        let config = ParserConfig::default();
        let parser = MultiFormatParser::new(config);
        
        // Test CSV detection
        let csv_content = "name,age\nJohn,25";
        let detected = parser.detect_format(csv_content);
        assert_eq!(detected, Some("csv".to_string()));
        
        // Test JSON detection
        let json_content = r#"{"name": "John", "age": 25}"#;
        let detected = parser.detect_format(json_content);
        assert_eq!(detected, Some("json".to_string()));
    }
}
```

---

## 🎯 Key Takeaways

1. **Design for error recovery** - Build parsers that can handle and recover from errors
2. **Provide meaningful error messages** - Include context like line numbers and expected values
3. **Use validation layers** - Separate parsing from validation for better error handling
4. **Test error conditions thoroughly** - Include comprehensive tests for error scenarios
5. **Consider performance** - Error handling shouldn't significantly impact performance
6. **Build composable systems** - Design parsers that can be combined and extended

---

## 🔗 Related Concepts

- **[[Custom Error Types]]** - Creating domain-specific error types
- **[[Error Propagation]]** - The ? operator and error forwarding
- **[[anyhow and thiserror]]** - Advanced error handling crates
- **[[Result Combinators]]** - Functional error handling methods
- **[[Error Handling Patterns]]** - When to panic vs handle errors

---

*Tags: #error-handling #parser-design #error-recovery #validation #testing #robust-parsing*

*Links: [[zettel-index]] | [[Custom Error Types]] | [[Error Propagation]] | [[anyhow and thiserror]] | [[Result Combinators]] | [[Error Handling Patterns]] | [[Week 5 Overview]]*
