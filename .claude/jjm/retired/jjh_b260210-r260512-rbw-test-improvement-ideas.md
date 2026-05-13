# Heat Trophy: rbw-test-improvement-ideas

**Firemark:** ₣Aa
**Created:** 260210
**Retired:** 260512
**Status:** retired

## Paddock

# Paddock: rbw-test-improvement-ideas

## Context

(Describe the initiative's background and goals)

## References

(List relevant files, docs, or prior work)

## Paces

### suite-git-precondition (₢AaAAA) [rough]

**[260210-1459] rough**

Add a suite-level precondition that fails fast if the local repo is not clean and aligned with its remote. This addresses failures caused by misalignment between local repo state and cloud state. The precondition should check that the working tree is clean (no uncommitted changes) and that the local branch is in sync with the remote, aborting the suite early with a clear error message if not.

### decide-inactive-gate-strictness (₢AaAAB) [rough]

**[260216-0547] rough**

Decide whether regime validators should reject non-empty variables belonging to inactive gates.

## Context

During the ATAAp regime voicing audit, we aligned RBRV with the AXLA gated-group pattern
(axhrgc_gate). The current implementation (matching RBRN) is permissive: variables from
inactive gates are kindled and listed in z_known but not validated. For example,
RBRV_VESSEL_MODE=bind with RBRV_CONJURE_DOCKERFILE set would silently carry the conjure
value without error.

The AXLA spec defines activation semantics for groups but does not mandate rejection of
competing groups' members.

## Options

1. **Permissive (status quo):** Inactive gate vars are silently ignored. Simple, forgiving,
   matches current RBRN behavior.
2. **Warn:** Log a warning for non-empty vars in inactive gates, but do not die.
3. **Strict:** Die if non-empty vars belong to an inactive gate. Catches misconfiguration
   but may break workflows that template all vars and select mode separately.

## Scope

If we decide to add strictness:
- Define the behavior in AXLA (normative spec)
- Implement in RBRN and RBRV validators as the pattern
- Audit other regimes with conditional groups

## Decision criteria
- Does silent carriage of inactive vars cause real bugs?
- Would strictness break reasonable workflows (e.g., template files with all vars)?
- Is the complexity of enforcement worth the safety gain?

## Commit Activity

```
File-touch bitmap: (no work file changes)
```

## Steeplechase

### 2026-02-16 05:47 - Heat - S

decide-inactive-gate-strictness

### 2026-02-10 14:59 - Heat - S

suite-git-precondition

### 2026-02-10 14:58 - Heat - N

rbw-test-improvement-ideas

