//! # Step 4: Multi-Value Patterns
//! 
//! **Learning Objective**: Master HashMap<K, Vec<V>> patterns for one-to-many relationships
//! 
//! **Time Estimate**: 25 minutes
//! 
//! **What You'll Learn**:
//! - Building adjacency lists for graph algorithms
//! - Grouping data by categories
//! - Managing parent-child relationships
//! - Indexing and reverse lookups
//! - Optimizing multi-value operations

use std::collections::HashMap;

/// Example 1: Basic Multi-Value Dictionary
/// Foundation pattern for one-to-many relationships
fn example_basic_multi_value() {
    println!("🦀 Example 1: Multi-Value Dictionary");
    println!("====================================");
    
    let mut student_courses: HashMap<&str, Vec<&str>> = HashMap::new();
    
    // Method 1: Insert complete vectors
    student_courses.insert("Alice", vec!["Math", "Physics", "Chemistry"]);
    student_courses.insert("Bob", vec!["Biology", "Chemistry"]);
    
    // Method 2: Build vectors incrementally
    student_courses.entry("Charlie").or_insert(Vec::new()).push("Math");
    student_courses.entry("Charlie").or_insert(Vec::new()).push("Computer Science");
    student_courses.entry("Charlie").or_insert(Vec::new()).push("Physics");
    
    println!("👨‍🎓 Student Enrollments:");
    for (student, courses) in &student_courses {
        println!("  {} -> {:?}", student, courses);
    }
    
    // Find students taking specific courses
    let target_course = "Math";
    let math_students: Vec<_> = student_courses.iter()
        .filter(|(_, courses)| courses.contains(&target_course))
        .map(|(student, _)| *student)
        .collect();
    
    println!("\n🔍 Students taking {}: {:?}", target_course, math_students);
    
    // Calculate course load statistics
    for (student, courses) in &student_courses {
        println!("📊 {}: {} courses", student, courses.len());
    }
    
    println!();
}

/// Example 2: Graph Adjacency Lists
/// Essential pattern for graph algorithms and pathfinding
fn example_graph_adjacency() {
    println!("🌐 Example 2: Graph Adjacency Lists");
    println!("===================================");
    
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    
    // Build a directed graph of city connections
    graph.insert("New York", vec!["Boston", "Philadelphia", "Washington"]);
    graph.insert("Boston", vec!["New York", "Portland"]);
    graph.insert("Philadelphia", vec!["New York", "Washington"]);
    graph.insert("Washington", vec!["New York", "Philadelphia", "Richmond"]);
    graph.insert("Portland", vec!["Boston"]);
    graph.insert("Richmond", vec!["Washington"]);
    
    println!("🗺️  City Connection Graph:");
    for (city, connections) in &graph {
        println!("  {} -> {:?}", city, connections);
    }
    
    // Find all reachable cities from a starting point (1 hop)
    let start_city = "New York";
    if let Some(direct_connections) = graph.get(start_city) {
        println!("\n✈️  Direct flights from {}:", start_city);
        for destination in direct_connections {
            println!("  - {}", destination);
        }
    }
    
    // Find cities with the most connections
    if let Some((hub_city, connections)) = graph.iter().max_by_key(|(_, connections)| connections.len()) {
        println!("\n🏙️  Major hub: {} ({} connections)", hub_city, connections.len());
    }
    
    // Check if path exists between two cities (direct connection)
    let from = "Boston";
    let to = "Portland";
    let has_direct_path = graph.get(from)
        .map_or(false, |connections| connections.contains(&to));
    
    println!("🛣️  Direct path {} -> {}: {}", from, to, has_direct_path);
    
    println!();
}

