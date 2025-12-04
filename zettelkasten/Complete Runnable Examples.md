# Complete Runnable Examples

*A core documentation standard ensuring all code examples are immediately executable, testable, and educational.*

---

## Core Philosophy

**Every code example in this workspace must be complete, runnable, and self-contained.** This isn't just about code quality—it's about **learning effectiveness** and **knowledge retention**.

### The Complete Runnable Example Standard

A "Complete Runnable Example" means:

1. **Self-Contained** - Includes all necessary imports, dependencies, and setup
2. **Immediately Executable** - Can be run without modification
3. **Produces Output** - Demonstrates behavior with visible results
4. **Educational** - Teaches concepts through working code
5. **Tested** - Verified to compile and run correctly

## Why This Matters

### 🎯 Learning Effectiveness

- **Active Learning** - Students can run and modify code immediately
- **Concept Validation** - See theory in practice
- **Error Exploration** - Experiment safely with working baseline
- **Confidence Building** - "It works!" reinforces understanding

### 🔧 Documentation Quality

- **No Guesswork** - Readers don't fill in missing pieces
- **Reduced Friction** - Copy, paste, run—instant feedback
- **Version Safety** - Examples don't break with Rust updates
- **Trust Building** - Demonstrates author expertise

### 📚 Knowledge Retention

- **Hands-On Practice** - Muscle memory through execution
- **Immediate Feedback** - Compiler errors teach syntax
- **Pattern Recognition** - Working examples show best practices
- **Spaced Repetition** - Runnable examples enable review

## Template Structure

### Markdown Section Format

```markdown
## Complete Runnable Example

This example demonstrates [concept] with [key features]:

\`\`\`rust
// Complete imports
use std::collections::HashMap;

// Full implementation
fn main() {
    // Setup
    let mut map = HashMap::new();
    
    // Core concept demonstration
    map.insert("key", "value");
    
    // Output for verification
    println!("Result: {:?}", map.get("key"));
    // Output: Result: Some("value")
}
\`\`\`

**Key Points:**
- [Learning objective 1]
- [Learning objective 2]
- [Common pitfall to avoid]

**Try It:**
```bash
cargo run --example [example_name]
```

```

## Implementation Patterns

### 1. Standalone Main Function

```rust
// File: examples/standalone_demo.rs
fn main() {
    println!("Complete example with output");
    
    // All code needed to demonstrate concept
    let data = vec![1, 2, 3];
    let sum: i32 = data.iter().sum();
    
    println!("Sum: {}", sum);
}
```

### 2. Library with Example

```rust
// File: src/lib.rs
pub fn calculate(x: i32) -> i32 {
    x * 2
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculate() {
        assert_eq!(calculate(5), 10);
    }
}

// File: examples/demo.rs
use my_crate::calculate;

fn main() {
    let result = calculate(21);
    println!("Result: {}", result);
}
```

### 3. Module with Tests

```rust
// Complete module demonstration
pub mod queue {
    pub struct Queue<T> {
        items: Vec<T>,
    }
    
    impl<T> Queue<T> {
        pub fn new() -> Self {
            Queue { items: Vec::new() }
        }
        
        pub fn enqueue(&mut self, item: T) {
            self.items.push(item);
        }
        
        pub fn dequeue(&mut self) -> Option<T> {
            if self.items.is_empty() {
                None
            } else {
                Some(self.items.remove(0))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::queue::Queue;
    
    #[test]
    fn complete_example() {
        let mut q = Queue::new();
        q.enqueue(1);
        q.enqueue(2);
        assert_eq!(q.dequeue(), Some(1));
        assert_eq!(q.dequeue(), Some(2));
        assert_eq!(q.dequeue(), None);
    }
}
```

## Daily Study Integration

### Day Notes Format

Each daily study file includes runnable examples:

```markdown
# Day X - [Topic]

## Complete Runnable Example

\`\`\`rust
fn main() {
    // Complete, executable code
    let example = "demonstrates concept";
    println!("{}", example);
}
\`\`\`

## Running This Example

```bash
cd daily_study/rust_learning_weekX_notes
cargo run --example dayX_standalone
```

```

### Script Support

```powershell
# Extract and run code from markdown
.\scripts\run_md.bat daily_study\rust_learning_week5_notes\Day34.md
```

## Mission Integration

### Mission README Pattern

