# Heat Trophy: rbw-gcb-builder-control

**Firemark:** ₣AQ
**Created:** 260125
**Retired:** 260622
**Status:** retired

## Paddock

# Paddock: rbw-gcb-builder-control

## Context

(Describe the initiative's background and goals)

## References

(List relevant files, docs, or prior work)

## Paces

### design-gcb-builder-pinning (₢AQAAA) [rough]

**[260125-1410] rough**

Design and implement digest-pinned GCB builder images. Research options: (A) pre-built jq image with digest pin, (B) alpine with digest pin + runtime apk install, (C) custom tool image in depot. Document in RBAGS, update RBSTB spec to include _RBGY_JQ_REF and _RBGY_SYFT_REF substitutions. Ensure validation (buv_env_odref) aligns with chosen approach.

## Commit Activity

```
File-touch bitmap: (no work file changes)
```

## Steeplechase

### 2026-01-25 14:10 - Heat - S

design-gcb-builder-pinning

### 2026-01-25 14:10 - Heat - N

rbw-gcb-builder-control

