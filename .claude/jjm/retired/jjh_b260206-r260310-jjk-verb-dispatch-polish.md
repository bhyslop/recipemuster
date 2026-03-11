# Heat Trophy: jjk-verb-dispatch-polish

**Firemark:** ₣AW
**Created:** 260206
**Retired:** 260310
**Status:** retired

## Paddock

# Paddock: jjk-boring-cli-rename

## Context

Rename jjx_* CLI commands from horse-racing vocabulary to boring, heterogeneous, self-evident names. The shared vocabulary between slash commands (workflow layer) and jjx_ commands (tool layer) causes the LLM to conflate the two layers, guess flags by analogy, and reach for raw CLI commands instead of following slash command workflows.

**Root problems identified:**
1. 12 of 22 jjx_ commands share names with slash commands (parade/parade, muster/muster, etc.)
2. Regular naming patterns invite analogical inference — seeing `--remaining` on parade leads to guessing it exists on muster
3. The `vvx_commit` / `jjx_notch` overlap compounds the confusion
4. Multipurpose `jjx_tally` causes repeated misfires (wrong flag combinations for different operations)
5. "spec" collides with .adoc specification documents; "direction" is too generic

**Solution:** Four coordinated changes:
1. Rename jjx_ commands to heterogeneous boring names (show, list, create, enroll, etc.)
2. Split jjx_tally into single-purpose primitives composable with `&&`
3. Add a CLI truth table to vocjjmc_core.md (always in LLM context, eliminates guessing)
4. Delete thin wrapper slash commands; keep only genuine orchestrations as skills

**Design constraints:**
- Horse vocabulary stays in the slash command layer (the human interface)
- The CLI layer becomes plumbing that the LLM doesn't pattern-match against
- JSON schema unchanged — vocabulary changes are presentation/CLI only
- Schema renames (text→docket, direction→warrant) deferred to ₣AG

## Vocabulary

| Layer | Old term | New term | Meaning |
|---|---|---|---|
| Presentation | spec / specification | **docket** | What the pace should accomplish (tack text field) |
| Presentation | direction | **warrant** | Execution guidance for autonomous operation (tack direction field) |
| CLI flag | --full | **--detail** | Show detailed view with paddock and docket text |
| JSON schema | text | text (unchanged) | Deferred to ₣AG ₢AGAAK |
| JSON schema | direction | direction (unchanged) | Deferred to ₣AG ₢AGAAL |

## Rename Mapping

| Current | Proposed | What it does |
|---|---|---|
| jjx_parade | jjx_show | Display heat or pace info |
| jjx_muster | jjx_list | List all heats |
| jjx_nominate | jjx_create | Create heat |
| jjx_slate | jjx_enroll | Add pace to heat |
| jjx_rail | jjx_reorder | Reorder paces |
| jjx_furlough | jjx_alter | Change heat status/rename |
| jjx_notch | jjx_record | JJ-aware commit |
| jjx_wrap | jjx_close | Mark pace complete |
| jjx_rein | jjx_log | Parse steeplechase history |
| jjx_scout | jjx_search | Regex search |
| jjx_retire | jjx_archive | Extract heat for archival |
| jjx_restring | jjx_transfer | Bulk move paces |
| jjx_garland | jjx_continue | Create continuation heat |
| jjx_chalk | jjx_mark | Marker commit |
| jjx_curry | jjx_paddock | Get/set paddock |
| jjx_draft | jjx_relocate | Move single pace |
| jjx_saddle | jjx_orient | Get context for starting |
| jjx_get_spec | jjx_get_brief | Get raw docket text |
| jjx_get_coronets | keep | List coronets |
| jjx_landing | keep | Record agent return |
| jjx_validate | keep | Validate gallops JSON |

## Tally Split

jjx_tally replaced by four single-purpose commands:

| Command | Signature | Does one thing |
|---|---|---|
| jjx_revise_docket | `CORONET < stdin` | Update docket, carry forward state/silks |
| jjx_arm | `CORONET < stdin` | Set state=bridled, stdin is the warrant |
| jjx_relabel | `CORONET --silks "name"` | Rename, carry forward docket/state |
| jjx_drop | `CORONET` | Set state=abandoned |

Compose with `&&` for multi-field updates:
```bash
jjx_revise_docket C < docket && jjx_relabel C --silks "name"
jjx_revise_docket C < docket && jjx_arm C < warrant
```

## Thin Wrapper Slash Commands to Delete

- /jjc-heat-muster (just calls jjx_list)
- /jjc-heat-rein (just calls jjx_log)
- /jjc-scout (just calls jjx_search)
- /jjc-parade (dispatch to jjx_show)
- /jjc-heat-nominate (just calls jjx_create)
- /jjc-pace-wrap (just calls jjx_close)

## References

- Tools/jjk/vov_veiled/JJS0-GallopsData.adoc — operation definitions
- Tools/jjk/vov_veiled/src/jjrx_cli.rs — CLI registration
- Tools/jjk/vov_veiled/vocjjmc_core.md — CLAUDE.md managed source
- .claude/commands/jjc-*.md — slash command definitions
- ₣AG ₢AGAAK, ₢AGAAL — deferred schema renames (text→docket, direction→warrant)
- Origin: ₣AH ₢AHAAV explore-parade-flag-reduction

## Paces

### design-invitatory-session-mark (₢AWAAM) [complete]

**[260207-0720] complete**

Design decisions for INVITATORY session-begin marker, migrating from jjb :s: to vvb :i:.

Decisions reached:
1. TRIGGER: Both saddle (mount) AND muster trigger invitatory. Standalone vvx invitatory command that checks session gap internally. Saddle and muster call it; the command is idempotent (gap check prevents double-fire).
2. COMMIT FORMAT: vvb:HALLMARK::i: INVITATORY YYMMDD-HHMM — fits existing VOS 4-field format with empty identity (like A and R). Body contains probe data.
3. VOS SPEC: Add voscai_invitatory action code i (lowercase) to VOS Commit Message Architecture section. vosr_probe already exists.
4. JJSA IMPACT: Remove s (session) action code from JJSA. Delete jjrn_format_session_message(). Move jjrc_needs_session_probe() to VVC. Rein ignores vvb: commits (different prefix). Clean separation.
5. BACKWARD COMPAT: Don't care. Old jjb:...:s: commits stay in history. Document transition.
6. SHARED FORMATTER: New vvcc_format_branded(brand, hallmark, identity, action, subject, body) in VVC. Both jjb: and vvb: commits use it. Eliminates 8 duplicate format strings in jjrn_notch.rs. Pure formatting, no side effects.

Deliverable: This design document (in the tack). Implementation follows in subsequent paces.

**[260207-0715] rough**

Design decisions for INVITATORY session-begin marker, migrating from jjb :s: to vvb :i:.

Decisions reached:
1. TRIGGER: Both saddle (mount) AND muster trigger invitatory. Standalone vvx invitatory command that checks session gap internally. Saddle and muster call it; the command is idempotent (gap check prevents double-fire).
2. COMMIT FORMAT: vvb:HALLMARK::i: INVITATORY YYMMDD-HHMM — fits existing VOS 4-field format with empty identity (like A and R). Body contains probe data.
3. VOS SPEC: Add voscai_invitatory action code i (lowercase) to VOS Commit Message Architecture section. vosr_probe already exists.
4. JJSA IMPACT: Remove s (session) action code from JJSA. Delete jjrn_format_session_message(). Move jjrc_needs_session_probe() to VVC. Rein ignores vvb: commits (different prefix). Clean separation.
5. BACKWARD COMPAT: Don't care. Old jjb:...:s: commits stay in history. Document transition.
6. SHARED FORMATTER: New vvcc_format_branded(brand, hallmark, identity, action, subject, body) in VVC. Both jjb: and vvb: commits use it. Eliminates 8 duplicate format strings in jjrn_notch.rs. Pure formatting, no side effects.

Deliverable: This design document (in the tack). Implementation follows in subsequent paces.

**[260206-2303] rough**

Design the INVITATORY session-begin marker as a vvb-domain concept.

Background: The current session probe (vvcp_probe) is called from jjrsd_saddle and stamps a session commit as jjb:HALLMARK:₣XX:s:. This is wrong on two axes:
1. Sessions are cross-heat — you muster, mount AW, switch to AR. Tying to a single firemark is incorrect.
2. The concept belongs in VOS/VVC (arcane universe), not JJK (equestrian universe).

Decisions reached:
- Name: INVITATORY (liturgical: the call that opens the Divine Office)
- Action letter: :i: (available, not used by any current chalk marker or heat action)
- Commit prefix: vvb: (not jjb:) — this is VVC's domain
- Searchable token: INVITATORY appears in commit body for easy git log --grep

Open questions to resolve:
1. What triggers the invitatory? Currently only saddle (via mount). Should muster or any first-jjx-command trigger it? Or should it be a standalone vvx command?
2. Commit format: vvb:HALLMARK:i: INVITATORY YYMMDD-HHMM — no identity field needed since it's heat-independent. But does this break the existing 4-field format convention?
3. VOS spec voicing: INVITATORY needs to be defined in VOS0-VoxObscuraSpec.adoc. What section? What linked terms?
4. JJSA impact: Remove session marker (s) from JJSA steeplechase protocol, or keep it as a JJK-level alias that delegates to vvb?
5. Backward compat: Existing :s: session commits in git history — do we care? Probably not, just document the transition.

Deliverable: A written design in the paddock or a memo that resolves all open questions above, ready for implementation in the subsequent pace (₢AWAAL probe-opus-as-interpreter).

### add-vvcc-format-branded (₢AWAAN) [complete]

**[260207-0830] complete**

Create vvcc_format_branded() and vvcc_get_hallmark() in VVC as shared infrastructure for all branded commits (jjb: and vvb:).

## vvcc_get_hallmark

Extract zjjrn_get_hallmark() from Tools/jjk/vov_veiled/src/jjrn_notch.rs into VVC as pub fn vvcc_get_hallmark(). Same logic: try .vvk/vvbf_brand.json field vvbh_hallmark, fall back to max hallmark from Tools/vok/vov_veiled/vovr_registry.json + git rev-parse --short HEAD. Export from Tools/vvc/src/lib.rs.

RCG compliance: The brand file field name "vvbh_hallmark" and registry path must reference const strings, not magic text. Define consts (e.g., VVCC_BRAND_FILE_PATH, VVCC_REGISTRY_PATH, VVCC_HALLMARK_FIELD) and use them in both parse and emit.

Also migrate jjrnc_notch.rs (the notch CLI handler): lines 78-100 have a SEPARATE hallmark implementation reading from "git config jj.hallmark" — a different source that diverges from the brand file/registry logic. Replace with vvc::vvcc_get_hallmark() call, eliminating the divergent path. Then replace the manual format! on lines 94-100 with vvcc_format_branded("jjb", hallmark, identity, "n", "", None).

## vvcc_format_branded

Signature:
  vvcc_format_branded(brand: &str, hallmark: &str, identity: &str, action: &str, subject: &str, body: Option<&str>) -> String

Output: "{brand}:{hallmark}:{identity}:{action}: {subject}" with optional "\n\n{body}" appended.

RCG compliance: The brand strings "jjb" and "vvb" should be const-defined at their respective crate boundaries (JJRN_COMMIT_PREFIX already exists in JJK; add VVCC_BRAND_PREFIX or similar in VVC). The formatter itself takes &str — it doesn't own these consts.

Location: new file Tools/vvc/src/vvcc_format.rs (keeps vvcc_commit.rs focused on commit workflow). Export from lib.rs.

## Migration

Migrate 6 format functions in Tools/jjk/vov_veiled/src/jjrn_notch.rs to call vvcc_format_branded():
- jjrn_format_notch_prefix: special case — subject="" produces "...:n: " and caller appends real message
- jjrn_format_chalk_message
- jjrn_format_heat_discussion
- jjrn_format_heat_message
- jjrn_format_bridle_message
- jjrn_format_landing_message
DO NOT touch jjrn_format_session_message — deleted by ₢AWAAP.

Delete zjjrn_get_hallmark() from jjrn_notch.rs (replaced by vvc::vvcc_get_hallmark()).

## Testing

Unit tests in new Tools/vvc/src/vvtf_format.rs:
- Empty identity produces "brand:hallmark::action: subject"
- With identity produces "brand:hallmark:identity:action: subject"
- With body appends "\n\n{body}"
- Without body, no trailing newlines
- Colon enforcement: 4 colon-delimited fields always present

Build and test: tt/vow-b.Build.sh && tt/vow-t.Test.sh

## Key files
- Tools/vvc/src/vvcc_format.rs (NEW — shared formatter + hallmark)
- Tools/vvc/src/vvtf_format.rs (NEW — tests)
- Tools/vvc/src/lib.rs (exports)
- Tools/jjk/vov_veiled/src/jjrn_notch.rs (migrate 6 functions, delete zjjrn_get_hallmark)
- Tools/jjk/vov_veiled/src/jjrnc_notch.rs (replace divergent hallmark + manual format)

**[260207-0817] bridled**

Create vvcc_format_branded() and vvcc_get_hallmark() in VVC as shared infrastructure for all branded commits (jjb: and vvb:).

## vvcc_get_hallmark

Extract zjjrn_get_hallmark() from Tools/jjk/vov_veiled/src/jjrn_notch.rs into VVC as pub fn vvcc_get_hallmark(). Same logic: try .vvk/vvbf_brand.json field vvbh_hallmark, fall back to max hallmark from Tools/vok/vov_veiled/vovr_registry.json + git rev-parse --short HEAD. Export from Tools/vvc/src/lib.rs.

RCG compliance: The brand file field name "vvbh_hallmark" and registry path must reference const strings, not magic text. Define consts (e.g., VVCC_BRAND_FILE_PATH, VVCC_REGISTRY_PATH, VVCC_HALLMARK_FIELD) and use them in both parse and emit.

Also migrate jjrnc_notch.rs (the notch CLI handler): lines 78-100 have a SEPARATE hallmark implementation reading from "git config jj.hallmark" — a different source that diverges from the brand file/registry logic. Replace with vvc::vvcc_get_hallmark() call, eliminating the divergent path. Then replace the manual format! on lines 94-100 with vvcc_format_branded("jjb", hallmark, identity, "n", "", None).

## vvcc_format_branded

Signature:
  vvcc_format_branded(brand: &str, hallmark: &str, identity: &str, action: &str, subject: &str, body: Option<&str>) -> String

Output: "{brand}:{hallmark}:{identity}:{action}: {subject}" with optional "\n\n{body}" appended.

RCG compliance: The brand strings "jjb" and "vvb" should be const-defined at their respective crate boundaries (JJRN_COMMIT_PREFIX already exists in JJK; add VVCC_BRAND_PREFIX or similar in VVC). The formatter itself takes &str — it doesn't own these consts.

Location: new file Tools/vvc/src/vvcc_format.rs (keeps vvcc_commit.rs focused on commit workflow). Export from lib.rs.

## Migration

Migrate 6 format functions in Tools/jjk/vov_veiled/src/jjrn_notch.rs to call vvcc_format_branded():
- jjrn_format_notch_prefix: special case — subject="" produces "...:n: " and caller appends real message
- jjrn_format_chalk_message
- jjrn_format_heat_discussion
- jjrn_format_heat_message
- jjrn_format_bridle_message
- jjrn_format_landing_message
DO NOT touch jjrn_format_session_message — deleted by ₢AWAAP.

Delete zjjrn_get_hallmark() from jjrn_notch.rs (replaced by vvc::vvcc_get_hallmark()).

## Testing

Unit tests in new Tools/vvc/src/vvtf_format.rs:
- Empty identity produces "brand:hallmark::action: subject"
- With identity produces "brand:hallmark:identity:action: subject"
- With body appends "\n\n{body}"
- Without body, no trailing newlines
- Colon enforcement: 4 colon-delimited fields always present

Build and test: tt/vow-b.Build.sh && tt/vow-t.Test.sh

## Key files
- Tools/vvc/src/vvcc_format.rs (NEW — shared formatter + hallmark)
- Tools/vvc/src/vvtf_format.rs (NEW — tests)
- Tools/vvc/src/lib.rs (exports)
- Tools/jjk/vov_veiled/src/jjrn_notch.rs (migrate 6 functions, delete zjjrn_get_hallmark)
- Tools/jjk/vov_veiled/src/jjrnc_notch.rs (replace divergent hallmark + manual format)

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/vvc/Cargo.toml, Tools/vvc/src/vvcc_format.rs NEW, Tools/vvc/src/vvtf_format.rs NEW, Tools/vvc/src/lib.rs, Tools/jjk/vov_veiled/src/jjrn_notch.rs, Tools/jjk/vov_veiled/src/jjrnc_notch.rs 6 files | Steps: 1. Add serde_json dependency to Tools/vvc/Cargo.toml 2. Create Tools/vvc/src/vvcc_format.rs with pub fn vvcc_get_hallmark same logic as zjjrn_get_hallmark in jjrn_notch.rs and pub fn vvcc_format_branded with signature brand, hallmark, identity, action, subject, body Option str returning String formatted as brand:hallmark:identity:action: subject with optional body after double newline -- define consts VVCC_BRAND_FILE_PATH for .vvk/vvbf_brand.json and VVCC_REGISTRY_PATH for Tools/vok/vov_veiled/vovr_registry.json and VVCC_HALLMARK_FIELD for vvbh_hallmark 3. Create Tools/vvc/src/vvtf_format.rs with unit tests: empty identity, with identity, with body, without body, colon enforcement 4. Update Tools/vvc/src/lib.rs to add pub mod vvcc_format and cfg test mod vvtf_format and re-export vvcc_get_hallmark and vvcc_format_branded 5. Migrate 6 functions in jjrn_notch.rs -- jjrn_format_notch_prefix and jjrn_format_chalk_message and jjrn_format_heat_discussion and jjrn_format_heat_message and jjrn_format_bridle_message and jjrn_format_landing_message -- to call vvc::vvcc_format_branded passing JJRN_COMMIT_PREFIX as brand and vvc::vvcc_get_hallmark for hallmark and appropriate identity and action code as str and subject -- delete zjjrn_get_hallmark and remove std::fs and std::process::Command imports if no longer needed -- DO NOT touch jjrn_format_session_message 6. In jjrnc_notch.rs replace the divergent hallmark block reading from git config jj.hallmark with vvc::vvcc_get_hallmark call and replace the manual format! with vvc::vvcc_format_branded call using jjrn_notch::JJRN_COMMIT_PREFIX as brand | Verify: tt/vow-b.Build.sh and tt/vow-t.Test.sh

**[260207-0803] rough**

Create vvcc_format_branded() and vvcc_get_hallmark() in VVC as shared infrastructure for all branded commits (jjb: and vvb:).

## vvcc_get_hallmark

Extract zjjrn_get_hallmark() from Tools/jjk/vov_veiled/src/jjrn_notch.rs into VVC as pub fn vvcc_get_hallmark(). Same logic: try .vvk/vvbf_brand.json field vvbh_hallmark, fall back to max hallmark from Tools/vok/vov_veiled/vovr_registry.json + git rev-parse --short HEAD. Export from Tools/vvc/src/lib.rs.

RCG compliance: The brand file field name "vvbh_hallmark" and registry path must reference const strings, not magic text. Define consts (e.g., VVCC_BRAND_FILE_PATH, VVCC_REGISTRY_PATH, VVCC_HALLMARK_FIELD) and use them in both parse and emit.

Also migrate jjrnc_notch.rs (the notch CLI handler): lines 78-100 have a SEPARATE hallmark implementation reading from "git config jj.hallmark" — a different source that diverges from the brand file/registry logic. Replace with vvc::vvcc_get_hallmark() call, eliminating the divergent path. Then replace the manual format! on lines 94-100 with vvcc_format_branded("jjb", hallmark, identity, "n", "", None).

## vvcc_format_branded

Signature:
  vvcc_format_branded(brand: &str, hallmark: &str, identity: &str, action: &str, subject: &str, body: Option<&str>) -> String

Output: "{brand}:{hallmark}:{identity}:{action}: {subject}" with optional "\n\n{body}" appended.

RCG compliance: The brand strings "jjb" and "vvb" should be const-defined at their respective crate boundaries (JJRN_COMMIT_PREFIX already exists in JJK; add VVCC_BRAND_PREFIX or similar in VVC). The formatter itself takes &str — it doesn't own these consts.

Location: new file Tools/vvc/src/vvcc_format.rs (keeps vvcc_commit.rs focused on commit workflow). Export from lib.rs.

## Migration

Migrate 6 format functions in Tools/jjk/vov_veiled/src/jjrn_notch.rs to call vvcc_format_branded():
- jjrn_format_notch_prefix: special case — subject="" produces "...:n: " and caller appends real message
- jjrn_format_chalk_message
- jjrn_format_heat_discussion
- jjrn_format_heat_message
- jjrn_format_bridle_message
- jjrn_format_landing_message
DO NOT touch jjrn_format_session_message — deleted by ₢AWAAP.

Delete zjjrn_get_hallmark() from jjrn_notch.rs (replaced by vvc::vvcc_get_hallmark()).

## Testing

Unit tests in new Tools/vvc/src/vvtf_format.rs:
- Empty identity produces "brand:hallmark::action: subject"
- With identity produces "brand:hallmark:identity:action: subject"
- With body appends "\n\n{body}"
- Without body, no trailing newlines
- Colon enforcement: 4 colon-delimited fields always present

Build and test: tt/vow-b.Build.sh && tt/vow-t.Test.sh

## Key files
- Tools/vvc/src/vvcc_format.rs (NEW — shared formatter + hallmark)
- Tools/vvc/src/vvtf_format.rs (NEW — tests)
- Tools/vvc/src/lib.rs (exports)
- Tools/jjk/vov_veiled/src/jjrn_notch.rs (migrate 6 functions, delete zjjrn_get_hallmark)
- Tools/jjk/vov_veiled/src/jjrnc_notch.rs (replace divergent hallmark + manual format)

**[260207-0733] rough**

Create vvcc_format_branded() in VVC as the single formatting function for all branded commits (jjb: and vvb:).

Signature:
  vvcc_format_branded(brand: &str, hallmark: &str, identity: &str, action: &str, subject: &str, body: Option<&str>) -> String

