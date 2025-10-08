← [[summary|AoC 2015 Index]]

--- Day 4: The Ideal Stocking Stuffer ---
Santa needs help mining some AdventCoins (very similar to bitcoins) to use as gifts for all the economically forward-thinking little girls and boys.

To do this, he needs to find MD5 hashes which, in hexadecimal, start with at least five zeroes. The input to the MD5 hash is some secret key (your puzzle input, given below) followed by a number in decimal. To mine AdventCoins, you must find Santa the lowest positive number (no leading zeroes: 1, 2, 3, ...) that produces such a hash.

For example:

If your secret key is abcdef, the answer is 609043, because the MD5 hash of abcdef609043 starts with five zeroes (000001dbbfa...), and it is the lowest such number to do so.
If your secret key is pqrstuv, the lowest number it combines with to make an MD5 hash starting with five zeroes is 1048970; that is, the MD5 hash of pqrstuv1048970 looks like 000006136ef....
Your puzzle answer was 282749.

--- Part Two ---
Now find one that starts with six zeroes.

Your puzzle answer was 9962624.

Both parts of this puzzle are complete! They provide two gold stars: **

At this point, you should return to your Advent calendar and try another puzzle.

Your puzzle input was yzbqklnj.

You can also [Share] this puzzle.

---

## 🔗 Related Zettelkasten Concepts

**Core Concepts:**
- [[hashing]] - MD5 hash algorithm and cryptographic hashing
- [[brute-force]] - Exhaustive search with early termination
- [[string-formatting]] - Concatenating key + number
- [[hexadecimal]] - Hexadecimal representation and prefix matching

**Data Structures:**
- [[hashmap]] - Understanding hash functions (conceptual)

**Patterns:**
- [[iterators]] - Counting up from 1 until condition met
- [[loop-until-found]] - Search until prefix match found
- [[starts-with]] - Prefix checking pattern

**Performance:**
- [[performance-benchmarking-grid-optimization]] - Optimization strategies for brute-force
- [[parallel-processing]] - Potential parallelization (external crate: rayon)

**Learning Resources:**
- [[Daily Study MOC]] - Days 6-10 cover HashMap and hash concepts

*Tags: #aoc #aoc2015 #day04 #hashing #brute-force #cryptography*