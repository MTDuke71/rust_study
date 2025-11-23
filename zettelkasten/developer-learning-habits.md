# Developer's Cheat Sheet: 5 Practical Learning Habits

*Created: 2025-10-10*  
*Source: "10 Things Software Developers Should Learn about Learning" by Neil Brown, Felienne Hermans, and Lauren Margulieux*

---

## Overview

Evidence-based learning habits from cognitive science, optimized for software developers. This system combines retrieval practice, spaced repetition, worked examples, error analysis, and metacognition into a daily practice that accelerates skill acquisition.

**Core Principle:** Active recall and deliberate practice beat passive rereading and unstructured "coding time."

**Time Investment:** 45-75 minutes per day (broken into micro-routines)

---

## Habit 1: Retrieval Practice > Rereading

**Duration:** 10-15 minutes/day  
**Goal:** Strengthen memory circuits through active recall

### Why It Works
Recalling information from memory forces the brain to reconstruct knowledge pathways, strengthening neural connections. Rereading creates false familiarity without building retrieval strength.

### Daily Micro-Routine

**Checkpoint Tasks:**
- [ ] List 3 concepts you *nearly* forgot yesterday (e.g., lifetimes, DFS, `async/await`)
- [ ] For each concept, **explain from memory** in 3-5 bullet points (no looking!)
- [ ] Write a 60-second code sketch demonstrating the concept (no copy/paste)
- [ ] Compare against reference material; identify and fix misconceptions

### Template: 3×5 Recall Card

```markdown
## Topic: [Concept Name]

### Explain it (max 5 bullets):
- 
- 
- 
- 
- 

### Tiny code sketch (≤ 10 lines):
```rust
// Your minimal working example
```

### One gotcha:
- 

### Verification date: YYYY-MM-DD
```

### Rust-Specific Examples

**Sample Retrieval Prompts:**
- "Explain ownership rules without looking"
- "Why does `String` move but `&str` copies?"
- "Implement `Option::map` from memory"
- "What's the difference between `iter()` and `into_iter()`?"

---

## Habit 2: Spaced Repetition Calendar

**Setup Time:** 5-10 minutes (one-time)  
**Daily Maintenance:** Autopilot (5 min review)  
**Goal:** Leverage spacing effect for long-term retention

### Why It Works
The brain consolidates information during sleep and rest periods. Reviewing at increasing intervals (1→3→7→21→60 days) optimizes the forgetting curve, moving knowledge from short-term to long-term memory.

### Minimal Schedule

**Spacing Intervals:**
1. Day 1: Initial learning
2. Day 3: First review (+2 days)
3. Day 7: Second review (+4 days)
4. Day 21: Third review (+14 days)
5. Day 60: Fourth review (+39 days)

### Implementation Options

**Low-Tech (Markdown):**
```markdown
## Due 2025-10-12
- [ ] Recall: Binary search invariants
- [ ] Recall: Rust lifetime elision rules

## Due 2025-10-15
- [ ] Recall: DFS vs BFS complexity
```

