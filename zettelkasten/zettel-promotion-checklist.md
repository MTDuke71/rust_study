# Zettel Promotion Checklist - When to Create a New Node

*A lightweight decision + workflow for promoting an insight into a stable, reusable zettelkasten node.*

---

## 🎯 Core Concept

A new zettel is worth creating when it becomes a **reusable integration point**: something you’ll reference from multiple places (AoC, Rust Book, missions, tutorials) as a stable concept or decision boundary.

The goal is to keep notes **discoverable** without letting the graph maintenance become the work.

## 🧠 Mental Models

- **Promote only what you want to reuse**: treat zettels as “library interfaces” for ideas.
- **Start local, then promote**: capture the insight where you found it; promote only if it repeats.

## 🔍 Detailed Content

### 1) Create a new zettel when…

- **Reuse**: it’s referenced (or will be) from $\ge 2$ places.
- **Stability**: the name/shape is stable next week (it’s a noun phrase / pattern / algorithm / decision boundary).
- **Structure**: it naturally needs $\ge 3$ subsections (tradeoffs, variants, pitfalls).
- **Decision boundary**: it answers a recurring choice question ("Which representation?", "When to memoize?", "Which pointer model?").
- **Graph hygiene**: it’s a broken link used in multiple files.

### 2) Do NOT create a new zettel when…

- It’s just a definition that fits inside an existing page.
- It’s highly specific to one AoC day’s implementation details (prefer a day doc + links to general zettels).
- You don’t yet know the stable name/shape (capture in a daily note or local section first).

### 3) 5–10 minute promotion workflow

1. **Capture (in place)**: write the insight in the file you’re already editing.
2. **Promote (new zettel)** if it meets the criteria above:
   - 2–4 sentence core concept
   - one mental model
   - tradeoffs/decision table (if relevant)
   - 3–8 links across Builds On / Enables / Related
3. **Wire (minimum backlinks)**:
   - add exactly **one** backlink from the best-fit MOC
   - add a second backlink only if it’s truly cross-domain
4. **Stop**: don’t chase perfection; add more links when it naturally comes up again.

## 💡 Key Takeaways

- Promote ideas that act like **reusable interfaces**, not raw scratch notes.
- Prefer the smallest viable node: clear definition + a few high-value links.
- Keep backlinking minimal to avoid graph-maintenance overload.

## 🔗 Integration Points

### Builds On
- [[zettel-index]] - Navigation hub and naming conventions
- [[Hands-On Learning]] - Iterative learning with small feedback loops
- [[Progressive Disclosure]] - Reveal detail as needed, keep the system usable

### Enables
- [[Daily Workflow]] - A repeatable “capture then promote” routine
- [[rust-concepts-MOC]] - Where promoted language concepts should land
- [[AoC Patterns MOC]] - Where promoted AoC patterns should land

---

*Tags: #zettelkasten #workflow #pattern #learning-system #practical*

*Links: [[zettel-index]] | [[Daily Workflow]] | [[Hands-On Learning]] | [[Progressive Disclosure]]*
