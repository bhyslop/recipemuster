# Paddock: trophy-alpha-reporting

## Dependency

**Blocked on ₣Ah (jjk-v4-vision).** V4 replaces the data model this heat consumes. All original paces were designed around V3 structures (tacks, steeplechase markers, flights) that V4 eliminates. Fresh paces must be slated after V4 schema settles.

## Surviving Ideas

**JSON trophy schema**: A structured JSON format for retired heats remains the right approach. The schema will be redesigned around V4 concepts (voltes, warrants, beats, corral decisions) rather than V3 concepts (tacks, steeplechase, flights).

**Bitmap inclusion**: Store pre-rendered file-touch bitmap and commit swim lane ASCII displays in the trophy. V4's volte branch structure actually enables *richer* bitmaps — per-volte file touches and beat-level attribution. This idea gets better under V4, not worse.

## What Was Abandoned (V3-specific)

Original 5 paces abandoned because they targeted V3 data structures:

- ₢ANAAA trophy-alpha-jjsa-json-format — JJSA spec for V3 trophy schema (tacks, steeplechase, flights)
- ₢ANAAB trophy-alpha-json-types — Rust types for V3 trophy (TrophySession, TrophyPace, Flight)
- ₢ANAAC trophy-alpha-compute — Session clustering from steeplechase timestamps, timeline from tacks
- ₢ANAAD trophy-alpha-generate — Rewrite jjx_retire for V3 JSON output
- ₢ANAAE trophy-bitmap-displays — Bitmap storage (idea survives, implementation needs V4 branch topology)

## References

- ₣Ah paddock — V4 vision: voltes, warrants, beats, school/breeze/corral
- V3 gallops data model: Tools/jjk/vov_veiled/JJS0-GallopsData.adoc