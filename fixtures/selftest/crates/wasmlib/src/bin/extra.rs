//! Built only under `--features extra` (see `required-features` in Cargo.toml).
//! If the `features: extra` job stops linting this file, nothing else will.

fn main() {
    println!("{}", selftest_wasmlib::extra());
}
