# Pinning-Boundary Gap: Vessel-less Captures (wsl/podvm) Ride a Floating Builder

Date: 2026-07-19
Author: Claude (Fable 5 investigation, Opus 4.8 write-up), with Brad
Status: Findings recorded. Repair slated in ₣B2 (rbk-15-r02-small-cleanups). The
repair approach is deliberately left open — see "Candidate repair shapes" below.

## One-line

The `rbsk_pinning_boundary` premise in RBS0 claims, in the present indicative, that
the sealed captures bole/wsl/podvm all resolve a *pinned* `gcrane`. In fact wsl and
podvm ride the *floating* (mutable-tag) bootstrap builder. This is a known, tracked
gap — RBSCB and the retired capture heat both record it correctly — so it is an
intra-spec inconsistency (RBS0 premise vs. RBSCB vs. code), not a code that betrays
its spec. The deeper defect is that the premise miscategorizes wsl/podvm as
sealed-reliquary captures when they are structurally vessel-less, and that
miscategorization is *why* the false "pinned" claim was written.

## What the code actually does

The capture-tool builder is `gcrane`, and the tool-image it runs is resolved one of
two ways: PINNED (from the project-controlled reliquary cohort, `z_rbfc_tool_gcrane`,
set by `zrbfc_resolve_tool_images` in `rbfca_assembly.sh`) or FLOATING (a mutable
Google-hosted tag, `ZRBLD_GCRANE_BUILDER = gcr.io/go-containerregistry/gcrane:debug`,
defined in `rbld0_lode.sh`).

| Capture kind | Resolution | Site |
|---|---|---|
| conclave | FLOATING (both steps) | `rbldr_reliquary.sh` recipe rows, `ZRBLD_GCRANE_BUILDER` |
| bole     | PINNED (both steps)   | `rbldb_bole.sh` recipe rows, `z_rbfc_tool_gcrane` |
| wsl (underpin) | FLOATING (wrap + vouch steps) | `rbldw_underpin.sh` recipe rows |
| podvm (immure) | FLOATING (cp + vouch steps; select on gcloud, residency on docker — all mutable tags) | `rbldv_immure.sh` recipe rows |

The builder constants in `rbld0_lode.sh` are all mutable tags:
`gcr.io/cloud-builders/docker`, `gcr.io/go-containerregistry/gcrane:debug`,
`gcr.io/cloud-builders/gcloud:latest`. The only digest-pinned bootstrap in the system
is `ZRBFC_DELETE_BUILDER` (`rbfc0_core.sh`), pinned to a `@sha256:` digest with a
dated comment because that step runs under the Director SA (delete authority).

Two precision notes:
- The "pinned" bole resolution is a *tag on the reliquary package* (`:rbi_gcrane`),
  project-controlled and conclave-sealed — not itself a `@sha256:` digest. It is
  pinned in the sense of provenance-controlled, not cryptographically digest-addressed.
- `zrbfc_resolve_tool_images` requires `RBRV_RELIQUARY` and `buc_die`s if it is absent.
  wsl and podvm are vessel-less — they carry no `RBRV_RELIQUARY` — so they *cannot*
  call it. This is the structural proof that they have no reliquary to resolve from
  in the current design.

## The categorization defect (the real insight)

The premise groups wsl/podvm with bole as "sealed-reliquary captures." That is the
root error. bole is vessel-adjacent: it has a reliquary slot and resolves the pinned
cohort. wsl/podvm are vessel-less: no slot, nothing to resolve from. They actually sit
in conclave's tier, not bole's. The code comments at the floating sites say exactly
this ("same tier as conclave/wsl"). Once the premise miscategorized them, the "pinned"
claim followed naturally — and wrongly.

The correct picture is a 2×2, not a binary:

|              | resolves from a reliquary | no reliquary |
|--------------|---------------------------|--------------|
| **pinned**   | bole                      | delete builder (digest-pinned by hand) |
| **floating** | —                         | conclave, wsl, podvm ← *current state* |

The "bootstrap-digest-pin itch" is the move that lifts wsl/podvm from the bottom-right
cell to the top-right — the cell the delete builder already occupies. Nothing forces
them onto a *mutable tag*; they can be digest-pinned by hand exactly as the delete
builder was. This is "not pinned yet," not "can't be pinned."

## Trust grades differ by kind

From the RBSL* specs: conclave, wsl (underpin), and bole carry the
`verified-against-published` grade. podvm (immure) carries `recorded-at-acquisition`
(RBSLI) — trust-on-first-capture, with no upstream publisher signature to verify
against. This makes podvm the weakest of the set: the honesty of its capture step is
its *entire* trust story, since there is no independent signature to fall back on.

## Security exposure (the part the docket deliberately omits)

This section is why the memo exists separately from the docket.

A floating tag can, in principle, be repointed at mutated bytes. If the upstream
builder behind the mutable tag were mutated, the poisoned builder would run inside the
capture step with the Mason SA's ambient GAR credentials, and could tamper with the
artifact being captured (a WSL rootfs / a podman-machine disk that later becomes a
trusted base). Worse, the same step that captures the bytes also writes the vouch
envelope that attests them (`rbgjl02-assemble-push-vouch.sh` runs on the gcrane
builder), so a compromised capture would sign its own certificate of authenticity —
the attestation is self-referential.

