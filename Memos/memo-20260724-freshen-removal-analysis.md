# Freshen / Managed-Include-Region Removal — Architecture Review

**Date:** 2026-07-24
**Provenance:** Produced during the ₣B5 (implement-matricula) grooming session, at
operator request, by a fable-model architecture review of the live tree. Informs
pace ₢B5·CAABU (`retire-freshen-mechanism`). This memo is provenance, not
authority — the pace's own census re-verifies every site against the live tree;
specifics here are as-of this date.

## Decision

Remove the machine-managed `CLAUDE.md` include region and its Freshen lifecycle
**in full** — not partial. A region wearing `MANAGED:` markers with no Freshen
behind it is the worst of both. Migrate to plain hand-`@`-includes. Justified by:
rbm is the sole near-consumer, and the two former consumers (`pb_paneboard02`,
`djo-DanielsJupyterObsidian`) are stale/dead installs.

## Pivotal fact

Claude Code `@`-imports accept **only explicit file paths — no globs, no
directories** (code.claude.com/docs/en/memory; glob support exists only in
`.claude/rules/` frontmatter, a different mechanism). So the replacement is an
explicit hand-`@` block, and the win is deleting the *compiled* `claude_includes`
registry plus the regeneration lifecycle — not the file enumeration, which stays.

## Load-bearing (must survive removal)

- **Curation record.** `claude_includes` names a curated *subset*, not a
  directory listing: CMK ships five `claude-cmk-*.md` but auto-loads two
  (roe/salutation are hand-included outside the block, roe-detail is on-demand);
  `claude-jjk-images.md` never auto-loads. "Include everything the parcel copied"
  would over-load context. rbm's inlined block becomes the record; a per-kit
  README line captures launch-vs-on-demand for any future consumer.
- **Install atomicity.** emplace currently ends with a working `CLAUDE.md` in a
  single commit (VOSO "installed atomically"; a "deterministic, no LLM" note).
  Removal replaces that with a manual "add these `@` lines" step — a bet that no
  second consumer appears (`pb` is still a registered cipher and is the
  migration test's named motivating consumer).

## Known-safe (de-risks removal)

- Release pipeline untouched: `vofr_release.rs` never reads `claude_includes`
  (collection is veiled-exclusion based).
- No freshness gate exists: `rbq_qualify` never checks freshen-currency, so the
  block is already only as current as the last hand-run of `vow-F`.
- HTML markers are stripped from context — no token cost either way.
- The two RBK build-generated files (`rbtdgc_consts.rs`, tabtarget-context) are
  zipper machinery with zero coupling to this.
- No stale-marker hazard: rbm's block is the only in-tree instance; the two
  sibling repos each carry the identical three-include block (buk-core,
  jjk-core, vvk-core).
- Uninstall is less fragile than it looks: vacate preserves kit directories, so
  orphaned `@`-lines still resolve; only wholesale-deleting `Tools/{kit}` dangles
  them, and that degrades visibly in `/context`, not silently.

## Deletable surface (census re-verifies against live tree)

- `voff_freshen.rs` (the whole module + its tests; a legacy-migration sweep that
  only serves consumers that don't exist).
- `vofe_emplace.rs` freshen/collapse plumbing (`zvofe_freshen_claude`,
  `zvofe_build_include_body`, `zvofe_legacy_tags`, `zvofe_collapse_claude`,
  `vofe_freshen_forge`).
- `vvx_freshen` command (`vorm_main.rs`).
- `vob_freshen` (`vob_build.sh`).
- zipper enrollment (`voz_zipper.sh`) + `tt/vow-F.Freshen.sh`.
- `claude_includes` + `VOFC_INCLUDE_REGION_TAG` (`vofc_registry.rs`).
- the `vosof_freshen` procedure + the install/uninstall `CLAUDE.md` steps
  (`VOSO-distribution.adoc`).
- the VOS0 include-region/freshen quoins (`vose_include_region`,
  `vose_kit_include`, `vose_marker`, `vose_uninstalled_marker`).

## Suggested migration order

1. Inline rbm's managed block into plain `@` lines; remove the markers and the
   "mirrors what consumers receive" comment.
2. Record per-kit launch-vs-on-demand include lists in each kit's README (the
   easy step to skip — don't; it is the curation record).
3. Delete the tabtarget + zipper row + `vob_freshen`.
4. Delete `vvx_freshen`, the emplace/vacate `CLAUDE.md` steps, `voff_freshen.rs`,
   `claude_includes`, `VOFC_INCLUDE_REGION_TAG`.
5. Rewrite the VOSO install/uninstall procedures (install gains a documented
   manual "add `@`-include lines per the kit's README" step) and retire the VOS0
   quoins — in the same pace, so no spec claims removed machinery.
6. `tt/vow-t.Test.sh` green.
7. Two sibling repos (`pb_paneboard02`, `djo-DanielsJupyterObsidian`): strip the
   markers to plain `@` lines, commit in each repo separately.

## If the manual-enumeration tax ever bites

The future fix is `.claude/rules/` routing (directory-discovered, glob-scoped) —
NOT resurrecting Freshen.
