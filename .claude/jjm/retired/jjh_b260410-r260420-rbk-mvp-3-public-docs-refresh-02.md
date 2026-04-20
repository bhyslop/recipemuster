# Heat Trophy: rbk-mvp-3-public-docs-refresh-02

**Firemark:** ₣A8
**Created:** 260410
**Retired:** 260420
**Status:** retired

## Paddock

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

### foundry-colophon-restore (₢A8AAK) [complete]

**[260412-1421] complete**

## Character

Mechanical rename with a Rust rebuild gate. Every change is a string substitution — no design judgment, no new code. But the blast radius touches zipper enrollments, onboarding display, theurge Rust constants, spec docs, and tabtarget filenames, so it needs careful execution and a test pass to confirm nothing broke.

## Docket

Restore `rbw-f` as the Foundry artifact operations colophon, vacating `rbw-h` for exclusive Handbook use. This resolves a terminal exclusivity violation where `rbw-h<TAB>` mixes Hallmark operations with Handbook pages.

**History**: `rbw-f` was originally Foundry (`rbw-fA` Abjure, `rbw-fB` Build, `rbw-fD` Delete) until ₣Av scattered them to `rbw-a`/`rbw-i`/`rbw-h`. This pace brings Foundry ops home.

### Colophon mapping

| Old | New | Frontispiece |
|-----|-----|-------------|
| `rbw-hO` | `rbw-fO` | DirectorOrdainsHallmark |
| `rbw-hk` | `rbw-fk` | LocalKludge |
| `rbw-hA` | `rbw-fA` | DirectorAbjuresHallmark |
| `rbw-ht` | `rbw-ft` | DirectorTalliesHallmarks |
| `rbw-hV` | `rbw-fV` | DirectorVouchesHallmarks |
| `rbw-hs` | `rbw-fs` | RetrieverSummonsHallmark |
| `rbw-hpf` | `rbw-fpf` | RetrieverPlumbsFull |
| `rbw-hpc` | `rbw-fpc` | RetrieverPlumbsCompact |

### Steps

1. **Rename 8 tabtarget files** in `tt/` (old → new per mapping above)
2. **Update `Tools/rbk/rbz_zipper.sh`** — 8 `buz_enroll` colophon strings
3. **Update `Tools/rbk/rbho_onboarding.sh`** — 14 `buh_tc` tabtarget filename references
4. **Update `Tools/rbk/rbtd/src/rbtdrm_manifest.rs`** — 4 Rust colophon constants
5. **Update `Tools/rbk/rbw_workbench.sh`** — 1 comment
6. **Update `Tools/rbk/rbfc_FoundryCore.sh`** — 1 comment
7. **Update `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`** — 2 linked term colophon refs
8. **Update `Tools/rbk/vov_veiled/CLAUDE.consumer.md`** — 1 troubleshooting reference
9. **Update `Tools/rbk/vov_veiled/RBSAK-ark_kludge.adoc`** — 1 spec reference
10. **Rebuild theurge**: `tt/rbtd-b.Build.sh`
11. **Regenerate tabtarget context**: `tt/rbw-MG.MarshalGenerate.sh`
12. **Run fast qualify**: `tt/rbw-tf.QualifyFast.sh`
13. **Verify** `tt/rbw-h<TAB>` shows only Handbook targets, `tt/rbw-f<TAB>` shows only Foundry targets
14. **Notch** with full before/after colophon inventory

### Verification gates

- Fast qualify passes
- Theurge builds clean
- Zero references to old `rbw-h{O,k,A,t,V,s,pf,pc}` colophons remain in live code (grep confirm)
- `rbw-h*` contains only Handbook targets
- `rbw-f*` contains only Foundry targets

**[260412-1154] rough**

## Character

Mechanical rename with a Rust rebuild gate. Every change is a string substitution — no design judgment, no new code. But the blast radius touches zipper enrollments, onboarding display, theurge Rust constants, spec docs, and tabtarget filenames, so it needs careful execution and a test pass to confirm nothing broke.

## Docket

Restore `rbw-f` as the Foundry artifact operations colophon, vacating `rbw-h` for exclusive Handbook use. This resolves a terminal exclusivity violation where `rbw-h<TAB>` mixes Hallmark operations with Handbook pages.

**History**: `rbw-f` was originally Foundry (`rbw-fA` Abjure, `rbw-fB` Build, `rbw-fD` Delete) until ₣Av scattered them to `rbw-a`/`rbw-i`/`rbw-h`. This pace brings Foundry ops home.

### Colophon mapping

| Old | New | Frontispiece |
|-----|-----|-------------|
| `rbw-hO` | `rbw-fO` | DirectorOrdainsHallmark |
| `rbw-hk` | `rbw-fk` | LocalKludge |
| `rbw-hA` | `rbw-fA` | DirectorAbjuresHallmark |
| `rbw-ht` | `rbw-ft` | DirectorTalliesHallmarks |
| `rbw-hV` | `rbw-fV` | DirectorVouchesHallmarks |
| `rbw-hs` | `rbw-fs` | RetrieverSummonsHallmark |
| `rbw-hpf` | `rbw-fpf` | RetrieverPlumbsFull |
| `rbw-hpc` | `rbw-fpc` | RetrieverPlumbsCompact |

### Steps

1. **Rename 8 tabtarget files** in `tt/` (old → new per mapping above)
2. **Update `Tools/rbk/rbz_zipper.sh`** — 8 `buz_enroll` colophon strings
3. **Update `Tools/rbk/rbho_onboarding.sh`** — 14 `buh_tc` tabtarget filename references
4. **Update `Tools/rbk/rbtd/src/rbtdrm_manifest.rs`** — 4 Rust colophon constants
5. **Update `Tools/rbk/rbw_workbench.sh`** — 1 comment
6. **Update `Tools/rbk/rbfc_FoundryCore.sh`** — 1 comment
7. **Update `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`** — 2 linked term colophon refs
8. **Update `Tools/rbk/vov_veiled/CLAUDE.consumer.md`** — 1 troubleshooting reference
9. **Update `Tools/rbk/vov_veiled/RBSAK-ark_kludge.adoc`** — 1 spec reference
10. **Rebuild theurge**: `tt/rbtd-b.Build.sh`
11. **Regenerate tabtarget context**: `tt/rbw-MG.MarshalGenerate.sh`
12. **Run fast qualify**: `tt/rbw-tf.QualifyFast.sh`
13. **Verify** `tt/rbw-h<TAB>` shows only Handbook targets, `tt/rbw-f<TAB>` shows only Foundry targets
14. **Notch** with full before/after colophon inventory

### Verification gates

- Fast qualify passes
- Theurge builds clean
- Zero references to old `rbw-h{O,k,A,t,V,s,pf,pc}` colophons remain in live code (grep confirm)
- `rbw-h*` contains only Handbook targets
- `rbw-f*` contains only Foundry targets

### foundry-quoin-elevation (₢A8AAH) [complete]

**[260410-0917] complete**

## Character

Naming decision with spec-layer consequences. Design judgment on the quoin definition, then mechanical propagation across documents. The decision itself (Foundry as peer to Crucible) is settled; this pace is execution.

## Docket

Elevate "Foundry" from code module name (`rbf_Foundry.sh`) to top-level concept — the umbrella term for Recipe Bottle's remote build orchestration system, peer to Crucible.

### Tasks

1. **Mint the quoin in RBS0.** Add mapping section entry, anchor, and definition for Foundry as the remote build orchestration apparatus. Category prefix TBD — likely `at_` (Core Architecture Terms) alongside `at_build_service`. Definition should name what Foundry encompasses: Depots, Vessels, Hallmarks, Rubrics, roles, egress-locked Cloud Build, SLSA provenance.

2. **Update README.md.** Rename `## Supply Chain` → `## Foundry`. Add `### Foundry` glossary entry with anchor. Update the elevator pitch bullets to name Foundry. Update the Important blockquote if it references "supply chain" generically.

3. **Audit RBSCO and other vov_veiled docs** for generic references to the build system that should now say Foundry. Check RBSHR, RBSGS, any spec that describes the build domain as a whole.

4. **Verify cross-links.** Any `[Supply Chain]` or similar links in README should become `[Foundry](#Foundry)`.

5. **Notch and present for review.** Do not auto-wrap.

**[260410-0809] rough**

## Character

Naming decision with spec-layer consequences. Design judgment on the quoin definition, then mechanical propagation across documents. The decision itself (Foundry as peer to Crucible) is settled; this pace is execution.

## Docket

Elevate "Foundry" from code module name (`rbf_Foundry.sh`) to top-level concept — the umbrella term for Recipe Bottle's remote build orchestration system, peer to Crucible.

### Tasks

1. **Mint the quoin in RBS0.** Add mapping section entry, anchor, and definition for Foundry as the remote build orchestration apparatus. Category prefix TBD — likely `at_` (Core Architecture Terms) alongside `at_build_service`. Definition should name what Foundry encompasses: Depots, Vessels, Hallmarks, Rubrics, roles, egress-locked Cloud Build, SLSA provenance.

2. **Update README.md.** Rename `## Supply Chain` → `## Foundry`. Add `### Foundry` glossary entry with anchor. Update the elevator pitch bullets to name Foundry. Update the Important blockquote if it references "supply chain" generically.

3. **Audit RBSCO and other vov_veiled docs** for generic references to the build system that should now say Foundry. Check RBSHR, RBSGS, any spec that describes the build domain as a whole.

4. **Verify cross-links.** Any `[Supply Chain]` or similar links in README should become `[Foundry](#Foundry)`.

5. **Notch and present for review.** Do not auto-wrap.

### svg-content-audit (₢A8AAC) [complete]

**[260412-1046] complete**

Drafted from ₢A5AAH in ₣A5.

## Character

Design review, non-destructive. Agent reads the restored SVG and the current project state, then produces a judgment about whether the diagram's scope matches current architecture. Output is a memo classifying each candidate concept as present / absent-should-add / absent-correctly-out-of-scope, optionally followed by a directive list for content additions the user would execute in drawio.

Cognitive posture: architectural judgment paired with mechanical inventory. This is the "consider both and merge them appropriately" half of the user's ₢A5AAC-follow-up instruction — the restored richer SVG is one input, the current project architecture is the other, and the merge decisions live in this pace's memo.

Hard boundary inherited from heat paddock: agent does not edit the drawio source or the SVG file. If additions are warranted, agent produces directives; user performs drawio edits.

## Docket

**Prerequisites**: ₢A5AAC notched (rough is fine). The restored SVG (commit `0adaa6ff`, 48,452 bytes) is the input.

**Ordering**: This pace runs BEFORE ₢A5AAF (vocabulary rename). Rationale: if the audit finds content additions are warranted, the user executes one drawio session covering both new content AND vocabulary updates on the new content, rather than two round trips. ₢A5AAF's re-extraction step will pick up whatever labels exist post-audit.

### Context

₢A5AAC restored the richer `scaleinv/recipebottle` version of `rbm-abstract-drawio.svg`, recovering a build-pipeline panel (Workbench Console, Image Build Service, OCI Container Repository, Podman VM Images, volume mounts) that the truncated bhyslop version had lost. Sibling pace ₢A5AAF handles label vocabulary rename.

This pace asks the separate structural question: does the restored diagram's **scope** match current project architecture, or are there concepts that should be depicted but aren't? The diagnostic from ₢A5AAC history (shared ancestor `3ea89905` Dec 2024, scaleinv evolution July 2025, bhyslop rename March 2026) establishes that the richest content is from July 2025 — nine months stale on structural evolution even if we rename every label.

### Concepts to check for representation

Drawn from current README glossary (post-A5AAC), RBSCO narrative, and known architecture:

| Concept | Source | Expected relevance |
|---|---|---|
| **Depot** | README glossary: "GCP project + registry + bucket" | The diagram shows a "Container Repository" — is its containment within a depot concept visible? |
| **Vessel modes (conjure/bind/graft)** | README glossary + elaboration | Three distinct arrival paths for artifacts. Does the build-pipeline panel generalize or distinguish them? |
| **Hallmark** | README glossary: "specific build instance identified by timestamp" | Does the diagram show the hallmark-vs-vessel distinction? |
| **Nameplate as policy declaration** | README expanded How It Works (post-A5AAC): "Each crucible's allowlist is declared in a nameplate regime file" | Is the nameplate concept represented? |
| **SLSA provenance / vouch chain** | README How It Works + glossary | The supply chain attestation — depicted or implicit? |
| **Enshrine (base image mirror)** | README glossary + Image Management bullets (post-A5AAC) | Upstream base image mirroring to depot registry — depicted or absent? |
| **Role separation (payor/governor/director/retriever)** | README Setup section | Are any roles shown, or is this entirely out of scope for the architecture diagram? |
| **Cloud Build dual-pool / egress lockdown** | README Project Direction (post-A5AAC) | The airgap build architecture — depicted or too fine-grained? |
| **Pentacle as namespace establisher** | README Key Concepts | Present in the restored diagram as "Censer Container" (to be renamed by ₢A5AAF) — confirm the concept is depicted, not just the label |

### Steps

1. **Inventory the restored SVG's scope** — extract all text labels and identify what concepts are explicitly depicted. Build an "is-present" list.

2. **Walk the candidate-concept checklist** above. For each: classify as **present** (with reference to which shape/label represents it), **absent-and-should-be-added** (with rationale), or **absent-and-correctly-out-of-scope** (with rationale — e.g., "diagram scope is architecture, not process; roles belong in a separate diagram").

3. **Produce an audit memo** at `Memos/memo-svg-content-audit.md` capturing the full walkthrough. Include: pre-audit inventory, concept-by-concept decisions, and a list of recommended additions (if any) with directive-list-ready specificity (shape placement hints, connector routing hints, label text).