/// Example 3: Grouping and Categorization
/// Pattern for organizing data into categories
fn example_grouping() {
    println!("📂 Example 3: Data Grouping");
    println!("===========================");
    
    // Sample data: (item, category)
    let items = vec![
        ("Apple", "Fruit"),
        ("Carrot", "Vegetable"),
        ("Banana", "Fruit"),
        ("Broccoli", "Vegetable"),
        ("Orange", "Fruit"),
        ("Spinach", "Vegetable"),
        ("Strawberry", "Fruit"),
    ];
    
    // Group items by category
    let mut categories: HashMap<&str, Vec<&str>> = HashMap::new();
    for (item, category) in items {
        categories.entry(category).or_insert(Vec::new()).push(item);
    }
    
    println!("🛒 Grocery Categories:");
    for (category, items) in &categories {
        println!("  {} ({} items): {:?}", category, items.len(), items);
    }
    
    // Alternative grouping: by first letter
    let mut by_first_letter: HashMap<char, Vec<&str>> = HashMap::new();
    for (_category, items) in &categories {
        for &item in items {
            if let Some(first_char) = item.chars().next() {
                by_first_letter.entry(first_char).or_insert(Vec::new()).push(item);
            }
        }
    }
    
    println!("\n🔤 Items by First Letter:");
    for (letter, items) in &by_first_letter {
        println!("  '{}': {:?}", letter, items);
    }
    
    println!();
}

/// Example 4: Reverse Index Building
/// Pattern for creating efficient lookup structures
fn example_reverse_index() {
    println!("🔍 Example 4: Reverse Index");
    println!("===========================");
    
    // Document collection
    let documents = vec![
        ("doc1", "rust programming language safety"),
        ("doc2", "python programming web development"),
        ("doc3", "javascript web frontend programming"),
        ("doc4", "rust systems programming performance"),
    ];
    
    // Build word -> document_ids index
    let mut word_index: HashMap<&str, Vec<&str>> = HashMap::new();
    
    for (doc_id, content) in &documents {
        for word in content.split_whitespace() {
            word_index.entry(word).or_insert(Vec::new()).push(doc_id);
        }
    }
    
    println!("📚 Word Index:");
    for (word, doc_ids) in &word_index {
        println!("  '{}' appears in: {:?}", word, doc_ids);
    }
    
    // Search functionality
    let search_terms = vec!["programming", "rust", "web"];
    
    for term in search_terms {
        if let Some(docs) = word_index.get(term) {
            println!("\n🔍 Search '{}': Found in {} documents: {:?}", 
                     term, docs.len(), docs);
        } else {
            println!("\n❌ Search '{}': No results found", term);
        }
    }
    
    // Find documents containing multiple terms
    let _query_terms = vec!["rust", "programming"];
    println!("\n🎯 Documents containing both 'rust' AND 'programming':");
    
    if let Some(rust_docs) = word_index.get("rust") {
        if let Some(prog_docs) = word_index.get("programming") {
            let intersection: Vec<_> = rust_docs.iter()
                .filter(|doc| prog_docs.contains(doc))
                .collect();
            println!("  {:?}", intersection);
        }
    }
    
    println!();
}

/// Example 5: Advanced Multi-Value Operations
/// Utility functions for complex multi-value operations
#[derive(Debug)]
struct MultiValueMap<K, V> {
    data: HashMap<K, Vec<V>>,
}

impl<K, V> MultiValueMap<K, V> 
where 
    K: std::hash::Hash + Eq + Clone,
    V: Clone + PartialEq,
{
    fn new() -> Self {
        MultiValueMap {
            data: HashMap::new(),
        }
    }
    
    fn add(&mut self, key: K, value: V) {
        self.data.entry(key).or_insert(Vec::new()).push(value);
    }
    
    fn add_all(&mut self, key: K, values: Vec<V>) {
        self.data.entry(key).or_insert(Vec::new()).extend(values);
    }
    
    fn get(&self, key: &K) -> Option<&Vec<V>> {
        self.data.get(key)
    }
    
    fn contains_value(&self, key: &K, value: &V) -> bool {
        self.data.get(key)
            .map_or(false, |values| values.contains(value))
    }
    
    fn remove_value(&mut self, key: &K, value: &V) -> bool {
        if let Some(values) = self.data.get_mut(key) {
            if let Some(pos) = values.iter().position(|v| v == value) {
                values.remove(pos);
                return true;
            }
        }
        false
    }
    
    fn count_values(&self, key: &K) -> usize {
        self.data.get(key).map_or(0, |values| values.len())
    }
    
    fn all_values(&self) -> Vec<V> {
        self.data.values()
            .flat_map(|values| values.iter())
            .cloned()
            .collect()
    }
}

