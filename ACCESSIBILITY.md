# Accessibility

What `ikigai-classify` emits that people read, and how to report a barrier. It is not a
conformance claim. Adapted from the
[ACCESSIBILITY.md template](https://github.com/mgifford/ACCESSIBILITY.md) (MIT).

| Field | Value |
|---|---|
| Target | WCAG 2.2 Level AA, for any page that presents this repository's output; it emits none yet |
| Status | Evaluation in progress. No conformance claim. |
| Owner | the maintainers of this repository |
| Report a barrier | [a GitHub issue](https://github.com/ikigai-rs/ikigai-classify/issues/new?labels=accessibility) with the `accessibility` label; anything security-sensitive through [private vulnerability reporting](https://github.com/ikigai-rs/ikigai-classify/security/advisories/new) |
| Last reviewed | 2026-10-31 |

## What it emits that people read

Nothing yet. This repository is a scaffold: `src/lib.rs` holds a crate doc comment and no
code, so it binds no resource and writes no text, page, message or error. This file grows with
the code: each change that adds output people read adds it here, with the rules it follows, how
it is tested, and what is not yet known.

## Contributing

Read this file before adding or changing any output people read: messages, pages, terminal
text. Add that output to this file in the same pull request, keep meaning in words, and say in
the pull request what was tested, with what, and what was not.

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
