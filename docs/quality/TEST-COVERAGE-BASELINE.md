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

## First measured baseline (foundation only)

At commit `ba572bca122164e86218de02793f97009a7157ef`, [GitHub Actions run 38268599638](https://github.com/Rwanbt/effectcraft/actions/runs/38268599638) completed the `effectcraft-time` foundation jobs on Linux, Windows and macOS. The Linux coverage job uploaded artifact `effectcraft-time-coverage` (id `11755318561`):

| LLVM metric | Covered | Total | Measured percentage |
|---|---:|---:|---:|
| Lines | 412 | 489 | 84.25% |
| Functions | 58 | 81 | 71.60% |
| Regions | 970 | 1106 | 87.70% |
| Branches | not instrumented | not instrumented | unavailable |

This is **not** an entire-workspace baseline. The full `xtask ci` job on that SHA was still running when later commits were pushed; a canceled run does not count as success. Tests later than `ba572bca`, including the `TimeRange::overlaps` regression, need their own exact-SHA results.

## Regression captured before the fix

At `1955d2a9227a790cdf6b7f24e1ef92f234b5e493`, [GitHub Actions run 38269812511](https://github.com/Rwanbt/effectcraft/actions/runs/38269812511) showed `empty_and_reversed_ranges_never_overlap_a_nonempty_range` **FAILED** while the ordinary half-open overlap test passed. Root cause: `TimeRange::overlaps` did not check `is_empty()` for either operand. This branch adds a guard and preserves the failing case as a regression test; do not mark fixed until the post-fix SHA is green.

## After interval-regression fix

At `7e8c4fdf2fa83d695205f1184dcf29439b9ed351`, [run 38270235028](https://github.com/Rwanbt/effectcraft/actions/runs/38270235028) reported the foundation suite **green** on Linux, Windows and macOS (19 unit + 2 SMPTE + 2 interval + 3 pre-existing integration = **26 tests on each platform**). LLVM artifact `11756094398` measures:

| LLVM metric | Covered | Total | Measured percentage |
|---|---:|---:|---:|
| Lines | 442 | 489 | **90.39%** |
| Functions | 68 | 81 | **83.95%** |
| Regions | 1021 | 1108 | **92.15%** |

Gains against `ba572bca`: +30 covered lines and +10 covered functions from testing TimeRange contracts; denominators differ slightly for regions after production-code changes. These figures are **module-level only**. This document's new display and Tick helper tests need the next CI/coverage run; do not count them in this measured baseline.
