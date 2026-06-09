## Job Jockey Configuration

Job Jockey (JJ) is installed for managing project initiatives.

**Concepts:**
- **Heat**: Bounded initiative with coherent goals that are clear and present (3-50 officia). Status: `racing` (active execution) or `stabled` (paused for planning). Location: `current/` or `retired/` (done).
- **Pace**: Discrete action within a heat.
- **Itch**: Future work (any detail level), lives in jji_itch.md
- **Scar**: Closed work with lessons learned, lives in jjs_scar.md
- **Spook**: Team infrastructure stumble — any workflow failure improvable with deft attention. Capture as a pace when encountered, don't lose the current thread.
- **Cinch**: A decision settled in a paddock or docket and not to be re-litigated — both noun and verb ("cinch the approach," "the inscribe-skip is cinched"). Distinct from a *lock* in the concurrency sense (JJK's git-ref commit lock), which keeps that word.

**Sincerity over efficiency:** When you notice something — a pattern, a concern, an insight about the work or the collaboration itself — say it. Discovery through conversation is part of the work, not a detour from it.

**Docket posture:** A docket is a specification of needed change, not architectural commentary. It articulates what done looks like — the completion criterion goes under a `## Done when` heading (never a bare `## Done`, which misreads as a record of work already finished) — and any cinched constraints, which go under a `## Cinched` heading (never `## Locked`: *lock* is reserved for the concurrency-sense git-ref commit lock); it points at sources rather than restating them. A short `## Character` line naming the cognitive posture (e.g., "intricate but mechanical," "design conversation requiring judgment") earns its keep; the rest of the docket should resist filling in.

**Slate-time vs mount-time.** The slate agent has just done analysis; the mount agent will reorient against the project as it stands then, with CLAUDE.md and specs already loaded. Hand off goal and boundary, not the analysis. Depth belongs in the slate commit message, not the docket body.

**Stale-by-mount filter.** Each line must still be true at mount-time. Fail (drift): line numbers, byte/match counts, version strings, exhaustive site enumerations. Pass (durable): file paths as entry points, commit hashes, spec/concept names.

**Docket anti-patterns:**
- Restating commits/specs the mount agent already loads. Point, don't paraphrase.
- Pre-baking implementation (function names, signatures, locations for new code). Mount-time decisions.
- Site enumerations. Write a discovery recipe (`grep "pattern" Tools/rbk/`) instead of listing six file:line entries.
- Body over ~15 lines warrants suspicion of overprescription.
- **Plan-step structure stays in the plan.** Docket phase/step labels (A/B/C, "Phase 1", "first/then/finally") must not appear as code comments — the plan won't exist at maintenance time, and line order already conveys execution.
- **Absolute paths to working trees.** Don't pin a docket to a specific repo clone (`/Users/foo/projects/your-project`, `~/proj/`). Mount may legitimately run against a different clone — if work needs to happen in *the* working directory, say so without naming it. Cross-repo coupling, if real, belongs in the paddock as a heat-shape note, not pace prose. Absolute paths *are* acceptable for external data roots (log dirs, `/tmp/...` scratch, machine-pinned data) — but flag those as operator-mutable rather than identity.

**Reference discipline.** Pace order is the dependency tree — single-operator workflow runs paces in heat order, so explicit dependency markers in docket prose are usually overspecified. Coronet cross-refs in dockets earn their keep only when the dependency crosses heats or skips order — rare, not never.

**Paddock posture.** A paddock articulates shape, cinched decisions, and what done looks like — not a progress journal. Git log and `jjx_log` are the journal; the paddock is the shape. **Coronets do not appear in paddock prose** — not retrospectively (annotating landed work, e.g., "BBAAM depot-identity-collapse"), not prospectively (naming planned paces, e.g., "AAF — Docker dual-daemon"), not as cross-references. A coronet is an enrollment-ledger key, not shape data; pace-state operations (drop, relabel, reorder, transfer) silently invalidate coronet refs while the prose keeps them. Refer to paces by purpose, not identifier; `jjx_show` is the authoritative source for what paces exist. Firemarks may appear (heats change rarely). When editing a paddock for other reasons, prune any coronet refs you find.

**Silks never appear in artifact prose.** Paddock and docket *prose* must never name heat silks — neither a heat's own silks nor another heat's. Silks are display names that evolve via `jjx_relabel`; the moment one drifts, the prose that quoted it is stale and quietly misleads. This is the same failure mode as the coronet-in-paddock rule above, applied to the third identity type, and it completes a three-way distinction by how each kind fares against the lifecycle:
- **Firemarks** — lifecycle-bound (a heat keeps its firemark for life) → fine in both paddock and docket prose.
- **Coronets** — stable but invalidated by pace-state ops (drop, relabel, reorder, transfer) → barred from paddock prose; in docket prose only for genuine cross-order or cross-heat dependencies (see **Reference discipline**).
- **Silks** — evolve freely via relabel → barred from *both* paddock and docket prose (this rule).

Refer to a heat or pace by its purpose, not its display name; the firemark — or, where dockets allow, the coronet — is the durable handle. When editing a paddock or docket for other reasons, prune any silks-shaped kebab strings you find, as you would a stray coronet.

**Mid-execution posture.** When a failure or surprise surfaces while a pace is mounted, the default response is mechanism + one specific repair you'd attempt. Do not proliferate options or weigh consequences across alternatives — pace scope and segmentation are operator territory. If the repair is obvious, proceed; if not, surface the question and stop.

**Identities vs Display Names:**
- **Firemark**: Heat identity, two characters from a nonstandard b64 alphabet (e.g., `₣AA`, `₣Av`, `₣A-`, `₣A_`, `₣A7`). Used in command params and JSON keys.
- **Coronet**: Pace identity, firemark plus three characters from the same alphabet (e.g., `₢AAAAk`, `₢A-AAm`, `₢A_A-p`, `₢AAA_7`). Used in command params and JSON keys.
- **Silks**: kebab-case display name. Human-readable only — NOT usable for lookups.

When a command takes a firemark or coronet, provide the identity, not the silks.

**Display discipline (agent text output to user):**
- **Always show full coronets and firemarks; never abbreviate.** Write
  `₢A-AA-`, never `AA-`. Write `₣A-`, never `A-`. The operator works in
  several heats simultaneously; a bare 3-char pace suffix is ambiguous and
  forces re-derivation of the heat from context. The ₣/₢ glyph + heat half
  is part of the identity, not decoration. This holds in prose, tables,
  bullets, headers, and casual back-references.
- **Prefer coronets over silks in references.** Coronets are precise and
  immutable; silks change via `jjx_relabel`. Lead with the coronet; silks
  may follow parenthetically for human readability (e.g.,
  `₢A-AA- (wsg-trim-to-spec-shape)`). The coronet alone is always
  sufficient; the silks alone never are.

**Case sensitivity**: Firemarks and coronets are case-sensitive. `Av` ≠ `AV` ≠ `av`. Passing the wrong case produces a confusing "not found" error. Copy identities exactly as displayed — the final character's case distinguishes heats (e.g., `₣Av` vs `₣AV` are different heats).

- Target repo dir: `.`
- JJ Kit path: `Tools/jjk/README.md`

**MCP Tool Usage:**

All JJK commands are accessed via the single `mcp__vvx__jjx` MCP tool with four parameters:
- `command`: string selecting the operation — always the canonical `jjx_*` name (e.g., `"jjx_show"`, `"jjx_enroll"`, `"jjx_record"`)
- `params`: JSON object with command-specific fields (see reference below)
- `officium`: officium identity string from `jjx_open` (required on all commands except `jjx_open` — see Officium Protocol below)
- `model`: agent's verbatim model ID string from its system prompt (e.g., `"claude-opus-4-8"`). Required on ALL commands including `jjx_open`. The server gates commands by model tier — currently all commands require opus.

**`params` must be a JSON object, never a string.** If params is accidentally stringified (e.g., `"{\"key\": \"val\"}"` instead of `{"key": "val"}`), deserialization will fail. The server has a defensive fallback for this, but always pass a native object.

**Verb names are NOT command names**: there is no `jjx_slate`, `jjx_mount`, `jjx_notch`, `jjx_groom` command. The verb table below maps horse vocabulary to actual MCP commands.
NEVER invent param fields — check the reference below first.

**Quick Verbs** — When user says just the verb, invoke the corresponding command:

| Verb | Noun | MCP command |
|------|------|-------------|
| muster | heats | `jjx_list` |
| parade | heat/pace | `jjx_show` |
| scout | heats | `jjx_search` |
| nominate | heat | `jjx_create` |
| mount | heat/pace | See Mount Protocol below |
| groom | heat | See Groom Protocol below |
| slate | pace | `jjx_enroll` |
| chivvy | pace | `jjx_enroll {first: true}` |
| cantle | pace | `jjx_enroll {after: <current_coronet>}` |
| reslate | pace | `jjx_redocket` |
| notch | pace | See Commit Discipline below |
| wrap | pace | `jjx_close` |
| rail | heat | `jjx_reorder` |
| furlough | heat status/silks | `jjx_alter` |
| retire | heat | `jjx_archive` |
| restring | heat | `jjx_transfer` |
| foray | remote dispatch | See Foray Protocol below |

**MCP Command Reference:**

All params are JSON objects. `?` = optional, `[]` = array. Booleans default to false.

```
jjx_open           {}
jjx_show           {target?, detail?, remaining?}
jjx_list           {status?}
jjx_orient         {firemark}
jjx_create         {silks}
jjx_enroll         {firemark, before?, after?, first?, size_limit?} # silks+docket via gazette_in.md only
jjx_reorder        {firemark, move?, before?, after?, first?, last?}
jjx_alter          {firemark, racing?, stabled?, silks?}
jjx_record         {identity, files[], size_limit?, intent?}
jjx_close          {coronet, summary?, size_limit?}
jjx_log            {firemark, limit?}
jjx_search         {pattern, actionable?}
jjx_archive        {firemark, size_limit?}
jjx_transfer       {firemark, to, coronets, size_limit?}
jjx_continue       {firemark}
jjx_paddock        {firemark?, note?, size_limit?}                  # gazette_in.md to set, gazette_out.md to get
jjx_relocate       {coronet, to, before?, after?, first?}
jjx_redocket       {size_limit?}                                    # docket via gazette_in.md only; supports mass reslate
jjx_relabel        {coronet, silks}
jjx_drop           {coronet}
jjx_brief      {coronet}                                            # raw docket text for ONE pace, returned inline (no gazette) — the clean single-docket read
jjx_coronets   {firemark, remaining?, rough?}                       # coronet IDs in heat order, one per line, inline — no silks, no docket
jjx_landing        {coronet, agent, content?}
jjx_validate       {}
jjx_bind           {alias, reldir}                                  # remote: create legatio session (alias resolves BURN profile)
jjx_send           {legatio, command}                               # remote: synchronous exec on fundus
jjx_plant          {legatio, commit}                                # remote: reset fundus to exact commit
jjx_relay          {legatio, tabtarget, timeout, firemark}          # remote: async dispatch via nohup
jjx_check          {pensum, timeout}                                # remote: probe/poll pensum status
jjx_fetch          {legatio, path}                                  # remote: read single file from fundus
```

**Key points:**
- `jjx_show` takes firemark OR coronet in the `target` param
  - Heat overview: `{"target": "AF"}`
  - Single pace: `{"target": "AFAAb"}`
  - Additional params: `detail`, `remaining` only
- `jjx_orient` output includes next actionable pace — no separate show call needed
- **Gazette output**: `jjx_orient`, `jjx_show` (with detail), and `jjx_paddock` (getter) write `gazette_out.md` with paddock and pace docket notices. Read the gazette file after these commands to get full content.
- **Never reach past the JJK interface to raw storage — NO exceptions.** Do not parse the harness's persisted tool-result files or the gallops JSON (`.claude/jjm/jjg_gallops.json`) directly. To read one pace's docket, call `jjx_brief {coronet}` (returns inline). To read full paddock/dockets after `jjx_show`/`jjx_orient`, read the `gazette_out.md` file directly. When a large `jjx_show` overflows the display and the harness persists the tool result, re-read `gazette_out.md` or loop `jjx_brief` per pace — never scrape the persisted blob. Same discipline as "never read regime files directly, go through the CLI."
- **Gazette input**: `jjx_enroll`, `jjx_redocket`, and `jjx_paddock` (setter) read docket/content from `gazette_in.md`. Gazette is the sole input path for docket content — no JSON param fallback.
- `jjx_redocket` supports **mass reslate**: multiple `# jjezs_reslate <coronet>` notices in a single `gazette_in.md`, each with its own docket body. All paces updated in one call.
- `jjx_paddock` `note` param: optional short string appended to the paddock discussion commit message (e.g., `{"note": "updated after spook fix"}`)
- `jjx_close` takes `summary` as a string param (not stdin pipe)
- `jjx_record` takes `files` as a native JSON array: `["file1.rs", "file2.rs"]`
- `jjx_transfer` takes `coronets` as a JSON-encoded string (not a native array): `"[\"AYAAA\", \"AYAAB\"]"`

### Officium Protocol

Each chat session must open an officium before using any jjx commands.

1. **At chat start**, call `jjx_open` (no params, no officium field). It returns a ☉-prefixed identity string (e.g., `☉260327-1000`).
2. **On every subsequent jjx call**, pass the returned identity as the `officium` field on the MCP tool (sibling to `command` and `params`).
3. **Self-healing**: If any jjx command fails with "Officium directory not found", call `jjx_open` again to create a fresh officium, then retry.

**⚠️ `jjx_open` must land alone — never co-batch it with any call that consumes its result.** The officium ID is a server-minted opaque token; it cannot be guessed, predicted, or pattern-matched from a prior session. Issue `jjx_open`, read the returned ☉-id, and only *then* issue `jjx_show`/`jjx_orient`/`jjx_list`/etc. in a later tool block. Inventing a plausible-looking officium ID to fill a parallel batch is the classic self-inflicted "Officium directory not found" — the same trap applies to any "mint an ID → use the ID" pair (`jjx_create` → operate on the new firemark, `jjx_bind` → use the legatio token, `jjx_relay` → use the pensum token).

The ☉ (U+2609 SUN) prefix parallels ₣/₢ for firemarks/coronets. Pass it exactly as returned — the dispatcher strips it. **On disk the officium directory name has the ☉ stripped** (`.claude/jjm/officia/260327-1000/`, never `☉260327-1000`) — relevant only when constructing a gazette path by hand (see Gazette paths below).

Gazette file exchange uses two directional files in the officium exchange directory. Every jjx MCP call unconditionally deletes both gazette files on entry (read+delete `gazette_in.md`, delete `gazette_out.md`). Gazette content has single-MCP-call lifetime — it is a parameter or a return value, not persistent state.

- **`gazette_in.md`** (agent → server): write before calling a setter command (`jjx_enroll`, `jjx_redocket`, `jjx_paddock` setter).
- **`gazette_out.md`** (server → agent): written by getter commands, read after they return. The next jjx call of any kind deletes it.
  - `jjx_orient` → `# jjezs_paddock <firemark>` + paddock content, `# jjezs_pace <coronet>` + docket content (for next actionable pace)
  - `jjx_show` (with detail) → `# jjezs_paddock <firemark>` + paddock content, `# jjezs_pace <coronet>` + docket per pace
  - `jjx_paddock` (getter) → `# jjezs_paddock <firemark>` + paddock content

**Gazette wire format (setter commands):**
Each notice is a `#`-header line with slug and lede, followed by content body. The lede is **exactly one whitespace-free token** (silks / coronet / firemark) — nothing follows it on the `#` line; the body goes on the lines beneath. This is uniform across every input slug: `jjezs_slate` takes silks, `jjezs_reslate` takes a coronet, `jjezs_paddock` takes a firemark, and in each case appending extra text to the lede folds it into the identity and fails validation. Write `gazette_in.md`, then call the command.

**Critical: `#` (H1) in gazette_in.md is a wire format delimiter, NOT a markdown heading.** For single-notice commands (enroll, paddock set), use exactly ONE `#` line. For mass reslate, each `# jjezs_reslate` line starts a new notice. All markdown headings within body content must use `##` or deeper — a bare `#` line inside content will be parsed as a notice boundary.

Wrong (parsed as TWO notices, fails with `unknown slug 'Paddock:'`):
```
# jjezs_paddock ₣BO
# Paddock: rbk-11-mvp-tactical
body...
```

Right:
```
# jjezs_paddock ₣BO
## Paddock: rbk-11-mvp-tactical
body...
```

**On failure:** If a gazette setter command fails, `gazette_in.md` is already consumed (deleted on entry). You must re-write it from scratch before retrying — there is nothing to re-read.

| Command | Write to `gazette_in.md` | Then call with params |
|---------|--------------------------|----------------------|
| `jjx_enroll` | `# jjezs_slate <silks>` + docket body | `{"firemark": "XX", "before?": ..., "after?": ..., "first?": ...}` |
| `jjx_redocket` | `# jjezs_reslate <coronet>` + docket body | `{}` (coronet is in gazette lede) |
| `jjx_redocket` (mass) | Multiple `# jjezs_reslate <coronet>` notices, each with docket body | `{}` |
| `jjx_paddock` (set) | `# jjezs_paddock <firemark>` + content body | `{"note?": "commit annotation"}` |

Gazette paths: `.claude/jjm/officia/<id>/gazette_in.md` and `gazette_out.md`, where **`<id>` is the officium id with the ☉ prefix stripped** (the dispatcher strips it; on disk the directory is `260327-1000`, never `☉260327-1000`).

**Construct the gazette path directly from the known officium id — never `find` for it.** Gazette files have single-MCP-call lifetime (deleted on entry, written only by getters on success), so a filesystem search will usually find nothing or a stale file, and an empty search result then poisons any command built on it. If a getter command *failed*, no gazette was written — fix the command, don't hunt the filesystem.

**Read-modify-write workflow** (paddock editing):
1. Call `jjx_paddock` getter → reads `gazette_out.md`
2. Rename `gazette_out.md` → `gazette_in.md`, edit content
3. Call `jjx_paddock` setter

### Parallel Tool Batch Discipline

When several tool calls share one parallel block and one of them exits non-zero, the harness **cancels the still-queued siblings** ("Cancelled: parallel tool call … errored"). A screenful of cancellations therefore almost always traces to *one* real error upstream — diagnose that single failure, don't react to the cascade. Two rules keep jjx work out of this trap:

- **Never co-batch a producer with its consumer.** A command and another that needs its output, its minted id, or a path built from it must be sequenced across tool blocks, not issued together. (`jjx_open` is the canonical case — see the Officium Protocol barrier above.) This includes batching a state-mutating command like `jjx_record` alongside the edits or checks that are supposed to *precede* it: verify the tree first, commit second.
- **Keep fragile or dependent commands out of wide read-only batches.** An empty/failed sibling can silently cancel unrelated reads, which then reads as a far bigger failure than it is.

### Mount Protocol

When user says "mount" or you need to engage the next pace:

1. Run `jjx_orient` command with the firemark. **Firemark is required** — if the user says "mount" without specifying a heat and you have no prior heat context in this session, ask which heat to mount rather than guessing. If you have prior context (previously mounted/groomed heat), use that firemark.
2. Parse output: Racing-heats table, Heat/Next/Docket/Recent-work sections. Read paddock and pace docket from the gazette file written to the officium exchange directory.
3. **Read the paddock before the docket.** The paddock tells you the shape of the work and what's been learned; the docket tells you what to do next. Orientation before action.
4. **Lead with the pace goal in one sentence**, distilled from the docket — what this pace is trying to accomplish, stated precisely. Paces can sit weeks between slating and mounting; the operator needs goal recall first, not heat scenery. Then display brief heat context (silks + paddock one-liner + recent work) and finally the full docket.
5. **Name assessment**: If pace silks doesn't fit docket, offer rename via `jjx_relabel`
6. Propose the approach compactly — a one-line recommended execution strategy (model tier + sequential/parallel), not a multi-part briefing. Offer the reasoning (tier rationale, parallelization analysis) on request, not unprompted. Tiers: haiku (mechanical), sonnet (standard dev), opus (architectural).
7. Ask to proceed, then begin work

### Groom Protocol

When user says "groom":

1. Run `jjx_show` command with `{target: FIREMARK, detail: true, remaining: true}`
2. Read `gazette_out.md` directly for full paddock and pace docket content — never the persisted tool-result blob or `jjg_gallops.json` (see "Never reach past the JJK interface" under Key points)
3. Display overview: heat silks, progress, remaining paces with dockets (from gazette)
4. Enter planning mode: suggest structural operations (slate new paces, rail to reorder, reslate to refine dockets, paddock review)

### Foray Protocol

When user says "foray" or asks to run something on a remote machine:

**Concepts:**
- **Legatio**: A standing SSH session to a remote project (fundus). Created by `jjx_bind`, identified by a token (L0, L1, ...). Persists for the officium lifetime.
- **Pensum**: An async dispatched job within a legatio. Created by `jjx_relay`, identified by a `₱`-prefixed token. Each pensum maps to a remote temp directory on the fundus.
- **Curia**: The invoking side (where Claude runs). **Fundus**: The executing side (where the tabtarget runs).

**Fundus constants** (use these for `jjx_bind`): the default reldir and BURN alias are project-specific — get them from your project's veiled `claude-jjk-*.md` (if present) or the target's BUK Regime Node profile.

**Workflow:**

1. **Bind** (once per session per host): `jjx_bind` with `{alias: "<BURN_ALIAS>", reldir: "<your-project-reldir>"}`. Resolves the BURN profile on the curia to extract host/user/command, then probes via SSH. Returns legatio token.
2. **Ensure curia is clean and pushed**: notch if needed, `git push` if needed. `jjx_relay` will refuse dispatch if working tree is dirty or HEAD is unpushed.
3. **Plant**: `jjx_plant` with `{legatio: "L0", commit: "<HEAD SHA>"}`. Resets fundus to exact commit.
4. **Relay**: `jjx_relay` with `{legatio: "L0", tabtarget: "<filename>.sh", timeout: <seconds>, firemark: "<heat>"}`. Returns pensum token.
5. **Check**: `jjx_check` with `{pensum: "<token>", timeout: 0}` for instant probe, or `timeout: <seconds>` to poll until terminal. Returns BURX fields + liveness report. On terminal status, also returns file list from remote temp dir.
6. **Fetch** (selective): `jjx_fetch` with `{legatio: "L0", path: "<absolute-path>"}` to retrieve specific result files. Use paths from the check file list.

**Liveness states** (from `jjx_check`):
- **running**: BURX_EXIT_STATUS absent, PID alive
- **orphaned**: BURX_EXIT_STATUS absent, PID dead (crashed without writing status)
- **stopped**: BURX_EXIT_STATUS present (check exit code)
- **lost**: burx.env file missing

**`jjx_send`** is for synchronous one-off commands (no pensum, no polling). Use when the command is short and you need inline results.

**Windows fundus body discipline:** When the fundus is a Windows host, both `jjx_send` and `jjx_relay`-dispatched tabtargets traverse the cmd.exe → wsl.exe / cygwin / PowerShell transport stack. Body authoring rules (escape `\$name` for w-letter wsl.exe transit, no heredocs, single-line `;`-joined for cmd.exe transit, lazy-flush avoidance for PowerShell cmdlets, etc.) live in your project's Windows scripting guide, if present. Empirical record under `Memos/memo-YYYYMMDD-windows-transport-{topic}.md`.

### Commit Discipline

When working on a heat, use `jjx_record` for commits with heat/pace affiliation.

**Pace-affiliated commit** (active pace provides context):
Use `jjx_record` with `{identity: "CORONET", files: ["file1", "file2"], intent: "description"}`

**Heat-affiliated commit** (no active pace, but part of heat work):
Use `jjx_record` with `{identity: "FIREMARK", files: ["file1", "file2"], intent: "description"}`

Synthesize intent from the conversation — describe *what* was accomplished, not *how*.

**Size guard — ALL commands accepting `size_limit?` (record, close, archive, paddock, enroll, redocket, transfer)**: If ANY jjx command fails due to size limits, STOP. Report the byte count, limit, and per-file breakdown to the user, then WAIT for the user's review. The guard is a byte-sanity review step — its purpose is catching unintended bulk (e.g., a binary that snuck in), not gating decomposition. The default is uniform 50KB across all seven commands. Expected outcome: the user confirms the bytes are legitimate and directs you to raise the limit. Do NOT offer splitting as a parallel option — splitting enters the picture only if the user explicitly raises it, because the work genuinely decomposes. NEVER auto-override `size_limit`.

When user says "notch", determine context (pace or heat affiliated) and invoke `jjx_record` with the appropriate identity and explicit file list.

**Multi-Officium Discipline:**
Multiple Claude officia (concurrent chat sessions, each with its own ☉-prefixed officium ID) may work concurrently in the same repo. The explicit file list in `jjx_record` enables orthogonal commits.

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

**JJX Commands Are Self-Committing:**
`jjx_enroll`, `jjx_close`, `jjx_record`, and other state-mutating jjx commands create git commits internally. **`jjx_close` (wrap) commits ALL uncommitted changes** — code files and gallops state together in one commit. Do NOT follow `jjx_record` or `jjx_close` with another commit command, and do NOT notch immediately before or after a wrap — the tree will already be clean. If a commit command says "Nothing to commit", check `git status --short` and accept the result. (For intermediate milestones, notch *during* work; whatever remains is captured by the wrap.)

**Diagnose Before Escalating:**
When a command fails, check the simplest explanation first. "Invalid params" means wrong field names — check the MCP Command Reference above, don't guess. One diagnostic command beats three speculative retries.

### Wrap Discipline

**NEVER auto-wrap a pace.** Always ask the user explicitly: "Ready to wrap ₢XXXXX?" and wait for confirmation before running `jjx_close`. The user decides when work is complete, not the agent.

When work is complete, report outcomes and ask. Do not wrap.

When wrapping (after user confirms), always include a summary of the work:
Use `jjx_close` with `{coronet: "CORONET", summary: "Added bitmap displays to orient output"}`
The agent always has context about what was accomplished — include it.

**Wrap is unscoped — known JJK bug, do NOT "fix" it.** Unlike `notch`/`jjx_record` (explicit file list), `jjx_close` (wrap) stages and commits **every** dirty file in the tree — your code, the gallops state, and anything another officium left uncommitted. Wrap is not yet as file-specific as notch; that gap is a known bug, deferred pending the planned git-worktrees switch — do not attempt to repair it. For now: before wrapping, make sure the tree holds only your work, or expect wrap to sweep all of it into one commit.
