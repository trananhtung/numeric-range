//! # numeric-range — compact integer-range strings
//!
//! Parse and format the familiar `"1,3-5,7"` range syntax used by print dialogs
//! (page ranges), CPU affinity lists (`taskset`/cgroups), line selectors, and
//! CLI flags — in **both** directions.
//!
//! ```
//! assert_eq!(numeric_range::parse("1,3-5,7").unwrap(), vec![1, 3, 4, 5, 7]);
//! assert_eq!(numeric_range::format(&[1, 3, 4, 5, 7]), "1,3-5,7");
//! ```
//!
//! Zero dependencies, `#![no_std]` (needs only `alloc`).

#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{self, Write};

/// Maximum number of values [`parse`] will expand, to bound memory on untrusted
/// input (a tiny string like `"1-9999999999"` would otherwise request gigabytes).
const MAX_VALUES: u128 = 10_000_000;

/// An error produced by [`parse`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseError {
    /// A comma-separated part was empty (e.g. `"1,,2"` or a trailing comma).
    EmptyPart,
    /// A part was not a valid unsigned integer.
    InvalidNumber(String),
    /// A part had the wrong shape for a range (not `a` or `a-b`).
    InvalidRange(String),
    /// A range's start was greater than its end (e.g. `"5-1"`).
    DescendingRange {
        /// The (larger) start value.
        start: u64,
        /// The (smaller) end value.
        end: u64,
    },
    /// Expansion would exceed the supported maximum number of values
    /// (guards against memory-exhaustion from a tiny input).
    RangeTooLarge {
        /// The range's start value.
        start: u64,
        /// The range's end value.
        end: u64,
    },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyPart => f.write_str("empty range part"),
            ParseError::InvalidNumber(s) => write!(f, "invalid number: {s:?}"),
            ParseError::InvalidRange(s) => write!(f, "invalid range: {s:?}"),
            ParseError::DescendingRange { start, end } => {
                write!(f, "descending range: {start}-{end}")
            }
            ParseError::RangeTooLarge { start, end } => {
                write!(f, "range too large to expand: {start}-{end}")
            }
        }
    }
}

impl core::error::Error for ParseError {}

/// Parse a compact range string like `"1,3-5,7"` into the expanded values,
/// preserving input order.
///
/// An empty (or whitespace-only) input yields an empty `Vec`.
///
/// # Errors
///
/// Returns [`ParseError`] for empty parts, non-numeric parts, malformed ranges,
/// or descending ranges.
pub fn parse(input: &str) -> Result<Vec<u64>, ParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    let mut out = Vec::new();
    for raw in trimmed.split(',') {
        let part = raw.trim();
        if part.is_empty() {
            return Err(ParseError::EmptyPart);
        }
        let mut bounds = part.split('-');
        let first = bounds.next().unwrap_or("").trim();
        match bounds.next() {
            None => out.push(number(first)?),
            Some(second) => {
                if bounds.next().is_some() {
                    return Err(ParseError::InvalidRange(part.into()));
                }
                let start = number(first)?;
                let end = number(second.trim())?;
                if start > end {
                    return Err(ParseError::DescendingRange { start, end });
                }
                // Bound expansion so a tiny string can't exhaust memory.
                let span = u128::from(end - start) + 1;
                if out.len() as u128 + span > MAX_VALUES {
                    return Err(ParseError::RangeTooLarge { start, end });
                }
                out.extend(start..=end);
            }
        }
    }
    Ok(out)
}

fn number(s: &str) -> Result<u64, ParseError> {
    // `u64::from_str` tolerates a leading '+'; reject it to stay strict and keep
    // parse/format round-trips canonical.
    if s.starts_with('+') {
        return Err(ParseError::InvalidNumber(s.into()));
    }
    s.parse().map_err(|_| ParseError::InvalidNumber(s.into()))
}

/// Format a slice of integers into a compact range string, e.g.
/// `[1, 3, 4, 5, 7]` → `"1,3-5,7"`. Values are sorted and de-duplicated first.
#[must_use]
pub fn format(values: &[u64]) -> String {
    if values.is_empty() {
        return String::new();
    }
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    sorted.dedup();

    let mut out = String::new();
    let mut i = 0;
    while i < sorted.len() {
        let start = sorted[i];
        let mut end = start;
        // Extend the run while the next value is exactly end + 1 (no overflow).
        while i + 1 < sorted.len() && end.checked_add(1) == Some(sorted[i + 1]) {
            end = sorted[i + 1];
            i += 1;
        }
        if !out.is_empty() {
            out.push(',');
        }
        // Writing to a `String` is infallible.
        if start == end {
            let _ = write!(out, "{start}");
        } else {
            let _ = write!(out, "{start}-{end}");
        }
        i += 1;
    }
    out
}
