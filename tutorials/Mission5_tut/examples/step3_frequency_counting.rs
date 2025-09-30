//! # Step 3: Frequency Counting Patterns
//! 
//! **Learning Objective**: Build statistical analysis tools using HashMap frequency patterns
//! 
//! **Time Estimate**: 30 minutes
//! 
//! **What You'll Learn**:
//! - Word frequency analysis for text processing
//! - Character and n-gram counting techniques
//! - Statistical analysis with HashMap (most common, percentiles)
//! - Frequency-based sorting and ranking
//! - Building reusable counter utilities

use std::collections::HashMap;

/// Example 1: Basic Word Frequency Counter
/// The foundation pattern for all frequency analysis
fn example_word_frequency() {
    println!("🦀 Example 1: Word Frequency Counter");
    println!("====================================");
    
    let text = "the quick brown fox jumps over the lazy dog the fox is quick";
    let mut word_count = HashMap::new();
    
    // Count each word
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("📊 Word frequencies:");
    for (word, count) in &word_count {
        println!("  '{}' -> {}", word, count);
    }
    
    // Find most common word
    if let Some((most_common_word, max_count)) = word_count.iter().max_by_key(|(_, &count)| count) {
        println!("🏆 Most common: '{}' appears {} times", most_common_word, max_count);
    }
    
    println!();
}

/// Example 2: Character Frequency Analysis
/// Essential for cryptography and text analysis problems
fn example_character_frequency() {
    println!("🔤 Example 2: Character Frequency");
    println!("=================================");
    
    let text = "Hello Programming World!";
    let mut char_count = HashMap::new();
    
    // Count characters (case-insensitive, skip spaces and punctuation)
    for ch in text.chars() {
        if ch.is_alphabetic() {
            let lower_ch = ch.to_lowercase().next().unwrap();
            let count = char_count.entry(lower_ch).or_insert(0);
            *count += 1;
        }
    }
    
    // Sort by frequency (highest first)
    let mut sorted_chars: Vec<_> = char_count.iter().collect();
    sorted_chars.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));
    
    println!("📈 Character frequencies (sorted by count):");
    for (ch, count) in sorted_chars {
        println!("  '{}': {}", ch, count);
    }
    
    // Calculate letter distribution
    let total_letters: u32 = char_count.values().sum();
    println!("\n📊 Letter distribution:");
    for (ch, count) in char_count.iter() {
        let percentage = (*count as f32 / total_letters as f32) * 100.0;
        println!("  '{}': {:.1}%", ch, percentage);
    }
    
    println!();
}

/// Example 3: Advanced Frequency Counter
/// A reusable struct for complex frequency analysis
#[derive(Debug)]
struct FrequencyCounter<T> {
    counts: HashMap<T, usize>,
    total: usize,
}

impl<T> FrequencyCounter<T> 
where 
    T: std::hash::Hash + Eq + Clone,
{
    fn new() -> Self {
        FrequencyCounter {
            counts: HashMap::new(),
            total: 0,
        }
    }
    
    fn add(&mut self, item: T) {
        *self.counts.entry(item).or_insert(0) += 1;
        self.total += 1;
    }
    
    fn add_multiple(&mut self, item: T, count: usize) {
        *self.counts.entry(item).or_insert(0) += count;
        self.total += count;
    }
    
    fn count(&self, item: &T) -> usize {
        self.counts.get(item).copied().unwrap_or(0)
    }
    
    fn total(&self) -> usize {
        self.total
    }
    
    fn most_common(&self, n: usize) -> Vec<(T, usize)> {
        let mut items: Vec<_> = self.counts.iter()
            .map(|(k, &v)| (k.clone(), v))
            .collect();
        items.sort_by_key(|&(_, count)| std::cmp::Reverse(count));
        items.into_iter().take(n).collect()
    }
    
    fn percentage(&self, item: &T) -> f32 {
        if self.total == 0 { return 0.0; }
        (self.count(item) as f32 / self.total as f32) * 100.0
    }
}

