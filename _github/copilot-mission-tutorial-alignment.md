# Mission-Tutorial Alignment Protocol

**CRITICAL**: This document explains how to coordinate Mission projects with their companion Tutorial projects.

## 🔗 Alignment Requirements

### Core Principle
**Daily Mission Focus** goals in MONTHLY_CALENDAR.md must correspond to specific `MissionX_tut/` tutorial steps to ensure cohesive learning progression.

### Integration Goals
1. **Mission Focus Days** → Work through corresponding `MissionX_tut/examples/stepN_*.rs` files
2. **Daily Tutorial Steps** → Complete exercises that build toward main mission requirements
3. **Weekly Review** → Ensure Mission Tutorial completion aligns with main mission progress
4. **Integration Check** → By mission end, both tutorial exercises AND main mission requirements should be fulfilled

## 📋 MANDATORY Pre-Creation Checklist

**BEFORE creating any new Mission + Tutorial pair:**

1. **Review MONTHLY_CALENDAR.md** - Check alignment requirements in "Track Alignment & Coordination" section
2. **Daily Focus Mapping** - Ensure MissionX_tut steps correspond to daily mission focus goals
3. **Tutorial Synchronization** - Design tutorial progression to support main mission REQ-X completion
4. **Integration Timeline** - Plan so tutorial completion = mission mastery achievement

## ✅ Alignment Verification Checklist

- [ ] Tutorial steps map to specific mission requirements (REQ-1 → step1_*, REQ-2 → step2_*, etc.)
- [ ] Daily tutorial activities build toward main mission implementation
- [ ] Tutorial completion provides all knowledge needed for mission success
- [ ] Both MissionX/ and MissionX_tut/ can be completed within planned calendar timeframe
- [ ] Each tutorial step has explicit learning objective
- [ ] Time estimates are realistic (20-40 minutes per step)
- [ ] Calendar shows exact tutorial file names (not abstractions)

## 🎯 Mission 5 Reference Implementation

### Actual Implementation Pattern
```
MONTHLY_CALENDAR.md Schedule (Sept 24-30):
Day 1: Mission 5 Setup → Mission5_tut/examples/step1_basic_hashmap.rs (REQ-1 foundation)
Day 2: Requirements Definition → Daily study + Define REQ-1 to REQ-5
Day 3: Basic Structure → step2_hashset_operations.rs (REQ-2 practice)
Day 4: Hash Function → step3_frequency_counting.rs (REQ-3 mastery)
Day 5: Core Operations → step4_multi_value_patterns.rs (REQ-4)
Day 6: Iterator Implementation → step5_memoization_cache.rs (REQ-5)
Day 7: Testing & Documentation → final_project.rs + Complete Mission 5 V-Cycle

Tutorial-to-Requirement Mapping:
step1_basic_hashmap.rs → REQ-1 (Dictionary wrapper with enhanced functionality)
step2_hashset_operations.rs → REQ-2 (Set operations for membership testing)
step3_frequency_counting.rs → REQ-3 (Efficient counting patterns)
step4_multi_value_patterns.rs → REQ-4 (Multi-value dictionaries)
step5_memoization_cache.rs → REQ-5 (Caching and memoization)
final_project.rs → REQ-6 integration (AoC-specific utilities)
```

### Mission5 Success Metrics
- ✅ 6 core tutorial steps + 24 supplementary examples = 30+ total files
- ✅ Each step: 20-40 minutes, 400-850 lines
- ✅ All 6 mission requirements addressed in tutorial progression
- ✅ 7-day calendar schedule matched tutorial completion timeline
- ✅ Tutorial README had learning objectives, time estimates, prerequisites

## 🚧 Common Alignment Mistakes

