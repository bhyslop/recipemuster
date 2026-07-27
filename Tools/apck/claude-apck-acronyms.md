### File Acronym Mappings

| Acronym | File | Description |
|---------|------|-------------|
| **APCAS** | `apck/APCAS-Specification.md` | Application spec — UX, workflow, deployment |
| **APCS0** | `apck/APCS0-SpecTop.adoc` | Detection pipeline spec — formal vocabulary (MCM concept model) |
| **APCPS** | `apck/APCPS-PrototypeSpecification.md` | Prototype spec — tech stack, data sources, project structure, container architecture, wire-format JSON schema |
| **APCW** | `apck/apcw_workbench.sh` | Workbench |
| **APCZ** | `apck/apcz_zipper.sh` | Zipper enrollment |
| **APCC** | `apck/apcc_cli.sh` | CLI command implementations |
| **APCAP** | `apck/apcd/src/apcap_main.rs` | Tauri app entry point |
| **APCAL** | `apck/apcd/src/apcal_main.rs` | Fixture loader (clipboard writer) |
| **APCAD** | `apck/apcd/src/apcad_main.rs` | Dictionary refresh (downloads public sources, regenerates dictionaries) |
| **APCAB** | `apck/apcd/src/apcab_main.rs` | Batch assay (run detection pipeline on HTML directory, write assay output) |
| **APCNSA** | `apck/apcd/src/apcnsa_main.rs` | Historical Stanford ONNX spike binary — reference/offline assay tool. The production Stanford discerner now runs in the container per APCS0; this binary is retained as reference code. |

**Source modules:**

| Source | Prefix | Test | Prefix | Purpose |
|--------|--------|------|--------|---------|
| `apcrl_log.rs` | `apcrl` | — | — | Logging macros (`apcrl_info!`, `apcrl_error!`, `apcrl_fatal!`) + optional file-tee sink via `apcrl_tee_init` |
| `apcre_engine.rs` | `apcre` | `apcte_engine.rs` | `apcte` | PHI detection orchestrator |
| `apcrp_parse.rs` | `apcrp` | `apctp_parse.rs` | `apctp` | HTML clipboard parsing |
| `apcrm_match.rs` | `apcrm` | `apctm_match.rs` | `apctm` | Dictionary/regex matching |
| `apcrd_dictionaries.rs` | `apcrd` | `apctd_dictionaries.rs` | `apctd` | Dictionary loading |
| `apcru_update.rs` | `apcru` | — | — | Self-update watcher (no unit tests — I/O + process) |
| `apcrh_harvest.rs` | `apcrh` | `apcth_harvest.rs` | `apcth` | Clipboard harvest orchestrator — creates journal dir, scans next index, delegates flavor enumeration to `apcrb_pasteboard` |
| `apcrj_journal.rs` | `apcrj` | — | — | Journal directory path resolver — `$HOME/apcjd/` holds harvests, normalized-text container inputs, container output JSON, anonymized outputs, `apcap.log`, and `container-log.txt`; also the container's bind-mount target |
| `apcrb_pasteboard.rs` | `apcrb` | `apctb_pasteboard.rs` | `apctb` | macOS NSPasteboard FFI — enumerates the first item's declared UTIs via `objc2-app-kit`, writes each `dataForType` payload to `{N}-in.{tag}.{ext}`; non-macOS stub returns an honest error |

**Other key paths:**
- `Tools/apck/apcd/ui/` — Frontend (HTML/CSS only — no JavaScript)
- `Tools/apck/apcd/dictionaries/` — Blacklist/whitelist data files
- `Tools/apck/test_fixtures/` — Synthetic Epic clipboard data