fn example_advanced_counter() {
    println!("⚡ Example 3: Advanced Frequency Counter");
    println!("=======================================");
    
    let mut word_counter = FrequencyCounter::new();
    
    // Analyze multiple texts
    let texts = vec![
        "rust programming is fun",
        "python programming is powerful", 
        "programming languages are diverse",
        "rust performance is excellent"
    ];
    
    for text in texts {
        for word in text.split_whitespace() {
            word_counter.add(word.to_string());
        }
    }
    
    println!("📊 Analysis of {} words:", word_counter.total());
    
    // Top 5 most common words
    let top_words = word_counter.most_common(5);
    println!("\n🏆 Top 5 most common words:");
    for (i, (word, count)) in top_words.iter().enumerate() {
        println!("  {}. '{}' - {} times ({:.1}%)", 
                 i + 1, word, count, word_counter.percentage(word));
    }
    
    // Specific word analysis
    let target_words = vec!["rust".to_string(), "python".to_string(), "programming".to_string()];
    println!("\n🎯 Specific word analysis:");
    for word in target_words {
        let count = word_counter.count(&word);
        let pct = word_counter.percentage(&word);
        println!("  '{}': {} occurrences ({:.1}%)", word, count, pct);
    }
    
    println!();
}

/// Example 4: N-gram Frequency Analysis
/// Advanced pattern for context-aware text analysis
fn example_ngram_analysis() {
    println!("🔍 Example 4: N-gram Analysis");
    println!("=============================");
    
    let text = "the quick brown fox jumps over the lazy dog";
    let words: Vec<&str> = text.split_whitespace().collect();
    
    // Bigram analysis (2-word phrases)
    let mut bigram_count = HashMap::new();
    for window in words.windows(2) {
        let bigram = format!("{} {}", window[0], window[1]);
        *bigram_count.entry(bigram).or_insert(0) += 1;
    }
    
    println!("📝 Bigrams found:");
    for (bigram, count) in &bigram_count {
        println!("  '{}' -> {}", bigram, count);
    }
    
    // Trigram analysis (3-word phrases)
    let mut trigram_count = HashMap::new();
    for window in words.windows(3) {
        let trigram = format!("{} {} {}", window[0], window[1], window[2]);
        *trigram_count.entry(trigram).or_insert(0) += 1;
    }
    
    println!("\n📝 Trigrams found:");
    for (trigram, count) in &trigram_count {
        println!("  '{}' -> {}", trigram, count);
    }
    
    println!();
}

/// Example 5: Real-World Application - Log Analysis
/// Practical example showing frequency analysis in system monitoring
fn example_log_analysis() {
    println!("📋 Example 5: Log Analysis");
    println!("==========================");
    
    // Simulated log entries
    let log_entries = vec![
        "INFO user_login alice",
        "ERROR connection_failed 192.168.1.100",
        "INFO user_login bob",
        "WARN slow_query database",
        "ERROR connection_failed 192.168.1.100",
        "INFO user_logout alice",
        "ERROR connection_failed 192.168.1.101",
        "INFO user_login charlie",
        "WARN slow_query database",
        "ERROR connection_failed 192.168.1.100",
    ];
    
    let mut level_count = HashMap::new();
    let mut error_types = HashMap::new();
    let mut user_activities = HashMap::new();
    
    for entry in log_entries {
        let parts: Vec<&str> = entry.split_whitespace().collect();
        if parts.len() >= 2 {
            let level = parts[0];
            let event_type = parts[1];
            
            // Count log levels
            *level_count.entry(level).or_insert(0) += 1;
            
            // Track error types
            if level == "ERROR" {
                *error_types.entry(event_type).or_insert(0) += 1;
            }
            
            // Track user activities
            if event_type.starts_with("user_") && parts.len() >= 3 {
                let user = parts[2];
                *user_activities.entry(user).or_insert(0) += 1;
            }
        }
    }
    
    println!("📊 Log Level Distribution:");
    for (level, count) in &level_count {
        println!("  {} -> {}", level, count);
    }
    
    println!("\n🚨 Error Type Analysis:");
    for (error, count) in &error_types {
        println!("  {} -> {} occurrences", error, count);
    }
    
    println!("\n👤 User Activity:");
    for (user, count) in &user_activities {
        println!("  {} -> {} actions", user, count);
    }
    
    // Find most problematic error
    if let Some((worst_error, max_count)) = error_types.iter().max_by_key(|(_, &count)| count) {
        println!("\n⚠️  Most frequent error: {} ({} times)", worst_error, max_count);
    }
    
    println!();
}

// 🧪 EXERCISE TIME!

