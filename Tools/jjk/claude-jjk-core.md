## Job Jockey Configuration

Job Jockey (JJ) is installed for managing project initiatives.

**Concepts:**
- **Heat**: Bounded initiative with coherent goals that are clear and present (3-50 officia). Status: `racing` (active execution) or `stabled` (paused for planning). Location: `current/` or `retired/` (done).
- **Pace**: Discrete action within a heat.
- **Itch**: Future work (any detail level), lives in jji_itch.md. A **human reminder only** — never load-bearing, never authority. The operator reads it and decides whether it graduates to work; an agent must not cut paces or implement against itch content as if it were authority. Load-bearing guidance belongs where agents read it as authority — heat-shape in a paddock, durable facts in a spec.
- **Scar**: Closed work with lessons learned, lives in jjs_scar.md
- **Spook**: Team infrastructure stumble — any workflow failure improvable with deft attention. Capture as a pace when encountered, don't lose the current thread.
- **Cinch**: A decision settled in a paddock or docket and not to be re-litigated — both noun and verb. Distinct from a *lock* in the concurrency sense (JJK's git-ref commit lock), which keeps that word.
- **Mulling heat**: A stabled heat paired with an aspirant sheaf — aspirancy is declared by the sheaf's `//axvd_sheaf axd_aspirant` voicing, never inferred from prose. Design leans bank in the sheaf; work-shape and cinches bank in the paddock — read the paddock before cutting any pace. Sheaf-authoring paces may race; implementation paces wait for *settling*, never for infusion. Full doctrine: JJS0 "The Mulling Heat" (Aspirant Nucleations).

**Sincerity over efficiency:** When you notice something — a pattern, a concern, an insight about the work or the collaboration — say it. Discovery through conversation is part of the work, not a detour from it.

**Docket posture:** A docket is a specification of needed change, not architectural commentary. It articulates what done looks like — the completion criterion goes under a `## Done when` heading (never a bare `## Done`, which misreads as a record of work already finished) — and any cinched constraints, which go under a `## Cinched` heading (never `## Locked`: *lock* is reserved for the concurrency-sense git-ref commit lock); it points at sources rather than restating them. A short `## Character` line naming the cognitive posture earns its keep; the rest of the docket should resist filling in.

**Slate-time vs mount-time.** The mount agent reorients against the project as it stands, with CLAUDE.md and specs already loaded. Hand off goal and boundary, not the analysis; depth belongs in the slate commit message, not the docket body.

**Stale-by-mount filter.** Each line must still be true at mount-time. Fail (drift): line numbers, byte/match counts, version strings, exhaustive site enumerations. Pass (durable): file paths as entry points, commit hashes, spec/concept names.

**Docket anti-patterns:**
- Restating commits/specs the mount agent already loads. Point, don't paraphrase.
- Pre-baking implementation (function names, signatures, locations for new code). Mount-time decisions.
- Site enumerations. Write a discovery recipe (`grep "pattern" Tools/rbk/`) instead of listing six file:line entries.
- Body over ~15 lines warrants suspicion of overprescription.
- **Plan-step structure stays in the plan.** Docket phase/step labels (A/B/C, "Phase 1", "first/then/finally") must not appear as code comments; line order already conveys execution.
- **Absolute paths to working trees.** Don't pin a docket to a specific repo clone (`/Users/foo/projects/your-project`, `~/proj/`). Mount may legitimately run against a different clone — if work needs to happen in *the* working directory, say so without naming it. Absolute paths *are* acceptable for external data roots (log dirs, `/tmp/...` scratch, machine-pinned data), flagged as operator-mutable rather than identity.

**Reference discipline.** Single-operator workflow runs paces in heat order, so explicit dependency markers in docket prose are usually overspecified; coronet cross-refs earn their keep only when the dependency crosses heats or skips order — rare, not never.

**Paddock posture.** A paddock articulates shape, cinched decisions, and what done looks like — not a progress journal. Git log and `jjx_log` are the journal; the paddock is the shape. A coronet is an enrollment-ledger key, not shape data, and silks are display names that drift via `jjx_relabel` — neither belongs in paddock or docket prose. `jjx_show` is the authoritative source for what paces exist; refer to a heat or pace by its purpose, not its identifier or display name. The three identity types fare differently against the lifecycle:
- **Firemarks** — lifecycle-bound (a heat keeps its firemark for life) → fine in both paddock and docket prose.
- **Coronets** — stable but invalidated by pace-state ops (drop, relabel, reorder, transfer) → barred from paddock prose; in docket prose only for genuine cross-order or cross-heat dependencies (see **Reference discipline**).
- **Silks** — evolve freely via relabel → barred from *both* paddock and docket prose.

When editing a paddock or docket for other reasons, prune any coronet refs or silks-shaped kebab strings you find.

**Semantic linefeeds in artifact prose.** Author paddock and docket prose — the bodies you write into `gazette_in.md` — with *semantic line breaks*: one sentence per line, and a long sentence broken at its major internal boundaries (em-dash, semicolon, colon, comma-before-conjunction); Markdown soft-wraps these into one rendered paragraph, so it is invisible in output. Do not break at the word level — when you want word-grain diffs, that is `git diff --word-diff`, not physically split prose. (Instantiates JJS0's *Diff-Friendly Prose* principle, which governs gazette markdown only — not the `.adoc` specs, whose source answers to MCM line-break discipline.)

**Mid-execution posture.** When a failure or surprise surfaces while a pace is mounted, the default response is mechanism + one specific repair you'd attempt. Do not proliferate options or weigh consequences across alternatives — pace scope and segmentation are operator territory. If the repair is obvious, proceed; else surface the question and stop.

**Identities vs Display Names:**
- **Firemark**: Heat identity, two characters from a nonstandard b64 alphabet (e.g., `₣AA`, `₣A-`, `₣A_`, `₣A7`). Used in command params and JSON keys.
- **Coronet**: Pace identity, firemark plus three characters from the same alphabet (e.g., `₢AAAAk`, `₢A_A-p`, `₢AAA_7`).
- **Silks**: kebab-case display name. Human-readable only — NOT usable for lookups.

When a command takes a firemark or coronet, provide the identity, not the silks.

**Display discipline (agent text output to user):**
- **Always show full coronets and firemarks; never abbreviate.** Write
  `₢A-AA-`, never `AA-`. Write `₣A-`, never `A-`. A bare 3-char pace suffix
  is ambiguous with the operator in several heats at once; the ₣/₢ glyph +
  heat half is part of the identity, not decoration. This holds in prose,
  tables, bullets, headers, and casual back-references.
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
- `command`: string selecting the operation — always the canonical `jjx_*` name (e.g., `"jjx_show"`, `"jjx_record"`)
- `params`: JSON object with command-specific fields (see reference below)
- `officium`: officium identity string from `jjx_open` (required on all commands except `jjx_open` — see Officium Protocol below)
- `model`: agent's verbatim model ID string from its system prompt (e.g., `"claude-opus-4-8"`). Required on ALL commands including `jjx_open`.

**`params` must be a JSON object, never a string.** If params is accidentally stringified (e.g., `"{\"key\": \"val\"}"` instead of `{"key": "val"}`), deserialization will fail. The server has a defensive fallback for this, but always pass a native object.

**Interdictum — the gating refusal.** A jjx result whose first word is `INTERDICTUM` bars the act; it is not a calling error to fix and retry.

- **Stop the act and its objective by any route** — no altered-params retry, no other command to the same end. Unrelated work continues.
- **Report it verbatim.** The message is self-sufficient: what refused, why, the remedies. Pursue one only if the operator directs it.
- **Do not model the guard.** This context says nothing about what generates an interdictum — carried mechanism becomes a self-check, and second implementations drift. Attempt the act; obey the token.

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
| curry | paddock | `jjx_curry` (write; the read is `jjx_paddock`) |
| bridle | pace | `jjx_apostille {coronet, tier, effort?}` — see Bridle Protocol below |
| unbridle | pace | `jjx_apostille {coronet, release: true}` |
| notch | pace | See Commit Discipline below |
| wrap | pace | `jjx_close` |
| rail | heat | `jjx_reorder` |
| furlough | heat status/silks | `jjx_alter` |
| retire | heat | `jjx_archive` |
| restring | heat | `jjx_transfer` |
| foray | remote dispatch | See Foray Protocol below |
| unfurl | image | See Unfurl Protocol below |

**MCP Command Reference:**

All params are JSON objects. `?` = optional, `[]` = array. Booleans default to false.

```
jjx_open           {}
jjx_show           {remaining}                                       # targets via gazette_in.md jjezs_halter notice(s) ONLY (one per target, firemark|coronet by length); no targets param, no auto-select. result terse, gazette always populated
jjx_list           {status?}
jjx_orient         {}                                                 # target via gazette_in.md jjezs_halter notice ONLY (exactly one, firemark|coronet by length); no firemark param
jjx_create         {silks}
jjx_enroll         {firemark, before?, after?, first?, size_limit?} # silks+docket via gazette_in.md only
jjx_reorder        {firemark, move?, before?, after?, first?, last?}
jjx_alter          {firemark, racing?, stabled?, silks?}
jjx_record         {identity, files[], size_limit?, intent?}
jjx_close          {coronet, summary?, spook?, size_limit?}     # spook: wrap-time friction report -> Spook: trailer on the W commit
jjx_log            {firemark, limit?}                              # limit sizes the GAZETTE (every entry, subjects whole); the inline result is a bounded index — newest 50 rows, subjects clipped. result terse, gazette always populated
jjx_search         {pattern, actionable?}
jjx_archive        {firemark, size_limit?}
jjx_transfer       {firemark, to, coronets, size_limit?}
jjx_continue       {firemark}
jjx_paddock        {firemark}                                       # READ paddock -> gazette_out.md; staged gazette_in.md rejects loud (the writer is jjx_curry)
jjx_curry          {note?, size_limit?}                             # WRITE paddock from gazette_in.md jjezs_paddock notice (firemark lede, non-empty body); auto-commits
jjx_relocate       {coronet, to, before?, after?, first?}
jjx_redocket       {size_limit?, before?, after?, first?}           # gazette_in.md only; mixed single-heat batch (paddock + reslate + slate); before/after/first aim the whole slate run
jjx_relabel        {coronet, silks}
jjx_drop           {coronet}
jjx_brief      {coronet}                                            # raw docket text for ONE pace, returned inline (no gazette) — the clean single-docket read; an abandoned pace's docket leads with an [abandoned] marker line
jjx_coronets   {firemark, remaining?, rough?}                       # coronet IDs in heat order, one per line, inline — no silks, no docket; the default listing tags an abandoned pace as "<coronet>  [abandoned]" and a bridled pace as "<coronet>  [bridled <tier>]" (coronet stays the first token; remaining includes bridled, rough excludes it)
jjx_landing        {coronet, agent, content?}
jjx_apostille      {coronet, tier?, effort?, release?}               # bridle/unbridle: designate {coronet, tier, effort?} (only a rough pace) or release {coronet, release: true}; exactly one of tier|release; tiers haiku|sonnet|opus|fable, efforts low|medium|high|xhigh|max
jjx_validate       {}                                                # normalize-and-report — exit 0 clean / 2 normalized (rewrote+committed) / 1 broken (untouched)
jjx_bind           {alias, reldir}                                  # remote: create legatio session (alias resolves BURN profile)
jjx_send           {legatio, command}                               # remote: synchronous exec on fundus
jjx_plant          {legatio, commit}                                # remote: reset fundus to exact commit
jjx_relay          {legatio, tabtarget, timeout, firemark}          # remote: async dispatch via nohup
jjx_check          {pensum, timeout}                                # remote: probe/poll pensum status
jjx_fetch          {legatio, path}                                  # remote: read single file from fundus
```

**Key points:**
- **`jjx_orient` and `jjx_show` take their target(s) solely from `gazette_in.md` via `jjezs_halter` notices — never a param.** Write one body-less `# jjezs_halter <firemark|coronet>` notice per target (lede self-types by length: firemark -> heat expansion, coronet -> single pace), then call. A `firemark`/`targets` param is **rejected, not a fallback**; an absent gazette is a loud error.
  - `jjx_orient` (mount) — **exactly one** `jjezs_halter` notice; more than one is an error.
  - `jjx_show` (groom/parade) — **one or more** `jjezs_halter` notices (the heterogeneous set).
  - `remaining` (show) is **required** and stays a param — it is a display mode, not a target; it filters firemark expansion only (a directly-named coronet returns regardless of state). The tool-result is always the terse table; paddock(s) + pace dockets always land in `gazette_out.md` — Read it for the bodies. There is no `detail` param.
- `jjx_orient` output includes next actionable pace — no separate show call needed
- **Gazette output**: `jjx_orient`/`jjx_show`/`jjx_paddock`/`jjx_log` write `gazette_out.md`; read that file for full content (shapes in the directional bullets below).
- **`jjx_validate` is normalize-and-report, not a read-only check** (exit codes in the command reference above). *Normalized is a structural verdict, not a semantic blessing* — a `2` means the bytes are canonical, not that the heat/pace inventory is right. After a merge convergence, eyeball the inventory against both branches yourself.
- **Never reach past the JJK interface to raw storage — NO exceptions.** Do not parse the harness's persisted tool-result files or the gallops JSON (`.claude/jjm/jjg_gallops.json`) directly. To read one pace's docket, call `jjx_brief {coronet}` (returns inline). To read full paddock/dockets after `jjx_show`/`jjx_orient`, read `gazette_out.md` directly (loop `jjx_brief` per pace if a large `jjx_show` overflows) — never scrape a persisted tool-result blob.
- **Gazette input**: `jjx_enroll`, `jjx_redocket`, and `jjx_curry` read docket/content from `gazette_in.md`; `jjx_orient` and `jjx_show` read their target selection from `gazette_in.md` (`jjezs_halter` notices). Gazette is the sole input path — no JSON param fallback on any of them.
- **`jjx_paddock` reads, `jjx_curry` writes — mode is named by the command, never inferred.** `jjx_paddock` takes the firemark param and rejects loud if anything is staged in `gazette_in.md`; `jjx_curry` takes the staged `jjezs_paddock` notice (firemark in the lede) and no firemark param.
- `jjx_redocket` applies a **mixed single-heat batch** (paddock + reslate + slate notices in one `gazette_in.md`, one commit; mass reslate is the common case — shapes in the wire-format table below). Constraints: **one heat only** — every reslate coronet's parent and the paddock firemark must name the same heat, else rejected; a slate-only batch has no heat anchor and is rejected (use `jjx_enroll`). Slate notices apply in **file order** (notice order = pace order); `before`/`after`/`first` aim the whole slate **run** — the first slate takes the cursor and each later one follows its predecessor, so the batch folds in as one unbroken sequence at the aimed position. Absent positioning, the run appends to the heat's end, still in file order. Reslate and paddock never move the cursor.
- `jjx_curry` `note` param: optional short string appended to the paddock discussion commit message
- `jjx_close` takes `summary` as a string param (not stdin pipe)
- `jjx_close` takes `spook` as an optional string param — the wrap-time friction report (see Wrap Discipline). It rides the W chalk commit as a single-line `Spook:` trailer; absent or empty becomes `Spook: none`. Grep the corpus with `git log --all --format='%b' | grep '^Spook:'`
- `jjx_record` takes `files` as a native JSON array: `["file1.rs", "file2.rs"]`
- `jjx_transfer` takes `coronets` as a JSON-encoded string (not a native array): `"[\"AYAAA\", \"AYAAB\"]"`

### Officium Protocol

Each chat session must open an officium before using any jjx commands.

1. **At chat start**, call `jjx_open` (no params, no officium field). It returns a ☉-prefixed identity string (e.g., `☉260327-1000`), plus the absolute `gazette_in` / `gazette_out` paths for this officium — use those verbatim (see Gazette paths below).
2. **On every subsequent jjx call**, pass the returned identity as the `officium` field on the MCP tool.
3. **Self-healing**: If any jjx command fails with "Officium directory not found", call `jjx_open` again to create a fresh officium, then retry.

**⚠️ `jjx_open` must land alone — never co-batch it with any call that consumes its result.** The officium ID is a server-minted opaque token that cannot be guessed from a prior session; issue `jjx_open`, read the returned ☉-id, and only *then* issue other jjx calls in a later tool block. This is the canonical "mint an ID → use the ID" case of the never-co-batch-a-producer-with-its-consumer rule — see **Parallel Tool Batch Discipline** below.

The ☉ (U+2609 SUN) prefix parallels ₣/₢ for firemarks/coronets. Pass it exactly as returned — the dispatcher strips it (see Gazette paths below).

Gazette file exchange uses two directional files in the officium exchange directory. Every jjx MCP call unconditionally deletes both gazette files on entry (read+delete `gazette_in.md`, delete `gazette_out.md`) — gazette content has single-MCP-call lifetime, a parameter or return value, not persistent state.

**⚠️ `gazette_in.md` is the argument to the *next* jjx call — whichever fires next consumes it on entry, not a particular command.** So write the gazette as the immediate last step before *that* call; if you reconsider which command to run, rewrite or clear it first. **Deleted-on-entry means read-on-entry** — a stale notice is consumed as this call's input *before* it is deleted, and a gazette *setter* writes and **auto-commits with no confirm gate**. Keep every gazette write welded to the single call it was written for.

- **`gazette_in.md`** (agent → server): write before a setter (`jjx_enroll`, `jjx_redocket`, `jjx_curry`) or before `jjx_orient` / `jjx_show` (their `jjezs_halter` target selection lives here).
- **`gazette_out.md`** (server → agent): written by getter commands, read after they return. Notices are `# jjezs_paddock <firemark>` / `# jjezs_pace <coronet>` / `# jjezs_steeplechase <firemark>`, content beneath each: `jjx_orient` → paddock + next actionable pace docket; `jjx_show` → paddock(s) + a docket per resolved pace; `jjx_paddock` → paddock only; `jjx_log` → every steeplechase entry with its subject untruncated (the inline table clips subjects, so this is the only full reading — read the file, never scrape a persisted tool-result blob).

**Gazette wire format (setter commands):**
Each notice is a `#`-header line with slug and lede, followed by a content body. The lede is **exactly one whitespace-free token** (silks / coronet / firemark, per the table below) — nothing follows it on the `#` line; the body goes beneath, and appending extra text to the lede folds it into the identity and fails validation. `jjezs_halter` is body-less (the lede is the whole notice); the content-bearing slugs (`jjezs_slate`, `jjezs_reslate`, `jjezs_paddock`) require a **non-empty** body — an empty or whitespace-only body rejects loud, a setter never executes a clear.

**Critical: `#` (H1) in gazette_in.md is a wire format delimiter, NOT a markdown heading.** For single-notice commands (enroll, curry, orient), use exactly ONE `#` line. For mass reslate and multi-target show, each `# jjezs_reslate` / `# jjezs_halter` line starts a new notice. All markdown headings within body content must use `##` or deeper — a bare `#` line inside content will be parsed as a notice boundary.

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

**On failure:** If a gazette setter command fails, `gazette_in.md` is already consumed (deleted on entry) — re-write it from scratch before retrying.

| Command | Write to `gazette_in.md` | Then call with params |
|---------|--------------------------|----------------------|
| `jjx_orient` | `# jjezs_halter <firemark\|coronet>` (exactly one, body-less) | `{}` (target is in gazette lede) |
| `jjx_show` | `# jjezs_halter <firemark\|coronet>` per target (one or more, body-less) | `{"remaining": true\|false}` |
| `jjx_enroll` | `# jjezs_slate <silks>` + docket body | `{"firemark": "XX", "before?": ..., "after?": ..., "first?": ...}` |
| `jjx_redocket` (single/mass) | One or more `# jjezs_reslate <coronet>` notices, each with docket body | `{}` |
| `jjx_redocket` (batch) | Mixed `# jjezs_paddock <firemark>` + `# jjezs_reslate <coronet>` + `# jjezs_slate <silks>` notices, one heat | `{"before?": ..., "after?": ..., "first?": ...}` (position the first slate) |
| `jjx_curry` | `# jjezs_paddock <firemark>` + content body (non-empty) | `{"note?": "commit annotation"}` |

**Gazette paths are emitted by the server** — `jjx_open`/`jjx_orient` return the absolute `gazette_in`/`gazette_out` paths; use them verbatim, never rebuilt from the id (on disk `<id>` has the ☉ **stripped**: `260327-1000`, never `☉260327-1000`, so hand-substituting the ☉-id lands in a nonexistent glyph-named sibling). Never `find` for a gazette: single-MCP-call lifetime means a search finds nothing or something stale, and a setter's not-found error already names the exact path it checked.

**Read-modify-write workflow** (paddock editing):
1. Call `jjx_paddock {firemark}` → writes `gazette_out.md`
2. Rename `gazette_out.md` → `gazette_in.md`, edit content
3. Call `jjx_curry`

**Show → reslate round-trip** (batch docket editing — the show output *is* the reslate input, bridged):
`jjx_show` populates `gazette_out.md` with the resolved set (paddock[s] + every pace docket); `jjx_redocket` consumes `gazette_in.md` as `jjezs_reslate` notices. A bash bridge turns one into the other, so a batch can be pulled, edited in place, and replumbed in a single mass reslate.

1. **Pull**: write the target selection to `gazette_in.md` — one `# jjezs_halter ₣XX` notice (or several, a heterogeneous set) — then call `jjx_show {"remaining": true}` and read the emitted `gazette_out.md`.
2. **Bridge**: drop the `# jjezs_paddock …` notices (and their bodies) and rewrite each output-typed `# jjezs_pace <coronet>` header to the input-typed `# jjezs_reslate <coronet>`. The pace *bodies* are already valid reslate dockets — only the slug changes:
   ```
   # keep the pace notices, drop the paddocks, rename the slug
   awk '/^# jjezs_paddock /{skip=1;next} /^# jjezs_/{skip=0} !skip' gazette_out.md \
     | sed 's/^# jjezs_pace /# jjezs_reslate /' > gazette_in.md
   ```
3. **Edit**: apply the actual docket change to `gazette_in.md`. For a uniform mechanical rewrite across many dockets, this is one more pass (portable in-place form, no GNU/BSD `-i` divergence):
   ```
   sed -e 's/^## Locked$/## Cinched/' -e 's/^## Done$/## Done when/' \
     gazette_in.md > gazette_in.tmp && mv gazette_in.tmp gazette_in.md
   ```
4. **Replumb**: `jjx_redocket {}` — mass reslate consumes every `jjezs_reslate` notice in one call, echoing a per-coronet diff.

Reach for the bridge when the edit is the same mechanical shape across N dockets (rename a heading, retag a term); when each docket needs its own novel prose, author each `# jjezs_reslate <coronet>` notice by hand instead. There is no reslate dry-run; trust the per-coronet diff `jjx_redocket` echoes.

### Parallel Tool Batch Discipline

When tool calls share one parallel block and one exits non-zero, the harness **cancels the still-queued siblings** ("Cancelled: parallel tool call … errored"), so a screenful of cancellations usually traces to *one* real error upstream — diagnose that, don't react to the cascade. Two rules:

- **Never co-batch a producer with its consumer.** A command and another that needs its output, minted id, or a path built from it must be sequenced across tool blocks (`jjx_open` is the canonical case). This includes `jjx_record` alongside the edits/checks that must *precede* it: verify the tree first, commit second.
- **Keep fragile or dependent commands out of wide read-only batches** — an empty/failed sibling can silently cancel unrelated reads.

### Mount Protocol

When user says "mount" or you need to engage the next pace:

1. Resolve the target: if the user says "mount" without specifying a heat and you have no prior heat context this session, ask which heat rather than guessing; if you have prior context (previously mounted/groomed heat), use that firemark. Write exactly one `# jjezs_halter <firemark|coronet>` notice to `gazette_in.md`, then run `jjx_orient {}`.
2. Parse output: Racing-heats table, Heat/Next/Docket/Recent-work sections; read paddock and pace docket from the gazette file.
3. **Read the paddock before the docket.** The paddock tells you the shape of the work; the docket tells you what to do next.
4. **Lead with the pace goal in one sentence**, distilled from the docket — what this pace is trying to accomplish, stated precisely (paces can sit weeks between slating and mounting; goal recall comes first). Then display brief heat context (silks + paddock one-liner + recent work) and finally the full docket.
5. **Name assessment**: If pace silks doesn't fit docket, offer rename via `jjx_relabel`
6. Propose the approach compactly — a one-line recommended execution strategy (model tier + sequential/parallel). Offer the reasoning (tier rationale, parallelization analysis) on request, not unprompted. Tiers: haiku (mechanical), sonnet (standard dev), opus (architectural).
7. Ask to proceed, then begin work

### Groom Protocol

When user says "groom":

1. Write one or more `# jjezs_halter <firemark>` notices to `gazette_in.md`, then run `jjx_show {remaining: true}` (paddock + remaining dockets land in `gazette_out.md`).
2. Read `gazette_out.md` directly for full paddock and pace docket content — never the persisted tool-result blob or `jjg_gallops.json`.
3. Display overview: heat silks, progress, remaining paces with dockets.
4. Enter planning mode: suggest structural operations (slate, rail to reorder, reslate to refine dockets, paddock review).

### Bridle Protocol

Bridling designates a pace for execution at a specific model tier: a frontier
agent — always the calling agent, never the server — records that it judged the
pace mechanically defined. States: a `rough` pace is undesignated judgment work;
a `bridled` pace is designated, carrying its tier (and optionally an effort word)
until it is released, reverted, or closed.

**Bridling (frontier session designates):**

1. Read the candidate pace's docket via `jjx_brief` (and the paddock via
   `jjx_paddock` if heat context is needed for the judgment).
2. Judge it: mechanically defined means no ambiguities and no lurking
   design-level decisions. If the docket has gaps, REFUSE to designate and
   report the gaps to the operator — a gap is a reslate need, not a lower-tier
   problem.

   Tier is set by where judgment lives during execution, not by how much the
   pace matters: operator presence LOWERS the requirement (judgment on tap, so
   the designee just executes carefully and surfaces clearly), and stakes hold
   no tier since stop-and-surface already forbids improvisation. What holds a
   frontier tier is plausible wrongness the operator cannot cheaply check —
   work that authors authority downstream work will obey (a ceremony document,
   a spec narrative), where an error reads fine and propagates.
3. Before designating a frontier tier, weigh mechanization: if the judgment
   the docket defers could be settled NOW — from the paddock's cinches, landed
   specs, and what this session holds — propose the reslate that bakes those
   decisions in, leaving a residue a lower tier can run (the proposal goes to
   the operator; on approval, redocket, then bridle). Judgment only the work
   itself can surface — spec authoring, gates, verification — is not
   mechanizable: designate it at its tier rather than manufacture a reslate.
4. Otherwise designate: `jjx_apostille {coronet, tier}` with the execution tier
   (haiku | sonnet | opus | fable), adding `effort` (low | medium | high |
   xhigh | max) only where the docket's depth is judged. Only a rough pace
   may be bridled.

**Designee session (the designated-tier agent executes):**

1. `jjx_orient` on the heat — resolution lands on the bridled pace.
2. Work the docket. Commit via `jjx_record` against the bridled coronet.
3. Finish with `jjx_landing {coronet, agent, content}` — the completion report.
4. NEVER wrap. A frontier session reviews the landed work and wraps.
5. On any hole or surprise: stop and surface it — a designee never restates its
   own orders.

**Escalation paths (designation is void when its judgment inputs change):**

- Edit-revert: any redocket of a bridled pace (single or mass reslate), and
  any transfer or relocate, automatically reverts it to rough and wipes the
  tier and effort. Relabel and reorder revert nothing.
- Deliberate release: `jjx_apostille {coronet, release: true}` returns the pace
  to rough judgment work (then optionally re-designate).
- Tier and effort persist through close as provenance of the executing session.

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

### Unfurl Protocol

`Tools/jjk/claude-jjk-images.md` holds the tools for VIEWING and UPDATING SVG/PNG
files (diagrams / rendered images) on the diagram viewer. When the user says
"unfurl" or asks to view or update an image, read that file for the precise
directions.

### Commit Discipline

When working on a heat, use `jjx_record` for commits with heat/pace affiliation.

**Pace-affiliated commit** (active pace provides context):
Use `jjx_record` with `{identity: "CORONET", files: ["file1", "file2"], intent: "description"}`

**Heat-affiliated commit** (no active pace, but part of heat work):
Use `jjx_record` with `{identity: "FIREMARK", files: ["file1", "file2"], intent: "description"}`

Synthesize intent from the conversation — describe *what* was accomplished, not *how*.

When user says "notch", determine context (pace or heat affiliated) and invoke `jjx_record` with the appropriate identity and explicit file list.

**Multi-Officium Discipline:**
Multiple Claude officia (concurrent chat sessions, each with its own ☉-prefixed officium ID) may work in the same repo. The explicit file list in `jjx_record` enables orthogonal commits.

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
`jjx_enroll`, `jjx_close`, `jjx_record`, and other state-mutating jjx commands create git commits internally. **`jjx_close` (wrap) commits ALL uncommitted changes** — code files and gallops state together in one commit. Do NOT follow `jjx_record` or `jjx_close` with another commit command, and do NOT notch immediately before or after a wrap — the tree will already be clean. If a commit command says "Nothing to commit", check `git status --short` and accept the result.

**Diagnose Before Escalating:**
When a command fails, check the simplest explanation first. "Invalid params" means wrong field names — check the MCP Command Reference above, don't guess.

### Wrap Discipline

**NEVER auto-wrap a pace.** Always ask the user explicitly: "Ready to wrap ₢XXXXX?" and wait for confirmation before running `jjx_close`. The user decides when work is complete, not the agent — when work is complete, report outcomes and ask; do not wrap.

When wrapping (after user confirms), always include a summary of the work:
Use `jjx_close` with `{coronet: "CORONET", summary: "Added bitmap displays to orient output"}`

**Always answer the spook (friction) question at wrap.** Alongside `summary`, pass `spook` — a short, grep-friendly report of friction *you* hit during this chat: re-reads forced by a missing pointer, a docket aimed at a renamed file, a confusing paddock, a verb that fought you. Ask yourself "what snagged?" assuming something did, not "did anything snag?" — at wrap the pull is to tidy up and under-report, and that bias is the failure mode this channel exists to resist. Report only first-person in-chat events you observed (actionable). "Nothing snagged" is a first-class answer: pass `spook: "none"` (or omit it) — required to answer, never required to invent.
The agent always has context about what was accomplished — include it.

**Wrap is unscoped — known JJK bug, do NOT "fix" it.** Unlike `notch`/`jjx_record` (explicit file list), `jjx_close` (wrap) stages and commits **every** dirty file in the tree — your code, the gallops state, and anything another officium left uncommitted. Wrap is not yet as file-specific as notch; that gap is a known bug, deferred pending the planned git-worktrees switch — do not attempt to repair it. For now: before wrapping, make sure the tree holds only your work, or expect wrap to sweep all of it into one commit.
