# Day 7: Camel Cards - Function-by-Function Walkthrough

This guide provides a detailed explanation of every type, function, and implementation in the Day 7 solution. Use this as a reference while reading through the code.

---

## 📋 Table of Contents
1. [Type Definitions](#type-definitions)
2. [Part 1 Implementation](#part-1-implementation)
3. [Part 2 Implementation](#part-2-implementation)
4. [Public API](#public-api)
5. [Design Patterns](#design-patterns)

---

## Type Definitions

### `enum Card` (Part 1)
**Location**: Lines 30-47  
**Purpose**: Represents individual playing cards with their natural ordering for Part 1

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Card {
    Two = 2,
    Three = 3,
    // ...
    Ace = 14,
}
```

**Key Design Decisions**:
- **Numeric values**: Each variant has explicit value (2-14) matching card strength
- **Derived traits**: 
  - `Ord`: Natural ordering (Two < Three < ... < Ace)
  - `Hash`: Can be used as HashMap key for frequency counting
  - `Copy`: Cheap to pass around (just an integer under the hood)
- **Why this works**: Rust's derived `Ord` for enums with explicit values compares by numeric value

**Usage**: Frequency counting in `determine_type()`, card-by-card comparison in `Hand::cmp()`

---

### `Card::from_char(c: char) -> Option<Self>`
**Location**: Lines 49-66  
**Purpose**: Parse a single character into a Card

**Algorithm**:
```
Input: Single char ('2'-'9', 'T', 'J', 'Q', 'K', 'A')
Output: Option<Card> (Some(card) or None for invalid input)

1. Match on character
2. Return corresponding Card variant
3. Return None for invalid characters
```

**Error Handling**: Returns `Option` - caller decides how to handle invalid input

**Why `Option` not `Result`**: No additional error context needed - invalid char is self-explanatory

**Example**:
```rust
Card::from_char('A') // Some(Card::Ace)
Card::from_char('5') // Some(Card::Five)
Card::from_char('X') // None
```

---

### `enum Card2` (Part 2)
**Location**: Lines 68-105  
**Purpose**: Card representation for Part 2 where J = Joker (weakest)

```rust
enum Card2 {
    Joker = 1,  // J is now weakest!
    Two = 2,
    // ... (no Jack variant)
    Ace = 14,
}
```

**Key Differences from `Card`**:
- `Joker = 1` instead of `Jack = 11` - changes tie-breaking order
- No `Jack` variant - 'J' character maps to `Joker`
- Same derived traits for consistency

**Why a separate enum?** Clean separation of Part 1/Part 2 logic. Could have used a single enum with a flag, but separate types are clearer and prevent mixing Part 1/Part 2 hands.

---

### `enum HandType`
**Location**: Lines 107-115  
**Purpose**: Poker hand rankings from weakest to strongest

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}
```

**Why explicit values?** Makes natural ordering match poker rules - derived `Ord` will compare numerically.

**Usage**: Primary comparison in `Hand::cmp()` - hands are sorted by type first, then by cards.

---

## Part 1 Implementation

### `struct Hand`
**Location**: Lines 117-124  
**Purpose**: Represents a complete poker hand for Part 1

```rust
struct Hand {
    cards: [Card; 5],      // Exactly 5 cards (array, not Vec)
    bid: u64,               // Bid amount
    hand_type: HandType,    // Cached hand type
}
```

**Design Decisions**:
- **`[Card; 5]` not `Vec<Card>`**: Fixed size known at compile time - more efficient, no heap allocation
- **Cached `hand_type`**: Computed once in constructor, reused in comparisons
- **No `Default`**: Hand must be created with valid data via `new()` or `parse()`

---

### `Hand::new(cards: [Card; 5], bid: u64) -> Self`
**Location**: Lines 126-133  
**Purpose**: Constructor that computes and caches the hand type

```rust
fn new(cards: [Card; 5], bid: u64) -> Self {
    let hand_type = Self::determine_type(&cards);  // Compute once
    Hand { cards, bid, hand_type }
}
```

**Why cache hand_type?**
- Hands are compared many times during sorting (O(n log n) comparisons)
- Hand type detection is O(5) work
- Cache makes each comparison O(1) for type check

**Tradeoff**: Slightly more memory (1 byte) vs repeated computation

---

### `Hand::determine_type(cards: &[Card; 5]) -> HandType`
**Location**: Lines 135-152  
**Purpose**: Core algorithm - determine poker hand type from 5 cards

**Algorithm**:
```
1. Count frequency of each card → HashMap<Card, usize>
2. Extract just the frequencies (ignore which cards) → Vec<usize>
3. Sort frequencies descending
4. Pattern match on frequency distribution
```

**Example Walkthrough**:
```rust
// Input: [A, A, 8, A, A] (four Aces, one 8)

// Step 1: Frequency counting
let mut counts = HashMap::new();
// counts = {A: 4, 8: 1}

// Step 2: Extract frequencies
let frequencies = [4, 1];  // (sorted descending)

// Step 3: Pattern match
match [4, 1] {
    [4, 1] => FourOfAKind  // ✓ Match!
}
```

**Why frequencies work?**
- Hand type depends only on frequency pattern, not which cards
- `[5]` = five of a kind (all same)
- `[4, 1]` = four of a kind (four same, one different)
- `[3, 2]` = full house (three same, two same)
- `[3, 1, 1]` = three of a kind (three same, two different)
- `[2, 2, 1]` = two pair (two same, two same, one different)
- `[2, 1, 1, 1]` = one pair (two same, three different)
- `[1, 1, 1, 1, 1]` = high card (all different)

**Mission 5 Integration**: Uses HashMap with Entry API pattern:
```rust
*counts.entry(card).or_insert(0) += 1;
// Single lookup instead of:
// if let Some(count) = counts.get_mut(&card) { *count += 1; }
// else { counts.insert(card, 1); }
```

---

### `Hand::parse(line: &str) -> Option<Self>`
**Location**: Lines 154-177  
**Purpose**: Parse a line of input into a Hand

**Algorithm**:
```
Input: "32T3K 765" (5 cards, space, bid)

1. Split on whitespace → ["32T3K", "765"]
2. Validate: exactly 2 parts
3. Parse bid as u64
4. Validate: exactly 5 characters in card string
5. Parse each char → Card
6. Validate: all chars parsed successfully
7. Create Hand with new()
```

**Error Handling**: Returns `Option<Self>`
- `None` if parsing fails at any step
- Uses `?` operator to propagate None through the chain

**Example**:
```rust
Hand::parse("AAAAA 100")  // Some(Hand { cards: [A,A,A,A,A], bid: 100, ... })
Hand::parse("AAAA 100")   // None (only 4 cards)
Hand::parse("AAXAA 100")  // None (X invalid)
Hand::parse("AAAAA")      // None (no bid)
```

**Why `filter_map` in parsing?**
```rust
let cards: Vec<Card> = cards_str.chars()
    .filter_map(Card::from_char)  // Parse and filter in one step
    .collect();
// Removes invalid chars automatically
// If any char invalid, vec length < 5, which we check
```

---

### `impl Ord for Hand`
**Location**: Lines 179-196  
**Purpose**: Define how hands compare (enables sorting with `.sort()`)

**Algorithm**: Two-level comparison
```
1. Compare hand types (primary)
   - Five of a kind > Four of a kind > ... > High card
   
2. If types equal, compare cards left-to-right (tiebreaker)
   - Compare cards[0] (if equal, compare cards[1], etc.)
   - First difference determines winner
```

**Code Walkthrough**:
```rust
fn cmp(&self, other: &Self) -> Ordering {
    match self.hand_type.cmp(&other.hand_type) {
        Ordering::Equal => {
            // Types equal - tiebreaker needed
            for i in 0..5 {
                match self.cards[i].cmp(&other.cards[i]) {
                    Ordering::Equal => continue,  // This card same, try next
                    other => return other,         // Found difference!
                }
            }
            Ordering::Equal  // All cards same (shouldn't happen in practice)
        }
        other => other,  // Different types - return that result
    }
}
```

**Example**:
```rust
// Both four of a kind - need tiebreaker
Hand("33332") vs Hand("2AAAA")
// Types equal → compare cards
// cards[0]: Three vs Two → Three > Two
// Result: Hand("33332") > Hand("2AAAA")
```

**Why implement `Ord`?**
- Enables `.sort()` on `Vec<Hand>`
- Type-safe - can't accidentally compare incompatible types
- Self-documenting - comparison logic in one place

---

### `impl PartialOrd for Hand`
**Location**: Lines 198-202  
**Purpose**: Required by Rust - defines partial ordering

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))  // Always succeeds - total ordering
}
```

**Why needed?** Rust's trait system requires `PartialOrd` to implement `Ord`. For types with total ordering (all values can be compared), just delegate to `cmp()`.

---

## Part 2 Implementation

### `struct Hand2`
**Location**: Lines 204-211  
**Purpose**: Hand representation for Part 2 (identical structure to `Hand`)

**Why duplicate?** Clean separation - Part 1 and Part 2 use different card types and logic. Prevents accidentally mixing them.

**Alternative approaches**:
```rust
// ❌ Generic: Complicated, shared logic not significant
struct Hand<C: CardType> { cards: [C; 5], ... }

// ❌ Enum: Match statements everywhere
enum Hand { Part1(...), Part2(...) }

// ✅ Separate types: Clear, simple, type-safe
struct Hand { ... }
struct Hand2 { ... }
```

---

### `Hand2::determine_type(cards: &[Card2; 5]) -> HandType`
**Location**: Lines 220-251  
**Purpose**: Determine hand type with Joker wildcard logic

**Algorithm**: Greedy joker assignment
```
1. Count frequency of each non-Joker card
2. Count Jokers separately
3. Special case: All jokers → Five of a kind
4. Add all jokers to the MOST FREQUENT non-joker card
5. Pattern match on resulting frequencies
```

**Why greedy is optimal?**
- Adding jokers to most frequent card always produces strongest hand
- Example: `[Q, J, J, Q, 2]`
  - Frequencies: Q=2, 2=1, Jokers=2
  - Add jokers to Q: Q=4, 2=1 → Four of a kind ✓
  - If added to 2: Q=2, 2=3 → Full house ✗ (weaker)

**Code Walkthrough**:
```rust
fn determine_type(cards: &[Card2; 5]) -> HandType {
    let mut counts: HashMap<Card2, usize> = HashMap::new();
    let mut joker_count = 0;

    // Step 1: Count non-jokers, track jokers separately
    for &card in cards {
        if card == Card2::Joker {
            joker_count += 1;
        } else {
            *counts.entry(card).or_insert(0) += 1;
        }
    }

    // Step 2: Special case - all jokers
    if joker_count == 5 {
        return HandType::FiveOfAKind;  // JJJJJ → best hand!
    }

    // Step 3: Extract frequencies and sort
    let mut frequencies: Vec<usize> = counts.values().copied().collect();
    frequencies.sort_by(|a, b| b.cmp(a));  // Descending
    
    // Step 4: Greedy - add jokers to most frequent
    frequencies[0] += joker_count;  // frequencies[0] is largest

    // Step 5: Pattern match as before
    match frequencies.as_slice() { ... }
}
```

**Example with jokers**:
```rust
// Input: [T, 5, 5, J, 5] → "T55J5"
// Step 1: counts = {T: 1, 5: 3}, joker_count = 1
// Step 2: Not all jokers, continue
// Step 3: frequencies = [3, 1]  (sorted descending)
// Step 4: frequencies[0] += 1 → [4, 1]
// Step 5: [4, 1] → FourOfAKind  (J acts as another 5)
```

**Edge cases handled**:
- All jokers: `JJJJJ` → Five of a kind
- Some jokers: Add to most frequent
- No jokers: Normal frequency counting
- Ties: Adding to any tied-for-most-frequent gives same result

---

### `impl Ord for Hand2`
**Location**: Lines 278-295  
**Purpose**: Same comparison logic as Part 1, but with `Card2` ordering

**Key Difference**: J (Joker) is now weakest in tie-breaking
```rust
// Part 1: J=11 (middle strength)
Card::Jack = 11

// Part 2: J=1 (weakest)
Card2::Joker = 1
```

**Example**:
```rust
// Both four of a kind
Hand2("JKKK2") vs Hand2("QQQQ2")
// Types equal → compare cards
// cards[0]: Joker(1) vs Queen(12) → Joker < Queen
// Result: Hand2("QQQQ2") > Hand2("JKKK2")
```

**Why same code?** The numeric values in `Card2` enum handle the difference - no special logic needed!

---

## Public API

### `solve_part1(input: &str) -> Result<String>`
**Location**: Lines 307-322  
**Purpose**: Main entry point for Part 1

**Algorithm**:
```
1. Parse all hands from input (filter_map handles errors)
2. Sort hands from weakest to strongest
3. Calculate winnings: rank × bid for each hand
4. Sum total winnings
```

**Code Walkthrough**:
```rust
pub fn solve_part1(input: &str) -> Result<String> {
    // Step 1: Parse (filter_map removes None results)
    let mut hands: Vec<Hand> = input
        .lines()
        .filter_map(Hand::parse)  // Parse each line, skip failures
        .collect();

    // Step 2: Sort (uses our Ord implementation)
    hands.sort();  // Weakest first

    // Step 3 & 4: Calculate and sum
    let total_winnings: u64 = hands
        .iter()
        .enumerate()  // (index, hand)
        .map(|(index, hand)| {
            let rank = (index + 1) as u64;  // Rank 1 for weakest
            rank * hand.bid
        })
        .sum();  // Sum all winnings

    Ok(total_winnings.to_string())
}
```

**Why rank = index + 1?**
- After sorting, weakest hand is at index 0
- Weakest hand should have rank 1 (not 0)
- Therefore: rank = index + 1

**Example**:
```
Input:
  32T3K 765  → Rank 1 (weakest) → 1 × 765 = 765
  KK677 28   → Rank 2          → 2 × 28 = 56
  ...
  
Total: 765 + 56 + ... = 6440
```

**Error Handling**: Returns `Result<String>` for consistency with other days, though this implementation doesn't actually error.

---

### `solve_part2(input: &str) -> Result<String>`
**Location**: Lines 324-341  
**Purpose**: Main entry point for Part 2 (identical structure to Part 1)

**Key Differences**:
- Uses `Hand2::parse()` instead of `Hand::parse()`
- Uses `Hand2` ordering (Joker is weakest)
- Hand type detection uses joker wildcard logic

**Otherwise identical**: Same parsing → sorting → ranking → summing pipeline

---

## Design Patterns

### Pattern 1: Type-Driven Design
**Where**: `Card` vs `Card2`, `Hand` vs `Hand2`

**Philosophy**: Use Rust's type system to prevent bugs
- Can't accidentally mix Part 1 and Part 2 hands
- Compiler enforces correct usage
- No runtime checks needed

**Alternative**: Single type with flags/enums
```rust
// ❌ Runtime checking required
struct Hand {
    cards: Vec<Card>,
    part: Part,  // Part1 or Part2
}

impl Hand {
    fn cmp(&self, other: &Self) {
        if self.part != other.part {
            panic!("Can't compare different parts!");
        }
        // ...
    }
}

// ✅ Compile-time checking
struct Hand { ... }
struct Hand2 { ... }
// Can't even call Hand::cmp(Hand2) - won't compile!
```

---

### Pattern 2: Enum Ordering with Explicit Values
**Where**: `Card`, `Card2`, `HandType`

**Why**: Makes derived `Ord` work naturally
```rust
#[derive(PartialOrd, Ord)]
enum HandType {
    HighCard = 1,    // Weakest
    OnePair = 2,
    // ...
    FiveOfAKind = 7, // Strongest
}

// Derived Ord compares by numeric value
// HighCard(1) < OnePair(2) < ... < FiveOfAKind(7)
```

**Alternative**: Custom Ord implementation
```rust
// ❌ More code, same result
impl Ord for HandType {
    fn cmp(&self, other: &Self) {
        match (self, other) {
            (HighCard, HighCard) => Ordering::Equal,
            (HighCard, _) => Ordering::Less,
            (_, HighCard) => Ordering::Greater,
            // ... 49 more arms!
        }
    }
}
```

---

### Pattern 3: Frequency Analysis
**Where**: `determine_type()` in both parts

**Concept**: Hand type determined by frequency distribution
```rust
// Frequencies → Hand type mapping
[5]          → Five of a kind    (AAAAA)
[4, 1]       → Four of a kind    (AAAA8)
[3, 2]       → Full house        (AAA88)
[3, 1, 1]    → Three of a kind   (AAA89)
[2, 2, 1]    → Two pair          (AA889)
[2, 1, 1, 1] → One pair          (AA892)
[1, 1, 1, 1, 1] → High card      (A8923)
```

**Why this works**: Poker hands are about patterns, not specific cards
- Don't care *which* card appears 4 times - just that *one* card appears 4 times
- Frequencies provide canonical representation

---

### Pattern 4: Custom Ord for Multi-Level Sorting
**Where**: `impl Ord for Hand` and `Hand2`

**Pattern**:
```rust
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // Level 1: Primary comparison
        match self.primary.cmp(&other.primary) {
            Ordering::Equal => {
                // Level 2: Tiebreaker
                self.tiebreaker.cmp(&other.tiebreaker)
            }
            other => other,  // Primary different - return that
        }
    }
}
```

**Benefits**:
- Clear hierarchy of comparisons
- Early termination (don't check tiebreaker if primary differs)
- Enables automatic `.sort()` usage

---

### Pattern 5: Greedy Optimization (Part 2)
**Where**: `Hand2::determine_type()`

**Theorem**: Adding all jokers to the most frequent card always produces the strongest hand

**Proof Sketch**:
- Hand strength determined by largest frequency
- Adding K jokers to frequency N → frequency (N+K)
- To maximize hand strength, maximize largest frequency
- Therefore add all jokers to current-largest frequency

**Example**:
```
Hand: [Q, Q, J, J, 2]
Frequencies: Q=2, 2=1, Jokers=2

