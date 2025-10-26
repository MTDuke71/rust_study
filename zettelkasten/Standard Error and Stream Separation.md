# 🖥️ Standard Error and Stream Separation

**Comprehensive guide to stdout/stderr separation in CLI applications and shell environments**

## 🎯 Core Concepts

### **The Two Standard Streams**
Every Unix-like process has three standard file descriptors:
- **stdin (0)** - Standard input (keyboard, pipes)
- **stdout (1)** - Standard output (results, data)  
- **stderr (2)** - Standard error (diagnostics, errors)

```rust
use std::io::{self, Write};

fn main() {
    println!("This goes to stdout");           // File descriptor 1
    eprintln!("This goes to stderr");         // File descriptor 2
    
    // Explicit writing
    writeln!(io::stdout(), "Explicit stdout").unwrap();
    writeln!(io::stderr(), "Explicit stderr").unwrap();
}
```

### **Why Separate Streams Matter**
- **🎯 Data vs Diagnostics** - Results don't mix with error messages
- **🔀 Independent Redirection** - Pipe results while seeing errors
- **🧪 Testing and Automation** - Scripts can handle success/failure separately
- **📏 Unix Philosophy** - "Do one thing well" with clean interfaces

## 🦀 Rust-Specific Patterns

### **println! vs eprintln!**
```rust
// Standard output - for program results
println!("Found {} matches:", count);
for line in results {
    println!("{}", line);
}

// Standard error - for diagnostics
eprintln!("⚠️  Warning: Large file, this might take time");
eprintln!("❌ Error: File not found: {}", filename);
eprintln!("ℹ️  Info: Processing {} files", file_count);
```

### **Structured Error Output**
```rust
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("❌ Problem parsing arguments: {}", err);
        eprintln!("💡 Usage: {} QUERY FILENAME", env!("CARGO_PKG_NAME"));
        process::exit(1);
    });

    if let Err(e) = run_application(config) {
        eprintln!("❌ Application error: {}", e);
        process::exit(1);
    }
}
```

### **Debug vs Production Error Messages**
```rust
#[cfg(debug_assertions)]
fn log_error(error: &str, context: &str) {
    eprintln!("🐛 DEBUG: {} (context: {})", error, context);
}

#[cfg(not(debug_assertions))]
fn log_error(error: &str, _context: &str) {
    eprintln!("❌ Error: {}", error);
}
```

## 🔀 Shell Redirection Patterns

### **Basic Redirection**
```bash
# Redirect stdout only
program > output.txt                # Results to file, errors to terminal

# Redirect stderr only  
program 2> errors.txt              # Errors to file, results to terminal

# Redirect both to separate files
program > output.txt 2> errors.txt

# Redirect both to same file
program > combined.txt 2>&1

# Discard stderr
program 2> /dev/null               # Unix/Linux
program 2> $null                   # PowerShell
```

### **PowerShell Redirection**
```powershell
# Basic redirection
cargo run > results.txt                    # stdout only
cargo run 2> errors.txt                    # stderr only
cargo run > results.txt 2> errors.txt     # both separated

# PowerShell-specific
cargo run *> all_output.txt               # All streams
cargo run 3> info.txt 4> verbose.txt      # Custom streams
```

### **Piping and Redirection**
```bash
# Pipe stdout, stderr to terminal
program | grep "pattern"

# Pipe both streams
program 2>&1 | grep "error"

# Complex pipeline
program 2> >(logger -t errors) | process_results
```

## 🧪 Testing Stream Separation

### **Manual Testing**
```bash
# Test successful output
cargo run -p error_messages_demo -- nobody poem.txt > results.txt
# Should see debug info in terminal, results in file

# Test error output  
cargo run -p error_messages_demo -- query nonexistent.txt 2> errors.txt
# Should see cargo output in terminal, app errors in file

# Test stream separation
cargo run -p error_messages_demo -- pattern file.txt > out.txt 2> err.txt
# Check both files separately
```

### **Automated Testing with PowerShell**
```powershell
# Test script pattern
function Test-StreamSeparation {
    param($Command, $Args)
    
    # Capture streams separately
    $stdout = & $Command $Args 2>$null
    $stderr = & $Command $Args 2>&1 | Where-Object { $_ -is [System.Management.Automation.ErrorRecord] }
    
    return @{
        StdOut = $stdout
        StdErr = $stderr
        ExitCode = $LASTEXITCODE
    }
}
```

### **Rust Testing Framework**
```rust
#[cfg(test)]
mod tests {
    use std::process::{Command, Stdio};
    
    #[test]
    fn test_stderr_separation() {
        let output = Command::new("cargo")
            .args(&["run", "--", "nonexistent"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .expect("Failed to execute command");
        
        // stdout should be empty for error case
        assert!(output.stdout.is_empty());
        
        // stderr should contain error message
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stderr.contains("error"));
        
        // Exit code should indicate failure
        assert!(!output.status.success());
    }
}
```

