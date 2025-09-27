//! # Understanding Rust Closures from Scratch
//! 
//! **Learning Path**: From simple functions to advanced closure patterns
//! **Focus**: Build intuition step by step with concrete examples

fn main() {
    println!("🧠 Understanding Closures Step-by-Step");
    println!("======================================\n");
    
    step1_what_is_a_closure();
    step2_closure_syntax();
    step3_closures_vs_functions();
    step4_capturing_environment();
    step5_closure_in_iterators();
    step6_real_world_examples();
    step7_the_confusing_parts();
}

/// Step 1: What exactly IS a closure?
fn step1_what_is_a_closure() {
    println!("🎯 Step 1: What is a Closure?");
    println!("=============================");
    
    println!("Think of a closure as a 'mini-function' that you can:");
    println!("  1. Define inline (right where you use it)");
    println!("  2. Pass to other functions");
    println!("  3. 'Capture' variables from surrounding scope");
    
    println!("\n📝 Real-world analogy:");
    println!("A closure is like a 'sticky note with instructions' that remembers");
    println!("some context from where you wrote it.");
    
    // Simple example - a closure that adds 10
    let add_ten = |x| x + 10;
    
    println!("\nExample: let add_ten = |x| x + 10;");
    println!("  Result: add_ten(5) = {}", add_ten(5));
    
    println!("  ↳ This creates a closure that takes x and returns x + 10");
    
    println!();
}

/// Step 2: Understanding closure syntax
fn step2_closure_syntax() {
    println!("📝 Step 2: Closure Syntax Breakdown");
    println!("===================================");
    
    println!("Basic pattern: |parameters| expression");
    println!("               ↑           ↑");
    println!("               │           └─ What to do");
    println!("               └─ What to take in");
    
    // Different syntax variations
    println!("\n🔍 Syntax Examples:");
    
    // No parameters
    let say_hello = || println!("Hello!");
    println!("1. No parameters:   || println!(\"Hello!\")");
    say_hello();
    
    // One parameter (parentheses optional)
    let double = |x| x * 2;
    println!("2. One parameter:   |x| x * 2");
    println!("   Result: double(4) = {}", double(4));
    
    // Multiple parameters
    let add = |a, b| a + b;
    println!("3. Two parameters:  |a, b| a + b");
    println!("   Result: add(3, 7) = {}", add(3, 7));
    
    // With type annotations
    let multiply: fn(i32, i32) -> i32 = |x, y| x * y;
    println!("4. With types:      |x: i32, y: i32| -> i32 {{ x * y }}");
    println!("   Result: multiply(6, 7) = {}", multiply(6, 7));
    
    // Multi-line closure
    let complex_calc = |x| {
        let doubled = x * 2;
        let plus_one = doubled + 1;
        plus_one * 3
    };
    println!("5. Multi-line:      |x| {{ /* multiple statements */ }}");
    println!("   Result: complex_calc(5) = {}", complex_calc(5));
    
    println!();
}

/// Step 3: How closures differ from regular functions
fn step3_closures_vs_functions() {
    println!("⚖️  Step 3: Closures vs Functions");
    println!("=================================");
    
    // Regular function
    fn regular_function(x: i32) -> i32 {
        x * 2
    }
    
    // Equivalent closure
    let closure_version = |x: i32| x * 2;
    
    println!("Regular Function:");
    println!("  fn regular_function(x: i32) -> i32 {{ x * 2 }}");
    println!("  Call: regular_function(5) = {}", regular_function(5));
    
    println!("\nClosure Version:");
    println!("  let closure_version = |x: i32| x * 2;");
    println!("  Call: closure_version(5) = {}", closure_version(5));
    
    println!("\n🤔 Key Differences:");
    println!("  ✅ Functions: Named, can be called from anywhere");
    println!("  ✅ Closures: Can be anonymous, can capture variables");
    
    // Show where closures shine
    let multiplier = 3;
    let multiply_by_three = |x| x * multiplier;  // Captures 'multiplier'!
    
    println!("\n💡 Closures can 'capture' variables:");
    println!("  let multiplier = 3;");
    println!("  let multiply_by_three = |x| x * multiplier;");
    println!("  Result: multiply_by_three(4) = {}", multiply_by_three(4));
    println!("  ↳ The closure 'remembers' multiplier = 3");
    
    println!();
}

