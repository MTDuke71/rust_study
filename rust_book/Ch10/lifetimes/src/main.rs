//! # Chapter 10.3: Validating References with Lifetimes
//!
//! Demonstrates lifetime annotations and how Rust prevents dangling references.
//! Shows lifetime elision rules and common lifetime patterns.
//!
//! Run with: `cargo run`

fn example1_basic_lifetime_annotations() {
    println!("🦀 Example 1: Basic Lifetime Annotations");
    println!("========================================");

    // Function with lifetime annotations
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // Lifetime annotations ensure the returned reference is valid
    // as long as both input references are valid

    println!();
}

fn example2_lifetime_in_structs() {
    println!("🏗️  Example 2: Lifetime Annotations in Structs");
    println!("==============================================");

    // Struct that holds a reference, so it needs a lifetime annotation
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        // Method with lifetime annotation
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }

        // Method that returns a reference with lifetime elision
        fn first_word(&self) -> &str {
            let bytes = self.part.as_bytes();
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &self.part[0..i];
                }
            }
            &self.part[..]
        }
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("Excerpt: {}", i.part);
    println!("First word: {}", i.first_word());
    
    let announcement = "This is an important announcement!";
    let part = i.announce_and_return_part(announcement);
    println!("Returned part: {}", part);

    println!();
}

fn example3_lifetime_elision_rules() {
    println!("📦 Example 3: Lifetime Elision Rules");
    println!("====================================");

    // These functions have the same signature due to lifetime elision

    // Explicit lifetime annotations
    fn first_word_explicit<'a>(s: &'a str) -> &'a str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }

    // Implicit lifetime annotations (elision rules apply)
    fn first_word_elided(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }

    let text = String::from("hello world rust");
    
    let word1 = first_word_explicit(&text);
    let word2 = first_word_elided(&text);
    
    println!("First word (explicit): {}", word1);
    println!("First word (elided): {}", word2);
    println!("Both functions work identically!");

    println!();
}

fn example4_multiple_lifetime_parameters() {
    println!("🔧 Example 4: Multiple Lifetime Parameters");
    println!("==========================================");

    // Function with multiple lifetime parameters
    fn longest_with_an_announcement<'a, 'b>(
        x: &'a str,
        y: &'a str,
        ann: &'b str,
    ) -> &'a str
    where
        'b: 'a,  // Lifetime bound: 'b must live at least as long as 'a
    {
        println!("Attention! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let announcement = String::from("This is an important announcement!");

    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        &announcement,
    );
    
    println!("The longest string is {}", result);

    println!();
}

fn example5_lifetime_bounds() {
    println!("🎯 Example 5: Lifetime Bounds and Generic Types");
    println!("===============================================");

    use std::fmt::Display;

    // Generic function with lifetime and trait bounds
    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result1 = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "String announcement",
    );
    
    let result2 = longest_with_an_announcement(
        string1.as_str(),
        string2,
        42,
    );

    println!("Result 1: {}", result1);
    println!("Result 2: {}", result2);

    println!();
}

fn example6_static_lifetime() {
    println!("⚡ Example 6: Static Lifetime");
    println!("=============================");

    // The 'static lifetime means the reference can live for the entire duration of the program
    let s: &'static str = "I have a static lifetime.";
    println!("Static string: {}", s);

    // String literals have 'static lifetime
    fn make_static() -> &'static str {
        "This string lives for the entire program"
    }

    let static_string = make_static();
    println!("Static string from function: {}", static_string);

    // Note: Be very careful with 'static lifetime
    // Most references should not need it

    println!();
}

fn example7_common_lifetime_patterns() {
    println!("🔗 Example 7: Common Lifetime Patterns");
    println!("======================================");

    // Pattern 1: Returning a reference from a function
    fn get_first_word(s: &str) -> &str {
        s.split_whitespace().next().unwrap_or("")
    }

    // Pattern 2: Struct holding references
    struct TextHolder<'a> {
        text: &'a str,
        word_count: usize,
    }

    impl<'a> TextHolder<'a> {
        fn new(text: &'a str) -> TextHolder<'a> {
            TextHolder {
                word_count: text.split_whitespace().count(),
                text,
            }
        }

        fn get_first_word(&self) -> &str {
            self.text.split_whitespace().next().unwrap_or("")
        }
    }

    // Pattern 3: Multiple references with same lifetime
    fn combine_strings<'a>(s1: &'a str, s2: &'a str) -> String {
        format!("{} {}", s1, s2)
    }

    let text = String::from("Hello world from Rust");
    let holder = TextHolder::new(&text);
    
    println!("Text: {}", holder.text);
    println!("Word count: {}", holder.word_count);
    println!("First word (from method): {}", holder.get_first_word());
    println!("First word (from function): {}", get_first_word(&text));
    println!("Combined: {}", combine_strings("Hello", "Rust"));

    println!();
}

fn example8_lifetime_errors_demonstration() {
    println!("⚠️  Example 8: Common Lifetime Mistakes (Commented Out)");
    println!("======================================================");

    // These would cause compilation errors - shown as comments:

    // ❌ This won't compile - returning reference to local variable
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s  // Error: returns a reference to data owned by the current function
    // }

    // ❌ This won't compile - lifetime mismatch
    // fn longest_broken<'a>(x: &'a str, y: &str) -> &'a str {
    //     y  // Error: y doesn't have lifetime 'a
    // }

    // ✅ Correct approach - return owned data instead of reference
    fn no_dangle() -> String {
        let s = String::from("hello");
        s  // Move ownership instead of returning reference
    }

    // ✅ Correct approach - ensure both parameters have same lifetime
    fn longest_correct<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    let result = no_dangle();
    println!("Owned string: {}", result);

    let s1 = "short";
    let s2 = "longer string";
    let longest = longest_correct(s1, s2);
    println!("Longest: {}", longest);

    println!("✅ All examples show correct lifetime usage!");
    println!();
}

fn main() {
    println!("📚 Chapter 10.3: Validating References with Lifetimes\n");

    example1_basic_lifetime_annotations();
    example2_lifetime_in_structs();
    example3_lifetime_elision_rules();
    example4_multiple_lifetime_parameters();
    example5_lifetime_bounds();
    example6_static_lifetime();
    example7_common_lifetime_patterns();
    example8_lifetime_errors_demonstration();

    println!("✅ All examples completed!");
    println!("📖 Next: Review the complete Chapter 10 README.md");
    println!("🔗 Related: Check out Mission3 and Mission5 for practical applications!");
}
