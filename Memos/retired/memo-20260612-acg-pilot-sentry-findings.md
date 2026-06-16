# ACG Pilot 1 — Sentry Pair Audit Findings

## Origin

First ACG audit pilot (₣Bb rbk-10-mvp-acg-scrub, pace ₢BbAAI), run 2026-06-12
at top tier in the operator's chat per the pace gate. Pair audited:
`rbmm_moorings/rbmv_vessels/common-sentry-context/rbjs_sentry.sh` (RBJS)
against RBSSR + RBSSS, with RBSPT / RBSII / RBSDS read as the receiving-home
surface (RBSSS cites `{scr_port_setup}` for the entry-port rules, so the
sibling specs are where relocations actually land).

Line numbers below are pinned to commit `1a99c1d02f4`. This memo retires when
the relocation pace dispositions the findings.

## Headline

The sentry pair is **well-specified but double-written**. Every dense comment
in RBJS already has a spec home carrying nearly the same wording — six
cross-medium paraphrase forks, not six unhomed comments. The spec copies are
consistently equal or better (more general, version-scoped). The move needed
everywhere is citation-collapse, not relocation; ACGm_104 was amended
accordingly (see Rulings).

## Findings

Each finding: site, content, move-type, receiving home, and what the residue
keeps. All detect-only; nothing was touched in this pace.

### F1 — Docker default-route ownership (rbjs_sentry.sh:53-57)

Five-line comment: Docker assigns the default route to the alphabetically-first
attached network's gateway, compose `priority` is not honored, so sentry derives
the uplink gateway itself and installs its own route.

- **Move:** ACGm_104 citation-collapse.
- **Home:** RBSSS step 2 NOTEs — already carries the content, *more* precisely
  (engine 28.x / Desktop 28.x version scoping the comment lacks).
- **Residue:** Docker's route choice is not trusted — deliberate override; cite.
- **Also:** the phrase "don't-trust-Docker discipline" appears in both comment
  and RBSSS:25 — ACGm_105 corpus seed.

### F2 — rp_filter loose mode (rbjs_sentry.sh:122-130)

Nine-line comment: strict reverse-path filtering drops published-port ingress
at routing time (before the FORWARD chain sees it) because delivery interface
and reverse route mismatch; loose mode is the actual invariant; rp_filter is
not the load-bearing security control.

