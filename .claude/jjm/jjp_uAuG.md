# Paddock: jjk-post-alpha-breaking

## Context

Breaking schema changes for JJK post-alpha cleanup. These paces intentionally break
backward compatibility with prior gallops formats and should only race after all legacy
gallops files have been migrated.

## References

- jjrt_types.rs — Gallops and Heat struct definitions
- jjri_io.rs — load/save path
- jjro_ops.rs — nominate, retire, furlough operations