## 🎨 CLI Design Best Practices

### **User Experience Guidelines**
```rust
// ✅ Good: Clear, actionable error messages
eprintln!("❌ File not found: '{}'", filename);
eprintln!("💡 Tip: Check the file path and permissions");

// ❌ Bad: Cryptic or unhelpful errors  
eprintln!("Error");
eprintln!("Something went wrong");
```

### **Progress and Status Updates**
```rust
fn process_files(files: &[String]) -> Result<(), Box<dyn Error>> {
    eprintln!("🚀 Processing {} files...", files.len());
    
    for (i, file) in files.iter().enumerate() {
        eprintln!("📁 [{}/{}] Processing: {}", i + 1, files.len(), file);
        
        match process_file(file) {
            Ok(result) => {
                println!("{}", result);  // Actual output to stdout
                eprintln!("✅ Completed: {}", file);  // Status to stderr
            }
            Err(e) => {
                eprintln!("❌ Failed {}: {}", file, e);
                // Continue processing other files
            }
        }
    }
    
    eprintln!("🎉 Processing complete!");
    Ok(())
}
```

### **Configuration and Debug Info**
```rust
fn main() {
    let config = parse_config();
    
    // Debug info to stderr (doesn't pollute output)
    if config.verbose {
        eprintln!("🔧 Configuration: {:#?}", config);
        eprintln!("📊 Memory usage: {} MB", get_memory_usage());
    }
    
    // Actual program output to stdout
    match run_program(&config) {
        Ok(results) => {
            for result in results {
                println!("{}", result);  // Clean output for piping
            }
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            process::exit(1);
        }
    }
}
```

## 🌍 Cross-Platform Considerations

### **Windows vs Unix Differences**
```rust
// Cross-platform error handling
#[cfg(windows)]
fn handle_file_error(error: std::io::Error, path: &str) {
    match error.kind() {
        std::io::ErrorKind::NotFound => {
            eprintln!("❌ File not found: {}", path);
            eprintln!("💡 Check if the path uses backslashes: C:\\path\\file.txt");
        }
        std::io::ErrorKind::PermissionDenied => {
            eprintln!("❌ Access denied: {}", path);
            eprintln!("💡 Try running as Administrator");
        }
        _ => eprintln!("❌ File error: {}", error),
    }
}

#[cfg(unix)]
fn handle_file_error(error: std::io::Error, path: &str) {
    match error.kind() {
        std::io::ErrorKind::NotFound => {
            eprintln!("❌ File not found: {}", path);
            eprintln!("💡 Check the path: {}", path);
        }
        std::io::ErrorKind::PermissionDenied => {
            eprintln!("❌ Permission denied: {}", path);
            eprintln!("💡 Try: chmod +r {} or run with sudo", path);
        }
        _ => eprintln!("❌ File error: {}", error),
    }
}
```

### **Terminal Detection**
```rust
use std::io::IsTerminal;

fn print_with_colors(message: &str, is_error: bool) {
    let use_colors = std::io::stderr().is_terminal();
    
    if use_colors {
        if is_error {
            eprintln!("\x1b[31m❌ {}\x1b[0m", message);  // Red
        } else {
            eprintln!("\x1b[32m✅ {}\x1b[0m", message);  // Green  
        }
    } else {
        // No colors when output is redirected
        let prefix = if is_error { "ERROR:" } else { "INFO:" };
        eprintln!("{} {}", prefix, message);
    }
}
```

## 🔗 Real-World Examples

### **Our Error Messages Demo**
See the complete implementation in [[../rust_book/Ch12/error_messages/|Chapter 12.6 Error Messages Demo]]:

```rust
// From our error_messages_demo project
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("❌ Problem parsing arguments: {}", err);
        show_help();  // Help goes to stderr
        process::exit(1);
    });
    
    // Debug info to stderr
    eprintln!("Configuration: {:?}", config);
    
    match run_search(&config) {
        Ok(results) => {
            eprintln!("Found {} match(es):", results.len());
            for result in results {
                println!("{}", result);  // Results to stdout
            }
        }
        Err(e) => {
            eprintln!("❌ Application error: {}", e);
            process::exit(1);
        }
    }
}
```

### **Production CLI Pattern**
```rust
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,
    
    #[arg(short, long)]
    quiet: bool,
    
    command: String,
}

fn main() {
    let cli = Cli::parse();
    
    // Set up logging based on verbosity
    let log_level = match (cli.verbose, cli.quiet) {
        (true, _) => LogLevel::Debug,
        (false, true) => LogLevel::Error,
        (false, false) => LogLevel::Info,
    };
    
    match execute_command(&cli.command, log_level) {
        Ok(output) => println!("{}", output),  // Results to stdout
        Err(e) => {
            eprintln!("❌ {}", e);  // Errors to stderr
            process::exit(1);
        }
    }
}
```

