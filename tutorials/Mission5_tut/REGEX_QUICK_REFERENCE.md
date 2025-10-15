# Regular Expressions Quick Reference

> **Navigation**: [[../../zettelkasten/zettel-index|zettel-index]] | [[../../zettelkasten/Collections MOC|Collections MOC]] | [[../../zettelkasten/AoC Patterns MOC|AoC Patterns MOC]] | [[README|Mission5_tut Overview]]
> 
> **Related**: [[../../daily_study/rust_learning_week4_notes/Day27|Day 27 - String Parsing]] (comprehensive parsing tutorial with regex examples)

## 🧠 Mental Model
**Regex = Pattern Matching Language**
- Think of it as "smart search" that finds patterns, not just exact text
- Like describing what you're looking for instead of typing the exact thing

## 🎯 Core Building Blocks

### 1. Literal Characters
```
hello    → matches exactly "hello"
cat      → matches exactly "cat"
123      → matches exactly "123"
```

### 2. Special Characters (Meta-characters)
```
.   → any single character
^   → start of string
$   → end of string
\   → escape character (makes special chars literal)
```

### 3. Character Classes `[...]`
```
[abc]     → matches 'a' OR 'b' OR 'c'
[a-z]     → any lowercase letter
[A-Z]     → any uppercase letter
[0-9]     → any digit
[^abc]    → NOT 'a', 'b', or 'c' (^ inside [] means NOT)

Shortcuts:
\d = [0-9]           (digits)
\w = [A-Za-z0-9_]    (word characters)
\s = [ \t\n\r]       (whitespace)
```

### 4. Quantifiers (How Many?)
```
?      → 0 or 1 (optional)
*      → 0 or more
+      → 1 or more
{n}    → exactly n times
{n,m}  → between n and m times
{n,}   → n or more times
```

### 5. Anchors (Where?)
```
^pattern  → at start of string
pattern$  → at end of string
^pattern$ → entire string matches pattern
\b        → word boundary
```

### 6. Groups `(...)`
```
(abc)         → group the letters abc together
(cat|dog)     → matches "cat" OR "dog"
(\d{3})-(\d{4}) → captures area code and number separately
```

## 🏆 Common Patterns

### Text Processing
```
\d+              → one or more digits (numbers)
[A-Za-z]+        → one or more letters (words)
\w+              → alphanumeric words
\s+              → one or more whitespace
.*               → anything (greedy)
.+               → something (at least one char)
```

### Validation Patterns
```
^\d{3}-\d{2}-\d{4}$     → SSN format (123-45-6789)
^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$  → email
^https?://[^\s]+$       → basic URL
^\$\d+\.\d{2}$         → money format ($12.34)
^[A-Z][a-z]+$          → capitalized word
```

### Data Extraction
```
(\d{1,2})/(\d{1,2})/(\d{4})  → date MM/DD/YYYY
(\d{3})-(\d{3})-(\d{4})      → phone number
"([^"]*)"                    → quoted strings
<([^>]+)>                    → HTML/XML tags
```

## 🎄 AoC Common Use Cases

### Pattern 1: Counting Characters
```rust
let vowels = Regex::new(r"[aeiou]").unwrap();
let count = vowels.find_iter(text).count();
```

### Pattern 2: Extracting Numbers
```rust
let numbers = Regex::new(r"-?\d+").unwrap();
let nums: Vec<i32> = numbers.find_iter(text)
    .map(|m| m.as_str().parse().unwrap())
    .collect();
```

### Pattern 3: Validation
```rust
let valid = Regex::new(r"^[a-z]+$").unwrap();
if valid.is_match(word) {
    // process valid word
}
```

### Pattern 4: String Splitting
```rust
let parts = Regex::new(r"[,\s]+").unwrap();
let tokens: Vec<&str> = parts.split(line).collect();
```

## 🚫 Rust Regex Limitations

**Not Supported:**
- Backreferences (`\1`, `\2`, etc.)
- Lookahead/lookbehind assertions
- Recursive patterns

**Workarounds:**
```rust
// Instead of ([a-z])\1 for doubles:
text.chars().zip(text.chars().skip(1)).any(|(a, b)| a == b)

// Instead of (?=pattern) for lookahead:
// Use multiple passes or manual checking
```

