# Study: Model Prompt Tuning (SMPT)

## Purpose

Discover minimal effective prompt framings that cause each Claude model tier
(haiku, sonnet, opus) to faithfully reproduce pre-formatted tabular text
without reformatting it into markdown tables.

## The Problem

When Claude models receive column-aligned text like:

```
Name          Role        Count   Status
alice         admin         142   active
bob           viewer          7   pending
```

They tend to "helpfully" reformat it into markdown table syntax with `|` pipes
and `---` horizontal rules, destroying the original column alignment. This
behavior varies by model and is influenced by the system prompt.

## Architecture

The study binary (`smpt`) is the **single point of experimental control**.
All parameters — system prompts, user messages, test content, evaluation
logic — are compiled into the Rust binary. This prevents any Claude session
(orchestrating or test-subject) from seeing the experiment design.

Two invocation paths are supported:

```
User Terminal                Study Binary              Test Claude
     |                            |                        |
     |--- tt/study-mpt.Run.FULL.sh                         |
     |--- tt/study-mpt.Run.api-FULL.sh                     |
     |         |                  |                        |
     |    BUK dispatch → build →  |                        |
     |                            |                        |
     | PATH A: OAuth/CLI (FULL)   |                        |
     |                   smpt run |-- claude -p ---------> |
     |                            |   --system-prompt      |
     |                            |   --no-session-persist |
     |                            |<-- JSON envelope ------| (~26.5K cache tokens)
     |                            |                        |
     | PATH B: Direct API (api-FULL)                       |
     |               smpt api-run |-- POST /v1/messages -> |
     |                            |   ANTHROPIC_API_KEY    |
     |                            |<-- bare API response --| (zero cache tokens)
     |                            |                        |
     |                   diff/evaluate                     |
     |<-- log: PASS/FAIL + token/cost/latency data         |
```

### Key Design Properties

- **Compiled binary hides experiment**: Test Claudes cannot see the source
- **Orchestrator is blind**: Even the Claude running the experiment only
  sees trial numbers, not prompt contents
- **Machine-evaluated**: Binary diffs responses character-by-character,
  no model judging another model
- **Reproducible**: Same binary, same parameters, same trial numbers

### Why Rust, Not Bash

A bash script would be readable by any Claude session in the repo. The
compiled binary keeps the experimental parameters opaque. Also, we reuse
`vvc::vvce_claude_command()` which handles the `CLAUDECODE` env var
removal needed for subprocess invocation.

## Dependencies

- `vvc` crate — for `vvce_claude_command()` (subprocess nesting guard bypass)
- `tokio` — async subprocess with timeout
- `serde_json` — JSON parsing for CLI output envelope and API request bodies
- `reqwest` — HTTP client for direct API calls (PATH B only)

PATH A uses the `claude` CLI via OAuth (no API key needed).
PATH B requires `ANTHROPIC_API_KEY` in the environment.

## Invocation

### Tabtargets (BUK infrastructure — logging, temp dirs)

```bash
tt/study-mpt.Run.smoke.sh      # 1 trial: haiku+minimal (connectivity check, OAuth)
tt/study-mpt.Run.FULL.sh       # All 12 trials × N repeats, OAuth/CLI path
tt/study-mpt.Run.api-FULL.sh   # All 12 trials × N repeats, direct API path
```

Output goes to BUK log files:
- `../_logs_buk/last.txt` — most recent run
- `../_logs_buk/same-study-mpt-*.txt` — per-command history
- `../_logs_buk/hist-study-mpt-*.txt` — timestamped archive

### Direct binary (after `cargo build`)

```bash
smpt plan          # Show trial matrix with full prompt text
smpt smoke         # Single connectivity check trial (OAuth)
smpt trial N [R]   # Run trial N (1-12), R repeats, verbose output
smpt run [R]       # All 12 trials sequentially, OAuth path (default R=1)
smpt api-smoke     # Single connectivity check (direct API, requires ANTHROPIC_API_KEY)
smpt api-run [R]   # All 12 trials sequentially, direct API path (default R=1)
```

## Trial Matrix

4 system prompt variants × 3 models = 12 trials:

| Trial | Model  | System Prompt Variant |
|-------|--------|-----------------------|
| 1     | haiku  | default (Claude Code built-in, or zero-system for API path) |
| 2     | haiku  | minimal ("You are a helpful assistant.") |
| 3     | haiku  | direct (explicit reformatting prohibition) |
| 4     | haiku  | example (BAD/GOOD pattern with anti-pattern) |
| 5     | sonnet | default |
| 6     | sonnet | minimal |
| 7     | sonnet | direct |
| 8     | sonnet | example |
| 9     | opus   | default |
| 10    | opus   | minimal |
| 11    | opus   | direct |
| 12    | opus   | example |

