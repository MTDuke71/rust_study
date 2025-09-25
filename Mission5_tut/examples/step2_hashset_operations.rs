//! # Step 2: HashSet Operations
//! 
//! **Learning Objective**: Master HashSet fundamentals and set theory operations
//! 
//! **Time Estimate**: 25 minutes
//! 
//! **What You'll Learn**:
//! - Creating and manipulating HashSets
//! - Membership testing and uniqueness enforcement
//! - Set theory operations (union, intersection, difference)
//! - Comparing sets and finding relationships
//! - Converting between collections

use std::collections::HashSet;

/// Example 1: Basic HashSet Operations
/// Shows fundamental HashSet usage patterns
fn example_basic_operations() {
    println!("🦀 Example 1: Basic HashSet Operations");
    println!("======================================");
    
    // Create a new HashSet for unique visitors
    let mut unique_visitors = HashSet::new();
    
    // Add some visitor IDs
    unique_visitors.insert("user_123");
    unique_visitors.insert("user_456");
    unique_visitors.insert("user_789");
    unique_visitors.insert("user_123"); // Duplicate - won't be added
    
    println!("👥 Unique visitors: {}", unique_visitors.len());
    for visitor in &unique_visitors {
        println!("  - {}", visitor);
    }
    
    // Check membership
    if unique_visitors.contains("user_123") {
        println!("✅ user_123 has visited our site");
    }
    
    // Remove a visitor
    if unique_visitors.remove("user_456") {
        println!("👋 user_456 removed from visitors");
    }
    
    println!("📊 Final visitor count: {}", unique_visitors.len());
    println!();
}

/// Example 2: Deduplication with HashSet
/// Demonstrates using HashSet to remove duplicates from data
fn example_deduplication() {
    println!("🔧 Example 2: Removing Duplicates");
    println!("=================================");
    
    // Raw data with duplicates
    let raw_tags = vec!["rust", "programming", "hashmap", "rust", "tutorial", "programming"];
    
    println!("📝 Original tags: {:?}", raw_tags);
    
    // Method 1: Collect into HashSet
    let unique_tags: HashSet<_> = raw_tags.iter().collect();
    println!("🎯 Unique tags (method 1): {:?}", unique_tags);
    
    // Method 2: Manual insertion with loop
    let mut tag_set = HashSet::new();
    for tag in &raw_tags {
        tag_set.insert(tag);
    }
    println!("🎯 Unique tags (method 2): {:?}", tag_set);
    
    // Convert back to Vec if needed
    let cleaned_tags: Vec<_> = tag_set.into_iter().collect();
    println!("📚 Back to Vec: {:?}", cleaned_tags);
    
    println!();
}

/// Example 3: Set Theory Operations
/// Shows mathematical set operations with real-world examples
fn example_set_operations() {
    println!("🧮 Example 3: Set Theory Operations");
    println!("===================================");
    
    // Create sets for different programming skills
    let rust_developers: HashSet<&str> = vec!["Alice", "Bob", "Charlie", "Diana"].into_iter().collect();
    let python_developers: HashSet<&str> = vec!["Bob", "Charlie", "Eve", "Frank"].into_iter().collect();
    let javascript_developers: HashSet<&str> = vec!["Alice", "Charlie", "Eve", "Grace"].into_iter().collect();
    
    println!("🦀 Rust developers: {:?}", rust_developers);
    println!("🐍 Python developers: {:?}", python_developers);
    println!("📜 JavaScript developers: {:?}", javascript_developers);
    println!();
    
    // Union: All developers who know at least one language
    let all_developers: HashSet<_> = rust_developers.union(&python_developers).collect();
    println!("🤝 Union (Rust ∪ Python): {:?}", all_developers);
    
    // Intersection: Developers who know both Rust and Python
    let polyglot_rust_python: HashSet<_> = rust_developers.intersection(&python_developers).collect();
    println!("🎯 Intersection (Rust ∩ Python): {:?}", polyglot_rust_python);
    
    // Difference: Rust developers who don't know Python
    let rust_only: HashSet<_> = rust_developers.difference(&python_developers).collect();
    println!("🔺 Difference (Rust - Python): {:?}", rust_only);
    
    // Symmetric difference: Developers who know exactly one of Rust or Python
    let exclusive_skills: HashSet<_> = rust_developers.symmetric_difference(&python_developers).collect();
    println!("⚡ Symmetric difference (Rust ⊕ Python): {:?}", exclusive_skills);
    
    println!();
}