4. **Present memo to user for review**.

5. **User decides**: execute the recommended additions, defer some, decline others. Agent updates the memo to reflect the decision.

6. **If additions are approved for execution**: user performs drawio edits per the directive list in the memo; agent re-inventories post-edit to confirm the new concepts are present.

7. **Notch** with intent describing the audit outcome and any executed additions. If user chose to defer all additions, notch with the memo alone and track the deferred items as itches in `jji_itch.md`.

### Explicitly NOT in scope

- **Label renaming** — sibling pace ₢A5AAF handles that. This pace does not alter existing labels even when discussing them in the memo.
- **Visual style / layout / color choices** — those are user composition decisions, never agent decisions.
- **Rewriting the diagram from scratch** — audit, don't rebuild.
- **Choosing what the diagram *should* communicate to a newcomer** — the audit checks against a concrete concept checklist, not against a generalized "is this diagram good" judgment.

### Verification gates

- Memo includes explicit rationale for every concept decision (present / add / out-of-scope)
- If additions are executed, post-edit inventory confirms each added concept is depicted
- User explicitly signs off on which additions to execute vs defer
- Do not auto-wrap — user decides when audit is closed

## References

- Restored SVG: `rbm-abstract-drawio.svg` (48,452 bytes, commit `0adaa6ff` from `scaleinv/recipebottle` main)
- ₢A5AAC notch chain: `28fbf579`, `aa813b95`, `0adaa6ff`
- Sibling pace: ₢A5AAF (SVG vocabulary rename, reslated with restoration context)
- `README.md` post-A5AAC — authoritative source for current architecture concepts (glossary, How It Works, Project Direction)
- `Tools/rbk/vov_veiled/RBSCO-CosmologyIntro.adoc` — narrative source for cosmology context
- ₣A5 paddock — divergent-history diagnostic, "consider both and merge them appropriately" framing
- Heat paddock hard boundary: "Claude does not touch the SVG source"

**[260410-0742] rough**

Drafted from ₢A5AAH in ₣A5.

## Character

Design review, non-destructive. Agent reads the restored SVG and the current project state, then produces a judgment about whether the diagram's scope matches current architecture. Output is a memo classifying each candidate concept as present / absent-should-add / absent-correctly-out-of-scope, optionally followed by a directive list for content additions the user would execute in drawio.

Cognitive posture: architectural judgment paired with mechanical inventory. This is the "consider both and merge them appropriately" half of the user's ₢A5AAC-follow-up instruction — the restored richer SVG is one input, the current project architecture is the other, and the merge decisions live in this pace's memo.

Hard boundary inherited from heat paddock: agent does not edit the drawio source or the SVG file. If additions are warranted, agent produces directives; user performs drawio edits.

## Docket

**Prerequisites**: ₢A5AAC notched (rough is fine). The restored SVG (commit `0adaa6ff`, 48,452 bytes) is the input.

**Ordering**: This pace runs BEFORE ₢A5AAF (vocabulary rename). Rationale: if the audit finds content additions are warranted, the user executes one drawio session covering both new content AND vocabulary updates on the new content, rather than two round trips. ₢A5AAF's re-extraction step will pick up whatever labels exist post-audit.

### Context

₢A5AAC restored the richer `scaleinv/recipebottle` version of `rbm-abstract-drawio.svg`, recovering a build-pipeline panel (Workbench Console, Image Build Service, OCI Container Repository, Podman VM Images, volume mounts) that the truncated bhyslop version had lost. Sibling pace ₢A5AAF handles label vocabulary rename.

This pace asks the separate structural question: does the restored diagram's **scope** match current project architecture, or are there concepts that should be depicted but aren't? The diagnostic from ₢A5AAC history (shared ancestor `3ea89905` Dec 2024, scaleinv evolution July 2025, bhyslop rename March 2026) establishes that the richest content is from July 2025 — nine months stale on structural evolution even if we rename every label.

### Concepts to check for representation

Drawn from current README glossary (post-A5AAC), RBSCO narrative, and known architecture:

| Concept | Source | Expected relevance |
|---|---|---|
| **Depot** | README glossary: "GCP project + registry + bucket" | The diagram shows a "Container Repository" — is its containment within a depot concept visible? |
| **Vessel modes (conjure/bind/graft)** | README glossary + elaboration | Three distinct arrival paths for artifacts. Does the build-pipeline panel generalize or distinguish them? |
| **Hallmark** | README glossary: "specific build instance identified by timestamp" | Does the diagram show the hallmark-vs-vessel distinction? |
| **Nameplate as policy declaration** | README expanded How It Works (post-A5AAC): "Each crucible's allowlist is declared in a nameplate regime file" | Is the nameplate concept represented? |
| **SLSA provenance / vouch chain** | README How It Works + glossary | The supply chain attestation — depicted or implicit? |
| **Enshrine (base image mirror)** | README glossary + Image Management bullets (post-A5AAC) | Upstream base image mirroring to depot registry — depicted or absent? |
| **Role separation (payor/governor/director/retriever)** | README Setup section | Are any roles shown, or is this entirely out of scope for the architecture diagram? |
| **Cloud Build dual-pool / egress lockdown** | README Project Direction (post-A5AAC) | The airgap build architecture — depicted or too fine-grained? |
| **Pentacle as namespace establisher** | README Key Concepts | Present in the restored diagram as "Censer Container" (to be renamed by ₢A5AAF) — confirm the concept is depicted, not just the label |

### Steps

1. **Inventory the restored SVG's scope** — extract all text labels and identify what concepts are explicitly depicted. Build an "is-present" list.

2. **Walk the candidate-concept checklist** above. For each: classify as **present** (with reference to which shape/label represents it), **absent-and-should-be-added** (with rationale), or **absent-and-correctly-out-of-scope** (with rationale — e.g., "diagram scope is architecture, not process; roles belong in a separate diagram").

3. **Produce an audit memo** at `Memos/memo-svg-content-audit.md` capturing the full walkthrough. Include: pre-audit inventory, concept-by-concept decisions, and a list of recommended additions (if any) with directive-list-ready specificity (shape placement hints, connector routing hints, label text).

4. **Present memo to user for review**.

5. **User decides**: execute the recommended additions, defer some, decline others. Agent updates the memo to reflect the decision.

6. **If additions are approved for execution**: user performs drawio edits per the directive list in the memo; agent re-inventories post-edit to confirm the new concepts are present.

7. **Notch** with intent describing the audit outcome and any executed additions. If user chose to defer all additions, notch with the memo alone and track the deferred items as itches in `jji_itch.md`.

### Explicitly NOT in scope

- **Label renaming** — sibling pace ₢A5AAF handles that. This pace does not alter existing labels even when discussing them in the memo.
- **Visual style / layout / color choices** — those are user composition decisions, never agent decisions.
- **Rewriting the diagram from scratch** — audit, don't rebuild.
- **Choosing what the diagram *should* communicate to a newcomer** — the audit checks against a concrete concept checklist, not against a generalized "is this diagram good" judgment.

### Verification gates

- Memo includes explicit rationale for every concept decision (present / add / out-of-scope)
- If additions are executed, post-edit inventory confirms each added concept is depicted
- User explicitly signs off on which additions to execute vs defer
- Do not auto-wrap — user decides when audit is closed

## References

- Restored SVG: `rbm-abstract-drawio.svg` (48,452 bytes, commit `0adaa6ff` from `scaleinv/recipebottle` main)
- ₢A5AAC notch chain: `28fbf579`, `aa813b95`, `0adaa6ff`
- Sibling pace: ₢A5AAF (SVG vocabulary rename, reslated with restoration context)
- `README.md` post-A5AAC — authoritative source for current architecture concepts (glossary, How It Works, Project Direction)
- `Tools/rbk/vov_veiled/RBSCO-CosmologyIntro.adoc` — narrative source for cosmology context
- ₣A5 paddock — divergent-history diagnostic, "consider both and merge them appropriately" framing
- Heat paddock hard boundary: "Claude does not touch the SVG source"

**[260409-1354] rough**

## Character

Design review, non-destructive. Agent reads the restored SVG and the current project state, then produces a judgment about whether the diagram's scope matches current architecture. Output is a memo classifying each candidate concept as present / absent-should-add / absent-correctly-out-of-scope, optionally followed by a directive list for content additions the user would execute in drawio.

Cognitive posture: architectural judgment paired with mechanical inventory. This is the "consider both and merge them appropriately" half of the user's ₢A5AAC-follow-up instruction — the restored richer SVG is one input, the current project architecture is the other, and the merge decisions live in this pace's memo.

Hard boundary inherited from heat paddock: agent does not edit the drawio source or the SVG file. If additions are warranted, agent produces directives; user performs drawio edits.

## Docket

**Prerequisites**: ₢A5AAC notched (rough is fine). The restored SVG (commit `0adaa6ff`, 48,452 bytes) is the input.

**Ordering**: This pace runs BEFORE ₢A5AAF (vocabulary rename). Rationale: if the audit finds content additions are warranted, the user executes one drawio session covering both new content AND vocabulary updates on the new content, rather than two round trips. ₢A5AAF's re-extraction step will pick up whatever labels exist post-audit.

### Context

₢A5AAC restored the richer `scaleinv/recipebottle` version of `rbm-abstract-drawio.svg`, recovering a build-pipeline panel (Workbench Console, Image Build Service, OCI Container Repository, Podman VM Images, volume mounts) that the truncated bhyslop version had lost. Sibling pace ₢A5AAF handles label vocabulary rename.

This pace asks the separate structural question: does the restored diagram's **scope** match current project architecture, or are there concepts that should be depicted but aren't? The diagnostic from ₢A5AAC history (shared ancestor `3ea89905` Dec 2024, scaleinv evolution July 2025, bhyslop rename March 2026) establishes that the richest content is from July 2025 — nine months stale on structural evolution even if we rename every label.

### Concepts to check for representation

Drawn from current README glossary (post-A5AAC), RBSCO narrative, and known architecture:

| Concept | Source | Expected relevance |
|---|---|---|
| **Depot** | README glossary: "GCP project + registry + bucket" | The diagram shows a "Container Repository" — is its containment within a depot concept visible? |
| **Vessel modes (conjure/bind/graft)** | README glossary + elaboration | Three distinct arrival paths for artifacts. Does the build-pipeline panel generalize or distinguish them? |
| **Hallmark** | README glossary: "specific build instance identified by timestamp" | Does the diagram show the hallmark-vs-vessel distinction? |
| **Nameplate as policy declaration** | README expanded How It Works (post-A5AAC): "Each crucible's allowlist is declared in a nameplate regime file" | Is the nameplate concept represented? |
| **SLSA provenance / vouch chain** | README How It Works + glossary | The supply chain attestation — depicted or implicit? |
| **Enshrine (base image mirror)** | README glossary + Image Management bullets (post-A5AAC) | Upstream base image mirroring to depot registry — depicted or absent? |
| **Role separation (payor/governor/director/retriever)** | README Setup section | Are any roles shown, or is this entirely out of scope for the architecture diagram? |
| **Cloud Build dual-pool / egress lockdown** | README Project Direction (post-A5AAC) | The airgap build architecture — depicted or too fine-grained? |
| **Pentacle as namespace establisher** | README Key Concepts | Present in the restored diagram as "Censer Container" (to be renamed by ₢A5AAF) — confirm the concept is depicted, not just the label |

### Steps

1. **Inventory the restored SVG's scope** — extract all text labels and identify what concepts are explicitly depicted. Build an "is-present" list.

2. **Walk the candidate-concept checklist** above. For each: classify as **present** (with reference to which shape/label represents it), **absent-and-should-be-added** (with rationale), or **absent-and-correctly-out-of-scope** (with rationale — e.g., "diagram scope is architecture, not process; roles belong in a separate diagram").

3. **Produce an audit memo** at `Memos/memo-svg-content-audit.md` capturing the full walkthrough. Include: pre-audit inventory, concept-by-concept decisions, and a list of recommended additions (if any) with directive-list-ready specificity (shape placement hints, connector routing hints, label text).

4. **Present memo to user for review**.

5. **User decides**: execute the recommended additions, defer some, decline others. Agent updates the memo to reflect the decision.

6. **If additions are approved for execution**: user performs drawio edits per the directive list in the memo; agent re-inventories post-edit to confirm the new concepts are present.

7. **Notch** with intent describing the audit outcome and any executed additions. If user chose to defer all additions, notch with the memo alone and track the deferred items as itches in `jji_itch.md`.

### Explicitly NOT in scope

- **Label renaming** — sibling pace ₢A5AAF handles that. This pace does not alter existing labels even when discussing them in the memo.
- **Visual style / layout / color choices** — those are user composition decisions, never agent decisions.
- **Rewriting the diagram from scratch** — audit, don't rebuild.
- **Choosing what the diagram *should* communicate to a newcomer** — the audit checks against a concrete concept checklist, not against a generalized "is this diagram good" judgment.

### Verification gates

- Memo includes explicit rationale for every concept decision (present / add / out-of-scope)
- If additions are executed, post-edit inventory confirms each added concept is depicted
- User explicitly signs off on which additions to execute vs defer
- Do not auto-wrap — user decides when audit is closed

## References

- Restored SVG: `rbm-abstract-drawio.svg` (48,452 bytes, commit `0adaa6ff` from `scaleinv/recipebottle` main)
- ₢A5AAC notch chain: `28fbf579`, `aa813b95`, `0adaa6ff`
- Sibling pace: ₢A5AAF (SVG vocabulary rename, reslated with restoration context)
- `README.md` post-A5AAC — authoritative source for current architecture concepts (glossary, How It Works, Project Direction)
- `Tools/rbk/vov_veiled/RBSCO-CosmologyIntro.adoc` — narrative source for cosmology context
- ₣A5 paddock — divergent-history diagnostic, "consider both and merge them appropriately" framing
- Heat paddock hard boundary: "Claude does not touch the SVG source"

