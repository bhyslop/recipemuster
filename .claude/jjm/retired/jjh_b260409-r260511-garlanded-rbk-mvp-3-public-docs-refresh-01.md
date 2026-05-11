# Heat Trophy: garlanded-rbk-mvp-3-public-docs-refresh-01

**Firemark:** ₣A5
**Created:** 260409
**Retired:** 260511
**Status:** retired

## Paddock

Garlanded at pace 5 — magnificent service

## Character

Architecture and content work for the public docs surface — the page that onboarding's `buh_tlt` links actually land on. Three interlocked concerns:

- **Rewrite `README.md`** as the canonical link-landing surface, harmonizing the current glossary + setup content with a new intro section ported from the internal cosmology spec
- **Move the docs URL to an RBRR field** (`RBRR_PUBLIC_DOCS_URL`) so the value can be updated per-release or per-incorporation without editing source code
- **Retire legacy scaffolding** (`index.html`, `README.consumer.md`, the `README.consumer.md` copy line in prep-release Step 8) that no longer serves any purpose under the new architecture. `CLAUDE.consumer.md` is NOT retired — it is kept as the canonical ship template for Claude Code customers and gets its stale tabtarget references updated in place.

The cognitive posture this work requires: design judgment during the README rewrite (voice, audience, structural choices) but mechanical execution on the field plumbing and retirement tracks. Pace 1 was resolved through live conversation review (officium ☉260409-1007) rather than memo drafting — all open questions have definitive answers baked directly into downstream pace dockets. The production paces split content editing from mechanical edits so each gets its own session space.

"Refresh" in the silks names the goal plainly: the public docs surface is presently stale and disconnected from current Recipe Bottle vocabulary. This heat refreshes the surface to a correct, sustainable shape — not a full release, not a polish pass, just the minimum for the `buh_tlt` links to land somewhere useful and for future iteration to build cleanly.

## Prerequisites

