# Remaining quality risks — 2026-10-12

1. **No verified full workspace CI at the baseline main SHA.** The new draft PR adds a CI gate but a passing run must be observed before integration.
2. **Open crash reports and platform issues** documented by upstream in `docs/gaps.md` (G2/G3) remain untriaged for this fork.
3. **Script BINARY regression** failed on a separate `test/artcraft-effectcraft-binary-817` branch; isolate fixes in its existing draft PR #1 rather than this baseline branch.
4. **Coverage outside `effectcraft-time`** has not been measured here. Never combine denominator-incompatible language/platform reports.
5. **UI native and browser E2E, fuzzing, mutation testing, media interoperability, GPU and packaging** are not covered by this initial CI change.
6. **Main branch protection** was reported disabled at audit time; manual policy must prevent merges until actual gates are verified.
7. **Dependent upstream Git revisions and native libraries** need reproducible setup; report failures as blockers, not passes.

Status: **PARTIALLY VERIFIED** as an audit, **NOT PRODUCTION READY** as an entire application.
