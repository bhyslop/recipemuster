# Heat Trophy: rbw-spec-axla-coherence

**Firemark:** ₣AS
**Created:** 260131
**Retired:** 260216
**Status:** retired

## Paddock

# Paddock: rbw-spec-axla-coherence

## Context

Bring Recipe Bottle specification ecosystem into coherent alignment with AXLA patterns. This heat establishes the structural foundation, culminating in RBSA — a single unified spec replacing both RBAGS and RBS.

## Decision: RBSA Consolidation (₢ASAAD)

**Decision**: Merge RBAGS-AdminGoogleSpec.adoc and RBS-Specification.adoc into a single RBSA-SpecTop.adoc. No two-level includes — RBSA is the parent, existing RBSXX subdocs are the only includes.

**Three-tier document structure:**

| Tier | Focus | Content source |
|------|-------|---------------|
| **Tier 1: Why and What** | Security architecture (significance-first) | RBS: trust challenge, bottle pattern, security properties |
| **Tier 2: How** | Operations in temporal order | RBAGS: cloud operations (by role). RBS: local operations (bottle lifecycle) — rewritten in AXLA style |
| **Tier 3: Reference** | Lookup material | Both: regimes, types, voicings, patterns, definitions, script internals, trade studies |

**Key sub-decisions:**

- **RBAGS patterns govern** — AXLA annotations, not CRR/cmk vocabulary, wherever applicable
- **cmk_* vocabulary is dead** — Console Makefile Discipline fully superseded by bash model (BUK)
- **Operations bifurcated from script internals** — Bottle lifecycle operations (sentry start, bottle start/stop) expressed as RBAGS-style procedures in Tier 2. Detailed script internals (iptables, DNS/dnsmasq, socat, eBPF) go to Tier 3 reference.
- **CRR not modified in this heat** — CRR-ConfigRegimeRequirements.adoc becomes orphaned when RBSA eliminates crg_* vocabulary; retirement is future work.

## Decision: Google Type Voicing Prefix (₢ASAAD, refining ₢ASAAG)

**Decision**: Google-specific types use `rbgt_*` prefix, NOT `rbst_gcp_*`.

**Rationale**: `rbst_gcp_*` is double-prefixing — a namespace hack. `rbgt_` is clean, sits naturally under `rbg` (already "RB Google" in the code namespace), and avoids the token/readability cost of compound prefixes.

**Two type voicing families in RBSA:**

| Prefix | Voices | Purpose |
|--------|--------|---------|
| `rbst_*` | `axtu_*` (universal AXLA types) | Universal types: xname, path, string, etc. |
| `rbgt_*` | `axtg_*` (Google AXLA types) | Google types: project_id, region, service_account, billing_account |

**New `rbgt_*` types** (in RBSA Type Voicings section):

| rbgt_ type | voices | RB constraint |
|---|---|---|
| `rbgt_project_id` | `axtg_project_id` | GCP project identifier in RB context |
| `rbgt_region` | `axtg_region` | GCP region in RB context |
| `rbgt_service_account` | `axtg_service_account` | Service account email in RB context |
| `rbgt_billing_account` | `axtg_billing_account` | Billing account identifier in RB context |

**Updated pattern for regime variable annotations:**
```asciidoc
[[rbrr_depot_project_id]]
// ⟦axl_voices axrg_variable rbgt_project_id⟧
{rbrr_depot_project_id}::
The GCP project where all Recipe Bottle resources are created.
```

**Updated pattern for type voicing definitions:**
```asciidoc
[[rbgt_project_id]]
// ⟦axl_voices axtg_project_id⟧
{rbgt_project_id}::
GCP project identifier. 6-30 lowercase letters, digits, and hyphens;
must start with letter, end with letter or digit.
```

**Note**: This supersedes the ₢ASAAG paddock entry that proposed `rbst_gcp_*`. The ₢ASAAG decision that Google types MUST get project-specific voicings still stands — only the prefix changed.

## Decision: Google Type Voicing Requirement (₢ASAAG)

**Decision**: Google-specific types (`axtg_*`) MUST get project-specific subspecializations, same as universal types.

**Rationale** (unchanged from original ₢ASAAG):

1. **`axtg_*` terms are annotation-only** — they appear exclusively in `// ⟦...⟧` Strachey bracket comments. Never AsciiDoc attribute references, never render in document body.

2. **Project type voicings are full document citizens** — `:rbst_*:` and `:rbgt_*:` attribute references with `<<anchor,Display>>` targets, render in tables and body text, participate in AsciiDoc's referential structure.

3. **Uniform type surface** — every regime variable table row uses `rbst_*` or `rbgt_*` in the Type column. No raw AXLA terms in tables. At a glance you can see which variables are Google-specific (`rbgt_*`) vs universal (`rbst_*`).

4. **Two-layer voicing is the design** — the type definition carries the `axtg_*` voicing in its own annotation. The regime variable voices the project type. The `axtg_*` knowledge is centralized in the type definition.

5. **Project-wide analysis** — querying "all variables typed as GCP project ID" is a grep for `rbgt_project_id`, not a comment-parsing exercise.

## Pattern Established in ₣AR (RBRV exemplar)

The RBRV regime subdocument (RBSRV) was cleaned up as an exemplar. Key patterns to follow:

### Type Voicing Pattern

1. **Parent doc defines type voicings** in "Type Voicings" section:
   - `rbst_*` voices universal AXLA types (`axtu_*`)
   - `rbgt_*` voices Google AXLA types (`axtg_*`)

2. **Regime variable voicings** reference subspecialized types:
   ```asciidoc
   [[rbrv_sigil]]
   // ⟦axl_voices axrg_variable rbst_xname⟧
   {rbrv_sigil}::
   Vessel identifier; basis for directory and ark naming.
   ```

3. **Subdocs use type voicings** in tables (they resolve via parent mappings).

4. **Subdocs have NO anchors** — parent doc owns all `[[anchor]]` definitions.

### Subdocument Structure

- **Overview**: Brief context, explain any bifurcations (e.g., binding vs conjuring)
- **Feature Groups**: Tables with Type column using `rbst_*` or `rbgt_*` references
- **NO "Core Term Definitions"** — parent doc owns these
- **Use NOTE blocks** for group-level applicability rules (not repeated per row)

## Architectural Insight: Annotation-First Structure (₢ASAAG discussion)

During ₢ASAAG we considered whether regimes need `axrs_*` section motifs (like `axs_*` for procedures). Decision: **NO**.

### Annotations ARE the structure

`axvr_*` and `axhr*_` annotations are **self-describing**. A linter can reconstruct the full regime hierarchy from annotations alone — no section headings needed to carry structural metadata. This contrasts with `axs_*` section motifs which embed structure in document headings.

### Deliberate duelling representations

The `axvr_*`/`axhr*_` annotation approach and the `axs_*` section approach are intentionally coexisting. Regimes use annotations exclusively. If the annotation-first pattern proves more efficient and consistent, it may eventually supersede section motifs for other domains too. **Do not create regime section vocabulary** — keep regimes as a clean testbed for the annotation-first approach.

### Parent vs subdoc: complementary layers, not unified

| Layer | Location | Annotations | Purpose |
|---|---|---|---|
| **Schema** | Parent (RBSA) | `axvr_*` | Anchors, types, cardinality, groups. One-sentence definitions. |
| **Operational** | Subdoc (RBSRV, RBRN) | `axhr*_` | Usage context, typical values, gate explanations, interactions. |

These stay separate:
- Parent definitions are terse type references — the "what"
- Subdoc expansions are operational narrative — the "how"
- `include::` connects them under the same heading
- ₢ASAAF reinforces this: move detail OUT of parent INTO subdoc

### ax_ terms are comment-only by design

All AXLA terms (`axt_*`, `axtu_*`, `axtg_*`, `axs_*`, etc.) appear exclusively in `// ⟦...⟧` Strachey bracket annotations or `//axvr_*`/`//axhr*_` prefix-discriminated annotations. They are **never** AsciiDoc attribute references (`:term: <<anchor,Display>>`). This is deliberate — the specs are too dense for AXLA terms to compete with project terms in body text. Project-specific voicings (`rbst_*`, `rbgt_*`, `rbrv_*`, etc.) are the only terms that render.

## RBS→RBSA Subdocument Explosion Map (₢ASAAH discussion)

**Decision**: Every anchored RBS procedure gets the RBAGS pattern treatment in RBSA:
```
[[anchor]]
// ⟦annotation⟧
=== {anchor}

include::RBSxx-name.adoc[]
```
Anchor+annotation+heading in RBSA parent; narrative detail in subdocument.
Existing attribute prefixes (`opss_*`, `mkr_*`, `scr_*`, `ops_*`, `opbs_*`, `opbr_*`) are NOT re-prefixed at this time.

**Existing RBSA subdoc suffixes (avoid collisions):**
`AA, DC, DD, DI, DL, GR, GS, ID, IR, OB, PE, PI, PR, RC, RV, SD, SL, TB`

**New subdocument map:**

