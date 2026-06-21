# numeric-range

[![Crates.io](https://img.shields.io/crates/v/numeric-range.svg)](https://crates.io/crates/numeric-range)
[![Documentation](https://docs.rs/numeric-range/badge.svg)](https://docs.rs/numeric-range)
[![CI](https://github.com/trananhtung/numeric-range/actions/workflows/ci.yml/badge.svg)](https://github.com/trananhtung/numeric-range/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/numeric-range.svg)](#license)
[![no_std](https://img.shields.io/badge/no__std-yes-brightgreen.svg)](#no_std)

**Parse and format compact integer-range strings** — the familiar `"1,3-5,7"`
syntax from print dialogs (page ranges), CPU affinity lists (`taskset`, cgroups),
line selectors, and CLI flags. **Both directions**, zero dependencies, `#![no_std]`.

```rust
assert_eq!(numeric_range::parse("1,3-5,7").unwrap(), vec![1, 3, 4, 5, 7]);
assert_eq!(numeric_range::format(&[1, 3, 4, 5, 7]), "1,3-5,7");
```

## Why numeric-range?

Rust's existing crates in this space are parse-only or restrictively licensed.
`numeric-range` is a small, permissive (MIT/Apache-2.0), `no_std` crate that does
**both** parse *and* compact-format, so you can round-trip a user's `--pages`
flag, normalize a CPU set, or render a selection back to its shortest form.

```toml
[dependencies]
numeric-range = "0.1"
```

## API

| Function | Behavior |
| --- | --- |
| `parse(&str) -> Result<Vec<u64>, ParseError>` | `"1,3-5,7"` → `[1,3,4,5,7]` (input order preserved) |
| `format(&[u64]) -> String` | `[5,3,4,1,7]` → `"1,3-5,7"` (sorted, de-duplicated, runs collapsed) |

- Whitespace around numbers and separators is tolerated (`"1, 3 - 5"`).
- Empty input parses to an empty `Vec`.
- Errors: empty parts (`"1,,2"`), non-numbers (`"a"`, `"+5"`), malformed ranges
  (`"1-2-3"`), descending ranges (`"5-1"`), and ranges that would expand beyond
  10 million values (so a tiny untrusted string can't exhaust memory).
- Values are unsigned (`u64`), matching page/CPU/line use cases.

## no_std

`numeric-range` is `#![no_std]` and only needs `alloc`. It builds for bare-metal
targets such as `thumbv7em-none-eabi`.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at
your option.
