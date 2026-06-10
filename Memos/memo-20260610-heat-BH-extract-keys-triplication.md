# Lode extract keys-dump triplication — cleanup candidate (heat ₣BH)

Date: 2026-06-10
Status: finding only, no verdict — for the terminal memo triage / a cleanup pass.

## What exists

Three byte-parallel copies of the same diagnostic block landed in the single-slot
capture extracts (`zrbld_underpin_extract` in `rbldw_Underpin.sh`,
`zrbld_conclave_extract` in `rbldr_Reliquary.sh`, `zrbld_immure_extract` in
`rbldv_Immure.sh`), differing only in the `ZRBLD_*_PREFIX` constant and the
kind word in the messages:

```bash
local -r z_keys_file="${ZRBLD_«KIND»_PREFIX}output_keys.txt"
jq -cr 'keys' "${z_output_file}" > "${z_keys_file}" \
  || buc_die "Failed to read keys from «kind» output"
local -r z_keys=$(<"${z_keys_file}")
test -n "${z_stamp}" || buc_die "«Kind» output carried no stamp in rbls_slot_1 (keys present: ${z_keys})"
```

The operator flagged it at review: this looks like it should have been a subfunction.

## Why three copies were written anyway

- The surrounding extract bodies are themselves already parallel per-kind copies —
  each kind body owns its extract whole (output-file decode, stamp read, fact
  emission, success message), per the spine design where the spine owns no kind
  knowledge and bodies are thin data-plus-extract. The keys dump rode the shape
  that was there rather than minting a shared helper mid-pace.
- The diagnostic was added under a live verify gate (the service-suite failure it
  exists to make self-diagnosing), in the conclave-live-verify pace whose scope
  was the cutover verify, not extract architecture.

## What the cleanup pass should judge

The duplication smell is wider than the four-line keys dump. The three single-slot
extracts are near-identical end to end — same decode call, same stamp jq, same
two fact writes, same success line — differing in prefix constant, brand constant,
and label words. Candidates, in increasing reach:

1. Shared keys-dump helper only (narrow, mechanical).
2. One shared single-slot extract (prefix/brand/label parameterized) consumed by
   underpin/conclave/immure; `rbldb_Bole.sh` stays separate — its multi-slot 1..3
   loop with continue-on-empty is genuinely different shape.
3. Leave as-is if the per-kind-body-owns-its-extract principle is judged
   load-bearing against the parallelism (the paddock's per-kind-verbs premise cuts
   this way for *vocabulary*; whether it extends to extract *implementation* is
   exactly the open judgment).

Evidence for the judgment: the rbls_ sprue sweep (0f18e5594) missed exactly one of
the then-parallel copies (underpin's stamp read) and broke the service suite hours
later — the propagation-drift failure mode is not hypothetical for this block; it
fired today. Same drift class as the gauntlet/skirmish fast-subset finding in the
durations memo.
