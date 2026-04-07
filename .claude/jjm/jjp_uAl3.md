## Character
User experience design with iterative testing. Each role track must feel obvious to someone who has never seen Recipe Bottle before — but only shows them what's relevant to their credentials. Requires testing on fresh machines with different role profiles.

## Prerequisites
₣A1 (rbk-theurge-rust-test-infrastructure) completed before this heat. Specifically:
- ₢A1AAe renamed "consecration" → "hallmark" system-wide
- ₢A1AAb reorganized colophons from actor-organized to domain-organized
- ₢A1AAg auto-generates CLAUDE.consumer.md colophon table from zipper

README.md is now the single consumer-facing readme (formerly README.consumer.md was copied during release prep). The glossary and setup instructions live in README.md directly.

## Context

Spun off from ₣AU (rbk-mvp-3-release-finalize). The original ₢AUAAm pace attempted a single "role-aware onboarding dashboard" in `rbgm_onboarding()`. Testing on a fresh machine as a retriever-only user revealed the fundamental problem: a monolithic guide can't serve four different humans with four different credential sets. Showing a retriever the payor's OAuth ceremony is noise that erodes trust.

## Design Decision Record

### Per-Role Track Guides

**Problem**: A single onboarding script that probes all roles and shows all steps is too clumsy for any single user. Retrievers don't need to see payor steps. Directors don't need governor context. The guide must meet each user where they are.

**Decision**: Replace the monolithic `rbgm_onboarding()` with a triage guide plus per-role track guides, each decomposed into educational units.

### Dual-Mode Rendering

**Problem**: An onboarding guide needs to serve two purposes — walk a new user through setup step by step, and serve as a reference for verifying health later.

**Decision**: Every role track guide operates in one of two modes, determined automatically by probe results:

**Walkthrough mode** (any probe red):
- Compact progress indicator at top (e.g., `Step 3 of 5`)
- Only the frontier step shown, fully expanded with guidance text
- No list of completed steps above, no preview of future steps below
- User's full attention on one action

**Reference mode** (all probes green):
- Full checklist with all steps and status
- Serves as health dashboard — re-run anytime to verify nothing has regressed
- If a probe regresses (expired token, deleted key), automatically drops back to walkthrough mode at the right step

Transition is automatic and reversible. No state file, no history. Probes are the state machine.

A single reference colophon (`rbw-gOr`) shows all 14 units across all 4 roles regardless of probe state — the system health dashboard.

### Colophon Tree

```
rbw-go       Triage entry point (terminal)
rbw-gO       Parent — not terminal
├── rbw-gOR  Retriever walkthrough
├── rbw-gOD  Director walkthrough
├── rbw-gOG  Governor walkthrough
├── rbw-gOP  Payor walkthrough
└── rbw-gOr  Reference (all roles, all units — single health dashboard)
```

**Retired**: `tt/rbw-gO.Onboarding.sh` (old monolithic guide) replaced by `rbw-go.OnboardMAIN.sh`.

Terminal exclusivity holds. `rbw-go` (triage) is terminal. `rbw-gO` has children only. `rbw-gOr` (lowercase r) is distinct from `rbw-gOR` (uppercase R).

### `bug_*` Display Module with `l` Link Combinator

**Problem**: Onboarding guides need to introduce Recipe Bottle vocabulary inline — each new term defined in one sentence at the moment it's needed, with a clickable link to the README glossary for deeper reading. The link should be invisible (no URL clutter) via OSC-8 terminal hyperlinks.

**Decision**: All guides use `bug_*` functions from `bug_guide.sh` (always-visible, not subject to verbosity control). A new `l` combinator letter is added to the existing combinator system (`t`, `c`, `u`, `W`, `E`). Unlike other letters which consume one positional arg, `l` consumes two (display text, URL). The display text renders as a blue underlined clickable link.

Example: `bug_tlt "A " "hallmark" "${url}#hallmark" " is a named, vouched artifact in your depot's registry."`

Renders as one natural sentence where "hallmark" is a clickable link. No URL visible.

The `l` combinator is a BUK extension (₢A3AAG) — prerequisite infrastructure before any guide can be built.

### Random-Access Glossary with Cross-Links

**Problem**: The README Key Concepts table has 9 nouns but no verbs (summon, ordain, charge, etc.) and no HTML anchors for deep-linking from CLI guides.

**Decision**: Expand the Key Concepts table to ~26 terms (nouns + verbs + roles + access operations). Each term gets an HTML anchor (`<a id="term">`), and definitions cross-link other terms (`[vessel](#vessel)`). This creates three access patterns from one anchor set:

- **CLI walkthrough**: `bug_tlt` links from terminal into README
- **README browsing**: click a term in any definition to jump to its definition (random access)
- **CLI reference** (`rbw-gOr`): links back to README for deep reading

Terms grouped: Nouns (vessel, ark, hallmark, vouch, depot, nameplate), Runtime (sentry, pentacle, bottle, charge, quench), Build (ordain, conjure, bind, graft, kludge, enshrine), Registry (summon, plumb, tally, levy), Roles (payor, governor, director, retriever), Access (charter, knight).

### Per-Track Term Self-Containment

**Problem**: Each role track must be self-contained (a governor-only user should never need to run the retriever track first), but terms like `depot` appear across multiple tracks.

**Decision**: Each track introduces terms via `bug_tlt` on first use *within that track*. If `depot` is used in both retriever and governor, both tracks introduce it with a one-sentence definition and glossary link. The README glossary is the canonical definition; the inline sentence is the teaching moment. Repetition across tracks is correct — different users, different sessions.

Within a single track, terms are introduced once and used freely afterward. The ordering within a track establishes a learning progression: you encounter `vessel` before `hallmark` because hallmark's definition references vessel.

### Educational Unit Structure

Each unit within a role track follows a consistent pattern:
- **Concept taught**: What the user learns
- **Terms introduced**: New vocabulary via `bug_tlt`, with one-sentence definitions woven into guidance prose (not a separate definitions block)
- **Probe**: What to check
- **Guidance**: What to do (with tabtarget commands)
- **Confirmation**: Observable result

### Role Model (dependency flows downhill)
- **Payor** — owns GCP project and billing, manages multiple depots
- **Governor** — administers a single depot: SA creation, IAM grants, role management
- **Director** — builds and publishes vessel images into a depot
- **Retriever** — pulls and runs vessel images from a depot

### Triage Behavior
Only shows role tracks where credentials are detected. No guidance for absent roles — just a URL to the public docs where users learn what credentials to obtain. The CLI guide activates once you have something to probe against.

**Explicitly deferred**: Theurge test cases for onboarding guides — ₣A1 territory if needed.

## Provenance
Emerged from testing ₢AUAAm's monolithic dashboard on a fresh machine as a retriever-only user. The guide couldn't help — too much irrelevant content, no clear path for a single role. Design refined in officium ☉260407-1002: dual-mode rendering, `l` combinator, random-access glossary, per-track self-containment.

## References
- `README.md` — consumer-facing readme with Key Concepts glossary (anchor target for `bug_tlt`)
- `Tools/rbk/rbgc_Constants.sh` — home for `RBGC_PUBLIC_DOCS_URL` constant
- `Tools/rbk/rbz_zipper.sh` — colophon registration
- `Tools/rbk/rbgm_ManualProcedures.sh` — current `rbgm_onboarding()` to retire
- `Tools/buk/bug_guide.sh` — `bug_*` display module, `l` combinator home
- `https://scaleinv.github.io/recipebottle` — public project page
- ₣AU paddock — MVP-3 release context