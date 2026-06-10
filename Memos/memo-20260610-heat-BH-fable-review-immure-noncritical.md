# Fable review — ₢BHAAW immure pace, noncritical findings + carried follow-ups

Date: 2026-06-10
Status: noncritical findings from the pre-wrap adversarial review of the podvm immure pace.
Nothing here blocks the wrap; the two stale-comment sites that DID block it went to the
implementing session directly. All items below are triage material for the ₣BH terminal
memo-walk pace.

## Findings (Fable, this review)

1. **rbgjl07 selection-entry fields are not asserted non-empty.** An entry like `:aarch64`
   (empty disktype) passes the `":" in entry` check and then *matches any descriptor lacking a
   disktype annotation* (`ann.get("disktype","") == ""`). Selection is a curated constant, not
   user input, so exposure is low — but one `test`-shaped assert (both fields non-empty) makes
   the malformed-constant failure loud instead of weird.
2. **rbgjl08/09 run network tools inside `while read < file` loops.** The house-documented
   stdin hazard (see rbld_divine's load-then-iterate comment: a child touching stdin consumes
   the loop's remaining input). gcrane/curl don't read stdin today and the live 2-member run
   processed both rows — but busybox sh has no arrays, so the cheap belt is `< /dev/null` on the
   in-loop tool calls rather than load-then-iterate.
3. **ALLOW_LOOSE also forfeits dead-key detection.** The spine comment correctly says the
   dispatch-time scan replaces MUST_MATCH's requires-side check; the provides-side check (a blob
   key no step references) is now unguarded in both layers. Harmless — a dead key costs nothing —
   recorded so nobody rediscovers it as a surprise.
4. **The CBp_102 python import floor accreted within a day** (`re`, via rbgjl07). CBG updated to
   include it and to state the enumeration is the temporary home. Evidence for the cupel python
   walk owning the floor mechanically (memo-20260610-heat-BH-fable-recommendation-python-import-allowlist).
5. **Two more Director-RBRA manifold sites landed** (`rbldv_Immure.sh` sources the RBRA file and
   passes the path to the token mint, copied from the rbldb template). Consistent with existing
   code — counted here only so the ₣BZ accessor-seam sweep knows its census grew.

## Carried follow-ups (flagged by the implementing session at wrap-readiness)

6. **rbw-di egress-check inventory**: add a line for quay.io's blob-CDN redirect hosts — the
   first breakage point if worker-pool egress posture ever tightens.
7. **buildStepOutputs 4KB ceiling vs the native family**: the 8-leaf podvm-native envelope
   should be size-checked against the now-cited 4KB contract (CBG CBh_103) before the FOLLOWING
   pace reuses the envelope-in-slot shape. Cheap arithmetic at design time, expensive surprise
   at runtime.
8. **The RBSL spec family (RBSLA/B/C/D/E/I/U) is not catalogued in claude-rbk-acronyms.md** —
   family-wide tidy, one entry block.
