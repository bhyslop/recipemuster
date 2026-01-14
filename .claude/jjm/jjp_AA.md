# Paddock: vok-fresh-install-release

## Context

VOK owns the complete release and installation lifecycle. Release produces an archive containing `vvx` and all kit assets. Installation is all-or-none — the archive represents a coherent, tested release. No configuration, no selectivity.

The install process is git-aware and Claude-assisted: it commits before and after, then analyzes diffs to help users recover any local modifications that were overwritten.

## Architecture

### Release Process

Runs from source repo. Responsibilities:

1. Run all testbenches (fail release if tests fail)
2. Build `vvx` for target platforms (darwin-arm64, darwin-x86_64, linux-x86_64)
3. Collect kit assets (Rust knows what files belong to each kit)
4. Package archive: `vok-release-YYMMDD-HHMM.tar.gz`

Rust owns the knowledge of what to collect. No external manifest files. Each kit's assets are defined in Rust structs with `include_str!()`.

### Archive Structure

```
vok-release-260115-1430/
├── bin/
│   ├── vvx-darwin-arm64
│   ├── vvx-darwin-x86_64
│   └── vvx-linux-x86_64
├── .buk/
│   └── burc.env           # Used during install, not installed
└── tt/
    └── vok-i.Install.sh   # Install tabtarget
```

Note: Kit files are NOT loose in the archive. They're embedded in `vvx`. The archive is lean: binaries + install tabtarget + minimal BUK config for the install process itself.

### Install Process

`vvx install --target /path/to/repo`:

1. **Pre-install snapshot** — `git commit -m "[vvx:pre-install] Snapshot before {version}"` (skip if clean)
2. **Extract assets** — Write all embedded kit files to their `install_path` locations
3. **Freshen CLAUDE.md** — Replace content between managed section markers
4. **Cleanup obsolete** — Remove files no longer part of current release
5. **Post-install commit** — `git commit -m "[vvx:install:{version}] {kit-list}"`
6. **Diff analysis** — Find previous install, diff pre-install against it, invoke Claude for recovery guidance

### CLAUDE.md Freshening

Section markers:
```markdown
<!-- MANAGED:JJK:BEGIN -->
## Job Jockey Configuration
...content from embedded template...
<!-- MANAGED:JJK:END -->
```

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
