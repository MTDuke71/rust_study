---
name: aoc-haskell
description: Solve Advent of Code problems in Haskell with a teaching-oriented workflow aimed at a newcomer — type-signature discipline, annotated solutions, beginner-friendly function guides, and gradual introduction of Haskell concepts alongside problem solving.
---

# AoC Haskell — Solve Problems While Learning Haskell

**Purpose**: Guide a Haskell newcomer through Advent of Code one day at a time, using each puzzle to introduce language features in the order they're actually useful. Every solution is also a teaching artifact.

**Audience**: Matt — experienced Rust/AUTOSAR integrator, new to Haskell. He learns by running examples, not by reading paragraphs of theory. Every new concept must be grounded in the day's code and ideally compared to its Rust analogue.

**Output contract** for every day: working code that compiles and passes tests, plus a function guide that a reader who has never opened a Haskell file before could follow line by line.

---

## CRITICAL: Teach as you solve

1. **Every top-level binding has an explicit type signature.** No exceptions. Type signatures are the primary teaching tool — they tell the reader what a function does before they read its body.
2. **Explain every Prelude function the first time it's used in that day**, in a short inline comment or (preferably) in the function guide. `foldl'`, `zipWith`, `iterate`, `scanl`, `mapMaybe`, etc. — the reader has not memorised the Prelude yet.
3. **Explain every operator symbol the first time it's used**: `$`, `.`, `<$>`, `<*>`, `>>=`, `<&>`, `&`, `!!`, `!`. The zoo of punctuation is the single biggest barrier for a beginner.
4. **Compare to Rust where it helps, but don't force it**. `Data.Map.Strict` ≈ `BTreeMap`, `fold`/`foldl'` ≈ `iter().fold()`, `Maybe` ≈ `Option`, `Either e a` ≈ `Result<a, e>`. But `IO`, laziness, and type classes have no clean Rust analogue — explain them on Haskell's own terms.
5. **Introduce concepts in the order the puzzles need them**, not in the order a textbook would. Don't explain `Monad` on Day 1 just because you can. Introduce it when the code actually benefits (parsing, state, IO composition).
6. **Prefer clarity over cleverness**. Point-free style is beautiful, but `foo = map (\x -> x * 2 + 1)` is clearer than `foo = map ((+1) . (*2))` for a beginner. Use point-free only when the transformation is genuinely more readable that way, and explain the composition.

---

## Toolchain (assume GHC 9.8+ via GHCup)

