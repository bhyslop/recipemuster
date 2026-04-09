# Heat Trophy: rbk-mvp-3-onboarding-guides

**Firemark:** ₣A3
**Created:** 260402
**Retired:** 260409
**Status:** retired

## Paddock

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

## Paces

### bug-link-combinator (₢A3AAG) [complete]

**[260407-2107] complete**

## Character
Mechanical BUK extension — add a new combinator letter to an established pattern. The design is settled (two-arg `l` slot); implementation is pattern-following from existing combinators. Test with BUTT self-test framework.

## Docket

Extend `bug_guide.sh` with `l` (link) as a text combinator letter. `l` renders an OSC-8 terminal hyperlink (blue underline, clickable). Unlike other combinator letters which consume one positional arg, `l` consumes two: display text and URL, always in that order.

### Arg Convention

Every combinator letter consumes positional args left to right:
- `t` — 1 arg: text
- `c` — 1 arg: text (rendered cyan)
- `u` — 1 arg: text (rendered magenta)
- `W` — 1 arg: text (rendered yellow)
- `E` — 1 arg: text (rendered red)
- `l` — 2 args: display_text, url (rendered as blue underlined OSC-8 hyperlink)

For `l`, the first arg is what the user sees (the clickable word), the second is where it links (invisible URL). This is the only two-arg letter.

Example arg mapping for `bug_tlt`:
```
bug_tlt  "A "  "hallmark"  "https://...#hallmark"  " is a named artifact."
         ^^^^  ^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^   ^^^^^^^^^^^^^^^^^^^^^^^^
         t(1)  l(1:text)   l(2:url)                t(1)
```

### Implementation

Add to `Tools/buk/bug_guide.sh`:

**Internal helper** (parallel to existing `zbug_show`):
- `zbug_link_fragment` — returns the OSC-8 formatted link fragment for embedding in combinator output. Must respect `BURD_NO_HYPERLINKS` fallback (same as `bug_link`).

**Combinators** (initial set — expand as needed during onboarding paces):

| Function | Args | Visual sequence |
|----------|------|-----------------|
| `bug_lt` | display_text, url, suffix | link + text |
| `bug_tl` | prefix, display_text, url | text + link |
| `bug_tlt` | prefix, display_text, url, suffix | text + link + text |
| `bug_tlc` | prefix, display_text, url, code | text + link + command |
| `bug_tltlt` | prefix, display_text1, url1, middle, display_text2, url2, suffix | text + link + text + link + text |

**Convention**: In the header comment block, document that `l` consumes two positional args (display_text, url) and note this exception to the one-letter-one-arg rule. Add an arg-mapping diagram like the example above.

### Relationship to existing `bug_link`

`bug_link` remains as-is — it's the standalone four-arg version with its own prefix/suffix signature. The `l` combinator integrates links into the composable system alongside `c`, `u`, `W`, `E`. No refactoring of `bug_link` needed.

### Test

Add test cases to BUK self-test (`Tools/buk/butt_testbench.sh` or appropriate fixture):
- `bug_tlt` produces expected output with OSC-8 sequences
- `bug_tlt` with `BURD_NO_HYPERLINKS` falls back to styled text + angle-bracket URL
- Arg count matches combinator letter count (accounting for `l` = 2)

### Validation
- Run `tt/buw-st.BukSelfTest.sh` — all existing + new cases pass
- Verify no regressions in existing `bug_link` or `bug_*` combinator behavior

## References
- `Tools/buk/bug_guide.sh` — implementation target
- `Tools/buk/butt_testbench.sh` — test framework
- BUS0 (`Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc`) — may need quoin for new combinator letter

**[260407-1011] rough**

## Character
Mechanical BUK extension — add a new combinator letter to an established pattern. The design is settled (two-arg `l` slot); implementation is pattern-following from existing combinators. Test with BUTT self-test framework.

## Docket

Extend `bug_guide.sh` with `l` (link) as a text combinator letter. `l` renders an OSC-8 terminal hyperlink (blue underline, clickable). Unlike other combinator letters which consume one positional arg, `l` consumes two: display text and URL, always in that order.

### Arg Convention

Every combinator letter consumes positional args left to right:
- `t` — 1 arg: text
- `c` — 1 arg: text (rendered cyan)
- `u` — 1 arg: text (rendered magenta)
- `W` — 1 arg: text (rendered yellow)
- `E` — 1 arg: text (rendered red)
- `l` — 2 args: display_text, url (rendered as blue underlined OSC-8 hyperlink)

For `l`, the first arg is what the user sees (the clickable word), the second is where it links (invisible URL). This is the only two-arg letter.

Example arg mapping for `bug_tlt`:
```
bug_tlt  "A "  "hallmark"  "https://...#hallmark"  " is a named artifact."
         ^^^^  ^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^   ^^^^^^^^^^^^^^^^^^^^^^^^
         t(1)  l(1:text)   l(2:url)                t(1)
```

### Implementation

Add to `Tools/buk/bug_guide.sh`:

**Internal helper** (parallel to existing `zbug_show`):
- `zbug_link_fragment` — returns the OSC-8 formatted link fragment for embedding in combinator output. Must respect `BURD_NO_HYPERLINKS` fallback (same as `bug_link`).

**Combinators** (initial set — expand as needed during onboarding paces):

| Function | Args | Visual sequence |
|----------|------|-----------------|
| `bug_lt` | display_text, url, suffix | link + text |
| `bug_tl` | prefix, display_text, url | text + link |
| `bug_tlt` | prefix, display_text, url, suffix | text + link + text |
| `bug_tlc` | prefix, display_text, url, code | text + link + command |
| `bug_tltlt` | prefix, display_text1, url1, middle, display_text2, url2, suffix | text + link + text + link + text |

**Convention**: In the header comment block, document that `l` consumes two positional args (display_text, url) and note this exception to the one-letter-one-arg rule. Add an arg-mapping diagram like the example above.

### Relationship to existing `bug_link`

`bug_link` remains as-is — it's the standalone four-arg version with its own prefix/suffix signature. The `l` combinator integrates links into the composable system alongside `c`, `u`, `W`, `E`. No refactoring of `bug_link` needed.

### Test

Add test cases to BUK self-test (`Tools/buk/butt_testbench.sh` or appropriate fixture):
- `bug_tlt` produces expected output with OSC-8 sequences
- `bug_tlt` with `BURD_NO_HYPERLINKS` falls back to styled text + angle-bracket URL
- Arg count matches combinator letter count (accounting for `l` = 2)

### Validation
- Run `tt/buw-st.BukSelfTest.sh` — all existing + new cases pass
- Verify no regressions in existing `bug_link` or `bug_*` combinator behavior

## References
- `Tools/buk/bug_guide.sh` — implementation target
- `Tools/buk/butt_testbench.sh` — test framework
- BUS0 (`Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc`) — may need quoin for new combinator letter

**[260407-0956] rough**

## Character
Mechanical BUK extension — add a new combinator letter to an established pattern. The design is settled (two-arg `l` slot); implementation is pattern-following from existing combinators. Test with BUTT self-test framework.

## Docket

Extend `bug_guide.sh` with `l` (link) as a text combinator letter. `l` renders an OSC-8 terminal hyperlink (blue underline, clickable). Unlike other combinator letters which consume one positional arg, `l` consumes two: display text and URL.

### Implementation

Add to `Tools/buk/bug_guide.sh`:

**Internal helper** (parallel to existing `zbug_show`):
- `zbug_link_fragment` — returns the OSC-8 formatted link fragment for embedding in combinator output. Must respect `BURD_NO_HYPERLINKS` fallback (same as `bug_link`).

**Combinators** (initial set — expand as needed during onboarding paces):

| Function | Args | Visual sequence |
|----------|------|-----------------|
| `bug_lt` | text, url, text | link + text |
| `bug_tl` | text, text, url | text + link |
| `bug_tlt` | text, text, url, text | text + link + text |
| `bug_tlc` | text, text, url, text | text + link + command |
| `bug_tltlt` | text, text, url, text, text, url, text | text + link + text + link + text |

**Convention**: In the header comment block, document that `l` consumes two positional args (text, url) and note this exception to the one-letter-one-arg rule.

### Relationship to existing `bug_link`

`bug_link` remains as-is — it's the standalone four-arg version with its own prefix/suffix signature. The `l` combinator integrates links into the composable system alongside `c`, `u`, `W`, `E`. No refactoring of `bug_link` needed.

### Test

Add test cases to BUK self-test (`Tools/buk/butt_testbench.sh` or appropriate fixture):
- `bug_tlt` produces expected output with OSC-8 sequences
- `bug_tlt` with `BURD_NO_HYPERLINKS` falls back to styled text + angle-bracket URL
- Arg count matches combinator letter count (accounting for `l` = 2)

### Validation
- Run `tt/buw-st.BukSelfTest.sh` — all existing + new cases pass
- Verify no regressions in existing `bug_link` or `bug_*` combinator behavior

## References
- `Tools/buk/bug_guide.sh` — implementation target
- `Tools/buk/butt_testbench.sh` — test framework
- BUS0 (`Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc`) — may need quoin for new combinator letter

### onboard-triage-and-infrastructure (₢A3AAA) [complete]

**[260407-2122] complete**

## Character
Infrastructure + framework design. Mints the colophon tree, builds the random-access glossary with internal cross-links, establishes the dual-mode rendering pattern that all role tracks will use. The design conversation is done — this is wiring and iterating on the triage display until it reads right.

## Docket

Build the triage entry point, the cross-linked glossary, and the dual-mode probe/display framework that all role track guides share.

### Prerequisites
₢A3AAG (bug-link-combinator) must be complete — the `bug_tlt` and related `l` combinators are the primary teaching mechanism in all guides.

### README.md Glossary Expansion

Expand the Key Concepts table to cover all Recipe Bottle vocabulary — nouns, verbs, and roles. Each term gets:
- An HTML anchor: `<a id="term"></a>` in the term cell
- Cross-links: other terms used in definitions become `[term](#term)` links
- One-sentence user-facing definition (not spec language)

This creates three access patterns from one anchor set:
- **CLI walkthrough**: `bug_tlt` links from terminal into README
- **README browsing**: click any term in a definition to jump to its definition
- **CLI reference** (`rbw-gOr`): links back to README for deep reading

**Grouped layout** (single table, logical clusters separated by visual grouping):

Nouns:
- `vessel` — a specification for a container image
- `ark` — an immutable container image artifact in the registry, produced from a vessel
- `hallmark` — a specific build instance of a vessel, identified by timestamp
- `vouch` — cryptographic attestation proving an ark was built by trusted infrastructure
- `depot` — the facility where container images are built and stored (GCP project + registry + bucket)
- `nameplate` — per-vessel configuration tying a sentry and bottle together into a runnable unit

Runtime:
- `sentry` — security container enforcing network policies via iptables and dnsmasq
- `pentacle` — privileged container establishing the network namespace shared with the bottle
- `bottle` — your workload container, running unmodified in a controlled network environment
- `charge` — start the sentry/pentacle/bottle triad for a nameplate
- `quench` — stop and clean up a charged nameplate's containers

Build:
- `ordain` — create a hallmark with full attestation (the production build command)
- `conjure` — ordain mode: Cloud Build creates the image from source
- `bind` — ordain mode: mirror an upstream image pinned by digest
- `graft` — ordain mode: push a locally-built image to the registry
- `kludge` — build a vessel image locally for fast iteration (no registry push)
- `enshrine` — mirror upstream base images into your depot's registry

Registry:
- `summon` — pull a hallmark image from the depot to your local machine
- `plumb` — inspect an artifact's provenance (SBOM, build info, vouch chain)
- `tally` — inventory hallmarks in the registry by health status
- `levy` — provision a new depot's GCP infrastructure

Roles:
- `payor` — owns the GCP project and funds it; authenticates via OAuth
- `governor` — administers a depot: creates service accounts, manages access
- `director` — builds and publishes vessel images into a depot
- `retriever` — pulls and runs vessel images from a depot

Access:
- `charter` — create a retriever service account (governor operation)
- `knight` — create a director service account (governor operation)

**Cross-linking examples** (every definition uses links where terms reference each other):
```markdown
| <a id="hallmark"></a>**Hallmark** | A specific build instance of a [vessel](#vessel), identified by timestamp |
| <a id="summon"></a>**Summon** | Pull a [hallmark](#hallmark) image from the [depot](#depot) to your local machine |
| <a id="ordain"></a>**Ordain** | Create a [hallmark](#hallmark) with full attestation — the production build command |
```

**Also update**: the existing Role table (line 84) to add anchors on role names, and cross-link the Setup section's references to glossary terms where natural. Update the stale `tt/rbw-gO.Onboarding.sh` reference (line 98) to point to `tt/rbw-go.OnboardMAIN.sh`.

### Dual-Mode Rendering Framework

Every role track guide operates in one of two modes, determined automatically by probe results:

**Walkthrough mode** (any probe red):
- Compact progress indicator at top (e.g., `Step 3 of 5`)
- Only the frontier step shown, fully expanded with guidance text
- No list of completed steps above, no preview of future steps below
- User's full attention on one action
- New terms introduced via `bug_tlt` with OSC-8 links to README glossary anchors

**Reference mode** (all probes green):
- Full checklist with all steps and status
- Serves as health dashboard — re-run anytime to verify nothing has regressed
- If a probe regresses (expired token, deleted key), automatically drops back to walkthrough mode at the right step

Transition is automatic and reversible. No state file, no history. Probes are the state machine.

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

Terminal exclusivity holds. `rbw-go` (triage) is terminal. `rbw-gO` has children only. `rbw-gOr` (lowercase r) is distinct from `rbw-gOR` (uppercase R).

### Display Module

All guides use `bug_*` functions exclusively (always-visible, not subject to verbosity control):
- `bug_section` — track and unit headers
- `bug_tlt` — inline term definitions with glossary links
- `bug_tct` — command references in cyan
- `bug_link` — standalone links (e.g., to public docs for absent roles)
- `bug_t`, `bug_e` — prose and spacing

URL root: `RBGC_PUBLIC_DOCS_URL` kindle constant in `rbgc_Constants.sh`, pointing to public project page. All glossary links constructed as `${RBGC_PUBLIC_DOCS_URL}#anchor`.

### Infrastructure

- Mint colophon tree in zipper (`rbz_zipper.sh`) — triage + 4 walkthrough + 1 reference = 6 colophons
- Add `RBGC_PUBLIC_DOCS_URL` kindle constant in `rbgc_Constants.sh`
- Create shared probe/display framework functions (likely in a new `rbgo_guide.sh` module or within the workbench)
- Create triage tabtarget `tt/rbw-go.OnboardMAIN.sh`
- Create reference tabtarget `tt/rbw-gOr.OnboardReference.sh`
- Wire workbench routing for all new colophons

### Triage Guide (rbw-go)

Probe credential presence for each role:
- **Retriever**: SA key file at expected path
- **Director**: SA key file at expected path
- **Governor**: SA key file or project-level access
- **Payor**: OAuth credential presence (`rbro-payor.env`)

Display detected roles with `bug_link` to the walkthrough tabtarget. Absent roles get `bug_link` to public docs URL. No instructions for absent roles — just the link out.

### Retire

Remove or gut `rbgm_onboarding()` in `rbgm_ManualProcedures.sh` — replace with pointer to `rbw-go`.

## References
- `README.md` — glossary anchor targets, getting-started sections, role table
- `Tools/rbk/rbgc_Constants.sh` — URL constant home
- `Tools/rbk/rbz_zipper.sh` — colophon registration
- `Tools/rbk/rbgm_ManualProcedures.sh` — current `rbgm_onboarding()` to retire
- `Tools/buk/bug_guide.sh` — display module

**[260407-1004] rough**

## Character
Infrastructure + framework design. Mints the colophon tree, builds the random-access glossary with internal cross-links, establishes the dual-mode rendering pattern that all role tracks will use. The design conversation is done — this is wiring and iterating on the triage display until it reads right.

## Docket

Build the triage entry point, the cross-linked glossary, and the dual-mode probe/display framework that all role track guides share.

### Prerequisites
₢A3AAG (bug-link-combinator) must be complete — the `bug_tlt` and related `l` combinators are the primary teaching mechanism in all guides.

### README.md Glossary Expansion

Expand the Key Concepts table to cover all Recipe Bottle vocabulary — nouns, verbs, and roles. Each term gets:
- An HTML anchor: `<a id="term"></a>` in the term cell
- Cross-links: other terms used in definitions become `[term](#term)` links
- One-sentence user-facing definition (not spec language)

This creates three access patterns from one anchor set:
- **CLI walkthrough**: `bug_tlt` links from terminal into README
- **README browsing**: click any term in a definition to jump to its definition
- **CLI reference** (`rbw-gOr`): links back to README for deep reading

**Grouped layout** (single table, logical clusters separated by visual grouping):

Nouns:
- `vessel` — a specification for a container image
- `ark` — an immutable container image artifact in the registry, produced from a vessel
- `hallmark` — a specific build instance of a vessel, identified by timestamp
- `vouch` — cryptographic attestation proving an ark was built by trusted infrastructure
- `depot` — the facility where container images are built and stored (GCP project + registry + bucket)
- `nameplate` — per-vessel configuration tying a sentry and bottle together into a runnable unit

Runtime:
- `sentry` — security container enforcing network policies via iptables and dnsmasq
- `pentacle` — privileged container establishing the network namespace shared with the bottle
- `bottle` — your workload container, running unmodified in a controlled network environment
- `charge` — start the sentry/pentacle/bottle triad for a nameplate
- `quench` — stop and clean up a charged nameplate's containers

Build:
- `ordain` — create a hallmark with full attestation (the production build command)
- `conjure` — ordain mode: Cloud Build creates the image from source
- `bind` — ordain mode: mirror an upstream image pinned by digest
- `graft` — ordain mode: push a locally-built image to the registry
- `kludge` — build a vessel image locally for fast iteration (no registry push)
- `enshrine` — mirror upstream base images into your depot's registry

Registry:
- `summon` — pull a hallmark image from the depot to your local machine
- `plumb` — inspect an artifact's provenance (SBOM, build info, vouch chain)
- `tally` — inventory hallmarks in the registry by health status
- `levy` — provision a new depot's GCP infrastructure

Roles:
- `payor` — owns the GCP project and funds it; authenticates via OAuth
- `governor` — administers a depot: creates service accounts, manages access
- `director` — builds and publishes vessel images into a depot
- `retriever` — pulls and runs vessel images from a depot

Access:
- `charter` — create a retriever service account (governor operation)
- `knight` — create a director service account (governor operation)

**Cross-linking examples** (every definition uses links where terms reference each other):
```markdown
| <a id="hallmark"></a>**Hallmark** | A specific build instance of a [vessel](#vessel), identified by timestamp |
| <a id="summon"></a>**Summon** | Pull a [hallmark](#hallmark) image from the [depot](#depot) to your local machine |
| <a id="ordain"></a>**Ordain** | Create a [hallmark](#hallmark) with full attestation — the production build command |
```

**Also update**: the existing Role table (line 84) to add anchors on role names, and cross-link the Setup section's references to glossary terms where natural. Update the stale `tt/rbw-gO.Onboarding.sh` reference (line 98) to point to `tt/rbw-go.OnboardMAIN.sh`.

### Dual-Mode Rendering Framework

Every role track guide operates in one of two modes, determined automatically by probe results:

**Walkthrough mode** (any probe red):
- Compact progress indicator at top (e.g., `Step 3 of 5`)
- Only the frontier step shown, fully expanded with guidance text
- No list of completed steps above, no preview of future steps below
- User's full attention on one action
- New terms introduced via `bug_tlt` with OSC-8 links to README glossary anchors

**Reference mode** (all probes green):
- Full checklist with all steps and status
- Serves as health dashboard — re-run anytime to verify nothing has regressed
- If a probe regresses (expired token, deleted key), automatically drops back to walkthrough mode at the right step

Transition is automatic and reversible. No state file, no history. Probes are the state machine.

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

Terminal exclusivity holds. `rbw-go` (triage) is terminal. `rbw-gO` has children only. `rbw-gOr` (lowercase r) is distinct from `rbw-gOR` (uppercase R).

### Display Module

All guides use `bug_*` functions exclusively (always-visible, not subject to verbosity control):
- `bug_section` — track and unit headers
- `bug_tlt` — inline term definitions with glossary links
- `bug_tct` — command references in cyan
- `bug_link` — standalone links (e.g., to public docs for absent roles)
- `bug_t`, `bug_e` — prose and spacing

URL root: `RBGC_PUBLIC_DOCS_URL` kindle constant in `rbgc_Constants.sh`, pointing to public project page. All glossary links constructed as `${RBGC_PUBLIC_DOCS_URL}#anchor`.

### Infrastructure

- Mint colophon tree in zipper (`rbz_zipper.sh`) — triage + 4 walkthrough + 1 reference = 6 colophons
- Add `RBGC_PUBLIC_DOCS_URL` kindle constant in `rbgc_Constants.sh`
- Create shared probe/display framework functions (likely in a new `rbgo_guide.sh` module or within the workbench)
- Create triage tabtarget `tt/rbw-go.OnboardMAIN.sh`
- Create reference tabtarget `tt/rbw-gOr.OnboardReference.sh`
- Wire workbench routing for all new colophons

### Triage Guide (rbw-go)

Probe credential presence for each role:
- **Retriever**: SA key file at expected path
- **Director**: SA key file at expected path
- **Governor**: SA key file or project-level access
- **Payor**: OAuth credential presence (`rbro-payor.env`)

Display detected roles with `bug_link` to the walkthrough tabtarget. Absent roles get `bug_link` to public docs URL. No instructions for absent roles — just the link out.

### Retire

Remove or gut `rbgm_onboarding()` in `rbgm_ManualProcedures.sh` — replace with pointer to `rbw-go`.

## References
- `README.md` — glossary anchor targets, getting-started sections, role table
- `Tools/rbk/rbgc_Constants.sh` — URL constant home
- `Tools/rbk/rbz_zipper.sh` — colophon registration
- `Tools/rbk/rbgm_ManualProcedures.sh` — current `rbgm_onboarding()` to retire
- `Tools/buk/bug_guide.sh` — display module

**[260407-0958] rough**

## Character
Infrastructure + framework design. Mints the colophon tree, establishes the dual-mode rendering pattern that all role tracks will use, anchors the README glossary. The design conversation is done — this is wiring and iterating on the triage display until it reads right.

## Docket

Build the triage entry point and the dual-mode probe/display framework that all role track guides share.

### Prerequisites
₢A3AAG (bug-link-combinator) must be complete — the `bug_tlt` and related `l` combinators are the primary teaching mechanism in all guides.

### Dual-Mode Rendering Framework

Every role track guide operates in one of two modes, determined automatically by probe results:

**Walkthrough mode** (any probe red):
- Compact progress indicator at top (e.g., `Step 3 of 5`)
- Only the frontier step shown, fully expanded with guidance text
- No list of completed steps above, no preview of future steps below
- User's full attention on one action
- New terms introduced via `bug_tlt` with OSC-8 links to README glossary anchors

**Reference mode** (all probes green):
- Full checklist with all steps and status
- Serves as health dashboard — re-run anytime to verify nothing has regressed
- If a probe regresses (expired token, deleted key), automatically drops back to walkthrough mode at the right step

Transition is automatic and reversible. No state file, no history. Probes are the state machine.

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

