# Heat ₣Aj: rbk-axla-term-voicing-scrub

## Goal

Align linked terms across RBS0 (and propagate to BUS0, JJS0, VOS0) so that:
1. All domain-specific terms in project specs use project-owned prefixes (rb*, bu*, jj*, etc.)
2. Google infrastructure concepts are voiced through AXLA motifs, not re-exported as google-prefixed terms
3. Operation/command/routine voicing annotations follow the structural hierarchy pattern established by regimes, not the older axl_voices lineage pattern

## Prerequisite: ₣Ai ₢AiAAP

₣Ai pace ₢AiAAP (introduce-rubric-vocabulary-fix-paddock) adds the trigger-migration
AXLA motifs (axig_developer_connect, axig_build_trigger, axig_repo_link,
axig_slsa_provenance, axig_build_config) and rbtgr_*/rbtgi_*/rbtgo_* linked terms.
This heat should NOT re-add those motifs — they are already landed by the time ₣Aj starts.

Work Area 1 below should focus on the REMAINING motifs needed beyond what ₢AiAAP provides,
plus the hierarchy marker design for operations.

## Background

RBS0 currently has ~30 google-domain-prefixed linked terms (gcb_*, gar_*, gcs_*, giam_*, gcp_*) that predate AXLA's voicing infrastructure. These were minted when RBS0 had to carry the burden of explaining Google concepts directly. Now that AXLA has axig_* (Infrastructure Google), axtg_* (Type Google), and related motifs, these terms should be refactored to rb*-prefixed voicings.

Additionally, the operation/command sections in RBS0 use `// ⟦axl_voices axo_command axe_bash_interactive⟧` annotations — a lineage-oriented pattern. The regime sections use a different, more declarative hierarchy pattern: `// ⟦axhrb_regime⟧`, `// ⟦axhrgb_group⟧`, `// ⟦axhrgv_variable⟧`. The regime pattern is preferred. Operations need equivalent hierarchy markers in AXLA (axho_* or similar) so that operation definitions can use the same structural annotation style.

## Scope

### Work Area 1: AXLA motif additions (beyond ₢AiAAP)

₢AiAAP already adds: axig_developer_connect, axig_build_trigger, axig_repo_link,
axig_slsa_provenance, axig_build_config. Do NOT duplicate these.

Additional AXLA motifs still needed for the google-prefix migration:
- Motifs for generic Google services (Cloud Build, Artifact Registry, IAM, etc.)
- Possibly others discovered during inventory of the ~30 google-prefixed terms

Also: design and add hierarchy markers for operations/commands/procedures parallel to the axhr* regime markers. The regime pattern (axhrb_regime, axhrgb_group, axhrgv_variable, axhro_kindle/render/validate) needs an operations equivalent so that RBS0 operation sections can use structural annotations instead of axl_voices lineage.

### Work Area 2: RBS0 google-prefix term migration

Rename existing google-domain terms to rb*-prefixed terms:

Current prefixes to migrate:
- gcb_build, gcb_service, gcb_service_p, gcb_service_s → rbtgi_* or rbw_* voicings
- gar_registry, gar_registry_s, gar_service → rbtgi_* voicings
- gcs_bucket, gcs_bucket_s, gcs_service → rbtgi_* voicings
- giam_binding, giam_binding_s, giam_role, giam_role_s, giam_service_account, giam_service_account_s → rbtgi_* voicings
- gcp_billing_enabled, gcp_delete_requested, gcp_folder, gcp_organization, gcp_project_id, gcp_project_number, gcp_lro, gcp_lien, gcp_service → rbtgi_* or appropriate rb* voicings
- oauth_token → appropriate rb* voicing

Each renamed term needs:
- New rb*-prefixed attribute, anchor, and definition
- Updated annotation using the new hierarchy markers (not axl_voices)
- Global search-and-replace across all lenses that reference the old attribute

### Work Area 3: Operation annotation pattern revision

Convert all operation/command/procedure annotations from:
  `// ⟦axl_voices axo_command axe_bash_interactive⟧`
to the new hierarchy marker pattern (designed in Work Area 1).

This touches every operation definition in RBS0 (~25 operations).

### Work Area 4: Propagation to other specs

After AXLA changes and RBS0 migration, check and update:
- BUS0 (BUS0-BashUtilitiesSpec.adoc) — uses AXLA annotations
- JJS0 (JJS0-GallopsData.adoc) — uses AXLA annotations
- VOS0 (VOS0-VoxObscuraSpec.adoc) — may use google-prefixed terms or AXLA annotations
- Any other specs that import google-prefixed terms from RBS0

## Constraints

- This is a terminology refactor, not a behavioral change — no code modifications
- All rename operations must be atomic (attribute + anchor + all references in single commit)
- AXLA changes must land first since specs voice AXLA motifs
- Consider whether gcp_service ("Google Cloud") is worth rb-prefixing or whether it's so generic it stays
- Run /cma-validate after each major rename to catch broken references

## Open Questions (resolve during pacing)

- What rb* prefix category for generic Google infrastructure? rbtgi_* (Google Instance) seems right for specific resources (the depot project, the build bucket), but what about generic service names (Cloud Build, Artifact Registry)?
- Should we introduce a new prefix category like rbtgs_* (Google Service) for service-level terms distinct from instance-level terms?
- The exact hierarchy marker design for operations — what parallels axhrb_regime for commands? axhoc_command? axhoo_operation?