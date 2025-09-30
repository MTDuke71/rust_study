//! # Parsing Pattern Recognition Module
//!
//! Recognizes and implements the most common AoC input parsing patterns:
//! - Line-by-line processing with iterators
//! - Regex vs manual string parsing
//! - Number extraction and transformation
//! - Complex structured input parsing
//!
//! ## Requirements Satisfied: REQ-P2
//!
//! ### Common AoC Parsing Problem Types:
//! 1. **Simple Line Processing**: One operation per line (AoC 2020 Day 2, 2021 Day 2)
//! 2. **Structured Data**: Tables, grids, coordinates (AoC 2018 Day 6, 2019 Day 3) 
//! 3. **Instruction Parsing**: Commands with parameters (AoC 2022 Day 5, 2020 Day 8)
//! 4. **Number Sequences**: Lists, ranges, mathematical input (AoC 2020 Day 9, 2021 Day 6)

use crate::{PatternResult, PatternError, PatternComplexity};
use std::collections::HashMap;

/// Trait for AoC input parsing patterns
pub trait ParsePattern<T> {
    /// Parse a single line into the target type
    fn parse_line(&self, line: &str) -> PatternResult<T>;
    
    /// Parse multiple lines into a collection
    fn parse_lines(&self, input: &str) -> PatternResult<Vec<T>> {
        input
            .lines()
            .map(|line| self.parse_line(line))
            .collect()
    }
    
    /// Get parsing complexity for performance awareness
    fn parse_complexity(&self) -> PatternComplexity;
    
    /// Get parser name for debugging
    fn parser_name(&self) -> &'static str;
}

/// Simple line processing pattern
/// 
/// # Requirements Satisfied: REQ-P2
/// Handles basic line-by-line processing that's extremely common in AoC
pub struct SimpleLineParser;

impl ParsePattern<String> for SimpleLineParser {
    fn parse_line(&self, line: &str) -> PatternResult<String> {
        Ok(line.trim().to_string())
    }
    
    fn parse_complexity(&self) -> PatternComplexity {
        PatternComplexity::Linear // O(n) where n is line length
    }
    
    fn parser_name(&self) -> &'static str {
        "Simple Line Parser"
    }
}

/// Number extraction pattern (integers from text)
/// 
/// # Requirements Satisfied: REQ-P2
/// Extremely common in AoC - extract numbers from mixed text
pub struct NumberExtractionParser;

impl ParsePattern<Vec<i32>> for NumberExtractionParser {
    /// Extract all integers from a line
    /// 
    /// # Example
    /// ```
    /// use aoc_pattern_recognition::parsing_patterns::*;
    /// 
    /// let parser = NumberExtractionParser;
    /// let result = parser.parse_line("move 3 from 2 to 1").unwrap();
    /// assert_eq!(result, vec![3, 2, 1]);
    /// ```
    fn parse_line(&self, line: &str) -> PatternResult<Vec<i32>> {
        use regex::Regex;
        
        // Use regex to find all integers (including negative numbers)
        let re = Regex::new(r"-?\d+").unwrap();
        let numbers: Vec<i32> = re
            .find_iter(line)
            .filter_map(|m| m.as_str().parse::<i32>().ok())
            .collect();
            
        Ok(numbers)
    }
    
    fn parse_complexity(&self) -> PatternComplexity {
        PatternComplexity::Linear // O(n) where n is line length
    }
    
    fn parser_name(&self) -> &'static str {
        "Number Extraction Parser"
    }
}

/// Coordinate parsing pattern (x,y pairs)
/// 
/// # Requirements Satisfied: REQ-P2  
/// Very common in AoC grid/geometry problems
#[derive(Debug, Clone, PartialEq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

pub struct CoordinateParser;

impl ParsePattern<Coordinate> for CoordinateParser {
    /// Parse coordinate from various formats
    /// 
    /// Supports: "x,y", "(x,y)", "x y", "x -> y"
    /// 
    /// # Example
    /// ```
    /// use aoc_pattern_recognition::parsing_patterns::*;
    /// 
    /// let parser = CoordinateParser;
    /// let coord = parser.parse_line("123,456").unwrap();
    /// assert_eq!(coord.x, 123);
    /// assert_eq!(coord.y, 456);
    /// ```
    fn parse_line(&self, line: &str) -> PatternResult<Coordinate> {
        let cleaned = line
            .trim()
            .replace("(", "")
            .replace(")", "")
            .replace(" -> ", ",");
            
        let parts: Vec<&str> = cleaned
            .split(&[',', ' '][..])
            .filter(|s| !s.is_empty())
            .collect();
            
        if parts.len() >= 2 {
            let x = parts[0].parse::<i32>()
                .map_err(|_| PatternError::InvalidInput(format!("Invalid x coordinate: {}", parts[0])))?;
            let y = parts[1].parse::<i32>()
                .map_err(|_| PatternError::InvalidInput(format!("Invalid y coordinate: {}", parts[1])))?;
                
            Ok(Coordinate { x, y })
        } else {
            Err(PatternError::InvalidInput("Could not find x,y coordinates".to_string()))
        }
    }
    
