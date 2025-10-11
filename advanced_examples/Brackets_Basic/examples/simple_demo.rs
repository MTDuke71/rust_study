// Interactive Bracket Validation Examples
// Run with: cargo run --example simple_demo

use brackets_basic::{validate_brackets, BracketErrorKind};

fn main() {
    println!("🎯 Interactive Bracket Validation Demo\n");

    // ==============================================
    // Example 1: Simple valid cases
    // ==============================================
    println!("✅ Valid Examples:");

    let valid_examples = vec![
        "",            // Empty string is valid
        "()",          // Simple pair
        "[]",          // Different brackets
        "{}",          // Curly braces
        "([])",        // Nested
        "{[()]}",      // Multiple nesting
        "abc(def)ghi", // With other characters (ignored)
    ];

    for example in valid_examples {
        match validate_brackets(example) {
            Ok(()) => println!("  '{}' ✅ Valid", example),
            Err(e) => println!("  '{}' ❌ Error: {:?}", example, e),
        }
    }

    // ==============================================
    // Example 2: Error cases with explanations
    // ==============================================
    println!("\n❌ Error Examples:");

    let error_examples = vec![
        ")",    // Unexpected closing
        "]",    // Unexpected closing (different type)
        "(",    // Unclosed opening
        "(]",   // Mismatched pair
        "([)]", // Wrong nesting order
        "(((",  // Multiple unclosed
        ")))",  // Multiple unexpected closing
    ];

    for example in error_examples {
        match validate_brackets(example) {
            Ok(()) => println!("  '{}' ✅ Valid (unexpected!)", example),
            Err(e) => {
                println!(
                    "  '{}' ❌ Error at position {}: {}",
                    example,
                    e.index,
                    explain_error(&e.kind)
                );
            }
        }
    }

    // ==============================================
    // Example 3: Step-by-step walkthrough
    // ==============================================
    println!("\n🔍 Step-by-Step Walkthrough:");
    walkthrough_example("(a[b{c}d]e)");
}

fn explain_error(kind: &BracketErrorKind) -> String {
    match kind {
        BracketErrorKind::UnexpectedClosing { found } => {
            format!("Found closing '{}' but no matching opening bracket", found)
        }
        BracketErrorKind::MismatchedPair { expected, found } => {
            format!("Expected '{}' but found '{}'", expected, found)
        }
        BracketErrorKind::UnclosedOpenings {
            expected,
            open_index,
        } => {
            format!(
                "Unclosed opening bracket at position {}, expected '{}'",
                open_index, expected
            )
        }
    }
}

fn walkthrough_example(input: &str) {
    println!("Walking through: '{}'", input);
    println!("Stack operations:");

    // This is a simplified version of the algorithm for demonstration
    let mut stack_simulation = Vec::new();

    for (i, ch) in input.char_indices() {
        match ch {
            '(' => {
                stack_simulation.push(')');
                println!(
                    "  Position {}: '{}' → Push ')' onto stack. Stack: {:?}",
                    i, ch, stack_simulation
                );
            }
            '[' => {
                stack_simulation.push(']');
                println!(
                    "  Position {}: '{}' → Push ']' onto stack. Stack: {:?}",
                    i, ch, stack_simulation
                );
            }
            '{' => {
                stack_simulation.push('}');
                println!(
                    "  Position {}: '{}' → Push '}}' onto stack. Stack: {:?}",
                    i, ch, stack_simulation
                );
            }
            ')' | ']' | '}' => {
                if let Some(expected) = stack_simulation.pop() {
                    if ch == expected {
                        println!(
                            "  Position {}: '{}' → Pop '{}' from stack. Match! Stack: {:?}",
                            i, ch, expected, stack_simulation
                        );
                    } else {
                        println!("  Position {}: '{}' → Pop '{}' from stack. MISMATCH! Expected '{}', found '{}'", i, ch, expected, expected, ch);
                        return;
                    }
                } else {
                    println!(
                        "  Position {}: '{}' → Stack empty! Unexpected closing bracket",
                        i, ch
                    );
                    return;
                }
            }
            _ => {
                println!("  Position {}: '{}' → Ignore (not a bracket)", i, ch);
            }
        }
    }

    if stack_simulation.is_empty() {
        println!("  End: Stack empty ✅ All brackets matched!");
    } else {
        println!(
            "  End: Stack not empty ❌ Unclosed brackets: {:?}",
            stack_simulation
        );
    }

    // Now run the actual validator
    println!("\nActual validator result:");
    match validate_brackets(input) {
        Ok(()) => println!("  ✅ Valid"),
        Err(e) => println!("  ❌ Error: {}", explain_error(&e.kind)),
    }
}