- **Move:** ACGm_104 citation-collapse.
- **Home:** RBSPT step 3 — near-verbatim, and generalized ("the framework's
  gateway" where the comment hardcodes Docker Desktop's 192.168.65.1).
- **Residue:** loose is deliberate; strict silently kills entry-port traffic
  before the firewall sees it — the trap is a security-minded editor
  "hardening" it back to 1. Highest-value tripwire of the six.

### F3 — DNAT per-IP classification (rbjs_sentry.sh:133-147)

Fifteen-line comment: entry-port DNAT excludes exactly the two enclave
container IPs via RETURN short-circuit, deliberately NOT whole-CIDR exclusion —
on linux Docker Engine the host attaches to the enclave bridge as a peer with
an in-CIDR address, so whole-CIDR exclusion rejects every legitimate host SYN.
States the invariant: enclave-internal sources MUST NOT reach the entry port
via DNAT.

- **Move:** ACGm_104 citation-collapse.
- **Home:** RBSPT step 4 — carries the rationale and the invariant sentence,
  generalized.
- **Residue:** per-IP RETURN is deliberately not whole-CIDR (the
  empirically-broken "simplification"); cite. The per-IP rules are the site an
  agent is most likely to "clean up" wrongly.
- **Comment-only extras, each with its own disposition:**
  - "Empirically confirmed on cerebro: PREROUTING DNAT fired 0 times" —
    history-time provenance; home is the commit; drop (ACGm_107-adjacent).
  - Concrete IPs (192.168.65.1, 10.0.2.100) — spec's general wording is the
    durable form; drop.
  - Ifrit sortie names (`direct_sentry_probe`, `net_dnat_entry_reflection`) —
    test names never in comments (ruling landed in ACG); linkage rides the
    future cited-constraint anchor: spec defines the invariant, comment and
    sorties cite it.
- **Also:** the invariant sentence is worded in both RBJS:146 and RBSPT:35 —
  ACGm_105 corpus seed.

### F4 — MASQUERADE return-path symmetry (rbjs_sentry.sh:157-160)

Four-line comment: MASQUERADE rewrites the post-DNAT source to sentry's enclave
IP so the bottle's reply routes back through sentry and conntrack reverses the
NAT; without it replies traverse default-via-sentry into transit — empirically
broken on Docker Desktop.

- **Move:** ACGm_104 citation-collapse.
- **Home:** RBSPT step 6 — near-verbatim.
- **Residue:** the MASQUERADE is load-bearing for the reply path; cite.
- **Extra:** "per the topology-reframe scry diagnostic" — history-time
  provenance pointer; home is the commit/memo; drop (ACGm_107-adjacent).

### F5 — conntrack-DNAT authorization (rbjs_sentry.sh:164-167)

Four-line comment: only flows that sentry's own PREROUTING DNAT created
conntrack state for may forward; DNAT-state is unforgeable from either bridge;
return path handled by the parent FORWARD chain's RELATED,ESTABLISHED ACCEPT.

- **Move:** ACGm_104 citation-collapse.
- **Home:** RBSPT step 5 — near-verbatim including "unforgeable from either
  bridge." Cleanest pure duplicate of the six; no extras to salvage.
- **Residue:** the conntrack match IS the authorization; cite.

### F6 — no-forwarding-path DNS comment (rbjs_sentry.sh:265-266)

Two-line comment: no `server=` lines means no forwarding path exists; subdomain
queries return frozen IPs; all non-allowed domains get NXDOMAIN.

- **Move:** ACGm_104 citation-collapse.
- **Home:** RBSDS resolve-then-freeze bullet — fuller than the comment (names
  the DNS subdomain exfiltration channel the design closes).
- **Residue:** the *absence* of `server=` lines is deliberate; cite. Purest
  case for an anchor citation — the trap is an absence, so no weird-looking
  line exists to hang suspicion on; an editor "fixing" missing forwarding
  re-opens the exfil channel.

## Leftovers

- **Spec side is clean (ACGm_106):** RBSPT / RBSII / RBSDS / RBSSR / RBSSS read
  as constraint throughout; no unmarked ornament worth draining. One
  clause-level nit: RBSSS states the sentry-takes-ownership doctrine twice in
  adjacent NOTEs (line 25 tail vs lines 26-28); one-line internal collapse,
  someday.
- **ACGm_107:** nothing in the specs; the two provenance pointers in RBJS are
  catalogued under F3/F4.
- **Non-ACG observation:** rbjs_sentry.sh lines 19-31 echo `RBJp0:` under the
  `RBJp1: Validate parameters` header — phase-label inconsistency in the
  startup announcements. Rides along with the relocation pace. Also the first
  catalogued violation of the unwritten jailer dialect rules — evidence for the
  jailer guide pace.

## Corpus-sweep seeds (ACGm_105, for the later sweep paces)

- Invariant sentence "enclave-internal sources MUST NOT reach the entry port
  via DNAT": RBJS:146 and RBSPT:35.
- "don't-trust-Docker discipline": RBJS:55 and RBSSS:25.
- RBSSS-internal: ownership doctrine stated at line 25 tail and lines 26-28.

## Rulings landed in ACG (this pace)

1. **Citation-collapse** added to ACGm_104 as the named degenerate case —
   the pilot showed it is the common case where the spec side is mature.
2. **Palisade fallback bound** added to the three-homes section: spec is
   authoritative for Palisade characterization when a home exists; the comment
   keeps residue only (signature + tripwire + citation). Includes the context-
   economy division rule: code carries what an editor needs *before* deciding
   to load the spec; the spec carries everything after.
3. **Test names never in source comments** + the **cited-constraint anchor**
   recorded as a deferred MCM/AXLA-level mechanism, with the
   wait-don't-cite-twice sequencing rule for relocation sweeps.
4. **One file, one guide** added to Related Guides.

## Cited-constraint anchor — design leanings (for the design pace)

Direction agreed in the pilot conversation; mechanism deferred. Leanings to
confirm or overturn at mount — only the direction is cinched:

- **Category taxonomy sketch** (from this pilot's specimens): *invariant*
  (must-hold property — F3/F5), *foreign-signature* (Palisade characterization
  — F1/F2's Docker behaviors), *deliberate-deviation* (don't-simplify tripwire
  — F2/F3/F6), *membrane* (Palisade workaround carrying a demolition date, per
  ROE conduct rule 5 — none in this pair, predicted category).
- **Opaque dense ID suffix** (firemark-flavored), not readable words: specs
  stay closed-source while code is released, so a readable anchor name leaks
  the security reasoning into open identifiers; an opaque token is inert. The
  tripwire *sentence* stays human-readable; only the anchor is opaque. Opaque
  IDs also carry near-zero rename pressure — no semantics to go stale — which
  keeps them out of cross-universe recension scope (diptych memo boundary).
- **Category as a definition-site field, not an ID letter:** firemark precedent
  (identity carries no semantics); recategorization must not remint released
  IDs; the validator answers grep-by-category by joining citations to
  definitions.
- **Allocator:** opaque IDs need collision-free minting; vvx is the precedent
  (jjx mints coronets). Open question for the design pace.
- **Validator linkage:** anchor rules join the diptych Phase-4 validator
  feedstock (memo-20260209-diptych-vision.md) — every code citation resolves to
  a spec definition; categories can carry per-category obligations (a membrane
  must name its demolition condition; an invariant must name a defender).
- **Naming:** the `RBs_`-style prefix and the category's minted word are
  design-pace work under full mint discipline (grep gate noted: lowercase
  `rbs*` is partially occupied; mixed-case anchors break the snake_case anchor
  convention — a deliberate break to record if taken).

## Decisions of record

- **Heat tipped toward guidance over code repair.** Pilot verdict: the guide
  needed amendments more than the script needs edits. Mechanical clincher: the
  six residues should cite anchors, so collapsing now with prose pointers means
  touching every site twice. Relocations deferred to a pace gated on the anchor
  mechanism.
- **Regenerate-from-specs strategy** (operator, this conversation): trials
  regenerating code from spec + guide with cheaper models, diffed against the
  live code, as the forcing function that measures guide/spec quality. Makes
  guides load-bearing infrastructure and the dedup direction a prerequisite,
  not hygiene. Anchor IDs localize regeneration failures.
- **Jailer dialect promoted to its own guide** despite rule-of-3: a guide is
  codification, not abstraction (low regret), and the regenerate strategy makes
  the dialect guide the generator's input for the security envelope — the code
  where an ungoverned agent "improvement" toward BCG idiom is most dangerous.

## Disposition

- ACG amendments: landed (this pace).
- CLAUDE.md memos-are-provenance test: landed (this pace).
- Cited-constraint anchor design: slated pace (₣Bb).
- Jailer dialect guide authoring: slated pace (₣Bb).
- Six citation-collapses + provenance drops + RBJp0/RBJp1 fix: slated pace
  (₣Bb), gated on the anchor mechanism; that pace retires this memo.
- ACGm_105 corpus seeds: held here for the corpus sweep paces.
- RBSSS internal nit: held here; rides any future RBSSS-touching pace.
