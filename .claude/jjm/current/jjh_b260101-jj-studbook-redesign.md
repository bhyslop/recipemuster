# Heat: JJ Studbook Redesign

Redesign Job Jockey around a JSON-based studbook registry, git-based steeplechase, and streamlined bash script APIs that keep implementation details hidden from Claude.

## Paddock

### Motivation

Pain points from prior heats that drive this redesign:

1. **Permission friction** - Editing steeplechase entries required interactive approval every time. Bash scripts bypass this.
2. **Rename brittleness** - Changing heat/pace names broke references in steeplechase, commits, etc. Favors provide stable identity.
3. **Monolithic heat files** - Mixing prose, paces, and activity log made files unwieldy. Decomposition separates concerns.
4. **Context recovery** - After resets, reconstructing "where were we?" required reading entire heat file. Saddle output optimized for this.
5. **Timestamp management** - Manual timestamps in steeplechase entries. Git commits handle this automatically.

### How to Work This Heat

**IMPORTANT**: This heat executes under the CURRENT JJK installation while building its replacement.

- Use existing `/jja-*` commands (heat-saddle, pace-wrap, notch, etc.)
- Do NOT attempt to use new `jj-*` scripts until they are implemented
- Build foundation first (favor encoding, studbook schema), then scripts that depend on them
- Test incrementally: each script should work before building the next
- The new system becomes active only after migration and arcanum update

**Pace dependencies** (must complete in rough order):
1. jju_favor.sh (everything depends on Favor encoding)
2. jjs_studbook.json schema (most scripts need this)
3. jj-muster, jj-saddle (can verify studbook works)
4. jj-chalk, jj-rein (can verify git steeple works)
5. Remaining scripts (slate, rail, tally, wrap, etc.)
6. Migration and arcanum update (cutover)
7. Testing and refinement paces

### Goal

Replace the current monolithic heat file approach with a decomposed architecture:
- **Studbook** (`jjs_studbook.json`) - structured registry of all heats/paces
- **Paddock files** (`jjp_HH.md`) - per-heat prose context
- **Git commits** - steeplechase entries (activity log)
- **Trophy files** (`jjy_HH_YYMMDD-YYMMDD_silks.md`) - retired heat archives

Claude interacts only through bash scripts; never touches JSON or git directly.

### Core Concepts

**Favor** - 5 URL-safe base64 digits identifying heat+pace:
- Format: `HHPPP` where HH = heat (2 digits), PPP = pace (3 digits)
- Heat-only reference: `HH` with PPP = 000 (illegal as pace)
- Display notation: `₣Kb002` (₣ in JSON and git commits; stripped only for filenames)
- Capacity: 4096 heats × 262,144 paces per heat
- **Character set**: `A-Za-z0-9-_` (64 chars, URL-safe, no + or /)
- **Encoding**: Simple modular arithmetic + lookup table, no base64 utility needed

**Studbook** - Single JSON registry (`jjs_studbook.json`):
```json
{
  "heats": {
    "₣Kb": {
      "datestamp": "260101",
      "display": "JJ Studbook Redesign",
      "silks": "jj-studbook-redesign",
      "status": "current",
      "paces": [
        {"id": "001", "display": "Define script APIs", "status": "current"},
        {"id": "002", "display": "Implement jju utilities", "status": "pending"}
      ]
    }
  },
  "saddled": "₣Kb001",
  "next_heat_seed": "Kc"
}
```
- Uses `jq --sort-keys --indent 2` for stable diffs
- Favors as keys provide stable anchoring
- `next_heat_seed` tracks allocation to prevent reuse after retirement
- ₣ included in JSON for distinctive grep/search (stripped only for filenames)

**Paddock** - Per-heat markdown (`jjp_Kb.md`):
- Human-authored prose context
- Goal, approach, constraints, guidelines
- Stays editable, not embedded in JSON

**Trophy** - Retired heat archive (`jjy_Kb_260101-260115_jj-studbook-redesign.md`):
- Contains: final paddock, all paces with outcomes, extracted steeplechase
- Created at retirement from studbook + paddock + git history

### Vocabulary