Every mission README includes:

```markdown
## Quick Start Example

\`\`\`rust
use mission5::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("key", 42);
    println!("Value: {:?}", map.get("key"));
}
\`\`\`

Run with:
```bash
cargo run -p mission5 --example demo
```

```

### Tutorial Examples

Tutorials progress through increasingly complex examples:

```

tutorials/Mission5_tut/examples/
├── step1_basic_hashmap.rs       # Complete, runnable
├── step2_collision_handling.rs  # Complete, runnable
├── step3_resize_strategy.rs     # Complete, runnable
└── step7_final_project.rs       # Complete, runnable

```

## Quality Checklist

### ✅ Example Requirements

- [ ] **Compiles** - `cargo build` succeeds
- [ ] **Runs** - `cargo run` executes without panic
- [ ] **Output** - Produces visible results
- [ ] **Imports** - All `use` statements included
- [ ] **Setup** - Any required initialization present
- [ ] **Documentation** - Comments explain key concepts
- [ ] **Error Handling** - Demonstrates proper patterns
- [ ] **Testing** - Includes test cases where appropriate

### ✅ Educational Value

- [ ] **Clear Purpose** - States what it demonstrates
- [ ] **Progressive** - Builds on previous knowledge
- [ ] **Focused** - One concept per example
- [ ] **Annotated** - Comments highlight key points
- [ ] **Runnable Output** - Shows expected results
- [ ] **Modification Friendly** - Easy to experiment with

### ✅ Documentation Standards

- [ ] **Title** - "Complete Runnable Example" heading
- [ ] **Context** - Brief explanation of concept
- [ ] **Code Block** - Properly formatted with syntax highlighting
- [ ] **Expected Output** - Shows what running produces
- [ ] **Try It Section** - Instructions for execution
- [ ] **Key Takeaways** - Learning objectives listed

## Validation Tools

### Automated Testing

```powershell
# Test all markdown examples
.\scripts\run_markdown_code.ps1 missions\Mission5\README.md

# Test specific day's examples
.\scripts\run_md.bat daily_study\rust_learning_week5_notes\Day34.md
```

### Quality Pipeline

```powershell
# Comprehensive validation
.\scripts\quality-pipeline.ps1

# Checks:
# - All examples compile
# - Tests pass
# - Clippy warnings addressed
# - Documentation complete
```

## Common Patterns

### 1. Data Structure Demonstration

```rust
fn main() {
    // Create
    let mut stack = Vec::new();
    
    // Populate
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    // Use
    while let Some(item) = stack.pop() {
        println!("Popped: {}", item);
    }
    // Output:
    // Popped: 3
    // Popped: 2
    // Popped: 1
}
```

### 2. Algorithm Implementation

```rust
fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        match arr[mid].cmp(target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
        }
    }
    None
}

fn main() {
    let data = vec![1, 3, 5, 7, 9];
    assert_eq!(binary_search(&data, &5), Some(2));
    assert_eq!(binary_search(&data, &4), None);
    println!("Binary search works!");
}
```

### 3. Error Handling Pattern

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file_contents(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    // Complete example with both success and failure cases
    match read_file_contents("example.txt") {
        Ok(contents) => println!("File: {}", contents),
        Err(e) => println!("Error: {}", e),
    }
}
```

## Anti-Patterns to Avoid

### ❌ Incomplete Code

```rust
// BAD: Missing imports and context
fn process() {
    let map = HashMap::new();  // What HashMap?
    // ...
}
```

### ❌ Unrealistic Examples

```rust
// BAD: Won't compile
fn example() {
    let x = 5;
    // ... 200 lines later ...
    println!("{}", x);  // Did we move x?
}
```

### ❌ Missing Setup

```rust
// BAD: Assumes existing state
fn demo() {
    data.push(5);  // What is data?
}
```

### ✅ Complete Examples

```rust
// GOOD: Complete, runnable, educational
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("rust", 2015);
    
    if let Some(&year) = map.get("rust") {
        println!("Rust 1.0 released in {}", year);
    }
}
```

## Tools and Scripts

### Extraction Script

```powershell
# scripts/run_markdown_code.ps1
# Extracts code blocks from markdown and runs them
param([string]$MarkdownFile)

