use brackets_extended::*;

fn main() {
    println!("🚀 Brackets Extended - Advanced Features Demo");
    println!("===============================================\n");

    // REQ-7: Configurable Alphabet
    demo_configurable_alphabet();
    
    // REQ-8: Error Collection Mode
    demo_error_collection();
    
    // REQ-9: Unclosed Policy Options
    demo_unclosed_policies();
    
    // Iterator APIs
    demo_iterator_apis();
}

fn demo_configurable_alphabet() {
    println!("🔤 REQ-7: Configurable Alphabet");
    println!("================================");
    
    // Default alphabet (only supports (), [], {})
    let default_opts = Options::default();
    println!("Default alphabet: (),[],{{}}");
    
    // Test with default alphabet
    match validate_with_options("([{}])", &default_opts) {
        Ok(_) => println!("✅ '([{{}}])' is valid with default alphabet"),
        Err(_) => println!("❌ '([{{}}])' is invalid"),
    }
    
    // Test angle brackets with default alphabet - should fail
    match validate_with_options("<test>", &default_opts) {
        Ok(_) => println!("✅ '<test>' is valid"),
        Err(_) => println!("❌ '<test>' is invalid with default alphabet (angle brackets not supported)"),
    }
    
    // Extended alphabet with angle brackets
    let mut extended_opts = Options::default();
    extended_opts.alphabet = Alphabet::with_pairs(&[
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),  // Add angle brackets!
    ]);
    
    println!("\nExtended alphabet: (),[],({{}}),<>");
    match validate_with_options("<[()]>", &extended_opts) {
        Ok(_) => println!("✅ '<[()]>' is valid with extended alphabet"),
        Err(errs) => println!("❌ '<[()]>' failed: {:?}", errs),
    }
    
    // Custom alphabet for specific use case (e.g., HTML-like)
    let html_opts = Options {
        alphabet: Alphabet::with_pairs(&[('<', '>')]),
        error_mode: ErrorMode::StopAtFirst,
        unclosed_policy: UnclosedPolicy::EarliestOpen,
    };
    
    println!("\nCustom HTML-like alphabet: <>");
    match validate_with_options("<div><span>content</span></div>", &html_opts) {
        Ok(_) => println!("✅ HTML-like brackets are valid"),
        Err(errs) => println!("❌ HTML-like brackets failed: {:?}", errs),
    }
    
    println!();
}

fn demo_error_collection() {
    println!("📝 REQ-8: Error Collection Mode");
    println!("================================");
    
    let test_input = ")](([";  // Multiple errors: unexpected closers + unclosed opener
    
    // Mode 1: Stop at first error (default)
    let mut stop_first_opts = Options::default();
    stop_first_opts.error_mode = ErrorMode::StopAtFirst;
    
    match validate_with_options(test_input, &stop_first_opts) {
        Ok(_) => println!("✅ No errors found"),
        Err(errs) => {
            println!("🛑 StopAtFirst mode found {} error:", errs.len());
            for (i, err) in errs.iter().enumerate() {
                println!("  {}. Position {}: {:?}", i + 1, err.index, err.kind);
            }
        }
    }
    
    // Mode 2: Collect all errors
    let mut collect_all_opts = Options::default();
    collect_all_opts.error_mode = ErrorMode::CollectAll;
    
    match validate_with_options(test_input, &collect_all_opts) {
        Ok(_) => println!("✅ No errors found"),
        Err(errs) => {
            println!("\n📋 CollectAll mode found {} errors:", errs.len());
            for (i, err) in errs.iter().enumerate() {
                println!("  {}. Position {}: {:?}", i + 1, err.index, err.kind);
            }
        }
    }
    
    println!();
}

fn demo_unclosed_policies() {
    println!("🏗️ REQ-9: Unclosed Policy Options");
    println!("===================================");
    
    let test_input = "((([";  // Multiple nested unclosed brackets
    
    // Policy 1: Latest Open (default - LIFO stack behavior)
    let mut latest_opts = Options::default();
    latest_opts.unclosed_policy = UnclosedPolicy::LatestOpen;
    
    match validate_with_options(test_input, &latest_opts) {
        Ok(_) => println!("✅ No errors found"),
        Err(errs) => {
            println!("📊 LatestOpen policy (LIFO - reports last opened):");
            for (i, err) in errs.iter().enumerate() {
                match &err.kind {
                    BracketErrorKind::UnclosedOpenings { expected, open_index } => {
                        println!("  {}. Position {}: Expected '{}' to close bracket at position {}", 
                                i + 1, err.index, expected, open_index);
                    }
                    _ => println!("  {}. Position {}: {:?}", i + 1, err.index, err.kind),
                }
            }
        }
    }
    
    // Policy 2: Earliest Open (FIFO behavior)
    let mut earliest_opts = Options::default();
    earliest_opts.unclosed_policy = UnclosedPolicy::EarliestOpen;
    
    match validate_with_options(test_input, &earliest_opts) {
        Ok(_) => println!("✅ No errors found"),
        Err(errs) => {
            println!("\n📊 EarliestOpen policy (FIFO - reports first opened):");
            for (i, err) in errs.iter().enumerate() {
                match &err.kind {
                    BracketErrorKind::UnclosedOpenings { expected, open_index } => {
                        println!("  {}. Position {}: Expected '{}' to close bracket at position {}", 
                                i + 1, err.index, expected, open_index);
                    }
                    _ => println!("  {}. Position {}: {:?}", i + 1, err.index, err.kind),
                }
            }
        }
    }
    
    println!();
}