| Term | Verb/Noun | Meaning |
|------|-----------|---------|
| **Favor** | noun | Heat+pace identifier (₣HHPPP) |
| **Studbook** | noun | JSON registry of heats/paces |
| **Paddock** | noun | Per-heat prose context file |
| **Trophy** | noun | Retired heat archive |
| **Chalk** | verb | Write steeplechase entry |
| **Rein** | verb | Read steeplechase entries |
| **Muster** | verb | List current heats with Favors |
| **Saddle** | verb | Mount up on heat, show full context |
| **Slate** | verb | Add new pace |
| **Reslate** | verb | Revise pace description |
| **Rail** | verb | Reorder paces |
| **Tally** | verb | Set pace state |
| **Wrap** | verb | Complete pace with ceremony |
| **Retire** | verb | Complete heat, create trophy |
| **Notch** | verb | Git commit |
| **Nominate** | verb | Create new heat |

### Pace States

| State | Meaning |
|-------|---------|
| `pending` | In the field, not yet run |
| `complete` | Crossed the wire cleanly |
| `abandoned` | Scratched, won't run |
| `malformed` | Flagged, needs revision |

### File Prefix Conventions

| Prefix | Purpose | Example |
|--------|---------|---------|
| `jja_` | Arcanum (install/uninstall) | `jja_arcanum.sh` |
| `jjb_` | suBagent | `jjb_*.md` |
| `jjc_` | slash Command | `jjc_saddle.md` |
| `jjh_` | Hook | `jjh_*.sh` |
| `jji_` | Itch | `jji_itch.md` |
| `jjk_` | sKill | `jjk_saddle.md` |
| `jjp_` | Paddock | `jjp_Kb.md` |
| `jjs_` | Studbook | `jjs_studbook.json` |
| `jjt_` | Testbench | `jjt_testbench.sh` |
| `jju_` | Utility (bash impl) | `jju_saddle.sh` |
| `jjw_` | Workbench | `jjw_workbench.sh` |
| `jjy_` | Trophy | `jjy_Kb_260101-260115_silks.md` |
| `jjz_` | Scar (declined itch) | `jjz_scar.md` |

### Script API

Claude's interface - all implementation hidden behind these:

| Script | Input | Output |
|--------|-------|--------|
| `jj-muster` | — | ₣Favor + silks per current heat |
| `jj-nominate` | "display" "silks" | New heat ₣Favor + paddock stub |
| `jj-saddle` | ₣Favor | Paddock + paces + recent steeple (2-3 entries) |
| `jj-chalk` | TYPE "title" <stdin | — (non-blocking) |
| `jj-rein` | ₣HH or ₣HHPPP | Full steeple (heat) or pace-filtered |
| `jj-slate` | "display" | New pace ₣Favor |
| `jj-reslate` | ₣Favor "display" | — |
| `jj-rail` | 001 002 003... | Updated pace list (pace IDs, heat implicit) |
| `jj-tally` | ₣Favor STATE | — |
| `jj-wrap` | — | Next ₣Favor or "heat complete" (auto-chalks WRAP) |
| `jj-retire` | — | Trophy path |
| `jj-notch` | "message" <stdin | — |

**Design principles:**
- Current Favor stays in chat context (Claude remembers, no temp file)
- Saddle output includes pace listing + recent steeple (no separate query)
- Completed paces vanish from view until retirement
- Chalk entries include files touched since last entry
- Scripts handle all git/jq mechanics
- Humans don't use these APIs directly; scripts enforce consistency

### Saddle Output Format

```
₣Kb JJ Studbook Redesign (260101)

## Paddock
[Full paddock file contents]

## Current
₣Kb001 [current] Define script APIs

## Remaining
₣Kb002 [pending] Implement jju utilities
₣Kb003 [pending] Implement studbook schema

## Recent (₣Kb001)
[2026-01-01 14:30] APPROACH: Starting API design
  [+jjh_b260101-jj-studbook-redesign.md]
[2026-01-01 15:45] NOTE: Clarified chalk vs tally distinction
  [~jjh_b260101-jj-studbook-redesign.md]
```

- Full paddock provides heat continuity
- Current pace highlighted separately
- Remaining excludes completed/abandoned (those vanish)
- Recent shows 2-3 steeple entries for current pace
- File changes: `[+added]` `[~modified]` `[-deleted]`

### Chalk Emblems

Freeform tags on steeplechase entries. Common patterns:
- **APPROACH** - Starting work on pace
- **WRAP** - Completing pace
- **BLOCKED** - Hit a blocker
- **NOTE** - General observation

Not constrained - Claude picks appropriate emblems contextually.

### Steeplechase in Git

Steeplechase entries become git commits:
- Empty commits (`git commit --allow-empty`)
- Structured message format (scripts handle formatting)
- Timestamped automatically by git
- Searchable via `git log --grep`
- Extracted to trophy at retirement
- Claude uses `jj-chalk` to write, `jj-rein` to read