## 💡 Learning Strategy

### Step 1: Master the Basics
- Literals: `cat`, `123`
- Character classes: `[aeiou]`, `\d`
- Quantifiers: `+`, `*`, `?`

### Step 2: Add Precision
- Anchors: `^`, `$`
- Word boundaries: `\b`
- Exact counts: `{n,m}`

### Step 3: Capture and Extract
- Groups: `(pattern)`
- Alternatives: `(cat|dog)`
- Named captures (in some regex flavors)

### Step 4: Practice with Real Data
- Start with simple patterns
- Test at regex101.com
- Build complexity incrementally

## 🎯 AoC Day 5 Walkthrough

**Problem**: Validate "nice" strings
1. ≥3 vowels
2. Double letter (aa, bb, etc.)
3. No forbidden substrings (ab, cd, pq, xy)

**Solution**:
```rust
fn is_nice(s: &str) -> bool {
    let vowels = Regex::new(r"[aeiou]").unwrap();
    let forbidden = Regex::new(r"ab|cd|pq|xy").unwrap();
    
    let vowel_count = vowels.find_iter(s).count();
    let has_double = s.chars().zip(s.chars().skip(1)).any(|(a, b)| a == b);
    let has_forbidden = forbidden.is_match(s);
    
    vowel_count >= 3 && has_double && !has_forbidden
}
```

**Key Insights**:
- Use regex for simple pattern matching (`[aeiou]`, `ab|cd|pq|xy`)
- Use manual logic for complex patterns (consecutive chars)
- Combine multiple conditions with boolean logic

## 🔥 Pro Tips

1. **Start Simple**: Begin with literal matches, add complexity gradually
2. **Test Frequently**: Use regex101.com or similar tools
3. **Escape Special Chars**: Use `\` before `.`, `*`, `+`, etc. for literal matches
4. **Be Specific**: `\d+` is better than `.+` for numbers
5. **Use Raw Strings**: In Rust, use `r"pattern"` to avoid double escaping

**Remember**: Regex is a tool for pattern matching. If the pattern is complex, sometimes manual string processing is clearer and more maintainable!

---

## 🔗 **Related Documentation**

### **Mission5 Tutorial Materials**
- **[[README|Mission5_tut Overview]]** - Complete HashMap tutorial with step-by-step guide
- **[[Step 1 - Basic HashMap|Step 1]]** - Building hash table foundations
- **[[Step 3 - String Keys|Step 3]]** - String key handling with pattern matching

### **String & Pattern Concepts**
- **[[../../zettelkasten/Day 09 - String Patterns|Day 09 - String Patterns]]** - String vs &str, UTF-8 handling
- **[[../../zettelkasten/Day 06 - Pattern Matching|Day 06 - Pattern Matching]]** - Match expressions and destructuring
- **[[../../zettelkasten/Error Handling Deep Dive|Error Handling Deep Dive]]** - Result types for regex operations

### **AoC Applications**
- **[[../../zettelkasten/AoC 2015 MOC|AoC 2015 MOC]]** - Day 5 (Nice Strings), Day 8 (String Escaping)
- **[[../../zettelkasten/AoC Patterns MOC|AoC Patterns MOC]]** - String algorithms and parsing techniques
- **[[../../zettelkasten/AoC Collection Problems|AoC Collection Problems]]** - When to use regex vs manual parsing

### **Parsing & Validation**
- **Text Parsing Patterns** - Line-by-line and regex-based parsing
- **Input Validation** - Using regex for data validation
- **String Manipulation** - Character extraction and transformation

### **Knowledge Hubs**
- **[[../../zettelkasten/Collections MOC|Collections MOC]]** - Data structure patterns
- **[[../../zettelkasten/Rust Concepts MOC|Rust Concepts MOC]]** - Core language features
- **[[../../missions/Mission5/README|Mission5 HashMap]]** - Main implementation using string keys

---

*Tags: #regex #pattern-matching #string-processing #validation #parsing #aoc #quick-reference #tutorial #mission5*

*Links: [[../../zettelkasten/zettel-index|zettel-index]] | [[README|Mission5_tut]] | [[../../zettelkasten/Day 09 - String Patterns|Day 09]] | [[../../zettelkasten/AoC Patterns MOC|AoC Patterns]]*