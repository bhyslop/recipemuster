# Memo: Chat-Log Capture & Cost Reconstruction (JJK)

*2026-06-15. Provenance for a design session (Brad + Claude Opus 4.8). Informs the
₣BD chat-capture paces — (1) store decision + first-time backfill, (2) auto-recurring
capture — and a deferred cost-analytics pace. No source changed this session; the
working prototypes were throwaway jq+bash.*

## Why

Claude Code transcripts live at `~/.claude/projects/<cwd-encoded>/*.jsonl`. They are
machine-local and GC-pruned (a `~/.claude/.last-cleanup` runs periodically), so the
chat record is a perishable asset: you cannot analyze a conversation that has already
been silently reaped. JJK should capture these logs into the project before they
disappear; **cost and other analytics ride on the captured corpus later** — capture
first, analytics after, because capture is the part that races the clock.

## Empirical findings (this repo, measured 2026-06-15)

### Cost is reconstructable, not stored

- Transcripts store per-assistant-message **token usage + model**, never a dollar
  cost. `/usage`'s cost is *computed*: tokens × public per-token rates. (No `costUSD`
  field in current Claude Code transcripts; older versions had one — format drift.)
- `message.usage` fields: `input_tokens`, `output_tokens`, `cache_read_input_tokens`,
  `cache_creation_input_tokens` (+ a `cache_creation.ephemeral_1h/_5m` split), plus
  `message.model`.
- Validated: a `/usage`-reported **$6.91** session reconstructs to **$6.89** — the
  gap is rounding in the displayed token figures. Method is sound.

### Rates (per 1M tokens, list price; cache rates derived from the input rate)

| Model | Input | Output |
|---|---|---|
| opus 4.8 / 4.7 / 4.6 | $5 | $25 |
| sonnet 4.6 | $3 | $15 |
| haiku 4.5 | $1 | $5 |
| fable 5 | $10 | $50 |

Cache read = 0.1× input; cache write 5m TTL = 1.25× input; 1h TTL = 2× input (Claude
Code writes 1h). Model `<synthetic>` = $0 (locally-injected turns, not billed).

### Whole-project totals

- **~$6,839** of API-equivalent value across **63 days** (deduped). On the
  subscription plan this is the shadow price, not what was paid.
- By model: opus-4-7 dominant (~$4,089), then opus-4-8 (~$1,121), opus-4-6 (~$830),
  fable-5 (~$796), sonnet-4-6 (~$2).
- **Dedup is load-bearing.** ~43,608 of ~89,516 assistant records are forked/resumed
  copies sharing the same API `message.id` but **distinct transcript `uuid`s** (a fork
  re-stamps uuids). Dedup **by `message.id`** in the cost parser bills each generation
  once; a naive sum nearly doubles the total. Git delta handles the *storage*
  redundancy of the verbatim copies on its own.

### Byte mass (839 MB raw for this project)

| Bucket | ~MB | What |
|---|---|---|
| tool_result | 102 | file reads, grep/glob, command output |
| tool_use | 26 | tool inputs incl. Write/Edit payloads |
| other_records | 18 | per-session injected context boilerplate |
| assistant_text | 16 | the assistant's prose |
| user_text | 4 | operator messages |
| thinking | ~0 | omitted/empty on 4.8/4.7 |
| embedded images | 0 | none present |

The conversation you'd actually study (prose) ≈ **20 MB**; the rest is machinery,
most of it recoverable from git (file reads/writes) or compressible (verbatim forks).

## Cinched decisions

- **Home:** JJK / Rust (`jjx`).
- **Attribution is in-band.** Every jjx call is recorded in the transcript as a
  `tool_use` carrying its coronet/firemark + officium, interleaved with billed turns
  in timestamp order. So heat/pace attribution is a single forward pass over one
  session's transcript — no external join, concurrency-safe across multiple officia.
  (Confirmed empirically: e.g. `jjx_record ₢A6AAC` under officium `☉260413-1019`.)
