# OQ-2 — Does cygwin bash exhibit the same `$`-eating?

## Hypothesis (original WSG framing)

WSG line 266–269: "Untested. The c-letter path is cmd.exe → C:/cygwin64/bin/bash.exe (no wsl.exe intermediate). Likely behavior differs and may admit a simpler escape rule."

## Resolution

Resolved by probe 1H in `oq-1.md`: cygwin bash does NOT pre-substitute `$name`. The same body that returns empty through wsl.exe (`ztmp=HELLO; echo TMPVAL=$ztmp`) returns `TMPVAL=HELLO` through cygwin.

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'C:/cygwin64/bin/bash --login -c "ztmp=HELLO; echo TMPVAL=$ztmp"'
# stdout: TMPVAL=HELLO
```

This is consistent with cygwin's argv path: cmd.exe → Windows argv parser → C:/cygwin64/bin/bash.exe. There is no wsl.exe intermediate to substitute env vars in argv. Bash receives the body as-passed (modulo cmd.exe and Windows argv-parser conventions, which do not include `$name` substitution per SH-4).

## Promotion plan

Add to WSG: "On the c-letter (cygwin) path, `$name` and `$(...)` reach bash without pre-substitution. No `\$` escape is required for body-side variables and command substitutions, unlike the w-letter (wsl.exe) path."

This makes the c-letter path simpler to author than w-letter.

## Caveats

- This was tested with `--login`. Whether cygwin bash without `--login` behaves the same was not separately verified; the cygwin transport in `bujb_jurisdiction.sh` uses `--login` so the rule covers production usage.
