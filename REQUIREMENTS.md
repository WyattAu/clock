# Requirements — clock

Numbered, testable requirements. Every requirement maps to at least one named
test or doc-comment contract; security-relevant items cite threat-model rows.

Scope: Injectable time source (`chronoshift`) — `Clock` trait, `SystemClock`, `MockClock`

## Functional

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-CS-001 | `MockClock` starts at a set instant and advances only explicitly (or via auto-advance) | MUST |
| REQ-CS-002 | `SystemClock::now_ns` returns monotonic wall time in nanoseconds | MUST |
| REQ-CS-003 | `no_std` builds compile (`std` feature off) using core atomics/critical-section | MUST |

## Security

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-CS-100 | No unsafe code (`#![forbid(unsafe_code)]`) | MUST |

## Observability & API hygiene

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-CS-900 | All fallible public APIs return typed errors; production `unwrap`/`expect` is denied or explicitly justified with an invariant comment | MUST |
| REQ-CS-901 | Public items carry doc comments with runnable examples where practical | SHOULD |
