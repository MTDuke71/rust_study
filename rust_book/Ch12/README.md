# Chapter 12: An I/O Project: Building a Command Line Program

## 🔗 Zettelkasten Links
- **Overview**: [[Chapter 12 Overview]]
- **Previous**: [[Chapter 11 Overview]]
- **Next**: [[Chapter 13 Overview]]
- **Missions**: [[mission8 Overview]] - Advanced project structure | [[Mission9 Overview]] - CLI pathfinding tools
- **Daily Study**: [[Day 42 - CLI Applications]] - Reinforces this chapter
- **Book MOC**: [[Rust Book MOC]]

## 📚 Overview
Chapter 12 builds a complete command-line program that combines many Rust concepts learned in previous chapters. We'll create `minigrep`, a simplified version of the `grep` tool, demonstrating real-world Rust development patterns including command-line argument parsing, file I/O, error handling, and project organization.

---

## 🎯 Key Concepts

### 1. **Command-Line Arguments**
Rust provides access to command-line arguments through `std::env::args()`.

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Program: {}", args[0]);
    println!("Arguments: {:?}", &args[1..]);
}
```

### 2. **File I/O Operations**
Reading files safely with error handling using `std::fs`.

```rust
use std::fs;
use std::error::Error;

fn read_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    Ok(contents)
}
```

### 3. **Error Handling Strategy**
Using `Result<T, E>` for recoverable errors and proper error propagation.

```rust
use std::error::Error;
use std::process;

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    
    Ok(())
}

fn main() {
    // ... config parsing ...
    
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
```

### 4. **Project Organization**
Separating concerns by extracting logic into `lib.rs`.

```rust
// lib.rs
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        
        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    
    results
}
```

### 5. **Testing Strategy**
Writing comprehensive tests for CLI applications.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."], 
            search_case_insensitive(query, contents)
        );
    }
}
```

---

## 🔑 Key Takeaways

### Command-Line Development Benefits
- **Argument Parsing**: `std::env::args()` provides simple access to CLI arguments
- **Error Handling**: Proper use of `Result<T, E>` for robust error management
- **Environment Variables**: `std::env::var()` for configuration options
- **Exit Codes**: `std::process::exit()` for proper program termination

### Project Organization Patterns
1. **Separation of Concerns** - Business logic in `lib.rs`, CLI handling in `main.rs`
2. **Configuration Structs** - Centralized configuration management
3. **Error Propagation** - Using `?` operator for clean error handling
4. **Testable Code** - Extracting pure functions for easy testing

### I/O Best Practices
- **File Reading** - `std::fs::read_to_string()` for simple text file operations
- **Standard Streams** - `println!()` for output, `eprintln!()` for errors
- **Iterator Patterns** - Using `.lines()`, `.split()`, and other iterators
- **Memory Efficiency** - Working with string slices to avoid unnecessary allocations

---

## 🛠️ Common Patterns

### Configuration Pattern
```rust
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // Skip program name
        
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        
        Ok(Config { query, filename, case_sensitive })
    }
}
```

### Search Function Pattern
```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
```

### Error Handling Pattern
```rust
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    
    for line in results {
        println!("{}", line);
    }
    
    Ok(())
}
```

### Main Function Pattern
```rust
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
```

---

## 🧠 Mental Model

### Think of CLI Programs as:
- **Input Processing Pipeline** = Arguments → Configuration → Execution → Output
- **Error Boundary** = Main function catches and handles all errors gracefully  
- **Layered Architecture** = CLI layer (main.rs) + Business logic layer (lib.rs)

**The CLI Development Flow:**
1. **Parse Arguments** → Extract and validate user input
2. **Configure Application** → Build configuration from arguments and environment
3. **Execute Business Logic** → Perform the core functionality
4. **Handle Results** → Output results or error messages appropriately

**Key Principle:** Separate CLI concerns (argument parsing, error display) from business logic (file processing, search algorithms) for better testing and maintainability.

---

## 📖 Further Reading
- [The Rust Book Chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
- [std::env documentation](https://doc.rust-lang.org/std/env/)
- [std::fs documentation](https://doc.rust-lang.org/std/fs/)
- [Error Handling in Rust](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

---

## 🔗 Related Content

**Missions:**
- [[mission8 Overview]] - Advanced project structure patterns
- [[Mission9 Overview]] - CLI pathfinding tools with argument parsing

**Daily Study:**
- [[Day 42 - CLI Applications]] - Practical CLI development exercises
- [[Day 38 - Error Handling]] - Comprehensive error management strategies

**Next Steps:**
- Complete exercises in `Ch12/accepting_arguments/`, `Ch12/reading_files/`, and `Ch12/refactoring/` directories
- Review [[Chapter 13 Overview]] when ready

---

*This chapter demonstrates how to combine multiple Rust concepts into a cohesive, real-world application. Essential for understanding how to structure larger Rust projects and handle I/O operations safely.*

*Links: [[Rust Book MOC]] | [[Chapter 11 Overview]] | [[Chapter 13 Overview]]*
*Tags: #rust-book #chapter12 #cli #io #error-handling #project-structure #testing #foundation*