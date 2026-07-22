## File Acronym Mappings — VOK Subdirectory (`Tools/vok/`)

- **RCG**  → `vok/vov_veiled/RCG-RustCodingGuide.md` (Rust Coding Guide - project Rust conventions)
- **VOS0**  → `vok/vov_veiled/VOS0-VoxObscuraSpec.adoc` (Vox Obscura Specification — the Vox Obscura cosmology SpecTop; centralizes the Liturgy naming vocabulary and hosts the Obscura distribution machinery, the VOSR* commit/lock family, and the Matricula (VOSMM) as branches. The former standalone VLS-VoxLiturgicalSpec.adoc dissolved into this top.)
- **VOSO** → `vok/vov_veiled/VOSO-distribution.adoc` (Vox Obscura distribution-procedure subdoc — pure-consumer subdoc of VOS0 (include::'d under its == Operations branch, defines zero quoins); holds the release/install/uninstall/freshen procedure bodies. The operation quoins are defined at the VOS0 cosmology top.)
- **VOSMM** → `vok/vov_veiled/VOSMM-entity.adoc` (Vox Matricula entity subdoc — pure-consumer subdoc of VOS0 (include::'d under its == Matricula branch, defines zero quoins); transient inscription census over the naming system; MVP scan + seating validators; worked instance of AXLA `axd_petrify`. Acronym mirrors the `vosmm_matricula` quoin. The Matricula vocabulary itself is centralized at the VOS0 cosmology top.)
- **VOSYD** → `vok/vov_veiled/VOSYD-diptych.adoc` (Diptych aspirant sheaf — dual-representation canon format under VOS0's == Diptych branch: word-per-line recto / joined verso, the pilcrow-table lexical law, charset enrollment, immutable mezzanine, projection registers, and the one-grammar-many-consumers spine (codec, validator, recension, vesture recognizer — all vom-resident). Aspirant: mints nothing citable.)
- **VOSVK** → `vok/vov_veiled/VOSVK-variants.adoc` (Variant-kinds aspirant sheaf — the closed roster of quoin surface-form kinds (base, plural, possessive, past, progressive), the declared-never-derived law, and the operator/declarator letter slots; one enumeration consumed by both the Diptych grammar and the grimoire. Aspirant: mints nothing citable.)

## Matricula (vom) Build Discipline

The matricula is a standalone operator-only crate at `Tools/vok/vom/` (never
ships — VOr_q4f); `tt/vow-b.Build.sh` does NOT build it. Always use its own
tabtargets, never raw cargo:

- `tt/vow-mb.MatriculaBuild.sh` — build the vom crate (also the remedy when
  `jjx_sift` reports the census binary missing)
- `tt/vow-mt.MatriculaTest.sh` — run vom unit tests
- `tt/vow-mr.MatriculaRun.sh` — run the matricula binary