fn example_advanced_operations() {
    println!("⚡ Example 5: Advanced Multi-Value Operations");
    println!("============================================");
    
    let mut tags_map = MultiValueMap::new();
    
    // Add tags to articles
    tags_map.add("article1", "rust");
    tags_map.add("article1", "programming");
    tags_map.add("article1", "systems");
    
    tags_map.add("article2", "javascript");
    tags_map.add("article2", "web");
    tags_map.add("article2", "frontend");
    
    tags_map.add_all("article3", vec!["python", "data-science", "machine-learning"]);
    
    println!("🏷️  Article Tags:");
    for (article, tags) in &tags_map.data {
        println!("  {} -> {:?}", article, tags);
    }
    
    // Query operations
    println!("\n🔍 Query Results:");
    println!("  article1 has {} tags", tags_map.count_values(&"article1"));
    println!("  article1 has 'rust' tag: {}", tags_map.contains_value(&"article1", &"rust"));
    println!("  article2 has 'rust' tag: {}", tags_map.contains_value(&"article2", &"rust"));
    
    // Remove a specific tag
    if tags_map.remove_value(&"article1", &"systems") {
        println!("  Removed 'systems' tag from article1");
    }
    
    // Get all unique tags across all articles
    let all_tags = tags_map.all_values();
    println!("\n📊 All tags used: {:?}", all_tags);
    
    println!();
}

// 🧪 EXERCISE TIME!

/// EXERCISE 1: Social Network Friends
/// 
/// **Task**: Build a friend network using adjacency lists
/// **Challenge**: Find mutual friends and friend recommendations
fn exercise_social_network() {
    println!("🧪 EXERCISE 1: Social Network");
    println!("=============================");
    
    // TODO: Create HashMap<&str, Vec<&str>> for person -> friends
    // TODO: Add these friendships (bidirectional):
    //   Alice: Bob, Charlie, Diana
    //   Bob: Alice, Charlie, Eve
    //   Charlie: Alice, Bob, Frank
    //   Diana: Alice
    //   Eve: Bob, Frank
    //   Frank: Charlie, Eve
    
    // TODO: Find mutual friends between Alice and Bob
    // TODO: Suggest friends for Diana (friends of friends who aren't already friends)
    // TODO: Find the person with the most friends

        let mut friends: HashMap<&str, Vec<&str>> = HashMap::new();
        
        // Add bidirectional friendships
        friends.insert("Alice", vec!["Bob", "Charlie", "Diana"]);
        friends.insert("Bob", vec!["Alice", "Charlie", "Eve"]);
        friends.insert("Charlie", vec!["Alice", "Bob", "Frank"]);
        friends.insert("Diana", vec!["Alice"]);
        friends.insert("Eve", vec!["Bob", "Frank"]);
        friends.insert("Frank", vec!["Charlie", "Eve"]);
        
        // Find mutual friends between Alice and Bob
        if let (Some(alice_friends), Some(bob_friends)) = (friends.get("Alice"), friends.get("Bob")) {
            let mutual: Vec<_> = alice_friends.iter()
                .filter(|friend| bob_friends.contains(friend))
                .collect();
            println!("🤝 Alice and Bob's mutual friends: {:?}", mutual);
        }
        
        // Suggest friends for Diana (friends of friends who aren't already friends)
        let target_person = "Diana";
        if let Some(diana_friends) = friends.get(target_person) {
            let mut suggestions: HashMap<&str, usize> = HashMap::new();
            
            // For each of Diana's friends
            for &friend in diana_friends {
                // Get that friend's friends
                if let Some(friend_of_friends) = friends.get(friend) {
                    for &potential_friend in friend_of_friends {
                        // Skip if it's Diana herself
                        if potential_friend == target_person {
                            continue;
                        }
                        // Skip if already friends with Diana
                        if diana_friends.contains(&potential_friend) {
                            continue;
                        }
                        // Count how many mutual connections this person has
                        *suggestions.entry(potential_friend).or_insert(0) += 1;
                    }
                }
            }
            
            // Sort suggestions by number of mutual connections (highest first)
            let mut sorted_suggestions: Vec<_> = suggestions.into_iter().collect();
            sorted_suggestions.sort_by(|a, b| b.1.cmp(&a.1));
            
            println!("\n👥 Friend suggestions for {}:", target_person);
            if sorted_suggestions.is_empty() {
                println!("  No suggestions found");
            } else {
                for (suggested_person, connection_count) in sorted_suggestions {
                    println!("  {} ({} mutual connection{})", 
                           suggested_person, 
                           connection_count,
                           if connection_count == 1 { "" } else { "s" });
                }
            }
        }
        
        // Person with most friends
        if let Some((popular_person, friend_list)) = friends.iter().max_by_key(|(_, friends)| friends.len()) {
            println!("👑 Most popular: {} with {} friends", popular_person, friend_list.len());
        }
    println!("✅ Exercise 1 complete!\n");
}