/// Example 4: Advanced Set Comparisons
/// Shows how to analyze relationships between sets
fn example_set_comparisons() {
    println!("📊 Example 4: Set Comparisons");
    println!("=============================");
    
    let senior_team: HashSet<&str> = vec!["Alice", "Bob", "Charlie"].into_iter().collect();
    let project_team: HashSet<&str> = vec!["Alice", "Bob"].into_iter().collect();
    let full_company: HashSet<&str> = vec!["Alice", "Bob", "Charlie", "Diana", "Eve"].into_iter().collect();
    
    println!("👔 Senior team: {:?}", senior_team);
    println!("🚀 Project team: {:?}", project_team);
    println!("🏢 Full company: {:?}", full_company);
    println!();
    
    // Check subset relationships
    if project_team.is_subset(&senior_team) {
        println!("✅ Project team is a subset of senior team");
    }
    
    if senior_team.is_superset(&project_team) {
        println!("✅ Senior team is a superset of project team");
    }
    
    // Check if sets are disjoint (no common elements)
    let remote_team: HashSet<&str> = vec!["Frank", "Grace"].into_iter().collect();
    if project_team.is_disjoint(&remote_team) {
        println!("🌍 Project team and remote team are disjoint");
    }
    
    // Set equality
    let project_backup = project_team.clone();
    if project_team == project_backup {
        println!("✅ Project team equals its backup");
    }
    
    println!();
}

/// Example 5: Performance Considerations
/// Demonstrates when and why to use HashSet
fn example_performance() {
    println!("⚡ Example 5: Performance Benefits");
    println!("=================================");
    
    // Scenario: Check if items are in a blacklist
    let blacklist: HashSet<i32> = (1..=1000).collect(); // Large set
    let items_to_check = vec![50, 500, 1500, 2000];
    
    println!("🛡️  Checking items against blacklist ({} entries):", blacklist.len());
    
    for item in items_to_check {
        // O(1) average case lookup with HashSet
        if blacklist.contains(&item) {
            println!("  ❌ Item {} is blacklisted", item);
        } else {
            println!("  ✅ Item {} is allowed", item);
        }
    }
    
    // Compare with Vec (would be O(n) lookup)
    println!("\n💡 Performance tip:");
    println!("   HashSet.contains() -> O(1) average");
    println!("   Vec.contains()     -> O(n) linear search");
    println!("   Use HashSet for membership testing!");
    
    println!();
}

// 🧪 EXERCISE TIME! 
// Complete these functions to practice what you've learned

/// EXERCISE 1: Email domain analyzer
/// 
/// **Task**: Extract unique domains from a list of email addresses
/// **Challenge**: Handle both .com and other domains
fn exercise_email_domains() {
    println!("🧪 EXERCISE 1: Email Domain Analyzer");
    println!("====================================");
    
    let emails = vec![
        "alice@gmail.com",
        "bob@company.org", 
        "charlie@gmail.com",
        "diana@university.edu",
        "eve@company.org",
        "frank@outlook.com"
    ];
    
      let mut domains = HashSet::new();
        
        for email in emails {
            if let Some(domain) = email.split('@').nth(1) {
                domains.insert(domain);
            }
        }
        
        println!("📧 Unique domains found: {}", domains.len());
        for domain in &domains {
            println!("  - {}", domain);
        }
     
    println!("✅ Exercise 1 complete!\n");
}

/// EXERCISE 2: Permission system analyzer
/// 
/// **Task**: Analyze user permissions using set operations
/// **Challenge**: Find users with specific permission combinations
fn exercise_permission_analysis() {
    println!("🧪 EXERCISE 2: Permission Analysis");
    println!("==================================");
    
    let read_users: HashSet<&str> = vec!["alice", "bob", "charlie", "diana"].into_iter().collect();
    let write_users: HashSet<&str> = vec!["alice", "charlie", "eve"].into_iter().collect();
    let admin_users: HashSet<&str> = vec!["alice", "frank"].into_iter().collect();
        
    // Users who can read but not write
    let read_only: HashSet<_> = read_users.difference(&write_users).collect();
    println!("📖 Read-only users: {:?}", read_only);
    
    // Users with all permissions
    let all_perms: HashSet<_> = read_users.intersection(&write_users)
        .filter(|user| admin_users.contains(*user))
        .collect();
    println!("👑 Super users (all permissions): {:?}", all_perms);
    
    // Users with at least one permission
    let any_perms: HashSet<_> = read_users.union(&write_users)
        .chain(admin_users.iter())
        .collect();
    println!("🎫 Users with any permission: {:?}", any_perms);

    println!("✅ Exercise 2 complete!\n");
}

