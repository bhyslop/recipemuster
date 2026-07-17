# Fable 5 Vanished From /usage Mid-Week — Outage, Not Cutoff

**Date:** 2026-07-17
**Domain:** Claude subscription / Claude Code model availability (operator-facing, not project infrastructure)
**Status:** Diagnosis recorded — most likely cause identified, one confirmation step still open

---

## Summary

On 2026-07-17, Fable 5 stopped appearing in Claude Code's `/usage` output — not
exhausted, but absent (no line at all). This happened while I was well under the
promotional cap (30% consumed the day before) and two days before the announced
July 19 subscription cutoff, so neither the cap nor the deadline explains it.

The most probable cause is an **active Anthropic outage**: status.claude.com
showed a live incident, *"Elevated errors across Fable 5,"* opened 2026-07-17 at
18:32 UTC (status: Investigating). A model under an active incident gets pulled
from Claude Code's available set, and `/usage` stops reporting a line for a model
it currently can't route to. Expected outcome: Fable and its remaining weekly
allowance return once the incident clears. **Not** a reason to buy usage credits.

---

## What I observed

- **Yesterday (2026-07-16):** Fable 5 usage tracked normally in `/usage`, sitting
  at ~30% of the weekly Fable allowance. In prior weeks I had pushed the same
  meter to 98–99% without losing the line.
- **Today (2026-07-17):** `/usage` reports **no Fable line at all** — not an
  exhausted meter, an absent one. Other models (Opus, Sonnet, Haiku) unaffected.

The absence-not-exhaustion distinction is the whole tell: a spent cap leaves a
0%-remaining line; a pulled model leaves no line.

---

## The published terms (what SHOULD be true)

Anthropic moved Fable 5 off standard subscription pools in early July citing
capacity, then reversed under backlash and kept it included on paid plans through
a chain of extensions: June 22 → July 7 → July 12 → **July 19, 2026 (11:59:59 PM
PT)**. Current promotional terms on Pro/Max/Team/select-Enterprise:

- Fable 5 included for **up to 50% of the weekly usage limit**, drawing from the
  same weekly pool as other models.
- Past 50%, continue on prepaid usage credits or switch models.
- **From July 20:** all Fable 5 usage runs on prepaid credits at **$10/M input,
  $50/M output**. Anthropic says this is temporary and intends to restore Fable to
  subscriptions "as soon as capacity allows."

By these terms, at 30% consumed and before July 19, Fable should still be present
and metered today. It is not. **The written policy does not describe any
mid-cycle removal, capacity pause, or per-account withdrawal** — I checked the
Anthropic announcement directly for exactly this and found no such language.

---

## The resolving explanation

status.claude.com (redirected from status.anthropic.com), read 2026-07-17:

- **Live:** "Elevated errors across Fable 5" — 2026-07-17 18:32 UTC, Investigating.
- **Same-day resolved incidents:** elevated errors on Sonnet 5 (18:27 UTC), Opus
  4.8 (15:33 UTC), Sonnet 5 + Haiku 4.5 (12:21 UTC) — Fable has been flickering
  all week, so it may bounce in and out until stabilized.

An active-incident pull reconciles every fact: the timing (today), the symptom
(absent line, not exhausted), the scope (Fable only, other models fine), and the
silence in the written terms (this is an operational event, not a policy change).

---

## What to expect

- Once the incident resolves, Fable should reappear in the model picker and
  `/usage`, with the ~70% remaining allowance intact (incident time generally
  isn't charged against the cap).
- Subscription access to Fable remains valid **through July 19**; the two days
  were real.

## Actions

1. Watch **status.claude.com** for the incident to move to Resolved.
2. Cross-check Claude Code's **model picker** (`/model`): if Fable is absent there
   too, it's pulled server-side (confirms outage) rather than a `/usage` display
   quirk.
3. **Do not** buy usage credits on the assumption of being cut off — the likely
   cause is transient.
4. **Escalation trigger:** if Fable is still gone well after the incident clears,
   AND consumption is under 50%, AND it's before July 19 — that is a genuine
   billing/account discrepancy that violates the published terms and warrants a
   support ticket.

---

## Sources

- Claude Status — https://status.claude.com/ (live "Elevated errors across Fable
  5" incident, 2026-07-17)
- Redeploying Claude Fable 5 — Anthropic — https://www.anthropic.com/news/redeploying-fable-5
- Claude Fable 5 stays free for paid users until July 19 — BleepingComputer —
  https://www.bleepingcomputer.com/news/artificial-intelligence/claude-fable-5-stays-free-for-paid-users-until-july-19-as-anthropic-buys-more-time/
- Claude just delayed the Fable 5 paywall again — Android Authority —
  https://www.androidauthority.com/claude-fable-5-access-extended-3686668/

---

*Note: this memo records an external-vendor operational situation, not
recipemuster/RBK project knowledge. It is a dated operator note, not a spec home
for any durable fact.*
