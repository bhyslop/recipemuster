# memo-20260511-windows-transport-wrapper-postlude-synthesis

## Topic

Why the canonical PowerShell wrapper in `bujb_jurisdiction.sh` (and any
sibling helper that invokes `powershell -NoProfile -Command "<body>"`
under ssh) has exactly three slots — prelude, body, postlude — and why
each piece is the minimum-viable response to a specific PowerShell
behavior. Integrative companion to WSp-101 (LASTEXITCODE init), WSp-102
(lazy formatter flush), WSp-105 (single-expression caller bodies and
wrapper carve-out), and WSp-109 (unconditional `"` escape in wrapper
helpers). WSG states each rule individually; this memo names the
synthesis that motivates the wrapper's exact shape and explains why
the apparently-simpler alternatives all fail.

## The wrapper anatomy

The canonical form, as it appears in `zbujb_admin_powershell`:

```bash
"${BUJB_ps_invoke_command} \"${BUJB_ps_prelude} ${z_body_escaped}; if (\$LASTEXITCODE -ne 0) { exit \$LASTEXITCODE }\""
```

Decomposed into three slots:

- **Prelude** (library, fixed):
  `$ErrorActionPreference = 'Stop'; $env:WSL_UTF8 = 1; $LASTEXITCODE = 0;`
- **Body slot** (caller-supplied, single expression per WSp-105):
  `${z_body_escaped}` — the caller's body, passed through the WSp-109
  `" → \"` escape transform unconditionally.
- **Postlude** (library, fixed):
  `if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }`

WSp-105 explicitly carves the prelude and postlude out of the "no
compound statements in PS bodies" rule — the constraint applies to
caller bodies, not to wrapper library code that surrounds the body
slot.

## Why each piece is load-bearing

The wrapper exists because PowerShell carries four behaviors that,
taken together, break the naïve assumption *"the body runs, then PS
exits with the right code."* Each scaffolding token neutralizes one
of them.

### Native binaries do not propagate exit codes through PS sessions

`wsl.exe`, `net.exe`, `icacls.exe`, and any other native binary set
`$LASTEXITCODE` to their own exit code, but the surrounding
PowerShell session's exit code is independent. If `wsl.exe --import`
returns 6, PS still exits 0, ssh returns 0, and bash sees success.
The postlude bridges this gap: *"if a native binary in the body set
`$LASTEXITCODE` non-zero, propagate that as PS's exit code so ssh
sees it."* This is the load-bearing reason the postlude exists at
all. Without it, every native-binary failure in every caller body
would be silently invisible to bash.

### `$LASTEXITCODE` is `$null` in a fresh PS session (WSp-101)

Before any native binary has run, `$LASTEXITCODE` is `$null`.
PowerShell's typed comparison evaluates `$null -ne 0` as `True`. An
unconditional trailing `exit $LASTEXITCODE` would therefore fire
`exit $null` on every body that didn't run a native binary. Two
consequences: the process exits spuriously, and the lazy
object-formatter pipeline is aborted before draining (see next
item). The prelude's `$LASTEXITCODE = 0` neutralizes this so the
postlude's test fires only on real native-command failures.

### Cmdlet object output renders lazily (WSp-102)

PowerShell flushes string output (Write-Host, plain strings) eagerly,
but cmdlet object output (`Get-LocalUser` tables, `Get-Date` objects,
anything that goes through `Out-Default → Format-Table → Out-Host`)
renders lazily — at session end or on explicit flush. Calling `exit`
mid-session aborts the formatter pipeline; buffered output is
discarded. The bash side captures empty stdout and cannot distinguish
"cmdlet failed" from "cmdlet returned nothing." The postlude's
`if`-guard solves this by skipping `exit` entirely on happy paths: PS
reaches end-of-session naturally, every formatter drains, all output
emerges, and PS exits 0 on its own. The postlude fires only on the
failure path, where preserving error propagation outweighs preserving
table output.

