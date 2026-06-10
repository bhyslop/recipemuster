# Fable recommendation — elect_base_anchor slot-count die is a dormant sibling of the brand-leak ordain killer

Date: 2026-06-10
Status: recommendation from the post-wrap ₣BH review (second Fable pass); not yet acted on.

## The behavior

`zrbfd_elect_base_anchor` (rbfd_FoundryDirectorBuild.sh) requires **exactly one** populated
`RBRV_IMAGE_n_ORIGIN` slot and `buc_die`s otherwise ("needs exactly one populated slot, found N").
The check runs whenever a bole touchmark is present in the depth-1 chain — before the build submits.

## Why it is the same failure family as the fixed brand bug

Commit 91666a97b fixed: a *non-bole* fact chained ahead of an ordain killed the ordain pre-submit.
The remaining shape: a *bole* fact chained ahead of an ordain of a **multi-origin vessel**
(the RBRV regime authors slots 1–3) dies the same way — pre-submit, one second in, on a real
operator sequence (ensconce vessel A's base, then ordain multi-origin vessel B before any other
dispatch ages the chain out).
Same mechanism, same cost profile as the bug ₢BHAAT diagnosed live; only the discriminator differs
(slot count instead of kind-brand).

## Dormancy

No live vessel populates `RBRV_IMAGE_2_ORIGIN`/`_3_ORIGIN` today (checked 2026-06-10: zero hits
across rbmm_moorings and the vessel regimes), so the die is unreachable until a multi-origin
nameplate exists. It will fire on the first one that ordains behind a chained bole fact.

## Recommended repair

Mirror the brand fix: when the slot count is not exactly one, log-and-leave-the-ANCHOR-as-is
(loud `buc_log_args`, not fatal) — the election simply cannot disambiguate, and absence-of-election
is already a normal outcome of this function. Reserve `buc_die` for genuinely corrupt state.
Decide whether count==0 (bole fact present, no origin slot at all) deserves the same no-op or a
warning with a different message.