### drawio-svg-vocabulary-review (₢A8AAD) [complete]

**[260412-1047] complete**

Drafted from ₢A5AAF in ₣A5.

## Character

Mechanical-but-user-dependent. Agent produces a directive list; user executes all drawio edits. Hard boundary (from heat paddock): agent does not touch the SVG source file. The restored 48 KB SVG (commit `0adaa6ff` landed on ₢A5AAC) contains stale vocabulary that must be renamed to match current project terminology.

## Docket

**Prerequisites**: ₢A5AAC notched (rough is fine — does not need to be wrapped). The restored richer SVG is the input to this pace. Content-audit sibling (slated before this pace) should run first so any newly-added content gets labeled with current vocabulary during the same drawio session.

### Context

This pace exists because ₢A5AAA Q1's grep claimed no vocabulary rename sweep was needed, but that grep did not scope `rbm-abstract-drawio.svg`. When ₢A5AAC restored the richer scaleinv version of the SVG (commit `0adaa6ff`), it deliberately re-introduced stale vocabulary from the pre-rename scaleinv base. This pace closes the gap by producing a directive list for the user to apply in drawio.

### Stale labels confirmed present in the restored SVG

Empirically verified via label extraction during ₢A5AAC investigation:

| Current (stale) | Target (current) | Rationale |
|---|---|---|
| `Bottle Service` | `Crucible` | Renamed in ₣Ax (commit `4f3de046`, March 2026); the sentry/pentacle/bottle triad as one runnable unit |
| `Censer Container` | `Pentacle Container` | Current term for the privileged namespace establisher |
| `Podman VM Images` | `Docker Images` | Project switched container runtime from podman to docker in December 2025 |
| `Image Build Service (github action)` | `Image Build Service (Google Cloud Build)` | Project migrated from GitHub Actions + GHCR to Cloud Build + GAR in August 2025 |
| `OCI Compliant Container Repository` | `Google Artifact Registry (GAR)` | Same migration; specific registry name is now knowable |
| `- Offsite Sentry/ Censer/ Bottle images` | `- Offsite Sentry/ Pentacle/ Bottle images` | Follows the Censer → Pentacle rename |

### Steps

1. **Re-extract current labels** from `rbm-abstract-drawio.svg` as the first act of this pace (the table above was captured at A5AAC time; the file may have evolved further via the content-audit sibling). Produce a fresh inventory and reconcile with the table above. Flag any additional stale vocabulary not captured in the initial list.

2. **Produce final directive list** as a memo (e.g., `Memos/memo-svg-vocabulary-rename.md`) or inline in this pace's docket. For each stale label: current text (verbatim, with surrounding context for disambiguation), target text, and — where labels are duplicated across multiple shapes — which instance to update.

3. **Present directive list to user for review** before they open drawio.

