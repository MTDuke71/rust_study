# 📚 Daily Study MOC - Structured Learning Progression

**Map of Content: Organized daily learning materials for systematic Rust skill development**

---

## 🎯 **Overview**

The Daily Study track provides **structured daily materials** following the 3-Track Learning System. Each day builds progressively on previous concepts, coordinated with Mission projects and Rust Book chapters.

### **Learning Philosophy**
- ✅ **Progressive Disclosure** - Concepts introduced in digestible daily chunks
- ✅ **Hands-On Practice** - Every day includes complete runnable examples
- ✅ **Cross-Track Integration** - Daily concepts support Mission implementations
- ✅ **AoC Preparation** - Competitive programming patterns and techniques
- 🌟 **Expanded Deep Dives** - Selected days include comprehensive expanded guides with detailed analogies and examples

---

## 📂 **Weekly Structure**

### **Week 1: Foundations** (Days 1-7)
**Focus**: Ownership, Borrowing, Core Language Features

| Day | Topic | Key Concepts | Links |
|-----|-------|--------------|-------|
| [[Day 01 - Setup\|Day 1]] | Setup & Tooling | Cargo, rustc, project structure | [[../rust_book/Ch1\|Rust Book Ch1]] |
| [[Day 02 - Ownership Basics\|Day 2]] | Ownership Basics | Move semantics, stack vs heap | [[../rust_book/Ch4\|Ch4]] · [[../daily_study/rust_learning_week1_notes/Day02_expanded\|**Expanded Deep Dive**]] |
| [[Day 03 - Borrowing\|Day 3]] | Borrowing Rules | Immutable/mutable references | [[../rust_book/Ch4\|Ch4]] · [[../daily_study/rust_learning_week1_notes/Day03_expanded\|**Expanded Deep Dive**]] |
| [[Day 04 - Lifetimes\|Day 4]] | Lifetimes | Reference validity, scope | [[../rust_book/Ch10\|Ch10]] |
| [[Day 05 - Option and Result\|Day 5]] | Error Handling | Option, Result, ? operator | [[../rust_book/Ch9\|Ch9]] |
| [[Day 06 - Pattern Matching\|Day 6]] | Pattern Matching | match, if let, destructuring | [[../rust_book/Ch6\|Ch6]] |
| [[Day 07 - Week 1 Summary\|Day 7]] | Week 1 Review | Practice & consolidation | [[Week 1 Overview]] |

**Mission Integration**: Ownership concepts → Mission 1 (Stack), Mission 4 (Linked Lists)

---

### **Week 2: Collections** (Days 8-14)
**Focus**: Foundation Data Structures

| Day | Topic | Key Concepts | Links |
|-----|-------|--------------|-------|
| Day 8 | Vectors | Dynamic arrays, capacity vs length | [[../missions/Mission1/README\|Mission1]] |
| Day 9 | Strings | String vs &str, UTF-8, manipulation | [[../rust_book/Ch8\|Ch8]] |
| [[Day 10 - HashMap Basics\|Day 10]] | HashMaps | Key-value storage, borrowing | [[../missions/Mission5/README\|Mission5]] |
| [[Day 11 - HashSet Operations\|Day 11]] | HashSets | Unique collections, set ops | [[Mission5 Overview]] |
| [[Day 12 - BTreeMap\|Day 12]] | BTreeMap & BTreeSet | Ordered collections | [[Collections MOC]] |
| [[Day 13 - Advanced Iterators\|Day 13]] | Advanced Iterators | Transforming collections | [[../missions/Mission6/SIZE_HINT_EXPLAINED]] |
| Day 14 | Error Handling | Robust error management | [[Error Handling Deep Dive]] |

**Mission Integration**: Collections mastery → Mission 5 (HashMap implementation from scratch)

**AoC Applications**: 
- HashMaps for coordinate tracking, frequency counting
- HashSets for unique element detection, visited tracking
- Iterators for efficient data transformation

---

### **Week 3: Abstractions** (Days 15-21)
**Focus**: Traits, Lifetimes, Generics

