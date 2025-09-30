# 🏷️ Zettelkasten Tagging Standards

**MANDATORY** for all new content creation in the `zettelkasten/` directory.

## Tag Categories & Patterns

```markdown
---
*Tags: #primary-topic #secondary-topic #content-type #learning-track*
*Links: [[zettel-index]] | [[Related MOC]] | [[Connected Concept]]*
---
```

## Primary Topic Tags (Choose ONE per note)

**Data Structures:**
- `#hashmap` `#hashset` `#btreemap` `#vector` `#linkedlist`

**Rust Concepts:**
- `#ownership` `#borrowing` `#lifetimes` `#traits` `#generics`

**Algorithm Types:**
- `#algorithms` `#sorting` `#searching` `#graph` `#tree`

**Processing Patterns:**
- `#parsing` `#regex` `#iterators` `#error-handling`

**Performance Topics:**
- `#performance` `#benchmarking` `#optimization`

**Engineering Practices:**
- `#testing` `#documentation` `#v-cycle`

## Content Type Tags (Choose ONE per note)

- `#concept` - Theoretical explanations and deep dives
- `#implementation` - Code examples and practical demonstrations
- `#tutorial` - Step-by-step learning progressions
- `#overview` - High-level summaries and navigation hubs
- `#reference` - Quick lookup and API documentation
- `#pattern` - Reusable design patterns and best practices

## Learning Track Tags (Choose ALL that apply)

- `#mission1` `#mission2` `#mission5` etc. - V-Cycle mission integration
- `#daily-study` - Daily study track concepts
- `#rust-book` - Rust Book chapter integration
- `#aoc` - Advent of Code applications
- `#competitive-programming` - Algorithm competition focus

## Cross-Reference Tags (Use for connections)

- `#cross-track` - Links multiple learning tracks
- `#prerequisite` - Required knowledge for other concepts
- `#application` - Practical usage of theoretical concepts
- `#comparison` - Comparative analysis between options
- `#troubleshooting` - Common errors and solutions

## Example Tag Combinations

### HashMap Internals Example
```markdown
*Tags: #hashmap #concept #data-structures #mission5 #daily-study #performance*
```

### Mission5 Tutorial Step 3 Example
```markdown
*Tags: #hashmap #tutorial #implementation #mission5 #step-by-step*
```

### Collections MOC Example
```markdown
*Tags: #collections #overview #data-structures #cross-track #navigation*
```

### Day 10 Study Notes Example
```markdown
*Tags: #hashmap #daily-study #concept #rust-book #prerequisite*
```

## Tag Placement Requirements

1. **Bottom of every zettelkasten file** - Include tag line before final links
2. **Consistent format** - Always use `*Tags: #tag1 #tag2 #tag3*`
3. **3-6 tags per note** - Balance discoverability with specificity
4. **Update existing files** - Add tags when editing existing zettelkasten content
5. **Cross-reference validation** - Ensure tagged concepts have corresponding `[[links]]`

## Tag-Based Navigation Workflows

- **By Topic**: `#hashmap` finds all HashMap-related content across tracks
- **By Type**: `#tutorial` finds all step-by-step learning content
- **By Mission**: `#mission5` finds all content related to Mission 5
- **By Application**: `#aoc` finds all Advent of Code usage patterns
- **Cross-Track**: `#cross-track` finds integration points between learning systems

## Obsidian Tag Integration

- Use Obsidian's **Tag Pane** for tag-based navigation
- Create **Tag Queries** for complex tag combinations
- Use **Graph View** with tag filtering for visual knowledge networks
- Set up **Tag Templates** in Obsidian for consistent tagging

**⚠️ CRITICAL**: Every new zettelkasten file MUST include appropriate tags. This enables powerful filtering, discovery, and cross-referencing within the knowledge management system.