Nothing downstream re-checks it. The vouch inventory (`rblv_members` / `rblv_digest`)
is read only by augur (display) and immure's own refresh logic — never by a
consume-time gate. `zrbfc_resolve_tool_images` hands tags to the build with no digest
cross-check against any envelope. The in-pool preflight (`rbsk_in_pool_preflight`)
checks only reachability, not content. SLSA/DSSE covers the made side (conjure/bind/
graft attest arks), not the capture side.

The one real bound: captures run as **Mason, writer-only**. Mason can push new
(poisoned) artifacts but cannot overwrite or delete existing trusted ones — so the
blast radius is "a newly-captured Lode is poisoned," not "existing Lodes rewritten."
This is the same asymmetry the delete-builder memo names
(`Memos/retired/memo-20260610-heat-BH-fable-recommendation-pin-delete-builder.md`):
the identity that executes floating bytes never holds delete authority. That memo
pinned the delete builder first precisely because it broke the asymmetry; these
capture builders don't, which is why they were left for later.

Severity: MODERATE, elevated for podvm (its recorded grade removes the signature
fallback). Not critical, because no sensitive-authority identity executes the floating
bytes.

## Candidate repair shapes

Two shapes, both viable; the choice is the design question the pace exists to settle.

1. **Reliquary-resolution.** Bring wsl/podvm into bole's pattern — give the captures a
   reliquary to resolve `gcrane` (and the gcloud/docker builders) from. bole is the
   existence proof that a capture *can* resolve from a reliquary; the obstacle is that
   wsl/podvm are vessel-less, so this means establishing a reliquary reference that is
   not vessel-bound (a standalone/shared reliquary the capture reads, or making the
   captures vessel-adjacent). This is the operator's leaning and the pace's framing.

2. **Digest-pin in place.** Pin the three floating builder constants
   (`ZRBLD_GCRANE_BUILDER`, `ZRBLD_GCLOUD_BUILDER`, `ZRBLD_GOOGLE_DOCKER_BUILDER`) to
   `@sha256:` digests with dated, reviewed comments and a refresh-on-purpose
   discipline — exactly as `ZRBFC_DELETE_BUILDER` is pinned. This needs no reliquary
   (the delete-of-the-last-reliquary case still works), resolves conclave and
   wsl/podvm together, and is low-staleness-cost because the step needs are frozen.

The retired capture heat (`.claude/jjm/retired/jjh_b260511-r260612-rbk-11-mvp-lode-universal-capture.md`)
recorded the intended end-state as "pinned LATER via the bootstrap-builder digest-pin
itch, NOT a reliquary" — i.e. shape 2. The operator now wants to weigh shape 1. Either
closes the gap; the pace settles which fits.

## Spec/rivet wording defect and proposed correction

`RBr_p7c` (RBS0) says reliquary generation is "the one phase permitted to ride the
floating bootstrap builder." That is false as written: wsl and podvm also ride it and
are not generation. The rivet skips them entirely, jumping from "generation floats" to
"the delete builder is the pinned exception." Yet **four code sites cite `RBr_p7c` to
justify their floating wsl/podvm builders** (`rbldw_underpin.sh`, `rbldv_immure.sh`,
`rbgjl05-underpin-wrap.sh`, `rbgjl08-immure-capture.sh`) — a citation pointing to a
home that does not describe the cited case. That is the concrete spec defect to fix.

Proposed corrected wording (ready for the pace to apply or supersede):

**For `rbsk_pinning_boundary`**, replace the "capture family obeys the same boundary"
sentence with:

> The capture family splits by whether it can resolve from a reliquary at all. Bole is
> vessel-adjacent — it carries a reliquary slot — so it resolves the pinned cohort
> `gcrane`, obeying the boundary as the made side does. Conclave (which builds the
> reliquary and so cannot resolve from it) and the vessel-less substrate captures (wsl,
> podvm — which carry no reliquary slot) ride the floating Google-hosted bootstrap
> builder; their transition to a pinned tool is the accepted bootstrap-digest-pin itch
> (see RBr_p7c), not a settled pinned state.

**For `RBr_p7c`**, add after the generation clause:

> Beyond generation, the vessel-less substrate captures (wsl underpin, podvm immure)
> likewise resolve no reliquary and currently ride the floating bootstrap builder;
> unlike the delete bootstrap they execute under writer-only authority, so their
> pinning is deferred to the bootstrap-digest-pin itch rather than mandated now. A
> reader must not read the current floating tag as the invariant.

If the pace chooses reliquary-resolution (shape 1), the wording should instead assert
the new pinned reality for wsl/podvm rather than record the deferral.

## Evidence trail (entry points)

- Premise + rivet: `rbsk_pinning_boundary`, `RBr_p7c` in `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`
- Capture recipes: `rbldr_reliquary.sh`, `rbldb_bole.sh`, `rbldw_underpin.sh`, `rbldv_immure.sh`
- Builder constants: `rbld0_lode.sh`; delete-builder pin: `rbfc0_core.sh`
- Tool resolution: `zrbfc_resolve_tool_images` in `rbfca_assembly.sh`
- Durable canon (correct framing): `Tools/rbk/vov_veiled/RBSCB-CloudBuildPosture.adoc`
- Trust grades: `Tools/rbk/vov_veiled/RBSLU-lode_underpin.adoc`, `RBSLI-lode_immure.adoc`
- Vouch step: `Tools/rbk/rbgjl/rbgjl02-assemble-push-vouch.sh`
- Intended end-state: `.claude/jjm/retired/jjh_b260511-r260612-rbk-11-mvp-lode-universal-capture.md`
- Delete-builder pin precedent: `Memos/retired/memo-20260610-heat-BH-fable-recommendation-pin-delete-builder.md`