/// EXERCISE 1: Grade Distribution Analyzer
/// 
/// **Task**: Analyze student grade distributions and calculate statistics
/// **Challenge**: Find grade ranges, averages, and performance insights
fn exercise_grade_analysis() {
    println!("🧪 EXERCISE 1: Grade Analysis");
    println!("=============================");
    
    let grades = vec![85, 92, 78, 96, 88, 92, 85, 90, 78, 94, 88, 92, 85, 90, 96];
    

    let mut grade_freq = HashMap::new();
    let mut range_freq = HashMap::new();
    
    for &grade in &grades {
        *grade_freq.entry(grade).or_insert(0) += 1;
        
        let range = match grade {
            90..=100 => "A (90-100)",
            80..=89 => "B (80-89)",
            70..=79 => "C (70-79)",
            60..=69 => "D (60-69)",
            _ => "F (0-59)",
        };
        *range_freq.entry(range).or_insert(0) += 1;
    }
    
    println!("📊 Grade frequencies:");
    for (grade, count) in &grade_freq {
        println!("  {} -> {}", grade, count);
    }
    
    println!("\n📈 Grade distribution:");
    for (range, count) in &range_freq {
        let percentage = (*count as f32 / grades.len() as f32) * 100.0;
        println!("  {}: {} students ({:.1}%)", range, count, percentage);
    }
    
    println!("✅ Exercise 1 complete!\n");
}

/// EXERCISE 2: DNA Sequence Analyzer
/// 
/// **Task**: Analyze DNA sequences for nucleotide frequencies and patterns
/// **Challenge**: Find GC content, most common codons, mutation hotspots
fn exercise_dna_analysis() {
    println!("🧪 EXERCISE 2: DNA Analysis");
    println!("===========================");
    
        let dna_sequence = "ATCGATCGATCGTAGCTAGCTAGCATCGATCGTAGC";
        let mut nucleotide_count = HashMap::new();
        
        for nucleotide in dna_sequence.chars() {
            *nucleotide_count.entry(nucleotide).or_insert(0) += 1;
        }
        
        println!("🧬 Nucleotide frequencies:");
        for (nucleotide, count) in &nucleotide_count {
            let percentage = (*count as f32 / dna_sequence.len() as f32) * 100.0;
            println!("  {}: {} ({:.1}%)", nucleotide, count, percentage);
        }
        
        let gc_count = nucleotide_count.get(&'G').unwrap_or(&0) + nucleotide_count.get(&'C').unwrap_or(&0);
        let gc_content = (gc_count as f32 / dna_sequence.len() as f32) * 100.0;
        println!("💡 GC Content: {:.1}%", gc_content);
        
        let mut codon_count = HashMap::new();
        for codon in dna_sequence.chars().collect::<Vec<_>>().chunks(3) {
            if codon.len() == 3 {
                let codon_str: String = codon.iter().collect();
                *codon_count.entry(codon_str).or_insert(0) += 1;
            }
        }
        
        if let Some((most_common_codon, count)) = codon_count.iter().max_by_key(|(_, &count)| count) {
            println!("🎯 Most common codon: {} (appears {} times)", most_common_codon, count);
        }
    
    println!("✅ Exercise 2 complete!\n");
}

/// EXERCISE 3: Web Analytics Simulator
/// 
/// **Task**: Build a simple web analytics system using frequency counting
/// **Challenge**: Track page views, user agents, and referrers
fn exercise_web_analytics() {
    println!("🧪 EXERCISE 3: Web Analytics");
    println!("============================");
    
    let page_visits = vec![
        ("/home", "Mozilla Firefox"),
        ("/about", "Chrome"),
        ("/home", "Safari"),
        ("/contact", "Mozilla Firefox"),
        ("/home", "Chrome"),
        ("/products", "Safari"),
        ("/home", "Mozilla Firefox"),
        ("/about", "Chrome"),
    ];
    
    let mut page_view_freq = HashMap::with_capacity(page_visits.len());
    for &(page, _) in &page_visits {
        *page_view_freq.entry(page).or_insert(0) += 1;
    }

    println!("?? Page view frequencies:");
    for (page, count) in &page_view_freq {
        println!("  {} -> {}", page, count);
    }
    let mut browser_freq = HashMap::with_capacity(page_visits.len());
    for &(_, browser) in &page_visits {
        *browser_freq.entry(browser).or_insert(0) += 1;
    }

    println!("?? Browser frequencies:");
    for (browser, count) in &browser_freq {
        println!("  {} -> {}", browser, count);
    }
    if let Some((page, count)) = page_view_freq.iter().max_by_key(|(_, &count)| count) {
        println!("?? Most popular page: {} ({} visits)", page, count);
    }
    if let Some((browser, count)) = browser_freq.iter().max_by_key(|(_, &count)| count) {
        println!("?? Most popular browser: {} ({} visits)", browser, count);
    }
    let single_page_visits = page_view_freq.values().filter(|&count| *count == 1).count();
    let total_visits = page_visits.len();

    let bounce_rate = if total_visits == 0 {
        0.0
    } else {
        (single_page_visits as f32 / total_visits as f32) * 100.0
    };

    println!("?? Bounce rate: {:.1}%", bounce_rate);

    println!("✅ Exercise 3 complete!\n");
}

