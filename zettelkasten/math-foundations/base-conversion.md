# Base Conversion

**Field**: Number Theory / Discrete Mathematics

**Prerequisites**: [[math-foundations/number-theory-basics]] (divisibility, modular arithmetic)

---

## Definition

**Positional notation** represents numbers as weighted sums of digits, where each digit's weight is determined by its position and the base (radix):

$$n = \sum_{i=0}^{k} d_i \cdot b^i$$

where `b` is the base, `d_i` are the digits, and `k` is the number of digits minus one.

**Intuition**: Just like decimal uses powers of 10 (ones, tens, hundreds...), any base `b` uses powers of `b`. The digit at each position tells you "how many of this power."

---

## Key Concepts

### **Standard Positional Systems**

| Base | Name | Digits | Common Use |
|------|------|--------|------------|
| 2 | Binary | 0-1 | Hardware, bitwise ops |
| 8 | Octal | 0-7 | Unix permissions |
| 10 | Decimal | 0-9 | Human default |
| 16 | Hexadecimal | 0-F | Memory addresses, colors |

### **Balanced Positional Systems**

In a **balanced** base-`b` system (where `b` is odd), digits range from `-(b-1)/2` to `+(b-1)/2` instead of `0` to `b-1`.

| Base | Name | Digits | Example |
|------|------|--------|---------|
| 3 | Balanced ternary | -1, 0, 1 | Soviet Setun computer |
| 5 | Balanced quinary (SNAFU) | -2, -1, 0, 1, 2 | AoC 2022 Day 25 |

**Key property**: No separate sign needed. Negative numbers naturally have leading negative digits.

### **Conversion: Base-N to Decimal (Horner's Method)**

Process digits left-to-right:

```
accumulator = 0
for each digit (left to right):
    accumulator = accumulator * base + digit_value
```

This is equivalent to the polynomial evaluation but avoids computing `base^k` explicitly. Each step is O(1), total O(d) where d = digit count.

**Example** (SNAFU `2=-01` → decimal):
```
acc = 0
'2': acc = 0 * 5 + 2   =    2
'=': acc = 2 * 5 + (-2) =    8
'-': acc = 8 * 5 + (-1) =   39
'0': acc = 39 * 5 + 0   =  195
'1': acc = 195 * 5 + 1  =  976
```

### **Conversion: Decimal to Standard Base-N**

Repeated division with remainder collection:

```
while n > 0:
    digit = n % base
    n = n / base
    emit digit
reverse collected digits
```

Each remainder becomes a digit (least-significant first). O(d) divisions.

### **Conversion: Decimal to Balanced Base-N**

Same as standard, but with **carry propagation** when a remainder exceeds the maximum allowed digit:

```
while n != 0:
    remainder = n % base
    n = n / base
    if remainder > max_digit:     // max_digit = (base-1)/2
        digit = remainder - base  // negative digit
        n += 1                    // carry +1
    else:
        digit = remainder
    emit digit
reverse collected digits
```

**Why the carry works**: When `remainder = 3` in base 5 (max digit = 2):
- `3 = 5 × 1 + (-2)` → emit digit `-2`, carry `+1`
- The carry adds one unit of the next-higher power of 5
- Net effect: `-2 + 5 = 3` ✓

**Worked example** (decimal 4890 → SNAFU):
```
4890 % 5 = 0 → digit '0'
 978 % 5 = 3 → digit '=' (-2), carry +1 → n=196
 196 % 5 = 1 → digit '1'
  39 % 5 = 4 → digit '-' (-1), carry +1 → n=8
   8 % 5 = 3 → digit '=' (-2), carry +1 → n=2
   2 % 5 = 2 → digit '2'

Reverse: "2=-1=0" ✓ (verify: 2×3125 - 2×625 - 125 + 25 - 10 = 4890)
```

---

## Properties and Theorems

### **Uniqueness of Representation**

Every non-negative integer has a unique representation in any standard base `b >= 2`. Every integer (positive, negative, or zero) has a unique representation in any balanced base.

### **Digit Count**

The number of digits needed to represent `n` in base `b`:
- `d = floor(log_b(n)) + 1`
- In base 5: `d = floor(log(n)/log(5)) + 1`

### **Generalized Carry Rule**

For balanced base `b` with max digit `m = (b-1)/2`:
- If `remainder <= m`: use digit as-is
- If `remainder > m`: digit becomes `remainder - b`, carry `+1`

This works because `(remainder - b) + b×1 = remainder`.

