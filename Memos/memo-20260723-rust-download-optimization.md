# Memo: Rust Download/Build-Space Optimization Under the Billet Model

Date: 2026-07-23
Provenance: chat investigation following a JJ disk-space interdictum (85%
threshold, `/` at 85.2%) and a billeting-process failure attributed to
"redownloading a lot of rust packages." Companion prompt (billet sweep facts):
scratchpad `fable-prompt-target-sweep.md`, facts restated here where durable.
Toolchain at time of study: cargo 1.90.0 stable, macOS.

## Corrected model: cargo's three storage layers

The investigation began with the layers conflated; separating them dissolved
most of the problem. Verified on this station, 2026-07-23:

| Layer | Location | Shared across billets? | Lock | GC |
|---|---|---|---|---|
| Download cache (registry src + `.crate`) | `~/.cargo/registry` | **Already shared** — nothing in the tree sets `CARGO_HOME` | Exclusive only during fetch; shared during builds | **Automatic and live**: `~/.cargo/.global-cache` tracking DB present; stable cargo prunes unused files (~1 month extracted src, ~3 months `.crate`) |
| Build intermediates (rlibs, fingerprints, incremental) | each crate's sibling `target/` | No — per billet, per crate | Exclusive for the whole build on stable 1.90 | **None on stable** |
| Final artifacts (`target/release/vvr` etc.) | same `target/` | No | same lock | none |

Key consequences:

- A "shared crate-download location under ~/projects" would recreate what
  `~/.cargo` already provides. All billets share it today; it self-prunes.
- The observed *redownloads* are the download cache's auto-GC reclaiming
  sources from trees cold past the GC horizon (the 8.0 G apck tree was three
  months cold). Self-healing, costs bytes, not the failure class to engineer
  against.
- The real disk consumer is the middle row: per-billet, per-crate `target/`
  trees with no GC and heavy variant duplication (measured on the apck tree:
  330 crates → 802 `.rlib`s; 12 variants of `tauri_utils`; debug profile 6.3 G
  vs release 1.8 G).
- The "globally locked during a build" concern is true of `target/`
  (whole-build exclusive lock on stable 1.90), NOT of the download cache.
  Upstream is dismantling it: the 2025h2 cargo goal splits `build-dir`
  (shareable intermediates) from `artifact-dir` (final binaries) with
  fine-grained locking. Probed here: `CARGO_BUILD_BUILD_DIR` is silently
  ignored on 1.90 stable — the designed fix exists but is not yet reachable
  on this toolchain.

## The vob_clean boundary (settling the "deliberate array" question)

`vob_clean`'s four-crate array (`vok`, `vok/vof`, `vvc`, `jjk/vov_veiled`) is
the VOW pipeline's own build footprint — it cleans exactly what `vob_build`
builds. The four uncovered crates each have a different build owner: `vok/vom`
(matricula, own tabtargets by ruling), `rbk/rbtd` (theurge, rbw workbench),
`rbk/vov_veiled/rbthd` (own CLI), `apck/apcd` (apcw workbench). The array is a
deliberate ownership boundary, not a stale roster. Any per-kit clean verb for
the uncovered crates belongs to the owning workbench, mirroring the vob
pattern.

## Recommendations (mull-list, none executed)

1. **Cadence over structure, for now.** The billet × target multiplier is
   dormant (billets reaped to one); the outlier tree was cold and non-billet.
   Absorb via: (a) reap-billet cadence — a reaped billet's `target/` dies with
   it; (b) a station-scoped cold-target hygiene sweep.

2. **Hygiene sweep semantics.** A sweep answering disk pressure is a station
   concern, distinct from pipeline clean verbs. Its roster is filesystem
   discovery (`find … -name Cargo.toml`, measure sibling `target/`, threshold
   by coldness, plan-then-confirm) — the filesystem is the single home of
   "what target dirs exist and how cold are they." This does not compete with
   `vob_clean`'s deliberate array; different question, different home. Mint no
   central crate registry (`BURC_MANAGED_KITS` is a kit roster, not a crate
   roster).

3. **Do NOT adopt a shared `CARGO_TARGET_DIR`** on this toolchain: whole-build
   exclusive lock serializes concurrent officia, reintroducing the shared
   mutable state billets exist to prevent; every artifact read-back site
   (e.g. `vob_build.sh` reading `target/release/vvr`) would need indirection.

4. **Revisit at toolchain upgrade.** When cargo stabilizes `build.build-dir`,
   adopt shared build-dir + per-billet artifact-dir: warm cuts, single-copy
   intermediates, upstream-designed locking, read-back paths unmoved. That is
   the structural fix worth waiting for. Trigger: next toolchain bump — probe
   `CARGO_BUILD_BUILD_DIR` again.

5. **Optional loudness hygiene, off the critical path:** `CARGO_NET_OFFLINE=true`
   in the build-environment kindle (one chokepoint, inherited by all cargo
   children including `cargo tauri`) plus a fetch verb running
   `cargo fetch --locked` per manifest with the var cleared. Value is
   legibility — a build that unexpectedly needs the network fails loud — not
   speed. Demoted from earlier fix-candidate status once redownloads were
   traced to cache GC.

6. **Held in reserve:** sccache (concurrency-safe shared compile cache) if
   cut-to-warm latency ever hurts before the build-dir split lands; costs
   incremental compilation, shrinks `target/` little.

## Upstream references

- Cargo build-dir layout rework (2025h2 project goal):
  https://rust-lang.github.io/rust-project-goals/2025h2/cargo-build-dir-layout.html
- build-dir lock split: https://github.com/rust-lang/cargo/pull/16708
- artifact-dir/build-dir locking fix: https://github.com/rust-lang/cargo/pull/16385
- Shared TARGET_DIR spurious failures: https://github.com/rust-lang/cargo/issues/14053
- Build cache reference: https://doc.rust-lang.org/cargo/reference/build-cache.html
- Cargo GC design notes: https://hackmd.io/@rust-cargo-team/SJT-p_rL2
