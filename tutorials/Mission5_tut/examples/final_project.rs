//! # Final Project: Comprehensive Hash Data Structures System
//!
//! **Objective**: Build a complete application combining all learned patterns
//!
//! **Time Estimate**: 20 minutes to understand, practice as long as needed
//!
//! **What This Demonstrates**:
//! - Integration of all HashMap and HashSet patterns
//! - Real-world application architecture
//! - Performance optimization with caching
//! - AoC-style problem solving
//! - Production-ready error handling

use std::collections::{HashMap, HashSet};
use std::time::Instant;

/// Comprehensive Text Analysis System
/// Combines frequency counting, caching, and set operations
#[derive(Debug)]
pub struct TextAnalysisEngine {
    // Core data structures
    word_frequencies: HashMap<String, usize>,
    document_index: HashMap<String, HashSet<String>>, // word -> document_ids
    document_content: HashMap<String, String>,

    // Caching layer
    analysis_cache: HashMap<String, AnalysisResult>,

    // Statistics
    total_documents: usize,
    total_words: usize,
    cache_hits: usize,
    cache_misses: usize,
}

#[derive(Debug, Clone)]
pub struct AnalysisResult {
    word_count: usize,
    unique_words: usize,
    most_common_words: Vec<(String, usize)>,
    readability_score: f32,
    computed_at: Instant,
}

impl Default for TextAnalysisEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl TextAnalysisEngine {
    pub fn new() -> Self {
        TextAnalysisEngine {
            word_frequencies: HashMap::new(),
            document_index: HashMap::new(),
            document_content: HashMap::new(),
            analysis_cache: HashMap::new(),
            total_documents: 0,
            total_words: 0,
            cache_hits: 0,
            cache_misses: 0,
        }
    }

    /// Add a document to the analysis engine
    pub fn add_document(&mut self, doc_id: String, content: String) {
        println!("📄 Adding document: {}", doc_id);

        // Store content
        self.document_content
            .insert(doc_id.clone(), content.clone());

        // Process words
        let words: Vec<String> = content
            .to_lowercase()
            .split_whitespace()
            .map(|word| word.trim_matches(|c: char| !c.is_alphabetic()))
            .filter(|word| !word.is_empty())
            .map(|word| word.to_string())
            .collect();

        // Update frequency counters
        for word in &words {
            *self.word_frequencies.entry(word.clone()).or_insert(0) += 1;
            self.document_index
                .entry(word.clone())
                .or_default()
                .insert(doc_id.clone());
        }

        self.total_words += words.len();
        self.total_documents += 1;

        // Invalidate cache for this document
        self.analysis_cache.remove(&doc_id);

        println!("  ✅ Processed {} words", words.len());
    }

    /// Perform comprehensive analysis with caching
    pub fn analyze_document(&mut self, doc_id: &str) -> Option<AnalysisResult> {
        // Check cache first
        if let Some(cached_result) = self.analysis_cache.get(doc_id) {
            self.cache_hits += 1;
            println!("  📄 Cache HIT for {}", doc_id);
            return Some(cached_result.clone());
        }

        self.cache_misses += 1;
        println!("  🔄 Cache MISS - analyzing {}", doc_id);

        // Get document content
        let content = self.document_content.get(doc_id)?;

        // Perform analysis
        let words: Vec<String> = content
            .to_lowercase()
            .split_whitespace()
            .map(|word| word.trim_matches(|c: char| !c.is_alphabetic()))
            .filter(|word| !word.is_empty())
            .map(|word| word.to_string())
            .collect();

        let word_count = words.len();
        let unique_words: HashSet<_> = words.iter().collect();
        let unique_count = unique_words.len();

        // Find most common words in this document
        let mut word_freq_local = HashMap::new();
        for word in &words {
            *word_freq_local.entry(word.clone()).or_insert(0) += 1;
        }

        let mut most_common: Vec<_> = word_freq_local.into_iter().collect();
        most_common.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
        most_common.truncate(5);

        // Calculate simple readability score
        let avg_word_length = if word_count > 0 {
            words.iter().map(|w| w.len()).sum::<usize>() as f32 / word_count as f32
        } else {
            0.0
        };
        let vocabulary_richness = if word_count > 0 {
            unique_count as f32 / word_count as f32
        } else {
            0.0
        };
        let readability_score = (avg_word_length * 10.0) + (vocabulary_richness * 100.0);

        let result = AnalysisResult {
            word_count,
            unique_words: unique_count,
            most_common_words: most_common,
            readability_score,
            computed_at: Instant::now(),
        };

        // Cache the result
        self.analysis_cache
            .insert(doc_id.to_string(), result.clone());

        Some(result)
    }

