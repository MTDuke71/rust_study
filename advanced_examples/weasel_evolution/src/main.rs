/// Dawkins' Weasel Program - Evolutionary Algorithm Demonstration
///
/// Simulates Richard Dawkins' famous "Weasel Program" from "The Blind Watchmaker"
/// to demonstrate cumulative selection vs pure random chance.
///
/// The program evolves a random string toward the target phrase:
/// "METHINKS IT IS LIKE A WEASEL"
///
/// Key Concepts:
/// - Mutation: Random character changes in offspring
/// - Fitness: How closely a string matches the target
/// - Selection: Always pick the best offspring
/// - Cumulative Selection: Small improvements compound over generations
use colored::Colorize;
use rand::Rng;
use std::fmt;

const TARGET: &str = "METHINKS IT IS LIKE A WEASEL";
const CHARSET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ ";
const DEFAULT_MUTATION_RATE: f64 = 0.05; // 5% chance per character
const DEFAULT_POPULATION_SIZE: usize = 100;

/// Represents the evolutionary simulation parameters
#[derive(Debug, Clone)]
struct EvolutionParams {
    mutation_rate: f64,
    population_size: usize,
    target: String,
}

impl Default for EvolutionParams {
    fn default() -> Self {
        Self {
            mutation_rate: DEFAULT_MUTATION_RATE,
            population_size: DEFAULT_POPULATION_SIZE,
            target: TARGET.to_string(),
        }
    }
}

/// Represents a candidate string and its fitness
#[derive(Debug, Clone)]
struct Organism {
    genome: String,
    fitness: usize,
}

impl Organism {
    /// Create a new organism with random genome
    fn random(length: usize) -> Self {
        let mut rng = rand::thread_rng();
        let genome: String = (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET.chars().nth(idx).unwrap()
            })
            .collect();

