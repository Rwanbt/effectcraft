# Feature-to-test traceability (initial risk inventory)

State values: `VERIFIED` = contract assertions actually run at the referenced commit; `PARTIAL` = some tests run, but not every required layer; `UNVERIFIED` = no run proven; `BLOCKED` = infrastructure cannot execute. All entries below are provisional until the new GitHub Actions SHA-specific run completes.

| ID | Component | Expected contract | Risk | Required evidence | Current state |
|---|---|---|---|---|---|
| TIM-01 | `crates/time` | exact time, SMPTE/NDF/DF boundaries including 119.88 | drift, wrong frames | deterministic boundary tests + OS matrix + LCOV | UNVERIFIED |
| PROJ-01 | `crates/project` | versioned save/reload and migrations | data loss | unit + roundtrip + malformed inputs + E2E | UNVERIFIED |
| ENG-01 | `crates/engine` | command dispatch, undo/redo, atomic failure | corruption | unit, integration and UI E2E | UNVERIFIED |
| SCRIPT-01 | `crates/script` | isolated script I/O, BINARY bytes, permissions | corruption, sandbox escape | boundary tests and roundtrips | PARTIAL (other-branch CI failed) |
| REND-01 | `crates/render`, `crates/effects` | deterministic pixels/time-seeking | fidelity drift | independent oracles, rendering corpus | UNVERIFIED |
| IMPORT-01 | `crates/psd`, `crates/svg`, `crates/lottie` | parse malformed/untrusted files safely | crash, denial of service | corruption corpus + fuzz + valid roundtrips | UNVERIFIED |
| EXP-01 | `crates/export`, codec crates | readable encoded outputs | silent corruption | decode, metadata, external oracle | UNVERIFIED |
| UI-01 | `crates/ui-egui`, desktop app | edit, save, restore, menus and dialogs | lost work | headless and real native UI tests | UNVERIFIED |
| WEB-01 | `apps/effectcraft-web` | browser startup, editing and export | browser regressions | real-browser E2E and WASM | UNVERIFIED |
| CLI-01 | `apps/effectcraft-cli`, `crates/automation` | stable CLI/MCP protocols | API regressions | contract tests, malformed messages | UNVERIFIED |

Do not upgrade a state based on the presence of test files; use the recorded test runner result, assertions, CI SHA and coverage artifact.
