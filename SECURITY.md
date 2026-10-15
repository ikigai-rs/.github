# Security policy

This policy covers every repository in the ikigai-rs organization that does not have its
own `SECURITY.md`.

## Supported versions

ikigai is pre-1.0. For each crate, only the latest published minor release on crates.io
gets security fixes: if 0.4.2 is current, a fix ships as a new 0.4.x release, and 0.3.x and
older get none. Code that is not published to a registry is supported on its `main` branch.

## Reporting a vulnerability

Use GitHub private vulnerability reporting: open the affected repository's **Security** tab
and choose **Report a vulnerability**. If you are not sure which repository is affected,
choose the closest one.

Do not open a public issue, pull request or discussion for a security problem.

Include what you can: the affected crate and version, what an attacker gains, and steps or
a test that reproduce it.

## Scope

In scope:

- A capability bypass: a request that succeeds without the capability (`urn:cap:*`) it
  requires, or a capability an endpoint declares but does not enforce.
- Reads or writes across a tenant or graph boundary the caller was not granted.
- Caller input that crashes a host or exhausts its memory, stack, CPU or disk.
- The HTTP and QUIC doors: authentication, authorization and request handling at the
  network edge.
- The Python and TypeScript peers: a peer that enforces less than the capabilities a
  resource requires.
- Publish and supply-chain integrity: what we release to crates.io, and the release and CI
  workflows that build it, including the shared workflows in this repository.

Out of scope:

- Findings in unpublished branches (anything not on `main` or in a published release).
- Denial of service that requires local root or administrator access.
- Issues in third-party dependencies with no ikigai-specific impact. Report those upstream;
  telling us as well helps.

## What to expect

We aim to acknowledge a report within 7 days.

## Disclosure

We follow coordinated disclosure: we fix the problem, publish the fixed release, then
disclose. We credit reporters who want credit; say in your report how you would like to be
named.
