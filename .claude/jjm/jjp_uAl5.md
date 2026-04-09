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