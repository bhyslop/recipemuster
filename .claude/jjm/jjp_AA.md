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
4. Package archive: `vok-release-YYMMDD-HHMM.tar.gz`

Rust owns the knowledge of what to collect and where each file installs. No external manifest files. Kit membership and install paths defined in Rust structs.

### Archive Structure

```
vok-release-260115-1430/
├── bin/
│   ├── vvx-darwin-arm64      # Lean: install/release logic only
│   ├── vvx-darwin-x86_64
│   └── vvx-linux-x86_64
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
    └── vok/                  # VOK slash commands, etc.
        └── commands/
            └── vvc-*.md
```

Kit assets are plain text in the archive, not embedded in binaries. This provides:
- **Lean binaries** — Rust code for install logic only, no duplicated content
- **Single source of truth** — Kit assets exist once in archive
- **Inspectability** — Plain text reviewable before install
- **Simpler kit development** — Edit text files, rebuild archive, done

### Install Process

Run from extracted archive directory:
```bash
./bin/vvx-darwin-arm64 install --target /path/to/repo
```

The executing binary:
1. **Pre-install snapshot** — `git commit -m "[vvx:pre-install] Snapshot before {version}"` (skip if clean)
2. **Copy kit assets** — Read from archive's `kits/` directory, write to `install_path` locations in target
3. **Copy platform binaries** — Copy all sibling binaries from archive's `bin/` to target
4. **Freshen CLAUDE.md** — Replace content between managed section markers
5. **Cleanup obsolete** — Remove files no longer part of current release
6. **Post-install commit** — `git commit -m "[vvx:install:{version}] {kit-list}"`
7. **Diff analysis** — Find previous install, diff pre-install against it, invoke Claude for recovery guidance

Install is platform-agnostic: any platform's binary can perform a full install (all binaries, all kit assets).

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

`vvx install` writes `.claude/vvx-manifest.json`:
```json
{
  "version": "260115-1430",
  "installed": "260115-1823",
  "commit": "abc123def",
  "kits": ["jjk", "buk", "cmk", "vok"]
}
```

## Key Constraints

1. **All Rust** — No YAML, no JSON config for release/install logic. Rust structs own the knowledge.
2. **All or none** — No selective installation. The release is a tested, coherent unit.
3. **Git is the safety net** — We commit before touching anything. User can always revert.
4. **Claude-assisted recovery** — Don't silently lose user modifications. Analyze and report.

## Open Decisions

- Binary placement in target repo: `Tools/vvk/bin/vvx`? (current approach)
- Manifest location: `.claude/vvx-manifest.json`? (proposed)
- Release trigger: Manual tabtarget? CI?

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