| # | Attribute | Suffix | File | RBS lines | ~Size |
|---|---|---|---|---:|---:|
| 1 | `opss_sentry_start` | SS | `RBSSS-sentry_start.adoc` | 314-328 | 13 |
| 2 | `mkr_network_create` | NC | `RBSNC-network_create.adoc` | 329-386 | 57 |
| 3 | `mkr_network_connect` | NX | `RBSNX-network_connect.adoc` | 387-429 | 41 |
| 4 | `scr_security_config` | SC | `RBSSC-security_config.adoc` | 430-442 | 10 |
| 5 | `scr_iptables_init` | IP | `RBSIP-iptables_init.adoc` | 443-467 | 24 |
| 6 | `scr_port_setup` | PT | `RBSPT-port_setup.adoc` | 468-511 | 43 |
| 7 | `scr_access_setup` | AX | `RBSAX-access_setup.adoc` | 512-565 | 52 |
| 8 | `scr_dns_step` | DS | `RBSDS-dns_step.adoc` | 566-662 | 97 |
| 9 | `opbs_bottle_start` | BS | `RBSBS-bottle_start.adoc` | 663-670 | 7 |
| 10 | `mkr_bottle_cleanup` | BK | `RBSBK-bottle_cleanup.adoc` | 671-687 | 15 |
| 11 | `mkr_bottle_launch` | BL | `RBSBL-bottle_launch.adoc` | 688-712 | 24 |
| 12 | `opbr_bottle_run` | BR | `RBSBR-bottle_run.adoc` | 713-720 | 7 |
| 13 | `mkr_bottle_create` | BC | `RBSBC-bottle_create.adoc` | 829-849 | 19 |
| 14 | `mkr_command_exec` | CE | `RBSCE-command_exec.adoc` | 850-859 | 8 |
| 15 | `ops_rbv_check` | VC | `RBSVC-rbv_check.adoc` | 725-767 | 42 |
| 16 | `ops_rbv_mirror` | VM | `RBSVM-rbv_mirror.adoc` | 768-828 | 60 |

**Anchor-only (no procedure section, inline in RBSA parent):**
- `mkr_sentry_run` — definition only, no narrative
- `mkc_interface_check` — definition + code example

**Dead RBS sections (not imported):**
- Console Makefile Elements (`cmk_*` definitions) — dead vocabulary
- Station Regime (`cfg_station_regime`) — stub, dead `crg_*`
- Script Requirements (`cmk_script` framing) — dead; content may inform BUK conventions later

**RBS sections already in RBSA (verify during assess paces):**
- System Overview / Trust / Bottle / Security Properties (Tier 1)
- Base Regime Definitions (`rbrr_*`) — in RBSA Config Regimes
- Nameplate Regime Definitions (`rbrn_*`) — via RBRN include
- Supporting Infrastructure Definitions (`st_*`) — in RBSA Term Definitions
- Architecture Term Definitions — partially merged, gap analysis needed

## Downstream Work

**CRITICAL: After completing this heat**, plan a follow-up heat to **refine AXLA regime vocabulary itself** based on what we learn during:
- RBSA consolidation experience
- Creating BURS and BURC specs
- Studying all RB regimes (BUD_, RBRN, RBRR, etc.)

AXLA will need refinement to capture patterns we discover. That work belongs in a future heat (not in ₣AS). ₢ASAAA (expand-axla-regime-section) was abandoned from this heat for re-slating there.

Also plan: CRR-ConfigRegimeRequirements.adoc retirement (orphaned once RBSA eliminates crg_* vocabulary).

## References

- AXLA: `Tools/cmk/vov_veiled/AXLA-Lexicon.adoc`
- CRR: `lenses/CRR-ConfigRegimeRequirements.adoc` (future retirement)
- RBS: `lenses/RBS-Specification.adoc` (retiring into RBSA)
- RBAGS: `lenses/RBAGS-AdminGoogleSpec.adoc` (retiring into RBSA)
- RBSA: `lenses/RBSA-SpecTop.adoc` (new, created by ₢ASAAH)
- RBRN: `lenses/RBRN-RegimeNameplate.adoc`
- BUS: `Tools/buk/vov_veiled/BUS-BashUtilitiesSpec.adoc`

## Paces

### migrate-missing-rbs-defs-to-rbsa (₢ASAAL) [complete]

**[260208-1453] complete**

Migrate ~17 architecture and naming definitions from RBS to RBSA before RBS deletion.

## Context

Audit found these RBS anchored definitions have no corresponding anchor in RBSA. They must be migrated before RBS can be deleted.

## Definitions to migrate (into RBSA "Local Architecture Definitions" section)

Architecture terms:
- at_sentry_image — sentry image definition (strip rbrn_sentry_repo_path ref if stale)
- at_moniker — unique service identifier
- at_rbm_console — primary console definition (reword, strip cmk references)
- at_rbm_config_makefile — config makefile (reword, strip cmk references)
- at_rbm_secret — sensitive config values
- at_rbm_repo — source repository
- at_user_repo — operator's repository
- at_container_registry — remote registry
- at_startup_script — initialization scripts (reword, strip cmk references)
- consumer (rbtr_consumer) — consumer role definition
- at_stash_machine — temporary podman VM for registry ops
- at_operational_machine — primary podman VM

Naming pattern terms:
- transit_network_name — moniker-uplink pattern
- enclave_network_name — moniker-enclave pattern
- enclave_namespace_name — moniker-namespace pattern
- bottle_container_name — moniker-bottle pattern
- sentry_container_name — moniker-sentry pattern

Supporting infrastructure:
- st_dockerfile
- st_published_image
- st_image / st_image_store
- st_subnet / st_gateway

## Rules
- Strip all cmk_* vocabulary from definitions — reword in plain terms or BUK terms
- Strip crg_* references — use plain "regime" language
- Keep definitions concise — match RBSA's existing style
- Add anchors and attribute mappings to RBSA mapping section if not already present
- Do NOT modify RBS — this pace is additive to RBSA only

## Do NOT do
- Do not delete RBS (separate pace)
- Do not rewrite existing RBSA definitions
- Do not touch subdoc includes

## Acceptance
- All ~20 definitions have anchors in RBSA
- RBSA mapping section has corresponding attribute entries
- No cmk_* or crg_* vocabulary in migrated definitions

**[260208-1442] rough**

Migrate ~17 architecture and naming definitions from RBS to RBSA before RBS deletion.

## Context

Audit found these RBS anchored definitions have no corresponding anchor in RBSA. They must be migrated before RBS can be deleted.

## Definitions to migrate (into RBSA "Local Architecture Definitions" section)

Architecture terms:
- at_sentry_image — sentry image definition (strip rbrn_sentry_repo_path ref if stale)
- at_moniker — unique service identifier
- at_rbm_console — primary console definition (reword, strip cmk references)
- at_rbm_config_makefile — config makefile (reword, strip cmk references)
- at_rbm_secret — sensitive config values
- at_rbm_repo — source repository
- at_user_repo — operator's repository
- at_container_registry — remote registry
- at_startup_script — initialization scripts (reword, strip cmk references)
- consumer (rbtr_consumer) — consumer role definition
- at_stash_machine — temporary podman VM for registry ops
- at_operational_machine — primary podman VM

Naming pattern terms:
- transit_network_name — moniker-uplink pattern
- enclave_network_name — moniker-enclave pattern
- enclave_namespace_name — moniker-namespace pattern
- bottle_container_name — moniker-bottle pattern
- sentry_container_name — moniker-sentry pattern

Supporting infrastructure:
- st_dockerfile
- st_published_image
- st_image / st_image_store
- st_subnet / st_gateway

## Rules
- Strip all cmk_* vocabulary from definitions — reword in plain terms or BUK terms
- Strip crg_* references — use plain "regime" language
- Keep definitions concise — match RBSA's existing style
- Add anchors and attribute mappings to RBSA mapping section if not already present
- Do NOT modify RBS — this pace is additive to RBSA only

## Do NOT do
- Do not delete RBS (separate pace)
- Do not rewrite existing RBSA definitions
- Do not touch subdoc includes

## Acceptance
- All ~20 definitions have anchors in RBSA
- RBSA mapping section has corresponding attribute entries
- No cmk_* or crg_* vocabulary in migrated definitions

### decide-google-type-voicing-pattern (₢ASAAG) [complete]

**[260207-0822] complete**

Determine voicing pattern for Google-specific types in regime variables.

## Context
RBRV uses `rbst_*` types that subspecialize AXLA universal types (`axtu_*`). But RBRR/RBRA/RBRP regime variables reference Google-specific types (`axtg_project_id`, `axtg_region`, `axtg_service_account`, etc.).

## Question to resolve
Should regime variables with Google types:
- (A) Voice `axtg_*` directly — they're already specific enough
- (B) Voice `rbst_*` subspecializations — e.g., `rbst_depot_project_id` voices `axtg_project_id`

## Considerations
- `axtg_*` types ARE already subspecializations of universal types
- Adding `rbst_*` layer may be unnecessary indirection
- BUT: `rbst_*` allows RB-specific constraints (naming conventions, validation rules)
- Consistency with RBRV pattern vs pragmatic simplicity

## Acceptance criteria
- Decision documented in ₣AS paddock
- Pattern guidance for regime specs using Google types
- Update RBAGS voicings if needed

**[260131-2309] rough**

Determine voicing pattern for Google-specific types in regime variables.

## Context
RBRV uses `rbst_*` types that subspecialize AXLA universal types (`axtu_*`). But RBRR/RBRA/RBRP regime variables reference Google-specific types (`axtg_project_id`, `axtg_region`, `axtg_service_account`, etc.).

