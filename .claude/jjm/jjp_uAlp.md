# Paddock: rbk-use-bash-remits

## Context

The `_remit` function pattern was designed to solve exit-status-swallowing when `_capture` output is destructured via `IFS read <<<`. Infrastructure was built (BUC_REMIT_VALID/DELIMITER/assert, rbgu_http_json_remit, rbgu_http_ok_remit) but never gained external callers. Backed out 2026-03-07 to reduce maintenance burden before MVP.

## References

- `Memos/memo-20260307-remit-pattern-backout.md` — full code preservation, git commits (4c9f965b, 58348244, b74534ef), BCG pattern docs, and all four original pace dockets
- Original paces covered: || true suppression patterns, LRO polling design, ~97 call-site migration, legacy eviction + _capture unification evaluation
- The memo preserves final versions of both remit functions (with curl timeouts) and all BUK infrastructure