fn demo_iterator_apis() {
    println!("🔄 Iterator APIs");
    println!("================");
    
    let opts = Options::default();
    
    // API 1: validate_iter - works with char iterators
    println!("🔗 validate_iter (char positions):");
    let chars = "([)]".chars();
    match validate_iter(chars, &opts) {
        Ok(_) => println!("  ✅ Valid bracket sequence"),
        Err(errs) => {
            println!("  ❌ Found {} errors:", errs.len());
            for err in errs {
                println!("    Position {}: {:?}", err.index, err.kind);
            }
        }
    }
    
    // API 2: validate_indexed - works with (index, char) iterators  
    println!("\n🎯 validate_indexed (preserves original indices):");
    let text = "hello(world]goodbye";
    let indexed_chars: Vec<(usize, char)> = text.char_indices().collect();
    match validate_indexed(indexed_chars, &opts) {
        Ok(_) => println!("  ✅ Valid bracket sequence"),
        Err(errs) => {
            println!("  ❌ Found {} errors:", errs.len());
            for err in errs {
                println!("    Byte position {}: {:?}", err.index, err.kind);
                if let Some(ch) = text.chars().nth(err.index) {
                    println!("      Character at position: '{}'", ch);
                }
            }
        }
    }
    
    // API 3: validate_with_options - convenience for strings
    println!("\n📄 validate_with_options (string convenience):");
    match validate_with_options("function(arg1, arg2) { return [1, 2, 3]; }", &opts) {
        Ok(_) => println!("  ✅ Programming code has valid brackets"),
        Err(errs) => {
            println!("  ❌ Found {} errors:", errs.len());
            for err in errs {
                println!("    Position {}: {:?}", err.index, err.kind);
            }
        }
    }
    
    println!();
}

#[cfg(test)]
mod demo_tests {
    use super::*;
    
    #[test]
    fn test_custom_alphabet_works() {
        let mut opts = Options::default();
        opts.alphabet = Alphabet::with_pairs(&[('❮', '❯'), ('⟨', '⟩')]);
        
        assert!(validate_with_options("❮⟨content⟩❯", &opts).is_ok());
        assert!(validate_with_options("❮⟨content❯⟩", &opts).is_err());
    }
    
    #[test]
    fn test_error_collection_comprehensive() {
        let mut opts = Options::default();
        opts.error_mode = ErrorMode::CollectAll;
        
        let result = validate_with_options("))(([[]", &opts);
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        assert!(errors.len() >= 3); // Multiple types of errors
        
        // Should have unexpected closing errors
        assert!(errors.iter().any(|e| matches!(e.kind, BracketErrorKind::UnexpectedClosing { .. })));
        
        // Should have unclosed opening errors  
        assert!(errors.iter().any(|e| matches!(e.kind, BracketErrorKind::UnclosedOpenings { .. })));
    }
    
    #[test]
    fn test_unclosed_policy_difference() {
        let input = "(([";
        
        let mut latest_opts = Options::default();
        latest_opts.unclosed_policy = UnclosedPolicy::LatestOpen;
        
        let mut earliest_opts = Options::default();
        earliest_opts.unclosed_policy = UnclosedPolicy::EarliestOpen;
        
        let latest_err = validate_with_options(input, &latest_opts).unwrap_err();
        let earliest_err = validate_with_options(input, &earliest_opts).unwrap_err();
        
        // Different policies should potentially report different positions
        // for the unclosed bracket error
        assert_eq!(latest_err.len(), 1);
        assert_eq!(earliest_err.len(), 1);
        
        match (&latest_err[0].kind, &earliest_err[0].kind) {
            (BracketErrorKind::UnclosedOpenings { open_index: latest_pos, .. }, 
             BracketErrorKind::UnclosedOpenings { open_index: earliest_pos, .. }) => {
                // Latest should report position 2 (the '['), earliest should report position 0 (first '(')
                assert_eq!(*latest_pos, 2);  // Last opened bracket position
                assert_eq!(*earliest_pos, 0); // First opened bracket position
            }
            _ => panic!("Expected UnclosedOpenings errors"),
        }
    }
}