## 🧪 Testing and Validation

### **Stream Separation Test Suite**
Create comprehensive tests using our demo script pattern:

```powershell
# PowerShell test script
function Test-StreamBehavior {
    param($Program, $Args)
    
    Write-Host "🧪 Testing: $Program $Args" -ForegroundColor Cyan
    
    # Test 1: Normal output
    & $Program @Args > stdout.txt 2> stderr.txt
    $exitCode = $LASTEXITCODE
    
    Write-Host "📄 stdout:" -ForegroundColor Green
    Get-Content stdout.txt | ForEach-Object { "  $_" }
    
    Write-Host "📄 stderr:" -ForegroundColor Yellow  
    Get-Content stderr.txt | ForEach-Object { "  $_" }
    
    Write-Host "🔢 Exit code: $exitCode" -ForegroundColor $(if ($exitCode -eq 0) { "Green" } else { "Red" })
    
    Remove-Item stdout.txt, stderr.txt -ErrorAction SilentlyContinue
    Write-Host ""
}
```

### **Automated Validation**
```bash
#!/bin/bash
# Bash test script for stream validation

test_stream_separation() {
    local program="$1"
    local expected_stdout="$2"  
    local expected_stderr="$3"
    local expected_exit="$4"
    
    # Capture streams
    stdout=$(eval "$program" 2>/dev/null)
    stderr=$(eval "$program" 2>&1 >/dev/null)
    exit_code=$?
    
    # Validate expectations
    [[ "$stdout" == *"$expected_stdout"* ]] || echo "❌ stdout mismatch"
    [[ "$stderr" == *"$expected_stderr"* ]] || echo "❌ stderr mismatch"  
    [[ "$exit_code" == "$expected_exit" ]] || echo "❌ exit code mismatch"
    
    echo "✅ Test passed: $program"
}
```

## 📚 Integration Points

### **Cross-References**
- **[[Error Handling Deep Dive]]** - Advanced error handling patterns
- **[[CLI Design Patterns]]** - Command-line interface best practices  
- **[[Chapter 12 Overview]]** - Complete minigrep implementation
- **[[../rust_book/Ch12/error_messages/README_DEMO]]** - Hands-on examples
- **[[../daily_study/rust_learning_week5_notes/README]]** - Advanced error handling

### **Learning Track Integration**
- **Rust Book Ch12.6** - Foundation concepts
- **Daily Study Week 2** - Basic I/O patterns  
- **Daily Study Week 5** - Advanced error handling
- **Mission Projects** - CLI tool development
- **AoC Solutions** - Input validation and error reporting

### **Code Examples**
- **[[../rust_book/Ch12/error_messages/src/main.rs]]** - Complete implementation
- **[[../rust_book/Ch12/error_messages/demo.ps1]]** - Testing framework
- **[[../missions/Mission9/examples/]]** - Advanced CLI patterns

## 🎯 Best Practices Summary

### **✅ DO:**
- Use `println!` for program output/results
- Use `eprintln!` for errors, warnings, debug info
- Provide clear, actionable error messages  
- Include context and suggestions in errors
- Test stream separation in your CLI tools
- Use appropriate exit codes (0 = success, 1+ = error)
- Consider terminal detection for colors/formatting

### **❌ DON'T:**
- Mix results and diagnostics in the same stream
- Write errors to stdout (breaks piping)
- Write progress/debug info to stdout  
- Ignore stderr when testing CLI tools
- Use unclear or cryptic error messages
- Forget to handle different platforms

### **🧪 Testing Checklist:**
- [ ] stdout contains only program results
- [ ] stderr contains only diagnostics/errors  
- [ ] Redirection works correctly (>, 2>, 2>&1)
- [ ] Exit codes are appropriate
- [ ] Error messages are helpful and actionable
- [ ] Works correctly when streams are redirected
- [ ] Cross-platform compatibility tested

## 📖 Further Reading

- **Unix Programming Environment** - Stream philosophy
- **Rust std::io documentation** - Stream manipulation
- **clap CLI framework** - Professional CLI development
- **PowerShell redirection** - Windows-specific patterns
- **ANSI escape codes** - Terminal formatting

---

*This comprehensive guide covers everything from basic concepts to advanced patterns for proper stream separation in CLI applications.*

*Created: 2025-10-26*  
*Tags: #cli #stdout #stderr #streams #error-handling #rust #chapter12 #testing*  
*Links: [[Error Handling Deep Dive]] | [[CLI Design Patterns]] | [[Chapter 12 Overview]] | [[Rust Concepts MOC]]*