/// EXERCISE 2: Course Prerequisites
/// 
/// **Task**: Model course prerequisites and find valid enrollment paths
/// **Challenge**: Detect prerequisite cycles and suggest course ordering
fn exercise_course_prerequisites() {
    println!("🧪 EXERCISE 2: Course Prerequisites");
    println!("===================================");
    
    let prerequisites = vec![
        ("CS201", vec!["CS101"]),
        ("CS301", vec!["CS201", "MATH201"]),
        ("CS401", vec!["CS301"]),
        ("MATH201", vec!["MATH101"]),
        ("MATH301", vec!["MATH201"]),
    ];
    
    // Build HashMap<&str, Vec<&str>> for course -> prerequisites
    let mut course_prereqs: HashMap<&str, Vec<&str>> = HashMap::new();
    for (course, prereqs) in prerequisites {
        course_prereqs.insert(course, prereqs);
    }
    
    println!("📚 Course Prerequisites:");
    for (course, prereqs) in &course_prereqs {
        println!("  {} requires: {:?}", course, prereqs);
    }
    
    // Find courses with no prerequisites (entry-level courses)
    let all_courses: std::collections::HashSet<_> = course_prereqs.keys().cloned().collect();
    let mut all_mentioned_courses = all_courses.clone();
    for prereqs in course_prereqs.values() {
        for &prereq in prereqs {
            all_mentioned_courses.insert(prereq);
        }
    }
    
    let entry_level: Vec<_> = all_mentioned_courses.iter()
        .filter(|course| !course_prereqs.contains_key(*course))
        .collect();
    
    println!("\n🎓 Entry-level courses (no prerequisites): {:?}", entry_level);
    
    // Find all courses a student can take if they've completed CS101 and MATH101
    let completed = vec!["CS101", "MATH101"];
    let mut available_courses = Vec::new();
    
    for (&course, prereqs) in &course_prereqs {
        let can_take = prereqs.iter().all(|prereq| completed.contains(prereq));
        if can_take {
            available_courses.push(course);
        }
    }
    
    println!("\n📝 Courses available after completing CS101 and MATH101: {:?}", available_courses);
    
    // Calculate minimum semesters needed to complete CS401
    fn min_semesters_for_course(
        target: &str, 
        prereqs_map: &HashMap<&str, Vec<&str>>, 
        memo: &mut HashMap<String, usize>
    ) -> usize {
        let target_string = target.to_string();
        if let Some(&cached) = memo.get(&target_string) {
            return cached;
        }
        
        if let Some(prereqs) = prereqs_map.get(target) {
            if prereqs.is_empty() {
                memo.insert(target_string, 1);
                return 1;
            }
            
            let max_prereq_depth = prereqs.iter()
                .map(|prereq| min_semesters_for_course(prereq, prereqs_map, memo))
                .max()
                .unwrap_or(0);
            
            let result = max_prereq_depth + 1;
            memo.insert(target_string, result);
            result
        } else {
            // Entry-level course
            memo.insert(target_string, 1);
            1
        }
    }
    
    let mut memo = HashMap::new();
    let semesters_for_cs401 = min_semesters_for_course("CS401", &course_prereqs, &mut memo);
    println!("\n⏱️  Minimum semesters to complete CS401: {}", semesters_for_cs401);
    
    // Show the path to CS401
    println!("\n🛤️  Path to CS401:");
    println!("  Semester 1: CS101, MATH101 (entry-level)");
    println!("  Semester 2: CS201, MATH201 (after CS101, MATH101)");
    println!("  Semester 3: CS301 (after CS201, MATH201)");
    println!("  Semester 4: CS401 (after CS301)");
    
    println!("✅ Exercise 2 complete!\n");
}

