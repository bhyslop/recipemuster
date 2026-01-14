# Heat: VOK Smart Install Release

VOK owns the complete release and installation lifecycle. Release produces an archive containing `vvx` and all kit assets. Installation is all-or-none — the archive represents a coherent, tested release. No configuration, no selectivity.

The install process is git-aware and Claude-assisted: it commits before and after, then analyzes diffs to help users recover any local modifications that were overwritten.

## Paddock

### Architecture

#### Release Process

Runs from source repo. Responsibilities:

1. Run all testbenches (fail release if tests fail)
2. Build `vvx` for target platforms (darwin-arm64, darwin-x86_64, linux-x86_64)
3. Collect kit assets (Rust knows what files belong to each kit)
4. Package archive: `vok-release-YYMMDD-HHMM.tar.gz`

Rust owns the knowledge of what to collect. No external manifest files. Each kit's assets are defined in Rust:

```rust
pub struct KitAsset {
    pub source_path: &'static str,
    pub install_path: &'static str,
    pub content: &'static str,
}

pub const JJK_ASSETS: &[KitAsset] = &[
    KitAsset {
        source_path: "Tools/jjk/jju_utility.sh",
        install_path: "Tools/jjk/jju_utility.sh",
        content: include_str!("../../../jjk/jju_utility.sh"),
    },
    KitAsset {
        source_path: "Tools/jjk/commands/jja-heat-saddle.md",
        install_path: ".claude/commands/jja-heat-saddle.md",
        content: include_str!("../../../jjk/commands/jja-heat-saddle.md"),
    },
    // ...
];
```

#### Archive Structure

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

#### Install Tabtarget

`tt/vok-i.Install.sh` is bash. It:

1. Detects platform
2. Invokes `bin/vvx-{platform} install --target <repo-path>`
3. `vvx install` does all the real work

The archive's `burc.env` is used by the install tabtarget's BUK calls but is not copied to the target repo.

#### Install Process (Rust)

`vvx install --target /path/to/repo`:

1. **Pre-install snapshot**
   ```
   git commit -m "[vvx:pre-install] Snapshot before {version}"
   ```
   (Skip if working tree clean)

2. **Extract assets**
   - Write all embedded kit files to their `install_path` locations
   - Create directories as needed

3. **Freshen CLAUDE.md**
   - Find `<!-- MANAGED:{KIT}:BEGIN -->` ... `<!-- MANAGED:{KIT}:END -->` markers
   - Replace content between markers
   - If no markers, append section with markers
   - Each kit has its CLAUDE.md section template embedded

4. **Cleanup obsolete files**
   - Rust knows what files existed in prior releases
   - Remove files that are no longer part of current release

5. **Post-install commit**
   ```
   git commit -m "[vvx:install:{version}] {kit-list}"
   ```

6. **Diff analysis**
   - Find previous `[vvx:install:*]` commit
   - Diff `[vvx:pre-install]` against previous install
   - If diff exists, invoke Claude Code session:
     ```
     claude --print "Analyze this diff showing local modifications..."
     ```
   - Report what was overwritten, suggest recovery paths

#### CLAUDE.md Freshening

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

#### Version Tracking

`vvx install` writes `.claude/vvx-manifest.json`:
```json
{
  "version": "260115-1430",
  "installed": "260115-1823",
  "commit": "abc123def",
  "kits": ["jjk", "buk", "cmk", "vok"]
}
```

Used by:
- Diff analysis (find previous install commit)
- `vvx --version` to show installed version
- Future upgrade logic

### Key Constraints

1. **All Rust** — No YAML, no JSON config for release/install logic. Rust structs own the knowledge.

2. **All or none** — No selective installation. The release is a tested, coherent unit.

3. **Git is the safety net** — We commit before touching anything. User can always revert.

4. **Claude-assisted recovery** — Don't silently lose user modifications. Analyze and report.

### Open Questions

- Binary placement in target repo: `Tools/vok/bin/vvx`? Somewhere in PATH?
- Manifest location: `.claude/vvx-manifest.json` or `Tools/vok/vvx-manifest.json`?
- How does release process get triggered? Manual tabtarget? CI?
- Build MCM concept model for this process? Or is this all managed by `Tools/vok/README.md` detail?

### References

- VOK README (to be written)
- VVK README (veiled crate patterns)
- BUK README (tabtarget/launcher infrastructure)

## Done

(none yet)

## Remaining

(paces to be identified as work proceeds)

## Steeplechase

---
### 2026-01-13 - Heat Created

**Context**: Emerged from discussion during gallops heat review. Recognized that release/install infrastructure is foundational VOK capability, separate from JJK-specific Rust implementation.

**Key decisions**:
- No YAML/JSON config — all knowledge lives in Rust structs
- All-or-none installation — release is a coherent tested unit
- Archive contains binaries + install tabtarget; kit files embedded in `vvx`
- Git-aware install with Claude-assisted diff recovery for user modifications
- Arcanums eliminated — replaced by static embedded assets + Rust install logic

**Relationship to gallops heat**:
- Gallops completes `vvx jjx_*` operations (JSON manipulation, commits)
- This heat adds `vvx install` and `vvx release` infrastructure
- Gallops arcanum paces may be revised or eliminated once this heat matures