## Question to resolve
Should regime variables with Google types:
- (A) Voice `axtg_*` directly — they're already specific enough
- (B) Voice `rbst_*` subspecializations — e.g., `rbst_depot_project_id` voices `axtg_project_id`

## Considerations
- `axtg_*` types ARE already subspecializations of universal types
- Adding `rbst_*` layer may be unnecessary indirection
- BUT: `rbst_*` allows RB-specific constraints (naming conventions, validation rules)
- Consistency with RBRV pattern vs pragmatic simplicity

## Acceptance criteria
- Decision documented in ₣AS paddock
- Pattern guidance for regime specs using Google types
- Update RBAGS voicings if needed

### plan-rbsa-consolidation (₢ASAAD) [complete]

**[260207-0909] complete**

Plan eventual Recipe Bottle System Architecture (RBSA) consolidation.

RBAGS and RBS will eventually merge into RBSA. This pace establishes the integration strategy without implementing it yet.

Before starting: Think through:
- How do RBAGS admin/GCP sections relate to RBS system sections?
- Which RBAGS content is universal vs Google-specific?
- How to preserve RBAGS's successful partition model in merged RBSA?
- Naming discipline for RBSA subfiles?
- Relationship between AXLA patterns and RBSA implementation?

Known stale content to eliminate:

RBS-Specification.adoc lines 157-160 have legacy variable mappings:
  :rbrn_sentry_moniker:    <<term_rbrn_sentry_moniker,RBRN_SENTRY_MONIKER>>
  :rbrn_bottle_moniker:    ...
  :rbrn_sentry_image_tag:  ...
  :rbrn_bottle_image_tag:  ...

These _image_tag concepts are superseded by the ARK/CONSECRATION pattern established in RBAGS. The consolidation should eliminate these legacy mappings in favor of the canonical RBAGS vocabulary.

Outcome: Design document outlining RBSA structure, section organization, term consolidation, and implementation sequence.

To be detailed during pace execution.

**[260201-2025] rough**

Plan eventual Recipe Bottle System Architecture (RBSA) consolidation.

RBAGS and RBS will eventually merge into RBSA. This pace establishes the integration strategy without implementing it yet.

Before starting: Think through:
- How do RBAGS admin/GCP sections relate to RBS system sections?
- Which RBAGS content is universal vs Google-specific?
- How to preserve RBAGS's successful partition model in merged RBSA?
- Naming discipline for RBSA subfiles?
- Relationship between AXLA patterns and RBSA implementation?

Known stale content to eliminate:

RBS-Specification.adoc lines 157-160 have legacy variable mappings:
  :rbrn_sentry_moniker:    <<term_rbrn_sentry_moniker,RBRN_SENTRY_MONIKER>>
  :rbrn_bottle_moniker:    ...
  :rbrn_sentry_image_tag:  ...
  :rbrn_bottle_image_tag:  ...

These _image_tag concepts are superseded by the ARK/CONSECRATION pattern established in RBAGS. The consolidation should eliminate these legacy mappings in favor of the canonical RBAGS vocabulary.

Outcome: Design document outlining RBSA structure, section organization, term consolidation, and implementation sequence.

To be detailed during pace execution.

**[260131-1152] rough**

Plan eventual Recipe Bottle System Architecture (RBSA) consolidation.

RBAGS and RBS will eventually merge into RBSA. This pace establishes the integration strategy without implementing it yet.

Before starting: Think through:
- How do RBAGS admin/GCP sections relate to RBS system sections?
- Which RBAGS content is universal vs Google-specific?
- How to preserve RBAGS's successful partition model in merged RBSA?
- Naming discipline for RBSA subfiles?
- Relationship between AXLA patterns and RBSA implementation?

Outcome: Design document outlining RBSA structure, section organization, term consolidation, and implementation sequence.

To be detailed during pace execution.

### create-rbsa-skeleton (₢ASAAH) [complete]

**[260208-1134] complete**

Create RBSA-SpecTop.adoc with unified mapping section and three-tier section structure.

## Status note

Commit c7908709 (affiliated with this pace) performed "git mv RBAGS to RBSA, restructure three-tier, merge RBS content" in a previous session. The bulk of this work may already be done. Action for next session: review RBSA-SpecTop.adoc against these acceptance criteria and either wrap if complete or identify remaining gaps.

## Context (from ASAAD design)

RBSA is a single document absorbing all RBAGS + RBS content. No two-level includes — RBSA is the parent, existing RBSXX subdocs are the only includes. "A" suffix indicates top spec document.

## Three-tier structure

### Tier 1: Why and What (significance-first)
- System overview: the trust challenge, the bottle pattern (from RBS)
- Security architecture: sentry/censer/enclave model, security properties (from RBS)
- Getting Started (existing include from RBAGS)

### Tier 2: How (temporal order)
- Payor Operations (existing RBAGS includes)
- Governor Operations (existing RBAGS includes)
- Director Operations (existing RBAGS includes)
- Retriever Operations (existing RBAGS includes)
- Multi Role Operations (existing RBAGS includes)
- Local Operations placeholder (bottle lifecycle — filled by ASAAB)

### Tier 3: Reference
- Configuration Regimes (unified — filled by ASAAC)
- Type Voicings (rbst_* universal, rbgt_* Google-specific)
- Control Voicings (rbbc_, rbhg_)
- Orchestration Patterns (rbtoe_*)
- Script Internals (iptables, DNS, socat, eBPF — from RBS)
- Term Definitions (consolidated from both docs)
- Trade Studies

## Work required

1. Create lenses/RBSA-SpecTop.adoc
2. Build unified mapping section — merge RBAGS and RBS mappings, resolve divergent anchors (e.g., RBAGS uses [[at_bottle_image]] while RBS uses [[bottle_image]])
3. Move Tier 1 content from RBS (security architecture sections)
4. Set up Tier 2 section headers with existing RBAGS include:: directives
5. Set up Tier 3 section headers with content from both docs
6. Introduce rbgt_* type voicings alongside rbst_* in Type Voicings section:
   - rbgt_project_id voices axtg_project_id
   - rbgt_region voices axtg_region
   - rbgt_service_account voices axtg_service_account
   - rbgt_billing_account voices axtg_billing_account
7. Verify all existing include:: paths still resolve

## Acceptance criteria
- RBSA-SpecTop.adoc exists with all three tiers
- Unified mapping section with no duplicate anchors
- All existing include:: subdocs referenced
- Tier 1 has security architecture content from RBS
- rbgt_* types defined in Type Voicings section
- Document renders without broken cross-references

**[260207-1407] rough**

Create RBSA-SpecTop.adoc with unified mapping section and three-tier section structure.

## Status note

Commit c7908709 (affiliated with this pace) performed "git mv RBAGS to RBSA, restructure three-tier, merge RBS content" in a previous session. The bulk of this work may already be done. Action for next session: review RBSA-SpecTop.adoc against these acceptance criteria and either wrap if complete or identify remaining gaps.

## Context (from ASAAD design)

RBSA is a single document absorbing all RBAGS + RBS content. No two-level includes — RBSA is the parent, existing RBSXX subdocs are the only includes. "A" suffix indicates top spec document.

## Three-tier structure

### Tier 1: Why and What (significance-first)
- System overview: the trust challenge, the bottle pattern (from RBS)
- Security architecture: sentry/censer/enclave model, security properties (from RBS)
- Getting Started (existing include from RBAGS)

### Tier 2: How (temporal order)
- Payor Operations (existing RBAGS includes)
- Governor Operations (existing RBAGS includes)
- Director Operations (existing RBAGS includes)
- Retriever Operations (existing RBAGS includes)
- Multi Role Operations (existing RBAGS includes)
- Local Operations placeholder (bottle lifecycle — filled by ASAAB)

### Tier 3: Reference
- Configuration Regimes (unified — filled by ASAAC)
- Type Voicings (rbst_* universal, rbgt_* Google-specific)
- Control Voicings (rbbc_, rbhg_)
- Orchestration Patterns (rbtoe_*)
- Script Internals (iptables, DNS, socat, eBPF — from RBS)
- Term Definitions (consolidated from both docs)
- Trade Studies

## Work required

1. Create lenses/RBSA-SpecTop.adoc
2. Build unified mapping section — merge RBAGS and RBS mappings, resolve divergent anchors (e.g., RBAGS uses [[at_bottle_image]] while RBS uses [[bottle_image]])
3. Move Tier 1 content from RBS (security architecture sections)
4. Set up Tier 2 section headers with existing RBAGS include:: directives
5. Set up Tier 3 section headers with content from both docs
6. Introduce rbgt_* type voicings alongside rbst_* in Type Voicings section:
   - rbgt_project_id voices axtg_project_id
   - rbgt_region voices axtg_region
   - rbgt_service_account voices axtg_service_account
   - rbgt_billing_account voices axtg_billing_account
7. Verify all existing include:: paths still resolve

## Acceptance criteria
- RBSA-SpecTop.adoc exists with all three tiers
- Unified mapping section with no duplicate anchors
- All existing include:: subdocs referenced
- Tier 1 has security architecture content from RBS
- rbgt_* types defined in Type Voicings section
- Document renders without broken cross-references

**[260207-0907] rough**

Create RBSA-SpecTop.adoc with unified mapping section and three-tier section structure.

