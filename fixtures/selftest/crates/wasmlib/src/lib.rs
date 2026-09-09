//! A wasm-clean library: no `std::time`, no filesystem, no threads. The
//! `wasm32 lib clippy` job lints exactly this target.

/// Adds two numbers. Deliberately trivial: this crate exists to be built on
/// several targets and toolchains, not to do anything.
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

/// Only compiled with `--features extra`; the `extra` binary calls it.
#[cfg(feature = "extra")]
pub fn extra() -> &'static str {
    "extra"
}

#[cfg(test)]
mod tests {
    #[test]
    fn adds() {
        assert_eq!(super::add(2, 2), 4);
    }

    #[cfg(feature = "extra")]
    #[test]
    fn extra_is_on() {
        assert_eq!(super::extra(), "extra");
    }
}