### Exit codes carry information bash uses for recovery decisions

`wsl.exe` exit 6 means "distribution not registered." `userdel` exit
6 means "user does not exist" — explicitly tolerated by the obliterate
function per memo-20260511-orchestration-style-axla-draft. `icacls`
exit codes distinguish access-denied from path-not-found from
transport-failed. Flattening these to `exit 1` would force every
bash callsite into probe-then-act or brute-force retry. The
postlude's `exit $LASTEXITCODE` (not `exit 1`) preserves the
information channel for exit-code-aware decisions on the bash side.

## Elimination argument

Remove any single piece and a specific failure mode reappears:

| Remove | Failure reintroduced |
|--------|----------------------|
| Prelude `$LASTEXITCODE = 0` | WSp-101 null-exit trap; postlude fires `exit $null` on every happy path, lazy formatters discarded |
| Postlude `if`-guard | Even with prelude init, `exit 0` on happy paths aborts lazy formatters mid-render (WSp-102) |
| Postlude `exit $LASTEXITCODE` (drop entirely) | Native-binary non-zero exits in body invisible to bash; failures silent |
| `exit $LASTEXITCODE` → `exit 1` | Bash loses distinguishable exit codes; recovery decisions collapse to brute-force retry |
| WSp-109 `" → \"` escape on body slot | Caller bodies containing `"` corrupt cmd.exe argv parsing on transit |

The three postlude tokens (`if`, `-ne 0`, `exit $LASTEXITCODE`) and
the one prelude token (`$LASTEXITCODE = 0`) are the minimum-viable
scaffolding for all five concerns simultaneously. The shape looks
elaborate because PowerShell's behavior is rich in traps; the wrapper
is the codified minimum to navigate them while preserving WSp-105's
single-expression discipline for caller bodies.

## Why this is wrapper code, not body code

WSp-105's enumeration is constructive (one cmdlet, one native binary,
or one native binary with inner shell — `if` is none of those) and
applies to *caller-supplied bodies*. The wrapper's prelude and
postlude are not body content; they are the discipline shell that
surrounds the body slot. WSp-105 names this exemption explicitly.
Without it, no wrapper could solve the four-trap problem above
without either violating WSp-105 or pushing exit-code propagation
responsibility onto every caller.

The asymmetry is intentional: callers stay simple (one expression
each, bash owns the state machine per the Capture-Decide-Dispatch
pattern), the wrapper absorbs the PowerShell-specific complexity once
at the library boundary. The complexity has to live somewhere; the
WSG choice is to concentrate it in one helper rather than smear it
across every callsite.

## Anti-pattern: caller bodies imitating the wrapper shape

A common misreading — recorded in
memo-20260510-windows-transport-ps5-anti-rationalization — is that
the wrapper's `if (\$LASTEXITCODE -ne 0) { exit \$LASTEXITCODE }`
trailer licenses callers to use similar shapes in their own bodies.
It does not. The carve-out is wrapper-side only; caller bodies must
remain single expressions per WSp-105. Decisions inside caller
bodies — including `if`-guards over destructive operations — push
state machinery into PowerShell, which is exactly the failure mode
WSp-105 was added to prevent. Decisions belong in bash via CDD;
the wrapper-side postlude is library scaffolding, not a model to
imitate.

## Cross-references

- WSp-101 (LASTEXITCODE init) — prelude rationale
- WSp-102 (lazy formatter flush) — postlude `if`-guard rationale
- WSp-105 (single-expression caller bodies, wrapper carve-out) — why this shape is permitted
- WSp-109 (unconditional `"` escape in wrapper helpers) — body slot transit
- `Memos/memo-20260510-windows-transport-ps5-anti-rationalization.md` — failure modes when caller bodies imitate the wrapper shape
- `Memos/memo-20260511-orchestration-style-axla-draft.md` — exit-code tolerance as information channel (userdel exit 6, false-branches principle)
