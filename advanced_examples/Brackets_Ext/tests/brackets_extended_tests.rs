use brackets_extended::*;

#[test]
fn req7_configurable_alphabet_with_angles() {
    let mut opts = Options::default();
    opts.alphabet = Alphabet::with_pairs(&[('(',')'),('[',']'),('{','}'),('<','>')]);
    assert!(validate_with_options("<[()]>", &opts).is_ok());
    let errs = validate_with_options("<[()>", &opts).unwrap_err();
    assert!(!errs.is_empty());
}

#[test]
fn req8_collect_all_errors() {
    let mut opts = Options::default();
    opts.error_mode = ErrorMode::CollectAll;
    // Start with two closers and one opener left unclosed
    let errs = validate_with_options(")](", &opts).unwrap_err();
    assert!(errs.len() >= 2);
    // Ensure we see at least one UnexpectedClosing
    assert!(errs.iter().any(|e| matches!(e.kind, BracketErrorKind::UnexpectedClosing{..})));
}

#[test]
fn req9_unclosed_policy_latest_vs_earliest() {
    let mut opts = Options::default();
    opts.unclosed_policy = UnclosedPolicy::LatestOpen;
    let e1 = validate_with_options("(((", &opts).unwrap_err();
    assert_eq!(e1[0].index, 2);

    opts.unclosed_policy = UnclosedPolicy::EarliestOpen;
    let e2 = validate_with_options("(((", &opts).unwrap_err();
    assert_eq!(e2[0].index, 0);
}

#[test]
fn iterator_api_char_positions() {
    let mut opts = Options::default();
    opts.alphabet = Alphabet::with_pairs(&[('<','>')]); // only angle brackets
    // characters positions: 0:'<',1:'>',2:')' ignored because not in alphabet
    let res = validate_iter("<>)".chars(), &opts);
    assert!(res.is_ok());
}

#[test]
fn iterator_api_indexed_preserves_byte_indices() {
    let s = "é)"; // 'é' is 2 bytes, ')' at byte index 2
    let iter = s.char_indices();
    let mut opts = Options::default();
    // only ')' considered as closer because '(' isn't present but ')' is in default closers set via values();
    // make an explicit alphabet to avoid accidental matches:
    opts.alphabet = Alphabet::with_pairs(&[('(', ')')]);
    let errs = validate_indexed(iter, &opts).unwrap_err();
    // Unexpected ')' at byte index 2
    assert!(matches!(errs[0].kind, BracketErrorKind::UnexpectedClosing { found: ')' }));
    assert_eq!(errs[0].index, 2);
}