/// EXERCISE 3: Word relationship analyzer
/// 
/// **Task**: Analyze relationships between two sentences
/// **Challenge**: Find common words, unique words, etc.
fn exercise_word_relationships() {
    println!("🧪 EXERCISE 3: Word Relationships");
    println!("=================================");
    
    let sentence1 = "the quick brown fox jumps over the lazy dog";
    let sentence2 = "a quick brown cat runs over the sleeping dog";
    
    let words1: HashSet<&str> = sentence1.split_whitespace().collect();
    let words2: HashSet<&str> = sentence2.split_whitespace().collect();
    
    let common: HashSet<_> = words1.intersection(&words2).collect();
    let unique1: HashSet<_> = words1.difference(&words2).collect();
    let unique2: HashSet<_> = words2.difference(&words1).collect();
    
    println!("🤝 Common words: {:?}", common);
    println!("🔹 Unique to sentence 1: {:?}", unique1);
    println!("🔸 Unique to sentence 2: {:?}", unique2);
    
    let total_unique = words1.union(&words2).count();
    let similarity = (common.len() as f32 / total_unique as f32) * 100.0;
    println!("📊 Similarity: {:.1}%", similarity);
    
    println!("✅ Exercise 3 complete!\n");
}

// 📝 SOLUTION SECTION
// Don't look here until you've tried the exercises!

#[cfg(feature = "solutions")]
mod solutions {
    use super::*;
    
    fn exercise_email_domains_solution() {
        let emails = vec![
            "alice@gmail.com", "bob@company.org", "charlie@gmail.com",
            "diana@university.edu", "eve@company.org", "frank@outlook.com"
        ];
        
        let mut domains = HashSet::new();
        
        for email in emails {
            if let Some(domain) = email.split('@').nth(1) {
                domains.insert(domain);
            }
        }
        
        println!("📧 Unique domains found: {}", domains.len());
        for domain in &domains {
            println!("  - {}", domain);
        }
    }
    
    fn exercise_permission_analysis_solution() {
        let read_users: HashSet<&str> = vec!["alice", "bob", "charlie", "diana"].into_iter().collect();
        let write_users: HashSet<&str> = vec!["alice", "charlie", "eve"].into_iter().collect();
        let admin_users: HashSet<&str> = vec!["alice", "frank"].into_iter().collect();
        
        // Users who can read but not write
        let read_only: HashSet<_> = read_users.difference(&write_users).collect();
        println!("📖 Read-only users: {:?}", read_only);
        
        // Users with all permissions
        let all_perms: HashSet<_> = read_users.intersection(&write_users)
            .filter(|user| admin_users.contains(*user))
            .collect();
        println!("👑 Super users (all permissions): {:?}", all_perms);
        
        // Users with at least one permission
        let any_perms: HashSet<_> = read_users.union(&write_users)
            .chain(admin_users.iter())
            .collect();
        println!("🎫 Users with any permission: {:?}", any_perms);
    }
    
    fn exercise_word_relationships_solution() {
        let sentence1 = "the quick brown fox jumps over the lazy dog";
        let sentence2 = "a quick brown cat runs over the sleeping dog";
        
        let words1: HashSet<&str> = sentence1.split_whitespace().collect();
        let words2: HashSet<&str> = sentence2.split_whitespace().collect();
        
        let common: HashSet<_> = words1.intersection(&words2).collect();
        let unique1: HashSet<_> = words1.difference(&words2).collect();
        let unique2: HashSet<_> = words2.difference(&words1).collect();
        
        println!("🤝 Common words: {:?}", common);
        println!("🔹 Unique to sentence 1: {:?}", unique1);
        println!("🔸 Unique to sentence 2: {:?}", unique2);
        
        let total_unique = words1.union(&words2).count();
        let similarity = (common.len() as f32 / total_unique as f32) * 100.0;
        println!("📊 Similarity: {:.1}%", similarity);
    }
}

/// Main function - runs all examples and exercises
fn main() {
    println!("🦀 Step 2: HashSet Operations");
    println!("=============================\n");
    
    // Run all examples
    example_basic_operations();
    example_deduplication();
    example_set_operations();
    example_set_comparisons();
    example_performance();
    
    // Exercise time!
    println!("🎯 Now it's your turn! Complete these exercises:");
    println!("================================================\n");
    
    exercise_email_domains();
    exercise_permission_analysis();
    exercise_word_relationships();
    
    println!("🎉 Congratulations! You've completed Step 2!");
    println!("📚 Ready for Step 3? Run: cargo run --example step3_frequency_counting");
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_hashset_operations() {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(1); // Duplicate
        
        assert_eq!(set.len(), 2);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(!set.contains(&3));
    }
    
    #[test]
    fn test_set_operations() {
        let set1: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
        let set2: HashSet<i32> = vec![2, 3, 4].into_iter().collect();
        
        let union: HashSet<_> = set1.union(&set2).cloned().collect();
        let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
        
        assert_eq!(union, vec![1, 2, 3, 4].into_iter().collect());
        assert_eq!(intersection, vec![2, 3].into_iter().collect());
    }
    
    #[test]
    fn test_deduplication() {
        let data = vec!["a", "b", "a", "c", "b"];
        let unique: HashSet<_> = data.into_iter().collect();
        
        assert_eq!(unique.len(), 3);
        assert!(unique.contains("a"));
        assert!(unique.contains("b"));
        assert!(unique.contains("c"));
    }
}

