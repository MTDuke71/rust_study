# Weasel Evolution - Dawkins' Evolutionary Algorithm

A Rust implementation of Richard Dawkins' famous **Weasel Program** from *The Blind Watchmaker* (1986).

## 🧬 What is the Weasel Program?

The Weasel Program demonstrates the power of **cumulative selection** in evolution:

- **Pure Random Chance**: Generating "METHINKS IT IS LIKE A WEASEL" randomly would take astronomically long
- **Cumulative Selection**: With mutation + selection, it happens in dozens of generations

This simple simulation illustrates how evolution works through small, incremental improvements that accumulate over time.

## 🎯 The Algorithm

1. **Initialize**: Start with a random 28-character string
2. **Reproduce**: Create 100 offspring, each with small random mutations
3. **Select**: Keep the offspring that best matches the target phrase
4. **Repeat**: Continue until target is reached

### Key Parameters

- **Target**: `"METHINKS IT IS LIKE A WEASEL"` (28 characters)
- **Mutation Rate**: 5% chance per character (configurable)
- **Population Size**: 100 offspring per generation (configurable)
- **Character Set**: A-Z plus space

## 🚀 Running the Simulation

```powershell
# Navigate to the project
cd advanced_examples/weasel_evolution

# Run the simulation
cargo run --release

# Run tests
cargo test

# Run with output showing every generation (verbose)
cargo run --release -- --verbose
```

## 📊 Output Examples

The program runs 4 different demonstrations:

### Example 1: Standard Evolution (5% mutation)
```
🧬 Dawkins' Weasel Program - Evolutionary Simulation
============================================================
Target:          METHINKS IT IS LIKE A WEASEL
Mutation Rate:   5.0%
Population Size: 100
============================================================

Generation 0:    WDLTMNLT DTAFO WTPDQMQ LWA L  (fitness: 2/28)
Generation 10:   METHINGS IT ISJLIPENAYWISMML  (fitness: 16/28)
Generation 20:   METHINKS IT ISJLIKE A WEASEL  (fitness: 26/28)
Generation 25:   METHINKS IT IS LIKE A WEASEL  (fitness: 28/28)

🎉 Evolution Complete!
```

### Example 2: Higher Mutation Rate (10%)
Faster changes but potentially more chaotic

### Example 3: Lower Mutation Rate (2%)
Slower but more stable convergence

### Example 4: Statistical Analysis (20 trials)
Shows average, min, max generations needed

## 🎨 Features

- ✅ **Color-coded output**: Green = correct characters, Red = incorrect
- ✅ **Multiple examples**: Different mutation rates for comparison
- ✅ **Statistical analysis**: Run multiple trials to see variability
- ✅ **Progress tracking**: Watch evolution happen in real-time
- ✅ **Comprehensive tests**: Validate algorithm correctness

## 🧪 Testing

The project includes tests for:
- Organism creation and validation
- Fitness calculation accuracy
- Mutation behavior (preserves length, respects rate)
- Evolution convergence guarantees
- Fitness monotonicity (never decreases)

```powershell
cargo test
```

## 📚 Learning Objectives

This implementation demonstrates key Rust concepts:

### Strings and Characters
- Character iteration with `.chars()`
- String building with `.collect()`
- UTF-8 string handling

### Randomness
- `rand` crate for RNG
- `gen_range()` for bounded random values
- Probability-based decisions

### Iterators
- `.map()`, `.filter()`, `.zip()`
- Functional programming patterns
- Efficient collection building

### Testing
- Unit tests for pure functions
- Property-based testing (fitness monotonicity)
- Statistical validation

### Performance
- Release mode optimization
- Efficient string operations
- Minimal allocations

## 🔬 Experimenting

Try modifying parameters to see effects:

```rust
// Faster evolution
let params = EvolutionParams {
    mutation_rate: 0.10,  // Higher mutation
    population_size: 200, // More offspring
    ..Default::default()
};

// Slower, more stable evolution
let params = EvolutionParams {
    mutation_rate: 0.01,  // Lower mutation
    population_size: 50,  // Fewer offspring
    ..Default::default()
};

// Custom target phrase
let params = EvolutionParams {
    target: "TO BE OR NOT TO BE".to_string(),
    ..Default::default()
};
```

## 🌟 Why This Matters

The Weasel Program elegantly demonstrates:

1. **Cumulative vs Random**: Small improvements compound exponentially
2. **Selection Pressure**: Keeping the best drives convergence
3. **Mutation Balance**: Too high = chaos, too low = stagnation
4. **Evolutionary Speed**: Even simple selection is remarkably efficient

This same principle applies to:
- Genetic algorithms in optimization
- Machine learning (gradient descent)
- Natural selection in biology
- Any iterative improvement process

## 📖 References

- Dawkins, Richard (1986). *The Blind Watchmaker*. Chapter 3: "Accumulating Small Change"
- [Wikipedia: Weasel Program](https://en.wikipedia.org/wiki/Weasel_program)
- [Genetic Algorithms](https://en.wikipedia.org/wiki/Genetic_algorithm)

## 🎓 Extensions to Try

Want to go further? Try implementing:

1. **Variable mutation rates**: Decrease mutation as fitness improves (simulated annealing)
2. **Sexual reproduction**: Combine two parents with crossover
3. **Multiple targets**: Evolve toward different phrases simultaneously
4. **Fitness landscapes**: Visualize how fitness changes over time
5. **Parallel evolution**: Multiple populations evolving independently
6. **Different selection strategies**: Tournament selection, roulette wheel selection

---

## Related Concepts

- **Evolutionary algorithms** - Mutation, selection, and fitness functions
- **Genetic algorithms** - Population-based optimization with crossover and mutation
- **Simulated annealing** - Optimization with gradually decreasing randomness
- **Hill climbing** - Greedy local search strategies

### Related Documentation

- **[[AoC Patterns MOC]]** - Optimization and search patterns
- **[[daily-study/Day13]]** - Genetic algorithms and evolutionary computation (if covered)

*Tags: #evolutionary-algorithm #genetic-programming #mutation #selection #cumulative-selection #dawkins*

---

**Enjoy watching evolution in action!** 🧬✨
