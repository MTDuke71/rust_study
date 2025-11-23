# 🔧 Day 7 AoC Debug Output Tools - Usage Guide

This directory now contains a comprehensive set of tools for generating and viewing debug output from the Day 7 AoC circuit simulation.

## 📁 Generated Files

- **`day07_debug_output_2025-09-30_19-33-06.txt`** (5.68 MB, 236,520 lines)
  - Complete debug output showing every instruction parse, wire evaluation, and hash table state
  - Includes both Part 1 and Part 2 full execution traces with emoji-decorated logging

- **`day07_debug_advanced_2025-09-30_19-34-20_filtered.txt`** (2.17 KB, 50 lines)
  - Filtered output showing only results and key milestones
  - Demonstrates advanced filtering capabilities

## 🛠️ Available Tools

### 1. **Basic Debug Runner** - `.\run_day07_debug.ps1`
```powershell
# Run with default settings (full debug output)
.\run_day07_debug.ps1

# Use custom input file
.\run_day07_debug.ps1 -InputFile "my_circuit.txt" -OutputFile "my_debug.txt"

# Show output to console as well as saving to file
.\run_day07_debug.ps1 -ShowOutput
```

**Features:**
- ✅ Automatic timestamped output files
- ✅ File size and line count reporting
- ✅ Preview of first/last lines
- ✅ Usage tips and recommendations

### 2. **Advanced Debug Runner** - `.\run_day07_advanced_debug.ps1`
```powershell
# Show help with all options
.\run_day07_advanced_debug.ps1 -Help

# Filter to show only final results
.\run_day07_advanced_debug.ps1 -FilterSections "results"

# Show only instruction parsing
.\run_day07_advanced_debug.ps1 -FilterSections "instructions"

# Combine filters and limit output
.\run_day07_advanced_debug.ps1 -FilterSections "instructions,results" -MaxLines 100

# Create custom input interactively
.\run_day07_advanced_debug.ps1 -CreateInput
```

**Advanced Features:**
- 🎯 **Section Filtering**: `instructions`, `memo`, `evaluation`, `results`
- 🔢 **Line Limiting**: Prevent massive output files
- ✏️ **Interactive Input Creation**: Build custom circuits
- 📊 **Smart Analysis**: Automatic filtering and statistics

### 3. **Interactive Viewer** - `.\view_day07_debug.ps1`
```powershell
# Browse all available debug files
.\view_day07_debug.ps1

# View specific file
.\view_day07_debug.ps1 -File "day07_debug_output*.txt"

# Search for specific content
.\view_day07_debug.ps1 -Search "wire.*a"

# Custom page size
.\view_day07_debug.ps1 -Lines 30
```

**Viewer Features:**
- 🗂️ **File Browser**: Select from available debug files
- 🔍 **Search & Highlight**: Find specific patterns
- 📄 **Pagination**: Navigate large files easily
- 🎯 **Go to Line**: Jump to specific locations
- ⌨️ **Interactive Controls**: Navigate with simple commands

## 🎯 Common Usage Patterns

### **Quick Results Check**
```powershell
.\run_day07_advanced_debug.ps1 -FilterSections "results" -MaxLines 20
```

### **Debug Instruction Parsing**  
```powershell
.\run_day07_advanced_debug.ps1 -FilterSections "instructions" -MaxLines 50
```

### **Full Trace with Custom Input**
```powershell
# Create your own circuit
.\run_day07_advanced_debug.ps1 -CreateInput
# Then view the results
.\view_day07_debug.ps1 -Search "Part.*result"
```

### **Analyze Memoization Performance**
```powershell
.\run_day07_advanced_debug.ps1 -FilterSections "memo,evaluation" -MaxLines 200
```

## 🎨 Debug Output Format

The debug output includes:

- **🔧 Debug mode enabled** - Startup confirmation
- **📁 Input file processing** - File loading status  
- **📋 Instruction parsing** - Each `wire -> instruction` mapping with emoji markers
- **🗃️ Hash table states** - Real-time instruction and memo tables
- **⚡ Wire evaluation** - Step-by-step computation with dependency resolution
- **🎯 Part 1/2 results** - Final answers with clear formatting
- **📊 Performance data** - Memo hit rates and evaluation counts