/// EXERCISE 3: File System Directory Structure
/// 
/// **Task**: Model a file system with directories and files
/// **Challenge**: Implement recursive directory listing and path finding
fn exercise_file_system() {
    println!("🧪 EXERCISE 3: File System");
    println!("==========================");
    
    let file_structure = vec![
        ("/", vec!["home", "usr", "var"]),
        ("/home", vec!["alice", "bob"]),
        ("/home/alice", vec!["documents", "downloads"]),
        ("/home/alice/documents", vec!["resume.pdf", "notes.txt"]),
        ("/home/bob", vec!["projects", "music"]),
        ("/home/bob/projects", vec!["rust_app", "web_site"]),
        ("/usr", vec!["bin", "lib"]),
        ("/var", vec!["log", "tmp"]),
    ];
    
    // Build HashMap<&str, Vec<&str>> for directory -> contents
    let mut fs: HashMap<&str, Vec<&str>> = HashMap::new();
    for (directory, contents) in file_structure {
        fs.insert(directory, contents);
    }
    
    println!("📁 File System Structure:");
    for (directory, contents) in &fs {
        println!("  {} -> {:?}", directory, contents);
    }
    
    // Helper function to check if an item is a file (has extension) or directory
    fn is_file(name: &str) -> bool {
        name.contains('.')
    }
    
    // Recursive function to list all files in a directory tree
    fn list_files_recursive(
        path: &str, 
        fs: &HashMap<&str, Vec<&str>>, 
        files: &mut Vec<String>
    ) {
        if let Some(contents) = fs.get(path) {
            for &item in contents {
                let full_path = if path == "/" {
                    format!("/{}", item)
                } else {
                    format!("{}/{}", path, item)
                };
                
                if is_file(item) {
                    files.push(full_path);
                } else {
                    // It's a directory, recurse into it
                    list_files_recursive(&full_path, fs, files);
                }
            }
        }
    }
    
    // List all files in /home/alice directory tree (recursive)
    let mut alice_files = Vec::new();
    list_files_recursive("/home/alice", &fs, &mut alice_files);
    
    println!("\n📄 All files in /home/alice (recursive):");
    for file in &alice_files {
        println!("  {}", file);
    }
    
    // Count total files vs directories in the entire system
    fn count_files_and_dirs(fs: &HashMap<&str, Vec<&str>>) -> (usize, usize) {
        let mut file_count = 0;
        let dir_count = fs.len(); // All keys are directories
        
        for contents in fs.values() {
            for &item in contents {
                if is_file(item) {
                    file_count += 1;
                }
            }
        }
        
        (file_count, dir_count)
    }
    
    let (total_files, total_dirs) = count_files_and_dirs(&fs);
    println!("\n📊 File System Statistics:");
    println!("  Total files: {}", total_files);
    println!("  Total directories: {}", total_dirs);
    
    // Find the deepest directory path
    let deepest_path = fs.keys()
        .max_by_key(|path| path.matches('/').count())
        .unwrap_or(&"/");
    
    let depth = deepest_path.matches('/').count();
    println!("\n🏔️  Deepest directory path: {} (depth: {})", deepest_path, depth);
    
    // Bonus: Find all paths to a specific file
    fn find_file_paths(
        filename: &str, 
        fs: &HashMap<&str, Vec<&str>>
    ) -> Vec<String> {
        let mut paths = Vec::new();
        
        for (&directory, contents) in fs {
            for &item in contents {
                if item == filename {
                    let full_path = if directory == "/" {
                        format!("/{}", item)
                    } else {
                        format!("{}/{}", directory, item)
                    };
                    paths.push(full_path);
                }
            }
        }
        
        paths
    }
    
    let pdf_files = find_file_paths("resume.pdf", &fs);
    println!("\n🔍 Location of 'resume.pdf': {:?}", pdf_files);
    
    // Show directory tree structure
    fn print_directory_tree(
        path: &str, 
        fs: &HashMap<&str, Vec<&str>>, 
        prefix: &str, 
        is_last: bool
    ) {
        let current_prefix = if is_last { "└── " } else { "├── " };
        let path_name = if path == "/" { 
            "/"
        } else { 
            path.split('/').last().unwrap_or(path) 
        };
        
        println!("{}{}{}", prefix, current_prefix, path_name);
        
        if let Some(contents) = fs.get(path) {
            let next_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };
            
            // Separate files and directories
            let mut files = Vec::new();
            let mut directories = Vec::new();
            
            for &item in contents {
                let full_path = if path == "/" {
                    format!("/{}", item)
                } else {
                    format!("{}/{}", path, item)
                };
                
                if is_file(item) {
                    files.push(item);
                } else if fs.contains_key(full_path.as_str()) {
                    directories.push((item, full_path));
                }
            }
            
            // Print directories first
            for (i, (_dir_name, full_path)) in directories.iter().enumerate() {
                let is_last_dir = i == directories.len() - 1 && files.is_empty();
                print_directory_tree(&full_path, fs, &next_prefix, is_last_dir);
            }
            
            // Then print files
            for (i, &file) in files.iter().enumerate() {
                let is_last_file = i == files.len() - 1;
                let file_prefix = if is_last_file { "└── " } else { "├── " };
                println!("{}{}{}", next_prefix, file_prefix, file);
            }
        }
    }
    
    println!("\n🌳 Directory Tree:");
    print_directory_tree("/", &fs, "", true);
    
    println!("✅ Exercise 3 complete!\n");
}

