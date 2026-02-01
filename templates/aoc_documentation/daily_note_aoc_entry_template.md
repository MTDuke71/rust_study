# Daily Note - AoC Entry Template

**Quick Links**: [← Templates](../README.md) | [Daily Notes](../../zettelkasten/Daily Notes/README.md) | [AoC Main](../../advent_of_code/README.md)

---

Add this to your daily note when solving an AoC problem. Provides a consistent format for tracking AoC progress in your zettelkasten daily notes.

**Usage**: Copy the template section below and fill in the values after solving a day.

---

## AoC [YEAR] Day [N]: [Problem Title] ⭐⭐

**Problem**: [1-2 sentence summary]  
**Answer**: Part 1: XXXXX | Part 2: XXXXX  
**Runtime**: XXX.Xµs (Part 1: XX.Xµs, Part 2: XX.Xµs)  
**Algorithm**: [Main approach - e.g., "BFS with state tracking", "Range mapping"]

**Key Insight**: [1-2 sentences on the breakthrough/core idea]

**Mission Integration**: Mission X ([component]) | None (custom implementation)

**Function Guide**: [[aoc[YEAR]/days/day[NN]_function_guide]]

---

**Session Time**: 
- Solving: XX minutes (Part 1: XX min, Part 2: XX min)
- Documentation: XX minutes

**Difficulty**: ⭐⭐⭐ (1-5 stars)

---

## Example Usage

### For AoC 2022 Day 1
```markdown
## AoC 2022 Day 1: Calorie Counting ⭐⭐

**Problem**: Find elf carrying most calories (Part 1), sum of top 3 elves (Part 2)  
**Answer**: Part 1: 69289 | Part 2: 205615  
**Runtime**: 65.2µs (Part 1: 27.3µs, Part 2: 37.9µs)  
**Algorithm**: Linear scan with delimiter parsing

**Key Insight**: Split on double newlines (`\n\n`) to separate elves, sum calories per elf, then find max (Part 1) or sort and sum top 3 (Part 2).

**Mission Integration**: None (simple string parsing)

**Function Guide**: [[aoc2022/days/day01_function_guide]]

---

**Session Time**: 
- Solving: 25 minutes (Part 1: 10 min, Part 2: 15 min)
- Documentation: 20 minutes

**Difficulty**: ⭐ (straightforward parsing and sorting)
```

**Note**: The example above uses hypothetical numbers from the template creation. In reality, AoC 2022 Day 1 solved on 2026-02-01 had:
- **Actual Answers**: Part 1: 70698 | Part 2: 206643
- **Actual Runtime**: 25.6µs (Part 1: 24.4µs, Part 2: 25.4µs)
- **Speedup**: Parse-once pattern achieved 49% improvement over separate parsing

Pretty close prediction! 😊

---

**Navigation**: [← Templates](../README.md) | [Daily Notes](../../zettelkasten/Daily Notes/README.md) | [AoC Main](../../advent_of_code/README.md)

**See Also**:
- [Function Guide Template](function_guide_template.md) - Detailed per-day documentation
- [Stats Dashboard Template](stats_dashboard_template.md) - Overall progress tracking
- [AoC Solver Template](../../advent_of_code/AOC_SOLVER_TEMPLATE.md) - Code implementation pattern
- [Daily Note: 2026-02-01](../../zettelkasten/Daily Notes/2026-02-01.md) - Today's actual entry

---

**Created**: 2026-01-20  
**Updated**: 2026-02-01 - Added navigation links and ironic actual results note  
**Purpose**: Consistent AoC tracking in daily notes
