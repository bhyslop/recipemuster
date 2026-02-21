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

```
User Terminal                Study Binary              Test Claude
     |                            |                        |
     |--- tt/study-mpt.Run.smoke.sh                        |
     |         |                  |                        |
     |    BUK dispatch → build →  |                        |
     |                            |                        |
     |                   smpt smoke                        |
     |                            |-- looks up trial:      |
     |                            |   model=haiku          |
     |                            |   system="minimal..."  |
     |                            |   content=test table   |
     |                            |                        |
     |                            |-- claude -p ---------> |
     |                            |<-- raw response -------|
     |                            |                        |
     |                            |-- diff/evaluate        |
     |<-- log: PASS/FAIL + detail |                        |
```

### Key Design Properties

- **Compiled binary hides experiment**: Test Claudes invoked via `claude -p`
  with `--system-prompt` replacement and no tools cannot see the source
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
- `serde_json` — future use for `--output-format json` parsing

The binary invokes `claude` CLI in print mode via OAuth (no API key needed).

## Invocation

### Tabtargets (BUK infrastructure — logging, temp dirs)

```bash
tt/study-mpt.Run.smoke.sh    # 1 trial: haiku+minimal (connectivity check)
tt/study-mpt.Run.FULL.sh     # All 12 trials, summary matrix
```

Output goes to BUK log files:
- `../_logs_buk/last.txt` — most recent run
- `../_logs_buk/same-study-mpt-*.txt` — per-command history
- `../_logs_buk/hist-study-mpt-*.txt` — timestamped archive

### Direct binary (after `cargo build`)

```bash
smpt plan          # Show trial matrix with full prompt text
smpt smoke         # Single connectivity check trial
smpt trial N       # Run trial N (1-12), verbose output
smpt run           # All 12 trials sequentially
```

## Trial Matrix

4 system prompt variants × 3 models = 12 trials:

| Trial | Model  | System Prompt Variant |
|-------|--------|-----------------------|
| 1     | haiku  | default (Claude Code built-in) |
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

## Evaluation

A trial **passes** if the original table text appears verbatim in the
response (after stripping code fences if the model wrapped it in a
code block).

A trial **fails** with diagnostics:
- Pipe characters detected (markdown table syntax)
- Horizontal rules detected (`---`)
- Header converted to markdown table
- Column alignment altered

## Early Results

Smoke test (haiku + minimal): **FAIL** — haiku immediately converts to
markdown table syntax with pipes and horizontal rules, destroying all
column alignment. This confirms the problem exists and the harness detects it.

## Infrastructure Notes

### Subprocess Invocation

`claude -p` is invoked with:
- `--model {tier}` — haiku, sonnet, or opus
- `--system-prompt "..."` — replaces Claude Code default (or omitted for default)
- `--no-session-persistence` — ephemeral, no transcript saved
- `--` separator before user message

All `CLAUDE*` env vars are stripped to prevent nesting detection:
`CLAUDECODE`, `CLAUDE_CODE_ENTRYPOINT`, `CLAUDE_CODE_DISABLE_AUTO_MEMORY`.

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
    Cargo.toml                    # Depends on vvc + tokio
    src/main.rs                   # All experiment logic
    README.md                     # This file

.buk/
  launcher.study_workbench.sh     # Launcher stub

tt/
  study-mpt.Run.smoke.sh          # Tabtarget: smoke test
  study-mpt.Run.FULL.sh           # Tabtarget: full matrix
```

## Future Work

- Run full matrix to compare prompt variants across models
- Test `--tools ""` for tool context suppression (was removed due to hang)
- Add repeat trials to measure stochasticity
- Extend to BCG-style experiments (constraint text vs pattern examples)
- Explore `--output-format json` for token count data
