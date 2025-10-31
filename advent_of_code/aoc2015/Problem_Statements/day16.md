--- Day 16: Aunt Sue ---
Your Aunt Sue has given you a wonderful gift, and you'd like to send her a thank you card. However, there's a small problem: she signed it "From, Aunt Sue".

You have 500 Aunts named "Sue".

So, to avoid sending the card to the wrong person, you need to figure out which Aunt Sue (which you conveniently number 1 to 500, for sanity) gave you the gift. You open the present and, as luck would have it, good ol' Aunt Sue got you a My First Crime Scene Analysis Machine! Just what you wanted. Or needed, as the case may be.

The My First Crime Scene Analysis Machine (MFCSAM for short) can detect a few specific compounds in a given sample, as well as how many distinct kinds of those compounds there are. According to the instructions, these are what the MFCSAM can detect:

children, by human DNA age analysis.
cats. It doesn't differentiate individual breeds.
Several seemingly random breeds of dog: samoyeds, pomeranians, akitas, and vizslas.
goldfish. No other kinds of fish.
trees, all in one group.
cars, presumably by exhaust or gasoline or something.
perfumes, which is handy, since many of your Aunts Sue wear a few kinds.
In fact, many of your Aunts Sue have many of these. You put the wrapping from the gift into the MFCSAM. It beeps inquisitively at you a few times and then prints out a message on ticker tape:

children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1
You make a list of the things you can remember about each Aunt Sue. Things missing from your list aren't zero - you simply don't remember the value.

What is the number of the Sue that got you the gift?

Your puzzle answer was 40.

The first half of this puzzle is complete! It provides one gold star: *

--- Part Two ---
As you're about to send the thank you note, something in the MFCSAM's instructions catches your eye. Apparently, it has an outdated retroencabulator, and so the output from the machine isn't exact values - some of them indicate ranges.

In particular, the cats and trees readings indicates that there are greater than that many (due to the unpredictable nuclear decay of cat dander and tree pollen), while the pomeranians and goldfish readings indicate that there are fewer than that many (due to the modial interaction of magnetoreluctance).

What is the number of the real Aunt Sue?

Your puzzle answer was 241.

Both parts of this puzzle are complete! They provide two gold stars: **

---

## Implementation Notes

### Algorithm Approach

**Part 1: Exact Matching**
- Parse 500 Aunt Sue records, each with 3 known properties
- Build target HashMap from MFCSAM results (10 properties)
- Linear search: check each aunt's known properties against target
- Return first aunt where all known properties exactly match target values

**Part 2: Range-Based Matching**
- Same parsing and target construction as Part 1
- Different comparison rules based on property type:
  - **cats, trees**: Aunt's value must be **> target** (greater than)
  - **pomeranians, goldfish**: Aunt's value must be **< target** (fewer than)
  - **All others**: Aunt's value must be **== target** (exact match)
- Linear search with custom matching logic

### Data Structures

**`AuntSue` struct**:
```rust
struct AuntSue {
    number: usize,                      // Sue ID (1-500)
    properties: HashMap<String, usize>, // Dynamic property storage (3 properties per aunt)
}
```

**Why HashMap?**
- Each aunt has exactly 3 out of 10 possible properties
- Unknown properties ≠ zero (just not remembered)
- HashMap allows sparse representation: only store known values
- O(1) lookup for property existence and value checks

**MFCSAM Target**:
```rust
HashMap<String, usize> with 10 entries:
- children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0
- vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1
```

### Rust-Specific Patterns

**1. Pattern Matching on String Keys**:
```rust
match key.as_str() {
    "cats" | "trees" => { /* greater than logic */ }
    "pomeranians" | "goldfish" => { /* fewer than logic */ }
    _ => { /* exact match logic */ }
}
```

**2. HashMap Iteration with Conditional Matching**:
```rust
for (key, value) in &self.properties {
    if let Some(&target_value) = target.get(key) {
        // Compare based on property type
    }
}
```

**3. Early Return for Non-Matches**:
```rust
if *value != target_value {
    return false;  // Short-circuit on first mismatch
}
```

