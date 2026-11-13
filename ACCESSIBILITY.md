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
| Last reviewed | 2026-11-12 |

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

Every named check has a result: Pass, Fail or Blocked, with its evidence. A Pass covers only the
stated check, not overall accessibility. The source reviews below were done on main at `d7a47df`
on 2026-11-12. This repository has no test of how its output reads; `selftest.yml` runs the shared
workflow against `fixtures/selftest` and checks that it behaves as intended.

| Check | Result | Evidence |
|---|---|---|
| The shared workflow behaves as intended | Pass | `selftest.yml` on main at `d7a47df`, 2026-11-03, success. Checks behavior, not how output reads. That run predates CI's move to rustc 1.100.0 (2026-11-09). |
| Every error annotation names what is wrong | Pass | Source review of the four `::error::` lines in `.github/workflows/rust.yml` and the one in `.github/scripts/checkout-credentials.rb`. Two state the fix (add `no-default-features: true`; the package-name rule); the forbidden-package and the member-not-named errors name the problem and its consequence, not a step to take. |
| Meaning not carried by color alone | Pass | Source review: the workflow writes no color of its own; `CARGO_TERM_COLOR: always` colors cargo's lines, each of which starts with its word (`error`, `warning`). |
| Profile page: one top-level heading, link text names its destination | Pass | Source review of `profile/README.md`: one `#` heading, no "here" or "click here" link text. GitHub's rendering of the page was not checked. |
| Job names read as words in every repository's checks list | Fail | A job GitHub skips is listed under its unexpanded name: `msrv ${{ needs.plan.outputs.msrv }}`, `features: ${{ matrix.set }}`, `clippy · test (macOS, ${{ matrix.set }})`, `clippy · test (Windows, ${{ matrix.set }})`. Seen in the checks of ikigai-rs repositories' pull requests on 2026-11-12. The names come from `.github/workflows/rust.yml`. |
| Job-name separator | Fail | Job names use a middle dot (`fmt · clippy · test`); screen readers announce it differently, and some skip it. Source review, not tested. |
| Browser rendering of the profile page and the checks list | Blocked | No browser evaluation was run; GitHub owns both pages' markup. |
| Automated accessibility checker | Blocked | No axe or other checker has been run on the profile page or a run's log view. |
| Manual keyboard | Blocked | No person has navigated the profile page or a checks list from the keyboard for this review. |
| Screen reader | Blocked | No screen reader has been used on the profile page, a checks list or a log. |
| Screen magnifier | Blocked | No screen magnifier has been used on this output. |
| Speech input | Blocked | No speech-input software has been used on this output. |
| Switch access | Blocked | No switch-access software has been used on this output. |
| Braille display | Blocked | No braille display has been used on this output. |

## Known limitations

Found by reading the source on 2026-10-31 and 2026-11-12, not by testing:

- A skipped job is listed under its unexpanded name (`msrv ${{ needs.plan.outputs.msrv }}`), so
  the checks list reads out expression syntax where a word belongs.
- Job names use a middle dot (`fmt · clippy · test`) as a separator. Screen readers announce it
  differently, and some skip it.
- Cargo's colored log lines carry ANSI escape codes. GitHub's log viewer renders them; a raw log
  download keeps them as text.

## Contributing

Read this file before changing anything listed above: the profile page, the security policy, a
job or step name, or a message the workflow writes. Keep meaning in words. In the pull request,
give each check a row in a Check | Result | Evidence table: Pass, Fail or Blocked, with the test or
command, the tool and its version, or why it could not run. Keep an unrun check as Blocked, a screen
reader nobody ran included; never omit it.

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
| 2026-11-12 | "How it is tested" as a Pass / Fail / Blocked table; skipped jobs' unexpanded names recorded as a Fail. |