## 🔍 Search Tips

When using the viewer or filtering:

- **Wire values**: `"wire.*a"` or `"a\s*=\s*\d+"` 
- **Instructions**: `"AND|OR|NOT|LSHIFT|RSHIFT"`
- **Results**: `"Part.*result|final.*result"`
- **Performance**: `"memo|cache|memoiz"`
- **Errors**: `"error|fail|invalid"`

## 💡 Pro Tips

1. **Large Files**: Use filtering to reduce output size before viewing
2. **Performance Analysis**: Search for "memo" to see caching effectiveness  
3. **Debugging Circuits**: Create small custom inputs to isolate issues
4. **Pattern Recognition**: Use the viewer's search to find evaluation patterns
5. **File Management**: Scripts auto-timestamp files to avoid overwriting

## 📈 File Size Guidelines

- **Full debug**: ~5-6 MB for 339-line AoC input (expect 200K+ lines)
- **Filtered results**: ~2 KB for results-only (expect 50-100 lines)
- **Instructions only**: ~50-100 KB for parsing info (expect 1K+ lines)
- **Custom inputs**: Varies by circuit complexity

## 🚀 Next Steps

Try these progressively:

1. **Start simple**: `.\run_day07_advanced_debug.ps1 -FilterSections "results"`
2. **Explore parsing**: `.\run_day07_advanced_debug.ps1 -FilterSections "instructions" -MaxLines 100`
3. **Full experience**: `.\run_day07_debug.ps1` (be ready for large files!)
4. **Interactive browsing**: `.\view_day07_debug.ps1` 
5. **Custom circuits**: `.\run_day07_advanced_debug.ps1 -CreateInput`

The debug tools provide complete visibility into the AoC Day 7 circuit simulation algorithm! 🎉

---

## 🔗 Related Zettelkasten Concepts

**Core Debugging Concepts:**
- [[debugging-strategies]] - Systematic debugging approaches
- [[tracing]] - Program execution tracing and logging
- [[debug-output]] - Structured debug output design
- [[interactive-debugging]] - Interactive debugging tools and workflows

**AoC-Specific:**
- [[day07]] - AoC 2015 Day 7 problem (circuit simulation)
- [[aoc2015]] - Full AoC 2015 problem set
- [[AoC 2015 MOC]] - Overview and navigation for AoC 2015
- [[bitwise-operations]] - AND, OR, NOT, LSHIFT, RSHIFT operations
- [[circuit-simulation]] - Digital circuit evaluation patterns

**Data Structures:**
- [[hashmap]] - Wire storage and memoization
- [[memoization]] - Caching computed values for performance
- [[graph-algorithms]] - Dependency resolution and evaluation order
- [[lazy-evaluation]] - Computing values only when needed

**Problem-Solving Patterns:**
- [[dependency-graph]] - Wire dependencies and evaluation order
- [[topological-sort]] - Evaluating dependencies in order
- [[recursive-evaluation]] - Recursive wire value computation

**Tooling & Scripting:**
- [[powershell-scripting]] - PowerShell automation scripts
- [[log-filtering]] - Filtering and analyzing log output
- [[interactive-tools]] - Building interactive command-line tools
- [[file-management]] - Timestamped output and file organization

**Performance Analysis:**
- [[performance-benchmarking-grid-optimization]] - Performance measurement techniques
- [[cache-hit-rate]] - Memoization effectiveness analysis
- [[profiling]] - Performance profiling and optimization

**Learning Resources:**
- [[Daily Study MOC]] - Daily learning progression
- [[mission-5]] - HashMap implementation (used for memoization)
- [[mission-7]] - Graph algorithms (dependency resolution)

*Tags: #aoc #aoc2015 #day07 #debugging #tools #memoization #circuit-simulation #hashmap #interactive-tools #powershell*