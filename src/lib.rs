//! `ikigai-classify`: classifiers as resources.
//!
//! Scaffold. The module will bind one contract, `urn:iki:classify:{c}:answer`: a configured
//! classifier `{c}` takes `content` and a question (a closed set of labels) and answers in
//! one shape whatever its backend: a label or an abstention, the probabilities, the
//! classifier's identity and version, and the cost. Which backend a classifier uses is
//! configuration, not code. The design is ledger #2690.

#![forbid(unsafe_code)]
