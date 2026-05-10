# memo-20260510-windows-transport-ps5-anti-rationalization

## Topic

PS-5 `if`-as-guard rationalization, and the `-ErrorAction Ignore`
substitution that followed. Empirical record motivating WSG additions:
Core Philosophy *Rules enumerate; they don't illustrate*, PS-5 tightening
to exclusive enumeration with second forbidden example, new rule PS-8
*Error suppression is not idempotency*, and the named Capture-Decide-Dispatch
(CDD) pattern with worked exemplars.

## Session

Cycle of `bujb_jurisdiction.sh` edits in 2026-05 cleaning up the WSL
install purge and cleanup steps (`bujb_wsl_install`, steps [1/6] and
[6/6]). The original bodies were compound state machines `;`-joining
3–4 statements each:

```bash
local -r z_purge_body="if ((wsl.exe --list --quiet) -match '${BUJB_wsl_distribution}') { wsl.exe --unregister ${BUJB_wsl_distribution}; if (\$LASTEXITCODE -ne 0) { exit \$LASTEXITCODE } }; if ((wsl.exe --list --quiet) -match '${BUJB_wsl_seed_distribution}') { wsl.exe --unregister ${BUJB_wsl_seed_distribution}; if (\$LASTEXITCODE -ne 0) { exit \$LASTEXITCODE } }; if (Test-Path '${z_tar_path}') { Remove-Item -Force '${z_tar_path}' }; if (Test-Path '${z_distro_dir}') { Remove-Item -Recurse -Force '${z_distro_dir}' }"
```

The agent (Claude Opus 4.7) decomposed them across three cycles before
reaching the correct shape. This memo records the two failure cycles to
prevent repetition.

## Cycle 1 — single guarded effect

First decomposition produced bodies like:

```bash
zbujb_admin_powershell "if ((wsl.exe --list --quiet) -match '${dist}') { wsl.exe --unregister ${dist} }"
zbujb_admin_powershell "if (Test-Path '${z_tar_path}') { Remove-Item -Force '${z_tar_path}' }"
```

The agent justified these as PS-5 compliant on the reading "single
guarded effect = one expression." This is wrong. PS-5's enumeration is
"one cmdlet call, one native binary call, or one native binary with an
inner-shell body" — `if ... { ... }` is none of those. The body contains
a decision (the test) and an action (the consequent of `{}`); even one of
each is two distinct things, and the decision belongs in bash per PS-5's
closing line "Bash owns the state machine."

The miss was a misread of rule type. PS-5 enumerates allowed shapes
constructively, not illustratively. The agent treated the enumeration as
examples of a more general "single effect" notion and admitted a fourth
shape that wasn't in the list.

User catch: *"Why do I still see 'if' inside of powershell statements?"*

## Cycle 2 — error suppression as idempotency

Repair attempt 2 substituted `-ErrorAction Ignore` for the `if`-guard:

```bash
zbujb_admin_powershell "Remove-Item -Force '${z_tar_path}' -ErrorAction Ignore"
zbujb_admin_powershell "Remove-Item -Recurse -Force '${z_dir}' -ErrorAction Ignore"
```

This trades a structural (PS-5) violation for an output-quality (OQ-1)
violation. `-ErrorAction Ignore` swallows every error class, not just
"target absent." Permission denied, file locked, ACL refusal — all
absorbed silently. The cmdlet appears to succeed; the actual file might
still be present. The function continues as if removal succeeded.

The agent had also been about to apply the `-ErrorAction Ignore` form to
step [6/6] cleanup before being challenged. The challenge surfaced the
silent-failure trade before further propagation.

User catch: *"Explain why you think ErrorAction Ignore is a good idea?"*

## Cycle 3 — correct CDD shape

Capture-Decide-Dispatch using `zbujb_powershell_capture` for state probes
and `zbujb_admin_powershell` for unconditional dispatches. State machine
in bash; PS bodies single-expression. See WSG §"Capture-Decide-Dispatch
Pattern" and §"Idempotency Exemplars" for the canonical recipe.

Step [1/6] purge: capture `wsl.exe --list --quiet` once for the
distro-membership test, then `Test-Path` separately for tar and dir.
Decide via `grep -qFx` and `[[ "$x" == "True" ]]`. Dispatch unconditional
`wsl.exe --unregister` and `Remove-Item` only on the bash-side branches
that need them. Step [6/6] cleanup: unconditional `wsl.exe --unregister`
(the seed must be registered at this point — we just exported from it)
followed by `Test-Path` capture + conditional `Remove-Item`.

## Lessons

1. **Rule type matters.** Enumerative rules with a closed list of allowed
   shapes look like illustrative rules with examples of a category. The
   reading flips between strict and lax depending on whether the agent
   needs a way out. WSG must declare rule type explicitly and forbid
   reasoning by analogy. (→ Core Philosophy *Rules enumerate; they don't
   illustrate* addition.)

2. **Convenience flags hide failure modes.** `-ErrorAction Ignore` looks
   like a parameter; structurally it's an error trap. PowerShell doesn't
   distinguish the two in syntax. WSG's general "trap, don't trust"
   philosophy didn't reach down to specifically forbid the
   convenience-flag idiom; PS-8 codifies the prohibition on destructive
   actions and explicitly distinguishes from probe-side use.

3. **Procedure beats principle.** "Bash owns the state machine" is correct
   but underspecified — the agent had to derive the right shape each time.
   Naming the procedure (Capture-Decide-Dispatch) and providing
   copy-pasteable exemplars converts a derivation problem into a
   pattern-match problem. Pattern matching is more robust against
   rationalization than derivation.

4. **The bool-predicate detour.** During WSG revision, the question of a
   `zbujb_powershell_predicate` (BCG `_predicate` class) came up. The
   function would collapse capture+compare into
   `if zbujb_powershell_predicate ...; then` form. The detour was tabled
   on the observation that ssh-layer transport failure adds a third value
   that doesn't fit either bool answer cleanly — strict BCG predicate
   semantics ("never dies, status only") would conflate transport failure
   with "false," reintroducing OQ-1 silent-swallow. WSG documents the
   canonical shape as `zbujb_powershell_capture` + `[[ "$x" == "True" ]]`,
   leaving the predicate variant as an open design question (logged in
   WSG §"Open question: bool-returning predicate variant").

## Verification

Corrected `bujb_wsl_install` purge (step [1/6]) and cleanup (step [6/6])
in `bujb_jurisdiction.sh` follow CDD throughout. Six exemplars added to
WSG's *Idempotency Exemplars* section cover the recurring patterns this
codebase actually uses (file/dir presence, WSL distro membership, local
user existence, service running state, registry key absence).

## Related

- `Tools/buk/vov_veiled/WSG-WindowsScriptingGuide.md` — Core Philosophy,
  PS-5, PS-8, Capture-Decide-Dispatch Pattern, Idempotency Exemplars
- `Memos/memo-20260508-windows-transport-experiments.md` — earlier
  transport-stack empirical record (OQ-1 through OQ-7)
- `Tools/buk/bujb_jurisdiction.sh` — `bujb_wsl_install` is the empirical
  site for cycle-3's correct CDD shape
- `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md` — `_predicate` /
  `_capture` function class definitions (BCG:710–786, BCG:1290)
