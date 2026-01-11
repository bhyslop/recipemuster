# Steeplechase: Vox First Whisper

---
### 2026-01-11 08:30 - bootstrap-vok-vvk - APPROACH
**Proposed approach**:
- Create minimal `Tools/vok/` with Cargo.toml (`name = "vvr"`) and src/main.rs using clap for `--version`
- Create `Tools/vvk/bin/vvx` platform wrapper (uname detection, execs `vvr-{platform}`)
- Build for darwin-arm64 (current platform), copy binary to `Tools/vvk/bin/vvr-darwin-arm64`
- Verify `vvx --version` works in source location
- Copy VVK to temp directory, verify it still works (simulates target install)
---

---
### 2026-01-11 09:00 - bootstrap-vok-vvk - WRAP
**Outcome**: Created VOK crate, VVK wrapper, release dirs, gitignore for dev/release separation
---

---
### 2026-01-11 09:01 - write-vok-readme - APPROACH
**Proposed approach**:
- Extract key concepts from paddock (already well-documented)
- Structure as kit-author-facing reference (not end-user docs)
- Include concrete examples: directory trees, command invocations
- Keep vocabulary glossary concise with linked terms
---

---
### 2026-01-11 09:15 - write-vok-readme - WRAP
**Outcome**: Created 301-line README covering compilation model, two-repo install, veiled convention, Voce Viva, Rust architecture, API, glossary
---

---
### 2026-01-11 09:16 - clarify-kit-facility-patterns - APPROACH
**Proposed approach**:
- Expand existing "Kit Facilities" section in VOK README
- Add decision matrix: when to use each facility type
- Clarify that kits can have multiple facilities
- Document model differential tool placement (JJ workbench)
---

---
### 2026-01-11 09:20 - clarify-kit-facility-patterns - WRAP
**Outcome**: Expanded Kit Facilities section (301→355 lines): types table, when to use each, combinations matrix, tool placement flowchart
---

---
### 2026-01-11 09:22 - create-vok-skeleton - APPROACH
**Proposed approach**:
- Create stub files: voa_arcanum.sh, build.rs, src/guard.rs, src/core.rs
- Create empty vol_ledger.json
- Update Cargo.toml to reference new modules
- Keep implementations minimal (logic in later paces)
---

---
### 2026-01-11 09:35 - create-vok-skeleton - WRAP
**Outcome**: Added voa_arcanum.sh, build.rs, vol_ledger.json; renamed sources to vor* mint prefixes (vorm_main.rs, vorg_guard.rs, vorc_core.rs); updated README prefix table
---
