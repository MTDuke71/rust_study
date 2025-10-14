# 📚 Collections MOC - Data Structures & Algorithms

**Comprehensive map of all collection-related learning across the workspace**

## 🏗️ Core Data Structures

### **Hash-Based Collections**
- [[HashMap Internals]] - Hash table implementation details
- [[HashSet Applications]] - Unique collection use cases
- [[Collision Resolution]] - Chaining vs open addressing
- [[Hash Function Design]] - Creating good hash functions
- [[Predicates and Higher-Order Functions Deep Dive]] - Filtering and searching with predicates

### **Tree-Based Collections** 
- [[BTreeMap Patterns]] - Ordered key-value storage
- [[BTreeSet Operations]] - Ordered unique collections
- [[Tree Traversal]] - In-order, pre-order, post-order

### **Sequential Collections**
- [[Vec Patterns]] - Dynamic arrays and capacity management
- [[VecDeque Usage]] - Double-ended queue operations
- [[LinkedList Design]] - When to use linked structures

## 🎯 Mission Implementations

### **Mission 4: Linked Lists**
- [[../../tutorials/Mission4_tut/README]] - Complete linked list tutorial
- [[../../tutorials/Mission4_tut/examples/README]] - Step-by-step runnable examples
- [[../../tutorials/Mission4_tut/compilation_stages/VISUAL_COMPILATION_PROCESS]] - Visual compilation: Rust → LLVM → Assembly → Machine Code
- [[../../tutorials/Mission4_tut/solutions/solutions]] - Complete exercise solutions with Rc/RefCell patterns
- Box<T> vs Rc<RefCell<T>> patterns
- Interior mutability and shared ownership
- 7 progressive examples from basic to performance analysis

### **Mission 5: Hash Collections**
- [[Mission5 HashMap]] - Custom hash table from scratch
- [[Mission5 HashSet]] - Set wrapper implementation
- [[Mission5 Iterator]] - Custom iteration patterns
- [[Mission5 Testing Strategy]] - Requirement-based validation
- [[Predicates and Higher-Order Functions Deep Dive]] - Advanced HashMap operations with predicates

### **Mission 1: Stack** → [[Stack Implementation Patterns]]
- **Application**: [[../../advanced_examples/Brackets_Ext/README_EXTENDED|Brackets Extended Validator]] - Advanced bracket validation with configurable alphabet and multi-error reporting

### **Mission 2: Queue & Ring Buffer**
- **Tutorial**: [[../../tutorials/Mission2_tut/README]] - Progressive queue learning with 7 comprehensive steps
- Ring buffer algorithms and circular indexing
- FIFO queue implementation with Option<T> patterns
- Linked queue with raw pointer optimization
- **Basic Implementation**: [[../../advanced_examples/Brackets_Basic/README_BASIC]] - V-Cycle bracket validation with requirements traceability
- **Extended Features**: [[../../advanced_examples/Brackets_Ext/README (2)]] - Advanced bracket validation with comprehensive error handling

### **Mission 2: Queue** → [[Queue Implementation Strategies]] 
### **Mission 6: Grids** → [[2D Array Navigation Patterns]]
- [[../../missions/Mission6/SIZE_HINT_EXPLAINED]] - Iterator optimization for grid traversal

## 📖 Daily Study Integration

### **Week 2: Collections Mastery**
- [[Day 08 - Vec Fundamentals]] - Dynamic arrays, capacity vs length
- [[Day 09 - String Patterns]] - String vs &str, UTF-8 handling
  - **Pattern Matching**: [[../../tutorials/Mission5_tut/REGEX_QUICK_REFERENCE|Regex Quick Reference]] - Comprehensive regex guide
- [[Day 10 - HashMap Basics]] - Key-value storage, borrowing keys  
- [[Day 11 - HashSet Operations]] - Unique collections, set operations
- [[Day 12 - BTreeMap]] - Ordered collections, range queries
- [[Day 13 - Advanced Iterators]] - Transforming and processing
- [[../../missions/Mission6/SIZE_HINT_EXPLAINED]] - Iterator size optimization

## 🔄 Iterator Patterns

### **Core Iterator Concepts**
- [[Day 13 - Advanced Iterators]] - Iterator adaptors and chains
- [[../../missions/Mission6/SIZE_HINT_EXPLAINED]] - size_hint() for performance optimization
- [[../../missions/Mission6/WHEN_SIZE_HINT_CALLED]] - When size_hint() is actually invoked
- [[Mission5 Iterator]] - Custom iteration patterns

## 🔗 Cross-Concept Connections

### **Ownership & Collections**
- [[Ownership in HashMap]] - Key and value borrowing patterns
- [[Clone vs Move]] - When collections take ownership
- [[Interior Mutability]] - RefCell in collections

### **Performance Patterns**
- [[O(1) vs O(log n)]] - HashMap vs BTreeMap trade-offs  
- [[Memory Layout]] - Contiguous vs fragmented storage
- [[Cache Efficiency]] - Access pattern optimization
- [[../../missions/Mission6/SIZE_HINT_EXPLAINED]] - Iterator size hints for allocation optimization
- [[../../missions/Mission6/WHEN_SIZE_HINT_CALLED]] - Understanding when optimizations trigger

### **AoC Applications**
- [[AoC 2015 MOC]] - Advent of Code 2015 solutions (8 days completed)
  - Day 3: HashSet for coordinate tracking
  - Day 7: HashMap memoization patterns
  - Day 8: String parsing and character counting
- [[Frequency Counting]] - Character/word occurrence patterns
- [[Deduplication]] - Removing duplicates efficiently with HashSet, sort+dedup, and order-preserving strategies
- [[Set Operations]] - Union, intersection, difference
- [[Memoization]] - Caching computation results
- [[Error Handling Deep Dive]] - Robust input parsing and validation
- [[Predicates and Higher-Order Functions Deep Dive]] - Filtering and validation patterns

## 🧪 Tutorial Progression

### **Mission5_tut Learning Path**
- [[Step 1 - Basic HashMap]] → HashMap structure setup
- [[Step 2 - Collision Handling]] → Chaining implementation
- [[Step 3 - Advanced Operations]] → Complex HashMap methods
- [[Step 4 - Multi-Value Patterns]] → One-to-many mappings
- [[Step 5 - MemoCache Integration]] → Performance optimization

## 📊 Assessment & Practice

- [[Collections Performance Quiz]] - Big-O analysis questions
- [[Implementation Challenges]] - Coding exercises
- [[AoC Collection Problems]] - Real-world applications

---
*Tags: #collections #overview #data-structures #cross-track #algorithms #performance*
*Links: [[zettel-index]] | [[Missions MOC]] | [[Rust Concepts MOC]]*