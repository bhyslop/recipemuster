# Paddock: vok-fresh-install-release

## Context

VOK owns the complete release and installation lifecycle. Release produces an archive containing `vvx` and all kit assets. Installation is all-or-none — the archive represents a coherent, tested release. No configuration, no selectivity.

The install process is git-aware and Claude-assisted: it commits before and after, then analyzes diffs to help users recover any local modifications that were overwritten.

## Architecture

### Release Process

Runs from source repo. Responsibilities:

1. Run all testbenches (fail release if tests fail)
2. Build `vvx` for target platforms (darwin-arm64, darwin-x86_64, linux-x86_64)
3. Collect kit assets from source tree (Rust knows what files belong to each kit)
4. Package archive: `vvk-parcel-{hallmark}.tar.gz`

Rust owns the knowledge of what to collect and where each file installs. No external manifest files. Kit membership and install paths defined in Rust structs.

### Archive Structure

```
vvk-parcel-1000/
├── vvi_install.sh            # Thin bash: detect platform, invoke vvx
├── vvbf_brand.json           # Brand identity (hallmark, date, sha)
└── kits/
    ├── buk/                  # Plain text shell scripts
    │   ├── buc_command.sh
    │   ├── bud_dispatch.sh
    │   └── ...
    ├── cmk/                  # Plain text concept model files
    │   ├── MCM-MetaConceptModel.adoc
    │   └── ...
    ├── jjk/                  # Plain text utilities, commands
    │   ├── jju_utility.sh
    │   ├── commands/
    │   │   └── jjc-*.md
    │   └── ...
    ├── vok/                  # VOK slash commands, etc.
    │   └── commands/
    │       └── vvc-*.md
    └── vvk/                  # VVK (distributed kit)
        └── bin/
            ├── vvx-darwin-arm64
            ├── vvx-darwin-x86_64
            └── vvx-linux-x86_64
```

Kit assets are plain text in the archive, not embedded in binaries. This provides:
- **Lean binaries** — Rust code for install logic only, no duplicated content
- **Single source of truth** — Kit assets exist once in archive
- **Inspectability** — Plain text reviewable before install
- **Simpler kit development** — Edit text files, rebuild archive, done

### Install Process

Run from extracted archive directory:
```bash
./vvi_install.sh /path/to/target/.buk/burc.env
```

The thin bash script detects platform and invokes the appropriate `vvx` binary. Rust does all the work:

1. **Validate BURC** — Parse burc.env, extract BURC_TOOLS_DIR, BURC_PROJECT_ROOT
2. **Pre-install snapshot** — `git commit` if working tree dirty (skip if clean)
3. **Copy kit assets** — Read from archive's `kits/` directory, write to `${BURC_TOOLS_DIR}/`
4. **Copy brand file** — Copy `vvbf_brand.json` to `.vvk/vvbf_brand.json`
5. **Freshen CLAUDE.md** — Replace content between managed section markers
6. **Post-install commit** — `git commit -m "VVK install: {hallmark}"`
7. **Diff analysis** — Compare pre-install to previous install, invoke Claude for recovery guidance

Install is platform-agnostic: any platform's binary can install everything (all kits, all binaries).

### CLAUDE.md Freshening

Section markers:
```markdown
<!-- MANAGED:JJK:BEGIN -->
## Job Jockey Configuration
...content from kit's CLAUDE.md template...
<!-- MANAGED:JJK:END -->
```

Each kit provides a CLAUDE.md template in its `kits/{kit}/` directory (e.g., `kits/jjk/CLAUDE.md.template`).

Rules:
- Markers are authoritative — content between them is replaced entirely
- User content outside markers is preserved
- Order of managed sections follows kit installation order
- Missing markers → append at end of file

### Version Tracking

Brand file (`.vvk/vvbf_brand.json`) identifies what's installed:
```json
{
  "vvbh_hallmark": 1000,
  "vvbd_date": "260115-1430",
  "vvbs_sha": "abc123...",
  "vvbc_commit": "9a8f15ef...",
  "vvbk_kits": ["buk", "cmk", "jjk", "vvk"]
}
```

The hallmark registry (`.vvk/vovr_registry.json`) stays in source repo only — never delivered.

## Key Constraints

1. **All Rust** — No YAML, no JSON config for release/install logic. Rust structs own the knowledge.
2. **All or none** — No selective installation. The release is a tested, coherent unit.
3. **Git is the safety net** — We commit before touching anything. User can always revert.
4. **Claude-assisted recovery** — Don't silently lose user modifications. Analyze and report.

## Open Decisions

- Release trigger: Manual tabtarget? CI?

## Closed Decisions

- Binary placement: `Tools/vvk/bin/vvx-{platform}` (consistent with kit structure)
- Brand file: `.vvk/vvbf_brand.json` (follows `.buk/` pattern for kit state)
- Registry: Source repo only (`.vvk/vovr_registry.json`), never delivered
- Install invocation: `./vvi_install.sh /path/to/burc.env` (thin bash, Rust does work)

## References

- Tools/vok/README.md — VOK architecture and concepts
- Tools/vvk/README.md — VVK (distributed kit)
- Tools/buk/README.md — Tabtarget/launcher infrastructure
- Legacy heat: .claude/jjm/current/jjh_b260113-vok-smart-install-release.md

## Steeplechase

### 2026-01-13 - Heat Created

**Context**: Emerged from discussion during gallops heat review. Recognized that release/install infrastructure is foundational VOK capability, separate from JJK-specific Rust implementation.

**Key decisions**:
- No YAML/JSON config — all knowledge lives in Rust structs
- All-or-none installation — release is a coherent tested unit
- Archive contains binaries + install tabtarget; kit files embedded in `vvx`
- Git-aware install with Claude-assisted diff recovery for user modifications
- Arcanums eliminated — replaced by static embedded assets + Rust install logic

### 2026-01-14 - Gallops Migration

Migrated from legacy heat file to Gallops system. Paddock populated from legacy Architecture section. Paces slated for implementation work.

### 2026-01-15 - Archive-Based Asset Model

**Context**: Resolved root architectural tension — whether distribution is self-contained binaries with embedded kit assets, or archive containing lean binaries plus plain text kit assets.

**Decision**: Archive with plain text kit assets.

**Rationale**:
- Install is inherently multi-platform (installs all platform binaries to target repo)
- Archive already bundles platform binaries, so it's the natural distribution unit
- Embedding kit assets in each binary triplicates content unnecessarily
- Plain text in archive is inspectable, single source of truth
- Simpler development cycle (edit text, rebuild archive, no Rust recompile for kit changes)
- Bootstrap problem solved: Rust install logic doesn't depend on BUK (which is being installed)

**Supersedes**: Earlier assumption of `include_str!()` embedded assets in binaries.

### 2026-01-17 - Install Architecture Finalized

**Context**: Reconciled VOS spec with paddock, resolved remaining design questions.

**Key decisions**:
- Parcel structure: `vvi_install.sh` + `vvbf_brand.json` at root, binaries in `kits/vvk/bin/`
- Install invocation: `./vvi_install.sh /path/to/burc.env` — thin bash detects platform, Rust does work
- Brand file location: `.vvk/vvbf_brand.json` (follows `.buk/` pattern for kit state)
- Brand file expanded: now includes `vvbc_commit` (source SHA) and `vvbk_kits` (kit list)
- Registry stays in source: never delivered, tracks all releases for hallmark allocation
- CLAUDE.md freshening: `UNINSTALLED` marker preserves user section ordering across reinstall
- Programmatic: Rust handles all install logic, no Claude assistance for marker manipulation