    /// Search for documents containing specific words
    pub fn search_documents(&self, query_words: &[&str]) -> Vec<String> {
        if query_words.is_empty() {
            return Vec::new();
        }

        // Start with documents containing the first word
        let mut result_docs = if let Some(docs) = self.document_index.get(query_words[0]) {
            docs.clone()
        } else {
            return Vec::new();
        };

        // Intersect with documents containing each additional word
        for &word in &query_words[1..] {
            if let Some(docs) = self.document_index.get(word) {
                result_docs = result_docs.intersection(docs).cloned().collect();
            } else {
                return Vec::new(); // No documents contain this word
            }
        }

        result_docs.into_iter().collect()
    }

    /// Find similar documents based on word overlap
    pub fn find_similar_documents(&self, doc_id: &str, threshold: f32) -> Vec<(String, f32)> {
        let doc_words = if let Some(content) = self.document_content.get(doc_id) {
            content
                .to_lowercase()
                .split_whitespace()
                .map(|word| word.trim_matches(|c: char| !c.is_alphabetic()))
                .filter(|word| !word.is_empty())
                .map(|word| word.to_string())
                .collect::<HashSet<String>>()
        } else {
            return Vec::new();
        };

        let mut similarities = Vec::new();

        for (other_doc_id, other_content) in &self.document_content {
            if other_doc_id == doc_id {
                continue;
            }

            let other_words: HashSet<String> = other_content
                .to_lowercase()
                .split_whitespace()
                .map(|word| word.trim_matches(|c: char| !c.is_alphabetic()))
                .filter(|word| !word.is_empty())
                .map(|word| word.to_string())
                .collect();

            // Calculate Jaccard similarity
            let intersection = doc_words.intersection(&other_words).count();
            let union = doc_words.union(&other_words).count();

            let similarity = if union > 0 {
                intersection as f32 / union as f32
            } else {
                0.0
            };

            if similarity >= threshold {
                similarities.push((other_doc_id.clone(), similarity));
            }
        }

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        similarities
    }

    /// Get comprehensive system statistics
    pub fn get_statistics(&self) -> SystemStats {
        SystemStats {
            total_documents: self.total_documents,
            total_words: self.total_words,
            unique_words: self.word_frequencies.len(),
            cache_hits: self.cache_hits,
            cache_misses: self.cache_misses,
            cache_hit_ratio: if self.cache_hits + self.cache_misses > 0 {
                self.cache_hits as f32 / (self.cache_hits + self.cache_misses) as f32
            } else {
                0.0
            },
            top_words: {
                let mut words: Vec<_> = self.word_frequencies.iter().collect();
                words.sort_by_key(|(_, count)| std::cmp::Reverse(**count));
                words
                    .into_iter()
                    .take(10)
                    .map(|(word, count)| (word.clone(), *count))
                    .collect()
            },
        }
    }
}

#[derive(Debug)]
pub struct SystemStats {
    pub total_documents: usize,
    pub total_words: usize,
    pub unique_words: usize,
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub cache_hit_ratio: f32,
    pub top_words: Vec<(String, usize)>,
}

