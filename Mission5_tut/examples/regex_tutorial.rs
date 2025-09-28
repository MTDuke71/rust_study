// Regular Expressions Tutorial: From Zero to Hero
// ================================================
// A step-by-step guide to understanding regex patterns

use regex::Regex;

fn main() {
    println!("🧠 Regular Expressions Tutorial");
    println!("==============================");
    
    // Step 1: What IS a Regular Expression?
    step1_what_is_regex();
    
    // Step 2: Literal Matching
    step2_literal_matching();
    
    // Step 3: Character Classes
    step3_character_classes();
    
    // Step 4: Quantifiers (How Many?)
    step4_quantifiers();
    
    // Step 5: Anchors (Where?)
    step5_anchors();
    
    // Step 6: Groups and Capture
    step6_groups();
    
    // Step 7: Common Patterns
    step7_common_patterns();
    
    // Step 8: AoC Day 5 Example
    step8_aoc_example();
    
    // Step 9: Practice Exercises
    step9_practice();
}

fn step1_what_is_regex() {
    println!("\n🎯 Step 1: What IS a Regular Expression?");
    println!("========================================");
    
    println!("Think of regex as a 'search pattern' - like Ctrl+F but MUCH more powerful!");
    println!();
    println!("🔍 Real-world analogy:");
    println!("  Regular text search: 'Find \"cat\"' - finds exactly 'cat'");
    println!("  Regex search: 'Find any 3-letter word ending in 'at'' - finds 'cat', 'bat', 'hat', etc.");
    println!();
    println!("💡 Key insight: Regex describes PATTERNS, not exact text");
    
    // Simple example
    let text = "I have a cat, a bat, and a hat";
    let pattern = Regex::new(r".at").unwrap();  // . means "any character"
    
    println!("\n📝 Example:");
    println!("  Text: '{}'", text);
    println!("  Pattern: '.at' (dot + 'at')");
    print!("  Matches: ");
    for mat in pattern.find_iter(text) {
        print!("'{}' ", mat.as_str());
    }
    println!("\n  ↳ The '.' matches any single character before 'at'");
}

fn step2_literal_matching() {
    println!("\n📝 Step 2: Literal Matching (Exact Text)");
    println!("=========================================");
    
    println!("Most characters in regex match themselves exactly:");
    println!();
    
    let text = "Hello world! How are you?";
    
    // Exact matches
    let patterns = vec![
        ("Hello", "Exact word match"),
        ("world", "Another exact match"), 
        ("o", "Single character (finds ALL o's)"),
    ];
    
    for (pattern_str, description) in patterns {
        let pattern = Regex::new(pattern_str).unwrap();
        println!("Pattern: '{}' - {}", pattern_str, description);
        print!("  Matches: ");
        for mat in pattern.find_iter(text) {
            print!("'{}' (at position {}) ", mat.as_str(), mat.start());
        }
        println!();
    }
    
    println!("\n💡 Key insight: Regex finds ALL matches, not just the first one!");
}

fn step3_character_classes() {
    println!("\n🎭 Step 3: Character Classes [...]");
    println!("==================================");
    
    println!("Character classes let you match 'any character from this set':");
    println!();
    
    let text = "I have 3 cats, 2 dogs, and 1 bird";
    
    let examples = vec![
        ("[aeiou]", "Any vowel", "Vowels"),
        ("[0-9]", "Any digit", "Numbers"), 
        ("[a-z]", "Any lowercase letter", "Lowercase"),
        ("[A-Za-z]", "Any letter", "Letters"),
        ("[^0-9]", "NOT a digit (^ means NOT)", "Non-numbers"),
    ];
    
    for (pattern_str, description, label) in examples {
        let pattern = Regex::new(pattern_str).unwrap();
        println!("Pattern: '{}' - {}", pattern_str, description);
        print!("  {} found: ", label);
        let matches: Vec<_> = pattern.find_iter(text).map(|m| m.as_str()).collect();
        println!("{}", matches.join(", "));
    }
    
    println!("\n🎯 Character class shortcuts:");
    println!("  \\d = [0-9]      (digits)");
    println!("  \\w = [A-Za-z0-9_] (word characters)");
    println!("  \\s = [ \\t\\n]    (whitespace)");
    println!("  .  = any character except newline");
}

