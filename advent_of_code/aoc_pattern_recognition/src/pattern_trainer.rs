//! # Pattern Recognition Trainer Module
//!
//! Interactive training system to help recognize AoC patterns from problem descriptions.
//! This module provides exercises and feedback to build pattern recognition skills.
//!
//! ## Requirements Satisfied: REQ-P5
//!
//! ### Training Categories:
//! 1. **Pattern Identification**: Given a problem, identify which patterns apply
//! 2. **Implementation Speed**: Practice implementing patterns quickly
//! 3. **Performance Analysis**: Understanding when to use each pattern
//! 4. **Real AoC Problems**: Practice with actual past problems

use crate::grid_patterns::*;
use crate::parsing_patterns::*;
use crate::{AocPattern, PatternResult};

/// Training exercise for pattern recognition
#[derive(Debug, Clone)]
pub struct TrainingExercise {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub input_sample: String,
    pub expected_patterns: Vec<String>,
    pub difficulty: TrainingDifficulty,
    pub aoc_reference: Option<String>, // e.g., "AoC 2020 Day 15"
}

/// Difficulty levels for training exercises
#[derive(Debug, Clone, PartialEq)]
pub enum TrainingDifficulty {
    Beginner,     // Single pattern, obvious choice
    Intermediate, // Multiple patterns, clear primary
    Advanced,     // Multiple patterns, optimization required
    Expert,       // Multiple patterns, non-obvious combination
}

impl TrainingDifficulty {
    pub fn time_limit_seconds(&self) -> u32 {
        match self {
            TrainingDifficulty::Beginner => 300,     // 5 minutes
            TrainingDifficulty::Intermediate => 600, // 10 minutes
            TrainingDifficulty::Advanced => 1200,    // 20 minutes
            TrainingDifficulty::Expert => 1800,      // 30 minutes
        }
    }
}

/// Pattern recognition trainer
pub struct PatternTrainer {
    exercises: Vec<TrainingExercise>,
    current_exercise: Option<usize>,
    score: TrainingScore,
}

/// Training score tracking
#[derive(Debug, Default)]
pub struct TrainingScore {
    pub correct_identifications: usize,
    pub total_attempts: usize,
    pub average_time_seconds: f64,
    pub patterns_mastered: Vec<String>,
}

impl TrainingScore {
    pub fn accuracy(&self) -> f64 {
        if self.total_attempts > 0 {
            self.correct_identifications as f64 / self.total_attempts as f64
        } else {
            0.0
        }
    }

    pub fn add_attempt(&mut self, correct: bool, time_seconds: f64) {
        self.total_attempts += 1;
        if correct {
            self.correct_identifications += 1;
        }

        // Update rolling average
        let total_time =
            self.average_time_seconds * (self.total_attempts - 1) as f64 + time_seconds;
        self.average_time_seconds = total_time / self.total_attempts as f64;
    }
}

impl PatternTrainer {
    /// Create new pattern trainer with built-in exercises
    ///
    /// # Requirements Satisfied: REQ-P5
    /// Provides comprehensive pattern recognition training
    pub fn new() -> Self {
        let exercises = Self::create_training_exercises();

        Self {
            exercises,
            current_exercise: None,
            score: TrainingScore::default(),
        }
    }

    /// Get next training exercise
    pub fn next_exercise(&mut self) -> Option<&TrainingExercise> {
        if let Some(current) = self.current_exercise {
            if current + 1 < self.exercises.len() {
                self.current_exercise = Some(current + 1);
            } else {
                self.current_exercise = None;
                return None;
            }
        } else {
            self.current_exercise = Some(0);
        }

        self.current_exercise
            .and_then(|idx| self.exercises.get(idx))
    }

    /// Get exercise by difficulty level
    pub fn exercises_by_difficulty(
        &self,
        difficulty: TrainingDifficulty,
    ) -> Vec<&TrainingExercise> {
        self.exercises
            .iter()
            .filter(|ex| ex.difficulty == difficulty)
            .collect()
    }