/// Coordinate-based Game State System
/// Demonstrates HashMap for spatial data and game logic
#[derive(Debug)]
pub struct GameWorld {
    grid: HashMap<(i32, i32), CellType>,
    entities: HashMap<String, (i32, i32)>,
    visited_positions: HashSet<(i32, i32)>,
    path_cache: HashMap<((i32, i32), (i32, i32)), Option<Vec<(i32, i32)>>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CellType {
    Empty,
    Wall,
    Treasure,
    Monster,
    Player,
}

impl Default for GameWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl GameWorld {
    pub fn new() -> Self {
        GameWorld {
            grid: HashMap::new(),
            entities: HashMap::new(),
            visited_positions: HashSet::new(),
            path_cache: HashMap::new(),
        }
    }

    pub fn set_cell(&mut self, pos: (i32, i32), cell_type: CellType) {
        self.grid.insert(pos, cell_type);
    }

    pub fn move_entity(&mut self, entity_id: &str, new_pos: (i32, i32)) -> bool {
        // Check if move is valid
        if let Some(cell) = self.grid.get(&new_pos) {
            if *cell == CellType::Wall {
                return false;
            }
        }

        // Update entity position
        self.entities.insert(entity_id.to_string(), new_pos);
        self.visited_positions.insert(new_pos);

        // Clear path cache (positions changed)
        self.path_cache.clear();

        true
    }

    pub fn find_entities_in_radius(
        &self,
        center: (i32, i32),
        radius: i32,
    ) -> Vec<(String, (i32, i32))> {
        self.entities
            .iter()
            .filter(|(_, &(x, y))| {
                let dx = (x - center.0).abs();
                let dy = (y - center.1).abs();
                dx <= radius && dy <= radius
            })
            .map(|(id, &pos)| (id.clone(), pos))
            .collect()
    }

