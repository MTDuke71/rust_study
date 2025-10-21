# 🤝 Contributing to Rust Study Workspace

Thank you for your interest in contributing! This workspace follows a **V-Cycle engineering methodology** with strict quality gates and traceability requirements.

---

## 📋 Quick Start for Contributors

### Prerequisites
1. **Rust toolchain** (stable) - Install from [rustup.rs](https://rustup.rs/)
2. **VS Code** (recommended) with extensions:
   - `rust-analyzer` (essential)
   - `CodeLLDB` (debugging)
3. **Git** configured with your identity:
   ```bash
   git config user.name "Your Name"
   git config user.email "your.email@example.com"
   ```

### First Steps
```bash
# Clone and setup
git clone https://github.com/MTDuke71/rust_study.git
cd rust_study

# Verify build passes
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings

# All should succeed before making changes
```

---

## 🎯 Contribution Types

### 1️⃣ Bug Fixes
- **Focus**: Fix broken functionality or incorrect behavior
- **Requirements**: Must include test demonstrating the bug
- **Checklist**: See [Verification Checklist](#verification-checklist)

### 2️⃣ New Missions
- **Focus**: Add new V-Cycle mission (Mission8+)
- **Requirements**: Full V-Cycle documentation (REQ-X → Design → Implementation → Tests)
- **Template**: See `missions/Mission5/README.md` for structure
- **Must Have**:
  - [ ] Requirements section (REQ-1 through REQ-N)
  - [ ] Design decisions documented
  - [ ] Unit tests named `req{N}_*`
  - [ ] Integration tests with real-world scenarios
  - [ ] Doctests in all public functions
  - [ ] Example programs in `examples/`
  - [ ] Companion tutorial in `tutorials/MissionX_tut/`

### 3️⃣ Tutorial Content
- **Focus**: Add or improve step-by-step learning materials
- **Requirements**: Must align with `MONTHLY_CALENDAR.md`
- **Template**: See `tutorials/Mission5_tut/README.md`
- **Must Have**:
  - [ ] Progressive steps (`step1_*.rs` through `stepN_*.rs`)
  - [ ] Maps to specific mission REQ-X
  - [ ] Runnable examples at each step
  - [ ] Clear learning objectives
  - [ ] Error anticipation section

### 4️⃣ Daily Study Content
- **Focus**: Add concept explanations with runnable examples
- **Requirements**: Must include "Complete Runnable Example"
- **Template**: See `.github/COMPLETE_RUNNABLE_EXAMPLE_TEMPLATE.md`
- **Must Have**:
  - [ ] Self-contained executable code
  - [ ] 4-7 progressive sections
  - [ ] Tested with `.\scripts\run_md.bat`
  - [ ] Zettelkasten links to related concepts

### 5️⃣ Documentation Improvements
- **Focus**: Enhance existing documentation
- **Standards**: Follow `.github/RUST_DOCUMENTATION_STANDARDS.md`
- **Must Have**:
  - [ ] All public items have `///` documentation
  - [ ] Examples section in every public function
  - [ ] Module-level `//!` documentation
  - [ ] Doctests pass (`cargo test --doc`)

---

## 🔄 Development Workflow

### Standard Process

1. **Check MONTHLY_CALENDAR.md** (if creating Mission + Tutorial)
   - Verify alignment with daily learning plan
   - Ensure timeline is realistic

2. **Create Feature Branch**
   ```bash
   git checkout -b feature/mission8-trees
   # OR
   git checkout -b fix/mission5-hash-collision
   ```

3. **Implement Changes**
   - Follow [Quick Decision Guide](#quick-decision-guide)
   - Write tests FIRST (test-driven development)
   - Reference REQ-X in code comments
   - Run `cargo clippy` frequently

4. **Verify Quality** (see [Verification Checklist](#verification-checklist))
   ```bash
   cargo test -p your_package
   cargo clippy -p your_package -- -D warnings
   cargo test --doc -p your_package
   cargo build --workspace
   ```

5. **Document Changes**
   - Update README.md with REQ-X if new feature
   - Add doctests to all public functions
   - Update Zettelkasten links if applicable

6. **Commit with Standard Format** (see [Commit Template](#commit-message-template))
   ```bash
   git add .
   git commit -m "[Category] Brief summary

   Detailed changes:
   - ✅ Change 1
   - ✅ Change 2
   
   Requirements: REQ-X
   Testing: All tests pass, zero clippy warnings"
   ```

7. **Push and Create PR**
   ```bash
   git push origin feature/mission8-trees
   # Create PR on GitHub with description
   ```

---

## ✅ Verification Checklist

**MUST PASS** before submitting PR:

### Code Quality
- [ ] `cargo test -p {package}` passes (all unit tests)
- [ ] `cargo clippy -p {package} -- -D warnings` clean (zero warnings mandatory)
- [ ] `cargo test --doc -p {package}` passes (all doctests execute)
- [ ] `cargo build --workspace` succeeds (no breaking changes)

### Documentation
- [ ] README.md updated with REQ-X statements if new feature
- [ ] All public functions have `///` documentation with Examples
- [ ] Module-level `//!` docs updated if API changed
- [ ] Doctests include error cases, not just happy path

### Testing
- [ ] Tests named `req{N}_*` for requirement traceability
- [ ] Edge cases covered (empty input, boundary conditions)
- [ ] Performance characteristics verified (O(1), O(n) claims)
- [ ] Integration tests pass with real-world data

### Project-Specific
- [ ] `MONTHLY_CALENDAR.md` checked if creating tutorial content
- [ ] Tutorial steps map to mission requirements (REQ-1 → step1_*, etc.)
- [ ] Daily study files include "Complete Runnable Example" section
- [ ] Zettelkasten links follow naming convention (`[[mission-X]]` not `[[MissionX]]`)
- [ ] All struct field renames reflected in ALL instantiations
- [ ] Unused fields prefixed with `_`, unused variables prefixed with `_`

### Common Gotchas
- [ ] Type annotations added for generic `::new()` methods (e.g., `HashSet<T>::new()`)
- [ ] Grid dimension calculations verified (format placeholders = arguments)
- [ ] Imports cleaned up (remove unused imports caught by clippy)
- [ ] Variables are `mut` only if actually modified
- [ ] AoC test data files paired with `.expected.csv` results

---

## 🧭 Quick Decision Guide

### Data Structure Choice
- Need FIFO? → `VecDeque` (Mission2)
- Need lookup? → `HashMap` (Mission5)
- Need order? → `BTreeMap` (Day12)
- Need uniqueness? → `HashSet` (Day11)
- Need LIFO? → `Vec` as stack (Mission1)
- Need graph? → `Vec<Vec<NodeId>>` adjacency list (Mission7)

### Documentation Style
- Public API function → `///` with Examples section
- Module/crate level → `//!` with Quick Start section
- Test function → Descriptive name only (no `///` needed)
- Implementation detail → `//` inline comment
- Requirement reference → `// REQ-X: description`

### Test Organization
- Unit test → `tests/` directory, named `req{N}_*`
- Integration → `tests/` directory, `*_integration.rs` or `*_checker_test.rs`
- Doctest → `/// # Examples` in function docs
- Tutorial → `examples/step{N}_*.rs` with progressive complexity

### File Locations
- Mission implementation → `missions/Mission{N}/src/lib.rs`
- Mission tests → `missions/Mission{N}/tests/`
- Tutorial steps → `tutorials/Mission{N}_tut/examples/step{N}_*.rs`
- Daily concepts → `daily_study/rust_learning_week{N}_notes/Day{N}.md`
- Rust book practice → `rust_book/Ch{N}/{topic}/`
- Real-world apps → `advanced_examples/{name}/`
- Knowledge notes → `zettelkasten/{topic}.md`

---

## 📝 Commit Message Template

Use this format for all commits:

```
[Category] Brief summary (50 characters max)

Detailed changes:
- ✅ Section/File 1: Specific change description
- ✅ Section/File 2: Specific change description  
- 🔧 Fixed: Issue that was resolved
- 📝 Updated: Documentation improvements
- ✨ Added: New features or functionality

Context:
- Why: Rationale for these changes
- Impact: What improved (performance, clarity, correctness)
- Related: Links to issues, requirements, or other commits

Requirements: REQ-X, REQ-Y (if applicable)
Testing: All tests pass, zero clippy warnings
```

**Category Options**:
- `[Mission]` - Core mission implementations
- `[Tutorial]` - Tutorial content and examples
- `[Daily Study]` - Daily study notes and examples
- `[Docs]` - Documentation improvements
- `[Tests]` - Test additions or fixes
- `[Fix]` - Bug fixes
- `[Refactor]` - Code restructuring without behavior change
- `[Chore]` - Build, tooling, or maintenance tasks

---

## 🔍 Code Review Process

### What Reviewers Check

1. **Requirement Traceability**
   - Every feature traces to REQ-X
   - Tests named `req{N}_*` verify requirements
   - README documents V-Cycle (Requirements → Design → Implementation → Verification)

2. **Quality Gates**
   - Zero clippy warnings
   - All tests pass
   - Doctests execute successfully
   - No breaking changes to workspace

3. **Documentation Standards**
   - Public APIs have `///` documentation
   - Examples section in every public function
   - Module-level `//!` documentation
   - Clear, runnable doctests

4. **Testing Coverage**
   - Unit tests for each requirement
   - Integration tests for real-world scenarios
   - Edge cases covered
   - Performance claims verified

5. **Project Conventions**
   - REQ-X references in code
   - Mission-Tutorial alignment
   - Zettelkasten link formatting
   - Naming conventions followed

### Addressing Review Feedback

1. **Respond to comments** - Engage in discussion
2. **Make requested changes** - Update code based on feedback
3. **Push updates** - Use `git commit --amend` or new commits
4. **Re-request review** - Once changes are complete
5. **Be patient** - Quality reviews take time

---

## 🐛 Reporting Issues

### Bug Reports

Include:
- **Description**: What's broken?
- **Steps to Reproduce**: Exact commands/steps
- **Expected Behavior**: What should happen?
- **Actual Behavior**: What actually happens?
- **Environment**: OS, Rust version (`rustc --version`)
- **Error Messages**: Full error output

### Enhancement Requests

Include:
- **Problem Statement**: What need does this address?
- **Proposed Solution**: How would it work?
- **Alternatives Considered**: Other approaches?
- **REQ-X Mapping**: Which requirements would this fulfill?
- **Learning Value**: How does this enhance learning?

---

## 🔗 Additional Resources

- **Full Developer Guide**: [[copilot-instructions.md]]
- **Error Troubleshooting**: [[COMMON_ERRORS.md]]
- **Documentation Standards**: [[RUST_DOCUMENTATION_STANDARDS.md]]
- **Test Documentation**: [[RUST_TEST_DOCUMENTATION_STANDARDS.md]]
- **Runnable Example Template**: [[COMPLETE_RUNNABLE_EXAMPLE_TEMPLATE.md]]
- **Learning Calendar**: [[../MONTHLY_CALENDAR.md]]

---

## 💡 Getting Help

- **Discord/Slack**: [Community channel]
- **GitHub Discussions**: Ask questions, share ideas
- **Code Review**: Request feedback on draft PRs
- **Documentation**: Check `.github/` directory for guides

---

## 🎯 Philosophy & Principles

This workspace follows:

1. **V-Cycle Engineering** - Requirements → Design → Implementation → Verification → Validation
2. **Test-Driven Development** - Write tests before implementation
3. **Zero Warnings Policy** - `cargo clippy -- -D warnings` must pass
4. **Complete Traceability** - Every feature maps to REQ-X
5. **Educational First** - All code teaches Rust concepts
6. **Real-World Validation** - AoC problems verify practical utility
7. **Clean Code Principles** - Meaningful names, single responsibility, DRY
8. **Professional Standards** - Industry-grade quality for all contributions

---

## 🙏 Thank You!

Your contributions help make this workspace a better learning resource for the Rust community. Every improvement, no matter how small, is valued and appreciated!

**Questions?** Don't hesitate to ask in GitHub Discussions or open a draft PR for feedback.

---

*Last Updated: October 12, 2025*
