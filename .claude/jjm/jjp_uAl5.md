## Character

Architecture and rewrite work for the doc surface that onboarding links land on. Three concerns interlocked into one coherent unit: rewrite README.md to serve mid-onboarding glossary lookups, add an RBRR field that will eventually pin customer binaries to a specific git ref of the docs, and retire the legacy `index.html` / `README.consumer.md` publishing path. Each concern is small individually; the bundle is "the first imprint of public docs that the `buh_tlt` links can actually land on usefully."

The cognitive posture this work requires: design conversation requiring judgment for the README rewrite (voice, audience, structural choices), but mechanical for the RBRR field plumbing and retirement work. Pace 1 is reading and decision; Pace 2 is interactive editing where the user is actively in the loop on the rewrite, with mechanical execution on the field/retirement tracks.

## Prerequisites

- ₣A3 (rbk-mvp-3-onboarding-guides) provides `Tools/rbk/rbho_onboarding.sh` with 53 `buh_tlt`/`buh_tl` link calls citing 27 unique anchor targets — exactly matching README.md's glossary table. This heat decides where those links resolve.
- ₣A3 was furloughed (stabled) at the start of this heat to keep its paddock from being polluted by this side concern; ₣A3 has its own remaining work (₢A3AAF) that resumes after this heat retires.
- The current `RBGC_PUBLIC_DOCS_URL` constant at `Tools/rbk/rbgc_Constants.sh:80` points at `https://scaleinv.github.io/recipebottle` — a crufty cosmology page with no glossary anchors. Customers clicking glossary terms in onboarding land on unrelated content.

## Context

Spun off from ₢A3AAF (docs-alignment-and-cross-machine-validation) when investigation of an underline-bleed bug surfaced the deeper issue: the `buh_tlt` URLs don't land on a useful page at all. The original bug got fixed and notched in ₢A3AAF; this heat carries the architectural work the investigation revealed.

The user is not at general release. The goal of this heat is "first valid imprint" — get a real README to a real public destination so URLs are correct and the link landing surface exists, with all heavier ceremony deferred until actual release time. Future iterations will polish; this heat just gets the basic shape right.

## Design Decision Record

### Position 2: release branches for doc mutability

Customer binaries will eventually pin to a release branch (`release/v0.1.x`-style), not an immutable tag. This permits doc-only fixes to reach customers running an older binary without bumping the binary version. Three positions were considered:

1. **Immutable tags.** Customer running v1.2.3 sees v1.2.3 docs forever. Doc fixes require version bump.
2. **Release branches.** Customer running v1.2.3 sees the latest `release/v1.2.x` branch state, including doc fixes that landed after their binary shipped.
3. **Always-main.** RBRR holds `main`. No version pinning at all.

Position 2 won as the only option satisfying both "iterate on docs without re-releasing" and "customer should see docs that match their binary version."

For this heat's MVP, the RBRR field defaults to `main` — branches are deferred until first actual release. The mechanism exists, the value is settable, the URL composition works; only the actual release branch creation waits for the moment there's something to pin.

### RBRR field pins the doc ref

A new RBRR field (name TBD in Pace 1, likely `RBRR_PUBLIC_DOCS_REF`) holds the git ref that customer binaries point at. Default `main` for in-source-tree dev. Set per-release by the (future) prep-release ceremony's automated substitution step.

### RBGC composes the URL from base + ref

`RBGC_PUBLIC_DOCS_URL` becomes a composition: stable base URL constant joined to the variable RBRR ref. The split is load-bearing — the base URL structure is locked in code, only the ref segment varies per release. Approximate shape:

```bash
readonly RBGC_PUBLIC_DOCS_BASE="https://github.com/scaleinv/recipebottle/blob"
readonly RBGC_PUBLIC_DOCS_FILE="README.md"
readonly RBGC_PUBLIC_DOCS_URL="${RBGC_PUBLIC_DOCS_BASE}/${RBRR_PUBLIC_DOCS_REF}/${RBGC_PUBLIC_DOCS_FILE}"
```