## Context (from ASAAD design)

RBSA is a single document absorbing all RBAGS + RBS content. No two-level includes — RBSA is the parent, existing RBSXX subdocs are the only includes. "A" suffix indicates top spec document.

## Three-tier structure

### Tier 1: Why and What (significance-first)
- System overview: the trust challenge, the bottle pattern (from RBS)
- Security architecture: sentry/censer/enclave model, security properties (from RBS)
- Getting Started (existing include from RBAGS)

### Tier 2: How (temporal order)
- Payor Operations (existing RBAGS includes)
- Governor Operations (existing RBAGS includes)
- Director Operations (existing RBAGS includes)
- Retriever Operations (existing RBAGS includes)
- Multi Role Operations (existing RBAGS includes)
- Local Operations placeholder (bottle lifecycle — filled by ASAAB)

### Tier 3: Reference
- Configuration Regimes (unified — filled by ASAAC)
- Type Voicings (rbst_* universal, rbgt_* Google-specific)
- Control Voicings (rbbc_, rbhg_)
- Orchestration Patterns (rbtoe_*)
- Script Internals (iptables, DNS, socat, eBPF — from RBS)
- Term Definitions (consolidated from both docs)
- Trade Studies

## Work required

1. Create lenses/RBSA-SpecTop.adoc
2. Build unified mapping section — merge RBAGS and RBS mappings, resolve divergent anchors (e.g., RBAGS uses [[at_bottle_image]] while RBS uses [[bottle_image]])
3. Move Tier 1 content from RBS (security architecture sections)
4. Set up Tier 2 section headers with existing RBAGS include:: directives
5. Set up Tier 3 section headers with content from both docs
6. Introduce rbgt_* type voicings alongside rbst_* in Type Voicings section:
   - rbgt_project_id voices axtg_project_id
   - rbgt_region voices axtg_region
   - rbgt_service_account voices axtg_service_account
   - rbgt_billing_account voices axtg_billing_account
7. Verify all existing include:: paths still resolve

## Acceptance criteria
- RBSA-SpecTop.adoc exists with all three tiers
- Unified mapping section with no duplicate anchors
- All existing include:: subdocs referenced
- Tier 1 has security architecture content from RBS
- rbgt_* types defined in Type Voicings section
- Document renders without broken cross-references

**[260207-0853] rough**

Create RBSA-SystemArchitecture.adoc with unified mapping section and three-tier section structure.

## Context (from ₢ASAAD design)

RBSA is a single document absorbing all RBAGS + RBS content. No two-level includes — RBSA is the parent, existing RBSXX subdocs are the only includes.

## Three-tier structure

### Tier 1: Why and What (significance-first)
- System overview: the trust challenge, the bottle pattern (from RBS)
- Security architecture: sentry/censer/enclave model, security properties (from RBS)
- Getting Started (existing include from RBAGS)

### Tier 2: How (temporal order)
- Payor Operations (existing RBAGS includes)
- Governor Operations (existing RBAGS includes)
- Director Operations (existing RBAGS includes)
- Retriever Operations (existing RBAGS includes)
- Multi Role Operations (existing RBAGS includes)
- Local Operations placeholder (bottle lifecycle — filled by ₢ASAAB)

### Tier 3: Reference
- Configuration Regimes (unified — filled by ₢ASAAC)
- Type Voicings
- Control Voicings (rbbc_, rbhg_)
- Orchestration Patterns (rbtoe_*)
- Script Internals (iptables, DNS, socat, eBPF — from RBS)
- Term Definitions (consolidated from both docs)
- Trade Studies

## Work required

1. Create RBSA-SystemArchitecture.adoc
2. Build unified mapping section — merge RBAGS and RBS mappings, resolve divergent anchors (e.g., at_bottle_image -> bottle_image vs at_bottle_image)
3. Move Tier 1 content from RBS (security architecture sections)
4. Set up Tier 2 section headers with existing RBAGS include:: directives
5. Set up Tier 3 section headers with content from both docs
6. Verify all existing include:: paths still resolve

## Acceptance criteria
- RBSA file exists with all three tiers
- Unified mapping section with no duplicate anchors
- All existing include:: subdocs referenced
- Tier 1 has security architecture content from RBS
- Document renders without broken cross-references

### assess-rbs-to-rbsa-loss (₢ASAAJ) [complete]

**[260208-1248] complete**

Compare old RBS-Specification.adoc against RBSA-SpecTop.adoc to verify no content was lost in the consolidation.

## Work required

1. Read RBS-Specification.adoc (the pre-consolidation version — use git history if needed)
2. Read RBSA-SpecTop.adoc (the new unified document)
3. Section-by-section comparison:
   - Tier 1 (Why and What): Security architecture, trust challenge, bottle pattern, security properties
   - Tier 3 (Reference): Script internals (iptables, DNS/dnsmasq, socat, eBPF), trade studies, term definitions
   - Local operations content that was in RBS
4. Flag any content present in RBS but missing from RBSA
5. Flag any content that was significantly altered (not just reformatted) in ways that change meaning

## Acceptance criteria
- Every substantive section of RBS accounted for in RBSA (or explicitly noted as intentionally dropped)
- No semantic loss — reformatting is fine, meaning changes are not
- Report produced listing: confirmed-present, intentionally-dropped, and missing items

**[260207-1404] bridled**

Compare old RBS-Specification.adoc against RBSA-SpecTop.adoc to verify no content was lost in the consolidation.

## Work required

1. Read RBS-Specification.adoc (the pre-consolidation version — use git history if needed)
2. Read RBSA-SpecTop.adoc (the new unified document)
3. Section-by-section comparison:
   - Tier 1 (Why and What): Security architecture, trust challenge, bottle pattern, security properties
   - Tier 3 (Reference): Script internals (iptables, DNS/dnsmasq, socat, eBPF), trade studies, term definitions
   - Local operations content that was in RBS
4. Flag any content present in RBS but missing from RBSA
5. Flag any content that was significantly altered (not just reformatted) in ways that change meaning

## Acceptance criteria
- Every substantive section of RBS accounted for in RBSA (or explicitly noted as intentionally dropped)
- No semantic loss — reformatting is fine, meaning changes are not
- Report produced listing: confirmed-present, intentionally-dropped, and missing items

*Direction:* Agent: opus | Cardinality: 1 sequential | Files: lenses/RBS-Specification.adoc via git show pre-ASAAH, lenses/RBSA-SpecTop.adoc (2 files) | Steps: 1. Retrieve pre-consolidation RBS-Specification.adoc from git history before commit c7908709 2. Read current RBSA-SpecTop.adoc 3. Section-by-section comparison covering Tier 1 security architecture, Tier 3 script internals and trade studies, local ops content, term definitions 4. Produce loss report listing confirmed-present, intentionally-dropped, and missing items | Verify: none - research only

**[260207-1403] rough**

Compare old RBS-Specification.adoc against RBSA-SpecTop.adoc to verify no content was lost in the consolidation.

## Work required

1. Read RBS-Specification.adoc (the pre-consolidation version — use git history if needed)
2. Read RBSA-SpecTop.adoc (the new unified document)
3. Section-by-section comparison:
   - Tier 1 (Why and What): Security architecture, trust challenge, bottle pattern, security properties
   - Tier 3 (Reference): Script internals (iptables, DNS/dnsmasq, socat, eBPF), trade studies, term definitions
   - Local operations content that was in RBS
4. Flag any content present in RBS but missing from RBSA
5. Flag any content that was significantly altered (not just reformatted) in ways that change meaning

## Acceptance criteria
- Every substantive section of RBS accounted for in RBSA (or explicitly noted as intentionally dropped)
- No semantic loss — reformatting is fine, meaning changes are not
- Report produced listing: confirmed-present, intentionally-dropped, and missing items

### assess-rbags-to-rbsa-loss (₢ASAAK) [complete]

**[260208-1257] complete**

Compare old RBAGS-AdminGoogleSpec.adoc against RBSA-SpecTop.adoc to verify no content was lost in the consolidation.

## Work required

1. Read RBAGS-AdminGoogleSpec.adoc (the pre-consolidation version — use git history if needed)
2. Read RBSA-SpecTop.adoc (the new unified document)
3. Section-by-section comparison:
   - Tier 2 (How): All cloud operations by role — Payor, Governor, Director, Retriever, Multi Role
   - Tier 3 (Reference): Term definitions, configuration patterns, regime variables
   - Mapping section: attribute references, anchors, linked terms
   - Getting Started content
4. Flag any content present in RBAGS but missing from RBSA
5. Flag any content that was significantly altered (not just reformatted) in ways that change meaning

## Acceptance criteria
- Every substantive section of RBAGS accounted for in RBSA (or explicitly noted as intentionally dropped)
- No semantic loss — reformatting is fine, meaning changes are not
- Report produced listing: confirmed-present, intentionally-dropped, and missing items

**[260207-1404] bridled**

Compare old RBAGS-AdminGoogleSpec.adoc against RBSA-SpecTop.adoc to verify no content was lost in the consolidation.

## Work required