// 📝 SOLUTION SECTION

#[cfg(feature = "solutions")]
mod solutions {
    use super::*;
    
    fn exercise_social_network_solution() {
        let mut friends: HashMap<&str, Vec<&str>> = HashMap::new();
        
        // Add bidirectional friendships
        friends.insert("Alice", vec!["Bob", "Charlie", "Diana"]);
        friends.insert("Bob", vec!["Alice", "Charlie", "Eve"]);
        friends.insert("Charlie", vec!["Alice", "Bob", "Frank"]);
        friends.insert("Diana", vec!["Alice"]);
        friends.insert("Eve", vec!["Bob", "Frank"]);
        friends.insert("Frank", vec!["Charlie", "Eve"]);
        
        // Find mutual friends between Alice and Bob
        if let (Some(alice_friends), Some(bob_friends)) = (friends.get("Alice"), friends.get("Bob")) {
            let mutual: Vec<_> = alice_friends.iter()
                .filter(|friend| bob_friends.contains(friend))
                .collect();
            println!("🤝 Alice and Bob's mutual friends: {:?}", mutual);
        }
        
        // Person with most friends
        if let Some((popular_person, friend_list)) = friends.iter().max_by_key(|(_, friends)| friends.len()) {
            println!("👑 Most popular: {} with {} friends", popular_person, friend_list.len());
        }
    }
}

/// Main function - runs all examples and exercises
fn main() {
    println!("🦀 Step 4: Multi-Value Patterns");
    println!("================================\n");
    
    // Run all examples
    example_basic_multi_value();
    example_graph_adjacency();
    example_grouping();
    example_reverse_index();
    example_advanced_operations();
    
    // Exercise time!
    println!("🎯 Now it's your turn! Complete these exercises:");
    println!("================================================\n");
    
    exercise_social_network();
    exercise_course_prerequisites();
    exercise_file_system();
    
    println!("🎉 Congratulations! You've completed Step 4!");
    println!("📚 Ready for Step 5? Run: cargo run --example step5_memoization_cache");
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_multi_value_map() {
        let mut map = MultiValueMap::new();
        map.add("key1", "value1");
        map.add("key1", "value2");
        map.add("key2", "value3");
        
        assert_eq!(map.count_values(&"key1"), 2);
        assert_eq!(map.count_values(&"key2"), 1);
        assert!(map.contains_value(&"key1", &"value1"));
        assert!(!map.contains_value(&"key2", &"value1"));
        
        assert!(map.remove_value(&"key1", &"value1"));
        assert_eq!(map.count_values(&"key1"), 1);
    }
    
    #[test]
    fn test_graph_adjacency() {
        let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
        graph.insert("A", vec!["B", "C"]);
        graph.insert("B", vec!["A", "D"]);
        graph.insert("C", vec!["A"]);
        graph.insert("D", vec!["B"]);
        
        assert_eq!(graph.get("A").unwrap().len(), 2);
        assert!(graph.get("A").unwrap().contains(&"B"));
        assert!(graph.get("A").unwrap().contains(&"C"));
    }
}