//! A crate whose only test is a doctest.
//!
//! `cargo test --all-targets` does NOT run doctests — rust.yml runs them in a
//! separate step when `test-args` contains `--all-targets`, and this crate is
//! how the selftest proves that step exists.

/// Doubles a number. See [`hidden`] for the negative.
///
/// ```
/// assert_eq!(selftest_doctested::double(21), 42);
/// ```
pub fn double(n: u32) -> n_alias::N {
    n * 2
}

/// A private-ish module the doc gate must be able to link to. See the header.
pub mod n_alias {
    /// The numeric type [`double`](super::double) returns.
    pub type N = u32;
}

#[allow(dead_code)]
fn hidden() {}