Terminal exclusivity holds. `rbw-go` (triage) is terminal. `rbw-gO` has children only. `rbw-gOr` (lowercase r) is distinct from `rbw-gOR` (uppercase R).

### Display Module

All guides use `bug_*` functions exclusively (always-visible, not subject to verbosity control):
- `bug_section` — track and unit headers
- `bug_tlt` — inline term definitions with glossary links
- `bug_tct` — command references in cyan
- `bug_link` — standalone links (e.g., to public docs for absent roles)
- `bug_t`, `bug_e` — prose and spacing

URL root: `RBGC_PUBLIC_DOCS_URL` kindle constant in `rbgc_Constants.sh`, pointing to public project page. All glossary links constructed as `${RBGC_PUBLIC_DOCS_URL}#anchor`.

### Infrastructure

- Mint colophon tree in zipper (`rbz_zipper.sh`) — triage + 4 walkthrough + 1 reference = 6 colophons
- Add `RBGC_PUBLIC_DOCS_URL` kindle constant in `rbgc_Constants.sh`
- Create shared probe/display framework functions (likely in a new `rbgo_guide.sh` module or within the workbench)
- Create triage tabtarget `tt/rbw-go.OnboardMAIN.sh`
- Create reference tabtarget `tt/rbw-gOr.OnboardReference.sh`
- Wire workbench routing for all new colophons

### README.md Glossary Anchoring

Add anchor IDs to the Key Concepts table and Role table in README.md so `bug_tlt` can deep-link:
- Term anchors: vessel, ark, hallmark, vouch, depot, sentry, pentacle, bottle, nameplate
- Role anchors: payor, governor, director, retriever
- Add missing operation verbs to glossary: summon, ordain (conjure/bind/graft), enshrine, vouch, tally, plumb, charge, quench, levy, kludge, graft

### Triage Guide (rbw-go)

Probe credential presence for each role:
- **Retriever**: SA key file at expected path
- **Director**: SA key file at expected path
- **Governor**: SA key file or project-level access
- **Payor**: OAuth credential presence (`rbro-payor.env`)

Display detected roles with `bug_link` to the walkthrough tabtarget. Absent roles get `bug_link` to public docs URL. No instructions for absent roles — just the link out.

### Retire

Remove or gut `rbgm_onboarding()` in `rbgm_ManualProcedures.sh` — replace with pointer to `rbw-go`.

## References
- `README.md` — glossary anchor targets, getting-started sections
- `Tools/rbk/rbgc_Constants.sh` — URL constant home
- `Tools/rbk/rbz_zipper.sh` — colophon registration
- `Tools/rbk/rbgm_ManualProcedures.sh` — current `rbgm_onboarding()` to retire
- `Tools/buk/bug_guide.sh` — display module

**[260407-0941] rough**

## Character
Infrastructure + framework design. Mints the colophon tree, establishes the dual-mode rendering pattern that all role tracks will use, anchors the README glossary. The design conversation is done — this is wiring and iterating on the triage display until it reads right.

## Docket

Build the triage entry point and the dual-mode probe/display framework that all role track guides share.

### Dual-Mode Rendering Framework

Every role track guide operates in one of two modes, determined automatically by probe results:

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

### Colophon Design

Walkthrough and reference are sibling colophons per role:

| Walkthrough | Reference | Role |
|-------------|-----------|------|
| `rbw-go` | — | Triage (single mode — always shows all roles) |
| `rbw-gOR` | `rbw-goR` | Retriever |
| `rbw-gOD` | `rbw-goD` | Director |
| `rbw-gOG` | `rbw-goG` | Governor |
| `rbw-gOP` | `rbw-goP` | Payor |

`rbw-gO` (uppercase O) prefixes walkthrough tracks. `rbw-go` (lowercase o) is triage when terminal, reference prefix when parent. Terminal exclusivity: `rbw-go` has children `R/D/G/P`, so triage needs its own terminal — use `rbw-go` as triage (it IS terminal today; the reference colophons `rbw-goR` etc. are children of a *different* prefix `rbw-go` — wait, that breaks terminal exclusivity).

**Revised**: `rbw-go` is triage (terminal). Reference colophons need a different parent. Options:
- `rbw-gfR` / `rbw-gfD` / `rbw-gfG` / `rbw-gfP` (f = full reference)
- Or: reference mode is a flag on the walkthrough colophon, not a separate colophon

Resolve the colophon naming during implementation. The key requirement: both modes must be reachable from `ls tt/` tab completion.

### Infrastructure

- Mint colophon tree in zipper (`rbz_zipper.sh`) — walkthrough + reference colophons for all 4 roles, plus triage
- Add `RBGC_PUBLIC_DOCS_URL` kindle constant in `rbgc_Constants.sh`, pointing to public project page
- Create shared probe/display framework functions (likely in a new `rbgo_guide.sh` module or within the workbench)
- Create triage tabtarget `tt/rbw-go.OnboardMAIN.sh`
- Wire workbench routing for all new colophons

### README.md Glossary Anchoring

Add anchor IDs to the Key Concepts table and Role table in README.md so `buc_link` can deep-link:
- Term anchors: vessel, ark, hallmark, vouch, depot, sentry, pentacle, bottle, nameplate
- Role anchors: payor, governor, director, retriever
- Add missing operation verbs to glossary: summon, ordain (conjure/bind/graft), enshrine, vouch, tally, plumb, charge, quench

### Triage Guide (rbw-go)

Probe credential presence for each role:
- **Retriever**: SA key file at expected path
- **Director**: SA key file at expected path
- **Governor**: SA key file or project-level access
- **Payor**: OAuth credential presence (`rbro-payor.env`)

Display detected roles with `buc_link` to the walkthrough tabtarget. Absent roles get `buc_link` to public docs URL. No instructions for absent roles — just the link out.

### Retire

Remove or gut `rbgm_onboarding()` in `rbgm_ManualProcedures.sh` — replace with pointer to `rbw-go`.

## References
- `README.md` — glossary anchor targets, getting-started sections
- `Tools/rbk/rbgc_Constants.sh` — URL constant home
- `Tools/rbk/rbz_zipper.sh` — colophon registration
- `Tools/rbk/rbgm_ManualProcedures.sh` — current `rbgm_onboarding()` to retire
- `Tools/buk/buc_command.sh` — `buc_link` for terminal URLs

**[260403-0943] rough**

## Character
Infrastructure + iterative UX — mint the colophon tree, wire the constant, add glossary anchors to README.md, then build and refine the triage guide until it reads right.

## Docket

Build the triage entry point that detects which roles a user has credentials for and points them to the right track.

### Infrastructure

- Mint `rbw-go` and `rbw-gO` colophon tree in zipper (`rbz_zipper.sh`)
- Add `RBGC_PUBLIC_DOCS_URL` kindle constant in `rbgc_Constants.sh`, initially pointing to fork repo
- Create tabtarget `tt/rbw-go.OnboardMAIN.sh`
- Wire workbench routing for `rbw-go` colophon
- Retire `tt/rbw-gO.Onboarding.sh` (old monolithic guide)

### README.md glossary anchoring

Add anchor IDs to the Key Concepts table and Role table in README.md so `buc_link` can deep-link:
- Term anchors: vessel, ark, hallmark, vouch, depot, sentry, pentacle, bottle, nameplate
- Role anchors: payor, governor, director, retriever
- Add missing operation verbs to glossary: summon, ordain (conjure/bind/graft), enshrine, vouch, tally, plumb, charge, quench

### Triage guide (rbw-go.OnboardMAIN.sh)

Probe credential presence for each role:
- **Retriever**: SA key file at expected path
- **Director**: SA key file at expected path
- **Governor**: SA key file or project-level access
- **Payor**: OAuth credential presence (`rbro-payor.env`)

Display:
- Detected roles with `buc_link` to the corresponding `rbw-gO*` tabtarget
- Absent roles with `buc_link` to public docs URL for "how to get credentialed"
- No instructions for absent roles — just the link out

### Iterate with README.md

The triage guide links back to README.md glossary and getting-started sections. Refine both sides until the handoff feels natural: CLI tells you what you have, docs tell you how to get what you don't.

### Retire

Remove or gut `rbgm_onboarding()` in `rbgm_ManualProcedures.sh` — replace with a pointer to `rbw-go`.

## References
- `README.md` — glossary anchor targets, getting-started sections
- `Tools/rbk/rbgc_Constants.sh` — URL constant home
- `Tools/rbk/rbz_zipper.sh` — colophon registration
- `Tools/rbk/rbgm_ManualProcedures.sh` — current `rbgm_onboarding()`
- `Tools/buk/buc_command.sh` — `buc_link`

**[260402-1617] rough**

## Character
Infrastructure + iterative UX — mint the colophon tree, wire the constant, then build and refine the triage guide until it reads right.

## Docket

Build the triage entry point that detects which roles a user has credentials for and points them to the right track.

### Infrastructure

- Mint `rbw-go` and `rbw-gO` colophon tree in zipper (`rbz_zipper.sh`)
- Add `RBGC_PUBLIC_DOCS_URL` kindle constant in `rbgc_Constants.sh`, initially pointing to fork repo
- Create tabtarget `tt/rbw-go.OnboardMAIN.sh`
- Wire workbench routing for `rbw-go` colophon
- Retire `tt/rbw-gO.Onboarding.sh` (old monolithic guide)

### Triage guide (rbw-go.OnboardMAIN.sh)

Probe credential presence for each role:
- **Retriever**: SA key file at expected path
- **Director**: SA key file at expected path
- **Governor**: SA key file or project-level access
- **Payor**: OAuth credential presence (`rbro-payor.env`)

Display:
- Detected roles with `buc_link` to the corresponding `rbw-gO*` tabtarget
- Absent roles with `buc_link` to public docs URL for "how to get credentialed"
- No instructions for absent roles — just the link out

### Iterate with README.md

The triage guide links back to README.md / getting-started docs. Refine both sides until the handoff feels natural: CLI tells you what you have, docs tell you how to get what you don't.

### Retire

Remove or gut `rbgm_onboarding()` in `rbgm_ManualProcedures.sh` — replace with a pointer to `rbw-go`.

## References
- `Tools/rbk/rbgc_Constants.sh` — URL constant home
- `Tools/rbk/rbz_zipper.sh` — colophon registration
- `Tools/rbk/rbgm_ManualProcedures.sh` — current `rbgm_onboarding()`
- `Tools/buk/buc_command.sh` — `buc_link`

**[260402-1520] rough**

## Character
Infrastructure + iterative UX — mint the colophon tree, wire the constant, then build and refine the triage guide until it reads right.

## Docket

Build the triage entry point that detects which roles a user has credentials for and points them to the right track.

### Infrastructure

- Mint `rbw-o` and `rbw-O` colophon tree in zipper (`rbz_zipper.sh`)
- Add `RBGC_PUBLIC_DOCS_URL` kindle constant in `rbgc_Constants.sh`, initially pointing to fork repo
- Create tabtarget `tt/rbw-o.OnboardMAIN.sh`
- Wire workbench routing for `rbw-o` colophon

### Triage guide (rbw-o.OnboardMAIN.sh)

Probe credential presence for each role:
- **Retriever**: SA key file at expected path
- **Director**: SA key file at expected path
- **Governor**: SA key file or project-level access
- **Payor**: OAuth credential presence (`rbro-payor.env`)

Display:
- Detected roles with `buc_link` to the corresponding `rbw-O*` tabtarget
- Absent roles with `buc_link` to public docs URL for "how to get credentialed"
- No instructions for absent roles — just the link out

### Iterate with README.md

The triage guide links back to README.md / getting-started docs. Refine both sides until the handoff feels natural: CLI tells you what you have, docs tell you how to get what you don't.

### Retire

Remove or gut `rbgm_onboarding()` in `rbgm_ManualProcedures.sh` — replace with a pointer to `rbw-o`.

## References
- `Tools/rbk/rbgc_Constants.sh` — URL constant home
- `Tools/rbk/rbz_zipper.sh` — colophon registration
- `Tools/rbk/rbgm_ManualProcedures.sh` — current `rbgm_onboarding()`
- `Tools/buk/buc_command.sh` — `buc_link`

**[260402-1520] rough**

Drafted from ₢AUAAm in ₣AU.

## Character
Iterative testing and refinement. The core implementation is done — role-aware dashboard replaces the linear staircase. Now it needs to be exercised against real machine states to find remaining issues.

## Docket
Continue testing and refining the role-aware onboarding dashboard. The implementation is structurally complete (5 commits so far). Key design decisions already landed:

- Payor role = OAuth credential presence (rbro-payor.env), not committed rbrp.env config
- Independent role probes, not sequential levels
- Per-role status tracks: payor, director, retriever (shown only when role is present)
- `buv_export_and_lock` added to BUV so all BURC variables cross the exec boundary
- Regime variable names always displayed with prefix for user contextualization

### What to test next

**Cerebro** (no credentials — all roles empty):
- Last clean run shows: role inventory (all empty), credential guidance, "Next Step" with two paths. Looks good.
- Verify no regressions after prefix-label commit (unpushed at session end, now pushed).

**Local workstation** (has payor OAuth + service accounts):
- Exercise the payor track (OAuth present, project configured, depot created, governor reset)
- Exercise the director track (reliquary, vessel builds)
- Exercise the retriever track (consecrations, summoned images)
- Verify the "Credential Guidance" section is suppressed when all roles are present

**Edge cases to probe:**
- Machine with only retriever credentials (the "retriever-only" path from docket research)
- Machine with director but no retriever (or vice versa)
- What happens if rbrr.env is missing entirely (fresh clone before any config)

### Scope boundary (unchanged)
RBRR regime validators enforce all fields even when irrelevant to a role. The onboarding probes use presence/absence checks, not regime validation, so this doesn't block.

## References
- `Tools/rbk/rbgm_ManualProcedures.sh` — `rbgm_onboarding()` (lines 485-800)
- `Tools/buk/buv_validation.sh` — `buv_export_and_lock()` (new)
- `Tools/buk/burc_regime.sh` — uses `buv_export_and_lock BURC`
- `Tools/rbk/rbdc_DerivedConstants.sh` — credential path resolution

**[260402-1519] rough**

Drafted from ₢AUAAm in ₣AU.

## Character
Iterative testing and refinement. The core implementation is done — role-aware dashboard replaces the linear staircase. Now it needs to be exercised against real machine states to find remaining issues.

## Docket
Continue testing and refining the role-aware onboarding dashboard. The implementation is structurally complete (5 commits so far). Key design decisions already landed:

- Payor role = OAuth credential presence (rbro-payor.env), not committed rbrp.env config
- Independent role probes, not sequential levels
- Per-role status tracks: payor, director, retriever (shown only when role is present)
- `buv_export_and_lock` added to BUV so all BURC variables cross the exec boundary
- Regime variable names always displayed with prefix for user contextualization

### What to test next

**Cerebro** (no credentials — all roles empty):
- Last clean run shows: role inventory (all empty), credential guidance, "Next Step" with two paths. Looks good.
- Verify no regressions after prefix-label commit (unpushed at session end, now pushed).

**Local workstation** (has payor OAuth + service accounts):
- Exercise the payor track (OAuth present, project configured, depot created, governor reset)
- Exercise the director track (reliquary, vessel builds)
- Exercise the retriever track (consecrations, summoned images)
- Verify the "Credential Guidance" section is suppressed when all roles are present

**Edge cases to probe:**
- Machine with only retriever credentials (the "retriever-only" path from docket research)
- Machine with director but no retriever (or vice versa)
- What happens if rbrr.env is missing entirely (fresh clone before any config)

### Scope boundary (unchanged)
RBRR regime validators enforce all fields even when irrelevant to a role. The onboarding probes use presence/absence checks, not regime validation, so this doesn't block.

## References
- `Tools/rbk/rbgm_ManualProcedures.sh` — `rbgm_onboarding()` (lines 485-800)
- `Tools/buk/buv_validation.sh` — `buv_export_and_lock()` (new)
- `Tools/buk/burc_regime.sh` — uses `buv_export_and_lock BURC`
- `Tools/rbk/rbdc_DerivedConstants.sh` — credential path resolution

**[260328-1019] rough**

## Character
Iterative testing and refinement. The core implementation is done — role-aware dashboard replaces the linear staircase. Now it needs to be exercised against real machine states to find remaining issues.

## Docket
Continue testing and refining the role-aware onboarding dashboard. The implementation is structurally complete (5 commits so far). Key design decisions already landed:

- Payor role = OAuth credential presence (rbro-payor.env), not committed rbrp.env config
- Independent role probes, not sequential levels
- Per-role status tracks: payor, director, retriever (shown only when role is present)
- `buv_export_and_lock` added to BUV so all BURC variables cross the exec boundary
- Regime variable names always displayed with prefix for user contextualization

### What to test next

**Cerebro** (no credentials — all roles empty):
- Last clean run shows: role inventory (all empty), credential guidance, "Next Step" with two paths. Looks good.
- Verify no regressions after prefix-label commit (unpushed at session end, now pushed).

**Local workstation** (has payor OAuth + service accounts):
- Exercise the payor track (OAuth present, project configured, depot created, governor reset)
- Exercise the director track (reliquary, vessel builds)
- Exercise the retriever track (consecrations, summoned images)
- Verify the "Credential Guidance" section is suppressed when all roles are present

**Edge cases to probe:**
- Machine with only retriever credentials (the "retriever-only" path from docket research)
- Machine with director but no retriever (or vice versa)
- What happens if rbrr.env is missing entirely (fresh clone before any config)

### Scope boundary (unchanged)
RBRR regime validators enforce all fields even when irrelevant to a role. The onboarding probes use presence/absence checks, not regime validation, so this doesn't block.

## References
- `Tools/rbk/rbgm_ManualProcedures.sh` — `rbgm_onboarding()` (lines 485-800)
- `Tools/buk/buv_validation.sh` — `buv_export_and_lock()` (new)
- `Tools/buk/burc_regime.sh` — uses `buv_export_and_lock BURC`
- `Tools/rbk/rbdc_DerivedConstants.sh` — credential path resolution

**[260328-0948] rough**

## Character
Design conversation with incremental implementation. The current onboarding is a 14-level linear staircase assuming one person does everything. This pace replaces it with a role-aware dashboard that adapts to what credentials are present on the machine.

## Docket
Replace `rbgm_onboarding()` with a role-aware dashboard. The existing 14-level staircase is retired — this is a full replacement, not a wrapper around it.

### Phase 1: Role inventory + credential guidance

**Role inventory section** — probe for each RBRA file independently (retriever, director, governor) plus payor (rbrp.env + OAuth). Show what roles are active on this machine. Probes read files and check value presence/absence — no regime validation at this stage (onboarding runs pre-kindle, before regimes are fully valid).

**RBRA credential guidance** — for absent roles, explain what an RBRA file is, where it goes (`RBRR_SECRETS_DIR`), expected filename per role, required permissions (600). This teaches users who *received* a credential file how to emplace it without running the creation commands.

Use `buc_tabtarget` for all command suggestions — maintain the existing pattern where the dashboard points users at specific tabtargets for their next action.

### Phase 2: Per-role status tracks (iterative, after Phase 1 works)

For each detected role, show its specific readiness. Priority order: payor first (if present), then director, then retriever. Director before retriever because you build a consecration before you use it.

- **Payor**: project configured → OAuth installed → depot created → governor reset
- **Director**: creds present → reliquary inscribed → vessels built → vouched
- **Retriever**: creds present → nameplate consecrations populated → images summoned → bottle runs

Governor doesn't get its own track — it's a sub-step of payor (create depot, then reset governor to administer the depot).

### Scope boundary

RBRR regime validators enforce all fields even when irrelevant to a role. Accept this for now — the onboarding probes use presence/absence checks, not regime validation, so this doesn't block. Relaxing validation is future work if it matters.

## References
- `Tools/rbk/rbgm_ManualProcedures.sh` — current `rbgm_onboarding()` (lines 485–880)
- `Tools/rbk/vov_veiled/CLAUDE.consumer.md` — role documentation
- `Tools/rbk/rbdc_DerivedConstants.sh` — credential path resolution

**[260328-0943] rough**

## Character
Design conversation with incremental implementation. The onboarding guide today is a 14-level linear staircase assuming one person does everything. This pace restructures it into a role-aware dashboard that adapts to what credentials are present on the machine.

## Docket
Restructure `rbgm_onboarding()` from a single linear staircase into a role-aware dashboard:

1. **Role inventory section** — probe for each RBRA file independently (retriever, director, governor) plus payor (rbrp.env + OAuth). Show what roles are active on this machine.

2. **RBRA credential guidance** — for absent roles, explain what an RBRA file is, where it goes (`RBRR_SECRETS_DIR`), expected filename per role, required permissions (600). This teaches users who *received* a credential file how to emplace it.

3. **Per-role status tracks** — for each detected role, show its specific readiness:
   - Retriever: creds → nameplate consecrations → images summoned → bottle runs
   - Director: creds → reliquary inscribed → vessels built → vouched
   - Governor: creds → directors/retrievers issued
   - Payor: project → OAuth → depot → governor reset

4. **Next step guidance** — scoped to the most actionable detected role, not the global staircase.

Key insight: RBRA file presence IS the role declaration. No flags, no parameters. The filesystem is the configuration.

Research finding: retriever-only machines need only two out-of-repo files: `burs.env` (station skeleton) and `rbra-retriever.env`. Everything else (RBRR, RBRN, BURC) is committed.

## References
- `Tools/rbk/rbgm_ManualProcedures.sh` — current `rbgm_onboarding()` (lines 485–880)
- `Tools/rbk/vov_veiled/CLAUDE.consumer.md` — role documentation
- `Tools/rbk/rbdc_DerivedConstants.sh` — credential path resolution

### onboard-retriever-track (₢A3AAB) [complete]

**[260407-2144] complete**

## Character
Framework-establishing pace. Builds the simplest role track (fewest steps, read-only operations plus local experimentation) and in doing so creates the probe/display patterns that all subsequent tracks reuse. Each unit is a standalone lesson: one concept introduced via `bug_tlt`, one action, one observable result.

## Docket

Build the retriever walkthrough guide (`tt/rbw-gOR.OnboardRetriever.sh`). Four educational units, each building on the previous gate. This is the first track built — the probe/display framework established here is reused by all subsequent tracks.

### Term Self-Containment

This track introduces all terms it uses. A retriever-only user encounters every Recipe Bottle concept they need through this track alone. Terms are introduced via `bug_tlt` on first use within the track — one sentence woven into the guidance prose, with an OSC-8 link to the README glossary.

**Full term introduction set for this track:**
- Unit 1: `depot`, `retriever`
- Unit 2: `vessel`, `hallmark`, `summon`, `vouch`, `plumb`
- Unit 3: `bottle`, `nameplate`, `sentry`, `pentacle`, `charge`, `quench`
- Unit 4: `kludge`

### Unit 1 — Credential Gate

**Concept taught**: SA keys, depot access model. A retriever is someone who can read from a depot.

**Terms introduced** (via `bug_tlt` with glossary links):
- `depot` — the facility where container images are built and stored
- `retriever` — a role with read access to a depot

**Probe**: SA key file exists at expected credential path; HEAD request against GAR succeeds with SA token.

**Guidance**: How to install the SA key file. Where it came from (governor chartered it). What "GAR access" means in one sentence.

**Confirmation**: Registry responds. Green.

