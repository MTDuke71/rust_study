Part 1 seems to be pretty straight forward. Do not code anything just confirm my analysis. after parsing the data you maintain a flag for each raindeer that sayas it it in fly mode or rest mode, and much longer it will be in that mode. 
If it is fly mode you increment distances by speed, and then decrement the counter. You do this for for each raindeer 2503 times

✅ Your analysis is correct!

That's exactly the right approach for Part 1:

Parse the reindeer data (name, speed, flight_time, rest_time)

Initialize state for each reindeer:

Current distance = 0
Current mode = "flying" (starts in fly mode)
Timer for current mode = flight_time
Simulate 2503 seconds:

For each second:
For each reindeer:
If in fly mode: add speed to distance
Decrement the mode timer
If timer reaches 0:
Switch modes (fly ↔ rest)
Reset timer to appropriate duration (flight_time or rest_time)
Find the winner: Return the maximum distance traveled

This is a straightforward state machine simulation - each reindeer cycles between flying and resting with fixed durations, and you just need to track which mode they're in and count down until the mode switches.

Part 2 will likely be similar but with a different scoring mechanism (probably awarding points each second to whoever is currently in the lead, rather than just measuring final distance). Ready to implement when you are! 🦌

when I write tests for this would I just pick a small time period that I can hand caluculate for example Dancer at time 1 would be 37 km, at time 2 would also be 37, at time 37 would still be 37, but at time 38 would be 74 km

✅ Exactly right!

That's the perfect approach for writing tests. Pick small time periods you can hand-calculate to verify your logic:

Using Dancer as your test case (37 km/s for 1 second, then rest 36 seconds):

Time 1: 37 km (flying, 1st second of flight)
Time 2: 37 km (resting, 1st second of rest)
Time 37: 37 km (still resting)
Time 38: 74 km (flying again, 1st second of 2nd flight cycle)
Time 39: 111 km (flying, 2nd second of flight)
Time 74: 111 km (back to resting)
Time 75: 148 km (flying again)
This approach is great because:

Easy to verify - You can calculate by hand
Catches off-by-one errors - Especially at cycle boundaries
Tests mode switching - Catches bugs in the fly/rest transition logic
Small dataset - Tests run fast, easy to debug if they fail
I'd suggest writing a few test cases:

Test at different points in the cycle (mid-flight, mid-rest, at boundaries)
Test with multiple complete cycles
Maybe test with one of the slower reindeer too for variety
This will give you good confidence that your state machine logic is correct! 🎯

would that test the parser is working as well

Good question! Not directly - those distance calculation tests would verify the simulation logic, but they wouldn't test the parser itself.

To test the parser, you'd want separate tests that:

Parse a single line and verify the struct fields:
Parse multiple lines and verify the Vec:
Then your simulation tests (with the small time calculations) would test that the parsed data is being used correctly in the simulation.

So ideally: Parser tests (verify parsing is correct) + Simulation tests (verify logic is correct) = comprehensive coverage! 🎯

---

## 🔗 **Zettelkasten Links**

**Core Concepts:**
- [[aoc-2015-day14]] - Main Day 14 problem statement and solution
- [[simulation-patterns]] - State machine simulation techniques
- [[testing-strategies]] - Unit testing and integration testing approaches
- [[state-machines]] - Flight/rest cycle implementation patterns

**Algorithm Analysis:**
- [[cyclic-algorithms]] - Algorithms for repeating behavior patterns
- [[mathematical-optimization]] - O(1) cycle calculations vs O(n) simulation
- [[parser-testing]] - Input parsing and validation strategies
- [[test-driven-development]] - Test-first development methodology

**Related Topics:**
- [[AoC 2015 MOC]] - Complete overview of 2015 problems
- [[AoC Patterns MOC]] - Common algorithm patterns in competitive programming
- [[performance-analysis]] - Benchmarking and complexity analysis

*Tags: #aoc-2015-day14 #analysis #simulation #testing #state-machines #cyclic-algorithms #parser-testing*

*Links: [[zettel-index]] | [[aoc-2015-day14]] | [[simulation-patterns]] | [[testing-strategies]] | [[AoC 2015 MOC]]*