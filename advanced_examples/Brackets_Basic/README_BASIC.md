# Balanced Brackets Validator (V-Cycle)

## Requirements
- **REQ-1**: Recognize `()[]{}` only, ignore all other chars.
- **REQ-2**: A string is valid iff brackets are properly matched and nested.
- **REQ-3**: On failure, return earliest error (UnexpectedClosing, MismatchedPair, UnclosedOpenings).
- **REQ-4**: Complexity O(n) time, O(n) space.
- **REQ-5**: API: `fn validate_brackets(&str) -> Result<(), BracketError>`
- **REQ-6**: Deterministic across platforms.

## Design
- Use a `Stack<(expected_closer, open_index)>` to track "promises."
- Push expected closer when encountering an opener.
- On closer: pop and check.
- At end: if leftover, return UnclosedOpenings.

## Implementation
See `src/brackets.rs`.

## Verification
- Unit tests in `tests/brackets_test.rs`
- Integration checker test (`tests/brackets_checker_test.rs`) compares actual output with expected CSVs.

## Validation
- Small and large datasets in `tests/data`.
- Run `cargo test` to confirm.

## Traceability Matrix
| REQ | Design | Test |
|-----|--------|------|
| REQ-1 | Ignore non-brackets | `valid_with_noise` |
| REQ-2 | Stack of expected closers | `valid_simple_pairs`, `nested_deep` |
| REQ-3 | BracketError | `unexpected_closing_reports_index`, `mismatched_pair_reports_expected_and_found`, `unclosed_openings_reports_first_unclosed` |
| REQ-4 | Linear scan | `long_valid_smoke` |
| REQ-5 | API fn | All compile |
| REQ-6 | Pure function, deterministic | All tests |

---
To run:

```bash
cargo test
cargo clippy -- -D warnings
```
