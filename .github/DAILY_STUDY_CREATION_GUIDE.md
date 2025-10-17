# Creating New Weeks of Daily Study Notes - Guidelines & Documentation

## 📋 **Primary Guidelines Documents to Use**

When creating a new week of daily_study notes, you should consult these documents in this order:

### **1. COMPLETE_RUNNABLE_EXAMPLE_TEMPLATE.md** ⭐ **[PRIMARY]**
**Location**: `.github/COMPLETE_RUNNABLE_EXAMPLE_TEMPLATE.md`

**Why this is essential**:
- Contains the **mandatory template** for every Day file
- Specifies the "Complete Runnable Example" section that MUST be in every day
- Shows the standard 4-7 section structure for progressive learning
- Includes helper functions and best practices
- Every single day file MUST follow this format

**What it covers**:
- Self-contained code examples that work in Rust Playground
- How to run examples via markdown runner (`.\scripts\run_md.bat`)
- Section structure guidelines (Basic → Advanced → Real-world)
- Testing and validation procedures

---

### **2. tutorial.engineer.md** ⭐ **[PEDAGOGICAL FOUNDATION]**
**Location**: `.github/tutorial.engineer.md`

**Why this is essential**:
- Defines the pedagogical principles for educational content
- Specifies how to structure learning progressions
- Shows error anticipation and common mistake prevention
- Aligns with cognitive science research on learning

**What it covers**:
- Learning objectives and prerequisites
- Progressive disclosure principles
- Exercise design (worked → faded → bare problem)
- Multiple learning styles support
- Quality checklist for tutorial content

---

### **3. MONTHLY_CALENDAR.md** ⭐ **[COORDINATION]**
**Location**: `./MONTHLY_CALENDAR.md`

**Why this is essential**:
- Specifies the daily learning routine (45-75 minute structure)
- Shows mission-tutorial alignment requirements
- Defines daily study focus areas by week
- Provides real calendar integration with mission dates

**What it covers**:
- Evidence-based learning protocol (retrieval practice, spaced repetition)
- Track alignment (daily study ↔ mission tutorials)
- Daily mission focus mappings
- Integration checkpoints

---

### **4. daily_study/README.md** ⭐ **[DIRECTORY GUIDE]**
**Location**: `./daily_study/README.md`

**Why this is essential**:
- Shows the structure of existing week files
- Provides navigation links to zettelkasten MOCs
- Demonstrates the linking pattern for daily notes
- Shows how weeks are organized

**What it covers**:
- Weekly directory structure
- Core topics by week
- Mission integration examples
- Quick start instructions

---

### **5. copilot-instructions.md** ⭐ **[CONTEXT & STANDARDS]**
**Location**: `.github/copilot-instructions.md`

**Why this is essential**:
- Establishes quality gates and verification standards
- Specifies V-Cycle mission alignment
- Shows test naming conventions
- Defines zettelkasten tagging and linking

**What it covers**:
- REQ-X traceability for daily content
- Zero-warnings enforcement
- Complete runnable example requirements
- Zettelkasten link naming conventions

---

### **6. RUST_DOCUMENTATION_STANDARDS.md** ⭐ **[CODE DOCUMENTATION]**
**Location**: `.github/RUST_DOCUMENTATION_STANDARDS.md`

**Why this is essential**:
- Shows how to document code examples properly
- Specifies rustdoc format for examples
- Provides documentation checklists

**What it covers**:
- `///` vs `//!` documentation patterns
- Example section requirements
- Best practices for example code

---

## 🎯 **Standard Week Structure for Daily Study & Mission Tutorials**

**⭐ IMPORTANT**: This incremental daily creation pattern applies to:
- ✅ **Daily Study weeks** (DayX.md + DayX.rs files)
- ✅ **Mission tutorials** (stepX_*.rs files + daily exercise solutions)
- ❌ **NOT for Rust Book chapters** (those are created all-at-once, see note below)

### **Note on Rust Book Chapters**
Rust Book chapters continue to be created **1 chapter at a time in full**:
- Create all files for Ch10 (Ch10/README.md + generics/ + traits/ + lifetimes/) at once
- Then move to Ch11 only when Ch10 is complete
- No incremental daily creation for Rust Book chapters

### **When creating a new week or mission tutorial (Week 6, Mission 9 tutorial, etc.), follow this pattern:**