1. Read RBAGS-AdminGoogleSpec.adoc (the pre-consolidation version — use git history if needed)
2. Read RBSA-SpecTop.adoc (the new unified document)
3. Section-by-section comparison:
   - Tier 2 (How): All cloud operations by role — Payor, Governor, Director, Retriever, Multi Role
   - Tier 3 (Reference): Term definitions, configuration patterns, regime variables
   - Mapping section: attribute references, anchors, linked terms
   - Getting Started content
4. Flag any content present in RBAGS but missing from RBSA
5. Flag any content that was significantly altered (not just reformatted) in ways that change meaning

## Acceptance criteria
- Every substantive section of RBAGS accounted for in RBSA (or explicitly noted as intentionally dropped)
- No semantic loss — reformatting is fine, meaning changes are not
- Report produced listing: confirmed-present, intentionally-dropped, and missing items

*Direction:* Agent: opus | Cardinality: 1 sequential | Files: lenses/RBAGS-AdminGoogleSpec.adoc via git show pre-ASAAH, lenses/RBSA-SpecTop.adoc (2 files) | Steps: 1. Retrieve pre-consolidation RBAGS-AdminGoogleSpec.adoc from git history before commit c7908709 2. Read current RBSA-SpecTop.adoc 3. Section-by-section comparison covering Tier 2 cloud operations by role, Getting Started, mapping section attribute references and anchors, term definitions, configuration patterns 4. Produce loss report listing confirmed-present, intentionally-dropped, and missing items | Verify: none - research only

**[260207-1403] rough**

Compare old RBAGS-AdminGoogleSpec.adoc against RBSA-SpecTop.adoc to verify no content was lost in the consolidation.

## Work required

1. Read RBAGS-AdminGoogleSpec.adoc (the pre-consolidation version — use git history if needed)
2. Read RBSA-SpecTop.adoc (the new unified document)
3. Section-by-section comparison:
   - Tier 2 (How): All cloud operations by role — Payor, Governor, Director, Retriever, Multi Role
   - Tier 3 (Reference): Term definitions, configuration patterns, regime variables
   - Mapping section: attribute references, anchors, linked terms
   - Getting Started content
4. Flag any content present in RBAGS but missing from RBSA
5. Flag any content that was significantly altered (not just reformatted) in ways that change meaning

## Acceptance criteria
- Every substantive section of RBAGS accounted for in RBSA (or explicitly noted as intentionally dropped)
- No semantic loss — reformatting is fine, meaning changes are not
- Report produced listing: confirmed-present, intentionally-dropped, and missing items

### express-local-ops-axla-voicings (₢ASAAB) [complete]

**[260208-1526] complete**

Add AXLA annotations and rbbc_* control voicings to local operations in RBSA.

## Context

Prior paces (₢ASAAH, ₢ASAAL) completed the structural work:
- All 16 RBSXX subdocs exist with content migrated from RBS
- RBSA parent has anchors, headings, and include:: directives in place
- Tier 2 operations (lines 698-794) and Tier 3 script internals (lines 2493-2533) are already bifurcated

What remains is voicing conversion — making the content match RBAGS quality.

## Work required

1. **Add AXLA annotations to local ops anchors in RBSA-SpecTop.adoc**
   - Each `[[anchor]]` needs `// ⟦axl_voices axo_command axe_bash_interactive⟧` on the line after it
   - Applies to: opss_sentry_start, mkr_network_create, mkr_sentry_run, mkr_network_connect, opbs_bottle_start, mkr_bottle_cleanup, mkr_bottle_launch, opbr_bottle_run, mkr_bottle_create, mkr_command_exec, ops_rbv_check, ops_rbv_mirror
   - scr_* anchors in Tier 3 may need different annotations (script internals, not interactive commands)

2. **Convert 11 Tier 2 subdoc bodies to rbbc_* procedure format**
   - Follow RBSDC-depot_create.adoc as the exemplar pattern
   - Use rbbc_* control voicings: {rbbc_require}, {rbbc_call}, {rbbc_store}, {rbbc_fatal}, {rbbc_show}, etc.
   - Express steps as ordered sequences with control flow
   - Files: RBSSS, RBSNC, RBSNX, RBSBS, RBSBK, RBSBL, RBSBR, RBSBC, RBSCE, RBSVC, RBSVM

3. **Purge dead cmk_* vocabulary**
   - Remove references to {at_rbm_console}, {cmk_recipe_line_s}, {cmk_script_line_s}, {cmk_external_rule} from subdocs and RBSA inline text
   - Replace with appropriate AXLA or neutral phrasing

