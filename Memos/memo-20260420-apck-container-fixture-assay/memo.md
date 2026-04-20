<!--
Copyright 2026 Scale Invariant, Inc.
SPDX-License-Identifier: Apache-2.0
See LICENSE for terms.
-->

# APCK Container — First Fixture Assay

**Date:** 2026-04-20
**Pace:** ₢A9AAZ (container-scaffolding)
**Inputs:** `test_fixtures/epic_initial.txt` (5,147 chars), `test_fixtures/epic_geriatric.txt` (4,988 chars)
**Outputs:** `epic_initial.json` (382 KB), `epic_geriatric.json` (361 KB) — sibling files in this directory
**Image:** `apck-container:local` (1.73 GB, Python 3.11-slim + torch 2.5.1-cpu + transformers 4.46.3 + scispaCy 0.5.5 + Stanza 1.9.2)

## Why this memo exists

The pace docket's smoke test used a single curated one-liner
(`"Patient: Margaret Thornton. DOB: 03/15/1952. Seen at Mercy General by Dr. Chen."`)
which exercised the wire contract but not the discerners' behavior on realistic
clinical prose. This memo captures the first end-to-end run of both HTML
fixtures (after whitespace-normalization via a stdlib HTML-to-text helper that
mirrors `zapcrp_normalize_whitespace` in `apcrp_parse.rs`) so that subsequent
paces — combining, Rust-side wiring, schema pruning — can reason from evidence
rather than assumption.

The two JSON files beside this memo are unedited container outputs and should
be treated as reference data. They are large because all three discerners emit
token-level findings; that size is itself a design data point (see
Engineering Notes below).

## Headline result

The container works end-to-end on realistic clinical text. The APCS0/APCPS
complementarity claim — Stanford strong on narrative PHI, weak on structured
identifiers; Stanza providing independent corroboration via OntoNotes NER;
spaCy contributing syntactic signal — is validated by concrete examples in
both outputs.

## Stanford de-identifier — strong narrative, weak on structure

### What it nailed (confidence ≥ 0.99)

- **All patient + family names**: Margaret J. Thornton, Robert Thornton, Ms.
  Thornton, Harold W. Eriksen, Karen Eriksen-Moody, Thomas Eriksen, Miriam
  Eriksen, Mr. Eriksen.
- **All clinicians as HCW**: Dr. Susan Chen, James R. Whitfield MD, Priya
  Anand MD, Elena Vasquez PA-C, Naomi Okwu MD, Patricia Nowak, Dr. Anand,
  Dr. Whitfield.
- **Every date at confidence 1.0**: DOBs, visit dates, historical years —
  9 dates on the geriatric fixture including freestanding years (`2024`,
  `1961`) and month-year stubs (`04/2026`, `01/2026`).
- **All structured IDs that didn't fragment**: MRN `00847293`, account
  `3391057842`, SSN `471-83-2956`, BCBS plan `BCBS-MEM-992817445`, device
  serial `1234567890`, DEA-style `AW1234567`.
- **All phone numbers** across both fixtures.

### Where it got confused

- **Address ↔ facility conflation.** "1847 Cranberry Lane, Westbrook, ME
  04092" (home address) and "Maine Medical Center" (a real hospital) both
  tagged `HOSPITAL`. The model has no separate ADDRESS label — APCS0
  documents this and the fixture assay makes it concrete.
- **ID fragmentation.** `ENC-2026-0051203` on the geriatric fixture split
  into two findings: `ENC-` at confidence 0.98 and `-0051203` at 1.0, as
  separate `ID` entries. Exactly the "structured identifiers fragment
  across labels" failure mode described in APCS0. Combining should prefer
  regex evidence for structured IDs.
- **URL/email surgery.** Email `m.thornton47@gmail.com` fragmented into
  `m.thornton47` tagged PATIENT + `gmail`/`com` tagged VENDOR. Portal URL
  `https://mychart.mainehealth.org/patient/thornton-m` fragmented into
  `mychart`/`mainehealth` (VENDOR) + `thornton-m` (PATIENT). Regex for
  email and URL shapes handles these cleanly.
- **Low-confidence noise.** "SS" (0.62) tagged HOSPITAL — a stray fragment
  from "SSN" context. Combining can reasonably filter sub-0.7 non-name
  labels; every high-signal finding was ≥ 0.95.

## spaCy — syntactic gold, NER underwhelming

- **1,008 / 942 token findings** (POS, morph, lemma, dep, head). This is
  the real value of spaCy in this pipeline: feedstock for homograph
  disambiguation in combining (e.g., "May" as proper noun vs. modal verb).
- **283 / 290 entity findings, all labeled generically `ENTITY`.**
  `en_core_sci_md`'s default NER emits a single entity type — it was
  designed for biomedical entity detection, not typed classification. The
  NER output is therefore low-value for PHI discrimination; combining
  should weight spaCy by its syntactic outputs, not its entity list.

## Stanza — complementary NER taxonomy, some leakage

- **PERSON captures mirror Stanford exactly**: independent corroboration
  on every person name in both fixtures. This is the two-parser
  redundancy the spec calls for.
