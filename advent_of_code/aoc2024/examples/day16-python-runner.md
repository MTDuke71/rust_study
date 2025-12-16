# Day 16 (Python) — Runner / Optimal Path Count

Runs the Python reference solution and prints:
- `min_score`
- `best_routes_len` (number of distinct optimal routes found by the Python implementation)
- `unique_tiles` (should match the AoC Part 2 answer)

## Command (PowerShell)

Run from the workspace root:

```powershell
python -c "from solutions.day16 import Solution; import pathlib; data = pathlib.Path('..\\inputs\\day16_example.txt').read_text().splitlines(); sol = Solution.__new__(Solution); routes = sol.find_routes(data); min_score = min(score for (_path, score) in routes); best_routes = [r for r in routes if r[1] == min_score]; print('min_score=', min_score); print('best_routes_len=', len(best_routes)); print('unique_tiles=', len({tile for route in best_routes for tile in route[0]}));"
```

Or, if you prefer to `cd` first:

```powershell
Set-Location "D:\repos\rust_study\advent_of_code\aoc2024\2024py"
python -c "from solutions.day16 import Solution; import pathlib; data = pathlib.Path('..\\inputs\\day16_example.txt').read_text().splitlines(); sol = Solution.__new__(Solution); routes = sol.find_routes(data); min_score = min(score for (_path, score) in routes); best_routes = [r for r in routes if r[1] == min_score]; print('min_score=', min_score); print('best_routes_len=', len(best_routes)); print('unique_tiles=', len({tile for route in best_routes for tile in route[0]}));"
```

## Notes

- This intentionally uses `Solution.__new__(Solution)` to avoid calling `SolutionBase.__init__()` (which expects the `2024py/data/dayXX/puzzle_input.txt` layout).
- This runner uses `inputs/day16_example.txt`. Swap the path if you want puzzle input.
