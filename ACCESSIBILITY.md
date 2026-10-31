# Accessibility

What this repository emits that people read, the rules that output follows, how it is tested, and
how to report a barrier. This is the organization's `.github` repository: the profile page and the
shared continuous-integration workflow that every Rust repository in the organization calls. It is
not a conformance claim. Adapted from the
[ACCESSIBILITY.md template](https://github.com/mgifford/ACCESSIBILITY.md) (MIT).

| Field | Value |
|---|---|
| Target | WCAG 2.2 Level AA, for any page that presents this output; this repository writes Markdown and plain log text only |
| Status | Evaluation in progress. No conformance claim. |
| Owner | the maintainers of this repository |
| Report a barrier | [a GitHub issue](https://github.com/ikigai-rs/.github/issues/new?labels=accessibility) with the `accessibility` label; anything security-sensitive through [private vulnerability reporting](https://github.com/ikigai-rs/.github/security/advisories/new) |
| Last reviewed | 2026-10-31 |

## What it emits that people read

- **The organization's profile page** (`profile/README.md`): Markdown that GitHub renders on the
  organization's page. GitHub owns that page's markup; this repository owns its words, headings,
  link text and structure.
- **The security policy** (`SECURITY.md`), which GitHub shows for every repository in the
  organization that has none of its own.
- **Continuous-integration output** from the shared workflow (`.github/workflows/rust.yml`) and
  its self-test (`.github/workflows/selftest.yml`): job and step names in each repository's checks
  list (for example `ci / fmt · clippy · test`), the plan step's plain lines (`feature sets: …`,
  `msrv: …`), and `::error::` annotations that GitHub shows on the run and the pull request.
- **The credentials check** (`.github/scripts/checkout-credentials.rb`): one summary line and an
  `::error::` line per finding.

Not this repository's: the output of the tools the workflow runs (cargo, clippy, rustdoc), each
repository's own `ACCESSIBILITY.md` and pages, and GitHub's own interface.

## The rules that output follows

Followed, by reading the source; none is pinned by a test:

- Every error annotation is a sentence that names what is wrong and, where there is one, the fix
  (for example: "no-default-features-forbid is set but no-default-features is not: nothing would
  check it. Add `no-default-features: true`.").
- Meaning is never carried by color alone. The workflow writes no color of its own. It sets
  `CARGO_TERM_COLOR: always`, so cargo's log lines are colored, but each also starts with its word
  (`error`, `warning`), which carries the meaning.
- The profile page uses one top-level heading and descriptive link text (a link names its
  destination, never "here").

## How it is tested

- `selftest.yml` runs the shared workflow against `fixtures/selftest`, which checks that the
  workflow behaves as intended. It does not check how its output reads.
- **No automated accessibility checker, browser or assistive technology has been used on this
  output.**

## Known limitations

Found by reading the source on 2026-10-31, not by testing:

- Job names use a middle dot (`fmt · clippy · test`) as a separator. Screen readers announce it
  differently, and some skip it.
- Cargo's colored log lines carry ANSI escape codes. GitHub's log viewer renders them; a raw log
  download keeps them as text.

## Contributing

Read this file before changing anything listed above: the profile page, the security policy, a
job or step name, or a message the workflow writes. Keep meaning in words. Say in the pull request
what was tested, with what, and what was not.

## Reporting a barrier

[Open an issue](https://github.com/ikigai-rs/.github/issues/new?labels=accessibility) with the
`accessibility` label (the link sets it when GitHub lets you add labels; otherwise a maintainer adds
it): what you were doing, what happened, and your browser and any assistive technology, if you are
comfortable sharing them. Anything security-sensitive goes through
[private vulnerability reporting](https://github.com/ikigai-rs/.github/security/advisories/new).
**We acknowledge a report within 7 days**, and promise nothing further.

## Change history

| Date | Change |
|---|---|
| 2026-10-31 | First version. |