    fn parse_complexity(&self) -> PatternComplexity {
        PatternComplexity::Constant // O(1) - fixed amount of work per line
    }
    
    fn parser_name(&self) -> &'static str {
        "Coordinate Parser"
    }
}

/// Instruction parsing pattern (command + parameters)
/// 
/// # Requirements Satisfied: REQ-P2
/// Common for AoC problems with step-by-step instructions
#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub command: String,
    pub parameters: Vec<String>,
}

pub struct InstructionParser;

impl ParsePattern<Instruction> for InstructionParser {
    /// Parse instruction with command and parameters
    /// 
    /// # Example
    /// ```
    /// use aoc_pattern_recognition::parsing_patterns::*;
    /// 
    /// let parser = InstructionParser;
    /// let inst = parser.parse_line("move 3 from stack2 to stack1").unwrap();
    /// assert_eq!(inst.command, "move");
    /// assert_eq!(inst.parameters, vec!["3", "from", "stack2", "to", "stack1"]);
    /// ```
    fn parse_line(&self, line: &str) -> PatternResult<Instruction> {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        
        if parts.is_empty() {
            return Err(PatternError::InvalidInput("Empty instruction line".to_string()));
        }
        
        let command = parts[0].to_string();
        let parameters = parts[1..].iter().map(|&s| s.to_string()).collect();
        
        Ok(Instruction { command, parameters })
    }
    
    fn parse_complexity(&self) -> PatternComplexity {
        PatternComplexity::Linear // O(n) where n is number of parameters
    }
    
    fn parser_name(&self) -> &'static str {
        "Instruction Parser"
    }
}

/// Key-value parsing pattern (configuration-style input)
/// 
/// # Requirements Satisfied: REQ-P2
/// For AoC problems with property/attribute definitions
pub struct KeyValueParser {
    pub separator: String,
}

impl KeyValueParser {
    pub fn new(separator: &str) -> Self {
        Self {
            separator: separator.to_string(),
        }
    }
    
    /// Parse multiple key-value pairs from text
    /// 
    /// # Example
    /// ```
    /// use aoc_pattern_recognition::parsing_patterns::*;
    /// 
    /// let parser = KeyValueParser::new(":");
    /// let input = "name: Alice\nage: 30\ncity: Boston";
    /// let result = parser.parse_lines(input).unwrap();
    /// ```
    pub fn parse_to_map(&self, input: &str) -> PatternResult<HashMap<String, String>> {
        let mut map = HashMap::new();
        
        for line in input.lines() {
            if let Ok((key, value)) = self.parse_line(line) {
                map.insert(key, value);
            }
        }
        
        Ok(map)
    }
}

impl ParsePattern<(String, String)> for KeyValueParser {
    fn parse_line(&self, line: &str) -> PatternResult<(String, String)> {
        let parts: Vec<&str> = line.splitn(2, &self.separator).collect();
        
        if parts.len() == 2 {
            let key = parts[0].trim().to_string();
            let value = parts[1].trim().to_string();
            Ok((key, value))
        } else {
            Err(PatternError::InvalidInput(format!("No separator '{}' found in line", self.separator)))
        }
    }
    
    fn parse_complexity(&self) -> PatternComplexity {
        PatternComplexity::Linear // O(n) where n is line length
    }
    
    fn parser_name(&self) -> &'static str {
        "Key-Value Parser"
    }
}

/// Advanced parsing pattern for complex structures
/// 
/// # Requirements Satisfied: REQ-P2, REQ-P4
/// Handles multi-line structured input with state
pub struct StructuredParser {
    pub current_section: Option<String>,
    pub sections: HashMap<String, Vec<String>>,
}

impl StructuredParser {
    pub fn new() -> Self {
        Self {
            current_section: None,
            sections: HashMap::new(),
        }
    }
    
    /// Parse input with section headers and content
    /// 
    /// # Example
    /// ```
    /// use aoc_pattern_recognition::parsing_patterns::*;
    /// 
    /// let mut parser = StructuredParser::new();
    /// let input = "[section1]\ndata1\ndata2\n[section2]\ndata3";
    /// let sections = parser.parse_structured(input).unwrap();
    /// ```
    pub fn parse_structured(&mut self, input: &str) -> PatternResult<HashMap<String, Vec<String>>> {
        for line in input.lines() {
            let trimmed = line.trim();
            
            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                // Section header
                let section_name = trimmed[1..trimmed.len()-1].to_string();
                self.current_section = Some(section_name.clone());
                self.sections.entry(section_name).or_insert_with(Vec::new);
            } else if !trimmed.is_empty() {
                // Section content
                if let Some(ref section) = self.current_section {
                    self.sections
                        .get_mut(section)
                        .unwrap()
                        .push(trimmed.to_string());
                } else {
                    return Err(PatternError::InvalidInput("Content found before section header".to_string()));
                }
            }
        }
        
