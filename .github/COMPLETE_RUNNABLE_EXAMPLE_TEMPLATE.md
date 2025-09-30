# Complete Runnable Example Template

## 📋 **Standard Template for Learning Day Files**

Use this template when creating new Day files in `rust_learning_week*_notes/`:

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
3. **In this workspace**: `.\run_md.bat rust_learning_week*_notes\Day[X].md`
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
.\run_md.bat rust_learning_week*_notes\DayX.md

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
- Place in appropriate `rust_learning_week*_notes/` directory
- Update MONTHLY_CALENDAR.md if needed

## 🚀 **Integration with Existing Tools**

The Complete Runnable Examples work seamlessly with:

1. **Markdown Runner**: `.\run_md.bat` automatically extracts and runs
2. **Rust Playground**: Copy-paste for online execution
3. **Cargo Examples**: Can be added to Mission*_tut crates
4. **Manual Compilation**: Standard `rustc` workflow

This template ensures consistency across all learning materials and provides multiple ways for users to interact with and learn from the code!