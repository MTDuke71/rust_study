---
name: zettel
description: Create interconnected zettelkasten knowledge nodes with bidirectional links, proper naming, and cross-content integration
---

# Zettelkasten Instructions - Knowledge Graph Creation

**Purpose**: Create interconnected knowledge nodes that form a comprehensive learning graph for Rust concepts, patterns, and implementations.

---

## Zettelkasten Philosophy

This is **knowledge graph construction** where:
- Each file is a concept node with bidirectional connections
- Links create learning pathways between related ideas
- Cross-references integrate all content types (missions, tutorials, daily study, rust book, zettelkasten)

**Core Principle**: Every file should answer: "How does this concept connect to what I already know?"

---

## Naming Conventions (CRITICAL - prevents collisions)

```
✅ CORRECT:
- [[borrow-checker-patterns]]         # Concept files: lowercase-with-dashes
- [[daily-study/Day24]]              # Daily study: with path prefix
- [[mission-5]]                      # Mission files: lowercase with dash
- [[rust_book/rust-book-ch8]]        # Rust book: with path and prefix

❌ INCORRECT:
- [[Day24]]                          # Missing path prefix
- [[Mission5]]                       # Wrong capitalization
- [[Borrow Checker Patterns]]       # Spaces instead of dashes
```

---

## File Structure Template

```markdown
# Title - Brief Description

*One-sentence summary that captures the core concept*

---

## Core Concept
Define the concept clearly. What is it? Why does it matter?

## Mental Models
Provide analogies, visualizations, or frameworks for understanding.

## Detailed Content
### Subsection 1: [Specific aspect]
### Subsection 2: [Another aspect]

## Key Takeaways
3-5 bullet points summarizing the most important insights.

## Integration Points

### Builds On
- [[prerequisite-concept-1]] - What you need to know first

### Enables
- [[advanced-concept-1]] - What this unlocks

### Related Concepts
- [[related-concept-1]] - Parallel/similar ideas

---

*Tags: #primary-tag #secondary-tag #content-type-tag*
*Links: [[zettel-index]] | [[related-moc]] | [[concept-1]]*
```

---

## Bidirectional Linking Protocol

When creating a new zettelkasten file:
1. **Add outgoing links** in the new file to related concepts
2. **Update related files** to include incoming links back
3. **Check MOC files** to see if new file should be included
4. **Update zettel-index.md** if creating a major concept or MOC

---

## Tagging System

**Primary tags**: `#concept`, `#pattern`, `#algorithm`, `#data-structure`, `#mission-X`, `#daily-study`, `#rust-book`, `#troubleshooting`, `#performance`

**Secondary tags**: `#beginner`/`#intermediate`/`#advanced`, `#practical`/`#theoretical`, `#memory-safety`/`#concurrency`/`#ownership`

Every file needs 3-5 tags minimum.

---

## Content Depth Guidelines

- **Concept Overview** (300-500 words): High-level explanation, key mental models, essential links
- **Deep Dive** (800-1500 words): Comprehensive explanation, multiple models, practical applications
- **MOC** (500-800 words): Navigate related concepts, learning pathways, integration

---

## Quality Standards

- Clear concept definition in first paragraph
- At least one practical example with code
- Mental model or analogy for understanding
- Integration with at least 2 other content types
- Bidirectional links to related concepts
- Code examples must be complete and runnable

---

## Publishing Checklist

- [ ] Title accurately reflects content
- [ ] One-sentence summary captures core concept
- [ ] At least 3 bidirectional links added
- [ ] Related files updated with links back
- [ ] Tags include primary category and difficulty level
- [ ] Code examples are complete and tested
- [ ] Integration section connects to other content types
- [ ] Daily study references verified (check file headers)