**High-Tech (Anki/RemNote):**
- Use pre-built decks or create custom cards
- Add code snippets as images or markdown
- Tag by topic (e.g., #rust #algorithms #patterns)

### Card Prompt Examples

**Concept Cards:**
- "Implement recursive in-order traversal from scratch"
- "What's the difference between `&str` and `String`? Give 2 code examples"
- "Explain the borrow checker's three rules with counterexamples"

**Pattern Recognition Cards:**
- "When is `Vec<T>` better than `&[T]`? List 3 scenarios"
- "Identify the graph problem: [scenario description]"
- "Why use `Rc<RefCell<T>>` vs `Arc<Mutex<T>>`?"

**Proof/Verification Cards:**
- "Prove why binary search invariants prevent off-by-one errors"
- "Show how Rust's type system prevents data races at compile time"

---

## Habit 3: Worked Example → Faded Example → Bare Problem

**Duration:** 25-45 minutes per learning block  
**Goal:** Reduce cognitive load while building independent problem-solving

### Why It Works
Cognitive load theory shows that beginners benefit from worked examples (full solutions with explanations), then gradually removing scaffolding (faded examples), finally solving independently (bare problems). This prevents overwhelm while building competence.

### Three-Phase Protocol

#### Phase 1: Worked Example (WE) - 10-15 min

**Task:** Read a clean, annotated solution

**Annotation Rubric:**
- [ ] Problem specification in 1 sentence
- [ ] Identify the key invariant/algorithm pattern
- [ ] Mark potential pitfalls (off-by-one, aliasing, panics)
- [ ] Note time/space complexity
- [ ] Sketch a property-based test approach

**Example WE Annotation:**
```rust
// Problem: Find first occurrence of target in sorted array
// Invariant: target is in [left, right] if it exists
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();  // ⚠️ PITFALL: Use len(), not len()-1
    
    while left < right {  // 📝 INVARIANT: search [left, right)
        let mid = left + (right - left) / 2;  // ⚠️ Avoids overflow
        
        match arr[mid].cmp(&target) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid,  // ⚠️ Not mid-1
        }
    }
    None
}
// ⏱️ O(log n) time, O(1) space
// 🧪 Property test: sorted input → found items return correct index
```

#### Phase 2: Faded Example (FE) - 10-15 min

**Task:** Fill in blanks (30-50% of solution hidden)

**Example FE:**
```rust
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = _________;  // Fill in
    
    while _________ {  // Fill in loop condition
        let mid = left + (right - left) / 2;
        
        match arr[mid].cmp(&target) {
            Ordering::Equal => _________,  // Fill in
            Ordering::Less => left = _________,
            Ordering::Greater => right = _________,
        }
    }
    None
}
```

#### Phase 3: Bare Problem (BP) - 15-20 min

**Task:** Solve from scratch, then diff against trusted solution

**Process:**
1. Write solution without looking
2. Run tests to verify correctness
3. Compare against reference implementation
4. Document differences and lessons learned

### Learning Log Template

```markdown
## Topic: [Algorithm/Pattern Name]

### Worked Example Notes:
- Problem: 
- Key invariant: 
- Pitfalls identified: 
- Test strategy: 

### Faded Example Blanks Filled:
- Challenges: 
- Incorrect attempts: 

### Bare Problem Deltas:
- My approach: 
- Reference approach: 
- Key differences: 

### One Generalizable Pattern:
- 

### Next Application:
- 
```

---

## Habit 4: Error Bank + Bug Drills

**Duration:** 10 minutes after each coding session  
**Goal:** Transform mistakes into learning assets

### Why It Works
Errors contain high-signal information about mental models and knowledge gaps. By systematically cataloging and drilling errors, you prevent repeated mistakes and build robust debugging skills.

### Error Bank Table Structure

Create `error_bank.md` or use a spreadsheet:

| Date       | Language | Symptom                           | Root Cause                    | Fast Repro (≤10 lines)           | Prevention Rule                     | Tags           |
| ---------- | -------- | --------------------------------- | ----------------------------- | -------------------------------- | ----------------------------------- | -------------- |
| 2025-10-10 | Rust     | "cannot move out of borrowed"     | Moved `String` after borrowing| `let s = &String; drop(s.clone())` | "Clone before move or use `&str`"   | #ownership     |
| 2025-10-09 | Rust     | "lifetime mismatch in closure"    | Captured ref outlived owner   | `|| &local_var` in long-lived fn | "Use `move` or extend lifetimes"    | #lifetimes     |
| 2025-10-08 | Python   | `KeyError` in dict access         | Assumed key existed           | `d['missing']`                   | "Use `.get()` or check `in d`"      | #error-handling|

### Bug Drill Protocol

**Time:** 15-20 minutes per bug

1. **Isolate:** Create minimal reproduction (≤10 lines)
2. **Fix Twice:** Solve using two different techniques
3. **Generalize:** Write one-line prevention rule
4. **Test:** Add regression test to prevent recurrence

**Example Bug Drill (Rust):**

```rust
// BUG: Borrow checker error
fn broken_code() {
    let mut vec = vec![1, 2, 3];
    let first = &vec[0];  // Immutable borrow
    vec.push(4);          // ❌ Mutable borrow while immutable exists
    println!("{}", first);
}

// FIX 1: Clone the value
fn fix1_clone() {
    let mut vec = vec![1, 2, 3];
    let first = vec[0];   // Copy the i32
    vec.push(4);
    println!("{}", first);
}

// FIX 2: Narrow borrow scope
fn fix2_scope() {
    let mut vec = vec![1, 2, 3];
    {
        let first = &vec[0];
        println!("{}", first);
    }  // Borrow ends here
    vec.push(4);
}

// PREVENTION RULE: "Don't hold references across mutations"
```

### Pre-Commit Checklist

Keep this in your IDE or print it:

**Before Committing:**
- [ ] Variable names reflect invariants (e.g., `sorted_vec`, `valid_count`)
- [ ] Panics only at truly impossible states (use `expect()` with clear messages)
- [ ] Tests cover: happy path + boundary conditions + edge cases
- [ ] Error handling uses `Result` or `Option` (no silent failures)
- [ ] Code passes `cargo clippy -- -D warnings`

---

## Habit 5: Metacognitive Loop

**Duration:** 5-20 minutes (daily exit + weekly retro)  
**Goal:** Optimize your learning process itself

### Why It Works
Metacognition (thinking about thinking) helps identify what learning strategies work best *for you*. Regular reflection accelerates improvement by surfacing patterns and blind spots.

### Daily Exit Ticket (5 min)

**End-of-day reflection:**

```markdown
## Learning Log - [Date]

### One concept that clicked:
- **What:** 
- **Why it clicked:** 
- **How I'll apply it:** 

### One confusion to revisit:
- **Topic:** 
- **Specific question:** 
- **Scheduled review:** [Date + 3 days]

### Next micro-experiment:
- **What I'll try differently tomorrow:** 
- **Expected outcome:** 
- **How I'll measure success:** 
```

### Weekly Retrospective (15 min)

**Every Friday or Sunday:**

```markdown
## Week of [Date Range]

### ROI Analysis:
- [ ] Which habit gave most value this week?
- [ ] Time spent per habit vs. learning gains
- [ ] Any habits to adjust/drop?

### Spaced Repetition Calibration:
- [ ] Cards that were too easy (increase spacing)
- [ ] Cards that were too hard (add worked examples first)
- [ ] New topics to add to rotation

### Automation Opportunities:
- [ ] What manual steps can be scripted?
- [ ] Which templates need refinement?
- [ ] What tools/aliases would save time?

### Next Week's Focus:
- **Primary skill:** 
- **Secondary skill:** 
- **One experiment:** 
```

---

## One-Week Starter Plan

**Goal:** Build the habit loop before expanding scope

### Day 1 (Monday) - Foundation

**Morning (10 min):**
- [ ] Create 3 retrieval cards on Topic A (e.g., Rust ownership)
- [ ] Schedule in spaced repetition system

**Afternoon (30 min):**
- [ ] Work through one WE→FE→BP cycle on small algorithm (binary search)
- [ ] Log one generalizable pattern

**Evening (10 min):**
- [ ] Add 1 entry to error bank from today's mistakes
- [ ] Daily exit ticket

### Day 2 (Tuesday) - Reinforcement

**Morning (10 min):**
- [ ] Review Day 1 cards (1→3 spacing)
- [ ] Add 2 new cards on Topic A

**Afternoon (25 min):**
- [ ] Tiny project using Topic A with 10-line test
- [ ] Document one bug and prevention rule

**Evening (5 min):**
- [ ] Exit ticket

### Day 3 (Wednesday) - Expansion

**Morning (10 min):**
- [ ] Create 2 cards on Topic B (e.g., graph algorithms)
- [ ] Review old cards due today

**Afternoon (30 min):**
- [ ] FE→BP cycle on parsing/string handling
- [ ] Add prevention rule to checklist

**Evening (5 min):**
- [ ] Exit ticket

### Day 4 (Thursday) - Integration

**Morning (10 min):**
- [ ] Review cards due today
- [ ] Create 1 integration card (Topics A + B)

**Afternoon (35 min):**
- [ ] Solve problem interleaving A + B
- [ ] Note where you slow down (knowledge gaps)

**Evening (5 min):**
- [ ] Exit ticket

### Day 5 (Friday) - Assessment + Reflection

**Morning (10 min):**
- [ ] Review cards due today

**Afternoon (30 min):**
- [ ] Bare problem from scratch (no guidance)
- [ ] Diff against reference solution
- [ ] Document learning deltas

**Evening (15 min):**
- [ ] Weekly retrospective
- [ ] Adjust next week's deck and drill focus

---

## Tooling Quick-Start Guide

### Ultra-Light Setup (Zero Friction)

**Files needed:**
1. `cheatsheet.md` - This document
2. `cards.md` - Spaced repetition cards with due dates
3. `error_bank.md` - Bug catalog table

**Daily workflow:**
1. Check `cards.md` for today's reviews
2. Do retrieval practice in scratch file
3. Add new cards/errors to respective files
4. Commit to Git daily

### Moderate Setup (Balanced)

**Tools:**
- **Anki** or **RemNote** for spaced repetition
- `~/code/learning/error_bank/` folder with snippets
- Simple markdown journal for daily logs

**Automation:**
- Git aliases: `git learning-log` opens today's log
- Snippet templates in IDE for recall cards

### Full Send Setup (Maximum ROI)

**Tools:**
- Anki with custom card types (code blocks, syntax highlighting)
- `Makefile` with `make drills` target to run bug snippets
- Pre-commit hooks to run checklist
- Weekly cron job to remind retrospective

**Example Makefile:**
```makefile
.PHONY: drills review log

drills:
	@echo "Running bug drills..."
	@cargo test --quiet bug_drills::

review:
	@echo "Opening spaced repetition..."
	@open anki://

log:
	@code ~/learning/$(shell date +%Y-%m-%d).md
```

---

## Mentoring & Pairing Add-Ons

### Solo Enhancement
- **Rubber duck debugging:** Narrate your thought process out loud
- **Pattern recognition practice:** Label problem types before solving
- **Deliberate struggle:** Time-box hard problems (Pomodoro technique)

### Pair Programming Integration

**During Pairing:**
- [ ] Narrate pattern recognition ("This looks like a graph problem because...")
- [ ] Swap WE/FE/BP roles (driver reads WE, navigator solves FE)
- [ ] Share error banks; discuss prevention strategies

**Team Practices:**
- [ ] Weekly "bug postmortem" sessions (celebrate good error analysis)
- [ ] Shared error bank repository (team knowledge base)
- [ ] Spaced repetition on team patterns/conventions

---

## Print-Friendly Reminder Card

```
╔═══════════════════════════════════════════╗
║   DAILY LEARNING LOOP (45-75 min)        ║
╠═══════════════════════════════════════════╣
║                                           ║
║  1. RECALL (10 min)                       ║
║     └─ Explain 3 concepts from memory    ║
║                                           ║
║  2. SPACE (5 min)                         ║
║     └─ Review cards due today            ║
║                                           ║
║  3. FADE GUIDANCE (30 min)                ║
║     └─ WE → FE → BP on one problem       ║
║                                           ║
║  4. DRILL BUGS (10 min)                   ║
║     └─ Add 1 error bank entry            ║
║                                           ║
║  5. REFLECT (5 min)                       ║
║     └─ Daily exit ticket                 ║
║                                           ║
║  Friday: +15 min weekly retrospective    ║
║                                           ║
╚═══════════════════════════════════════════╝
```

---

## Rust-Specific Applications

### Topic Rotation Schedule

**Week 1-2:** Ownership & Borrowing
- Retrieval: Explain borrow checker rules
- WE→FE→BP: Lifetime annotation exercises
- Error bank: Common borrowing mistakes

**Week 3-4:** Collections & Iterators
- Retrieval: Vec vs slice vs array trade-offs
- WE→FE→BP: Iterator transformation chains
- Error bank: Off-by-one errors, mutation while iterating

**Week 5-6:** Traits & Generics
- Retrieval: When to use `impl Trait` vs `dyn Trait`
- WE→FE→BP: Implementing custom traits
- Error bank: Trait bound errors, orphan rule violations

**Week 7-8:** Error Handling
- Retrieval: `Result` vs `Option` vs `panic!`
- WE→FE→BP: Error propagation with `?` operator
- Error bank: Unwrap abuse, missing error context

**Week 9-10:** Concurrency
- Retrieval: `Send` vs `Sync` marker traits
- WE→FE→BP: Channel patterns, `Arc<Mutex<T>>` usage
- Error bank: Data races caught by type system

### Integration with Existing Missions

**Mission 1-3:** Use for mastering basic data structures
**Mission 4-5:** Apply to algorithm implementation patterns
**Mission 6-7:** Practice on graph algorithms and BFS/DFS
**Daily Study:** Use retrieval cards for daily topics

---

## Success Metrics

### Leading Indicators (Track Weekly)
- Number of cards reviewed consistently
- Error bank entries with prevention rules
- Completed WE→FE→BP cycles
- Daily exit tickets filled out

### Lagging Indicators (Track Monthly)
- Time to solve similar problems (decreasing)
- Mistakes repeated from error bank (decreasing)
- Comfort with new concepts (self-rated 1-10)
- Project completion velocity (increasing)

### Adjustment Triggers
- **Too many failed cards:** Add more worked examples
- **Too easy:** Increase problem difficulty, expand spacing
- **Taking too long:** Reduce scope, focus on fundamentals
- **Not sticking:** Reduce time commitment, increase frequency

---

## Common Pitfalls & Solutions

### Pitfall 1: "I don't have time"
**Solution:** Start with 15 minutes (Habit 1 only). Consistency beats volume.

### Pitfall 2: "My cards are too hard/easy"
**Solution:** Calibrate difficulty in weekly retro. Use FE phase as bridge.

### Pitfall 3: "I forget to do my reviews"
**Solution:** Calendar reminders, habit stacking (after morning coffee), accountability partner.

### Pitfall 4: "This feels like busywork"
**Solution:** Measure outcomes (problem-solving speed, bug reduction). If not improving after 1 month, adjust.

### Pitfall 5: "I'm just memorizing, not understanding"
**Solution:** Focus more on WE→FE→BP (understanding) vs raw flashcards. Add "explain why" to retrieval prompts.

---

## References & Further Reading

### Primary Source
- **"10 Things Software Developers Should Learn about Learning"**  
  Neil Brown, Felienne Hermans, Lauren Margulieux  
  [Link to paper/article if available]

### Supporting Research
- **Cognitive Load Theory:** Sweller et al.
- **Spacing Effect:** Ebbinghaus forgetting curve
- **Deliberate Practice:** Ericsson & Pool
- **Testing Effect:** Roediger & Karpicke

### Tools Mentioned
- **Anki:** Open-source spaced repetition software
- **RemNote:** Note-taking + spaced repetition
- **Obsidian:** Markdown-based note system (Zettelkasten)

---

*Tags: #learning-science #deliberate-practice #spaced-repetition #retrieval-practice #error-analysis #metacognition #rust #software-development #pedagogy*

*Links: [[zettel-index]] | [[MONTHLY_CALENDAR]] | [[mission-1]] | [[mission-7]] | [[course-creation-strategy]]*

---

**Next Actions:**
- [ ] Set up initial card deck (10 cards minimum)
- [ ] Create `error_bank.md` file
- [ ] Print reminder card and tape to monitor
- [ ] Schedule first weekly retrospective (Friday 5pm)
- [ ] Commit to 2-week trial before evaluating

**Review Schedule:** Revisit this note on Day 7, Day 21, and Day 60 to refine based on experience.
