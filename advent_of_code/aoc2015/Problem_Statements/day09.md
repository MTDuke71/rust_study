← [[summary|AoC 2015 Index]]

--- Day 9: All in a Single Night ---
Every year, Santa manages to deliver all of his presents in a single night.

This year, however, he has some new locations to visit; his elves have provided him the distances between every pair of locations. He can start and end at any two (different) locations he wants, but he must visit each location exactly once. What is the shortest distance he can travel to achieve this?

For example, given the following distances:

London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
The possible routes are therefore:

Dublin -> London -> Belfast = 982
London -> Dublin -> Belfast = 605
London -> Belfast -> Dublin = 659
Dublin -> Belfast -> London = 659
Belfast -> Dublin -> London = 605
Belfast -> London -> Dublin = 982
The shortest of these is London -> Dublin -> Belfast = 605, and so the answer is 605 in this example.

What is the distance of the shortest route?

Your puzzle answer was 141.

--- Part Two ---
The next year, just to show off, Santa decides to take the route with the longest distance instead.

He can still start and end at any two (different) locations he wants, and he still must visit each location exactly once.

For example, given the distances above, the longest route would be 982 via (for example) Dublin -> London -> Belfast.

What is the distance of the longest route?

Your puzzle answer was 736.

Both parts of this puzzle are complete! They provide two gold stars: **

At this point, you should return to your Advent calendar and try another puzzle.

If you still want to see it, you can get your puzzle input.

You can also [Share] this puzzle.

---

## 🔗 Related Zettelkasten Concepts

**Core Concepts:**
- [[permutations]] - Generating all possible routes (n! permutations)
- [[graph-algorithms]] - Traveling Salesman Problem (TSP)
- [[shortest-path]] - Finding minimum distance path
- [[backtracking]] - Exploring all route possibilities

**Data Structures:**
- [[hashmap]] - Distance lookup table between cities
- [[hashset]] - Tracking visited cities
- [[vec]] - Storing route permutations

**Patterns:**
- [[recursion]] - Recursive permutation generation
- [[min-max]] - Finding shortest/longest route
- [[combinatorics]] - Factorial growth of search space

**Algorithms:**
- [[brute-force]] - Exhaustive search for small n (8-10 cities)
- [[dynamic-programming]] - Held-Karp algorithm for larger instances
- [[greedy-algorithms]] - Nearest neighbor heuristic

**Learning Resources:**
- [[Daily Study MOC]] - Days 6-10 cover HashMap for distance lookup

*Tags: #aoc #aoc2015 #day09 #graph #tsp #permutations #optimization*