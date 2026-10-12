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
