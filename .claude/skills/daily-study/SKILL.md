---
name: daily-study
description: Create systematic concept deep dives with runnable examples, progressive learning, and cross-content integration
---

# Daily Study Instructions - Concept Deep Dive Format

**Purpose**: Create systematic, progressive concept deep dives that build Rust expertise through evidence-based learning protocols and practical application.

---

## Daily Study Philosophy

Daily Study is **systematic concept mastery** where:
- Each day targets specific learning objectives with measurable outcomes
- Concepts build progressively from simple to complex
- Theory integrates with practice through runnable examples
- Evidence validates understanding through working code and tests

**Core Principle**: "I can explain this concept AND implement it correctly in working code."

---

## Weekly Structure

```
Day 1-2: Foundation Building    # Core concepts and basic patterns
Day 3-4: Practical Application  # Working implementations and examples
Day 5-6: Advanced Patterns      # Complex scenarios and edge cases
Day 7: Integration & Review     # Synthesis with broader knowledge
```

---

## Daily File Template

```markdown
# Day X - [Concept Name]

*Today's Focus: One-sentence learning objective*

---

## Learning Objectives
By the end of today, I will be able to:
1. [Specific measurable objective 1]
2. [Specific measurable objective 2]

## Concept Overview
### What is [Concept]?
### Why Does This Matter?
### Mental Model

## Complete Runnable Example
[Self-contained, compiling, running code]

## Deep Dive Analysis
### How It Works
### Key Patterns
### Edge Cases

## Common Pitfalls
[❌ Wrong way / ✅ Correct way examples]

## Practice Exercises

## Integration Points
### Mission Applications
### Zettelkasten Connections
### Rust Book Alignment

## Key Takeaways

## Tomorrow's Preview
```

---

## "Complete Runnable Example" Standards (CRITICAL)

Every daily study MUST include a runnable example that:
1. Compiles without errors (`cargo check`)
2. Runs without panics (`cargo run --example dayX`)
3. Demonstrates core concept through working code
4. Is self-contained (no external dependencies beyond std)
5. Includes tests that validate understanding

---

## Learning Objectives Framework (SMART)

- **Knowledge**: "I can explain the difference between Box<T> and Rc<T> with specific use cases"
- **Implementation**: "I can implement a generic stack using Vec<T> with proper error handling"
- **Analysis**: "I can analyze borrow checker errors and apply 3 resolution strategies"
- **Integration**: "I can connect today's lifetime concepts to Mission 4's linked list"

---

## Integration Requirements

- **Mission Integration** (MANDATORY): Connect to specific mission REQ-IDs
- **Zettelkasten Integration** (MANDATORY): Reference foundation and advanced concepts
- **Rust Book Integration** (RECOMMENDED): Link to official documentation

---

## Quality Standards

- All code compiles without warnings under clippy
- Include error handling appropriate to difficulty level
- Code examples are complete and runnable
- Cross-references to related content included
- Tags include week, concept, and difficulty level