| Day | Topic | Key Concepts | Links |
|-----|-------|--------------|-------|
| Day 15 | Traits Fundamentals | Defining/implementing behavior | [[../rust_book/Ch10\|Ch10]] |
| Day 16 | Generic Types | Type parameters, constraints | [[Generic Programming]] |
| Day 17 | Lifetime Annotations | Explicit lifetime syntax | [[Multiple Lifetimes Deep Dive]] |
| Day 18 | Advanced Traits | Associated types, defaults | [[Rust Trinity - Struct Trait Impl]] |
| Day 19 | Trait Objects | Dynamic dispatch with dyn | [[Trait Objects]] |
| Day 20 | Advanced Lifetimes | Elision, 'static | [[Multiple Lifetimes Deep Dive]] |
| Day 21 | Generics Practice | Building flexible APIs | [[PhantomData Type Safety Patterns]] |

**Mission Integration**: 
- Generic implementations → Mission 1 (generic Stack\<T\>)
- Trait design → Mission 5 (Hash trait implementation)
- Type safety → All missions use generic data structures

---

### **Week 4: Applied Problem Solving** (Days 22-28)
**Focus**: AoC-Style Algorithms & Patterns

| Day | Topic | Key Concepts | Links |
|-----|-------|--------------|-------|
| Day 22 | Grid Fundamentals | 2D arrays, coordinates, storage | [[../missions/Mission6/README\|Mission6]] |
| Day 23 | Grid Navigation | Directions, bounds checking | [[Chebyshev Distance]], [[Manhattan Distance]] |
| Day 24 | Grid Algorithms | Flood fill, connected components | [[DFS Patterns]], [[BFS Patterns]] |
| Day 25 | Queue Applications | BFS, level traversal | [[../missions/Mission2/README\|Mission2]] |
| Day 26 | Advanced Queues | Priority queues, deque patterns | [[A* Search]], [[../missions/Mission6/README\|Mission6]] |
| Day 27 | String Parsing | Splitting, regex, custom parsers | [[AoC Patterns MOC]] |
| Day 28 | Week 4 Integration | Complete problem solving | [[../missions/Mission6/README\|Mission6]] |

**Mission Integration**: 
- Week 4 provides complete toolkit for Mission 6 (2D grids + pathfinding)
- Queue patterns support Mission 2 (ring buffer implementation)
- Parsing utilities used across all missions for input handling

**AoC Applications**:
- **Grid problems**: Maze solving, island counting, pathfinding
- **BFS/Dijkstra**: Shortest path, distance calculation
- **Flood fill**: Region detection, area calculation
- **Parsing**: Converting puzzle inputs to data structures

---

## 🚀 **How to Use This MOC**

### **Find Your Current Day**
1. Check [[../MONTHLY_CALENDAR|MONTHLY_CALENDAR]] for today's focus
2. Navigate to the corresponding Day file
3. Complete the runnable example
4. Explore cross-references to related content

### **Running Examples**
```bash
# From workspace root
.\scripts\run_md.bat daily_study\rust_learning_week2_notes\Day10.md

# Or with PowerShell
.\run_markdown_code.ps1 daily_study\rust_learning_week4_notes\Day24.md
```

### **Cross-Track Navigation**
- **Mission Work** → See [[Missions MOC]] for V-Cycle engineering projects
- **Rust Book** → See [[Rust Concepts MOC]] for chapter-by-chapter notes
- **AoC Practice** → See [[AoC Patterns MOC]] for problem-solving techniques

---

## 📊 **Learning Milestones**

### **Week 1 Completion** ✅
- Understand ownership and borrowing rules
- Write safe code without data races
- Handle errors with Option and Result
- Use pattern matching effectively

### **Week 2 Completion** ✅
- Master core collections (Vec, String, HashMap, HashSet)
- Use iterators for efficient data processing
- Implement robust error handling
- Apply collections to real problems

### **Week 3 Completion** ✅
- Design and implement traits
- Write generic, reusable code
- Understand lifetime annotations
- Use advanced type system features