- **₣A3 (rbk-mvp-3-onboarding-guides)** provides `Tools/rbk/rbho_onboarding.sh` with 53 `buh_tlt`/`buh_tl` link calls citing 27 unique anchor targets — exactly matching README.md's glossary table. This heat decides where those links resolve.
- **₣A3 was furloughed (stabled)** at the start of this heat to keep its paddock from being polluted by this side concern; its remaining work (₢A3AAF, rough, notched with a bug fix but not yet wrapped) resumes when this heat retires.
- **`RBGC_PUBLIC_DOCS_URL`** currently lives as a `readonly` constant in `Tools/rbk/rbgc_Constants.sh`, hard-coded to `https://scaleinv.github.io/recipebottle` — a crufty cosmology page with no glossary anchors. Customers clicking glossary terms land on an unrelated page.
- **`rbho_cli.sh`** currently has thin deps (per ₢A3AAJ's intentional design): no RBRR sourcing, no RBRR enforce. Adding an RBRR field for the docs URL means this CLI must grow RBRR sourcing — a conscious relaxation of the thin-deps constraint, justified by the incorporation-time flexibility it enables.

## Context

Spun off from ₢A3AAF (docs-alignment-and-cross-machine-validation) when investigation of an underline-bleed bug in the triage role display surfaced the deeper issue: the `buh_tlt` URLs don't land on a useful page at all. The bug fix itself (adding the `buh_tltT` combinator and reshaping `zrbho_triage_role` to move column padding outside the link envelope) was notched in ₢A3AAF as commit 930ca93f, but A3AAF remains rough pending broader validation. This heat carries the architectural work the investigation revealed, leaving A3AAF for when ₣A3 resumes.

The user is not at general release. The goal of this heat is "refresh the docs surface to a correct shape" — get `README.md` rewritten, get the URL pinned via RBRR, get the legacy scaffolding out of the way, then commit clean to origin. The public-repo PR ceremony is explicitly deferred; this heat ends at a clean commit on `bhyslop/recipemuster`.

## Design Decision Record

### Position 2: release branches for doc mutability

Customer binaries will eventually pin to a release branch (`release/v0.1`-style), not an immutable tag. This permits doc-only fixes to reach customers running an older binary without bumping the binary version. Three positions were considered:

1. **Immutable tags.** Customer running v1.2.3 sees v1.2.3 docs forever. Doc fixes require version bump.
2. **Release branches.** Customer running v1.2.3 sees the latest `release/v1.2` branch state, including doc fixes that landed after their binary shipped.
3. **Always-main.** RBRR holds `main`. No version pinning at all.

Position 2 won as the only option satisfying both "iterate on docs without re-releasing" and "customer should see docs that match their binary version."

For this heat's refresh, the RBRR field defaults to a URL ending in `/blob/main/README.md`. Release branches are deferred until first actual release. The mechanism exists, the value is settable; only the actual release branch creation waits for the moment there's something to pin.

### Whole URL in RBRR, no RBGC constant

The docs URL moves entirely to an RBRR field. No split into base+ref, no proxy constant in RBGC, no composition at kindle time. The field holds a complete, ready-to-use URL.

**Rationale**: reincorporation-time flexibility. When someone pulls Recipe Bottle into their own project via git subtree/subrepo (per the project's stated open-source integration model), they need to point the docs URL at their own fork's README. If the URL were split into RBGC-base + RBRR-ref, they'd have to edit two files; a whole-URL-in-RBRR design lets them edit one `.env` line and be done. Release-time updates (swap `main` for a release branch name) also become a single-line edit.

The proposed shape:

```bash
# In .rbk/rbrr.env
RBRR_PUBLIC_DOCS_URL="https://github.com/scaleinv/recipebottle/blob/main/README.md"

# In Tools/rbk/rbrr_regime.sh (new enrollment in zrbrr_kindle)
buv_string_enroll RBRR_PUBLIC_DOCS_URL 1 512 \
  "Public docs URL — readme target for buh_tlt links, updated per-release or per-incorporation"

# In Tools/rbk/rbho_onboarding.sh (consumer — changes from RBGC to RBRR)
local -r z_docs="${RBRR_PUBLIC_DOCS_URL}"

# In Tools/rbk/rbgc_Constants.sh
# (RBGC_PUBLIC_DOCS_URL line is DELETED — no fallback, no proxy)
```

The current `RBGC_PUBLIC_DOCS_URL` constant is deleted entirely. Two consumer files touch it (`rbgc_Constants.sh` declaration + `rbho_onboarding.sh` reader), and the reader has **six call sites** (lines 320, 341, 442, 564, 786, 904 — most assign to a local `z_docs` once per function). The refactor updates one declaration line and six consumer call sites.

### rbho_cli.sh gains RBRR sourcing

Because `rbho_onboarding.sh` now reads `RBRR_PUBLIC_DOCS_URL`, the onboarding CLI must source `rbrr_regime.sh` and `.rbk/rbrr.env`, then call `zrbrr_kindle` to enroll and lock the schema. It does **not** call `zrbrr_enforce` — that function does filesystem-gate validation (vessel dir, secrets dir existence, GCB timeout regex) which would fail on a fresh install and block onboarding entry.

This relaxes ₢A3AAJ's "thin deps" design for `rbho_cli.sh` by adding a single schema source. The full depot-provisioning validation chain remains out of scope for onboarding.

**Kindle tolerance (resolved in ₢A5AAA)**: GO. `zrbrr_kindle` is schema-only by design — it enrolls variables into the buv validator and reads `RBRR_DNS_SERVER` directly for the `ZRBRR_DOCKER_ENV` array, but does NOT call `buv_vet` (length/format checks) or `buv_dir_exists` (filesystem checks). Those live in `zrbrr_enforce`. Pre-depot-setup state (marshal-zeroed rbrr.env with blanked depot fields but pre-filled DNS/vessel/secrets) succeeds at kindle time. No empirical probe needed — user confirmed this is the established design pattern.

### README.md is the canonical link landing surface

The right page for onboarding-link landing is the glossary table, not a project tour. README.md already has the 27-row glossary with `<a id>` anchors (capitalized via ₢A3AAH). Under the new architecture, this README is what customers see when they click a `buh_tlt` link — served by GitHub's default markdown rendering at the public blob URL.

Anchor resolution was verified empirically against `bhyslop/recipemuster` during the design conversation: GitHub case-folds anchor IDs to lowercase but its JS shim does case-insensitive hash matching, so `#Payor` correctly scrolls to `user-content-payor`. ₢A3AAH's capitalization work is forward-compatible: it doesn't help on GitHub's default rendering but doesn't hurt either, and remains correct for any case-preserving renderer the project may adopt later (pandoc, etc.).

### README rewrite scope

Port cosmology content from `RBSCO-CosmologyIntro.adoc` into README.md as a new intro section. Frame around **two complexity domains**: Cloud Build features and crucible orchestration features. Voice is **third person**. Lede reader is "a developer who has never heard of Recipe Bottle."

Cross-linkify ported content with `[term](#Term)` markdown links wherever a Recipe Bottle term appears — matching what `rbho_onboarding.sh` already does heavily. Embed `rbm-abstract-drawio.svg` in the intro as a static image, without interactivity. Clickable internal SVG links is a deferred polish concern requiring inline-SVG and pandoc rendering.

Drop Significant Events from README (relocates to new RBSPH per below). Drop Vision Part One/Part Two narrative as duplicating README's existing How It Works content — replace both with the two-complexity-domain framing.

**Term rename scope (resolved in ₢A5AAA)**: essentially empty. The "Bottle Service → crucible" and "censer → pentacle" examples that seeded this concern were old index.html wording already swept by prior work (₣Au verb colorization, ₢AUAAP freshen). Current RBSCO, RBS0 mapping section, README, and all live vov_veiled/ files use current vocabulary. Pace 3 does NOT run a dedicated rename sweep — any incidental corrections fold into the ordinary rewrite. One residual: RBSCO's Aug 23 2025 Significant Events entry mentions "Governor, Director, Mason, and Retriever"; Mason is still valid internal vocabulary (`{rbtr_mason}`, 12 mentions in vov_veiled/) as the build service account role, not a customer-facing role. Since this line relocates to RBSPH as historical record, leave it verbatim.

### Full retirement of `index.html` and `README.consumer.md`

Both files cease to serve any purpose under the new architecture and get fully deleted from the tree. `.nojekyll` joins them — the GitHub Pages jekyll-disable marker becomes vestigial without a github.io publishing target.

- **`index.html`** was a render target for `RBSCO-CosmologyIntro.adoc` (last freshened in ₢AUAAP). The cosmology content now lives in `README.md` (intro section) and RBSCO becomes internal narrative memory. No render target needed at repo root, no `index.html` on any branch after this heat.
- **`Tools/rbk/vov_veiled/README.consumer.md`** is a stale legacy template that the `rbk-prep-release` ceremony Step 8 copies onto `README.md`. After this heat, that copy operation would overwrite the new authoritative root README with stale content — a real foot-gun. The ceremony step gets rewritten in tandem with the template deletion.
- **`.nojekyll`** at repo root is a leftover from the github.io publishing target that this heat retires. Delete for cleanliness.

The prep-release ceremony audit in this heat is **full-ceremony scope** (not just Step 8). Every step that references retired files or assumes the old README/index.html architecture needs review and update.

### `CLAUDE.consumer.md` kept and updated, not retired (resolved in ₢A5AAA)

Unlike `README.consumer.md`, `CLAUDE.consumer.md` is **kept as the canonical Claude Code ship template**. It is not a duplicate of the root `CLAUDE.md` — root `CLAUDE.md` contains internal development-only content (File Acronym Mappings, Prefix Naming Discipline, heat workflow discipline, JJK/VVK/CMK bindings) that cannot and should not serve customers. Consumer `CLAUDE.md` is a distinct customer-facing document. The prep-release Step 8 copy operation for `CLAUDE.consumer.md` → `CLAUDE.md` is the intended mechanism: at ship time, the internal development CLAUDE.md is replaced by the consumer template. No overwrite hazard.

**Stale references to update in place** (Pace 4 mechanical edit):

- Line 13: `tt/rbw-gO.Onboarding.sh` → `tt/rbw-go.OnboardMAIN.sh`
- Glossary (lines 18-31): currently 11 terms; align with README's 27-term canonical form or trim to a curated introductory subset (decision during Pace 4)

Everything else in `CLAUDE.consumer.md` (verb guide, role table, regime inspection, architecture) appears current per audit.

### Project history relocates to new RBSPH

`Tools/rbk/vov_veiled/RBSPH-ProjectHistory.adoc` is created to hold the Significant Events timeline currently embedded in RBSCO. Format: preserve RBSCO's AsciiDoc table structure (`[%header,cols="1,3"]`) — terser than bullets for date-driven chronology and preserves comment syntax cleanly. **Commented-out entries** (e.g., the `// | Feb 23, 2022 | Got Docker ID.` lines) are preserved verbatim as historical notes the project may want to reactivate later.

RBSCO sheds the timeline and becomes the project's narrative-and-vision spec doc, no longer a render target. Its header comment is updated to reflect the new purpose.

**Header format shift (resolved in ₢A5AAA)**: both RBSPH's new header and RBSCO's rewritten header use **first-class document prose**, not AsciiDoc comment blocks. The comment form was a relic of the old render pipeline where `index.adoc` generated `index.html` — with `vov_veiled/` fully withheld from customers, purpose and ancestry become ordinary document content. Pace 1 drafted both headers verbatim; Pace 4 pastes them as drafted.

### Push destination for this refresh

The first refresh commit lands on `bhyslop/recipemuster` (origin) only. The PR ceremony from origin to `scaleinv/recipebottle-staging` and ultimately to public `scaleinv/recipebottle` is explicitly deferred to a future heat — that's release-ceremony work, not refresh work. This heat ends at "clean commit on origin, push ceremony awaits user." **The push itself is user-executed, not agent-executed**, per remote-action-confirmation discipline.

## Explicitly Deferred

- **Pandoc rendering pipeline + inline-SVG-with-clickable-links.** Real value (brand-clean URLs, custom presentation, diagram-to-glossary navigation) but not blocking the link-landing problem. Defer until the GitHub default render is observed in real use and shown wanting.
- **Per-version github.io subdirectories.** Pre-supposes multiple released binary versions in circulation. The user is pre-release; pin when there's something to pin against.
- **prep-release ceremony integration with RBRR URL substitution.** The RBRR field is created in this heat; the ceremony's automated update of the field at release time is deferred until the first actual release ceremony.
- **RBS0 ↔ README drift audit slash command (`/rbk-audit-readme`).** The drift between RBS0's mapping section and README's glossary is real but small today. Build the audit when the cost of drift becomes visible in practice.
- **PR ceremony from `bhyslop/recipemuster` to `scaleinv/recipebottle`.** This heat ends at origin commit. The origin-to-public push is its own future heat, likely scheduled around first actual release.
- **Cosmology narrative as a full essay.** Pace 3 produces a working intro section from RBSCO's highlights. Further polish or expansion of the project narrative voice is its own future work.

## Additional Safety Paces

Two paces extend the core refresh work with targeted safety passes that are not part of the DDR above:

- **₢A5AAE (retention audit)** — a post-rewrite second-look pass catching content that may have been lost during AAC's rewriting motion. Produces a memo classifying dropped passages (obsolete / better-placed-elsewhere / should-reappear-in-README), then applies user-approved re-additions. Added because content-loss during rewrites is hard to see from inside the rewriting motion; not a duplicate of AAA's inventory work, which ran *before* any rewrite. Produces a `Memos/memo-*.md` artifact — intentionally different from AAA's "dockets ARE the deliverable" pattern, because a retention audit has standalone future-reference value.

- **₢A5AAF (SVG vocabulary review)** — closes an audit gap from ₢A5AAA Q1. That grep scoped RBSCO, RBS0, README, and live vov_veiled/ files but **not** `rbm-abstract-drawio.svg`, so the SVG remains the one surface where stale vocabulary might still live. Claude advises via a directive list; the user performs all drawio edits (hard boundary — Claude does not touch the SVG source). Ensures the embedded architecture diagram doesn't carry stale vocabulary into the refreshed README.

Neither pace alters the core refresh scope; both run after AAC/AAD land.

## Provenance

Emerged from chat in officium ☉260409-1000 during ₢A3AAF investigation of the underline-bleed bug. The bug fix itself was notched in ₢A3AAF as commit 930ca93f. Further investigation of *where* the fixed links actually land surfaced this heat's architectural concerns.

Architectural pivots traversed during the conversation:

1. **First framing**: render README.md → index.html locally with pandoc, push the rendered file to a github.io publishing target.
2. **Second framing**: retarget `buh_tlt` URLs to GitHub blob URLs, retire `index.html` entirely, rely on GitHub's default README rendering.
3. **Third framing**: per-version subdirectories under github.io with versioned `index.html` publishing.
4. **Fourth framing**: GitHub default README rendering at a versioned blob URL, with version pinning via split RBGC-base + RBRR-ref.
5. **Fifth framing (settled)**: GitHub default README rendering at a whole-URL RBRR field, README rewritten as harmonious combo of cosmology + glossary. The split base+ref was collapsed to whole-URL after recognizing that reincorporation-time users need to edit a single field, not two.

Case-folding concern for ₢A3AAH's capitalized anchors was resolved by empirical test: GitHub's anchor shim handles case-insensitively.

The RBS0 constraint — `.adoc` files never ship, the spec language is proprietary — eliminated any approach that would have required exposing AsciiDoc syntax to customers. RBSCO and other `vov_veiled/` docs remain internal source-of-truth for definitions and project history.

**Mid-chat discovery**: during the design conversation, two recent structural commits in the tree were not in the agent's initial mental model: ₢A3AAI renamed `bug_guide.sh` → `buh_handbook.sh` (all `bug_*` identifiers became `buh_*`), and ₢A3AAJ retired `rbgm_ManualProcedures.sh` by splitting it into `rbho_onboarding.sh` + `rbhp_payor.sh`. Any future work in this heat should use the current names (`buh_*`, `zrbho_*`, `zrbhp_*`); any grep for the old names during Paces 2-4 should come up empty.

**Paddock damage discovered during ₢A5AAA (officium ☉260409-1007)**: the original paddock (committed as 906d3e3c) carried several stale framings from its curry pass:

1. **Phantom term renames**: "Confirmed starting points: Bottle Service → crucible, censer → pentacle" cited examples that were already swept by prior heats (verb colorization ₣Au, freshen ₢AUAAP). Grep of current RBSCO returned zero hits. The rename sweep framing was obsolete.
2. **Consumer count miscount**: "Only two consumers exist" undercounted the rbho_onboarding.sh reader — the file has 6 call sites to `${RBGC_PUBLIC_DOCS_URL}`, not one.
3. **Cosmology port line budget overestimate**: "50-100 lines" estimate was ~2-3x reality; the actual port is ~30 lines since README's existing How It Works covers most of the same ground.
4. **`CLAUDE.consumer.md` misframed as parallel retirement concern**: deferred list treated it as "if yes, retirement is included in Pace 4; if no, defer." The correct framing is kept-and-updated — different audience from internal CLAUDE.md, no overwrite hazard, the prep-release Step 8 copy IS the intended mechanism.

These corrections were resolved through live conversation review during ₢A5AAA and baked into both this paddock (in place) and the downstream pace dockets (AAB, AAC, AAD). The original Q1-Q11 docket structure for ₢A5AAA was collapsed into the resolved-answer form — the memo deliverable was replaced by inline resolution in the pace docket itself.

"Refresh" in the silks came after the first attempt ("first imprint") was flagged as collision-prone with BUK's existing "imprint" tabtarget terminology. "Refresh" names the goal plainly: the public docs surface is stale and needs refreshing to a correct, current shape.

## References

- `Tools/rbk/rbho_onboarding.sh` — onboarding code; 6 call sites to `${RBGC_PUBLIC_DOCS_URL}` (lines 320, 341, 442, 564, 786, 904) all updated to `${RBRR_PUBLIC_DOCS_URL}` in Pace 2
- `Tools/rbk/rbho_cli.sh` — onboarding CLI; gains RBRR sourcing (source rbrr_regime.sh + rbrr.env + zrbrr_kindle, NOT zrbrr_enforce)
- `Tools/rbk/rbgc_Constants.sh` — `RBGC_PUBLIC_DOCS_URL` constant to be deleted (line 80)
- `Tools/rbk/rbrr_regime.sh` — RBRR enrollment; new "Public Docs" group added, single `buv_string_enroll` with bounds 1 512
- `Tools/rbk/vov_veiled/RBSRR-RegimeRepo.adoc` — spec for RBRR; target for new field documentation
- `.rbk/rbrr.env` — the one and only RBRR instance file; target for new URL value
- `README.md` — current root README (348 lines); rewrite target (~30 line growth expected, not 50-100)
- `index.html` — to be deleted (504 lines, the retired render target)
- `.nojekyll` — to be deleted (vestigial GitHub Pages marker)
- `Tools/rbk/vov_veiled/README.consumer.md` — to be deleted (stale template, ceremony foot-gun)
- `Tools/rbk/vov_veiled/CLAUDE.consumer.md` — **kept**, stale tabtarget refs updated in place
- `Tools/rbk/vov_veiled/RBSCO-CosmologyIntro.adoc` — source of porting content; header rewritten as first-class prose; Significant Events removed after RBSPH populated
- `Tools/rbk/vov_veiled/RBSHR-HorizonRoadmap.adoc` — audit for achieved-vs-gap list (only egress lockdown fully implemented; rest legitimately-deferred; CDN/CIDR weakness worth naming in README Project Direction)
- `Tools/rbk/vov_veiled/RBSPH-ProjectHistory.adoc` — to be created with relocated Significant Events, header as first-class prose
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc` — mapping section for canonical term definitions; audit found no stale attribute resolutions
- `Tools/buk/buh_handbook.sh` — handbook display module; where `buh_tltT` lives (added in ₢A3AAF, renamed from `bug_*` in ₢A3AAI)
- `.claude/commands/rbk-prep-release.md` — ceremony; Pace 1 Q9 produced per-step delta list baked into AAD docket
- ₣A3 paddock — onboarding development context (furloughed for duration of this heat)

## Paces

### cosmology-port-inventory (₢A5AAA) [complete]

**[260409-1227] complete**

## Character

Reading, judgment, and structured-note work — resolved through live conversation review in officium ☉260409-1007 rather than memo drafting. No code edits that ship, no file moves, no content rewrites. The point of this pace is to eliminate surprise before Pace 2 starts; that eliminate-surprise work has now been done in conversation and the answers are recorded directly in this docket (below) plus in the downstream pace dockets (AAB, AAC, AAD, AAE).

The original Q1-Q11 open-question structure has been collapsed. Each question has an inline answer. No separate `Memos/memo-*.md` deliverable is produced — the answers live in the dockets that consume them.

## Resolved Answers

### Q1 — Term-rename inventory: EMPTY

No rename work needed in Pace 3. The "Bottle Service → crucible" and "censer → pentacle" examples from the original paddock were old index.html wording already swept by prior heats (₣Au verb colorization, ₢AUAAP cosmology freshen). Grep of current RBSCO, README, and all live vov_veiled/ files returns zero hits for either term. RBS0 mapping section resolutions all align with current vocabulary.

One residual, no action required: RBSCO's Aug 23 2025 Significant Events entry mentions "Governor, Director, Mason, and Retriever." Mason is still valid internal vocabulary (`{rbtr_mason}`, 12 mentions in vov_veiled/) as the build service account role. Since this line relocates to RBSPH as historical record, leave verbatim.

### Q2 — RBSHR achievements vs. gaps

Only **egress lockdown** (₣Av dual-pool architecture) is fully implemented. Nine other items in RBSHR are legitimately-deferred with clear triggers, not open gaps. The user's "most items achieved" expectation is slightly off — the posture is more accurately "most items deferred-with-good-reason," which is still healthy.

Three items worth naming in README's new "Project Direction" subsection:

1. **Podman / rootless / macOS-native** — user-demand trigger
2. **Hairpin peer access** (bottle-to-bottle communication) — use-case trigger
3. **CDN/CIDR allowlist weakness** — the one real known weakness worth naming (CDN-hosted allowed domains sprawl the CIDR allowlist across shared ranges; DNS gating remains precise, IP gating is porous)

Draft prose for the subsection (~12 lines, tone OK'd by user):

> **Build pipeline.** Egress lockdown is implemented via a dual-pool Cloud Build architecture. VPC Service Controls and cosign signing are evaluated and deferred until organizational policy or external distribution triggers them.
>
> **Runtime.** Docker on Linux is the first-class runtime; rootless Podman and macOS-native workflows are deferred pending user demand. One known weakness: when allowed domains are CDN-hosted (e.g. Cloudflare), the sentry's CIDR allowlist becomes coarse — DNS-level gating remains precise, but IP-level gating is porous across shared CDN ranges.
>
> **Networking.** Bottle-to-bottle communication is feasible under the current sentry model but not implemented; waiting for a concrete use case.

Skip entirely from README (internal plumbing, no customer interest): SA numeric ID, read-back verification universalization, SA preflight universalization.

### Q3 — Cosmology Vision Part One/Part Two reshape

**Decision**: fold cosmology content INTO the existing How It Works, reframe around "two complexity domains" (Cloud Build + crucible orchestration), keep it simple — detailed content decisions deferred to Pace 3 interactive review. The user's guidance: "keep it simple, acknowledge two zones, review later."

Minimum concrete changes:
- Rename H3 "Bottle Orchestration" → "Crucible Orchestration" (matches current vocabulary)
- 2-sentence bridge acknowledging the two zones
- Pace 3 decides actual port content during interactive rewrite

### Q4 — Significant Events → RBSPH

**File**: `Tools/rbk/vov_veiled/RBSPH-ProjectHistory.adoc` (confirmed).

**Format**: preserve RBSCO's AsciiDoc table structure (`[%header,cols="1,3"]`) — terser than bullets for date-driven chronology. Preserve commented-out entries verbatim.

**Header format**: first-class document prose, NOT AsciiDoc comments. The comment form was a relic of the old index.adoc → index.html render pipeline; with vov_veiled/ fully withheld, purpose and ancestry become ordinary document content.

**Drafted header** (verbatim for Pace 4):

```asciidoc
= RBSPH-ProjectHistory: Recipe Bottle Project History

Internal project memory. Each entry captures a structural turning point:
infrastructure migrations, failed approaches, newly established concepts.
Terse by design — dates, one-line events, no narrative wrapping.
Commented-out entries are historical notes preserved for possible
reactivation.

Extracted from RBSCO-CosmologyIntro.adoc during ₣A5
(rbk-mvp-3-public-docs-refresh). Requires RBS0-SpecTop.adoc mapping
section for attribute resolution — entries cite `{at_*}` and `{st_*}`
attributes.

== Significant Events

[%header,cols="1,3"]
|===
...
```

**Date range**: all entries move, no cutoff. **Cross-reference**: RBSCO gains a one-line forward pointer: `See RBSPH-ProjectHistory.adoc for the project's significant events timeline.` No back-reference from RBSPH.

### Q5 — README structural plan

Touch nothing except the three insertion points per Q3/Q2. No structural reorganization. Pace 3 handles actual wording during interactive rewrite.

Insertion points (order is top-to-bottom):
1. New "About Recipe Bottle" paragraph (~15 lines) after elevator pitch, before Key Concepts
2. How It Works expansion per Q3 (folded, not separate)
3. "Project Direction" subsection per Q2 (~12 lines) at end of How It Works

Net growth: ~30 lines, not 50-100. The original paddock's 50-100 estimate was ~2-3x reality because README's existing How It Works already covers most of the material that RBSCO's Vision I/II sections cover.

### Q6 — RBRR field enrollment minutiae

**Name**: `RBRR_PUBLIC_DOCS_URL` — confirmed.
**Default**: `https://github.com/scaleinv/recipebottle/blob/main/README.md`
**Validator**: `buv_string_enroll` with bounds `1 512` (matches other RBRR string fields, no URL regex)
**Group**: new single-item `"Public Docs"` group, inserted after `"Secrets Directory"` and before `buv_scope_sentinel`

Exact enrollment block (paste-ready for Pace 2):

```bash
  buv_group_enroll "Public Docs"
  buv_string_enroll  RBRR_PUBLIC_DOCS_URL          1  512  "Public docs URL — readme target for buh_tlt glossary links, updated per-release or per-incorporation"
```

`.rbk/rbrr.env` line (insert before `# eof`):

```bash
# Public documentation URL — target for buh_tlt glossary links.
# Update per-release (swap branch name) or per-incorporation (point at your fork).
RBRR_PUBLIC_DOCS_URL="https://github.com/scaleinv/recipebottle/blob/main/README.md"
```

**Consumer count correction**: original paddock said "only two consumers." Actual: 2 consumer *files* (rbgc_Constants.sh declaration + rbho_onboarding.sh reader), but rbho_onboarding.sh has **6 call sites** (lines 320, 341, 442, 564, 786, 904). Pace 2 updates 1 declaration + 6 call sites.

### Q7 — Kindle tolerance verification: GO, no probe needed

User confirmed: `zrbrr_kindle` is schema-only by design. Validation (length, regex, filesystem existence) lives in a separate `rbrr_*` enforce function that is only called when valid values are known needed. This is the established design pattern; no empirical probe needed before Pace 2 begins.

Pace 2 proceeds with: source `rbrr_regime.sh`, source `.rbk/rbrr.env`, call `zrbrr_kindle`, skip `zrbrr_enforce`.

### Q8 — CLAUDE.consumer.md: KEEP AND UPDATE

Not retired. Different from README.consumer.md because root CLAUDE.md has legitimately internal-only content (File Acronym Mappings, Prefix Naming Discipline, heat workflow discipline, JJK/VVK/CMK bindings) that cannot serve customers. Consumer CLAUDE.md is a distinct customer-facing document. The prep-release Step 8 copy IS the intended mechanism — no overwrite hazard.

**Stale references to update in place** (Pace 4 mechanical edit):
- Line 13: `tt/rbw-gO.Onboarding.sh` → `tt/rbw-go.OnboardMAIN.sh`
- Glossary (lines 18-31): currently 11 terms; align with README's 27-term canonical form or trim to a curated introductory subset (decision during Pace 4)

Everything else (verb guide, role table, regime inspection, architecture) appears current per audit.

### Q9 — prep-release ceremony full-scope audit

Walked the entire `.claude/commands/rbk-prep-release.md` (13 steps). Four touch points need rewrites:

| Step | Issue | Fix |
|---|---|---|
| **Step 2** (line 35) | "Declared dependencies (**index.html promise**): bash, git, curl..." cites index.html as source of truth for deps | Change "index.html promise" → "README.md Prerequisites section" |
| **Step 8** (lines 106-115) | Copies both `CLAUDE.consumer.md → CLAUDE.md` AND `README.consumer.md → README.md` | Delete only the `README.consumer.md` copy line. Keep the `CLAUDE.consumer.md` copy. Also drop the note about lowercase `readme.md` removal |
| **Step 10e** (line 202) | Lists `readme.md` (lowercase) for removal | Delete the `readme.md` line if it no longer exists. Add defensive `git rm -f --ignore-unmatch index.html` AND `git rm -f --ignore-unmatch .nojekyll` |
| **Step 10 surviving list** (lines 215-226) | Lists `index.html` and `.nojekyll` as survivors | Remove both lines |
| **Step 10f** (lines 205-209) | `git add CLAUDE.md README.md` | Change to `git add CLAUDE.md` only (README.md already committed on main) |

Everything else (Steps 0, 1, 3-7, 9, 10a-d, 11, 12, 13) is unchanged.

### Q10 — RBSCO header rewrite

First-class prose, not comment block (same pattern as Q4). **Drafted verbatim for Pace 4**:

```asciidoc
= RBSCO-CosmologyIntro: Recipe Bottle Cosmology Overview

Internal narrative memory. The project's "what Recipe Bottle is and why
it exists" story, the two-complexity-domain framing (image management
and crucible orchestration), and reference material that informs
customer-facing documentation without itself being shipped. Never
rendered to a publishing target, never consumed by the build pipeline —
a source-of-truth for writers porting into README.md and other
customer-visible surfaces.

Renamed from index.adoc (last committed 84323c31, 2025-08-18). Full git
lineage: 31 commits from 2642121c through 84323c31. Significant Events
timeline relocated to RBSPH-ProjectHistory.adoc during ₣A5
(rbk-mvp-3-public-docs-refresh). Requires RBS0-SpecTop.adoc mapping
section for attribute resolution.
```

### Q11 — Lingering reference sweep

**Net Pace 4 file actions**:

- **DELETE**: `index.html`, `.nojekyll`, `Tools/rbk/vov_veiled/README.consumer.md`
- **UPDATE**: `.claude/commands/rbk-prep-release.md` (per Q9), `Tools/rbk/vov_veiled/RBSCO-CosmologyIntro.adoc` (per Q10 + strip Significant Events), `Tools/rbk/vov_veiled/CLAUDE.consumer.md` (per Q8)
- **CREATE**: `Tools/rbk/vov_veiled/RBSPH-ProjectHistory.adoc` (per Q4)

**KEEP (historical, immutable)**: all hits in `.claude/jjm/retired/*`, `.claude/jjm/jjp_uAl5.md` (current paddock), `.claude/jjm/jjp_uAuU.md`, `.claude/jjm/jjg_gallops.json`. These reference retired content as part of heat history and should not be touched.

## Deliverable

The answers above ARE the deliverable. Downstream paces (AAB, AAC, AAD, AAE) have their dockets reslated to reference these answers inline. No separate `Memos/memo-*.md` file is produced — the resolved form lives in the pace docket itself for the duration of the heat and in git history after wrap.

## References

- ₣A5 paddock (curried 9053a668) — DDR and resolved answers
- Full research reads performed during resolution:
  - `Tools/rbk/vov_veiled/RBSCO-CosmologyIntro.adoc` — 301 lines, Q1/Q3/Q10 source
  - `Tools/rbk/vov_veiled/RBSHR-HorizonRoadmap.adoc` — 193 lines, Q2 source
  - `Tools/rbk/vov_veiled/CLAUDE.consumer.md` — 210 lines, Q8 audit target
  - `Tools/rbk/vov_veiled/README.consumer.md` — 328 lines, confirmed-stale-retire
  - `.claude/commands/rbk-prep-release.md` — 257 lines, Q9 audit target
  - `Tools/rbk/rbrr_regime.sh` — Q6/Q7 analysis source
  - `Tools/rbk/rbho_cli.sh` — Q7 sourcing target
  - `README.md` — 348 lines, Q5 baseline
  - `.rbk/rbrr.env` — Q6 instance reference

**[260409-1219] rough**

## Character

Reading, judgment, and structured-note work — resolved through live conversation review in officium ☉260409-1007 rather than memo drafting. No code edits that ship, no file moves, no content rewrites. The point of this pace is to eliminate surprise before Pace 2 starts; that eliminate-surprise work has now been done in conversation and the answers are recorded directly in this docket (below) plus in the downstream pace dockets (AAB, AAC, AAD, AAE).

The original Q1-Q11 open-question structure has been collapsed. Each question has an inline answer. No separate `Memos/memo-*.md` deliverable is produced — the answers live in the dockets that consume them.

## Resolved Answers

### Q1 — Term-rename inventory: EMPTY

No rename work needed in Pace 3. The "Bottle Service → crucible" and "censer → pentacle" examples from the original paddock were old index.html wording already swept by prior heats (₣Au verb colorization, ₢AUAAP cosmology freshen). Grep of current RBSCO, README, and all live vov_veiled/ files returns zero hits for either term. RBS0 mapping section resolutions all align with current vocabulary.

One residual, no action required: RBSCO's Aug 23 2025 Significant Events entry mentions "Governor, Director, Mason, and Retriever." Mason is still valid internal vocabulary (`{rbtr_mason}`, 12 mentions in vov_veiled/) as the build service account role. Since this line relocates to RBSPH as historical record, leave verbatim.

### Q2 — RBSHR achievements vs. gaps

Only **egress lockdown** (₣Av dual-pool architecture) is fully implemented. Nine other items in RBSHR are legitimately-deferred with clear triggers, not open gaps. The user's "most items achieved" expectation is slightly off — the posture is more accurately "most items deferred-with-good-reason," which is still healthy.

Three items worth naming in README's new "Project Direction" subsection:

1. **Podman / rootless / macOS-native** — user-demand trigger
2. **Hairpin peer access** (bottle-to-bottle communication) — use-case trigger
3. **CDN/CIDR allowlist weakness** — the one real known weakness worth naming (CDN-hosted allowed domains sprawl the CIDR allowlist across shared ranges; DNS gating remains precise, IP gating is porous)

Draft prose for the subsection (~12 lines, tone OK'd by user):

> **Build pipeline.** Egress lockdown is implemented via a dual-pool Cloud Build architecture. VPC Service Controls and cosign signing are evaluated and deferred until organizational policy or external distribution triggers them.
>
> **Runtime.** Docker on Linux is the first-class runtime; rootless Podman and macOS-native workflows are deferred pending user demand. One known weakness: when allowed domains are CDN-hosted (e.g. Cloudflare), the sentry's CIDR allowlist becomes coarse — DNS-level gating remains precise, but IP-level gating is porous across shared CDN ranges.
>
> **Networking.** Bottle-to-bottle communication is feasible under the current sentry model but not implemented; waiting for a concrete use case.

Skip entirely from README (internal plumbing, no customer interest): SA numeric ID, read-back verification universalization, SA preflight universalization.

### Q3 — Cosmology Vision Part One/Part Two reshape

**Decision**: fold cosmology content INTO the existing How It Works, reframe around "two complexity domains" (Cloud Build + crucible orchestration), keep it simple — detailed content decisions deferred to Pace 3 interactive review. The user's guidance: "keep it simple, acknowledge two zones, review later."

Minimum concrete changes:
- Rename H3 "Bottle Orchestration" → "Crucible Orchestration" (matches current vocabulary)
- 2-sentence bridge acknowledging the two zones
- Pace 3 decides actual port content during interactive rewrite

### Q4 — Significant Events → RBSPH

**File**: `Tools/rbk/vov_veiled/RBSPH-ProjectHistory.adoc` (confirmed).

**Format**: preserve RBSCO's AsciiDoc table structure (`[%header,cols="1,3"]`) — terser than bullets for date-driven chronology. Preserve commented-out entries verbatim.

**Header format**: first-class document prose, NOT AsciiDoc comments. The comment form was a relic of the old index.adoc → index.html render pipeline; with vov_veiled/ fully withheld, purpose and ancestry become ordinary document content.

**Drafted header** (verbatim for Pace 4):

```asciidoc
= RBSPH-ProjectHistory: Recipe Bottle Project History

Internal project memory. Each entry captures a structural turning point:
infrastructure migrations, failed approaches, newly established concepts.
Terse by design — dates, one-line events, no narrative wrapping.
Commented-out entries are historical notes preserved for possible
reactivation.

Extracted from RBSCO-CosmologyIntro.adoc during ₣A5
(rbk-mvp-3-public-docs-refresh). Requires RBS0-SpecTop.adoc mapping
section for attribute resolution — entries cite `{at_*}` and `{st_*}`
attributes.

== Significant Events

[%header,cols="1,3"]
|===
...
```

**Date range**: all entries move, no cutoff. **Cross-reference**: RBSCO gains a one-line forward pointer: `See RBSPH-ProjectHistory.adoc for the project's significant events timeline.` No back-reference from RBSPH.

### Q5 — README structural plan

Touch nothing except the three insertion points per Q3/Q2. No structural reorganization. Pace 3 handles actual wording during interactive rewrite.

Insertion points (order is top-to-bottom):
1. New "About Recipe Bottle" paragraph (~15 lines) after elevator pitch, before Key Concepts
2. How It Works expansion per Q3 (folded, not separate)
3. "Project Direction" subsection per Q2 (~12 lines) at end of How It Works

Net growth: ~30 lines, not 50-100. The original paddock's 50-100 estimate was ~2-3x reality because README's existing How It Works already covers most of the material that RBSCO's Vision I/II sections cover.

### Q6 — RBRR field enrollment minutiae

**Name**: `RBRR_PUBLIC_DOCS_URL` — confirmed.
**Default**: `https://github.com/scaleinv/recipebottle/blob/main/README.md`
**Validator**: `buv_string_enroll` with bounds `1 512` (matches other RBRR string fields, no URL regex)
**Group**: new single-item `"Public Docs"` group, inserted after `"Secrets Directory"` and before `buv_scope_sentinel`

Exact enrollment block (paste-ready for Pace 2):

```bash
  buv_group_enroll "Public Docs"
  buv_string_enroll  RBRR_PUBLIC_DOCS_URL          1  512  "Public docs URL — readme target for buh_tlt glossary links, updated per-release or per-incorporation"
```

`.rbk/rbrr.env` line (insert before `# eof`):

```bash
# Public documentation URL — target for buh_tlt glossary links.
# Update per-release (swap branch name) or per-incorporation (point at your fork).
RBRR_PUBLIC_DOCS_URL="https://github.com/scaleinv/recipebottle/blob/main/README.md"
```

**Consumer count correction**: original paddock said "only two consumers." Actual: 2 consumer *files* (rbgc_Constants.sh declaration + rbho_onboarding.sh reader), but rbho_onboarding.sh has **6 call sites** (lines 320, 341, 442, 564, 786, 904). Pace 2 updates 1 declaration + 6 call sites.

### Q7 — Kindle tolerance verification: GO, no probe needed

User confirmed: `zrbrr_kindle` is schema-only by design. Validation (length, regex, filesystem existence) lives in a separate `rbrr_*` enforce function that is only called when valid values are known needed. This is the established design pattern; no empirical probe needed before Pace 2 begins.

Pace 2 proceeds with: source `rbrr_regime.sh`, source `.rbk/rbrr.env`, call `zrbrr_kindle`, skip `zrbrr_enforce`.

### Q8 — CLAUDE.consumer.md: KEEP AND UPDATE

Not retired. Different from README.consumer.md because root CLAUDE.md has legitimately internal-only content (File Acronym Mappings, Prefix Naming Discipline, heat workflow discipline, JJK/VVK/CMK bindings) that cannot serve customers. Consumer CLAUDE.md is a distinct customer-facing document. The prep-release Step 8 copy IS the intended mechanism — no overwrite hazard.

**Stale references to update in place** (Pace 4 mechanical edit):
- Line 13: `tt/rbw-gO.Onboarding.sh` → `tt/rbw-go.OnboardMAIN.sh`
- Glossary (lines 18-31): currently 11 terms; align with README's 27-term canonical form or trim to a curated introductory subset (decision during Pace 4)

Everything else (verb guide, role table, regime inspection, architecture) appears current per audit.

### Q9 — prep-release ceremony full-scope audit

Walked the entire `.claude/commands/rbk-prep-release.md` (13 steps). Four touch points need rewrites:

| Step | Issue | Fix |
|---|---|---|
| **Step 2** (line 35) | "Declared dependencies (**index.html promise**): bash, git, curl..." cites index.html as source of truth for deps | Change "index.html promise" → "README.md Prerequisites section" |
| **Step 8** (lines 106-115) | Copies both `CLAUDE.consumer.md → CLAUDE.md` AND `README.consumer.md → README.md` | Delete only the `README.consumer.md` copy line. Keep the `CLAUDE.consumer.md` copy. Also drop the note about lowercase `readme.md` removal |
| **Step 10e** (line 202) | Lists `readme.md` (lowercase) for removal | Delete the `readme.md` line if it no longer exists. Add defensive `git rm -f --ignore-unmatch index.html` AND `git rm -f --ignore-unmatch .nojekyll` |
| **Step 10 surviving list** (lines 215-226) | Lists `index.html` and `.nojekyll` as survivors | Remove both lines |
| **Step 10f** (lines 205-209) | `git add CLAUDE.md README.md` | Change to `git add CLAUDE.md` only (README.md already committed on main) |

Everything else (Steps 0, 1, 3-7, 9, 10a-d, 11, 12, 13) is unchanged.

### Q10 — RBSCO header rewrite

First-class prose, not comment block (same pattern as Q4). **Drafted verbatim for Pace 4**:

```asciidoc
= RBSCO-CosmologyIntro: Recipe Bottle Cosmology Overview

Internal narrative memory. The project's "what Recipe Bottle is and why
it exists" story, the two-complexity-domain framing (image management
and crucible orchestration), and reference material that informs
customer-facing documentation without itself being shipped. Never
rendered to a publishing target, never consumed by the build pipeline —
a source-of-truth for writers porting into README.md and other
customer-visible surfaces.

Renamed from index.adoc (last committed 84323c31, 2025-08-18). Full git
lineage: 31 commits from 2642121c through 84323c31. Significant Events
timeline relocated to RBSPH-ProjectHistory.adoc during ₣A5
(rbk-mvp-3-public-docs-refresh). Requires RBS0-SpecTop.adoc mapping
section for attribute resolution.
```

### Q11 — Lingering reference sweep

**Net Pace 4 file actions**:

- **DELETE**: `index.html`, `.nojekyll`, `Tools/rbk/vov_veiled/README.consumer.md`
- **UPDATE**: `.claude/commands/rbk-prep-release.md` (per Q9), `Tools/rbk/vov_veiled/RBSCO-CosmologyIntro.adoc` (per Q10 + strip Significant Events), `Tools/rbk/vov_veiled/CLAUDE.consumer.md` (per Q8)
- **CREATE**: `Tools/rbk/vov_veiled/RBSPH-ProjectHistory.adoc` (per Q4)

**KEEP (historical, immutable)**: all hits in `.claude/jjm/retired/*`, `.claude/jjm/jjp_uAl5.md` (current paddock), `.claude/jjm/jjp_uAuU.md`, `.claude/jjm/jjg_gallops.json`. These reference retired content as part of heat history and should not be touched.

## Deliverable

The answers above ARE the deliverable. Downstream paces (AAB, AAC, AAD, AAE) have their dockets reslated to reference these answers inline. No separate `Memos/memo-*.md` file is produced — the resolved form lives in the pace docket itself for the duration of the heat and in git history after wrap.

## References

- ₣A5 paddock (curried 9053a668) — DDR and resolved answers
- Full research reads performed during resolution:
  - `Tools/rbk/vov_veiled/RBSCO-CosmologyIntro.adoc` — 301 lines, Q1/Q3/Q10 source
  - `Tools/rbk/vov_veiled/RBSHR-HorizonRoadmap.adoc` — 193 lines, Q2 source
  - `Tools/rbk/vov_veiled/CLAUDE.consumer.md` — 210 lines, Q8 audit target
  - `Tools/rbk/vov_veiled/README.consumer.md` — 328 lines, confirmed-stale-retire
  - `.claude/commands/rbk-prep-release.md` — 257 lines, Q9 audit target
  - `Tools/rbk/rbrr_regime.sh` — Q6/Q7 analysis source
  - `Tools/rbk/rbho_cli.sh` — Q7 sourcing target
  - `README.md` — 348 lines, Q5 baseline
  - `.rbk/rbrr.env` — Q6 instance reference

**[260409-1053] rough**

## Character

Reading and structured-note work. Mostly RBSCO, RBSHR, RBS0 spelunking plus README re-read, RBRR enrollment pattern review, and kindle-tolerance empirical verification. No code edits that ship, no file moves, no content rewrites.

Cognitive posture: careful, comprehensive, not hurried. Each question below needs a complete answer before downstream paces can execute cleanly. The point of this pace is to eliminate surprise — by the time Pace 2 starts, every structural choice should already be made and every verification result known.

## Docket

Resolve the open questions below. For each, the output is either a decision, a named artifact (table, skeleton, drafted text), or a verified go/no-go on a structural choice. The aggregate output is a single structured note committed in this dev repo under `Memos/`, which Paces 2-4 read as their working specification.

### Q1. Full term-rename inventory (RBSCO sweep)

Walk `Tools/rbk/vov_veiled/RBSCO-CosmologyIntro.adoc` end-to-end. Identify every term whose vocabulary has shifted in current Recipe Bottle. Confirmed starting points:

- Bottle Service → crucible
- censer → pentacle

Likely more. Cross-reference RBSCO's MCM attribute references (`{at_*}`, `{rbtr_*}`, etc.) against `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`'s mapping section to find attributes that resolve to stale display text.

**Output**: A table mapping old term → new term → MCM attribute (if any) → reason for change. Drives Pace 3's term rename sweep.

### Q2. RBSHR achievements vs. gaps audit

Read `Tools/rbk/vov_veiled/RBSHR-HorizonRoadmap.adoc` end-to-end. For each roadmap item, determine:

- **Achieved**: drop from any README mention. Keep in the internal spec for history.
- **In-progress**: no README mention. Still speculative.
- **Deferred-with-rationale**: one-or-two-sentence README mention in the "Project Direction" section, podman-VM-versioning style.

User's expectation (from design chat): "most items achieved." Q2's output validates or contradicts this expectation. If most items are achieved, the README's project-direction subsection is terse; if many are still open, it's longer.

**Output**: A list of "big gaps documented" entries to include in README's project-direction subsection. Terse — each entry is a single sentence or two.

### Q3. Cosmology Vision Part One/Part Two reshape

RBSCO has 80+ lines under Vision Part One (Image Management) and Part Two (Crucible Orchestration). README has its own `## How It Works` section with `### Image Management` and `### Bottle Orchestration` subsections.

**Decision**: Replace README's existing How It Works with the cosmology version? Fold the cosmology content INTO the existing How It Works? Keep existing and add the cosmology as a separate "Architecture" section? The new framing per the paddock DDR is "two complexity domains: Cloud Build features, crucible orchestration features" — reshape the old material around that framing.

**Output**: A structural plan for the new section — section headings, approximate length per subsection, what content comes from RBSCO vs what stays from current README vs what is new bridging prose.

### Q4. Significant Events relocation plan (RBSPH skeleton draft)

Confirm `Tools/rbk/vov_veiled/RBSPH-ProjectHistory.adoc` as the new home. Decide:

- File header comment establishing purpose (internal project memory, never ships, replaces RBSCO's timeline role). **Draft the actual text verbatim**, don't describe it.
- Format: preserve current RBSCO table-style or switch to bullet list? The paddock says "terse" — which format is terser for this content?
- Timeline date range: all entries from RBSCO move, or only entries from a specific cutoff?
- Whether to link back to RBSCO (or forward from RBSCO) to cross-reference
- Preserve commented-out entries verbatim

**Output**: A draft RBSPH skeleton ready for Pace 4 to populate. Header comment drafted verbatim, section structure decided, one or two example entries illustrating the chosen format.

### Q5. README structural plan

Decide the order of sections in revised README. Current structure:

1. Intro + elevator pitch (~20 lines)
2. Key Concepts (27-row glossary table)
3. How It Works (Image Management + Bottle Orchestration)
4. Prerequisites + Using the CLI
5. Setup (four phases)
6. Day-to-Day Operations
7. Credential Safety
8. Configuration
9. Vessels and Nameplates
10. Testing + closing

**Decision**: Where does the new cosmology-port intro fit? Extend the existing intro? Add a new "About Recipe Bottle" section between intro and Key Concepts? Fold into How It Works? What is the approximate length budget for the cosmology port (current intro is ~20 lines; cosmology port might add 50-100 lines)?

**Output**: A revised section outline with approximate line budgets and a clear placement for the new cosmology content.

### Q6. RBRR field enrollment minutiae

Decide:

- **Variable name**: `RBRR_PUBLIC_DOCS_URL` or alternative? The paddock uses this name; confirm it's the right one.
- **Default value** in `.rbk/rbrr.env`: `https://github.com/scaleinv/recipebottle/blob/main/README.md` (per paddock)
- **Validator type**: `buv_string_enroll` with length bounds (min 1, max 512)? Or stricter URL regex? String-with-bounds is consistent with other RBRR fields.
- **Enrollment placement**: which `buv_group_enroll` group does it belong to? A new "Public Docs" group, or fold into an existing group?

**Output**: The exact enrollment line ready to drop into `rbrr_regime.sh`, and a documentation snippet for `RBSRR-RegimeRepo.adoc`.

### Q7. Kindle tolerance empirical verification

**Critical architectural verification**. The paddock assumes `rbho_cli.sh` can source `rbrr_regime.sh`, source `.rbk/rbrr.env`, call `zrbrr_kindle`, and skip `zrbrr_enforce`. Verify this assumption empirically:

- Does `zrbrr_kindle` succeed when values are present but filesystem enforce would fail (e.g., vessel dir doesn't exist)? Expected yes (kindle is schema-only), verify.
- Does `zrbrr_kindle` succeed when values are empty strings? Expected maybe — length validators may reject. Verify.
- Does `zrbrr_kindle` fail loudly when `.rbk/rbrr.env` is missing entirely? Expected yes, verify.
- What happens if the user runs onboarding on a fresh install before touching `rbrr.env`? Simulate (e.g., temporarily rename the file) and observe.

If kindle fails on realistic pre-depot-setup states, the architecture needs a different resolution. Pace 1 surfaces the failure and proposes alternatives (e.g., lightweight sub-regime, conditional sourcing based on onboarding phase).

**Output**: A go/no-go verdict on the sourcing-and-kindle approach. If no-go, a written alternative design.

### Q8. CLAUDE.consumer.md audit

`Tools/rbk/vov_veiled/CLAUDE.consumer.md` is a parallel concern to `README.consumer.md`. Does the `rbk-prep-release` ceremony Step 8 also copy this onto root `CLAUDE.md`? If yes, and the current root `CLAUDE.md` has authoritative content that would be overwritten, the template needs similar retirement in Pace 4. If the consumer template is already canonical for customers, it stays — but then the root `CLAUDE.md` for development work is separate and the prep-release copy is correct. Investigate and decide.

**Output**: Explicit decision on `CLAUDE.consumer.md` fate (retire / keep / update). Added to Pace 4's file-action manifest.

### Q9. prep-release ceremony full-scope audit

The previous framing focused only on Step 8 (consumer template copy). Widen the audit to **the entire `.claude/commands/rbk-prep-release.md` file**. Every step that references retired files or assumes the old architecture needs review:

- Step 8 copies README.consumer.md and CLAUDE.consumer.md — needs rewrite after retirements
- Step 10 "strip proprietary content" — does it enumerate `index.html` as a surviving file? If so, update
- Step 10's "surviving files" list — does it still make sense under the new architecture?
- Steps 9-13 — audit each for assumptions that break

**Output**: A per-step delta list. For each prep-release step, either "no change" or "specific rewrite needed." Pace 4 implements the rewrites.

### Q10. RBSCO header comment draft

`Tools/rbk/vov_veiled/RBSCO-CosmologyIntro.adoc` header comment currently says "the index.html frontmatter source for the Recipe Bottle project page." After this heat, the purpose shifts to "internal narrative memory — cosmology spec, project vision, and reference material; never ships, never rendered."

**Draft the replacement header comment verbatim.** Pace 4 will paste this into the file — no wording decisions left for downstream paces.

**Output**: Complete drafted header comment text, ready for verbatim paste.

### Q11. Lingering reference sweep

Grep the entire tree for `index.html`, `README.consumer.md`, `CLAUDE.consumer.md`. Catalogue every hit. Some are legitimate historical references (commits, paddocks, old specs) that should remain; some are live assumptions that need updating. Separate the two.

**Output**: A categorized list — keep / update / delete — with file:line references. The "update" and "delete" entries become Pace 4 cleanup items.

## Pace 1 Deliverable

A single structured note, committed to `Memos/` (filename like `memo-260409-public-docs-refresh-inventory.md`), containing the decisions, drafts, and verdicts for all 11 questions above. Paces 2-4 reference this note as their working specification.

## References

- `Tools/rbk/vov_veiled/RBSCO-CosmologyIntro.adoc` — primary source for term inventory and cosmology port
- `Tools/rbk/vov_veiled/RBSHR-HorizonRoadmap.adoc` — roadmap for gap audit
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc` — mapping section for canonical definitions
- `Tools/rbk/vov_veiled/RBSRR-RegimeRepo.adoc` — spec for RBRR fields
- `Tools/rbk/rbrr_regime.sh` — enrollment pattern reference
- `Tools/rbk/rbho_cli.sh` — sourcing pattern; verification target for Q7
- `Tools/rbk/vov_veiled/CLAUDE.consumer.md` — parallel template to audit
- `.claude/commands/rbk-prep-release.md` — ceremony full-audit target
- `README.md` — current state reference
- `.rbk/rbrr.env` — the instance file
- ₣A5 paddock — DDR and deferred list

**[260409-1022] rough**

## Character

Reading and structured-note work. Mostly RBSCO, RBSHR, RBS0 spelunking plus README re-read and light RBRR enrollment pattern review. No code edits, no file moves, no content rewrites. Output is a structured note that drives Pace 2.

Cognitive posture: careful, comprehensive, not hurried. Each question below needs a complete answer before Pace 2 can execute cleanly. The point of this pace is to eliminate surprise — by the time Pace 2 starts, every structural choice should already be made.

## Docket

Resolve the open questions below. For each, the output is either a decision, a named artifact (e.g., a table, a skeleton), or a go/no-go on a structural choice. The aggregate output is a single structured note (committed in this dev repo, likely under `Memos/`) that Pace 2 reads as its working specification.

### Open Questions

#### 1. Full term-rename inventory (RBSCO sweep)

Walk `RBSCO-CosmologyIntro.adoc` end-to-end. Identify every term whose vocabulary has shifted in current Recipe Bottle. Confirmed starting points:

- Bottle Service → crucible
- censer → pentacle

Likely more. Cross-reference RBSCO's MCM attribute references against RBS0-SpecTop.adoc's mapping section to find attributes that resolve to stale display text.

**Output**: A table mapping old term → new term → MCM attribute (if any) → reason for change. This table drives search-and-replace during Pace 2's README rewrite.

#### 2. RBSHR achievements vs. gaps audit

Read `Tools/rbk/vov_veiled/RBSHR-HorizonRoadmap.adoc` end-to-end. For each roadmap item, determine:

- **Achieved**: drop from any README mention. Keep in the internal spec for history.
- **In-progress**: no README mention. Still speculative.
- **Deferred-with-rationale**: one-or-two-sentence README mention in the "Project Direction" section, podman-VM-versioning style (i.e., a brief note explaining the thinking without over-committing).

**Output**: A list of "big gaps documented" entries to include in README's project-direction subsection. Terse — each entry is a single sentence or two.

#### 3. Cosmology Vision Part One/Part Two reshape

RBSCO has 80+ lines under Vision Part One (Image Management) and Part Two (Crucible Orchestration). README already has a `## How It Works` section with `### Image Management` and `### Bottle Orchestration` subsections.

**Decision to make**: Replace README's existing How It Works with the cosmology version? Fold the cosmology content INTO the existing How It Works? Keep existing and add the cosmology as a separate "Architecture" section? The new framing per the paddock DDR is "two complexity domains: Cloud Build features, crucible orchestration features" — reshape the old material around that framing.

**Output**: A structural plan for the new section — section headings, approximate length per subsection, what content comes from RBSCO vs what stays from current README vs what is new bridging prose.

#### 4. Significant Events relocation plan (RBSPH skeleton)

Confirm `Tools/rbk/vov_veiled/RBSPH-ProjectHistory.adoc` as the new home. Decide:

- File header comment establishing purpose (internal project memory, never ships, replaces RBSCO's timeline role)
- Format: preserve current table-style or switch to bullet list? The paddock says "terse" — which format is terser for this content?
- Preserve commented-out entries (`// | Feb 23, 2022 | ...` style) as-is
- Timeline date range: all entries from RBSCO move, or only entries from a specific cutoff?
- Whether to link back to RBSCO (or forward from RBSCO) to cross-reference

**Output**: A draft RBSPH skeleton ready for Pace 2 to populate. Header comment drafted, section structure decided, one or two example entries to illustrate the chosen format.

#### 5. README structural plan

Decide the order of sections in revised README. Current structure:

1. Intro + elevator pitch
2. Key Concepts (glossary table)
3. How It Works (Image Management + Bottle Orchestration)
4. Prerequisites
5. Using the CLI
6. Setup (four phases)
7. Day-to-Day Operations
8. Credential Safety
9. Configuration
10. Vessels and Nameplates
11. Testing + closing

**Decision to make**: Where does the new cosmology-port intro fit? Extend the existing intro? Add a new "About Recipe Bottle" section between intro and Key Concepts? Fold into How It Works? What's the approximate length budget for the cosmology port (current intro is ~20 lines; cosmology port might add 50-100 lines)?

**Output**: A revised section outline with approximate line budgets and a clear placement for the new cosmology content.

#### 6. RBRR field minutiae

Decide:

- **Variable name**: `RBRR_PUBLIC_DOCS_REF` or alternative? Matches `RBRR_*` naming conventions, implies "a git ref" which matches the semantic.
- **Default value** for in-source-tree dev: `main`. (Paddock says so; confirm and write into the enrollment.)
- **Validator type**: `buv_string_enroll` with length bounds? Or stricter regex that matches valid git ref names (no spaces, no control chars, no leading hyphens)?
- **Where the URL composition lives**: Kindle-time in `rbgc_Constants.sh`? Or per-call in a helper function? The simpler answer is kindle-time but that requires RBRR to be sourced before RBGC, which may or may not be the current order.
- **Scope variable name** for the enrollment (the `ZRBRR_SCOPE_*` pattern).

**Output**: An enrollment line ready to drop into `rbrr_regime.sh`, a documentation snippet for `RBSRR-RegimeRepo.adoc`, and a composition snippet ready for `rbgc_Constants.sh`. All three ready for Pace 2 to paste-and-verify.

#### 7. Files-to-retire confirmation + `CLAUDE.consumer.md` audit

Confirmed for retirement: `index.html`, `Tools/rbk/vov_veiled/README.consumer.md`.

Audit:

- **`Tools/rbk/vov_veiled/CLAUDE.consumer.md`** — parallel concern. Does the prep-release ceremony Step 8 also copy this onto root `CLAUDE.md`? If yes, and the current root `CLAUDE.md` has authoritative content that would be overwritten, the template needs similar retirement. If the consumer template is already canonical for customers, it stays — but then the root `CLAUDE.md` for development work is separate and the prep-release copy is correct. Investigate and decide.
- **`Tools/rbk/vov_veiled/RBSCO-CosmologyIntro.adoc`** — header comment update needed (purpose shifts from "index.html source" to "internal narrative memory"). Draft the replacement header comment.
- **`.claude/commands/rbk-prep-release.md`** — Step 8 copies both consumer templates onto root files. After this heat's retirements, Step 8 needs rewriting. Draft the replacement language.
- **Any lingering references** in code, spec docs, or test fixtures to `index.html` or `README.consumer.md` — grep across the tree and list for Pace 2 cleanup.

**Output**: A complete file-action manifest for Pace 2 — every file to create, modify, or delete, with a one-line reason for each.

## References

- `Tools/rbk/vov_veiled/RBSCO-CosmologyIntro.adoc` — primary source for term inventory + Vision content
- `Tools/rbk/vov_veiled/RBSHR-HorizonRoadmap.adoc` — roadmap for gap audit
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc` — mapping section for term canonical definitions and display text
- `Tools/rbk/vov_veiled/RBSRR-RegimeRepo.adoc` — spec doc for RBRR fields
- `README.md` — current state, structural reference, rewrite target
- `Tools/rbk/rbrr_regime.sh` — enrollment patterns to follow
- `Tools/rbk/rbgc_Constants.sh` — URL composition site
- `Tools/rbk/vov_veiled/CLAUDE.consumer.md` — parallel template to audit
- `.claude/commands/rbk-prep-release.md` — ceremony Step 8 that references the consumer templates
- ₣A5 paddock — DDR and deferred list for context

### rbrr-docs-url-field (₢A5AAB) [complete]

**[260409-1305] complete**

## Character

Mechanical, atomic, verification-gated. Adds the new RBRR field, updates the sourcing chain in the onboarding CLI, deletes the RBGC constant, updates the onboarding consumer. No content work, no file retirements. Must land before Pace 3 (README rewrite) because the README rewrite's link verification depends on the new URL form being active.

Cognitive posture: careful execution per ₢A5AAA's resolved answers. No decisions left — all minutiae confirmed.

Silks: `rbrr-docs-url-field`.

## Docket

**Prerequisite**: ₢A5AAA resolved (all Q answers in place). Q6 (field minutiae) and Q7 (kindle tolerance) both return GO.

### Steps

1. **Add `RBRR_PUBLIC_DOCS_URL` enrollment** to `Tools/rbk/rbrr_regime.sh`. Insert new `"Public Docs"` group after the `"Secrets Directory"` group and before `buv_scope_sentinel`. Exact paste:

   ```bash
     buv_group_enroll "Public Docs"
     buv_string_enroll  RBRR_PUBLIC_DOCS_URL          1  512  "Public docs URL — readme target for buh_tlt glossary links, updated per-release or per-incorporation"
   ```

2. **Document the field** in `Tools/rbk/vov_veiled/RBSRR-RegimeRepo.adoc`. Follow the existing RBRR field documentation pattern — single linked term block, mapping section attribute reference, definition site. Read RBSRR first to match the pattern; the snippet is a 5-10 line addition.

3. **Add the field to `.rbk/rbrr.env`** (insert before `# eof`):

   ```bash
   # Public documentation URL — target for buh_tlt glossary links.
   # Update per-release (swap branch name) or per-incorporation (point at your fork).
   RBRR_PUBLIC_DOCS_URL="https://github.com/scaleinv/recipebottle/blob/main/README.md"
   ```

4. **Delete `RBGC_PUBLIC_DOCS_URL`** from `Tools/rbk/rbgc_Constants.sh:80`. Single-line deletion.

5. **Update all 6 consumer call sites in `Tools/rbk/rbho_onboarding.sh`**: lines 320, 341, 442, 564, 786, 904. Each reads `${RBGC_PUBLIC_DOCS_URL}` — change to `${RBRR_PUBLIC_DOCS_URL}`. Most assign to a local `z_docs` once per function. Verify by grep after the edit:

   ```
   grep -n 'RBGC_PUBLIC_DOCS_URL' Tools/rbk/rbho_onboarding.sh
   # Expect: zero hits
   grep -n 'RBRR_PUBLIC_DOCS_URL' Tools/rbk/rbho_onboarding.sh
   # Expect: 6 hits
   ```

6. **Update `Tools/rbk/rbho_cli.sh`** sourcing chain. Before `zrbho_kindle` (current line 50), add RBRR sourcing and kindle call. Do NOT call `zrbrr_enforce` — that's the thin-deps concession per the paddock DDR. The exact source path pattern depends on how rbrr.env is referenced (likely via `RBBC_rbrr_file` constant — check `rbbc_constants.sh` first).

7. **Run regime validation**: `tt/rbw-rrv.ValidateRepoRegime.sh`. Expect success.

8. **Run fast qualification**: `tt/rbw-tf.QualifyFast.sh`. Expect success.

9. **Verify onboarding renders with the new URL form**: `tt/rbw-go.OnboardMAIN.sh`. OSC-8 hyperlinks should now point at `https://github.com/scaleinv/recipebottle/blob/main/README.md#Retriever` (etc.). Inspect raw escape sequences via `tt/rbw-go.OnboardMAIN.sh 2>&1 | cat -v` to confirm.

10. **User verification**: present the command output. **Ask the user to click one link in their terminal and confirm browser navigation lands at the right glossary row.** Agent cannot do this step.

11. **Notch** with intent: "Move public docs URL from RBGC constant to RBRR field — enables per-release and per-incorporation URL customization. Delete RBGC_PUBLIC_DOCS_URL, add RBRR_PUBLIC_DOCS_URL enrollment, add RBRR sourcing to rbho_cli.sh (skipping enforce), update 6 consumer call sites in rbho_onboarding.sh."

### Verification gates

- Regime validates clean (Step 7)
- Fast qualify passes (Step 8)
- Onboarding renders correctly with new URL form (Step 9)
- User confirms browser navigation (Step 10) — **do not notch without this**

## References

- ₣A5 paddock — DDR (whole URL in RBRR, thin-deps relaxation)
- ₢A5AAA resolved answers — Q6 exact enrollment line, Q7 GO verdict
- `Tools/rbk/rbrr_regime.sh` — enrollment target
- `Tools/rbk/rbgc_Constants.sh:80` — deletion target
- `Tools/rbk/rbho_cli.sh` — sourcing update target
- `Tools/rbk/rbho_onboarding.sh` — 6 call sites: lines 320, 341, 442, 564, 786, 904
- `Tools/rbk/vov_veiled/RBSRR-RegimeRepo.adoc` — doc update target
- `.rbk/rbrr.env` — instance update target

**[260409-1227] rough**

## Character

Mechanical, atomic, verification-gated. Adds the new RBRR field, updates the sourcing chain in the onboarding CLI, deletes the RBGC constant, updates the onboarding consumer. No content work, no file retirements. Must land before Pace 3 (README rewrite) because the README rewrite's link verification depends on the new URL form being active.

Cognitive posture: careful execution per ₢A5AAA's resolved answers. No decisions left — all minutiae confirmed.

Silks: `rbrr-docs-url-field`.

## Docket

**Prerequisite**: ₢A5AAA resolved (all Q answers in place). Q6 (field minutiae) and Q7 (kindle tolerance) both return GO.

### Steps

1. **Add `RBRR_PUBLIC_DOCS_URL` enrollment** to `Tools/rbk/rbrr_regime.sh`. Insert new `"Public Docs"` group after the `"Secrets Directory"` group and before `buv_scope_sentinel`. Exact paste:

   ```bash
     buv_group_enroll "Public Docs"
     buv_string_enroll  RBRR_PUBLIC_DOCS_URL          1  512  "Public docs URL — readme target for buh_tlt glossary links, updated per-release or per-incorporation"
   ```

2. **Document the field** in `Tools/rbk/vov_veiled/RBSRR-RegimeRepo.adoc`. Follow the existing RBRR field documentation pattern — single linked term block, mapping section attribute reference, definition site. Read RBSRR first to match the pattern; the snippet is a 5-10 line addition.

3. **Add the field to `.rbk/rbrr.env`** (insert before `# eof`):

   ```bash
   # Public documentation URL — target for buh_tlt glossary links.
   # Update per-release (swap branch name) or per-incorporation (point at your fork).
   RBRR_PUBLIC_DOCS_URL="https://github.com/scaleinv/recipebottle/blob/main/README.md"
   ```

4. **Delete `RBGC_PUBLIC_DOCS_URL`** from `Tools/rbk/rbgc_Constants.sh:80`. Single-line deletion.

5. **Update all 6 consumer call sites in `Tools/rbk/rbho_onboarding.sh`**: lines 320, 341, 442, 564, 786, 904. Each reads `${RBGC_PUBLIC_DOCS_URL}` — change to `${RBRR_PUBLIC_DOCS_URL}`. Most assign to a local `z_docs` once per function. Verify by grep after the edit:

   ```
   grep -n 'RBGC_PUBLIC_DOCS_URL' Tools/rbk/rbho_onboarding.sh
   # Expect: zero hits
   grep -n 'RBRR_PUBLIC_DOCS_URL' Tools/rbk/rbho_onboarding.sh
   # Expect: 6 hits
   ```

6. **Update `Tools/rbk/rbho_cli.sh`** sourcing chain. Before `zrbho_kindle` (current line 50), add RBRR sourcing and kindle call. Do NOT call `zrbrr_enforce` — that's the thin-deps concession per the paddock DDR. The exact source path pattern depends on how rbrr.env is referenced (likely via `RBBC_rbrr_file` constant — check `rbbc_constants.sh` first).

7. **Run regime validation**: `tt/rbw-rrv.ValidateRepoRegime.sh`. Expect success.

8. **Run fast qualification**: `tt/rbw-tf.QualifyFast.sh`. Expect success.

9. **Verify onboarding renders with the new URL form**: `tt/rbw-go.OnboardMAIN.sh`. OSC-8 hyperlinks should now point at `https://github.com/scaleinv/recipebottle/blob/main/README.md#Retriever` (etc.). Inspect raw escape sequences via `tt/rbw-go.OnboardMAIN.sh 2>&1 | cat -v` to confirm.

10. **User verification**: present the command output. **Ask the user to click one link in their terminal and confirm browser navigation lands at the right glossary row.** Agent cannot do this step.

11. **Notch** with intent: "Move public docs URL from RBGC constant to RBRR field — enables per-release and per-incorporation URL customization. Delete RBGC_PUBLIC_DOCS_URL, add RBRR_PUBLIC_DOCS_URL enrollment, add RBRR sourcing to rbho_cli.sh (skipping enforce), update 6 consumer call sites in rbho_onboarding.sh."

### Verification gates

- Regime validates clean (Step 7)
- Fast qualify passes (Step 8)
- Onboarding renders correctly with new URL form (Step 9)
- User confirms browser navigation (Step 10) — **do not notch without this**

## References

- ₣A5 paddock — DDR (whole URL in RBRR, thin-deps relaxation)
- ₢A5AAA resolved answers — Q6 exact enrollment line, Q7 GO verdict
- `Tools/rbk/rbrr_regime.sh` — enrollment target
- `Tools/rbk/rbgc_Constants.sh:80` — deletion target
- `Tools/rbk/rbho_cli.sh` — sourcing update target
- `Tools/rbk/rbho_onboarding.sh` — 6 call sites: lines 320, 341, 442, 564, 786, 904
- `Tools/rbk/vov_veiled/RBSRR-RegimeRepo.adoc` — doc update target
- `.rbk/rbrr.env` — instance update target

**[260409-1219] rough**

## Character

Mechanical, atomic, verification-gated. Adds the new RBRR field, updates the sourcing chain in the onboarding CLI, deletes the RBGC constant, updates the onboarding consumer. No content work, no file retirements. Must land before Pace 3 (README rewrite) because the README rewrite's link verification depends on the new URL form being active.

Cognitive posture: careful execution per ₢A5AAA's resolved answers. No decisions left — all minutiae confirmed.

Silks: `rbrr-docs-url-field`.

## Docket

**Prerequisite**: ₢A5AAA resolved (all Q answers in place). Q6 (field minutiae) and Q7 (kindle tolerance) both return GO.

### Steps

1. **Add `RBRR_PUBLIC_DOCS_URL` enrollment** to `Tools/rbk/rbrr_regime.sh`. Insert new `"Public Docs"` group after the `"Secrets Directory"` group and before `buv_scope_sentinel`. Exact paste:

   ```bash
     buv_group_enroll "Public Docs"
     buv_string_enroll  RBRR_PUBLIC_DOCS_URL          1  512  "Public docs URL — readme target for buh_tlt glossary links, updated per-release or per-incorporation"
   ```

2. **Document the field** in `Tools/rbk/vov_veiled/RBSRR-RegimeRepo.adoc`. Follow the existing RBRR field documentation pattern — single linked term block, mapping section attribute reference, definition site. Read RBSRR first to match the pattern; the snippet is a 5-10 line addition.

3. **Add the field to `.rbk/rbrr.env`** (insert before `# eof`):

   ```bash
   # Public documentation URL — target for buh_tlt glossary links.
   # Update per-release (swap branch name) or per-incorporation (point at your fork).
   RBRR_PUBLIC_DOCS_URL="https://github.com/scaleinv/recipebottle/blob/main/README.md"
   ```

4. **Delete `RBGC_PUBLIC_DOCS_URL`** from `Tools/rbk/rbgc_Constants.sh:80`. Single-line deletion.

5. **Update all 6 consumer call sites in `Tools/rbk/rbho_onboarding.sh`**: lines 320, 341, 442, 564, 786, 904. Each reads `${RBGC_PUBLIC_DOCS_URL}` — change to `${RBRR_PUBLIC_DOCS_URL}`. Most assign to a local `z_docs` once per function. Verify by grep after the edit:

   ```
   grep -n 'RBGC_PUBLIC_DOCS_URL' Tools/rbk/rbho_onboarding.sh
   # Expect: zero hits
   grep -n 'RBRR_PUBLIC_DOCS_URL' Tools/rbk/rbho_onboarding.sh
   # Expect: 6 hits
   ```

6. **Update `Tools/rbk/rbho_cli.sh`** sourcing chain. Before `zrbho_kindle` (current line 50), add RBRR sourcing and kindle call. Do NOT call `zrbrr_enforce` — that's the thin-deps concession per the paddock DDR. The exact source path pattern depends on how rbrr.env is referenced (likely via `RBBC_rbrr_file` constant — check `rbbc_constants.sh` first).

7. **Run regime validation**: `tt/rbw-rrv.ValidateRepoRegime.sh`. Expect success.

8. **Run fast qualification**: `tt/rbw-tf.QualifyFast.sh`. Expect success.

9. **Verify onboarding renders with the new URL form**: `tt/rbw-go.OnboardMAIN.sh`. OSC-8 hyperlinks should now point at `https://github.com/scaleinv/recipebottle/blob/main/README.md#Retriever` (etc.). Inspect raw escape sequences via `tt/rbw-go.OnboardMAIN.sh 2>&1 | cat -v` to confirm.

10. **User verification**: present the command output. **Ask the user to click one link in their terminal and confirm browser navigation lands at the right glossary row.** Agent cannot do this step.

11. **Notch** with intent: "Move public docs URL from RBGC constant to RBRR field — enables per-release and per-incorporation URL customization. Delete RBGC_PUBLIC_DOCS_URL, add RBRR_PUBLIC_DOCS_URL enrollment, add RBRR sourcing to rbho_cli.sh (skipping enforce), update 6 consumer call sites in rbho_onboarding.sh."

### Verification gates

- Regime validates clean (Step 7)
- Fast qualify passes (Step 8)
- Onboarding renders correctly with new URL form (Step 9)
- User confirms browser navigation (Step 10) — **do not notch without this**

## References

- ₣A5 paddock — DDR (whole URL in RBRR, thin-deps relaxation)
- ₢A5AAA resolved answers — Q6 exact enrollment line, Q7 GO verdict
- `Tools/rbk/rbrr_regime.sh` — enrollment target
- `Tools/rbk/rbgc_Constants.sh:80` — deletion target
- `Tools/rbk/rbho_cli.sh` — sourcing update target
- `Tools/rbk/rbho_onboarding.sh` — 6 call sites: lines 320, 341, 442, 564, 786, 904
- `Tools/rbk/vov_veiled/RBSRR-RegimeRepo.adoc` — doc update target
- `.rbk/rbrr.env` — instance update target

**[260409-1053] rough**

## Character

Mechanical, atomic, verification-gated. Adds the new RBRR field, updates the sourcing chain in the onboarding CLI, deletes the RBGC constant, updates the onboarding consumer. No content work, no file retirements. Must land before Pace 3 (README rewrite) because the README rewrite's link verification depends on the new URL form being active.

Cognitive posture: careful execution per Pace 1's spec. No decisions left — all minutiae resolved by Pace 1's Q6 and Q7.

Silks: `rbrr-docs-url-field`.

## Docket

**Prerequisite**: ₢A5AAA complete and its structured note committed. Q6 (RBRR field minutiae) and Q7 (kindle tolerance verification) must have definitive answers. If Q7 returned no-go, this pace reopens for re-planning — do not proceed with the current design.

### Steps

1. **Add `RBRR_PUBLIC_DOCS_URL` enrollment** to `Tools/rbk/rbrr_regime.sh` per Pace 1's Q6 output. Use the exact enrollment line drafted there. Place in the correct `buv_group_enroll` group per Q6's decision.

2. **Document the field** in `Tools/rbk/vov_veiled/RBSRR-RegimeRepo.adoc` per Pace 1's documentation snippet.

3. **Add the field to `.rbk/rbrr.env`** with the default value: `RBRR_PUBLIC_DOCS_URL="https://github.com/scaleinv/recipebottle/blob/main/README.md"`.

4. **Delete `RBGC_PUBLIC_DOCS_URL`** from `Tools/rbk/rbgc_Constants.sh`. One-line deletion.

5. **Update `Tools/rbk/rbho_onboarding.sh`** consumer: wherever `${RBGC_PUBLIC_DOCS_URL}` or `${RBGC_PUBLIC_DOCS_URL}#${z_name}` appears, change to read from `${RBRR_PUBLIC_DOCS_URL}`. Most call sites assign to a local `z_docs` variable once per function — those are the edit points, not the individual `buh_tlt` calls.

6. **Update `Tools/rbk/rbho_cli.sh`** sourcing chain: add RBRR sourcing and kindle call. Before `zrbho_kindle`, insert:
   ```bash
   source "${z_rbk_kit_dir}/rbrr_regime.sh" || buc_die "Failed to source rbrr_regime.sh"
   source "${RBBC_rbrr_file}"                || buc_die "Failed to source rbrr.env"
   zrbrr_kindle
   ```
   Do NOT call `zrbrr_enforce` — that's the thin-deps concession.

7. **Run regime validation**: `tt/rbw-rrv.ValidateRepoRegime.sh`. Expect success — the new field is a plain string with bounds, values from `.rbk/rbrr.env` should pass.

8. **Run fast qualification**: `tt/rbw-tf.QualifyFast.sh`. Expect success — tabtarget structure, colophons, and context are unaffected.

9. **Verify onboarding renders with the new URL form**: `tt/rbw-go.OnboardMAIN.sh`. The OSC-8 hyperlinks in the output should now point at `https://github.com/scaleinv/recipebottle/blob/main/README.md#Retriever` (etc.) instead of the old scaleinv.github.io URL. Inspect raw escape sequences via `tt/rbw-go.OnboardMAIN.sh 2>&1 | cat -v` to confirm.

10. **User verification**: present the command output to the user. **Ask the user to click one link in their terminal and confirm browser navigation lands at the right glossary row.** This step is not agent-executable — Claude cannot click links in a browser.

11. **Notch** with intent describing the plumbing change: "Move public docs URL from RBGC constant to RBRR field — enables per-release and per-incorporation URL customization. Delete RBGC_PUBLIC_DOCS_URL, add RBRR_PUBLIC_DOCS_URL enrollment, add RBRR sourcing to rbho_cli.sh (skipping enforce), update rbho_onboarding.sh consumers."

### Verification gates

- Regime kindles clean (Step 7)
- Fast qualify passes (Step 8)
- Onboarding renders correctly (Step 9)
- User confirms browser navigation (Step 10) — **do not notch without this**

## References

- ₣A5 paddock — DDR
- ₢A5AAA structured note — Q6 (field minutiae), Q7 (kindle tolerance verdict)
- `Tools/rbk/rbrr_regime.sh` — enrollment target
- `Tools/rbk/rbgc_Constants.sh` — deletion target
- `Tools/rbk/rbho_cli.sh` — sourcing update target
- `Tools/rbk/rbho_onboarding.sh` — consumer update target
- `Tools/rbk/vov_veiled/RBSRR-RegimeRepo.adoc` — doc update target
- `.rbk/rbrr.env` — instance update target

**[260409-1023] rough**

## Character

Interactive content rewrite + small code edits + mechanical retirements. Three artifact tracks land together in this pace:

- **Track A (mechanical, opus-level)**: RBRR field enrollment and RBGC URL composition
- **Track B (interactive, sonnet-level with user in the loop per section)**: README rewrite with cosmology port, term renames, and cross-linkification
- **Track C (mechanical, opus-level)**: Retirements (`index.html`, `README.consumer.md`) and RBSPH creation with relocated Significant Events

Depends on ₢A5AAA (cosmology-port-inventory) being complete — that pace produces the structured note this pace executes against. Do not start Track B without Pace 1's term inventory, structural plan, and RBSPH skeleton in hand.

The work mode for Track B is explicitly iterative: user reviews each major README section as it's drafted, before moving to the next. Not a one-shot draft.

## Docket

Carry out the production work specified by ₢A5AAA's structured note. Final output: a clean push to `bhyslop/recipemuster` (origin) that materially advances the docs surface for onboarding link landing.

### Track A — RBRR field + RBGC composition

1. **Add the new field** to `Tools/rbk/rbrr_regime.sh` per Pace 1's enrollment line. Default `main`. Follow the existing enrollment pattern in the regime (validator, scope registration, readonly lock).
2. **Update `.rbk/rbrr.env`** to include the new field with the default value.
3. **Document the field** in `Tools/rbk/vov_veiled/RBSRR-RegimeRepo.adoc` per Pace 1's documentation snippet.
4. **Update `Tools/rbk/rbgc_Constants.sh`** URL composition per Pace 1's snippet. Replace the hard-coded `https://scaleinv.github.io/recipebottle` with the composition from base + `RBRR_PUBLIC_DOCS_REF` + README filename.
5. **Run `tt/rbw-rrv.ValidateRepoRegime.sh`** to confirm the regime kindles cleanly with the new field.
6. **Verify `tt/rbw-go.OnboardMAIN.sh`** renders with the new URL form. Click at least one onboarding link end-to-end and confirm anchor resolution in a browser. The URL should now look like `https://github.com/scaleinv/recipebottle/blob/main/README.md#Payor`.

### Track B — README rewrite (interactive, section-by-section)

Pre-req: Pace 1's structured note is committed and readable.

1. **Front-port intro.** Draft the new intro section per Pace 1's structural plan — third person, newcomer reader, two-complexity-domain framing (Cloud Build + crucible orchestration). Review with user before proceeding.

2. **Embed architecture diagram.** Insert `rbm-abstract-drawio.svg` as a static image reference in the intro. GitHub will render it inline on the repo page. No interactivity (deferred polish).

3. **Cross-linkify ported content.** Every Recipe Bottle term in the new intro gets a `[term](#Term)` markdown link to the glossary anchor. Follow the pattern that `rbho_onboarding.sh` uses heavily. Match the case of the anchor target (capitalized per ₢A3AAH).

4. **Apply term renames** per Pace 1's inventory table. Search-and-replace across the entire README for old terms. Review the diff with user to catch false positives.

5. **Reshape How It Works** per Pace 1's structural decision (replace / fold / keep-and-augment). Review with user.

6. **Integrate project-direction gaps** per Pace 1's list — terse, one-or-two-sentence entries in a dedicated subsection.

7. **Final read-through** with user — check voice consistency, reader orientation, link resolution, cross-reference correctness.

### Track C — Retirement and reorganization

1. **Create `Tools/rbk/vov_veiled/RBSPH-ProjectHistory.adoc`** per Pace 1's skeleton. Header comment establishes purpose (internal project memory). Populate with Significant Events relocated from RBSCO. Preserve commented-out entries as-is. Terse format.

2. **Update RBSCO header comment** per Pace 1's draft — purpose shifts from "index.html source" to "internal narrative memory." Strip the Significant Events section from RBSCO after RBSPH is populated. Preserve all other RBSCO content (Vision, Project Direction, etc.) as internal spec material.

3. **Delete `index.html`** from the repo root.

4. **Delete `Tools/rbk/vov_veiled/README.consumer.md`**.

5. **Handle `CLAUDE.consumer.md`** per Pace 1's audit decision — retire in parallel, or leave alone, or update in place.

6. **Update `.claude/commands/rbk-prep-release.md`** Step 8 per Pace 1's replacement language. README.md is no longer copied from a template; it survives stripping as-is. Remove or replace the consumer template copy logic for both README and CLAUDE (per Pace 1's CLAUDE.consumer.md decision).

7. **Sweep for lingering references** per Pace 1's manifest. Grep for `index.html`, `README.consumer.md`, `CLAUDE.consumer.md` across code, specs, test fixtures. Update or remove each reference.

### Final — Verification and push

1. **Run fast qualification**: `tt/rbw-tf.QualifyFast.sh`. Confirms tabtarget structure, colophon registrations, generated context still pass. No shellcheck, no full test suite — this is a docs-surface change.

2. **Manual smoke**: `tt/rbw-go.OnboardMAIN.sh` and each of the four role tracks (`rbw-gOR`, `rbw-gOD`, `rbw-gOG`, `rbw-gOP`). Each should render with OSC-8 hyperlinks to the new URL form. Click at least one link per track and confirm browser navigation lands at the right glossary row.

3. **Notch with intent** describing the imprint — "first valid imprint of public docs surface: README rewritten with cosmology port, RBRR field pins docs ref, index.html and README.consumer.md retired, Significant Events relocated to RBSPH."

4. **Push to `bhyslop/recipemuster`** (origin). This concludes the heat's work — the PR ceremony from origin to public `scaleinv/recipebottle` is explicitly deferred to a future heat per the paddock's deferred list.

## References

- ₣A5 paddock — DDR, deferred list, decision context
- ₢A5AAA (cosmology-port-inventory) — structured note that drives this pace's execution
- `Tools/rbk/rbho_onboarding.sh` — onboarding code; verification target
- `Tools/rbk/rbgc_Constants.sh` — URL composition site (Track A)
- `Tools/rbk/rbrr_regime.sh` — RBRR enrollment (Track A)
- `Tools/rbk/vov_veiled/RBSRR-RegimeRepo.adoc` — spec for RBRR (Track A)
- `README.md` — rewrite target (Track B)
- `Tools/rbk/vov_veiled/RBSCO-CosmologyIntro.adoc` — source of porting content; header update (Track C)
- `Tools/rbk/vov_veiled/RBSPH-ProjectHistory.adoc` — create in Track C
- `index.html`, `Tools/rbk/vov_veiled/README.consumer.md` — retirement targets (Track C)
- `.claude/commands/rbk-prep-release.md` — ceremony update (Track C)
- `Tools/buk/buh_handbook.sh` — where `buh_tltT` lives; no edits but its URL consumption is the verification path

### readme-rewrite-interactive (₢A5AAC) [complete]

**[260409-1827] complete**

## Character

Content-heavy, interactive, user-in-the-loop per section. The main README rewrite happens here. No code edits, no file deletions.

Cognitive posture: editorial judgment, active collaboration. Each section gets drafted, reviewed, and refined before moving to the next. Not a one-shot draft. User sees each major section as it lands. The scope is intentionally narrow — ₢A5AAA's Q1 resolution eliminates the term-rename sweep that the original paddock anticipated, and Q3/Q5 collapse to "keep it simple, acknowledge two zones, review interactively."

Depends on ₢A5AAA (answers in place) and ₢A5AAB (RBRR field landed, URL form active). The README link verification requires the new URL form to be live.

## Docket

**Prerequisites**:
- ₢A5AAA resolved (inline answers in docket)
- ₢A5AAB complete — RBRR field enrolled, URL form active, onboarding verified with new URLs

Do not start without both.

### Steps

1. **Front-port intro paragraph** after the elevator pitch, before Key Concepts. Third-person voice, newcomer reader, acknowledge two complexity domains (Cloud Build image management + crucible runtime orchestration). Target ~15 lines. Source highlights from RBSCO's Overview and Vision sections, but do not copy wholesale — most of what RBSCO's Vision I/II covers is already in README's existing How It Works.

   **User review point**: present the drafted paragraph. Wait for approval or feedback.

2. **Rename H3** "Bottle Orchestration" → "Crucible Orchestration" in How It Works.

3. **Add 2-sentence bridge** before the two H3s acknowledging the two complexity domains. Suggested: "Recipe Bottle addresses two orthogonal complexity domains: building images with verifiable provenance, and running untrusted images with enforced network isolation. A developer can adopt one without the other."

4. **Light expansion of How It Works H3s** — ~5-10 lines each, porting the most important bits from RBSCO Vision I/II that aren't already covered. Examples: "gcloud never on workstation" for Image Management, "pentacle shares namespace with unmodified bottle" for Crucible Orchestration. Keep it simple; this is the "review later" work per the user's Q3 guidance.

   **User review point**: present the reshaped How It Works. Wait for approval.

5. **Add "Project Direction" subsection** at end of How It Works, using the Q2 draft prose from ₢A5AAA:

   > **Build pipeline.** Egress lockdown is implemented via a dual-pool Cloud Build architecture. VPC Service Controls and cosign signing are evaluated and deferred until organizational policy or external distribution triggers them.
   >
   > **Runtime.** Docker on Linux is the first-class runtime; rootless Podman and macOS-native workflows are deferred pending user demand. One known weakness: when allowed domains are CDN-hosted (e.g. Cloudflare), the sentry's CIDR allowlist becomes coarse — DNS-level gating remains precise, but IP-level gating is porous across shared CDN ranges.
   >
   > **Networking.** Bottle-to-bottle communication is feasible under the current sentry model but not implemented; waiting for a concrete use case.

   **User review point**: present. Wait for approval.

6. **Cross-linkify new content.** Every Recipe Bottle term in the new intro and expanded How It Works gets a `[term](#Term)` markdown link to the glossary anchor. Follow the pattern that `rbho_onboarding.sh` uses heavily (53 call sites). Match the case of the anchor target (capitalized per ₢A3AAH).

   **User review point**: present the cross-linkified content. Wait for approval.

7. **Final read-through** with user — voice consistency, reader orientation, link resolution, cross-reference correctness.

8. **Verify rendered output in browser.** User-executed, not agent-executed. Options:
   - Push to a scratch branch on origin and view on github.com
   - Render locally with a markdown preview tool

   Verify: glossary anchors resolve, cross-links work, architecture diagram renders, structure reads naturally top-to-bottom.

9. **Notch** with intent: "Rewrite README.md intro and How It Works to acknowledge the two complexity domains (Cloud Build + crucible orchestration). Rename Bottle Orchestration → Crucible Orchestration. Add Project Direction subsection naming the CDN/CIDR weakness and the deferred Podman/hairpin items. Cross-linkify new content per the 27-anchor glossary."

### Explicitly NOT in scope

- **Term rename sweep**: ₢A5AAA Q1 found no rename work needed. Do not run a dedicated rename pass.
- **Significant Events removal from README**: README never had a Significant Events section — it lives in RBSCO, not README. Pace 4 handles the RBSCO → RBSPH relocation.
- **Structural reorganization**: per ₢A5AAA Q5, touch nothing except the three insertion points. Do not renumber sections, reorder setup phases, rewrite Credential Safety, etc.
- **SVG re-render**: the `rbm-abstract-drawio.svg` reference stays where it is. ₢A5AAF handles vocabulary review of the SVG later.

### Verification gates

- Each section has a user review checkpoint — do not proceed without explicit approval
- Final browser verification before notching
- Do not auto-wrap this pace — user decides when README is done enough
- Do not push — that's end-of-heat user action

## References

- ₣A5 paddock — DDR, rewrite scope, voice and audience decisions
- ₢A5AAA resolved answers — Q1 (no renames), Q2 (Project Direction prose), Q3 (acknowledge two zones, keep simple), Q5 (three insertion points, ~30 line growth)
- ₢A5AAB — RBRR field and URL plumbing (must be landed first)
- `README.md` — rewrite target (348 lines baseline)
- `Tools/rbk/vov_veiled/RBSCO-CosmologyIntro.adoc` — source highlights for intro (header update deferred to Pace 4)
- `rbm-abstract-drawio.svg` — architecture diagram, already embedded at line 20-22 of current README
- `Tools/rbk/rbho_onboarding.sh` — cross-linkification pattern reference (53 `buh_tlt` call sites)

**[260409-1219] rough**

## Character

Content-heavy, interactive, user-in-the-loop per section. The main README rewrite happens here. No code edits, no file deletions.

Cognitive posture: editorial judgment, active collaboration. Each section gets drafted, reviewed, and refined before moving to the next. Not a one-shot draft. User sees each major section as it lands. The scope is intentionally narrow — ₢A5AAA's Q1 resolution eliminates the term-rename sweep that the original paddock anticipated, and Q3/Q5 collapse to "keep it simple, acknowledge two zones, review interactively."

Depends on ₢A5AAA (answers in place) and ₢A5AAB (RBRR field landed, URL form active). The README link verification requires the new URL form to be live.

## Docket

**Prerequisites**:
- ₢A5AAA resolved (inline answers in docket)
- ₢A5AAB complete — RBRR field enrolled, URL form active, onboarding verified with new URLs

Do not start without both.

### Steps

1. **Front-port intro paragraph** after the elevator pitch, before Key Concepts. Third-person voice, newcomer reader, acknowledge two complexity domains (Cloud Build image management + crucible runtime orchestration). Target ~15 lines. Source highlights from RBSCO's Overview and Vision sections, but do not copy wholesale — most of what RBSCO's Vision I/II covers is already in README's existing How It Works.

   **User review point**: present the drafted paragraph. Wait for approval or feedback.

2. **Rename H3** "Bottle Orchestration" → "Crucible Orchestration" in How It Works.

3. **Add 2-sentence bridge** before the two H3s acknowledging the two complexity domains. Suggested: "Recipe Bottle addresses two orthogonal complexity domains: building images with verifiable provenance, and running untrusted images with enforced network isolation. A developer can adopt one without the other."

4. **Light expansion of How It Works H3s** — ~5-10 lines each, porting the most important bits from RBSCO Vision I/II that aren't already covered. Examples: "gcloud never on workstation" for Image Management, "pentacle shares namespace with unmodified bottle" for Crucible Orchestration. Keep it simple; this is the "review later" work per the user's Q3 guidance.

   **User review point**: present the reshaped How It Works. Wait for approval.

5. **Add "Project Direction" subsection** at end of How It Works, using the Q2 draft prose from ₢A5AAA:

   > **Build pipeline.** Egress lockdown is implemented via a dual-pool Cloud Build architecture. VPC Service Controls and cosign signing are evaluated and deferred until organizational policy or external distribution triggers them.
   >
   > **Runtime.** Docker on Linux is the first-class runtime; rootless Podman and macOS-native workflows are deferred pending user demand. One known weakness: when allowed domains are CDN-hosted (e.g. Cloudflare), the sentry's CIDR allowlist becomes coarse — DNS-level gating remains precise, but IP-level gating is porous across shared CDN ranges.
   >
   > **Networking.** Bottle-to-bottle communication is feasible under the current sentry model but not implemented; waiting for a concrete use case.

   **User review point**: present. Wait for approval.

6. **Cross-linkify new content.** Every Recipe Bottle term in the new intro and expanded How It Works gets a `[term](#Term)` markdown link to the glossary anchor. Follow the pattern that `rbho_onboarding.sh` uses heavily (53 call sites). Match the case of the anchor target (capitalized per ₢A3AAH).

   **User review point**: present the cross-linkified content. Wait for approval.

7. **Final read-through** with user — voice consistency, reader orientation, link resolution, cross-reference correctness.

8. **Verify rendered output in browser.** User-executed, not agent-executed. Options:
   - Push to a scratch branch on origin and view on github.com
   - Render locally with a markdown preview tool

   Verify: glossary anchors resolve, cross-links work, architecture diagram renders, structure reads naturally top-to-bottom.

9. **Notch** with intent: "Rewrite README.md intro and How It Works to acknowledge the two complexity domains (Cloud Build + crucible orchestration). Rename Bottle Orchestration → Crucible Orchestration. Add Project Direction subsection naming the CDN/CIDR weakness and the deferred Podman/hairpin items. Cross-linkify new content per the 27-anchor glossary."

### Explicitly NOT in scope

- **Term rename sweep**: ₢A5AAA Q1 found no rename work needed. Do not run a dedicated rename pass.
- **Significant Events removal from README**: README never had a Significant Events section — it lives in RBSCO, not README. Pace 4 handles the RBSCO → RBSPH relocation.
- **Structural reorganization**: per ₢A5AAA Q5, touch nothing except the three insertion points. Do not renumber sections, reorder setup phases, rewrite Credential Safety, etc.
- **SVG re-render**: the `rbm-abstract-drawio.svg` reference stays where it is. ₢A5AAF handles vocabulary review of the SVG later.

### Verification gates

- Each section has a user review checkpoint — do not proceed without explicit approval
- Final browser verification before notching
- Do not auto-wrap this pace — user decides when README is done enough
- Do not push — that's end-of-heat user action

## References

- ₣A5 paddock — DDR, rewrite scope, voice and audience decisions
- ₢A5AAA resolved answers — Q1 (no renames), Q2 (Project Direction prose), Q3 (acknowledge two zones, keep simple), Q5 (three insertion points, ~30 line growth)
- ₢A5AAB — RBRR field and URL plumbing (must be landed first)
- `README.md` — rewrite target (348 lines baseline)
- `Tools/rbk/vov_veiled/RBSCO-CosmologyIntro.adoc` — source highlights for intro (header update deferred to Pace 4)
- `rbm-abstract-drawio.svg` — architecture diagram, already embedded at line 20-22 of current README
- `Tools/rbk/rbho_onboarding.sh` — cross-linkification pattern reference (53 `buh_tlt` call sites)

**[260409-1054] rough**

## Character

Content-heavy, interactive, user-in-the-loop per section. Sonnet-to-opus level depending on the section. The main README rewrite happens here. No code edits, no file deletions.

Cognitive posture: editorial judgment, active collaboration. Each section gets drafted, reviewed, and refined before moving to the next. Not a one-shot draft. User sees each major section as it lands.

Depends on both Pace 1 (structured note with decisions and drafts) and Pace 2 (RBRR field landed, URL form active) — verification of the rewritten README's links requires the new URL form to be live.

## Docket

**Prerequisites**:

- ₢A5AAA complete — structured note with term inventory (Q1), structural plan (Q3, Q5), gap list (Q2), RBSPH skeleton draft (Q4), and all Pace 1 drafts committed
- ₢A5AAB complete — RBRR field enrolled, URL form active, onboarding verified with new URLs

Do not start without both.

### Steps

1. **Front-port intro section.** Draft the new README intro per Pace 1's Q3 and Q5 outputs — third person voice, newcomer reader, two-complexity-domain framing (Cloud Build + crucible orchestration). Source material is RBSCO's Overview + Vision + Accomplished Infrastructure sections. Port selectively, don't copy wholesale. Target length per Q5.

   **User review point**: present the drafted intro. Wait for approval or feedback before Step 2.

2. **Embed architecture diagram.** Insert `rbm-abstract-drawio.svg` as a static image reference in the intro section via standard markdown image syntax: `![Recipe Bottle Architecture](rbm-abstract-drawio.svg)`. GitHub renders inline on the repo page. No interactivity — clickable internal SVG links is a deferred polish concern.

3. **Cross-linkify ported content.** Every Recipe Bottle term in the new intro gets a `[term](#Term)` markdown link to the glossary anchor. Follow the pattern that `rbho_onboarding.sh` uses heavily in its 53 `buh_tlt` call sites. Match the case of the anchor target (capitalized per ₢A3AAH). Cover the full term inventory from Pace 1's Q1 table.

   **User review point**: present the cross-linkified intro. Wait for approval before Step 4.

4. **Apply term renames** per Pace 1's Q1 inventory table. Search-and-replace across the entire README for old terms. Review the diff with user to catch false positives (e.g., "bottle" as a generic English word vs "bottle" as a Recipe Bottle term — linkification is legit only in the latter case).

   **User review point**: present the diff. Wait for approval before Step 5.

5. **Reshape How It Works** per Pace 1's Q3 decision (replace / fold / keep-and-augment). This may be a large edit or small depending on Q3's outcome.

   **User review point**: present the reshape result. Wait for approval before Step 6.

6. **Integrate project-direction gaps** per Pace 1's Q2 list — terse, one-or-two-sentence entries in a dedicated subsection. Place per Q5's structural plan.

   **User review point**: present the project-direction content. Wait for approval before Step 7.

7. **Final read-through** with user — check voice consistency, reader orientation, link resolution, cross-reference correctness, term uniformity across all sections.

8. **Verify rendered output in browser.** This is user-executed, not agent-executed. Two options for the user:
   - Push to a scratch branch on origin and view the rendered README on github.com
   - Render locally with a markdown preview tool

   Verify:
   - Glossary anchors still resolve correctly
   - Cross-links work (`[vessel](#Vessel)` scrolls to the vessel row)
   - Architecture diagram renders
   - Overall structure reads naturally top-to-bottom

   **User verification**: user clicks through the rendered README in a browser. Agent cannot do this step.

9. **Notch** with intent describing the rewrite scope: "Rewrite README.md as harmonious combo: port cosmology intro from RBSCO, frame around two complexity domains (Cloud Build + crucible orchestration), apply term renames per inventory, cross-linkify all Recipe Bottle terms, integrate project-direction gap summary, embed architecture diagram without interactivity."

### Verification gates

- Each section has a user review checkpoint — do not proceed without explicit approval
- Final browser verification before notching
- Do not auto-wrap this pace — user decides when README is done enough
- Do not push — that's end-of-heat user action

## References

- ₣A5 paddock — DDR, rewrite scope, voice and audience decisions
- ₢A5AAA structured note — inventory (Q1), gap list (Q2), structural plan (Q3, Q5)
- ₢A5AAB — RBRR field and URL plumbing (landed; verification foundation)
- `README.md` — rewrite target
- `Tools/rbk/vov_veiled/RBSCO-CosmologyIntro.adoc` — source of porting content (header update deferred to Pace 4)
- `rbm-abstract-drawio.svg` — architecture diagram to embed
- `Tools/rbk/rbho_onboarding.sh` — cross-linkification pattern reference (53 `buh_tlt` call sites demonstrate the idiom)

### readme-reshape-iterative (₢A5AAI) [complete]

**[260410-0643] complete**

## Character

Iterative prose rewrite. Open-ended — iterate until user signs off. Heavy user-in-the-loop; each pass gets reviewed and the work may loop back to any earlier decision. Not a one-shot draft. Editorial judgment and verb-vocabulary discipline.

## Docket

**Prerequisites**: ₢A5AAC notched.

### Direction

Reshape `README.md` toward pure reference and conceptual orientation. Remove tabtarget commands and numbered setup procedures. Keep — and enrich — prose that teaches the *shape of the work* through linked verb vocabulary. `CLAUDE.md` (customer-facing) becomes the command lookup surface; the interactive onboarding walkthroughs remain the authoritative procedure source.

### Key shifts

- **Delete tabtarget commands from README.** No `tt/rbw-*` references anywhere in the body. Setup Phases 1-4, Day-to-Day Operations, Recovery, Testing all lose their command listings.

- **Replace with verb-vocabulary talk-throughs.** Instead of command listings, prose narrates the conceptual cycles using linked glossary terms. Example direction (not prescription): "The [director](#Director) [ordains](#Ordain) a [hallmark](#Hallmark), which [conjures](#Conjure), [binds](#Bind), or [grafts](#Graft) depending on the [vessel's](#Vessel) mode." The reader learns the shape of each workflow through the verbs. Commands live elsewhere.

- **Convert Key Concepts from table to H3-per-term.** Each glossary entry becomes `### Term` with a ~3-sentence definition: meaning, context, relationship to other terms. The short-form table stops carrying the primary explanation. Definitions themselves cross-link to other terms.

- **Dense cross-linkification.** Every occurrence of a glossary term in prose becomes a link to its anchor — plurals, inflections, possessives included. Display text matches prose case; anchor stays capitalized. Code blocks, headers, and definitional self-references are excluded.

- **Promote Crucible.** Introduce the term earlier and more prominently, framed as *the local safety orchestration*. It currently sits mid-glossary and mid-How-It-Works; it deserves surface weight as the name of the runtime safety apparatus, appearing in the intro paragraphs and ideally in the elevator pitch.

### Iteration

No fixed endpoint. Iterate until the user signs off that README reads as intended. Loop back to any earlier decision at any time. Do not auto-wrap.

### Downstream flags

- **₢A5AAE (retention audit)** was scoped against ₢A5AAC's rewrite only. After this pace, its scope may need to be reslated to cover content lost across both passes.
- **₢A5AAD (retirement + ceremony audit)** touches the prep-release ceremony's Step 8 behavior; if that ceremony references README Setup content that will no longer exist, the step delta in AAD may need to account for this reshape.
- **CLAUDE.md**: confirm which file becomes the customer-facing command reference (likely `Tools/rbk/vov_veiled/CLAUDE.consumer.md` per the heat paddock's "kept and updated" decision) before declaring the command-reference story closed.

## References

- `README.md` — reshape target (current baseline: post-₢A5AAC commit `0adaa6ff`)
- `Tools/rbk/vov_veiled/CLAUDE.consumer.md` — customer Claude template
- `Tools/rbk/rbho_onboarding.sh` — interactive procedure surface
- ₣A5 paddock — link-landing-surface framing

**[260409-1425] rough**

## Character

Iterative prose rewrite. Open-ended — iterate until user signs off. Heavy user-in-the-loop; each pass gets reviewed and the work may loop back to any earlier decision. Not a one-shot draft. Editorial judgment and verb-vocabulary discipline.

## Docket

**Prerequisites**: ₢A5AAC notched.

### Direction

Reshape `README.md` toward pure reference and conceptual orientation. Remove tabtarget commands and numbered setup procedures. Keep — and enrich — prose that teaches the *shape of the work* through linked verb vocabulary. `CLAUDE.md` (customer-facing) becomes the command lookup surface; the interactive onboarding walkthroughs remain the authoritative procedure source.

### Key shifts

- **Delete tabtarget commands from README.** No `tt/rbw-*` references anywhere in the body. Setup Phases 1-4, Day-to-Day Operations, Recovery, Testing all lose their command listings.

- **Replace with verb-vocabulary talk-throughs.** Instead of command listings, prose narrates the conceptual cycles using linked glossary terms. Example direction (not prescription): "The [director](#Director) [ordains](#Ordain) a [hallmark](#Hallmark), which [conjures](#Conjure), [binds](#Bind), or [grafts](#Graft) depending on the [vessel's](#Vessel) mode." The reader learns the shape of each workflow through the verbs. Commands live elsewhere.

- **Convert Key Concepts from table to H3-per-term.** Each glossary entry becomes `### Term` with a ~3-sentence definition: meaning, context, relationship to other terms. The short-form table stops carrying the primary explanation. Definitions themselves cross-link to other terms.

- **Dense cross-linkification.** Every occurrence of a glossary term in prose becomes a link to its anchor — plurals, inflections, possessives included. Display text matches prose case; anchor stays capitalized. Code blocks, headers, and definitional self-references are excluded.

- **Promote Crucible.** Introduce the term earlier and more prominently, framed as *the local safety orchestration*. It currently sits mid-glossary and mid-How-It-Works; it deserves surface weight as the name of the runtime safety apparatus, appearing in the intro paragraphs and ideally in the elevator pitch.

### Iteration

No fixed endpoint. Iterate until the user signs off that README reads as intended. Loop back to any earlier decision at any time. Do not auto-wrap.

### Downstream flags

- **₢A5AAE (retention audit)** was scoped against ₢A5AAC's rewrite only. After this pace, its scope may need to be reslated to cover content lost across both passes.
- **₢A5AAD (retirement + ceremony audit)** touches the prep-release ceremony's Step 8 behavior; if that ceremony references README Setup content that will no longer exist, the step delta in AAD may need to account for this reshape.
- **CLAUDE.md**: confirm which file becomes the customer-facing command reference (likely `Tools/rbk/vov_veiled/CLAUDE.consumer.md` per the heat paddock's "kept and updated" decision) before declaring the command-reference story closed.

## References

- `README.md` — reshape target (current baseline: post-₢A5AAC commit `0adaa6ff`)
- `Tools/rbk/vov_veiled/CLAUDE.consumer.md` — customer Claude template
- `Tools/rbk/rbho_onboarding.sh` — interactive procedure surface
- ₣A5 paddock — link-landing-surface framing

### readme-structure-and-dedup (₢A5AAJ) [complete]

**[260410-0741] complete**

## Character

Iterative structural editing. Same user-in-the-loop posture as ₢A5AAI — iterate until sign-off. Editorial judgment on section titles and content placement.

## Docket

**Prerequisites**: ₢A5AAI wrapped.

### Tasks

1. **Split Key Concepts into two top-level sections.** The current flat glossary mixes build-pipeline terms (Vessel, Hallmark, Vouch, Depot, Ordain, Conjure, Bind, Graft, Kludge, Enshrine, Summon, Plumb, Tally, Levy, Payor, Governor, Director, Retriever, Charter, Knight) with crucible-runtime terms (Nameplate, Sentry, Pentacle, Bottle, Crucible, Charge, Quench). Split into two sections with well-chosen titles. Some terms may belong in both or need judgment calls on placement.

2. **Deduplicate intro content.** The introduction paragraphs (lines 10-28) repeat concepts that would read better as introductions to the two new sections. Remove duplication from the top-level intro; relocate relevant content as section-level introductions for the build and crucible sections.

3. **Remove onboarding references.** The "Onboarding" subsection under Setup and any other onboarding mentions should be removed from this document entirely.

4. **Remove Ark concept.** Delete the `### Ark` definition, its `<a id="Ark"></a>` anchor, and all `[Ark](#Ark)` links throughout the document. Rewrite affected sentences to use Hallmark and Vessel as needed.

### Iteration

No fixed endpoint. Iterate until user signs off. Do not auto-wrap.

**[260410-0643] rough**

## Character

Iterative structural editing. Same user-in-the-loop posture as ₢A5AAI — iterate until sign-off. Editorial judgment on section titles and content placement.

## Docket

**Prerequisites**: ₢A5AAI wrapped.

### Tasks

1. **Split Key Concepts into two top-level sections.** The current flat glossary mixes build-pipeline terms (Vessel, Hallmark, Vouch, Depot, Ordain, Conjure, Bind, Graft, Kludge, Enshrine, Summon, Plumb, Tally, Levy, Payor, Governor, Director, Retriever, Charter, Knight) with crucible-runtime terms (Nameplate, Sentry, Pentacle, Bottle, Crucible, Charge, Quench). Split into two sections with well-chosen titles. Some terms may belong in both or need judgment calls on placement.

2. **Deduplicate intro content.** The introduction paragraphs (lines 10-28) repeat concepts that would read better as introductions to the two new sections. Remove duplication from the top-level intro; relocate relevant content as section-level introductions for the build and crucible sections.

3. **Remove onboarding references.** The "Onboarding" subsection under Setup and any other onboarding mentions should be removed from this document entirely.

4. **Remove Ark concept.** Delete the `### Ark` definition, its `<a id="Ark"></a>` anchor, and all `[Ark](#Ark)` links throughout the document. Rewrite affected sentences to use Hallmark and Vessel as needed.

### Iteration

No fixed endpoint. Iterate until user signs off. Do not auto-wrap.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 A cosmology-port-inventory
  2 B rbrr-docs-url-field
  3 C readme-rewrite-interactive
  4 I readme-reshape-iterative
  5 J readme-structure-and-dedup

ABCIJ
··xxx README.md
···x· memo-20260409-prompt-base64-haiku.md, memo-20260409-prompt-base64-opus.md, memo-20260409-prompt-base64-sonnet.md, memo-20260409-system-prompt-capture-haiku.md, memo-20260409-system-prompt-capture-opus.md, memo-20260409-system-prompt-capture-sonnet.md
··x·· rbm-abstract-drawio.svg
·x··· RBS0-SpecTop.adoc, RBSRR-RegimeRepo.adoc, rbgc_Constants.sh, rbho_cli.sh, rbrr.env, rbrr_regime.sh

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 39 commits)

  1 A cosmology-port-inventory
  2 B rbrr-docs-url-field
  3 C readme-rewrite-interactive
  4 I readme-reshape-iterative
  5 D unknown
  6 J readme-structure-and-dedup

123456789abcdefghijklmnopqrstuvwxyz
···········x·······················  A  1c
··············xx···················  B  2c
················xxx··x·············  C  4c
······················xxxx·xx······  I  6c
·····························x·····  D  1c
·······························xxxx  J  4c
```

## Steeplechase

### 2026-04-10 07:41 - ₢A5AAJ - W

Restructured README.md: split Key Concepts into Supply Chain (20 terms) and Crucible (7 terms) sections with domain-specific intro paragraphs, removed Ark concept and rewrote references, removed Onboarding subsection, deduplicated intro content, rewrote elevator pitch with full supply-chain range (three modes), rewrote Important blockquote as security-mechanism summary (Bottle unprivileged in Pentacle namespace, dual-layer egress, SLSA/digest-pinned/mirrored base images), trimmed project description to narrow-footprint principle

### 2026-04-10 07:40 - ₢A5AAJ - n

Trim intro paragraph to principle (narrow bash 3.2 footprint, no heavy CLIs) — move specific tool enumeration and incorporation methods out of top-level framing, details already live in Prerequisites and How It Works

### 2026-04-10 07:36 - ₢A5AAJ - n

Restructure document opening: move elevator pitch before Important blockquote with full supply-chain range (three modes), rewrite Important as security-mechanism-only summary (SLSA/digest-pinned/mirrored base images, Bottle unprivileged in Pentacle namespace, dual-layer egress), eliminate duplication between intro and callout

### 2026-04-10 07:19 - ₢A5AAJ - n

Split Key Concepts into Supply Chain (20 terms) and Crucible (7 terms) sections with relocated intro paragraphs, remove Ark concept and rewrite references, remove Onboarding subsection, deduplicate intro content into section-level introductions

### 2026-04-10 07:13 - Heat - r

moved A5AAJ to first

### 2026-04-10 07:12 - ₢A5AAD - n

Tracks D1-D3 partial: create RBSPH-ProjectHistory.adoc with relocated Significant Events, rewrite RBSCO header as first-class prose and strip timeline, delete index.html/.nojekyll/README.consumer.md, fix CLAUDE.consumer.md stale tabtarget (rbw-gO→rbw-go)

### 2026-04-10 06:43 - ₢A5AAI - W

Reshaped README.md from command-reference to conceptual-orientation document. Stripped all tt/rbw-* tabtarget references (370→255 lines), replacing Setup Phases 1-4, Day-to-Day Operations, Testing, Recovery, Credential Safety, and Configuration with verb-vocabulary prose narratives. Converted 27-row Key Concepts table to H3-per-term glossary with ~3-sentence cross-linked definitions. Capitalized all link display text as house style ([Vessel](#Vessel) not [vessel](#Vessel)). Dense cross-linkification pass: 19 bare glossary terms in prose linked. Promoted Crucible into elevator pitch, blockquote, and runtime paragraph (first appearance moved from line 24 to line 6). Four notches across the pace.

### 2026-04-10 06:43 - ₢A5AAI - n

System prompt capture experiment: base64 refusal memos (haiku/sonnet/opus) and direct system prompt captures across all three tiers, documenting refusal character differences and effective prompt structure

### 2026-04-10 06:43 - Heat - S

readme-structure-and-dedup

### 2026-04-10 06:43 - ₢A5AAI - n

Promote Crucible into intro paragraphs: surface the term at lines 6 (blockquote), 10 (elevator pitch), and 14 (runtime paragraph) — up from first appearance at line 24. Framed as the local safety orchestration per docket.

### 2026-04-10 06:33 - ₢A5AAI - n

Dense cross-linkification: linkify all 19 remaining bare glossary terms in prose outside code blocks and headers. Covers intro blockquote (pentacle/sentry/bottle), intro paragraph (sentry, crucible), How It Works bullet labels and prose (sentry/pentacle/bottle/crucible), Project Direction (sentry/bottle), Setup narrative (payor x4, director), Build and Retrieve (vouched, tally), Credential Safety (depot), Configuration (vessel), Vessels section (Conjure/Bind/Graft vessels, bold Vessels). All link display text capitalized per house style.

### 2026-04-10 06:21 - ₢A5AAI - n

Convert Key Concepts from 27-row table to H3-per-term glossary with ~3-sentence definitions cross-linked to related terms. Capitalize all link display text throughout README (e.g. [Vessel](#Vessel) not [vessel](#Vessel)) so Recipe Bottle terms read as proper nouns when linked.

### 2026-04-10 06:06 - ₢A5AAI - n

Strip all tt/rbw-* tabtarget command references from README.md (370→255 lines). Replace Setup Phases 1-4, Day-to-Day Operations, Testing, Recovery, Credential Safety, and Configuration sections with verb-vocabulary prose narratives using linked glossary terms. Commands now live in CLAUDE.md and onboarding walkthroughs; README teaches the shape of the work through verbs (levy, ordain, conjure, bind, graft, charge, quench, plumb, tally, vouch, summon, knight, charter).

### 2026-04-09 18:27 - ₢A5AAC - W

Rewrote README.md intro and How It Works to acknowledge the two complexity domains (Cloud Build image management + crucible orchestration). Renamed Bottle Orchestration → Crucible Orchestration. Added Project Direction subsection naming the CDN/CIDR weakness and deferred items. Cross-linkified new content against the 27-anchor glossary. Added Crucible glossary row (notch chain: 28fbf579 README rewrite → aa813b95 Crucible row + openssl → 0adaa6ff SVG restoration from scaleinv/recipebottle).

### 2026-04-09 14:25 - Heat - S

readme-reshape-iterative

### 2026-04-09 13:54 - Heat - S

svg-content-audit

### 2026-04-09 13:48 - ₢A5AAC - n

Restore richer architecture diagram from scaleinv/recipebottle (48,452 bytes) over the smaller bhyslop copy (37,033 bytes). History diagnostic revealed divergent evolution from shared ancestor 3ea89905 (2024-12-19 'first reviewer feedback'): scaleinv side got two 2025-07-29 commits (e19e60be 'CENSER container addition' and 3ebe4d5e 'Revised top diagram') that added the build-pipeline panel, while bhyslop side only got 4f3de046 (2026-03-31 AxAAE 'bottle service -> crucible' rename sweep) applied to the old base without pulling the July 2025 content additions. Result: bhyslop had newer labels but a truncated diagram missing the Workbench Console / Image Build Service / OCI Container Repository / Podman VM Images panel. This restore recovers the missing content at the cost of re-introducing stale vocabulary (Bottle Service, Censer, Podman VM, github action) that was swept from other surfaces during the crucible rename. Two follow-up paces anticipated on this heat: (1) vocabulary rename directive list on the restored SVG (Bottle Service -> Crucible, Censer -> Pentacle, Podman VM -> Docker, github action -> Cloud Build, OCI Container Repository -> Google Artifact Registry) that the user will execute in drawio, subsuming what A5AAF was originally scoped for; (2) content merge audit to determine whether the restored diagram needs further additions (depot, vessel modes, nameplate-as-policy) to match current architecture. Per heat paddock hard boundary Claude did not edit the drawio source; this is a file-level restoration of a previously-committed version from a sibling remote, approved by user. Size limit raised to 100000 on this call only (staged content 85,518 bytes reflects diff payload of old+new SVG bytes) per explicit user approval.

### 2026-04-09 13:33 - ₢A5AAC - n

Close the two gaps flagged in the prior notch: add Crucible glossary row (positioned between Bottle and Charge; cross-linked to sentry/pentacle/bottle/nameplate per A3AAH capitalization convention) so the load-bearing term now has a definition anchor resolvable by all the cross-links in the rewritten intro and How It Works. Add 'openssl' to both the line 16 native-tools statement and the Prerequisites list, aligning README with BCG's declared-dependency discipline (openssl replaces base64/sha256sum as the single portable binary across GNU/BSD platforms). Charge/Quench rows intentionally left unchanged — tightening their 'triad' language to reference 'crucible' would be a polish pass beyond scope.

### 2026-04-09 13:28 - ₢A5AAC - n

Rewrite README.md intro and How It Works to acknowledge the two complexity domains (image management + crucible orchestration). Front-port ~30-line intro after the SVG covering: airgap Cloud Build pipeline, upstream base image retention via enshrine, distinctive 'running untrusted code' use case, and the narrow bash-3.2 workstation floor (no python runtime, no gcloud on workstation, openssl as declared dep per BCG). Rename Bottle Orchestration H3 → Crucible Orchestration; add bridge sentence under How It Works; expand both H3s with gcloud-never-on-workstation bullet + enshrined base images bullet for Image Management, and layered dnsmasq/iptables egress policy prose for Crucible Orchestration. Add Project Direction subsection with Q2 prose naming CDN/CIDR weakness and deferred Podman/macOS/bottle-hairpin items. Cross-linkify new content per the 27-anchor glossary (bottle/pentacle/sentry/enshrine/depot/nameplate/vessel). Two gaps flagged for follow-up paces: 'crucible' lacks a glossary anchor despite becoming load-bearing in the rewritten text, and Prerequisites section omits openssl even though BCG names it as the declared dependency replacing base64/sha256sum.

### 2026-04-09 13:05 - ₢A5AAB - W

Moved public docs URL from RBGC constant to RBRR field. Added RBRR_PUBLIC_DOCS_URL (1-512, new Public Docs group) in rbrr_regime.sh, deleted RBGC_PUBLIC_DOCS_URL from rbgc_Constants.sh, widened rbho_cli.sh thin-deps to source rbrr_regime + rbrr.env and kindle buv+rbrr (no enforce, per DDR). Documented field in RBSRR (new group block) and RBS0 (mapping + field definition + group definition). Initial value pinned to bhyslop/recipemuster origin README to match current ground truth; click-test verified after push. The 6→8 call-site swap in rbho_onboarding.sh was absorbed into commit 9844ce32 via concurrent officium activity on ₣A6. Docs track ran in parallel via background agent; all verification gates (regime validate, fast qualify, OnboardMAIN render, user click-test) passed.

### 2026-04-09 13:00 - ₢A5AAB - n

Move public docs URL from RBGC constant to RBRR field — enables per-release and per-incorporation URL customization. Add RBRR_PUBLIC_DOCS_URL enrollment under new Public Docs group in rbrr_regime.sh, delete RBGC_PUBLIC_DOCS_URL from rbgc_Constants.sh, add RBRR sourcing + kindle (not enforce, per thin-deps DDR) to rbho_cli.sh, document the field in RBSRR and RBS0 (field + group, with mapping entries and definition blocks). Initial value set to bhyslop/recipemuster origin README to match current ground truth (click-test verified after push). Note: rbho_onboarding.sh RBGC→RBRR call-site swap was absorbed into commit 9844ce32 via concurrent officium activity.

### 2026-04-09 12:30 - Heat - d

paddock curried: acknowledge AAE retention audit and AAF SVG review as safety paces outside core DDR (per A5 groom)

### 2026-04-09 12:27 - Heat - T

rbrr-docs-url-field

### 2026-04-09 12:27 - ₢A5AAA - W

Resolved Q1-Q11 through live conversation review in officium ☉260409-1007 and baked answers directly into downstream pace dockets AAB/AAC/AAD. Eliminated phantom term-rename scope (Q1), drafted Project Direction prose (Q2), settled two-zone framing (Q3), produced RBSPH format and header (Q4), confirmed README insertion points (Q5), pinned RBRR field minutiae (Q6), verified kindle tolerance GO (Q7), kept CLAUDE.consumer.md as update-in-place (Q8), produced full prep-release ceremony delta (Q9), drafted RBSCO header rewrite (Q10), enumerated lingering reference sweep targets (Q11). No separate memo produced — the pace dockets ARE the deliverable.

### 2026-04-09 12:15 - Heat - d

paddock curried: currying after A5AAA live resolution — corrections to damaged framings

### 2026-04-09 11:38 - Heat - n

tighten axvr_* operation voicings to dedicated-quoin form and strip misapplied kindle/validate/render from RBRR variables

### 2026-04-09 11:19 - Heat - T

handbook-split-dangling-refs-audit

### 2026-04-09 11:18 - Heat - D

restring 1 paces from ₣A3

### 2026-04-09 11:00 - Heat - S

drawio-svg-vocabulary-review

### 2026-04-09 10:59 - Heat - S

retired-files-retention-audit

### 2026-04-09 10:55 - Heat - S

retirements-rbsph-ceremony-sweep

### 2026-04-09 10:54 - Heat - S

readme-rewrite-interactive

### 2026-04-09 10:51 - Heat - d

paddock curried: rewrite per self-review: whole URL in RBRR, silks refresh, four-pace split

### 2026-04-09 10:49 - Heat - f

racing, silks=rbk-mvp-3-public-docs-refresh

### 2026-04-09 10:37 - Heat - f

racing

### 2026-04-09 10:23 - Heat - S

readme-rewrite-rbrr-pinning-and-retirement

### 2026-04-09 10:22 - Heat - S

cosmology-port-inventory

### 2026-04-09 10:21 - Heat - d

paddock curried: initial paddock from design chat

### 2026-04-09 10:19 - Heat - N

rbk-mvp-3-public-docs-first-imprint

