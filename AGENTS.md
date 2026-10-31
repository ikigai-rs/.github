# AGENTS.md

This is the ikigai-rs organization's `.github` repository: the organization profile page, the
default security policy, and the shared Rust CI workflow (`rust.yml`) that every Rust repository
calls.

- Read `ACCESSIBILITY.md` before any change to output people read: the profile page, the security
  policy, job and step names, and the messages the workflow writes.
- A change to `rust.yml` reaches every repository that calls it on their next run; `selftest.yml`
  checks it against `fixtures/selftest`.
- Security reports go through private vulnerability reporting, never a public issue.
- American spelling.
