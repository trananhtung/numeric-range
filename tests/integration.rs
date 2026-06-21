//! End-to-end behavioral spec for the public `numeric-range` API.

use numeric_range::{format, parse, ParseError};

// ---------------------------------------------------------------------------
// parse
// ---------------------------------------------------------------------------

#[test]
fn parse_mixed() {
    assert_eq!(parse("1,3-5,7").unwrap(), vec![1, 3, 4, 5, 7]);
}

#[test]
fn parse_single_and_range() {
    assert_eq!(parse("5").unwrap(), vec![5]);
    assert_eq!(parse("3-5").unwrap(), vec![3, 4, 5]);
    assert_eq!(parse("10-10").unwrap(), vec![10]);
}

#[test]
fn parse_preserves_input_order() {
    assert_eq!(parse("3-5,1").unwrap(), vec![3, 4, 5, 1]);
}

#[test]
fn parse_tolerates_whitespace() {
    assert_eq!(parse("  1 , 2 - 4 ").unwrap(), vec![1, 2, 3, 4]);
}

#[test]
fn parse_empty_is_empty() {
    assert_eq!(parse("").unwrap(), Vec::<u64>::new());
    assert_eq!(parse("   ").unwrap(), Vec::<u64>::new());
}

#[test]
fn parse_errors() {
    assert!(matches!(
        parse("5-1"),
        Err(ParseError::DescendingRange { .. })
    ));
    assert!(matches!(parse("a"), Err(ParseError::InvalidNumber(_))));
    assert!(matches!(parse("1-"), Err(ParseError::InvalidNumber(_))));
    assert!(matches!(parse("1,,2"), Err(ParseError::EmptyPart)));
    assert!(matches!(parse("1-2-3"), Err(ParseError::InvalidRange(_))));
}

// ---------------------------------------------------------------------------
// Regression tests from the adversarial pre-publish review
// ---------------------------------------------------------------------------

#[test]
fn huge_range_is_rejected_not_oom() {
    // A tiny string must not be allowed to allocate gigabytes.
    assert!(matches!(
        parse("1-9999999999"),
        Err(ParseError::RangeTooLarge { .. })
    ));
    assert!(matches!(
        parse("0-18446744073709551615"),
        Err(ParseError::RangeTooLarge { .. })
    ));
    // A large-but-reasonable range still expands.
    assert_eq!(parse("1-1000").unwrap().len(), 1000);
}

#[test]
fn leading_plus_is_rejected() {
    assert!(matches!(parse("+5"), Err(ParseError::InvalidNumber(_))));
    assert!(matches!(parse("1-+3"), Err(ParseError::InvalidNumber(_))));
}

// ---------------------------------------------------------------------------
// format
// ---------------------------------------------------------------------------

#[test]
fn format_collapses_runs() {
    assert_eq!(format(&[1, 3, 4, 5, 7]), "1,3-5,7");
    assert_eq!(format(&[1, 2, 3, 4]), "1-4");
    assert_eq!(format(&[1, 3, 5]), "1,3,5");
}

#[test]
fn format_pairs_and_singletons() {
    assert_eq!(format(&[5]), "5");
    assert_eq!(format(&[1, 2]), "1-2");
    assert_eq!(format(&[]), "");
}

#[test]
fn format_sorts_and_dedups() {
    assert_eq!(format(&[5, 3, 4, 1, 7]), "1,3-5,7");
    assert_eq!(format(&[1, 1, 2, 2, 3]), "1-3");
}

// ---------------------------------------------------------------------------
// round-trip
// ---------------------------------------------------------------------------

#[test]
fn round_trip() {
    for s in ["1,3-5,7", "1-4", "5", "1,3,5", "0,2,100-103"] {
        let values = parse(s).unwrap();
        assert_eq!(format(&values), s, "round trip changed {s:?}");
    }
}
