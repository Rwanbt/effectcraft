# Test execution and reproducibility

The repo requires Rust >=1.95 and mandates `cargo xtask ci` (see `CONTRIBUTING.md` and `AGENTS.md`).

## Local commands

```sh
cargo fmt --all -- --check
cargo test --locked -p effectcraft-time
cargo llvm-cov --locked -p effectcraft-time --lcov --output-path coverage-time.lcov
cargo llvm-cov report --json --output-path coverage-time.json
cargo run --locked -p xtask -- ci
```

Install `cargo-llvm-cov` and Rust `llvm-tools-preview` before measuring coverage; install `wasm32-unknown-unknown` before `xtask ci`. Linux full checks need ALSA, udev, Wayland, X11 and Mesa development libraries (see workflow).

## GitHub Actions

Workflow: `.github/workflows/rust-quality.yml`. Check jobs and artifacts on the **latest head SHA**; never treat an earlier run as verification for a new commit. Failure must remain visible and the internal PR must remain draft.

## Determinism

New SMPTE drop-frame tests use published counting rules, fixed integer boundaries, exact frame/tick round trips and no random seeds or wall-clock timing. Existing property-based tests remain enabled; newly discovered failing seeds must be retained as regressions.

## Current execution state

As of the baseline commit, these new tests and the new workflow are **not yet executed**. Update this record with exact SHA, run URLs and measured values after GitHub Actions completes.

## Recorded exact-SHA observations (2026-10-12)

- `ba572bca122164e86218de02793f97009a7157ef`: GitHub Actions `38268599638` foundation job **success** on Linux, Windows and macOS; 19 unit + 2 SMPTE contract + 3 prior integration tests = **24 tests** per platform. Linux formatting check succeeded; coverage artifact `11755318561` uploaded and measured.
- `1955d2a9227a790cdf6b7f24e1ef92f234b5e493`: GitHub Actions `38269812511` Linux foundation test **failed by design** on the newly added empty-range overlap regression, establishing red-before-green evidence.
- Any later fix requires a fresh GitHub Actions success at that specific commit; canceled old runs and a previous SHA's coverage cannot validate it.

- `7e8c4fdf2fa83d695205f1184dcf29439b9ed351`: GitHub Actions `38270235028` foundation suite passed on Linux/Windows/macOS, **26 tests per OS**; LLVM coverage `11756094398`: 90.39% lines, 83.95% functions, 92.15% regions in `effectcraft-time`. The full-workspace release gate was still running when recorded.
- Branch `test/**` pushes no longer duplicate the long full-workspace gate; the required gate runs on every PR and main push, plus manual dispatch. This prevents cancellation/resource contention between identical push/PR workloads.

## Project integrity suite

```sh
cargo test --locked -p effectcraft-project
cargo llvm-cov --locked -p effectcraft-project --lcov --output-path coverage-project.lcov
cargo llvm-cov report --json --output-path coverage-project.json
```

The PR Rust quality workflow now runs time and project suites on Linux, macOS and Windows and archives their independent reports. A successful run at the exact commit is mandatory before assigning any measured project coverage figure.