$codeBlocks = Select-String -Path $MarkdownFile `
    -Pattern '```rust(.*?)```' -AllMatches

foreach ($block in $codeBlocks) {
    $tempFile = New-TemporaryFile
    $block.Matches.Groups[1].Value | Out-File $tempFile
    rustc $tempFile.FullName && .\a.exe
}
```

### Validation Script

```powershell
# Validate all examples in workspace
Get-ChildItem -Recurse -Filter "*.md" | ForEach-Object {
    Write-Host "Validating: $_"
    .\scripts\run_markdown_code.ps1 $_.FullName
}
```

## Mission-Specific Examples

### Mission 1: Stack

- **examples/demo.rs** - Complete stack usage
- Tests demonstrate all operations
- Performance benchmarks included

### Mission 5: HashMap

- **examples/step1-7.rs** - Progressive learning
- Each step is complete and runnable
- Builds toward full implementation

### Mission 9: Pathfinding

- **examples/dijkstra_demo.rs** - Complete graph pathfinding
- Includes graph setup and visualization
- Shows algorithm step-by-step

## Learning Benefits

### Immediate Feedback Loop

```
Read Example → Run Code → See Results → Understand Concept
     ↑                                            ↓
     └────────────── Modify & Experiment ←────────┘
```

### Confidence Building

1. **"It works!"** - Validation of understanding
2. **"I can modify it"** - Safe experimentation
3. **"I understand why"** - Concept mastery

### Knowledge Transfer

- **See Pattern** - In working code
- **Practice Pattern** - By modifying
- **Apply Pattern** - In own projects

## Related Standards

### Documentation Templates

- [[../.github/COMPLETE_RUNNABLE_EXAMPLE_TEMPLATE.md]] - Official template
- [[../.github/RUST_DOCUMENTATION_STANDARDS.md]] - rustdoc standards
- [[../.github/DAILY_STUDY_CREATION_GUIDE.md]] - Daily study format

### Quality Assurance

- [[Quality Assurance]] - Quality metrics and standards
- [[V-Cycle Integration]] - Verification through examples
- [[REQ-1 Test Strategy]] - Testing methodology

## Best Practices Summary

### 🎯 Do's

✅ Include all imports and dependencies  
✅ Provide expected output in comments  
✅ Make examples self-contained  
✅ Test examples before committing  
✅ Use meaningful variable names  
✅ Add "Complete Runnable Example" heading  
✅ Include execution instructions  

### 🚫 Don'ts

❌ Use placeholder code like `// ... rest of code`  
❌ Assume reader context  
❌ Reference undefined variables  
❌ Omit error handling in examples  
❌ Use incomplete type definitions  
❌ Skip testing examples  
❌ Forget output demonstrations  

## Implementation Workflow

### Creating New Examples

1. **Write Complete Code** - All imports, full implementation
2. **Test Execution** - `cargo run --example name`
3. **Verify Output** - Matches expected results
4. **Document** - Add explanation and context
5. **Validate** - Run quality checks
6. **Commit** - Include in version control

### Updating Existing Examples

1. **Verify Current State** - Does it still run?
2. **Make Changes** - Preserve completeness
3. **Re-test** - Ensure still works
4. **Update Documentation** - Reflect changes
5. **Validate** - Quality pipeline check

## Success Metrics

### Example Quality

- **100% Compilation Rate** - All examples compile
- **100% Execution Rate** - All examples run
- **Zero Manual Setup** - No configuration needed
- **Clear Output** - Results demonstrate concept

### Learning Impact

- **Reduced Questions** - "How do I run this?"
- **Increased Confidence** - Students can experiment
- **Better Retention** - Hands-on reinforces learning
- **Faster Progress** - No debugging incomplete code

---

## Related Resources

- [[learning-plan]] - Learning schedule with complete examples
- [[CALENDER_ARCHIVE]] - Historical examples and patterns
- [[Zettelkasten System]] - Knowledge management approach
- [[AoC Integration]] - Advent of Code complete solutions
- [[../.github/COMPLETE_RUNNABLE_EXAMPLE_TEMPLATE.md]] - Official template
- [[V-Cycle Integration]] - Testing and validation methodology
- [[Quality Assurance]] - Quality standards and metrics

*Tags: #documentation-standards #complete-examples #learning-methodology #code-quality #educational-design #rust-learning #best-practices #workspace-standards*

---

*This standard ensures every learner can immediately execute, understand, and build upon examples—transforming passive reading into active learning.*
