# EffectCraft test coverage baseline

- Repository: `Rwanbt/effectcraft` (fork only)
- Audited `main`: `9a7d3e76991d5372448033b473dc12c2480a6a04`
- Date: 2026-10-12
- Inventory: 32 Rust crates under `crates/`, 3 apps under `apps/`, plus `xtask`.
- Initial instrumented coverage: **not measured**. Do not infer percentages from test file counts or upstream documentation.
- Main-branch `cargo test --workspace`: **not yet run/verified** in this audit.
- Pre-existing external branch failure: [Script regression run 38187956556](https://github.com/Rwanbt/effectcraft/actions/runs/38187956556), commit `d8772b6c20782bcf3026d9c0097c597fcdd40298`, branch `test/artcraft-effectcraft-binary-817`: 50 passed, 1 failed (`binary_file_encoding_roundtrips_invalid_utf8_bytes`). **Not a result for main or this branch.**
- Separate internal draft [PR #1](https://github.com/Rwanbt/effectcraft/pull/1) addresses BINARY file encoding; avoid conflicting changes in this baseline PR.

## Evidence collection

The new `Rust quality and deterministic coverage` workflow collects an LCOV file and JSON coverage report for `effectcraft-time` on Linux, with an artifact retained by GitHub Actions. Numbers become authoritative only after a successful run at the **exact branch SHA**. The full workspace gate executes `xtask ci` (format, release Clippy, release tests, dependency layering, asset attribution and WASM checks). The foundational test suite also runs on Linux, Windows and macOS.

## Exclusions and limitations

No coverage exclusions have been applied or claimed. Coverage of other crates, native UI automation, browser E2E, packaging, mutation testing, fuzzing, performance, GPU device-specific paths and multiplatform full-workspace builds is **UNVERIFIED** until independently executed. Rust's `cargo llvm-cov` reports lines/regions/functions; branch availability depends on instrumentation support. The baseline must not be reported as 100%.