    /// Submit answer for current exercise
    ///
    /// # Requirements Satisfied: REQ-P5
    /// Provides feedback on pattern recognition accuracy
    pub fn submit_answer(
        &mut self,
        identified_patterns: Vec<String>,
        time_seconds: f64,
    ) -> TrainingResult {
        if let Some(current_idx) = self.current_exercise {
            if let Some(exercise) = self.exercises.get(current_idx) {
                let correct =
                    self.evaluate_answer(&exercise.expected_patterns, &identified_patterns);
                self.score.add_attempt(correct, time_seconds);

                let feedback = if correct {
                    "Correct! You identified the right patterns.".to_string()
                } else {
                    format!(
                        "Not quite. Expected: {:?}, You identified: {:?}",
                        exercise.expected_patterns, identified_patterns
                    )
                };

                return TrainingResult {
                    correct,
                    feedback,
                    expected_patterns: exercise.expected_patterns.clone(),
                    identified_patterns,
                    time_taken: time_seconds,
                    hints: self.generate_hints(exercise),
                };
            }
        }

        TrainingResult {
            correct: false,
            feedback: "No active exercise".to_string(),
            expected_patterns: vec![],
            identified_patterns: vec![],
            time_taken: time_seconds,
            hints: vec![],
        }
    }

    /// Get current training statistics
    pub fn get_stats(&self) -> &TrainingScore {
        &self.score
    }

    /// Reset training progress
    pub fn reset(&mut self) {
        self.current_exercise = None;
        self.score = TrainingScore::default();
    }

    fn evaluate_answer(&self, expected: &[String], identified: &[String]) -> bool {
        // Check if at least one expected pattern was identified
        expected
            .iter()
            .any(|exp| identified.iter().any(|id| id.contains(exp)))
    }

    fn generate_hints(&self, exercise: &TrainingExercise) -> Vec<String> {
        let mut hints = Vec::new();

        // Analyze input sample for hints
        if exercise.input_sample.lines().any(|line| line.contains(',')) {
            hints.push(
                "Notice the comma-separated values - consider coordinate parsing".to_string(),
            );
        }

        if exercise.description.to_lowercase().contains("grid")
            || exercise.description.to_lowercase().contains("map")
        {
            hints.push("Grid-based problem - consider pathfinding or flood fill".to_string());
        }

        if exercise.description.to_lowercase().contains("shortest")
            || exercise.description.to_lowercase().contains("path")
        {
            hints.push("Shortest path problem - BFS or Dijkstra might be needed".to_string());
        }

        if exercise.description.to_lowercase().contains("repeat")
            || exercise.description.to_lowercase().contains("cycle")
        {
            hints.push("Look for repeating patterns - cycle detection could be useful".to_string());
        }

        hints
    }

