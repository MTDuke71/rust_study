# Complete Runnable Example Template

## 📋 **Standard Template for Learning Day Files**

Use this template when creating new Day files in `daily_study/rust_learning_week*_notes/`:

```markdown
## 🚀 **Complete Runnable Example**

```rust
// Copy this entire block to Rust Playground or save as a .rs file

fn main() {
    println!("=== [Topic] Demo from Day [X] ===\n");
    
    // 1. Basic concept demonstration
    println!("1. [Concept Name]:");
    // Minimal working example
    
    // 2. Advanced usage
    println!("\n2. [Advanced Pattern]:");
    // More complex example
    
    // 3. Real-world application  
    println!("\n3. Real-World Usage:");
    // Practical AoC-style example
    
    // Add more sections as needed (aim for 4-7 sections)
}

// Helper functions that make the main() function work
```

### **🛠️ How to Run This Code:**

1. **Online**: Copy to [Rust Playground](https://play.rust-lang.org/)
2. **Local file**: Save as `day[X]_demo.rs` and run `rustc day[X]_demo.rs && ./day[X]_demo`
3. **In this workspace**: `**Example Usage:**
```bash
```bash
# PowerShell example
.\scripts\run_md.bat daily_study\rust_learning_week1_notes\Day01.md

# Git Bash example  
./scripts/run_md.bat daily_study/rust_learning_week1_notes/Day01.md
```
````
4. **As Cargo example**: `cargo run --example day[X]_[topic]_demo` (if you add it to Mission5_tut)
```

## 🎯 **Guidelines for Complete Runnable Examples**

### **Essential Elements:**
1. **Self-contained**: All code needed to run (no external dependencies beyond std)
2. **Educational**: Each section teaches a specific concept
3. **Progressive**: Start simple, build to complex
4. **Practical**: Include real-world/AoC-style examples
5. **Commented**: Clear section headers and explanations

### **Section Structure:**
1. **Basic Usage** - Minimal working example of core concept
2. **Key Features** - Demonstrate 2-3 important features/methods
3. **Practical Application** - Real-world or AoC-style usage
4. **Advanced Patterns** - More sophisticated examples
5. **Comparisons** - Compare with alternatives (HashMap vs BTreeMap)
6. **Error Handling** - Show proper error handling when relevant
7. **Performance Notes** - Mention Big-O or performance characteristics

### **Common Helper Functions:**
- Always include helper functions that make main() work
- Use realistic data (names, scores, tasks, etc.)
- Make examples that could appear in AoC problems

### **Testing Your Examples:**
```bash
# Always test with the markdown runner
.\scripts\run_md.bat daily_study\rust_learning_week*_notes\DayX.md

# Should compile and run without errors
# Output should be educational and easy to follow
```

## 📚 **Examples by Topic Category**

### **Collections (HashMap, HashSet, BTreeMap, etc.)**
- Creation methods (new, from, collect)
- Basic operations (insert, remove, contains)
- Iteration patterns
- Real-world data processing

### **Iterators**
- Lazy evaluation demonstration
- Adapter vs consumer chains
- Custom iterator implementation
- Performance comparison with loops

### **Error Handling**
- Basic Result usage with match
- Error propagation with ?
- Custom error types
- Result combinators
- AoC parsing patterns

### **Language Features (traits, generics, etc.)**
- Basic trait implementation
- Generic usage
- Practical applications
- Common patterns

### **AoC-Specific Patterns**
- Grid coordinate processing
- Input parsing
- State tracking
- Algorithm implementations

## ✅ **Checklist for New Day Files**

When creating a new Day file:

- [ ] Core concepts section with fundamentals
- [ ] Multiple code examples throughout
- [ ] Common use cases section
- [ ] Advanced patterns (if applicable)
- [ ] Complete Runnable Example at the end
- [ ] Test the runnable example works
- [ ] Link to next day's topic

### **File Naming Convention:**
- `DayXX.md` where XX is zero-padded (Day01, Day02, ..., Day10, Day11, etc.)
- Place in appropriate `daily_study/rust_learning_week*_notes/` directory
- Update MONTHLY_CALENDAR.md if needed

## 🚀 **Integration with Existing Tools**

The Complete Runnable Examples work seamlessly with:

1. **Markdown Runner**: `.\scripts\run_md.bat` automatically extracts and runs
2. **Rust Playground**: Copy-paste for online execution
3. **Cargo Examples**: Can be added to Mission*_tut crates
4. **Manual Compilation**: Standard `rustc` workflow

This template ensures consistency across all learning materials and provides multiple ways for users to interact with and learn from the code!

---

## 🔧 Troubleshooting Guide

### Common Issues Running Examples

**Issue**: `.\scripts\run_md.bat` not found or won't run
```powershell
# ❌ Problem: File not in current directory or using wrong shell
.\scripts\run_md.bat DayXX.md

# ✅ Solution: Ensure you're in workspace root with PowerShell
cd D:\repos\rust_study
.\run_md.bat rust_learning_week1_notes\Day01.md

# Alternative: Run PowerShell script directly
powershell -ExecutionPolicy Bypass -File .\run_markdown_code.ps1 DayXX.md
```

**Issue**: "cannot find `main` function"
```rust
// ❌ Problem: Missing main() function
fn example_code() {
    println!("Hello");
}

// ✅ Solution: Add main() function that calls examples
fn main() {
    example_code();
}
```

**Issue**: Code compiles but doesn't show output
```rust
// ❌ Problem: Functions defined but never called
fn example1() { println!("Example 1"); }
fn example2() { println!("Example 2"); }

fn main() {
    // Empty main!
}

// ✅ Solution: Call all examples in main
fn main() {
    example1();
    example2();
}
```

**Issue**: "use of undeclared crate or module"
```rust
// ❌ Problem: Missing imports
fn main() {
    let mut map = HashMap::new();  // Error!
}

// ✅ Solution: Add necessary imports at top
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
}
```

### Cross-Platform Compatibility

**Windows:**
```powershell
.\run_md.bat rust_learning_week1_notes\Day01.md
```

**Linux/Mac:**
```bash
# Extract and run code blocks manually
rustc -o /tmp/day01 day01_code.rs && /tmp/day01

# Or use this one-liner to extract first rust block:
awk '/```rust/,/```/' Day01.md | rustc - && ./a.out
```

**Rust Playground (All Platforms):**
1. Open https://play.rust-lang.org/
2. Copy entire code block from markdown
3. Click "Run" button
4. See output immediately

### Debugging Code Examples

**Enable detailed error messages:**
```bash
# Show full backtrace on errors
RUST_BACKTRACE=1 cargo run --example demo

# Show all compiler warnings
cargo build --example demo -- -W warnings
```

**Check for common mistakes:**
```bash
# Run clippy for suggestions
cargo clippy --example demo

# Format code to see structure
cargo fmt --check
```

### Getting Help

- 📋 Check the example's `//!` documentation at top of file
- 🔍 Look for "Common Errors" sections within examples
- 📚 Reference Mission README files for context
- 💡 See [MISSION5_CASE_STUDY.md](.github/MISSION5_CASE_STUDY.md) for working patterns