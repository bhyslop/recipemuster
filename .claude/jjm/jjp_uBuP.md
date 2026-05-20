## Shape

Grab-bag cleanup heat. Theme: consolidate to the single rightful owner — remove capability and constants that are redundantly held in more than one place. Collects post-cutover RBK cleanups of that nature.

## Locked decisions

- **Work that reads RBCC or the moorings layout is gated on ₣BK (moorings cutover) closing** — stable RBCC, settled layout, RBBC aliases retired. Cleanups touching neither are free to run.
- **Constant-projection scope: single canonical bash author, or out.** A constant is projectable to another language only if exactly one bash module canonically owns it (the zipper registry, RBCC flat tinder). Constants whose identity is distributed across bash, disk, and Rust are excluded unless a separate canonical-author decision is taken first.

## What done looks like

The redundancies in scope are gone: no capability that has a better-owned home elsewhere survives in two places, and no constant is hand-maintained on both sides of a language boundary that a single canonical source could feed.