Option 1: Add to Q → [4, 1] = Four of a kind
Option 2: Add to 2 → [2, 3] = Full house
Option 3: Split → [3, 2] = Full house

Best: Option 1 (Four of a kind > Full house)
```

**Edge case**: What if frequencies are tied?
```
Hand: [Q, Q, 2, 2, J]
Frequencies: Q=2, 2=2, Jokers=1

Add to either → [3, 2] = Full house (same result!)
```

---

## Testing Strategy

### Unit Tests
**Coverage**:
1. **Hand type detection**: All 7 hand types verified
2. **Hand ordering**: Type comparison and card tiebreaking
3. **Joker logic**: Wildcard hand types and weakest-card ordering
4. **Examples**: Part 1 (6440) and Part 2 (5905)

### Test Philosophy
- Test the algorithm, not the implementation
- Example-driven (use puzzle examples)
- Edge cases (all jokers, no jokers, ties)

---

## Performance Characteristics

### Time Complexity
- **Parsing**: O(n) where n = number of hands
- **Hand type detection**: O(1) per hand (5 cards always)
- **Sorting**: O(n log n)
- **Ranking**: O(n)
- **Total**: O(n log n) dominated by sorting

### Space Complexity
- **Input**: O(n) hands
- **Frequency map**: O(1) per hand (max 5 distinct cards)
- **Total**: O(n)

### Actual Performance
- **Part 1**: ~290µs (1001 hands)
- **Part 2**: ~435µs (slightly slower due to joker logic)

---

## Key Takeaways

1. **Type-driven design**: Separate types for Part 1/Part 2 prevents bugs
2. **Derived traits**: Explicit enum values make `Ord` work naturally
3. **Frequency analysis**: Abstract hand types to frequency patterns
4. **Multi-level comparison**: Primary + tiebreaker pattern
5. **Greedy optimization**: Adding jokers to most-frequent is provably optimal
6. **HashMap Entry API**: Mission 5 pattern for efficient counting
7. **Custom Ord**: Enables clean `.sort()` usage on complex types

---

## Follow-Up Questions

As you read the code, consider:
1. Why use `[Card; 5]` instead of `Vec<Card>`?
2. Could we use a single `Hand` type with a generic parameter?
3. What happens if we have duplicate hands (same cards and bid)?
4. How would you extend this for 7-card poker hands?
5. Could we optimize away the `hand_type` caching?

---

**Next Steps**: Try implementing 7-card poker or adding new hand types (straight, flush) to see how the frequency analysis pattern extends!