- **Wholesale capture, per-project boundary.** Capture every session in the transcript
  dir matching this repo's cwd; no relevance filtering. The boundary is the
  **project, across hosts** — not the host, and not a cross-project consolidation.
  cerebro-alpha (same project, other host) merges in; **beta (a different project) is
  excluded by construction** — one repo must never absorb another's chats.
- **Never-forget but update.** Additive sync: pick up new session files and the growth
  of existing ones; never delete from the store even when the source disappears. Git
  history carries the "never forget."
- **Compression is git's own zlib + delta.** No external compressor. Supply-chain
  reasoning: a pinned/checksummed Cargo crate beats an *ambient, unpinned* system
  binary (`zstd` CLI), but git-zlib adds **zero** new trust surface and is the tightest
  exposure management — and for a rarely-read, git-delta-friendly archive the ratio
  gain of zstd did not justify the added C/FFI/build surface.
- **Triggers:** mount + groom + muster (the frequent orientation ceremonies; more
  frequent than wrap, which is rare under chat daisy-chaining).
- **Deferred:** cost accounting, attribution, analytics — later paces on the captured
  corpus.

## Storage finding (corrects a prior assumption)

The store does **not** belong in `vov_veiled/`. From the delivery-path investigation:

- `vov_veiled/` is **excluded from the parcel** (`Tools/vok/vof/src/vofr_release.rs`
  skips veiled paths; the `rbk-prep-release` ceremony `git rm`s them).
- Delivery is a **nuclear install**: `Tools/vok/vof/src/vofe_emplace.rs`
  `remove_dir_all`s each `Tools/{kit}/` and rebuilds from the parcel. So a store under
  `Tools/jjk/vov_veiled/` would be **both absent on consumers and destroyed on
  re-delivery**.
- `.claude/jjm/` is the right home: emplace only touches `Tools/{kit}` + `.vvk`, never
  `.claude/`; jjm is committed to git and consumer-local (not in the parcel → not
  distributed). The store lives **alongside gallops**, e.g. `.claude/jjm/chat-archive/`.
- Defensive `mkdir -p` on first write; commit a `.gitkeep` so the dir exists on fresh
  clones.

## Open thread: the state gestalt (not resolved)

There are **two "never ships" categories, and only one is named:**

- **veiled** = kit-private *source* — proprietary authored content under `Tools/{kit}/`,
  reviewed as design, *replaceable* (wiped + rebuilt on re-emplace).
- **jjm** = project-local *state* — generated runtime data under `.claude/`, must
  *survive* updates, scoped *per-project* not per-kit.

jjm survives re-delivery today only because emplace's deletion scope happens to be
narrow — an **emergent** guarantee, not a **stated** one. That is the "murkiness"
next to veiled's clean rule. Open question: name the state-gestalt as a first-class
rule (release never ships it, emplace never touches it) so jjm's protection becomes a
contract parallel to veiled's. **Gallops should not move into veiled** — lifecycle
(wiped on re-emplace), category (source vs state), and scope (per-kit vs per-project)
all mismatch. Discussion ongoing.

## Follow-ups

- **Two capture paces drafted, not yet slated** into ₣BD: (1) decide store +
  first-time multi-source backfill (local-alpha + cerebro-alpha); (2) auto-recurring
  capture at mount/groom/muster. Order: (1) first. Store pinned "alongside gallops" so
  it tracks that category regardless of the gestalt outcome.
- **Beta's history needs its own capturer** (beta's own JJK once this lands, or a
  one-off backup) — alpha must not reach across.
- **Cost-analytics pace** deferred until the corpus is captured.
- **State-gestalt naming** — open (above).

## Prototype residue

Throwaway jq + bash prototypes from this session lived in `/tmp` (`/tmp/rbcost.jq`,
`/tmp/breakdown.jq`) — proof-of-method only, never repo artifacts. The real
implementation is Rust in `jjx`.
