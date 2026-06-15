# Moorings-Basename Tinder Relocation — Design Record

Date: 2026-06-15

Status: **ANALYZED, execution deferred for operator review.** Records the
investigation behind pace ₢BeAAK (heat ₣Be, rbk-09-better-negatives). No code
landed; the one in-flight edit was reverted so the tree stayed clean. The pace
docket was redocketed against this memo the same day.

---

## What surfaced it

`bubc_constants.sh` declares itself "Source-time literal constants … no kindle
dependency — available immediately upon sourcing." One member breaks that
promise:

```
BUBC_moorings_dir="${BURD_CONFIG_DIR##*/}"
```

`BURD_CONFIG_DIR` is a runtime (dispatch-provided) variable, so this is a
**runtime expansion in tinder position** — the exact anti-pattern BCG names in
its tinder-vs-kindle section. It works today only because `BURD_CONFIG_DIR` is
exported and `bubc` is re-sourced per process. The heat's own regime-poison
seam fix widened `bubc`'s sourcing to non-dispatch contexts (e.g. test
fixtures via `buv_validation.sh`) where `BURD_CONFIG_DIR` is unset; nothing
breaks there only because the basename is unused. The latent trap is now
broader.

## The topology (so the fix isn't mis-modelled)

The moorings directory name lives in **exactly one place**: `z_moorings_dir =
"rbmm_moorings"` in `tt/z-launcher.sh` — one trampoline, not one per launcher.
z-launcher joins it into `BURD_CONFIG_DIR` (absolute) and exports it; every
launcher stub and downstream consumer **inherits** it through the exec
environment. The launcher stubs are 3-line delegators — they set nothing,
name nothing.

Consequences worth stating plainly, because they're easy to get wrong:

- Cutting `BUBC_moorings_dir` does **not** create per-launcher copies of
  `BURD_CONFIG_DIR` and imposes **no** manual synchronization. The name has one
  home regardless.
- `BUBC_moorings_dir` is **not** a deduplication mechanism. It is a derived
  display convenience riding on top of the already-single-sourced absolute
  path. The `##*/` is *derivation*, not *duplication* — so the current design
  is already DRY (one literal, derived forms). There is no DRY violation to
  eliminate.
- The only place the name appears more than once is **across separate consumer
  projects** — each project's own z-launcher names its own moorings dir
  (`.buk`, `rbmm_moorings`, …). Those values are *meant* to differ; the shared
  kit needs none of them because it consumes `BURD_CONFIG_DIR`. That is the
  decoupling seam, documented thrice (z-launcher, bubc, bul_launcher comments).

## Consumer survey: it's a display idiom, not load-bearing state

All ~9 consumers use the basename for one purpose — rendering a
**repo-root-relative path in operator-facing text**: "Available investitures
under `rbmm_moorings/rbmn_nodes/`", "Author one at `rbmm_moorings/…/burn.env`",
and enroll-description help. The relative form is friendly precisely because
z-launcher leaves the operator *at* repo root, so the path is directly
actionable.

Even `buq_qualify.sh` (the one site that looked like a real path) tests against
the **absolute** form — `test -f "${z_project_root}/${z_launcher_path}"` — and
uses the basename only to print a friendly relative "launcher not found"
message. So the basename is cosmetic-but-valuable; it is not dispatch state.

## Alternatives considered and rejected

1. **Export the basename from z-launcher** ("more DRY"). z-launcher holds the
   literal, so `export BURD_MOORINGS_DIR="${z_moorings_dir}"` would remove the
   `##*/` entirely. **Rejected.** It isn't actually more DRY (the name already
   has one home; `##*/` is derivation). And it widens the project↔kit contract
   from one variable to two, pushing a standing obligation onto every
   consumer's hand-authored z-launcher — cutting against the documented
   "consume the path, not the name" decoupling. Trades a cheap local derivation
   for a cross-repo contract.

2. **Delete the basename, show absolute paths.** Every site already holds
   `BURD_CONFIG_DIR`; messages switch to the absolute form. Maximal removal,
   restores purity by *deletion*. **Rejected.** Degrades operator-onboarding
   messages (the relative, copy-pasteable-from-cwd form has real value) for
   marginal code savings. This was tempting mainly because it *felt* like
   removing rather than relocating — not because it's the better outcome.

## Chosen plan: relocate to `BURD_MOORINGS_DIR`

The basename is derived from a `BURD_` dispatch value, so it **is** a `BURD_`
value and should live as one — exactly mirroring `BURD_CONFIG_DIR`'s own
lifecycle (born in bootstrap, enrolled in the kindle).

- **bul_launcher.sh** — `export BURD_MOORINGS_DIR="${BURD_CONFIG_DIR##*/}"`,
  beside the existing `export BURD_REGIME_FILE="${BURD_CONFIG_DIR}/burc.env"`
  (identical shape, one line away). Bootstrap is the correct home, not
  `zburd_kindle` — see the timing gotcha below.
- **bubc_constants.sh** — delete the impostor line + comment; the `lower_snake`
  subdir literals stay (genuinely tinder).
- **burd_regime.sh** — enroll `BURD_MOORINGS_DIR` (sibling to
  `BURD_CONFIG_DIR`); drop the now-dead `source bubc` in `zburd_kindle` (no
  `BURD_` enroll description references a `BUBC_` const after the rename).
- **bud_dispatch.sh** — add `BURD_MOORINGS_DIR` to the `z_known` allowlist.
- Rename ~9 consumers `BUBC_moorings_dir → BURD_MOORINGS_DIR`; update the
  `RBS0-SpecTop.adoc` cross-ref comment.

### Gotchas this investigation surfaced (the reusable part)

- **Naming.** BCG: tinder = `lower_snake` pure literals; kindle / runtime-
  derived = `SCREAMING`. The `lower_snake` name was itself the tell that the
  member was misfiled. The relocated constant must be `BURD_MOORINGS_DIR`, not
  `BURD_moorings_dir`.
- **Two scope guards, not one.** A `BURD_` variable must satisfy *both*
  `bud_dispatch.sh`'s hardcoded `z_known` allowlist (`compgen -v BURD_` → dies
  on any unlisted one) *and* `zburd_kindle`'s `buv_scope_sentinel BURD BURD_`
  (via enrollment). An exported-but-unregistered `BURD_` var dies at dispatch.
- **Bootstrap vs dispatch timing.** The home must be bootstrap
  (`bul_launcher`), not `zburd_kindle`: two consumers run *before* the BURD
  kindle — `bul_launcher`'s SETUP-NEEDED block, and `burs_regime`'s enroll
  description (built in `zburs_kindle`, called from `bul_launcher` during
  bootstrap). A dispatch-phase home would leave those empty.

## Scope boundary

The hoped-for prize — "heavily reduce string processing / gluing for something
extremely crisp" — is **not reachable in this neighborhood.** The path-gluing
is load-bearing display work; the only removable thing was the basename
round-trip, and removing it still leaves per-site `${dir}/${subdir}/`
composition. Rethinking how the kit computes and passes config paths kit-wide
is a **separate design heat**, not this pace.

## Pointers

- BCG `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md` — "Tinder vs kindle
  constant" (the violation + the `lower_snake`/`SCREAMING` naming rule),
  "Dispatch-Provided Directory Variables".
- `tt/z-launcher.sh` (sole moorings-name literal), `Tools/buk/bul_launcher.sh`
  (bootstrap, `BURD_REGIME_FILE` precedent), `Tools/buk/burd_regime.sh`
  (`zburd_kindle` enroll), `Tools/buk/bud_dispatch.sh` (the `z_known` guard).