### **Week 4 Completion** ✅
- Solve grid-based problems
- Implement BFS and Dijkstra's algorithm
- Apply flood fill and component detection
- Parse complex input formats
- Ready for Mission 6 and advanced AoC challenges

---

## 🔗 **Related Resources**

### **Mission Projects**
- [[../missions/Mission1/README|Mission1 - Stack]] - Generic stack with Week 1 ownership concepts
- [[../missions/Mission2/README|Mission2 - Queue]] - Ring buffer with Week 4 queue patterns
- [[../missions/Mission5/README|Mission5 - HashMap]] - Week 2 collections applied to hash table
- [[../missions/Mission6/README|Mission6 - 2D Grids]] - Complete Week 4 integration (grids + BFS + Dijkstra)

### **AoC Integration**
- [[AoC Patterns MOC]] - Problem-solving patterns from daily study
- [[AoC Collection Problems]] - Collections usage in competitive programming
- [[../aoc2015/README|AoC 2015 Solutions]] - Real problems using daily concepts

### **Rust Book Coordination**
- [[../rust_book/Ch4/ownership/README|Ch4 - Ownership]] → Week 1
- [[../rust_book/Ch8/collections/README|Ch8 - Collections]] → Week 2
- [[../rust_book/Ch10/generics/README|Ch10 - Generics & Traits]] → Week 3

---

## 📝 **Content Standards**

### **Every Day File Includes**
1. **🚀 Complete Runnable Example** - Copy-paste ready code (Playground or local)
2. **📖 Learning Context** - Cross-track integration header
3. **💡 Concept Explanation** - Theory with practical examples
4. **🔗 Cross-References** - Links to missions, zettelkasten, book chapters
5. **🛠️ Key Takeaways** - Summary and best practices
6. **🎯 Related Topics** - Preview and connections

### **Execution Methods**
- **Online**: Copy to [Rust Playground](https://play.rust-lang.org/)
- **Local file**: Save as `dayXX_demo.rs` and compile
- **Workspace runner**: Use `.\scripts\run_md.bat` script
- **Cargo example**: Add to examples/ directory

---

## 🎓 **Study Tips**

### **Daily Practice Pattern**
1. **Morning** (15 min): Read day's concept explanation
2. **Midday** (15 min): Run and modify the example code
3. **Evening** (15 min): Complete related mission work or AoC problem

### **When Stuck**
- Check cross-references for deeper explanations
- Review previous week's summary
- Explore related mission implementations
- Ask questions with concrete examples

### **Progress Tracking**
- Mark completed days in [[../MONTHLY_CALENDAR|MONTHLY_CALENDAR]]
- Track mission implementations in [[Missions MOC]]
- Note patterns discovered in personal notes

---

## 🔄 **3-Track Integration Flows**

### **Daily Study → Mission Projects**
```
Day 10 (HashMap Basics)
    ↓
Mission 5 (HashMap Implementation)
    ↓
Real-world hash table from scratch
```

### **Daily Study → AoC Problems**
```
Day 24 (Flood Fill)
    ↓
AoC Island Counting Problems
    ↓
Competitive programming success
```

### **Daily Study → Rust Book**
```
Day 15 (Traits)
    ↓
Rust Book Ch10 (Traits & Generics)
    ↓
Deep language understanding
```

---

## 📈 **Future Weeks Preview**

### **Week 5: Advanced Algorithms** (Planned)
- Graph representations
- A* pathfinding with heuristics
- Topological sorting
- Minimum spanning trees

### **Week 6: Performance & Optimization** (Planned)
- Benchmarking techniques
- Memory optimization
- Cache-friendly data structures
- Parallel processing basics

---

*Tags: #daily-study #moc #learning-progression #rust-fundamentals #collections #algorithms #competitive-programming #3-track-system*

*Links: [[zettel-index]] | [[Missions MOC]] | [[Collections MOC]] | [[AoC Patterns MOC]] | [[Rust Concepts MOC]] | [[../MONTHLY_CALENDAR|MONTHLY_CALENDAR]]*
