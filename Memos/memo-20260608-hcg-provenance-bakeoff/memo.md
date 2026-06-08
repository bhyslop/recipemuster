# HCG Provenance — the two-model authoring bake-off

**Memo:** `memo-20260608-hcg-provenance-bakeoff`
**Datestamp:** 2026-06-08
**Title:** HCG Provenance — the two-model authoring bake-off

*Source artifacts authored 2025-10-12 in cnmp.*

## What this is

This directory is the development archive (provenance) of **HCG** — the Handbook
Curation Guide (`Tools/buk/vov_veiled/HCG-HandbookCurationGuide.md`). HCG was
previously the cnmp lens **PCG (Procedure Curation Guide)**; it was moved into
this repo and renamed (rbm commit `35edba781`), freeing the PCG acronym for the
PlantUml Coding Guide. These seven files are the record of how that guide was
originally written.

They originated in `cnmp_CellNodeMessagePrototype:lenses/bpu-PCG-workups/` and
were rehomed here when the guide left cnmp — the making-of follows the guide.

## The bake-off

Two model families each drafted the meta-spec independently, then each merged the
two drafts and reflected on the process. Opus's merge became the release.

| File | Stage |
|------|-------|
| `001-chatgpt5p0-basicInterview.md`        | ChatGPT-5.0 — first-pass draft |
| `002-claudeOpus4p1-basicInterview.md`     | Claude Opus 4.1 — first-pass draft |
| `003-chatgpt5p0-guidedMix001and002.md`    | ChatGPT-5.0 — merge of 001 + 002 |
| `003-chatgpt5p0-retrospective.md`         | ChatGPT-5.0 — process retrospective |
| `004-claudeOpus4p1-guidedMix001and002.md` | Claude Opus 4.1 — merge of 001 + 002 |
| `004-claudeOpus4p1-retrospective.md`      | Claude Opus 4.1 — process retrospective |
| `005-claudeOpus4p1-release.md`            | **Released guide** — byte-identical to HCG as imported |

## Why it earns a home here

The throughline — sharpest in the two retrospectives — is the **"Digital Mind as
primary author and steward"** pattern: documents *authored by* an LLM,
*interpreted by* an LLM in later sessions, *approved by* a human on outcomes
rather than deep reading, and *refined through* backpressure when a procedure
reveals a gap. That inverts human-writes-for-humans documentation, and it rhymes
directly with this repo's own AI-for-AI doc philosophy (the officium, the CMK
salutation, dockets dense-by-default). The concept is the reason this archive is
worth keeping; `005` itself is redundant with HCG and is retained only for
provenance integrity.

## Preserved verbatim — including the corruption

The source files are imported **byte-for-byte, uncorrected** (cksum-verified on
import). They carry their own mojibake — e.g. `Meta Spec � This document`, a dash
mangled at original authorship in 2025. It is left in place as historical record;
this repo does not rewrite historical artifacts. Fittingly, repairing exactly
this class of em-dash corruption in the federation-tier diagrams is the thread
that surfaced this archive in the first place.
