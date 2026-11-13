# Accessibility

What `ikigai-classify` shows people, how it is checked, and how to report a barrier. It is not a
conformance claim. Adapted from the
[ACCESSIBILITY.md template](https://github.com/mgifford/ACCESSIBILITY.md) (MIT).

| Field | Value |
|---|---|
| Target | WCAG 2.2 Level AA, for this repository's documentation and any future runtime output |
| Status | Evaluation in progress. No conformance claim. |
| Owner | the maintainers of this repository |
| Report a barrier | [a GitHub issue](https://github.com/ikigai-rs/ikigai-classify/issues/new?labels=accessibility) with the `accessibility` label; anything security-sensitive through [private vulnerability reporting](https://github.com/ikigai-rs/ikigai-classify/security/advisories/new) |
| Last reviewed | 2026-11-12 (America/Los_Angeles) |

## What it shows people

- **Documentation:** `README.md`, `AGENTS.md` (also linked as `CLAUDE.md`), this file,
  and the crate documentation in `src/lib.rs`. People read Markdown on GitHub or in an
  editor, and may read generated crate documentation through rustdoc.
- **Runtime output:** none yet. This repository is a scaffold: `src/lib.rs` contains
  crate documentation and `#![forbid(unsafe_code)]`, with no implementation. It binds
  no resource and writes no runtime text, page, message or error.

GitHub, editors and rustdoc supply the rendering and navigation interfaces; their
accessibility has not been evaluated here. License text and package metadata are also
readable repository content; they have not received a separate accessibility review.

## How it is tested

Every named check has a result: Pass, Fail or Blocked, with evidence. Pass covers only
its stated scope, never overall accessibility. The source inspection below ran on
2026-11-12 (America/Los_Angeles) against fetched `origin/main` at
[`6b550eeb979941f3adf6a2f33745298f3be2d126`](https://github.com/ikigai-rs/ikigai-classify/commit/6b550eeb979941f3adf6a2f33745298f3be2d126),
using Git 2.54.0 (Apple Git-157). It is a current source review, not a browser or
assistive-technology evaluation. CI does not repeat this manual inspection.

| Check | Result | Evidence |
|---|---|---|
| Runtime output inventory | Pass | `git ls-tree -r --name-only origin/main`, `git show origin/main:src/lib.rs` and `git show origin/main:Cargo.toml`, Git 2.54.0, 2026-11-12. Reviewed the tracked inventory, sole Rust source and manifest: no runtime implementation, binary or repository web interface. Does not test dependencies or generated documentation accessibility. |
| Documentation source structure | Pass | `git show origin/main:README.md`, `git show origin/main:AGENTS.md` and `git show origin/main:ACCESSIBILITY.md`, Git 2.54.0, 2026-11-12. Manual source review found Markdown headings, lists, descriptive link text and written reporting instructions; status is expressed in words. Does not verify rendered semantics, contrast, navigation or comprehension. |
| Rendered documentation: headings, links and table semantics | Blocked | No rendered GitHub, editor or rustdoc evaluation was run in this update; source inspection cannot establish the accessibility tree. |
| Browser zoom, reflow and contrast | Blocked | No browser measurements were run on rendered documentation; no browser evaluation session was available for this update. |
| Manual keyboard navigation and focus | Blocked | No person performed a keyboard pass through rendered documentation; no interactive browser evaluation session was available. |
| Automated accessibility checker (axe or equivalent) | Blocked | No rendered-documentation checker was configured or run. Rust build, lint, test and documentation CI gates do not evaluate accessibility. |
| Screen reader | Blocked | No screen reader was used on this repository's documentation; no screen reader evaluation session was available. |
| Screen magnifier | Blocked | No magnifier was used on this repository's documentation; no magnifier evaluation session was available. |
| Speech input | Blocked | No speech-input software was used on this repository's documentation; no speech-input evaluation session was available. |
| Switch access | Blocked | No switch-access software was used on this repository's documentation; no switch-access evaluation session was available. |
| Braille display | Blocked | No braille display was used on this repository's documentation; no braille display was available for this update. |

No historical browser or assistive-technology run is recorded in this repository's
previous accessibility record. A historical run that CI does not repeat must be listed
as Blocked with its date and limits, rather than carried forward as Pass.

## Contributing

Read this file before adding or changing any output people read: messages, pages, terminal
text. Add that output to this file in the same pull request and keep meaning in words.
In the pull request's accessibility section, give each check a row in a
`Check | Result | Evidence` table: Pass, Fail or Blocked, with the test or command,
tool and version used, or why it could not run. Include what each check covers and
what it does not. Keep unrun checks Blocked, including assistive technology nobody used.

## Reporting a barrier

[Open an issue](https://github.com/ikigai-rs/ikigai-classify/issues/new?labels=accessibility)
with the `accessibility` label (the link sets it when GitHub lets you add labels; otherwise a
maintainer adds it): what you were doing, what happened, and your tools and any assistive
technology, if you are comfortable sharing them. Anything security-sensitive goes through
[private vulnerability reporting](https://github.com/ikigai-rs/ikigai-classify/security/advisories/new).
**We acknowledge a report within 7 days**, and promise nothing further.

## Change history

| Date | Change |
|---|---|
| 2026-10-31 | First version. |
| 2026-11-12 | Record documentation and runtime scope, current source checks, and blocked rendered and assistive-technology checks. |