### ❌ What NOT to Do
1. **Abstract calendar entries**: "Day 2: Requirements → step2_collision_handling.rs" when file is actually "step2_hashset_operations.rs"
2. **Missing REQ tags**: Tutorial steps don't document which requirements they address
3. **Time misalignment**: 10-day mission with 3-day tutorial
4. **Requirement gaps**: REQ-4 exists but no tutorial step covers it
5. **Calendar ambiguity**: "Work on Mission 5" instead of specific file names

### ✅ Best Practices
1. **Exact file names**: Calendar uses actual file paths from examples/
2. **REQ traceability**: Each step documents "Requirements Addressed: REQ-X"
3. **Time budget**: Total tutorial time ≤ allocated calendar days × 15 min/day
4. **Complete coverage**: Every mission requirement has corresponding tutorial step
5. **Explicit navigation**: Each step ends with "Next: cargo run --example stepN+1"

## 📅 Calendar Integration Template

```markdown
### Week X: Mission N (Topic)

**BEFORE writing calendar entries:**
1. Define all mission REQ-1 to REQ-N
2. Design tutorial steps that each address 1-2 requirements
3. Name tutorial files: stepN_specific_concept.rs
4. Verify total tutorial time fits allocated days

**Calendar Entry Format:**
### **Day Name, Date** emoji
**Mission Focus**: Mission N [Specific Focus]
**Daily Study**: Week X, Day Y - [Topic]
**Rust Book**: Chapter Z.N - [Section]
```bash
# Daily Tasks
cd MissionN && cargo test reqN_*
cargo run --example stepN_specific_concept  # Tutorial step
# Daily study activity
# Rust book reading
```

**Tutorial Files Referenced:**
- Mission{N}_tut/examples/step1_foundation.rs (20 min, REQ-1)
- Mission{N}_tut/examples/step2_building.rs (25 min, REQ-2)
- Mission{N}_tut/examples/step3_advanced.rs (30 min, REQ-3)
- Mission{N}_tut/examples/final_project.rs (20 min, integration)
```

## 🔍 Verification Commands

Before finalizing Mission + Tutorial pair:

```bash
# Check all tutorial steps exist
ls Mission{N}_tut/examples/step*.rs

# Verify they're runnable
cargo run --example step1_foundation
cargo run --example step2_building

# Check main mission tests
cd Mission{N}
cargo test

# Validate requirement coverage
grep -r "REQ-" Mission{N}/README.md
grep -r "Requirements Addressed" Mission{N}_tut/examples/
```

## 📚 Additional Resources

- See [MISSION5_CASE_STUDY.md](MISSION5_CASE_STUDY.md) for complete working example
- See [tutorial.engineer.md](tutorial.engineer.md) for pedagogical design principles
- See [COMPLETE_RUNNABLE_EXAMPLE_TEMPLATE.md](COMPLETE_RUNNABLE_EXAMPLE_TEMPLATE.md) for tutorial step structure

## ⚠️ Critical Reminder

**Every Mission + Tutorial pair must be planned as a unified system.** The tutorial is not optional supplementary material - it's the primary learning path that builds toward completing the main mission. Design them together, not sequentially.

---

## 🔧 Troubleshooting Alignment Issues

### Problem: Tutorial and Mission Out of Sync

**Symptom**: Completed tutorial but main mission tests fail

**Diagnosis:**
```bash
# Check tutorial coverage
cd MissionX_tut
cargo run --example step1_*  # Do all steps work?

# Check mission requirements
cd ../MissionX
grep "REQ-" README.md  # List all requirements

# Check which tests exist
ls tests/req*.rs  # Do we have test for each REQ?
```

**Solution:**
1. Map each tutorial step to specific REQ-X
2. Add missing tutorial steps for uncovered requirements
3. Update calendar to include all steps

### Problem: Calendar Shows Wrong File Names

**Symptom**: MONTHLY_CALENDAR.md references `step2_collision_handling.rs` but file doesn't exist

**Diagnosis:**
```bash
# List actual tutorial files
ls MissionX_tut/examples/step*.rs

# Compare with calendar entries
grep "step[0-9]_" MONTHLY_CALENDAR.md
```

