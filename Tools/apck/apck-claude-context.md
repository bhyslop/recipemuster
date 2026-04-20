## Ann's PHI Clipbuddy Kit (APCK)

APCK is a Tauri desktop app that intercepts clipboard content from Epic EHR, detects PHI via ten discerners running across two placements (seven in the Rust application, three in a long-running container), presents a triage view, and writes anonymized plain text to clipboard for pasting into Open Evidence. The conceptual vocabulary of the detection pipeline lives in APCS0; implementation and wire-format details in APCPS.

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

**No JavaScript rule:** All rendering logic lives in Rust. The Tauri webview is a passive display surface — Rust pushes complete HTML on every state change via `eval()`. Inline `onclick` attributes use the Tauri bridge primitive (`window.__TAURI__.core.invoke`) for command dispatch. No `.js` files, no JS application logic. This eliminates the Rust-JS language boundary as a bug class.

**Rust conventions:** Follow RCG (`Tools/vok/vov_veiled/RCG-RustCodingGuide.md`). Source prefix: `apcr{classifier}_{name}.rs`. Test prefix: `apct{classifier}_{name}.rs`. Classifier matches between source and test.

### Tabtargets

| Tabtarget | Colophon | Purpose |
|-----------|----------|---------|
| `tt/apcw-b.Build.sh` | `apcw-b` | `cargo tauri build` (release) |
| `tt/apcw-r.Run.sh` | `apcw-r` | `cargo run --bin apcap` (local development) |
| `tt/apcw-D.Deploy.sh` | `apcw-D` | Build + scp to `anns-macbook-air:/Users/Shared/apcua/` |
| `tt/apcw-fl.FixtureLoad.sh` | `apcw-fl` | Run `apcal` to load fixture HTML onto clipboard |
| `tt/apcw-t.Test.sh` | `apcw-t` | `cargo test` in `apcd/` |
| `tt/apcw-dr.DictionaryRefresh.sh` | `apcw-dr` | `cargo run --bin apcad` (refresh dictionaries from public sources) |
| `tt/apcw-ba.BatchAssay.sh` | `apcw-ba` | `cargo run --bin apcab` (batch assay on HTML directory) |
| `tt/apcw-nsi.NeuralStanfordInstall.sh` | `apcw-nsi` | Neural Stanford spike — convergent install: create venv, install optimum + optimum-onnx + onnxruntime if needed, clear any prior artifacts, re-export ONNX. Always reaches a working state. |
| `tt/apcw-nsa.NeuralStanfordAssay.sh` | `apcw-nsa` | Neural Stanford spike — `cargo run --bin apcnsa` on HTML directory |

### Prefix Tree

```
apc  (non-terminal)
├── apca   (non-terminal)
│   ├── apcab  — App Batch binary (assay — detection pipeline on HTML files)
│   ├── apcad  — App Dictionary binary (refresh from public sources)
│   ├── apcal  — App Loader binary (fixture clipboard tool)
│   ├── apcap  — App Prototype binary (Tauri main)
│   └── apcas  — application specification document (UX, workflow)
├── apcc   — CLI command implementations
├── apcd   — Rust/Tauri source directory
│   ├── apcrb  — macOS NSPasteboard FFI — declared-UTI enumeration for harvest
│   ├── apcrh  — Clipboard harvest orchestrator (delegates enumeration to apcrb)
│   ├── apcrj  — Journal directory path resolver
│   └── apcrl  — Logging macros (info, error, fatal with file/line) + file-tee sink
├── apcj   (non-terminal — journal)
│   └── apcjd  — journal directory ($HOME/apcjd/) — shared by app and container (harvests, normalized inputs, container outputs, logs)
├── apck   — kit directory
├── apcn   (non-terminal — neural)
│   └── apcns  (non-terminal — neural stanford)
│       └── apcnsa  — Historical Stanford ONNX spike binary (reference only)
├── apcps  — prototype specification document
├── apcs   (non-terminal)
│   └── apcs0  — detection pipeline specification (MCM concept model)
├── apcu   (non-terminal)
│   └── apcua — update staging directory (/Users/Shared/apcua/)
├── apcw   — workbench
└── apcz   — zipper
```

### Detection Architecture

Ten discerners across two placements, each a pure function of `apcs_normalized_text`, feeding a single `apcs_evidence` pool; `apcs_combining` (deferred) unifies. See APCS0 for the conceptual vocabulary and APCPS for concrete parameterization and wire format.

**Rust-application discerners (7, in-process):**
- `apcs_regex_scan` — structural patterns (SSN, phone, email, dates, addresses, zip, labeled identifiers)
- `apcs_label_scan` — words following Epic labels (Patient:, Attending:, Facility:, etc.)
- `apcs_surname_scan` — US Census surname dictionary
- `apcs_firstname_scan` — SSA first-name dictionary
- `apcs_city_scan` — US cities dictionary
- `apcs_english_scan` — common-English-word whitelist (suppression evidence)
- `apcs_medical_scan` — medical-term whitelist (suppression evidence)

**Container discerners (3, via bind-mounted JSON):**
- `apcs_stanford_scan` — `StanfordAIMI/stanford-deidentifier-base` (HuggingFace transformers, PyTorch CPU)
- `apcs_spacy_scan` — scispaCy `en_core_sci_md` (POS, dependency parse, biomedical NER)
- `apcs_stanza_scan` — Stanza English UD pipeline (POS, dependency parse, OntoNotes NER)

The container runs as a long-running process with `--network=none`, `--cap-drop=all`, non-root user, read-only root filesystem. Bind-mounts `$HOME/apcjd/` only. Clipbuddy writes `{N}-in.txt` (normalized text); container writes `{N}.json` atomically (via `{N}.json.tmp` → `rename(2)`) consolidating all three container discerners' findings. No sockets, no HTTP — POSIX file I/O is the wire format.

Combining rules are deferred; the prototype's current anonymization is a stand-in derived from Rust-side findings only.

### Deploy Workflow

1. `tt/apcw-b.Build.sh` — produces `.app` bundle
2. `tt/apcw-D.Deploy.sh` — scp to Ann's machine at `/Users/Shared/apcua/`; emits a `=== Forward to Ann ===` block with manual quit + relaunch instructions
3. Self-update watcher is currently dormant (see APCPS); Ann's ceremony is manual quit + relaunch
