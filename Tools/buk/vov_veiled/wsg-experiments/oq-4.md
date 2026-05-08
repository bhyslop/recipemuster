# OQ-4 — Canonical escape rule for body-side `$var` and `$(...)`

## Hypothesis (original WSG framing)

WSG line 277–282: "If OQ-1 settles on a specific layer eating `$`, the canonical escape may be `\$` (which empirically survives), or a different mechanism. The rule that goes here governs how `zbujb_admin_exec` callers write bodies that need remote-side variable expansion or command substitution."

## Resolution

Drops out of OQ-1 (`oq-1.md`). The escape rule is per-letter:

| Letter | Path                                       | `$name` escape | `${name}` escape | `$(...)` escape |
|--------|--------------------------------------------|----------------|------------------|-----------------|
| b      | direct ssh to Linux/Mac (deferred — OQ-3)  | (deferred)     | (deferred)       | (deferred)      |
| c      | cmd.exe → cygwin bash                      | none needed    | none needed      | none needed     |
| w      | cmd.exe → wsl.exe → bash                   | `\$name`       | `\${name}`       | none needed     |

Probe pair (verification):

```
# ❌ Fails on w-letter — wsl.exe substitutes $ztmp to empty before bash runs
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "ztmp=HELLO; echo $ztmp"'
# stdout: (empty line)

# ✅ Works on w-letter — \$ defers expansion to bash
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "ztmp=HELLO; echo \$ztmp"'
# stdout: HELLO
```

## `zbujb_admin_exec` caller contract

Looking at the current `zbujb_admin_exec` (`Tools/buk/bujb_jurisdiction.sh:306-335`), it ships the body literally — caller writes the body string, the function joins multi-arg form with `;` and does `"` → `\"` escaping. **It does not transform `$`.** Therefore: callers calling `zbujb_admin_exec w` must write `\$varname` and `\${varname}` for body-side expansion; callers calling `zbujb_admin_exec c` may use unescaped `$varname`.

The caller contract becomes asymmetric across letters. This is empirically required (wsl.exe vs cygwin behave differently), not a discipline choice. Workarounds:

1. **Status quo (asymmetric)**: caller knows the letter, caller escapes per-letter. The current bujb_jurisdiction.sh callers already do this (e.g. line 529's `bash -c 'test -e /home/''${z_wlu}'' && echo PRESENT || true'` — this is via `zbujb_admin_powershell` which wraps wsl.exe inside PowerShell single quotes, a different escape regime; the discipline holds within its lane).
2. **Symmetric (uniform)**: have `zbujb_admin_exec` apply `$` → `\$` substitution for w-letter only. Hidden cost: callers writing `\$` for compatibility would get `\\$` after the transform; not robust against caller-already-escaped bodies.
3. **Symmetric via temp-file (OQ-7)**: ship body as a remote temp file, then `bash <file>`. Bypasses wsl.exe's argv substitution entirely.

For release-1 the **status quo** is the rule of record. Callers write per-letter; this document is the contract reference.

## Promotion plan

Replace OQ-4 in WSG with a new rule "SH-N: `$` escape rule per shell letter" containing the table above and the probe pair.

The "Bash-via-wsl.exe wrapper discipline" section in WSG (currently flagged "open — see OQ-1, OQ-4") can be filled in: body-as-arg (SH-1), `;`-joined (SH-2), `\"` for embedded `"` (SH-3), `\$` for `$name` and `${name}` (new), `$(...)` not affected (new).
