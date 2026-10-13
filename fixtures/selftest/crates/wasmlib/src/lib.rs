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

/// Only compiled with the default feature `heavy`, which links `selftest-heavy`.
#[cfg(feature = "heavy")]
pub fn weight() -> u32 {
    selftest_heavy::weight()
}

#[cfg(test)]
mod tests {
    #[test]
    fn adds() {
        assert_eq!(super::add(2, 2), 4);
    }

    #[cfg(feature = "heavy")]
    #[test]
    fn heavy_is_on() {
        assert_eq!(super::weight(), 7);
    }

    // Exists only in the `no default features` job's build: the proof that job
    // compiles a configuration no other job does.
    #[cfg(not(feature = "heavy"))]
    #[test]
    fn built_without_heavy() {
        assert_eq!(super::add(1, 1), 2);
    }

    #[cfg(feature = "extra")]
    #[test]
    fn extra_is_on() {
        assert_eq!(super::extra(), "extra");
    }
}
