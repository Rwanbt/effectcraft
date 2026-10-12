# Remaining quality risks — 2026-10-12

1. **No verified full workspace CI at the baseline main SHA.** The new draft PR adds a CI gate but a passing run must be observed before integration.
2. **Open crash reports and platform issues** documented by upstream in `docs/gaps.md` (G2/G3) remain untriaged for this fork.
3. **Script BINARY regression** failed on a separate `test/artcraft-effectcraft-binary-817` branch; isolate fixes in its existing draft PR #1 rather than this baseline branch.
4. **Coverage outside `effectcraft-time`** has not been measured here. Never combine denominator-incompatible language/platform reports.
5. **UI native and browser E2E, fuzzing, mutation testing, media interoperability, GPU and packaging** are not covered by this initial CI change.
6. **Main branch protection** was reported disabled at audit time; manual policy must prevent merges until actual gates are verified.
7. **Dependent upstream Git revisions and native libraries** need reproducible setup; report failures as blockers, not passes.

Status: **PARTIALLY VERIFIED** as an audit, **NOT PRODUCTION READY** as an entire application.

## New regression finding: zero-duration timeline overlap

The comparison `TimeRange::new(Tick(10), Tick(0)).overlaps(&TimeRange::from_bounds(Tick(0), Tick(20)))` unexpectedly yielded `true` before correction. A deterministic test **failed in CI** on SHA `1955d2a9227a790cdf6b7f24e1ef92f234b5e493` (run `38269812511`). The minimal fix checks both ranges for emptiness before comparing endpoints. **Post-fix validation remains required**; do not merge the draft PR until validated.

- `TimeRange::overlaps` is **green after correction** in GitHub Actions on all three OS at `7e8c4fdf2fa83d695205f1184dcf29439b9ed351`, retaining red-before-green evidence.
- New Tick/SMPTE/parser tests must be checked at their own newer SHA before they count as verified coverage.

## Project integrity follow-up (new code awaiting exact-SHA run)

- `Project::comp_contains` previously recursed without tracking visited compositions, allowing a cyclic user project to exhaust the stack. The new iterative visited-set traversal terminates on cyclic graphs while preserving reachability.
- `Project::fix_next_id` previously used unchecked integer increments. Saturating normalization plus a `from_json` rejection of exhausted id space prevents a crash or silent id reuse when opening malformed project files. `Project::alloc` still returns `u64` (no fallible API), so exhaustion from programmatically constructed projects remains outside the scope of this patch and is tracked for design review.
- Integration contracts for cyclic graphs, stale ids, exhausted ids, invalid/future schemas and every prefix truncation are added, pending CI execution and measured `effectcraft-project` LLVM baseline.

## Essential Graphics follow-up (pending SHA-specific CI)

- `EssentialGraphics::resolve` previously stopped after 8 mirror hops even for valid acyclic chains. It now follows the chain until the first nonmirror, detecting repeat control IDs with a visited set and returning `None` for missing targets or cycles.
- New project integration suites independently check mirror lookup, nested groups, control-type coercion, hidden controls, source/link mapping, match IDs, transform/mask/text selector builder defaults and unique UIDs. These do not replace end-to-end Essential Graphics / MOGRT render tests.
- Track hostile nested `EgKind::Group` depth and the recursive `find_mut`/`remove` traversal separately; no unsubstantiated claim that all recursive nesting is safe.

## Model-level Essential Graphics integration contracts (pending CI)

Three new tests exercise `sync_project` across a nested composition, alias/master overrides, read-only `with_overrides`, transitive dependency enumeration, and `offset_ids` preserving template references. These are deterministic model tests; CLI+real-render E2E remains a separate gate.
