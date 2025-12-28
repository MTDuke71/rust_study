← [[advent_of_code/aoc2015/Problem_Statements/summary|AoC 2015 Index]]

--- Day 3: Perfectly Spherical Houses in a Vacuum ---
Santa is delivering presents to an infinite two-dimensional grid of houses.

He begins by delivering a present to the house at his starting location, and then an elf at the North Pole calls him via radio and tells him where to move next. Moves are always exactly one house to the north (^), south (v), east (>), or west (<). After each move, he delivers another present to the house at his new location.

However, the elf back at the north pole has had a little too much eggnog, and so his directions are a little off, and Santa ends up visiting some houses more than once. How many houses receive at least one present?

For example:

> delivers presents to 2 houses: one at the starting location, and one to the east.
^>v< delivers presents to 4 houses in a square, including twice to the house at his starting/ending location.
^v^v^v^v^v delivers a bunch of presents to some very lucky children at only 2 houses.

Your puzzle answer was 2592.

The first half of this puzzle is complete! It provides one gold star: *

--- Part Two ---
The next year, to speed up the process, Santa creates a robot version of himself, Robo-Santa, to deliver presents with him.

Santa and Robo-Santa start at the same location (delivering two presents to the same starting house), then take turns moving based on instructions from the elf, who is eggnoggedly reading from the same script as the previous year.

This year, how many houses receive at least one present?

For example:

^v delivers presents to 3 houses, because Santa goes north, and then Robo-Santa goes south.
^>v< now delivers presents to 3 houses, and Santa and Robo-Santa end up back where they started.
^v^v^v^v^v now delivers presents to 11 houses, with Santa going one direction and Robo-Santa going the other.
Your puzzle answer was 2360.

Both parts of this puzzle are complete! They provide two gold stars: **

At this point, you should return to your Advent calendar and try another puzzle.

If you still want to see it, you can get your puzzle input.

You can also [Share] this puzzle.

---

## 🔗 Related Zettelkasten Concepts

**Core Concepts:**
- [[hashset]] - Tracking unique visited locations
- [[grid-navigation]] - 2D coordinate movement (up/down/left/right)
- [[coordinate-systems]] - (x, y) position tracking
- [[deduplication]] - Counting unique houses

**Data Structures:**
- [[hashset]] - Efficient duplicate detection O(1)
- [[tuple]] - Representing (x, y) coordinates
- [[struct]] - Position representation

**Patterns:**
- [[state-machines]] - Tracking current position
- [[set-operations]] - Union of two paths (Part 2)
- [[alternating-iteration]] - Santa/Robo-Santa turns

**Mission Connections:**
- [[mission6]] - Grid navigation and coordinate systems

**Learning Resources:**
- [[Daily Study MOC]] - Days 6-10 cover HashSet and deduplication
- [[performance-benchmarking-grid-optimization]] - Coordinate system patterns

*Tags: #aoc #aoc2015 #day03 #hashset #grid #coordinates #navigation*