**Solution:**
1. Update calendar with exact file names from `ls` output
2. Use format: `MissionX_tut/examples/stepN_actual_name.rs`
3. Verify each file exists before committing calendar

### Problem: Time Budget Doesn't Match

**Symptom**: 5-day calendar but tutorial needs 10 hours

**Diagnosis:**
```bash
# Calculate total tutorial time
grep "Time Estimate" MissionX_tut/README.md
# Sum: step1 (20min) + step2 (25min) + ... = Total

# Check allocated calendar days
grep "Mission X" MONTHLY_CALENDAR.md | wc -l
# Allocated days × 15 minutes = Available time
```

**Solution:**
1. Budget 15 minutes/day for tutorial work
2. If tutorial > allocated time:
   - Split large steps (step5 → step5a, step5b)
   - Extend calendar duration
   - Move deep-dives to "optional" section
3. Update time estimates to be realistic

### Problem: Missing REQ Tags in Tutorial

**Symptom**: Can't trace which tutorial step addresses which requirement

**Diagnosis:**
```bash
# Check tutorial step headers
grep -r "Requirements Addressed" MissionX_tut/examples/

# If empty, REQ tags are missing
```

**Solution:**
Add to each tutorial step file header:
```rust
//! # Step N: [Concept]
//!
//! **Requirements Addressed**: REQ-1, REQ-3
//! ^^^^ ADD THIS LINE
```

### Problem: Supplementary Examples Not Documented

**Symptom**: Found 20+ example files but only 6 core steps mentioned

**Diagnosis:**
```bash
# Count all example files
ls MissionX_tut/examples/*.rs | wc -l

# Count documented steps
grep "step[0-9]" MissionX_tut/README.md | wc -l

# If big difference, many undocumented examples
```

**Solution:**
Add to tutorial README and calendar:
```markdown
### Core Learning Path (Required)
- step1_foundation.rs (20 min)
- step2_building.rs (25 min)
- ...

### Optional Deep-Dives (Recommended)
- collision_handling_deep_dive.rs (15 min)
- performance_analysis.rs (20 min)
- real_world_application.rs (30 min)
```

### Problem: Learner Stuck on Exercise

**Symptom**: Tutorial exercise has no visible solution

**Diagnosis:**
```bash
# Check if solutions are feature-gated
grep "#\[cfg(feature" MissionX_tut/examples/step*.rs
```

**Solution:**
Either:
1. Remove feature gate, add clear "SPOILER ALERT" marker
2. Or document how to enable: `cargo run --example step1 --features solutions`
3. Or put solutions at end of same file after clear separator

### Common Integration Mistakes Checklist

Run this checklist before finalizing Mission + Tutorial:

```bash
# 1. File names match calendar
diff <(grep "step[0-9]" MONTHLY_CALENDAR.md) <(ls MissionX_tut/examples/step*.rs)

# 2. All requirements have tests
for req in $(grep "REQ-" MissionX/README.md | grep -oP 'REQ-\d+' | sort -u); do
    test -f "MissionX/tests/${req,}_*.rs" || echo "Missing test for $req"
done

# 3. All tutorial steps compile
cd MissionX_tut
for step in examples/step*.rs; do
    cargo check --example $(basename $step .rs) || echo "Failed: $step"
done

# 4. Main mission tests pass
cd ../MissionX
cargo test --all || echo "Mission tests failing"

# 5. Time budget reasonable
echo "Total tutorial time should be ≤ (calendar days × 15 min)"
```

### Getting Help with Alignment

- 📋 Review [MISSION5_CASE_STUDY.md](MISSION5_CASE_STUDY.md) for working example
- 🔍 Check existing Mission + Tutorial pairs for patterns
- 📚 See [tutorial.engineer.md](tutorial.engineer.md) for pedagogical guidance
- 💡 Ask: "Can I complete tutorial and pass mission tests in allocated time?"