// 📝 SOLUTION SECTION

#[cfg(feature = "solutions")]
mod solutions {
    use super::*;
    
    fn exercise_grade_analysis_solution() {
        let grades = vec![85, 92, 78, 96, 88, 92, 85, 90, 78, 94, 88, 92, 85, 90, 96];
        let mut grade_freq = HashMap::new();
        let mut range_freq = HashMap::new();
        
        for &grade in &grades {
            *grade_freq.entry(grade).or_insert(0) += 1;
            
            let range = match grade {
                90..=100 => "A (90-100)",
                80..=89 => "B (80-89)",
                70..=79 => "C (70-79)",
                60..=69 => "D (60-69)",
                _ => "F (0-59)",
            };
            *range_freq.entry(range).or_insert(0) += 1;
        }
        
        println!("📊 Grade frequencies:");
        for (grade, count) in &grade_freq {
            println!("  {} -> {}", grade, count);
        }
        
        println!("\n📈 Grade distribution:");
        for (range, count) in &range_freq {
            let percentage = (*count as f32 / grades.len() as f32) * 100.0;
            println!("  {}: {} students ({:.1}%)", range, count, percentage);
        }
    }
    
    fn exercise_dna_analysis_solution() {
        let dna_sequence = "ATCGATCGATCGTAGCTAGCTAGCATCGATCGTAGC";
        let mut nucleotide_count = HashMap::new();
        
        for nucleotide in dna_sequence.chars() {
            *nucleotide_count.entry(nucleotide).or_insert(0) += 1;
        }
        
        println!("🧬 Nucleotide frequencies:");
        for (nucleotide, count) in &nucleotide_count {
            let percentage = (*count as f32 / dna_sequence.len() as f32) * 100.0;
            println!("  {}: {} ({:.1}%)", nucleotide, count, percentage);
        }
        
        let gc_count = nucleotide_count.get(&'G').unwrap_or(&0) + nucleotide_count.get(&'C').unwrap_or(&0);
        let gc_content = (gc_count as f32 / dna_sequence.len() as f32) * 100.0;
        println!("💡 GC Content: {:.1}%", gc_content);
        
        let mut codon_count = HashMap::new();
        for codon in dna_sequence.chars().collect::<Vec<_>>().chunks(3) {
            if codon.len() == 3 {
                let codon_str: String = codon.iter().collect();
                *codon_count.entry(codon_str).or_insert(0) += 1;
            }
        }
        
        if let Some((most_common_codon, count)) = codon_count.iter().max_by_key(|(_, &count)| count) {
            println!("🎯 Most common codon: {} (appears {} times)", most_common_codon, count);
        }
    }
}

/// Main function - runs all examples and exercises
fn main() {
    println!("🦀 Step 3: Frequency Counting Patterns");
    println!("=======================================\n");
    
    // Run all examples
    example_word_frequency();
    example_character_frequency();
    example_advanced_counter();
    example_ngram_analysis();
    example_log_analysis();
    
    // Exercise time!
    println!("🎯 Now it's your turn! Complete these exercises:");
    println!("================================================\n");
    
    exercise_grade_analysis();
    exercise_dna_analysis();
    exercise_web_analytics();
    
    println!("🎉 Congratulations! You've completed Step 3!");
    println!("📚 Ready for Step 4? Run: cargo run --example step4_multi_value_patterns");
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_frequency_counter() {
        let mut counter = FrequencyCounter::new();
        counter.add("apple");
        counter.add("banana");
        counter.add("apple");
        
        assert_eq!(counter.count(&"apple"), 2);
        assert_eq!(counter.count(&"banana"), 1);
        assert_eq!(counter.total(), 3);
        
        let most_common = counter.most_common(1);
        assert_eq!(most_common[0].0, "apple");
        assert_eq!(most_common[0].1, 2);
    }
    
    #[test]
    fn test_word_frequency() {
        let text = "hello world hello";
        let mut word_count = HashMap::new();
        
        for word in text.split_whitespace() {
            *word_count.entry(word).or_insert(0) += 1;
        }
        
        assert_eq!(word_count.get("hello"), Some(&2));
        assert_eq!(word_count.get("world"), Some(&1));
    }
}



