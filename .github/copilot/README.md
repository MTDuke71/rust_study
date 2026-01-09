# GitHub Copilot Custom Skills

This directory contains custom skill definitions that extend GitHub Copilot's capabilities specifically for the Rust Study workspace.

## Available Skills

### `aoc-solver` - Advent of Code Problem Solver

A systematic AoC problem solver that follows the workspace's incremental development philosophy and mission-reuse approach.

#### What It Does

The `aoc-solver` skill encapsulates your complete AoC workflow:

1. **Mission Scanner** - Automatically checks existing missions for reusable components before implementing
2. **Incremental Solver** - Implements solutions in logical stages with user feedback
3. **File Manager** - Handles AoC file structure (problem statements, inputs, solutions)
4. **Benchmark Updater** - Updates benchmark files with new day solutions
5. **Math Documenter** - Identifies and documents mathematical concepts in zettelkasten
6. **Zettelkasten Linker** - Creates bidirectional links between code and knowledge notes
7. **Quality Checker** - Ensures code quality before completion

#### How to Use

When GitHub Copilot skills are enabled, you can invoke the skill in several ways:

**Direct invocation:**
```
@aoc-solver Solve Day 10 of 2024
```

**With specific capability:**
```
@aoc-solver Scan missions for Day 12 components
@aoc-solver Add benchmarks for Day 8
@aoc-solver Document mathematics for Day 15
```

**Conversation starters:**
- "Solve AoC 2024 Day X"
- "Scan missions for Day X solution components"
- "Add benchmarks for Day X"
- "Document mathematics for Day X"

#### Workflow Example

```
User: @aoc-solver Solve AoC 2024 Day 10

Skill Response:
1. Mission Scan Results:
   ✓ Grid problem detected → Mission 6 Grid<T> applicable
   ✓ Graph traversal needed → Mission 8 BFS/Graph trait applicable
   
2. Reading problem statement from Problem_Statements/day10.md...
3. Reading puzzle input from inputs/day10.txt...

Proposed approach: Use Mission 6 Grid for 2D storage, implement Graph trait 
for TopoMap, use Mission 8 BFS for pathfinding.

Ready to proceed with Stage 1 (Parse input)? [wait for confirmation]

Stage 1: Implementing input parser using Mission 6 Grid<u8>...
[code implementation]

Testing with inputs/day10_example.txt... ✓ Pass
Continue to Stage 2 (Core logic)?
```

#### Benefits

✅ **Systematic approach** - No steps skipped, follows proven workflow
✅ **Mission reuse** - Automatically identifies applicable mission libraries
✅ **Incremental development** - User stays in the loop, understands progression
✅ **Complete documentation** - Benchmarks, math notes, zettelkasten all updated
✅ **Quality assurance** - Built-in quality checks before completion
✅ **Knowledge integration** - Bidirectional linking between code and concepts

#### Integration with Workspace

The skill is deeply integrated with your workspace structure:
- Understands all 10 missions and their capabilities
- Knows your file organization patterns
- Follows your integrator philosophy (compose, don't reimplement)
- Maintains your zettelkasten knowledge graph
- Updates benchmarks and documentation automatically
- Respects your incremental development preference

#### Customization

To modify the skill behavior, edit `.github/copilot/aoc-solver.json`:
- `instructions` - Core behavior guidelines
- `capabilities` - Specific functions and checklists
- `workflow.phases` - Step-by-step process
- `best_practices` - Guidelines to follow
- `avoid` - Anti-patterns to prevent

#### Prerequisites

- GitHub Copilot with skills support enabled
- VS Code with GitHub Copilot extension
- Rust Study workspace properly configured

## Creating New Skills

To create additional skills for this workspace:

1. Create a new `.json` file in this directory
2. Follow the structure in `aoc-solver.json`
3. Define clear capabilities and workflows
4. Add examples demonstrating usage
5. Document in this README

### Potential Future Skills

- **mission-implementer** - V-Cycle mission development with REQ tracing
- **zettelkasten-builder** - Automated knowledge graph creation and linking
- **rust-book-processor** - Systematic Rust Book chapter integration
- **benchmark-analyzer** - Performance analysis and optimization suggestions
- **math-concept-mapper** - Identify and document mathematical patterns across codebase

---

*For more information about GitHub Copilot skills, see: [GitHub Copilot Documentation](https://docs.github.com/en/copilot)*
