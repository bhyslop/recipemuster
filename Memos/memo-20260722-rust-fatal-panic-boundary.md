# Rust `fatal` → Panic-Through-Boundary Memo

## Status
Draft v0.1 — Design capture (2026-07-22). Job Jockey was unavailable at authoring
time; this memo is the holding form for the design so it can be decomposed into
paces (and RCG edits) once JJK returns. Provenance, not authority: the durable
rules land in RCG and — where invariant semantics need formal statement — a spec,
not here.

## Purpose
Record a proposed change to how Rust code in this repo terminates on
unrecoverable conditions: convert the `fatal` severity from `process::exit` to a
`panic` that unwinds to a **required, constrained boundary** at (or near) each
app's `main`, plus the surrounding RCG guidance about `assert!` vs `fatal_*`. The
memo captures the reasoning, the code-grounded evidence gathered on 2026-07-22,
the constraints that make the scheme sound, the open questions, and a suggested
pace decomposition.

## Origin
The thread began with a narrower question: does Rust's `assert!` carry C's
`NDEBUG`-gated "disabled in production" connotation? It does not — `assert!`/
`assert_eq!`/`assert_ne!` are always compiled in, including `--release`;
`debug_assert!` is the release-gated variant. RCG currently says nothing about
`assert!` vs `debug_assert!` and has no panic/invariant policy at all (the only
`debug_assertions` mention, RCG line ~627, governs *trace* gating, not asserts).

That surfaced a candidate RCG rule — "ban `assert!` in non-test code in favor of
`fatal_*`" — which in turn surfaced a better idea: rather than police the boundary
between two termination mechanisms (`assert!`/panic = unwind; `fatal_*` =
`process::exit`), **unify them** by making `fatal` panic and installing one
controlled boundary. This memo is about that unification.

## Current state (as-built, 2026-07-22)

Facts gathered by direct inspection; file:line anchors are entry points, not
guaranteed stable line numbers.

- **No crate forces `panic = "abort"`.** Grep of every `Cargo.toml` under
  `Tools/` found no `panic =` setting — unwind is the default everywhere, so
  `catch_unwind` is viable. (This is a load-bearing precondition that must be
  *pinned*; see Constraints.)
- **`fatal` is centralized and small.** Only 6 `process::exit` call sites exist
  across the whole Rust tree. APCK's `fatal` is a flat `std::process::exit(1)` in
  `apcrl_log.rs` (~line 92) — no exit-code fidelity today.
- **Assert usage is overwhelmingly test code.** ~11,200 assert-family calls;
  ~9,927 are `assert_eq!` — a test suite, not production invariant-checking. Only
  3 `debug_assert!` exist in the entire tree.
- **One `catch_unwind` already exists**, in `jjrfr_farrier.rs` (~line 398),
  inside a `Drop` impl. It is the template the scheme generalizes (see Evidence).
- **An inconsistency already present:** `vorm_main.rs` (~line 280) uses a raw
  `panic!` for a startup invariant while APCK's `fatal` exits. Unification cleans
  this up.

## The proposal

1. **`fatal_*` panics instead of exiting.** The macro still emits the canonical
   `[FATAL] [file:line] message` line through the output module *first*, then
   `panic`s (rather than `process::exit`).
2. **Every Rust app installs a required boundary** at/near `main` that catches the
   unwind, and — for CLI/one-shot apps — maps it to the process exit code and
   exits. For GUI/FFI apps the boundary is *per-handler* (see Constraints).
3. **`fatal` behaves uniformly** — it always panics. It is NOT `cfg!(test)`-
   branched to exit-in-prod / panic-in-test. Uniform panic means test and
   production share the identical mechanism up to the boundary; only the boundary
   differs (production installs one; the test harness *is* one). A cfg-branched
   `fatal` would make tests exercise a different code path than ships — testing a
   fiction — and is explicitly rejected.

### What the conversion buys (verified, not speculative)

- **Fatal paths become testable.** With `process::exit`, a fatal path is
  literally untestable — it kills the test runner. As a panic it is caught by the
  harness, so code-under-test can call `fatal_*` and be exercised via
  `#[should_panic]` / `catch_unwind`. This also dissolves the ugliest carve-out
  from the earlier "ban assert" idea: the RCG rule can become "invariant checks
  use `fatal_*` everywhere; test *assertions* stay `assert_eq!` for the harness's
  value-diffs and `#[should_panic]` ergonomics" — no "non-test" hedge needed.
- **Destructors run on the way up.** `process::exit` runs zero `Drop` impls;
  unwinding runs them. This is a real correctness win in JJK (see Evidence: the
  RAII commit lock). It does NOT help resources that lack a destructor (see the
  temp-file caveat) — do not oversell it.
- **One exit-code mapping**, in one place, itself testable.
- **Backtraces** (`RUST_BACKTRACE`) that `process::exit` can never provide.

