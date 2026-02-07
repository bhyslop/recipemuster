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

- Tools/jjk/vov_veiled/JJSA-GallopsData.adoc — operation definitions
- Tools/jjk/vov_veiled/src/jjrx_cli.rs — CLI registration
- Tools/jjk/vov_veiled/vocjjmc_core.md — CLAUDE.md managed source
- .claude/commands/jjc-*.md — slash command definitions
- ₣AG ₢AGAAK, ₢AGAAL — deferred schema renames (text→docket, direction→warrant)
- Origin: ₣AH ₢AHAAV explore-parade-flag-reduction
