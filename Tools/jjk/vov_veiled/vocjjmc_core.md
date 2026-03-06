## Job Jockey Configuration

Job Jockey (JJ) is installed for managing project initiatives.

**Concepts:**
- **Heat**: Bounded initiative with coherent goals that are clear and present (3-50 officia). Status: `racing` (active execution) or `stabled` (paused for planning). Location: `current/` or `retired/` (done).
- **Pace**: Discrete action within a heat.
- **Itch**: Future work (any detail level), lives in jji_itch.md
- **Scar**: Closed work with lessons learned, lives in jjs_scar.md
- **Spook**: Team infrastructure stumble — any workflow failure improvable with deft attention. Capture as a pace when encountered, don't lose the current thread.

**Identities vs Display Names:**
- **Firemark**: Heat identity (`₣AA` or `AA`). Used in command params and JSON keys.
- **Coronet**: Pace identity (`₢AAAAk` or `AAAAk`). Used in command params and JSON keys.
- **Silks**: kebab-case display name. Human-readable only — NOT usable for lookups.

When a command takes a firemark or coronet, provide the identity, not the silks.

- Target repo dir: `.`
- JJ Kit path: `Tools/jjk/README.md`

**MCP Tool Usage:**

All JJK commands are accessed via the single `mcp__vvx__jjx` MCP tool with two parameters:
- `command`: string selecting the operation (e.g., `"show"`, `"enroll"`, `"record"`)
- `params`: JSON object with command-specific fields (see reference below)

The tool strips any `jjx_` prefix, so both `"record"` and `"jjx_record"` work.

**Verb names are NOT command names**: there is no `slate`, `mount`, `notch`, `groom` command. The verb table below maps horse vocabulary to actual MCP commands.
NEVER invent param fields — check the reference below first.

**Quick Verbs** — When user says just the verb, invoke the corresponding command:

| Verb | Noun | MCP command |
|------|------|-------------|
| muster | heats | `list` |
| parade | heat/pace | `show` |
| scout | heats | `search` |
| nominate | heat | `create` |
| mount | heat/pace | See Mount Protocol below |
| groom | heat | See Groom Protocol below |
| slate | pace | `enroll` |
| reslate | pace | `revise_docket` |
| notch | pace | See Commit Discipline below |
| wrap | pace | `close` |
| rail | heat | `reorder` |
| furlough | heat | `alter` |
| retire | heat | `archive` |
| restring | heat | `transfer` |

**MCP Command Reference:**

All params are JSON objects. `?` = optional, `[]` = array. Booleans default to false.

```
show           {target?, detail?, remaining?}
list           {status?}
orient         {firemark?}
create         {silks}
enroll         {firemark, silks, docket, before?, after?, first?}
reorder        {firemark, move?, before?, after?, first?, last?}
alter          {firemark, racing?, stabled?, silks?}
record         {identity, files[], size_limit?, intent?}
close          {coronet, summary?, size_limit?}
log            {firemark, limit?}
search         {pattern, actionable?}
archive        {firemark, execute?, size_limit?}
transfer       {firemark, to, coronets}
continue       {firemark}
mark           {identity, marker, description}
paddock        {firemark, content?, note?}
relocate       {coronet, to, before?, after?, first?}
revise_docket  {coronet, docket}
relabel        {coronet, silks}
drop           {coronet}
get_brief      {coronet}
get_coronets   {firemark, remaining?, rough?}
landing        {coronet, agent, content?}
validate       {}
arm            {coronet, warrant}
```

**Key points:**
- `show` takes firemark OR coronet in the `target` param
  - Heat overview: `{"target": "AF"}`
  - Single pace: `{"target": "AFAAb"}`
  - Additional params: `detail`, `remaining` only
- `orient` output includes next actionable pace — no separate show call needed
- `enroll` takes `docket` as a string param (not stdin)
- `close` takes `summary` as a string param (not stdin pipe)
- `record` takes `files` as a JSON array: `["file1.rs", "file2.rs"]`
- `transfer` takes `coronets` as a string (newline-separated coronet list)

