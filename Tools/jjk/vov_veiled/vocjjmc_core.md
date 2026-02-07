## Job Jockey Configuration

Job Jockey (JJ) is installed for managing project initiatives.

**Concepts:**
- **Heat**: Bounded initiative with coherent goals that are clear and present (3-50 officia). Status: `racing` (active execution) or `stabled` (paused for planning). Location: `current/` or `retired/` (done).
- **Pace**: Discrete action within a heat; can be bridled for autonomous execution via `/jjc-pace-bridle`
- **Itch**: Future work (any detail level), lives in jji_itch.md
- **Scar**: Closed work with lessons learned, lives in jjs_scar.md

**Identities vs Display Names:**
- **Firemark**: Heat identity (`₣AA` or `AA`). Used in CLI args and JSON keys.
- **Coronet**: Pace identity (`₢AAAAk` or `AAAAk`). Used in CLI args and JSON keys.
- **Silks**: kebab-case display name. Human-readable only — NOT usable for lookups.

When a command takes `<firemark>` or `<coronet>`, provide the identity, not the silks.

- Target repo dir: `.`
- JJ Kit path: `Tools/jjk/README.md`

**JJ Slash Command Reference:**

ALWAYS read the corresponding slash command before attempting JJ operations.

**CRITICAL**: JJK CLI syntax is non-standard. Do NOT guess based on common CLI conventions.
- Specs go via stdin, not `--spec`
- Positioning uses `--move X --first`, not `--position first`
- Read the slash command to see the exact `./tt/vvw-r.RunVVX.sh jjx_*` invocation pattern.

**Heredoc for stdin**: Use `cat <<'DELIM' | jjx_*` pattern with quoted delimiter. The delimiter must not appear alone on any line in the content — if content shows heredoc examples with `EOF`, use a different delimiter like `SPEC` or `PACESPEC`.

| When you need to... | Read first |
|---------------------|------------|
| Add a new pace | /jjc-pace-slate |
| Refine pace spec | /jjc-pace-reslate |
| Bridle for autonomous execution | /jjc-pace-bridle |
| Mark pace complete | `jjx_close` |
| Commit with JJ context | /jjc-pace-notch |
| Execute next pace | /jjc-heat-mount |
| Review heat plan | /jjc-heat-groom |
| Evaluate bridleable paces | /jjc-heat-quarter |
| Reorder paces | /jjc-heat-rail |
| Add steeplechase marker | /jjc-heat-chalk |
| Pause or resume heat | /jjc-heat-furlough |
| Create new heat | `jjx_create` |
| List all heats | `jjx_list` |
| Draft paces between heats | /jjc-heat-restring |
| Retire completed heat | /jjc-heat-retire |
| View heat or pace | `jjx_show` |

**Quick Verbs** — When user says just the verb, invoke the corresponding command:

| Verb | Command |
|------|---------|
| mount | /jjc-heat-mount |
| slate | /jjc-pace-slate |
| reslate | /jjc-pace-reslate |
| wrap | `jjx_close` |
| bridle | /jjc-pace-bridle |
| notch | /jjc-pace-notch |
| furlough | /jjc-heat-furlough |
| muster | `jjx_list` |
| groom | /jjc-heat-groom |
| quarter | /jjc-heat-quarter |
| rail | /jjc-heat-rail |
| rein | `jjx_log` |
| scout | `jjx_search` |
| parade | `jjx_show` |

**Common CLI Pitfalls:**
- `jjx_parade` takes firemark OR coronet directly as sole argument — NO flags for target selection
  - Heat overview: `jjx_parade AF`
  - Single pace: `jjx_parade AFAAb`
  - Valid flags: `--full`, `--remaining` only
- NEVER invent CLI flags — run `--help` first or read the slash command
- `jjx_saddle` output includes `pace_coronet` (next actionable pace) — no separate parade call needed to find what's next

**Commit Discipline:**
When working on a heat, ALWAYS use `/jjc-pace-notch` for commits. NEVER use `vvx_commit` directly — it bypasses heat/pace affiliation and steeplechase tracking.

Notch handles two cases:
- **Pace-affiliated**: Active pace provides context (coronet in commit)
- **Heat-affiliated**: No active pace, but commit is part of heat work (firemark only)

When user says "notch", invoke `/jjc-pace-notch` — don't second-guess based on pace state.

**Multi-Officium Discipline:**
Multiple Claude officia may work concurrently in the same repo. The explicit file list in `/jjc-pace-notch` enables orthogonal commits.

- Claude is **additive only** — make commits, never discard changes
- "Unexpected" uncommitted changes are likely another officium's work
- If something looks wrong, ASK — do not "fix" by discarding
- Commit only YOUR files; ignore everything else

**Forbidden Git Commands — NO exceptions, NO "safe" variants:**
- `git reset` — ALL forms: `--hard`, `--soft`, `--mixed`, with paths, without paths. Even `git reset HEAD <file>` (unstaging) is forbidden — it's too close to destructive variants and Claude will reason its way into worse forms.
- `git restore` — ALL forms: working tree, staged, with `--source`, without
- `git checkout <file>` — when used to discard changes (navigating branches is fine)
- `git clean` — ALL forms
- `git stash` — ALL forms

**What to do instead:**
- Staging wrong? Just run `/jjc-pace-notch` with the correct file list — it handles staging
- Made a mistake? Make a new commit that fixes it — additive, not destructive
- Confused by repo state? ASK the user — another officium may be mid-work
- Need to undo something? Explain the situation to the user and let them decide

**Build & Run Discipline:**
Always run these after Rust code changes:
- `tt/vow-b.Build.sh` — Build
- `tt/vvw-r.RunVVX.sh` — Run VVX

**Important**: New commands are not available in this installation session. You must restart Claude Code before the new commands become available.

### Bridleability Assessment

When evaluating whether a pace is ready for autonomous execution (bridled state), apply these criteria:

**Bridleable** (can bridle for autonomous execution) — ALL must be true:
- **Mechanical**: Clear transformation, not design work
- **Pattern exists**: Following established pattern, not creating new one
- **No forks**: Single obvious approach, not "we could do X or Y"
- **Bounded**: Touches known files, not "find where this should go"

**NOT bridleable** (needs human judgment):
- Language like "define", "design", "architect", "decide"
- Establishing new patterns others will follow
- Multiple valid approaches requiring human choice
- Scope unclear or requires judgment calls

**Examples:**
- ✓ Bridleable: "Rename function `getCwd` to `getCurrentWorkingDirectory` across codebase"
- ✓ Bridleable: "Add error handling to `fetchUser` following pattern in `fetchOrder`"
- ✗ Not bridleable: "Define KitAsset struct and registry pattern" (design decisions)
- ✗ Not bridleable: "Improve performance of dashboard" (unclear scope, many approaches)

### Wrap Discipline

**NEVER auto-wrap a pace.** Always ask the user explicitly: "Ready to wrap ₢XXXXX?" and wait for confirmation before running `jjx_close`. The user decides when work is complete, not the agent.

This applies to:
- Direct work on rough paces
- Spawned agents executing bridled paces
- Any situation where work appears "done"

When work is complete, report outcomes and ask. Do not wrap.
