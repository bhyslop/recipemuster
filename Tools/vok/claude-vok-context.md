## Matricula (vom) Build Discipline

The matricula is a standalone operator-only crate at `Tools/vok/vom/` (never
ships — VOr_q4f); `tt/vow-b.Build.sh` does NOT build it. Always use its own
tabtargets, never raw cargo:

- `tt/vow-mb.MatriculaBuild.sh` — build the vom crate (also the remedy when
  `jjx_sift` reports the census binary missing)
- `tt/vow-mt.MatriculaTest.sh` — run vom unit tests
- `tt/vow-mr.MatriculaRun.sh` — run the matricula binary (the read-only
  report: presentments, estray section, digest audit, cadastre freshness)
- `tt/vow-mc.RenderCadastre.sh` — render the cadastre, the one generated
  complete name-census file (`Tools/vok/vod_cadastre.md`, do-not-hand-edit);
  the hand-curated `claude-*-acronyms.md` digests are never rewritten by it

## Acronym Notes

Annotations for the acronym homes indexed in `claude-vok-acronyms.md` — the per-row descriptions and family topology the index does not carry.

- **RCG**  → `vok/vov_veiled/RCG-RustCodingGuide.md` (Rust Coding Guide - project Rust conventions)
- **VOS0**  → `jjqs_studbook/specs/vok/VOS0-VoxObscuraSpec.adoc` (Vox Obscura Specification — the Vox Obscura cosmology SpecTop; centralizes the Liturgy naming vocabulary and hosts the Obscura distribution machinery, the VOSR* commit/lock family, and the Matricula (VOSMM) as branches. The former standalone VLS-VoxLiturgicalSpec.adoc dissolved into this top.)
- **VOSO** → `jjqs_studbook/specs/vok/VOSO-distribution.adoc` (Vox Obscura distribution-procedure subdoc — pure-consumer subdoc of VOS0 (include::'d under its == Operations branch, defines zero quoins); holds the release/install/uninstall procedure bodies. The operation quoins are defined at the VOS0 cosmology top.)
- **VOSMM** → `jjqs_studbook/specs/vok/VOSMM-entity.adoc` (Vox Matricula entity subdoc — pure-consumer subdoc of VOS0 (include::'d under its == Matricula branch, defines zero quoins); transient inscription census over the naming system; MVP scan + seating validators; worked instance of AXLA `axd_petrify`. Acronym mirrors the `vosmm_matricula` quoin. The Matricula vocabulary itself is centralized at the VOS0 cosmology top.)
- **VOSYD** → `jjqs_studbook/specs/vok/VOSYD-diptych.adoc` (Diptych aspirant sheaf — dual-representation canon format under VOS0's == Diptych branch: word-per-line recto / joined verso, the pilcrow-table lexical law, charset enrollment, immutable mezzanine, projection registers, and the one-grammar-many-consumers spine (codec, validator, recension, vesture recognizer — all vom-resident). Aspirant: mints nothing citable.)
- **VOSVK** → `jjqs_studbook/specs/vok/VOSVK-variants.adoc` (Variant-kinds aspirant sheaf — the closed roster of quoin surface-form kinds (base, plural, possessive, past, progressive), the declared-never-derived law, and the operator/declarator letter slots; one enumeration consumed by both the Diptych grammar and the grimoire. Aspirant: mints nothing citable.)