/// Step 4: The magic of capturing environment
fn step4_capturing_environment() {
    println!("🎪 Step 4: Capturing the Environment");
    println!("====================================");
    
    println!("This is where closures get their superpowers!");
    
    let name = "Alice".to_string();
    let age = 25;
    
    // Closure that captures both variables
    let introduce = || {
        println!("Hi, I'm {} and I'm {} years old", name, age);
    };
    
    println!("\nVariables in scope:");
    println!("  let name = \"Alice\";");
    println!("  let age = 25;");
    
    println!("\nClosure that captures them:");
    println!("  let introduce = || {{");
    println!("      println!(\"Hi, I'm {{}} and I'm {{}} years old\", name, age);");
    println!("  }};");
    
    println!("\nCalling the closure:");
    introduce();
    
    println!("  ↳ The closure 'captured' name and age from its environment!");
    
    // Show different capture types
    println!("\n🔍 Different ways to capture:");
    
    let mut count = 0;
    
    // Immutable borrow
    let read_count = || println!("Count is: {}", count);
    println!("1. Immutable capture: || println!(\"Count: {{}}\", count)");
    read_count();
    
    // Mutable borrow
    let mut increment = || {
        count += 1;
        println!("Incremented to: {}", count);
    };
    println!("2. Mutable capture: || {{ count += 1; ... }}");
    increment();
    increment();
    
    // Move capture
    let message = "Hello".to_string();
    let take_ownership = move || {
        println!("I own this message: {}", message);
        // message is now owned by the closure
    };
    println!("3. Move capture: move || {{ /* uses message */ }}");
    take_ownership();
    // println!("{}", message); // ← This would error! message was moved
    
    println!();
}

/// Step 5: Closures with iterators (the common case)
fn step5_closure_in_iterators() {
    println!("🔄 Step 5: Closures with Iterators");
    println!("==================================");
    
    let numbers = vec![1, 2, 3, 4, 5];
    
    println!("Working with: {:?}", numbers);
    
    // Example 1: map (transform each item)
    println!("\n1. map() - Transform each item:");
    println!("   numbers.iter().map(|x| x * 2)");
    
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("   Result: {:?}", doubled);
    println!("   ↳ The closure |x| x * 2 is applied to each number");
    
    // Example 2: filter (keep only some items)
    println!("\n2. filter() - Keep only some items:");
    println!("   numbers.iter().filter(|&x| x > 3)");
    
    let big_numbers: Vec<&i32> = numbers.iter().filter(|&x| *x > 3).collect();
    println!("   Result: {:?}", big_numbers);
    println!("   ↳ The closure |&x| *x > 3 tests each number");
    
    // Example 3: The confusing max_by_key
    println!("\n3. max_by_key() - Find max by some criteria:");
    
    let words = vec!["cat", "elephant", "dog"];
    println!("   Words: {:?}", words);
    
    if let Some(longest) = words.iter().max_by_key(|&word| word.len()) {
        println!("   words.iter().max_by_key(|&word| word.len())");
        println!("   Longest word: '{}'", longest);
        println!("   ↳ The closure |&word| word.len() extracts length for comparison");
    }
    
    println!();
}

/// Step 6: Real-world examples
fn step6_real_world_examples() {
    println!("🌍 Step 6: Real-World Examples");
    println!("==============================");
    
    // Example 1: Event handling (conceptual)
    println!("1. Event Handlers:");
    println!("   button.on_click(|| println!(\"Button clicked!\"));");
    println!("   ↳ The closure runs when button is clicked");
    
    // Example 2: Configuration
    let debug_mode = true;
    let logger = |message: &str| {
        if debug_mode {
            println!("[DEBUG] {}", message);
        }
    };
    
    println!("\n2. Configuration-aware functions:");
    println!("   let debug_mode = true;");
    println!("   let logger = |message| {{ if debug_mode {{ ... }} }};");
    logger("System started");
    println!("   ↳ Logger closure captures debug_mode setting");
    
    // Example 3: Custom sorting
    let mut people = vec![
        ("Alice", 30),
        ("Bob", 25), 
        ("Charlie", 35),
    ];
    
    println!("\n3. Custom sorting:");
    println!("   people: {:?}", people);
    
    people.sort_by_key(|(_, age)| *age);  // Sort by age
    println!("   After sort_by_key(|(_, age)| *age):");
    println!("   Result: {:?}", people);
    println!("   ↳ Closure extracts age for sorting comparison");
    
    println!();
}

