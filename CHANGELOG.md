# Change Log

All user visible changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/), as described
for Rust libraries in [RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)

## [0.2.0] - (pending)

 * Instead of panicing, all methods that could return a `Result<T, grok::Error>`.
 * `Grok::new()` has been renamed to `Grok::empty()` (or `Grok::default()`)
 * `is_empty()` is available in the `Matches` API to check if there are matches at all.
 * `len()` is available in the `Matches` API to get the total number of matches.

## 0.1.0 - 2017-09-05

 * Initial Release

[0.2.0]: https://github.com/daschl/grok/compare/v0.1.0...v0.2.0