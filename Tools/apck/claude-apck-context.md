## Ann's PHI Clipbuddy Kit (APCK)

APCK is a Tauri desktop app that intercepts clipboard content from Epic EHR, detects PHI via ten discerners running across two placements (seven in the Rust application, three in a long-running container), presents a triage view, and writes anonymized plain text to clipboard for pasting into Open Evidence. The conceptual vocabulary of the detection pipeline lives in APCS0; implementation and wire-format details in APCPS.

### Conventions

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

Ten discerners across two placements, each a pure function of `apcsgt_normalized_text`, feeding a single `apcsde_evidence` pool; `apcsnc_combining` (deferred) unifies. See APCS0 for the conceptual vocabulary and APCPS for concrete parameterization and wire format.

**Rust-application discerners (7, in-process):**
- `apcsds_regex` — structural patterns (SSN, phone, email, dates, addresses, zip, labeled identifiers)
- `apcsds_label` — words following Epic labels (Patient:, Attending:, Facility:, etc.)
- `apcsds_surname` — US Census surname dictionary
- `apcsds_firstname` — SSA first-name dictionary
- `apcsds_city` — US cities dictionary
- `apcsds_english` — common-English-word whitelist (suppression evidence)
- `apcsds_medical` — medical-term whitelist (suppression evidence)

**Container discerners (3, via bind-mounted JSON):**
- `apcscs_stanford` — `StanfordAIMI/stanford-deidentifier-base` (HuggingFace transformers, PyTorch CPU)
- `apcscs_spacy` — scispaCy `en_core_sci_md` (POS, dependency parse, biomedical NER)
- `apcscs_stanza` — Stanza English UD pipeline (POS, dependency parse, OntoNotes NER)

The container runs as a long-running process with `--network=none`, `--cap-drop=all`, non-root user, read-only root filesystem. Bind-mounts `$HOME/apcjd/` only. Clipbuddy writes `{N}-in.txt` (normalized text); container writes `{N}.json` atomically (via `{N}.json.tmp` → `rename(2)`) consolidating all three container discerners' findings. No sockets, no HTTP — POSIX file I/O is the wire format.

Combining rules are deferred; the prototype's current anonymization is a stand-in derived from Rust-side findings only.

### Deploy Workflow

1. `tt/apcw-b.Build.sh` — produces `.app` bundle
2. `tt/apcw-D.Deploy.sh` — scp to Ann's machine at `/Users/Shared/apcua/`; emits a `=== Forward to Ann ===` block with manual quit + relaunch instructions
3. Self-update watcher is currently dormant (see APCPS); Ann's ceremony is manual quit + relaunch
