// Advanced Bracket Validation Examples
// Run with: cargo run --example advanced_demo

use brackets_basic::{validate_brackets, BracketErrorKind};

fn main() {
    println!("🚀 Advanced Bracket Validation Examples\n");

    // ==============================================
    // Real-world programming examples
    // ==============================================
    println!("📝 Real Programming Examples:");

    let code_examples = vec![
        ("if (x > 0) { print(\"positive\"); }", true),
        ("array[index] = { key: values[i] };", true),
        ("func(a, [b, c], {d: e})", true),
        (
            "JSON: {\"users\": [{\"name\": \"Alice\"}, {\"name\": \"Bob\"}]}",
            true,
        ),
        ("function add(a, b) { return a + b; }", true),
        // Error cases
        ("if (x > 0 { print(\"missing closing paren\"); }", false),
        ("array[index = { key: values[i] };", false),
        ("func(a, [b, c}, {d: e})", false),
    ];

    for (code, should_be_valid) in code_examples {
        let result = validate_brackets(code);
        let is_valid = result.is_ok();
        let status = if is_valid == should_be_valid {
            "✅"
        } else {
            "⚠️"
        };

        match result {
            Ok(()) => println!("  {} '{}' → Valid", status, code),
            Err(e) => println!(
                "  {} '{}' → Error: {}",
                status,
                code,
                explain_error(&e.kind)
            ),
        }
    }

    // ==============================================
    // Edge cases and tricky examples
    // ==============================================
    println!("\n🎯 Edge Cases and Tricky Examples:");

    let tricky_examples = vec![
        "((()))",                              // Deeply nested same type
        "()()()()()()()(",                     // Many pairs + one unclosed
        "([{}])",                              // All three types nested
        "{[()()()()()]]}",                     // Mixed nesting with repeats
        "text(more[text{inside}more]text)end", // Real text mixed in
        "(((",                                 // Multiple unclosed (reports first)
        ")))",                                 // Multiple unexpected (reports first)
        "([)]",                                // Classic wrong order
        "({[}])",                              // Complex wrong order
    ];

    for example in tricky_examples {
        match validate_brackets(example) {
            Ok(()) => println!("  ✅ '{}' → Valid", example),
            Err(e) => println!(
                "  ❌ '{}' → Error at pos {}: {}",
                example,
                e.index,
                explain_error(&e.kind)
            ),
        }
    }

    // ==============================================
    // Performance demonstration
    // ==============================================
    println!("\n⚡ Performance Test:");

    // Create a large valid string
    let large_valid = "(".repeat(1000) + &")".repeat(1000);
    let start = std::time::Instant::now();
    let result = validate_brackets(&large_valid);
    let duration = start.elapsed();

    println!(
        "  Large string (2000 chars): {:?} in {:?}",
        result.is_ok(),
        duration
    );

    // Create a large invalid string (error at the end)
    let large_invalid = "(".repeat(1000) + &")".repeat(999) + "]";
    let start = std::time::Instant::now();
    let result = validate_brackets(&large_invalid);
    let duration = start.elapsed();

    match result {
        Ok(()) => println!("  Large invalid: Valid (unexpected!)"),
        Err(e) => println!(
            "  Large invalid: Error at pos {} in {:?}",
            e.index, duration
        ),
    }

    // ==============================================
    // Interactive challenge
    // ==============================================
    println!("\n🎲 Can you predict these results?");

    let challenge_cases = vec!["()()()(", "([)]", "{[(])}", "(((", ")))", ""];

    for case in &challenge_cases {
        println!("  '{}' → ?", case);
    }

    println!("\nPress Enter to see answers...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    println!("Answers:");
    for case in &challenge_cases {
        match validate_brackets(case) {
            Ok(()) => println!("  '{}' → ✅ Valid", case),
            Err(e) => println!("  '{}' → ❌ Error: {}", case, explain_error(&e.kind)),
        }
    }
}

fn explain_error(kind: &BracketErrorKind) -> String {
    match kind {
        BracketErrorKind::UnexpectedClosing { found } => {
            format!("Unexpected '{}' (no matching opener)", found)
        }
        BracketErrorKind::MismatchedPair { expected, found } => {
            format!("Expected '{}' but found '{}'", expected, found)
        }
        BracketErrorKind::UnclosedOpenings { expected, .. } => {
            format!("Unclosed bracket (expected '{}')", expected)
        }
    }
}
