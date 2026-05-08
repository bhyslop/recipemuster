# OQ-7 — Multi-line bodies via temp-file feed

## Hypothesis (original WSG framing)

WSG line 298–304: "SH-2 mandates `;`-join for cmd.exe transit. Whether process substitution (`bash <(echo body)`) or remote-side file-feed (write to remote temp file via separate scp, then `bash <file>`) cleanly avoid the cmd.exe newline issue is open. May enable larger bodies; tests temp-file discipline boundaries for remote-side artifacts."

## Resolution: file feed works, with normalization caveat

Two-phase ship-and-run cleanly avoids both cmd.exe newline fragility and wsl.exe argv `$`-substitution. Recommended only for bodies that materially exceed `;`-join's reach; SH-2 remains the default.

## Probe matrix

### 7A — Single-line `;`-joined body via file feed (sanity baseline)

Phase 1 (write):
```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'powershell -NoProfile -Command "Set-Content -Path C:\Users\bhyslop\probe.sh -Value '\''echo LINE1; echo LINE2; ztmp=HELLO; echo VAL=$ztmp'\''"'
# EXIT_WRITE=0
```

Phase 2 (run):
```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash /mnt/c/Users/bhyslop/probe.sh'
# stdout:
# LINE1
# LINE2
# VAL=HELLO
# EXIT_RUN=0
```

`$ztmp` evaluated as `HELLO` — bash's own expansion against the assignment within the script. **wsl.exe's argv `$`-substitution does not apply** because the script body comes from a file, not from argv. This makes file feed a workaround for OQ-1's escape requirement when the body is heavy in `$` references.

### 7B — Multi-line body via PS array → `Set-Content`

Phase 1 (write):
```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'powershell -NoProfile -Command "Set-Content -Path C:\Users\bhyslop\probe.sh -Value @('\''#!/bin/bash'\'','\''set -e'\'','\''ztmp=$(mktemp)'\'','\''echo TMPVAL=$ztmp'\'','\''rm -f $ztmp'\'')"'
# EXIT_WRITE=0
```

Phase 2 (run):
```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash /mnt/c/Users/bhyslop/probe.sh'
# stderr: /mnt/c/Users/bhyslop/probe.sh: line 2: set: -: invalid option
# stderr: set: usage: set [-abefhkmnptuvxBCEHPT] [-o option-name] [--] [-] [arg ...]
# stdout: TMPVAL=/tmp/tmp.W0BUvegbUi
# EXIT_RUN=0
```

`Set-Content` writes CRLF line endings by default. bash on Linux reads `set -e\r` and treats the `\r` as part of the option, which fails. The script keeps running because `set -e` itself never activated. Output partial.

This is a normalization issue, not a transport issue. The file is on disk; cmd.exe newline fragility was avoided. The remaining problem is the encoding choice on the writer side.

### 7C — CRLF normalization via `tr` before exec

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "tr -d \"\\r\" </mnt/c/Users/bhyslop/probe.sh | bash"'
# stdout: TMPVAL=/tmp/tmp.lNbvGRvEbD
# EXIT_RUN=0
```

CRLF stripped, body runs cleanly.

**Caveat**: this form pipes into `bash` (no `-c` arg, no source file), which means bash reads its script from FD 0. **This re-introduces SH-1's stdin-consumption hazard** if the body spawns any subprocess that itself reads FD 0 (notably `wsl.exe`-interop binaries). For the simple probe body this is fine; for production bodies this form is unsafe.

The safer normalization: write to a *second* file with LF endings, then exec from that file. Two extra steps. Or use a writer that produces LF on the first pass (PS `[System.IO.File]::WriteAllText` with explicit `\n`-joined content; or `Out-File -Encoding utf8` with manual newline construction; or shell-side `printf` via cygwin/wsl).

## Cleanup

Probe.sh removed:
```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'powershell -NoProfile -Command "Remove-Item -Path C:\Users\bhyslop\probe.sh"'
# EXIT_RM=0
```

## Promotion plan

Replace OQ-7 in WSG with a new ✅ rule paragraph:

> **Multi-line bodies via file feed**: When a body materially exceeds what `;`-join (SH-2) can express, the canonical pattern is two-phase: phase 1 ships the body as a remote file via PowerShell `Set-Content` or `[System.IO.File]::WriteAllText`; phase 2 runs `wsl.exe ... bash /path/to/file` (or `cygwin bash --login -c "bash /path/to/file"`). Side benefit: bypasses wsl.exe's argv `$`-substitution since the script body comes from disk, not argv. Caveat: PS `Set-Content` writes CRLF; either use an LF-explicit writer (`[IO.File]::WriteAllText($path, $content)` where `$content` joins lines with `"`n"`) or normalize CRLF→LF into a second file before exec. Do NOT pipe the file into `bash` via stdin (reintroduces SH-1).

This is gated to "bodies that don't fit"; default discipline remains SH-2 `;`-join.

## Caveats (operational)

- The probe used `C:\Users\bhyslop\probe.sh` directly — production code should use a process-unique path under `$env:TEMP` or similar with cleanup discipline.
- The two-phase approach is not atomic: if phase 1 succeeds and phase 2 never runs, the temp file persists. Cleanup belongs in the caller's flow, ideally with a `trap`-equivalent on the curia side.
- Process substitution (`bash <(...)`) was not separately probed; the syntactic complexity through cmd.exe quoting layers is high, and the file-feed approach already covers the use case.
