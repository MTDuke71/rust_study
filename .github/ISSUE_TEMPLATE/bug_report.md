---
name: 🐛 Bug Report
about: Report a bug or unexpected behavior in the codebase
title: '[BUG] '
labels: ['bug', 'needs-triage']
assignees: ''
---

## 🐛 Bug Description

**Clear, concise description of the bug:**
<!-- What's broken? What unexpected behavior are you seeing? -->

---

## 📍 Location

**Which component is affected?**
- [ ] Mission (specify: Mission1/Mission2/etc.)
- [ ] Tutorial (specify: Mission1_tut/Mission2_tut/etc.)
- [ ] Daily Study (specify: Day/Week)
- [ ] Rust Book exercises (specify: Chapter)
- [ ] Advanced examples (specify: which)
- [ ] Build/Tooling
- [ ] Documentation
- [ ] Other: _____________

**Specific file(s):**
<!-- List the file paths where the issue occurs -->
- `path/to/file.rs`

**Function/Module:**
<!-- Specific function, struct, or module name if applicable -->

---

## 🔁 Steps to Reproduce

**Exact commands/steps to trigger the bug:**

1. <!-- First step -->
2. <!-- Second step -->
3. <!-- Third step -->

**Minimal code example (if applicable):**
```rust
// Minimal reproduction code
fn main() {
    // Your code here
}
```

**Command that fails:**
```bash
cargo test -p mission5
# OR
cargo run -p mission5_tut --example step3
# OR
.\scripts\run_md.bat daily_study\DayXX.md
```

---

## ✅ Expected Behavior

**What should happen?**
<!-- Describe the correct/expected behavior -->

**Why is this the expected behavior?**
<!-- Reference REQ-X, documentation, or standard Rust behavior -->

---

## ❌ Actual Behavior

**What actually happens?**
<!-- Describe the incorrect/unexpected behavior -->

**Error message (if any):**
```
Paste complete error message here, including:
- Error code (e.g., E0282, E0308)
- File and line number
- Full error text
- Any compiler suggestions
```

**Unexpected output:**
```
Paste actual output vs expected output
```

---

## 🖥️ Environment

**Operating System:**
<!-- e.g., Windows 11, macOS 14, Ubuntu 22.04 -->

**Rust Version:**
```bash
# Run: rustc --version
rustc 1.XX.X (XXXXXXX YYYY-MM-DD)
```

**Cargo Version:**
```bash
# Run: cargo --version
cargo 1.XX.X
```

**VS Code Extensions (if relevant):**
- rust-analyzer: vX.X.X
- CodeLLDB: vX.X.X

**Workspace State:**
```bash
# Run: git status
On branch: main
Modified files: [list any local modifications]
```

---

## 🔍 Investigation Done

**What have you tried?**
- [ ] Read error message and compiler suggestions
- [ ] Checked `.github/COMMON_ERRORS.md` for similar issues
- [ ] Ran `cargo clean && cargo build`
- [ ] Ran `cargo clippy` to check for warnings
- [ ] Verified with minimal reproduction case
- [ ] Searched existing issues

**Findings:**
<!-- What did you discover while investigating? -->

**Related errors/warnings:**
<!-- Any other errors that appeared alongside this bug? -->

---

## 📊 Impact Assessment

**Severity:**
- [ ] 🔴 Critical - Blocks all work (cannot compile workspace)
- [ ] 🟠 High - Blocks specific mission/tutorial
- [ ] 🟡 Medium - Workaround exists but inconvenient
- [ ] 🟢 Low - Minor issue, easy workaround

**Scope:**
- [ ] Affects entire workspace
- [ ] Affects specific mission/tutorial only
- [ ] Affects only specific test/example
- [ ] Documentation/clarity issue only

**Work Blocked:**
<!-- What tasks cannot be completed due to this bug? -->

---

## 💡 Possible Solution (Optional)

**Suspected root cause:**
<!-- Your theory about why this is happening -->

**Potential fix:**
<!-- Suggestion for how to fix it -->

**Alternative workarounds:**
<!-- Temporary solutions that work for now -->

---

## 📸 Screenshots (Optional)

**Visual evidence:**
<!-- Paste screenshots showing the issue, especially for:
- Unexpected output
- UI issues
- Complex error messages
-->

---

## 🔗 Additional Context

**REQ-X Relationship:**
<!-- Does this violate a specific requirement? Which one? -->

**Related Issues/PRs:**
<!-- Link to related issues or PRs -->
- Related to #XX
- Introduced by #YY

**AoC Problem (if applicable):**
<!-- If this affects AoC validation, which problem/year/day? -->

**Test Failures:**
<!-- If tests are failing, paste test output -->
```bash
running X tests
test req1_feature ... FAILED
test req2_feature ... ok

failures:
---- req1_feature stdout ----
thread 'req1_feature' panicked at 'assertion failed'
```

---

## 📚 References

**Documentation consulted:**
- [ ] `.github/copilot-instructions.md`
- [ ] `.github/COMMON_ERRORS.md`
- [ ] `.github/CONTRIBUTING.md`
- [ ] Mission/Tutorial README.md
- [ ] Rust documentation/error index

**External resources:**
<!-- Links to Stack Overflow, Rust forums, documentation that might be relevant -->

---

## ✅ Pre-Submission Checklist

Before submitting, I have:
- [ ] Verified this is reproducible (not a one-time fluke)
- [ ] Checked if this is already reported in existing issues
- [ ] Provided complete reproduction steps
- [ ] Included all relevant error messages and environment info
- [ ] Checked `.github/COMMON_ERRORS.md` for known solutions
- [ ] Tested with latest code from main branch

---

**Additional Notes:**
<!-- Any other information that might be helpful -->