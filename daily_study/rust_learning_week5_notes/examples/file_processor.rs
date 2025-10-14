// Robust File Processing with Error Recovery
// This example demonstrates comprehensive error handling for file processing pipelines

use anyhow::{Result, Context};
use thiserror::Error;
use std::path::Path;
use std::fs;

#[derive(Error, Debug)]
pub enum ProcessingError {
    #[error("File not found: {path}")]
    FileNotFound { path: String },
    
    #[error("Permission denied: {path}")]
    PermissionDenied { path: String },
    
    #[error("Invalid file format: {expected}, got {actual}")]
    InvalidFormat { expected: String, actual: String },
    
    #[error("Processing failed at stage {stage}: {reason}")]
    ProcessingFailed { stage: String, reason: String },
    
    #[error("Output directory not writable: {path}")]
    OutputNotWritable { path: String },
    
    #[error("IO error: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },
}

#[derive(Debug)]
struct ProcessingResult {
    input_files: usize,
    processed_files: usize,
    failed_files: usize,
    errors: Vec<String>,
}

struct FileProcessor {
    input_dir: String,
    output_dir: String,
    #[allow(dead_code)]
    temp_dir: String,
}

impl FileProcessor {
    fn new(input_dir: String, output_dir: String, temp_dir: String) -> Self {
        Self {
            input_dir,
            output_dir,
            temp_dir,
        }
    }
    
    fn process_all_files(&self) -> Result<ProcessingResult> {
        let files = self.list_input_files()
            .context("Failed to list input files")?;
        
        let mut processed_count = 0;
        let mut failed_count = 0;
        let mut errors = Vec::new();
        
        for file_path in files {
            match self.process_single_file(&file_path) {
                Ok(_) => {
                    processed_count += 1;
                    println!("✅ Processed: {}", file_path);
                }
                Err(e) => {
                    failed_count += 1;
                    let error_msg = format!("Failed to process {}: {}", file_path, e);
                    errors.push(error_msg.clone());
                    println!("❌ {}", error_msg);
                }
            }
        }
        
        Ok(ProcessingResult {
            input_files: processed_count + failed_count,
            processed_files: processed_count,
            failed_files: failed_count,
            errors,
        })
    }
    
    fn list_input_files(&self) -> Result<Vec<String>> {
        let mut files = Vec::new();
        
        // Simulate directory listing
        if self.input_dir == "empty" {
            return Ok(files);
        }
        
        files.push(format!("{}/file1.txt", self.input_dir));
        files.push(format!("{}/file2.csv", self.input_dir));
        files.push(format!("{}/file3.json", self.input_dir));
        
        Ok(files)
    }
    
    fn process_single_file(&self, file_path: &str) -> Result<()> {
        // Read file
        let content = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read file: {}", file_path))?;
        
        // Validate format
        let validated_content = self.validate_format(&content, file_path)
            .with_context(|| format!("Failed to validate format for: {}", file_path))?;
        
        // Parse content
        let parsed_data = self.parse_content(&validated_content, file_path)
            .with_context(|| format!("Failed to parse content for: {}", file_path))?;
        
        // Transform data
        let transformed_data = self.transform_data(parsed_data, file_path)
            .with_context(|| format!("Failed to transform data for: {}", file_path))?;
        
        // Write output
        let output_path = self.generate_output_path(file_path)
            .context("Failed to generate output path")?;
        
        self.write_output(&transformed_data, &output_path)
            .with_context(|| format!("Failed to write output to: {}", output_path))?;
        
        Ok(())
    }
    
    fn validate_format(&self, content: &str, file_path: &str) -> Result<String> {
        if content.trim().is_empty() {
            return Err(ProcessingError::ProcessingFailed {
                stage: "validation".to_string(),
                reason: "File is empty".to_string(),
            }.into());
        }
        
        // Check if file has expected format based on extension
        let extension = Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        match extension {
            "csv" => {
                if !content.contains(',') {
                    return Err(ProcessingError::InvalidFormat {
                        expected: "CSV format with comma separators".to_string(),
                        actual: "No comma separators found".to_string(),
                    }.into());
                }
            }
            "json" => {
                if !content.trim().starts_with('{') && !content.trim().starts_with('[') {
                    return Err(ProcessingError::InvalidFormat {
                        expected: "JSON format".to_string(),
                        actual: "Does not start with { or [".to_string(),
                    }.into());
                }
            }
            _ => {} // Accept other formats
        }
        
        Ok(content.to_string())
    }
    