- **GPE** (geo-political): Westbrook, Portland, Scarborough, Cape
  Elizabeth, South Portland — the locality signal Stanford conflates into
  HOSPITAL. Combining can separate "this is a city" (GPE) from "this is
  a hospital" (HOSPITAL) using Stanza evidence.
- **FAC** (facility): addresses + Maine Medical Center, separating
  facility from narrative context more cleanly than Stanford's HOSPITAL
  bucket.
- **ORG**: mostly right (Maine Medical Center) but with false positives
  on medical abbreviations (NSTEMI, PCI, ED, EMS) — Stanza treats
  all-caps strings as organizations.
- **CARDINAL leakage**: `471-83-2956` (SSN) and `555-0143` (phone
  fragment) tagged `CARDINAL` — PHI hidden as "just a number".
  `207-555-0488` tagged `TIME`. Combining should recognize that
  structured-ID regex evidence dominates Stanza's numeric
  misclassifications.

## Predictions from APCS0 / APCPS — validated

1. **Stanford narrative strength + structured weakness** — confirmed with
   concrete examples.
2. **Two independent parsers corroborating PERSON** — confirmed.
   Stanford PATIENT/HCW and Stanza PERSON agree across every name in
   both documents.
3. **NER taxonomy complementarity** — confirmed. Stanza GPE, FAC,
   CARDINAL give combining signals Stanford lacks.
4. **Regex will earn its keep** — confirmed. SSN, phone, encounter ID,
   email, URL all parse cleaner in regex than any of the three models.

## Engineering notes worth carrying forward

- **JSON size: ~370 KB per normalized-page.** For the Rust consumer,
  this is substantial deserialization per clipboard event. Options to
  consider in the next pace: (a) schema-side prune in the container
  (e.g., drop token-level findings whose `pos` is punctuation,
  drop Stanza CARDINAL when the surface matches a known regex
  structured-ID shape); (b) Rust-side streaming parse that skips
  uninteresting records.
- **Steady-state throughput is fast.** Both fixtures processed in
  under a second each after warmup. The docket's 30–90 s estimate
  referred to cold model load; once `ready`, discerner latency per
  document is negligible at this text size.
- **Drop-old staging matters for testing.** The container processes
  only the highest-indexed `{N}-in.txt`. If you copy both fixtures
  into `~/apcjd/` before the scanner tick, only the higher-numbered
  file gets processed. The assay sequenced 10001 → wait for 10001.json
  → 10002 → wait for 10002.json. Worth calling out in the container
  README.
- **Container security posture held.** Runtime was `--network=none
  --cap-drop=all --read-only --user nobody:nogroup` throughout both
  assays. No network-exfiltration surface; PHI stayed within the
  bind-mount.

## Items for subsequent paces to study

These are flagged by the data, not prescribed. The design conversation
can continue from here.

- **Combining rule for HOSPITAL vs. ADDRESS vs. GPE**: Stanford's
  HOSPITAL label is doing triple duty. A combining rule that triangulates
  Stanford HOSPITAL + Stanza FAC/GPE + regex address shape into distinct
  apcs_phi_category values would eliminate the conflation.
- **Structured-ID precedence**: when regex says "SSN" and Stanford says
  two fragmented IDs at the same offsets, regex wins. When regex says
  "email" and Stanford says VENDOR+PATIENT fragments, regex wins. A
  combining primitive for "regex overrides model fragmentation at the
  same span" appears load-bearing.
- **spaCy entity weight**: given all spaCy entities are generic, its
  entity findings may be net noise. Combining could weight spaCy at
  zero for entity decisions, use it only for syntactic features.
- **JSON payload pruning**: if the Rust consumer only needs tokens for
  specific homograph disambiguation cases, token-level findings could
  be filtered in the container to reduce JSON size by an order of
  magnitude. Decision depends on whether combining wants to evaluate
  token-level features broadly or only selectively.
- **VENDOR label as PHI**: `gmail`, `com`, `mychart`, `mainehealth` are
  all technically low-entropy strings caught by the VENDOR label.
  Whether Safe Harbor requires these scrubbed is an APCAS question,
  not a detection-pipeline question — but the evidence is collected
  either way.

## Reproduction

```
# HTML-to-text normalization (stdlib only; mirrors apcrp_parse whitespace collapse)
python3 Tools/apck/.../normalize.py  # ad hoc during this run

# Container lifecycle
tt/apcw-cb.ContainerBuild.sh
tt/apcw-cs.ContainerStart.sh

# Wait for readiness
until grep -q "ready" ~/apcjd/container-log.txt; do sleep 1; done

# Stage fixtures sequentially (drop-old policy requires this)
cp Tools/apck/test_fixtures/epic_initial.txt   ~/apcjd/10001-in.txt
until test -f ~/apcjd/10001.json; do sleep 1; done
cp Tools/apck/test_fixtures/epic_geriatric.txt ~/apcjd/10002-in.txt
until test -f ~/apcjd/10002.json; do sleep 1; done

tt/apcw-cx.ContainerStop.sh
```
