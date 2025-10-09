--- Day 10: Elves Look, Elves Say ---
Today, the Elves are playing a game called look-and-say. They take turns making sequences by reading aloud the previous sequence and using that reading as the next sequence. For example, 211 is read as "one two, two ones", which becomes 1221 (1 2, 2 1s).

Look-and-say sequences are generated iteratively, using the previous value as input for the next step. For each step, take the previous value, and replace each run of digits (like 111) with the number of digits (3) followed by the digit itself (1).

For example:

1 becomes 11 (1 copy of digit 1).
11 becomes 21 (2 copies of digit 1).
21 becomes 1211 (one 2 followed by one 1).
1211 becomes 111221 (one 1, one 2, and two 1s).
111221 becomes 312211 (three 1s, two 2s, and one 1).
Starting with the digits in your puzzle input, apply this process 40 times. What is the length of the result?

Your puzzle answer was 492982.

--- Part Two ---
Neat, right? You might also enjoy hearing John Conway talking about this sequence (that's Conway of Conway's Game of Life fame).

Now, starting again with the digits in your puzzle input, apply this process 50 times. What is the length of the new result?

Your puzzle answer was 6989950.

Both parts of this puzzle are complete! They provide two gold stars: **

At this point, you should return to your Advent calendar and try another puzzle.

Your puzzle input was 1321131112.

You can also [Share] this puzzle.

---

## 🔗 Related Resources

**Day 10 Implementation:**
- [[day10.rs|../src/solver/day10]] - Production iterative solution
- [[day10_with_memo.rs|../examples/day10_with_memo]] - Memoized recursive approach

**Day 10 Documentation:**
- [[DAY10_README|../examples/DAY10_README]] - Complete performance analysis
- [[DAY10_LEARNING_GUIDE|../examples/DAY10_LEARNING_GUIDE]] - Step-by-step implementation
- [[DAY10_BENCHMARK_ANALYSIS|../examples/DAY10_BENCHMARK_ANALYSIS]] - Iterative vs memoized
- [[DAY10_MEMOIZATION_WALKTHROUGH|../examples/DAY10_MEMOIZATION_WALKTHROUGH]] - Memoization deep dive

**Mission5 Integration:**
- [[Mission5 README|../../../missions/Mission5/README]] - Custom HashMap implementation
- [[Mission5_tut Overview|../../../zettelkasten/Mission5_tut Overview]] - HashMap tutorial

**Zettelkasten Concepts:**
- [[String Manipulation|../../../zettelkasten/String Manipulation]] - String processing patterns
- [[Iteration Patterns|../../../zettelkasten/Iteration Patterns]] - Loop techniques
- [[Memoization Patterns|../../../zettelkasten/Memoization Patterns]] - Caching strategies
- [[Performance Patterns|../../../zettelkasten/Performance Patterns]] - Optimization techniques
- [[AoC 2015 MOC|../../../zettelkasten/AoC 2015 MOC]] - All 2015 problems
- [[AoC Patterns MOC|../../../zettelkasten/AoC Patterns MOC]] - Common algorithms

**Problem Catalog:**
- [[summary.md|summary]] - All AoC 2015 problems overview

*Tags: #aoc2015 #day10 #look-and-say #sequence #string-processing #run-length-encoding*