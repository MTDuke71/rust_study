// Algorithm Analysis: Why Stack is Perfect for Bracket Validation
// Run with: cargo run --example algorithm_analysis

use brackets_basic::{validate_brackets, BracketErrorKind};

fn main() {
    println!("🧠 Algorithm Analysis: Why Stack is Perfect for Bracket Validation\n");

    // ==============================================
    // The Core Insight
    // ==============================================
    println!("🎯 The Core Insight:");
    println!("Bracket validation is fundamentally about MATCHING PAIRS in NESTED STRUCTURES");
    println!("  • When we see '(' we PROMISE to see ')' later");
    println!("  • When we see '[' we PROMISE to see ']' later");
    println!("  • When we see '{{' we PROMISE to see '}}' later");
    println!("  • The LAST promise made must be the FIRST promise fulfilled (LIFO = Stack!)");

    // ==============================================
    // Why other data structures don't work
    // ==============================================
    println!("\n❌ Why Other Data Structures Don't Work:");

    println!("\n📋 Array/Vec (without stack behavior):");
    println!("  Example: '([)]'");
    println!("  • Array would store: [')', ']']");
    println!("  • When we see ')', which one to check? First? Last?");
    println!("  • No clear ordering rule → Wrong!");

    println!("\n🔄 Queue (FIFO):");
    println!("  Example: '([)]'");
    println!("  • Queue stores: [')' ← first, ']' ← last]");
    println!("  • When we see ')', check first in queue → ')' ✅");
    println!("  • When we see ']', check first in queue → ']' ✅");
    println!("  • But this is WRONG! '([)]' should be invalid!");

    println!("\n📚 Stack (LIFO):");
    println!("  Example: '([)]'");
    println!("  • Stack stores: [bottom ')' ... top ']']");
    println!("  • When we see ')', check top of stack → ']' ≠ ')' ❌");
    println!("  • Correctly identifies the error!");

    // ==============================================
    // Step-by-step comparison
    // ==============================================
    println!("\n🔍 Step-by-Step Comparison with '([)]':");

    demonstrate_with_different_structures("([)]");

    // ==============================================
    // Time and Space Complexity
    // ==============================================
    println!("\n⚡ Time and Space Complexity:");
    println!("  Time: O(n) - visit each character exactly once");
    println!("  Space: O(n) - worst case: all opening brackets like '((((('");
    println!("  This is OPTIMAL - you can't do better than O(n) time!");

    // ==============================================
    // Real-world applications
    // ==============================================
    println!("\n🌍 Real-World Applications of This Algorithm:");

    let applications = vec![
        ("Code Editors", "Syntax highlighting, auto-completion"),
        ("Compilers", "Parsing programming languages"),
        ("JSON/XML Parsers", "Validating nested structures"),
        (
            "Mathematical Expression Evaluators",
            "Parsing (2 + (3 * 4))",
        ),
        ("HTML/XML Validators", "Matching <div> with </div>"),
        ("Shell Script Parsers", "Matching quotes and brackets"),
    ];

    for (app, desc) in applications {
        println!("  • {}: {}", app, desc);
    }

    // ==============================================
    // Extension possibilities
    // ==============================================
    println!("\n🚀 How to Extend This Algorithm:");
    println!("  • XML Tags: <div> must match </div>");
    println!("  • String Quotes: \"hello\" and 'world'");
    println!("  • Code Blocks: if/endif, begin/end");
    println!("  • Custom Delimiters: /* comment */");

    test_extension_idea();
}

fn demonstrate_with_different_structures(input: &str) {
    println!("Testing: '{}'", input);

    // Simulate different approaches
    println!("\n  📚 Stack Approach (CORRECT):");
    let mut stack = Vec::new();

    for (i, ch) in input.char_indices() {
        match ch {
            '(' => {
                stack.push(')');
                println!("    {}. '{}' → push ')'  Stack: {:?}", i, ch, stack);
            }
            '[' => {
                stack.push(']');
                println!("    {}. '{}' → push ']'  Stack: {:?}", i, ch, stack);
            }
            '{' => {
                stack.push('}');
                println!("    {}. '{}' → push '}}'  Stack: {:?}", i, ch, stack);
            }
            ')' | ']' | '}' => {
                if let Some(expected) = stack.pop() {
                    if ch == expected {
                        println!(
                            "    {}. '{}' → pop '{}'   Stack: {:?} ✅ Match!",
                            i, ch, expected, stack
                        );
                    } else {
                        println!(
                            "    {}. '{}' → pop '{}'   Stack: {:?} ❌ Expected '{}', got '{}'",
                            i, ch, expected, stack, expected, ch
                        );
                        break;
                    }
                } else {
                    println!("    {}. '{}' → empty stack ❌ Unexpected closing", i, ch);
                    break;
                }
            }
            _ => {}
        }
    }

    // Now run actual validator
    match validate_brackets(input) {
        Ok(()) => println!("  ✅ Actual result: Valid"),
        Err(e) => println!("  ❌ Actual result: Error - {}", explain_error(&e.kind)),
    }
}

fn test_extension_idea() {
    println!("\n🧪 Extension Example - XML-style Tags:");
    println!("Imagine extending the algorithm for <tag></tag> matching:");

    let xml_examples = vec![
        "<div></div>",
        "<p><b>text</b></p>",
        "<div><p></div></p>", // Wrong nesting
    ];

    for example in xml_examples {
        println!(
            "  '{}' → Would need modified algorithm for tag names",
            example
        );
    }

    println!("  The core stack principle still applies!");
    println!("  Stack would store: [(tag_name, position), ...]");
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
