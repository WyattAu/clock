# Threat Model — clock

Reference: STRIDE. Scope: the crate's public API surface. Trust boundary:
(1) bytes/inputs entering public constructors and parsers, (2) concurrent
callers sharing interior state. clock is an in-process library — it opens
no sockets and inherits the embedding process's trust domain.

Purpose: Injectable time source (`chronoshift`) — `Clock` trait, `SystemClock`, `MockClock`

## Assets

| ID | Asset | Exposed via |
|----|-------|-------------|
| A1 | determinism of time in tests | hostile input, concurrent callers |
| A2 | monotonicity of production time | hostile input, concurrent callers |

## STRIDE Analysis

| # | Threat | Category | Surface | Mitigation | Residual risk |
|---|--------|----------|---------|------------|---------------|
| T1 | Test nondeterminism from real time | Spoofing | `test wiring` | MockClock advances only by explicit calls | documented |
| T2 | Backwards time on the system clock | Tampering | `SystemClock` | documented: wall-clock source, not monotonic guarantee; consumers needing monotonic use monotonic APIs | documented |

## Repudiation

The crate keeps no audit trail; attribution of calls to callers is out of
scope for an in-process library.

## Out of Scope

- Network transport security (the crate never opens sockets).
- Storage-host compromise: an attacker who controls the host can bypass all
  in-process mitigations.
- Denial of service via resource exhaustion of the host process beyond the
  bounds enforced above.

Reviewed: 2026-09-11