### What the conversion must NOT buy

`fatal` stays **terminal**. Converting to a panic must not be quietly reread as
"servers now survive fatals." A `fatal` declared process state untrustworthy;
catching the unwind and *continuing* is the classic `catch_unwind`-and-resume
hazard (resuming atop broken invariants). Therefore:

- The boundary's post-catch action is **emit final diagnostic → exit** (with
  cleanup having run on the way up). It never resumes the main flow.
- **Per-request recoverable failure is `Result`/`error`, and must never be spelled
  `fatal`.** If a long-running server (the MCP servers `jjx`/`vvx`) should survive
  a bad request, that is an argument for making *those* paths `error`-typed —
  orthogonal to this conversion, which must not smuggle it in.

## Constraints (make-or-break; each becomes an RCG rule)

1. **`panic = "unwind"` required; no profile may set `abort`.** One `abort` in any
   release profile silently degrades the whole scheme to hard-kill (catch_unwind
   never runs) with no warning. Currently satisfied — must be pinned and asserted,
   ideally by a check, not just prose.
2. **No `fatal_*`/panic may unwind across an FFI frame** — that is UB (or an
   abort). This bites in APCK/Tauri (see Evidence). The "one catch near main"
   model is *false* for FFI/GUI apps: the boundary must be at each `#[tauri::command]`
   handler (already Result-typed there) plus startup-before-`.run()`. `fatal` is
   safe only at startup or in CLI binaries; on the event-loop thread, use `Result`.
3. **No `fatal_*` or panic in `Drop` without the catch template.** A panic
   escaping a destructor *during* an unwind is a double-panic → immediate `abort`,
   bypassing the boundary. Every panic-capable `Drop` must wrap its fallible work
   in `catch_unwind(AssertUnwindSafe(...))` — the pattern `jjrfr_farrier.rs`
   already uses. An audit of all `Drop` impls is part of the work.
4. **Spawned threads do not propagate `fatal` to the app boundary.** A panic in a
   `std::thread::spawn` closure dies in that thread and is caught at the thread
   boundary by the runtime — the process keeps running. So "fatal terminates the
   process" silently fails inside spawned threads (APCK `copy_anonymized` spawns
   one). Threads that may fatal need their own in-thread boundary (join-and-
   propagate, or explicit handling). RCG must state this.
5. **Exit-code fidelity — decide, don't default.** Today APCK's fatal is flat
   `exit(1)`. If flat is acceptable, keep it and build no machinery (load-bearing
   test — no unearned complexity). If distinct fatal codes ever matter (the repo
   has precision-exit-code discipline elsewhere — BUBC bands), the macro must
   `panic` with a *structured payload* carrying the code, and the boundary
   downcasts it. Bonus of the structured form: the boundary can distinguish "our
   fatal" (payload → policy exit code) from "a genuine bug / stray `assert!`"
   (string payload → a distinct internal-error code) — strictly better diagnostics
   than today.

The observable contract (process ends, nonzero) is preserved, so the name
`fatal` still holds — no rename.

## Evidence appendix

### JJK: the leak is real, RAII-based, and already a known failure mode

- The commit lock is a Drop-based RAII guard, `jjrfr_LockGuard`
  (`Tools/jjk/vov_veiled/src/jjrfr_farrier.rs:355`). Release runs in `Drop::drop`
  (~line 390) via `jjrfr_pluck`. The doc comment states the design (~line 350):
  *"object lifetime is lock lifetime … a die mid-hold leaves a stale lock still
  flying its guidon, recoverable only by the break sequence."*