4. **User executes in drawio**. User opens the source file (or the SVG directly in drawio's web editor), applies each rename per the directive list, exports back to `rbm-abstract-drawio.svg`.

5. **Post-export verification** — agent re-runs label extraction on the exported file. Diff against the directive list. Confirm zero remaining stale vocabulary.

6. **Notch** with intent describing the rename pass and the pre/post label diff.

### Explicitly NOT in scope

- Content changes to the diagram (adding/removing shapes, moving panels, rewiring connectors) — that's the content-audit sibling pace.
- Rewriting the diagram from scratch.
- Re-embedding or resizing the SVG in README.

### Verification gates

- Fresh label extraction at step 1 confirms the rename list is complete before user opens drawio
- Post-export label extraction shows zero remaining stale vocabulary
- User confirms drawio session is clean (no unsaved state)
- Do not auto-wrap — user decides when vocab is correct

## References

- Restored SVG: `rbm-abstract-drawio.svg` (48,452 bytes, restored in ₢A5AAC commit `0adaa6ff` from `scaleinv/recipebottle` main)
- ₢A5AAC notch chain: `28fbf579` (README rewrite) → `aa813b95` (Crucible glossary row + openssl) → `0adaa6ff` (SVG restoration)
- Sibling pace: SVG content-audit (new, slated before this pace)
- ₣A5 paddock — divergent-history diagnostic establishing this pace's context
- Heat paddock hard boundary: "Claude does not touch the SVG source"

**[260410-0742] rough**

Drafted from ₢A5AAF in ₣A5.

## Character

Mechanical-but-user-dependent. Agent produces a directive list; user executes all drawio edits. Hard boundary (from heat paddock): agent does not touch the SVG source file. The restored 48 KB SVG (commit `0adaa6ff` landed on ₢A5AAC) contains stale vocabulary that must be renamed to match current project terminology.

## Docket

**Prerequisites**: ₢A5AAC notched (rough is fine — does not need to be wrapped). The restored richer SVG is the input to this pace. Content-audit sibling (slated before this pace) should run first so any newly-added content gets labeled with current vocabulary during the same drawio session.

### Context

This pace exists because ₢A5AAA Q1's grep claimed no vocabulary rename sweep was needed, but that grep did not scope `rbm-abstract-drawio.svg`. When ₢A5AAC restored the richer scaleinv version of the SVG (commit `0adaa6ff`), it deliberately re-introduced stale vocabulary from the pre-rename scaleinv base. This pace closes the gap by producing a directive list for the user to apply in drawio.

### Stale labels confirmed present in the restored SVG

Empirically verified via label extraction during ₢A5AAC investigation:

| Current (stale) | Target (current) | Rationale |
|---|---|---|
| `Bottle Service` | `Crucible` | Renamed in ₣Ax (commit `4f3de046`, March 2026); the sentry/pentacle/bottle triad as one runnable unit |
| `Censer Container` | `Pentacle Container` | Current term for the privileged namespace establisher |
| `Podman VM Images` | `Docker Images` | Project switched container runtime from podman to docker in December 2025 |
| `Image Build Service (github action)` | `Image Build Service (Google Cloud Build)` | Project migrated from GitHub Actions + GHCR to Cloud Build + GAR in August 2025 |
| `OCI Compliant Container Repository` | `Google Artifact Registry (GAR)` | Same migration; specific registry name is now knowable |
| `- Offsite Sentry/ Censer/ Bottle images` | `- Offsite Sentry/ Pentacle/ Bottle images` | Follows the Censer → Pentacle rename |

### Steps

1. **Re-extract current labels** from `rbm-abstract-drawio.svg` as the first act of this pace (the table above was captured at A5AAC time; the file may have evolved further via the content-audit sibling). Produce a fresh inventory and reconcile with the table above. Flag any additional stale vocabulary not captured in the initial list.

2. **Produce final directive list** as a memo (e.g., `Memos/memo-svg-vocabulary-rename.md`) or inline in this pace's docket. For each stale label: current text (verbatim, with surrounding context for disambiguation), target text, and — where labels are duplicated across multiple shapes — which instance to update.

3. **Present directive list to user for review** before they open drawio.

4. **User executes in drawio**. User opens the source file (or the SVG directly in drawio's web editor), applies each rename per the directive list, exports back to `rbm-abstract-drawio.svg`.

5. **Post-export verification** — agent re-runs label extraction on the exported file. Diff against the directive list. Confirm zero remaining stale vocabulary.

6. **Notch** with intent describing the rename pass and the pre/post label diff.

### Explicitly NOT in scope

- Content changes to the diagram (adding/removing shapes, moving panels, rewiring connectors) — that's the content-audit sibling pace.
- Rewriting the diagram from scratch.
- Re-embedding or resizing the SVG in README.

### Verification gates

- Fresh label extraction at step 1 confirms the rename list is complete before user opens drawio
- Post-export label extraction shows zero remaining stale vocabulary
- User confirms drawio session is clean (no unsaved state)
- Do not auto-wrap — user decides when vocab is correct

## References

- Restored SVG: `rbm-abstract-drawio.svg` (48,452 bytes, restored in ₢A5AAC commit `0adaa6ff` from `scaleinv/recipebottle` main)
- ₢A5AAC notch chain: `28fbf579` (README rewrite) → `aa813b95` (Crucible glossary row + openssl) → `0adaa6ff` (SVG restoration)
- Sibling pace: SVG content-audit (new, slated before this pace)
- ₣A5 paddock — divergent-history diagnostic establishing this pace's context
- Heat paddock hard boundary: "Claude does not touch the SVG source"

**[260409-1353] rough**

## Character

Mechanical-but-user-dependent. Agent produces a directive list; user executes all drawio edits. Hard boundary (from heat paddock): agent does not touch the SVG source file. The restored 48 KB SVG (commit `0adaa6ff` landed on ₢A5AAC) contains stale vocabulary that must be renamed to match current project terminology.

## Docket

**Prerequisites**: ₢A5AAC notched (rough is fine — does not need to be wrapped). The restored richer SVG is the input to this pace. Content-audit sibling (slated before this pace) should run first so any newly-added content gets labeled with current vocabulary during the same drawio session.

### Context

This pace exists because ₢A5AAA Q1's grep claimed no vocabulary rename sweep was needed, but that grep did not scope `rbm-abstract-drawio.svg`. When ₢A5AAC restored the richer scaleinv version of the SVG (commit `0adaa6ff`), it deliberately re-introduced stale vocabulary from the pre-rename scaleinv base. This pace closes the gap by producing a directive list for the user to apply in drawio.

### Stale labels confirmed present in the restored SVG

Empirically verified via label extraction during ₢A5AAC investigation:

| Current (stale) | Target (current) | Rationale |
|---|---|---|
| `Bottle Service` | `Crucible` | Renamed in ₣Ax (commit `4f3de046`, March 2026); the sentry/pentacle/bottle triad as one runnable unit |
| `Censer Container` | `Pentacle Container` | Current term for the privileged namespace establisher |
| `Podman VM Images` | `Docker Images` | Project switched container runtime from podman to docker in December 2025 |
| `Image Build Service (github action)` | `Image Build Service (Google Cloud Build)` | Project migrated from GitHub Actions + GHCR to Cloud Build + GAR in August 2025 |
| `OCI Compliant Container Repository` | `Google Artifact Registry (GAR)` | Same migration; specific registry name is now knowable |
| `- Offsite Sentry/ Censer/ Bottle images` | `- Offsite Sentry/ Pentacle/ Bottle images` | Follows the Censer → Pentacle rename |

### Steps

1. **Re-extract current labels** from `rbm-abstract-drawio.svg` as the first act of this pace (the table above was captured at A5AAC time; the file may have evolved further via the content-audit sibling). Produce a fresh inventory and reconcile with the table above. Flag any additional stale vocabulary not captured in the initial list.

2. **Produce final directive list** as a memo (e.g., `Memos/memo-svg-vocabulary-rename.md`) or inline in this pace's docket. For each stale label: current text (verbatim, with surrounding context for disambiguation), target text, and — where labels are duplicated across multiple shapes — which instance to update.

3. **Present directive list to user for review** before they open drawio.

4. **User executes in drawio**. User opens the source file (or the SVG directly in drawio's web editor), applies each rename per the directive list, exports back to `rbm-abstract-drawio.svg`.

5. **Post-export verification** — agent re-runs label extraction on the exported file. Diff against the directive list. Confirm zero remaining stale vocabulary.

6. **Notch** with intent describing the rename pass and the pre/post label diff.

### Explicitly NOT in scope

- Content changes to the diagram (adding/removing shapes, moving panels, rewiring connectors) — that's the content-audit sibling pace.
- Rewriting the diagram from scratch.
- Re-embedding or resizing the SVG in README.

### Verification gates

- Fresh label extraction at step 1 confirms the rename list is complete before user opens drawio
- Post-export label extraction shows zero remaining stale vocabulary
- User confirms drawio session is clean (no unsaved state)
- Do not auto-wrap — user decides when vocab is correct

## References

- Restored SVG: `rbm-abstract-drawio.svg` (48,452 bytes, restored in ₢A5AAC commit `0adaa6ff` from `scaleinv/recipebottle` main)
- ₢A5AAC notch chain: `28fbf579` (README rewrite) → `aa813b95` (Crucible glossary row + openssl) → `0adaa6ff` (SVG restoration)
- Sibling pace: SVG content-audit (new, slated before this pace)
- ₣A5 paddock — divergent-history diagnostic establishing this pace's context
- Heat paddock hard boundary: "Claude does not touch the SVG source"

**[260409-1100] rough**

## Character

Review-only, directive-producing. **Claude reads the SVG and advises; the user performs all drawio edits.** This is a hard boundary. Claude cannot and should not modify the drawio source or the rendered SVG directly — drawio editing is visual iteration in the drawio app, and the composition decisions belong to the user.

The agent's role is narrow: extract every text label from the SVG, cross-check against the new README vocabulary, produce a precise directive list for the user to execute in drawio, then verify the re-exported SVG after the user applies the edits.

Depends on ₢A5AAC (README rewrite) and ideally ₢A5AAE (retention audit) being complete so the final vocabulary is settled before the SVG is compared against it. Running this before vocabulary settles would require redoing the review.

Cognitive posture: mechanical extraction, precise cross-check, no interpretation of diagram composition (the user owns that). Flag vocabulary drift only, not layout or composition concerns.

## Docket

**Prerequisites**: ₢A5AAC and ₢A5AAE both complete. README vocabulary settled.

### Steps

1. **Extract SVG text content.** Read `rbm-abstract-drawio.svg` and enumerate every text element: box labels, arrow labels, annotations, legend entries, title text. Record each with enough positional context that the user can locate it in drawio (which box, which arrow, approximate position in the diagram).

2. **Build the vocabulary reference set.** From the current README.md post-AAE state, extract the authoritative list of Recipe Bottle terms: everything in the glossary table, everything cross-linked via `[term](#Term)` syntax, any term defined in the intro sections. Also pull from ₢A5AAA's Q1 term-rename inventory for historical context.

3. **Cross-check each SVG text element** against the reference set. For each element, classify:
   - **current** — term matches the authoritative vocabulary
   - **stale** — old term that needs renaming (e.g., "Bottle Service" → "crucible"); produce exact current text and exact replacement
   - **non-vocabulary** — plain English or diagram scaffolding, no action
   - **ambiguous** — text that *might* be a term but could also be generic English; flag for user judgment

4. **Produce the directive list.** Format as a table:
   ```
   | # | Location hint        | Current text        | Replacement         | Notes           |
   |---|---------------------|---------------------|---------------------|-----------------|
   | 1 | Top-left box label  | Bottle Service      | crucible            | Per AAA Q1      |
   | 2 | Arrow from X to Y   | censer instance     | pentacle instance   | Per AAA Q1      |
   ```
   Present this to the user as the directive list.

5. **User review point**: present directives. User reviews, confirms, and may push back on any replacement Claude missed or proposed incorrectly. Refine the list until user approves.

6. **User executes edits in drawio.** User opens `rbm-abstract-drawio.svg` in drawio (or re-exports from the `.drawio` source if that's how the project maintains it), applies each directive, saves, and re-exports the SVG. **Claude does not touch the file during this step.**

7. **Verify the re-exported SVG.** After user reports the edits are applied, re-read `rbm-abstract-drawio.svg` and verify:
   - Every stale term from the directive list is gone
   - Every replacement term appears in the expected location (to the extent position is inferrable from SVG text order)
   - No new vocabulary drift was introduced (no new terms that aren't in the reference set)
   - Report verification result to user

8. **If drift is found in step 7**, loop: produce a delta directive list, user applies, re-verify. Continue until clean.

9. **Final: confirm embedded reference in README.** The new README embeds the SVG via markdown image reference (per ₢A5AAC step 2). Confirm the reference still resolves and the updated SVG will render at the expected location.

10. **Notch** with intent describing the directive-driven sync: "Sync rbm-abstract-drawio.svg vocabulary with post-rewrite README. Produced <N> directives; user applied edits in drawio; verified re-exported SVG matches authoritative vocabulary with no new drift."

### Verification gates

- Directive list approved by user before any SVG edits happen
- All edits executed by user in drawio, never by Claude
- Re-exported SVG verified against directive list before notching
- Clean verification (no remaining stale terms, no new drift) before notching
- Do not auto-wrap — user decides when SVG review is closed

### Hard boundaries (do not cross)

- **Claude does NOT edit `rbm-abstract-drawio.svg`.** No Edit tool calls, no Write tool calls, no sed/awk bash commands on the SVG file.
- **Claude does NOT open or modify any `.drawio` source file** if one exists.
- **Claude does NOT make composition decisions** (box layout, arrow routing, colors, legend structure). Those belong to the user.
- Vocabulary sync only. Everything else is out of scope.

## References

- ₣A5 paddock — rewrite scope, vocabulary source
- ₢A5AAA structured note — Q1 term-rename inventory (source of ground truth for vocabulary drift)
- ₢A5AAC — README rewrite (landed); final vocabulary source
- ₢A5AAE — retention audit (landed); may have added vocabulary
- `rbm-abstract-drawio.svg` — review target (READ-ONLY for Claude)
- `README.md` — post-rewrite state; authoritative vocabulary source

### readme-structure-continued (₢A8AAG) [rough]

**[260410-0745] rough**

## Character

Continuation of ₢A5AAJ work. Same iterative structural editing posture — iterate until user signs off.

## Docket

Continuation of README structural editing from ₢A5AAJ (wrapped prematurely in ₣A5). The four original tasks landed; this pace carries any remaining iteration the user wants on the document's structure, section titles, intro flow, or content placement.

### Iteration

No fixed endpoint. Iterate until user signs off. Do not auto-wrap.

### lifecycle-teardown-anchors (₢A8AAI) [complete]

**[260410-0921] complete**

## Character

Content addition plus cross-linking pass. Low design judgment — the three terms (Abjure, Jettison, Unmake) are well-defined in RBS0 and their README definitions were drafted in the ₣A6 groom session (officium ☉260410-1004). The work is: add the glossary entries, then sweep README.md for places that describe teardown/removal without linking to the new anchors.

## Docket

Add end-of-lifecycle glossary entries to README.md and cross-link them throughout existing prose.

### Add three glossary entries

Place after the creation counterparts they reverse:

- **Abjure** — after Tally, before Levy. Removes a Hallmark's `-image`, `-about`, `-vouch` artifacts as a unit. Reverse of Ordain. Director operation.
- **Jettison** — after Abjure, before Levy. Deletes a specific image tag. Lower-level than Abjure. Director operation.
- **Unmake** — after Levy. Permanently destroys a Depot's GCP infrastructure. Reverse of Levy. Payor operation. Irreversible.

Draft wording (adapt to README structure at time of implementation — Foundry restructuring may have changed section layout):

```
### <a id="Abjure"></a>Abjure

Remove a [Hallmark's](#Hallmark) artifacts from the [Depot's](#Depot) registry — the `-image`, `-about`, and `-vouch` tags deleted as a coherent unit. [Abjuring](#Abjure) is the reverse of [Ordaining](#Ordain): it formally renounces a build instance. The [Director](#Director) [Abjures](#Abjure) [Hallmarks](#Hallmark) that are superseded, broken, or no longer needed.

### <a id="Jettison"></a>Jettison

Delete a specific image tag from the [Depot's](#Depot) registry. [Jettisoning](#Jettison) is lower-level than [Abjure](#Abjure) — it removes a single tag rather than a complete [Hallmark](#Hallmark) artifact set. Used for cleanup of individual registry entries.

### <a id="Unmake"></a>Unmake

Permanently destroy a [Depot's](#Depot) GCP infrastructure — project, artifact registry, storage bucket, and all contents. [Unmaking](#Unmake) is the reverse of [Levying](#Levy). This is a [Payor](#Payor) operation and is irreversible.
```

### Cross-link sweep

After adding the entries, search README.md for prose that describes removal, deletion, or teardown operations without using the new anchor terms. Likely locations:

- **Recovery section** — "Forfeit the compromised account" already uses Forfeit; check for other teardown references
- **How It Works / Image Management** — any mention of deleting images or cleaning up
- **Configuration / Day-to-Day** — any mention of depot removal or project teardown

Replace generic language with linked terms where natural. Do not force links where the context doesn't warrant it.

### Update A6 paddock anchor inventory

Add `Abjure`, `Jettison`, `Unmake` to the README Anchor Inventory section in the ₣A6 paddock.

### Verify

- All three anchors resolve in GitHub preview
- Cross-links use consistent verb forms ([Abjuring](#Abjure), [Unmaking](#Unmake), etc.)
- No orphan references to the old generic language remain

### Out of Scope

- Forfeit — already appears in Recovery prose; anchoring it is a separate decision
- RBS0 quoin changes — these are README-only additions
- Foundry section restructuring — that's ₢A8AAH's job; this pace works with whatever structure exists at mount time

**[260410-0857] rough**

## Character

Content addition plus cross-linking pass. Low design judgment — the three terms (Abjure, Jettison, Unmake) are well-defined in RBS0 and their README definitions were drafted in the ₣A6 groom session (officium ☉260410-1004). The work is: add the glossary entries, then sweep README.md for places that describe teardown/removal without linking to the new anchors.

## Docket

Add end-of-lifecycle glossary entries to README.md and cross-link them throughout existing prose.

### Add three glossary entries

Place after the creation counterparts they reverse:

- **Abjure** — after Tally, before Levy. Removes a Hallmark's `-image`, `-about`, `-vouch` artifacts as a unit. Reverse of Ordain. Director operation.
- **Jettison** — after Abjure, before Levy. Deletes a specific image tag. Lower-level than Abjure. Director operation.
- **Unmake** — after Levy. Permanently destroys a Depot's GCP infrastructure. Reverse of Levy. Payor operation. Irreversible.

Draft wording (adapt to README structure at time of implementation — Foundry restructuring may have changed section layout):

```
### <a id="Abjure"></a>Abjure

Remove a [Hallmark's](#Hallmark) artifacts from the [Depot's](#Depot) registry — the `-image`, `-about`, and `-vouch` tags deleted as a coherent unit. [Abjuring](#Abjure) is the reverse of [Ordaining](#Ordain): it formally renounces a build instance. The [Director](#Director) [Abjures](#Abjure) [Hallmarks](#Hallmark) that are superseded, broken, or no longer needed.

### <a id="Jettison"></a>Jettison

Delete a specific image tag from the [Depot's](#Depot) registry. [Jettisoning](#Jettison) is lower-level than [Abjure](#Abjure) — it removes a single tag rather than a complete [Hallmark](#Hallmark) artifact set. Used for cleanup of individual registry entries.

### <a id="Unmake"></a>Unmake

Permanently destroy a [Depot's](#Depot) GCP infrastructure — project, artifact registry, storage bucket, and all contents. [Unmaking](#Unmake) is the reverse of [Levying](#Levy). This is a [Payor](#Payor) operation and is irreversible.
```

### Cross-link sweep

After adding the entries, search README.md for prose that describes removal, deletion, or teardown operations without using the new anchor terms. Likely locations:

- **Recovery section** — "Forfeit the compromised account" already uses Forfeit; check for other teardown references
- **How It Works / Image Management** — any mention of deleting images or cleaning up
- **Configuration / Day-to-Day** — any mention of depot removal or project teardown

Replace generic language with linked terms where natural. Do not force links where the context doesn't warrant it.

### Update A6 paddock anchor inventory

Add `Abjure`, `Jettison`, `Unmake` to the README Anchor Inventory section in the ₣A6 paddock.

### Verify

- All three anchors resolve in GitHub preview
- Cross-links use consistent verb forms ([Abjuring](#Abjure), [Unmaking](#Unmake), etc.)
- No orphan references to the old generic language remain

### Out of Scope

- Forfeit — already appears in Recovery prose; anchoring it is a separate decision
- RBS0 quoin changes — these are README-only additions
- Foundry section restructuring — that's ₢A8AAH's job; this pace works with whatever structure exists at mount time

### adversarial-testing-readme-section (₢A8AAJ) [complete]

**[260410-0928] complete**

## Character

Content drafting requiring human review. The concepts (Ifrit, Theurge, coordinated testing) are well-defined in RBS0 and internal docs, but the public-facing framing — how much to say, what tone to strike, where the line is between "inviting security scrutiny" and "overselling" — requires judgment the user must approve. Draft, present, wait.

## Docket

Replace the existing README.md "Testing" and "Recovery" sections with stronger content covering adversarial security testing, and introduce the Ifrit and Theurge concepts to external readers. **Do not auto-wrap — this pace requires human review of the final text.**

### Remove existing sections

Delete the current "Testing" section (qualification gates boilerplate) and "Recovery" section. This pace's content subsumes and improves on both.

### Add "Adversarial Security Testing" section

Place after "Day-to-Day Operations" (or wherever the Crucible operational content ends at mount time).

Two anchored glossary entries:

- **Ifrit** — adversarial attack vessel running inside a Bottle. Purpose-built with scapy (arbitrary packet construction), strace (syscall boundary probing), and minimal footprint. Named for the djinn imprisoned in a bottle, seeking escape.
- **Theurge** — test orchestrator running on the host, outside the Crucible. Charges a crucible, dispatches Ifrit attacks, and observes results from both inside the Bottle and outside via the Sentry. Coordinated testing — simultaneous attack and observation — is the distinctive capability.

### Framing guidance

- The escape tests were **developed through adversarial Claude Code sessions** with full visibility into the Sentry's source, iptables rules, dnsmasq configuration, and RBS0 spec. The Ifrit vessel is the delivery vehicle; the intelligence came from the authoring process.
- Do NOT say "the Ifrit has perfect information" or similar — that conflates the author (Claude Code session) with the running artifact (attack binary).
- Do NOT introduce "Sortie" as a glossary term. Describe the escape tests as curated, reproducible, version-controlled attack scripts targeting specific surfaces (DNS exfiltration, ICMP covert channels, metadata probing, namespace breakout, etc.) — in plain prose, not coined vocabulary.
- Tone should match the Important blockquote at the top of README: honest about early-stage status, inviting scrutiny, not claiming certainty. "Every test that passes is evidence the containment holds" — not proof.

### Draft, review, iterate

1. Read current README.md structure at mount time (Foundry restructuring and other A8 paces may have changed layout)
2. Draft the section following the framing guidance above
3. Present draft to user for review — **do not commit without explicit approval**
4. Iterate on feedback
5. Notch after approval, present for wrap

### Out of scope

- Sortie framework internals (roster, adjutant, debrief, fronts — implementer vocabulary)
- Theurge Rust architecture or build instructions
- Ifrit vessel Dockerfile details
- Test case inventory or fixture names
- Changes to RBS0 or other specs

**[260410-0914] rough**

## Character

Content drafting requiring human review. The concepts (Ifrit, Theurge, coordinated testing) are well-defined in RBS0 and internal docs, but the public-facing framing — how much to say, what tone to strike, where the line is between "inviting security scrutiny" and "overselling" — requires judgment the user must approve. Draft, present, wait.

## Docket

Replace the existing README.md "Testing" and "Recovery" sections with stronger content covering adversarial security testing, and introduce the Ifrit and Theurge concepts to external readers. **Do not auto-wrap — this pace requires human review of the final text.**

### Remove existing sections

Delete the current "Testing" section (qualification gates boilerplate) and "Recovery" section. This pace's content subsumes and improves on both.

### Add "Adversarial Security Testing" section

Place after "Day-to-Day Operations" (or wherever the Crucible operational content ends at mount time).

Two anchored glossary entries:

- **Ifrit** — adversarial attack vessel running inside a Bottle. Purpose-built with scapy (arbitrary packet construction), strace (syscall boundary probing), and minimal footprint. Named for the djinn imprisoned in a bottle, seeking escape.
- **Theurge** — test orchestrator running on the host, outside the Crucible. Charges a crucible, dispatches Ifrit attacks, and observes results from both inside the Bottle and outside via the Sentry. Coordinated testing — simultaneous attack and observation — is the distinctive capability.

### Framing guidance

- The escape tests were **developed through adversarial Claude Code sessions** with full visibility into the Sentry's source, iptables rules, dnsmasq configuration, and RBS0 spec. The Ifrit vessel is the delivery vehicle; the intelligence came from the authoring process.
- Do NOT say "the Ifrit has perfect information" or similar — that conflates the author (Claude Code session) with the running artifact (attack binary).
- Do NOT introduce "Sortie" as a glossary term. Describe the escape tests as curated, reproducible, version-controlled attack scripts targeting specific surfaces (DNS exfiltration, ICMP covert channels, metadata probing, namespace breakout, etc.) — in plain prose, not coined vocabulary.
- Tone should match the Important blockquote at the top of README: honest about early-stage status, inviting scrutiny, not claiming certainty. "Every test that passes is evidence the containment holds" — not proof.

### Draft, review, iterate

1. Read current README.md structure at mount time (Foundry restructuring and other A8 paces may have changed layout)
2. Draft the section following the framing guidance above
3. Present draft to user for review — **do not commit without explicit approval**
4. Iterate on feedback
5. Notch after approval, present for wrap

### Out of scope

- Sortie framework internals (roster, adjutant, debrief, fronts — implementer vocabulary)
- Theurge Rust architecture or build instructions
- Ifrit vessel Dockerfile details
- Test case inventory or fixture names
- Changes to RBS0 or other specs

**[260410-0907] rough**

## Character

Content drafting requiring human review. The concepts (Ifrit, Theurge, coordinated testing) are well-defined in RBS0 and internal docs, but the public-facing framing — how much to say, what tone to strike, where the line is between "inviting security scrutiny" and "overselling" — requires judgment the user must approve. Draft, present, wait.

## Docket

Add an "Adversarial Security Testing" section to README.md introducing the Ifrit and Theurge concepts to external readers. **Do not auto-wrap — this pace requires human review of the final text.**

### Section placement

After the existing "Testing" section, before "Recovery". Peer to Testing (which covers qualification gates), this section covers security validation.

### Concepts to introduce

Two anchored glossary entries:

- **Ifrit** — adversarial attack vessel running inside a Bottle. Purpose-built with scapy (arbitrary packet construction), strace (syscall boundary probing), and minimal footprint. Named for the djinn imprisoned in a bottle, seeking escape.
- **Theurge** — test orchestrator running on the host, outside the Crucible. Charges a crucible, dispatches Ifrit attacks, and observes results from both inside the Bottle and outside via the Sentry. Coordinated testing — simultaneous attack and observation — is the distinctive capability.

### Framing guidance

- The escape tests were **developed through adversarial Claude Code sessions** with full visibility into the Sentry's source, iptables rules, dnsmasq configuration, and RBS0 spec. The Ifrit vessel is the delivery vehicle; the intelligence came from the authoring process.
- Do NOT say "the Ifrit has perfect information" or similar — that conflates the author (Claude Code session) with the running artifact (attack binary).
- Do NOT introduce "Sortie" as a glossary term. Describe the escape tests as curated, reproducible, version-controlled attack scripts targeting specific surfaces (DNS exfiltration, ICMP covert channels, metadata probing, namespace breakout, etc.) — in plain prose, not coined vocabulary.
- Tone should match the Important blockquote at the top of README: honest about early-stage status, inviting scrutiny, not claiming certainty. "Every test that passes is evidence the containment holds" — not proof.

### Draft, review, iterate

1. Read current README.md structure at mount time (Foundry restructuring and other A8 paces may have changed layout)
2. Draft the section following the framing guidance above
3. Present draft to user for review — **do not commit without explicit approval**
4. Iterate on feedback
5. Notch after approval, present for wrap

### Out of scope

- Sortie framework internals (roster, adjutant, debrief, fronts — implementer vocabulary)
- Theurge Rust architecture or build instructions
- Ifrit vessel Dockerfile details
- Test case inventory or fixture names
- Changes to RBS0 or other specs

### readme-structure-continued (₢A8AAF) [abandoned]

**[260410-0746] abandoned**

## Character

Continuation of ₢A5AAJ work. Same iterative structural editing posture — iterate until user signs off.

## Docket

Continuation of README structural editing from ₢A5AAJ (wrapped prematurely in ₣A5). The four original tasks landed; this pace carries any remaining iteration the user wants on the document's structure, section titles, intro flow, or content placement.

### Iteration

No fixed endpoint. Iterate until user signs off. Do not auto-wrap.

**[260410-0744] rough**

## Character

Continuation of ₢A5AAJ work. Same iterative structural editing posture — iterate until user signs off.

## Docket

Continuation of README structural editing from ₢A5AAJ (wrapped prematurely in ₣A5). The four original tasks landed; this pace carries any remaining iteration the user wants on the document's structure, section titles, intro flow, or content placement.

### Iteration

No fixed endpoint. Iterate until user signs off. Do not auto-wrap.

### readme-structure-continued (₢A8AAE) [abandoned]

**[260410-0746] abandoned**

## Character

Continuation of ₢A5AAJ work. Same iterative structural editing posture — iterate until user signs off.

## Docket

Continuation of README structural editing from ₢A5AAJ (wrapped prematurely in ₣A5). The four original tasks landed; this pace carries any remaining iteration the user wants on the document's structure, section titles, intro flow, or content placement.

### Iteration

No fixed endpoint. Iterate until user signs off. Do not auto-wrap.

**[260410-0744] rough**

## Character

Continuation of ₢A5AAJ work. Same iterative structural editing posture — iterate until user signs off.

## Docket

Continuation of README structural editing from ₢A5AAJ (wrapped prematurely in ₣A5). The four original tasks landed; this pace carries any remaining iteration the user wants on the document's structure, section titles, intro flow, or content placement.

### Iteration

No fixed endpoint. Iterate until user signs off. Do not auto-wrap.

### retirements-rbsph-ceremony-sweep (₢A8AAA) [abandoned]

**[260420-1024] abandoned**

Drafted from ₢A5AAD in ₣A5.

## Character

Remaining mechanical cleanup from the original D1-D5 plan. Tracks D1 (RBSPH/RBSCO), D2 (file retirements), and D3's tabtarget fix already landed in commit b7c1dbdd. What remains: the CLAUDE.consumer.md glossary decision (D3 user review point), the prep-release ceremony rewrite (D4), and the final verification sweep (D5).

## Docket

**Prerequisites**: Tracks D1-D3 partial landed (commit b7c1dbdd). README structural work (₢A5AAJ) complete.

### Track D3 remainder: CLAUDE.consumer.md glossary alignment

1. **Glossary decision** (user review point): The current 11-term glossary covers core nouns; the 16 verb terms and 4 role terms are already covered by the Verb Guide and Roles sections. Present both options (expand to 27 vs keep curated 11) and let the user pick.

### Track D4: prep-release ceremony rewrite

2. **Rewrite `.claude/commands/rbk-prep-release.md`** per ₢A5AAA Q9 delta list:
   - **Step 2 (line 35)**: "index.html promise" -> "README.md Prerequisites section"
   - **Step 8 (lines 106-115)**: Delete the `cp README.consumer.md README.md` line. Keep CLAUDE.consumer.md copy. Drop lowercase readme.md note.
   - **Step 10e (line 202)**: Delete `git rm readme.md`. Add `git rm index.html` and `git rm .nojekyll` as defensive removals.
   - **Step 10 surviving list (lines 215-226)**: Remove `index.html` and `.nojekyll` lines.
   - **Step 10f (lines 205-209)**: `git add CLAUDE.md README.md` -> `git add CLAUDE.md` only.

   **User review point**: present ceremony diff before commit.

### Track D5: Final verification sweep

3. **Verification grep**: confirm no live references to `index.html`, `README.consumer.md`, or `.nojekyll` outside retired gallops.
4. **Run fast qualification**: `tt/rbw-tf.QualifyFast.sh`.
5. **Manual smoke**: `tt/rbw-go.OnboardMAIN.sh` and role tracks.
6. **Notch** remaining changes.
7. **Ask user for go-ahead to wrap the heat.** Do NOT auto-wrap.
8. **After wrap: user-executed push.** Agent does NOT push.

## References

- Commit b7c1dbdd — D1-D3 partial (RBSPH, RBSCO, retirements, tabtarget fix)
- `.claude/commands/rbk-prep-release.md` — ceremony rewrite target
- `Tools/rbk/vov_veiled/CLAUDE.consumer.md` — glossary decision pending

**[260410-0742] rough**

Drafted from ₢A5AAD in ₣A5.

## Character

Remaining mechanical cleanup from the original D1-D5 plan. Tracks D1 (RBSPH/RBSCO), D2 (file retirements), and D3's tabtarget fix already landed in commit b7c1dbdd. What remains: the CLAUDE.consumer.md glossary decision (D3 user review point), the prep-release ceremony rewrite (D4), and the final verification sweep (D5).

## Docket

**Prerequisites**: Tracks D1-D3 partial landed (commit b7c1dbdd). README structural work (₢A5AAJ) complete.

### Track D3 remainder: CLAUDE.consumer.md glossary alignment

1. **Glossary decision** (user review point): The current 11-term glossary covers core nouns; the 16 verb terms and 4 role terms are already covered by the Verb Guide and Roles sections. Present both options (expand to 27 vs keep curated 11) and let the user pick.

### Track D4: prep-release ceremony rewrite

2. **Rewrite `.claude/commands/rbk-prep-release.md`** per ₢A5AAA Q9 delta list:
   - **Step 2 (line 35)**: "index.html promise" -> "README.md Prerequisites section"
   - **Step 8 (lines 106-115)**: Delete the `cp README.consumer.md README.md` line. Keep CLAUDE.consumer.md copy. Drop lowercase readme.md note.
   - **Step 10e (line 202)**: Delete `git rm readme.md`. Add `git rm index.html` and `git rm .nojekyll` as defensive removals.
   - **Step 10 surviving list (lines 215-226)**: Remove `index.html` and `.nojekyll` lines.
   - **Step 10f (lines 205-209)**: `git add CLAUDE.md README.md` -> `git add CLAUDE.md` only.

   **User review point**: present ceremony diff before commit.

### Track D5: Final verification sweep

3. **Verification grep**: confirm no live references to `index.html`, `README.consumer.md`, or `.nojekyll` outside retired gallops.
4. **Run fast qualification**: `tt/rbw-tf.QualifyFast.sh`.
5. **Manual smoke**: `tt/rbw-go.OnboardMAIN.sh` and role tracks.
6. **Notch** remaining changes.
7. **Ask user for go-ahead to wrap the heat.** Do NOT auto-wrap.
8. **After wrap: user-executed push.** Agent does NOT push.

## References

- Commit b7c1dbdd — D1-D3 partial (RBSPH, RBSCO, retirements, tabtarget fix)
- `.claude/commands/rbk-prep-release.md` — ceremony rewrite target
- `Tools/rbk/vov_veiled/CLAUDE.consumer.md` — glossary decision pending

**[260410-0713] rough**

## Character

Remaining mechanical cleanup from the original D1-D5 plan. Tracks D1 (RBSPH/RBSCO), D2 (file retirements), and D3's tabtarget fix already landed in commit b7c1dbdd. What remains: the CLAUDE.consumer.md glossary decision (D3 user review point), the prep-release ceremony rewrite (D4), and the final verification sweep (D5).

## Docket

**Prerequisites**: Tracks D1-D3 partial landed (commit b7c1dbdd). README structural work (₢A5AAJ) complete.

### Track D3 remainder: CLAUDE.consumer.md glossary alignment

1. **Glossary decision** (user review point): The current 11-term glossary covers core nouns; the 16 verb terms and 4 role terms are already covered by the Verb Guide and Roles sections. Present both options (expand to 27 vs keep curated 11) and let the user pick.

### Track D4: prep-release ceremony rewrite

2. **Rewrite `.claude/commands/rbk-prep-release.md`** per ₢A5AAA Q9 delta list:
   - **Step 2 (line 35)**: "index.html promise" -> "README.md Prerequisites section"
   - **Step 8 (lines 106-115)**: Delete the `cp README.consumer.md README.md` line. Keep CLAUDE.consumer.md copy. Drop lowercase readme.md note.
   - **Step 10e (line 202)**: Delete `git rm readme.md`. Add `git rm index.html` and `git rm .nojekyll` as defensive removals.
   - **Step 10 surviving list (lines 215-226)**: Remove `index.html` and `.nojekyll` lines.
   - **Step 10f (lines 205-209)**: `git add CLAUDE.md README.md` -> `git add CLAUDE.md` only.

   **User review point**: present ceremony diff before commit.

### Track D5: Final verification sweep

3. **Verification grep**: confirm no live references to `index.html`, `README.consumer.md`, or `.nojekyll` outside retired gallops.
4. **Run fast qualification**: `tt/rbw-tf.QualifyFast.sh`.
5. **Manual smoke**: `tt/rbw-go.OnboardMAIN.sh` and role tracks.
6. **Notch** remaining changes.
7. **Ask user for go-ahead to wrap the heat.** Do NOT auto-wrap.
8. **After wrap: user-executed push.** Agent does NOT push.

## References

- Commit b7c1dbdd — D1-D3 partial (RBSPH, RBSCO, retirements, tabtarget fix)
- `.claude/commands/rbk-prep-release.md` — ceremony rewrite target
- `Tools/rbk/vov_veiled/CLAUDE.consumer.md` — glossary decision pending

**[260409-1219] rough**

## Character

Mechanical cleanup plus reorganization. File deletions, file creation, targeted edits per ₢A5AAA's resolved answers. No content judgment — ₢A5AAA drafted all the replacement text (RBSCO header, RBSPH header, ceremony delta, CLAUDE.consumer update list). The only interactive review point is the ceremony file rewrite (multi-step, high blast radius, benefits from user eyes before commit).

Depends on ₢A5AAC being complete so that retirement doesn't race with the README rewrite. After this pace notches, the heat is ready to wrap.

## Docket

**Prerequisites**: Paces AAA, AAB, AAC complete. README is in its new shape; RBRR field is active; answers are in the AAA docket.

### Track D1: RBSPH creation and RBSCO reorganization

1. **Create `Tools/rbk/vov_veiled/RBSPH-ProjectHistory.adoc`** with the header drafted verbatim in ₢A5AAA Q4 (first-class prose, not AsciiDoc comment block):

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
   ```

2. **Populate RBSPH** by moving the Significant Events table from RBSCO (currently RBSCO lines 189-301) verbatim. Preserve the `[%header,cols="1,3"]` table format. Preserve commented-out entries (e.g., `// | Feb 23, 2022 | Got Docker ID.`, `// | Apr 2025 | Attempted podman-unshare approach...`). All entries move, no cutoff.

3. **Update `RBSCO-CosmologyIntro.adoc`** header per ₢A5AAA Q10 (first-class prose, replaces current comment-block header lines 1-11):

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

4. **Strip the Significant Events section from RBSCO** (currently lines 189-301). Replace with a one-line forward pointer: `See \`RBSPH-ProjectHistory.adoc\` for the project's significant events timeline.`

5. Leave the rest of RBSCO (Overview, Accomplished Infrastructure, Vision Part One/Two, Project Direction, Links) intact as internal narrative memory.

### Track D2: File retirements

6. **Delete `index.html`** from repo root. 504-line file deletion.

7. **Delete `.nojekyll`** from repo root. Vestigial GitHub Pages marker; no github.io target after this heat.

8. **Delete `Tools/rbk/vov_veiled/README.consumer.md`**. 328-line file deletion.

### Track D3: CLAUDE.consumer.md update in place

9. **Update `Tools/rbk/vov_veiled/CLAUDE.consumer.md`** (NOT retire — per ₢A5AAA Q8):
   - Line 13: `tt/rbw-gO.Onboarding.sh` → `tt/rbw-go.OnboardMAIN.sh`
   - Glossary (lines 18-31, currently 11 terms): align with README's 27-term canonical form, or trim to a curated introductory subset. Decision made during this step — present both options to the user and let them pick.
   - Any other stale tabtarget references found during read-through (none anticipated, but verify).

   **User review point**: present the glossary alignment option before committing.

### Track D4: prep-release ceremony rewrite

10. **Rewrite `.claude/commands/rbk-prep-release.md`** per ₢A5AAA Q9 delta list:

    - **Step 2 (line 35)**: "Declared dependencies (**index.html promise**): bash, git, curl..." → "Declared dependencies (**README.md Prerequisites section**): bash, git, curl..."
    - **Step 8 (lines 106-115)**: Delete the `cp Tools/rbk/vov_veiled/README.consumer.md README.md` line. Keep the `cp Tools/rbk/vov_veiled/CLAUDE.consumer.md CLAUDE.md` line. Drop the parenthetical note about lowercase `readme.md` removal.
    - **Step 10e (line 202)**: Delete the `git rm -f --ignore-unmatch readme.md` line. Add `git rm -f --ignore-unmatch index.html` and `git rm -f --ignore-unmatch .nojekyll` as defensive removals.
    - **Step 10 surviving list (lines 215-226)**: Remove the `index.html` line AND the `.nojekyll` line. (Leave all other survivors — LICENSE, SVG, rbev-vessels/, Tools/buk/, Tools/rbk/, tt/.)
    - **Step 10f (lines 205-209)**: `git add CLAUDE.md README.md` → `git add CLAUDE.md` only. README.md is now committed on main (not produced by Step 8).

    **User review point**: present the ceremony diff. Multi-step, high blast radius — user eyes before commit.

### Track D5: Final verification sweep

11. **Final verification grep**:

    ```
    grep -rn 'index\.html\|README\.consumer\.md\|\.nojekyll' Tools/ .claude/commands/ .buk/ README.md CLAUDE.md
    ```

    Expect: only historical references in `.claude/jjm/retired/*` and active paddocks (which stay). No live code, no spec references, no command files.

12. **Run fast qualification**: `tt/rbw-tf.QualifyFast.sh`. Expect success.

13. **Manual smoke**: `tt/rbw-go.OnboardMAIN.sh` and each role track. Each should render cleanly. Retirements don't touch onboarding code, but confirm nothing broke.

14. **Notch** with intent: "Retire index.html, README.consumer.md, .nojekyll. Relocate Significant Events from RBSCO to new RBSPH-ProjectHistory.adoc. Reframe RBSCO as internal narrative memory (first-class prose header). Update CLAUDE.consumer.md stale tabtarget refs (rbw-gO.Onboarding.sh → rbw-go.OnboardMAIN.sh). Rewrite rbk-prep-release ceremony for new docs architecture (Steps 2, 8, 10e, 10f, surviving list)."

15. **Ask user for go-ahead to wrap the heat.** After wrap, ₣A5 retires; ₣A3 resumes for ₢A3AAF. **Do NOT auto-wrap** — wrap is user-confirmed.

16. **After wrap: user-executed push** to `bhyslop/recipemuster`. **Agent does NOT push.** Present the `git push` command for the user to run.

### Verification gates

- Each deletion happens only after prerequisite tracks land (no retirement before RBSPH exists)
- Ceremony rewrite gets user review before commit
- CLAUDE.consumer.md glossary alignment gets user review before commit
- Final grep returns clean of live references
- Notch happens before wrap
- **Wrap is user-confirmed (not auto)**
- **Push is user-executed (not agent)**

## References

- ₣A5 paddock — DDR, retirement scope, push discipline
- ₢A5AAA resolved answers — Q4 (RBSPH header + format), Q8 (CLAUDE.consumer.md keep+update), Q9 (ceremony delta), Q10 (RBSCO header), Q11 (sweep list)
- ₢A5AAB — RBRR field foundation (landed)
- ₢A5AAC — README rewrite (landed; must be done before this pace)
- `Tools/rbk/vov_veiled/RBSPH-ProjectHistory.adoc` — create target
- `Tools/rbk/vov_veiled/RBSCO-CosmologyIntro.adoc` — header rewrite (lines 1-11) + Significant Events removal (lines 189-301)
- `Tools/rbk/vov_veiled/CLAUDE.consumer.md` — update target (NOT delete)
- `index.html`, `.nojekyll`, `Tools/rbk/vov_veiled/README.consumer.md` — deletion targets
- `.claude/commands/rbk-prep-release.md` — ceremony rewrite target (Steps 2, 8, 10e, 10f, surviving list)

**[260409-1055] rough**

## Character

Mechanical cleanup plus reorganization. File deletions, file creation, targeted edits per Pace 1's specifications. No content judgment — Pace 1 drafted all the replacement text (RBSCO header comment, RBSPH skeleton, ceremony delta). No interactive review except for the ceremony file rewrite (which touches many steps and benefits from user eyes before commit).

Depends on Pace 3 being complete so that retirement doesn't race with the README rewrite. After this pace notches, the heat is ready to wrap.

## Docket

**Prerequisites**: Paces 1, 2, 3 complete. README is in its new shape; RBRR field is active; inventory and drafts are in the structured note.

### Track D1: RBSPH creation and RBSCO reorganization

1. **Create `Tools/rbk/vov_veiled/RBSPH-ProjectHistory.adoc`** per Pace 1's Q4 skeleton. Copy the drafted header comment verbatim. Apply the chosen format (table or bullet per Q4 decision).

2. **Populate RBSPH** with the Significant Events timeline relocated from RBSCO. Include commented-out entries verbatim. Cover the date range decided in Q4.

3. **Update `RBSCO-CosmologyIntro.adoc`** header comment per Pace 1's Q10 drafted replacement text. Verbatim paste, no rewording.

4. **Strip the Significant Events section from RBSCO.** Leave the rest of RBSCO (Overview, Accomplished Infrastructure, Vision, Project Direction, Links) intact as internal narrative memory.

### Track D2: File retirements

5. **Delete `index.html`** from the repo root. Single file deletion.

6. **Delete `Tools/rbk/vov_veiled/README.consumer.md`**. Single file deletion.

7. **Handle `Tools/rbk/vov_veiled/CLAUDE.consumer.md`** per Pace 1's Q8 audit decision:
   - If retire: delete the file
   - If keep: leave alone
   - If update: apply the edits from Pace 1

### Track D3: prep-release ceremony rewrite

8. **Rewrite `.claude/commands/rbk-prep-release.md`** per Pace 1's Q9 per-step delta list. For each prep-release step flagged by Q9, apply the specified edit.

   **User review point**: ceremony file is multi-step and high-blast-radius. Present the diff to the user before committing.

### Track D4: Lingering reference sweep

9. **Apply the reference sweep** per Pace 1's Q11 categorized list. For each "update" or "delete" hit, apply the edit.

10. **Final verification grep**:
    ```
    grep -r 'index\.html\|README\.consumer\.md' Tools/ .claude/commands/ .buk/ 2>&1
    ```
    Expect: only historical references in jjm paddocks and commit messages (which stay). No live code or spec references.

### Final — verification and conclusion

11. **Run fast qualification**: `tt/rbw-tf.QualifyFast.sh`. Expect success.

12. **Manual smoke**: `tt/rbw-go.OnboardMAIN.sh` and each of the four role tracks. Each should render cleanly. The retirements don't touch onboarding code, but context-loaded files may have moved — verify nothing broke.

13. **Notch** with intent: "Retire index.html and README.consumer.md (and CLAUDE.consumer.md per Q8 decision), relocate Significant Events from RBSCO to new RBSPH-ProjectHistory.adoc, reframe RBSCO as internal narrative memory, rewrite rbk-prep-release ceremony full-scope to match new docs architecture."

14. **Ask user for go-ahead to wrap the heat.** After wrap, ₣A5 retires; ₣A3 resumes for ₢A3AAF. **Do NOT auto-wrap** — wrap is user-confirmed.

15. **After wrap: user-executed push** to `bhyslop/recipemuster`. **Agent does NOT push.** Present the `git push` command to the user for them to run. The PR ceremony from origin to `scaleinv/recipebottle` is explicitly deferred to a future heat per the paddock's deferred list.

### Verification gates

- Each deletion happens only after prerequisite tracks land (no retirement before RBSPH exists)
- Ceremony rewrite gets user review before commit
- Final grep returns clean of live references
- Notch happens before wrap
- **Wrap is user-confirmed (not auto)**
- **Push is user-executed (not agent)**

## References

- ₣A5 paddock — DDR, retirement scope, push-destination discipline
- ₢A5AAA structured note — Q4 (RBSPH skeleton), Q8 (CLAUDE.consumer.md audit), Q9 (ceremony delta), Q10 (RBSCO header draft), Q11 (reference sweep)
- ₢A5AAB — RBRR field foundation (landed)
- ₢A5AAC — README rewrite (landed; must be done before this pace)
- `Tools/rbk/vov_veiled/RBSPH-ProjectHistory.adoc` — create target
- `Tools/rbk/vov_veiled/RBSCO-CosmologyIntro.adoc` — header update + Significant Events removal
- `Tools/rbk/vov_veiled/CLAUDE.consumer.md` — audit decision target
- `index.html`, `Tools/rbk/vov_veiled/README.consumer.md` — deletion targets
- `.claude/commands/rbk-prep-release.md` — full-scope ceremony rewrite

### retired-files-retention-audit (₢A8AAB) [rough]

**[260410-0742] rough**

Drafted from ₢A5AAE in ₣A5.

## Character

Retrospective retention review. Reading + judgment + a memo, then user-approved targeted edits to README. No blind additions — separate decision from execution. Sonnet-level judgment on what counts as "should have been retained"; user is the final arbiter.

Depends on ₢A5AAC (README rewrite) and ₢A5AAD (retirements) both landing. This pace re-reads the *pre-retirement* state of dropped content via git history and cross-checks it against the post-rewrite README to catch anything valuable that got lost in the transition. The assumption is that content-loss during a rewrite is common and hard to see from inside the rewriting motion — this pace exists because the user explicitly wants a second-look pass after the dust settles.

Cognitive posture: careful, comprehensive, adversarial-to-the-new-README in a friendly way. Ask "would a newcomer miss this?" for each dropped passage. Do not assume the rewrite was complete.

## Docket

**Prerequisites**: ₢A5AAC and ₢A5AAD both complete and notched. README.md in its new shape. Retirements landed in git so `git show` references are stable.

### Steps

1. **Enumerate retired content**. For each of the following, obtain the pre-retirement text via `git show <pre-AAD-commit>:<path>`:
   - `index.html` (504 lines)
   - `Tools/rbk/vov_veiled/README.consumer.md` (328 lines)
   - `.nojekyll` (likely just a GitHub Pages marker; no textual content worth reviewing)
   - RBSCO Significant Events section that moved to RBSPH (lines 189-301 of pre-AAD RBSCO) — the relocation is content-preserving but cross-check anyway
   - RBSCO header block (pre-AAD lines 1-11) — replaced by ₢A5AAA Q10 draft; verify nothing useful was dropped

   Note: `CLAUDE.consumer.md` is NOT in this enumeration because it was kept-and-updated per ₢A5AAA Q8, not retired. Track the edits made to it in AAD separately if review is warranted.

2. **Read each retired file end-to-end** for content, not just skim. The point is to catch specific passages that might have value.

3. **Build a retention candidate list.** For each passage of interest, record:
   - Source file and approximate line range
   - The passage itself (quoted)
   - Why it might be worth retaining
   - Initial classification:
     - **obsolete** — correctly dropped, no action
     - **better-placed-elsewhere** — valuable but belongs in a spec doc, future heat, or internal memo rather than README
     - **should-reappear-in-README** — valuable and missing from the new README; propose exact integration point

4. **Cross-check against the new README.** For each "should-reappear" candidate, verify the new README really doesn't already cover the same ground. Sometimes rewritten content covers the same territory in different words — that's not a retention failure.

5. **Produce a retention memo.** Commit as `Memos/memo-<date>-retained-content-audit.md`. Structure:
   - Header: what this memo is and when it was produced
   - For each candidate: source, passage, classification, proposed action
   - Summary: count of each classification, recommended edits to README (if any)

6. **User review point**: present the retention memo. User reads, approves or vetoes each "should-reappear" recommendation. Discuss edge cases.

7. **Apply approved edits** to README.md per user's decisions. No edits without explicit approval per candidate.

8. **Re-run cross-link verification** if new content was added — new terms need cross-linking per ₢A5AAC's pattern. Preserve the cross-linkification discipline established in AAC.

9. **Notch** with intent describing the retention outcome: "Retention audit of files retired in ₢A5AAD. Reviewed <N> passages from <files>; found <M> worth re-adding to README (applied), <K> better placed elsewhere (noted), <rest> correctly dropped." Numbers filled in based on actual audit result.

### Verification gates

- Memo committed before any README edits
- User approves each "should-reappear" recommendation individually
- Cross-link discipline preserved for any new content
- Do not auto-wrap — user decides when retention audit is closed

## References

- ₣A5 paddock — original rewrite scope for cross-checking intent
- ₢A5AAA resolved answers — original inventory, gap list, structural plan
- ₢A5AAC — README rewrite (landed)
- ₢A5AAD — retirements (landed); `index.html`, `.nojekyll`, `README.consumer.md` deleted, RBSCO reshaped
- `README.md` — post-rewrite state (target of retention edits)
- Git history of deleted files and RBSCO — source material
- `Memos/` — destination for retention memo

**[260409-1219] rough**

## Character

Retrospective retention review. Reading + judgment + a memo, then user-approved targeted edits to README. No blind additions — separate decision from execution. Sonnet-level judgment on what counts as "should have been retained"; user is the final arbiter.

Depends on ₢A5AAC (README rewrite) and ₢A5AAD (retirements) both landing. This pace re-reads the *pre-retirement* state of dropped content via git history and cross-checks it against the post-rewrite README to catch anything valuable that got lost in the transition. The assumption is that content-loss during a rewrite is common and hard to see from inside the rewriting motion — this pace exists because the user explicitly wants a second-look pass after the dust settles.

Cognitive posture: careful, comprehensive, adversarial-to-the-new-README in a friendly way. Ask "would a newcomer miss this?" for each dropped passage. Do not assume the rewrite was complete.

## Docket

**Prerequisites**: ₢A5AAC and ₢A5AAD both complete and notched. README.md in its new shape. Retirements landed in git so `git show` references are stable.

### Steps

1. **Enumerate retired content**. For each of the following, obtain the pre-retirement text via `git show <pre-AAD-commit>:<path>`:
   - `index.html` (504 lines)
   - `Tools/rbk/vov_veiled/README.consumer.md` (328 lines)
   - `.nojekyll` (likely just a GitHub Pages marker; no textual content worth reviewing)
   - RBSCO Significant Events section that moved to RBSPH (lines 189-301 of pre-AAD RBSCO) — the relocation is content-preserving but cross-check anyway
   - RBSCO header block (pre-AAD lines 1-11) — replaced by ₢A5AAA Q10 draft; verify nothing useful was dropped

   Note: `CLAUDE.consumer.md` is NOT in this enumeration because it was kept-and-updated per ₢A5AAA Q8, not retired. Track the edits made to it in AAD separately if review is warranted.

2. **Read each retired file end-to-end** for content, not just skim. The point is to catch specific passages that might have value.

3. **Build a retention candidate list.** For each passage of interest, record:
   - Source file and approximate line range
   - The passage itself (quoted)
   - Why it might be worth retaining
   - Initial classification:
     - **obsolete** — correctly dropped, no action
     - **better-placed-elsewhere** — valuable but belongs in a spec doc, future heat, or internal memo rather than README
     - **should-reappear-in-README** — valuable and missing from the new README; propose exact integration point

4. **Cross-check against the new README.** For each "should-reappear" candidate, verify the new README really doesn't already cover the same ground. Sometimes rewritten content covers the same territory in different words — that's not a retention failure.

5. **Produce a retention memo.** Commit as `Memos/memo-<date>-retained-content-audit.md`. Structure:
   - Header: what this memo is and when it was produced
   - For each candidate: source, passage, classification, proposed action
   - Summary: count of each classification, recommended edits to README (if any)

6. **User review point**: present the retention memo. User reads, approves or vetoes each "should-reappear" recommendation. Discuss edge cases.

7. **Apply approved edits** to README.md per user's decisions. No edits without explicit approval per candidate.

8. **Re-run cross-link verification** if new content was added — new terms need cross-linking per ₢A5AAC's pattern. Preserve the cross-linkification discipline established in AAC.

9. **Notch** with intent describing the retention outcome: "Retention audit of files retired in ₢A5AAD. Reviewed <N> passages from <files>; found <M> worth re-adding to README (applied), <K> better placed elsewhere (noted), <rest> correctly dropped." Numbers filled in based on actual audit result.

### Verification gates

- Memo committed before any README edits
- User approves each "should-reappear" recommendation individually
- Cross-link discipline preserved for any new content
- Do not auto-wrap — user decides when retention audit is closed

## References

- ₣A5 paddock — original rewrite scope for cross-checking intent
- ₢A5AAA resolved answers — original inventory, gap list, structural plan
- ₢A5AAC — README rewrite (landed)
- ₢A5AAD — retirements (landed); `index.html`, `.nojekyll`, `README.consumer.md` deleted, RBSCO reshaped
- `README.md` — post-rewrite state (target of retention edits)
- Git history of deleted files and RBSCO — source material
- `Memos/` — destination for retention memo

**[260409-1059] rough**

## Character

Retrospective retention review. Reading + judgment + a memo, then user-approved targeted edits to README. No blind additions — separate decision from execution. Sonnet-level judgment on what counts as "should have been retained"; user is the final arbiter.

Depends on ₢A5AAC (README rewrite) and ₢A5AAD (retirements) both landing. This pace re-reads the *pre-retirement* state of dropped content via git history and cross-checks it against the post-rewrite README to catch anything valuable that got lost in the transition. The assumption is that content-loss during a rewrite is common and hard to see from inside the rewriting motion — this pace exists because the user explicitly wants a second-look pass after the dust settles.

Cognitive posture: careful, comprehensive, adversarial-to-the-new-README in a friendly way. Ask "would a newcomer miss this?" for each dropped passage. Do not assume the rewrite was complete.

## Docket

**Prerequisites**: ₢A5AAC and ₢A5AAD both complete and notched. README.md in its new shape. Retirements landed in git so `git show` references are stable.

### Steps

1. **Enumerate retired content.** For each of the following, obtain the pre-retirement text via `git show <pre-AAD-commit>:<path>`:
   - `index.html` (full file)
   - `Tools/rbk/vov_veiled/README.consumer.md` (full file)
   - `Tools/rbk/vov_veiled/RBSCO-CosmologyIntro.adoc` — the Significant Events section that moved to RBSPH, plus any Vision/Overview content dropped during ₢A5AAC's rewrite (compare pre-AAC and post-AAC RBSCO state if RBSCO was also edited during the rewrite)
   - `Tools/rbk/vov_veiled/CLAUDE.consumer.md` — if retired per ₢A5AAA Q8 decision

2. **Read each retired file end-to-end** for content, not just skim. This is slow-mode reading — the point is to catch specific passages that might have value.

3. **Build a retention candidate list.** For each passage of interest, record:
   - Source file and approximate line range
   - The passage itself (quoted)
   - Why it might be worth retaining (audience, information content, voice)
   - Initial classification:
     - **obsolete** — correctly dropped, no action
     - **better-placed-elsewhere** — valuable but belongs in a spec doc, future heat, or internal memo rather than README
     - **should-reappear-in-README** — valuable and missing from the new README; propose exact integration point

4. **Cross-check against the new README.** For each "should-reappear" candidate, verify the new README really doesn't already cover the same ground. Sometimes rewritten content covers the same territory in different words — that's not a retention failure.

5. **Produce a retention memo.** Commit as `Memos/memo-<date>-retained-content-audit.md`. Structure:
   - Header: what this memo is and when it was produced
   - For each candidate: source, passage, classification, proposed action
   - Summary: count of each classification, recommended edits to README (if any)

6. **User review point**: present the retention memo. User reads, approves or vetoes each "should-reappear" recommendation. Discuss edge cases.

7. **Apply approved edits** to README.md per user's decisions. No edits without explicit approval per candidate.

8. **Re-run cross-link verification** if new content was added — new terms need cross-linking per ₢A5AAC's pattern. Preserve the cross-linkification discipline established in AAC.

9. **Notch** with intent describing the retention outcome: "Retention audit of files retired in ₢A5AAD. Reviewed <N> passages from <files>; found <M> worth re-adding to README (applied), <K> better placed elsewhere (noted), <rest> correctly dropped." Numbers filled in based on actual audit result.

### Verification gates

- Memo committed before any README edits
- User approves each "should-reappear" recommendation individually
- Cross-link discipline preserved for any new content
- Do not auto-wrap — user decides when retention audit is closed

## References

- ₣A5 paddock — original rewrite scope for cross-checking intent
- ₢A5AAA structured note — original term inventory, gap list, structural plan
- ₢A5AAC — README rewrite (landed)
- ₢A5AAD — retirements (landed)
- `README.md` — post-rewrite state (target of retention edits)
- Git history of `index.html`, `README.consumer.md`, `RBSCO-CosmologyIntro.adoc`, `CLAUDE.consumer.md` — source material
- `Memos/` — destination for retention memo

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 K foundry-colophon-restore
  2 H foundry-quoin-elevation
  3 C svg-content-audit
  4 D drawio-svg-vocabulary-review
  5 G readme-structure-continued
  6 I lifecycle-teardown-anchors
  7 J adversarial-testing-readme-section

KHCDGIJ
·x··xxx README.md
x···x·· CLAUDE.consumer.md
x·x···· rbm-abstract-drawio.svg
xx····· RBS0-SpecTop.adoc
··x···· .gitignore, memo-20260412-svg-content-audit.md
·x····· RBSCO-CosmologyIntro.adoc, RBSGS-GettingStarted.adoc, RBSRV-RegimeVessel.adoc
x······ RBSAK-ark_kludge.adoc, buh_handbook.sh, rbcc_Constants.sh, rbfc_FoundryCore.sh, rbho_cli.sh, rbho_onboarding.sh, rbk-claude-tabtarget-context.md, rbk-claude-theurge-ifrit-context.md, rbtdrm_manifest.rs, rbw-fA.DirectorAbjuresHallmark.sh, rbw-fO.DirectorOrdainsHallmark.sh, rbw-fV.DirectorVouchesHallmarks.sh, rbw-fk.LocalKludge.sh, rbw-fpc.RetrieverPlumbsCompact.sh, rbw-fpf.RetrieverPlumbsFull.sh, rbw-fs.RetrieverSummonsHallmark.sh, rbw-ft.DirectorTalliesHallmarks.sh, rbw-hA.DirectorAbjuresHallmark.sh, rbw-hO.DirectorOrdainsHallmark.sh, rbw-hV.DirectorVouchesHallmarks.sh, rbw-hk.LocalKludge.sh, rbw-hpc.RetrieverPlumbsCompact.sh, rbw-hpf.RetrieverPlumbsFull.sh, rbw-hs.RetrieverSummonsHallmark.sh, rbw-ht.DirectorTalliesHallmarks.sh, rbw_workbench.sh, rbz_zipper.sh

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 53 commits)

  1 H foundry-quoin-elevation
  2 I lifecycle-teardown-anchors
  3 J adversarial-testing-readme-section
  4 G readme-structure-continued
  5 C svg-content-audit
  6 D drawio-svg-vocabulary-review
  7 K foundry-colophon-restore

123456789abcdefghijklmnopqrstuvwxyz
x··································  H  1c
·xx································  I  2c
···xx······························  J  2c
·····xxxxxxxxxxxxx·····xxxx········  G  17c
····················xx·············  C  2c
······················x············  D  1c
····························xxxxxx·  K  6c
```

## Steeplechase

### 2026-04-20 10:24 - Heat - T

retirements-rbsph-ceremony-sweep

### 2026-04-12 14:21 - ₢A8AAK - W

Restored rbw-f as Foundry colophon, vacating rbw-h for exclusive Handbook use. Renamed 8 tabtargets, updated zipper/Rust/spec/doc references. Eliminated all 43 hardcoded tabtarget paths from rbho_onboarding.sh by converting to buh_tT/tI/tTc with zipper constants. Added I (imprint) and Tc/Ic (trailing cyan) combinators to buh_handbook.sh. Kindled BUWZ in rbho_cli.sh for BUK tabtarget constants. Added RBCC_onboarding_nameplate constant.

### 2026-04-12 14:21 - ₢A8AAK - n

Update architecture diagram (draw.io re-export with minor layout adjustments)

### 2026-04-12 14:20 - ₢A8AAK - n

Stage deletions of old rbw-h* tabtarget files (renames were committed with new names only, leaving deletions unstaged)

### 2026-04-12 14:17 - ₢A8AAK - n

Eliminate all hardcoded tabtarget paths from rbho_onboarding.sh. Added I (imprint) and Tc/Ic (trailing cyan arg) combinators to buh_handbook.sh. Kindled BUWZ zipper in rbho_cli.sh for BUK tabtarget constants. Converted remaining 10 buh_tc calls: 3 crucible imprint (buh_tI + RBZ_CRUCIBLE_*), 3 param1-with-args (buh_tTc), 4 BUK regime (buh_tT + BUWZ_*). Added RBCC_onboarding_nameplate constant to replace naked 'tadmor' strings. rbho_onboarding.sh now has zero hardcoded tabtarget paths.

### 2026-04-12 12:14 - ₢A8AAK - n

Convert 33 hardcoded buh_tc tabtarget paths to buh_tT with zipper constants (RBZ_ONBOARD_*, RBZ_CHARTER_RETRIEVER, RBZ_KNIGHT_DIRECTOR, RBZ_MANTLE_GOVERNOR, RBZ_LIST_SERVICE_ACCOUNTS, RBZ_INSCRIBE_RELIQUARY, RBZ_ENSHRINE_VESSEL, RBZ_LEVY_DEPOT, RBZ_LIST_DEPOT, RBZ_PAYOR_*, RBZ_RENDER_REPO, RBZ_VALIDATE_REPO). 10 calls remain: 5 crucible imprint/param1 with arguments, 1 gPI with argument, 4 BUK zipper (BUWZ not in scope).

### 2026-04-12 12:08 - ₢A8AAK - n

Restore rbw-f as Foundry colophon, vacating rbw-h for exclusive Handbook use. Renamed 8 tabtargets (rbw-h{O,k,A,t,V,s,pf,pc} -> rbw-f*), updated zipper group+enrollments, Rust manifest constants, spec/doc references. Converted 15 hardcoded buh_tc Foundry tabtarget paths in rbho_onboarding.sh to buh_tT with zipper constants (RBZ_ORDAIN_HALLMARK, RBZ_KLUDGE_VESSEL, etc.) — eliminating magic strings so future colophon renames propagate automatically. Theurge rebuilt, tabtarget context regenerated, fast qualify passes.

### 2026-04-12 11:54 - Heat - S

foundry-colophon-restore

### 2026-04-12 11:38 - ₢A8AAG - n

Add RecipeBottle anchor to heading and linkify all 12 body mentions of Recipe Bottle. 63 anchors, 501 links, zero broken.

### 2026-04-12 11:32 - ₢A8AAG - n

Rewrite Environment section from glossary-definition style to onboarding prose. Regime paragraph now distinguishes in-repo regimes (RBRV, RBRN, RBRR, RBRP) from filesystem-only regimes (RBRO, RBRA, BURS). Tabtarget paragraph leads with the critical gestalt: tab completion finds the command you want.

### 2026-04-12 11:21 - ₢A8AAG - n

Apply semantic linebreaks (sentence-per-line) throughout README.md for cleaner diffs. Restored Specific Regimes appendix and License section that were truncated during the linebreak pass. All 62 anchors and 480 links verified intact.

### 2026-04-12 11:10 - ₢A8AAG - n

Restructure README into mainline concepts + operation appendices. Mainline role/vessel sections slimmed to brief overviews with verb links. Added Appendix: Foundry Operations (23 definitions, lifecycle-organized: Infrastructure, Credentials, Supply Chain, Building, Verification, Distribution, Removal, Diagnostics) and Appendix: Crucible Operations (Charge, Quench, Rack, Hail, Scry). 15 new anchored terms (Establish, Install, Refresh, Quota, Mantle, Forfeit, ListSAs, About, Wrest, ListDepots, JWTProbe, OAuthProbe, Rack, Hail, Scry). Total: 62 anchors, 480 internal links, zero broken. Replaced CLAUDE.consumer.md 11-term glossary with trigger directive pointing to README.md.

### 2026-04-12 10:47 - ₢A8AAD - W

Vocabulary rename completed within A8AAC's drawio session — all 6 stale labels (Bottle Service, Censer, github action, OCI Compliant Container Repository, Podman VM Images, Offsite Sentry/Censer/Bottle) were renamed in the same user drawio edit that applied the content audit results. Post-edit label extraction confirmed zero stale vocabulary. No separate work needed.

### 2026-04-12 10:46 - ₢A8AAC - W

SVG content audit complete. Classified 9 candidate architecture concepts against restored diagram: 1 present (Pentacle), 8 absent-and-correctly-out-of-scope (Depot, vessel modes, hallmark, nameplate, SLSA/vouch, enshrine, roles, Cloud Build details — all provisioning/process/data-model concerns outside runtime architecture scope). User refreshed diagram in drawio: vocabulary renames (Crucible, Pentacle, Google Cloud Build), added Depot label, replaced stale registry bullet list with current 6-item GAR inventory. Added drawio temp file patterns to .gitignore. No content additions needed; no deferred itches.

### 2026-04-12 10:45 - ₢A8AAC - n

SVG content audit and diagram refresh. Classified 9 candidate concepts (1 present, 8 absent-correctly-out-of-scope). User updated diagram in drawio: renamed Bottle Service->Crucible, Censer->Pentacle, github action->Google Cloud Build, added Depot label, replaced stale registry contents with current inventory (vessel images, SBOMs, attestations, reliquary, base image mirrors, pouches). Added drawio temp file patterns to .gitignore.

### 2026-04-12 10:15 - Heat - r

moved A8AAD after A8AAC

### 2026-04-12 10:14 - Heat - r

moved A8AAC to first

### 2026-04-10 10:46 - ₢A8AAG - n

Collapse Sentry/Pentacle/Bottle from three ### subsections into bullet-anchored entries under ### Containers, parallel with ### Crucible Lifecycle

### 2026-04-10 10:44 - ₢A8AAG - n

Promote Adversarial Testing from ### under Crucible to top-level Appendix: Adversarial Test Method

### 2026-04-10 10:44 - ₢A8AAG - n

Rename Project Direction to Appendix: Roadmap; replace standalone Claude Code section with CLAUDE.md row in Reference Project tree

### 2026-04-10 10:41 - ₢A8AAG - n

Promote Adversarial Testing to ### peer of Crucible Lifecycle, fold Day-to-Day Operations into Crucible Lifecycle body

### 2026-04-10 10:38 - ₢A8AAG - n

Reduce iptables mentions from 5 to 2: keep Sentry definition and tree annotation, replace others with generic references since the mechanism is explained once

### 2026-04-10 10:36 - ₢A8AAG - n

Remove duplicated Crucible Lifecycle prose: three-container bullet list, two-layer egress paragraph, and first-packet sentence all restated what Sentry/Pentacle/Bottle definitions already say

### 2026-04-10 10:33 - ₢A8AAG - n

Remove Prerequisites and Recovery sections from README — internal operational details not needed in public-facing document

### 2026-04-10 10:30 - ₢A8AAG - n

Switch annotated tree from backtick-padded code spans to two-column table — fixes GitHub trailing-whitespace trimming that broke column alignment

### 2026-04-10 10:27 - ₢A8AAG - n

Restore Appendix: Specific Regimes as standalone section with all regime anchors; remove inline <a id> tags from annotated tree annotations

### 2026-04-10 10:24 - ₢A8AAG - n

Replace Architecture, Vessels and Nameplates, and Specific Regimes sections with unified Appendix: Reference Project — annotated tree format mapping files to concepts, regime anchors woven in situ, workstation-only regimes (BURS/RBRO/RBRA) kept as trailing definitions

### 2026-04-10 10:14 - ₢A8AAG - n

Remove Credential Safety and Testing sections from README — internal details not needed in public-facing document

### 2026-04-10 10:07 - ₢A8AAG - n

Rewrite Appendix regime definitions: corrected order (BU first, then RBR by role flow), added RBRO/RBRA, fixed RBRN to per-Nameplate not per-Vessel, dropped file paths, linked terms throughout

### 2026-04-10 09:47 - ₢A8AAG - n

Rewrite Project Direction as bullet list: VPC Service Controls explained simply, cosign separated, CDN weakness kept, Podman deferred with macOS VM reality, crucible-to-crucible networking clarifies Sentry routing constraint

### 2026-04-10 09:28 - ₢A8AAJ - W

Added Adversarial Testing subsection to Crucible Lifecycle with bullet-anchored Ifrit and Theurge definitions. Adapted docket's standalone heading placement to bullet-under-narrative pattern matching current README structure. Testing and Recovery sections preserved.

### 2026-04-10 09:27 - ₢A8AAJ - n

Add Adversarial Testing subsection to Crucible Lifecycle with bullet-anchored Ifrit and Theurge definitions. Ifrit: attack vessel with scapy/strace probing containment surfaces. Theurge: host-side orchestrator for coordinated attack+observation. Closing paragraph names Claude Code authorship and epistemic posture (evidence not proof).

### 2026-04-10 09:21 - ₢A8AAI - W

Added Unmake under Depot (reverse of Levy), Abjure and Jettison under Director (reverse of Ordain, image cleanup). Adapted docket's flat-list placement to new bulleted noun/verb structure. Cross-link sweep clean.

### 2026-04-10 09:20 - ₢A8AAI - n

Add lifecycle teardown anchors: Unmake under Depot (reverse of Levy, Payor operation), Abjure and Jettison under Director (reverse of Ordain, image cleanup). Director lead-in updated to five operations. Cross-link sweep clean — no generic teardown language to replace.

### 2026-04-10 09:17 - ₢A8AAH - W

Elevated Foundry to top-level quoin in RBS0, propagated into README and vov_veiled docs. Then reorganized README definitions into pedagogical order (Environment section with Regime/Tabtarget, noun headings with demoted verb bullet lists under lead-in sentences, Foundry/Crucible lifecycle narratives absorbing Setup/HowItWorks/DayToDay), and linked bare registry references to Depot.

### 2026-04-10 09:16 - Heat - r

moved A8AAI after A8AAG

### 2026-04-10 09:16 - Heat - r

moved A8AAG after A8AAH

### 2026-04-10 09:15 - Heat - r

moved A8AAI after A8AAG

### 2026-04-10 09:14 - ₢A8AAH - n

Link 9 unlinked 'registry' occurrences to Depot where the word means the Depot's artifact registry (Ordain, Tally, Retriever, Graft, Kludge, Hallmark, Foundry intro, Recovery). Leave unlinked: elevator pitch, Depot self-definition, Levy, Establishment, Google Artifact Registry proper noun, third-party registry.

### 2026-04-10 09:08 - ₢A8AAH - n

Convert demoted verb definitions to bullet lists under lead-in sentences (Depot egress profiles, Governor credentials, Director operations, Retriever operations, Vessel modes, Foundry supply-chain infrastructure, Crucible lifecycle). Change definition separator from period to dash throughout.

### 2026-04-10 09:07 - Heat - S

adversarial-testing-readme-section

### 2026-04-10 09:00 - ₢A8AAH - n

Reorganize README definitions into pedagogical order: Environment section (Regime, Tabtarget), noun headings with demoted verb anchors (Depot→Levy/Tethered/Airgap, Governor→Knight/Charter, Director→Ordain/Tally/Vouch, Retriever→Summon/Plumb, Vessel→Conjure/Bind/Graft/Kludge), lifecycle narratives absorbing Setup/HowItWorks/DayToDay, Enshrine/Reliquary/Pouch defined inline in Foundry Lifecycle, Charge/Quench in Crucible Lifecycle

### 2026-04-10 08:57 - Heat - S

lifecycle-teardown-anchors

### 2026-04-10 08:31 - ₢A8AAH - n

Add vision sentence (verified origin, controlled version, network safeguards) and rewrite Foundry bullet to name private cloud registry with retention

### 2026-04-10 08:22 - ₢A8AAH - n

Incorporate Foundry into README: rename Supply Chain section to Foundry, add Foundry glossary entry, update elevator pitch to name Foundry/Crucible as parallel proper nouns, reference Foundry in Important blockquote and Project Direction, use formal names in How It Works intro

### 2026-04-10 08:18 - ₢A8AAH - n

Propagate {at_foundry} into four vov_veiled docs: RBS0 at_bottle_image def, RBSCO Project Direction heading, RBSGS depot levy description, RBSRV image origin resolution. Leave alone: Podman VM supply chain refs, RBSCB internal GCB scope, historical narrative, generic supply-chain-as-concept

### 2026-04-10 08:13 - ₢A8AAH - n

Mint Foundry as top-level quoin in RBS0: mapping section entries (at_foundry, at_foundry_p) and definition site as peer to Crucible — the remote build orchestration system encompassing Depots, Vessels, Hallmarks, Rubrics, and three vessel modes

### 2026-04-10 08:09 - Heat - S

foundry-quoin-elevation

### 2026-04-10 08:00 - ₢A8AAG - n

Rewrite Important blockquote as review invitation per domain: Cloud Build (attestation chain, build isolation, toolchains) and Crucible (multi-container apparatus, unprivileged workload namespace, iptables/network enforcement). Drop supply-chain claims that were asserting strength in a scrutiny block.

### 2026-04-10 07:51 - ₢A8AAG - n

Rewrite elevator pitch as two-bullet parallel structure (Remote: egress-locked Cloud Build with provenance, Local: enforced network isolation for untrusted containers)

### 2026-04-10 07:46 - Heat - T

readme-structure-continued

### 2026-04-10 07:46 - Heat - T

readme-structure-continued

### 2026-04-10 07:46 - Heat - s

slate readme-structure-continued as first pace in A8

