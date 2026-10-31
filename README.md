# ikigai-classify

Classifiers as resources for [ikigai](https://github.com/ikigai-rs): one contract,
`urn:iki:classify:{c}:answer`, over interchangeable backends.

A configured classifier takes `content` and a question (a closed set of labels) and answers
in one shape whatever its backend:

- a label, or an abstention;
- the probabilities;
- the classifier's identity and version;
- what the answer cost.

Which backend a classifier uses, a local naive-Bayes model, a classifier service over HTTP,
or a prompted model, is configuration, not code. So a question can change its classifier,
or run a second one in shadow beside it, without changing anything that reads the answer.

**Status:** scaffold. The design is ledger #2690; the module is not built or published yet.

## License

MIT or Apache-2.0, at your option.