/// Step 7: The parts that commonly confuse people
fn step7_the_confusing_parts() {
    println!("😵 Step 7: The Confusing Parts Explained");
    println!("========================================");
    
    // Confusing part 1: Pattern matching in parameters
    println!("1. Pattern Matching in Closure Parameters:");
    
    let pairs = vec![(1, "one"), (2, "two"), (3, "three")];
    
    println!("   pairs = {:?}", pairs);
    println!();
    
    // Different ways to handle the same tuple
    println!("   Method A: |tuple| tuple.0");
    let first_elements_a: Vec<i32> = pairs.iter().map(|tuple| tuple.0).collect();
    println!("   Result: {:?}", first_elements_a);
    
    println!("   Method B: |(number, _)| number");  
    let first_elements_b: Vec<i32> = pairs.iter().map(|(number, _)| *number).collect();
    println!("   Result: {:?}", first_elements_b);
    
    println!("   Method C: |&(number, _)| number");
    let first_elements_c: Vec<i32> = pairs.iter().map(|&(number, _)| number).collect();
    println!("   Result: {:?}", first_elements_c);
    
    println!("   ↳ All three do the same thing, just different syntax!");
    
    // Confusing part 2: References and dereferencing
    println!("\n2. References and Dereferencing:");
    
    let numbers = vec![1, 2, 3];
    println!("   numbers = {:?} (type: Vec<i32>)", numbers);
    println!("   numbers.iter() produces &i32 references");
    
    // Show what the closure actually receives
    numbers.iter().for_each(|item| {
        println!("     Closure gets: {:?} (type: &i32)", item);
    });
    
    println!("\n   To get the actual value:");
    println!("   Method A: |item| *item    (manual dereference)");
    println!("   Method B: |&item| item    (pattern match dereference)");
    
    let doubled_a: Vec<i32> = numbers.iter().map(|item| *item * 2).collect();
    let doubled_b: Vec<i32> = numbers.iter().map(|&item| item * 2).collect();
    
    println!("   Both give: {:?}", doubled_a);
    println!("   And:       {:?}", doubled_b);
    
    // Confusing part 3: The original max_by_key example!
    println!("\n3. Breaking Down the Original Example:");
    println!("   word_count.iter().max_by_key(|(_, &count)| count)");
    println!("                                  ↑    ↑       ↑");
    println!("                                  │    │       └─ Return count for comparison");
    println!("                                  │    └─ Pattern match: &i32 → i32"); 
    println!("                                  └─ Ignore the word with _");
    
    println!("\n💡 Key Insight: Closures are just anonymous functions");
    println!("   that can capture variables from their environment!");
    
    println!();
}

// Bonus: Interactive exercises to build intuition
fn bonus_exercises() {
    println!("🎯 Bonus: Try These Mental Exercises");
    println!("====================================");
    
    println!("1. What does this closure do?");
    println!("   |x| x > 10");
    println!("   Answer: Takes a number, returns true if greater than 10");
    
    println!("\n2. What does this closure capture?");
    println!("   let threshold = 5;");
    println!("   let check = |x| x > threshold;");
    println!("   Answer: Captures 'threshold' from environment");
    
    println!("\n3. Why use |&x| instead of |x|?");
    println!("   vec![1,2,3].iter().map(|&x| x * 2)");
    println!("   Answer: iter() gives &i32, |&x| pattern matches to get i32");
    
    println!("\n4. What's the difference?");
    println!("   |x| x * 2        vs        |&x| x * 2");
    println!("   Answer: First takes i32, second takes &i32 and dereferences");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_closure_basics() {
        // Simple closure
        let add_one = |x| x + 1;
        assert_eq!(add_one(5), 6);
        
        // Closure with capture
        let multiplier = 3;
        let multiply = |x| x * multiplier;
        assert_eq!(multiply(4), 12);
        
        // Closure with pattern matching
        let pairs = vec![(1, 2), (3, 4)];
        let first_elements: Vec<i32> = pairs.iter().map(|&(first, _)| first).collect();
        assert_eq!(first_elements, vec![1, 3]);
    }
}