### Concept Surgery Log

Tracking new/modified terms for continuity during this heat:

**New Terms:**
| Term | Type | Meaning |
|------|------|---------|
| Favor | noun | 5-digit heat+pace identifier (₣HHPPP) |
| Studbook | noun | JSON registry of heats/paces |
| Paddock | noun | Per-heat prose context file (was: section in heat file) |
| Trophy | noun | Retired heat archive (was: retired heat file) |
| Chalk | verb | Write steeplechase entry |
| Rein | verb | Read steeplechase entries |
| Muster | verb | List current heats |
| Slate | verb | Add new pace |
| Reslate | verb | Revise pace description |
| Rail | verb | Reorder paces |
| Tally | verb | Set pace state |
| Nominate | verb | Create new heat |

**Modified Terms:**
| Term | Was | Now |
|------|-----|-----|
| Saddle | Read heat file, pick pace | Read studbook + paddock + steeple, unified context |
| Wrap | Mark pace done in heat file | Mark complete in studbook, auto-chalk, advance |
| Retire | Move heat file to retired/ | Extract from studbook, create trophy |
| Notch | Git commit (unchanged) | Git commit (adding jju script) |

**Preserved Terms:**
- Heat, Pace, Itch, Scar (concepts unchanged, storage changed)
- Silks, Steeplechase (concepts unchanged)

**Prefix Assignments (VOK-aligned):**
| Prefix | Purpose | Status |
|--------|---------|--------|
| `jja_` | Arcanum | existing |
| `jjc_` | Command | new usage |
| `jji_` | Itch | existing |
| `jjk_` | sKill | new usage |
| `jjp_` | Paddock | new |
| `jjs_` | Studbook | new |
| `jju_` | Utility | new |
| `jjy_` | Trophy | new |
| `jjz_` | Scar | renamed from jjs_ |

### File Locations

| File Type | Location | Example |
|-----------|----------|---------|
| Studbook | `.claude/jjm/jjs_studbook.json` | Single file |
| Paddock | `.claude/jjm/jjp_Kb.md` | Per heat |
| Trophy | `.claude/jjm/retired/jjy_Kb_*.md` | Retired heats |
| Itch | `.claude/jjm/jji_itch.md` | Single file |
| Scar | `.claude/jjm/jjz_scar.md` | Single file |
| Utility scripts | `Tools/jjk/jju_*.sh` | Bash implementations |
| Commands | `.claude/commands/jjc_*.md` | Slash commands |
| Skills | `.claude/skills/jjk_*.md` | Skills (if used) |
| Workbench | `Tools/jjk/jjw_workbench.sh` | Dispatch |
| Arcanum | `Tools/jjk/jja_arcanum.sh` | Install/uninstall |

### Steeplechase Git Commit Format

```
[₣Kb001] APPROACH: Starting API design

Body text with details about the approach...

---
[+] jjh_b260101-jj-studbook-redesign.md
[~] Tools/jjk/jjw_workbench.sh
```

- Subject: `[₣Favor] EMBLEM: Title`
- Body: Freeform markdown
- Footer: Files touched since last chalk (`[+]` added, `[~]` modified, `[-]` deleted)
- Created by `jj-chalk`, queried by `jj-rein`

### Fresh Session Handling

On fresh session, Claude doesn't know current Favor. Resolution:
1. Run `jj-muster` to see current heats
2. Run `jj-saddle ₣Kb` (or user indicates which heat)
3. Studbook `saddled` field updated; Claude now has context

The `saddled` field persists across sessions in the studbook.

### Constraints

- URL-safe base64 for Favors (simple bash math + printf, no base64 utility)
- No Unicode in filenames (₣ is display notation only)
- `--sort-keys` for all JSON writes (stable diffs)
- Paddock stays markdown (human-editable prose)
- Scripts abstract all git complexity from Claude
- Backward compatible migration path from current heat files

## Done

(none yet)

## Remaining

- **Define JJU dirty-worktree guards** — Specify which jj-* scripts must fail when git has uncommitted changes. Prevent silent data loss during dual-repo workflows or context switches. Document rationale for each script's policy.

- **Update JJK README with VOK prefix conventions** — Document the prefix-to-type mapping (jja_, jjc_, jjk_, jjp_, jjs_, jju_, jjw_, jjy_) in Tools/jjk/README.md. Align with Vox Obscura reserved suffix pattern.