### Unit 2 — First Artifact

**Concept taught**: Vessels, hallmarks, summoning, provenance. An artifact has a specification (vessel), a build identity (hallmark), can be pulled (summoned), carries proof of origin (vouch), and can be inspected (plumbed).

**Terms introduced**:
- `vessel` — a specification for a container image
- `hallmark` — a specific build instance of a vessel, identified by timestamp
- `summon` — pull a hallmark image from the depot to your local machine
- `vouch` — cryptographic attestation proving the artifact was built by trusted infrastructure
- `plumb` — inspect an artifact's provenance (SBOM, build info, vouch chain)

**Probe**: A hallmark has been summoned locally (local image exists).

**Guidance**: Run `tt/rbw-hs.RetrieverSummonsHallmark.sh` with a specific vessel and hallmark. Then inspect with `tt/rbw-hpf.RetrieverPlumbsFull.sh` or `tt/rbw-hpc.RetrieverPlumbsCompact.sh`. Point out: SBOM, build info, vouch chain — this is how you know what you're running.

**Confirmation**: Local image present, provenance metadata visible. Green.

### Unit 3 — Container Runtime

**Concept taught**: Bottles, nameplates, the sentry/pentacle/bottle triad. A nameplate defines a security configuration. Charging it starts three containers that work together. Quenching stops them.

**Terms introduced**:
- `bottle` — your workload container, running unmodified in a controlled network environment
- `nameplate` — per-vessel configuration tying a sentry and bottle together into a runnable unit
- `sentry` — security container enforcing network policies via iptables and dnsmasq
- `pentacle` — privileged container establishing the network namespace shared with the bottle
- `charge` — start the sentry/pentacle/bottle triad for a nameplate
- `quench` — stop and clean up a charged nameplate's containers

**Probe**: A crucible is currently charged for a nameplate.

**Guidance**: Run `tt/rbw-cC.Charge.<nameplate>.sh`. Explain what just happened: sentry (network jailer), pentacle (namespace setup), bottle (the workload). Suggest `tt/rbw-cr.Rack.<nameplate>.sh` to shell in and look around. When done, quench with `tt/rbw-cQ.Quench.<nameplate>.sh`.

**Confirmation**: Containers running. Green.

### Unit 4 — Local Experimentation

**Concept taught**: Kludge lets you build a vessel image locally for experimentation. No registry, no credentials beyond Docker — just build and test.

**Terms introduced**:
- `kludge` — build a vessel image locally for fast iteration (no registry push)

**Probe**: A locally-kludged image exists for a vessel.

**Guidance**: Run `tt/rbw-hk.LocalKludge.sh` to build a vessel locally. Charge the nameplate to test your local build, rack in and look around. Kludge is the retriever's experimentation tool — iterate on your local environment without needing director credentials or Cloud Build.

**Confirmation**: Local kludge image present. Green.

### Rendering

Walkthrough (`rbw-gOR`) shows only the frontier unit with full guidance and inline term definitions. Reference (`rbw-gOr`) shows retriever units alongside all other role tracks. Framework functions built here are reused by all subsequent track paces.

### Wire
- Mint `rbw-gOR` colophon in zipper
- Create walkthrough tabtarget
- Wire workbench routing
- Add retriever section to reference view (`rbw-gOr`)

## References
- `Tools/rbk/rbdc_DerivedConstants.sh` — credential path resolution
- `Tools/rbk/rbga_ArtifactRegistry.sh` — GAR access patterns
- Tabtargets: `rbw-hs` (summon), `rbw-hpf`/`rbw-hpc` (plumb), `rbw-cC` (charge), `rbw-cQ` (quench), `rbw-cr` (rack), `rbw-hk` (kludge)

**[260407-1021] rough**

## Character
Framework-establishing pace. Builds the simplest role track (fewest steps, read-only operations plus local experimentation) and in doing so creates the probe/display patterns that all subsequent tracks reuse. Each unit is a standalone lesson: one concept introduced via `bug_tlt`, one action, one observable result.

## Docket

Build the retriever walkthrough guide (`tt/rbw-gOR.OnboardRetriever.sh`). Four educational units, each building on the previous gate. This is the first track built — the probe/display framework established here is reused by all subsequent tracks.

### Term Self-Containment

This track introduces all terms it uses. A retriever-only user encounters every Recipe Bottle concept they need through this track alone. Terms are introduced via `bug_tlt` on first use within the track — one sentence woven into the guidance prose, with an OSC-8 link to the README glossary.

**Full term introduction set for this track:**
- Unit 1: `depot`, `retriever`
- Unit 2: `vessel`, `hallmark`, `summon`, `vouch`, `plumb`
- Unit 3: `bottle`, `nameplate`, `sentry`, `pentacle`, `charge`, `quench`
- Unit 4: `kludge`

### Unit 1 — Credential Gate

**Concept taught**: SA keys, depot access model. A retriever is someone who can read from a depot.

**Terms introduced** (via `bug_tlt` with glossary links):
- `depot` — the facility where container images are built and stored
- `retriever` — a role with read access to a depot

**Probe**: SA key file exists at expected credential path; HEAD request against GAR succeeds with SA token.

**Guidance**: How to install the SA key file. Where it came from (governor chartered it). What "GAR access" means in one sentence.

**Confirmation**: Registry responds. Green.

### Unit 2 — First Artifact

**Concept taught**: Vessels, hallmarks, summoning, provenance. An artifact has a specification (vessel), a build identity (hallmark), can be pulled (summoned), carries proof of origin (vouch), and can be inspected (plumbed).

**Terms introduced**:
- `vessel` — a specification for a container image
- `hallmark` — a specific build instance of a vessel, identified by timestamp
- `summon` — pull a hallmark image from the depot to your local machine
- `vouch` — cryptographic attestation proving the artifact was built by trusted infrastructure
- `plumb` — inspect an artifact's provenance (SBOM, build info, vouch chain)

**Probe**: A hallmark has been summoned locally (local image exists).

**Guidance**: Run `tt/rbw-hs.RetrieverSummonsHallmark.sh` with a specific vessel and hallmark. Then inspect with `tt/rbw-hpf.RetrieverPlumbsFull.sh` or `tt/rbw-hpc.RetrieverPlumbsCompact.sh`. Point out: SBOM, build info, vouch chain — this is how you know what you're running.

**Confirmation**: Local image present, provenance metadata visible. Green.

### Unit 3 — Container Runtime

**Concept taught**: Bottles, nameplates, the sentry/pentacle/bottle triad. A nameplate defines a security configuration. Charging it starts three containers that work together. Quenching stops them.

**Terms introduced**:
- `bottle` — your workload container, running unmodified in a controlled network environment
- `nameplate` — per-vessel configuration tying a sentry and bottle together into a runnable unit
- `sentry` — security container enforcing network policies via iptables and dnsmasq
- `pentacle` — privileged container establishing the network namespace shared with the bottle
- `charge` — start the sentry/pentacle/bottle triad for a nameplate
- `quench` — stop and clean up a charged nameplate's containers

**Probe**: A crucible is currently charged for a nameplate.

**Guidance**: Run `tt/rbw-cC.Charge.<nameplate>.sh`. Explain what just happened: sentry (network jailer), pentacle (namespace setup), bottle (the workload). Suggest `tt/rbw-cr.Rack.<nameplate>.sh` to shell in and look around. When done, quench with `tt/rbw-cQ.Quench.<nameplate>.sh`.

**Confirmation**: Containers running. Green.

### Unit 4 — Local Experimentation

**Concept taught**: Kludge lets you build a vessel image locally for experimentation. No registry, no credentials beyond Docker — just build and test.

**Terms introduced**:
- `kludge` — build a vessel image locally for fast iteration (no registry push)

**Probe**: A locally-kludged image exists for a vessel.

**Guidance**: Run `tt/rbw-hk.LocalKludge.sh` to build a vessel locally. Charge the nameplate to test your local build, rack in and look around. Kludge is the retriever's experimentation tool — iterate on your local environment without needing director credentials or Cloud Build.

**Confirmation**: Local kludge image present. Green.

### Rendering

Walkthrough (`rbw-gOR`) shows only the frontier unit with full guidance and inline term definitions. Reference (`rbw-gOr`) shows retriever units alongside all other role tracks. Framework functions built here are reused by all subsequent track paces.

### Wire
- Mint `rbw-gOR` colophon in zipper
- Create walkthrough tabtarget
- Wire workbench routing
- Add retriever section to reference view (`rbw-gOr`)

## References
- `Tools/rbk/rbdc_DerivedConstants.sh` — credential path resolution
- `Tools/rbk/rbga_ArtifactRegistry.sh` — GAR access patterns
- Tabtargets: `rbw-hs` (summon), `rbw-hpf`/`rbw-hpc` (plumb), `rbw-cC` (charge), `rbw-cQ` (quench), `rbw-cr` (rack), `rbw-hk` (kludge)

**[260407-1011] rough**

## Character
Framework-establishing pace. Builds the simplest role track (fewest steps, read-only operations) and in doing so creates the probe/display patterns that all subsequent tracks reuse. Each unit is a standalone lesson: one concept introduced via `bug_tlt`, one action, one observable result.

## Docket

Build the retriever walkthrough guide (`tt/rbw-gOR.OnboardRetriever.sh`). Three educational units, each building on the previous gate. This is the first track built — the probe/display framework established here is reused by all subsequent tracks.

### Term Self-Containment

This track introduces all terms it uses. A retriever-only user encounters every Recipe Bottle concept they need through this track alone. Terms are introduced via `bug_tlt` on first use within the track — one sentence woven into the guidance prose, with an OSC-8 link to the README glossary.

**Full term introduction set for this track:**
- Unit 1: `depot`, `retriever`
- Unit 2: `vessel`, `hallmark`, `summon`, `vouch`, `plumb`
- Unit 3: `bottle`, `nameplate`, `sentry`, `pentacle`, `charge`, `quench`

### Unit 1 — Credential Gate

**Concept taught**: SA keys, depot access model. A retriever is someone who can read from a depot.

**Terms introduced** (via `bug_tlt` with glossary links):
- `depot` — the facility where container images are built and stored
- `retriever` — a role with read access to a depot

**Probe**: SA key file exists at expected credential path; HEAD request against GAR succeeds with SA token.

**Guidance**: How to install the SA key file. Where it came from (governor chartered it). What "GAR access" means in one sentence.

**Confirmation**: Registry responds. Green.

### Unit 2 — First Artifact

**Concept taught**: Vessels, hallmarks, summoning, provenance. An artifact has a specification (vessel), a build identity (hallmark), can be pulled (summoned), carries proof of origin (vouch), and can be inspected (plumbed).

**Terms introduced**:
- `vessel` — a specification for a container image
- `hallmark` — a specific build instance of a vessel, identified by timestamp
- `summon` — pull a hallmark image from the depot to your local machine
- `vouch` — cryptographic attestation proving the artifact was built by trusted infrastructure
- `plumb` — inspect an artifact's provenance (SBOM, build info, vouch chain)

**Probe**: A hallmark has been summoned locally (local image exists).

**Guidance**: Run `tt/rbw-hs.RetrieverSummonsHallmark.sh` with a specific vessel and hallmark. Then inspect with `tt/rbw-hpf.RetrieverPlumbsFull.sh` or `tt/rbw-hpc.RetrieverPlumbsCompact.sh`. Point out: SBOM, build info, vouch chain — this is how you know what you're running.

**Confirmation**: Local image present, provenance metadata visible. Green.

### Unit 3 — Container Runtime

**Concept taught**: Bottles, nameplates, the sentry/pentacle/bottle triad. A nameplate defines a security configuration. Charging it starts three containers that work together. Quenching stops them.

**Terms introduced**:
- `bottle` — your workload container, running unmodified in a controlled network environment
- `nameplate` — per-vessel configuration tying a sentry and bottle together into a runnable unit
- `sentry` — security container enforcing network policies via iptables and dnsmasq
- `pentacle` — privileged container establishing the network namespace shared with the bottle
- `charge` — start the sentry/pentacle/bottle triad for a nameplate
- `quench` — stop and clean up a charged nameplate's containers

**Probe**: A crucible is currently charged for a nameplate.

**Guidance**: Run `tt/rbw-cC.Charge.<nameplate>.sh`. Explain what just happened: sentry (network jailer), pentacle (namespace setup), bottle (the workload). Suggest `tt/rbw-cr.Rack.<nameplate>.sh` to shell in and look around. When done, quench with `tt/rbw-cQ.Quench.<nameplate>.sh`.

**Confirmation**: Containers running. Green.

### Rendering

Walkthrough (`rbw-gOR`) shows only the frontier unit with full guidance and inline term definitions. Reference (`rbw-gOr`) shows retriever units alongside all other role tracks. Framework functions built here are reused by all subsequent track paces.

### Wire
- Mint `rbw-gOR` colophon in zipper
- Create walkthrough tabtarget
- Wire workbench routing
- Add retriever section to reference view (`rbw-gOr`)

## References
- `Tools/rbk/rbdc_DerivedConstants.sh` — credential path resolution
- `Tools/rbk/rbga_ArtifactRegistry.sh` — GAR access patterns
- Tabtargets: `rbw-hs` (summon), `rbw-hpf`/`rbw-hpc` (plumb), `rbw-cC` (charge), `rbw-cQ` (quench), `rbw-cr` (rack)

**[260407-0958] rough**

## Character
Framework-establishing pace. Builds the simplest role track (fewest steps, read-only operations) and in doing so creates the probe/display patterns that all subsequent tracks reuse. Each unit is a standalone lesson: one concept introduced via `bug_tlt`, one action, one observable result.

## Docket

Build the retriever walkthrough guide (`tt/rbw-gOR.OnboardRetriever.sh`). Three educational units, each building on the previous gate.

### Unit 1 — Credential Gate

**Concept taught**: SA keys, depot access model. A retriever is someone who can read from a depot.

**Terms introduced** (via `bug_tlt` with glossary links):
- `depot` — the registry your artifacts live in
- `retriever` — a role with read access to a depot

**Probe**: SA key file exists at expected credential path; HEAD request against GAR succeeds with SA token.

**Guidance**: How to install the SA key file. Where it came from (governor chartered it). What "GAR access" means in one sentence.

**Confirmation**: Registry responds. Green.

### Unit 2 — First Artifact

**Concept taught**: Hallmarks, summoning, provenance. An artifact has an identity, can be pulled, and carries proof of its origin.

**Terms introduced**:
- `hallmark` — a named, vouched artifact in the registry
- `summon` — pull a hallmark image to your local machine
- `vouch` — cryptographic attestation of an artifact's provenance

**Probe**: A hallmark has been summoned locally (local image exists).

**Guidance**: Run `tt/rbw-hs.RetrieverSummonsHallmark.sh` with a specific vessel and hallmark. Then inspect with `tt/rbw-hpf.RetrieverPlumbsFull.sh` or `tt/rbw-hpc.RetrieverPlumbsCompact.sh`. Point out: SBOM, build info, vouch chain — this is how you know what you're running.

**Confirmation**: Local image present, provenance metadata visible. Green.

### Unit 3 — Container Runtime

**Concept taught**: Bottles, nameplates, the sentry/pentacle/bottle triad. A nameplate defines a security configuration. Charging it starts three containers that work together.

**Terms introduced**:
- `bottle` — the secured container workload
- `nameplate` — a named security configuration for a bottle
- `sentry` — the network jailer container
- `charge` — start the container triad for a nameplate

**Probe**: A crucible is currently charged for a nameplate.

**Guidance**: Run `tt/rbw-cC.Charge.<nameplate>.sh`. Explain what just happened: sentry (network jailer), pentacle (orchestrator), bottle (the workload). Suggest `tt/rbw-cr.Rack.<nameplate>.sh` to shell in and look around.

**Confirmation**: Containers running. Green.

### Rendering

Walkthrough (`rbw-gOR`) shows only the frontier unit with full guidance and inline term definitions. Reference (`rbw-gOr`) shows retriever units alongside all other role tracks. Framework functions built here are reused by all subsequent track paces.

### Wire
- Mint `rbw-gOR` colophon in zipper
- Create walkthrough tabtarget
- Wire workbench routing
- Add retriever section to reference view (`rbw-gOr`)

## References
- `Tools/rbk/rbdc_DerivedConstants.sh` — credential path resolution
- `Tools/rbk/rbga_ArtifactRegistry.sh` — GAR access patterns
- Tabtargets: `rbw-hs` (summon), `rbw-hpf`/`rbw-hpc` (plumb), `rbw-cC` (charge), `rbw-cr` (rack)

**[260407-0941] rough**

## Character
Framework-establishing pace. Builds the simplest role track (fewest steps, read-only operations) and in doing so creates the probe/display patterns that all subsequent tracks reuse. Each unit is a standalone lesson: one concept, one action, one observable result.

## Docket

Build the retriever walkthrough and reference guides. Three educational units, each building on the previous gate.

### Unit 1 — Credential Gate

**Concept taught**: SA keys, depot access model. A retriever is someone who can read from a depot.

**Probe**: SA key file exists at expected credential path; HEAD request against GAR succeeds with SA token.

**Guidance**: How to install the SA key file. Where it came from (governor chartered it). What "GAR access" means in one sentence.

**Confirmation**: Registry responds. Green.

### Unit 2 — First Artifact

**Concept taught**: Hallmarks, summoning, provenance. An artifact has an identity (hallmark), can be pulled (summoned), and carries proof of its origin (provenance via plumb).

**Probe**: A hallmark has been summoned locally (local image exists).

**Guidance**: Run `tt/rbw-hs.RetrieverSummonsHallmark.sh` with a specific vessel and hallmark. Then inspect with `tt/rbw-hpf.RetrieverPlumbsFull.sh` or `tt/rbw-hpc.RetrieverPlumbsCompact.sh`. Point out: SBOM, build info, vouch chain — this is how you know what you're running.

**Confirmation**: Local image present, provenance metadata visible. Green.

### Unit 3 — Container Runtime

**Concept taught**: Bottles, nameplates, the sentry/pentacle/bottle triad. A nameplate defines a security configuration. Charging it starts three containers that work together.

**Probe**: A crucible is currently charged for a nameplate.

**Guidance**: Run `tt/rbw-cC.Charge.<nameplate>.sh`. Explain what just happened: sentry (network jailer), pentacle (orchestrator), bottle (the workload). Suggest `tt/rbw-cr.Rack.<nameplate>.sh` to shell in and look around.

**Confirmation**: Containers running. Green.

### Rendering

Both walkthrough (`rbw-gOR`) and reference (`rbw-goR` or equivalent) colophons use the same probe logic. Walkthrough shows only the frontier unit. Reference shows all three with status. Framework functions built here are reused by all subsequent track paces.

### Wire
- Mint retriever colophons in zipper
- Create walkthrough tabtarget and reference tabtarget
- Wire workbench routing

## References
- `Tools/rbk/rbdc_DerivedConstants.sh` — credential path resolution
- `Tools/rbk/rbga_ArtifactRegistry.sh` — GAR access patterns
- Tabtargets: `rbw-hs` (summon), `rbw-hpf`/`rbw-hpc` (plumb), `rbw-cC` (charge), `rbw-cr` (rack)

**[260403-0943] rough**

## Character
Linear probe sequence — each step has a concrete test. Simplest role track (fewest steps, read-only operations).

## Docket

Build `tt/rbw-gOR.OnboardRetriever.sh` — the retriever's onboarding guide.

### Probes (in order)

1. **SA key installed** — file exists at expected credential path
2. **GAR access verified** — HEAD request against registry succeeds with SA token
3. **Summon a hallmark** — guide user to run `tt/rbw-hs.RetrieverSummonsHallmark.sh`, verify local image lands
4. **Inspect the artifact** — guide user to run `tt/rbw-hpf.RetrieverPlumbsFull.sh` or `tt/rbw-hpc.RetrieverPlumbsCompact.sh`, verify about/vouch metadata visible
5. **Start a bottle** — guide user to run `tt/rbw-cC.Charge.<nameplate>.sh`, verify container running

Each step: probe current state → if green, one-line confirmation → if red, show the action with the exact tabtarget to run. `buc_link` to README.md glossary anchors for concept context.

### Wire
- Mint `rbw-gOR` colophon in zipper
- Create tabtarget `tt/rbw-gOR.OnboardRetriever.sh`
- Wire workbench routing

## References
- `Tools/rbk/rbdc_DerivedConstants.sh` — credential path resolution
- `Tools/rbk/rbga_ArtifactRegistry.sh` — GAR access patterns
- Post-rename tabtargets: `rbw-hs` (summon), `rbw-hpf`/`rbw-hpc` (plumbs), `rbw-cC` (charge)

**[260402-1618] rough**

## Character
Linear probe sequence — each step has a concrete test. Simplest role track (fewest steps, read-only operations).

## Docket

Build `tt/rbw-gOR.OnboardRetriever.sh` — the retriever's onboarding guide.

### Probes (in order)

1. **SA key installed** — file exists at expected credential path
2. **GAR access verified** — HEAD request against registry succeeds with SA token
3. **Summon a consecration** — guide user to run summon tabtarget, verify local image lands
4. **Inspect the artifact** — guide user to run inspect, verify about/vouch metadata visible
5. **Start a bottle** — guide user to run bottle start, verify container running

Each step: probe current state → if green, one-line confirmation → if red, show the action with the exact tabtarget to run. `buc_link` to docs for context where helpful.

### Wire
- Mint `rbw-gOR` colophon in zipper
- Create tabtarget `tt/rbw-gOR.OnboardRetriever.sh`
- Wire workbench routing

## References
- `Tools/rbk/rbdc_DerivedConstants.sh` — credential path resolution
- `Tools/rbk/rbga_ArtifactRegistry.sh` — GAR access patterns
- Existing summon/inspect/start tabtargets for the action commands

**[260402-1521] rough**

## Character
Linear probe sequence — each step has a concrete test. Simplest role track (fewest steps, read-only operations).

## Docket

Build `tt/rbw-OR.OnboardRetriever.sh` — the retriever's onboarding guide.

### Probes (in order)

1. **SA key installed** — file exists at expected credential path
2. **GAR access verified** — HEAD request against registry succeeds with SA token
3. **Summon a consecration** — guide user to run summon tabtarget, verify local image lands
4. **Inspect the artifact** — guide user to run inspect, verify about/vouch metadata visible
5. **Start a bottle** — guide user to run bottle start, verify container running

Each step: probe current state → if green, one-line confirmation → if red, show the action with the exact tabtarget to run. `buc_link` to docs for context where helpful.

### Wire
- Mint `rbw-OR` colophon in zipper
- Create tabtarget `tt/rbw-OR.OnboardRetriever.sh`
- Wire workbench routing

## References
- `Tools/rbk/rbdc_DerivedConstants.sh` — credential path resolution
- `Tools/rbk/rbga_ArtifactRegistry.sh` — GAR access patterns
- Existing summon/inspect/start tabtargets for the action commands

### onboard-director-track (₢A3AAC) [complete]

**[260407-2212] complete**

## Character
Extension pace — the largest role track. Reuses the probe/display framework established in the retriever track. Seven units covering the full build taxonomy: local iteration, depot preparation, and all three vessel modes (conjure, bind, graft). Each unit operates on a real vessel so the director builds concrete experience, not abstract understanding.

## Docket

Build the director walkthrough guide (`tt/rbw-gOD.OnboardDirector.sh`). Seven educational units progressing from local builds through depot preparation to the three production vessel modes.

