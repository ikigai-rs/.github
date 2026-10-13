//! Stands in for the heavy dependency a default feature pulls in (an RDF parser, a
//! network stack, a database). `selftest-wasmlib` links it only under its default
//! feature `heavy`, so the `no default features` job's tree check has something real
//! to keep out. NOT a workspace member (see the root manifest's `exclude`): a member
//! is a root of `cargo tree --workspace` whatever features are on.

/// A value only the default build can reach.
pub fn weight() -> u32 {
    7
}
