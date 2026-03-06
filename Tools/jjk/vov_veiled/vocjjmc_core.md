## Job Jockey Configuration

Job Jockey (JJ) is installed for managing project initiatives.

**Concepts:**
- **Heat**: Bounded initiative with coherent goals that are clear and present (3-50 officia). Status: `racing` (active execution) or `stabled` (paused for planning). Location: `current/` or `retired/` (done).
- **Pace**: Discrete action within a heat.
- **Itch**: Future work (any detail level), lives in jji_itch.md
- **Scar**: Closed work with lessons learned, lives in jjs_scar.md
- **Spook**: Team infrastructure stumble — any workflow failure improvable with deft attention. Capture as a pace when encountered, don't lose the current thread.

**Identities vs Display Names:**
- **Firemark**: Heat identity (`₣AA` or `AA`). Used in CLI args and JSON keys.
- **Coronet**: Pace identity (`₢AAAAk` or `AAAAk`). Used in CLI args and JSON keys.
- **Silks**: kebab-case display name. Human-readable only — NOT usable for lookups.

When a command takes `<firemark>` or `<coronet>`, provide the identity, not the silks.

- Target repo dir: `.`
- JJ Kit path: `Tools/jjk/README.md`

**CLI Syntax:**

JJK CLI syntax is non-standard. Do NOT guess based on common CLI conventions.
- Dockets go via stdin, not `--docket`
- Positioning uses `--move X --first`, not `--position first`
- **Verb names are NOT jjx_ subcommands**: there is no `jjx_slate`, `jjx_mount`, `jjx_notch`, `jjx_groom`, etc. The verb table below maps horse vocabulary to actual CLI commands.
- NEVER invent CLI flags — run `--help` first

**Heredoc for stdin**: Use `cat <<'DELIM' | jjx_*` pattern with quoted delimiter. The delimiter must not appear alone on any line in the content — if content shows heredoc examples with `EOF`, use a different delimiter like `DOCKET` or `PACESPEC`.

**Quick Verbs** — When user says just the verb, invoke the corresponding command:

| Verb | Noun | Command |
|------|------|---------|
| muster | heats | `jjx_list` |
| parade | heat/pace | `jjx_show` |
| scout | heats | `jjx_search` |
| nominate | heat | `jjx_create` |
| mount | heat/pace | See Mount Protocol below |
| groom | heat | See Groom Protocol below |
| slate | pace | `jjx_enroll FIREMARK --silks SILKS <stdin` |
| reslate | pace | `jjx_revise_docket CORONET <stdin` |
| notch | pace | See Commit Discipline below |
| wrap | pace | `jjx_close CORONET` |
| rail | heat | `jjx_reorder FIREMARK --move C --first\|--last\|--before C\|--after C` |
| furlough | heat | `jjx_alter FIREMARK --racing\|--stabled` |
| retire | heat | `jjx_archive FIREMARK [--execute]` |
| restring | heat | `jjx_transfer FIREMARK --to FIREMARK <stdin` |

**CLI Command Reference:**

```
jjx_show [TARGET] [--detail] [--remaining]
jjx_list [--status racing|stabled|retired]
jjx_create --silks SILKS
jjx_enroll FIREMARK --silks SILKS [--first|--before C|--after C] <stdin
jjx_reorder FIREMARK --move C --first|--last|--before C|--after C
jjx_alter FIREMARK [--racing|--stabled] [--silks SILKS]
jjx_record CORONET FILE [FILE...] [--intent "msg"]
jjx_close CORONET
jjx_log FIREMARK [--limit N]
jjx_search PATTERN [--actionable]
jjx_archive FIREMARK [--execute] [--size-limit BYTES]
jjx_transfer FIREMARK --to FIREMARK <stdin
jjx_continue FIREMARK
jjx_mark CORONET --marker M --description "text"
jjx_paddock FIREMARK [<stdin for set]
jjx_relocate CORONET --to FIREMARK [--first|--before C|--after C]
jjx_orient [FIREMARK]
jjx_revise_docket CORONET <stdin
jjx_relabel CORONET --silks "name"
jjx_drop CORONET
jjx_get_brief CORONET
jjx_get_coronets FIREMARK [--remaining] [--rough]
jjx_landing CORONET AGENT <stdin
jjx_validate [--file PATH]
```

**Composition Recipes:**

Common command combinations using `&&`:

```bash
# Revise docket and rename:
jjx_revise_docket C <docket_stdin && jjx_relabel C --silks "name"
```