### **Files to Create**
```
daily_study/
├── rust_learning_week6_notes/
│   ├── README.md                 # Week overview with zettelkasten links
│   ├── Cargo.toml                # Workspace configuration (optional bin examples)
│   ├── TODO.md                   # ⭐ NEW: Created on Day 1 with week plan
│   ├── Day29.md                  # Monday's lesson (conceptual content)
│   ├── Day29.rs                  # Monday's runnable example code
│   ├── Day30.md                  # ⭐ Created on Day 2
│   ├── Day30.rs                  # ⭐ Created on Day 2
│   ├── Day31.md                  # ⭐ Created on Day 3
│   ├── Day31.rs                  # ⭐ Created on Day 3
│   ├── Day32.md                  # ⭐ Created on Day 4
│   ├── Day32.rs                  # ⭐ Created on Day 4
│   ├── Day33.md                  # ⭐ Created on Day 5
│   ├── Day33.rs                  # ⭐ Created on Day 5
│   ├── Day34.md                  # ⭐ Created on Day 6
│   ├── Day34.rs                  # ⭐ Created on Day 6
│   ├── Day35.md                  # ⭐ Created on Day 7
│   ├── Day35.rs                  # ⭐ Created on Day 7
│   └── examples/                 # Optional: cargo examples/ directory
```

### **⭐ IMPORTANT: Incremental Daily Creation Workflow**

**Starting with Week 5**, the workflow has changed from creating all files on Day 1 to **daily incremental creation**:

#### **Day 1 (Week Start)**
- Create ONLY: `Day29.md` + `Day29.rs` (today's content)
- Create ONLY: `TODO.md` (week plan/outline)
- ❌ DO NOT create Days 30-35 files yet
- Purpose: Start the week focused on today's learning

#### **Day 2-7 (Each Following Day)**
- Create ONLY that day's files: `DayX.md` + `DayX.rs`
- Follow the TODO.md plan created on Day 1
- Update progress in TODO.md as you go
- ❌ DO NOT create multiple days at once

### **Purpose of TODO.md**

The `TODO.md` file created on Day 1 serves as the **week planning document**:

```markdown
# Week 6 Daily Study Plan

## 📋 Learning Objectives
- [Week 5 overall learning goal 1]
- [Week 5 overall learning goal 2]
- [Week 5 overall learning goal 3]

## 📅 Daily Breakdown

- [x] **Day 29 (Mon)**: [Topic] - COMPLETE
  - Created: Day29.md + Day29.rs
  - Coverage: [what was covered]
  - Learning: [key takeaway]

- [ ] **Day 30 (Tue)**: [Topic]
  - Planned content: [outline of what will be taught]
  - Related REQ: [mission requirements]

- [ ] **Day 31 (Wed)**: [Topic]
  - Planned content: [outline]
  - Related REQ: [mission requirements]

- [ ] **Day 32 (Thu)**: [Topic]
  - Planned content: [outline]
  - Related REQ: [mission requirements]

- [ ] **Day 33 (Fri)**: [Topic]
  - Planned content: [outline]
  - Related REQ: [mission requirements]

- [ ] **Day 34 (Sat)**: [Topic]
  - Planned content: [outline]
  - Related REQ: [mission requirements]

- [ ] **Day 35 (Sun)**: [Topic - Integration/Review]
  - Planned content: [outline]
  - Related REQ: [mission requirements]

## 📚 Integration Points
- **Mission Focus**: [which mission is this week coordinated with]
- **Rust Book Chapter**: [which chapter/section]
- **Theme**: [overall theme of the week]

## 🔗 Zettelkasten Links
- Related concepts: [[concept1]], [[concept2]]
- Previous week: [[daily-study/Week5 Overview]]
- Next week: [[daily-study/Week7 Overview]]
```

### **Key Change**: Each day now has **TWO files**:
1. **`DayX.md`** - Contains conceptual content, explanations, and links
2. **`DayX.rs`** - Contains the runnable example code (standalone, executable file)

### **Each DayX.md File Should Include**
(Following `COMPLETE_RUNNABLE_EXAMPLE_TEMPLATE.md`):

```markdown
# Day [X] · [Topic Title]

**Learning Focus**: [One sentence summary of key learning objective]

---

## 🎯 Learning Objectives

By the end of this day, you should understand:
- [Objective 1]
- [Objective 2]
- [Objective 3]

---

## 📚 Core Concepts

### **[Concept 1 Title]**
- Explanation of concept
- Code snippets showing usage
- Important traits or patterns

### **[Concept 2 Title]**
- Explanation of concept
- Code snippets showing usage

## Essential Trait Bounds / Patterns / APIs
- [Key trait requirements]
- [Pattern usage guidelines]
- [API best practices]

## Advanced Patterns
- [More sophisticated usage]
- [Real-world examples from libraries]

## Real-World Applications
- [AoC-style problems using this concept]
- [Practical use cases in production code]

## 🔗 Zettelkasten Connections
- **Implementation**: [[relevant-mission]]
- **Tutorial**: [[MissionX_tut Overview]]
- **Related Concepts**: [[concept-name]]
- **Next**: [[daily-study/Day[X+1]]]
- **Previous**: [[daily-study/Day[X-1]]]

*Tags: #daily-study #week6 #[topic-tag] #[related-mission]*
```

**Note**: The actual runnable code is in the companion `DayX.rs` file (see below).

---

## 📄 **Each DayX.rs File Should Include**

The companion `.rs` file contains all executable code, following the Complete Runnable Example format:

```rust
// Day [X] - [Topic Title]
// Runnable examples demonstrating [concept]

use std::collections::HashMap;  // Use actual imports needed
// ... other imports

// Example 1: Basic Concept
// ========================
#[derive(Debug)]
struct Example1 {
    field: String,
}

impl Example1 {
    fn new() -> Self {
        Example1 { field: String::new() }
    }
}

// Example 2: Key Feature
// =====================
fn demonstrate_feature() {
    // Implementation
}

// Example 3: Real-World Application
// =================================
fn solve_aoc_style_problem() {
    // AoC-style example
}

// Example 4+: Additional Patterns
// ===============================
// ... more examples

fn main() {
    println!("=== Day [X]: [Topic Title] ===\n");
    
    println!("1. Basic Concept:");
    // Call Example1 demonstrations
    
    println!("\n2. Key Features:");
    demonstrate_feature();
    
    println!("\n3. Real-World Application:");
    solve_aoc_style_problem();
    
    println!("\n4. Advanced Patterns:");
    // Additional examples
}
```

### **🛠️ How to Run the .rs File:**

1. **Online**: Copy entire file to [Rust Playground](https://play.rust-lang.org/)
2. **Local file**: Save as `day[X]_example.rs` and run:
   ```bash
   rustc day[X]_example.rs && ./day[X]_example.exe
   ```
3. **In this workspace** (if Cargo.toml configured):
   ```bash
   cargo run --bin day[X]
   ```

---

## ⚙️ **Quality Checklist for New Week**

Before marking a week complete, verify:

- [ ] **File Structure**:
  - [ ] Each day has both `DayX.md` AND `DayX.rs` files
  - [ ] File names follow exact pattern: `Day29.md`, `Day29.rs`, etc.
  - [ ] Week README.md exists with navigation links to all days
  - [ ] Cargo.toml exists in week directory (if using cargo bin targets)

- [ ] **DayX.md File Compliance**:
  - [ ] Contains "Learning Focus" statement
  - [ ] Includes "Learning Objectives" section (3-5 objectives)
  - [ ] Contains "Core Concepts" with explanations
  - [ ] Includes trait bounds and pattern information
  - [ ] Advanced patterns section included
  - [ ] Real-world applications explained
  - [ ] Zettelkasten connections with proper links at bottom
  - [ ] Tags line at bottom with 4-6 appropriate tags

- [ ] **DayX.rs File Compliance**:
  - [ ] File compiles without errors: `rustc DayX.rs`
  - [ ] File runs without panics: `./DayX.exe` (or `./DayX` on Unix)
  - [ ] Contains multiple examples (4-7 sections)
  - [ ] Examples are self-contained (all helper functions included)
  - [ ] Output is educational and clearly labeled
  - [ ] Can be copied directly to Rust Playground and run
  - [ ] No clippy warnings: `rustc --edition 2021 DayX.rs`

- [ ] **Pedagogical Standards** (from tutorial.engineer.md):
  - [ ] Progressive disclosure (basic → advanced → real-world)
  - [ ] Clear learning objectives stated
  - [ ] Error anticipation included
  - [ ] Multiple explanations for complex concepts
  - [ ] Worked → Faded → Bare problem structure visible

- [ ] **MONTHLY_CALENDAR.md alignment**:
  - [ ] Each day corresponds to calendar entry
  - [ ] Mission tutorial activities specified
  - [ ] Daily focus goals clearly stated
  - [ ] Integration with V-Cycle missions coordinated

- [ ] **Zettelkasten Integration**:
  - [ ] Each day file has zettelkasten links (Previous/Next/Related)
  - [ ] Links follow naming convention: `[[daily-study/DayXX]]` or `[[ds-dayXX]]`
  - [ ] Links properly created in markdown: `[[file-name]]`
  - [ ] Cross-references to mission tutorials added
  - [ ] Tags added at bottom: `*Tags: #daily-study #week6 #topic #cross-track*`

- [ ] **Rust Documentation Standards**:
  - [ ] Code examples compile and run
  - [ ] Inline comments explain concepts
  - [ ] Real-world data used in examples
  - [ ] All doctests/examples are validated

- [ ] **Quality Gates**:
  - [ ] All examples run without errors
  - [ ] No clippy warnings in example code
  - [ ] Links are valid (can follow them)
  - [ ] Markdown renders correctly
  - [ ] File names follow pattern: `Day29.md`, `Day30.md`, etc.
  - [ ] Both `.md` AND `.rs` files exist for each day

---

## 📖 **Summary: Two-File Structure for Each Day**

**Starting with Week 5**, each day has been split into two complementary files:

| File | Purpose | Contains |
|------|---------|----------|
| **DayX.md** | **Educational Content** | Concepts, explanations, trait bounds, real-world applications, zettelkasten links, tags |
| **DayX.rs** | **Executable Code** | Self-contained runnable examples, helper functions, main() with organized sections |

### **Why Two Files?**

1. **Separation of Concerns**: Theory (`.md`) vs. Practice (`.rs`)
2. **Better User Experience**: Readers can focus on either understanding or running code
3. **Easier Code Execution**: `.rs` file can be directly copied to Rust Playground
4. **Cleaner Markdown**: No large code blocks bloating the `.md` file
5. **Version Control**: Clear distinction between documentation and implementation

### **Workflow for Creating a New Week**

1. **Create the `.md` file first**:
   - Write all conceptual content
   - Explain the topic thoroughly
   - Add links and tags
   - Reference the `.rs` file for "running the code"

2. **Create the `.rs` file second**:
   - Implement the examples from the `.md` file
   - Test that it compiles and runs cleanly
   - Ensure it can be copied to Rust Playground
   - Make output educational and clearly labeled

3. **Link them together**:
   - In `.md` file, mention: "See Day[X].rs for runnable examples"
   - In `.rs` file comment header, reference the `.md` file for concepts
   - Both files use the same "Day[X]" naming pattern

## 🚀 **About "Claude Skills" Integration**

### **Current Status**
There is **NO mention of "Claude Skills"** in the current repository documentation. The repository currently uses:

1. **Copilot Instructions** (`.github/copilot-instructions.md`) - Agent guidelines
2. **Tutorial Engineering** (`tutorial.engineer.md`) - Pedagogical framework
3. **Learning Templates** - Standard content structure

### **Should We Integrate "Claude Skills"?**

#### **Possible Interpretations of "Claude Skills":**

1. **If it refers to Anthropic's Claude Model Capabilities**:
   - Not currently integrated
   - Would need to be added to `.github/` as documentation
   - Could specify Claude-specific optimization patterns
   - Example: "Claude should use semantic_search for concept discovery"

2. **If it refers to a Training Framework**:
   - Could complement existing pedagogical framework
   - Might specify learning progression pathways
   - Could define competency levels

3. **If it refers to Custom Instructions for LLMs**:
   - Could be added to `.github/claude-skills.md`
   - Would specify how to leverage LLM strengths (pattern synthesis, connection-making)
   - Could optimize tutorial generation

### **My Recommendation**

✅ **YES, we should integrate Claude Skills documentation IF:**
- We want to explicitly optimize tutorial generation for Claude model strengths
- We want to document specific prompt patterns that work well
- We want AI-assisted content generation to be consistent and high-quality

📋 **Suggested file structure if adding Claude Skills:**
```
.github/
├── copilot-instructions.md       # Existing agent instructions
├── tutorial.engineer.md          # Existing pedagogical framework
├── claude-skills.md              # NEW: Claude-specific capabilities
└── COMPLETE_RUNNABLE_EXAMPLE_TEMPLATE.md
```

---

## 📋 **Recommended File to Create for Claude Skills**

If you want to proceed, I would suggest creating:

**`.github/claude-skills.md`**

With sections like:
- Claude strengths for tutorial generation
- Prompt patterns for concept explanation
- Connection-making between complex topics
- Code synthesis best practices
- Learning progression optimization
- Integration with existing pedagogical framework

Would you like me to:
1. **Create a `claude-skills.md` file** with framework for enhancing tutorial generation?
2. **Add references to Claude Skills** throughout the existing documentation?
3. **Proceed with creating Week 6 daily study notes** using only current guidelines?
4. **Something else specific** you have in mind?

---

## 🔗 **Quick Navigation: All Relevant Documents**

| Document | Purpose | Usage |
|----------|---------|-------|
| `COMPLETE_RUNNABLE_EXAMPLE_TEMPLATE.md` | Example format standard | Every day file |
| `tutorial.engineer.md` | Pedagogical framework | Structure & progression |
| `MONTHLY_CALENDAR.md` | Scheduling & alignment | Week planning |
| `daily_study/README.md` | Directory guide | Navigation |
| `copilot-instructions.md` | Agent guidelines | Quality standards |
| `RUST_DOCUMENTATION_STANDARDS.md` | Code docs | Example quality |
| `RUST_TEST_DOCUMENTATION_STANDARDS.md` | Test docs | Test examples |
| `copilot-3-track-system.md` | Learning system | System architecture |
| `zetkelkasten/Daily Study MOC.md` | Knowledge graph | Concept connections |

---

**Created**: October 17, 2025  
**Purpose**: Guidelines for creating new weeks of daily study notes  
**Next Step**: Clarify Claude Skills integration direction