    pub fn get_exploration_stats(&self) -> (usize, f32) {
        let total_accessible = self
            .grid
            .iter()
            .filter(|(_, cell)| **cell != CellType::Wall)
            .count();

        let visited_count = self.visited_positions.len();
        let exploration_percentage = if total_accessible > 0 {
            visited_count as f32 / total_accessible as f32 * 100.0
        } else {
            0.0
        };

        (visited_count, exploration_percentage)
    }
}

/// Demonstration function showing all patterns working together
fn comprehensive_demo() {
    println!("🌟 COMPREHENSIVE HASH STRUCTURES DEMO");
    println!("=====================================\n");

    // Part 1: Text Analysis Engine
    println!("📚 Part 1: Text Analysis Engine");
    println!("===============================");

    let mut engine = TextAnalysisEngine::new();

    // Add sample documents
    engine.add_document(
        "doc1".to_string(),
        "Rust is a systems programming language focused on safety and performance".to_string(),
    );
    engine.add_document(
        "doc2".to_string(),
        "Python is a high-level programming language great for rapid development".to_string(),
    );
    engine.add_document(
        "doc3".to_string(),
        "JavaScript is a versatile programming language used for web development".to_string(),
    );
    engine.add_document(
        "doc4".to_string(),
        "Rust offers memory safety without garbage collection and excellent performance"
            .to_string(),
    );

    // Perform analyses (demonstrates caching)
    println!("\n🔍 Document Analysis:");
    for doc_id in ["doc1", "doc2", "doc1", "doc3", "doc1"] {
        // Note: doc1 repeated
        if let Some(analysis) = engine.analyze_document(doc_id) {
            println!(
                "  {} -> {} words, {} unique, readability: {:.1}",
                doc_id, analysis.word_count, analysis.unique_words, analysis.readability_score
            );
        }
    }

    // Search functionality
    println!("\n🔍 Search Results:");
    let search_results = engine.search_documents(&["programming", "language"]);
    println!(
        "  Documents containing 'programming' AND 'language': {:?}",
        search_results
    );

    // Find similar documents
    println!("\n🔗 Similar Documents:");
    let similar = engine.find_similar_documents("doc1", 0.1);
    for (doc_id, similarity) in similar {
        println!("  {} similar to doc1: {:.1}%", doc_id, similarity * 100.0);
    }

    // System statistics
    println!("\n📊 System Statistics:");
    let stats = engine.get_statistics();
    println!(
        "  Documents: {}, Words: {}, Unique: {}",
        stats.total_documents, stats.total_words, stats.unique_words
    );
    println!("  Cache hit ratio: {:.1}%", stats.cache_hit_ratio * 100.0);
    println!(
        "  Top words: {:?}",
        &stats.top_words[..3.min(stats.top_words.len())]
    );

    println!("\n");

    // Part 2: Game World System
    println!("🎮 Part 2: Game World System");
    println!("============================");

    let mut world = GameWorld::new();

    // Build a simple game world
    for x in 0..10 {
        for y in 0..10 {
            if x == 0 || x == 9 || y == 0 || y == 9 {
                world.set_cell((x, y), CellType::Wall);
            } else {
                world.set_cell((x, y), CellType::Empty);
            }
        }
    }

    // Add some treasures and monsters
    world.set_cell((3, 3), CellType::Treasure);
    world.set_cell((7, 7), CellType::Treasure);
    world.set_cell((5, 5), CellType::Monster);
    world.set_cell((2, 8), CellType::Monster);

    // Move player around
    println!("🚶 Moving player through world:");
    let player_path = vec![(1, 1), (2, 1), (3, 1), (3, 2), (3, 3), (4, 3), (5, 3)];

    for pos in player_path {
        if world.move_entity("player", pos) {
            println!("  Player moved to {:?}", pos);
        } else {
            println!("  Cannot move to {:?} - blocked!", pos);
        }
    }

    // Find nearby entities
    println!("\n🔍 Entities near player:");
    let nearby = world.find_entities_in_radius((5, 3), 2);
    for (entity_id, pos) in nearby {
        println!("  {} at {:?}", entity_id, pos);
    }

    // Exploration statistics
    let (visited, percentage) = world.get_exploration_stats();
    println!(
        "\n📊 Exploration: {} positions visited ({:.1}% of accessible area)",
        visited, percentage
    );

    println!("\n🎉 Demo Complete! All hash data structures working together!");
}

/// Final challenge exercises
fn final_challenges() {
    println!("\n🏆 FINAL CHALLENGES");
    println!("==================\n");

    println!("Challenge 1: Build a URL shortener service");
    println!("  - Use HashMap<String, String> for short_code -> full_url");
    println!("  - Use HashMap<String, u32> for click tracking");
    println!("  - Use HashSet<String> for blacklisted domains");
    println!("  - Add LRU cache for popular URLs\n");

    println!("Challenge 2: Create a social media hashtag analyzer");
    println!("  - Track hashtag frequencies across posts");
    println!("  - Find trending hashtags (rapid frequency increase)");
    println!("  - Group posts by hashtag combinations");
    println!("  - Cache analysis results with expiration\n");

    println!("Challenge 3: Design a multiplayer game state manager");
    println!("  - HashMap<PlayerId, Position> for player locations");
    println!("  - HashMap<RoomId, HashSet<PlayerId>> for room membership");
    println!("  - Memoized pathfinding between rooms");
    println!("  - Real-time proximity detection\n");

    println!("🎯 Pick a challenge and implement it using all the patterns you've learned!");
    println!(
        "💡 Remember: HashMap for key-value, HashSet for uniqueness, caching for performance!"
    );
}

fn main() {
    comprehensive_demo();
    final_challenges();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_analysis_engine() {
        let mut engine = TextAnalysisEngine::new();
        engine.add_document("test1".to_string(), "hello world hello".to_string());

        let analysis = engine.analyze_document("test1").unwrap();
        assert_eq!(analysis.word_count, 3);
        assert_eq!(analysis.unique_words, 2);

        let search_results = engine.search_documents(&["hello"]);
        assert_eq!(search_results, vec!["test1"]);
    }

    #[test]
    fn test_game_world() {
        let mut world = GameWorld::new();
        world.set_cell((0, 0), CellType::Empty);
        world.set_cell((0, 1), CellType::Wall);

        assert!(world.move_entity("player", (0, 0)));
        assert!(!world.move_entity("player", (0, 1))); // Wall should block

        let (visited, _) = world.get_exploration_stats();
        assert_eq!(visited, 1);
    }
}