fn step4_quantifiers() {
    println!("\n🔢 Step 4: Quantifiers (How Many?)");
    println!("==================================");
    
    println!("Quantifiers tell regex HOW MANY of the previous thing to match:");
    println!();
    
    let text = "a aa aaa aaaa b bb bbb";
    
    let examples = vec![
        ("a", "Exactly one 'a'"),
        ("a+", "One or more 'a's"),
        ("a*", "Zero or more 'a's"), 
        ("a?", "Zero or one 'a'"),
        ("a{3}", "Exactly 3 'a's"),
        ("a{2,4}", "Between 2 and 4 'a's"),
        ("b+", "One or more 'b's"),
    ];
    
    for (pattern_str, description) in examples {
        let pattern = Regex::new(pattern_str).unwrap();
        println!("Pattern: '{}' - {}", pattern_str, description);
        print!("  Matches: ");
        for mat in pattern.find_iter(text) {
            print!("'{}' ", mat.as_str());
        }
        println!();
    }
    
    println!("\n💡 Quantifier cheat sheet:");
    println!("  ?  = 0 or 1 (optional)");
    println!("  *  = 0 or more (any amount)");
    println!("  +  = 1 or more (at least one)");
    println!("  {{n}} = exactly n times");
    println!("  {{n,m}} = between n and m times");
}

fn step5_anchors() {
    println!("\n⚓ Step 5: Anchors (Where in the string?)");
    println!("=========================================");
    
    println!("Anchors specify WHERE in the string to match:");
    println!();
    
    let lines = vec![
        "cat in the hat",
        "a cat walked by", 
        "catch the cat",
        "cat",
    ];
    
    let examples = vec![
        ("cat", "Anywhere in the string"),
        ("^cat", "At the START of string (^ = start)"),
        ("cat$", "At the END of string ($ = end)"),
        ("^cat$", "ENTIRE string is exactly 'cat'"),
    ];
    
    for (pattern_str, description) in examples {
        let pattern = Regex::new(pattern_str).unwrap();
        println!("Pattern: '{}' - {}", pattern_str, description);
        for line in &lines {
            let matches = if pattern.is_match(line) { "✅" } else { "❌" };
            println!("  '{}' {}", line, matches);
        }
        println!();
    }
}

fn step6_groups() {
    println!("\n👥 Step 6: Groups and Capturing ()");
    println!("==================================");
    
    println!("Parentheses group parts together and 'capture' matched text:");
    println!();
    
    let text = "Call me at 555-1234 or email test@example.com";
    
    // Phone number pattern
    let phone_pattern = Regex::new(r"(\d{3})-(\d{4})").unwrap();
    println!("Phone pattern: '(\\d{{3}})-(\\d{{4}})' - groups area code and number");
    
    if let Some(captures) = phone_pattern.captures(text) {
        println!("  Full match: '{}'", captures.get(0).unwrap().as_str());
        println!("  Group 1 (area code): '{}'", captures.get(1).unwrap().as_str());
        println!("  Group 2 (number): '{}'", captures.get(2).unwrap().as_str());
    }
    
    // Email pattern  
    let email_pattern = Regex::new(r"([a-zA-Z0-9]+)@([a-zA-Z0-9]+\.[a-zA-Z]+)").unwrap();
    println!("\nEmail pattern: '([a-zA-Z0-9]+)@([a-zA-Z0-9]+\\.[a-zA-Z]+)'");
    
    if let Some(captures) = email_pattern.captures(text) {
        println!("  Full match: '{}'", captures.get(0).unwrap().as_str());
        println!("  Group 1 (username): '{}'", captures.get(1).unwrap().as_str());
        println!("  Group 2 (domain): '{}'", captures.get(2).unwrap().as_str());
    }
    
    println!("\n💡 Groups let you extract PARTS of matches, not just find them!");
}

fn step7_common_patterns() {
    println!("\n🏆 Step 7: Common Real-World Patterns");
    println!("=====================================");
    
    let examples = vec![
        (r"\d+", "Numbers", "123, 456, 789"),
        (r"[A-Za-z]+", "Words", "Hello, world, test"),  
        (r"\b\w+\b", "Whole words", "Each complete word"),
        (r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}", "Email addresses", "Basic email validation"),
        (r"https?://[^\s]+", "URLs", "Web links"),
        (r"\$\d+\.\d{2}", "Money", "$12.34, $567.89"),
        (r"(\d{1,2})/(\d{1,2})/(\d{4})", "Dates", "MM/DD/YYYY format"),
    ];
    
    println!("Here are some patterns you'll use constantly:");
    println!();
    
    for (pattern, name, description) in examples {
        println!("📋 {}: '{}'", name, pattern);
        println!("   Use case: {}", description);
        println!();
    }
}

