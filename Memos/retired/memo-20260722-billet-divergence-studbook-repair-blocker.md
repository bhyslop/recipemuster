# Memo: Billet divergence + stale MCP binary blocking the paddock-tenancy repair

**Date:** 2026-07-22
**Author:** Claude Opus 4.8 (officium ☉260722-1007-k59i)
**Heat:** ₣B3 (jjk-04-studbook-conversion)
**Status:** Blocker analysis for operator + Fable to mull the repair; no repair attempted.

---

## One-line problem

The paddock-tenancy repair (studbook cutover follow-up) landed on billet branch
`CAAAU`, but the live MCP `vvx` engine still runs a pre-repair binary, and *this*
billet (`CAAAy`, where the drain pace ₢B3·CAAAy is meant to be mounted) does not
contain the repair — so neither the live paddock reads nor the drain's own source
inputs are in a workable state here.

## Background — what the repair was

On 2026-07-22, an out-of-pace, operator-ruled repair addressed a gap left by the
studbook cutover: the de-insertion sweep (₢B3·CAAAT) emptied the consumer-repo
`.claude/jjm/` home that the paddock subsystem still read. The repair moved the JJ
state tenants into the studbook. Two commits carry it:

- **studbook `e0d4f73`** — migration seat: 34 live paddocks → `paddocks/`, 101
  retire trophies → `retired/`, `jji_itch.md` + `jjz_scar.md` at studbook root.
- **alpha `ffd8afe4d`** — code repoint + spec accord: `jjri_paddock_path` now emits
  studbook-relative `paddocks/jjp_*.md`; the four readers (orient / show / scout /
  paddock-read) take the studbook root from `zjjrm_studbook_root`; writers ride
  tenant files on each mutation's own journal commit. Specs brought accordant:
  JJSVS "Scope at birth" SUPERSEDED note; JJSAS Founding-and-cutover TENANCY
  FOLLOW-UP note; JJS0 / JJSCNO / JJSCRT / JJSCRS / JJSCCU repoints.

The `ffd8afe4d` commit message itself flags the residual:
> "Live orient awaits server swap (close/resume) and disk relief."

## Verification run this session (the four operator-specified checks)

| # | Check | Result |
|---|-------|--------|
| 1 | `jjx_open` from this seat | ✅ Pass — `☉260722-1007-k59i` |
| 2 | `tt/vow-t.Test.sh` → 683/0 green | ✅ Pass — `683 passed; 0 failed` in 18.58s; the 6 prior `jjtpd_parade` failures (85% disk gate) are gone, disk now 77% |
| 3 | Live paddock read `jjx_paddock {B3}` | ❌ Fail — `error reading paddock: No such file or directory` |
| 4 | `jjx_orient` on ₢B3·CAAAy | ❌ Fail — `error reading paddock file '.claude/jjm/jjp_uBl3.md'` |

## Filesystem truth (the migration itself is correct)

- New home present: `/Users/bhyslop/projects/jjqs_studbook/paddocks/jjp_uBl3.md`
  (16563 bytes; all 20 `uBl*` paddocks migrated).
- Old home correctly emptied: `.claude/jjm/jjp_uBl3.md` absent (only
  `chat-archive/` + `jjg_gallops.json` remain under `.claude/jjm/`).
- The step-4 error names the **old** path `.claude/jjm/jjp_uBl3.md` — proof the
  live engine is still the pre-repair binary, reading the consumer-repo home.

## The two axes (the crux — the fix differs per axis)

**Axis 1 — live-orient fix (server swap): merge NOT required.**
The MCP `vvx` server loads its *installed* binary on restart. Per `ffd8afe4d`'s
message, the binary was "rebuilt and installed after the change" (built from
`CAAAU`, where the repair lives). A close/resume should make steps 3–4 pass
regardless of this billet's branch. **Unverified assumption:** that the installed
binary is the CAAAU build and the server does not rebuild from cwd on launch — the
commit message asserts it; it was not independently confirmed from inside the
session.

**Axis 2 — working the drain pace ₢B3·CAAAy in this billet: merge IS required.**
- `ffd8afe4d` and its whole stack live on **`CAAAU` only** — not on `CAAAy` (this
  billet), not on `main`.
- This billet's specs are **pre-repair**: `grep` finds no `TENANCY FOLLOW-UP` note
  in `JJSAS-state-repo.adoc` and no SUPERSEDED tenancy note in
  `JJSVS-studbook.adoc` here. Both exist on `CAAAU` (JJSAS line 167 there).
- The drain pace's explicit charge is to verify the new TENANCY FOLLOW-UP note's
  normative home *before any strike*. That note does not exist in this working
  tree. Draining JJSAS here would operate on stale, pre-repair specs — missing the
  very input the pace exists to process.

## Billet divergence state

`CAAAy` and `CAAAU` have split; neither is on `main`:

| Branch | Unique commits it carries |
|--------|---------------------------|
| `CAAAU` | The full repair stack — `ffd8afe4d` (paddock-tenancy repair) + `688d3a7e1` (JJSV sheaf hardening to normative) + officium commits |
| `CAAAy` (here) | `beacb6ebd` (Rust fatal→panic-through-boundary memo) + this session's officium commit (+ this memo) |

`git log CAAAy..CAAAU` shows the whole repair run; `git log CAAAU..CAAAy` shows
only the Rust memo (+ officium). So a convergence must preserve both sides.

## What the operator/Fable must decide (repair topology — operator territory)

1. **Where does the drain get worked?**
   - (a) Merge `CAAAU`'s repair stack into `CAAAy`, then mount the drain here; or
   - (b) Mount and work the drain in the `CAAAU` billet (it already has every input),
     and reconcile the lone Rust memo separately.
2. **Convergence to `main`.** The repair (`ffd8afe4d` + `688d3a7e1`) has not reached
   `main` yet. The earlier alpha commit `1eda78115` ("Converge the B3 studbook
   cutover to main") converged an *earlier* slice, not this repair. Decide when the
   repair stack + the Rust memo both land on `main`.
3. **Server swap timing.** The live-orient fix (Axis 1) needs a close/resume of the
   Claude session (or MCP restart) irrespective of the merge decision. Sequence it
   against whichever billet becomes the working seat.

## Banked observations (itch-worthy; do not act without direction)

- **Parade hark mode** resolves paddock blobs at historical revisions via the rel
  path; pre-cutover history keeps old `.claude/jjm` paths, studbook-era history the
  new ones — coherent backward, blind forward; silent-optional either way.
- **JJS0 drift:** the Officium Lifecycle / gallops-path prose still names
  `.claude/jjm` homes — cutover drift predating the tenancy ruling, drain-family
  territory, deliberately not touched by the repair.
- **Left unmigrated in alpha history by design:** `.claude/jjm/current/` fossils,
  `jjp_uBl-.md` (shakedown droppings), `jjp_uAlf_catalog.md`.

## What was NOT done

No repair to store machinery, no merge, no branch mutation. Per the operator's
standing instruction ("characterize the exact failure and stop — do not improvise
repairs on the store machinery"), this session stopped at characterization. Fable
will mull the appropriate repair.