### Vessel Assignments

Each unit operates on a specific vessel so the director performs real operations:

| Unit | Operation | Vessel | Why this vessel |
|------|-----------|--------|-----------------|
| 2 | Kludge | `rbev-sentry-debian-slim` | Conjure vessel with Dockerfile — proves local build works |
| 3 | Reliquary + Enshrine | (depot-wide) | Prepares tool and base images for all vessels |
| 4 | Conjure | `rbev-sentry-debian-slim` | Same vessel kludged in Unit 2, now built via Cloud Build — shows the production path for a familiar image |
| 5 | Bind | `rbev-bottle-plantuml` | The motivating example: an untrusted upstream image pinned and bottled |
| 6 | Graft | `rbev-sentry-debian-slim` | Kludge from Unit 2 pushed to registry — completes the local→registry cycle |
| 7 | Full Ark | All three above | Compare conjure/bind/graft arks side by side |

### Term Self-Containment

This track introduces all terms it uses. A director-only user encounters every concept they need through this track alone. Terms already introduced in other tracks (depot, vessel, hallmark, vouch, kludge) are re-introduced here — different user, different session.

**Full term introduction set for this track:**
- Unit 1: `depot`, `director`
- Unit 2: `vessel`, `kludge`
- Unit 3: `ark`, `enshrine`
- Unit 4: `hallmark`, `ordain`, `conjure`, `vouch`, `tally`
- Unit 5: `bind`
- Unit 6: `graft`
- Unit 7: `plumb`

### Unit 1 — Credential Gate

**Concept taught**: Director SA, write access vs retriever's read access. A director can publish to the depot, not just read from it.

**Terms introduced**:
- `depot` — the facility where container images are built and stored
- `director` — a role with build and publish access to a depot

**Probe**: Director SA key file at expected credential path; GAR access with write scope verified.

**Guidance**: How to install the director SA key. Contrast with retriever: same depot, broader permissions. The governor knighted this SA for build operations.

**Confirmation**: Registry responds with write scope. Green.

### Unit 2 — Kludge: Local Build

**Vessel**: `rbev-sentry-debian-slim`

**Concept taught**: You can start building immediately — no Cloud Build setup needed. Kludge builds a vessel image locally using Docker.

**Terms introduced**:
- `vessel` — a specification for a container image
- `kludge` — build a vessel image locally for fast iteration (no registry push)

**Probe**: A locally-kludged image exists for `rbev-sentry-debian-slim`.

**Guidance**: Run `tt/rbw-hk.LocalKludge.sh` to build `rbev-sentry-debian-slim` locally. Test with charge/rack. This is the fastest way to see a vessel come to life. Later units teach how to push builds to the registry — and how to build this same vessel via Cloud Build for production.

**Confirmation**: Local kludge image present, can charge and rack. Green.

### Unit 3 — Depot Foundation: Reliquary and Enshrine

**Concept taught**: Before Cloud Build can run, the depot needs two kinds of upstream images: tool images (reliquary) that Cloud Build steps consume, and base images (enshrine) that vessels build FROM. Both are mirrored from upstream registries into your depot's GAR.

**Terms introduced**:
- `ark` — an immutable container image artifact in the registry, produced from a vessel
- `enshrine` — mirror upstream base images into your depot's registry

**Probe**: Reliquary tool images present in GAR (gcloud, docker, syft, skopeo, binfmt); base images enshrined with content-addressed anchors.

**Guidance**: Run `tt/rbw-dI.DirectorInscribesReliquary.sh` to mirror tool images. Run `tt/rbw-dE.DirectorEnshrinesVessel.sh` to enshrine base images. Explain: reliquary provides the tools Cloud Build uses; enshrine provides the foundations your vessels build on. Both must be in place before conjure or bind.

**Confirmation**: Tool images and base images present in GAR. Green.

### Unit 4 — Conjure: Production Build

**Vessel**: `rbev-sentry-debian-slim`

**Concept taught**: Conjure is the production build path — Cloud Build creates an image from a Dockerfile with full SLSA provenance. Every conjure produces a three-part ark: image, about (SBOM + build info), and vouch (DSSE signature verification). The director chooses a worker pool: tether (with network) or airgap (restricted egress) for security hardening.

**Terms introduced**:
- `hallmark` — a specific build instance of a vessel, identified by timestamp
- `ordain` — create a hallmark with full attestation (the production build command)
- `conjure` — ordain mode: Cloud Build creates the image from source
- `vouch` — cryptographic attestation proving the artifact was built by trusted infrastructure
- `tally` — inventory hallmarks in the registry by health status

**Probe**: At least one conjure-mode hallmark exists for `rbev-sentry-debian-slim` with vouch attestation.

**Guidance**: Run `tt/rbw-hO.DirectorOrdainsHallmark.sh` for `rbev-sentry-debian-slim`. This is the same vessel you kludged locally in Unit 2 — now Cloud Build creates it with full provenance. Explain the flow: Cloud Build runs Dockerfile, generates SBOM via syft, writes build_info.json, then vouch verifies SLSA provenance via DSSE envelope. Two Cloud Build jobs total. Mention tether vs airgap pool choice. Verify with `tt/rbw-hV.DirectorVouchesHallmarks.sh` and `tt/rbw-ht.DirectorTalliesHallmarks.sh`.

**Confirmation**: Hallmark in registry with SLSA vouch verdict. Green.

### Unit 5 — Bind: Pin Upstream Image

**Vessel**: `rbev-bottle-plantuml`

**Concept taught**: Bind mirrors a pinned upstream image into your depot. No Dockerfile, no build — just a content-addressed copy. Vouch verifies the digest matches the pinned upstream reference. This is how you use third-party software you don't fully trust.

**Terms introduced**:
- `bind` — ordain mode: mirror an upstream image pinned by digest

**Probe**: At least one bind-mode hallmark exists for `rbev-bottle-plantuml` with digest-pin vouch.

**Guidance**: PlantUML is useful for rendering architecture diagrams, but its Docker Hub image could send your private diagrams anywhere. Bind pins it by digest — no silent updates. Then charge it as a bottle: the sentry blocks all egress. You get the tool without the risk. Run `tt/rbw-hO.DirectorOrdainsHallmark.sh` for `rbev-bottle-plantuml`. Explain: the upstream image is pulled by digest, pushed to GAR, about metadata generated, vouch records a digest-pin verdict. Two Cloud Build jobs. No SLSA provenance — the image wasn't built here, but it's pinned and bottled.

**Confirmation**: Bind hallmark in registry with digest-pin vouch verdict. Green.

### Unit 6 — Graft: Push Local to Registry

**Vessel**: `rbev-sentry-debian-slim`

**Concept taught**: Graft pushes a locally-built image to GAR. The image push is local (docker push), but about and vouch still run in Cloud Build. Vouch stamps GRAFTED — explicit trust boundary marker, no provenance chain.

**Terms introduced**:
- `graft` — ordain mode: push a locally-built image to the registry

**Probe**: A graft-mode hallmark exists for `rbev-sentry-debian-slim` in the registry.

**Guidance**: You kludged `rbev-sentry-debian-slim` in Unit 2 and conjured it in Unit 4. Now push your local build to the registry via graft. Run `tt/rbw-hO.DirectorOrdainsHallmark.sh` in graft mode. Explain: one combined Cloud Build job runs about + vouch. The GRAFTED verdict means "this image was locally built — trust it at your own assessment." The development cycle: kludge → test → graft when satisfied.

**Confirmation**: Graft hallmark in registry with GRAFTED vouch verdict. Green.

### Unit 7 — The Full Ark: About and Vouch Pipeline

**Vessels**: All three — `rbev-sentry-debian-slim` (conjure + graft) and `rbev-bottle-plantuml` (bind)

**Concept taught**: Every hallmark — regardless of mode — produces the same three-part structure: image, about, and vouch. About contains the SBOM and build_info.json. Vouch contains the mode-specific verification. Plumb lets you inspect all of it. This unit ties the previous units together.

**Terms introduced**:
- `plumb` — inspect an artifact's provenance (SBOM, build info, vouch chain)

**Probe**: Conjure, bind, and graft hallmarks all present with complete about + vouch artifacts.

**Guidance**: Run `tt/rbw-hpf.RetrieverPlumbsFull.sh` against each mode's hallmark. Compare the about metadata (build_info.json fields differ by mode) and vouch verdicts (DSSE vs digest-pin vs GRAFTED). Run `tt/rbw-ht.DirectorTalliesHallmarks.sh` to see the full registry health view. This is the director's operational dashboard.

**Confirmation**: All three modes produce complete arks. Tally shows healthy across the board. Green.

### Wire
- Mint `rbw-gOD` colophon in zipper
- Create walkthrough tabtarget
- Wire workbench routing
- Add director section to reference view (`rbw-gOr`)

## References
- `Tools/rbk/rbdc_DerivedConstants.sh` — credential path resolution
- `Tools/rbk/rbga_ArtifactRegistry.sh` — GAR access
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc` — vessel modes, ark structure, about/vouch pipeline
- Tabtargets: `rbw-hk` (kludge), `rbw-dI` (inscribe reliquary), `rbw-dE` (enshrine), `rbw-hO` (ordain), `rbw-hV` (vouch), `rbw-ht` (tally), `rbw-hpf` (plumb full)

**[260407-1026] rough**

## Character
Extension pace — the largest role track. Reuses the probe/display framework established in the retriever track. Seven units covering the full build taxonomy: local iteration, depot preparation, and all three vessel modes (conjure, bind, graft). Each unit operates on a real vessel so the director builds concrete experience, not abstract understanding.

## Docket

Build the director walkthrough guide (`tt/rbw-gOD.OnboardDirector.sh`). Seven educational units progressing from local builds through depot preparation to the three production vessel modes.

### Vessel Assignments

Each unit operates on a specific vessel so the director performs real operations:

| Unit | Operation | Vessel | Why this vessel |
|------|-----------|--------|-----------------|
| 2 | Kludge | `rbev-sentry-debian-slim` | Conjure vessel with Dockerfile — proves local build works |
| 3 | Reliquary + Enshrine | (depot-wide) | Prepares tool and base images for all vessels |
| 4 | Conjure | `rbev-sentry-debian-slim` | Same vessel kludged in Unit 2, now built via Cloud Build — shows the production path for a familiar image |
| 5 | Bind | `rbev-bottle-plantuml` | The motivating example: an untrusted upstream image pinned and bottled |
| 6 | Graft | `rbev-sentry-debian-slim` | Kludge from Unit 2 pushed to registry — completes the local→registry cycle |
| 7 | Full Ark | All three above | Compare conjure/bind/graft arks side by side |

### Term Self-Containment

This track introduces all terms it uses. A director-only user encounters every concept they need through this track alone. Terms already introduced in other tracks (depot, vessel, hallmark, vouch, kludge) are re-introduced here — different user, different session.

**Full term introduction set for this track:**
- Unit 1: `depot`, `director`
- Unit 2: `vessel`, `kludge`
- Unit 3: `ark`, `enshrine`
- Unit 4: `hallmark`, `ordain`, `conjure`, `vouch`, `tally`
- Unit 5: `bind`
- Unit 6: `graft`
- Unit 7: `plumb`

### Unit 1 — Credential Gate

**Concept taught**: Director SA, write access vs retriever's read access. A director can publish to the depot, not just read from it.

**Terms introduced**:
- `depot` — the facility where container images are built and stored
- `director` — a role with build and publish access to a depot

**Probe**: Director SA key file at expected credential path; GAR access with write scope verified.

**Guidance**: How to install the director SA key. Contrast with retriever: same depot, broader permissions. The governor knighted this SA for build operations.

**Confirmation**: Registry responds with write scope. Green.

### Unit 2 — Kludge: Local Build

**Vessel**: `rbev-sentry-debian-slim`

**Concept taught**: You can start building immediately — no Cloud Build setup needed. Kludge builds a vessel image locally using Docker.

**Terms introduced**:
- `vessel` — a specification for a container image
- `kludge` — build a vessel image locally for fast iteration (no registry push)

**Probe**: A locally-kludged image exists for `rbev-sentry-debian-slim`.

**Guidance**: Run `tt/rbw-hk.LocalKludge.sh` to build `rbev-sentry-debian-slim` locally. Test with charge/rack. This is the fastest way to see a vessel come to life. Later units teach how to push builds to the registry — and how to build this same vessel via Cloud Build for production.

**Confirmation**: Local kludge image present, can charge and rack. Green.

### Unit 3 — Depot Foundation: Reliquary and Enshrine

**Concept taught**: Before Cloud Build can run, the depot needs two kinds of upstream images: tool images (reliquary) that Cloud Build steps consume, and base images (enshrine) that vessels build FROM. Both are mirrored from upstream registries into your depot's GAR.

**Terms introduced**:
- `ark` — an immutable container image artifact in the registry, produced from a vessel
- `enshrine` — mirror upstream base images into your depot's registry

**Probe**: Reliquary tool images present in GAR (gcloud, docker, syft, skopeo, binfmt); base images enshrined with content-addressed anchors.

**Guidance**: Run `tt/rbw-dI.DirectorInscribesReliquary.sh` to mirror tool images. Run `tt/rbw-dE.DirectorEnshrinesVessel.sh` to enshrine base images. Explain: reliquary provides the tools Cloud Build uses; enshrine provides the foundations your vessels build on. Both must be in place before conjure or bind.

**Confirmation**: Tool images and base images present in GAR. Green.

### Unit 4 — Conjure: Production Build

**Vessel**: `rbev-sentry-debian-slim`

**Concept taught**: Conjure is the production build path — Cloud Build creates an image from a Dockerfile with full SLSA provenance. Every conjure produces a three-part ark: image, about (SBOM + build info), and vouch (DSSE signature verification). The director chooses a worker pool: tether (with network) or airgap (restricted egress) for security hardening.

**Terms introduced**:
- `hallmark` — a specific build instance of a vessel, identified by timestamp
- `ordain` — create a hallmark with full attestation (the production build command)
- `conjure` — ordain mode: Cloud Build creates the image from source
- `vouch` — cryptographic attestation proving the artifact was built by trusted infrastructure
- `tally` — inventory hallmarks in the registry by health status

**Probe**: At least one conjure-mode hallmark exists for `rbev-sentry-debian-slim` with vouch attestation.

**Guidance**: Run `tt/rbw-hO.DirectorOrdainsHallmark.sh` for `rbev-sentry-debian-slim`. This is the same vessel you kludged locally in Unit 2 — now Cloud Build creates it with full provenance. Explain the flow: Cloud Build runs Dockerfile, generates SBOM via syft, writes build_info.json, then vouch verifies SLSA provenance via DSSE envelope. Two Cloud Build jobs total. Mention tether vs airgap pool choice. Verify with `tt/rbw-hV.DirectorVouchesHallmarks.sh` and `tt/rbw-ht.DirectorTalliesHallmarks.sh`.

**Confirmation**: Hallmark in registry with SLSA vouch verdict. Green.

### Unit 5 — Bind: Pin Upstream Image

**Vessel**: `rbev-bottle-plantuml`

**Concept taught**: Bind mirrors a pinned upstream image into your depot. No Dockerfile, no build — just a content-addressed copy. Vouch verifies the digest matches the pinned upstream reference. This is how you use third-party software you don't fully trust.

**Terms introduced**:
- `bind` — ordain mode: mirror an upstream image pinned by digest

**Probe**: At least one bind-mode hallmark exists for `rbev-bottle-plantuml` with digest-pin vouch.

**Guidance**: PlantUML is useful for rendering architecture diagrams, but its Docker Hub image could send your private diagrams anywhere. Bind pins it by digest — no silent updates. Then charge it as a bottle: the sentry blocks all egress. You get the tool without the risk. Run `tt/rbw-hO.DirectorOrdainsHallmark.sh` for `rbev-bottle-plantuml`. Explain: the upstream image is pulled by digest, pushed to GAR, about metadata generated, vouch records a digest-pin verdict. Two Cloud Build jobs. No SLSA provenance — the image wasn't built here, but it's pinned and bottled.

**Confirmation**: Bind hallmark in registry with digest-pin vouch verdict. Green.

### Unit 6 — Graft: Push Local to Registry

**Vessel**: `rbev-sentry-debian-slim`

**Concept taught**: Graft pushes a locally-built image to GAR. The image push is local (docker push), but about and vouch still run in Cloud Build. Vouch stamps GRAFTED — explicit trust boundary marker, no provenance chain.

**Terms introduced**:
- `graft` — ordain mode: push a locally-built image to the registry

**Probe**: A graft-mode hallmark exists for `rbev-sentry-debian-slim` in the registry.

**Guidance**: You kludged `rbev-sentry-debian-slim` in Unit 2 and conjured it in Unit 4. Now push your local build to the registry via graft. Run `tt/rbw-hO.DirectorOrdainsHallmark.sh` in graft mode. Explain: one combined Cloud Build job runs about + vouch. The GRAFTED verdict means "this image was locally built — trust it at your own assessment." The development cycle: kludge → test → graft when satisfied.

**Confirmation**: Graft hallmark in registry with GRAFTED vouch verdict. Green.

### Unit 7 — The Full Ark: About and Vouch Pipeline

**Vessels**: All three — `rbev-sentry-debian-slim` (conjure + graft) and `rbev-bottle-plantuml` (bind)

**Concept taught**: Every hallmark — regardless of mode — produces the same three-part structure: image, about, and vouch. About contains the SBOM and build_info.json. Vouch contains the mode-specific verification. Plumb lets you inspect all of it. This unit ties the previous units together.

**Terms introduced**:
- `plumb` — inspect an artifact's provenance (SBOM, build info, vouch chain)

**Probe**: Conjure, bind, and graft hallmarks all present with complete about + vouch artifacts.

**Guidance**: Run `tt/rbw-hpf.RetrieverPlumbsFull.sh` against each mode's hallmark. Compare the about metadata (build_info.json fields differ by mode) and vouch verdicts (DSSE vs digest-pin vs GRAFTED). Run `tt/rbw-ht.DirectorTalliesHallmarks.sh` to see the full registry health view. This is the director's operational dashboard.

**Confirmation**: All three modes produce complete arks. Tally shows healthy across the board. Green.

### Wire
- Mint `rbw-gOD` colophon in zipper
- Create walkthrough tabtarget
- Wire workbench routing
- Add director section to reference view (`rbw-gOr`)

## References
- `Tools/rbk/rbdc_DerivedConstants.sh` — credential path resolution
- `Tools/rbk/rbga_ArtifactRegistry.sh` — GAR access
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc` — vessel modes, ark structure, about/vouch pipeline
- Tabtargets: `rbw-hk` (kludge), `rbw-dI` (inscribe reliquary), `rbw-dE` (enshrine), `rbw-hO` (ordain), `rbw-hV` (vouch), `rbw-ht` (tally), `rbw-hpf` (plumb full)

**[260407-1021] rough**

## Character
Extension pace — the largest role track. Reuses the probe/display framework established in the retriever track. Seven units covering the full build taxonomy: local iteration, depot preparation, and all three vessel modes (conjure, bind, graft). Each unit is a standalone lesson.

## Docket

Build the director walkthrough guide (`tt/rbw-gOD.OnboardDirector.sh`). Seven educational units progressing from local builds through depot preparation to the three production vessel modes.

### Term Self-Containment

This track introduces all terms it uses. A director-only user encounters every concept they need through this track alone. Terms already introduced in other tracks (depot, vessel, hallmark, vouch, kludge) are re-introduced here — different user, different session.

**Full term introduction set for this track:**
- Unit 1: `depot`, `director`
- Unit 2: `vessel`, `kludge`
- Unit 3: `ark`, `enshrine`
- Unit 4: `hallmark`, `ordain`, `conjure`, `vouch`, `tally`
- Unit 5: `bind`
- Unit 6: `graft`
- Unit 7: (no new terms)

### Unit 1 — Credential Gate

**Concept taught**: Director SA, write access vs retriever's read access. A director can publish to the depot, not just read from it.

**Terms introduced**:
- `depot` — the facility where container images are built and stored
- `director` — a role with build and publish access to a depot

**Probe**: Director SA key file at expected credential path; GAR access with write scope verified.

**Guidance**: How to install the director SA key. Contrast with retriever: same depot, broader permissions. The governor knighted this SA for build operations.

**Confirmation**: Registry responds with write scope. Green.

### Unit 2 — Kludge: Local Build

**Concept taught**: You can start building immediately — no Cloud Build setup needed. Kludge builds a vessel image locally using Docker.

**Terms introduced**:
- `vessel` — a specification for a container image
- `kludge` — build a vessel image locally for fast iteration (no registry push)

**Probe**: A locally-kludged image exists for a vessel.

**Guidance**: Run `tt/rbw-hk.LocalKludge.sh` to build a vessel locally. Test with charge/rack. This is the fastest way to see a vessel come to life. Later units teach how to push builds to the registry.

**Confirmation**: Local kludge image present, can charge and rack. Green.

### Unit 3 — Depot Foundation: Reliquary and Enshrine

**Concept taught**: Before Cloud Build can run, the depot needs two kinds of upstream images: tool images (reliquary) that Cloud Build steps consume, and base images (enshrine) that vessels build FROM. Both are mirrored from upstream registries into your depot's GAR.

**Terms introduced**:
- `ark` — an immutable container image artifact in the registry, produced from a vessel
- `enshrine` — mirror upstream base images into your depot's registry

**Probe**: Reliquary tool images present in GAR (gcloud, docker, syft, skopeo, binfmt); base images enshrined with content-addressed anchors.

**Guidance**: Run `tt/rbw-dI.DirectorInscribesReliquary.sh` to mirror tool images. Run `tt/rbw-dE.DirectorEnshrinesVessel.sh` to enshrine base images. Explain: reliquary provides the tools Cloud Build uses; enshrine provides the foundations your vessels build on. Both must be in place before conjure or bind.

**Confirmation**: Tool images and base images present in GAR. Green.

### Unit 4 — Conjure: Production Build

**Concept taught**: Conjure is the production build path — Cloud Build creates an image from a Dockerfile with full SLSA provenance. Every conjure produces a three-part ark: image, about (SBOM + build info), and vouch (DSSE signature verification). The director chooses a worker pool: tether (with network) or airgap (restricted egress) for security hardening.

**Terms introduced**:
- `hallmark` — a specific build instance of a vessel, identified by timestamp
- `ordain` — create a hallmark with full attestation (the production build command)
- `conjure` — ordain mode: Cloud Build creates the image from source
- `vouch` — cryptographic attestation proving the artifact was built by trusted infrastructure
- `tally` — inventory hallmarks in the registry by health status

**Probe**: At least one conjure-mode hallmark exists with vouch attestation.

**Guidance**: Run `tt/rbw-hO.DirectorOrdainsHallmark.sh` for a conjure vessel. Explain the flow: Cloud Build runs Dockerfile, generates SBOM via syft, writes build_info.json, then vouch verifies SLSA provenance via DSSE envelope. Two Cloud Build jobs total. Mention tether vs airgap pool choice. Verify with `tt/rbw-hV.DirectorVouchesHallmarks.sh` and `tt/rbw-ht.DirectorTalliesHallmarks.sh`.

**Confirmation**: Hallmark in registry with SLSA vouch verdict. Green.

### Unit 5 — Bind: Pin Upstream Image