### Mount Protocol

When user says "mount" or you need to engage the next pace:

1. Run `orient` command (with optional firemark) to get context
2. Parse output: Racing-heats table, Heat/Paddock/Next/Docket/Recent-work sections
3. Display context to user: racing heats, heat silks, paddock summary, recent work, current pace and docket
4. **Name assessment**: If pace silks doesn't fit docket, offer rename via `relabel`
5. Analyze docket, propose approach (2-4 bullets), assess execution strategy:
   - Model tier: haiku (mechanical), sonnet (standard dev), opus (architectural)
   - Parallelization: file independence, task decomposability
   - State recommendation explicitly (e.g., "Sequential sonnet — single file")
6. Create chalk APPROACH marker via `mark` command with `marker: "A"`
7. Ask to proceed, then begin work

### Groom Protocol

When user says "groom":

1. Run `show` command with `{target: FIREMARK, detail: true, remaining: true}`
2. Display overview: heat silks, progress, remaining paces with dockets
3. Enter planning mode: suggest structural operations (slate new paces, rail to reorder, reslate to refine dockets, paddock review)

### Commit Discipline

When working on a heat, use the `record` command for commits with heat/pace affiliation.

**Pace-affiliated commit** (active pace provides context):
Use `record` with `{identity: "CORONET", files: ["file1", "file2"], intent: "description"}`

**Heat-affiliated commit** (no active pace, but part of heat work):
Use `record` with `{identity: "FIREMARK", files: ["file1", "file2"], intent: "description"}`

Synthesize intent from the conversation — describe *what* was accomplished, not *how*.

**Size guard**: If the commit fails due to size limits, report the failure to the user and ask how to proceed. Do not retry silently.

When user says "notch", determine context (pace or heat affiliated) and invoke `record` with the appropriate identity and explicit file list.

**Multi-Officium Discipline:**
Multiple Claude officia (concurrent git-activity streams, not sessions — see VOS `vost_officium`) may work concurrently in the same repo. The explicit file list in `record` enables orthogonal commits.

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
- Staging wrong? Run `record` with the correct file list — it handles staging
- Made a mistake? Make a new commit that fixes it — additive, not destructive
- Confused by repo state? ASK the user — another officium may be mid-work
- Need to undo something? Explain the situation to the user and let them decide

**Build & Run Discipline:**
Always run these after Rust code changes:
- `tt/vow-b.Build.sh` — Build
- `tt/vvw-r.RunVVX.sh` — Run VVX

**JJX Commands Are Self-Committing:**
`enroll`, `close`, `record`, and other state-mutating commands create git commits internally. **`close` (wrap) commits ALL uncommitted changes** — code files and gallops state together in one commit. Do NOT follow `record` or `close` with another commit command — the tree will already be clean. If a commit command says "Nothing to commit", check `git status --short` and accept the result.

**Diagnose Before Escalating:**
When a command fails, check the simplest explanation first. "Nothing to commit" means the tree is clean — verify with `git status`, don't try creative workarounds. "Invalid params" means wrong field names — check the MCP Command Reference above, don't guess. One diagnostic command beats three speculative retries.

### Wrap Discipline

**NEVER auto-wrap a pace.** Always ask the user explicitly: "Ready to wrap ₢XXXXX?" and wait for confirmation before running `close`. The user decides when work is complete, not the agent.

When work is complete, report outcomes and ask. Do not wrap.

When wrapping (after user confirms), always include a summary of the work:
Use `close` with `{coronet: "CORONET", summary: "Added bitmap displays to orient output"}`
The agent always has context about what was accomplished — include it.

**Wrap commits everything.** `close` stages and commits all dirty files (code edits + gallops state) in one commit. Do NOT notch before or after wrapping — the wrap IS the final commit. If you want separate commits for intermediate code milestones, notch during work; remaining uncommitted changes are captured by wrap.
