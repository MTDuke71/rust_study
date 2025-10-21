# Pull Request

## 📋 Description

**Brief Summary:**
<!-- Provide a clear, concise description of the changes in this PR -->

**Problem/Motivation:**
<!-- What problem does this solve? Why is this change needed? -->

**Solution Approach:**
<!-- How does this PR address the problem? Key decisions made? -->

---

## 🎯 Type of Change

Select all that apply:

- [ ] 🐛 Bug fix (non-breaking change which fixes an issue)
- [ ] ✨ New Mission (Mission8+)
- [ ] 📚 Tutorial content (MissionX_tut)
- [ ] 📝 Daily study content (DayXX.md)
- [ ] 📖 Documentation improvements
- [ ] ♻️ Refactoring (no functional changes)
- [ ] 🧪 Test additions/improvements
- [ ] 🔧 Build/tooling changes

---

## 📊 REQ-X Traceability

**Requirements Fulfilled:**
<!-- List all REQ-X this PR addresses (e.g., REQ-1, REQ-2) -->
- REQ-X: [Description]

**Mission/Tutorial Alignment:**
<!-- If applicable, document alignment with MONTHLY_CALENDAR.md -->
- [ ] Checked `MONTHLY_CALENDAR.md` for alignment
- [ ] Tutorial steps map to mission requirements
- [ ] Timeline is realistic (30-45 min daily budget)

---

## ✅ Verification Checklist

### Code Quality (MANDATORY)
- [ ] `cargo test -p {package}` passes (all unit tests)
- [ ] `cargo clippy -p {package} -- -D warnings` clean (zero warnings)
- [ ] `cargo test --doc -p {package}` passes (all doctests execute)
- [ ] `cargo build --workspace` succeeds (no breaking changes)

### Documentation
- [ ] README.md updated with REQ-X statements if new feature
- [ ] All public functions have `///` documentation with Examples section
- [ ] Module-level `//!` docs updated if API changed
- [ ] Doctests include error cases, not just happy path

### Testing
- [ ] Tests named `req{N}_*` for requirement traceability
- [ ] Edge cases covered (empty input, boundary conditions)
- [ ] Performance characteristics verified (O(1), O(n) claims documented)
- [ ] Integration tests pass with real-world data

### Project-Specific
- [ ] Tutorial steps map to mission requirements (REQ-1 → step1_*, etc.)
- [ ] Daily study files include "Complete Runnable Example" section
- [ ] Zettelkasten links follow naming convention (`[[mission-X]]`, not `[[MissionX]]`)
- [ ] All struct field renames reflected in ALL instantiations
- [ ] Unused fields prefixed with `_`, unused variables prefixed with `_`

### Common Gotchas
- [ ] Type annotations added for generic `::new()` methods (e.g., `HashSet<T>::new()`)
- [ ] Imports cleaned up (no unused imports)
- [ ] Variables are `mut` only if actually modified
- [ ] Format string placeholders match argument count
- [ ] AoC test data files paired with `.expected.csv` results (if applicable)

---

## 🧪 Testing Evidence

**Commands Run:**
```bash
# Copy the commands you ran to verify
cargo test -p {package}
cargo clippy -p {package} -- -D warnings
cargo test --doc -p {package}
```

**Test Results:**
<!-- Paste relevant test output showing all tests pass -->
```
running X tests
test result: ok. X passed; 0 failed; 0 ignored
```

**Edge Cases Tested:**
<!-- List specific edge cases you verified -->
- [ ] Empty input
- [ ] Boundary conditions (min/max values)
- [ ] Error handling paths
- [ ] Performance with large inputs

---

## 📸 Screenshots/Examples (if applicable)

**Before:**
<!-- Show the problem or old behavior -->

**After:**
<!-- Show the solution or new behavior -->

**Example Output:**
<!-- Paste example program output or demo results -->
```
Example output here
```

---

## 🔗 Related Issues/PRs

**Closes:** #XX
**Related to:** #YY, #ZZ

**Dependencies:**
<!-- Does this PR depend on other PRs being merged first? -->

---

## 📚 Additional Context

**Design Decisions:**
<!-- Explain why you chose this approach over alternatives -->

**Breaking Changes:**
<!-- List any breaking changes and migration path -->

**Performance Impact:**
<!-- Describe any performance improvements or regressions -->

**Documentation Updates:**
<!-- Link to related documentation changes -->

---

## 🎓 Learning Notes (Optional)

**What I Learned:**
<!-- Share insights gained while working on this -->

**Challenges Encountered:**
<!-- Problems faced and how you solved them -->

**Useful Resources:**
<!-- Links to docs, articles, or discussions that helped -->

---

## ✍️ Checklist for Reviewers

- [ ] Code follows V-Cycle methodology (REQ-X → Design → Implementation → Tests)
- [ ] Tests verify stated requirements
- [ ] Documentation is clear and complete
- [ ] No clippy warnings introduced
- [ ] Changes align with project philosophy (Clean Code, testability, traceability)

---

## 📝 Notes for Maintainer

<!-- Any special instructions for merging or deployment? -->

---

**By submitting this PR, I confirm:**
- [ ] I have read and followed the [CONTRIBUTING.md](.github/CONTRIBUTING.md) guidelines
- [ ] I have completed the verification checklist above
- [ ] All tests pass locally with zero warnings
- [ ] Documentation is updated and accurate
