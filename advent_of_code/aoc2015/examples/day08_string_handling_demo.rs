// Day 8 String Handling Demo
// Demonstrates the CRITICAL difference between:
// 1. Raw input strings (what AoC gives you)
// 2. Rust string literals (what the compiler processes)

fn main() {
    println!("🎄 Day 8: String Escape Sequence Analysis");
    println!("==========================================\n");

    // ============================================================================
    // SCENARIO 1: AoC Input String (RAW TEXT)
    // ============================================================================
    println!("📄 SCENARIO 1: Reading from AoC input file");
    println!("-------------------------------------------");

    // This is what you READ from the file (raw bytes):
    let raw_input = r#""aaa\"aaa""#; // Using raw string literal to preserve backslashes

    println!("Raw input (as read from file): {}", raw_input);
    println!("Length of raw input: {} chars", raw_input.len());
    println!("Bytes: {:?}", raw_input.as_bytes());

    // Breaking it down character by character:
    println!("\nCharacter-by-character breakdown:");
    for (i, ch) in raw_input.chars().enumerate() {
        println!("  [{}] = '{}' (byte: {})", i, ch, ch as u32);
    }

    println!("\n✅ This is what's ACTUALLY in the file:");
    println!("   - First char is '\"' (double quote)");
    println!("   - Then 'a', 'a', 'a'");
    println!("   - Then '\\' (backslash)");
    println!("   - Then '\"' (double quote) ");
    println!("   - Then 'a', 'a', 'a'");
    println!("   - Last char is '\"' (double quote)");
    println!("   Total: 10 characters in the CODE");

    println!("\n");

    // ============================================================================
    // SCENARIO 2: Rust String Literal (COMPILER PROCESSED)
    // ============================================================================
    println!("🦀 SCENARIO 2: Rust string literal in your code");
    println!("------------------------------------------------");

    // This is a Rust string literal - compiler processes escape sequences!
    let rust_literal = "aaa\"aaa";

    println!("Rust literal: {}", rust_literal);
    println!("Length of Rust literal: {} chars", rust_literal.len());
    println!("Bytes: {:?}", rust_literal.as_bytes());

    println!("\nCharacter-by-character breakdown:");
    for (i, ch) in rust_literal.chars().enumerate() {
        println!("  [{}] = '{}' (byte: {})", i, ch, ch as u32);
    }

    println!("\n✅ The compiler INTERPRETED the escape sequence:");
    println!("   - 'a', 'a', 'a'");
    println!("   - ONE double-quote character (the \\\" became just \")");
    println!("   - 'a', 'a', 'a'");
    println!("   Total: 7 characters in MEMORY");

    println!("\n");

    // ============================================================================
    // THE CRITICAL DIFFERENCE FOR DAY 8
    // ============================================================================
    println!("⚠️  THE CRITICAL DIFFERENCE FOR DAY 8");
    println!("=====================================\n");

    println!("When you read from the AoC input file:");
    println!("  ❌ Rust does NOT automatically process escape sequences!");
    println!("  ❌ You get the RAW text exactly as it appears in the file!");
    println!("  ❌ Backslashes are LITERAL backslash characters!");

    println!("\nExample from the problem:");
    println!("  File contains:  \"aaa\\\"aaa\"");
    println!("  You receive:    {} (10 chars)", raw_input);
    println!("  NOT:            {} (7 chars)", rust_literal);

    println!("\n");

    // ============================================================================
    // DEMONSTRATION: Reading vs Literals
    // ============================================================================
    println!("🔬 PROOF: Comparing Both Approaches");
    println!("------------------------------------\n");

    // Simulating file read (raw string)
    let from_file = r#""abc""#;
    println!("From file (raw):       '{}'", from_file);
    println!("  Length: {} chars", from_file.len());
    println!("  Bytes: {:?}", from_file.as_bytes());

    // Rust literal (compiler processed)
    let as_literal = "abc";
    println!("\nAs Rust literal:       '{}'", as_literal);
    println!("  Length: {} chars", as_literal.len());
    println!("  Bytes: {:?}", as_literal.as_bytes());

    println!("\n❌ These are DIFFERENT!");
    println!("   File input includes the surrounding quotes as characters!");

    println!("\n");

    // ============================================================================
    // WHAT YOU NEED TO DO FOR DAY 8
    // ============================================================================
    println!("✅ WHAT YOU NEED TO DO FOR DAY 8");
    println!("=================================\n");

    println!("1️⃣  Read the file → You get RAW text with literal backslashes");
    println!("   Example: \"aaa\\\\\"aaa\" (as a string of characters)\n");

    println!("2️⃣  MANUALLY parse the escape sequences:");
    println!("   - Find \\\\  → Convert to single \\");
    println!("   - Find \\\" → Convert to single \"");
    println!("   - Find \\xHH → Convert to single character with ASCII code HH\n");

    println!("3️⃣  Count characters:");
    println!("   - Code length = raw input length (including quotes)");
    println!("   - Memory length = length after parsing escape sequences (excluding quotes)\n");

    println!("4️⃣  Calculate the difference!");

    println!("\n");

    // ============================================================================
    // EXAMPLE IMPLEMENTATION
    // ============================================================================
    println!("🛠️  EXAMPLE: Manual Parsing");
    println!("============================\n");

    let test_cases = vec![
        (r#""""#, "Empty string"),
        (r#""abc""#, "Simple string"),
        (r#""aaa\"aaa""#, "Escaped quote"),
        (r#""\x27""#, "Hex escape"),
    ];

    for (raw, description) in test_cases {
        println!("Test: {}", description);
        println!("  Raw input: {}", raw);
        println!("  Code length: {}", raw.len());

        // This is what YOU need to implement:
        let memory_length = parse_memory_length(raw);
        println!("  Memory length: {}", memory_length);
        println!("  Difference: {}", raw.len() - memory_length);
        println!();
    }
}

// Example parsing function (simplified)
fn parse_memory_length(s: &str) -> usize {
    let mut count = 0;
    let mut chars = s.chars().peekable();

    // Skip opening quote
    chars.next();

    while let Some(ch) = chars.next() {
        if ch == '"' && chars.peek().is_none() {
            // Closing quote, skip it
            break;
        } else if ch == '\\' {
            // Escape sequence
            match chars.next() {
                Some('\\') | Some('"') => {
                    count += 1; // \\ or \" becomes one character
                }
                Some('x') => {
                    // \xHH becomes one character
                    chars.next(); // Skip first hex digit
                    chars.next(); // Skip second hex digit
                    count += 1;
                }
                _ => {}
            }
        } else {
            count += 1; // Regular character
        }
    }

    count
}
