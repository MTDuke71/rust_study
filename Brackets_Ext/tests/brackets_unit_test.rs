use brackets_extended::{validate_brackets, BracketErrorKind};

#[test]
fn unit_empty_and_noise_only() {
    assert!(validate_brackets("").is_ok());
    assert!(validate_brackets("abc 123 🚀").is_ok());
}

#[test]
fn unit_simple_pairs_and_nesting() {
    assert!(validate_brackets("()").is_ok());
    assert!(validate_brackets("[]{}()").is_ok());
    assert!(validate_brackets("([{}])").is_ok());
    assert!(validate_brackets("{[()(){}([])]}").is_ok());
}

#[test]
fn unit_unexpected_closing() {
    let e = validate_brackets("]x").unwrap_err();
    match e.kind {
        BracketErrorKind::UnexpectedClosing { found } => {
            assert_eq!(found, ']'); 
            assert_eq!(e.index, 0);
        }
        _ => panic!("wrong kind"),
    }
}

#[test]
fn unit_mismatched_pair() {
    // 0:'(', 1:']' -> expected ')', found ']'
    let e = validate_brackets("(]").unwrap_err();
    match e.kind {
        BracketErrorKind::MismatchedPair { expected, found } => {
            assert_eq!(expected, ')');
            assert_eq!(found, ']');
            assert_eq!(e.index, 1);
        }
        _ => panic!("wrong kind"),
    }
}

#[test]
fn unit_unclosed_topmost_is_reported() {
    // "(((" leaves three opens; we report the most recent (index 2)
    let e = validate_brackets("(((").unwrap_err();
    match e.kind {
        BracketErrorKind::UnclosedOpenings { expected, open_index } => {
            assert_eq!(expected, ')');
            assert_eq!(open_index, 2);
            assert_eq!(e.index, 2);
        }
        _ => panic!("wrong kind"),
    }
}

#[test]
fn unit_utf8_indices() {
    // 'é' is 2 bytes; ')' is at byte index 2
    let e = validate_brackets("(é]").unwrap_err();
    match e.kind {
        BracketErrorKind::MismatchedPair { expected, found } => {
            assert_eq!(expected, ')');
            assert_eq!(found, ']');
            assert_eq!(e.index, 3); // indices: 0:'(', 1..=2:'é', 3:']'
        }
        _ => panic!("wrong kind"),
    }
}