Output: "{brand}:{hallmark}:{identity}:{action}: {subject}" with optional "\n\n{body}" appended.

Implementation:
1. Extract zjjrn_get_hallmark() from Tools/jjk/vov_veiled/src/jjrn_notch.rs into VVC as a pub function (e.g., vvcc_get_hallmark() in vvcc_commit.rs or vvcc_format.rs). Same logic: try .vvk/vvbf_brand.json, fall back to registry+HEAD. Export from Tools/vvc/src/lib.rs. JJK callers switch to vvc::vvcc_get_hallmark().
2. Add pub fn vvcc_format_branded() to Tools/vvc/src/vvcc_commit.rs (or a new vvcc_format.rs if cleaner). Export from lib.rs.
3. Add unit tests: empty identity, with identity, with body, without body, colon enforcement.
4. Migrate 6 format functions in Tools/jjk/vov_veiled/src/jjrn_notch.rs to call vvcc_format_branded():
   - jjrn_format_notch_prefix: special case — subject="" produces "...:n: " and caller appends real message
   - jjrn_format_chalk_message
   - jjrn_format_heat_discussion
   - jjrn_format_heat_message
   - jjrn_format_bridle_message
   - jjrn_format_landing_message
   DO NOT touch jjrn_format_session_message — it is deleted by ₢AWAAP.
5. Delete zjjrn_get_hallmark() from jjrn_notch.rs (replaced by vvc::vvcc_get_hallmark()).
6. Build and test: tt/vow-b.Build.sh && tt/vow-t.Test.sh
7. Verify existing commit formats unchanged by inspecting format output in tests.

Key files:
- Tools/vvc/src/vvcc_commit.rs (or new vvcc_format.rs) — new shared formatter + hallmark
- Tools/vvc/src/lib.rs — exports
- Tools/jjk/vov_veiled/src/jjrn_notch.rs — migrate 6 functions, delete zjjrn_get_hallmark

**[260207-0716] rough**

Create vvcc_format_branded() in VVC as the single formatting function for all branded commits (jjb: and vvb:).

Signature:
  vvcc_format_branded(brand: &str, hallmark: &str, identity: &str, action: &str, subject: &str, body: Option<&str>) -> String

Output: "{brand}:{hallmark}:{identity}:{action}: {subject}" with optional "\n\n{body}" appended.

Implementation:
1. Add pub fn vvcc_format_branded() to Tools/vvc/src/vvcc_commit.rs (or a new vvcc_format.rs if cleaner)
2. Export from Tools/vvc/src/lib.rs
3. Add unit tests: empty identity, with identity, with body, without body, colon enforcement
4. Migrate all 8 format functions in Tools/jjk/vov_veiled/src/jjrn_notch.rs to call vvcc_format_branded():
   - jjrn_format_notch_prefix → vvcc_format_branded("jjb", hallmark, identity, "n", "", None) + " "
   - jjrn_format_chalk_message → vvcc_format_branded("jjb", hallmark, identity, marker, description, None)
   - jjrn_format_heat_discussion → vvcc_format_branded("jjb", hallmark, identity, "d", description, None)
   - jjrn_format_heat_message → vvcc_format_branded("jjb", hallmark, identity, action, description, None)
   - jjrn_format_bridle_message → vvcc_format_branded("jjb", hallmark, identity, "B", subject, None)
   - jjrn_format_landing_message → vvcc_format_branded("jjb", hallmark, identity, "L", subject, None)
   - jjrn_format_session_message → DELETE (replaced by invitatory in next pace)
5. Build and test: tt/vow-b.Build.sh && tt/vow-t.Test.sh
6. Verify existing commit formats unchanged by running jjx_chalk, jjx_notch dry-run or inspecting format output

Key files:
- Tools/vvc/src/vvcc_commit.rs (or new vvcc_format.rs)
- Tools/vvc/src/lib.rs
- Tools/jjk/vov_veiled/src/jjrn_notch.rs (8 format functions to migrate)

### add-vvx-invitatory (₢AWAAO) [complete]

**[260207-0849] complete**

Create vvx invitatory command in VVC: officium gap check + probe + vvb:...:i: commit.

Terminology: "officium" (Latin: the Divine Office) replaces "session" for the bounded work period concept. The invitatory opens a new officium.

## Gap detection: vvcp_needs_officium

New function in Tools/vvc/src/vvcp_probe.rs. Logic: scan git log for most recent vvb:...:i: commit. If none found or gap >1 hour, return true. No backward compat with old jjb:...:s: commits.

RCG compliance: Build the grep pattern from const strings, not magic text. Use VVCC_BRAND_PREFIX (from ₢AWAAN) and a new const for the invitatory action code (e.g., VVCC_ACTION_INVITATORY: &str = "i"). Pattern assembled as format!("^{}:.*:{}:", VVCC_BRAND_PREFIX, VVCC_ACTION_INVITATORY). Pass to git log --all --grep=<pattern> --format="%ai" -1.

## CLI registration

Register "invitatory" as a new Commands variant in Tools/vok/src/vorm_main.rs (alongside VvxCommit, VvxGuard, etc.). The handler calls vvc::invitatory() — a thin delegation, same pattern as existing VvxCommit → vvc::commit().

## VVC async export

Export pub async fn vvcc_invitatory() from VVC (in vvcp_probe.rs or vvcc_format.rs). This is the function that both the CLI handler AND JJK (in ₢AWAAP) call directly via Rust async — no subprocess. VVC already has tokio in Cargo.toml.

The invitatory function:
a. Calls vvcp_needs_officium() — if not needed, print "Officium current", return Ok
b. Calls vvcp_probe() for model IDs
c. Gets hallmark via vvcc_get_hallmark() (from ₢AWAAN)
d. Formats commit: vvcc_format_branded(VVCC_BRAND_PREFIX, hallmark, "", VVCC_ACTION_INVITATORY, "OFFICIUM {timestamp}", Some(probe_data))
e. Creates empty commit via vvc::commit() with allow_empty=true, no_stage=true
f. Prints "Officium: {commit_hash} ({timestamp})" on success

RCG compliance: "OFFICIUM" in the subject should be a const (e.g., VVCC_OFFICIUM_TOKEN). The timestamp format should reference or derive from existing timestamp consts if any.

## VOS spec

Add to Tools/vok/vov_veiled/VOS0-VoxObscuraSpec.adoc:
- voscai_invitatory action code i (lowercase) in Commit Message Architecture section
- Definition references "officium" as the concept being opened
- Example: vvb:1011::i: OFFICIUM 260207-0701

## Testing

Build and test: tt/vow-b.Build.sh && tt/vow-t.Test.sh
Manual test: lower gap threshold temporarily, run vvx invitatory, verify commit format with git log -1.

## Key files
- Tools/vvc/src/vvcp_probe.rs (add vvcp_needs_officium, vvcc_invitatory)
- Tools/vvc/src/lib.rs (export invitatory)
- Tools/vok/src/vorm_main.rs (register Commands::Invitatory, thin handler)
- Tools/vok/vov_veiled/VOS0-VoxObscuraSpec.adoc (add voscai_invitatory, define officium)

**[260207-0823] bridled**

Create vvx invitatory command in VVC: officium gap check + probe + vvb:...:i: commit.

Terminology: "officium" (Latin: the Divine Office) replaces "session" for the bounded work period concept. The invitatory opens a new officium.

## Gap detection: vvcp_needs_officium

New function in Tools/vvc/src/vvcp_probe.rs. Logic: scan git log for most recent vvb:...:i: commit. If none found or gap >1 hour, return true. No backward compat with old jjb:...:s: commits.

RCG compliance: Build the grep pattern from const strings, not magic text. Use VVCC_BRAND_PREFIX (from ₢AWAAN) and a new const for the invitatory action code (e.g., VVCC_ACTION_INVITATORY: &str = "i"). Pattern assembled as format!("^{}:.*:{}:", VVCC_BRAND_PREFIX, VVCC_ACTION_INVITATORY). Pass to git log --all --grep=<pattern> --format="%ai" -1.

## CLI registration

Register "invitatory" as a new Commands variant in Tools/vok/src/vorm_main.rs (alongside VvxCommit, VvxGuard, etc.). The handler calls vvc::invitatory() — a thin delegation, same pattern as existing VvxCommit → vvc::commit().

## VVC async export

Export pub async fn vvcc_invitatory() from VVC (in vvcp_probe.rs or vvcc_format.rs). This is the function that both the CLI handler AND JJK (in ₢AWAAP) call directly via Rust async — no subprocess. VVC already has tokio in Cargo.toml.

The invitatory function:
a. Calls vvcp_needs_officium() — if not needed, print "Officium current", return Ok
b. Calls vvcp_probe() for model IDs
c. Gets hallmark via vvcc_get_hallmark() (from ₢AWAAN)
d. Formats commit: vvcc_format_branded(VVCC_BRAND_PREFIX, hallmark, "", VVCC_ACTION_INVITATORY, "OFFICIUM {timestamp}", Some(probe_data))
e. Creates empty commit via vvc::commit() with allow_empty=true, no_stage=true
f. Prints "Officium: {commit_hash} ({timestamp})" on success

RCG compliance: "OFFICIUM" in the subject should be a const (e.g., VVCC_OFFICIUM_TOKEN). The timestamp format should reference or derive from existing timestamp consts if any.

## VOS spec

Add to Tools/vok/vov_veiled/VOS0-VoxObscuraSpec.adoc:
- voscai_invitatory action code i (lowercase) in Commit Message Architecture section
- Definition references "officium" as the concept being opened
- Example: vvb:1011::i: OFFICIUM 260207-0701

## Testing

Build and test: tt/vow-b.Build.sh && tt/vow-t.Test.sh
Manual test: lower gap threshold temporarily, run vvx invitatory, verify commit format with git log -1.

## Key files
- Tools/vvc/src/vvcp_probe.rs (add vvcp_needs_officium, vvcc_invitatory)
- Tools/vvc/src/lib.rs (export invitatory)
- Tools/vok/src/vorm_main.rs (register Commands::Invitatory, thin handler)
- Tools/vok/vov_veiled/VOS0-VoxObscuraSpec.adoc (add voscai_invitatory, define officium)

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/vvc/src/vvcp_probe.rs, Tools/vvc/src/lib.rs, Tools/vok/src/vorm_main.rs, Tools/vok/vov_veiled/VOS0-VoxObscuraSpec.adoc 4 files | Steps: 1. In vvcp_probe.rs add const VVCP_OFFICIUM_GAP_SECS: u64 = 3600 and const VVCC_ACTION_INVITATORY: &str = i and const VVCC_OFFICIUM_TOKEN: &str = OFFICIUM 2. In vvcp_probe.rs add fn vvcp_needs_officium that builds grep pattern from vvc::VVCC_BRAND_PREFIX and VVCC_ACTION_INVITATORY using format, runs git log --all --grep=pattern --format=%ai -1, parses timestamp, returns true if none found or gap exceeds VVCP_OFFICIUM_GAP_SECS 3. In vvcp_probe.rs add pub async fn vvcc_invitatory that calls vvcp_needs_officium and if not needed prints Officium current and returns Ok, otherwise calls vvcp_probe for model IDs, calls vvc::vvcc_get_hallmark for hallmark, formats commit via vvc::vvcc_format_branded with VVCC_BRAND_PREFIX as brand and hallmark and empty string identity and VVCC_ACTION_INVITATORY action and VVCC_OFFICIUM_TOKEN plus space plus YYMMDD-HHMM timestamp as subject and probe_data as body, creates empty commit via vvc::commit with allow_empty=true and no_stage=true, prints Officium: commit_hash timestamp on success 4. In lib.rs re-export vvcc_invitatory and VVCC_ACTION_INVITATORY 5. In vorm_main.rs register Commands::Invitatory variant with command name vvx_invitatory, add run_invitatory async handler that calls vvc::vvcc_invitatory().await, wire into main match arm 6. In VOS0-VoxObscuraSpec.adoc add linked term vost_officium with anchor and attribute definition as a bounded work period opened by the invitatory, then add voscai_invitatory action code i in Action Codes section after voscar_release following same pattern as existing action codes, referencing vost_officium, with example vvb:1011::i: OFFICIUM 260207-0701 | Verify: tt/vow-b.Build.sh and tt/vow-t.Test.sh

**[260207-0803] rough**

Create vvx invitatory command in VVC: officium gap check + probe + vvb:...:i: commit.

Terminology: "officium" (Latin: the Divine Office) replaces "session" for the bounded work period concept. The invitatory opens a new officium.

## Gap detection: vvcp_needs_officium

New function in Tools/vvc/src/vvcp_probe.rs. Logic: scan git log for most recent vvb:...:i: commit. If none found or gap >1 hour, return true. No backward compat with old jjb:...:s: commits.

RCG compliance: Build the grep pattern from const strings, not magic text. Use VVCC_BRAND_PREFIX (from ₢AWAAN) and a new const for the invitatory action code (e.g., VVCC_ACTION_INVITATORY: &str = "i"). Pattern assembled as format!("^{}:.*:{}:", VVCC_BRAND_PREFIX, VVCC_ACTION_INVITATORY). Pass to git log --all --grep=<pattern> --format="%ai" -1.

## CLI registration

Register "invitatory" as a new Commands variant in Tools/vok/src/vorm_main.rs (alongside VvxCommit, VvxGuard, etc.). The handler calls vvc::invitatory() — a thin delegation, same pattern as existing VvxCommit → vvc::commit().

## VVC async export

Export pub async fn vvcc_invitatory() from VVC (in vvcp_probe.rs or vvcc_format.rs). This is the function that both the CLI handler AND JJK (in ₢AWAAP) call directly via Rust async — no subprocess. VVC already has tokio in Cargo.toml.

The invitatory function:
a. Calls vvcp_needs_officium() — if not needed, print "Officium current", return Ok
b. Calls vvcp_probe() for model IDs
c. Gets hallmark via vvcc_get_hallmark() (from ₢AWAAN)
d. Formats commit: vvcc_format_branded(VVCC_BRAND_PREFIX, hallmark, "", VVCC_ACTION_INVITATORY, "OFFICIUM {timestamp}", Some(probe_data))
e. Creates empty commit via vvc::commit() with allow_empty=true, no_stage=true
f. Prints "Officium: {commit_hash} ({timestamp})" on success

RCG compliance: "OFFICIUM" in the subject should be a const (e.g., VVCC_OFFICIUM_TOKEN). The timestamp format should reference or derive from existing timestamp consts if any.

## VOS spec

Add to Tools/vok/vov_veiled/VOS0-VoxObscuraSpec.adoc:
- voscai_invitatory action code i (lowercase) in Commit Message Architecture section
- Definition references "officium" as the concept being opened
- Example: vvb:1011::i: OFFICIUM 260207-0701

## Testing

Build and test: tt/vow-b.Build.sh && tt/vow-t.Test.sh
Manual test: lower gap threshold temporarily, run vvx invitatory, verify commit format with git log -1.

## Key files
- Tools/vvc/src/vvcp_probe.rs (add vvcp_needs_officium, vvcc_invitatory)
- Tools/vvc/src/lib.rs (export invitatory)
- Tools/vok/src/vorm_main.rs (register Commands::Invitatory, thin handler)
- Tools/vok/vov_veiled/VOS0-VoxObscuraSpec.adoc (add voscai_invitatory, define officium)

**[260207-0748] rough**

Create vvx invitatory command in VVC: officium gap check + probe + vvb:...:i: commit.

Terminology: "officium" (Latin: the Divine Office) replaces "session" for the bounded work period concept. The invitatory opens a new officium.

Implementation:
1. Add officium gap detection to VVC (new function vvcp_needs_officium() in Tools/vvc/src/vvcp_probe.rs). Logic: scan git log for most recent vvb:...:i: commit. If none found or gap >1 hour, return true. No backward compat with old jjb:...:s: commits — burn bridges.
2. Register "invitatory" as a new Commands variant in Tools/vok/src/vorm_main.rs (alongside VvxCommit, VvxGuard, etc.). NOT in jjrx_cli.rs — this is a VOK command, not JJK.
3. The invitatory command:
   a. Calls vvcp_needs_officium() — if not needed, print "Officium current", exit 0
   b. Calls vvcp_probe() for model IDs
   c. Gets hallmark via vvc::vvcc_get_hallmark() (shared, from ₢AWAAN)
   d. Formats commit: vvcc_format_branded("vvb", hallmark, "", "i", "OFFICIUM {timestamp}", Some(probe_data))
   e. Creates empty commit via vvc::commit() with allow_empty=true, no_stage=true
   f. Prints "Officium: {commit_hash} ({timestamp})" on success
4. Export invitatory as pub async fn from VVC (e.g., vvc::invitatory()) so JJK can call it directly in ₢AWAAP — not as subprocess.
5. Add VOS spec updates to Tools/vok/vov_veiled/VOS0-VoxObscuraSpec.adoc:
   - voscai_invitatory action code i (lowercase) in Commit Message Architecture section
   - Definition references "officium" as the concept being opened
   - Example: vvb:1011::i: OFFICIUM 260207-0701
6. Build and test: tt/vow-b.Build.sh && tt/vow-t.Test.sh
7. Manual test: lower gap threshold temporarily, run vvx invitatory, verify commit format with git log -1.

Key files:
- Tools/vvc/src/vvcp_probe.rs (add vvcp_needs_officium)
- Tools/vvc/src/lib.rs (export invitatory function)
- Tools/vok/src/vorm_main.rs (register Commands::Invitatory)
- Tools/vok/vov_veiled/VOS0-VoxObscuraSpec.adoc (add voscai_invitatory, define officium)

**[260207-0734] rough**

Create vvx invitatory command in VVC: session gap check + probe + vvb:...:i: commit.

Implementation:
1. Add session gap detection to VVC (new function vvcp_needs_session() in Tools/vvc/src/vvcp_probe.rs). Logic: scan git log for most recent vvb:...:i: commit. If none found or gap >1 hour, return true. No backward compat with old jjb:...:s: commits — burn bridges.
2. Register "invitatory" as a new Commands variant in Tools/vok/src/vorm_main.rs (alongside VvxCommit, VvxGuard, etc.). NOT in jjrx_cli.rs — this is a VOK command, not JJK.
3. The invitatory command:
   a. Calls vvcp_needs_session() — if not needed, print "Session current", exit 0
   b. Calls vvcp_probe() for model IDs
   c. Gets hallmark via vvc::vvcc_get_hallmark() (shared, from ₢AWAAN)
   d. Formats commit: vvcc_format_branded("vvb", hallmark, "", "i", "INVITATORY {timestamp}", Some(probe_data))
   e. Creates empty commit via vvc::commit() with allow_empty=true, no_stage=true
   f. Prints "Session-marker: {commit_hash} ({timestamp})" on success
4. Export invitatory as pub async fn from VVC (e.g., vvc::invitatory()) so JJK can call it directly in ₢AWAAP — not as subprocess.
5. Add VOS spec: voscai_invitatory action code i (lowercase) in Tools/vok/vov_veiled/VOS0-VoxObscuraSpec.adoc Commit Message Architecture section.
6. Build and test: tt/vow-b.Build.sh && tt/vow-t.Test.sh
7. Manual test: lower gap threshold temporarily, run vvx invitatory, verify commit format with git log -1.

Key files:
- Tools/vvc/src/vvcp_probe.rs (add vvcp_needs_session)
- Tools/vvc/src/lib.rs (export invitatory function)
- Tools/vok/src/vorm_main.rs (register Commands::Invitatory)
- Tools/vok/vov_veiled/VOS0-VoxObscuraSpec.adoc (add voscai_invitatory)

**[260207-0716] rough**

Create vvx invitatory command in VVC: session gap check + probe + vvb:...:i: commit.