### **Range of Balanced Representations**

A balanced base-`b` number with `d` digits can represent values in the range:
- Min: `-(b^d - 1) / 2`
- Max: `+(b^d - 1) / 2`

For SNAFU (base 5) with 1 digit: -2 to +2. With 2 digits: -12 to +12.

---

## Rust Implementations

### **AoC 2022 Day 25**: Full of Hot Air (SNAFU Numbers)
- **What**: Sum fuel requirements in balanced base-5, return result in SNAFU
- **How it uses this concept**:
  - Horner's method with `fold` for SNAFU → decimal
  - Div-mod with carry propagation for decimal → SNAFU
  - Digits `=`(-2), `-`(-1), `0`, `1`, `2` map via match
- **Link**: [[aoc2022/src/solver/day25.rs]]
- **Performance**: 6.09µs for 143 SNAFU numbers

---

## Code Examples

### **Horner's Method (Any Base)**

```rust
/// Evaluate a number string in any base using Horner's method.
fn from_base(s: &str, base: i64, digit_fn: impl Fn(char) -> i64) -> i64 {
    s.chars().fold(0i64, |acc, c| acc * base + digit_fn(c))
}

// Standard bases
let binary = from_base("1101", 2, |c| c.to_digit(2).unwrap() as i64);  // 13
let hex = from_base("FF", 16, |c| c.to_digit(16).unwrap() as i64);     // 255

// Balanced base-5 (SNAFU)
let snafu = from_base("2=-01", 5, |c| match c {
    '=' => -2, '-' => -1, '0' => 0, '1' => 1, '2' => 2, _ => panic!()
});  // 976
```

### **Decimal to Balanced Base-5 (SNAFU)**

```rust
fn decimal_to_snafu(mut n: i64) -> String {
    if n == 0 { return "0".to_string(); }
    let mut digits = Vec::new();
    while n != 0 {
        let rem = n % 5;
        n /= 5;
        let (digit, carry) = match rem {
            0 => ('0', 0),
            1 => ('1', 0),
            2 => ('2', 0),
            3 => ('=', 1),  // -2 + carry: 3 = 5×1 + (-2)
            4 => ('-', 1),  // -1 + carry: 4 = 5×1 + (-1)
            _ => unreachable!(),
        };
        digits.push(digit);
        n += carry;
    }
    digits.iter().rev().collect()
}
```

### **Generalized Balanced Conversion**

```rust
/// Convert decimal to any balanced odd base.
/// For base b, digits range from -(b-1)/2 to +(b-1)/2.
fn to_balanced_base(mut n: i64, base: i64) -> Vec<i64> {
    assert!(base % 2 == 1, "balanced bases must be odd");
    let max_digit = (base - 1) / 2;
    let mut digits = Vec::new();
    while n != 0 {
        let mut rem = n % base;
        n /= base;
        if rem > max_digit {
            rem -= base;
            n += 1;
        }
        digits.push(rem);
    }
    digits.reverse();
    digits
}

// Balanced ternary: 42 → [1, -1, -1, 0] → 27 - 9 - 3 + 0 = 15...
// Actually: 42 = 27 + 9 + 3 + 3 → [1, 1, -1, 0] → 1×27 + 1×9 + (-1)×3 + 0 = 33
// Let's trace: 42%3=0→0, 14%3=2→-1,carry→5, 5%3=2→-1,carry→2, 2%3=2→-1,carry→1, 1%3=1→1
// Digits reversed: [1, -1, -1, -1, 0] = 81-27-9-3 = 42 ✓
```

---

## Related Concepts

- **Prerequisites**: [[math-foundations/number-theory-basics]] (modular arithmetic, division)
- **Related**: [[math-foundations/modular-arithmetic]] (remainder operations)
- **Applications**: [[math-foundations/palindromes]] (digit extraction), AoC encoding problems

---

## Historical Context

- **Balanced ternary**: Used in the Soviet Setun computer (1958). Advantages include natural negation (flip all digits) and rounding by truncation.
- **SNAFU (balanced quinary)**: AoC 2022 Day 25's fictional system. The name is a humorous backronym: "Special Numeral-Analogue Fuel Units."
- **Hexadecimal**: Became standard because 16 = 2^4, making each hex digit a 4-bit nibble.

---

*Tags: #mathematics #number-theory #base-conversion #positional-notation #balanced-numbers*

*Created*: 2026-02-25
*Last Updated*: 2026-02-25
*Implementations*: 1 (AoC 2022 Day 25)
