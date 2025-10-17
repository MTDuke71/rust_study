# ⚡ Quick Reference: Daily Study & Mission Tutorial Creation Workflow

**⭐ APPLIES TO:**
- ✅ Daily Study Weeks (DayX.md + DayX.rs)
- ✅ Mission Tutorials (stepX_*.rs + exercises)
- ❌ NOT Rust Book chapters (created all-at-once instead)

---

## 📅 Creation Workflow - Daily Incremental (NOT All-at-Once)

**⭐ KEY CHANGE**: Files are created **daily**, not all on Day 1!

### **Day 1 (Week Start)**
```
Create:
  ✅ Day29.md + Day29.rs    (Today's content)
  ✅ TODO.md                 (Week plan outline)

Do NOT create:
  ❌ Day30-35 files (yet!)
```

### **Day 2-7 (Each Subsequent Day)**
```
Create:
  ✅ DayX.md + DayX.rs (That day's content only)
  ✅ Update TODO.md with progress

Do NOT create:
  ❌ Multiple days at once
  ❌ Days ahead of time
```

### **Purpose of TODO.md**
- Week planning document created on Day 1
- Outlines learning objectives and daily breakdown
- Provides structure and coordination with missions
- Updated daily as progress is made
- Follows same format as mission TODO.md guidelines

---

## �📂 File Organization

```
daily_study/
└── rust_learning_week6_notes/          # New week folder
    ├── Cargo.toml                       # Workspace config (optional)
    ├── README.md                        # Week overview & navigation
    │
    ├── Day29.md                         # 📖 THEORY FILE
    ├── Day29.rs                         # 💻 CODE FILE
    │
    ├── Day30.md                         # 📖 THEORY FILE
    ├── Day30.rs                         # 💻 CODE FILE
    │
    ├── [... continue pattern ...]
    │
    ├── Day35.md                         # 📖 THEORY FILE
    ├── Day35.rs                         # 💻 CODE FILE
    │
    └── examples/                        # Optional: cargo examples dir
```

---

## 📖 What Goes in `DayX.md` (THEORY FILE)

**✅ DO Include:**
- Learning Focus (1 sentence)
- Learning Objectives (3-5 bullet points)
- Core Concepts (explanations)
- Trait bounds and patterns
- Advanced patterns (conceptual)
- Real-world applications (conceptual)
- Zettelkasten connections (links)
- Tags at bottom

**❌ DON'T Include:**
- Large code blocks (put in `.rs` file)
- Complete runnable examples (put in `.rs` file)
- Executable code (put in `.rs` file)

---

## 💻 What Goes in `DayX.rs` (CODE FILE)

**✅ DO Include:**
- File header comment with Day and topic
- Multiple examples (4-7 sections)
- Helper functions and structs
- Main function with organized sections
- Clear println! labels
- Comments explaining concepts
- Self-contained code (can copy to Playground)

**❌ DON'T Include:**
- Theory and explanations (put in `.md` file)
- Markdown syntax
- Educational narrative (just show code)

---

## 🔗 How They Work Together

### **For the Learner**

**Path 1: Understanding First**
1. Read `Day29.md` for concepts
2. Read `Day29.rs` to see code examples
3. Run `Day29.rs` or copy to Playground

**Path 2: Learning by Doing**
1. See reference to `Day29.rs` in `Day29.md`
2. Copy `Day29.rs` to Rust Playground
3. Run it to see output
4. Go back to `Day29.md` to understand concepts

### **For the Maintainer**

- **Single File Pair**: Easy to find related content
- **Clear Separation**: Theory updates don't touch code
- **Version Control**: Distinct changes per file type
- **Reusability**: `.rs` file can be shared as standalone

---

## 📋 Checklist Before Committing a Week

### File Existence
- [ ] All 7 days have `.md` files (Day29-Day35)
- [ ] All 7 days have `.rs` files (Day29-Day35)
- [ ] README.md exists in week folder
- [ ] Cargo.toml exists (optional but recommended)

### DayX.md Files
- [ ] Contains "Learning Focus" (1 line)
- [ ] Contains "Learning Objectives" (3-5 items)
- [ ] Contains "Core Concepts" section
- [ ] Contains trait/pattern information
- [ ] Contains "Advanced Patterns" section
- [ ] Contains "Real-World Applications" section
- [ ] Contains Zettelkasten connections (Previous/Next/Related)
- [ ] Contains tags line at bottom
- [ ] NO large code blocks (should be in .rs file)
- [ ] References to `.rs` file for "running the code"

### DayX.rs Files
- [ ] File compiles: `rustc DayX.rs` ✅
- [ ] File runs: `./DayX.exe` (no panics) ✅
- [ ] Output is clearly labeled with sections
- [ ] Contains 4-7 organized examples
- [ ] All helper functions defined (self-contained)
- [ ] Can be copied to Rust Playground and run
- [ ] No clippy warnings: `rustc --edition 2021 DayX.rs` ✅
- [ ] File header has Day and topic comment

### Organization
- [ ] File names exactly match (Day29.md ↔ Day29.rs)
- [ ] README.md links to all days
- [ ] Tags consistent across week files
- [ ] Zettelkasten links follow convention: `[[daily-study/DayXX]]`

---

## 🚀 Running Examples

### Option 1: Rust Playground (Easiest)
```
1. Open https://play.rust-lang.org/
2. Copy entire Day29.rs content
3. Click "Run"
4. See output
```

### Option 2: Local Compilation
```bash
cd daily_study/rust_learning_week6_notes/
rustc Day29.rs
./Day29.exe          # Windows
./Day29              # Unix/Mac
```

### Option 3: Cargo (If Cargo.toml Configured)
```bash
cd daily_study/rust_learning_week6_notes/
cargo run --bin day29
```

---

## 📚 Cross-Reference Pattern

In **DayX.md**:
```markdown
See **Day[X].rs** for executable code examples
and run with Rust Playground or `rustc Day[X].rs`
```

In **DayX.rs** (file header):
```rust
// Day [X] - [Topic Title]
// Runnable examples for Day [X] from rust_learning_week6_notes
// See Day[X].md for detailed concept explanations
```

---

## 🎯 Why This Structure Works

| Aspect | Benefit |
|--------|---------|
| **Separation** | Theory stays in .md, code stays in .rs |
| **Portability** | .rs file works standalone in Playground |
| **Discovery** | Easy to find either content type |
| **Maintenance** | Changes to theory don't affect code versioning |
| **Reuse** | .rs files can be shared as code examples |
| **Testing** | .rs files can be run to verify correctness |

---

## 🔍 Week 5 Examples (Reference)

```
rust_learning_week5_notes/
├── Day29.md (493 lines)    ← Concepts about Custom Errors
├── Day29.rs (339 lines)    ← Runnable custom error examples
├── Day30.md (487 lines)    ← Concepts about Error Chaining
├── Day30.rs (302 lines)    ← Runnable chaining examples
└── ... similar pattern for Days 31-35
```

**Pattern confirmed**: Theory in .md, executable code in .rs

---

## 📞 Questions?

Refer to:
- **Full guidelines**: `.github/DAILY_STUDY_CREATION_GUIDE.md`
- **Week 5 examples**: `daily_study/rust_learning_week5_notes/`
- **Pedagogical framework**: `.github/tutorial.engineer.md`
- **Rust code standards**: `.github/RUST_DOCUMENTATION_STANDARDS.md`

---

**Last Updated**: October 17, 2025  
**Status**: Ready for Week 6+ creation