    /// Create comprehensive set of training exercises
    ///
    /// # Requirements Satisfied: REQ-P5
    /// Based on real AoC problem patterns
    fn create_training_exercises() -> Vec<TrainingExercise> {
        vec![
            // Beginner exercises
            TrainingExercise {
                id: 1,
                title: "Simple Coordinate Navigation".to_string(),
                description: "Given a list of coordinates, find the Manhattan distance between the first and last points.".to_string(),
                input_sample: "1,2\n3,4\n5,6".to_string(),
                expected_patterns: vec!["Coordinate parsing".to_string()],
                difficulty: TrainingDifficulty::Beginner,
                aoc_reference: Some("AoC 2018 Day 6 (simplified)".to_string()),
            },

            TrainingExercise {
                id: 2,
                title: "Instruction Following".to_string(),
                description: "Process a series of movement commands and track final position.".to_string(),
                input_sample: "forward 5\ndown 3\nup 1".to_string(),
                expected_patterns: vec!["Instruction parsing".to_string()],
                difficulty: TrainingDifficulty::Beginner,
                aoc_reference: Some("AoC 2021 Day 2".to_string()),
            },

            TrainingExercise {
                id: 3,
                title: "Grid Pathfinding".to_string(),
                description: "Find the shortest path through a grid from start to end, avoiding obstacles (#).".to_string(),
                input_sample: "..#.\n.#..\n..#.\nS..E".to_string(),
                expected_patterns: vec!["Grid parsing".to_string(), "Shortest path".to_string()],
                difficulty: TrainingDifficulty::Intermediate,
                aoc_reference: Some("AoC 2021 Day 15".to_string()),
            },

            TrainingExercise {
                id: 4,
                title: "Connected Regions".to_string(),
                description: "Count the number of connected regions of the same character in a grid.".to_string(),
                input_sample: "AAA\nABA\nAAA".to_string(),
                expected_patterns: vec!["Grid parsing".to_string(), "Flood fill".to_string()],
                difficulty: TrainingDifficulty::Intermediate,
                aoc_reference: Some("AoC 2017 Day 14".to_string()),
            },

            TrainingExercise {
                id: 5,
                title: "Recursive Sequence".to_string(),
                description: "Calculate the 1000th number in a Fibonacci-like sequence where each number depends on the previous two.".to_string(),
                input_sample: "f(0) = 1\nf(1) = 1\nf(n) = f(n-1) + f(n-2)".to_string(),
                expected_patterns: vec!["Memoization".to_string(), "Dynamic programming".to_string()],
                difficulty: TrainingDifficulty::Advanced,
                aoc_reference: Some("AoC 2021 Day 6".to_string()),
            },

            TrainingExercise {
                id: 6,
                title: "Simulation with Cycles".to_string(),
                description: "Simulate a cellular automaton for 1 billion iterations and find the final state.".to_string(),
                input_sample: ".#.\n#.#\n.#.".to_string(),
                expected_patterns: vec!["Grid parsing".to_string(), "State tracking".to_string(), "Cycle detection".to_string()],
                difficulty: TrainingDifficulty::Expert,
                aoc_reference: Some("AoC 2020 Day 17".to_string()),
            },

            TrainingExercise {
                id: 7,
                title: "Complex Parsing Challenge".to_string(),
                description: "Parse a complex input format with multiple sections and nested structures.".to_string(),
                input_sample: "[section1]\ndata: value1\nmore: value2\n\n[section2]\nlist: a,b,c\ncount: 42".to_string(),
                expected_patterns: vec!["Structured parsing".to_string(), "Key-value parsing".to_string()],
                difficulty: TrainingDifficulty::Advanced,
                aoc_reference: Some("AoC 2020 Day 4".to_string()),
            },

            TrainingExercise {
                id: 8,
                title: "Optimization Problem".to_string(),
                description: "Given items with weights and values, find the maximum value that fits in a knapsack.".to_string(),
                input_sample: "item1: weight=10, value=60\nitem2: weight=20, value=100\nitem3: weight=30, value=120\ncapacity: 50".to_string(),
                expected_patterns: vec!["Number extraction".to_string(), "Dynamic programming".to_string()],
                difficulty: TrainingDifficulty::Expert,
                aoc_reference: Some("Generic knapsack pattern".to_string()),
            },
        ]
    }
}

impl Default for PatternTrainer {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a training exercise attempt
#[derive(Debug)]
pub struct TrainingResult {
    pub correct: bool,
    pub feedback: String,
    pub expected_patterns: Vec<String>,
    pub identified_patterns: Vec<String>,
    pub time_taken: f64,
    pub hints: Vec<String>,
}

/// Interactive pattern recognition quiz
///
/// # Requirements Satisfied: REQ-P5
/// Provides quick pattern identification practice
pub struct PatternQuiz {
    questions: Vec<QuizQuestion>,
    current_question: usize,
    score: usize,
}

#[derive(Debug, Clone)]
pub struct QuizQuestion {
    pub question: String,
    pub options: Vec<String>,
    pub correct_answer: usize,
    pub explanation: String,
}

impl PatternQuiz {
    /// Create new pattern recognition quiz
    pub fn new() -> Self {
        let questions = Self::create_quiz_questions();

        Self {
            questions,
            current_question: 0,
            score: 0,
        }
    }

    /// Get current question
    pub fn current_question(&self) -> Option<&QuizQuestion> {
        self.questions.get(self.current_question)
    }

    /// Submit answer and move to next question
    pub fn submit_answer(&mut self, answer: usize) -> bool {
        let correct = if let Some(question) = self.questions.get(self.current_question) {
            answer == question.correct_answer
        } else {
            false
        };

        if correct {
            self.score += 1;
        }

        self.current_question += 1;
        correct
    }

    /// Get final score
    pub fn final_score(&self) -> (usize, usize) {
        (self.score, self.questions.len())
    }

    /// Check if quiz is complete
    pub fn is_complete(&self) -> bool {
        self.current_question >= self.questions.len()
    }

