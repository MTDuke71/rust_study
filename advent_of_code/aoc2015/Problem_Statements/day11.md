--- Day 11: Corporate Policy ---
Santa's previous password expired, and he needs help choosing a new one.

To help him remember his new password after the old one expires, Santa has devised a method of coming up with a password based on the previous one. Corporate policy dictates that passwords must be exactly eight lowercase letters (for security reasons), so he finds his new password by incrementing his old password string repeatedly until it is valid.

Incrementing is just like counting with numbers: xx, xy, xz, ya, yb, and so on. Increase the rightmost letter one step; if it was z, it wraps around to a, and repeat with the next letter to the left until one doesn't wrap around.

Unfortunately for Santa, a new Security-Elf recently started, and he has imposed some additional password requirements:

Passwords must include one increasing straight of at least three letters, like abc, bcd, cde, and so on, up to xyz. They cannot skip letters; abd doesn't count.
Passwords may not contain the letters i, o, or l, as these letters can be mistaken for other characters and are therefore confusing.
Passwords must contain at least two different, non-overlapping pairs of letters, like aa, bb, or zz.
For example:

hijklmmn meets the first requirement (because it contains the straight hij) but fails the second requirement requirement (because it contains i and l).
abbceffg meets the third requirement (because it repeats bb and ff) but fails the first requirement.
abbcegjk fails the third requirement, because it only has one double letter (bb).
The next password after abcdefgh is abcdffaa.
The next password after ghijklmn is ghjaabcc, because you eventually skip all the passwords that start with ghi..., since i is not allowed.
Given Santa's current password (your puzzle input), what should his next password be?

Your puzzle answer was hxbxxyzz.

--- Part Two ---
Santa's password expired again. What's the next one?

Your puzzle answer was hxcaabcc.

Both parts of this puzzle are complete! They provide two gold stars: **

At this point, you should return to your Advent calendar and try another puzzle.

Your puzzle input was hxbxwxba.

You can also [Share] this puzzle.

---

## 🔗 Related Resources

**Day 11 Implementation:**
- [[day11.rs|../src/solver/day11]] - Your implementation workspace
- [[day11_test.rs|../tests/day11_test]] - Comprehensive test suite (21 tests)
- [[day11_example.txt|../inputs/day11_example.txt]] - Puzzle input

**Day 11 Documentation:**
- [[DAY11_IMPLEMENTATION_GUIDE|../examples/DAY11_IMPLEMENTATION_GUIDE]] - Step-by-step guide
- [[DAY11_QUICK_REFERENCE|../examples/DAY11_QUICK_REFERENCE]] - Quick reference card
- [[DAY11_SETUP_COMPLETE|../examples/DAY11_SETUP_COMPLETE]] - Template summary

**Zettelkasten Concepts:**
- [[String Manipulation|../../../zettelkasten/String Manipulation]] - String processing in Rust
- [[Iteration Patterns|../../../zettelkasten/Iteration Patterns]] - Windowing and loops
- [[Validation Patterns|../../../zettelkasten/Validation Patterns]] - Rule-based validation
- [[TDD (Test-Driven Development)|../../../zettelkasten/Test-Driven Development]] - TDD workflow
- [[AoC 2015 MOC|../../../zettelkasten/AoC 2015 MOC]] - All 2015 problems
- [[AoC Patterns MOC|../../../zettelkasten/AoC Patterns MOC]] - Common algorithms

**Rust Book Integration:**
- [[Chapter 8|../../../rust_book/Ch8/README]] - Collections and strings
- [[Week 1 Overview|../../../zettelkasten/Week 1 Overview]] - String fundamentals

**Problem Catalog:**
- [[summary.md|summary]] - All AoC 2015 problems overview

*Tags: #aoc2015 #day11 #corporate-policy #password-generation #validation #base26 #string-processing*