**Concept taught**: Bind mirrors a pinned upstream image into your depot. No Dockerfile, no build — just a content-addressed copy. Vouch verifies the digest matches the pinned upstream reference.

**Terms introduced**:
- `bind` — ordain mode: mirror an upstream image pinned by digest

**Probe**: At least one bind-mode hallmark exists with digest-pin vouch.

**Guidance**: Run `tt/rbw-hO.DirectorOrdainsHallmark.sh` for a bind vessel (e.g., `rbev-bottle-plantuml`). Explain: the upstream image is pulled by digest, pushed to GAR, about metadata generated, vouch records a digest-pin verdict. Two Cloud Build jobs. No SLSA provenance — the image wasn't built here.

**Confirmation**: Bind hallmark in registry with digest-pin vouch verdict. Green.

### Unit 6 — Graft: Push Local to Registry

**Concept taught**: Graft pushes a locally-built image to GAR. The image push is local (docker push), but about and vouch still run in Cloud Build. Vouch stamps GRAFTED — explicit trust boundary marker, no provenance chain.

**Terms introduced**:
- `graft` — ordain mode: push a locally-built image to the registry

**Probe**: A graft-mode hallmark exists in the registry.

**Guidance**: First kludge a vessel locally (already learned in Unit 2). Then run `tt/rbw-hO.DirectorOrdainsHallmark.sh` in graft mode to push it. Explain: one combined Cloud Build job runs about + vouch. The GRAFTED verdict means "this image was locally built — trust it at your own assessment." The development cycle: kludge → test → graft when satisfied.

**Confirmation**: Graft hallmark in registry with GRAFTED vouch verdict. Green.

### Unit 7 — The Full Ark: About and Vouch Pipeline

**Concept taught**: Every hallmark — regardless of mode — produces the same three-part structure: image, about, and vouch. About contains the SBOM and build_info.json. Vouch contains the mode-specific verification. Plumb lets you inspect all of it. This unit is a summary that ties the previous units together.

**Terms introduced**:
- `plumb` — inspect an artifact's provenance (SBOM, build info, vouch chain)

**Probe**: Conjure, bind, and graft hallmarks all present with complete about + vouch artifacts.

**Guidance**: Run `tt/rbw-hpf.RetrieverPlumbsFull.sh` against each mode's hallmark. Compare the about metadata (build_info.json fields differ by mode) and vouch verdicts (DSSE vs digest-pin vs GRAFTED). Run `tt/rbw-ht.DirectorTalliesHallmarks.sh` to see the full registry health view. This is the director's operational dashboard.

**Confirmation**: All three modes produce complete arks. Tally shows healthy across the board. Green.

### Wire
- Mint `rbw-gOD` colophon in zipper
- Create walkthrough tabtarget
- Wire workbench routing
- Add director section to reference view (`rbw-gOr`)