- **Build tool**: `cabal` (modern, built-in, fewer moving parts than stack for a single-package workspace).
- **Test framework**: `hspec` (readable `it "does the thing" $ ... \`shouldBe\` ...` style matches Rust's `assert_eq!` mental model).
- **Benchmarks**: `criterion` (direct analogue of Rust's `criterion` crate).
- **Parsers**: start with `words`/`lines`/`reads`/`splitOn`. Graduate to `megaparsec` once parsing complexity warrants it (roughly Day 4+ in a typical AoC year). Introduce `megaparsec` as a dedicated teaching day.
- **Strict collections**: always `Data.Map.Strict`, `Data.IntMap.Strict`, `Data.HashMap.Strict`, never the lazy defaults. Explain why (space leaks from lazy map values) the first time.
- **Vectors**: `Data.Vector` (boxed) or `Data.Vector.Unboxed` (for `Int`, `Char`, tuples of primitives) when `[a]` is too slow or indexing matters.
- **Hot parsing**: `Data.ByteString.Char8` once per-line parsing becomes a bottleneck. Only introduce when a solution is demonstrably slow with `String`.

---

## Project Layout

```
aoc2018-haskell/
├── aoc2018.cabal                    -- single library + exe per day
├── cabal.project                    -- optional, for multi-package later
├── src/
│   ├── AOC/Common.hs                -- shared helpers (parse utilities, Grid, etc.)
│   ├── AOC/Day01.hs                 -- one module per day
│   ├── AOC/Day02.hs
│   └── ...
├── app/
│   └── Main.hs                      -- CLI: `cabal run aoc2018 -- 1` runs Day 1
├── test/
│   ├── Spec.hs                      -- hspec driver
│   └── AOC/Day01Spec.hs             -- per-day test file
├── bench/
│   └── Main.hs                      -- criterion driver
├── inputs/
│   ├── day01.txt
│   └── ...
└── docs/
    ├── summary.md                   -- master table
    └── days/
        ├── day01.md                 -- function guide
        └── ...
```

**One module per day** keeps compile times low and isolates teaching per puzzle. `AOC.Common` holds whatever you find yourself re-implementing (3+ days running).

---

## File Template — `src/AOC/DayXX.hs`

```haskell
-- |
-- Module      : AOC.DayXX
-- Description : Day XX — [Puzzle Title]
--
-- Brief problem statement in 1–2 sentences.
--
-- Part 1: [what it asks]
-- Part 2: [what it asks]
--
-- Concepts introduced this day:
--   * [new concept 1 — e.g. "foldl' for strict accumulation"]
--   * [new concept 2 — e.g. "Maybe as failure"]
module AOC.DayXX
  ( solve
  , solvePart1
  , solvePart2
  , parseInput
  ) where

import Data.List (foldl')           -- explain: strict left fold, O(1) space accumulator
-- import other modules; annotate each one the first time it appears in the codebase.

-- | A parsed puzzle input. Choose a type that makes both parts easy.
--   Use a data type, not a bare tuple, once you have more than 2 fields.
type Input = [Int]

-- | Parse the raw puzzle input into 'Input'. Keep this pure and total:
--   parsing errors should either panic (via 'error', fine for AoC) or
--   return 'Either'. For AoC, panic is simpler and acceptable.
parseInput :: String -> Input
parseInput = map read . lines       -- explain `map`, `read`, `lines`, and `.`

-- | Part 1 logic, operating on parsed data only. No parsing here.
solvePart1 :: Input -> Int
solvePart1 = sum                    -- explain any new Prelude functions on first use

-- | Part 2 logic, operating on parsed data only. No parsing here.
solvePart2 :: Input -> Int
solvePart2 xs = foldl' (+) 0 xs     -- explain foldl' vs foldr vs foldl

-- | Parse once, compute both parts. In Haskell this works out of the box
--   because 'input' is bound once in a 'let'/'where' and shared — laziness
--   does NOT cause re-parsing. But state the pattern explicitly anyway
--   for readers coming from strict languages.
solve :: String -> (Int, Int)
solve raw =
  let input = parseInput raw
  in  (solvePart1 input, solvePart2 input)
```

**Rules the template enforces**:

1. Module header with a `Concepts introduced` bullet list — this is the learning contract for the day.
2. Every top-level binding has a type signature.
3. `parseInput`, `solvePart1`, `solvePart2`, `solve` as the four public functions (same shape as the Rust aoc skill — consistency across languages helps Matt).
4. `solve` parses exactly once and feeds both parts. Even though Haskell's sharing means you *could* be sloppy, being explicit models the habit.
5. Exports list is explicit (`module AOC.DayXX ( solve, solvePart1, solvePart2, parseInput )`). Teach why: it pins the public API and flags unused bindings as warnings. Rust analogue: `pub` visibility.

---

## Test Template — `test/AOC/DayXXSpec.hs`

```haskell
module AOC.DayXXSpec (spec) where

import Test.Hspec
import AOC.DayXX

-- Example input from the puzzle description. Pin this as a Haskell
-- multi-line string; use 'unlines' to build from lines so trailing
-- newlines are explicit.
example :: String
example = unlines
  [ "line 1"
  , "line 2"
  ]

spec :: Spec
spec = describe "Day XX" $ do
  it "parses the example" $ do
    let input = parseInput example
    length input `shouldBe` 2   -- or whatever structural check fits

  it "solves Part 1 on the example" $
    solvePart1 (parseInput example) `shouldBe` 42

  it "solves Part 2 on the example" $
    solvePart2 (parseInput example) `shouldBe` 99

  -- Actual input tests. Load via 'readFile' in 'IO', so the structure
  -- differs from the pure example tests — good teaching moment for 'IO'.
  it "solves Part 1 on the actual input" $ do
    raw <- readFile "inputs/dayXX.txt"
    solvePart1 (parseInput raw) `shouldBe` 0  -- replace with real answer

  it "solves Part 2 on the actual input" $ do
    raw <- readFile "inputs/dayXX.txt"
    solvePart2 (parseInput raw) `shouldBe` 0  -- replace with real answer

  it "solve returns both parts from a single parse" $ do
    raw <- readFile "inputs/dayXX.txt"
    solve raw `shouldBe` (0, 0)
```

**Teaching notes this template provides**:
- `do` notation for `IO` (explain the first time).
- `readFile :: FilePath -> IO String` — marks the file read as an effect.
- `shouldBe` reads like `assert_eq!(a, b)` but swapped: `actual \`shouldBe\` expected`.
- Explain the difference between `let x = ...` (pure binding inside `do`) vs `x <- action` (monadic bind).

---

## Benchmark Template — `bench/Main.hs`

```haskell
module Main where

import Criterion.Main
import qualified AOC.Day01 as D01
-- import additional days as they're written

main :: IO ()
main = do
  input01 <- readFile "inputs/day01.txt"
  -- force the raw string so we time solve, not readFile
  length input01 `seq` defaultMain
    [ bgroup "day01"
        [ bench "combined" $ nf D01.solve input01
        , bench "part1"    $ nf D01.solvePart1 (D01.parseInput input01)
        , bench "part2"    $ nf D01.parseInput input01 >>= \p -> nf D01.solvePart2 p `seq` pure ()  -- simpler: see note below
        ]
    ]
```

**Simplification**: the `part2` line above is clunky. In practice, benchmark the parsed-input version directly:

```haskell
, env (return (D01.parseInput input01)) $ \p ->
    bgroup "parsed"
      [ bench "part1" $ nf D01.solvePart1 p
      , bench "part2" $ nf D01.solvePart2 p
      ]
```

**Teaching points**: `nf` forces the result to normal form (analogue of `black_box`); `env` sets up a shared value across benches; `seq` forces evaluation of its first argument before returning the second (this is where *laziness* gets real and needs a paragraph of explanation).

---

## Concept Introduction Schedule (suggested)

Introduce roughly one major concept per day. Don't front-load.

| Days | Concept | Why now |
|------|---------|---------|
| 1 | `map`, `filter`, `foldl'`, `sum`, `lines`, `words`, `read`, `$`, `.` | The bread-and-butter list plumbing |
| 2 | Pattern matching, guards, `where` clauses, `case` | Most puzzles need structural dispatch |
| 3 | `Data.Map.Strict`, `Data.Set`, tuple unpacking | First puzzle that benefits from keyed lookup |
| 4 | `Maybe`, `fromMaybe`, `mapMaybe`, `Just`/`Nothing` patterns | First puzzle with "might fail" semantics |
| 5 | `foldr` vs `foldl'`, laziness, strict vs lazy evaluation | When accumulator choice matters for correctness or space |
| 6 | Records, `data` with named fields, `deriving (Show, Eq, Ord)` | First puzzle where tuples become unwieldy |
| 7 | `Data.IntMap.Strict`, `Data.Array` or `Data.Vector` for indexing | First puzzle where `[a] !! i` is too slow |
| 8 | `Megaparsec` (dedicated teaching day) | Input format finally justifies a real parser |
| 9+ | `State` monad, `Reader`, `Writer`, list comprehensions, type classes | Only when a puzzle genuinely benefits |

Adjust based on what each year's puzzles actually need. The schedule above is a suggestion, not a mandate.

---

## Function Guide Format — `docs/days/dayXX.md`

Every day gets a function guide written for someone new to Haskell. Structure:

```markdown
# Day XX: [Puzzle Title]

**Problem**: [one-paragraph summary]
**Answers**: Part 1 = **X**, Part 2 = **Y**
**Code**: [DayXX.hs](../../src/AOC/DayXX.hs)
**Runtime**: Part 1 Xµs, Part 2 Xµs, Combined Xµs

**New concepts this day**:
- [concept 1 with 1-line takeaway]
- [concept 2 with 1-line takeaway]

---

## Problem Summary

[2–4 paragraphs explaining the puzzle in plain English, before any code.]

---

## Data Model

```haskell
-- Actual code block showing the types chosen, with inline comments.
type Input = [(Int, Int)]
data Particle = Particle { pos :: !Vec3, vel :: !Vec3 } deriving (Eq, Show)
```

**Why these types**:
- [one paragraph per non-obvious choice, e.g. "why strict fields", "why Data.Map over [(k,v)]"]

---

## Parsing

```haskell
parseInput :: String -> Input
parseInput = ...
```

Walk through the parser **line by line**, naming every function:
- `lines :: String -> [String]` — splits on `\n`.
- `words :: String -> [String]` — splits on whitespace.
- `read :: Read a => String -> a` — parses a showable type; will panic on malformed input (fine for AoC).
- `(.)` — function composition, reads right-to-left.
- `($)` — function application with low precedence, saves parentheses.

Explain each one the **first time** it appears in the codebase. On Day 10 you don't need to re-explain `map`.

---

## Part 1

```haskell
solvePart1 :: Input -> Int
solvePart1 = ...
```

Walk the reader through the algorithm. If there's a Prelude function they haven't seen, explain it here. If the solution uses laziness, point that out explicitly.

Every new Haskell mechanism gets its own subsection:

### `foldl'` vs `foldl` vs `foldr`

```haskell
sum' :: [Int] -> Int
sum' = foldl' (+) 0
```

- `foldl'` walks left-to-right with a **strict** accumulator → O(1) space.
- `foldl` walks left-to-right with a lazy accumulator → builds a thunk tower, O(n) space, usually a bug.
- `foldr` walks right-to-left, good for structures that can short-circuit (e.g. `any`, `all`, infinite lists).

**Rule of thumb**: default to `foldl'`. Reach for `foldr` only when short-circuit or laziness is wanted.

---

## Part 2

Same structure as Part 1. If Part 2 reuses Part 1's machinery, say so — reuse is the whole point of parse-once.

---

## Key Patterns

2–4 short takeaways that generalise beyond this puzzle. For Haskell learners, this is often a language-feature insight as much as a problem-solving one.

---

## If I Were Writing This in Rust

One paragraph comparing the Haskell solution to the Rust equivalent Matt would have written. This is high-leverage for him — anchors the new language in the one he already knows.

Typical comparisons:
- `Data.Map.Strict.insertWith (+) k 1` ≈ `*map.entry(k).or_insert(0) += 1`
- `filter (> 0)` ≈ `.filter(|&x| x > 0)`
- `map fst` ≈ `.map(|(a, _)| a)`
- `foldr` with early termination ≈ `iter().take_while(...).fold(...)`

---

**Navigation**: [← Day (XX-1)](dayXX-1.md) | [All Days](../summary.md) | [Day (XX+1) →](dayXX+1.md)
```

---

## The `summary.md` Table

Grow this file as days complete. Columns:

| Day | Title | Part 1 | Part 2 | Combined | Concepts | Notes |
|-----|-------|--------|--------|----------|----------|-------|

Keep it scannable — one row per day, answers hidden under links to the function guide. Link the day title to the function guide, not the source file.

---

## Development Workflow per Day

1. **Read the problem, write a plan in plain English** (5 min). Don't start typing Haskell until you can explain the algorithm without it.
2. **Skeleton** (5 min): copy `DayXX.hs` + `DayXXSpec.hs` from the previous day, wire into `Spec.hs` driver.
3. **Parse input** (10–15 min): write `parseInput`, write a test that checks the structural shape (length, first element, etc.). Get the parser green on the example before touching logic.
4. **Part 1** (20–40 min): small function-at-a-time, each with a type signature, each tested against the example before moving on. Introduce new Prelude functions/concepts with a one-line comment at the point of use — the full explanation goes in the function guide later.
5. **Run on actual input**. If the answer is right, add an `actual input` test.
6. **Part 2** (20–40 min): same loop.
7. **Benchmark** (2 min): `cabal bench -- dayXX`. Targets below.
8. **Clean up warnings**: `cabal build -- -Wall -Wcompat -Widentities` should be clean. Treat warnings as errors during learning — they catch real bugs.
9. **Write the function guide** (25–40 min). This is where the teaching happens. Do not skip it — it's half the value of the exercise.
10. **Update `summary.md`**, commit.

---

## Performance Targets (soft)

Haskell with `-O2` is usually within 2–3× of equivalent Rust for AoC-scale problems. Realistic targets:

- **Per day combined**: < 500 ms on typical puzzles (vs Rust's ~100 ms — accept the gap).
- **Over 1 second**: look for a real algorithmic improvement, not micro-optimisation.
- **Over 10 seconds**: something is wrong (usually space leak, lazy `Map`, or `[a] !! i` where a `Vector` belongs).

**Optimisation order** (same spirit as Rust, different tools):
1. Right algorithm (parse-once, right data structure).
2. Strict accumulator / strict fields — `foldl'`, `!fieldName` in records, `Data.Map.Strict`.
3. `Data.IntMap.Strict` or `Data.Vector.Unboxed` over lists/boxed vectors when profiling points there.
4. `Data.ByteString.Char8` for hot parsing paths.
5. `parallel` / `async` only when clearly embarrassingly parallel and big enough to matter.

---

## Common Beginner Pitfalls to Watch For

Explicitly call these out in code review and function guides when they arise:

- **`foldl` instead of `foldl'`**: near-universal space leak source. Almost always wrong for numeric accumulation.
- **Lazy `Data.Map`**: values accumulate as unevaluated thunks. Use `Data.Map.Strict` by default.
- **`[a] !! i` for indexing**: O(n). Use `Data.Vector` or `Data.IntMap` for random access.
- **`String` for large inputs**: `String = [Char]` is a linked list of boxed `Char`. Use `Data.Text` or `Data.ByteString` when it matters.
- **`read` panicking silently**: acceptable for AoC (input is trusted), but call it out the first time — in real code you'd use `readMaybe`.
- **Confusing `<$>` and `$`**: `$` is just "apply this function" (low precedence); `<$>` is `fmap` (lifts a function into a Functor).
- **Point-free before understanding**: don't write `(. f) . g` in teaching code. Prefer explicit lambdas when clarity is at stake.

---

## `AOC.Common` — Shared Utility Module

Grow this lazily as patterns repeat. Likely early inhabitants:

```haskell
module AOC.Common
  ( splitOn
  , count
  , frequencies
  , Grid, gridFromLines, gridGet, gridNeighbours4, gridNeighbours8
  ) where

import qualified Data.Map.Strict as Map
import Data.Map.Strict (Map)

-- | Counts how many elements of the list satisfy the predicate.
count :: (a -> Bool) -> [a] -> Int
count p = length . filter p

-- | Builds a Map from element to occurrence count.
frequencies :: Ord a => [a] -> Map a Int
frequencies = foldr (\x -> Map.insertWith (+) x 1) Map.empty
```

**Rule**: don't pre-populate. Add a function to `AOC.Common` only after you've written it for the third day in a row.

---

## Quick Reference Checklist

Before committing a day:

- [ ] **Type signatures** on every top-level binding.
- [ ] **`parseInput`, `solvePart1`, `solvePart2`, `solve`** all present.
- [ ] **`solve` parses once** via `let` binding.
- [ ] **Tests**: parse + part 1 example + part 2 example + part 1 actual + part 2 actual + combined.
- [ ] **Warnings clean** under `-Wall -Wcompat`.
- [ ] **Benchmarks** added for the new day.
- [ ] **Function guide written** and explains every new concept introduced this day.
- [ ] **summary.md** updated with timings, concepts, link.
- [ ] **Imports** are explicit and qualified (`import qualified Data.Map.Strict as Map`) when there's name collision risk.

---

## Teaching Style Reminders

- **One concept per day**, roughly. Resist the urge to show off a beautiful applicative parser on Day 2.
- **Show the type signature, then the body, then walk through it**. Not the other way around.
- **Run `cabal repl`** (GHCi) for exploration — the REPL is where Haskell learning clicks. Encourage Matt to try the functions interactively.
- **Compare to Rust explicitly** in the "If I were writing this in Rust" section of each function guide. He's an integrator — translation between validated mental models is his native skill.
- **Don't apologise for Haskell weirdness**. Laziness, type classes, and monads are weird for a reason; explain the reason.
- **Celebrate the REPL**. `ghci> :t foldl'` is a superpower. Mention it often.

---

## First-Day Special: `Day 01`

The first day's function guide is load-bearing — it introduces the project structure, the toolchain, GHCi, type signatures, and the fundamental Prelude vocabulary. Budget more time for it (2–3 hours vs the normal 45–90 min) and treat it as the tutorial entry point for the whole year.

Structure the Day 01 guide to include:
- "How to run the code" (cabal commands, GHCi walkthrough).
- "How to read a Haskell type signature" (half a page, with examples).
- "The five operators you'll see every day: `$`, `.`, `<$>`, `=<<`, `<>`".
- A very explicit walk through each line of the Day 01 solution.

Subsequent days build on this foundation and can be terser.
