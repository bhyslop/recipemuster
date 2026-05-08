# OQ-3 — Does Linux bash on a non-Windows ssh target exhibit any of these?

## Hypothesis (original WSG framing)

WSG line 271–275: "Untested. Likely no — no cmd.exe in path, no Windows argv parser, just remote bash directly. But verify; the ssh-to-Linux path may have its own quirks."

## Resolution: deferred (no Linux/Mac fundus in matrix)

The only registered BURN profile in this regime is `bujn-winpc` (`BURN_PLATFORM=bubep_windows`). There is no Linux or Mac investiture available for the probe vehicle (`tt/buw-jpS.PrivilegedSsh.sh`).

Rationale for deferral being acceptable:

- The b-letter ssh path on a Linux/Mac fundus bypasses every Windows-specific layer that produces the quirks in OQ-1, OQ-2, OQ-5, and OQ-6: no cmd.exe DefaultShell, no Windows argv parser, no wsl.exe argv substitution. The remote command goes ssh → remote sshd → user shell (bash) directly.
- The body-as-arg shape (SH-1) is the canonical one, untouched by the Windows-only failure modes. SH-2 (`;`-join one-line) is conservatively still useful (some sshd configs may impose newline handling), but is not load-bearing.
- The `$` escape rule from OQ-1/OQ-4 is wsl.exe-specific by mechanism. Linux bash via direct ssh has the bash quoting model exclusively; rules from BCG carry forward without Windows-side modification.

If a Linux or Mac fundus is added in the future, the matrix to run against it is: probes 1A, 1B, 1G, 1J, 1P from `oq-1.md` (substituting `bash -c "..."` directly, no wsl.exe / cygwin wrapper). Expectation: every probe behaves as plain Linux bash quoting predicts; no `\$` escape needed for body-side variables.

## Promotion plan

In WSG: replace the OQ-3 stub with a "deferred — no Linux/Mac fundus in release-1 matrix" entry citing this experiment file. Add a one-line note: "b-letter (Linux/Mac native ssh) follows BCG body discipline directly; no Windows-layer escape rules apply."

The deferral is bounded — adding a Linux/Mac BURN profile is the trigger, not a separate decision.
