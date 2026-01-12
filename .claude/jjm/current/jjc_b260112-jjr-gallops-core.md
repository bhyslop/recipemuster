# Steeplechase: JJR Gallops Core

---
### 2026-01-12 10:30 - rename-studbook-to-gallops - APPROACH
**Proposed approach**:
- Rename file `JJD-StudbookData.adoc` → `JJD-GallopsData.adoc`
- Update category declarations in mapping section: `jjdsr_` → `jjdgr_`, `jjdsm_` → `jjdgm_`
- Update all attribute references and anchors using those prefixes
- Search/replace prose references: "Studbook" → "Gallops", "studbook" → "gallops"
- Update file path reference: `jjd_studbook.json` → `jjg_gallops.json`
- Verify with grep that no "studbook" references remain
---
