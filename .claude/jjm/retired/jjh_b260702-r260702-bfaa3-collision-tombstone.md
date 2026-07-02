# Heat Trophy: bfaa3-collision-tombstone

**Firemark:** ₣Bp
**Created:** 260702
**Retired:** 260702
**Status:** retired

## Paddock

# Paddock: bfaa3-collision-tombstone

## Context

(Describe the initiative's background and goals)

## References

(List relevant files, docs, or prior work)

## Paces

### terrier-noun-sheaf-carve (₢BpAAA) [abandoned]

**[260702-0237] abandoned**

Drafted from ₢BfAA3 in ₣Bf.

Carve the terrier noun-internals out of RBS0 into a companion normative sheaf, so the terrier quoin reads as a one-line definition plus include::s — symmetric with the realm->RBSFK and terrier-access->RBSTR sheaf precedents, and consistent with how every operation block already sheafs.

## Spec of needed change
The terrier noun internals (resource shape, muniment shape + rbgft_ sprue, key format, the provider collision-rationale) are the heaviest inline concept block in RBS0's civic-quoin section, under the terrier and muniment quoins.
Relocate that internals prose into a new normative sheaf that RBS0 include::s at the terrier quoin, beside the existing RBSTR access include.
The terrier quoin's anchor + one-line definition stay inline in RBS0; only the heavy internals move.

Do NOT fold the noun into RBSTR — RBSTR's header deliberately scopes it to access-only and defers the noun to RBS0, so a fresh noun sheaf is the consistent move, not widening RBSTR.
Mint the sheaf acronym in the RBST* terrier family applying the mint gates (lean: RBSTN, noun); register it in claude-rbk-acronyms.md.

## Done when
The terrier noun internals live in a normative sheaf include::d at the terrier quoin; RBS0's terrier quoin carries only its definition plus the two includes (noun, access); every quoin anchor, mapping-section attribute reference, and axvd_ normative marker resolves unchanged; the rendered spec is byte-faithful to today's content.

## Cinched
Cosmetic relocation ONLY — no rewording of normative statements, no change to specification rigor.
Anchors, attribute references, and normativity markers are preserved; the move is a pure include:: relocation mirroring RBSTR/RBSFK.
If execution reveals rewording is needed to make the split cohere, that is out of scope — stop and surface it.

## Character
Mechanical AsciiDoc relocation; the care is in preserving anchors/refs/normativity, not in judgment.

## Commit Activity

```
File-touch bitmap: (no work file changes)
```

## Steeplechase

### 2026-07-02 02:37 - Heat - D

₢BfAA3 → ₢BpAAA

### 2026-07-02 02:37 - Heat - N

bfaa3-collision-tombstone