**Note on `default` prompt**: In the OAuth/CLI path, `default` means no
`--system-prompt` override (Claude Code's built-in system prompt is used).
In the direct API path, `default` means the `system` field is omitted
entirely — zero system prompt injected.

## Evaluation

A trial **passes** if the original table text appears verbatim in the
response (after stripping code fences if the model wrapped it in a
code block).

A trial **fails** with diagnostics:
- Pipe characters detected (markdown table syntax)
- Horizontal rules detected (`---`)
- Header converted to markdown table
- Column alignment altered

## Results: OAuth/CLI Path

Run date: 2026-02-21. 60 invocations (12 trials × 5 repeats).
Log: `hist-study-mpt-FULL-20260221-083854-*.txt`

### Pass Rates (out of 5 repeats)

```
              default   minimal   direct    example
haiku           1/5       0/5      5/5       5/5
sonnet          5/5       5/5      5/5       5/5
opus            5/5       3/5      5/5       5/5
```

Total: **49/60 pass**, 11 fail.

### Median Latency (seconds per invocation)

```
              default   minimal   direct    example
haiku          2.67      3.16      3.21      3.89
sonnet         3.45      2.79      1.95      2.51
opus           3.66      3.67      3.12      3.56
```

Wall time for full run: **203 seconds**.

### Token Usage (median per invocation)

Format: `input + cache_read / output`

```
              default           minimal           direct            example
haiku         10+30489/193      10+26652/259      10+26687/189      10+26740/271
sonnet          3+30295/74        3+26462/80        3+26497/43        3+26550/75
opus            3+30328/48        3+26497/75        3+26532/44        3+26585/48
```

**Observation**: The `default` variant reads ~30.3K cache tokens; all
`--system-prompt` override variants read ~26.5K. The ~3.8K difference is
the Claude Code system prompt being replaced, but the ~26.5K base layer
(Anthropic's model system prompt injected by the CLI) is immovable.

### Cost (5 repeats per trial, USD equivalent)

```
              default   minimal   direct    example
haiku         $0.0694   $0.0216   $0.0184   $0.0199
sonnet        $0.1516   $0.0761   $0.0717   $0.0761
opus          $0.1484   $0.0774   $0.0720   $0.0725
```

**Total equivalent: $0.8751**

These figures come from `total_cost_usd` in the Claude Code CLI JSON envelope.
They represent token-equivalent cost at Anthropic API pay-as-you-go rates.
On a Max plan (flat subscription), no per-token charges actually occur —
these numbers are informational, not real charges.

## Results: Direct API Path

Run date: 2026-02-21. 60 invocations (12 trials × 5 repeats).
Log: `hist-study-mpt-api-FULL-20260221-090536-*.txt`

`ANTHROPIC_API_KEY` required in environment. Direct POST to
`api.anthropic.com/v1/messages` via `reqwest`.

**Important**: `default` in this path = zero system prompt (no `system`
field in the API request at all). This is the hardest possible test.

### Pass Rates (out of 5 repeats)

```
              default   minimal   direct    example
haiku           0/5       0/5      5/5       5/5
sonnet          5/5       5/5      5/5       5/5
opus            5/5       5/5      5/5       5/5
```

Total: **50/60 pass**, 10 fail.

### Median Latency (seconds per invocation)

```
              default   minimal   direct    example
haiku          1.23      1.02      0.70      0.84
sonnet         1.13      1.44      1.14      1.24
opus           2.14      2.07      2.03      2.25
```

Wall time for full run: **88.4 seconds** (vs 203s for OAuth path — 2.3× faster).

### Token Usage (median per invocation)

Format: `input / output` (no cache layer at all)

```
              default   minimal   direct    example
haiku          56/87     62/87     97/43     150/47
sonnet         56/47     63/47     98/43     151/47
opus           56/47     63/47     98/43     151/47
```

**Observation**: Token counts are dramatically lower with no system prompt
layer — 56–151 input tokens vs 26,500+ via OAuth/CLI. Zero cache reads.

### Cost (Direct API Path)

**Real charges occurred; not reflected in results.** Unlike OAuth/CLI, the
direct API path uses `ANTHROPIC_API_KEY` which bills per token at
pay-as-you-go rates. However, the raw `api.anthropic.com/v1/messages`
response body does not include billing data — `total_cost_usd` is a
Claude Code CLI envelope feature, not a raw API field. The binary captures
$0.0000 for all API-path trials because there is nothing to parse, not
because the calls were free. Actual charges are calculable from the token
counts above using published pricing, or visible in the Anthropic billing
console.

## Key Findings

### 1. Haiku's reformatting is training-baked, not context-induced

Haiku fails (`0/5`) even with zero system prompt (direct API, no `system`
field). No context can suppress this behavior — it is part of haiku's
training. The only effective interventions are `direct` and `example`
prompts, which likely work by triggering an override pathway, not by
removing default behavior.

### 2. The OAuth/CLI path injects ~26.5K immovable cache tokens

The Claude Code CLI injects Anthropic's base model system prompt as a
cache layer that `--system-prompt` cannot reach. The flag only replaces
~3.8K of Claude Code's own system prompt. Any `claude -p` invocation
carries at least 26,500 cache-read tokens regardless of prompt flags.

### 3. Direct API eliminates the context floor entirely

With `reqwest` + `ANTHROPIC_API_KEY`, experiments run with 56–151 input
tokens and zero cache reads. This removes billing distortion and clarifies
which behaviors are model-intrinsic vs context-influenced.

### 4. Opus minimal is unreliable via OAuth, perfect via API

Via OAuth: `3/5` pass rate. Via direct API: `5/5`. The ~26.5K base context
destabilizes opus on this narrow task. When context is stripped, opus
handles the task reliably without any special prompting.

### 5. `direct` prompt is the practical optimum

Across both paths and all models, `direct` achieves `5/5` everywhere except
haiku (where it succeeds but no prompt can fix haiku's zero-system failure).
It uses fewer output tokens than `example` (43 vs 47) and is the minimal
effective instruction.

### 6. API path is 2–3× faster per invocation

Median latencies via direct API: haiku 0.70–1.23s, sonnet 1.13–1.44s, opus
2.03–2.25s. Via OAuth: 1.95–3.89s per invocation. The CLI startup and
nesting detection overhead is ~1.5–2s per call.

## Infrastructure Notes

### Subprocess Invocation (OAuth/CLI Path)

`claude -p` is invoked with:
- `--model {tier}` — haiku, sonnet, or opus
- `--system-prompt "..."` — replaces Claude Code default (or omitted for default)
- `--no-session-persistence` — ephemeral, no transcript saved
- `--output-format json` — enables token/cost/latency data in JSON envelope
- `--` separator before user message

All `CLAUDE*` env vars are stripped to prevent nesting detection:
`CLAUDECODE`, `CLAUDE_CODE_ENTRYPOINT`, `CLAUDE_CODE_DISABLE_AUTO_MEMORY`.

### Direct API Invocation

`reqwest` POST to `api.anthropic.com/v1/messages`:
- `Authorization: Bearer {ANTHROPIC_API_KEY}`
- `anthropic-version: 2023-06-01`
- Model ID mapped: `haiku`→`claude-haiku-4-5-20251001`, `sonnet`→`claude-sonnet-4-6`,
  `opus`→`claude-opus-4-6`
- `system` field omitted entirely for `default` variant (not just empty string)

**Cost not captured**: The raw API response does not include `total_cost_usd`
or `duration_api_ms`. These are CLI envelope fields only.

### Timeout

90-second subprocess timeout via `tokio::time::timeout`. Reports `[TIMEOUT]`
instead of hanging.

### Stdout Flushing

All `println!` calls are followed by explicit `flush()` to ensure output
is visible before any subprocess hang.

## Workbench Structure

```
Study/
  study_workbench.sh              # BUK workbench (routes study-* colophons)
  study-model-prompt-tuning/
    Cargo.toml                    # Depends on vvc, tokio, serde_json, reqwest
    src/main.rs                   # All experiment logic
    README.md                     # This file

.buk/
  launcher.study_workbench.sh     # Launcher stub

tt/
  study-mpt.Run.smoke.sh          # Tabtarget: smoke test (OAuth)
  study-mpt.Run.FULL.sh           # Tabtarget: full matrix (OAuth)
  study-mpt.Run.api-FULL.sh       # Tabtarget: full matrix (direct API)
```

## Future Work

- **Capture API cost**: Add token-count-based cost calculation for the API
  path (pricing table compiled into binary; API doesn't return billing data)
- **Extend to BCG-style experiments**: Test constraint text vs pattern
  examples with more complex tabular structures
- **Haiku characterization**: Explore exactly what instruction forms can
  override haiku's reformatting instinct beyond `direct` and `example`
- **Context size experiment**: Test how much system prompt context is needed
  to destabilize models (opus minimal fails at 26.5K — what threshold?)
- **Temperature sensitivity**: Re-run failed trials with temperature=0 to
  confirm stochasticity vs determinism