        Self { fitness: 0, genome }
    }

    /// Create organism from existing genome
    fn from_genome(genome: String) -> Self {
        Self { genome, fitness: 0 }
    }

    /// Calculate fitness (number of correct characters in correct positions)
    fn calculate_fitness(&mut self, target: &str) {
        self.fitness = self
            .genome
            .chars()
            .zip(target.chars())
            .filter(|(a, b)| a == b)
            .count();
    }

    /// Create a mutated copy of this organism
    fn mutate(&self, mutation_rate: f64) -> Self {
        let mut rng = rand::thread_rng();
        let mutated_genome: String = self
            .genome
            .chars()
            .map(|c| {
                if rng.gen::<f64>() < mutation_rate {
                    // Mutate this character
                    let idx = rng.gen_range(0..CHARSET.len());
                    CHARSET.chars().nth(idx).unwrap()
                } else {
                    // Keep original character
                    c
                }
            })
            .collect();

        Self::from_genome(mutated_genome)
    }

    /// Display with color-coded matches
    fn display_colored(&self, target: &str) -> String {
        self.genome
            .chars()
            .zip(target.chars())
            .map(|(c, t)| {
                if c == t {
                    c.to_string().green().bold().to_string()
                } else {
                    c.to_string().red().to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("")
    }
}

impl fmt::Display for Organism {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (fitness: {})", self.genome, self.fitness)
    }
}

/// The evolutionary simulation
struct WeaselSimulation {
    params: EvolutionParams,
    current_best: Organism,
    generation: usize,
}

impl WeaselSimulation {
    /// Create a new simulation with random starting organism
    fn new(params: EvolutionParams) -> Self {
        let mut current_best = Organism::random(params.target.len());
        current_best.calculate_fitness(&params.target);

        Self {
            params,
            current_best,
            generation: 0,
        }
    }

    /// Run one generation of evolution
    fn evolve_generation(&mut self) -> bool {
        // Create offspring with mutations
        let offspring: Vec<Organism> = (0..self.params.population_size)
            .map(|_| {
                let mut child = self.current_best.mutate(self.params.mutation_rate);
                child.calculate_fitness(&self.params.target);
                child
            })
            .collect();

        // Find the best offspring
        if let Some(best_offspring) = offspring.iter().max_by_key(|o| o.fitness) {
            // Only replace if offspring is better or equal (allow lateral moves)
            if best_offspring.fitness >= self.current_best.fitness {
                self.current_best = best_offspring.clone();
            }
        }

        self.generation += 1;

        // Check if we've reached the target
        self.current_best.fitness == self.params.target.len()
    }

    /// Run the simulation until target is reached
    fn run(&mut self, verbose: bool) {
        println!(
            "\n{}",
            "🧬 Dawkins' Weasel Program - Evolutionary Simulation"
                .cyan()
                .bold()
        );
        println!("{}", "=".repeat(60).cyan());
        println!("Target:          {}", self.params.target.yellow().bold());
        println!("Mutation Rate:   {:.1}%", self.params.mutation_rate * 100.0);
        println!("Population Size: {}", self.params.population_size);
        println!("{}", "=".repeat(60).cyan());
        println!(
            "\nGeneration 0:    {}",
            self.current_best.display_colored(&self.params.target)
        );
        println!(
            "                 Fitness: {}/{}\n",
            self.current_best.fitness,
            self.params.target.len()
        );

        loop {
            let found = self.evolve_generation();

            if verbose || self.generation.is_multiple_of(10) || found {
                println!(
                    "Generation {:4}: {}",
                    self.generation,
                    self.current_best.display_colored(&self.params.target)
                );
                println!(
                    "                 Fitness: {}/{}",
                    self.current_best.fitness,
                    self.params.target.len()
                );
            }

            if found {
                println!("\n{}", "🎉 Evolution Complete!".green().bold());
                println!("{}", "=".repeat(60).green());
                println!(
                    "Final:           {}",
                    self.current_best.genome.green().bold()
                );
                println!("Generations:     {}", self.generation);
                println!("{}", "=".repeat(60).green());
                break;
            }

            // Safety: prevent infinite loops (shouldn't happen with reasonable params)
            if self.generation > 100000 {
                println!("\n⚠️  Reached maximum generations (100,000). Stopping.");
                break;
            }
        }
    }

    /// Run multiple trials and report statistics
    fn run_trials(params: EvolutionParams, num_trials: usize) {
        println!("\n{}", "📊 Running Multiple Trials".cyan().bold());
        println!("{}", "=".repeat(60).cyan());
        println!("Trials:          {}", num_trials);
        println!("Target:          {}", params.target.yellow().bold());
        println!("Mutation Rate:   {:.1}%", params.mutation_rate * 100.0);
        println!("Population Size: {}", params.population_size);
        println!("{}", "=".repeat(60).cyan());

        let mut generations_needed: Vec<usize> = Vec::new();

        for trial in 1..=num_trials {
            let mut sim = WeaselSimulation::new(params.clone());

            loop {
                let found = sim.evolve_generation();
                if found {
                    generations_needed.push(sim.generation);
                    println!(
                        "Trial {:3}: Converged in {:4} generations",
                        trial, sim.generation
                    );
                    break;
                }

                if sim.generation > 100000 {
                    println!("Trial {:3}: Failed to converge (stopped at 100,000)", trial);
                    break;
                }
            }
        }

        // Calculate statistics
        if !generations_needed.is_empty() {
            let sum: usize = generations_needed.iter().sum();
            let avg = sum as f64 / generations_needed.len() as f64;
            let min = *generations_needed.iter().min().unwrap();
            let max = *generations_needed.iter().max().unwrap();

            println!("\n{}", "Statistics:".green().bold());
            println!("  Average generations: {:.1}", avg);
            println!("  Minimum generations: {}", min);
            println!("  Maximum generations: {}", max);
            println!(
                "  Success rate:        {}/{}",
                generations_needed.len(),
                num_trials
            );
        }
    }
}

fn main() {
    // Example 1: Single run with default parameters
    println!("\n{}", "Example 1: Standard Evolution".bright_blue().bold());
    let params = EvolutionParams::default();
    let mut sim = WeaselSimulation::new(params);
    sim.run(false); // verbose = false (show every 10th generation)

    // Example 2: Run with higher mutation rate
    println!(
        "\n\n{}",
        "Example 2: Higher Mutation Rate (10%)".bright_blue().bold()
    );
    let params_high_mutation = EvolutionParams {
        mutation_rate: 0.10,
        ..Default::default()
    };
    let mut sim2 = WeaselSimulation::new(params_high_mutation);
    sim2.run(false);

    // Example 3: Run with lower mutation rate
    println!(
        "\n\n{}",
        "Example 3: Lower Mutation Rate (2%)".bright_blue().bold()
    );
    let params_low_mutation = EvolutionParams {
        mutation_rate: 0.02,
        ..Default::default()
    };
    let mut sim3 = WeaselSimulation::new(params_low_mutation);
    sim3.run(false);

    // Example 4: Statistical analysis across multiple trials
    println!(
        "\n\n{}",
        "Example 4: Statistical Analysis".bright_blue().bold()
    );
    let params_trials = EvolutionParams::default();
    WeaselSimulation::run_trials(params_trials, 20);

    println!("\n{}", "✨ All simulations complete!".bright_green().bold());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organism_creation() {
        let org = Organism::random(28);
        assert_eq!(org.genome.len(), 28);
        assert!(org.genome.chars().all(|c| CHARSET.contains(c)));
    }

    #[test]
    fn test_fitness_calculation() {
        let mut org = Organism::from_genome("METHINKS IT IS LIKE A WEASEL".to_string());
        org.calculate_fitness(TARGET);
        assert_eq!(org.fitness, 28); // Perfect match

        // "AAAAAAAAAAAAAAAAAAAAAAAAAAA " has A at position 20 (matches) and space at position 27 (matches)
        let mut org2 = Organism::from_genome("AAAAAAAAAAAAAAAAAAAAAAAAAAA ".to_string());
        org2.calculate_fitness(TARGET);
        assert_eq!(org2.fitness, 2); // Positions 20 ('A') and 27 (' ') match
    }

    #[test]
    fn test_mutation_preserves_length() {
        let org = Organism::random(28);
        let mutated = org.mutate(0.5);
        assert_eq!(mutated.genome.len(), org.genome.len());
    }

    #[test]
    fn test_mutation_rate_zero() {
        let org = Organism::from_genome("HELLO WORLD".to_string());
        let mutated = org.mutate(0.0); // 0% mutation
        assert_eq!(org.genome, mutated.genome);
    }

    #[test]
    fn test_evolution_converges() {
        let params = EvolutionParams {
            mutation_rate: 0.05,
            population_size: 100,
            target: "HELLO".to_string(),
        };

        let mut sim = WeaselSimulation::new(params);

        // Run for limited generations
        for _ in 0..10000 {
            if sim.evolve_generation() {
                assert_eq!(sim.current_best.genome, "HELLO");
                return; // Success
            }
        }

        // Should converge within 10000 generations for short target
        panic!("Evolution did not converge in 10000 generations");
    }

    #[test]
    fn test_fitness_never_decreases() {
        let params = EvolutionParams::default();
        let mut sim = WeaselSimulation::new(params);

        let mut previous_fitness = sim.current_best.fitness;

        for _ in 0..100 {
            sim.evolve_generation();
            assert!(
                sim.current_best.fitness >= previous_fitness,
                "Fitness decreased! {} -> {}",
                previous_fitness,
                sim.current_best.fitness
            );
            previous_fitness = sim.current_best.fitness;
        }
    }
}
