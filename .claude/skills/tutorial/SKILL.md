---
name: tutorial
description: Create step-by-step progressive learning scaffolds that bridge concepts to mission-level implementations
---

# Tutorial Instructions - Progressive Learning Scaffolding

**Purpose**: Create step-by-step learning progressions that bridge the gap between basic concepts and complex mission implementations through scaffolded practice.

---

## Tutorial Philosophy

Tutorials are **learning scaffolds** where:
- Each step builds incrementally on previous understanding
- Complexity increases gradually from basic to mission-level
- Mistakes are anticipated and addressed proactively
- Practice reinforces theory through hands-on implementation

**Core Principle**: "I can confidently implement each step independently, and I understand how each step prepares me for the next level."

---

## 7-Step Progression Framework

```
Step 1-2: Foundation Building     # Basic structure and core concepts
Step 3-4: Core Implementation     # Essential functionality and patterns
Step 5-6: Advanced Features       # Complex scenarios and optimizations
Step 7: Mission Integration       # Production-ready, mission-level code
```

---

## Tutorial Directory Structure (MANDATORY)

```
tutorials/MissionX_tut/
├── README.md                     # 7-step roadmap and learning objectives
├── examples/
│   ├── step1_basic_structure.rs
│   ├── step2_core_operations.rs
│   ├── step3_error_handling.rs
│   ├── step4_advanced_patterns.rs
│   ├── step5_optimization.rs
│   ├── step6_integration.rs
│   └── step7_mission_ready.rs
├── exercises/
│   ├── exercise1_basics.md
│   ├── exercise2_intermediate.md
│   └── exercise3_advanced.md
├── solutions/
│   ├── exercise1_solution.rs
│   ├── exercise2_solution.rs
│   └── exercise3_solution.rs
└── TROUBLESHOOTING.md
```

---

## Step File Structure (REQUIRED)

```rust
//! Step X: [Focus Area]
//!
//! **Learning Objective**: By the end of this step, you will be able to [specific skill].
//! **Prerequisites**: Completion of Step X-1 and understanding of [concepts].
//! **Next Step Preview**: Step X+1 will build on this by adding [next capability].

fn main() {
    println!("=== Step X: [Focus Area] ===");
    demonstrate_core_concept();
    show_common_patterns();
    handle_edge_cases();
    validate_understanding();
    println!("✅ Step X completed! Ready for Step {}.", X + 1);
}
```

---

## Progressive Complexity Standards

- **Step 1-2 (Foundation)**: Simple types, constructors, basic ownership
- **Step 3-4 (Core)**: Error handling with Result/Option, essential operations
- **Step 5-6 (Advanced)**: Performance optimization, benchmarks, complex scenarios
- **Step 7 (Mission Ready)**: Full API, REQ-ID documentation, production-quality code

---

## Mission Preparation Matrix

| Tutorial Step | Mission REQ | Skill Developed | Validation |
|---------------|-------------|-----------------|------------|
| Step 1-2 | REQ-1, REQ-2 | Basic structure, memory safety | Unit tests |
| Step 3-4 | REQ-3, REQ-4 | Operations, error handling | Integration tests |
| Step 5-6 | REQ-5, REQ-6 | Performance, optimization | Benchmarks |
| Step 7 | All REQs | Mission-ready implementation | Full test suite |

---

## Quality Standards

- All steps compile cleanly with `cargo clippy -- -D warnings`
- Include working tests that demonstrate functionality
- Clear learning objectives that are measurable
- Build incrementally with no gaps
- Connect to broader learning (missions, daily study, zettelkasten)