Implementation:
1. Move jjrc_needs_session_probe() from Tools/jjk/vov_veiled/src/jjrc_core.rs to VVC (new function vvcp_needs_session() or similar in Tools/vvc/src/vvcp_probe.rs). Keep same logic: >1 hour gap or no commits. The function needs to scan git log for the most recent vvb:...:i: commit (not jjb: commits — those are JJK's domain now).
2. Add new vvx subcommand "invitatory" in Tools/vok/vov_veiled/src/jjrx_cli.rs (or wherever vvx subcommands are registered). This command:
   a. Checks if session probe is needed (call vvcp_needs_session)
   b. If not needed: print "Session current" and exit 0
   c. If needed: call vvcp_probe(), format commit using vvcc_format_branded("vvb", hallmark, "", "i", "INVITATORY {timestamp}", Some(probe_data)), create empty commit via vvc::commit()
   d. Print "Session-marker: {commit_hash} ({timestamp})" on success
3. Export from VVC lib.rs as needed
4. Add VOS spec update: voscai_invitatory action code i in VOS0-VoxObscuraSpec.adoc Commit Message Architecture section
5. Build and test: tt/vow-b.Build.sh && tt/vow-t.Test.sh
6. Manual test: lower gap threshold temporarily, run vvx invitatory, verify commit format with git log -1

Key files:
- Tools/vvc/src/vvcp_probe.rs (add session gap check)
- Tools/vvc/src/lib.rs (export)
- Tools/vok/vov_veiled/src/jjrx_cli.rs (register subcommand)
- Tools/vok/vov_veiled/VOS0-VoxObscuraSpec.adoc (add voscai_invitatory)

### wire-invitatory-triggers (₢AWAAP) [complete]

**[260207-0856] complete**

Wire saddle and muster to call vvc::invitatory(). Remove session marker from JJK. Rename surviving "session" references to "officium".

## Saddle wiring

Update Tools/jjk/vov_veiled/src/jjrsd_saddle.rs:
- Remove the inline session probe block (lines ~295-343 that call jjrc_needs_session_probe, vvcp_probe, jjrn_format_session_message, vvc::commit)
- Replace with: direct Rust async call to vvc::vvcc_invitatory(). Saddle is already async, so just .await it. The invitatory handles its own gap check, so saddle calls unconditionally.
- Remove the import of jjrn_format_session_message (line 16)

## Muster wiring

Update Tools/jjk/vov_veiled/src/jjrmu_muster.rs:
- Change jjrmu_run_muster from sync to async: pub async fn jjrmu_run_muster(args) -> i32
- Add vvc::vvcc_invitatory().await call at end of muster output
- Update dispatch in Tools/jjk/vov_veiled/src/jjrx_cli.rs: change Muster match arm from jjrmu_run_muster(args) to jjrmu_run_muster(args).await
- This follows the existing pattern: saddle is already async and dispatched with .await

No spec changes needed for the sync→async transformation — this is an internal implementation detail of the Rust binary, not a behavioral change visible to CLI users or documented in JJSA/VOS.

## JJK cleanup

Remove from JJK:
- Delete jjrn_format_session_message() from jjrn_notch.rs
- Delete Session variant from jjrn_ChalkMarker enum (update parse/code/as_str/requires_pace match arms)
- Delete jjrc_needs_session_probe() from jjrc_core.rs
- Update jjrch_chalk.rs: remove Session marker handling block AND the jjrn_format_session_message import

## Spec updates

JJSA (Tools/jjk/vov_veiled/JJS0-GallopsData.adoc):
- Remove s (session) from pace-level and heat-level action codes
- Remove session marker commit pattern example
- Replace "session history" with "steeplechase history" where redundant
- Add cross-reference: "Officium markers migrated to VVC INVITATORY (vvb:...:i:), see VOS0-VoxObscuraSpec.adoc"

vocjjmc_core.md (Tools/jjk/vov_veiled/vocjjmc_core.md):
- "3-50 sessions" → "3-50 officia" (or "3-50 work periods")
- "Multi-Session Discipline:" → "Multi-Officium Discipline:"
- "Multiple Claude sessions" → "Multiple Claude officia"
- "another session's work" → "another officium's work"
- "another session may be mid-work" → "another officium may be mid-work"
- Leave "installation session" (line 113) alone — different sense

VOSRP-probe.adoc (Tools/vok/vov_veiled/VOSRP-probe.adoc):
- "session markers" → "officium markers"

## Testing

Build and test: tt/vow-b.Build.sh && tt/vow-t.Test.sh

## Key files
- Tools/jjk/vov_veiled/src/jjrsd_saddle.rs (replace inline probe with vvc::vvcc_invitatory().await)
- Tools/jjk/vov_veiled/src/jjrmu_muster.rs (make async, add invitatory call)
- Tools/jjk/vov_veiled/src/jjrx_cli.rs (update Muster dispatch to .await)
- Tools/jjk/vov_veiled/src/jjrn_notch.rs (delete Session variant, delete format_session_message)
- Tools/jjk/vov_veiled/src/jjrc_core.rs (delete needs_session_probe)
- Tools/jjk/vov_veiled/src/jjrch_chalk.rs (remove Session handling + import)
- Tools/jjk/vov_veiled/JJS0-GallopsData.adoc (remove :s:, add cross-ref)
- Tools/jjk/vov_veiled/vocjjmc_core.md (session → officium, 5 occurrences)
- Tools/vok/vov_veiled/VOSRP-probe.adoc (session → officium)

**[260207-0825] bridled**

Wire saddle and muster to call vvc::invitatory(). Remove session marker from JJK. Rename surviving "session" references to "officium".

## Saddle wiring

Update Tools/jjk/vov_veiled/src/jjrsd_saddle.rs:
- Remove the inline session probe block (lines ~295-343 that call jjrc_needs_session_probe, vvcp_probe, jjrn_format_session_message, vvc::commit)
- Replace with: direct Rust async call to vvc::vvcc_invitatory(). Saddle is already async, so just .await it. The invitatory handles its own gap check, so saddle calls unconditionally.
- Remove the import of jjrn_format_session_message (line 16)

## Muster wiring

Update Tools/jjk/vov_veiled/src/jjrmu_muster.rs:
- Change jjrmu_run_muster from sync to async: pub async fn jjrmu_run_muster(args) -> i32
- Add vvc::vvcc_invitatory().await call at end of muster output
- Update dispatch in Tools/jjk/vov_veiled/src/jjrx_cli.rs: change Muster match arm from jjrmu_run_muster(args) to jjrmu_run_muster(args).await
- This follows the existing pattern: saddle is already async and dispatched with .await

No spec changes needed for the sync→async transformation — this is an internal implementation detail of the Rust binary, not a behavioral change visible to CLI users or documented in JJSA/VOS.

## JJK cleanup

Remove from JJK:
- Delete jjrn_format_session_message() from jjrn_notch.rs
- Delete Session variant from jjrn_ChalkMarker enum (update parse/code/as_str/requires_pace match arms)
- Delete jjrc_needs_session_probe() from jjrc_core.rs
- Update jjrch_chalk.rs: remove Session marker handling block AND the jjrn_format_session_message import

## Spec updates

JJSA (Tools/jjk/vov_veiled/JJS0-GallopsData.adoc):
- Remove s (session) from pace-level and heat-level action codes
- Remove session marker commit pattern example
- Replace "session history" with "steeplechase history" where redundant
- Add cross-reference: "Officium markers migrated to VVC INVITATORY (vvb:...:i:), see VOS0-VoxObscuraSpec.adoc"

vocjjmc_core.md (Tools/jjk/vov_veiled/vocjjmc_core.md):
- "3-50 sessions" → "3-50 officia" (or "3-50 work periods")
- "Multi-Session Discipline:" → "Multi-Officium Discipline:"
- "Multiple Claude sessions" → "Multiple Claude officia"
- "another session's work" → "another officium's work"
- "another session may be mid-work" → "another officium may be mid-work"
- Leave "installation session" (line 113) alone — different sense

VOSRP-probe.adoc (Tools/vok/vov_veiled/VOSRP-probe.adoc):
- "session markers" → "officium markers"

## Testing

Build and test: tt/vow-b.Build.sh && tt/vow-t.Test.sh

## Key files
- Tools/jjk/vov_veiled/src/jjrsd_saddle.rs (replace inline probe with vvc::vvcc_invitatory().await)
- Tools/jjk/vov_veiled/src/jjrmu_muster.rs (make async, add invitatory call)
- Tools/jjk/vov_veiled/src/jjrx_cli.rs (update Muster dispatch to .await)
- Tools/jjk/vov_veiled/src/jjrn_notch.rs (delete Session variant, delete format_session_message)
- Tools/jjk/vov_veiled/src/jjrc_core.rs (delete needs_session_probe)
- Tools/jjk/vov_veiled/src/jjrch_chalk.rs (remove Session handling + import)
- Tools/jjk/vov_veiled/JJS0-GallopsData.adoc (remove :s:, add cross-ref)
- Tools/jjk/vov_veiled/vocjjmc_core.md (session → officium, 5 occurrences)
- Tools/vok/vov_veiled/VOSRP-probe.adoc (session → officium)

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/jjk/vov_veiled/src/jjrsd_saddle.rs, Tools/jjk/vov_veiled/src/jjrmu_muster.rs, Tools/jjk/vov_veiled/src/jjrx_cli.rs, Tools/jjk/vov_veiled/src/jjrn_notch.rs, Tools/jjk/vov_veiled/src/jjrc_core.rs, Tools/jjk/vov_veiled/src/jjrch_chalk.rs, Tools/jjk/vov_veiled/JJS0-GallopsData.adoc, Tools/jjk/vov_veiled/vocjjmc_core.md, Tools/vok/vov_veiled/VOSRP-probe.adoc 9 files | Steps: 1. In jjrsd_saddle.rs delete the entire inline session probe block starting at the comment Check if session probe is needed through the closing brace before return 0, replace with single call vvc::vvcc_invitatory.await and ignore the result with if-let-Err eprintln warning -- also remove the import of jjrn_format_session_message and jjrc_timestamp_full 2. In jjrmu_muster.rs change jjrmu_run_muster signature from pub fn to pub async fn, add vvc::vvcc_invitatory.await call after the table print loop before return 0, ignore errors with if-let-Err eprintln warning 3. In jjrx_cli.rs change the Muster match arm from jjrmu_run_muster args to jjrmu_run_muster args .await 4. In jjrn_notch.rs delete jjrn_format_session_message function entirely -- also delete Session variant from jjrn_ChalkMarker enum and remove Session from all match arms in jjrn_parse and jjrn_code and jjrn_as_str and jjrn_requires_pace 5. In jjrc_core.rs delete jjrc_needs_session_probe function entirely 6. In jjrch_chalk.rs remove the entire Session marker handling block starting at if marker == ChalkMarker::Session through its closing brace -- also remove jjrn_format_session_message from the import line, and remove the stdin import io::BufRead if no longer used 7. In JJS0-GallopsData.adoc remove s session from action codes, remove session marker commit pattern example, add cross-reference note that officium markers migrated to VVC invitatory see VOS0-VoxObscuraSpec.adoc 8. In vocjjmc_core.md change 3-50 sessions to 3-50 officia, Multi-Session Discipline to Multi-Officium Discipline, Multiple Claude sessions to Multiple Claude officia, anothers sessions work to another officiums work, another session may be mid-work to another officium may be mid-work -- leave installation session unchanged 9. In VOSRP-probe.adoc change session markers to officium markers | Verify: tt/vow-b.Build.sh and tt/vow-t.Test.sh

**[260207-0804] rough**

Wire saddle and muster to call vvc::invitatory(). Remove session marker from JJK. Rename surviving "session" references to "officium".

## Saddle wiring

Update Tools/jjk/vov_veiled/src/jjrsd_saddle.rs:
- Remove the inline session probe block (lines ~295-343 that call jjrc_needs_session_probe, vvcp_probe, jjrn_format_session_message, vvc::commit)
- Replace with: direct Rust async call to vvc::vvcc_invitatory(). Saddle is already async, so just .await it. The invitatory handles its own gap check, so saddle calls unconditionally.
- Remove the import of jjrn_format_session_message (line 16)

## Muster wiring

Update Tools/jjk/vov_veiled/src/jjrmu_muster.rs:
- Change jjrmu_run_muster from sync to async: pub async fn jjrmu_run_muster(args) -> i32
- Add vvc::vvcc_invitatory().await call at end of muster output
- Update dispatch in Tools/jjk/vov_veiled/src/jjrx_cli.rs: change Muster match arm from jjrmu_run_muster(args) to jjrmu_run_muster(args).await
- This follows the existing pattern: saddle is already async and dispatched with .await

No spec changes needed for the sync→async transformation — this is an internal implementation detail of the Rust binary, not a behavioral change visible to CLI users or documented in JJSA/VOS.

## JJK cleanup

Remove from JJK:
- Delete jjrn_format_session_message() from jjrn_notch.rs
- Delete Session variant from jjrn_ChalkMarker enum (update parse/code/as_str/requires_pace match arms)
- Delete jjrc_needs_session_probe() from jjrc_core.rs
- Update jjrch_chalk.rs: remove Session marker handling block AND the jjrn_format_session_message import

## Spec updates

JJSA (Tools/jjk/vov_veiled/JJS0-GallopsData.adoc):
- Remove s (session) from pace-level and heat-level action codes
- Remove session marker commit pattern example
- Replace "session history" with "steeplechase history" where redundant
- Add cross-reference: "Officium markers migrated to VVC INVITATORY (vvb:...:i:), see VOS0-VoxObscuraSpec.adoc"

vocjjmc_core.md (Tools/jjk/vov_veiled/vocjjmc_core.md):
- "3-50 sessions" → "3-50 officia" (or "3-50 work periods")
- "Multi-Session Discipline:" → "Multi-Officium Discipline:"
- "Multiple Claude sessions" → "Multiple Claude officia"
- "another session's work" → "another officium's work"
- "another session may be mid-work" → "another officium may be mid-work"
- Leave "installation session" (line 113) alone — different sense

VOSRP-probe.adoc (Tools/vok/vov_veiled/VOSRP-probe.adoc):
- "session markers" → "officium markers"

## Testing

Build and test: tt/vow-b.Build.sh && tt/vow-t.Test.sh

## Key files
- Tools/jjk/vov_veiled/src/jjrsd_saddle.rs (replace inline probe with vvc::vvcc_invitatory().await)
- Tools/jjk/vov_veiled/src/jjrmu_muster.rs (make async, add invitatory call)
- Tools/jjk/vov_veiled/src/jjrx_cli.rs (update Muster dispatch to .await)
- Tools/jjk/vov_veiled/src/jjrn_notch.rs (delete Session variant, delete format_session_message)
- Tools/jjk/vov_veiled/src/jjrc_core.rs (delete needs_session_probe)
- Tools/jjk/vov_veiled/src/jjrch_chalk.rs (remove Session handling + import)
- Tools/jjk/vov_veiled/JJS0-GallopsData.adoc (remove :s:, add cross-ref)
- Tools/jjk/vov_veiled/vocjjmc_core.md (session → officium, 5 occurrences)
- Tools/vok/vov_veiled/VOSRP-probe.adoc (session → officium)

**[260207-0749] rough**

Wire saddle and muster to call vvc::invitatory(). Remove session marker from JJK. Rename surviving "session" references to "officium".

Implementation:
1. Update Tools/jjk/vov_veiled/src/jjrsd_saddle.rs:
   - Remove the inline session probe block (lines ~295-343 that call jjrc_needs_session_probe, vvcp_probe, jjrn_format_session_message, vvc::commit)
   - Replace with: direct Rust async call to vvc::invitatory(). The invitatory handles its own gap check, so saddle calls unconditionally.
2. Update Tools/jjk/vov_veiled/src/jjrmu_muster.rs:
   - Add vvc::invitatory() call at end of muster output (same pattern as saddle)
   - Note: muster is currently sync — will need async or block_on wrapper
3. Remove from JJK:
   - Delete jjrn_format_session_message() from jjrn_notch.rs
   - Delete Session variant from jjrn_ChalkMarker enum (and update parse/code/as_str/requires_pace match arms)
   - Delete jjrc_needs_session_probe() from jjrc_core.rs
   - Update jjrch_chalk.rs: remove Session marker handling
4. Update JJSA spec (Tools/jjk/vov_veiled/JJS0-GallopsData.adoc):
   - Remove s (session) from pace-level and heat-level action codes
   - Remove session marker commit pattern example
   - Replace "session history" with "steeplechase history" where redundant
   - Add cross-reference: "Officium markers migrated to VVC INVITATORY (vvb:...:i:), see VOS0-VoxObscuraSpec.adoc"
5. Update vocjjmc_core.md (Tools/jjk/vov_veiled/vocjjmc_core.md):
   - "3-50 sessions" → "3-50 officia" (or "3-50 work periods")
   - "Multi-Session Discipline:" → "Multi-Officium Discipline:"
   - "Multiple Claude sessions" → "Multiple Claude officia"
   - "another session's work" → "another officium's work"
   - "another session may be mid-work" → "another officium may be mid-work"
   - Leave "installation session" (line 113) alone — different sense (Claude Code's session)
6. Update VOSRP-probe.adoc (Tools/vok/vov_veiled/VOSRP-probe.adoc):
   - "session markers" → "officium markers"
7. Build and test: tt/vow-b.Build.sh && tt/vow-t.Test.sh

Key files:
- Tools/jjk/vov_veiled/src/jjrsd_saddle.rs (replace inline probe with vvc::invitatory())
- Tools/jjk/vov_veiled/src/jjrmu_muster.rs (add vvc::invitatory() call)
- Tools/jjk/vov_veiled/src/jjrn_notch.rs (delete Session variant, delete format_session_message)
- Tools/jjk/vov_veiled/src/jjrc_core.rs (delete needs_session_probe)
- Tools/jjk/vov_veiled/src/jjrch_chalk.rs (remove Session handling)
- Tools/jjk/vov_veiled/JJS0-GallopsData.adoc (remove :s:, add cross-ref)
- Tools/jjk/vov_veiled/vocjjmc_core.md (session → officium, 5 occurrences)
- Tools/vok/vov_veiled/VOSRP-probe.adoc (session markers → officium markers)

**[260207-0734] rough**

Wire saddle and muster to call vvc::invitatory(). Remove session marker from JJK.

Implementation:
1. Update Tools/jjk/vov_veiled/src/jjrsd_saddle.rs:
   - Remove the inline session probe block (lines ~295-343 that call jjrc_needs_session_probe, vvcp_probe, jjrn_format_session_message, vvc::commit)
   - Replace with: direct Rust async call to vvc::invitatory() (NOT subprocess). The invitatory handles its own gap check, so saddle calls unconditionally.
2. Update Tools/jjk/vov_veiled/src/jjrmu_muster.rs:
   - Add vvc::invitatory() call at end of muster output (same pattern as saddle)
   - Note: muster is currently sync — will need async or block_on wrapper
3. Remove from JJK:
   - Delete jjrn_format_session_message() from jjrn_notch.rs
   - Delete Session variant from jjrn_ChalkMarker enum (and update parse/code/as_str/requires_pace match arms)
   - Delete jjrc_needs_session_probe() from jjrc_core.rs
   - Update jjrch_chalk.rs: remove Session marker handling
4. Update JJSA spec (Tools/jjk/vov_veiled/JJS0-GallopsData.adoc):
   - Remove s (session) from pace-level and heat-level action codes
   - Remove session marker commit pattern example
   - Add cross-reference note: "Session markers migrated to VVC INVITATORY (vvb:...:i:), see VOS0-VoxObscuraSpec.adoc"
5. Build and test: tt/vow-b.Build.sh && tt/vow-t.Test.sh

Key files:
- Tools/jjk/vov_veiled/src/jjrsd_saddle.rs (replace inline probe with vvc::invitatory())
- Tools/jjk/vov_veiled/src/jjrmu_muster.rs (add vvc::invitatory() call)
- Tools/jjk/vov_veiled/src/jjrn_notch.rs (delete Session variant, delete format_session_message)
- Tools/jjk/vov_veiled/src/jjrc_core.rs (delete needs_session_probe)
- Tools/jjk/vov_veiled/src/jjrch_chalk.rs (remove Session handling)
- Tools/jjk/vov_veiled/JJS0-GallopsData.adoc (remove :s: documentation)

**[260207-0717] rough**

Wire saddle and muster to call vvx invitatory. Remove session marker from JJK.

Implementation:
1. Update Tools/jjk/vov_veiled/src/jjrsd_saddle.rs:
   - Remove the inline session probe block (lines ~295-343 that call jjrc_needs_session_probe, vvcp_probe, jjrn_format_session_message, vvc::commit)
   - Replace with: call vvx invitatory (either as subprocess or direct Rust call to the new VVC function)
   - The invitatory handles its own gap check, so saddle just calls it unconditionally
2. Update jjx_muster (Tools/jjk/vov_veiled/src/jjrm_muster.rs or similar):
   - Add invitatory call at the end of muster output (same pattern as saddle)
3. Remove from JJK:
   - Delete jjrn_format_session_message() from jjrn_notch.rs
   - Delete Session variant from jjrn_ChalkMarker enum
   - Delete jjrc_needs_session_probe() from jjrc_core.rs
   - Update any chalk command code that handles Session marker
4. Update JJSA spec (Tools/jjk/vov_veiled/JJS0-GallopsData.adoc):
   - Remove s (session) from action codes
   - Remove session marker commit pattern
   - Add cross-reference note: "Session markers migrated to VVC INVITATORY (vvb:...:i:)"
5. Build and test: tt/vow-b.Build.sh && tt/vow-t.Test.sh

Key files:
- Tools/jjk/vov_veiled/src/jjrsd_saddle.rs (remove inline probe, add invitatory call)
- Tools/jjk/vov_veiled/src/jjrm_muster.rs (add invitatory call)
- Tools/jjk/vov_veiled/src/jjrn_notch.rs (delete Session, delete format_session_message)
- Tools/jjk/vov_veiled/src/jjrc_core.rs (delete needs_session_probe)
- Tools/jjk/vov_veiled/src/jjrch_chalk.rs (remove Session handling)
- Tools/jjk/vov_veiled/JJS0-GallopsData.adoc (remove :s: documentation)

### probe-opus-as-interpreter (₢AWAAL) [complete]

**[260207-1242] complete**

Refactor vvcp_probe (Tools/vvc/src/vvcp_probe.rs) so that opus interprets raw haiku/sonnet output into structured XML, instead of trusting raw stdout directly.

Current problem: probe_model_tier spawns `claude -p --model {tier} -- "Report your exact model ID string only"` and trims stdout. This fails in some environments (e.g., work machines) where haiku/sonnet return verbose or unexpected output that doesn't parse as a bare model ID.

XML element prefixes (minted under vvp → vvpx):
- <vvpxh_haiku> — haiku model ID
- <vvpxs_sonnet> — sonnet model ID
- <vvpxo_opus> — opus model ID

Prefix tree:
  vvp (probe — non-terminal)
  └── vvpx (probe XML — non-terminal)
      ├── vvpxh  haiku
      ├── vvpxs  sonnet
      └── vvpxo  opus

Host and platform are assembled by Rust directly (uname, hostname) — no XML, no opus involvement.

Changes:
1. Probe haiku and sonnet in parallel (as today), collect raw stdout
2. Write all raw probe results to a file in BUD_TEMP_DIR (requires threading BUD_TEMP_DIR through to the probe — likely via env var or parameter)
3. Feed raw haiku and sonnet output to opus, asking opus to: self-report its own model ID, interpret the other two, return XML with <vvpxh_haiku>, <vvpxs_sonnet>, <vvpxo_opus> elements
4. Parse opus response with regex: Regex::new(r"<vvpxh_haiku>(.*?)</vvpxh_haiku>") etc. — no XML parser dependency needed
5. Assemble final probe output: three model ID lines from XML + host/platform from Rust
6. Output appears in officium commit body (vvb:...:i: OFFICIUM ...)
7. Fallback: if opus XML parsing fails, fall back to "unavailable" for affected tiers

Testing: Lower the gap threshold in vvcp_needs_officium (now in VVC per ₢AWAAO) to trigger without waiting an hour. Restore after verification.

Key files:
- Tools/vvc/src/vvcp_probe.rs (probe implementation)

**[260207-0839] bridled**

Refactor vvcp_probe (Tools/vvc/src/vvcp_probe.rs) so that opus interprets raw haiku/sonnet output into structured XML, instead of trusting raw stdout directly.

Current problem: probe_model_tier spawns `claude -p --model {tier} -- "Report your exact model ID string only"` and trims stdout. This fails in some environments (e.g., work machines) where haiku/sonnet return verbose or unexpected output that doesn't parse as a bare model ID.

XML element prefixes (minted under vvp → vvpx):
- <vvpxh_haiku> — haiku model ID
- <vvpxs_sonnet> — sonnet model ID
- <vvpxo_opus> — opus model ID

Prefix tree:
  vvp (probe — non-terminal)
  └── vvpx (probe XML — non-terminal)
      ├── vvpxh  haiku
      ├── vvpxs  sonnet
      └── vvpxo  opus

Host and platform are assembled by Rust directly (uname, hostname) — no XML, no opus involvement.

Changes:
1. Probe haiku and sonnet in parallel (as today), collect raw stdout
2. Write all raw probe results to a file in BUD_TEMP_DIR (requires threading BUD_TEMP_DIR through to the probe — likely via env var or parameter)
3. Feed raw haiku and sonnet output to opus, asking opus to: self-report its own model ID, interpret the other two, return XML with <vvpxh_haiku>, <vvpxs_sonnet>, <vvpxo_opus> elements
4. Parse opus response with regex: Regex::new(r"<vvpxh_haiku>(.*?)</vvpxh_haiku>") etc. — no XML parser dependency needed
5. Assemble final probe output: three model ID lines from XML + host/platform from Rust
6. Output appears in officium commit body (vvb:...:i: OFFICIUM ...)
7. Fallback: if opus XML parsing fails, fall back to "unavailable" for affected tiers

Testing: Lower the gap threshold in vvcp_needs_officium (now in VVC per ₢AWAAO) to trigger without waiting an hour. Restore after verification.

Key files:
- Tools/vvc/src/vvcp_probe.rs (probe implementation)

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/vvc/src/vvcp_probe.rs 1 file | Steps: 1. Define consts: VVCP_RAW_HAIKU_FILE vvcp_raw_haiku.txt, VVCP_RAW_SONNET_FILE vvcp_raw_sonnet.txt, VVCP_RAW_OPUS_FILE vvcp_raw_opus.txt, VVCP_ELEMENT_HAIKU vvpxh_haiku, VVCP_ELEMENT_SONNET vvpxs_sonnet, VVCP_ELEMENT_OPUS vvpxo_opus, VVCP_BUD_TEMP_DIR_VAR BUD_TEMP_DIR 2. Refactor vvcp_probe: spawn haiku and sonnet probes in parallel as today collecting raw stdout strings 3. Read BUD_TEMP_DIR from env via VVCP_BUD_TEMP_DIR_VAR -- if set, write raw haiku stdout to VVCP_RAW_HAIKU_FILE and raw sonnet stdout to VVCP_RAW_SONNET_FILE in that dir before any parsing 4. Build opus prompt from consts: the prompt is -- Report your own model ID. Then extract the Claude model ID from each raw output below. raw_haiku tags around haiku_stdout, raw_sonnet tags around sonnet_stdout. Respond with exactly: element tags built from VVCP_ELEMENT_OPUS VVCP_ELEMENT_HAIKU VVCP_ELEMENT_SONNET -- use format! to assemble prompt so element tag names come from consts not literals 5. Invoke claude -p --model opus with no --system-prompt flag, passing assembled prompt -- no special flags, just default context 6. If BUD_TEMP_DIR set, write raw opus response to VVCP_RAW_OPUS_FILE before parsing 7. Parse opus response with three regexes built from consts: format regex pattern from each VVCP_ELEMENT const as opening-tag capture closing-tag -- if element missing or empty after parse, substitute unavailable 8. Delete the old probe_model_tier function that probed opus directly -- host and platform functions unchanged 9. Assemble final 5-line output same as before: haiku sonnet opus host platform 10. Add unit tests to existing inline test module for XML regex parsing: test with clean XML, test with missing element returns unavailable, test with extra text around XML still extracts correctly | Verify: tt/vow-b.Build.sh and tt/vow-t.Test.sh

**[260207-0749] rough**

Refactor vvcp_probe (Tools/vvc/src/vvcp_probe.rs) so that opus interprets raw haiku/sonnet output into structured XML, instead of trusting raw stdout directly.

Current problem: probe_model_tier spawns `claude -p --model {tier} -- "Report your exact model ID string only"` and trims stdout. This fails in some environments (e.g., work machines) where haiku/sonnet return verbose or unexpected output that doesn't parse as a bare model ID.

XML element prefixes (minted under vvp → vvpx):
- <vvpxh_haiku> — haiku model ID
- <vvpxs_sonnet> — sonnet model ID
- <vvpxo_opus> — opus model ID

Prefix tree:
  vvp (probe — non-terminal)
  └── vvpx (probe XML — non-terminal)
      ├── vvpxh  haiku
      ├── vvpxs  sonnet
      └── vvpxo  opus

Host and platform are assembled by Rust directly (uname, hostname) — no XML, no opus involvement.

Changes:
1. Probe haiku and sonnet in parallel (as today), collect raw stdout
2. Write all raw probe results to a file in BUD_TEMP_DIR (requires threading BUD_TEMP_DIR through to the probe — likely via env var or parameter)
3. Feed raw haiku and sonnet output to opus, asking opus to: self-report its own model ID, interpret the other two, return XML with <vvpxh_haiku>, <vvpxs_sonnet>, <vvpxo_opus> elements
4. Parse opus response with regex: Regex::new(r"<vvpxh_haiku>(.*?)</vvpxh_haiku>") etc. — no XML parser dependency needed
5. Assemble final probe output: three model ID lines from XML + host/platform from Rust
6. Output appears in officium commit body (vvb:...:i: OFFICIUM ...)
7. Fallback: if opus XML parsing fails, fall back to "unavailable" for affected tiers

Testing: Lower the gap threshold in vvcp_needs_officium (now in VVC per ₢AWAAO) to trigger without waiting an hour. Restore after verification.

Key files:
- Tools/vvc/src/vvcp_probe.rs (probe implementation)

**[260207-0744] rough**

Refactor vvcp_probe (Tools/vvc/src/vvcp_probe.rs) so that opus interprets raw haiku/sonnet output into structured XML, instead of trusting raw stdout directly.

Current problem: probe_model_tier spawns `claude -p --model {tier} -- "Report your exact model ID string only"` and trims stdout. This fails in some environments (e.g., work machines) where haiku/sonnet return verbose or unexpected output that doesn't parse as a bare model ID.

XML element prefixes (minted under vvp → vvpx):
- <vvpxh_haiku> — haiku model ID
- <vvpxs_sonnet> — sonnet model ID
- <vvpxo_opus> — opus model ID

Prefix tree:
  vvp (probe — non-terminal)
  └── vvpx (probe XML — non-terminal)
      ├── vvpxh  haiku
      ├── vvpxs  sonnet
      └── vvpxo  opus

Host and platform are assembled by Rust directly (uname, hostname) — no XML, no opus involvement.

Changes:
1. Probe haiku and sonnet in parallel (as today), collect raw stdout
2. Write all raw probe results to a file in BUD_TEMP_DIR (requires threading BUD_TEMP_DIR through to the probe — likely via env var or parameter)
3. Feed raw haiku and sonnet output to opus, asking opus to: self-report its own model ID, interpret the other two, return XML with <vvpxh_haiku>, <vvpxs_sonnet>, <vvpxo_opus> elements
4. Parse opus response with regex: Regex::new(r"<vvpxh_haiku>(.*?)</vvpxh_haiku>") etc. — no XML parser dependency needed
5. Assemble final probe output: three model ID lines from XML + host/platform from Rust
6. Fallback: if opus XML parsing fails, fall back to "unavailable" for affected tiers

Testing: Lower the gap threshold in vvcp_needs_session (now in VVC per ₢AWAAO) to trigger without waiting an hour. Restore after verification.

Key files:
- Tools/vvc/src/vvcp_probe.rs (probe implementation)

**[260206-2248] rough**

Refactor vvcp_probe (Tools/vvc/src/vvcp_probe.rs) so that opus interprets raw haiku/sonnet output into structured model-ID JSON, instead of trusting raw stdout directly.

Current problem: probe_model_tier spawns `claude -p --model {tier} -- "Report your exact model ID string only"` and trims stdout. This fails in some environments (e.g., work machines) where haiku/sonnet return verbose or unexpected output that doesn't parse as a bare model ID.

Changes:
1. Make haiku/sonnet prompts more typical/robust (less likely to get "unavailable" from environments that restrict novel queries)
2. Collect raw stdout from all three model tiers
3. Write all raw probe results to a file in BUD_TEMP_DIR (requires threading BUD_TEMP_DIR through to the probe — likely via env var or parameter)
4. Feed the raw haiku and sonnet output to the opus query, asking opus to extract model IDs and return structured JSON that jjx can place into the session commit body
5. Opus already returns its own model ID, so it self-reports while also interpreting the other two

Testing: Lower the session gap threshold in jjrc_needs_session_probe (jjrc_core.rs:91, currently 3600s) to a small value (e.g., 5s) so the probe triggers without waiting an hour. Restore after verification.

Key files:
- Tools/vvc/src/vvcp_probe.rs (probe implementation)
- Tools/jjk/vov_veiled/src/jjrc_core.rs (session gap threshold)
- Tools/jjk/vov_veiled/src/jjrsd_saddle.rs (calls vvcp_probe, consumes result)

### test-officium-threshold (₢AWAAQ) [complete]

**[260207-1313] complete**

Manual integration test for officium/invitatory mechanism.

Lower VVCP_OFFICIUM_GAP_SECS to a small value (e.g., 5 seconds), then run jjx_orient, jjx_list, and jjc-heat-groom to verify:
1. jjx_orient (saddle) triggers an invitatory commit (vvb:...:i: OFFICIUM ...) when gap exceeded
2. jjx_list (muster) triggers an invitatory commit when gap exceeded
3. A second immediate call does NOT trigger (gap not exceeded)
4. The commit body contains probe data (model IDs, host, platform)
5. git log shows the correct vvb: branded format

After verification, restore VVCP_OFFICIUM_GAP_SECS to 3600.

Key files:
- Tools/vvc/src/vvcp_probe.rs (lower/restore VVCP_OFFICIUM_GAP_SECS)

Depends on: ₢AWAAO (add-vvx-invitatory) and ₢AWAAP (wire-invitatory-triggers) must be complete first.

**[260207-0853] rough**

Manual integration test for officium/invitatory mechanism.

Lower VVCP_OFFICIUM_GAP_SECS to a small value (e.g., 5 seconds), then run jjx_orient, jjx_list, and jjc-heat-groom to verify:
1. jjx_orient (saddle) triggers an invitatory commit (vvb:...:i: OFFICIUM ...) when gap exceeded
2. jjx_list (muster) triggers an invitatory commit when gap exceeded
3. A second immediate call does NOT trigger (gap not exceeded)
4. The commit body contains probe data (model IDs, host, platform)
5. git log shows the correct vvb: branded format

After verification, restore VVCP_OFFICIUM_GAP_SECS to 3600.

Key files:
- Tools/vvc/src/vvcp_probe.rs (lower/restore VVCP_OFFICIUM_GAP_SECS)

Depends on: ₢AWAAO (add-vvx-invitatory) and ₢AWAAP (wire-invitatory-triggers) must be complete first.

### fix-scout-utf8-panic (₢AWAAK) [complete]

**[260206-2237] complete**

Fix jjx_scout panic on UTF-8 boundary in long spec text.

Scout panics at jjrsc_scout.rs:143 with "byte index 1359 is not a char boundary; it is inside '—'" when truncating long spec text for context display. The substring operation uses byte indexing on a UTF-8 string containing multi-byte characters (em-dash, currency symbols).

Fix: use char-boundary-safe truncation (e.g., floor to nearest char boundary before slicing, or use char_indices).

Files: Tools/jjk/vov_veiled/src/jjrsc_scout.rs

Acceptance: `jjx_scout spec` completes without panic. tt/vow-b.Build.sh compiles, tt/vow-t.Test.sh passes.

**[260206-2236] abandoned**

Fix jjx_scout panic on UTF-8 boundary in long spec text.

Scout panics at jjrsc_scout.rs:143 with "byte index 1359 is not a char boundary; it is inside '—'" when truncating long spec text for context display. The substring operation uses byte indexing on a UTF-8 string containing multi-byte characters (em-dash, currency symbols).

Fix: use char-boundary-safe truncation (e.g., floor to nearest char boundary before slicing, or use char_indices).

Files: Tools/jjk/vov_veiled/src/jjrsc_scout.rs

Acceptance: `jjx_scout spec` completes without panic. tt/vow-b.Build.sh compiles, tt/vow-t.Test.sh passes.

**[260206-2223] rough**

Fix jjx_scout panic on UTF-8 boundary in long spec text.

Scout panics at jjrsc_scout.rs:143 with "byte index 1359 is not a char boundary; it is inside '—'" when truncating long spec text for context display. The substring operation uses byte indexing on a UTF-8 string containing multi-byte characters (em-dash, currency symbols).

Fix: use char-boundary-safe truncation (e.g., floor to nearest char boundary before slicing, or use char_indices).

Files: Tools/jjk/vov_veiled/src/jjrsc_scout.rs

Acceptance: `jjx_scout spec` completes without panic. tt/vow-b.Build.sh compiles, tt/vow-t.Test.sh passes.

### rename-jjrx-cli-strings (₢AWAAA) [complete]

**[260207-1322] complete**

Rename jjx_ command strings in Rust CLI source.

Two kinds of changes:

**Command renames** — Update all #[command(name = "jjx_*")] attributes in jjrx_cli.rs to use boring names from paddock mapping table. ~18 string edits (22 minus tally which is being split).

**Tally split** — Replace jjx_tally with four single-purpose commands:
- jjx_revise_docket CORONET < stdin — update docket text, carry forward state/silks
- jjx_arm CORONET < stdin — set state=bridled, stdin is the warrant
- jjx_relabel CORONET --silks "name" — rename, carry forward docket/state
- jjx_drop CORONET — set state=abandoned
All four create a new tack internally. Each carries forward unchanged fields from tacks[0].

**Flag rename** — Rename --full to --detail in jjrpd_parade.rs ParadeArgs struct.

**Presentation labels** — In parade output (jjrpd_parade.rs), change:
- "Spec:" / spec text labels → "Docket:"
- "Direction:" → "Warrant:"

Files:
- Tools/jjk/vov_veiled/src/jjrx_cli.rs (command names, tally split registration)
- Tools/jjk/vov_veiled/src/jjrtl_tally.rs (refactor into 4 entry points)
- Tools/jjk/vov_veiled/src/jjrpd_parade.rs (--full→--detail, label renames)
- Tools/jjk/vov_veiled/src/jjrsd_saddle.rs (label renames: Spec→Docket, Direction→Warrant)

Acceptance: tt/vow-b.Build.sh compiles. jjx_show, jjx_revise_docket, jjx_arm, jjx_relabel, jjx_drop all respond to --help. jjx_tally and jjx_parade produce "unknown command." tt/vow-t.Test.sh passes.

**[260207-0841] bridled**

Rename jjx_ command strings in Rust CLI source.

Two kinds of changes:

**Command renames** — Update all #[command(name = "jjx_*")] attributes in jjrx_cli.rs to use boring names from paddock mapping table. ~18 string edits (22 minus tally which is being split).

**Tally split** — Replace jjx_tally with four single-purpose commands:
- jjx_revise_docket CORONET < stdin — update docket text, carry forward state/silks
- jjx_arm CORONET < stdin — set state=bridled, stdin is the warrant
- jjx_relabel CORONET --silks "name" — rename, carry forward docket/state
- jjx_drop CORONET — set state=abandoned
All four create a new tack internally. Each carries forward unchanged fields from tacks[0].

**Flag rename** — Rename --full to --detail in jjrpd_parade.rs ParadeArgs struct.

**Presentation labels** — In parade output (jjrpd_parade.rs), change:
- "Spec:" / spec text labels → "Docket:"
- "Direction:" → "Warrant:"

Files:
- Tools/jjk/vov_veiled/src/jjrx_cli.rs (command names, tally split registration)
- Tools/jjk/vov_veiled/src/jjrtl_tally.rs (refactor into 4 entry points)
- Tools/jjk/vov_veiled/src/jjrpd_parade.rs (--full→--detail, label renames)
- Tools/jjk/vov_veiled/src/jjrsd_saddle.rs (label renames: Spec→Docket, Direction→Warrant)

Acceptance: tt/vow-b.Build.sh compiles. jjx_show, jjx_revise_docket, jjx_arm, jjx_relabel, jjx_drop all respond to --help. jjx_tally and jjx_parade produce "unknown command." tt/vow-t.Test.sh passes.

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/jjk/vov_veiled/src/jjrx_cli.rs, Tools/jjk/vov_veiled/src/jjrtl_tally.rs, Tools/jjk/vov_veiled/src/jjrpd_parade.rs, Tools/jjk/vov_veiled/src/jjrsd_saddle.rs 4 files | Steps: 1. In jjrx_cli.rs rename all command name attributes per paddock mapping: jjx_parade to jjx_show, jjx_muster to jjx_list, jjx_nominate to jjx_create, jjx_slate to jjx_enroll, jjx_rail to jjx_reorder, jjx_furlough to jjx_alter, jjx_notch to jjx_record, jjx_wrap to jjx_close, jjx_rein to jjx_log, jjx_scout to jjx_search, jjx_retire to jjx_archive, jjx_restring to jjx_transfer, jjx_garland to jjx_continue, jjx_chalk to jjx_mark, jjx_curry to jjx_paddock, jjx_draft to jjx_relocate, jjx_saddle to jjx_orient, jjx_get_spec to jjx_get_brief -- keep jjx_get_coronets jjx_landing jjx_validate unchanged 2. In jjrx_cli.rs remove the Tally variant and its match arm -- replace with four new variants: ReviseDocket and Arm and Relabel and Drop -- each with its own Args struct and handler dispatch 3. In jjrtl_tally.rs create four new public functions: jjrtl_run_revise_docket takes coronet and reads stdin for new text, calls jjrg_tally with state=None direction=None silks=None text=stdin -- jjrtl_run_arm takes coronet and reads stdin for warrant text, calls jjrg_tally with state=Some Bridled direction=stdin-as-warrant text=None silks=None then creates B commit same as current bridling flow -- jjrtl_run_relabel takes coronet and --silks flag, calls jjrg_tally with state=None direction=None text=None silks=Some -- jjrtl_run_drop takes coronet only, calls jjrg_tally with state=Some Abandoned direction=None text=None silks=None -- each function follows the same lock acquire, load gallops, call jjrg_tally, persist, print hash pattern from existing jjrtl_run_tally -- keep jjrtl_run_tally as deprecated or delete it 4. In jjrpd_parade.rs rename the full field from full to detail in ParadeArgs and update all references -- change output label Spec: to Docket: and Direction: to Warrant: in all println statements 5. In jjrsd_saddle.rs change output labels Spec: to Docket: and Direction: to Warrant: in the println statements 6. Also update the jjrx_is_jjk_command function if it has a command name list that needs updating | Verify: tt/vow-b.Build.sh and tt/vow-t.Test.sh -- also verify jjx_show --help and jjx_revise_docket --help and jjx_arm --help and jjx_relabel --help and jjx_drop --help all respond

**[260206-2232] rough**

Rename jjx_ command strings in Rust CLI source.

Two kinds of changes:

**Command renames** — Update all #[command(name = "jjx_*")] attributes in jjrx_cli.rs to use boring names from paddock mapping table. ~18 string edits (22 minus tally which is being split).

**Tally split** — Replace jjx_tally with four single-purpose commands:
- jjx_revise_docket CORONET < stdin — update docket text, carry forward state/silks
- jjx_arm CORONET < stdin — set state=bridled, stdin is the warrant
- jjx_relabel CORONET --silks "name" — rename, carry forward docket/state
- jjx_drop CORONET — set state=abandoned
All four create a new tack internally. Each carries forward unchanged fields from tacks[0].

**Flag rename** — Rename --full to --detail in jjrpd_parade.rs ParadeArgs struct.

**Presentation labels** — In parade output (jjrpd_parade.rs), change:
- "Spec:" / spec text labels → "Docket:"
- "Direction:" → "Warrant:"

Files:
- Tools/jjk/vov_veiled/src/jjrx_cli.rs (command names, tally split registration)
- Tools/jjk/vov_veiled/src/jjrtl_tally.rs (refactor into 4 entry points)
- Tools/jjk/vov_veiled/src/jjrpd_parade.rs (--full→--detail, label renames)
- Tools/jjk/vov_veiled/src/jjrsd_saddle.rs (label renames: Spec→Docket, Direction→Warrant)

Acceptance: tt/vow-b.Build.sh compiles. jjx_show, jjx_revise_docket, jjx_arm, jjx_relabel, jjx_drop all respond to --help. jjx_tally and jjx_parade produce "unknown command." tt/vow-t.Test.sh passes.

**[260206-2209] rough**

Rename jjx_ command strings in jjrx_cli.rs.

Update all #[command(name = "jjx_*")] attributes to use the new boring names from the paddock mapping table. This is ~22 string edits in one file.

Also rename the --full flag to --detail in jjrpd_parade.rs (ParadeArgs struct). This is a flag rename, same kind of edit — the flag means "show detailed view" not "show full view."

Files:
- Tools/jjk/vov_veiled/src/jjrx_cli.rs (command names)
- Tools/jjk/vov_veiled/src/jjrpd_parade.rs (--full → --detail)

Acceptance: `tt/vow-b.Build.sh` compiles, `./tt/vvw-r.RunVVX.sh jjx_show --help` works, `jjx_show AH --detail` works.

**[260206-2202] rough**

Rename jjx_ command strings in jjrx_cli.rs.

Update all #[command(name = "jjx_*")] attributes to use the new boring names from the paddock mapping table. This is ~22 string edits in one file.

Files: Tools/jjk/vov_veiled/src/jjrx_cli.rs

Acceptance: `tt/vow-b.Build.sh` compiles, `./tt/vvw-r.RunVVX.sh jjx_show --help` works.

### split-tally-into-primitives (₢AWAAJ) [abandoned]

**[260206-2234] abandoned**

Split jjx_tally (jjx_revise) into four single-purpose commands.

Current jjx_tally is multipurpose: spec revision, rename, bridle, abandon — with conditional flag validity that causes LLM misfires. Split into:

1. jjx_revise CORONET < stdin — update spec text, carry forward state/silks
2. jjx_relabel CORONET --silks "name" — rename, carry forward text/state
3. jjx_arm CORONET --direction "..." < stdin — set state=bridled + direction + spec
4. jjx_drop CORONET — set state=abandoned

All four create a new tack (same underlying mechanism). Each has a minimal, unambiguous signature.

Files:
- Tools/jjk/vov_veiled/src/jjrtl_tally.rs — refactor into separate handler functions
- Tools/jjk/vov_veiled/src/jjrx_cli.rs — register 4 new subcommands, remove jjx_tally
- May need new source files for each command, or keep in jjrtl_tally.rs with separate entry points

Delete jjx_tally entirely — no backwards compatibility.

Acceptance: Each command works independently. jjx_tally produces "unknown command." tt/vow-b.Build.sh compiles, tt/vow-t.Test.sh passes.

**[260206-2212] rough**

Split jjx_tally (jjx_revise) into four single-purpose commands.

Current jjx_tally is multipurpose: spec revision, rename, bridle, abandon — with conditional flag validity that causes LLM misfires. Split into:

1. jjx_revise CORONET < stdin — update spec text, carry forward state/silks
2. jjx_relabel CORONET --silks "name" — rename, carry forward text/state
3. jjx_arm CORONET --direction "..." < stdin — set state=bridled + direction + spec
4. jjx_drop CORONET — set state=abandoned

All four create a new tack (same underlying mechanism). Each has a minimal, unambiguous signature.

Files:
- Tools/jjk/vov_veiled/src/jjrtl_tally.rs — refactor into separate handler functions
- Tools/jjk/vov_veiled/src/jjrx_cli.rs — register 4 new subcommands, remove jjx_tally
- May need new source files for each command, or keep in jjrtl_tally.rs with separate entry points

Delete jjx_tally entirely — no backwards compatibility.

Acceptance: Each command works independently. jjx_tally produces "unknown command." tt/vow-b.Build.sh compiles, tt/vow-t.Test.sh passes.

### update-jjsa-operation-names (₢AWAAB) [complete]

**[260207-1329] complete**

Update JJS0-GallopsData.adoc operation names and vocabulary.

**Operation renames** — Change :jjdo_*: attribute display text to new boring names per paddock mapping. E.g., :jjdo_parade: display text becomes "jjx_show".

**Tally split** — Remove :jjdo_tally: and add four new operations:
- :jjdo_revise_docket: → jjx_revise_docket
- :jjdo_arm: → jjx_arm
- :jjdo_relabel: → jjx_relabel
- :jjdo_drop: → jjx_drop

**Presentation vocabulary** — Add notes that:
- "docket" is the presentation term for the tack text field (schema stays "text")
- "warrant" is the presentation term for the tack direction field (schema stays "direction")
- These are display/CLI terms, not schema changes

No file renames — JJSC* spec filenames stay as-is.

Files: Tools/jjk/vov_veiled/JJS0-GallopsData.adoc

Acceptance: All :jjdo_*: attributes reference new command names. Docket/warrant vocabulary documented.

**[260207-0845] bridled**

Update JJS0-GallopsData.adoc operation names and vocabulary.

**Operation renames** — Change :jjdo_*: attribute display text to new boring names per paddock mapping. E.g., :jjdo_parade: display text becomes "jjx_show".

**Tally split** — Remove :jjdo_tally: and add four new operations:
- :jjdo_revise_docket: → jjx_revise_docket
- :jjdo_arm: → jjx_arm
- :jjdo_relabel: → jjx_relabel
- :jjdo_drop: → jjx_drop

**Presentation vocabulary** — Add notes that:
- "docket" is the presentation term for the tack text field (schema stays "text")
- "warrant" is the presentation term for the tack direction field (schema stays "direction")
- These are display/CLI terms, not schema changes

No file renames — JJSC* spec filenames stay as-is.

Files: Tools/jjk/vov_veiled/JJS0-GallopsData.adoc

Acceptance: All :jjdo_*: attributes reference new command names. Docket/warrant vocabulary documented.

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/jjk/vov_veiled/JJS0-GallopsData.adoc 1 file | Steps: 1. In the attribute mapping section, rename display text in each jjdo_ attribute per paddock mapping: jjdo_chalk display to jjx_mark, jjdo_draft to jjx_relocate, jjdo_furlough to jjx_alter, jjdo_garland to jjx_continue, jjdo_get_spec to jjx_get_brief, jjdo_muster to jjx_list, jjdo_nominate to jjx_create, jjdo_notch to jjx_record, jjdo_rail to jjx_reorder, jjdo_rein to jjx_log, jjdo_restring to jjx_transfer, jjdo_retire to jjx_archive, jjdo_saddle to jjx_orient, jjdo_scout to jjx_search, jjdo_slate to jjx_enroll -- keep jjdo_get_coronets jjdo_landing jjdo_validate display text unchanged 2. Remove jjdo_tally attribute and its anchor definition -- replace with four new attributes: jjdo_revise_docket with display jjx_revise_docket, jjdo_arm with display jjx_arm, jjdo_relabel with display jjx_relabel, jjdo_drop with display jjx_drop -- add corresponding anchor definitions following the existing pattern for each new operation describing what it does 3. Also rename anchor display text throughout the document body wherever these operations are referenced -- search for all occurrences of the old display text in cross-references and update 4. Add a vocabulary note section or paragraph documenting that docket is the presentation term for the tack text field and warrant is the presentation term for the tack direction field and that these are display and CLI terms not schema changes -- schema fields text and direction remain unchanged and are deferred to a future heat | Verify: grep for old command names like jjx_tally jjx_parade jjx_muster etc to confirm none remain in display text

**[260206-2232] rough**

Update JJS0-GallopsData.adoc operation names and vocabulary.

**Operation renames** — Change :jjdo_*: attribute display text to new boring names per paddock mapping. E.g., :jjdo_parade: display text becomes "jjx_show".

**Tally split** — Remove :jjdo_tally: and add four new operations:
- :jjdo_revise_docket: → jjx_revise_docket
- :jjdo_arm: → jjx_arm
- :jjdo_relabel: → jjx_relabel
- :jjdo_drop: → jjx_drop

**Presentation vocabulary** — Add notes that:
- "docket" is the presentation term for the tack text field (schema stays "text")
- "warrant" is the presentation term for the tack direction field (schema stays "direction")
- These are display/CLI terms, not schema changes

No file renames — JJSC* spec filenames stay as-is.

Files: Tools/jjk/vov_veiled/JJS0-GallopsData.adoc

Acceptance: All :jjdo_*: attributes reference new command names. Docket/warrant vocabulary documented.

**[260206-2202] rough**

Update JJS0-GallopsData.adoc operation names.

Change :jjdo_*: attribute references and operation section headers to use new boring names. E.g., :jjdo_parade: becomes display text "jjx_show" instead of "jjx_parade".

Files: Tools/jjk/vov_veiled/JJS0-GallopsData.adoc

No file renames — JJSC* spec filenames stay as-is (PD, MU, etc. are stable internal codes).

Acceptance: All :jjdo_*: attributes reference the new command names.

### update-jjsc-spec-prose (₢AWAAC) [complete]

**[260207-1350] complete**

Update JJSC* individual spec files to reference new command names and vocabulary.

Each spec file mentions jjx_ command names in descriptions and examples. Update:
- Old command names → new boring names per paddock mapping
- "spec"/"specification" in pace context → "docket"
- "direction" in bridle context → "warrant"
- --full flag references → --detail
- jjx_tally references → appropriate new command (jjx_revise_docket, jjx_arm, jjx_relabel, jjx_drop)

Files: Tools/jjk/vov_veiled/JJSC*.adoc (all spec files)

Acceptance: No spec file references old jjx_ command names or old vocabulary in prose. grep confirms.

**[260207-0847] bridled**

Update JJSC* individual spec files to reference new command names and vocabulary.

Each spec file mentions jjx_ command names in descriptions and examples. Update:
- Old command names → new boring names per paddock mapping
- "spec"/"specification" in pace context → "docket"
- "direction" in bridle context → "warrant"
- --full flag references → --detail
- jjx_tally references → appropriate new command (jjx_revise_docket, jjx_arm, jjx_relabel, jjx_drop)

Files: Tools/jjk/vov_veiled/JJSC*.adoc (all spec files)

Acceptance: No spec file references old jjx_ command names or old vocabulary in prose. grep confirms.

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/jjk/vov_veiled/JJSC*.adoc 20 files | Steps: 1. In all JJSC*.adoc files rename old command names to new names per paddock mapping where they appear in prose or examples: jjx_parade to jjx_show, jjx_muster to jjx_list, jjx_tally to the appropriate split command based on context 2. In all JJSC*.adoc files replace vocabulary in pace and bridle contexts only: the word spec or specification when referring to pace text field becomes docket, the word direction when referring to bridle execution guidance becomes warrant, the flag --full becomes --detail -- DO NOT change spec when it refers to the AsciiDoc document itself or the JJSA specification or the word specification in general English, only when it means the pace text content 3. JJSCTL-tally.adoc needs the most work: update to describe the four replacement commands jjx_revise_docket and jjx_arm and jjx_relabel and jjx_drop with their signatures and purpose, or mark it as superseded by the four new operations 4. JJSCPD-parade.adoc: update --full references to --detail and Spec label references to Docket and Direction label references to Warrant 5. JJSCSD-saddle.adoc: update Spec and Direction label references to Docket and Warrant 6. JJSCGS-get-spec.adoc: update to reference jjx_get_brief and docket terminology | Verify: grep across all JJSC*.adoc for old command names jjx_parade jjx_muster jjx_tally jjx_notch etc to confirm none remain in prose -- also grep for --full to confirm replaced with --detail

**[260206-2233] rough**

Update JJSC* individual spec files to reference new command names and vocabulary.

Each spec file mentions jjx_ command names in descriptions and examples. Update:
- Old command names → new boring names per paddock mapping
- "spec"/"specification" in pace context → "docket"
- "direction" in bridle context → "warrant"
- --full flag references → --detail
- jjx_tally references → appropriate new command (jjx_revise_docket, jjx_arm, jjx_relabel, jjx_drop)

Files: Tools/jjk/vov_veiled/JJSC*.adoc (all spec files)

Acceptance: No spec file references old jjx_ command names or old vocabulary in prose. grep confirms.

**[260206-2202] rough**

Update JJSC* individual spec files to reference new command names in prose.

Each spec file (JJSCPD-parade.adoc, JJSCMU-muster.adoc, etc.) mentions its own jjx_ command name in descriptions and examples. Update these references to the new boring names.

Files: Tools/jjk/vov_veiled/JJSC*.adoc (all spec files)

Acceptance: No spec file references old jjx_ command names in prose text.

### update-orchestration-slash-cmds (₢AWAAD) [complete]

**[260207-1401] complete**

Update remaining orchestration slash commands to use new jjx_ names and vocabulary.

**Command renames** — Update every ./tt/vvw-r.RunVVX.sh jjx_* invocation per paddock mapping. Key changes:
- jjx_saddle → jjx_orient
- jjx_tally → jjx_revise_docket, jjx_arm, jjx_relabel, or jjx_drop (per use case)
- jjx_parade → jjx_show
- jjx_chalk → jjx_mark
- All others per mapping

**Vocabulary** — In slash command prose:
- "spec"/"specification" (pace context) → "docket"
- "direction" (bridle context) → "warrant"
- --full → --detail
- "Spec:" output labels → "Docket:"
- "Direction:" output labels → "Warrant:"

**Tally disambiguation** — Each slash command's tally call maps to exactly one new command:
- jjc-pace-bridle: jjx_tally --state bridled --direction → jjx_arm < stdin
- jjc-pace-reslate: jjx_tally [--silks] < stdin → jjx_revise_docket < stdin [&& jjx_relabel]
- jjc-heat-mount: jjx_tally --silks → jjx_relabel
- jjc-heat-braid: jjx_tally < stdin → jjx_revise_docket < stdin
- jjc-heat-restring: jjx_tally < stdin → jjx_revise_docket < stdin

Files: .claude/commands/jjc-*.md (surviving orchestration commands)

Acceptance: grep for old jjx_ names and old vocabulary in .claude/commands/ returns zero hits.

**[260207-0850] bridled**

Update remaining orchestration slash commands to use new jjx_ names and vocabulary.

**Command renames** — Update every ./tt/vvw-r.RunVVX.sh jjx_* invocation per paddock mapping. Key changes:
- jjx_saddle → jjx_orient
- jjx_tally → jjx_revise_docket, jjx_arm, jjx_relabel, or jjx_drop (per use case)
- jjx_parade → jjx_show
- jjx_chalk → jjx_mark
- All others per mapping

**Vocabulary** — In slash command prose:
- "spec"/"specification" (pace context) → "docket"
- "direction" (bridle context) → "warrant"
- --full → --detail
- "Spec:" output labels → "Docket:"
- "Direction:" output labels → "Warrant:"

**Tally disambiguation** — Each slash command's tally call maps to exactly one new command:
- jjc-pace-bridle: jjx_tally --state bridled --direction → jjx_arm < stdin
- jjc-pace-reslate: jjx_tally [--silks] < stdin → jjx_revise_docket < stdin [&& jjx_relabel]
- jjc-heat-mount: jjx_tally --silks → jjx_relabel
- jjc-heat-braid: jjx_tally < stdin → jjx_revise_docket < stdin
- jjc-heat-restring: jjx_tally < stdin → jjx_revise_docket < stdin

Files: .claude/commands/jjc-*.md (surviving orchestration commands)

Acceptance: grep for old jjx_ names and old vocabulary in .claude/commands/ returns zero hits.

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: .claude/commands/jjc-*.md 20 files | Steps: 1. In all 20 jjc-*.md files rename every jjx_ command invocation per paddock mapping: jjx_saddle to jjx_orient, jjx_parade to jjx_show, jjx_muster to jjx_list, jjx_chalk to jjx_mark, jjx_nominate to jjx_create, jjx_slate to jjx_enroll, jjx_rail to jjx_reorder, jjx_furlough to jjx_alter, jjx_notch to jjx_record, jjx_wrap to jjx_close, jjx_rein to jjx_log, jjx_scout to jjx_search, jjx_retire to jjx_archive, jjx_restring to jjx_transfer, jjx_garland to jjx_continue, jjx_curry to jjx_paddock, jjx_draft to jjx_relocate, jjx_get_spec to jjx_get_brief 2. Disambiguate every jjx_tally call to the correct split command: in jjc-pace-bridle.md replace jjx_tally CORONET --state bridled --direction with jjx_arm CORONET passing warrant via stdin, in jjc-pace-reslate.md replace jjx_tally with jjx_revise_docket for text updates and jjx_relabel for silks updates composed with ampersand-ampersand, in jjc-heat-mount.md replace jjx_tally --silks with jjx_relabel, in jjc-heat-braid.md replace jjx_tally text updates with jjx_revise_docket via stdin, in jjc-heat-restring.md replace jjx_tally text updates with jjx_revise_docket via stdin 3. Update vocabulary in prose: spec or specification in pace context to docket, direction in bridle context to warrant, --full flag to --detail, Spec: output labels to Docket:, Direction: output labels to Warrant: 4. Update any remaining references to old output labels like Spec: and Direction: in parsing instructions to Docket: and Warrant: | Verify: grep across all .claude/commands/jjc-*.md for old command names jjx_parade jjx_muster jjx_tally jjx_saddle etc and for --full to confirm zero hits

**[260206-2233] rough**

Update remaining orchestration slash commands to use new jjx_ names and vocabulary.

**Command renames** — Update every ./tt/vvw-r.RunVVX.sh jjx_* invocation per paddock mapping. Key changes:
- jjx_saddle → jjx_orient
- jjx_tally → jjx_revise_docket, jjx_arm, jjx_relabel, or jjx_drop (per use case)
- jjx_parade → jjx_show
- jjx_chalk → jjx_mark
- All others per mapping

**Vocabulary** — In slash command prose:
- "spec"/"specification" (pace context) → "docket"
- "direction" (bridle context) → "warrant"
- --full → --detail
- "Spec:" output labels → "Docket:"
- "Direction:" output labels → "Warrant:"

**Tally disambiguation** — Each slash command's tally call maps to exactly one new command:
- jjc-pace-bridle: jjx_tally --state bridled --direction → jjx_arm < stdin
- jjc-pace-reslate: jjx_tally [--silks] < stdin → jjx_revise_docket < stdin [&& jjx_relabel]
- jjc-heat-mount: jjx_tally --silks → jjx_relabel
- jjc-heat-braid: jjx_tally < stdin → jjx_revise_docket < stdin
- jjc-heat-restring: jjx_tally < stdin → jjx_revise_docket < stdin

Files: .claude/commands/jjc-*.md (surviving orchestration commands)

Acceptance: grep for old jjx_ names and old vocabulary in .claude/commands/ returns zero hits.

**[260206-2202] rough**

Update remaining orchestration slash commands to use new jjx_ names.

These slash commands invoke jjx_* commands and must reference the new names:
- jjc-heat-mount.md (jjx_saddle→jjx_orient, jjx_tally→jjx_revise, jjx_chalk→jjx_mark, jjx_landing→keep)
- jjc-heat-braid.md (jjx_parade→jjx_show, jjx_curry→jjx_paddock, jjx_rein→jjx_log, jjx_get_spec→keep, jjx_tally→jjx_revise, jjx_chalk→jjx_mark)
- jjc-heat-quarter.md (jjx_get_coronets→keep, jjx_get_spec→keep)
- jjc-heat-groom.md (jjx_parade→jjx_show)
- jjc-pace-bridle.md (jjx_saddle→jjx_orient, jjx_tally→jjx_revise)
- jjc-pace-reslate.md (jjx_parade→jjx_show, jjx_get_spec→keep, jjx_tally→jjx_revise)
- jjc-pace-slate.md (jjx_muster→jjx_list, jjx_slate→jjx_enroll)
- jjc-heat-garland.md (jjx_muster→jjx_list, jjx_parade→jjx_show, jjx_garland→jjx_continue)
- jjc-heat-restring.md (jjx_restring→jjx_transfer, jjx_tally→jjx_revise)
- jjc-heat-retire-dryrun.md (jjx_retire→jjx_archive)
- jjc-heat-retire-FINAL.md (jjx_retire→jjx_archive)
- jjc-heat-furlough.md (jjx_muster→jjx_list, jjx_furlough→jjx_alter)
- jjc-heat-rail.md (jjx_rail→jjx_reorder, jjx_parade→jjx_show)
- jjc-pace-notch.md (jjx_notch→jjx_record)

Files: .claude/commands/jjc-*.md (surviving orchestration commands only)

Acceptance: grep for old jjx_ names in .claude/commands/ returns zero hits (except in commented/historical references).

### delete-thin-wrapper-slash-cmds (₢AWAAE) [complete]

**[260207-1408] complete**

Delete thin wrapper slash commands that will be replaced by the CLI truth table.

Delete these files:
- .claude/commands/jjc-heat-muster.md
- .claude/commands/jjc-heat-rein.md
- .claude/commands/jjc-scout.md
- .claude/commands/jjc-parade.md
- .claude/commands/jjc-heat-nominate.md
- .claude/commands/jjc-pace-wrap.md

Also update any remaining slash commands that reference these deleted commands in their "Available Operations" footer sections — change references to describe the direct CLI invocation or remove the line.

Acceptance: Deleted files do not exist. No remaining slash command references a deleted command.

**[260207-0850] bridled**

Delete thin wrapper slash commands that will be replaced by the CLI truth table.

Delete these files:
- .claude/commands/jjc-heat-muster.md
- .claude/commands/jjc-heat-rein.md
- .claude/commands/jjc-scout.md
- .claude/commands/jjc-parade.md
- .claude/commands/jjc-heat-nominate.md
- .claude/commands/jjc-pace-wrap.md

Also update any remaining slash commands that reference these deleted commands in their "Available Operations" footer sections — change references to describe the direct CLI invocation or remove the line.

Acceptance: Deleted files do not exist. No remaining slash command references a deleted command.

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: .claude/commands/jjc-heat-muster.md, .claude/commands/jjc-heat-rein.md, .claude/commands/jjc-scout.md, .claude/commands/jjc-parade.md, .claude/commands/jjc-heat-nominate.md, .claude/commands/jjc-pace-wrap.md plus surviving jjc-*.md files ~20 files | Steps: 1. Delete these 6 files: .claude/commands/jjc-heat-muster.md, .claude/commands/jjc-heat-rein.md, .claude/commands/jjc-scout.md, .claude/commands/jjc-parade.md, .claude/commands/jjc-heat-nominate.md, .claude/commands/jjc-pace-wrap.md 2. In all surviving .claude/commands/jjc-*.md files search for references to the deleted commands in Available Operations footer sections or anywhere else -- for jjc-heat-muster replace with direct CLI note like jjx_list, for jjc-heat-rein replace with jjx_log, for jjc-scout replace with jjx_search, for jjc-parade replace with jjx_show, for jjc-heat-nominate replace with jjx_create, for jjc-pace-wrap replace with jjx_close -- change lines like /jjc-heat-muster to jjx_list and /jjc-parade to jjx_show etc 3. Also check vocjjmc_core.md for references to deleted commands in the Quick Verbs table or slash command reference and update or remove | Verify: confirm the 6 files are deleted with ls, grep across .claude/commands/ for the deleted command names jjc-heat-muster jjc-heat-rein jjc-scout jjc-parade jjc-heat-nominate jjc-pace-wrap to confirm zero references remain

**[260206-2202] rough**

Delete thin wrapper slash commands that will be replaced by the CLI truth table.

Delete these files:
- .claude/commands/jjc-heat-muster.md
- .claude/commands/jjc-heat-rein.md
- .claude/commands/jjc-scout.md
- .claude/commands/jjc-parade.md
- .claude/commands/jjc-heat-nominate.md
- .claude/commands/jjc-pace-wrap.md

Also update any remaining slash commands that reference these deleted commands in their "Available Operations" footer sections — change references to describe the direct CLI invocation or remove the line.

Acceptance: Deleted files do not exist. No remaining slash command references a deleted command.

### add-cli-truth-table (₢AWAAF) [complete]

**[260207-1527] complete**

Add CLI truth table and update all JJK CLAUDE.md content in vocjjmc_core.md.

**CLI truth table** — Replace "Common CLI Pitfalls" with compact reference:
```
jjx_show [TARGET] [--detail] [--remaining]
jjx_list [--status racing|stabled|retired]
jjx_create --silks SILKS
jjx_enroll FIREMARK --silks SILKS [--first|--before C|--after C] <stdin
jjx_reorder FIREMARK [--move C --first|--last|--before C|--after C | CORONET...]
jjx_alter FIREMARK [--status racing|stabled] [--silks SILKS]
jjx_record CORONET FILE [FILE...] [--intent "msg"]
jjx_close CORONET
jjx_log FIREMARK [--limit N]
jjx_search PATTERN [--actionable]
jjx_archive FIREMARK [--execute]
jjx_transfer FIREMARK --to FIREMARK <stdin
jjx_continue FIREMARK
jjx_mark CORONET --marker M --description "text"
jjx_paddock FIREMARK [<stdin for set]
jjx_relocate CORONET --to FIREMARK
jjx_orient [FIREMARK]
jjx_revise_docket CORONET <stdin
jjx_arm CORONET <stdin
jjx_relabel CORONET --silks "name"
jjx_drop CORONET
jjx_get_brief CORONET (was get_spec)
jjx_get_coronets FIREMARK [--rough]
jjx_landing CORONET AGENT <stdin
jjx_validate [--file PATH]
```

**Vocabulary updates:**
- "spec" (pace context) → "docket" throughout
- "direction" (bridle context) → "warrant" throughout
- Update Quick Verbs table (remove deleted thin wrappers)
- Update Slash Command Reference table (remove deleted thin wrappers)
- jjx_notch → jjx_record in Commit Discipline section
- jjx_get_spec → jjx_get_brief

**Composition recipes** — Add section showing && patterns:
```
# Revise docket and rename:
jjx_revise_docket C < stdin && jjx_relabel C --silks "name"
# Bridle with revised docket:
jjx_revise_docket C < stdin && jjx_arm C < warrant
```

Files: Tools/jjk/vov_veiled/vocjjmc_core.md

Acceptance: No old jjx_ names or old vocabulary. Truth table covers all commands.

**[260207-0851] bridled**

Add CLI truth table and update all JJK CLAUDE.md content in vocjjmc_core.md.

**CLI truth table** — Replace "Common CLI Pitfalls" with compact reference:
```
jjx_show [TARGET] [--detail] [--remaining]
jjx_list [--status racing|stabled|retired]
jjx_create --silks SILKS
jjx_enroll FIREMARK --silks SILKS [--first|--before C|--after C] <stdin
jjx_reorder FIREMARK [--move C --first|--last|--before C|--after C | CORONET...]
jjx_alter FIREMARK [--status racing|stabled] [--silks SILKS]
jjx_record CORONET FILE [FILE...] [--intent "msg"]
jjx_close CORONET
jjx_log FIREMARK [--limit N]
jjx_search PATTERN [--actionable]
jjx_archive FIREMARK [--execute]
jjx_transfer FIREMARK --to FIREMARK <stdin
jjx_continue FIREMARK
jjx_mark CORONET --marker M --description "text"
jjx_paddock FIREMARK [<stdin for set]
jjx_relocate CORONET --to FIREMARK
jjx_orient [FIREMARK]
jjx_revise_docket CORONET <stdin
jjx_arm CORONET <stdin
jjx_relabel CORONET --silks "name"
jjx_drop CORONET
jjx_get_brief CORONET (was get_spec)
jjx_get_coronets FIREMARK [--rough]
jjx_landing CORONET AGENT <stdin
jjx_validate [--file PATH]
```

**Vocabulary updates:**
- "spec" (pace context) → "docket" throughout
- "direction" (bridle context) → "warrant" throughout
- Update Quick Verbs table (remove deleted thin wrappers)
- Update Slash Command Reference table (remove deleted thin wrappers)
- jjx_notch → jjx_record in Commit Discipline section
- jjx_get_spec → jjx_get_brief

**Composition recipes** — Add section showing && patterns:
```
# Revise docket and rename:
jjx_revise_docket C < stdin && jjx_relabel C --silks "name"
# Bridle with revised docket:
jjx_revise_docket C < stdin && jjx_arm C < warrant
```

Files: Tools/jjk/vov_veiled/vocjjmc_core.md

Acceptance: No old jjx_ names or old vocabulary. Truth table covers all commands.

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/jjk/vov_veiled/vocjjmc_core.md 1 file | Steps: 1. Replace the Common CLI Pitfalls section with a CLI truth table section containing the exact command signatures from the spec -- all 24 commands with their arguments and stdin indicators 2. Add a Composition Recipes section after the truth table showing ampersand-ampersand composition patterns: revise_docket plus relabel, revise_docket plus arm 3. Update vocabulary throughout: spec in pace context to docket, direction in bridle context to warrant, jjx_notch to jjx_record, jjx_tally to the appropriate split command, jjx_parade to jjx_show, jjx_saddle to jjx_orient, jjx_get_spec to jjx_get_brief, and all other old command names per paddock mapping 4. Update Quick Verbs table: remove verbs for deleted thin wrappers -- muster, rein, scout, parade -- keep verbs that map to surviving orchestration slash commands 5. Update Slash Command Reference table: remove rows for deleted commands /jjc-heat-muster /jjc-heat-rein /jjc-scout /jjc-parade /jjc-heat-nominate /jjc-pace-wrap -- keep rows for surviving orchestration commands 6. Update Commit Discipline section: jjx_notch references to jjx_record, vvx_commit to jjx_record context 7. Update Identities vs Display Names section if it references old command names 8. Update any remaining references to --full flag to --detail | Verify: grep for old command names jjx_parade jjx_muster jjx_tally jjx_notch jjx_saddle jjx_tally and for --full and for the word spec in pace context to confirm none remain

**[260206-2234] rough**

Add CLI truth table and update all JJK CLAUDE.md content in vocjjmc_core.md.

**CLI truth table** — Replace "Common CLI Pitfalls" with compact reference:
```
jjx_show [TARGET] [--detail] [--remaining]
jjx_list [--status racing|stabled|retired]
jjx_create --silks SILKS
jjx_enroll FIREMARK --silks SILKS [--first|--before C|--after C] <stdin
jjx_reorder FIREMARK [--move C --first|--last|--before C|--after C | CORONET...]
jjx_alter FIREMARK [--status racing|stabled] [--silks SILKS]
jjx_record CORONET FILE [FILE...] [--intent "msg"]
jjx_close CORONET
jjx_log FIREMARK [--limit N]
jjx_search PATTERN [--actionable]
jjx_archive FIREMARK [--execute]
jjx_transfer FIREMARK --to FIREMARK <stdin
jjx_continue FIREMARK
jjx_mark CORONET --marker M --description "text"
jjx_paddock FIREMARK [<stdin for set]
jjx_relocate CORONET --to FIREMARK
jjx_orient [FIREMARK]
jjx_revise_docket CORONET <stdin
jjx_arm CORONET <stdin
jjx_relabel CORONET --silks "name"
jjx_drop CORONET
jjx_get_brief CORONET (was get_spec)
jjx_get_coronets FIREMARK [--rough]
jjx_landing CORONET AGENT <stdin
jjx_validate [--file PATH]
```

**Vocabulary updates:**
- "spec" (pace context) → "docket" throughout
- "direction" (bridle context) → "warrant" throughout
- Update Quick Verbs table (remove deleted thin wrappers)
- Update Slash Command Reference table (remove deleted thin wrappers)
- jjx_notch → jjx_record in Commit Discipline section
- jjx_get_spec → jjx_get_brief

**Composition recipes** — Add section showing && patterns:
```
# Revise docket and rename:
jjx_revise_docket C < stdin && jjx_relabel C --silks "name"
# Bridle with revised docket:
jjx_revise_docket C < stdin && jjx_arm C < warrant
```

Files: Tools/jjk/vov_veiled/vocjjmc_core.md

Acceptance: No old jjx_ names or old vocabulary. Truth table covers all commands.

**[260206-2203] rough**

Add CLI truth table to vocjjmc_core.md and update all JJK CLAUDE.md content.

Replace the "Common CLI Pitfalls" section with a compact CLI truth table showing every jjx_ command with its exact argument/flag syntax. Also update:
- Quick Verbs table (slash command names stay, but remove verbs for deleted thin wrappers)
- Slash Command Reference table (remove rows for deleted thin wrappers)
- Commit Discipline section (jjx_notch → jjx_record, vvx_commit reference stays)
- Any other jjx_ name references in vocjjmc_core.md

Format for truth table:
```
jjx_show [TARGET] [--full] [--remaining]
jjx_list [--status racing|stabled|retired]
jjx_create --silks SILKS
jjx_enroll FIREMARK --silks SILKS [--first|--before C|--after C] <stdin
...
```

Files: Tools/jjk/vov_veiled/vocjjmc_core.md

Acceptance: vocjjmc_core.md contains no old jjx_ names. Truth table covers all 22 commands.

### implement-vvx-freshen (₢AWAAG) [complete]

**[260208-0502] complete**

Implement vvx_freshen: a new VOS operation that applies MANAGED block templates from kit forge source directly to CLAUDE.md, bypassing the release/parcel pipeline.

Context: The kit forge (origination repo) has no way to propagate vocjjmc_core.md changes to CLAUDE.md without a full release+install cycle. The freshen engine (voff_freshen.rs) and registry (vofc_registry.rs) already exist — only CLI wiring and a forge-source template reader are missing.

Steps:
1. Add vosof_freshen operation to VOS spec (VOS0-VoxObscuraSpec.adoc):
   - New operation section parallel to vosor_release, vosoi_install, vosou_uninstall
   - Inputs: burc.env path (to resolve project_root, tools_dir, BURC_MANAGED_KITS)
   - Behavior: read templates from Tools/{kit}/vov_veiled/, call voff_freshen on CLAUDE.md
   - No parcel, no git commit (caller decides when to commit)
2. Add CLI truth table section to VOS spec, covering at minimum:
   - vvx_freshen (new)
3. Implement vvx_freshen in Rust:
   - New subcommand in vorm_main.rs
   - Template reader that goes to Tools/{kit}/vov_veiled/{template_path} (not parcel)
   - Reuse voff_freshen engine, vofc_registry for kit→template mapping
   - Reuse burc.env parsing (zvofe_parse_burc or equivalent)
   - Estimated ~40-50 lines of new Rust
4. Build and test: tt/vow-b.Build.sh && tt/vow-t.Test.sh
5. Run vvx_freshen to propagate all four *_core.md templates to CLAUDE.md
6. Verify CLAUDE.md MANAGED:JJK block matches vocjjmc_core.md content
7. Verify new CLI names work, old names fail (from prior pace's unfinished verification)

INTERACTIVE: This is the first use of forge-freshen — work interactively to catch surprises. Do not bridle.

Files: Tools/vok/vov_veiled/VOS0-VoxObscuraSpec.adoc, Tools/vok/src/vorm_main.rs, Tools/vok/vof/src/vofe_emplace.rs (pub visibility), CLAUDE.md

Acceptance: vvx_freshen exists, VOS spec documents it with truth table, CLAUDE.md MANAGED:JJK block is current, all four managed templates can be applied.

**[260208-0446] rough**

Implement vvx_freshen: a new VOS operation that applies MANAGED block templates from kit forge source directly to CLAUDE.md, bypassing the release/parcel pipeline.

Context: The kit forge (origination repo) has no way to propagate vocjjmc_core.md changes to CLAUDE.md without a full release+install cycle. The freshen engine (voff_freshen.rs) and registry (vofc_registry.rs) already exist — only CLI wiring and a forge-source template reader are missing.

Steps:
1. Add vosof_freshen operation to VOS spec (VOS0-VoxObscuraSpec.adoc):
   - New operation section parallel to vosor_release, vosoi_install, vosou_uninstall
   - Inputs: burc.env path (to resolve project_root, tools_dir, BURC_MANAGED_KITS)
   - Behavior: read templates from Tools/{kit}/vov_veiled/, call voff_freshen on CLAUDE.md
   - No parcel, no git commit (caller decides when to commit)
2. Add CLI truth table section to VOS spec, covering at minimum:
   - vvx_freshen (new)
3. Implement vvx_freshen in Rust:
   - New subcommand in vorm_main.rs
   - Template reader that goes to Tools/{kit}/vov_veiled/{template_path} (not parcel)
   - Reuse voff_freshen engine, vofc_registry for kit→template mapping
   - Reuse burc.env parsing (zvofe_parse_burc or equivalent)
   - Estimated ~40-50 lines of new Rust
4. Build and test: tt/vow-b.Build.sh && tt/vow-t.Test.sh
5. Run vvx_freshen to propagate all four *_core.md templates to CLAUDE.md
6. Verify CLAUDE.md MANAGED:JJK block matches vocjjmc_core.md content
7. Verify new CLI names work, old names fail (from prior pace's unfinished verification)

INTERACTIVE: This is the first use of forge-freshen — work interactively to catch surprises. Do not bridle.

Files: Tools/vok/vov_veiled/VOS0-VoxObscuraSpec.adoc, Tools/vok/src/vorm_main.rs, Tools/vok/vof/src/vofe_emplace.rs (pub visibility), CLAUDE.md

Acceptance: vvx_freshen exists, VOS spec documents it with truth table, CLAUDE.md MANAGED:JJK block is current, all four managed templates can be applied.

**[260208-0446] rough**

Implement vvx_freshen: a new VOS operation that applies MANAGED block templates from kit forge source directly to CLAUDE.md, bypassing the release/parcel pipeline.

Context: The kit forge (origination repo) has no way to propagate vocjjmc_core.md changes to CLAUDE.md without a full release+install cycle. The freshen engine (voff_freshen.rs) and registry (vofc_registry.rs) already exist — only CLI wiring and a forge-source template reader are missing.

Steps:
1. Add vosof_freshen operation to VOS spec (VOS0-VoxObscuraSpec.adoc):
   - New operation section parallel to vosor_release, vosoi_install, vosou_uninstall
   - Inputs: burc.env path (to resolve project_root, tools_dir, BURC_MANAGED_KITS)
   - Behavior: read templates from Tools/{kit}/vov_veiled/, call voff_freshen on CLAUDE.md
   - No parcel, no git commit (caller decides when to commit)
2. Add CLI truth table section to VOS spec, covering at minimum:
   - vvx_freshen (new)
3. Implement vvx_freshen in Rust:
   - New subcommand in vorm_main.rs
   - Template reader that goes to Tools/{kit}/vov_veiled/{template_path} (not parcel)
   - Reuse voff_freshen engine, vofc_registry for kit→template mapping
   - Reuse burc.env parsing (zvofe_parse_burc or equivalent)
   - Estimated ~40-50 lines of new Rust
4. Build and test: tt/vow-b.Build.sh && tt/vow-t.Test.sh
5. Run vvx_freshen to propagate all four *_core.md templates to CLAUDE.md
6. Verify CLAUDE.md MANAGED:JJK block matches vocjjmc_core.md content
7. Verify new CLI names work, old names fail (from prior pace's unfinished verification)

INTERACTIVE: This is the first use of forge-freshen — work interactively to catch surprises. Do not bridle.

Files: Tools/vok/vov_veiled/VOS0-VoxObscuraSpec.adoc, Tools/vok/src/vorm_main.rs, Tools/vok/vof/src/vofe_emplace.rs (pub visibility), CLAUDE.md

Acceptance: vvx_freshen exists, VOS spec documents it with truth table, CLAUDE.md MANAGED:JJK block is current, all four managed templates can be applied.

**[260206-2203] rough**

Reinstall JJK kit to propagate changes to active .claude/ installation.

Steps:
1. Build: tt/vow-b.Build.sh
2. Test: tt/vow-t.Test.sh
3. Reinstall kit (propagate vocjjmc_core.md → CLAUDE.md MANAGED:JJK block)
4. Verify: run jjx_show, jjx_list, jjx_record --help to confirm new names work
5. Verify: old names (jjx_parade, jjx_muster, jjx_notch) produce "unknown command" errors
6. Verify: CLAUDE.md MANAGED:JJK section shows new truth table

Acceptance: New CLI names work. Old CLI names fail. CLAUDE.md is updated.

### simplify-thin-wrapper-commands (₢AWAAH) [abandoned]

**[260206-2205] abandoned**

Drafted from ₢AHAAS in ₣AH.

Evaluate collapsing thin-wrapper slash commands (reslate, slate, wrap, bridle, rail) into direct CLAUDE.md verb-to-vvx mappings.

Pain point: Claude regularly guesses wrong vvx subcommand names because slash command names don't match vvx names (e.g., reslate → jjx_tally). The indirection through slash command .md files is a recurring failure mode.

Proposed direction:
- Identify which slash commands are thin wrappers vs. genuine orchestrations
- For thin wrappers, replace with CLAUDE.md verb table entries that map directly to vvx invocations
- Standardize shared post-mutation protocol (report, assess, commit) once in CLAUDE.md
- Rely on vvx --help as authoritative syntax reference
- Eliminate the dereference step that causes guessing errors

Design pace — needs human judgment on which commands to collapse and what the CLAUDE.md verb table format should look like.

**[260206-2204] rough**

Drafted from ₢AHAAS in ₣AH.

Evaluate collapsing thin-wrapper slash commands (reslate, slate, wrap, bridle, rail) into direct CLAUDE.md verb-to-vvx mappings.

Pain point: Claude regularly guesses wrong vvx subcommand names because slash command names don't match vvx names (e.g., reslate → jjx_tally). The indirection through slash command .md files is a recurring failure mode.

Proposed direction:
- Identify which slash commands are thin wrappers vs. genuine orchestrations
- For thin wrappers, replace with CLAUDE.md verb table entries that map directly to vvx invocations
- Standardize shared post-mutation protocol (report, assess, commit) once in CLAUDE.md
- Rely on vvx --help as authoritative syntax reference
- Eliminate the dereference step that causes guessing errors

Design pace — needs human judgment on which commands to collapse and what the CLAUDE.md verb table format should look like.

**[260203-1853] rough**

Evaluate collapsing thin-wrapper slash commands (reslate, slate, wrap, bridle, rail) into direct CLAUDE.md verb-to-vvx mappings.

Pain point: Claude regularly guesses wrong vvx subcommand names because slash command names don't match vvx names (e.g., reslate → jjx_tally). The indirection through slash command .md files is a recurring failure mode.

Proposed direction:
- Identify which slash commands are thin wrappers vs. genuine orchestrations
- For thin wrappers, replace with CLAUDE.md verb table entries that map directly to vvx invocations
- Standardize shared post-mutation protocol (report, assess, commit) once in CLAUDE.md
- Rely on vvx --help as authoritative syntax reference
- Eliminate the dereference step that causes guessing errors

Design pace — needs human judgment on which commands to collapse and what the CLAUDE.md verb table format should look like.

### add-upper-api-to-jjsa (₢AWAAR) [complete]

**[260208-0658] complete**

Add Upper API section to JJSA defining the two-layer verb architecture.

## Context

Heat ₣AW renamed jjx_ commands to boring verbs but left the horse vocabulary unmapped in the spec. The human-facing horse terms (slate, mount, bridle, etc.) need formal definitions in JJSA with explicit bindings to their jjdo_ lower-layer operations.

## Deliverables

New `== Upper API` section in JJS0-GallopsData.adoc containing:

1. Introductory prose explaining the two-layer separation (horse vocabulary upper, boring CLI lower)
2. Category declarations: jjsud_ (direct verbs), jjsum_ (multistep verbs), jjsus_ (slash commands), jjsuc_ (context artifacts)
3. Attribute references in the mapping section for all new terms
4. 13 direct verb definitions (jjsud_) each mapping to one jjdo_ operation
5. 5 multistep verb definitions (jjsum_) describing orchestration and linking to jjsus_ slash commands
6. 14 slash command definitions (jjsus_) linking to the verb they implement
7. 1 context artifact definition (jjsuc_insert) for the CLAUDE.md managed section

Acceptance: JJSA renders with all new cross-references resolving. Upper API section is a peer to the existing Operations section.

**[260208-0657] rough**

Add Upper API section to JJSA defining the two-layer verb architecture.

## Context

Heat ₣AW renamed jjx_ commands to boring verbs but left the horse vocabulary unmapped in the spec. The human-facing horse terms (slate, mount, bridle, etc.) need formal definitions in JJSA with explicit bindings to their jjdo_ lower-layer operations.

## Deliverables

New `== Upper API` section in JJS0-GallopsData.adoc containing:

1. Introductory prose explaining the two-layer separation (horse vocabulary upper, boring CLI lower)
2. Category declarations: jjsud_ (direct verbs), jjsum_ (multistep verbs), jjsus_ (slash commands), jjsuc_ (context artifacts)
3. Attribute references in the mapping section for all new terms
4. 13 direct verb definitions (jjsud_) each mapping to one jjdo_ operation
5. 5 multistep verb definitions (jjsum_) describing orchestration and linking to jjsus_ slash commands
6. 14 slash command definitions (jjsus_) linking to the verb they implement
7. 1 context artifact definition (jjsuc_insert) for the CLAUDE.md managed section

Acceptance: JJSA renders with all new cross-references resolving. Upper API section is a peer to the existing Operations section.

### align-claude-insert-with-upper-api (₢AWAAS) [complete]

**[260208-0700] complete**

Align the CLAUDE.md JJK managed insert (vocjjmc_core.md) with the new Upper API architecture.

## Context

The Quick Verbs table and slash command reference in vocjjmc_core.md predate the two-layer verb architecture added to JJSA. Several entries reference deleted slash commands or mix layers. The insert needs to derive its content from the JJSA Upper API definitions.

## Deliverables

Update vocjjmc_core.md to:

1. Replace the Quick Verbs table with a mapping derived from jjsud_ and jjsum_ definitions: user verb → action (either direct jjx_ invocation or slash command skill)
2. For direct verbs (jjsud_): map verb directly to ./tt/vvw-r.RunVVX.sh jjx_ invocation
3. For multistep verbs (jjsum_): map verb to the corresponding /jjc- slash command
4. Remove references to deleted slash commands
5. Ensure the "When you need to..." table is consistent with the verb mapping

Acceptance: vocjjmc_core.md Quick Verbs table matches JJSA Upper API. No references to deleted slash commands. Freshen produces clean CLAUDE.md.

**[260208-0657] rough**

Align the CLAUDE.md JJK managed insert (vocjjmc_core.md) with the new Upper API architecture.

## Context

The Quick Verbs table and slash command reference in vocjjmc_core.md predate the two-layer verb architecture added to JJSA. Several entries reference deleted slash commands or mix layers. The insert needs to derive its content from the JJSA Upper API definitions.

## Deliverables

Update vocjjmc_core.md to:

1. Replace the Quick Verbs table with a mapping derived from jjsud_ and jjsum_ definitions: user verb → action (either direct jjx_ invocation or slash command skill)
2. For direct verbs (jjsud_): map verb directly to ./tt/vvw-r.RunVVX.sh jjx_ invocation
3. For multistep verbs (jjsum_): map verb to the corresponding /jjc- slash command
4. Remove references to deleted slash commands
5. Ensure the "When you need to..." table is consistent with the verb mapping

Acceptance: vocjjmc_core.md Quick Verbs table matches JJSA Upper API. No references to deleted slash commands. Freshen produces clean CLAUDE.md.

### add-axi-slash-command-voicing (₢AWAAT) [complete]

**[260208-0709] complete**

Create a new AXLA voicing annotation for slash commands and Claude Code concept artifacts.

## Context

The Upper API section in JJSA introduces jjsus_ (slash command) and jjsuc_ (context artifact) categories. These need AXLA annotations to categorize them in the concept model, similar to how jjdo_ operations use axi_cli_subcommand.

## Deliverables

1. Define new axi_ voicing(s) in AXLA-Lexicon.adoc for:
   - Slash command skill artifacts (the .claude/commands/*.md files)
   - Context artifacts (the CLAUDE.md managed sections)
2. Add annotations to jjsus_ and jjsuc_ definitions in JJSA

Acceptance: All jjsus_ and jjsuc_ entries in JJSA have AXLA annotations. New voicings are defined in AXLA-Lexicon.adoc.

**[260208-0657] rough**

Create a new AXLA voicing annotation for slash commands and Claude Code concept artifacts.

## Context

The Upper API section in JJSA introduces jjsus_ (slash command) and jjsuc_ (context artifact) categories. These need AXLA annotations to categorize them in the concept model, similar to how jjdo_ operations use axi_cli_subcommand.

## Deliverables

1. Define new axi_ voicing(s) in AXLA-Lexicon.adoc for:
   - Slash command skill artifacts (the .claude/commands/*.md files)
   - Context artifacts (the CLAUDE.md managed sections)
2. Add annotations to jjsus_ and jjsuc_ definitions in JJSA

Acceptance: All jjsus_ and jjsuc_ entries in JJSA have AXLA annotations. New voicings are defined in AXLA-Lexicon.adoc.

### jjx-advice-output (₢AWAAI) [complete]

**[260208-0721] complete**

Drafted from ₢AHAAM in ₣AH.

Add "Recommended:" advice lines to JJX commands to save Claude tokens.

## Context

Currently only jjrwp_wrap.rs prints advice ("Recommended: /clear then /jjc-heat-mount {firemark}"). Claude often generates similar guidance prose, wasting tokens. If JJX commands print actionable next-step advice, Claude can stay silent.

## Deliverables

Add stderr advice to these commands:

1. **jjx_saddle** (jjrsd_saddle.rs) — after outputting pace context:
   - If bridled: "Recommended: /jjc-heat-mount {firemark} to execute"
   - If rough: "Recommended: /jjc-pace-bridle {coronet} or /jjc-heat-mount {firemark}"

2. **jjx_tally with --state bridled** (bridle operation) — after success:
   - "Recommended: /jjc-heat-mount {firemark} to execute"

3. **jjx_parade** (jjrpd_parade.rs) — when showing remaining paces:
   - If next pace exists: "Recommended: /jjc-heat-mount {firemark}"

## Pattern

Use eprintln! for advice (stderr), matching wrap.rs pattern. Keep advice terse — one line max.

Acceptance: Running jjx_saddle, jjx_tally --state bridled, and jjx_parade shows appropriate "Recommended:" line on stderr.

**[260208-0714] complete**

Drafted from ₢AHAAM in ₣AH.

Add "Recommended:" advice lines to JJX commands to save Claude tokens.

## Context

Currently only jjrwp_wrap.rs prints advice ("Recommended: /clear then /jjc-heat-mount {firemark}"). Claude often generates similar guidance prose, wasting tokens. If JJX commands print actionable next-step advice, Claude can stay silent.

## Deliverables

Add stderr advice to these commands:

1. **jjx_saddle** (jjrsd_saddle.rs) — after outputting pace context:
   - If bridled: "Recommended: /jjc-heat-mount {firemark} to execute"
   - If rough: "Recommended: /jjc-pace-bridle {coronet} or /jjc-heat-mount {firemark}"

2. **jjx_tally with --state bridled** (bridle operation) — after success:
   - "Recommended: /jjc-heat-mount {firemark} to execute"

3. **jjx_parade** (jjrpd_parade.rs) — when showing remaining paces:
   - If next pace exists: "Recommended: /jjc-heat-mount {firemark}"

## Pattern

Use eprintln! for advice (stderr), matching wrap.rs pattern. Keep advice terse — one line max.

Acceptance: Running jjx_saddle, jjx_tally --state bridled, and jjx_parade shows appropriate "Recommended:" line on stderr.

**[260206-2204] rough**

Drafted from ₢AHAAM in ₣AH.

Add "Recommended:" advice lines to JJX commands to save Claude tokens.

## Context

Currently only jjrwp_wrap.rs prints advice ("Recommended: /clear then /jjc-heat-mount {firemark}"). Claude often generates similar guidance prose, wasting tokens. If JJX commands print actionable next-step advice, Claude can stay silent.

## Deliverables

Add stderr advice to these commands:

1. **jjx_saddle** (jjrsd_saddle.rs) — after outputting pace context:
   - If bridled: "Recommended: /jjc-heat-mount {firemark} to execute"
   - If rough: "Recommended: /jjc-pace-bridle {coronet} or /jjc-heat-mount {firemark}"

2. **jjx_tally with --state bridled** (bridle operation) — after success:
   - "Recommended: /jjc-heat-mount {firemark} to execute"

3. **jjx_parade** (jjrpd_parade.rs) — when showing remaining paces:
   - If next pace exists: "Recommended: /jjc-heat-mount {firemark}"

## Pattern

Use eprintln! for advice (stderr), matching wrap.rs pattern. Keep advice terse — one line max.

Acceptance: Running jjx_saddle, jjx_tally --state bridled, and jjx_parade shows appropriate "Recommended:" line on stderr.

**[260127-1145] rough**

Add "Recommended:" advice lines to JJX commands to save Claude tokens.

## Context

Currently only jjrwp_wrap.rs prints advice ("Recommended: /clear then /jjc-heat-mount {firemark}"). Claude often generates similar guidance prose, wasting tokens. If JJX commands print actionable next-step advice, Claude can stay silent.

## Deliverables

Add stderr advice to these commands:

1. **jjx_saddle** (jjrsd_saddle.rs) — after outputting pace context:
   - If bridled: "Recommended: /jjc-heat-mount {firemark} to execute"
   - If rough: "Recommended: /jjc-pace-bridle {coronet} or /jjc-heat-mount {firemark}"

2. **jjx_tally with --state bridled** (bridle operation) — after success:
   - "Recommended: /jjc-heat-mount {firemark} to execute"

3. **jjx_parade** (jjrpd_parade.rs) — when showing remaining paces:
   - If next pace exists: "Recommended: /jjc-heat-mount {firemark}"

## Pattern

Use eprintln! for advice (stderr), matching wrap.rs pattern. Keep advice terse — one line max.

Acceptance: Running jjx_saddle, jjx_tally --state bridled, and jjx_parade shows appropriate "Recommended:" line on stderr.

### fix-furlough-verb-dispatch (₢AWAAU) [complete]

**[260224-0736] complete**

The phrase "furlough that to stabled" should just work via Quick Verb dispatch, but Claude guessed `jjx_alter AW --status stabled` instead of `jjx_alter AW --stabled`.

## Failure observed

```
⏺ Bash(./tt/vvw-r.RunVVX.sh jjx_alter AW --status stabled)
  ⎿  Error: exit code 1
     error: unexpected argument '--status' found
```

Claude then had to `--help` and retry with `--stabled`. The user's natural phrase should have worked on the first try.

## Root cause candidates

1. **Quick Verb table routes furlough to `/jjc-heat-furlough`** but the LLM didn't invoke the slash command — it went straight to `jjx_alter` and guessed the flag
2. **CLI truth table** lists `jjx_alter FIREMARK [--status racing|stabled]` which shows `--status` as a flag name — but the actual CLI uses `--racing` / `--stabled` as boolean flags
3. The mismatch between the documented CLI signature and the actual clap definition

## Deliverables

- Fix the CLI Command Reference in vocjjmc_core.md to match actual `jjx_alter` syntax (`--racing` / `--stabled`, not `--status racing|stabled`)
- Verify all other CLI signatures in the truth table match their actual clap definitions
- Consider whether furlough should stay as a slash command route or become a direct verb with correct syntax shown

**[260208-0724] rough**

The phrase "furlough that to stabled" should just work via Quick Verb dispatch, but Claude guessed `jjx_alter AW --status stabled` instead of `jjx_alter AW --stabled`.

## Failure observed

```
⏺ Bash(./tt/vvw-r.RunVVX.sh jjx_alter AW --status stabled)
  ⎿  Error: exit code 1
     error: unexpected argument '--status' found
```

Claude then had to `--help` and retry with `--stabled`. The user's natural phrase should have worked on the first try.

## Root cause candidates

1. **Quick Verb table routes furlough to `/jjc-heat-furlough`** but the LLM didn't invoke the slash command — it went straight to `jjx_alter` and guessed the flag
2. **CLI truth table** lists `jjx_alter FIREMARK [--status racing|stabled]` which shows `--status` as a flag name — but the actual CLI uses `--racing` / `--stabled` as boolean flags
3. The mismatch between the documented CLI signature and the actual clap definition

## Deliverables

- Fix the CLI Command Reference in vocjjmc_core.md to match actual `jjx_alter` syntax (`--racing` / `--stabled`, not `--status racing|stabled`)
- Verify all other CLI signatures in the truth table match their actual clap definitions
- Consider whether furlough should stay as a slash command route or become a direct verb with correct syntax shown

### furlough-reorder-heat (₢AWAAW) [complete]

**[260224-0800] complete**

Add heat reordering to furlough: when status changes, move the heat to the first position of its target status group in heat_order.

## Behavior

- `--racing`: move firemark to just before the first racing heat in heat_order
- `--stabled`: move firemark to just before the first stabled heat in heat_order
- Rename-only (no status flag): no reorder
- If no heats of target status exist yet, move to end of heat_order

## Files

- `Tools/jjk/vov_veiled/src/jjro_ops.rs` — `jjrg_furlough`: after status change, reposition firemark in `gallops.heat_order`
- `Tools/jjk/vov_veiled/JJSCFU-furlough.adoc` — update spec to document reorder behavior (remove "position is unchanged" note)

## Implementation

heat_order is a Vec<String> of firemark display strings. The reorder logic:
1. Remove the firemark from its current position in heat_order
2. Find the index of the first heat with the target status
3. Insert at that index (or push to end if none found)

This mirrors the existing pace reorder logic in jjrg_rail.

## Acceptance

- Furloughing to racing moves heat before first racing heat in muster
- Furloughing to stabled moves heat before first stabled heat in muster
- Rename-only does not reorder
- Existing furlough tests still pass
- Build passes

**[260223-0821] rough**

Add heat reordering to furlough: when status changes, move the heat to the first position of its target status group in heat_order.

## Behavior

- `--racing`: move firemark to just before the first racing heat in heat_order
- `--stabled`: move firemark to just before the first stabled heat in heat_order
- Rename-only (no status flag): no reorder
- If no heats of target status exist yet, move to end of heat_order

## Files

- `Tools/jjk/vov_veiled/src/jjro_ops.rs` — `jjrg_furlough`: after status change, reposition firemark in `gallops.heat_order`
- `Tools/jjk/vov_veiled/JJSCFU-furlough.adoc` — update spec to document reorder behavior (remove "position is unchanged" note)

## Implementation

heat_order is a Vec<String> of firemark display strings. The reorder logic:
1. Remove the firemark from its current position in heat_order
2. Find the index of the first heat with the target status
3. Insert at that index (or push to end if none found)

This mirrors the existing pace reorder logic in jjrg_rail.

## Acceptance

- Furloughing to racing moves heat before first racing heat in muster
- Furloughing to stabled moves heat before first stabled heat in muster
- Rename-only does not reorder
- Existing furlough tests still pass
- Build passes

### archive-size-guard-spook (₢AWAAX) [complete]

**[260224-0815] complete**

Fix jjx_archive size guard failure mode (spook from ₣AO retirement).

## Problem

When `jjx_archive --execute` produces a trophy that exceeds the vvc size guard:
1. Files are written (trophy created, gallops modified, paddock deleted) but commit is blocked
2. Command reports "Heat retired successfully" — misleading, commit didn't happen
3. No `--size-limit` flag on archive to override
4. No clear recovery path — user must fall back to raw git

## Fix (two parts)

### A. Atomic failure reporting
`jjx_archive` must not report success if the commit fails. Either:
- Propagate the error clearly ("Archive files written but commit blocked by size guard — use X to complete")
- Or roll back file changes on commit failure

### B. Human-approved size limit override
Do NOT silently exempt trophies from the guard. Instead, add a human approval step:

1. Add `--size-limit <bytes>` to `jjx_archive` (pass through to commit layer)
2. Update `/jjc-heat-retire-FINAL` slash command (or CLAUDE.md retire instructions) to:
   - Attempt archive normally
   - If size guard blocks: display the measured size, ask user for approval
   - On approval: retry with `--size-limit` set to the measured size (or a comfortable margin above)
3. This keeps the guard as a checkpoint while giving the human the final say

### C. Spec synchronization
Update JJS0 (`Tools/jjk/vov_veiled/JJS0-GallopsData.adoc`) to reflect the new `--size-limit` flag on archive and any changes to the retire workflow.

## Acceptance

- `jjx_archive` fails clearly when commit is blocked (no false "success")
- `--size-limit` flag available on `jjx_archive`
- Slash command or CLAUDE.md handles the approval loop
- Large heat retirements (like 39-pace ₣Af) complete without raw git fallback
- JJS0 spec updated to match implementation

**[260224-0711] rough**

Fix jjx_archive size guard failure mode (spook from ₣AO retirement).

## Problem

When `jjx_archive --execute` produces a trophy that exceeds the vvc size guard:
1. Files are written (trophy created, gallops modified, paddock deleted) but commit is blocked
2. Command reports "Heat retired successfully" — misleading, commit didn't happen
3. No `--size-limit` flag on archive to override
4. No clear recovery path — user must fall back to raw git

## Fix (two parts)

### A. Atomic failure reporting
`jjx_archive` must not report success if the commit fails. Either:
- Propagate the error clearly ("Archive files written but commit blocked by size guard — use X to complete")
- Or roll back file changes on commit failure

### B. Human-approved size limit override
Do NOT silently exempt trophies from the guard. Instead, add a human approval step:

1. Add `--size-limit <bytes>` to `jjx_archive` (pass through to commit layer)
2. Update `/jjc-heat-retire-FINAL` slash command (or CLAUDE.md retire instructions) to:
   - Attempt archive normally
   - If size guard blocks: display the measured size, ask user for approval
   - On approval: retry with `--size-limit` set to the measured size (or a comfortable margin above)
3. This keeps the guard as a checkpoint while giving the human the final say

### C. Spec synchronization
Update JJS0 (`Tools/jjk/vov_veiled/JJS0-GallopsData.adoc`) to reflect the new `--size-limit` flag on archive and any changes to the retire workflow.

## Acceptance

- `jjx_archive` fails clearly when commit is blocked (no false "success")
- `--size-limit` flag available on `jjx_archive`
- Slash command or CLAUDE.md handles the approval loop
- Large heat retirements (like 39-pace ₣Af) complete without raw git fallback
- JJS0 spec updated to match implementation

**[260224-0710] rough**

Fix jjx_archive size guard failure mode (spook from ₣AO retirement).

## Problem

When `jjx_archive --execute` produces a trophy that exceeds the vvc size guard:
1. Files are written (trophy created, gallops modified, paddock deleted) but commit is blocked
2. Command reports "Heat retired successfully" — misleading, commit didn't happen
3. No `--size-limit` flag on archive to override
4. No clear recovery path — user must fall back to raw git

## Fix (two parts)

### A. Atomic failure reporting
`jjx_archive` must not report success if the commit fails. Either:
- Propagate the error clearly ("Archive files written but commit blocked by size guard — use X to complete")
- Or roll back file changes on commit failure

### B. Human-approved size limit override
Do NOT silently exempt trophies from the guard. Instead, add a human approval step:

1. Add `--size-limit <bytes>` to `jjx_archive` (pass through to commit layer)
2. Update `/jjc-heat-retire-FINAL` slash command (or CLAUDE.md retire instructions) to:
   - Attempt archive normally
   - If size guard blocks: display the measured size, ask user for approval
   - On approval: retry with `--size-limit` set to the measured size (or a comfortable margin above)
3. This keeps the guard as a checkpoint while giving the human the final say

## Acceptance

- `jjx_archive` fails clearly when commit is blocked (no false "success")
- `--size-limit` flag available on `jjx_archive`
- Slash command or CLAUDE.md handles the approval loop
- Large heat retirements (like 39-pace ₣Af) complete without raw git fallback

### rail-noop-graceful-exit (₢AWAAY) [complete]

**[260225-1935] complete**

jjx_reorder should exit 0 with a message when pace is already in requested position.

## Problem

`jjx_reorder Ai --move AiAAh --last` fails with `jjx_rail: error: git commit failed:` (empty stderr) when AiAAh is already last. The reorder produces no gallops diff, the git commit has nothing to commit, and the error path treats this as a failure.

## Fix

1. After computing the new order, compare against current order. If identical, exit 0 with message: `jjx_rail: no change — {coronet} is already {position}`
2. No commit attempted for no-op.

## Spec

Update JJS0 rail ceremony to document: "If the pace is already in the requested position, rail succeeds with no commit."

## Files

- Rust source for jjx_reorder (rail verb)
- Tools/jjk/vov_veiled/JJSCRL-rail.adoc

**[260225-1928] rough**

jjx_reorder should exit 0 with a message when pace is already in requested position.

## Problem

`jjx_reorder Ai --move AiAAh --last` fails with `jjx_rail: error: git commit failed:` (empty stderr) when AiAAh is already last. The reorder produces no gallops diff, the git commit has nothing to commit, and the error path treats this as a failure.

## Fix

1. After computing the new order, compare against current order. If identical, exit 0 with message: `jjx_rail: no change — {coronet} is already {position}`
2. No commit attempted for no-op.

## Spec

Update JJS0 rail ceremony to document: "If the pace is already in the requested position, rail succeeds with no commit."

## Files

- Rust source for jjx_reorder (rail verb)
- Tools/jjk/vov_veiled/JJSCRL-rail.adoc

### fix-parade-pace-state-visibility (₢AWAAZ) [complete]

**[260302-0831] complete**

Spook: jjx_show pace view obscures current state, causing agents to
misread wrapped paces as rough.

## Root Cause

The pace parade view in jjrpd_parade.rs has three issues:

1. **Header omits pace state** (line 101):
   `println!("Pace: {} ({})", current_tack.silks, coronet_key);`
   prints silks and coronet from tacks[0] (latest) but never prints
   `current_tack.state`. A wrapped pace looks identical to a rough pace.

2. **All tacks displayed reversed** (line 114):
   `pace.tacks.iter().rev().enumerate()` reverses storage order so [0]
   in display is the oldest tack (always rough). The complete tack from
   wrap only appears as the last [N] entry — invisible when truncated.

3. **Spec inconsistency** (JJSCPD-parade.adoc):
   Line 163 says "oldest first, index 0" for display. Line 204 says
   "Print full tack text from tacks[0]" meaning storage index (latest).
   Same notation, different meanings.

## Repairs

1. Add state to header line:
   `Pace: remove-builds-create-path (₢AiAAN) [complete]`

2. Default view shows only latest docket (tacks[0]) rendered directly
   as plain text — no `[N]` index prefix, since there is only one tack
   shown. `--detail` restores the full tack history with indexed
   multi-tack view. Code change: skip the `.rev().enumerate()` loop
   for the default case; render `tacks[0].text` directly.

3. Fix JJSCPD spec to disambiguate storage vs display indexing.

## Key Files

- Tools/jjk/vov_veiled/src/jjrpd_parade.rs (lines 99-130)
- Tools/jjk/vov_veiled/JJSCPD-parade.adoc (lines 163, 204)

## Impact

Any agent or human truncating jjx_show output will misread pace state.
Critical reliability issue for multi-officium workflows.

**[260302-0805] rough**

Spook: jjx_show pace view obscures current state, causing agents to
misread wrapped paces as rough.

## Root Cause

The pace parade view in jjrpd_parade.rs has three issues:

1. **Header omits pace state** (line 101):
   `println!("Pace: {} ({})", current_tack.silks, coronet_key);`
   prints silks and coronet from tacks[0] (latest) but never prints
   `current_tack.state`. A wrapped pace looks identical to a rough pace.

2. **All tacks displayed reversed** (line 114):
   `pace.tacks.iter().rev().enumerate()` reverses storage order so [0]
   in display is the oldest tack (always rough). The complete tack from
   wrap only appears as the last [N] entry — invisible when truncated.

3. **Spec inconsistency** (JJSCPD-parade.adoc):
   Line 163 says "oldest first, index 0" for display. Line 204 says
   "Print full tack text from tacks[0]" meaning storage index (latest).
   Same notation, different meanings.

## Repairs

1. Add state to header line:
   `Pace: remove-builds-create-path (₢AiAAN) [complete]`

2. Default view shows only latest docket (tacks[0]) rendered directly
   as plain text — no `[N]` index prefix, since there is only one tack
   shown. `--detail` restores the full tack history with indexed
   multi-tack view. Code change: skip the `.rev().enumerate()` loop
   for the default case; render `tacks[0].text` directly.

3. Fix JJSCPD spec to disambiguate storage vs display indexing.

## Key Files

- Tools/jjk/vov_veiled/src/jjrpd_parade.rs (lines 99-130)
- Tools/jjk/vov_veiled/JJSCPD-parade.adoc (lines 163, 204)

## Impact

Any agent or human truncating jjx_show output will misread pace state.
Critical reliability issue for multi-officium workflows.

**[260302-0800] rough**

Spook: jjx_show pace view obscures current state, causing agents to
misread wrapped paces as rough.

## Root Cause

The pace parade view in jjrpd_parade.rs has three issues:

1. **Header omits pace state** (line 101):
   `println!("Pace: {} ({})", current_tack.silks, coronet_key);`
   prints silks and coronet from tacks[0] (latest) but never prints
   `current_tack.state`. A wrapped pace looks identical to a rough pace.

2. **All tacks displayed reversed** (line 114):
   `pace.tacks.iter().rev().enumerate()` reverses storage order so [0]
   in display is the oldest tack (always rough). The complete tack from
   wrap only appears as the last [N] entry — invisible when truncated.

3. **Spec inconsistency** (JJSCPD-parade.adoc):
   Line 163 says "oldest first, index 0" for display. Line 204 says
   "Print full tack text from tacks[0]" meaning storage index (latest).
   Same notation, different meanings.

## Repairs

1. Add state to header line:
   `Pace: remove-builds-create-path (₢AiAAN) [complete]`

2. Default view shows only latest docket. `--detail` shows full tack
   history. This eliminates the "all revisions are noise" problem.

3. Fix JJSCPD spec to disambiguate storage vs display indexing.

## Key Files

- Tools/jjk/vov_veiled/src/jjrpd_parade.rs (lines 99-130)
- Tools/jjk/vov_veiled/JJSCPD-parade.adoc (lines 163, 204)

## Impact

Any agent or human truncating jjx_show output will misread pace state.
Critical reliability issue for multi-officium workflows.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 M design-invitatory-session-mark
  2 N add-vvcc-format-branded
  3 O add-vvx-invitatory
  4 P wire-invitatory-triggers
  5 L probe-opus-as-interpreter
  6 Q test-officium-threshold
  7 K fix-scout-utf8-panic
  8 A rename-jjrx-cli-strings
  9 B update-jjsa-operation-names
  10 C update-jjsc-spec-prose
  11 D update-orchestration-slash-cmds
  12 E delete-thin-wrapper-slash-cmds
  13 F add-cli-truth-table
  14 G implement-vvx-freshen
  15 R add-upper-api-to-jjsa
  16 S align-claude-insert-with-upper-api
  17 T add-axi-slash-command-voicing
  18 I jjx-advice-output
  19 U fix-furlough-verb-dispatch
  20 W furlough-reorder-heat
  21 X archive-size-guard-spook
  22 Y rail-noop-graceful-exit
  23 Z fix-parade-pace-state-visibility

MNOPLQKABCDEFGRSTIUWXYZ
···x········x··x·xx·x·· vocjjmc_core.md
···x····xx····x·x······ JJSA-GallopsData.adoc
·············x···xx·x·· CLAUDE.md
·xx··········x·······x· lib.rs
·······x·········x····x jjrpd_parade.rs
···x···x·········x····· jjrsd_saddle.rs
··x·xx················· vvcp_probe.rs
·xx·x·················· Cargo.lock, Cargo.toml
·········x············x JJSCPD-parade.adoc
·········x···········x· JJSCRL-rail.adoc
·········x··········x·· JJSCRT-retire.adoc
·······x·········x····· jjrtl_tally.rs
·······x·x············· RBSA-SpecTop.adoc
····x··············x··· jjro_ops.rs
····x·····x············ jjc-heat-furlough.md
···x···x··············· jjrx_cli.rs
··x··········x········· VOS-VoxObscuraSpec.adoc, vorm_main.rs
·x·x··················· jjrn_notch.rs
·····················x· jjrrl_rail.rs, jjtrl_rail.rs, rbf_Foundry.sh, rbgc_Constants.sh, rbgjb01-derive-tag-base.sh, rbrr.env, rbrr_cli.sh, rbrr_regime.sh
····················x·· RBS0-SpecTop.adoc, jjrrt_retire.rs
···················x··· JJSCFU-furlough.adoc
················x······ AXLA-Lexicon.adoc
·············x········· burc.env, vob_build.sh, vofe_emplace.rs, vow-F.Freshen.sh, vow_workbench.sh
··········x············ jjc-heat-braid.md, jjc-heat-garland.md, jjc-heat-groom.md, jjc-heat-mount.md, jjc-heat-muster.md, jjc-heat-nominate.md, jjc-heat-quarter.md, jjc-heat-rail.md, jjc-heat-rein.md, jjc-heat-restring.md, jjc-heat-retire-FINAL.md, jjc-heat-retire-dryrun.md, jjc-pace-bridle.md, jjc-pace-notch.md, jjc-pace-reslate.md, jjc-pace-slate.md, jjc-pace-wrap.md, jjc-parade.md, jjc-scout.md
·········x············· JJSCDR-draft.adoc, JJSCGL-garland.adoc, JJSCGS-get-spec.adoc, JJSCRS-restring.adoc, JJSCSC-scout.adoc, JJSCSD-saddle.adoc, JJSCSL-slate.adoc, JJSCTL-tally.adoc, JJSCVL-validate.adoc, JJSRPS-persist.adoc, JJSRWP-wrap.adoc
·······x··············· RBAGS-AdminGoogleSpec.adoc
······x················ jjrsc_scout.rs
····x·················· jjrfu_furlough.rs, jjtfu_furlough.rs
···x··················· VOSRP-probe.adoc, jjrc_core.rs, jjrch_chalk.rs, jjrmu_muster.rs
·x····················· jjrnc_notch.rs, vvcc_format.rs, vvtf_format.rs

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 153 commits)

  1 T add-axi-slash-command-voicing
  2 I jjx-advice-output
  3 U fix-furlough-verb-dispatch
  4 W furlough-reorder-heat
  5 X archive-size-guard-spook
  6 Y rail-noop-graceful-exit
  7 Z fix-parade-pace-state-visibility

123456789abcdefghijklmnopqrstuvwxyz
xxx································  T  3c
···xxxxx···························  I  5c
················xxx················  U  3c
····················xxx············  W  3c
·······················xxx·········  X  3c
····························xxx····  Y  3c
·································xx  Z  2c
```

## Steeplechase

### 2026-03-02 08:31 - ₢AWAAZ - W

Fixed pace parade: state in header, default shows latest docket only, spec indexing disambiguated

### 2026-03-02 08:31 - ₢AWAAZ - n

Add pace state to header, default view shows latest docket only, disambiguate spec indexing

### 2026-03-02 08:05 - Heat - T

fix-parade-pace-state-visibility

### 2026-03-02 08:00 - Heat - S

fix-parade-pace-state-visibility

### 2026-02-25 19:35 - ₢AWAAY - W

Added no-op detection to jjx_reorder: exits 0 with message when pace already in position, added 10 rail tests

### 2026-02-25 19:35 - ₢AWAAY - n

Compare old/new order in jjrrl_run_rail; skip persist if identical; update spec

### 2026-02-25 19:31 - ₢AWAAY - A

Compare old/new order in jjrrl_run_rail; skip persist if identical; update spec

### 2026-02-25 19:30 - Heat - r

moved AWAAY to first

### 2026-02-25 19:28 - Heat - S

rail-noop-graceful-exit

### 2026-02-24 08:15 - ₢AWAAX - W

Fixed archive size guard: commit failure now rolls back file changes and exits non-zero; added --size-limit override flag; tested with 41-pace AT retirement

### 2026-02-24 08:15 - ₢AWAAX - n

Fix archive commit failure masking, add --size-limit flag, update spec

### 2026-02-24 08:00 - ₢AWAAX - A

Fix archive commit failure masking, add --size-limit flag, update spec

### 2026-02-24 08:00 - ₢AWAAW - W

Added heat reorder on furlough: status change moves heat to front of target status group in heat_order

### 2026-02-24 08:00 - ₢AWAAW - n

Add heat reorder to jjrg_furlough so status changes move heat to front of target status group in heat_order

### 2026-02-24 07:57 - ₢AWAAW - A

Add heat reorder to jjrg_furlough in jjro_ops.rs, update JJSCFU spec

### 2026-02-24 07:57 - Heat - r

moved AWAAV to last

### 2026-02-24 07:36 - ₢AWAAU - W

Fixed jjx_alter CLI signature (--racing/--stabled not --status), added undocumented flags to jjx_relocate and jjx_get_coronets

### 2026-02-24 07:36 - ₢AWAAU - n

Compare vocjjmc_core.md CLI signatures against jjrx_cli.rs clap defs, fix mismatches

### 2026-02-24 07:25 - ₢AWAAU - A

Compare vocjjmc_core.md CLI signatures against jjrx_cli.rs clap defs, fix mismatches

### 2026-02-24 07:24 - Heat - f

racing

### 2026-02-24 07:11 - Heat - T

archive-size-guard-spook

### 2026-02-24 07:10 - Heat - S

archive-size-guard-spook

### 2026-02-23 08:21 - Heat - S

furlough-reorder-heat

### 2026-02-16 10:39 - Heat - D

AOAAB → ₢AWAAV

### 2026-02-08 07:24 - Heat - f

silks=jjk-verb-dispatch-polish

### 2026-02-08 07:24 - Heat - S

fix-furlough-verb-dispatch

### 2026-02-08 07:22 - Heat - f

stabled

### 2026-02-08 07:21 - ₢AWAAI - W

pace complete

### 2026-02-08 07:21 - ₢AWAAI - n

Cut startup menu from CLAUDE.md; add Noun column to Quick Verbs tables in vocjjmc_core.md and CLAUDE.md; unify CLAUDE.md verb table with correct routing and all 18 verbs including muster

### 2026-02-08 07:14 - ₢AWAAI - W

pace complete

### 2026-02-08 07:14 - ₢AWAAI - n

Add eprintln Recommended advice to orient, arm, and show commands following wrap.rs pattern

### 2026-02-08 07:10 - ₢AWAAI - A

Add eprintln Recommended advice to orient, arm, and show commands following wrap.rs pattern

### 2026-02-08 07:09 - ₢AWAAT - W

pace complete

### 2026-02-08 07:09 - ₢AWAAT - n

Define CC slash command, CLAUDE.md section, and CLAUDE.md verb motifs in AXLA; annotate all jjsus_ and jjsuc_ defs in JJSA with axi_cc_slash_command/claudemd_section/claudemd_verb voices

### 2026-02-08 07:01 - ₢AWAAT - A

Define axi_slash_command and axi_context_artifact in AXLA; annotate all jjsus_ and jjsuc_ defs in JJSA

### 2026-02-08 07:00 - ₢AWAAS - W

pace complete

### 2026-02-08 07:00 - ₢AWAAS - n

Align vocjjmc_core.md with JJSA Upper API: two-layer Quick Verbs table (direct→jjx, multistep→slash command), unified slash command reference, remove stale references

### 2026-02-08 06:59 - ₢AWAAS - A

Align Quick Verbs and slash command reference with JJSA Upper API two-layer architecture

### 2026-02-08 06:58 - ₢AWAAR - W

pace complete

### 2026-02-08 06:58 - ₢AWAAR - n

Add Upper API section to JJSA: two-layer verb architecture with jjsud_ direct verbs, jjsum_ multistep verbs, jjsus_ slash commands, and jjsuc_ context artifacts

### 2026-02-08 06:57 - Heat - S

add-axi-slash-command-voicing

### 2026-02-08 06:57 - Heat - S

align-claude-insert-with-upper-api

### 2026-02-08 06:57 - Heat - S

add-upper-api-to-jjsa

### 2026-02-08 05:02 - ₢AWAAG - W

pace complete

### 2026-02-08 05:02 - ₢AWAAG - n

VOS spec + pub parse_burc + forge template reader + vvx_freshen CLI wiring

### 2026-02-08 04:47 - ₢AWAAG - A

VOS spec + pub parse_burc + forge template reader + vvx_freshen CLI wiring

### 2026-02-08 04:46 - Heat - T

implement-vvx-freshen

### 2026-02-08 04:46 - Heat - T

reinstall-and-verify

### 2026-02-07 15:27 - ₢AWAAG - A

Build, test, reinstall kit, verify new CLI names work and old names fail

### 2026-02-07 15:27 - ₢AWAAF - W

pace complete

### 2026-02-07 15:27 - ₢AWAAF - n

Rename spec→docket, add full CLI reference and composition recipes to JJK core doc

### 2026-02-07 15:26 - ₢AWAAF - L

sonnet landed

### 2026-02-07 15:23 - ₢AWAAF - F

Executing bridled pace via sonnet agent

### 2026-02-07 14:08 - ₢AWAAE - W

pace complete

### 2026-02-07 14:05 - ₢AWAAE - L

sonnet landed

### 2026-02-07 14:02 - ₢AWAAE - F

Executing bridled pace via sonnet agent

### 2026-02-07 14:01 - ₢AWAAD - W

pace complete

### 2026-02-07 14:01 - ₢AWAAD - n

Align JJK slash commands with renamed CLI verbs (parade→show, saddle→orient, muster→list, etc.) and update spec→docket, direction→warrant terminology

### 2026-02-07 14:01 - ₢AWAAD - L

sonnet landed

### 2026-02-07 13:52 - ₢AWAAD - F

Executing bridled pace via sonnet agent

### 2026-02-07 13:50 - ₢AWAAC - W

pace complete

### 2026-02-07 13:50 - ₢AWAAC - n

Align JJK operation anchors with CLI verb names and restructure RBSA-SpecTop with system overview, local architecture, and tiered organization

### 2026-02-07 13:30 - ₢AWAAC - F

Executing bridled pace via sonnet agent

### 2026-02-07 13:29 - ₢AWAAB - W

pace complete

### 2026-02-07 13:29 - ₢AWAAB - n

Rename JJK CLI operations to plain-English verbs and add revise_docket, arm, relabel, drop subcommand specs with presentation vocabulary section

### 2026-02-07 13:28 - ₢AWAAB - L

sonnet landed

### 2026-02-07 13:23 - ₢AWAAB - F

Executing bridled pace via sonnet agent

### 2026-02-07 13:22 - ₢AWAAA - W

pace complete

### 2026-02-07 13:22 - ₢AWAAA - n

Rename JJK CLI commands to plain-English verbs and split tally into four focused subcommands

### 2026-02-07 13:14 - ₢AWAAA - F

Executing bridled pace via sonnet agent

### 2026-02-07 13:13 - ₢AWAAQ - W

pace complete

### 2026-02-07 13:13 - ₢AWAAQ - n

Add testing hint comment to officium gap threshold constant

### 2026-02-07 12:43 - ₢AWAAQ - A

Manual integration test: lower officium gap to 5s, verify trigger/suppress cycle across orient/list/groom, check git log format, restore gap to 3600

### 2026-02-07 12:42 - ₢AWAAL - W

pace complete

### 2026-02-07 12:42 - ₢AWAAL - n

Refactor probe to use opus-as-interpreter pattern: haiku/sonnet raw outputs parsed by opus via XML elements, furlough promotes heat to top of list even when already in target status

### 2026-02-07 12:37 - ₢AWAAL - F

Executing bridled pace via sonnet agent

### 2026-02-07 12:35 - Heat - r

moved AWAAQ after AWAAL

### 2026-02-07 12:26 - ₢AWAAQ - A

Manual test: lower gap to 5s, verify trigger/suppress cycle, restore

### 2026-02-07 12:24 - Heat - f

racing

### 2026-02-07 08:56 - ₢AWAAP - W

pace complete

### 2026-02-07 08:56 - ₢AWAAP - n

Migrate session markers to VVC invitatory: remove session chalk type and probe logic from JJK, replace with vvcp_invitatory() calls in saddle/muster, rename session→officium throughout docs

### 2026-02-07 08:53 - Heat - S

test-officium-threshold

### 2026-02-07 08:51 - ₢AWAAP - F

Executing bridled pace via sonnet agent

### 2026-02-07 08:51 - ₢AWAAF - B

tally | add-cli-truth-table

### 2026-02-07 08:51 - Heat - T

add-cli-truth-table

### 2026-02-07 08:50 - ₢AWAAE - B

tally | delete-thin-wrapper-slash-cmds

### 2026-02-07 08:50 - Heat - T

delete-thin-wrapper-slash-cmds

### 2026-02-07 08:50 - ₢AWAAD - B

tally | update-orchestration-slash-cmds

### 2026-02-07 08:50 - Heat - T

update-orchestration-slash-cmds

### 2026-02-07 08:49 - ₢AWAAO - W

pace complete

### 2026-02-07 08:49 - ₢AWAAO - n

add vvx_invitatory command and officium lifecycle to VVC/VOK

### 2026-02-07 08:47 - ₢AWAAC - B

tally | update-jjsc-spec-prose

### 2026-02-07 08:47 - Heat - T

update-jjsc-spec-prose

### 2026-02-07 08:45 - ₢AWAAB - B

tally | update-jjsa-operation-names

### 2026-02-07 08:45 - Heat - T

update-jjsa-operation-names

### 2026-02-07 08:41 - ₢AWAAA - B

tally | rename-jjrx-cli-strings

### 2026-02-07 08:41 - Heat - T

rename-jjrx-cli-strings

### 2026-02-07 08:39 - ₢AWAAL - B

tally | probe-opus-as-interpreter

### 2026-02-07 08:39 - Heat - T

probe-opus-as-interpreter

### 2026-02-07 08:33 - ₢AWAAO - F

Executing bridled pace via sonnet agent

### 2026-02-07 08:30 - ₢AWAAN - W

pace complete

### 2026-02-07 08:30 - ₢AWAAN - n

Extract hallmark and branded format to shared vvcc_format module, wire JJK callers through vvc::vvcc_format_branded

### 2026-02-07 08:25 - ₢AWAAP - B

tally | wire-invitatory-triggers

### 2026-02-07 08:25 - Heat - T

wire-invitatory-triggers

### 2026-02-07 08:24 - ₢AWAAN - F

Executing bridled pace via sonnet agent

### 2026-02-07 08:23 - ₢AWAAO - B

tally | add-vvx-invitatory

### 2026-02-07 08:23 - Heat - T

add-vvx-invitatory

### 2026-02-07 08:17 - ₢AWAAN - B

tally | add-vvcc-format-branded

### 2026-02-07 08:17 - Heat - T

add-vvcc-format-branded

### 2026-02-07 08:10 - ₢AWAAN - A

Extract hallmark+format to VVC, migrate 6 JJK formatters, replace divergent hallmark in jjrnc

### 2026-02-07 08:04 - Heat - T

wire-invitatory-triggers

### 2026-02-07 08:03 - Heat - T

add-vvx-invitatory

### 2026-02-07 08:03 - Heat - T

add-vvcc-format-branded

### 2026-02-07 07:49 - Heat - T

probe-opus-as-interpreter

### 2026-02-07 07:49 - Heat - T

wire-invitatory-triggers

### 2026-02-07 07:48 - Heat - T

add-vvx-invitatory

### 2026-02-07 07:44 - Heat - T

probe-opus-as-interpreter

### 2026-02-07 07:34 - Heat - T

wire-invitatory-triggers

### 2026-02-07 07:34 - Heat - T

add-vvx-invitatory

### 2026-02-07 07:33 - Heat - T

add-vvcc-format-branded

### 2026-02-07 07:20 - ₢AWAAM - W

pace complete

### 2026-02-07 07:17 - Heat - S

wire-invitatory-triggers

### 2026-02-07 07:16 - Heat - S

add-vvx-invitatory

### 2026-02-07 07:16 - Heat - S

add-vvcc-format-branded

### 2026-02-07 07:15 - Heat - T

design-invitatory-session-mark

### 2026-02-07 07:01 - ₢AWAAM - A

Design invitatory: read VOS, JJSA, vvcp_probe, resolve 5 open questions

### 2026-02-07 07:01 - Heat - s

260207-0701 session

### 2026-02-06 23:03 - Heat - S

design-invitatory-session-mark

### 2026-02-06 22:48 - Heat - S

probe-opus-as-interpreter

### 2026-02-06 22:37 - ₢AWAAK - W

pace complete

### 2026-02-06 22:37 - ₢AWAAK - n

fix scout UTF-8 panic by snapping context window to char boundaries

### 2026-02-06 22:36 - Heat - T

fix-scout-utf8-panic

### 2026-02-06 22:34 - Heat - T

split-tally-into-primitives

### 2026-02-06 22:34 - Heat - T

add-cli-truth-table

### 2026-02-06 22:33 - Heat - T

update-orchestration-slash-cmds

### 2026-02-06 22:33 - Heat - T

update-jjsc-spec-prose

### 2026-02-06 22:32 - Heat - T

update-jjsa-operation-names

### 2026-02-06 22:32 - Heat - T

rename-jjrx-cli-strings

### 2026-02-06 22:31 - ₢AWAAK - A

Snap window_start/window_end to char boundaries using str::is_char_boundary() before slicing in zjrsc_extract_match_context

### 2026-02-06 22:30 - Heat - f

racing

### 2026-02-06 22:23 - Heat - S

fix-scout-utf8-panic

### 2026-02-06 22:12 - Heat - S

split-tally-into-primitives

### 2026-02-06 22:09 - Heat - T

rename-jjrx-cli-strings

### 2026-02-06 22:05 - Heat - T

simplify-thin-wrapper-commands

### 2026-02-06 22:04 - Heat - D

restring 2 paces from ₣AH

### 2026-02-06 22:03 - Heat - S

reinstall-and-verify

### 2026-02-06 22:03 - Heat - S

add-cli-truth-table

### 2026-02-06 22:02 - Heat - S

delete-thin-wrapper-slash-cmds

### 2026-02-06 22:02 - Heat - S

update-orchestration-slash-cmds

### 2026-02-06 22:02 - Heat - S

update-jjsc-spec-prose

### 2026-02-06 22:02 - Heat - S

update-jjsa-operation-names

### 2026-02-06 22:02 - Heat - S

rename-jjrx-cli-strings

### 2026-02-06 22:01 - Heat - N

jjk-boring-cli-rename

