# Scripts - Workspace Automation Tools

**Utility scripts for running, testing, and managing the learning workspace**

---

## 🛠️ **Available Scripts**

### **run_md.bat** - Markdown Code Runner (Windows)
Extracts and executes Rust code from markdown files automatically.

```batch
# Usage
.\scripts\run_md.bat <path_to_markdown_file>

# Examples
.\scripts\run_md.bat daily_study\rust_learning_week2_notes\Day10.md
.\scripts\run_md.bat rust_book\Ch4\README.md
```

**Features:**
- **Automatic extraction** - Finds "Complete Runnable Example" sections
- **Temporary compilation** - Creates temp Rust files for execution
- **Error handling** - Clear messages for missing code or compilation issues
- **Cross-platform paths** - Handles both Windows and Unix path separators

### **run_markdown_code.ps1** - PowerShell Code Runner
Advanced PowerShell version with additional features and error handling.

```powershell
# Usage
.\scripts\run_markdown_code.ps1 <path_to_markdown_file>

# Examples  
.\scripts\run_markdown_code.ps1 daily_study\rust_learning_week2_notes\Day11.md
```

**Features:**
- **Enhanced parsing** - Better regex-based code extraction
- **Detailed logging** - Verbose output for debugging
- **Multiple code blocks** - Can handle files with multiple Rust examples
- **PowerShell integration** - Native PowerShell error handling and output

---

## 🎯 **Primary Use Cases**

### **Daily Study Workflow**
```bash
# Test today's learning example
.\scripts\run_md.bat daily_study\rust_learning_week2_notes\Day10.md

# Verify all examples in a week
for %f in (daily_study\rust_learning_week2_notes\*.md) do .\scripts\run_md.bat "%f"
```

### **Mission Development**
```bash
# Test mission documentation examples
.\scripts\run_md.bat Mission5\README.md

# Validate tutorial progression  
.\scripts\run_md.bat Mission5_tut\step1_basic_structure.md
```

### **Rust Book Exercises**
```bash
# Run chapter examples that include markdown documentation
.\scripts\run_md.bat rust_book\Ch4\README.md
```

### **Content Creation Validation**
- **Before committing** - Ensure all runnable examples work
- **Tutorial development** - Test step-by-step progressions
- **Documentation updates** - Verify code examples remain functional

---

## 🔧 **Technical Details**

### **Code Extraction Logic**
Both scripts look for these patterns:
```markdown
## 🚀 **Complete Runnable Example**

```rust
fn main() {
    // Your code here
}
```
```

**Search Priority:**
1. **"Complete Runnable Example"** sections (highest priority)
2. **First Rust code block** if no complete example found
3. **Error message** if no Rust code found

### **Compilation Process**
1. **Extract** Rust code from markdown
2. **Create** temporary `.rs` file
3. **Compile** with `rustc` 
4. **Execute** compiled binary
5. **Cleanup** temporary files

### **Error Handling**
- **Missing files** - Clear error message with usage instructions
- **No Rust code** - Warning with suggestion to add examples  
- **Compilation errors** - Full rustc output for debugging
- **Runtime errors** - Program output with exit codes

---

## 🚀 **Integration with Learning System**

### **3-Track Learning Support**
- **V-Cycle Missions** - Validate implementation examples in documentation
- **Daily Study** - Test complete runnable examples in Day files
- **Rust Book** - Execute chapter-specific documentation examples

### **Quality Assurance Integration**
```markdown
## Workflow Checklist
- [ ] Write code example
- [ ] Test with `.\scripts\run_md.bat filename.md`
- [ ] Fix any compilation or runtime issues  
- [ ] Commit working example
```

### **Zettelkasten Integration**
- **Cross-validation** - Ensure linked examples remain functional
- **Content verification** - Test examples when updating knowledge base
- **Reference integrity** - Validate code snippets in MOC files

---

## 🔄 **Future Enhancements**

### **Planned Features**
- **Linux/Mac support** - Shell script versions for cross-platform development
- **Multiple language support** - Extract and run Python, JavaScript examples
- **Batch processing** - Run all markdown files in a directory tree
- **Integration testing** - Validate cross-file dependencies

### **Performance Optimizations**
- **Caching** - Avoid recompilation of unchanged examples
- **Parallel execution** - Run multiple examples simultaneously  
- **Incremental updates** - Only test files that have changed

---

## 🛡️ **Best Practices**

### **Script Usage Guidelines**
- **Always test** before committing new markdown content
- **Use absolute paths** when calling from subdirectories
- **Check exit codes** in automated workflows
- **Include usage examples** in all markdown files

### **Code Example Standards**
- **Self-contained** - Include all necessary dependencies
- **Commented** - Explain key concepts within code
- **Realistic** - Use practical examples, not just "hello world"
- **Progressive** - Build complexity appropriately for learning level

### **Maintenance**
- **Regular testing** - Run scripts on all content periodically
- **Version control** - Track script changes with workspace evolution
- **Documentation updates** - Keep this README current with script capabilities

---

*These scripts are essential infrastructure for maintaining the quality and reliability of the learning workspace's executable content.*

*Tags: #scripts #automation #testing #markdown-processing #quality-assurance #workflow*
*Links: [[../daily_study/README]] | [[../.github/COMPLETE_RUNNABLE_EXAMPLE_TEMPLATE]] | [[../zettelkasten/README]] | [[../Mission5/README]]*