        Ok(self.sections.clone())
    }
}

/// High-level parsing pattern recognizer
/// 
/// # Requirements Satisfied: REQ-P2, REQ-P5
/// Automatically detects which parsing pattern to use
pub struct ParsingPatternRecognizer;

impl ParsingPatternRecognizer {
    /// Analyze input and suggest the best parsing pattern
    /// 
    /// # Requirements Satisfied: REQ-P5
    /// Teaches pattern recognition skills
    pub fn analyze_input(&self, input: &str) -> Vec<String> {
        let mut patterns = Vec::new();
        let lines: Vec<&str> = input.lines().collect();
        
        if lines.is_empty() {
            return patterns;
        }
        
        // Check for coordinate patterns
        if self.has_coordinate_pattern(&lines) {
            patterns.push("Coordinate parsing recommended".to_string());
        }
        
        // Check for instruction patterns  
        if self.has_instruction_pattern(&lines) {
            patterns.push("Instruction parsing recommended".to_string());
        }
        
        // Check for number extraction patterns
        if self.has_number_pattern(&lines) {
            patterns.push("Number extraction recommended".to_string());
        }
        
        // Check for key-value patterns
        if self.has_key_value_pattern(&lines) {
            patterns.push("Key-value parsing recommended".to_string());
        }
        
        // Check for structured/sectioned patterns
        if self.has_structured_pattern(&lines) {
            patterns.push("Structured parsing recommended".to_string());
        }
        
        if patterns.is_empty() {
            patterns.push("Simple line parsing recommended".to_string());
        }
        
        patterns
    }
    
    fn has_coordinate_pattern(&self, lines: &[&str]) -> bool {
        lines.iter().any(|line| {
            line.contains(',') && 
            line.chars().filter(|c| c.is_ascii_digit()).count() >= 2
        })
    }
    
    fn has_instruction_pattern(&self, lines: &[&str]) -> bool {
        lines.iter().any(|line| {
            let words: Vec<&str> = line.split_whitespace().collect();
            words.len() >= 2 && 
            words[0].chars().all(|c| c.is_ascii_alphabetic())
        })
    }
    
    fn has_number_pattern(&self, lines: &[&str]) -> bool {
        lines.iter().any(|line| {
            line.split_whitespace()
                .filter(|word| word.parse::<i32>().is_ok())
                .count() >= 2
        })
    }
    
    fn has_key_value_pattern(&self, lines: &[&str]) -> bool {
        lines.iter().any(|line| {
            line.contains(':') || line.contains('=')
        })
    }
    
    fn has_structured_pattern(&self, lines: &[&str]) -> bool {
        lines.iter().any(|line| {
            line.trim().starts_with('[') && line.trim().ends_with(']')
        })
    }
}

/// Helper functions for common parsing operations
pub mod parse_utils {
    use super::*;
    
    /// Extract all integers from text, including negative numbers
    /// 
    /// # Requirements Satisfied: REQ-P4
    /// Fast number extraction using regex-like approach
    pub fn extract_numbers(text: &str) -> Vec<i32> {
        let mut numbers = Vec::new();
        let mut current_number = String::new();
        let mut in_number = false;
        
        for ch in text.chars() {
            if ch.is_ascii_digit() || (ch == '-' && !in_number) {
                current_number.push(ch);
                in_number = true;
            } else {
                if in_number && !current_number.is_empty() {
                    if let Ok(num) = current_number.parse::<i32>() {
                        numbers.push(num);
                    }
                    current_number.clear();
                }
                in_number = false;
            }
        }
        
        // Handle number at end of string
        if in_number && !current_number.is_empty() {
            if let Ok(num) = current_number.parse::<i32>() {
                numbers.push(num);
            }
        }
        
        numbers
    }
    
    /// Split text by multiple delimiters
    pub fn multi_split(text: &str, delimiters: &[char]) -> Vec<String> {
        text.split(delimiters)
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().to_string())
            .collect()
    }
    
    /// Parse ranges like "1-5" or "10..20"
    pub fn parse_range(text: &str) -> PatternResult<(i32, i32)> {
        if let Some(pos) = text.find("..") {
            let start = text[..pos].trim().parse::<i32>()
                .map_err(|_| PatternError::InvalidInput("Invalid range start".to_string()))?;
            let end = text[pos+2..].trim().parse::<i32>()
                .map_err(|_| PatternError::InvalidInput("Invalid range end".to_string()))?;
            Ok((start, end))
        } else if let Some(pos) = text.find('-') {
            let start = text[..pos].trim().parse::<i32>()
                .map_err(|_| PatternError::InvalidInput("Invalid range start".to_string()))?;
            let end = text[pos+1..].trim().parse::<i32>()
                .map_err(|_| PatternError::InvalidInput("Invalid range end".to_string()))?;
            Ok((start, end))
        } else {
            Err(PatternError::InvalidInput("No range delimiter found".to_string()))
        }
    }
}