- Therefore `process::exit` while holding the lock **strands it** — `Drop` never
  runs. This is not hypothetical: the stranded state is user-facing
  (`jjrdm_muck.rs:109` — *"Another station holds the studbook lock. If it crashed,
  the lock is stranded"*) and has a dedicated recovery verb `jjrfr_break`
  (`jjrfr_farrier.rs:412`). **Converting `fatal`→unwind lets `Drop` release the
  lock on a fatal instead of stranding it.** This is the single strongest
  argument for the conversion, and it is verified, not inferred.
- **Caveat — the atomic-write temp file is NOT RAII.** `jjri_io.rs:343` builds a
  raw `.tmp.{pid}.json` path; cleanup is manual and only on the validation-failure
  branch (~line 360). A fatal between `fs::write` (~346) and `fs::rename` (~354)
  orphans it whether you exit *or* unwind — there is no destructor for unwind to
  run. It is a benign pid-scoped turd, not a correctness bug. The Drop-cleanup
  argument covers the lock, not the temp; keep the claim honest.
- **The double-panic rule is already load-bearing.** `Drop for jjrfr_LockGuard`
  (~line 398) already wraps its release in
  `catch_unwind(AssertUnwindSafe(...))`, with a comment: *"a panic escaping a
  destructor during an unwind aborts the process."* This is both proof the
  codebase understands the hazard and the exact template Constraint 3 generalizes.

### APCK: the FFI risk is real but localized — and existing code already dodges it

- **objc2 NSPasteboard FFI (`apcrb_pasteboard.rs`) is low-risk.** Leaf calls
  (Rust→ObjC→return: `generalPasteboard` ~66, `dataForType` ~96); no ObjC frame
  calls back into Rust, so a Rust panic never unwinds *through* an ObjC frame. It
  already returns `Result`/`Err(String)` (~100, ~145), never fatals. Not the
  concern.
- **Tauri is the real hazard — but the handlers are already Result-typed.**
  `toggle_finding` and `copy_anonymized` (`apcap_main.rs:184`, `:199`) return
  `Result<(), String>` and `?`-propagate every failure. The hazard is introduced
  only if a future `fatal_*` fires inside a handler or the `window.eval` path
  (`:266`) and unwinds into wry/tao's native event loop (Cocoa) = abort/UB. Rule:
  `fatal` banned on the event-loop thread; failures there are `Result`.
- **Spawned-thread wrinkle:** `copy_anonymized` spawns a thread (`:225`); a
  fatal-panic there dies in-thread and never reaches the boundary (Constraint 4).
- **Boundary model for APCK:** Tauri owns the event loop via `.run()`, so the
  boundary is per-command-handler (already Result-typed) plus
  startup-before-`.run()`. Confirms FFI/GUI apps need per-handler discipline;
  CLI binaries (`jjx`, `vvx`, theurge, `apcab`) get the single near-main boundary.

## RCG guidance shape (target of the eventual edit)

Two additions, framed as corollaries of the existing **Output Discipline** section
(a `fatal` that writes a panic message straight to stderr bypasses the single
canonical emission path exactly as a stray `println!` does):

1. **The `fatal`-is-panic mechanism + boundary contract** — `fatal_*` emits then
   panics; every app installs a boundary; the boundary exits (never resumes);
   fatal is terminal; per-request recoverable failure is `Result`, not `fatal`.
2. **The constraint list** — the five Constraints above, each stated as a rule
   with its rationale, plus the `assert!` guidance: `assert!`/`assert_eq!` are for
   test assertions and const/compile-time checks (`fatal_*` cannot run in a const
   context); production invariant checks use `fatal_*`; rule explicitly on
   `debug_assert!` (only 3 in tree — either banned outright, or retained solely for
   provably-redundant-in-release hot-path checks — pick one on purpose).

## Open questions (resolve before or during implementation)

- **Flat vs structured exit code** (Constraint 5) — operator call. Drives whether
  the macro panics with a bare message or a typed payload.
- **`debug_assert!` policy** — ban outright, or retain for the hot-path niche.
- **Do the MCP servers want per-request survival at all?** If yes, that is a
  separate `error`-typing effort; if no, the boundary is purely
  cleanup-then-exit. Either way `fatal` stays terminal.
- **Enforcement of "no `fatal` on the event-loop thread"** — review-only (grep
  gate) vs a structural affordance. APCK is the only current FFI/GUI app.

## Suggested pace decomposition (for when JJK returns)

This is a cross-cutting heat, not a one-pace edit. A plausible cut:

1. **Census `Drop` impls repo-wide** — enumerate every `impl Drop`, flag which do
   fallible work that could panic under a fatal-unwind; the census is the plan for
   Constraint 3. (Two known today: `jjrfr_LockGuard`, and the test grounds
   `JjkTestDir` / `ZjjtvbGround`.)
2. **Pin `panic = "unwind"`** — assert no profile sets `abort`; add a check
   (Constraint 1).
3. **Convert the output modules** — `apcrl_log.rs` (and `vvco_*` / any JJK output
   module) `fatal_*` from `process::exit` to emit-then-panic, with the exit-code
   decision (Q1) baked in.
4. **Install/verify boundaries** — CLI binaries get a near-main catch→exit;
   APCK/Tauri gets the per-handler + startup discipline; audit spawned threads
   (Constraint 4).
5. **RCG section** — write the two-part guidance above once mechanism + boundary
   are settled (frontier-tier: it authors authority downstream code obeys).
6. **Reconcile the stray raw `panic!`/exit sites** — `vorm_main.rs` startup
   invariant and the 6 `process::exit` sites route through the unified path.

## Companion / provenance
Grew out of the 2026-07-22 conversation on `assert!` semantics and RCG output
discipline. RCG is at `Tools/vok/vov_veiled/RCG-RustCodingGuide.md` (Output
Discipline section). No spec home yet for invariant-termination semantics; if the
"fatal is terminal / recoverable is Result" boundary needs to survive as
authority, it wants a spec line, not a memo.