4. **Verify Tier 3 subdocs are clean**
   - RBSSC, RBSIP, RBSPT, RBSAX, RBSDS — confirm no cmk_* remains
   - These already have detailed content; no rbbc_* conversion needed (they're reference, not procedures)

## Parallelization

All subdocs are independent files. RBSA parent edits touch different lines from subdoc includes. Natural parallel streams:
- Stream A (haiku): RBSA annotations + cmk_* purge in parent
- Stream B (sonnet): Sentry + sessile subdocs (RBSSS, RBSNC, RBSNX, RBSBS, RBSBK, RBSBL)
- Stream C (sonnet): Agile + VM subdocs (RBSBR, RBSBC, RBSCE, RBSVC, RBSVM)
- Stream D (haiku): Verify Tier 3 subdocs clean

## Acceptance criteria
- All Tier 2 local ops anchors have // ⟦axl_voices ...⟧ annotations
- All Tier 2 subdocs use rbbc_* control voicings following RBSDC pattern
- No cmk_* vocabulary remains anywhere in local ops content
- Script internals in Tier 3 remain as reference (not converted to rbbc_*)

**[260208-1511] rough**

Add AXLA annotations and rbbc_* control voicings to local operations in RBSA.

## Context

Prior paces (₢ASAAH, ₢ASAAL) completed the structural work:
- All 16 RBSXX subdocs exist with content migrated from RBS
- RBSA parent has anchors, headings, and include:: directives in place
- Tier 2 operations (lines 698-794) and Tier 3 script internals (lines 2493-2533) are already bifurcated

What remains is voicing conversion — making the content match RBAGS quality.

## Work required

1. **Add AXLA annotations to local ops anchors in RBSA-SpecTop.adoc**
   - Each `[[anchor]]` needs `// ⟦axl_voices axo_command axe_bash_interactive⟧` on the line after it
   - Applies to: opss_sentry_start, mkr_network_create, mkr_sentry_run, mkr_network_connect, opbs_bottle_start, mkr_bottle_cleanup, mkr_bottle_launch, opbr_bottle_run, mkr_bottle_create, mkr_command_exec, ops_rbv_check, ops_rbv_mirror
   - scr_* anchors in Tier 3 may need different annotations (script internals, not interactive commands)

2. **Convert 11 Tier 2 subdoc bodies to rbbc_* procedure format**
   - Follow RBSDC-depot_create.adoc as the exemplar pattern
   - Use rbbc_* control voicings: {rbbc_require}, {rbbc_call}, {rbbc_store}, {rbbc_fatal}, {rbbc_show}, etc.
   - Express steps as ordered sequences with control flow
   - Files: RBSSS, RBSNC, RBSNX, RBSBS, RBSBK, RBSBL, RBSBR, RBSBC, RBSCE, RBSVC, RBSVM

3. **Purge dead cmk_* vocabulary**
   - Remove references to {at_rbm_console}, {cmk_recipe_line_s}, {cmk_script_line_s}, {cmk_external_rule} from subdocs and RBSA inline text
   - Replace with appropriate AXLA or neutral phrasing

4. **Verify Tier 3 subdocs are clean**
   - RBSSC, RBSIP, RBSPT, RBSAX, RBSDS — confirm no cmk_* remains
   - These already have detailed content; no rbbc_* conversion needed (they're reference, not procedures)

## Parallelization

All subdocs are independent files. RBSA parent edits touch different lines from subdoc includes. Natural parallel streams:
- Stream A (haiku): RBSA annotations + cmk_* purge in parent
- Stream B (sonnet): Sentry + sessile subdocs (RBSSS, RBSNC, RBSNX, RBSBS, RBSBK, RBSBL)
- Stream C (sonnet): Agile + VM subdocs (RBSBR, RBSBC, RBSCE, RBSVC, RBSVM)
- Stream D (haiku): Verify Tier 3 subdocs clean

## Acceptance criteria
- All Tier 2 local ops anchors have // ⟦axl_voices ...⟧ annotations
- All Tier 2 subdocs use rbbc_* control voicings following RBSDC pattern
- No cmk_* vocabulary remains anywhere in local ops content
- Script internals in Tier 3 remain as reference (not converted to rbbc_*)

**[260207-0851] rough**

Express bottle lifecycle operations (sentry start, bottle start/stop, security configuration) as RBAGS-style procedures within RBSA.

## Context (from ₢ASAAD design)

RBSA consolidates RBAGS + RBS into a single document. Tier 2 (Operations) organizes all procedures — cloud and local — as peers. Local operations must follow the same RBAGS pattern:

```asciidoc
[[operation_anchor]]
// ⟦axl_voices axo_command axe_bash_interactive⟧
=== {operation_name}

include::RBSXX-subdoc.adoc[]
```

## Work required

1. Identify RBS procedure sections to convert:
   - Sentry Start (network create, sentry run, network connect, security config)
   - Sessile Bottle Start (cleanup, launch)
   - Agile Bottle Run (create, exec)
   - Bottle Stop (not yet specified in RBS but needed)

2. For each procedure, create an RBSXX subdoc following RBAGS naming:
   - Use rbbc_* control voicings (CALL, REQUIRE, FATAL, SHOW, etc.)
   - Document steps as ordered sequences with control flow
   - Replace cmk_* vocabulary entirely (cmk is dead)

3. Bifurcate operations vs script internals:
   - Operations in Tier 2: describe WHAT happens at each step
   - Script internals in Reference: iptables chain setup, NAT rules, dnsmasq config, socat proxy, eBPF attachment detail

## Acceptance criteria
- Each local operation has its own RBSXX subdoc
- All operations use axo_command + axe_bash_interactive annotations
- No cmk_* vocabulary remains in converted content
- Script internals are in reference section, not inline

**[260131-1149] rough**

Partition RBS procedural specifications into AXLA-consistent subfiles.

Move ₢APAAV from ₣AP. RBS currently monolithic; should follow RBAGS precedent of partitioned subfiles with AXLA voicings.

Identify RBS procedure sections (Sentry Start, Bottle Start, Bottle Run, etc.) and extract into standalone subfiles.

Each subfile:
- Follows RBAGS naming pattern (RBSXY-Description.adoc)
- Includes AXLA-consistent voicings: // ⟦axl_voices axo_procedure axd_transient⟧
- Documents sections: axs_inputs, axs_behavior, axs_outputs, axs_completion
- Maintains term mappings compatible with parent RBS document

Establish naming convention for RBS subfiles before implementation.

### await-retirement-heat-at (₢ASAAC) [complete]

**[260216-0732] complete**

BLOCKED — awaiting completion of ₣AT (rbw-regime-consolidation).

₣AT was spun out to handle the full regime consolidation as a dedicated heat. This pace's original scope (unify regime definitions into RBSA Tier 3, resolve CRR/AXLA vocabulary split) is entirely covered by ₣AT paces:

- ₢ATAAA study-all-recipe-bottle-regimes — regime inventory
- ₢ATAAC expand-busa-regime-vocabulary — BUSA regime vocab
- ₢ATAAF audit-rbrr-legacy-variables — RBRR cleanup

Once ₣AT completes, revisit whether any residual RBSA Tier 3 integration work remains, or drop this pace entirely.

**[260209-0634] rough**

BLOCKED — awaiting completion of ₣AT (rbw-regime-consolidation).

₣AT was spun out to handle the full regime consolidation as a dedicated heat. This pace's original scope (unify regime definitions into RBSA Tier 3, resolve CRR/AXLA vocabulary split) is entirely covered by ₣AT paces:

- ₢ATAAA study-all-recipe-bottle-regimes — regime inventory
- ₢ATAAC expand-busa-regime-vocabulary — BUSA regime vocab
- ₢ATAAF audit-rbrr-legacy-variables — RBRR cleanup

Once ₣AT completes, revisit whether any residual RBSA Tier 3 integration work remains, or drop this pace entirely.

**[260209-0632] rough**

BLOCKED — awaiting completion of ₣AT (rbw-regime-consolidation).

₣AT was spun out to handle the full regime consolidation as a dedicated heat. This pace's original scope (unify regime definitions into RBSA Tier 3, resolve CRR/AXLA vocabulary split) is entirely covered by ₣AT paces:

- ₢ATAAA study-all-recipe-bottle-regimes — regime inventory
- ₢ATAAC expand-busa-regime-vocabulary — BUSA regime vocab
- ₢ATAAF audit-rbrr-legacy-variables — RBRR cleanup

Once ₣AT completes, revisit whether any residual RBSA Tier 3 integration work remains, or drop this pace entirely.

**[260208-1544] rough**

BLOCKED — awaiting completion of ₣AT (rbw-regime-consolidation).

₣AT was spun out to handle the full regime consolidation as a dedicated heat. This pace's original scope (unify regime definitions into RBSA Tier 3, resolve CRR/AXLA vocabulary split) is entirely covered by ₣AT paces:

- ₢ATAAA study-all-recipe-bottle-regimes — regime inventory
- ₢ATAAC expand-busa-regime-vocabulary — BUSA regime vocab
- ₢ATAAF audit-rbrr-legacy-variables — RBRR cleanup

Once ₣AT completes, revisit whether any residual RBSA Tier 3 integration work remains, or drop this pace entirely.

**[260208-1544] rough**

BLOCKED — awaiting completion of ₣AT (rbw-regime-consolidation).

₣AT was spun out to handle the full regime consolidation as a dedicated heat. This pace's original scope (unify regime definitions into RBSA Tier 3, resolve CRR/AXLA vocabulary split) is entirely covered by ₣AT paces:

- ₢ATAAA study-all-recipe-bottle-regimes — regime inventory
- ₢ATAAC expand-busa-regime-vocabulary — BUSA regime vocab
- ₢ATAAF audit-rbrr-legacy-variables — RBRR cleanup

Once ₣AT completes, revisit whether any residual RBSA Tier 3 integration work remains, or drop this pace entirely.

**[260207-0907] rough**

Unify all regime definitions into RBSA's Tier 3 (Reference), resolving the CRR/AXLA vocabulary split.

## Context (from ASAAD design)

RBSA absorbs both RBAGS and RBS. RBAGS defines regimes with AXLA annotations (axvr_*, axhr*_). RBS defines regimes using CRR vocabulary (crg_*). RBAGS model governs.

Currently:
- RBAGS has: RBRR, RBRA, RBRP, RBRO, RBRV (with include), RBRN (with include), RBEV — all using AXLA annotations
- RBS has: Base Regime (RBB_*), Station Regime (RBS_*), Nameplate Regime (RBRN_*) — using CRR vocabulary
- RBS defines RBRR variables (dns_server, nameplate_path, registry_*, build_architectures, etc.) that RBAGS's RBRR section does not cover

## Type voicing prefix decision (from ASAAD)

- rbst_* — universal types voicing axtu_* motifs (existing)
- rbgt_* — Google-specific types voicing axtg_* motifs (new, introduced in ASAAH)

Apply rbgt_* for Google-typed regime variables. Apply rbst_* for universal-typed variables. No double-prefixing (rbst_gcp_* is rejected).

## Work required

1. Identify which RBS regime content is unique vs duplicated:
   - RBS RBRN definitions duplicate RBAGS RBRN (RBAGS is authoritative) — eliminate RBS copies
   - RBS RBRR variables (lines 138-153: dns_server, registry_*, vm image, crane) are Makefile-era variables NOT in RBAGS — assess which survive
   - RBS Base/Station regimes (RBB_*, RBS_*) may need new RBSA-era equivalents

2. For surviving regime content, convert to AXLA annotation style:
   - Replace crg_* vocabulary with axvr_*/axhr*_ annotations
   - Apply rbst_* or rbgt_* type voicings as appropriate

3. Eliminate fully superseded content:
   - Stale _image_tag and _moniker mappings (RBS lines 157-160)
   - CRR-specific vocabulary from regime sections

## Acceptance criteria
- Single coherent regime section in RBSA Tier 3
- All regime definitions use AXLA annotations
- Google-typed variables use rbgt_* voicings
- No crg_* vocabulary in regime content
- Stale _image_tag/_moniker mappings eliminated

**[260207-0851] rough**

Unify all regime definitions into RBSA's Tier 3 (Reference), resolving the CRR/AXLA vocabulary split.

## Context (from ₢ASAAD design)

RBSA absorbs both RBAGS and RBS. RBAGS defines regimes with AXLA annotations (axvr_*, axhr*_). RBS defines regimes using CRR vocabulary (crg_*). RBAGS model governs.

Currently:
- RBAGS has: RBRR, RBRA, RBRP, RBRO, RBRV (with include), RBRN (with include), RBEV — all using AXLA annotations
- RBS has: Base Regime (RBB_*), Station Regime (RBS_*), Nameplate Regime (RBRN_*) — using CRR vocabulary
- RBS defines RBRR variables (dns_server, nameplate_path, registry_*, build_architectures, etc.) that RBAGS's RBRR section doesn't cover

## Work required

1. Identify which RBS regime content is unique vs duplicated:
   - RBS RBRN definitions duplicate RBAGS RBRN (RBAGS is authoritative) — eliminate RBS copies
   - RBS RBRR variables (lines 138-153: dns_server, registry_*, vm image, crane) are Makefile-era variables NOT in RBAGS — assess which survive
   - RBS Base/Station regimes (RBB_*, RBS_*) may need new RBSA-era equivalents

2. For surviving regime content, convert to AXLA annotation style:
   - Replace crg_* vocabulary with axvr_*/axhr*_ annotations
   - Apply rbst_* type voicings per ₢ASAAG decision

3. Eliminate fully superseded content:
   - stale _image_tag and _moniker mappings (RBS lines 157-160)
   - CRR-specific vocabulary from regime sections

## Acceptance criteria
- Single coherent regime section in RBSA Tier 3
- All regime definitions use AXLA annotations
- No crg_* vocabulary in regime content
- Stale _image_tag/_moniker mappings eliminated

**[260131-2309] rough**

Fold RBRN (Nameplate configuration regime) into RBS as subsection.

## Context
RBRN is currently separate but is RBS's regime definition—RBS references its variables throughout (rbrn_moniker, rbrn_entry_enabled, etc.). Keeping separate creates cross-reference friction.

## Pattern to follow (from ₣AR RBRV exemplar)

Apply the type voicing and subdocument patterns established for RBRV:

1. **Upgrade RBRN types**: Replace old `crg_*` types with `rbst_*` type voicings
2. **Parent owns anchors**: RBS defines `[[rbrn_*]]` anchors with gestalt definitions
3. **Detail section structure**: Tables with `rbst_*` types, NOTE blocks for applicability
4. **No redundant definitions**: Remove "Core Term Definitions" if present — parent owns these

## Work required
- Integrate RBRN as subsection within RBS structure
- Add `rbst_*` mappings to RBS for any new types needed
- Update RBRN variable voicings to use `rbst_*` types
- Remove RBRN.adoc from lenses/ (or keep as archived reference)

## Acceptance criteria
- RBRN content merged into RBS with proper structure
- All RBRN variables use `rbst_*` type voicings
- No duplicate anchor definitions
- RBS mappings include all RBRN terms

**[260131-1149] rough**

Fold RBRN (Nameplate configuration regime) into RBS as subsection.

RBRN is currently separate but is RBS's regime definition—RBS references its variables throughout (rbrn_moniker, rbrn_entry_enabled, etc.). Keeping separate creates cross-reference friction.

Integrate RBRN as subsection within RBS structure:
- Service Configuration Regime section in RBS main document
- Incorporates RBRN's variable definitions, constraints, term glossary
- Maintains AXLA voicing pattern with axrg_regime and axrg_variable motifs
- Update RBS mappings to include RBRN terms
- Remove RBRN.adoc from lenses/ (or keep as archived reference)

Result: Complete service specification in one coherent document.

### move-regime-vars-to-detail-subsection (₢ASAAF) [complete]

**[260216-0742] complete**

Move all regime variable detail from anchor/definition blocks to detail subdocuments within RBSA.

## Context (from ₢ASAAD design)

RBSA Tier 3 (Reference) houses all regime definitions. Each regime's anchor definitions in the parent should be 1-sentence gestalt descriptions. Detailed operational documentation (usage context, typical values, gate explanations, interactions) belongs in the regime's detail subdocument.

This follows the pattern established in ₣AR (RBRV exemplar) and reinforced by ₢ASAAG:
- Parent (RBSA): [[anchor]] with terse type reference — the "what"
- Subdoc (RBSRV, RBRN, etc.): Operational narrative — the "how"
- include:: connects them under the same heading

## Work required

1. For each regime with detail subdocs (RBRV, RBRN, and any new ones from ASAAC):
   - Audit anchor definitions in parent for verbosity
   - Move operational detail to subdoc
   - Keep parent anchors to 1-sentence definitions

2. For regimes without subdocs yet (RBRR, RBRA, RBRP, RBRO, RBEV):
   - Assess whether they need subdocs (based on definition length)
   - Create subdocs where warranted

## Acceptance criteria
- All parent anchor definitions are 1-sentence gestalt
- Operational detail lives in subdocs
- include:: pattern connects parent to subdocs

**[260207-0853] rough**

Move all regime variable detail from anchor/definition blocks to detail subdocuments within RBSA.

## Context (from ₢ASAAD design)

RBSA Tier 3 (Reference) houses all regime definitions. Each regime's anchor definitions in the parent should be 1-sentence gestalt descriptions. Detailed operational documentation (usage context, typical values, gate explanations, interactions) belongs in the regime's detail subdocument.

This follows the pattern established in ₣AR (RBRV exemplar) and reinforced by ₢ASAAG:
- Parent (RBSA): [[anchor]] with terse type reference — the "what"
- Subdoc (RBSRV, RBRN, etc.): Operational narrative — the "how"
- include:: connects them under the same heading

## Work required

1. For each regime with detail subdocs (RBRV, RBRN, and any new ones from ASAAC):
   - Audit anchor definitions in parent for verbosity
   - Move operational detail to subdoc
   - Keep parent anchors to 1-sentence definitions

2. For regimes without subdocs yet (RBRR, RBRA, RBRP, RBRO, RBEV):
   - Assess whether they need subdocs (based on definition length)
   - Create subdocs where warranted

## Acceptance criteria
- All parent anchor definitions are 1-sentence gestalt
- Operational detail lives in subdocs
- include:: pattern connects parent to subdocs

**[260131-1215] rough**

Move all regime variable detail, except for 1-sentence anchor descriptions, from anchor/definition blocks to the detail document subsection. This consolidates detailed regime variable documentation into a dedicated subsection while keeping anchors concise reference points.

### retire-rbags-rbs-files (₢ASAAI) [complete]

**[260216-0756] complete**

Delete RBAGS-AdminGoogleSpec.adoc and RBS-Specification.adoc after RBSA is complete and verified.

## Context (from ₢ASAAD design)

Once RBSA absorbs all content from both files, the originals become dead weight. This pace is the final cleanup.

## Work required

1. Verify RBSA contains all content from both files:
   - Every include:: from RBAGS is present in RBSA
   - Every definition section from RBS is present in RBSA
   - Mapping section terms are superset of both originals

2. Delete:
   - lenses/RBAGS-AdminGoogleSpec.adoc
   - lenses/RBS-Specification.adoc

3. Update any external references:
   - CLAUDE.md file acronym mappings (RBAGS, RBS entries -> RBSA)
   - Any cross-references in other lenses/ documents
   - Paddock references

4. Eliminate remaining stale content:
   - cmk_* vocabulary (if any survived earlier paces)
   - CRR references in RBSA (crg_* terms)
   - Any orphaned terms from the merge

## Acceptance criteria
- RBAGS and RBS files deleted
- RBSA is the sole parent spec
- No broken references anywhere in lenses/
- CLAUDE.md updated

**[260207-0854] rough**

Delete RBAGS-AdminGoogleSpec.adoc and RBS-Specification.adoc after RBSA is complete and verified.

## Context (from ₢ASAAD design)

Once RBSA absorbs all content from both files, the originals become dead weight. This pace is the final cleanup.

## Work required

1. Verify RBSA contains all content from both files:
   - Every include:: from RBAGS is present in RBSA
   - Every definition section from RBS is present in RBSA
   - Mapping section terms are superset of both originals

2. Delete:
   - lenses/RBAGS-AdminGoogleSpec.adoc
   - lenses/RBS-Specification.adoc

3. Update any external references:
   - CLAUDE.md file acronym mappings (RBAGS, RBS entries -> RBSA)
   - Any cross-references in other lenses/ documents
   - Paddock references

4. Eliminate remaining stale content:
   - cmk_* vocabulary (if any survived earlier paces)
   - CRR references in RBSA (crg_* terms)
   - Any orphaned terms from the merge

## Acceptance criteria
- RBAGS and RBS files deleted
- RBSA is the sole parent spec
- No broken references anywhere in lenses/
- CLAUDE.md updated

### expand-axla-regime-section (₢ASAAA) [abandoned]

**[260207-0906] abandoned**

Deferred to downstream AXLA heat. See paddock Downstream Work section.

Original scope preserved: Expand AXLA's axrg_* regime terms into complete pattern documentation. Extract regime architecture from CRR and document as AXLA voicings of axrg_* motifs.

**[260207-0853] rough**

DEFERRED to downstream AXLA heat.

## Rationale (from ₢ASAAD design and paddock)

The paddock explicitly states: "After completing this heat, plan a follow-up heat to refine AXLA regime vocabulary itself." Expanding axrg_* is AXLA vocabulary work, not RBSA consolidation work.

This pace should move to the downstream AXLA refinement heat once ₣AS completes. The RBSA consolidation work (especially ₢ASAAC regime unification) will generate insights about what AXLA regime patterns need, making this pace more informed when it executes later.

## Original scope (preserved for transfer)
Expand AXLA's axrg_* regime terms into complete pattern documentation. Extract regime architecture from CRR and document as AXLA voicings of axrg_* motifs.

**[260131-1149] rough**

Expand AXLA's axrg_* regime terms into complete pattern documentation.

Currently AXLA defines abstract regime motifs (axrg_regime, axrg_variable, axrg_assignment, axrg_prefix) but leaves implementation patterns unexplained. CRR documents these patterns concretely but is Recipe Bottle-specific.

Extract regime architecture from CRR and document as AXLA voicings of axrg_* motifs:
- Structure of regime specifications (sections, variable tables, format requirements)
- Glossary and mapping section patterns
- Assignment file format (makefile, bash, JSON, etc.)
- Validation and rendering architecture

Result: AXLA becomes authoritative reference for regime pattern; specs reference AXLA rather than repeating CRR.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 L migrate-missing-rbs-defs-to-rbsa
  2 G decide-google-type-voicing-pattern
  3 D plan-rbsa-consolidation
  4 H create-rbsa-skeleton
  5 J assess-rbs-to-rbsa-loss
  6 K assess-rbags-to-rbsa-loss
  7 B express-local-ops-axla-voicings
  8 C await-retirement-heat-at
  9 F move-regime-vars-to-detail-subsection
  10 I retire-rbags-rbs-files
  11 * heat-level

LGDHJKBCFI*
x··xx···x·· RBSA-SpecTop.adoc
··········x JJSCRT-retire.adoc, jjc-heat-braid.md, jjc-heat-furlough.md, jjc-heat-garland.md, jjc-heat-groom.md, jjc-heat-mount.md, jjc-heat-muster.md, jjc-heat-nominate.md, jjc-heat-rail.md, jjc-heat-rein.md, jjc-heat-retire-FINAL.md, jjc-heat-retire-dryrun.md, jjc-pace-bridle.md, jjc-pace-reslate.md, jjc-pace-slate.md, jjc-pace-wrap.md, jjc-parade.md, jjc-scout.md, jjro_ops.rs, jjrpd_parade.rs, vocjjmc_core.md
·········x· CLAUDE.md
···x······· RBSAX-access_setup.adoc, RBSBC-bottle_create.adoc, RBSBK-bottle_cleanup.adoc, RBSBL-bottle_launch.adoc, RBSBR-bottle_run.adoc, RBSBS-bottle_start.adoc, RBSCE-command_exec.adoc, RBSDS-dns_step.adoc, RBSIP-iptables_init.adoc, RBSNC-network_create.adoc, RBSNX-network_connect.adoc, RBSPT-port_setup.adoc, RBSSC-security_config.adoc, RBSSS-sentry_start.adoc, RBSVC-rbv_check.adoc, RBSVM-rbv_mirror.adoc

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 68 commits)

  1 K assess-rbags-to-rbsa-loss
  2 H create-rbsa-skeleton
  3 J assess-rbs-to-rbsa-loss
  4 L migrate-missing-rbs-defs-to-rbsa
  5 B express-local-ops-axla-voicings
  6 C await-retirement-heat-at
  7 F move-regime-vars-to-detail-subsection
  8 I retire-rbags-rbs-files
  9 * heat-level

123456789abcdefghijklmnopqrstuvwxyz
·x··········xxx····················  K  4c
···xxxx····························  H  4c
·······xxxxx·······················  J  5c
················xxxx···············  L  4c
····················x·x············  B  2c
···························x·······  C  1c
····························xxx····  F  3c
·······························xxx·  I  3c
x·x············x·····x·xxxx·······x  *  9c
```

## Steeplechase

### 2026-02-16 08:15 - Heat - n

Add file-touch bitmap and commit swim lanes to trophy format

### 2026-02-16 07:56 - ₢ASAAI - W

Removed stale RBAGS/RBS acronym mappings from CLAUDE.md; RBS file already deleted by APAAj

### 2026-02-16 07:55 - ₢ASAAI - n

Remove stale RBAGS and RBS acronym mappings from CLAUDE.md, update doc pattern example to RBSA

### 2026-02-16 07:54 - ₢ASAAI - A

Delete RBS (RBAGS already gone), update CLAUDE.md mappings, verify no broken references

### 2026-02-16 07:42 - ₢ASAAF - W

Trimmed RBRR/RBRA/RBRP/RBRO parent defs to 1-sentence gestalt; all operational detail already in subdocs

### 2026-02-16 07:42 - ₢ASAAF - n

Trim RBRR/RBRA/RBRP/RBRO parent anchor defs to 1-sentence gestalt; operational detail already in subdocs

### 2026-02-16 07:35 - ₢ASAAF - A

Trim parent anchor defs in RBRR/RBRA/RBRP/RBRO to 1-sentence gestalt; detail already in subdocs

### 2026-02-16 07:32 - ₢ASAAC - W

Heat AT retired; dependency satisfied

### 2026-02-09 06:34 - Heat - T

await-retirement-heat-at

### 2026-02-09 06:32 - Heat - T

await-heat-at-retirement

### 2026-02-08 15:44 - Heat - T

blocked-on-at-regime-consolidation

### 2026-02-08 15:44 - Heat - T

consolidate-regime-definitions-rbsa

### 2026-02-08 15:26 - ₢ASAAB - W

pace complete

### 2026-02-08 15:11 - Heat - T

express-local-ops-axla-voicings

### 2026-02-08 15:02 - ₢ASAAB - A

Add AXLA annotations to local ops anchors, convert subdocs to rbbc_* control voicing pattern, eliminate cmk_* vocabulary

### 2026-02-08 14:53 - ₢ASAAL - W

pace complete

### 2026-02-08 14:53 - ₢ASAAL - n

Migrate ~20 missing RBS defs to RBSA: add attribute mappings + anchor definitions for architecture terms, naming patterns, and supporting infrastructure

### 2026-02-08 14:46 - ₢ASAAL - F

Executing via sonnet agent: migrate ~20 missing RBS defs to RBSA

### 2026-02-08 14:44 - ₢ASAAL - A

Migrate ~20 missing RBS defs to RBSA: add attribute mappings + anchor definitions for architecture terms, naming patterns, and supporting infrastructure

### 2026-02-08 14:42 - Heat - S

migrate-missing-rbs-defs-to-rbsa

### 2026-02-08 12:57 - ₢ASAAK - W

pace complete

### 2026-02-08 12:57 - ₢ASAAK - L

opus landed

### 2026-02-08 12:51 - ₢ASAAK - F

Executing bridled pace via opus agent

### 2026-02-08 12:48 - ₢ASAAJ - W

pace complete

### 2026-02-08 12:48 - ₢ASAAJ - n

Add 20 missing attribute mappings to RBSA-SpecTop and define at_sessile_service/at_agile_service anchors

### 2026-02-08 12:46 - ₢ASAAJ - A

Fixing 20 undefined attribute mappings in RBSA-SpecTop.adoc

### 2026-02-08 11:52 - ₢ASAAJ - L

opus landed

### 2026-02-08 11:36 - ₢ASAAJ - F

Executing bridled pace via opus agent

### 2026-02-08 11:34 - ₢ASAAH - W

pace complete

### 2026-02-08 11:32 - ₢ASAAH - n

Explode 16 RBS procedures into subdocs, wire into RBSA Local Operations and Script Internals

### 2026-02-08 10:01 - ₢ASAAH - n

Consolidate duplicate RBRA sections into single Authentication Regime section

### 2026-02-08 09:56 - ₢ASAAH - A

Review RBSA-SpecTop.adoc against 6 acceptance criteria, fix gaps

### 2026-02-07 14:07 - Heat - T

create-rbsa-skeleton

### 2026-02-07 14:04 - ₢ASAAK - B

arm | assess-rbags-to-rbsa-loss

### 2026-02-07 14:04 - Heat - T

assess-rbags-to-rbsa-loss

### 2026-02-07 14:04 - ₢ASAAJ - B

arm | assess-rbs-to-rbsa-loss

### 2026-02-07 14:04 - Heat - T

assess-rbs-to-rbsa-loss

### 2026-02-07 14:03 - Heat - S

assess-rbags-to-rbsa-loss

### 2026-02-07 14:03 - Heat - S

assess-rbs-to-rbsa-loss

### 2026-02-07 13:19 - ₢ASAAH - A

git mv RBAGS→RBSA, restructure three-tier, merge RBS content

### 2026-02-07 09:09 - ₢ASAAD - W

pace complete

### 2026-02-07 09:09 - ₢ASAAD - n

consolidate-regime-definitions-rbsa

### 2026-02-07 09:07 - Heat - T

consolidate-regime-definitions-rbsa

### 2026-02-07 09:07 - Heat - T

create-rbsa-skeleton

### 2026-02-07 09:06 - Heat - T

expand-axla-regime-section

### 2026-02-07 08:54 - Heat - r

moved ASAAA after ASAAI

### 2026-02-07 08:54 - Heat - S

retire-rbags-rbs-files

### 2026-02-07 08:53 - Heat - S

create-rbsa-skeleton

### 2026-02-07 08:53 - Heat - T

expand-axla-regime-section

### 2026-02-07 08:53 - Heat - T

move-regime-vars-to-detail-subsection

### 2026-02-07 08:51 - Heat - T

merge-rbrn-into-rbs

### 2026-02-07 08:51 - Heat - T

explode-rbs-procedures-axla-voicings

### 2026-02-07 08:23 - ₢ASAAD - A

Analyze RBAGS/RBS structure, draft RBSA consolidation design document

### 2026-02-07 08:22 - ₢ASAAG - W

pace complete

### 2026-02-07 08:22 - ₢ASAAG - n

jjb:1011-a8c3738f:₢ASAAG:A: Document Google type voicing decision and annotation-first architectural insight

### 2026-02-07 07:47 - ₢ASAAG - A

Analyzed existing patterns: rbst_ voices axtu_, axtg_ used directly in variable annotations

### 2026-02-07 07:45 - Heat - f

racing

### 2026-02-01 20:25 - Heat - T

plan-rbsa-consolidation

### 2026-01-31 23:09 - Heat - T

merge-rbrn-into-rbs

### 2026-01-31 23:09 - Heat - S

decide-google-type-voicing-pattern

### 2026-01-31 12:15 - Heat - S

move-regime-vars-to-detail-subsection

### 2026-01-31 11:52 - Heat - r

moved ASAAD to first

### 2026-01-31 11:52 - Heat - S

create-burs-regime-spec

### 2026-01-31 11:52 - Heat - S

plan-rbsa-consolidation

### 2026-01-31 11:49 - Heat - S

merge-rbrn-into-rbs

### 2026-01-31 11:49 - Heat - S

explode-rbs-procedures-axla-voicings

### 2026-01-31 11:49 - Heat - S

expand-axla-regime-section

### 2026-01-31 11:49 - Heat - N

rbw-spec-axla-coherence