Final exact form decided in Pace 1 (kindle-time vs per-call composition, validator strictness, etc.).

### README.md is the canonical link landing surface

The right page for onboarding-link landing is the glossary table, not a project tour. README.md already has the 27-row glossary table with `<a id>` anchors (capitalized via ₢A3AAH). The MVP gets this README served from the public repo with anchor resolution working via GitHub's case-insensitive markdown anchor shim — verified empirically against `bhyslop/recipemuster` during the design conversation: GitHub case-folds anchor IDs to lowercase but its JS shim does case-insensitive hash matching, so `#Payor` correctly scrolls to `user-content-payor`.

This means ₢A3AAH's capitalization work is forward-compatible across rendering paths: it doesn't help on GitHub's default rendering (which case-folds anyway) but doesn't hurt either, and remains correct for any case-preserving renderer the project may adopt later (pandoc, etc.).

### README rewrite scope

Port cosmology content from `RBSCO-CosmologyIntro.adoc` into README.md as a new intro section. Frame around two complexity domains: **Cloud Build features** and **crucible orchestration features**. Voice is third person. Lede reader is "a developer who has never heard of Recipe Bottle."

Cross-linkify ported content with `[term](#Term)` markdown links wherever a Recipe Bottle term appears — matching what `rbho_onboarding.sh` already does heavily. Embed `rbm-abstract-drawio.svg` in the intro as a static image, without interactivity. Clickable internal SVG links is a deferred polish concern requiring inline-SVG and pandoc rendering.

Drop Significant Events from README (relocates to new RBSPH per below). Drop Vision Part One/Part Two narrative as duplicating README's existing How It Works content; replace with the two-complexity-domain framing.

### Retire `index.html` and `README.consumer.md`

Both files cease to serve any purpose under the new architecture:

- **`index.html`** was a render target for `RBSCO-CosmologyIntro.adoc` (last freshened in ₢AUAAP). The cosmology content now lives in README.md (intro section) and RBSCO becomes internal narrative memory. No render target needed at the repo root.
- **`Tools/rbk/vov_veiled/README.consumer.md`** is a stale 328-line legacy template that the `rbk-prep-release` ceremony Step 8 still copies onto README.md. After this heat, that copy operation would overwrite the new authoritative root README.md on the next release — a real foot-gun. The ceremony step needs updating in tandem with the template retirement.

### Project history relocates to new RBSPH

`Tools/rbk/vov_veiled/RBSPH-ProjectHistory.adoc` is created to hold the Significant Events timeline currently embedded in RBSCO. Terse — bullet-style entries, no narrative wrapping. **Commented-out lines** (e.g., the `// | Feb 23, 2022 | Got Docker ID.` entries in current RBSCO) are preserved as historical notes the project may want to reactivate later. RBSCO sheds the timeline and becomes the project's narrative-and-vision spec doc, no longer a render target.

The RBSPH header comment establishes the file's purpose: internal project memory, never ships, lives in `vov_veiled/`.

### Push destination for first imprint

The first imprint pushes to `bhyslop/recipemuster` (origin) only. The PR ceremony from origin to `scaleinv/recipebottle-staging` and ultimately to public `scaleinv/recipebottle` is explicitly deferred to a future heat — that's release-ceremony work, not first-imprint work. This heat ends at "good push to origin."

## Explicitly Deferred