**4. String Parsing with `split()` and `strip_prefix()`**:
```rust
let sue_part = parts[0].trim();
let number = sue_part
    .strip_prefix("Sue ")
    .and_then(|s| s.parse::<usize>().ok())
    .unwrap_or(0);
```

**5. Dynamic Property Addition**:
```rust
pub fn add_property(&mut self, key: String, value: usize) {
    self.properties.insert(key, value);
}
```

### Performance Characteristics

- **Time Complexity**: O(n × m) where n = 500 aunts, m = 3 properties per aunt
- **Space Complexity**: O(n × m) for storing all aunt data
- **Search**: Linear scan stops at first match (average case: scan ~50-250 aunts)
- **Part 1 Result**: Found at Sue #40 (early termination)
- **Part 2 Result**: Found at Sue #241 (mid-range termination)

### Key Insights

**The Retroencabulator Effect**:
- Part 1: All properties are exact measurements
- Part 2: Some properties are **range indicators**:
  - Nuclear decay → readings are **lower bounds** (actual > reading)
  - Magnetoreluctance → readings are **upper bounds** (actual < reading)

**Why Different Aunts?**:
- Sue #40 has exact matches for all known properties
- Sue #241 satisfies range constraints but wouldn't match exact comparison
- Example: Sue #241 might have cats: 8 (> 7) where Sue #40 has cats: 7 (== 7)

### Test Coverage (11 tests)

1. **Parsing**: Multi-line input with property extraction
2. **Part 1 Matching**: Exact value comparison, subset matching
3. **Part 2 Greater-Than**: cats, trees must exceed target
4. **Part 2 Fewer-Than**: pomeranians, goldfish must be below target
5. **Part 2 Exact**: Other properties must match exactly
6. **Edge Cases**: Equal values fail for range properties

### Results

- **Part 1**: Sue #40 (exact matching)
- **Part 2**: Sue #241 (range-based matching)

---

## Zettelkasten Links

### Outgoing Links (Concepts Used)
- [[../../../zettelkasten/hashmap-sparse-storage|HashMap Sparse Storage]] - Efficient storage of partial data
- [[../../../zettelkasten/pattern-matching-strings|Pattern Matching on Strings]] - match on key.as_str()
- [[../../../zettelkasten/early-return-optimization|Early Return Optimization]] - Short-circuit on first mismatch
- [[../../../zettelkasten/string-parsing-patterns|String Parsing Patterns]] - strip_prefix(), split(), parse()
- [[../../../zettelkasten/conditional-comparison|Conditional Comparison Logic]] - Different rules for different properties
- [[../../../zettelkasten/range-based-matching|Range-Based Matching]] - Greater-than, less-than, equals comparisons

### Incoming Links (Related Problems)
- [[summary|Problem Summary]] - Day 16 entry
- [[../../../zettelkasten/aoc-collection-problems|AoC Collection Problems]]
- [[../../../zettelkasten/aoc-filtering-problems|AoC Filtering Problems]] - Find item matching criteria

### Related Concepts
- [[../../../zettelkasten/partial-information-matching|Partial Information Matching]] - Matching with incomplete data
- [[../../../zettelkasten/rust-hashmap-usage|Rust HashMap Usage]] - Dynamic property storage
- [[../../../zettelkasten/linear-search-early-termination|Linear Search with Early Termination]]
- [[../../../zettelkasten/multi-criteria-filtering|Multi-Criteria Filtering]] - Multiple property checks

### Learning Path
- **Prerequisites**: [[../../../zettelkasten/rust-basics-hashmap|HashMap Basics]], [[../../../zettelkasten/string-parsing|String Parsing]], [[../../../zettelkasten/pattern-matching|Pattern Matching]]
- **Next Steps**: [[../../../zettelkasten/constraint-satisfaction|Constraint Satisfaction Problems]], [[../../../zettelkasten/rule-based-systems|Rule-Based Systems]]
- **Related Missions**: [[../../../missions/Mission5/README|Mission 5 - HashMap]] (Dictionary data structure)

---

*Tags: #aoc-2015 #day16 #hashmap #pattern-matching #filtering #partial-matching #range-comparison #string-parsing #linear-search #constraint-satisfaction*