**Key points:**
- `jjx_show` takes firemark OR coronet directly as sole argument — NO flags for target selection
  - Heat overview: `jjx_show AF`
  - Single pace: `jjx_show AFAAb`
  - Valid flags: `--detail`, `--remaining` only
- `jjx_orient` output includes `pace_coronet` (next actionable pace) — no separate show call needed to find what's next

### Mount Protocol

When user says "mount" or you need to engage the next pace:

1. Run `jjx_orient [FIREMARK]` to get context
2. Parse output: Racing-heats table, Heat/Paddock/Next/Docket/Recent-work sections
3. Display context to user: racing heats, heat silks, paddock summary, recent work, current pace and docket
4. **Name assessment**: If pace silks doesn't fit docket, offer rename via `jjx_relabel`
5. Analyze docket, propose approach (2-4 bullets), assess execution strategy:
   - Model tier: haiku (mechanical), sonnet (standard dev), opus (architectural)
   - Parallelization: file independence, task decomposability
   - State recommendation explicitly (e.g., "Sequential sonnet — single file")
6. Create chalk APPROACH marker: `jjx_mark CORONET --marker A --description "approach summary"`
7. Ask to proceed, then begin work

### Groom Protocol

When user says "groom":

1. Run `jjx_show FIREMARK --detail --remaining`
2. Display overview: heat silks, progress, remaining paces with dockets
3. Enter planning mode: suggest structural operations (slate new paces, rail to reorder, reslate to refine dockets, paddock review)

### Commit Discipline

When working on a heat, use `jjx_record` for commits with heat/pace affiliation.

**Pace-affiliated commit** (active pace provides context):
```bash
./tt/vvw-r.RunVVX.sh jjx_record CORONET file1 file2 --intent "description of changes"
```

**Heat-affiliated commit** (no active pace, but part of heat work):
```bash
./tt/vvw-r.RunVVX.sh jjx_record FIREMARK file1 file2 --intent "description of changes"
```

Synthesize intent from the conversation — describe *what* was accomplished, not *how*.

**Size guard**: If the commit fails due to size limits, report the failure to the user and ask how to proceed. Do not retry silently.

When user says "notch", determine context (pace or heat affiliated) and invoke `jjx_record` with the appropriate identity and explicit file list.

**Multi-Officium Discipline:**
Multiple Claude officia (concurrent git-activity streams, not sessions — see VOS `vost_officium`) may work concurrently in the same repo. The explicit file list in `jjx_record` enables orthogonal commits.

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
- Staging wrong? Run `jjx_record` with the correct file list — it handles staging
- Made a mistake? Make a new commit that fixes it — additive, not destructive
- Confused by repo state? ASK the user — another officium may be mid-work
- Need to undo something? Explain the situation to the user and let them decide

**Build & Run Discipline:**
Always run these after Rust code changes:
- `tt/vow-b.Build.sh` — Build
- `tt/vvw-r.RunVVX.sh` — Run VVX

**JJX Commands Are Self-Committing:**
`jjx_enroll`, `jjx_close`, `jjx_record`, and other state-mutating jjx commands create git commits internally. **`jjx_close` (wrap) commits ALL uncommitted changes** — code files and gallops state together in one commit. Do NOT follow `jjx_record` or `jjx_close` with another commit command — the tree will already be clean. If a commit command says "Nothing to commit", check `git status --short` and accept the result.

**Diagnose Before Escalating:**
When a command fails, check the simplest explanation first. "Nothing to commit" means the tree is clean — verify with `git status`, don't try creative workarounds. "Unexpected argument" means wrong CLI syntax — read `--help`, don't guess. One diagnostic command beats three speculative retries.

### Wrap Discipline

**NEVER auto-wrap a pace.** Always ask the user explicitly: "Ready to wrap ₢XXXXX?" and wait for confirmation before running `jjx_close`. The user decides when work is complete, not the agent.

When work is complete, report outcomes and ask. Do not wrap.

When wrapping (after user confirms), always pipe a one-line summary of the work via stdin:
`echo "Added bitmap displays to orient output" | jjx_close CORONET`
The agent always has context about what was accomplished — include it.

**Wrap commits everything.** `jjx_close` stages and commits all dirty files (code edits + gallops state) in one commit. Do NOT notch before or after wrapping — the wrap IS the final commit. If you want separate commits for intermediate code milestones, notch during work; remaining uncommitted changes are captured by wrap.