- **Pandoc rendering pipeline + inline-SVG-with-clickable-links.** Real value (brand-clean URLs, custom presentation, diagram-to-glossary navigation) but not blocking the link-landing problem. Defer until the GitHub default render is observed in real use and shown wanting.
- **Per-version github.io subdirectories.** Pre-supposes multiple released binary versions in circulation. The user is pre-release; pin when there's something to pin against.
- **prep-release ceremony integration with RBRR ref substitution.** The RBRR field is created in this heat; the ceremony's automated update of the field at release time is deferred until the first actual release ceremony.
- **RBS0 ↔ README drift audit slash command (`/rbk-audit-readme`).** The drift between RBS0's mapping section and README's glossary is real but small today. Build the audit when the cost of drift becomes visible in practice.
- **First push to `scaleinv/recipebottle` (the public destination).** This heat ends at origin push. The PR ceremony from origin to public is its own future heat — likely scheduled around first actual release.
- **Cosmology Vision rewrite as a full essay.** Pace 2 produces a working intro section; further polish/expansion of the project narrative voice is its own work.
- **Parallel `CLAUDE.consumer.md` audit.** Pace 1 will check whether the same template-copy-overwrite concern applies to `Tools/rbk/vov_veiled/CLAUDE.consumer.md`. If yes, retirement is in scope. If it's already in good shape, defer.

## Provenance

Emerged from chat in officium ☉260409-1000 during ₢A3AAF investigation of the underline-bleed bug. Architectural pivots traversed during the conversation:

1. **First framing**: render `README.md` → `index.html` with pandoc, push the rendered file to a github.io publishing target.
2. **Second framing**: retarget `buh_tlt` URLs to GitHub blob URLs, retire `index.html` entirely, rely on GitHub's default README rendering.
3. **Third framing**: per-version subdirectories under github.io with versioned `index.html` publishing.
4. **Fourth framing (settled)**: GitHub default README rendering at a versioned blob URL, with version pinning via RBRR field, README rewritten as harmonious combo of cosmology + glossary.

The case-folding concern for ₢A3AAH's capitalized anchors was resolved by empirical test: GitHub's anchor shim handles case-insensitively, so the capitalization remains forward-compatible.

The RBS0 constraint — `.adoc` files never ship, the spec language is proprietary — eliminated any approach that would have required exposing AsciiDoc syntax to customers. RBSCO and other `vov_veiled/` docs remain internal source-of-truth for definitions and project history.

The "first imprint" framing came from the user's instinct to ship a real docs surface to a real public destination, with bug-tlt links landing correctly, while explicitly deferring all the polish work that doesn't block the link-landing problem.

## References

- `Tools/rbk/rbho_onboarding.sh` — onboarding code that generates `buh_tlt` calls; consumer of the URL constant
- `Tools/rbk/rbgc_Constants.sh:80` — current `RBGC_PUBLIC_DOCS_URL` constant
- `Tools/rbk/rbrr_regime.sh` — RBRR enrollment, target for new field
- `Tools/rbk/vov_veiled/RBSRR-RegimeRepo.adoc` — spec for RBRR fields, target for new field documentation
- `README.md` — current root README, target for rewrite (348 lines, has glossary table + Setup + day-to-day + configuration)
- `index.html` — to be retired
- `Tools/rbk/vov_veiled/README.consumer.md` — stale 328-line template, to be retired
- `Tools/rbk/vov_veiled/CLAUDE.consumer.md` — parallel concern, audit in Pace 1
- `Tools/rbk/vov_veiled/RBSCO-CosmologyIntro.adoc` — source of porting content; reframes as internal narrative memory after this heat
- `Tools/rbk/vov_veiled/RBSHR-HorizonRoadmap.adoc` — to read for achieved-vs-gap audit feeding the README's project-direction section
- `Tools/rbk/vov_veiled/RBSPH-ProjectHistory.adoc` — to be created with relocated Significant Events
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc` — mapping section for canonical term definitions
- `Tools/buk/buh_handbook.sh` — handbook display module, where `buh_tltT` lives (added in ₢A3AAF, renamed from `bug_*` in ₢A3AAI)
- `.claude/commands/rbk-prep-release.md` — Step 8 references README.consumer.md, must be updated when the template retires
- ₣A3 paddock — onboarding development context