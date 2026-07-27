## Matricula (vom) Build Discipline

The matricula is a standalone operator-only crate at `Tools/vok/vom/` (never
ships — VOr_q4f); `tt/vow-b.Build.sh` does NOT build it. Always use its own
tabtargets, never raw cargo:

- `tt/vow-mb.MatriculaBuild.sh` — build the vom crate (also the remedy when
  `jjx_sift` reports the census binary missing)
- `tt/vow-mt.MatriculaTest.sh` — run vom unit tests
- `tt/vow-mr.MatriculaRun.sh` — run the matricula binary