fn step8_aoc_example() {
    println!("\n🎄 Step 8: AoC Day 5 Example");
    println!("============================");
    
    println!("Let's solve AoC 2015 Day 5 using regex!");
    println!("Requirements:");
    println!("1. At least 3 vowels [aeiou]");
    println!("2. At least one double letter (aa, bb, etc.)");  
    println!("3. No forbidden strings: ab, cd, pq, xy");
    println!();
    
    // Test strings from the problem
    let test_strings = vec![
        ("ugknbfddgicrmopn", true),  // nice
        ("aaa", true),              // nice
        ("jchzalrnumimnmhp", false), // no double letter
        ("haegwjzuvuyypxyu", false), // contains xy
        ("dvszwmarrgswjxmb", false), // only one vowel
    ];
    
    // Create the regex patterns
    let vowels = Regex::new(r"[aeiou]").unwrap();
    // Note: Rust regex doesn't support backreferences, so we'll check manually for doubles
    let forbidden = Regex::new(r"ab|cd|pq|xy").unwrap(); // | means OR
    
    println!("Regex patterns:");
    println!("  Vowels: '[aeiou]' - any vowel");
    println!("  Doubles: Check manually (Rust regex doesn't support backreferences)");
    println!("  Forbidden: 'ab|cd|pq|xy' - any of these strings");
    println!();
    
    for (test_str, expected) in test_strings {
        let vowel_count = vowels.find_iter(test_str).count();
        // Check for doubles manually since Rust regex doesn't support backreferences
        let has_double = test_str.chars()
            .zip(test_str.chars().skip(1))
            .any(|(a, b)| a == b);
        let has_forbidden = forbidden.is_match(test_str);
        
        let is_nice = vowel_count >= 3 && has_double && !has_forbidden;
        let result = if is_nice == expected { "✅" } else { "❌" };
        
        println!("'{}' {}", test_str, result);
        println!("  Vowels: {} (need ≥3)", vowel_count);
        println!("  Double letter: {} (need true)", has_double);
        println!("  Has forbidden: {} (need false)", has_forbidden);
        println!("  → {}", if is_nice { "NICE" } else { "NAUGHTY" });
        println!();
    }
}

fn step9_practice() {
    println!("\n🏃‍♂️ Step 9: Practice Time!");
    println!("==========================");
    
    println!("Here are some exercises to try:");
    println!();
    println!("🎯 Beginner:");
    println!("1. Match any 3-digit number: '\\d{{3}}'");
    println!("2. Match words starting with capital: '[A-Z][a-z]+'");  
    println!("3. Match lines ending with period: '.*\\.$'");
    println!();
    println!("🎯 Intermediate:");
    println!("4. Phone numbers: '\\(?\\d{{3}}\\)?[-.]?\\d{{3}}[-.]?\\d{{4}}'");
    println!("5. Valid variable names: '[a-zA-Z_][a-zA-Z0-9_]*'");
    println!("6. HTML tags: '<[^>]+>'");
    println!();
    println!("🎯 Advanced:");
    println!("7. IPv4 addresses: '\\d{{1,3}}\\.\\d{{1,3}}\\.\\d{{1,3}}\\.\\d{{1,3}}'");
    println!("8. Balanced parentheses: (requires recursive regex - very tricky!)");
    println!();
    
    println!("💡 Learning tip: Start simple and build complexity gradually!");
    println!("💡 Test your patterns at regex101.com - great for learning!");
}

// Bonus: Helper functions for the examples above
#[allow(dead_code)]
fn is_nice_string(s: &str) -> bool {
    let vowels = Regex::new(r"[aeiou]").unwrap();
    let forbidden = Regex::new(r"ab|cd|pq|xy").unwrap();
    
    let vowel_count = vowels.find_iter(s).count();
    let has_double = s.chars().zip(s.chars().skip(1)).any(|(a, b)| a == b);
    let has_forbidden = forbidden.is_match(s);
    
    vowel_count >= 3 && has_double && !has_forbidden
}

#[allow(dead_code)]
fn extract_phone_parts(text: &str) -> Option<(String, String)> {
    let pattern = Regex::new(r"(\d{3})-(\d{4})").unwrap();
    pattern.captures(text).map(|caps| {
        (caps[1].to_string(), caps[2].to_string())
    })
}