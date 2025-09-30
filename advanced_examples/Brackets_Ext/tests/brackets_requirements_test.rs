use brackets_extended::{validate_brackets, BracketErrorKind};

// REQ-1: recognize only ()[]{} and ignore others
#[test]
fn req1_scope_ignores_non_brackets() {
    assert!(validate_brackets("a(b[c]{d}e)f 123 🚀").is_ok());
}

// REQ-2: valid iff properly matched and nested
#[test]
fn req2_correctness_proper_nesting() {
    assert!(validate_brackets("([{}])").is_ok());
    assert!(validate_brackets("{[()(){}([])]}").is_ok());
}

#[test]
fn req2_correctness_detects_incorrect_nesting() {
    let e = validate_brackets("([)]").unwrap_err();
    matches!(e.kind, BracketErrorKind::MismatchedPair{..});
}

// REQ-3: earliest error with details
#[test]
fn req3_unexpected_closing_earliest() {
    let e = validate_brackets("]abc").unwrap_err();
    match e.kind {
        BracketErrorKind::UnexpectedClosing { found } => {
            assert_eq!(found, ']'); 
            assert_eq!(e.index, 0);
        }
        _ => panic!("wrong kind"),
    }
}

#[test]
fn req3_mismatched_pair_details() {
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
fn req3_unclosed_reports_top_of_stack() {
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

// REQ-4: O(n) time/space (we do a smoke on big string)
#[test]
fn req4_smoke_big_input() {
    let s = "(".repeat(50_000) + &")".repeat(50_000);
    assert!(validate_brackets(&s).is_ok());
}

// REQ-5: API already satisfied by compilation; no runtime test needed.