- **Decide pace index emplacement API** — Evaluate whether we need an API (or API change) to emplace a new pace at a specific index position rather than always appending. Consider use cases like inserting urgent work or reordering paces.

- **Draft script API specifications** — Finalize exact arguments, output formats, error handling for each jj-* script. Document in paddock for reference during implementation.

- **Implement jju_favor.sh** — Favor encoding/decoding utilities. Base64-ish math, validation, heat/pace extraction.

- **Implement jjs_studbook.json schema** — Create initial studbook, document jq patterns for common operations (add heat, add pace, update state, query).

- **Implement jj-muster** — List current heats with Favors and silks.

- **Implement jj-saddle** — Read studbook + paddock + recent steeple, format output for Claude context.

- **Implement jj-chalk** — Create empty git commit with structured steeplechase entry. Include files-touched info.

- **Implement jj-rein** — Query git log for steeple entries, format for context recovery.

- **Implement jj-slate / jj-reslate** — Add/revise paces in studbook.

- **Implement jj-rail** — Reorder paces in studbook.

- **Implement jj-tally** — Set pace state in studbook.

- **Implement jj-wrap** — Mark pace complete, show next pace, chalk wrap entry.

- **Implement jj-retire** — Extract heat from studbook, merge paddock + steeple, create trophy file.

- **Implement jj-nominate** — Allocate next heat Favor from seed, create paddock stub, register in studbook.

- **Create jju_notch.sh** — Standalone git commit script for notch. Provides notch functionality before CGK (Claude Git Kit) exists.

- **Document trophy extraction** — Specify what gets pulled from studbook when creating trophy (paces, outcomes, Favor→silks mapping). Studbook stays lean; trophy is self-contained archive.

- **Design wrap advancement flow** — Discuss: when wrap completes, how does next pace become current? Should wrap output include new saddle context, or require explicit re-saddle? Consider jju utility for state transitions.

- **Migrate current heats** — Convert existing jjh_* heat files to studbook + paddock format.

- **Update jja_arcanum.sh** — Install/uninstall for new JJ structure in CLAUDE.md.

- **Revise CLAUDE.md term definitions** — Update arcanum-generated Job Jockey section with all new terms (Favor, Studbook, Paddock, Trophy, Chalk, Rein, Muster, Slate, Reslate, Rail, Tally, Nominate) and modified terms (Saddle, Wrap, Retire). Align with Concept Surgery Log.

- **Test full workflow** — Create test heat, run through full lifecycle: saddle → work → chalk → wrap → retire.

- **Phase transformation analysis** — Evaluate whether retirement creates a fundamentally different object type. Document decision.

- **Term releveling** — Final pass on vocabulary. Ensure consistent horse/jockey/race metaphor throughout.

- **Reconsider scar naming** — Currently `jjz_scar.md`. Scar = formally declined itch (won't scratch). Evaluate: is `jjz_` the right prefix? Find better horse metaphor for "work I've decided not to do."

- **Revise JJK README future directions** — Update Tools/jjk/README.md future directions section to reflect what was built during this heat. Document lessons learned, what changed from original vision, and remaining opportunities.

## Steeplechase

---
### 2026-01-01 - Paddock Refined

**Clarifications added after deep review**:
- Chalk emblems are freeform (APPROACH, WRAP, BLOCKED, NOTE as patterns)
- Saddle output format specified (full paddock, current/remaining paces, 2-3 steeple entries)
- jj-rein takes heat Favor (full) or pace Favor (filtered)
- jj-nominate added for heat creation
- ₣ included in JSON for distinctive grep; stripped only for filenames
- Studbook stays lean: `next_heat_seed` tracks allocation, heats removed on retirement
- Trophy is self-contained with full Favor→silks mapping
- jjz_ prefix for scar (declined itch, pending reconsideration)

**New paces added**: jj-nominate, jju_notch.sh, trophy extraction doc, wrap advancement design, scar reconsideration

---
### 2026-01-01 - Heat Created

**Context**: Spun off from `b251231-jj-retro-cloud-bashize` after design discussion clarified a larger architectural change than originally scoped.

**Key decisions made**:
- Studbook (JSON) + Paddock (markdown) + Git steeple architecture
- Favor as 5-digit base64 identifier
- Bash scripts as Claude's only interface
- Full vocabulary: muster, saddle, chalk, rein, slate, rail, tally, wrap, retire, notch

**This heat executes under current JJK** while building its replacement.

---