## References
- `Tools/rbk/rbdc_DerivedConstants.sh` — credential path resolution
- `Tools/rbk/rbga_ArtifactRegistry.sh` — GAR access
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc` — vessel modes, ark structure, about/vouch pipeline
- Tabtargets: `rbw-hk` (kludge), `rbw-dI` (inscribe reliquary), `rbw-dE` (enshrine), `rbw-hO` (ordain), `rbw-hV` (vouch), `rbw-ht` (tally), `rbw-hpf` (plumb full)

**[260407-1011] rough**

## Character
Extension pace — reuses the probe/display framework established in the retriever track. More units than retriever, introduces write operations and the build pipeline. Each unit is a standalone lesson.

## Docket

Build the director walkthrough guide (`tt/rbw-gOD.OnboardDirector.sh`). Four educational units progressing from read access through build pipeline to local development workflow.

### Term Self-Containment

This track introduces all terms it uses. A director-only user encounters every concept they need through this track alone. Terms already introduced in other tracks (depot, vessel, hallmark, vouch) are re-introduced here — different user, different session.

**Full term introduction set for this track:**
- Unit 1: `depot`, `director`
- Unit 2: `vessel`, `ark`, `enshrine`
- Unit 3: `hallmark`, `ordain`, `conjure`, `bind`, `vouch`, `tally`
- Unit 4: `kludge`, `graft`

### Unit 1 — Credential Gate

**Concept taught**: Director SA, write access vs retriever's read access. A director can publish to the depot, not just read from it.

**Terms introduced**:
- `depot` — the facility where container images are built and stored
- `director` — a role with build and publish access to a depot

**Probe**: Director SA key file at expected credential path; GAR access with write scope verified.

**Guidance**: How to install the director SA key. Contrast with retriever: same depot, broader permissions. The governor knighted this SA for build operations.

**Confirmation**: Registry responds with write scope. Green.

### Unit 2 — Upstream Pipeline

**Concept taught**: Enshrinement and base images. Before you can build vessels, the depot needs upstream foundations mirrored into it.

**Terms introduced**:
- `vessel` — a specification for a container image
- `ark` — an immutable container image artifact in the registry, produced from a vessel
- `enshrine` — mirror upstream base images into your depot's registry

**Probe**: Reliquary images present in GAR; base images enshrined.

**Guidance**: Run `tt/rbw-dI.DirectorInscribesReliquary.sh` if tool images missing. Run `tt/rbw-dE.DirectorEnshrinesVessel.sh` for base images. Explain: these are the foundations your builds stand on.

**Confirmation**: Tool images and base images present in GAR. Green.

### Unit 3 — Cloud Build Pipeline

**Concept taught**: Ordaining a hallmark through Cloud Build — the production path with full provenance. Three ordain modes exist: conjure (from source), bind (mirror upstream), graft (push local).

**Terms introduced**:
- `hallmark` — a specific build instance of a vessel, identified by timestamp
- `ordain` — create a hallmark with full attestation (the production build command)
- `conjure` — ordain mode: Cloud Build creates the image from source
- `bind` — ordain mode: mirror an upstream image pinned by digest
- `vouch` — cryptographic attestation proving the artifact was built by trusted infrastructure
- `tally` — inventory hallmarks in the registry by health status

**Probe**: At least one hallmark exists with vouch attestation.

**Guidance**: Run `tt/rbw-hO.DirectorOrdainsHallmark.sh` to submit a Cloud Build. Explain the conjure flow: Cloud Build runs, SBOM generated (syft), about metadata written, vouch attestation attached. Mention bind mode for upstream images (like PlantUML). Verify with `tt/rbw-hV.DirectorVouchesHallmarks.sh` and `tt/rbw-ht.DirectorTalliesHallmarks.sh`.

**Confirmation**: Hallmark in registry with vouch present. Green.

### Unit 4 — Local Development

**Concept taught**: The local iteration cycle — build fast, test locally, push when satisfied.

**Terms introduced**:
- `kludge` — build a vessel image locally for fast iteration (no registry push)
- `graft` — ordain mode: push a locally-built image to the registry

**Probe**: A graft-mode hallmark exists in the registry (or: local kludge image present).

**Guidance**: Run `tt/rbw-hk.LocalKludge.sh` to build locally. Test with charge/rack. When satisfied, run `tt/rbw-hO.DirectorOrdainsHallmark.sh` in graft mode to push. Explain: kludge is fast iteration, graft is "I'm satisfied, push this."

**Confirmation**: Local image built, or graft hallmark in registry. Green.

### Wire
- Mint `rbw-gOD` colophon in zipper
- Create walkthrough tabtarget
- Wire workbench routing
- Add director section to reference view (`rbw-gOr`)

## References
- `Tools/rbk/rbdc_DerivedConstants.sh` — credential path resolution
- `Tools/rbk/rbga_ArtifactRegistry.sh` — GAR access
- Tabtargets: `rbw-dE` (enshrine), `rbw-dI` (inscribe), `rbw-hO` (ordain), `rbw-hV` (vouch), `rbw-ht` (tally), `rbw-hk` (kludge)

**[260407-0958] rough**

## Character
Extension pace — reuses the probe/display framework established in the retriever track. More units than retriever, introduces write operations and the build pipeline. Each unit is a standalone lesson.

## Docket

Build the director walkthrough guide (`tt/rbw-gOD.OnboardDirector.sh`). Four educational units progressing from read access through build pipeline to local development workflow.

### Unit 1 — Credential Gate

**Concept taught**: Director SA, write access vs retriever's read access. A director can publish to the depot, not just read from it.

**Terms introduced**:
- `director` — a role with build and publish access to a depot

**Probe**: Director SA key file at expected credential path; GAR access with write scope verified.

**Guidance**: How to install the director SA key. Contrast with retriever: same depot, broader permissions. The governor knighted this SA for build operations.

**Confirmation**: Registry responds with write scope. Green.

### Unit 2 — Upstream Pipeline

**Concept taught**: Enshrinement and base images. Before you can build vessels, the depot needs upstream foundations mirrored into it.

**Terms introduced**:
- `enshrine` — mirror upstream base images into your depot's registry
- `ark` — a vessel's build recipe and identity within the registry

**Probe**: Reliquary images present in GAR; base images enshrined.

**Guidance**: Run `tt/rbw-dI.DirectorInscribesReliquary.sh` if tool images missing. Run `tt/rbw-dE.DirectorEnshrinesVessel.sh` for base images. Explain: these are the foundations your builds stand on.

**Confirmation**: Tool images and base images present in GAR. Green.

### Unit 3 — Cloud Build Pipeline

**Concept taught**: Ordaining a hallmark through Cloud Build — the production path with full provenance.

**Terms introduced**:
- `ordain` — create a hallmark with full attestation (the production build command)
- `conjure` — ordain mode where Cloud Build creates the image from source

**Probe**: At least one hallmark exists with vouch attestation.

**Guidance**: Run `tt/rbw-hO.DirectorOrdainsHallmark.sh` to submit a Cloud Build. Explain the conjure flow: Cloud Build runs, SBOM generated (syft), about metadata written, vouch attestation attached. Verify with `tt/rbw-hV.DirectorVouchesHallmarks.sh` and `tt/rbw-ht.DirectorTalliesHallmarks.sh`.

**Confirmation**: Hallmark in registry with vouch present. Green.

### Unit 4 — Local Development

**Concept taught**: The local iteration cycle — build fast, test locally, push when satisfied.

**Terms introduced**:
- `kludge` — build a vessel image locally for fast iteration
- `graft` — push a locally-built image to the registry

**Probe**: A graft-mode hallmark exists in the registry (or: local kludge image present).

**Guidance**: Run `tt/rbw-hk.LocalKludge.sh` to build locally. Test with charge/rack. When satisfied, run `tt/rbw-hO.DirectorOrdainsHallmark.sh` in graft mode to push. Explain: kludge is fast iteration, graft is "I'm satisfied, push this."

**Confirmation**: Local image built, or graft hallmark in registry. Green.

### Wire
- Mint `rbw-gOD` colophon in zipper
- Create walkthrough tabtarget
- Wire workbench routing
- Add director section to reference view (`rbw-gOr`)

## References
- `Tools/rbk/rbdc_DerivedConstants.sh` — credential path resolution
- `Tools/rbk/rbga_ArtifactRegistry.sh` — GAR access
- Tabtargets: `rbw-dE` (enshrine), `rbw-dI` (inscribe), `rbw-hO` (ordain), `rbw-hV` (vouch), `rbw-ht` (tally), `rbw-hk` (kludge)

**[260407-0941] rough**

## Character
Extension pace — reuses the probe/display framework established in the retriever track. More units than retriever, introduces write operations and the build pipeline. Each unit is a standalone lesson.

## Docket

Build the director walkthrough and reference guides. Four educational units progressing from read access through build pipeline to local development workflow.

### Unit 1 — Credential Gate

**Concept taught**: Director SA, write access vs retriever's read access. A director can publish to the depot, not just read from it.

**Probe**: Director SA key file at expected credential path; GAR access with write scope verified.

**Guidance**: How to install the director SA key. Contrast with retriever: same depot, broader permissions. The governor knighted this SA for build operations.

**Confirmation**: Registry responds with write scope. Green.

### Unit 2 — Upstream Pipeline

**Concept taught**: Reliquary, enshrinement, base images. Before you can build vessels, the depot needs upstream base images and tool images (skopeo, syft) mirrored into it.

**Probe**: Reliquary images present in GAR; base images enshrined.

**Guidance**: Run `tt/rbw-dI.DirectorInscribesReliquary.sh` if tool images missing. Run `tt/rbw-dE.DirectorEnshrinesVessel.sh` for base images. Explain: these are the foundations your builds stand on.

**Confirmation**: Tool images and base images present in GAR. Green.

### Unit 3 — Cloud Build Pipeline

**Concept taught**: Ordain, conjure mode, about/vouch attestation. This is the production build path — Cloud Build creates a hallmark with full provenance.

**Probe**: At least one hallmark exists with vouch attestation.

**Guidance**: Run `tt/rbw-hO.DirectorOrdainsHallmark.sh` to submit a Cloud Build. Explain the conjure flow: Cloud Build runs, SBOM generated (syft), about metadata written, vouch attestation attached. Verify with `tt/rbw-hV.DirectorVouchesHallmarks.sh` and `tt/rbw-ht.DirectorTalliesHallmarks.sh`.

**Confirmation**: Hallmark in registry with vouch present. Green.

### Unit 4 — Local Development

**Concept taught**: Kludge, graft mode, the development cycle. For iteration: build locally (kludge), test locally, then push to GAR (graft) when satisfied.

**Probe**: A graft-mode hallmark exists in the registry (or: local kludge image present).

**Guidance**: Run `tt/rbw-hk.LocalKludge.sh` to build locally. Test with charge/rack. When satisfied, run `tt/rbw-hO.DirectorOrdainsHallmark.sh` in graft mode to push. Explain: kludge is fast iteration, graft is "I'm satisfied, push this."

**Confirmation**: Local image built, or graft hallmark in registry. Green.

### Wire
- Mint director colophons in zipper
- Create walkthrough and reference tabtargets
- Wire workbench routing

## References
- `Tools/rbk/rbdc_DerivedConstants.sh` — credential path resolution
- `Tools/rbk/rbga_ArtifactRegistry.sh` — GAR access
- Tabtargets: `rbw-dE` (enshrine), `rbw-dI` (inscribe), `rbw-hO` (ordain), `rbw-hV` (vouch), `rbw-ht` (tally), `rbw-hk` (kludge)

**[260403-0943] rough**

## Character
Linear probe sequence — more steps than retriever, involves build pipeline verification.

## Docket

Build `tt/rbw-gOD.OnboardDirector.sh` — the director's onboarding guide.

### Probes (in order)

1. **SA key installed** — director SA key file at expected credential path
2. **GAR access verified** — HEAD request against registry with director SA token
3. **Reliquary images present** — tool images (skopeo, syft) exist in GAR
4. **Enshrine upstream** — guide user to run `tt/rbw-dE.DirectorEnshrinesVessel.sh`, verify base images in GAR
5. **Ordain a hallmark** — guide user to submit a Cloud Build via `tt/rbw-hO.DirectorOrdainsHallmark.sh`, verify hallmark lands
6. **Vouch exists** — verify about + vouch artifacts present via `tt/rbw-hV.DirectorVouchesHallmarks.sh`
7. **Graft from local** — guide user through local build + graft push via `tt/rbw-hO.DirectorOrdainsHallmark.sh` (graft mode, optional kludge path)

Each step: probe → green confirmation or red with action tabtarget. `buc_link` to README.md glossary anchors.

### Wire
- Mint `rbw-gOD` colophon in zipper
- Create tabtarget `tt/rbw-gOD.OnboardDirector.sh`
- Wire workbench routing

## References
- `Tools/rbk/rbga_ArtifactRegistry.sh` — GAR access
- `Tools/rbk/rbdc_DerivedConstants.sh` — credential path resolution
- Post-rename tabtargets: `rbw-dE` (enshrine), `rbw-hO` (ordain), `rbw-hV` (vouch), `rbw-ht` (tally)

**[260402-1618] rough**

## Character
Linear probe sequence — more steps than retriever, involves build pipeline verification.

## Docket

Build `tt/rbw-gOD.OnboardDirector.sh` — the director's onboarding guide.

### Probes (in order)

1. **SA key installed** — director SA key file at expected credential path
2. **GAR access verified** — HEAD request against registry with director SA token
3. **Reliquary images present** — tool images (skopeo, syft) exist in GAR
4. **Enshrine upstream** — guide user to enshrine base images into GAR
5. **Conjure a vessel** — guide user to submit a Cloud Build conjure, verify consecration lands
6. **Vouch exists** — verify about + vouch artifacts present for the consecration
7. **Graft from local** — guide user through local build + graft push (optional, kludge path)

Each step: probe → green confirmation or red with action tabtarget.

### Wire
- Mint `rbw-gOD` colophon in zipper
- Create tabtarget `tt/rbw-gOD.OnboardDirector.sh`
- Wire workbench routing

## References
- `Tools/rbk/rbfd_FoundryDirectorBuild.sh` — conjure, enshrine, graft operations
- `Tools/rbk/rbga_ArtifactRegistry.sh` — GAR access
- `Tools/rbk/rbdc_DerivedConstants.sh` — credential path resolution

**[260402-1521] rough**

## Character
Linear probe sequence — more steps than retriever, involves build pipeline verification.

## Docket

Build `tt/rbw-OD.OnboardDirector.sh` — the director's onboarding guide.

### Probes (in order)

1. **SA key installed** — director SA key file at expected credential path
2. **GAR access verified** — HEAD request against registry with director SA token
3. **Reliquary images present** — tool images (skopeo, syft) exist in GAR
4. **Enshrine upstream** — guide user to enshrine base images into GAR
5. **Conjure a vessel** — guide user to submit a Cloud Build conjure, verify consecration lands
6. **Vouch exists** — verify about + vouch artifacts present for the consecration
7. **Graft from local** — guide user through local build + graft push (optional, kludge path)

Each step: probe → green confirmation or red with action tabtarget.

### Wire
- Mint `rbw-OD` colophon in zipper
- Create tabtarget `tt/rbw-OD.OnboardDirector.sh`
- Wire workbench routing

## References
- `Tools/rbk/rbfd_FoundryDirectorBuild.sh` — conjure, enshrine, graft operations
- `Tools/rbk/rbga_ArtifactRegistry.sh` — GAR access
- `Tools/rbk/rbdc_DerivedConstants.sh` — credential path resolution

### onboard-governor-track (₢A3AAD) [complete]

**[260407-2212] complete**

## Character
Extension pace — administrative role, creating resources for others. Shorter than director but conceptually different: the governor doesn't build or run things, they provision access for those who do.

## Docket

Build the governor walkthrough guide (`tt/rbw-gOG.OnboardGovernor.sh`). Three educational units covering project access, SA lifecycle, and IAM delegation.

### Term Self-Containment

This track introduces all terms it uses. A governor-only user encounters every concept they need through this track alone.

**Full term introduction set for this track:**
- Unit 1: `depot`, `governor`
- Unit 2: `retriever`, `director`, `charter`, `knight`
- Unit 3: (no new terms)

### Unit 1 — Project Access

**Concept taught**: Depot as a provisioned GCP project. The governor works within a depot that the payor created.

**Terms introduced**:
- `depot` — the facility where container images are built and stored
- `governor` — a role that administers a depot: creates service accounts, manages access

**Probe**: gcloud project accessible with current credentials; depot initialized.

**Guidance**: Verify project access. If depot doesn't exist, this is a payor responsibility (link to payor track). Governor works within an existing depot.

**Confirmation**: Depot accessible. Green.

### Unit 2 — Service Account Lifecycle

**Concept taught**: The governor as SA factory — chartering retrievers and knighting directors. These are the two downstream roles the governor provisions access for.

**Terms introduced**:
- `retriever` — a role with read access to a depot (the governor creates credentials for this role)
- `director` — a role with build and publish access to a depot (the governor creates credentials for this role)
- `charter` — create a retriever service account (read access)
- `knight` — create a director service account (build access)

**Probe**: Retriever SA exists; director SA exists.

**Guidance**: Run `tt/rbw-aC.GovernorChartersRetriever.sh` to charter a retriever SA. Run `tt/rbw-aK.GovernorKnightsDirector.sh` to knight a director SA. Explain: these SAs are the credentials you hand to retriever and director users. List with `tt/rbw-aL.GovernorListsServiceAccounts.sh`.

**Confirmation**: Both SAs exist, keys downloadable. Green.

### Unit 3 — IAM Grants

**Concept taught**: Least-privilege access. The SA alone isn't enough — it needs specific IAM grants to function.

**Terms introduced**:
- (No new terms — IAM is GCP vocabulary, not Recipe Bottle vocabulary. `retriever` and `director` already introduced in Unit 2.)

**Probe**: Retriever SA has correct role bindings on GAR; director SA has correct role bindings on GAR and Cloud Build.

**Guidance**: Apply IAM grants. Explain the principle: each SA gets exactly the permissions its role requires, no more. Retriever gets read. Director gets read + write + build trigger.

**Confirmation**: Role bindings verified. Green.

### Wire
- Mint `rbw-gOG` colophon in zipper
- Create walkthrough tabtarget
- Wire workbench routing
- Add governor section to reference view (`rbw-gOr`)

## References
- `Tools/rbk/rbgm_ManualProcedures.sh` — governor manual procedures
- `Tools/rbk/rbgi_IAM.sh` — IAM grant operations
- `Tools/rbk/rbgg_Governor.sh` — governor SA management
- Tabtargets: `rbw-aM` (mantle), `rbw-aK` (knight), `rbw-aC` (charter), `rbw-aL` (list), `rbw-aF` (forfeit)

**[260407-1011] rough**

## Character
Extension pace — administrative role, creating resources for others. Shorter than director but conceptually different: the governor doesn't build or run things, they provision access for those who do.

## Docket

Build the governor walkthrough guide (`tt/rbw-gOG.OnboardGovernor.sh`). Three educational units covering project access, SA lifecycle, and IAM delegation.

### Term Self-Containment

This track introduces all terms it uses. A governor-only user encounters every concept they need through this track alone.

**Full term introduction set for this track:**
- Unit 1: `depot`, `governor`
- Unit 2: `retriever`, `director`, `charter`, `knight`
- Unit 3: (no new terms)

### Unit 1 — Project Access

**Concept taught**: Depot as a provisioned GCP project. The governor works within a depot that the payor created.

**Terms introduced**:
- `depot` — the facility where container images are built and stored
- `governor` — a role that administers a depot: creates service accounts, manages access

**Probe**: gcloud project accessible with current credentials; depot initialized.

**Guidance**: Verify project access. If depot doesn't exist, this is a payor responsibility (link to payor track). Governor works within an existing depot.

**Confirmation**: Depot accessible. Green.

### Unit 2 — Service Account Lifecycle

**Concept taught**: The governor as SA factory — chartering retrievers and knighting directors. These are the two downstream roles the governor provisions access for.

**Terms introduced**:
- `retriever` — a role with read access to a depot (the governor creates credentials for this role)
- `director` — a role with build and publish access to a depot (the governor creates credentials for this role)
- `charter` — create a retriever service account (read access)
- `knight` — create a director service account (build access)

**Probe**: Retriever SA exists; director SA exists.

**Guidance**: Run `tt/rbw-aC.GovernorChartersRetriever.sh` to charter a retriever SA. Run `tt/rbw-aK.GovernorKnightsDirector.sh` to knight a director SA. Explain: these SAs are the credentials you hand to retriever and director users. List with `tt/rbw-aL.GovernorListsServiceAccounts.sh`.

**Confirmation**: Both SAs exist, keys downloadable. Green.

### Unit 3 — IAM Grants

**Concept taught**: Least-privilege access. The SA alone isn't enough — it needs specific IAM grants to function.

**Terms introduced**:
- (No new terms — IAM is GCP vocabulary, not Recipe Bottle vocabulary. `retriever` and `director` already introduced in Unit 2.)

**Probe**: Retriever SA has correct role bindings on GAR; director SA has correct role bindings on GAR and Cloud Build.

**Guidance**: Apply IAM grants. Explain the principle: each SA gets exactly the permissions its role requires, no more. Retriever gets read. Director gets read + write + build trigger.

**Confirmation**: Role bindings verified. Green.

### Wire
- Mint `rbw-gOG` colophon in zipper
- Create walkthrough tabtarget
- Wire workbench routing
- Add governor section to reference view (`rbw-gOr`)

## References
- `Tools/rbk/rbgm_ManualProcedures.sh` — governor manual procedures
- `Tools/rbk/rbgi_IAM.sh` — IAM grant operations
- `Tools/rbk/rbgg_Governor.sh` — governor SA management
- Tabtargets: `rbw-aM` (mantle), `rbw-aK` (knight), `rbw-aC` (charter), `rbw-aL` (list), `rbw-aF` (forfeit)

**[260407-0958] rough**

## Character
Extension pace — administrative role, creating resources for others. Shorter than director but conceptually different: the governor doesn't build or run things, they provision access for those who do.

## Docket

Build the governor walkthrough guide (`tt/rbw-gOG.OnboardGovernor.sh`). Three educational units covering project access, SA lifecycle, and IAM delegation.

### Unit 1 — Project Access

**Concept taught**: Depot as a provisioned GCP project. The governor works within a depot that the payor created.

**Terms introduced**:
- `governor` — a role that administers a depot: creates service accounts, manages access

**Probe**: gcloud project accessible with current credentials; depot initialized.

**Guidance**: Verify project access. If depot doesn't exist, this is a payor responsibility (link to payor track). Governor works within an existing depot.

**Confirmation**: Depot accessible. Green.

### Unit 2 — Service Account Lifecycle

**Concept taught**: The governor as SA factory — chartering retrievers and knighting directors.

**Terms introduced**:
- `charter` — create a retriever service account (read access)
- `knight` — create a director service account (build access)

**Probe**: Retriever SA exists; director SA exists.

**Guidance**: Run `tt/rbw-aC.GovernorChartersRetriever.sh` to charter a retriever SA. Run `tt/rbw-aK.GovernorKnightsDirector.sh` to knight a director SA. Explain: these SAs are the credentials you hand to retriever and director users. List with `tt/rbw-aL.GovernorListsServiceAccounts.sh`.

**Confirmation**: Both SAs exist, keys downloadable. Green.

### Unit 3 — IAM Grants

**Concept taught**: Least-privilege access. The SA alone isn't enough — it needs specific IAM grants to function.

**Terms introduced**:
- (No new glossary terms — IAM is GCP vocabulary, not Recipe Bottle vocabulary)

**Probe**: Retriever SA has correct role bindings on GAR; director SA has correct role bindings on GAR and Cloud Build.

**Guidance**: Apply IAM grants. Explain the principle: each SA gets exactly the permissions its role requires, no more. Retriever gets read. Director gets read + write + build trigger.

**Confirmation**: Role bindings verified. Green.

### Wire
- Mint `rbw-gOG` colophon in zipper
- Create walkthrough tabtarget
- Wire workbench routing
- Add governor section to reference view (`rbw-gOr`)

## References
- `Tools/rbk/rbgm_ManualProcedures.sh` — governor manual procedures
- `Tools/rbk/rbgi_IAM.sh` — IAM grant operations
- `Tools/rbk/rbgg_Governor.sh` — governor SA management
- Tabtargets: `rbw-aM` (mantle), `rbw-aK` (knight), `rbw-aC` (charter), `rbw-aL` (list), `rbw-aF` (forfeit)

**[260407-0941] rough**

## Character
Extension pace — administrative role, creating resources for others. Shorter than director but conceptually different: the governor doesn't build or run things, they provision access for those who do.

## Docket

Build the governor walkthrough and reference guides. Three educational units covering project access, SA lifecycle, and IAM delegation.

### Unit 1 — Project Access

**Concept taught**: Depot, GCP project relationship. A depot is a provisioned GCP project with the right APIs enabled and a GAR repository ready.

**Probe**: gcloud project accessible with current credentials; depot initialized.

**Guidance**: Verify project access. If depot doesn't exist, this is a payor responsibility (link to payor track). Governor works within an existing depot.

**Confirmation**: Depot accessible. Green.

### Unit 2 — Service Account Lifecycle

**Concept taught**: Governor role as SA factory. The governor creates service accounts for other roles — chartering a retriever (read access), knighting a director (build access).

**Probe**: Retriever SA exists; director SA exists.

**Guidance**: Run `tt/rbw-aC.GovernorChartersRetriever.sh` to charter a retriever SA. Run `tt/rbw-aK.GovernorKnightsDirector.sh` to knight a director SA. Explain: these SAs are the credentials you hand to retriever and director users. List with `tt/rbw-aL.GovernorListsServiceAccounts.sh`.

**Confirmation**: Both SAs exist, keys downloadable. Green.

### Unit 3 — IAM Grants

**Concept taught**: Role bindings, least-privilege model. The SA alone isn't enough — it needs IAM grants on specific depot resources to function.

**Probe**: Retriever SA has correct role bindings on GAR; director SA has correct role bindings on GAR and Cloud Build.

**Guidance**: Apply IAM grants. Explain the principle: each SA gets exactly the permissions its role requires, no more. Retriever gets read. Director gets read + write + build trigger.

**Confirmation**: Role bindings verified. Green.

### Wire
- Mint governor colophons in zipper
- Create walkthrough and reference tabtargets
- Wire workbench routing

## References
- `Tools/rbk/rbgm_ManualProcedures.sh` — governor manual procedures
- `Tools/rbk/rbgi_IAM.sh` — IAM grant operations
- `Tools/rbk/rbgg_Governor.sh` — governor SA management
- Tabtargets: `rbw-aM` (mantle), `rbw-aK` (knight), `rbw-aC` (charter), `rbw-aL` (list), `rbw-aF` (forfeit)

**[260403-0943] rough**

## Character
Linear probe sequence — administrative operations, creating resources for others.

## Docket

Build `tt/rbw-gOG.OnboardGovernor.sh` — the governor's onboarding guide.

### Probes (in order)

1. **Project access verified** — gcloud project accessible with current credentials
2. **Depot exists** — depot initialized in the project
3. **Governor SA active** — governor service account exists and is functional
4. **Retriever SA created** — SA for retriever role exists, key downloadable via `tt/rbw-aC.GovernorChartersRetriever.sh`
5. **Director SA created** — SA for director role exists, key downloadable via `tt/rbw-aK.GovernorKnightsDirector.sh`
6. **IAM grants applied** — verify role bindings for retriever and director SAs on depot resources

Each step: probe → green confirmation or red with action. Governor steps are `rbgm_*` manual procedure calls or account management via `rbw-a*` tabtargets. `buc_link` to README.md glossary anchors.

### Wire
- Mint `rbw-gOG` colophon in zipper
- Create tabtarget `tt/rbw-gOG.OnboardGovernor.sh`
- Wire workbench routing

## References
- `Tools/rbk/rbgm_ManualProcedures.sh` — governor manual procedures
- `Tools/rbk/rbgi_IAM.sh` — IAM grant operations
- `Tools/rbk/rbgg_Governor.sh` — governor SA management
- Post-rename tabtargets: `rbw-aM` (mantle), `rbw-aK` (knight), `rbw-aC` (charter), `rbw-aL` (list), `rbw-aF` (forfeit)

**[260402-1618] rough**

## Character
Linear probe sequence — administrative operations, creating resources for others.

## Docket

Build `tt/rbw-gOG.OnboardGovernor.sh` — the governor's onboarding guide.

### Probes (in order)

1. **Project access verified** — gcloud project accessible with current credentials
2. **Depot exists** — depot initialized in the project
3. **Governor SA active** — governor service account exists and is functional
4. **Retriever SA created** — SA for retriever role exists, key downloadable
5. **Director SA created** — SA for director role exists, key downloadable
6. **IAM grants applied** — verify role bindings for retriever and director SAs on depot resources

Each step: probe → green confirmation or red with action. Governor steps are mostly `rbgm_*` manual procedure calls or `rbgi_*` IAM operations.

### Wire
- Mint `rbw-gOG` colophon in zipper
- Create tabtarget `tt/rbw-gOG.OnboardGovernor.sh`
- Wire workbench routing

## References
- `Tools/rbk/rbgm_ManualProcedures.sh` — governor manual procedures
- `Tools/rbk/rbgi_IAM.sh` — IAM grant operations
- `Tools/rbk/rbgg_Governor.sh` — governor SA management
- `Tools/rbk/rgbs_ServiceAccounts.sh` — SA creation

**[260402-1521] rough**

## Character
Linear probe sequence — administrative operations, creating resources for others.

## Docket

Build `tt/rbw-OG.OnboardGovernor.sh` — the governor's onboarding guide.

### Probes (in order)

1. **Project access verified** — gcloud project accessible with current credentials
2. **Depot exists** — depot initialized in the project
3. **Governor SA active** — governor service account exists and is functional
4. **Retriever SA created** — SA for retriever role exists, key downloadable
5. **Director SA created** — SA for director role exists, key downloadable
6. **IAM grants applied** — verify role bindings for retriever and director SAs on depot resources

Each step: probe → green confirmation or red with action. Governor steps are mostly `rbgm_*` manual procedure calls or `rbgi_*` IAM operations.

### Wire
- Mint `rbw-OG` colophon in zipper
- Create tabtarget `tt/rbw-OG.OnboardGovernor.sh`
- Wire workbench routing

## References
- `Tools/rbk/rbgm_ManualProcedures.sh` — governor manual procedures
- `Tools/rbk/rbgi_IAM.sh` — IAM grant operations
- `Tools/rbk/rbgg_Governor.sh` — governor SA management
- `Tools/rbk/rgbs_ServiceAccounts.sh` — SA creation

### onboard-payor-track (₢A3AAE) [complete]

**[260407-2222] complete**

## Character
Extension pace — heaviest track, project-level and billing operations. The payor is the root of the trust chain: they create the project, fund it, and delegate to the governor. Each unit introduces a layer of the foundation.

## Docket

Build the payor walkthrough guide (`tt/rbw-gOP.OnboardPayor.sh`). Four educational units progressing from OAuth bootstrap through project setup to governor delegation.

### Term Self-Containment

This track introduces all terms it uses. A payor-only user encounters every concept they need through this track alone.

**Full term introduction set for this track:**
- Unit 1: `payor`
- Unit 2: (no new terms)
- Unit 3: `depot`, `levy`
- Unit 4: `governor`

### Unit 1 — OAuth Bootstrap

**Concept taught**: The payor authenticates via OAuth, not SA keys, because they own the GCP project itself.

**Terms introduced**:
- `payor` — the role that owns the GCP project and funds it; authenticates via OAuth

**Probe**: `rbro-payor.env` exists with client ID/secret; OAuth token refresh succeeds.

**Guidance**: Run `tt/rbw-gPI.PayorInstall.sh` to ingest OAuth credentials from JSON key file. If token expired, run `tt/rbw-gPR.PayorRefresh.sh`. Explain: OAuth is different from SA keys — it represents the human project owner, not a service role.

**Confirmation**: OAuth token valid. Green.

### Unit 2 — Project Setup

**Concept taught**: A depot needs a funded GCP project with the right APIs enabled.

**Terms introduced**:
- (No new terms — GCP project and billing are external concepts)

**Probe**: Project ID set in regime; project accessible; billing account linked.

**Guidance**: Run `tt/rbw-gPE.PayorEstablish.sh` for guided project + OAuth consent screen setup. Explain: billing must be enabled before any resources can be provisioned. The project ID becomes the depot's identity.

**Confirmation**: Project accessible, billing enabled. Green.

### Unit 3 — Depot Provisioning

**Concept taught**: Levying a depot provisions the infrastructure: APIs, GAR repository, Cloud Storage bucket.

**Terms introduced**:
- `depot` — the facility where container images are built and stored (GCP project + registry + bucket)
- `levy` — provision a new depot's GCP infrastructure

**Probe**: Depot exists; bucket accessible; GAR repository present.

**Guidance**: Run `tt/rbw-dL.PayorLeviesDepot.sh` to levy. Explain what was created and why. List depots with `tt/rbw-dl.PayorListsDepots.sh`.

**Confirmation**: Depot fully provisioned. Green.

### Unit 4 — Governor Handoff

**Concept taught**: The trust boundary — payor funds, governor operates. After this handoff, the governor can act independently.

**Terms introduced**:
- `governor` — a role that administers a depot: creates service accounts, manages access

**Probe**: Governor SA exists and is functional.

**Guidance**: Run `tt/rbw-aM.PayorMantlesGovernor.sh` to create the governor SA. Explain: from here, the governor can charter retrievers and knight directors independently. The payor's job for this depot is done unless billing or project-level changes are needed.

**Confirmation**: Governor SA active. Green.

### Wire
- Mint `rbw-gOP` colophon in zipper
- Create walkthrough tabtarget
- Wire workbench routing
- Add payor section to reference view (`rbw-gOr`)

## References
- `Tools/rbk/rbgo_OAuth.sh` — OAuth flow
- `Tools/rbk/rbgp_Payor.sh` — payor operations
- `Tools/rbk/rbgb_Buckets.sh` — depot/bucket management
- `Tools/rbk/rbgg_Governor.sh` — governor SA setup
- Tabtargets: `rbw-gPI` (install), `rbw-gPE` (establish), `rbw-gPR` (refresh), `rbw-dL` (levy), `rbw-dl` (list), `rbw-aM` (mantle)

**[260407-1011] rough**

## Character
Extension pace — heaviest track, project-level and billing operations. The payor is the root of the trust chain: they create the project, fund it, and delegate to the governor. Each unit introduces a layer of the foundation.

## Docket

Build the payor walkthrough guide (`tt/rbw-gOP.OnboardPayor.sh`). Four educational units progressing from OAuth bootstrap through project setup to governor delegation.

### Term Self-Containment

This track introduces all terms it uses. A payor-only user encounters every concept they need through this track alone.

**Full term introduction set for this track:**
- Unit 1: `payor`
- Unit 2: (no new terms)
- Unit 3: `depot`, `levy`
- Unit 4: `governor`

### Unit 1 — OAuth Bootstrap

**Concept taught**: The payor authenticates via OAuth, not SA keys, because they own the GCP project itself.

**Terms introduced**:
- `payor` — the role that owns the GCP project and funds it; authenticates via OAuth

**Probe**: `rbro-payor.env` exists with client ID/secret; OAuth token refresh succeeds.

**Guidance**: Run `tt/rbw-gPI.PayorInstall.sh` to ingest OAuth credentials from JSON key file. If token expired, run `tt/rbw-gPR.PayorRefresh.sh`. Explain: OAuth is different from SA keys — it represents the human project owner, not a service role.

**Confirmation**: OAuth token valid. Green.

### Unit 2 — Project Setup

**Concept taught**: A depot needs a funded GCP project with the right APIs enabled.

**Terms introduced**:
- (No new terms — GCP project and billing are external concepts)

**Probe**: Project ID set in regime; project accessible; billing account linked.

**Guidance**: Run `tt/rbw-gPE.PayorEstablish.sh` for guided project + OAuth consent screen setup. Explain: billing must be enabled before any resources can be provisioned. The project ID becomes the depot's identity.

**Confirmation**: Project accessible, billing enabled. Green.

### Unit 3 — Depot Provisioning

**Concept taught**: Levying a depot provisions the infrastructure: APIs, GAR repository, Cloud Storage bucket.

**Terms introduced**:
- `depot` — the facility where container images are built and stored (GCP project + registry + bucket)
- `levy` — provision a new depot's GCP infrastructure

**Probe**: Depot exists; bucket accessible; GAR repository present.

**Guidance**: Run `tt/rbw-dL.PayorLeviesDepot.sh` to levy. Explain what was created and why. List depots with `tt/rbw-dl.PayorListsDepots.sh`.

**Confirmation**: Depot fully provisioned. Green.

### Unit 4 — Governor Handoff

**Concept taught**: The trust boundary — payor funds, governor operates. After this handoff, the governor can act independently.

**Terms introduced**:
- `governor` — a role that administers a depot: creates service accounts, manages access

**Probe**: Governor SA exists and is functional.

**Guidance**: Run `tt/rbw-aM.PayorMantlesGovernor.sh` to create the governor SA. Explain: from here, the governor can charter retrievers and knight directors independently. The payor's job for this depot is done unless billing or project-level changes are needed.

**Confirmation**: Governor SA active. Green.

### Wire
- Mint `rbw-gOP` colophon in zipper
- Create walkthrough tabtarget
- Wire workbench routing
- Add payor section to reference view (`rbw-gOr`)

## References
- `Tools/rbk/rbgo_OAuth.sh` — OAuth flow
- `Tools/rbk/rbgp_Payor.sh` — payor operations
- `Tools/rbk/rbgb_Buckets.sh` — depot/bucket management
- `Tools/rbk/rbgg_Governor.sh` — governor SA setup
- Tabtargets: `rbw-gPI` (install), `rbw-gPE` (establish), `rbw-gPR` (refresh), `rbw-dL` (levy), `rbw-dl` (list), `rbw-aM` (mantle)

**[260407-0958] rough**

## Character
Extension pace — heaviest track, project-level and billing operations. The payor is the root of the trust chain: they create the project, fund it, and delegate to the governor. Each unit introduces a layer of the foundation.

## Docket

Build the payor walkthrough guide (`tt/rbw-gOP.OnboardPayor.sh`). Four educational units progressing from OAuth bootstrap through project setup to governor delegation.

### Unit 1 — OAuth Bootstrap

**Concept taught**: The payor authenticates via OAuth, not SA keys, because they own the GCP project itself.

**Terms introduced**:
- `payor` — the role that owns the GCP project and funds it

**Probe**: `rbro-payor.env` exists with client ID/secret; OAuth token refresh succeeds.

**Guidance**: Run `tt/rbw-gPI.PayorInstall.sh` to ingest OAuth credentials from JSON key file. If token expired, run `tt/rbw-gPR.PayorRefresh.sh`. Explain: OAuth is different from SA keys — it represents the human project owner, not a service role.

**Confirmation**: OAuth token valid. Green.

### Unit 2 — Project Setup

**Concept taught**: A depot needs a funded GCP project with the right APIs enabled.

**Terms introduced**:
- (No new glossary terms — GCP project and billing are external concepts)

**Probe**: Project ID set in regime; project accessible; billing account linked.

**Guidance**: Run `tt/rbw-gPE.PayorEstablish.sh` for guided project + OAuth consent screen setup. Explain: billing must be enabled before any resources can be provisioned. The project ID becomes the depot's identity.

**Confirmation**: Project accessible, billing enabled. Green.

### Unit 3 — Depot Provisioning

**Concept taught**: Levying a depot provisions the infrastructure: APIs, GAR repository, Cloud Storage bucket.

**Terms introduced**:
- `levy` — provision a new depot's GCP infrastructure

**Probe**: Depot exists; bucket accessible; GAR repository present.

**Guidance**: Run `tt/rbw-dL.PayorLeviesDepot.sh` to levy. Explain what was created and why. List depots with `tt/rbw-dl.PayorListsDepots.sh`.

**Confirmation**: Depot fully provisioned. Green.

### Unit 4 — Governor Handoff

**Concept taught**: The trust boundary — payor funds, governor operates. After this handoff, the governor can act independently.

**Terms introduced**:
- (governor already introduced in governor track; `mantle` if adding to glossary)

**Probe**: Governor SA exists and is functional.

**Guidance**: Run `tt/rbw-aM.PayorMantlesGovernor.sh` to create the governor SA. Explain: from here, the governor can charter retrievers and knight directors independently. The payor's job for this depot is done unless billing or project-level changes are needed.

**Confirmation**: Governor SA active. Green.

### Wire
- Mint `rbw-gOP` colophon in zipper
- Create walkthrough tabtarget
- Wire workbench routing
- Add payor section to reference view (`rbw-gOr`)

## References
- `Tools/rbk/rbgo_OAuth.sh` — OAuth flow
- `Tools/rbk/rbgp_Payor.sh` — payor operations
- `Tools/rbk/rbgb_Buckets.sh` — depot/bucket management
- `Tools/rbk/rbgg_Governor.sh` — governor SA setup
- Tabtargets: `rbw-gPI` (install), `rbw-gPE` (establish), `rbw-gPR` (refresh), `rbw-dL` (levy), `rbw-dl` (list), `rbw-aM` (mantle)

**[260407-0941] rough**

## Character
Extension pace — heaviest track, project-level and billing operations. The payor is the root of the trust chain: they create the project, fund it, and delegate to the governor. Each unit introduces a layer of the foundation.

## Docket

Build the payor walkthrough and reference guides. Four educational units progressing from OAuth bootstrap through project setup to governor delegation.

### Unit 1 — OAuth Bootstrap

**Concept taught**: OAuth consent screen, credential flow. The payor authenticates via OAuth (not SA keys) because they own the GCP project itself.

**Probe**: `rbro-payor.env` exists with client ID/secret; OAuth token refresh succeeds.

**Guidance**: Run `tt/rbw-gPI.PayorInstall.sh` to ingest OAuth credentials from JSON key file. If token expired, run `tt/rbw-gPR.PayorRefresh.sh`. Explain: OAuth is different from SA keys — it represents the human project owner, not a service role.

**Confirmation**: OAuth token valid. Green.

### Unit 2 — Project Setup

**Concept taught**: GCP project, billing linkage. A depot needs a funded GCP project with the right APIs enabled.

**Probe**: Project ID set in regime; project accessible; billing account linked.

**Guidance**: Run `tt/rbw-gPE.PayorEstablish.sh` for guided project + OAuth consent screen setup. Explain: billing must be enabled before any resources can be provisioned. The project ID becomes the depot's identity.

**Confirmation**: Project accessible, billing enabled. Green.

### Unit 3 — Depot Provisioning

**Concept taught**: Levy, depot lifecycle. Levying a depot provisions the GCP infrastructure: enables APIs, creates GAR repository, creates Cloud Storage bucket.

**Probe**: Depot exists; bucket accessible; GAR repository present.

**Guidance**: Run `tt/rbw-dL.PayorLeviesDepot.sh` to levy. Explain what was created and why. List depots with `tt/rbw-dl.PayorListsDepots.sh`.

**Confirmation**: Depot fully provisioned. Green.

### Unit 4 — Governor Handoff

**Concept taught**: SA delegation model, the role chain. The payor creates a governor SA, then the governor takes over day-to-day administration. This is the trust boundary: payor funds, governor operates.

**Probe**: Governor SA exists and is functional.

**Guidance**: Run `tt/rbw-aM.PayorMantlesGovernor.sh` to create the governor SA. Explain: from here, the governor can charter retrievers and knight directors independently. The payor's job for this depot is done unless billing or project-level changes are needed.

**Confirmation**: Governor SA active. Green.

### Wire
- Mint payor colophons in zipper
- Create walkthrough and reference tabtargets
- Wire workbench routing

## References
- `Tools/rbk/rbgo_OAuth.sh` — OAuth flow
- `Tools/rbk/rbgp_Payor.sh` — payor operations
- `Tools/rbk/rbgb_Buckets.sh` — depot/bucket management
- `Tools/rbk/rbgg_Governor.sh` — governor SA setup
- Tabtargets: `rbw-gPI` (install), `rbw-gPE` (establish), `rbw-gPR` (refresh), `rbw-dL` (levy), `rbw-dl` (list), `rbw-aM` (mantle)

**[260403-0943] rough**

## Character
Linear probe sequence — heaviest track, project-level and billing operations.

## Docket

Build `tt/rbw-gOP.OnboardPayor.sh` — the payor's onboarding guide.

### Probes (in order)

1. **OAuth credentials present** — `rbro-payor.env` exists with client ID/secret
2. **OAuth token valid** — token refresh succeeds
3. **GCP project configured** — project ID set in regime, accessible
4. **Billing enabled** — billing account linked to project
5. **Depot initialized** — depot exists, bucket accessible (levy via `tt/rbw-dL.PayorLeviesDepot.sh`)
6. **Governor SA created** — governor SA exists via `tt/rbw-aM.PayorMantlesGovernor.sh`
7. **Governor reset clean** — governor IAM grants in expected state

Each step: probe → green confirmation or red with action. Payor steps involve `rbgo_*` OAuth, `rbgp_*` Payor, and depot management via `rbw-d*` tabtargets. `buc_link` to README.md glossary anchors.

### Wire
- Mint `rbw-gOP` colophon in zipper
- Create tabtarget `tt/rbw-gOP.OnboardPayor.sh`
- Wire workbench routing

## References
- `Tools/rbk/rbgo_OAuth.sh` — OAuth flow
- `Tools/rbk/rbgp_Payor.sh` — payor operations
- `Tools/rbk/rbgb_Buckets.sh` — depot/bucket management
- `Tools/rbk/rbgg_Governor.sh` — governor SA setup
- Post-rename tabtargets: `rbw-dL` (levy), `rbw-dU` (unmake), `rbw-dl` (list), `rbw-aM` (mantle)

**[260402-1618] rough**

## Character
Linear probe sequence — heaviest track, project-level and billing operations.

## Docket

Build `tt/rbw-gOP.OnboardPayor.sh` — the payor's onboarding guide.

### Probes (in order)

1. **OAuth credentials present** — `rbro-payor.env` exists with client ID/secret
2. **OAuth token valid** — token refresh succeeds
3. **GCP project configured** — project ID set in regime, accessible
4. **Billing enabled** — billing account linked to project
5. **Depot initialized** — depot exists, bucket accessible
6. **Governor SA created** — governor SA exists for depot administration
7. **Governor reset clean** — governor IAM grants in expected state

Each step: probe → green confirmation or red with action. Payor steps involve `rbgo_*` OAuth, `rbgp_*` Payor, `rbgb_*` Buckets, and `rbgg_*` Governor operations.

### Wire
- Mint `rbw-gOP` colophon in zipper
- Create tabtarget `tt/rbw-gOP.OnboardPayor.sh`
- Wire workbench routing

## References
- `Tools/rbk/rbgo_OAuth.sh` — OAuth flow
- `Tools/rbk/rbgp_Payor.sh` — payor operations
- `Tools/rbk/rbgb_Buckets.sh` — depot/bucket management
- `Tools/rbk/rbgg_Governor.sh` — governor SA setup

**[260402-1521] rough**

## Character
Linear probe sequence — heaviest track, project-level and billing operations.

## Docket

Build `tt/rbw-OP.OnboardPayor.sh` — the payor's onboarding guide.

### Probes (in order)

1. **OAuth credentials present** — `rbro-payor.env` exists with client ID/secret
2. **OAuth token valid** — token refresh succeeds
3. **GCP project configured** — project ID set in regime, accessible
4. **Billing enabled** — billing account linked to project
5. **Depot initialized** — depot exists, bucket accessible
6. **Governor SA created** — governor SA exists for depot administration
7. **Governor reset clean** — governor IAM grants in expected state

Each step: probe → green confirmation or red with action. Payor steps involve `rbgo_*` OAuth, `rbgp_*` Payor, `rbgb_*` Buckets, and `rbgg_*` Governor operations.

### Wire
- Mint `rbw-OP` colophon in zipper
- Create tabtarget `tt/rbw-OP.OnboardPayor.sh`
- Wire workbench routing

## References
- `Tools/rbk/rbgo_OAuth.sh` — OAuth flow
- `Tools/rbk/rbgp_Payor.sh` — payor operations
- `Tools/rbk/rbgb_Buckets.sh` — depot/bucket management
- `Tools/rbk/rbgg_Governor.sh` — governor SA setup

### readme-glossary-anchor-capitalization (₢A3AAH) [complete]

**[260408-0932] complete**

## Character
Mechanical find-and-replace across README.md and guide callsites. No judgment calls — every anchor gets the same treatment.

## Docket

Capitalize all 27 glossary anchor IDs in README.md from lowercase to title case (e.g., `<a id="vessel">` → `<a id="Vessel">`), and update all references:

### README.md
- All `<a id="...">` definitions: lowercase → title case
- All `[text](#anchor)` cross-links within the glossary table
- All `[**Role**](#role)` links in the roles table

### Guide callsites (rbgm_ManualProcedures.sh)
- All `bug_tlt` URLs containing `${z_docs}#anchor` fragments
- After this, `zrbgm_triage_role` drops its 3rd parameter (anchor) and uses `z_name` directly

### Verification
- All `bug_tlt` URLs resolve to valid anchors
- All README cross-links resolve
- Triage helper simplified to 3 parameters

**[260408-0916] rough**

## Character
Mechanical find-and-replace across README.md and guide callsites. No judgment calls — every anchor gets the same treatment.

## Docket

Capitalize all 27 glossary anchor IDs in README.md from lowercase to title case (e.g., `<a id="vessel">` → `<a id="Vessel">`), and update all references:

### README.md
- All `<a id="...">` definitions: lowercase → title case
- All `[text](#anchor)` cross-links within the glossary table
- All `[**Role**](#role)` links in the roles table

### Guide callsites (rbgm_ManualProcedures.sh)
- All `bug_tlt` URLs containing `${z_docs}#anchor` fragments
- After this, `zrbgm_triage_role` drops its 3rd parameter (anchor) and uses `z_name` directly

### Verification
- All `bug_tlt` URLs resolve to valid anchors
- All README cross-links resolve
- Triage helper simplified to 3 parameters

### buh-handbook-rename-sweep (₢A3AAI) [complete]

**[260409-0845] complete**

## Character
Mechanical rename sweep across a known file set — disciplined find/replace, completeness over design judgment. Spans ~900 symbol occurrences but within narrow territory. Prerequisite for Pace β (rbgm split) because that pace consumes `buh_*` symbols.

## Docket

Rename the BUK guide module to "handbook" to prepare for the `rbh*` Handbook cluster in rbk. This pace only touches BUK and its consumers — it does not yet split rbgm.

### File Rename
- `Tools/buk/bug_guide.sh` → `Tools/buk/buh_handbook.sh`

### Symbol Renames
- Public functions: `bug_*` → `buh_*` (e.g., `bug_tlt` → `buh_tlt`, `bug_log_args` → `buh_log_args`)
- Private helpers: `zbug_*` → `zbuh_*`
- State variables: `ZBUG_*` → `ZBUH_*` (68 occurrences, all internal to the module)
- Module identity: `ZBUG_SOURCED` → `ZBUH_SOURCED`, `ZBUG_KINDLED` → `ZBUH_KINDLED`
- Kindle function: `zbug_kindle` → `zbuh_kindle`

### Live Touch Points (7 files, ~900 occurrences)
- `Tools/buk/buh_handbook.sh` — renamed file, 48 self-refs + 68 ZBUG_*
- `Tools/buk/butt_testbench.sh` — 1 ref
- `Tools/buk/buts/butclc_LinkCombinator.sh` — 11 refs
- `Tools/rbk/rbgm_ManualProcedures.sh` — **704 refs** (heaviest consumer — deferred rename in Pace β)
- `Tools/rbk/rbgm_cli.sh` — 3 refs (source path + branch cases)
- `Tools/rbk/rbgp_cli.sh` — 1 ref
- `Tools/rbk/rbgp_Payor.sh` — 12 refs
- `Tools/rbk/rblm_cli.sh` — 59 refs
- `CLAUDE.md` — 1 ref (acronym map BUG → BUH)

### Infrastructure Touch Points
- `Tools/buk/buz_zipper.sh` — module registration if bug is tracked there
- Any workbench kindle calls — `zbug_kindle` → `zbuh_kindle`
- Any remaining `source .../bug_guide.sh` paths in cli dispatchers

### Historical Record (leave as-is)
- Retired `jjh_*` files in `.claude/jjm/retired/` — historical record, do not update
- `.claude/jjm/jjg_gallops.json` — JJ state, commit history is immutable
- `Memos/memo-20260110-acronym-selection-study.md` — historical analysis, leave untouched

### Validation
- `tt/buw-st.BukSelfTest.sh` passes (9 cases — exercises the renamed display module)
- `tt/rbtd-s.TestSuite.fast.sh` passes (75 cases — catches any consumer breakage)
- Manual smoke: `tt/rbw-go.OnboardMAIN.sh` renders triage correctly; `tt/rbw-gOR.OnboardRetriever.sh` walks through retriever track with term introductions visible via `buh_tlt` links

### Out of Scope
- Rename of `rbgm → rbho/rbhp` (Pace β)
- Retirement of `rbgm_onboarding()` old monolithic (Pace β)
- Any semantic changes to display behavior — this pace is pure rename
- Updating historical records in retired heats, memos, or gallops state

## References
- `Tools/buk/bug_guide.sh` — current source of truth
- `₢A3AAG` (bug-link-combinator) — established the `l` combinator this pace renames
- `CLAUDE.md` — acronym map to update

**[260409-0821] rough**

## Character
Mechanical rename sweep across a known file set — disciplined find/replace, completeness over design judgment. Spans ~900 symbol occurrences but within narrow territory. Prerequisite for Pace β (rbgm split) because that pace consumes `buh_*` symbols.

## Docket

Rename the BUK guide module to "handbook" to prepare for the `rbh*` Handbook cluster in rbk. This pace only touches BUK and its consumers — it does not yet split rbgm.

### File Rename
- `Tools/buk/bug_guide.sh` → `Tools/buk/buh_handbook.sh`

### Symbol Renames
- Public functions: `bug_*` → `buh_*` (e.g., `bug_tlt` → `buh_tlt`, `bug_log_args` → `buh_log_args`)
- Private helpers: `zbug_*` → `zbuh_*`
- State variables: `ZBUG_*` → `ZBUH_*` (68 occurrences, all internal to the module)
- Module identity: `ZBUG_SOURCED` → `ZBUH_SOURCED`, `ZBUG_KINDLED` → `ZBUH_KINDLED`
- Kindle function: `zbug_kindle` → `zbuh_kindle`

### Live Touch Points (7 files, ~900 occurrences)
- `Tools/buk/buh_handbook.sh` — renamed file, 48 self-refs + 68 ZBUG_*
- `Tools/buk/butt_testbench.sh` — 1 ref
- `Tools/buk/buts/butclc_LinkCombinator.sh` — 11 refs
- `Tools/rbk/rbgm_ManualProcedures.sh` — **704 refs** (heaviest consumer — deferred rename in Pace β)
- `Tools/rbk/rbgm_cli.sh` — 3 refs (source path + branch cases)
- `Tools/rbk/rbgp_cli.sh` — 1 ref
- `Tools/rbk/rbgp_Payor.sh` — 12 refs
- `Tools/rbk/rblm_cli.sh` — 59 refs
- `CLAUDE.md` — 1 ref (acronym map BUG → BUH)

### Infrastructure Touch Points
- `Tools/buk/buz_zipper.sh` — module registration if bug is tracked there
- Any workbench kindle calls — `zbug_kindle` → `zbuh_kindle`
- Any remaining `source .../bug_guide.sh` paths in cli dispatchers

### Historical Record (leave as-is)
- Retired `jjh_*` files in `.claude/jjm/retired/` — historical record, do not update
- `.claude/jjm/jjg_gallops.json` — JJ state, commit history is immutable
- `Memos/memo-20260110-acronym-selection-study.md` — historical analysis, leave untouched

### Validation
- `tt/buw-st.BukSelfTest.sh` passes (9 cases — exercises the renamed display module)
- `tt/rbtd-s.TestSuite.fast.sh` passes (75 cases — catches any consumer breakage)
- Manual smoke: `tt/rbw-go.OnboardMAIN.sh` renders triage correctly; `tt/rbw-gOR.OnboardRetriever.sh` walks through retriever track with term introductions visible via `buh_tlt` links

### Out of Scope
- Rename of `rbgm → rbho/rbhp` (Pace β)
- Retirement of `rbgm_onboarding()` old monolithic (Pace β)
- Any semantic changes to display behavior — this pace is pure rename
- Updating historical records in retired heats, memos, or gallops state

## References
- `Tools/buk/bug_guide.sh` — current source of truth
- `₢A3AAG` (bug-link-combinator) — established the `l` combinator this pace renames
- `CLAUDE.md` — acronym map to update

### rbgm-split-to-rbho-rbhp (₢A3AAJ) [complete]

**[260409-1115] complete**

## Character
Structural refactor — file split with function renames, cli reorganization, acronym-map update. More design than mechanical: decisions about what lives where, how cli differentially furnishes, preserving dependency stacks. Requires validation the split preserves behavior across all `rbw-gO*`, `rbw-gP*`, and `rbw-gq` tabtargets. Prerequisite: Pace α (`bug → buh`) must be complete.

## Docket

Retire `rbgm_ManualProcedures.sh` and split its surviving content into two new modules under a new `rbh*` (Handbook) prefix, reflecting that manual procedures and guides are not Google subsystems and deserve their own tree parallel to `rbg*` (Google machinery).

### Rationale for the Split

The `rbg*` family is GCP subsystems (Artifact Registry, Buckets, IAM, OAuth, Payor machinery, etc.). `rbgm_ManualProcedures` has always been the outlier — human-facing procedures shoehorned into the Google tree. As guides proliferate, they deserve their own root. `rbh*` = Handbook.

Within the handbook, two axes:
- **rbho** = *cross-role teaching* (triage + role walkthroughs + reference). Organized around what users learn.
- **rbhp** = *payor-only ceremonies* (credential-gated actions only the GCP project owner can perform). Organized around who has authorization.

The onboarding/ceremonies split is semantic, not arbitrary: different users, different credentials, different dependency stacks, and the current `rbgm_cli.sh` already differential-furnishes them along this exact seam.

### Retirements (delete first)
- `rbgm_onboarding()` — old monolithic guide, superseded by `rbgm_onboard_*` in prior A3 paces
- `rbgm_LEGACY_setup_admin()` — ITCH_DELETE marker
- Associated probe helpers used only by the old monolithic: `zrbgm_po_*`, `zrbgm_probe_retriever_units`, `zrbgm_probe_director_units`, `zrbgm_probe_governor_units`, `zrbgm_probe_payor_units`, `zrbgm_po_review_defaults`

### New File: `Tools/rbk/rbho_onboarding.sh`

Cross-role educational walkthroughs. Thin dependency stack (no regime, no OAuth).

**Functions migrated:**
- `rbgm_onboard_triage` → `rbho_triage`
- `rbgm_onboard_reference` → `rbho_reference`
- `rbgm_onboard_retriever` → `rbho_retriever`
- `rbgm_onboard_director` → `rbho_director`
- `rbgm_onboard_governor` → `rbho_governor`
- `rbgm_onboard_payor` → `rbho_payor`

**Helpers migrated:**
- `zrbgm_probe_role_credentials` → `zrbho_probe_role_credentials`
- `zrbgm_triage_role` → `zrbho_triage_role`

**Dependencies:** `buh_handbook.sh`, `buz_zipper.sh`, `rbcc_Constants.sh`, `rbgc_Constants.sh`, `rbz_zipper.sh`

Module identity: `ZRBHO_SOURCED`, `ZRBHO_KINDLED`, `zrbho_kindle`

### New File: `Tools/rbk/rbhp_payor.sh`

Payor-only ceremonies — credential-gated actions only the GCP project owner can perform. Full GCP subsystem dependencies.

**Functions migrated:**
- `rbgm_payor_establish` → `rbhp_establish` (file prefix already says "payor" — don't repeat)
- `rbgm_payor_refresh` → `rbhp_refresh`
- `rbgm_quota_build` → `rbhp_quota_build` (keep as-is; legacy tabtarget shape, cleanup deferred)

**Helpers migrated (local to rbhp only):**
- `zrbgm_kindle` → `zrbhp_kindle`
- `zrbgm_sentinel` → `zrbhp_sentinel`
- `zrbgm_enforce` → `zrbhp_enforce`
- `zrbgm_show` → `zrbhp_show`
- All `zrbgm_*` display combinators (`s1`, `s2`, `s3`, `e`, `d`, `dc`, `dcd`, `dcdm`, `dm`, `dmd`, `dmdr`, `dmdm`, `dwdwd`, `dy`, `de`, `cmd`, `critic`, `dld`) → `zrbhp_*`
- Color state: `ZRBGM_R`, `ZRBGM_S`, `ZRBGM_C`, `ZRBGM_W`, `ZRBGM_Y`, `ZRBGM_CR`, `ZRBGM_CLICK_MOD`, `ZRBGM_RBRP_FILE` → `ZRBHP_*`

**Dependencies:** full regime + OAuth + IAM + manual procedures stack (mirrors current `rbgm_cli.sh` default branch)

### CLI Reorganization

Delete: `Tools/rbk/rbgm_cli.sh`

Create: `Tools/rbk/rbho_cli.sh` — thin furnish, derived from the current `rbgm_cli.sh` `rbgm_onboard_*` branch:
- Sources: `buh_handbook.sh`, `buz_zipper.sh`, `rbcc_Constants.sh`, `rbgc_Constants.sh`, `rbz_zipper.sh`, `rbho_onboarding.sh`
- Dispatch: `buc_execute rbho_ "Onboarding Guides" zrbho_furnish "$@"`

Create: `Tools/rbk/rbhp_cli.sh` — full GCP furnish, derived from the current `rbgm_cli.sh` default branch:
- Full regime, OAuth, IAM, utility, `rbhp_payor.sh`
- Dispatch: `buc_execute rbhp_ "Payor Ceremonies" zrbhp_furnish "$@"`

### Zipper Updates

`Tools/rbk/rbz_zipper.sh` — update colophon → module mapping:
- `rbw-go`, `rbw-gOR`, `rbw-gOD`, `rbw-gOG`, `rbw-gOP`, `rbw-gOr` → rbho module
- `rbw-gPE`, `rbw-gPR`, `rbw-gq` → rbhp module
- Note: `rbw-gPI` (PayorInstall) may live in `rbgp_Payor.sh` rather than `rbgm_ManualProcedures.sh` — verify during execution; if so, it is not affected by this pace

### Deletions (after migration complete)
- `Tools/rbk/rbgm_ManualProcedures.sh` — all content migrated or retired
- `Tools/rbk/rbgm_cli.sh` — replaced by `rbho_cli.sh` + `rbhp_cli.sh`

### Acronym Map Update (`CLAUDE.md`)

Remove: `RBGM → rbk/rbgm_ManualProcedures.sh`

Add:
- `RBHO → rbk/rbho_onboarding.sh` (handbook — cross-role onboarding)
- `RBHP → rbk/rbhp_payor.sh` (handbook — payor-only ceremonies)
- Note in Project Prefix Registry: `rbh*` = Handbook (human-facing procedures), parallel to `rbg*` (Google machinery)
- Reserve future slots documented but not created: `RBHG` (governor ceremonies), `RBHD` (director ceremonies), `RBHR` (retriever ceremonies)

### Validation

- `tt/rbtd-s.TestSuite.fast.sh` — all 75 cases pass
- Manual smoke for each affected tabtarget:
  - `tt/rbw-go.OnboardMAIN.sh` — triage detects roles and routes
  - `tt/rbw-gOR.OnboardRetriever.sh` — retriever walkthrough renders
  - `tt/rbw-gOD.OnboardDirector.sh` — director walkthrough renders
  - `tt/rbw-gOG.OnboardGovernor.sh` — governor walkthrough renders
  - `tt/rbw-gOP.OnboardPayor.sh` — payor walkthrough renders
  - `tt/rbw-gOr.OnboardReference.sh` — reference dashboard renders
  - `tt/rbw-gPE.PayorEstablish.sh` — OAuth consent walkthrough renders (do not complete the ceremony)
  - `tt/rbw-gPR.PayorRefresh.sh` — refresh procedure renders
  - `tt/rbw-gq.QuotaBuild.sh` — quota review text renders
- `ls tt/` — confirm no tabtarget references a deleted module

### Out of Scope
- Renaming `rbhp_quota_build` to something cleaner — legacy tabtarget name, defer to future cleanup
- Reshaping onboarding role tracks — separate conversation (the malformation discussion), happens inside `rbho_onboarding.sh` in a later pace
- Migration of any other guides to `rbh*` — none currently exist
- Touching retired `jjh_*` files, `jjg_gallops.json`, or memos — historical record

## References
- `Tools/rbk/rbgm_ManualProcedures.sh` — current home of all migrated content
- `Tools/rbk/rbgm_cli.sh` — differential furnish pattern to preserve across split
- `Tools/rbk/rbz_zipper.sh` — colophon registration to update
- `CLAUDE.md` — acronym map and project prefix registry
- `₢A3AAI` (buh-handbook-rename-sweep) — must complete first (this pace consumes `buh_*` symbols)

**[260409-0822] rough**

## Character
Structural refactor — file split with function renames, cli reorganization, acronym-map update. More design than mechanical: decisions about what lives where, how cli differentially furnishes, preserving dependency stacks. Requires validation the split preserves behavior across all `rbw-gO*`, `rbw-gP*`, and `rbw-gq` tabtargets. Prerequisite: Pace α (`bug → buh`) must be complete.

## Docket

Retire `rbgm_ManualProcedures.sh` and split its surviving content into two new modules under a new `rbh*` (Handbook) prefix, reflecting that manual procedures and guides are not Google subsystems and deserve their own tree parallel to `rbg*` (Google machinery).

### Rationale for the Split

The `rbg*` family is GCP subsystems (Artifact Registry, Buckets, IAM, OAuth, Payor machinery, etc.). `rbgm_ManualProcedures` has always been the outlier — human-facing procedures shoehorned into the Google tree. As guides proliferate, they deserve their own root. `rbh*` = Handbook.

Within the handbook, two axes:
- **rbho** = *cross-role teaching* (triage + role walkthroughs + reference). Organized around what users learn.
- **rbhp** = *payor-only ceremonies* (credential-gated actions only the GCP project owner can perform). Organized around who has authorization.

The onboarding/ceremonies split is semantic, not arbitrary: different users, different credentials, different dependency stacks, and the current `rbgm_cli.sh` already differential-furnishes them along this exact seam.

### Retirements (delete first)
- `rbgm_onboarding()` — old monolithic guide, superseded by `rbgm_onboard_*` in prior A3 paces
- `rbgm_LEGACY_setup_admin()` — ITCH_DELETE marker
- Associated probe helpers used only by the old monolithic: `zrbgm_po_*`, `zrbgm_probe_retriever_units`, `zrbgm_probe_director_units`, `zrbgm_probe_governor_units`, `zrbgm_probe_payor_units`, `zrbgm_po_review_defaults`

### New File: `Tools/rbk/rbho_onboarding.sh`

Cross-role educational walkthroughs. Thin dependency stack (no regime, no OAuth).

**Functions migrated:**
- `rbgm_onboard_triage` → `rbho_triage`
- `rbgm_onboard_reference` → `rbho_reference`
- `rbgm_onboard_retriever` → `rbho_retriever`
- `rbgm_onboard_director` → `rbho_director`
- `rbgm_onboard_governor` → `rbho_governor`
- `rbgm_onboard_payor` → `rbho_payor`

**Helpers migrated:**
- `zrbgm_probe_role_credentials` → `zrbho_probe_role_credentials`
- `zrbgm_triage_role` → `zrbho_triage_role`

**Dependencies:** `buh_handbook.sh`, `buz_zipper.sh`, `rbcc_Constants.sh`, `rbgc_Constants.sh`, `rbz_zipper.sh`

Module identity: `ZRBHO_SOURCED`, `ZRBHO_KINDLED`, `zrbho_kindle`

### New File: `Tools/rbk/rbhp_payor.sh`

Payor-only ceremonies — credential-gated actions only the GCP project owner can perform. Full GCP subsystem dependencies.

**Functions migrated:**
- `rbgm_payor_establish` → `rbhp_establish` (file prefix already says "payor" — don't repeat)
- `rbgm_payor_refresh` → `rbhp_refresh`
- `rbgm_quota_build` → `rbhp_quota_build` (keep as-is; legacy tabtarget shape, cleanup deferred)

**Helpers migrated (local to rbhp only):**
- `zrbgm_kindle` → `zrbhp_kindle`
- `zrbgm_sentinel` → `zrbhp_sentinel`
- `zrbgm_enforce` → `zrbhp_enforce`
- `zrbgm_show` → `zrbhp_show`
- All `zrbgm_*` display combinators (`s1`, `s2`, `s3`, `e`, `d`, `dc`, `dcd`, `dcdm`, `dm`, `dmd`, `dmdr`, `dmdm`, `dwdwd`, `dy`, `de`, `cmd`, `critic`, `dld`) → `zrbhp_*`
- Color state: `ZRBGM_R`, `ZRBGM_S`, `ZRBGM_C`, `ZRBGM_W`, `ZRBGM_Y`, `ZRBGM_CR`, `ZRBGM_CLICK_MOD`, `ZRBGM_RBRP_FILE` → `ZRBHP_*`

**Dependencies:** full regime + OAuth + IAM + manual procedures stack (mirrors current `rbgm_cli.sh` default branch)

### CLI Reorganization

Delete: `Tools/rbk/rbgm_cli.sh`

Create: `Tools/rbk/rbho_cli.sh` — thin furnish, derived from the current `rbgm_cli.sh` `rbgm_onboard_*` branch:
- Sources: `buh_handbook.sh`, `buz_zipper.sh`, `rbcc_Constants.sh`, `rbgc_Constants.sh`, `rbz_zipper.sh`, `rbho_onboarding.sh`
- Dispatch: `buc_execute rbho_ "Onboarding Guides" zrbho_furnish "$@"`

Create: `Tools/rbk/rbhp_cli.sh` — full GCP furnish, derived from the current `rbgm_cli.sh` default branch:
- Full regime, OAuth, IAM, utility, `rbhp_payor.sh`
- Dispatch: `buc_execute rbhp_ "Payor Ceremonies" zrbhp_furnish "$@"`

### Zipper Updates

`Tools/rbk/rbz_zipper.sh` — update colophon → module mapping:
- `rbw-go`, `rbw-gOR`, `rbw-gOD`, `rbw-gOG`, `rbw-gOP`, `rbw-gOr` → rbho module
- `rbw-gPE`, `rbw-gPR`, `rbw-gq` → rbhp module
- Note: `rbw-gPI` (PayorInstall) may live in `rbgp_Payor.sh` rather than `rbgm_ManualProcedures.sh` — verify during execution; if so, it is not affected by this pace

### Deletions (after migration complete)
- `Tools/rbk/rbgm_ManualProcedures.sh` — all content migrated or retired
- `Tools/rbk/rbgm_cli.sh` — replaced by `rbho_cli.sh` + `rbhp_cli.sh`

### Acronym Map Update (`CLAUDE.md`)

Remove: `RBGM → rbk/rbgm_ManualProcedures.sh`

Add:
- `RBHO → rbk/rbho_onboarding.sh` (handbook — cross-role onboarding)
- `RBHP → rbk/rbhp_payor.sh` (handbook — payor-only ceremonies)
- Note in Project Prefix Registry: `rbh*` = Handbook (human-facing procedures), parallel to `rbg*` (Google machinery)
- Reserve future slots documented but not created: `RBHG` (governor ceremonies), `RBHD` (director ceremonies), `RBHR` (retriever ceremonies)

### Validation

- `tt/rbtd-s.TestSuite.fast.sh` — all 75 cases pass
- Manual smoke for each affected tabtarget:
  - `tt/rbw-go.OnboardMAIN.sh` — triage detects roles and routes
  - `tt/rbw-gOR.OnboardRetriever.sh` — retriever walkthrough renders
  - `tt/rbw-gOD.OnboardDirector.sh` — director walkthrough renders
  - `tt/rbw-gOG.OnboardGovernor.sh` — governor walkthrough renders
  - `tt/rbw-gOP.OnboardPayor.sh` — payor walkthrough renders
  - `tt/rbw-gOr.OnboardReference.sh` — reference dashboard renders
  - `tt/rbw-gPE.PayorEstablish.sh` — OAuth consent walkthrough renders (do not complete the ceremony)
  - `tt/rbw-gPR.PayorRefresh.sh` — refresh procedure renders
  - `tt/rbw-gq.QuotaBuild.sh` — quota review text renders
- `ls tt/` — confirm no tabtarget references a deleted module

### Out of Scope
- Renaming `rbhp_quota_build` to something cleaner — legacy tabtarget name, defer to future cleanup
- Reshaping onboarding role tracks — separate conversation (the malformation discussion), happens inside `rbho_onboarding.sh` in a later pace
- Migration of any other guides to `rbh*` — none currently exist
- Touching retired `jjh_*` files, `jjg_gallops.json`, or memos — historical record

## References
- `Tools/rbk/rbgm_ManualProcedures.sh` — current home of all migrated content
- `Tools/rbk/rbgm_cli.sh` — differential furnish pattern to preserve across split
- `Tools/rbk/rbz_zipper.sh` — colophon registration to update
- `CLAUDE.md` — acronym map and project prefix registry
- `₢A3AAI` (buh-handbook-rename-sweep) — must complete first (this pace consumes `buh_*` symbols)

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 G bug-link-combinator
  2 A onboard-triage-and-infrastructure
  3 B onboard-retriever-track
  4 C onboard-director-track
  5 D onboard-governor-track
  6 E onboard-payor-track
  7 H readme-glossary-anchor-capitalization
  8 I buh-handbook-rename-sweep
  9 J rbgm-split-to-rbho-rbhp

GABCDEHIJ
·xxxxxxxx rbgm_ManualProcedures.sh
·x·····xx rbgm_cli.sh
·x····xx· README.md
·······xx CLAUDE.md
·x······x rbz_zipper.sh
x······x· bug_guide.sh, butclc_LinkCombinator.sh, butt_testbench.sh
········x RBS0-SpecTop.adoc, rbho_cli.sh, rbho_onboarding.sh, rbhp_cli.sh, rbhp_payor.sh
·······x· buh_handbook.sh, rbgp_Payor.sh, rbgp_cli.sh, rblm_cli.sh
··x······ rbk-claude-tabtarget-context.md, rbq_Qualify.sh
·x······· rbgc_Constants.sh, rbw-gO.Onboarding.sh, rbw-gOD.OnboardDirector.sh, rbw-gOG.OnboardGovernor.sh, rbw-gOP.OnboardPayor.sh, rbw-gOR.OnboardRetriever.sh, rbw-gOr.OnboardReference.sh, rbw-go.OnboardMAIN.sh

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 49 commits)

  1 G bug-link-combinator
  2 A onboard-triage-and-infrastructure
  3 B onboard-retriever-track
  4 C onboard-director-track
  5 D onboard-governor-track
  6 E onboard-payor-track
  7 F unknown
  8 H readme-glossary-anchor-capitalization
  9 I buh-handbook-rename-sweep
  10 J rbgm-split-to-rbho-rbhp

123456789abcdefghijklmnopqrstuvwxyz
·xx································  G  2c
···xx······························  A  2c
·····xxx···························  B  3c
········x·xx·······················  C  3c
·········x··x······················  D  2c
·············xx····················  E  2c
···············xxxxxxxx····x·······  F  9c
·························xx········  H  2c
······························xx···  I  2c
································x·x  J  2c
```

## Steeplechase

### 2026-04-09 11:15 - ₢A3AAJ - W

rbgm split to rbho/rbhp completed by another officium in parallel with interview ☉260409-1001. Created rbho_onboarding.sh (cross-role onboarding, thin dependency stack) and rbhp_payor.sh (payor-only ceremonies, full GCP dependencies) with rbho_cli.sh and rbhp_cli.sh dispatchers. Retired rbgm_ManualProcedures.sh and rbgm_cli.sh. Updated RBS0-SpecTop.adoc, CLAUDE.md acronym map (RBGM → RBHO/RBHP with rbh* as Handbook parallel to rbg* Google), and rbz_zipper.sh colophon mapping. Infrastructure now in place for ☺6 handbook pedagogy work. Administrative wrap from this session after user confirmed the other chat ended.

### 2026-04-09 10:18 - Heat - f

stabled

### 2026-04-09 09:15 - ₢A3AAJ - n

Retire rbgm_ManualProcedures.sh; split into rbho_onboarding.sh (handbook cross-role walkthroughs, thin deps) and rbhp_payor.sh (handbook payor-only ceremonies, full GCP stack). Delete rbgm_onboarding() retired stub, rbgm_LEGACY_setup_admin() ITCH marker, and zrbgm_po_review_defaults (truly unused). Migrate 6 walkthrough functions + probe helpers (po_status, po_extract_capture, probe_role_credentials, probe_retriever/director/governor/payor_units, triage_role) to zrbho_*; migrate 3 payor ceremonies (rbhp_establish, rbhp_refresh, rbhp_quota_build — quota rename deferred out of scope per docket) + full zrbhp_* display combinators and ZRBHP_* color state. Slim rbhp_kindle — drop unused temp-file constants that were for retired LEGACY_setup_admin and ONBOARDING_MONIKER/_NAMEPLATE that were only referenced from retired rbgm_onboarding dead code. Split rbgm_cli.sh into rbho_cli.sh (thin: buh/buz/rbcc/rbgc/rbz) and rbhp_cli.sh (full regime+OAuth+IAM stack). Update rbz_zipper.sh colophon routing: rbw-go/rbw-gO* -> rbho_cli.sh, rbw-gPE/rbw-gPR/rbw-gq -> rbhp_cli.sh (rbw-gPI remains in rbgp_cli.sh as docket predicted). Update CLAUDE.md acronym map (remove RBGM, add RBHO + RBHP with rbh*=Handbook family note and reserved RBHG/RBHD/RBHR slots). Update RBS0-SpecTop.adoc 3 display-text refs to point at new function names. Docket deviation: the probe_*_units and po_* helpers were listed in docket as 'used only by old monolithic' but are actually called directly by per-role walkthroughs — they were migrated to rbho as zrbho_* rather than deleted. Validation: tt/rbtd-s.TestSuite.fast.sh 75/75, tt/rbw-tf.QualifyFast.sh passes (tabtarget structure, colophon registrations, generated context), manual smoke rbw-go/gOR/gOD/gOG/gOP/gOr/gPE/gPR/gq all render with OSC-8 hyperlinks and resolved tabtarget colophons. Size limit raised to 150000 — rbho_onboarding.sh (~42KB) plus rbhp_payor.sh (~21KB) plus two new cli files plus full rbgm_ManualProcedures.sh deletion (1867 lines) make this unsplittable: partial commits would leave zipper routing broken or create dangling symbol references. Unblocks later A3 paces that reshape role tracks inside rbho_onboarding.sh.

### 2026-04-09 08:45 - ₢A3AAI - W

Renamed BUK guide module to handbook. File rename bug_guide.sh -> buh_handbook.sh plus symbol sweep bug_/zbug_/ZBUG_ -> buh_/zbuh_/ZBUH_ across 8 consumer files (~900 occurrences). Docket extensions: Tools/buk/README.md prefix table (not in docket, trivial doc fix), bug-link -> buh-link fixture ID in butt_testbench (docket undercounted this file), module header 'Bash Utility Guide' -> 'Bash Utility Handbook', and error tag 'bug:' -> 'buh:' in zbuh_tabtarget_fragment. Per-file count parity verified for every file. Validation: BUK self-test 18/18 (buh-link fixture cases exercise renamed combinators directly), rbtd fast suite 75/75, manual smoke rbw-go triage and rbw-gOR retriever walkthrough both render correctly with OSC-8 hyperlinks and tabtarget resolution intact. Size limit raised to 150000 for the notch because atomic mechanical rename cannot be split without breaking intermediate state. Unblocks rbgm-split-to-rbho-rbhp.

### 2026-04-09 08:43 - ₢A3AAI - n

Rename BUK guide module to handbook: bug_guide.sh -> buh_handbook.sh, sweep bug_/zbug_/ZBUG_ -> buh_/zbuh_/ZBUH_ across 8 consumer files (~900 occurrences, 704 in rbgm_ManualProcedures.sh alone), update CLAUDE.md acronym map (BUG -> BUH) and BUK README module prefix table; rename bug-link test fixture to buh-link. Header text 'Bash Utility Guide' -> 'Bash Utility Handbook' and error tag 'bug:' -> 'buh:' in renamed module. Validation: BUK self-test 18/18, rbtd fast suite 75/75, manual smoke rbw-go/rbw-gOR render correctly with OSC-8 hyperlinks and tabtarget resolution intact. Size limit raised to 150000 bytes (default 50000) because this is a pure mechanical rename — one identifier, atomic by nature, partial commits would leave the system broken.

### 2026-04-09 08:22 - Heat - S

rbgm-split-to-rbho-rbhp

### 2026-04-09 08:21 - Heat - S

buh-handbook-rename-sweep

### 2026-04-09 07:56 - ₢A3AAF - n

fix triage role underline bleeding into alignment padding by adding bug_tltT combinator (plain-text segment between link and tabtarget) and moving column padding outside the link envelope in zrbgm_triage_role

### 2026-04-08 09:32 - ₢A3AAH - W

Capitalized all 27 glossary anchor IDs in README.md from lowercase to title case, updated all cross-links in glossary and roles tables, capitalized all 50 bug_tlt URL fragments in rbgm_ManualProcedures.sh, and simplified zrbgm_triage_role from 4 to 3 parameters by dropping the redundant anchor arg (z_name now serves as the anchor directly)

### 2026-04-08 09:32 - ₢A3AAH - n

Capitalize glossary anchor IDs to match display terms — deduplicate triage_role anchor parameter

### 2026-04-08 09:19 - Heat - r

moved A3AAH before A3AAF

### 2026-04-08 09:16 - Heat - S

readme-glossary-anchor-capitalization

### 2026-04-08 09:16 - ₢A3AAF - n

Replace bash 3-incompatible ${,,} lowercase with explicit anchor parameter — zero subshells, zero computation, bash 3 compatible

### 2026-04-08 09:00 - ₢A3AAF - n

Eliminate 4 subshells from triage helper: bash ${,,} for lowercase, printf -v for padding, inline [*]/[ ] in branches

### 2026-04-08 08:57 - ₢A3AAF - n

Add bug_tlT combinator (text+link+tabtarget), update triage helper to link role names in both detected and absent cases for color distinction and consistent docs access

### 2026-04-08 08:56 - ₢A3AAF - n

Extract zrbgm_triage_role helper to collapse 24-line if/else role ladder into 4 declarative calls with derived anchor and printf-aligned labels

### 2026-04-08 08:53 - ₢A3AAF - n

Add T combinator to bug guide system (zbug_tabtarget_fragment + bug_T/tT/cT), fix buc_tabtarget to use glob instead of temp file, convert triage and director/governor back-links to symbolic colophon resolution

### 2026-04-08 08:46 - ₢A3AAF - n

Add bug_tabtarget() for guide-style colophon resolution using bash glob (no temp file), add zipper kindle to onboarding furnish path, convert two Triage back-links from hardcoded paths to symbolic RBZ_ONBOARD_TRIAGE

### 2026-04-08 08:27 - ₢A3AAF - n

Suppress logging (BURD_NO_LOG=1) on all 6 onboarding tabtargets and remove all 8 buc_success calls from onboarding functions — guides are display-only, no log trail or success banner needed

### 2026-04-08 08:20 - ₢A3AAF - n

Fix OSC-8 hyperlink escape sequence collision in zbug_link_fragment: ST trailing backslash was consuming ZBUG_R leading backslash in printf %b, causing raw 033[0m to leak into terminal output

### 2026-04-07 22:22 - ₢A3AAE - W

Built payor walkthrough with dual-mode rendering (4 units: OAuth bootstrap, project setup, depot provisioning, governor handoff), BCG-compliant payor probe function using case glob matching for config file scanning, expanded reference dashboard with per-unit payor probes, 4 term introductions via bug_tlt (payor, depot, levy, governor)

### 2026-04-07 22:20 - ₢A3AAE - n

Built payor walkthrough with dual-mode rendering (4 units: OAuth bootstrap, project setup, depot provisioning, governor handoff), BCG-compliant payor probe function, expanded reference dashboard with per-unit payor probes, 4 term introductions via bug_tlt

### 2026-04-07 22:12 - ₢A3AAD - W

Built governor walkthrough with 3-unit probe-driven progression (project access, SA lifecycle, verification), dual-mode rendering matching retriever/director pattern, 6 term introductions via bug_tlt, and expanded reference dashboard with per-unit governor probes

### 2026-04-07 22:12 - ₢A3AAC - W

Built director walkthrough with dual-mode rendering (7 units: credential gate, kludge, depot foundation, conjure, bind, graft, full ark comparison), BCG-compliant director probe function using temp files and case glob matching, expanded reference dashboard with per-unit director probes, glossary links for key verbs throughout, removed duplicate zrbgm_probe_role_credentials definition

### 2026-04-07 22:12 - ₢A3AAC - n

BCG compliance: replaced grep with case glob matching in single-pass loop, docker output to temp file with stderr capture, split multi-variable locals to one-per-line with initialization, local -r for immutables, eliminated duplicate conjure probe

### 2026-04-07 22:06 - ₢A3AAD - n

Built governor walkthrough with 3-unit probe-driven progression (project access, SA lifecycle, verification), dual-mode rendering, 6 term introductions via bug_tlt, and per-unit reference dashboard probes

### 2026-04-07 21:59 - ₢A3AAC - n

Director walkthrough with dual-mode rendering (7 units: credential gate, kludge, depot foundation, conjure, bind, graft, full ark comparison), director probe function with local filesystem+docker detection, expanded reference dashboard with per-unit director probes, removed duplicate zrbgm_probe_role_credentials definition, added glossary links for key verbs throughout

### 2026-04-07 21:44 - ₢A3AAB - W

Built retriever walkthrough with dual-mode rendering (walkthrough shows frontier unit, reference shows all green), 4 educational units with probe-driven progression, shared probe helpers for role credentials and per-unit status, expanded reference dashboard with per-unit retriever probes, exempt standalone buw-SI tabtarget from structural qualification

### 2026-04-07 21:40 - ₢A3AAB - n

Exempt standalone buw-SI tabtarget from structural qualification; regenerate context

### 2026-04-07 21:36 - ₢A3AAB - n

Retriever walkthrough with dual-mode rendering, 4 educational units with probe-driven frontier detection, shared probe helpers for role credentials and per-unit status, expanded reference dashboard

### 2026-04-07 21:22 - ₢A3AAA - W

Built onboarding triage and infrastructure: expanded README glossary to 26 cross-linked terms with HTML anchors; minted 6-colophon tree (rbw-go triage, 4 walkthrough stubs, rbw-gOr reference); added RBGC_PUBLIC_DOCS_URL constant; triage probes role credentials and routes detected roles to walkthroughs via OSC-8 links; reference dashboard skeleton shows per-role credential status; retired monolithic rbgm_onboarding with early-return stub; light CLI furnish path for new commands. Fast test suite passes (75/75).

### 2026-04-07 21:21 - ₢A3AAA - n

Onboarding triage and infrastructure: expanded 26-term cross-linked glossary with HTML anchors in README; minted 6-colophon tree (rbw-go triage, rbw-gOR/gOD/gOG/gOP walkthroughs, rbw-gOr reference); added RBGC_PUBLIC_DOCS_URL constant; built triage guide with role credential probing and OSC-8 links; reference dashboard skeleton; 4 walkthrough stubs; retired monolithic rbgm_onboarding

### 2026-04-07 21:07 - ₢A3AAG - W

Added l (link) combinator to bug_guide.sh: ZBUG_L color constant, zbug_link_fragment helper with BURD_NO_HYPERLINKS fallback, 5 combinators (bug_lt/tl/tlt/tlc/tltlt), and 3 test cases in new bug-link BUTT fixture. All 18 self-test cases pass.

### 2026-04-07 20:59 - ₢A3AAG - n

Add l (link) combinator to bug_guide.sh: zbug_link_fragment helper, 5 combinators (bug_lt/tl/tlt/tlc/tltlt), ZBUG_L color constant, and 3 test cases in new bug-link fixture

### 2026-04-07 10:09 - Heat - d

paddock curried: updated DDR: dual-mode, bug_tlt, glossary cross-links, per-track self-containment

### 2026-04-07 09:56 - Heat - S

bug-link-combinator

### 2026-04-03 09:42 - Heat - d

paddock curried: post-A1 rename: hallmark terminology, domain colophons, README.md consolidation, glossary integration

### 2026-04-03 08:53 - Heat - n

Replace lowercase readme.md with consumer README.md as the single user-facing project readme

### 2026-04-02 16:17 - Heat - d

paddock curried: corrected colophons from rbw-o/rbw-O to rbw-go/rbw-gO

### 2026-04-02 15:22 - Heat - S

docs-alignment-and-cross-machine-validation

### 2026-04-02 15:21 - Heat - S

onboard-payor-track

### 2026-04-02 15:21 - Heat - S

onboard-governor-track

### 2026-04-02 15:21 - Heat - S

onboard-director-track

### 2026-04-02 15:21 - Heat - S

onboard-retriever-track

### 2026-04-02 15:20 - Heat - T

onboard-triage-and-infrastructure

### 2026-04-02 15:19 - Heat - d

paddock curried: initial paddock from design conversation in AU groom

### 2026-04-02 15:19 - Heat - D

restring 1 paces from ₣AU

### 2026-04-02 15:18 - Heat - f

racing

### 2026-04-02 15:18 - Heat - N

rbk-mvp-3-onboarding-guides