    fn create_quiz_questions() -> Vec<QuizQuestion> {
        vec![
            QuizQuestion {
                question: "You need to find the shortest path in a grid with obstacles. Which pattern is most appropriate?".to_string(),
                options: vec![
                    "Flood fill".to_string(),
                    "BFS shortest path".to_string(),
                    "Memoization".to_string(),
                    "Cycle detection".to_string(),
                ],
                correct_answer: 1,
                explanation: "BFS guarantees shortest path in unweighted grids".to_string(),
            },

            QuizQuestion {
                question: "Your problem involves calculating f(1000000) where f(n) = f(n-1) + f(n-2). What pattern helps?".to_string(),
                options: vec![
                    "Grid parsing".to_string(),
                    "Instruction parsing".to_string(),
                    "Memoization".to_string(),
                    "Set operations".to_string(),
                ],
                correct_answer: 2,
                explanation: "Memoization prevents recalculating the same values repeatedly".to_string(),
            },

            QuizQuestion {
                question: "You need to simulate 1 billion iterations but suspect patterns repeat. Which approach?".to_string(),
                options: vec![
                    "Brute force simulation".to_string(),
                    "Cycle detection".to_string(),
                    "Binary search".to_string(),
                    "Flood fill".to_string(),
                ],
                correct_answer: 1,
                explanation: "Cycle detection lets you skip ahead once you find the repeating pattern".to_string(),
            },

            QuizQuestion {
                question: "Input format: 'move 5 from stack2 to stack1'. Best parsing approach?".to_string(),
                options: vec![
                    "Regex parsing".to_string(),
                    "Character-by-character parsing".to_string(),
                    "Instruction parsing".to_string(),
                    "Key-value parsing".to_string(),
                ],
                correct_answer: 2,
                explanation: "Instruction parsing handles command + parameters format efficiently".to_string(),
            },
        ]
    }
}

impl Default for PatternQuiz {
    fn default() -> Self {
        Self::new()
    }
}

/// Performance benchmarking for pattern implementations
///
/// # Requirements Satisfied: REQ-P4, REQ-P5
/// Helps understand when each pattern is appropriate
pub struct PatternBenchmark;

impl PatternBenchmark {
    /// Benchmark different parsing approaches
    ///
    /// # Requirements Satisfied: REQ-P4
    /// Compare performance of parsing patterns
    pub fn benchmark_parsing(input: &str, iterations: usize) -> PatternResult<BenchmarkResults> {
        use std::time::Instant;

        let mut results = BenchmarkResults::new();

        // Benchmark simple line parsing
        let start = Instant::now();
        for _ in 0..iterations {
            let parser = SimpleLineParser;
            let _ = parser.parse_lines(input)?;
        }
        results.add_result(
            "Simple Line Parser",
            start.elapsed().as_nanos() as f64 / iterations as f64,
        );

        // Benchmark number extraction
        let start = Instant::now();
        for _ in 0..iterations {
            let parser = NumberExtractionParser;
            let _ = parser.parse_lines(input)?;
        }
        results.add_result(
            "Number Extraction",
            start.elapsed().as_nanos() as f64 / iterations as f64,
        );

        Ok(results)
    }

    /// Benchmark grid algorithms
    pub fn benchmark_grid_algorithms(grid: &Grid<char>, iterations: usize) -> BenchmarkResults {
        use std::time::Instant;

        let mut results = BenchmarkResults::new();
        let start_coord = grid
            .cells
            .keys()
            .next()
            .copied()
            .unwrap_or(Coord::new(0, 0));

        // Benchmark flood fill
        let start = Instant::now();
        for _ in 0..iterations {
            let pattern = FloodFillPattern;
            let _ = pattern.solve((grid.clone(), start_coord, '.'));
        }
        results.add_result(
            "Flood Fill",
            start.elapsed().as_nanos() as f64 / iterations as f64,
        );

        results
    }
}

/// Benchmark results container
#[derive(Debug)]
pub struct BenchmarkResults {
    results: Vec<(String, f64)>, // (pattern_name, avg_nanoseconds)
}

impl BenchmarkResults {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    pub fn add_result(&mut self, pattern_name: &str, avg_nanoseconds: f64) {
        self.results
            .push((pattern_name.to_string(), avg_nanoseconds));
    }

    pub fn fastest(&self) -> Option<&(String, f64)> {
        self.results
            .iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }

    pub fn slowest(&self) -> Option<&(String, f64)> {
        self.results
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }

    pub fn results(&self) -> &[(String, f64)] {
        &self.results
    }
}

impl Default for BenchmarkResults {
    fn default() -> Self {
        Self::new()
    }
}
