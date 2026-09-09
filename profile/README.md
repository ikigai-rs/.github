# ikigai

A resource-resolution kernel in Rust. Address information by *identity*, resolve it
through composable spaces, and cache once. Deterministic by default — a language model is
the last resort, never the first.

Everything is a resource named by a URI, resolved through a runtime-free kernel against
bound endpoints with five verbs: Source, Sink, Exists, Delete, Meta. Endpoints describe
themselves, so the catalog is machine-legible and an agent's tool list *is* the manifold
under its capability.

- **Start here: [The ikigai Book](https://ikigai-rs.github.io/ikigai-tutorial/)** — the
  tutorial, from resolution to a kernel behind a socket. Every code block is compiled
  against the published crates.
- [`ikigai-core`](https://github.com/ikigai-rs/ikigai-core) — the kernel: identity,
  representations, resolution, caching, capabilities. Compiles to WebAssembly.
- [`ikigai-cli`](https://github.com/ikigai-rs/ikigai-cli) — the `ikigai` command and host:
  REPL, transports (IPC, QUIC), the MCP projection.
- The vocabulary is published at [ikigai-rs.dev/ns](https://ikigai-rs.dev/ns) as Turtle
  and as a JSON-LD context.

Site: [ikigai-rs.dev](https://ikigai-rs.dev) · Pre-alpha · MIT / Apache-2.0