    fn parse_content(&self, content: &str, file_path: &str) -> Result<ParsedData> {
        let extension = Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        match extension {
            "csv" => self.parse_csv(content),
            "json" => self.parse_json(content),
            _ => self.parse_text(content),
        }
    }
    
    fn parse_csv(&self, content: &str) -> Result<ParsedData> {
        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return Err(ProcessingError::ProcessingFailed {
                stage: "CSV parsing".to_string(),
                reason: "No lines found".to_string(),
            }.into());
        }
        
        Ok(ParsedData {
            records: lines.len(),
            content: content.to_string(),
            format: "CSV".to_string(),
        })
    }
    
    fn parse_json(&self, content: &str) -> Result<ParsedData> {
        if content.contains("invalid") {
            return Err(ProcessingError::ProcessingFailed {
                stage: "JSON parsing".to_string(),
                reason: "Invalid JSON structure".to_string(),
            }.into());
        }
        
        Ok(ParsedData {
            records: 1,
            content: content.to_string(),
            format: "JSON".to_string(),
        })
    }
    
    fn parse_text(&self, content: &str) -> Result<ParsedData> {
        Ok(ParsedData {
            records: content.lines().count(),
            content: content.to_string(),
            format: "TEXT".to_string(),
        })
    }
    
    fn transform_data(&self, data: ParsedData, _file_path: &str) -> Result<String> {
        if data.content.contains("TRANSFORM_ERROR") {
            return Err(ProcessingError::ProcessingFailed {
                stage: "transformation".to_string(),
                reason: "Data contains transform error marker".to_string(),
            }.into());
        }
        
        Ok(format!("PROCESSED: {}\nRecords: {}\nFormat: {}", 
                   data.content, data.records, data.format))
    }
    
    fn generate_output_path(&self, input_path: &str) -> Result<String> {
        let filename = Path::new(input_path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");
        
        Ok(format!("{}/processed_{}", self.output_dir, filename))
    }
    
    fn write_output(&self, content: &str, output_path: &str) -> Result<()> {
        fs::write(output_path, content)?;
        Ok(())
    }
}

#[derive(Debug)]
struct ParsedData {
    records: usize,
    content: String,
    format: String,
}

fn main() -> Result<()> {
    println!("=== Robust File Processing Example ===\n");
    
    // Create test directories and files
    fs::create_dir_all("test_input").context("Failed to create test input directory")?;
    fs::create_dir_all("test_output").context("Failed to create test output directory")?;
    
    // Create test files
    fs::write("test_input/file1.txt", "Hello, World!\nThis is a test file.")?;
    fs::write("test_input/file2.csv", "name,age,city\nJohn,25,NYC\nJane,30,LA")?;
    fs::write("test_input/file3.json", r#"{"name": "Test", "value": 42}"#)?;
    fs::write("test_input/invalid.csv", "This is not a valid CSV file")?;
    
    // Process files
    let processor = FileProcessor::new(
        "test_input".to_string(),
        "test_output".to_string(),
        "temp".to_string(),
    );
    
    match processor.process_all_files() {
        Ok(result) => {
            println!("\n📊 Processing Summary:");
            println!("  Input files: {}", result.input_files);
            println!("  Processed successfully: {}", result.processed_files);
            println!("  Failed: {}", result.failed_files);
            
            if !result.errors.is_empty() {
                println!("\n❌ Errors encountered:");
                for error in result.errors {
                    println!("  - {}", error);
                }
            }
            
            if result.processed_files > 0 {
                println!("\n✅ Successfully processed files:");
                for entry in fs::read_dir("test_output")? {
                    let entry = entry?;
                    println!("  - {}", entry.file_name().to_string_lossy());
                }
            }
        }
        Err(e) => {
            println!("❌ Processing failed: {}", e);
        }
    }
    
    // Cleanup
    let _ = fs::remove_dir_all("test_input");
    let _ = fs::remove_dir_all("test_output");
    
    Ok(())
}
