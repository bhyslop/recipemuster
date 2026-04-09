# Heat Trophy: rbk-theurge-rust-test-infrastructure

**Firemark:** ₣A1
**Created:** 260401
**Retired:** 260409
**Status:** retired

## Paddock

## Character
Architectural migration with careful sequencing. Early paces are infrastructure and minting; middle paces are mechanical porting; late paces require judgment about bash test equivalence. The "two binaries" design is settled but internal structure will emerge during implementation.

## Design Decision Record

**Problem**: Bash tests (BUK framework, ~75 cases) and Python sorties (ifrit adjutant, 11 attack modules) are two separate worlds that cannot coordinate. Neither can simultaneously observe from outside the bottle while attacking from inside. The testing language split adds maintenance burden without adding capability.

**Decision**: Replace both with Rust infrastructure:
- **Theurge** — orchestrator binary (`Tools/rbk/rbtd/`), runs on host, owns charge/exec/monitor/report lifecycle. Code prefix: `rbtd`.
- **Ifrit** — attack binary, source at `Tools/rbk/rbid/`, built inside the ifrit vessel Dockerfile (no cross-compilation). The vessel image IS both the build artifact and the runtime bottle. Code prefix: `rbid`.
- Separate crates — completely different dependency profiles and build targets.
- **Not `rbtr`** — that AsciiDoc attribute category ("RB Term Role") is already populated in RBS0. Theurge uses `rbtd`, ifrit uses `rbid`.

**Key design choices**:
- Detached from `cargo test` model — theurge is an application that produces test verdicts, not a unit test suite. All tiers (fast, service, crucible) are selector arguments to the one theurge binary, selected via tabtarget imprints.
- Colophon manifest protocol — theurge always invokes bottle operations via tabtargets (never reimplements). Compiled-in colophon strings verified against live zipper registry at launch. The colophon set grows as paces add dependencies (charge/quench initially, then writ/fiat/bark, then foundry colophons for four-mode).
- New container exec tabtargets: writ (sentry), fiat (pentacle), bark (bottle) — non-interactive exec-and-return, distinct from interactive hail/rack/scry.
- Ifrit discovers its network environment from container state (env vars, resolv.conf, probing), no config arguments beyond attack selector.
- Reporting follows bash test precedent: colored pass/fail to terminal, verbose trace written to per-case temp dir files (never rerun for detail). No JSON.
- BUK test framework files (`butX`) retained with a minimal BUK self-test; only RBK consumers retired.

**Ifrit build model — single Dockerfile, single image:**
- One Dockerfile at `rbev-vessels/rbev-bottle-ifrit/Dockerfile`. Single stage: `rust:bookworm-slim` base, network tools installed, `Cargo.toml`+`Cargo.lock` copied first (layer cached), then source copied and built. Rust toolchain stays in image — it's a test vessel, size doesn't matter, iteration speed does.
- The resulting image IS the runtime bottle. When the tadmor crucible charges, the ifrit binary is already inside. No extraction, no docker cp, no two-image dance.
- Custom `rbi-iK` kludge tabtarget writes to a constant hallmark for daily iteration.
- Steady state: cloud conjure builds the same Dockerfile.
- No Docker volumes, no persistent cargo cache. Layer caching handles dependency compilation; source-only changes recompile just the ifrit crate.
- Adding a new crate dependency requires a tethered kludge (same cost as adding an apt package today). `Cargo.lock` pins versions.

**RBTW workbench — orthogonal Rust pipeline:**
- Theurge build/test is fully independent of the VOW pipeline (vvk/jjk/cmk kits destined for parcel delivery). VOW will be peeled away; theurge stays with rbk permanently.
- `rbtw` minted as new sibling under `rbt` (alongside `rbtc` colophons, `rbtd` theurge, `rbtr` roles). Workbench at `Tools/rbk/rbtd/rbtw_workbench.sh`.
- Tabtargets: `rbtd-b` (build), `rbtd-t` (unit test). Future: `rbtd-r` (run orchestration suite with tier imprints).
- Theurge unit tests (`cargo test` on rbtd) verify engine internals — distinct from theurge-the-application which produces crucible verdicts. The paddock's "detached from cargo test" describes the application's purpose, not a prohibition on testing engine code.
- `rbrt` was considered but rejected: `rbr` branch is semantically "Regime" (rbra, rbrm, rbrn, rbro, rbrp, rbrr, rbrs, rbrv) — a Rust test prefix would be an interloper.
- Ifrit has no macOS unit tests — it's a Linux container binary. Its testing comes from theurge orchestrating the charged bottle.

**Explicitly deferred**: new test design, tcpdump/continuous monitoring, cross-nameplate ifrit deployment.

## Provenance
Emerged from conversation about the fundamental flaw in split bash/python test architecture. The name "theurge" chosen for `rbt` prefix compatibility and the mythology (one who compels spirits through ceremony — commanding the ifrit).

## Paces

### add-buz-group-to-buk-zipper (₢A1AAf) [complete]

**[260403-0941] complete**

## Character
Small, precise infrastructure work — add a no-op API to BUK that declares colophon group categories.

## Docket
Add `buz_group` function to BUK's zipper infrastructure. Initially a no-op that accepts three arguments: constant name, group prefix, description string.

Then add `buz_group` declarations to `rbz_zipper.sh` for the current (pre-rename) colophon categories. These document today's actor-organized structure; the colophon rename pace (₢A1AAb) will update them to domain-organized categories.

### API
```
buz_group <CONSTANT_NAME> <prefix> <description>
```

### Current pre-rename group declarations (what to write in this pace)
```
buz_group RBZ__GROUP_PAYOR      "rbw-P"   "Payor depot and governor operations"
buz_group RBZ__GROUP_GUIDE      "rbw-g"   "Human-directed procedures"
buz_group RBZ__GROUP_MARSHAL    "rbw-M"   "Lifecycle marshal"
buz_group RBZ__GROUP_GOVERNOR   "rbw-G"   "Governor service account operations"
buz_group RBZ__GROUP_DIRECTOR   "rbw-D"   "Director foundry operations"
buz_group RBZ__GROUP_LEDGER     "rbw-L"   "Ledger local operations"
buz_group RBZ__GROUP_RETRIEVER  "rbw-R"   "Retriever operations"
buz_group RBZ__GROUP_REGIME     "rbw-r"   "Regime config files"
buz_group RBZ__GROUP_NAMEPLATE  "rbw-n"   "Cross-nameplate operations"
buz_group RBZ__GROUP_CRUCIBLE   "rbw-c"   "Crucible container runtime"
buz_group RBZ__GROUP_THEURGE    "rbw-T"   "Theurge test infrastructure"
buz_group RBZ__GROUP_IFRIT      "rbw-I"   "Ifrit attack binary"
buz_group RBZ__GROUP_QUALIFY    "rbw-Q"   "Qualification operations"
```

### Post-rename target state (for reference — applied by ₢A1AAb)
```
buz_group RBZ__GROUP_ACCOUNTS   "rbw-a"   "Google Cloud service accounts"
buz_group RBZ__GROUP_CRUCIBLE   "rbw-c"   "Crucible container runtime"
buz_group RBZ__GROUP_DEPOT      "rbw-d"   "GCP project infrastructure and supply chain"
buz_group RBZ__GROUP_GUIDE      "rbw-g"   "Human-directed procedures"
buz_group RBZ__GROUP_HALLMARK   "rbw-h"   "Hallmark registry artifact lifecycle"
buz_group RBZ__GROUP_IMAGE      "rbw-i"   "Container image operations"
buz_group RBZ__GROUP_IFRIT      "rbw-I"   "Ifrit attack binary"
buz_group RBZ__GROUP_MARSHAL    "rbw-M"   "Lifecycle marshal"
buz_group RBZ__GROUP_NAMEPLATE  "rbw-n"   "Cross-nameplate operations"
buz_group RBZ__GROUP_REGIME     "rbw-r"   "Regime config files"
buz_group RBZ__GROUP_THEURGE    "rbw-t"   "Theurge test infrastructure"
```

### Work items
- Add `buz_group` no-op function to BUK
- Add pre-rename group declarations to `rbz_zipper.sh`
- Double-underscore convention separates group constants from enrollment constants

### Verification
- `tt/buw-st.BukSelfTest.sh` passes (9 cases)
- `tt/rbtd-s.TestSuite.fast.sh` passes (75 cases)
- Existing dispatch routing unaffected

### Acceptance
- `buz_group` callable without error
- All current colophon groups declared in rbz_zipper.sh
- No behavioral change to enrollment or dispatch

**[260403-0930] rough**

## Character
Small, precise infrastructure work — add a no-op API to BUK that declares colophon group categories.

## Docket
Add `buz_group` function to BUK's zipper infrastructure. Initially a no-op that accepts three arguments: constant name, group prefix, description string.

Then add `buz_group` declarations to `rbz_zipper.sh` for the current (pre-rename) colophon categories. These document today's actor-organized structure; the colophon rename pace (₢A1AAb) will update them to domain-organized categories.

### API
```
buz_group <CONSTANT_NAME> <prefix> <description>
```

### Current pre-rename group declarations (what to write in this pace)
```
buz_group RBZ__GROUP_PAYOR      "rbw-P"   "Payor depot and governor operations"
buz_group RBZ__GROUP_GUIDE      "rbw-g"   "Human-directed procedures"
buz_group RBZ__GROUP_MARSHAL    "rbw-M"   "Lifecycle marshal"
buz_group RBZ__GROUP_GOVERNOR   "rbw-G"   "Governor service account operations"
buz_group RBZ__GROUP_DIRECTOR   "rbw-D"   "Director foundry operations"
buz_group RBZ__GROUP_LEDGER     "rbw-L"   "Ledger local operations"
buz_group RBZ__GROUP_RETRIEVER  "rbw-R"   "Retriever operations"
buz_group RBZ__GROUP_REGIME     "rbw-r"   "Regime config files"
buz_group RBZ__GROUP_NAMEPLATE  "rbw-n"   "Cross-nameplate operations"
buz_group RBZ__GROUP_CRUCIBLE   "rbw-c"   "Crucible container runtime"
buz_group RBZ__GROUP_THEURGE    "rbw-T"   "Theurge test infrastructure"
buz_group RBZ__GROUP_IFRIT      "rbw-I"   "Ifrit attack binary"
buz_group RBZ__GROUP_QUALIFY    "rbw-Q"   "Qualification operations"
```

### Post-rename target state (for reference — applied by ₢A1AAb)
```
buz_group RBZ__GROUP_ACCOUNTS   "rbw-a"   "Google Cloud service accounts"
buz_group RBZ__GROUP_CRUCIBLE   "rbw-c"   "Crucible container runtime"
buz_group RBZ__GROUP_DEPOT      "rbw-d"   "GCP project infrastructure and supply chain"
buz_group RBZ__GROUP_GUIDE      "rbw-g"   "Human-directed procedures"
buz_group RBZ__GROUP_HALLMARK   "rbw-h"   "Hallmark registry artifact lifecycle"
buz_group RBZ__GROUP_IMAGE      "rbw-i"   "Container image operations"
buz_group RBZ__GROUP_IFRIT      "rbw-I"   "Ifrit attack binary"
buz_group RBZ__GROUP_MARSHAL    "rbw-M"   "Lifecycle marshal"
buz_group RBZ__GROUP_NAMEPLATE  "rbw-n"   "Cross-nameplate operations"
buz_group RBZ__GROUP_REGIME     "rbw-r"   "Regime config files"
buz_group RBZ__GROUP_THEURGE    "rbw-t"   "Theurge test infrastructure"
```

### Work items
- Add `buz_group` no-op function to BUK
- Add pre-rename group declarations to `rbz_zipper.sh`
- Double-underscore convention separates group constants from enrollment constants

### Verification
- `tt/buw-st.BukSelfTest.sh` passes (9 cases)
- `tt/rbtd-s.TestSuite.fast.sh` passes (75 cases)
- Existing dispatch routing unaffected

### Acceptance
- `buz_group` callable without error
- All current colophon groups declared in rbz_zipper.sh
- No behavioral change to enrollment or dispatch

**[260403-0912] rough**

## Character
Small, precise infrastructure work — add a no-op API to BUK that declares colophon group categories.

## Docket
Add `buz_group` function to BUK's zipper infrastructure. Initially a no-op that accepts three arguments: constant name, group prefix, description string.

Then add `buz_group` declarations to `rbz_zipper.sh` for the current (pre-rename) colophon categories, documenting what exists today. The colophon rename pace will update these to the new domain-organized categories.

### API
```
buz_group <CONSTANT_NAME> <prefix> <description>
```

### Post-rename group declarations (target state after ₢A1AAb)
```
buz_group RBZ__GROUP_ACCOUNTS   "rbw-a"   "Google Cloud service accounts"
buz_group RBZ__GROUP_CRUCIBLE   "rbw-c"   "Crucible container runtime"
buz_group RBZ__GROUP_DEPOT      "rbw-d"   "GCP project infrastructure and supply chain"
buz_group RBZ__GROUP_GUIDE      "rbw-g"   "Human-directed procedures"
buz_group RBZ__GROUP_HALLMARK   "rbw-h"   "Hallmark registry artifact lifecycle"
buz_group RBZ__GROUP_IMAGE      "rbw-i"   "Container image operations"
buz_group RBZ__GROUP_IFRIT      "rbw-I"   "Ifrit attack binary"
buz_group RBZ__GROUP_MARSHAL    "rbw-M"   "Lifecycle marshal"
buz_group RBZ__GROUP_NAMEPLATE  "rbw-n"   "Cross-nameplate operations"
buz_group RBZ__GROUP_REGIME     "rbw-r"   "Regime config files"
buz_group RBZ__GROUP_THEURGE    "rbw-t"   "Theurge test infrastructure"
```

### Work items
- Add `buz_group` no-op function to BUK
- Add group declarations to `rbz_zipper.sh` for current pre-rename categories
- Double-underscore convention separates group constants from enrollment constants

### Verification
- `tt/buw-st.BukSelfTest.sh` passes (9 cases)
- `tt/rbtd-s.TestSuite.fast.sh` passes (75 cases)
- Existing dispatch routing unaffected

### Acceptance
- `buz_group` callable without error
- All current colophon groups declared in rbz_zipper.sh
- No behavioral change to enrollment or dispatch

**[260403-0906] rough**

## Character
Small, precise infrastructure work — add a no-op API to BUK that declares colophon group categories.

## Docket
Add `buz_group` function to BUK's zipper infrastructure. Initially a no-op that accepts three arguments: constant name, group prefix, description string.

Then add `buz_group` declarations to `rbz_zipper.sh` for the current (pre-rename) colophon categories, documenting what exists today. The colophon rename pace will update these to the new domain-organized categories.

### API
```
buz_group <CONSTANT_NAME> <prefix> <description>
```

### Post-rename group declarations (target state after ₢A1AAb)
```
buz_group RBZ__GROUP_ACCOUNTS   "rbw-a"   "Google Cloud service accounts"
buz_group RBZ__GROUP_CRUCIBLE   "rbw-c"   "Crucible container runtime"
buz_group RBZ__GROUP_DEPOT      "rbw-d"   "GCP project infrastructure"
buz_group RBZ__GROUP_GUIDE      "rbw-g"   "Human-directed procedures"
buz_group RBZ__GROUP_HALLMARK   "rbw-h"   "Hallmark registry artifact lifecycle"
buz_group RBZ__GROUP_IMAGE      "rbw-i"   "Container image operations"
buz_group RBZ__GROUP_IFRIT      "rbw-I"   "Ifrit attack binary"
buz_group RBZ__GROUP_MARSHAL    "rbw-M"   "Lifecycle marshal"
buz_group RBZ__GROUP_NAMEPLATE  "rbw-n"   "Cross-nameplate operations"
buz_group RBZ__GROUP_REGIME     "rbw-r"   "Regime config files"
buz_group RBZ__GROUP_THEURGE    "rbw-t"   "Theurge test infrastructure"
buz_group RBZ__GROUP_VESSEL     "rbw-v"   "Vessel build operations"
```

Note: Reliquary (`rbw-R`) group TBD pending design decision.

### Work items
- Add `buz_group` no-op function to BUK
- Add group declarations to `rbz_zipper.sh` for current pre-rename categories
- Double-underscore convention separates group constants from enrollment constants

### Verification
- `tt/buw-st.BukSelfTest.sh` passes (9 cases)
- `tt/rbtd-s.TestSuite.fast.sh` passes (75 cases)
- Existing dispatch routing unaffected

### Acceptance
- `buz_group` callable without error
- All current colophon groups declared in rbz_zipper.sh
- No behavioral change to enrollment or dispatch

**[260403-0859] rough**

## Character
Small, precise infrastructure work — add a no-op API to BUK that declares colophon group categories.

## Docket
Add `buz_group` function to BUK's zipper infrastructure (`bud_dispatch.sh` or appropriate BUK module). Initially a no-op that accepts three arguments: constant name, group prefix, description string.

Then add `buz_group` declarations to `rbz_zipper.sh` for the current (pre-rename) colophon categories, documenting what exists today. The colophon rename pace will update these to the new domain-organized categories.

### API
```
buz_group <CONSTANT_NAME> <prefix> <description>
```
Example: `buz_group RBZ__GROUP_FOUNDRY "rbw-D" "Director/foundry operations"`

### Work items
- Add `buz_group` no-op function to BUK
- Add group declarations to `rbz_zipper.sh` for current categories
- Consider: double-underscore convention for group constants vs enrollment constants

### Verification
- `tt/buw-st.BukSelfTest.sh` passes (9 cases)
- `tt/rbtd-s.TestSuite.fast.sh` passes (75 cases)
- Existing dispatch routing unaffected

### Acceptance
- `buz_group` callable without error
- All current colophon groups declared in rbz_zipper.sh
- No behavioral change to enrollment or dispatch

### rename-consecration-to-hallmark (₢A1AAe) [complete]

**[260403-1004] complete**

## Character
Large mechanical rename with judgment calls at boundaries. Every instance needs inspection — quoin references, display strings, regex patterns, variable names, and comments all have different replacement contexts.

## Docket
Rename the domain concept "Consecration" to "Hallmark" across the entire system. Consecration collides with Crucible (both start with C, both are ceremony metaphors). Hallmark means "a distinguishing mark of quality and origin stamped on an artifact" — exactly what the dual-timestamp version identifier is.

### Scope
- RBS0 quoin definitions: `rbtga_consecration`, `rbst_consecration`, and all references
- AsciiDoc attribute references throughout `vov_veiled/` specs
- Bash variable names, function names, CLI output strings, comments, error messages
- Regime env files if any reference the term
- Theurge Rust code (constants, string literals)
- Consumer-facing docs (README, CLAUDE.consumer.md)
- `buz_group` descriptions in `rbz_zipper.sh` if any reference consecration

### Exclusions
- Colophon strings — those change in the separate tectonic-colophon-reorganization pace
- Git history — no rewriting

### Approach
- Single-axis change: rename vocabulary while colophons still use old routing names
- Mechanical find-replace with per-instance review
- Quoin anchors and attributes must stay coordinated (three-part linked term)
- **Build after Rust changes**: run `tt/rbtd-b.Build.sh` after modifying theurge code, before running tests

### Verification
- `tt/rbtd-b.Build.sh` succeeds (theurge build)
- `tt/rbtd-t.Test.sh` passes (theurge unit tests)
- `tt/rbtd-s.TestSuite.fast.sh` passes (75 cases)
- No remaining references to "consecration" (case-insensitive grep, excluding git history)

### Acceptance
- All spec quoins updated with hallmark terminology
- All code references updated
- Qualify-fast clean

**[260403-0935] rough**

## Character
Large mechanical rename with judgment calls at boundaries. Every instance needs inspection — quoin references, display strings, regex patterns, variable names, and comments all have different replacement contexts.

## Docket
Rename the domain concept "Consecration" to "Hallmark" across the entire system. Consecration collides with Crucible (both start with C, both are ceremony metaphors). Hallmark means "a distinguishing mark of quality and origin stamped on an artifact" — exactly what the dual-timestamp version identifier is.

### Scope
- RBS0 quoin definitions: `rbtga_consecration`, `rbst_consecration`, and all references
- AsciiDoc attribute references throughout `vov_veiled/` specs
- Bash variable names, function names, CLI output strings, comments, error messages
- Regime env files if any reference the term
- Theurge Rust code (constants, string literals)
- Consumer-facing docs (README, CLAUDE.consumer.md)
- `buz_group` descriptions in `rbz_zipper.sh` if any reference consecration

### Exclusions
- Colophon strings — those change in the separate tectonic-colophon-reorganization pace
- Git history — no rewriting

### Approach
- Single-axis change: rename vocabulary while colophons still use old routing names
- Mechanical find-replace with per-instance review
- Quoin anchors and attributes must stay coordinated (three-part linked term)
- **Build after Rust changes**: run `tt/rbtd-b.Build.sh` after modifying theurge code, before running tests

### Verification
- `tt/rbtd-b.Build.sh` succeeds (theurge build)
- `tt/rbtd-t.Test.sh` passes (theurge unit tests)
- `tt/rbtd-s.TestSuite.fast.sh` passes (75 cases)
- No remaining references to "consecration" (case-insensitive grep, excluding git history)

### Acceptance
- All spec quoins updated with hallmark terminology
- All code references updated
- Qualify-fast clean

**[260403-0930] rough**

## Character
Large mechanical rename with judgment calls at boundaries. Every instance needs inspection — quoin references, display strings, regex patterns, variable names, and comments all have different replacement contexts.

## Docket
Rename the domain concept "Consecration" to "Hallmark" across the entire system. Consecration collides with Crucible (both start with C, both are ceremony metaphors). Hallmark means "a distinguishing mark of quality and origin stamped on an artifact" — exactly what the dual-timestamp version identifier is.

### Scope
- RBS0 quoin definitions: `rbtga_consecration`, `rbst_consecration`, and all references
- AsciiDoc attribute references throughout `vov_veiled/` specs
- Bash variable names, function names, CLI output strings, comments, error messages
- Regime env files if any reference the term
- Theurge Rust code (constants, string literals)
- Consumer-facing docs (README, CLAUDE.consumer.md)
- `buz_group` descriptions in `rbz_zipper.sh` if any reference consecration

### Exclusions
- Colophon strings — those change in the separate tectonic-colophon-reorganization pace
- Git history — no rewriting

### Approach
- Single-axis change: rename vocabulary while colophons still use old routing names
- Mechanical find-replace with per-instance review
- Quoin anchors and attributes must stay coordinated (three-part linked term)

### Verification
- `tt/rbtd-t.Test.sh` passes (theurge unit tests)
- `tt/rbtd-s.TestSuite.fast.sh` passes (75 cases)
- No remaining references to "consecration" (case-insensitive grep, excluding git history)

### Acceptance
- All spec quoins updated with hallmark terminology
- All code references updated
- Qualify-fast clean

**[260403-0850] rough**

## Character
Large mechanical rename with judgment calls at boundaries. Every instance needs inspection — quoin references, display strings, regex patterns, variable names, and comments all have different replacement contexts.

## Docket
Rename the domain concept "Consecration" to "Hallmark" across the entire system. Consecration collides with Crucible (both start with C, both are ceremony metaphors). Hallmark means "a distinguishing mark of quality and origin stamped on an artifact" — exactly what the dual-timestamp version identifier is.

### Scope
- RBS0 quoin definitions: `rbtga_consecration`, `rbst_consecration`, and all references
- AsciiDoc attribute references throughout `vov_veiled/` specs
- Bash variable names, function names, CLI output strings, comments, error messages
- Regime env files if any reference the term
- Theurge Rust code (constants, string literals)
- Consumer-facing docs (README, CLAUDE.consumer.md)

### Exclusions
- Colophon strings — those change in the separate tectonic-colophon-reorganization pace
- Git history — no rewriting

### Approach
- Single-axis change: rename vocabulary while colophons still use old routing names
- Mechanical find-replace with per-instance review
- Quoin anchors and attributes must stay coordinated (three-part linked term)

### Verification
- `tt/rbtd-t.Test.sh` passes (theurge unit tests)
- `tt/rbtd-s.TestSuite.fast.sh` passes (75 cases)
- No remaining references to "consecration" (case-insensitive grep, excluding git history)

### Acceptance
- All spec quoins updated with hallmark terminology
- All code references updated
- Qualify-fast clean

### add-vessel-quoin-to-rbs0 (₢A1AAd) [complete]

**[260403-1013] complete**

## Character
Small, precise spec work — add a missing formal definition.

## Docket
Vessel is used throughout RBS0 but has no `[[rbtga_vessel]]` quoin definition. Add one in the Ark Definitions section (near line 1577), capturing: the container image specification in the regime that defines what to build, with a mode (conjure/bind/graft). Reference its relationship to ark (build output) and hallmark (version stamp).

### Verification
- Quoin has all three parts: attribute reference in mapping section, anchor at definition site, definition text
- References ark and hallmark correctly
- No broken cross-references in RBS0

### Acceptance
- `[[rbtga_vessel]]` quoin exists with complete linked term structure

**[260403-1009] rough**

## Character
Small, precise spec work — add a missing formal definition.

## Docket
Vessel is used throughout RBS0 but has no `[[rbtga_vessel]]` quoin definition. Add one in the Ark Definitions section (near line 1577), capturing: the container image specification in the regime that defines what to build, with a mode (conjure/bind/graft). Reference its relationship to ark (build output) and hallmark (version stamp).

### Verification
- Quoin has all three parts: attribute reference in mapping section, anchor at definition site, definition text
- References ark and hallmark correctly
- No broken cross-references in RBS0

### Acceptance
- `[[rbtga_vessel]]` quoin exists with complete linked term structure

**[260403-0930] rough**

## Character
Small, precise spec work — add a missing formal definition.

## Docket
Vessel is used throughout RBS0 but has no `[[rbtga_vessel]]` quoin definition. Add one in the Ark Definitions section (near line 1577), capturing: the container image specification in the regime that defines what to build, with a mode (conjure/bind/graft). Reference its relationship to ark (build output) and consecration (version stamp).

Note: this pace uses current vocabulary ("consecration"). The subsequent pace ₢A1AAe renames consecration → hallmark system-wide.

### Verification
- Quoin has all three parts: attribute reference in mapping section, anchor at definition site, definition text
- References ark and consecration correctly
- No broken cross-references in RBS0

### Acceptance
- `[[rbtga_vessel]]` quoin exists with complete linked term structure

**[260403-0830] rough**

## Character
Small, precise spec work — add a missing formal definition.

## Docket
Vessel is used throughout RBS0 but has no `[[rbtga_vessel]]` quoin definition. Add one in the Ark Definitions section (near line 1577), capturing: the container image specification in the regime that defines what to build, with a mode (conjure/bind/graft). Reference its relationship to ark (build output) and consecration (version stamp).

### reclaim-rbi-prefix (₢A1AAL) [complete]

**[260401-1319] complete**

## Character
Housekeeping — confirm unused, delete, document.

## Docket
Confirm `rbi_Image.sh` is fully unused (no references from other scripts, no tabtargets, no sourcing). Delete it to reclaim the `rbi` prefix for ifrit directory (`rbid`). Update CLAUDE.md prefix mappings to remove the RBI entry. This must complete before the mint pace since ifrit directory placement depends on `rbi` being available.

**[260401-1237] rough**

## Character
Housekeeping — confirm unused, delete, document.

## Docket
Confirm `rbi_Image.sh` is fully unused (no references from other scripts, no tabtargets, no sourcing). Delete it to reclaim the `rbi` prefix for ifrit directory (`rbid`). Update CLAUDE.md prefix mappings to remove the RBI entry. This must complete before the mint pace since ifrit directory placement depends on `rbi` being available.

### mint-rbtd-rbid-prefix-and-workspace (₢A1AAA) [complete]

**[260401-1322] complete**

## Character
Minting discipline — careful, mechanical.

## Docket
Mint the `rbtd` (theurge) and `rbid` (ifrit) prefix trees. Two separate crates in distinct directories:
- `Tools/rbk/rbtd/` — Theurge directory. Code prefix `rbtd_*`, variables `RBTD_*`.
- `Tools/rbk/rbid/` — Ifrit directory. Code prefix `rbid_*`, variables `RBID_*`.

**Not `rbtr`** — that AsciiDoc attribute prefix ("RB Term Role") is populated in RBS0.

These are fully distinct Rust codebases, NOT under Tools/vok. Each gets its own `Cargo.toml`, `src/main.rs`. Verify theurge builds on host with cargo. Ifrit builds inside its Dockerfile (no host cargo build needed for ifrit).

No functionality yet — just skeletons that compile.

**[260401-1258] rough**

## Character
Minting discipline — careful, mechanical.

## Docket
Mint the `rbtd` (theurge) and `rbid` (ifrit) prefix trees. Two separate crates in distinct directories:
- `Tools/rbk/rbtd/` — Theurge directory. Code prefix `rbtd_*`, variables `RBTD_*`.
- `Tools/rbk/rbid/` — Ifrit directory. Code prefix `rbid_*`, variables `RBID_*`.

**Not `rbtr`** — that AsciiDoc attribute prefix ("RB Term Role") is populated in RBS0.

These are fully distinct Rust codebases, NOT under Tools/vok. Each gets its own `Cargo.toml`, `src/main.rs`. Verify theurge builds on host with cargo. Ifrit builds inside its Dockerfile (no host cargo build needed for ifrit).

No functionality yet — just skeletons that compile.

**[260401-1258] rough**

## Character
Minting discipline — careful, mechanical.

## Docket
Mint the `rbtd` (theurge) and `rbid` (ifrit) prefix trees. Two separate crates in distinct directories:
- `Tools/rbk/rbtd/` — Theurge directory. Code prefix `rbtd_*`, variables `RBTD_*`.
- `Tools/rbk/rbid/` — Ifrit directory. Code prefix `rbid_*`, variables `RBID_*`.

**Not `rbtr`** — that AsciiDoc attribute prefix ("RB Term Role") is populated in RBS0.

These are fully distinct Rust codebases, NOT under Tools/vok. Each gets its own `Cargo.toml`, `src/main.rs`. Verify theurge builds on host with cargo. Ifrit builds inside its Dockerfile (no host cargo build needed for ifrit).

No functionality yet — just skeletons that compile.

**[260401-1238] rough**

## Character
Minting discipline — careful, mechanical.

## Docket
Mint the `rbtr` prefix tree for theurge-rust test infrastructure. Two separate crates in distinct directories:
- `Tools/rbk/rbtd/` — Theurge directory (orchestrator binary, runs on host macOS)
- `Tools/rbk/rbid/` — Ifrit directory (attack binary, runs inside Linux containers)

These are fully distinct Rust codebases, NOT under Tools/vok. Each gets its own `Cargo.toml`, `src/main.rs`. Establish workspace structure. Verify theurge builds with existing cargo infrastructure. Ifrit build target is a separate discussion (see cross-compilation note in paddock).

No functionality yet — just skeletons that compile.

**[260401-1201] rough**

## Character
Minting discipline — careful, mechanical.

## Docket
Mint the `rbtr` prefix tree. Create Rust workspace entries for theurge and ifrit binaries under Tools/vok (or a new crate location — decide during pace). Establish Cargo.toml structure, empty main.rs for both binaries. Verify builds with `tt/vow-b.Build.sh`. No functionality yet — just the skeleton that compiles.

Decide: do theurge and ifrit live as separate crates in the workspace, or separate binaries in one crate? Leaning separate crates since they have completely different dependency profiles (theurge needs process/Docker interaction, ifrit needs raw network/socket capabilities).

### colophon-manifest-protocol (₢A1AAK) [complete]

**[260401-1337] complete**

## Character
Load-bearing interface design — this is the trust boundary between bash zipper world and Rust binary world. Get it right once.

## Docket
Implement the colophon manifest protocol that lets theurge verify its compiled-in colophon strings match the live zipper registry.

**Bash side (rbz zipper):**
- Add kindle constant `readonly ZRBZ_COLOPHON_MANIFEST="${z_buz_colophon_roll[*]}"` to `zrbz_kindle()`, before the `KINDLED=1` sentinel
- Not exported — stays internal, passed explicitly at dispatch boundary

**Rust side (theurge):**
- Theurge accepts the manifest string as an argument
- Compiled-in constant array of colophon strings theurge depends on (initially: charge `rbw-cC`, quench `rbw-cQ`; grows as later paces add writ/fiat/bark, then foundry colophons for four-mode)
- On startup: for each compiled-in colophon, confirm it appears in the manifest string. Missing → fatal naming the specific string. Manifest absent entirely → fatal with "theurge must be launched via tabtarget"
- The compiled-in set is a subset of the manifest — theurge doesn't care about colophons it doesn't use

**Dispatch glue:**
- The theurge launcher passes `ZRBZ_COLOPHON_MANIFEST` as argument to the binary before any other args

**[260401-1315] rough**

## Character
Load-bearing interface design — this is the trust boundary between bash zipper world and Rust binary world. Get it right once.

## Docket
Implement the colophon manifest protocol that lets theurge verify its compiled-in colophon strings match the live zipper registry.

**Bash side (rbz zipper):**
- Add kindle constant `readonly ZRBZ_COLOPHON_MANIFEST="${z_buz_colophon_roll[*]}"` to `zrbz_kindle()`, before the `KINDLED=1` sentinel
- Not exported — stays internal, passed explicitly at dispatch boundary

**Rust side (theurge):**
- Theurge accepts the manifest string as an argument
- Compiled-in constant array of colophon strings theurge depends on (initially: charge `rbw-cC`, quench `rbw-cQ`; grows as later paces add writ/fiat/bark, then foundry colophons for four-mode)
- On startup: for each compiled-in colophon, confirm it appears in the manifest string. Missing → fatal naming the specific string. Manifest absent entirely → fatal with "theurge must be launched via tabtarget"
- The compiled-in set is a subset of the manifest — theurge doesn't care about colophons it doesn't use

**Dispatch glue:**
- The theurge launcher passes `ZRBZ_COLOPHON_MANIFEST` as argument to the binary before any other args

**[260401-1238] rough**

## Character
Load-bearing interface design — this is the trust boundary between bash zipper world and Rust binary world. Get it right once.

## Docket
Implement the colophon manifest protocol that lets theurge verify its compiled-in colophon strings match the live zipper registry.

**Bash side (rbz zipper):**
- Add kindle constant `readonly ZRBZ_COLOPHON_MANIFEST="${z_buz_colophon_roll[*]}"` to `zrbz_kindle()`, before the `KINDLED=1` sentinel
- Not exported — stays internal, passed explicitly at dispatch boundary

**Rust side (theurge):**
- Theurge accepts the manifest string as an argument
- Compiled-in constant array of colophon strings theurge depends on (initially: charge `rbw-cC`, quench `rbw-cQ`, writ/fiat/bark TBD)
- On startup: for each compiled-in colophon, confirm it appears in the manifest string. Missing → fatal naming the specific string. Manifest absent entirely → fatal with "theurge must be launched via tabtarget"
- The compiled-in set is a subset of the manifest — theurge doesn't care about colophons it doesn't use

**Dispatch glue:**
- The theurge launcher passes `ZRBZ_COLOPHON_MANIFEST` as argument to the binary before any other args

**[260401-1214] rough**

## Character
Load-bearing interface design — this is the trust boundary between bash zipper world and Rust binary world. Get it right once.

## Docket
Implement the colophon manifest protocol that lets theurge verify its compiled-in colophon strings match the live zipper registry.

**Bash side (zipper):**
- Add kindle constant `readonly ZRBZ_COLOPHON_MANIFEST="${z_buz_colophon_roll[*]}"` to `zrbz_kindle()`, before the `KINDLED=1` sentinel
- Not exported — stays internal, passed explicitly at dispatch boundary

**Rust side (theurge):**
- Theurge accepts `--colophon-manifest "rbw-cC rbw-cQ rbw-cs ..."` argument
- Compiled-in constant array of colophon strings theurge depends on (initially: charge, quench, and whatever else the first cases need)
- On startup: for each compiled-in colophon, confirm it appears in the manifest. Missing → fatal naming the specific string. Manifest absent entirely → fatal with "theurge must be launched via tabtarget"
- Exhaustive match on the compiled-in enum — adding a new colophon dependency forces updating the constant

**Dispatch glue:**
- The theurge launcher (workbench dispatch or tabtarget) passes `ZRBZ_COLOPHON_MANIFEST` as argument to the binary before any other args

This pace comes after mint (₢A1AAA) and before container interaction (₢A1AAC), since every tabtarget invocation from Rust depends on this protocol being in place.

### mint-writ-fiat-bark-colophons (₢A1AAN) [complete]

**[260401-1347] complete**

## Character
Minting + mechanical implementation.

## Docket
Mint and implement three new non-interactive container exec tabtargets:
- **writ** — execute arbitrary command in sentry
- **fiat** — execute arbitrary command in pentacle
- **bark** — execute arbitrary command in bottle

These parallel the existing interactive commands (hail/rack/scry) but are exec-and-return: run command, capture output, exit. Designed for theurge to invoke programmatically.

Steps:
1. Assign colophon strings (candidates: `rbw-cw`, `rbw-cf`, `rbw-cb` — following bottle `rbw-c*` pattern)
2. Implement backing functions in `rbob_bottle.sh` / `rbob_cli.sh`
3. Enroll in `rbz_zipper.sh`
4. Create tabtarget files in `tt/`
5. Add to theurge's compiled-in colophon set

**[260401-1258] rough**

## Character
Minting + mechanical implementation.

## Docket
Mint and implement three new non-interactive container exec tabtargets:
- **writ** — execute arbitrary command in sentry
- **fiat** — execute arbitrary command in pentacle
- **bark** — execute arbitrary command in bottle

These parallel the existing interactive commands (hail/rack/scry) but are exec-and-return: run command, capture output, exit. Designed for theurge to invoke programmatically.

Steps:
1. Assign colophon strings (candidates: `rbw-cw`, `rbw-cf`, `rbw-cb` — following bottle `rbw-c*` pattern)
2. Implement backing functions in `rbob_bottle.sh` / `rbob_cli.sh`
3. Enroll in `rbz_zipper.sh`
4. Create tabtarget files in `tt/`
5. Add to theurge's compiled-in colophon set

### theurge-reporting-and-case-framework (₢A1AAB) [complete]

**[260401-1417] complete**

## Character
Design work — judgment required on reporting model.

## Docket
Build the theurge case execution framework: case registration, sequential dispatch, pass/fail/skip tracking, colored terminal output, verbose trace files written to a per-case temp directory. This is the equivalent of BUK's bute_engine + buto_operations + butd_dispatch but as a Rust application.

Key behaviors to match from bash precedent:
- Section headers for test groups
- Green PASSED / red FAILED per case
- Summary line at end (N passed, M failed, K skipped)
- Verbose detail written to files per-case in temp dir (never rerun for detail, just read)
- Fatal on first failure option vs run-all mode
- Per-case temp dir isolation (matches `zbute_tcase` pattern from `bute_engine.sh`)

No container interaction yet — test with dummy cases that pass/fail. No JSON anywhere — terminal text and files.

**[260401-1238] rough**

## Character
Design work — judgment required on reporting model.

## Docket
Build the theurge case execution framework: case registration, sequential dispatch, pass/fail/skip tracking, colored terminal output, verbose trace files written to a per-case temp directory. This is the equivalent of BUK's bute_engine + buto_operations + butd_dispatch but as a Rust application.

Key behaviors to match from bash precedent:
- Section headers for test groups
- Green PASSED / red FAILED per case
- Summary line at end (N passed, M failed, K skipped)
- Verbose detail written to files per-case in temp dir (never rerun for detail, just read)
- Fatal on first failure option vs run-all mode
- Per-case temp dir isolation (matches `zbute_tcase` pattern from `bute_engine.sh`)

No container interaction yet — test with dummy cases that pass/fail. No JSON anywhere — terminal text and files.

**[260401-1202] rough**

## Character
Design work — judgment required on reporting model.

## Docket
Build the theurge case execution framework: case registration, sequential dispatch, pass/fail/skip tracking, colored terminal output, verbose trace files written to TEMP_DIR. This is the equivalent of BUK's bute_engine + buto_operations + butd_dispatch but as a Rust application.

Key behaviors to match from bash precedent:
- Section headers for test groups
- Green PASSED / red FAILED per case
- Summary line at end (N passed, M failed, K skipped)
- Verbose detail written to files (not rerun, just read)
- Fatal on first failure option vs run-all mode

No container interaction yet — test with dummy cases that pass/fail.

### qualify-case-engine (₢A1AAO) [complete]

**[260401-1439] complete**

## Character
Verification gate — mechanical but disciplined. Run what was built, confirm it works before building on it.

## Docket
Exercise the theurge case engine (built in ₢A1AAB) through scenarios that stress its contracts before C/D build real infrastructure on top of it.

**Scenarios to run:**
- Mixed verdicts: cases that pass, fail, and skip in one run — verify counts and colored output
- Fatal-on-first mode: confirm execution stops after first failure, summary reflects partial run
- Run-all mode: confirm all cases execute despite failures
- Multiple sections: verify section headers group correctly in output
- Edge cases: zero cases registered, all-skip, single case
- Temp dir isolation: two cases running sequentially get distinct temp dirs, files don't leak between them
- Verbose trace files: confirm per-case output files are written, readable, contain expected detail

**Exit criteria:** All scenarios produce correct output. No code changes to the engine expected — if something fails, fix it in this pace before proceeding.

**[260401-1353] rough**

## Character
Verification gate — mechanical but disciplined. Run what was built, confirm it works before building on it.

## Docket
Exercise the theurge case engine (built in ₢A1AAB) through scenarios that stress its contracts before C/D build real infrastructure on top of it.

**Scenarios to run:**
- Mixed verdicts: cases that pass, fail, and skip in one run — verify counts and colored output
- Fatal-on-first mode: confirm execution stops after first failure, summary reflects partial run
- Run-all mode: confirm all cases execute despite failures
- Multiple sections: verify section headers group correctly in output
- Edge cases: zero cases registered, all-skip, single case
- Temp dir isolation: two cases running sequentially get distinct temp dirs, files don't leak between them
- Verbose trace files: confirm per-case output files are written, readable, contain expected detail

**Exit criteria:** All scenarios produce correct output. No code changes to the engine expected — if something fails, fix it in this pace before proceeding.

### theurge-container-interaction-layer (₢A1AAC) [complete]

**[260401-1448] complete**

## Character
Infrastructure — mechanical but needs to be solid.

## Docket
Build theurge's Rust code for interacting with bottles and container layers, always via tabtargets.

**Tabtarget invocation layer:**
Theurge shells out to tabtargets using colophon strings verified by the manifest protocol. It constructs paths by globbing `tt/{colophon}.*.{nameplate}.sh`. Uses writ/fiat/bark tabtargets established in ₢A1AAN for container exec, charge/quench for lifecycle.

**BURV isolation for tabtarget invocations:**
When theurge invokes a tabtarget, it must provide per-invocation `BURV_OUTPUT_ROOT_DIR` and `BURV_TEMP_ROOT_DIR` so child tabtarget temp space doesn't collide with theurge's own or with other invocations. Match the isolation pattern from `buto_operations.sh` `zbuto_invoke()`.

**Ifrit invocation:**
Theurge runs the ifrit binary via bark tabtarget (exec in bottle). The binary is already inside the vessel — no copy step. Theurge parses ifrit's stdout for verdict, checks exit code.

**[260401-1315] rough**

## Character
Infrastructure — mechanical but needs to be solid.

## Docket
Build theurge's Rust code for interacting with bottles and container layers, always via tabtargets.

**Tabtarget invocation layer:**
Theurge shells out to tabtargets using colophon strings verified by the manifest protocol. It constructs paths by globbing `tt/{colophon}.*.{nameplate}.sh`. Uses writ/fiat/bark tabtargets established in ₢A1AAN for container exec, charge/quench for lifecycle.

**BURV isolation for tabtarget invocations:**
When theurge invokes a tabtarget, it must provide per-invocation `BURV_OUTPUT_ROOT_DIR` and `BURV_TEMP_ROOT_DIR` so child tabtarget temp space doesn't collide with theurge's own or with other invocations. Match the isolation pattern from `buto_operations.sh` `zbuto_invoke()`.

**Ifrit invocation:**
Theurge runs the ifrit binary via bark tabtarget (exec in bottle). The binary is already inside the vessel — no copy step. Theurge parses ifrit's stdout for verdict, checks exit code.

**[260401-1238] rough**

## Character
Infrastructure — mechanical but needs to be solid. Several design decisions embedded.

## Docket
Build theurge's ability to interact with bottles and container layers, always via tabtargets.

**Tabtarget invocations (charge/quench/observe):**
Theurge shells out to tabtargets using colophon strings verified by the manifest protocol. It constructs paths by globbing `tt/{colophon}.*.{nameplate}.sh`. Charge and quench use existing `rbw-cC` / `rbw-cQ` colophons.

**New container exec tabtargets — three new words:**
- **writ** — execute arbitrary command in sentry (`rbw-cw` or similar colophon)
- **fiat** — execute arbitrary command in pentacle
- **bark** — execute arbitrary command in bottle

These need zipper enrollment in `rbz_zipper.sh`, tabtarget files in `tt/`, and backing implementations in `rbob_cli.sh` / `rbob_bottle.sh`. They parallel the existing hail/rack/scry interactive commands but are non-interactive exec-and-return.

**BURV isolation for tabtarget invocations:**
When theurge invokes a tabtarget, it must provide per-invocation `BURV_OUTPUT_ROOT_DIR` and `BURV_TEMP_ROOT_DIR` so child tabtarget temp space doesn't collide with theurge's own or with other invocations. Match the isolation pattern from `buto_operations.sh` `zbuto_invoke()`.

**Copy ifrit into bottle:**
The "copy ifrit binary into bottle" operation — `docker cp` or equivalent to place the compiled ifrit at a known path inside the bottle before test cases run.

**[260401-1202] rough**

## Character
Infrastructure — mechanical but needs to be solid.

## Docket
Build theurge's ability to interact with bottles: charge a nameplate, exec commands into sentry/pentacle/bottle, capture stdout/stderr/exit code, copy files into containers. This is the bridge between theurge and the existing `rbob_bottle.sh` / podman/docker runtime.

Decide: does theurge shell out to `docker exec` / existing tabtargets, or does it use a Docker API client crate? Leaning toward shelling out — simpler, reuses existing bottle infrastructure, avoids a large dependency.

Include: the "copy ifrit binary into bottle" operation that will be the core of the new model.

### ifrit-binary-with-attack-selector (₢A1AAD) [complete]

**[260401-1513] complete**

## Character
Design + implementation — the attack dispatch model matters.

## Docket
Build the ifrit binary and its vessel image.

**Ifrit binary:**
- Takes an attack selector argument, executes one attack, prints verdict and detail to stdout, exits 0 (pass/SECURE) or nonzero (fail). Plain text output.
- Design the attack enum — exhaustive match means adding a variant forces handling.
- Start with 2-3 simple attacks ported from bash tadmor-security (dns-allowed-anthropic, dns-blocked-google, apt-get-blocked).

**Ifrit vessel Dockerfile (single-stage):**
- Base: `rust:bookworm-slim`
- Install network tools: socat, dnsutils, netcat-openbsd, traceroute, strace
- Copy `Cargo.toml` + `Cargo.lock` first, dummy build to cache dependency layer
- Copy source, real build
- Rust toolchain stays in image (test vessel, size irrelevant, iteration speed matters)

**Build tabtarget:**
- `rbi-iB` colophon for ifrit kludge build
- Writes to a constant kludge consecration for daily iteration (no timestamped tags during development)
- Enroll in zipper, create tabtarget file

**Capabilities:** The ifrit bottle intentionally grants `CAP_NET_RAW` and other dangerous capabilities so attack code can exercise all security boundaries. This is by design.

**[260401-1258] rough**

## Character
Design + implementation — the attack dispatch model matters.

## Docket
Build the ifrit binary and its vessel image.

**Ifrit binary:**
- Takes an attack selector argument, executes one attack, prints verdict and detail to stdout, exits 0 (pass/SECURE) or nonzero (fail). Plain text output.
- Design the attack enum — exhaustive match means adding a variant forces handling.
- Start with 2-3 simple attacks ported from bash tadmor-security (dns-allowed-anthropic, dns-blocked-google, apt-get-blocked).

**Ifrit vessel Dockerfile (single-stage):**
- Base: `rust:bookworm-slim`
- Install network tools: socat, dnsutils, netcat-openbsd, traceroute, strace
- Copy `Cargo.toml` + `Cargo.lock` first, dummy build to cache dependency layer
- Copy source, real build
- Rust toolchain stays in image (test vessel, size irrelevant, iteration speed matters)

**Build tabtarget:**
- `rbi-iB` colophon for ifrit kludge build
- Writes to a constant kludge consecration for daily iteration (no timestamped tags during development)
- Enroll in zipper, create tabtarget file

**Capabilities:** The ifrit bottle intentionally grants `CAP_NET_RAW` and other dangerous capabilities so attack code can exercise all security boundaries. This is by design.

**[260401-1238] rough**

## Character
Design + implementation — the attack dispatch model matters.

## Docket
Build the ifrit binary: a static Rust executable that takes an attack selector argument, executes one attack, prints verdict and detail to stdout, exits with 0 (pass/SECURE) or nonzero (fail/BREACH or ERROR). Plain text output, no JSON.

Start with 2-3 simple attacks ported from the bash tadmor-security cases (e.g., dns-allowed-anthropic, dns-blocked-google, apt-get-blocked). These are simpler than the scapy-based python sorties and prove the model.

Design the attack enum — exhaustive match means adding a variant forces handling. The selector is a string on the command line mapped to an enum variant internally.

**Build target:** Ifrit must produce a Linux ELF binary from a macOS host. This is a cross-OS compilation requirement (same architecture, different OS). Resolve the toolchain question during this pace — options include `cross-rs`, `brew install musl-cross`, or building inside a lightweight container. See paddock for discussion.

**Capabilities:** The ifrit bottle intentionally grants `CAP_NET_RAW` and other dangerous capabilities so that attack code can exercise all security boundaries. This is by design — default bottles do NOT have these permissions.

**[260401-1202] rough**

## Character
Design + implementation — the attack dispatch model matters.

## Docket
Build the ifrit binary: a static Rust executable that takes an attack selector argument, executes one attack, prints structured output (verdict + detail to stdout), exits with 0 (SECURE) or 1 (BREACH/ERROR).

Start with 2-3 simple attacks ported from the bash tadmor-security cases (e.g., dns-allowed-anthropic, dns-blocked-google, apt-get-blocked). These are simpler than the scapy-based python sorties and prove the model.

Design the attack enum — exhaustive match means adding a variant forces handling. The selector is a string on the command line mapped to an enum variant internally.

Static linking: target `x86_64-unknown-linux-musl` so the binary runs in any container with no runtime dependencies.

### qualify-ifrit-vessel-standalone (₢A1AAP) [complete]

**[260401-1526] complete**

## Character
Verification gate — prove the ifrit vessel works in isolation before attempting the full theurge→ifrit pipeline.

## Docket
Verify the ifrit binary and vessel image (built in ₢A1AAD) work standalone before ₢A1AAE wires theurge+ifrit end-to-end.

**Verify build:**
- `rbi-iB` kludge tabtarget produces a vessel image successfully
- Image contains the ifrit binary at expected path
- Image contains required network tools (socat, dnsutils, netcat-openbsd, traceroute, strace)

**Verify ifrit binary:**
- Manually charge a tadmor crucible (via `rbw-cC` tabtarget)
- Exec ifrit inside the bottle via bark tabtarget with each initial attack selector
- Confirm: correct stdout format (parseable by theurge's expected contract), correct exit codes (0 for pass/SECURE, nonzero for fail)
- Confirm: unknown attack selector produces clear error, not a panic

**Verify layer caching:**
- Rebuild with source-only change, confirm dependency layer is cached (fast rebuild)

**Exit criteria:** Ifrit builds, runs inside a charged bottle, produces parseable output for all initial attack variants. Theurge can depend on this contract in ₢A1AAE.

**[260401-1353] rough**

## Character
Verification gate — prove the ifrit vessel works in isolation before attempting the full theurge→ifrit pipeline.

## Docket
Verify the ifrit binary and vessel image (built in ₢A1AAD) work standalone before ₢A1AAE wires theurge+ifrit end-to-end.

**Verify build:**
- `rbi-iB` kludge tabtarget produces a vessel image successfully
- Image contains the ifrit binary at expected path
- Image contains required network tools (socat, dnsutils, netcat-openbsd, traceroute, strace)

**Verify ifrit binary:**
- Manually charge a tadmor crucible (via `rbw-cC` tabtarget)
- Exec ifrit inside the bottle via bark tabtarget with each initial attack selector
- Confirm: correct stdout format (parseable by theurge's expected contract), correct exit codes (0 for pass/SECURE, nonzero for fail)
- Confirm: unknown attack selector produces clear error, not a panic

**Verify layer caching:**
- Rebuild with source-only change, confirm dependency layer is cached (fast rebuild)

**Exit criteria:** Ifrit builds, runs inside a charged bottle, produces parseable output for all initial attack variants. Theurge can depend on this contract in ₢A1AAE.

### first-end-to-end-tadmor-cases (₢A1AAE) [complete]

**[260401-1542] complete**

## Character
Integration milestone — the proof that the architecture works.

## Docket
Wire theurge + ifrit together for the first time against the tadmor nameplate. Theurge charges the crucible via `rbw-cC` tabtarget (ifrit binary is already inside the vessel image), runs 3-5 attack cases via bark tabtarget (exec ifrit in bottle), monitors sentry state from outside via writ tabtarget (exec in sentry), correlates results, reports verdicts.

This is the moment where "simultaneous inside/outside observation" becomes real. Pick cases where outside monitoring adds value (e.g., iptables counter delta on a blocked DNS query).

Theurge owns the full lifecycle: charge → run cases → quench. Establish tabtarget using existing infix pattern with imprint for tier/fixture selection.

**[260401-1315] rough**

## Character
Integration milestone — the proof that the architecture works.

## Docket
Wire theurge + ifrit together for the first time against the tadmor nameplate. Theurge charges the crucible via `rbw-cC` tabtarget (ifrit binary is already inside the vessel image), runs 3-5 attack cases via bark tabtarget (exec ifrit in bottle), monitors sentry state from outside via writ tabtarget (exec in sentry), correlates results, reports verdicts.

This is the moment where "simultaneous inside/outside observation" becomes real. Pick cases where outside monitoring adds value (e.g., iptables counter delta on a blocked DNS query).

Theurge owns the full lifecycle: charge → run cases → quench. Establish tabtarget using existing infix pattern with imprint for tier/fixture selection.

**[260401-1238] rough**

## Character
Integration milestone — the proof that the architecture works.

## Docket
Wire theurge + ifrit together for the first time against the tadmor nameplate. Theurge charges the bottle via `rbw-cC` tabtarget, copies ifrit in, runs 3-5 attack cases via bark tabtarget (exec in bottle), monitors sentry state from outside via writ tabtarget (exec in sentry), correlates results, reports verdicts.

This is the moment where "simultaneous inside/outside observation" becomes real. Pick cases where outside monitoring adds value (e.g., iptables counter delta on a blocked DNS query).

Theurge owns the full lifecycle: charge → copy ifrit → run cases → quench. Establish tabtarget using existing infix pattern.

**[260401-1202] rough**

## Character
Integration milestone — the proof that the architecture works.

## Docket
Wire theurge + ifrit together for the first time against the tadmor nameplate. Theurge charges the bottle, copies ifrit in, runs 3-5 attack cases via `docker exec`, monitors sentry state from outside, correlates results, reports verdicts.

This is the moment where "simultaneous inside/outside observation" becomes real. Pick cases where outside monitoring adds value (e.g., iptables counter delta on a blocked DNS query).

Establish tabtarget: reuse existing infix pattern, something like `tt/rbw-tf.TestFixture.tadmor.sh` invoking the theurge binary.

### port-remaining-bash-tadmor-security (₢A1AAF) [complete]

**[260401-1915] complete**

## Character
Mechanical porting + incremental cutover — volume work with a verification gate and retirement at the end.

## Docket
Port the tadmor-security bash cases (22 total in `rbtcns_TadmorSecurity.sh`) to the theurge+ifrit model, verify parity, and retire the bash originals.

**Port** — categorize and port each existing case:
- **ifrit attack** — runs inside bottle (DNS queries, apt-get, ICMP probes)
- **theurge-only** — exec into sentry/pentacle via writ/fiat tabtargets to observe (dnsmasq listening, iptables rules, ping within enclave)
- **correlated** — ifrit attacks while theurge simultaneously monitors from outside

Categories from the existing 22:
- Basic infra (dnsmasq listening, ping, iptables) — theurge-only via writ (sentry observation)
- DNS allow/block (11 cases) — correlated (ifrit tries via bark, theurge checks counters via writ)
- TCP 443 allow/block — correlated
- Package mgmt block — ifrit-only via bark
- ICMP restrictions — correlated

Do NOT deduplicate with python sorties during this pace — faithful port only.

**Verify parity** — run both bash (`rbw-tf.TestFixture.tadmor-security.sh`) and theurge tadmor-security. Confirm same cases, same verdicts. Document any intentional differences (e.g., correlated observation is new — that's an addition, not a divergence).

**Retire** — remove tadmor-security case functions from `rbtcns_TadmorSecurity.sh`. Remove or hollow out the fixture file. Leave the fixture tabtarget pointing at theurge (or remove if theurge has its own invocation path by now).

**[260401-1358] rough**

## Character
Mechanical porting + incremental cutover — volume work with a verification gate and retirement at the end.

## Docket
Port the tadmor-security bash cases (22 total in `rbtcns_TadmorSecurity.sh`) to the theurge+ifrit model, verify parity, and retire the bash originals.

**Port** — categorize and port each existing case:
- **ifrit attack** — runs inside bottle (DNS queries, apt-get, ICMP probes)
- **theurge-only** — exec into sentry/pentacle via writ/fiat tabtargets to observe (dnsmasq listening, iptables rules, ping within enclave)
- **correlated** — ifrit attacks while theurge simultaneously monitors from outside

Categories from the existing 22:
- Basic infra (dnsmasq listening, ping, iptables) — theurge-only via writ (sentry observation)
- DNS allow/block (11 cases) — correlated (ifrit tries via bark, theurge checks counters via writ)
- TCP 443 allow/block — correlated
- Package mgmt block — ifrit-only via bark
- ICMP restrictions — correlated

Do NOT deduplicate with python sorties during this pace — faithful port only.

**Verify parity** — run both bash (`rbw-tf.TestFixture.tadmor-security.sh`) and theurge tadmor-security. Confirm same cases, same verdicts. Document any intentional differences (e.g., correlated observation is new — that's an addition, not a divergence).

**Retire** — remove tadmor-security case functions from `rbtcns_TadmorSecurity.sh`. Remove or hollow out the fixture file. Leave the fixture tabtarget pointing at theurge (or remove if theurge has its own invocation path by now).

**[260401-1238] rough**

## Character
Mechanical porting — volume work, low judgment.

## Docket
Port the remaining tadmor-security bash cases (22 total in `rbtcns_TadmorSecurity.sh`) to the theurge+ifrit model. Categorize each existing case:
- **ifrit attack** — runs inside bottle (DNS queries, apt-get, ICMP probes)
- **theurge-only** — exec into sentry/pentacle via writ/fiat tabtargets to observe (dnsmasq listening, iptables rules, ping within enclave)
- **correlated** — ifrit attacks while theurge simultaneously monitors from outside

Categories from the existing 22:
- Basic infra (dnsmasq listening, ping, iptables) — theurge-only via writ (sentry observation)
- DNS allow/block (11 cases) — correlated (ifrit tries via bark, theurge checks counters via writ)
- TCP 443 allow/block — correlated
- Package mgmt block — ifrit-only via bark
- ICMP restrictions — correlated

Do NOT deduplicate with python sorties during this pace — faithful port only.

**[260401-1202] rough**

## Character
Mechanical porting — volume work, low judgment.

## Docket
Port the remaining tadmor-security bash cases (22 total in `rbtcns_TadmorSecurity.sh`) to the theurge+ifrit model. Some become ifrit attack variants (things that run inside the bottle), some become theurge-only cases (exec a command into sentry/pentacle and check from outside). Categorize each existing case as inside, outside, or correlated (both).

Categories from the existing 22:
- Basic infra (dnsmasq listening, ping, iptables) — likely theurge-only (outside observation)
- DNS allow/block (11 cases) — correlated (ifrit tries, theurge checks counters)
- TCP 443 allow/block — correlated
- Package mgmt block — ifrit-only
- ICMP restrictions — correlated

### kludge-consecration-override-design (₢A1AAR) [complete]

**[260402-0715] complete**

## Character
Design conversation requiring judgment — touches charge contract, nameplate semantics, compose plumbing, vouch validation, and the boundary between local iteration and regime truth. Get the design right before implementing; the implementation is probably small.

## Docket
Design and implement a mechanism for kludge builds to automatically override nameplate vessel consecrations for local testing, without mutating the nameplate's regime-truth state or dirtying the git working tree.

**Problem**: After `rbi-iK` builds a local kludge image, you must manually edit `rbrn.env` to change `RBRN_BOTTLE_CONSECRATION`, then manually restore it later. This is error-prone, creates git-dirty state (rbrn.env is tracked), and conflates iteration state with regime truth.

**Design angles to resolve:**

1. **All consumers of consecration** — `rbob_charge` reads `RBRN_BOTTLE_CONSECRATION` and `RBRN_SENTRY_CONSECRATION` to construct image tags and vouch tags. Compose receives these via environment. Summon uses them for pulling. The override mechanism must work for every consumer that flows through `rbob_charge`, not just theurge.

2. **Vouch gate** — `rbob_charge` checks `image inspect` on both vouch and image tags before starting. Kludge builds produce both `-image` and `-vouch` tags, so this should work if the override consecration matches what kludge tagged. Verify this is true for the current `rbi-iK` flow.

3. **Git cleanliness** — `rbrn.env` is version-controlled. Any override mechanism that writes to it creates dirty working tree state, which interferes with jjx_record file lists and general repo hygiene. The override must not modify tracked files.

4. **Partial vessel override** — A nameplate has both sentry and bottle vessels with independent consecrations. Kludging the bottle (ifrit) while keeping the real sentry is the common case. The mechanism must support overriding one vessel's consecration without affecting the other.

5. **Compose plumbing** — How does compose get the image tags? Does `rbob_charge` pass them as env vars to `docker compose up`, or does compose read `rbrn.env` directly? This determines whether the override hooks into the charge bash code or needs a compose-level mechanism.

6. **Persistence across sessions** — An env var override vanishes when the terminal closes. A file-based override persists. Consider: `.rbk/{nameplate}/kludge.env` (gitignored sidecar) that charge reads as an overlay. Kludge tabtarget writes it, conjure/real-build deletes it.

7. **Multiple independent kludges** — What if you kludge both sentry and bottle at different times? The mechanism should handle accumulation (each kludge adds/replaces its vessel's override) and selective clearing.

8. **Kludge consecration naming** — `kdev` is a constant baked into the kludge tabtarget. Is one name sufficient (latest-wins), or do you need distinguishable iterations? For daily local dev, latest-wins is probably fine. The name should be recognizable as a kludge when it appears in logs/errors.

9. **Who sets and who clears?** — Your musing: "the tabtarget that causes the kludge ought to sub in the name, and the build-real-cloud-consecration ought to change it back." This implies: kludge tabtarget writes the override, conjure clears it. What about manual clearing? A `rbi-iU.IfritUnkludge` tabtarget, or just `rm .rbk/tadmor/kludge.env`?

10. **Theurge `--kludge` flag vs transparent override** — Two approaches: (a) charge itself transparently reads the sidecar — all consumers get the override automatically; (b) theurge passes `--kludge` to charge — explicit but requires every consumer to opt in. Option (a) is simpler and covers manual `rbw-cC.Charge` too.

**Candidate design (to validate or reject):**
- Kludge tabtarget writes `.rbk/{nameplate}/kludge.env` with `RBRN_BOTTLE_CONSECRATION=kludge` (or whichever vessel it built)
- `.gitignore` includes `.rbk/*/kludge.env`
- `rbob_charge` (early, after loading nameplate) checks for `kludge.env` and overlays any values onto the regime environment, with a visible log line: "Kludge override active: RBRN_BOTTLE_CONSECRATION=kludge"
- Cloud conjure success deletes the sidecar (or: conjure doesn't touch it — the sidecar is local-only and the user clears it when satisfied)
- `rbrn.env` is never modified by kludge or override machinery

**Deliverable**: chosen design, implementation in charge path, kludge tabtarget writes sidecar, gitignore entry, verify full cycle (kludge → charge → theurge → clear).

**[260401-1918] rough**

## Character
Design conversation requiring judgment — touches charge contract, nameplate semantics, compose plumbing, vouch validation, and the boundary between local iteration and regime truth. Get the design right before implementing; the implementation is probably small.

## Docket
Design and implement a mechanism for kludge builds to automatically override nameplate vessel consecrations for local testing, without mutating the nameplate's regime-truth state or dirtying the git working tree.

**Problem**: After `rbi-iK` builds a local kludge image, you must manually edit `rbrn.env` to change `RBRN_BOTTLE_CONSECRATION`, then manually restore it later. This is error-prone, creates git-dirty state (rbrn.env is tracked), and conflates iteration state with regime truth.

**Design angles to resolve:**

1. **All consumers of consecration** — `rbob_charge` reads `RBRN_BOTTLE_CONSECRATION` and `RBRN_SENTRY_CONSECRATION` to construct image tags and vouch tags. Compose receives these via environment. Summon uses them for pulling. The override mechanism must work for every consumer that flows through `rbob_charge`, not just theurge.

2. **Vouch gate** — `rbob_charge` checks `image inspect` on both vouch and image tags before starting. Kludge builds produce both `-image` and `-vouch` tags, so this should work if the override consecration matches what kludge tagged. Verify this is true for the current `rbi-iK` flow.

3. **Git cleanliness** — `rbrn.env` is version-controlled. Any override mechanism that writes to it creates dirty working tree state, which interferes with jjx_record file lists and general repo hygiene. The override must not modify tracked files.

4. **Partial vessel override** — A nameplate has both sentry and bottle vessels with independent consecrations. Kludging the bottle (ifrit) while keeping the real sentry is the common case. The mechanism must support overriding one vessel's consecration without affecting the other.

5. **Compose plumbing** — How does compose get the image tags? Does `rbob_charge` pass them as env vars to `docker compose up`, or does compose read `rbrn.env` directly? This determines whether the override hooks into the charge bash code or needs a compose-level mechanism.

6. **Persistence across sessions** — An env var override vanishes when the terminal closes. A file-based override persists. Consider: `.rbk/{nameplate}/kludge.env` (gitignored sidecar) that charge reads as an overlay. Kludge tabtarget writes it, conjure/real-build deletes it.

7. **Multiple independent kludges** — What if you kludge both sentry and bottle at different times? The mechanism should handle accumulation (each kludge adds/replaces its vessel's override) and selective clearing.

8. **Kludge consecration naming** — `kdev` is a constant baked into the kludge tabtarget. Is one name sufficient (latest-wins), or do you need distinguishable iterations? For daily local dev, latest-wins is probably fine. The name should be recognizable as a kludge when it appears in logs/errors.

9. **Who sets and who clears?** — Your musing: "the tabtarget that causes the kludge ought to sub in the name, and the build-real-cloud-consecration ought to change it back." This implies: kludge tabtarget writes the override, conjure clears it. What about manual clearing? A `rbi-iU.IfritUnkludge` tabtarget, or just `rm .rbk/tadmor/kludge.env`?

10. **Theurge `--kludge` flag vs transparent override** — Two approaches: (a) charge itself transparently reads the sidecar — all consumers get the override automatically; (b) theurge passes `--kludge` to charge — explicit but requires every consumer to opt in. Option (a) is simpler and covers manual `rbw-cC.Charge` too.

**Candidate design (to validate or reject):**
- Kludge tabtarget writes `.rbk/{nameplate}/kludge.env` with `RBRN_BOTTLE_CONSECRATION=kludge` (or whichever vessel it built)
- `.gitignore` includes `.rbk/*/kludge.env`
- `rbob_charge` (early, after loading nameplate) checks for `kludge.env` and overlays any values onto the regime environment, with a visible log line: "Kludge override active: RBRN_BOTTLE_CONSECRATION=kludge"
- Cloud conjure success deletes the sidecar (or: conjure doesn't touch it — the sidecar is local-only and the user clears it when satisfied)
- `rbrn.env` is never modified by kludge or override machinery

**Deliverable**: chosen design, implementation in charge path, kludge tabtarget writes sidecar, gitignore entry, verify full cycle (kludge → charge → theurge → clear).

**[260401-1914] rough**

## Character
Design conversation requiring judgment — touches charge contract, nameplate semantics, and the boundary between local iteration and regime truth.

## Docket
Design and implement a mechanism for kludge builds (ifrit, and potentially other vessels) to automatically override the nameplate bottle consecration for local testing, without mutating the nameplate's regime-truth consecration.

**Problem**: After `rbi-iK` builds a local kludge image, you must manually edit `rbrn.env` to `RBRN_BOTTLE_CONSECRATION=kdev`, then manually restore it later. This is error-prone and conflates iteration state with regime state.

**Design space**:
- **Runtime override at charge**: theurge (or charge itself) accepts a `--kludge` flag that substitutes the kludge consecration at charge time, leaving the nameplate untouched
- **Sidecar save/restore**: kludge tabtarget saves prior consecration to a sidecar file, restores on conjure — fragile if interrupted
- **Naming**: `kdev` is too generic — consider nameplate-scoped (`kludge`) or vessel-scoped naming

**Constraints**:
- Nameplate must remain source of truth for "what's deployed"
- Must not break cloud conjure flow
- Should work for any vessel kludge, not just ifrit

**Deliverable**: chosen design, implementation in charge path and/or theurge, updated kludge tabtarget if needed.

### port-python-sorties-to-ifrit (₢A1AAG) [complete]

**[260402-0736] complete**

## Character
Mechanical porting + incremental cutover — occasional design judgment for raw socket attacks.

## Docket
Port the 11 rostered python sorties from `Tools/rbk/rbtid/rbtis_*.py` to ifrit attack variants, verify parity, and retire the python infrastructure.

**Port** — each python sortie's `run()` function becomes a Rust attack variant.

Complexity gradient:
- Simple (dig/nc based): dns_exfil_subdomain, meta_cloud_endpoint, net_forbidden_cidr, direct_sentry_probe — straightforward port
- Medium (need raw sockets): icmp_exfil_payload, net_ipv6_escape, net_srcip_spoof — need `socket2` or similar crate. The ifrit bottle intentionally grants `CAP_NET_RAW` for this purpose.
- Hard (scapy-level): proto_smuggle_rawsock, net_fragment_evasion, direct_arp_poison, ns_capability_escape — may need careful Rust equivalents

For each, add corresponding theurge outside-observation via writ/fiat tabtargets where it adds value.

Do NOT deduplicate with ported bash cases during this pace — faithful port only.

**Verify parity** — run both python sorties (via existing `rbw-Is` sortie tabtargets) and theurge ifrit-attack cases. Confirm same attack surfaces exercised, same security verdicts. Document where Rust implementation improves on python (e.g., proper raw socket vs scapy shim).

**Retire** — remove `Tools/rbk/rbtid/` (python adjutant, roster, all `rbtis_*.py` sorties, debrief). Remove sortie-related zipper enrollments (`rbw-Is`). Remove sortie tabtargets.

**[260401-1358] rough**

## Character
Mechanical porting + incremental cutover — occasional design judgment for raw socket attacks.

## Docket
Port the 11 rostered python sorties from `Tools/rbk/rbtid/rbtis_*.py` to ifrit attack variants, verify parity, and retire the python infrastructure.

**Port** — each python sortie's `run()` function becomes a Rust attack variant.

Complexity gradient:
- Simple (dig/nc based): dns_exfil_subdomain, meta_cloud_endpoint, net_forbidden_cidr, direct_sentry_probe — straightforward port
- Medium (need raw sockets): icmp_exfil_payload, net_ipv6_escape, net_srcip_spoof — need `socket2` or similar crate. The ifrit bottle intentionally grants `CAP_NET_RAW` for this purpose.
- Hard (scapy-level): proto_smuggle_rawsock, net_fragment_evasion, direct_arp_poison, ns_capability_escape — may need careful Rust equivalents

For each, add corresponding theurge outside-observation via writ/fiat tabtargets where it adds value.

Do NOT deduplicate with ported bash cases during this pace — faithful port only.

**Verify parity** — run both python sorties (via existing `rbw-Is` sortie tabtargets) and theurge ifrit-attack cases. Confirm same attack surfaces exercised, same security verdicts. Document where Rust implementation improves on python (e.g., proper raw socket vs scapy shim).

**Retire** — remove `Tools/rbk/rbtid/` (python adjutant, roster, all `rbtis_*.py` sorties, debrief). Remove sortie-related zipper enrollments (`rbw-Is`). Remove sortie tabtargets.

**[260401-1238] rough**

## Character
Mechanical porting with occasional design judgment — some sorties use scapy which needs a Rust raw socket equivalent.

## Docket
Port the 11 rostered python sorties from `Tools/rbk/rbtid/rbtis_*.py` to ifrit attack variants. Each python sortie's `run()` function becomes a Rust attack variant.

Complexity gradient:
- Simple (dig/nc based): dns_exfil_subdomain, meta_cloud_endpoint, net_forbidden_cidr, direct_sentry_probe — straightforward port
- Medium (need raw sockets): icmp_exfil_payload, net_ipv6_escape, net_srcip_spoof — need `socket2` or similar crate. The ifrit bottle intentionally grants `CAP_NET_RAW` for this purpose.
- Hard (scapy-level): proto_smuggle_rawsock, net_fragment_evasion, direct_arp_poison, ns_capability_escape — may need careful Rust equivalents

For each, add corresponding theurge outside-observation via writ/fiat tabtargets where it adds value.

Do NOT deduplicate with ported bash cases during this pace — faithful port only.

**[260401-1203] rough**

## Character
Mechanical porting with occasional design judgment — some sorties use scapy which needs a Rust raw socket equivalent.

## Docket
Port the 11 rostered python sorties from `Tools/rbk/rbtid/rbtis_*.py` to ifrit attack variants. Each python sortie's `run()` function becomes a Rust attack variant.

Complexity gradient:
- Simple (dig/nc based): dns_exfil_subdomain, meta_cloud_endpoint, net_forbidden_cidr, direct_sentry_probe — straightforward port
- Medium (need raw sockets): icmp_exfil_payload, net_ipv6_escape, net_srcip_spoof — need `socket2` or similar crate
- Hard (scapy-level): proto_smuggle_rawsock, net_fragment_evasion, direct_arp_poison, ns_capability_escape — may need careful Rust equivalents

For each, add corresponding theurge outside-observation where it adds value.

### port-srjcl-pluml-service-tests (₢A1AAH) [complete]

**[260402-0955] complete**

## Character
Mechanical porting + incremental cutover — different bottle types but same pattern.

## Docket
Port the service-tier and integration bash test fixtures to theurge, verify parity, and retire the bash originals.

**Port:**

*Service health tests (outside-only, no ifrit):*
- srjcl-jupyter (3 cases): process check, HTTP connectivity, WebSocket kernel
- pluml-diagram (5 cases): text rendering, local diagram, HTTP headers, invalid hash, malformed diagram
- These charge the appropriate nameplate and probe from the host via theurge

*Four-mode integration test (15 steps):*
- Conjure/bind/graft/kludge/run/abjure cycle testing the full foundry build pipeline
- This is NOT optional — all tests migrate. Port faithfully even though it tests build pipeline rather than container security.
- Theurge invokes foundry tabtargets (enshrine, ordain, abjure, wrest, kludge, tally) — these colophons must be in the manifest protocol

*Access-probe tests (4 cases):*
- JWT and OAuth credential validation with retry logic
- Service-tier dependency (GCP credentials required)

**Verify parity** — run both bash fixtures (`rbw-tf.TestFixture.srjcl-jupyter.sh`, `rbw-tf.TestFixture.pluml-diagram.sh`, `rbw-tf.TestFixture.four-mode.sh`, `rbw-tf.TestFixture.access-probe.sh`) and theurge equivalents. Confirm same cases, same verdicts.

**Retire** — remove the bash case files for these fixtures (`rbtcsj_*.sh`, `rbtcpd_*.sh`, `rbtcfm_*.sh`, `rbtcap_*.sh` or however they're named). Remove or redirect fixture tabtargets.

**[260401-1358] rough**

## Character
Mechanical porting + incremental cutover — different bottle types but same pattern.

## Docket
Port the service-tier and integration bash test fixtures to theurge, verify parity, and retire the bash originals.

**Port:**

*Service health tests (outside-only, no ifrit):*
- srjcl-jupyter (3 cases): process check, HTTP connectivity, WebSocket kernel
- pluml-diagram (5 cases): text rendering, local diagram, HTTP headers, invalid hash, malformed diagram
- These charge the appropriate nameplate and probe from the host via theurge

*Four-mode integration test (15 steps):*
- Conjure/bind/graft/kludge/run/abjure cycle testing the full foundry build pipeline
- This is NOT optional — all tests migrate. Port faithfully even though it tests build pipeline rather than container security.
- Theurge invokes foundry tabtargets (enshrine, ordain, abjure, wrest, kludge, tally) — these colophons must be in the manifest protocol

*Access-probe tests (4 cases):*
- JWT and OAuth credential validation with retry logic
- Service-tier dependency (GCP credentials required)

**Verify parity** — run both bash fixtures (`rbw-tf.TestFixture.srjcl-jupyter.sh`, `rbw-tf.TestFixture.pluml-diagram.sh`, `rbw-tf.TestFixture.four-mode.sh`, `rbw-tf.TestFixture.access-probe.sh`) and theurge equivalents. Confirm same cases, same verdicts.

**Retire** — remove the bash case files for these fixtures (`rbtcsj_*.sh`, `rbtcpd_*.sh`, `rbtcfm_*.sh`, `rbtcap_*.sh` or however they're named). Remove or redirect fixture tabtargets.

**[260401-1239] rough**

## Character
Mechanical porting — different bottle types but same pattern.

## Docket
Port ALL remaining bash test fixtures to theurge:

**Service health tests (outside-only, no ifrit):**
- srjcl-jupyter (3 cases): process check, HTTP connectivity, WebSocket kernel
- pluml-diagram (5 cases): text rendering, local diagram, HTTP headers, invalid hash, malformed diagram
- These charge the appropriate nameplate and probe from the host via theurge

**Four-mode integration test (15 steps):**
- Conjure/bind/graft/kludge/run/abjure cycle testing the full foundry build pipeline
- This is NOT optional — all tests migrate. Port faithfully even though it tests build pipeline rather than container security.
- Theurge invokes foundry tabtargets (enshrine, ordain, abjure, wrest, kludge, tally) — these colophons must be in the manifest protocol

**Access-probe tests (4 cases):**
- JWT and OAuth credential validation with retry logic
- Service-tier dependency (GCP credentials required)

**[260401-1203] rough**

## Character
Mechanical porting — different bottle types but same pattern.

## Docket
Port the srjcl-jupyter (3 cases) and pluml-diagram (5 cases) bash test fixtures to theurge. These don't use the ifrit — they're service health tests (HTTP GET, WebSocket kernel creation, rendering verification). Theurge handles them as outside-only cases that charge the appropriate nameplate and probe from the host.

Also port the four-mode integration test (15 steps) if it fits theurge's model, or assess whether it stays as a separate concern (it tests the build pipeline, not container security).

### port-fast-tier-regime-enrollment-validation (₢A1AAI) [complete]

**[260402-1019] complete**

## Character
Careful porting + incremental cutover — testing bash regime parsing from Rust requires understanding the contract without reimplementing it.

## Docket
Port the fast-tier bash tests to theurge, verify parity, and retire the bash originals.

**Port:**
- 47 enrollment-validation cases (`rbtcev_EnrollmentValidation.sh`)
- 21 regime-validation cases (`rbtcrv_RegimeValidation.sh`)
- 7 regime-smoke cases

Approach: shell out to bash regime scripts and validate their behavior. Theurge invokes the bash utilities (source rbrr.sh, kindle, enforce) and asserts on exit codes and stderr. This tests the *actual* bash regime loading, not a Rust reimplementation. Cases will likely be easier to express in Rust even though they shell to bash.

Do NOT reimplement regime validation in Rust — that's a future concern if it ever becomes one.

**Verify parity** — run both bash (`rbw-tf.TestFixture.enrollment-validation.sh`, `rbw-tf.TestFixture.regime-validation.sh`, `rbw-tf.TestFixture.regime-smoke.sh`) and theurge equivalents. Confirm same cases, same verdicts, same edge-case coverage.

**Retire** — remove the bash case files (`rbtcev_EnrollmentValidation.sh`, `rbtcrv_RegimeValidation.sh`, regime-smoke cases). Remove or redirect fixture tabtargets.

**[260401-1358] rough**

## Character
Careful porting + incremental cutover — testing bash regime parsing from Rust requires understanding the contract without reimplementing it.

## Docket
Port the fast-tier bash tests to theurge, verify parity, and retire the bash originals.

**Port:**
- 47 enrollment-validation cases (`rbtcev_EnrollmentValidation.sh`)
- 21 regime-validation cases (`rbtcrv_RegimeValidation.sh`)
- 7 regime-smoke cases

Approach: shell out to bash regime scripts and validate their behavior. Theurge invokes the bash utilities (source rbrr.sh, kindle, enforce) and asserts on exit codes and stderr. This tests the *actual* bash regime loading, not a Rust reimplementation. Cases will likely be easier to express in Rust even though they shell to bash.

Do NOT reimplement regime validation in Rust — that's a future concern if it ever becomes one.

**Verify parity** — run both bash (`rbw-tf.TestFixture.enrollment-validation.sh`, `rbw-tf.TestFixture.regime-validation.sh`, `rbw-tf.TestFixture.regime-smoke.sh`) and theurge equivalents. Confirm same cases, same verdicts, same edge-case coverage.

**Retire** — remove the bash case files (`rbtcev_EnrollmentValidation.sh`, `rbtcrv_RegimeValidation.sh`, regime-smoke cases). Remove or redirect fixture tabtargets.

**[260401-1239] rough**

## Character
Careful — testing bash regime parsing from Rust requires understanding the contract without reimplementing it.

## Docket
Port the fast-tier bash tests to theurge:
- 47 enrollment-validation cases (`rbtcev_EnrollmentValidation.sh`)
- 21 regime-validation cases (`rbtcrv_RegimeValidation.sh`)
- 7 regime-smoke cases

Approach: shell out to bash regime scripts and validate their behavior. Theurge invokes the bash utilities (source rbrr.sh, kindle, enforce) and asserts on exit codes and stderr. This tests the *actual* bash regime loading, not a Rust reimplementation. Cases will likely be easier to express in Rust even though they shell to bash.

Do NOT reimplement regime validation in Rust — that's a future concern if it ever becomes one.

**[260401-1203] rough**

## Character
Careful — reimplementing regime validation in Rust means understanding the bash regime loading contract deeply.

## Docket
Port the fast-tier bash tests to theurge:
- 47 enrollment-validation cases (`rbtcev_EnrollmentValidation.sh`)
- 21 regime-validation cases (`rbtcrv_RegimeValidation.sh`)
- 7 regime-smoke cases

These are pure config validation — no containers. They currently test bash regime loading (source rbrr.sh, kindle, enforce). The Rust equivalent either:
(a) shells out to the bash regime scripts and validates their behavior, or
(b) re-implements regime validation in Rust

Option (a) is more faithful and less work. Option (b) is cleaner but risks divergence. Decide during pace.

### audit-coverage-parity (₢A1AAQ) [abandoned]

**[260401-1358] abandoned**

## Character
Verification gate — systematic audit, no code changes. This must complete cleanly before retirement (₢A1AAJ) touches anything.

## Docket
After all porting paces (₢A1AAF, ₢A1AAG, ₢A1AAH, ₢A1AAI) are complete, audit that every existing test case has a theurge equivalent before retirement deletes the originals.

**Audit steps:**
- Extract complete case list from bash test fixtures: `rbtcns_TadmorSecurity.sh` (22), `rbtcev_EnrollmentValidation.sh` (47), `rbtcrv_RegimeValidation.sh` (21), regime-smoke (7), srjcl-jupyter (3), pluml-diagram (5), four-mode (15), access-probe (4)
- Extract complete sortie list from python roster: `rbtis_*.py` (11 modules)
- Extract complete theurge case catalog from Rust source
- Produce a side-by-side diff: original case → theurge equivalent, flagging any gaps
- Run theurge complete suite, confirm all cases pass

**Exit criteria:** Zero gaps between original and theurge catalogs. All theurge cases green. Written parity report committed as evidence for the retirement pace. Any gaps discovered here become fix-paces slated before ₢A1AAJ.

**[260401-1353] rough**

## Character
Verification gate — systematic audit, no code changes. This must complete cleanly before retirement (₢A1AAJ) touches anything.

## Docket
After all porting paces (₢A1AAF, ₢A1AAG, ₢A1AAH, ₢A1AAI) are complete, audit that every existing test case has a theurge equivalent before retirement deletes the originals.

**Audit steps:**
- Extract complete case list from bash test fixtures: `rbtcns_TadmorSecurity.sh` (22), `rbtcev_EnrollmentValidation.sh` (47), `rbtcrv_RegimeValidation.sh` (21), regime-smoke (7), srjcl-jupyter (3), pluml-diagram (5), four-mode (15), access-probe (4)
- Extract complete sortie list from python roster: `rbtis_*.py` (11 modules)
- Extract complete theurge case catalog from Rust source
- Produce a side-by-side diff: original case → theurge equivalent, flagging any gaps
- Run theurge complete suite, confirm all cases pass

**Exit criteria:** Zero gaps between original and theurge catalogs. All theurge cases green. Written parity report committed as evidence for the retirement pace. Any gaps discovered here become fix-paces slated before ₢A1AAJ.

### retire-bash-test-and-python-sortie-infrastructure (₢A1AAJ) [complete]

**[260402-1038] complete**

## Character
Cleanup and final sweep — lighter now that each porting pace handled its own retirement.

## Docket
With all cases ported and retired incrementally across ₢A1AAF–₢A1AAI, clean up the remaining framework scaffolding.

**Final inventory sweep** (replaces the former audit-coverage-parity pace):
- Enumerate theurge's complete case catalog from Rust source
- Confirm total count matches or exceeds the original combined total (bash 124 + python 11)
- Flag any lingering bash test files that weren't retired in their porting pace
- Run theurge complete suite, confirm all green

**Remove framework scaffolding:**
- `Tools/rbk/rbtb_testbench.sh` (RBK testbench router — should be empty/vestigial by now)
- Any remaining `Tools/rbk/rbts/*.sh` files not already retired
- Suite tabtargets (`rbw-ts`) — replace with theurge suite invocation or remove if theurge has its own
- Fixture tabtargets (`rbw-tf`) — same treatment
- Zipper enrollments for retired test colophons (`rbw-tf`, `rbw-ts`, `rbw-to`) if not already cleaned up

**Establish:**
- Minimal BUK self-test (`butt_testbench.sh` or similar) that exercises the BUK test framework itself (kick-tires level) so butX files stay tested after RBK consumers are gone

**Update:** CLAUDE.md prefix mappings to reflect `rbtd`/`rbid` trees and retired `rbtb`/`rbtid` entries. Update test execution table to reference theurge tiers instead of bash suite tabtargets.

**[260401-1358] rough**

## Character
Cleanup and final sweep — lighter now that each porting pace handled its own retirement.

## Docket
With all cases ported and retired incrementally across ₢A1AAF–₢A1AAI, clean up the remaining framework scaffolding.

**Final inventory sweep** (replaces the former audit-coverage-parity pace):
- Enumerate theurge's complete case catalog from Rust source
- Confirm total count matches or exceeds the original combined total (bash 124 + python 11)
- Flag any lingering bash test files that weren't retired in their porting pace
- Run theurge complete suite, confirm all green

**Remove framework scaffolding:**
- `Tools/rbk/rbtb_testbench.sh` (RBK testbench router — should be empty/vestigial by now)
- Any remaining `Tools/rbk/rbts/*.sh` files not already retired
- Suite tabtargets (`rbw-ts`) — replace with theurge suite invocation or remove if theurge has its own
- Fixture tabtargets (`rbw-tf`) — same treatment
- Zipper enrollments for retired test colophons (`rbw-tf`, `rbw-ts`, `rbw-to`) if not already cleaned up

**Establish:**
- Minimal BUK self-test (`butt_testbench.sh` or similar) that exercises the BUK test framework itself (kick-tires level) so butX files stay tested after RBK consumers are gone

**Update:** CLAUDE.md prefix mappings to reflect `rbtd`/`rbid` trees and retired `rbtb`/`rbtid` entries. Update test execution table to reference theurge tiers instead of bash suite tabtargets.

**[260401-1315] rough**

## Character
Pruning — satisfying but irreversible. Verify full coverage first.

## Docket
Remove the RBK-specific test consumers and python sortie framework. **Do NOT remove BUK test infrastructure files** (`bute_engine.sh`, `butd_dispatch.sh`, `butr_registry.sh`, `buto_operations.sh`) — those are BUK framework used by other projects.

**Remove:**
- `Tools/rbk/rbtb_testbench.sh` and `Tools/rbk/rbts/*.sh` (all RBK bash test cases)
- `Tools/rbk/rbtid/` (python adjutant, roster, all sorties, debrief)
- Test suite/fixture tabtargets that pointed to bash infrastructure
- Zipper enrollments for retired test colophons (`rbw-tf`, `rbw-ts`, `rbw-to`)
- Sortie-related zipper enrollments (`rbw-Is`)

**Establish:**
- Minimal BUK self-test (`butt_testbench.sh` or similar) that exercises the BUK test framework itself (kick-tires level) so butX files stay tested after RBK consumers are removed.

**Before removing:** run theurge full suite, confirm all ported cases pass, diff coverage against bash/python case lists to ensure nothing was dropped.

Update CLAUDE.md prefix mappings to reflect new `rbtd`/`rbid` trees and retired `rbtb`/`rbtid` entries.

**[260401-1239] rough**

## Character
Pruning — satisfying but irreversible. Verify full coverage first.

## Docket
Remove the RBK-specific test consumers and python sortie framework. **Do NOT remove BUK test infrastructure files** (`bute_engine.sh`, `butd_dispatch.sh`, `butr_registry.sh`, `buto_operations.sh`) — those are BUK framework used by other projects.

**Remove:**
- `Tools/rbk/rbtb_testbench.sh` and `Tools/rbk/rbts/*.sh` (all RBK bash test cases)
- `Tools/rbk/rbtid/` (python adjutant, roster, all sorties, debrief)
- Test suite/fixture tabtargets that pointed to bash infrastructure
- Zipper enrollments for retired test colophons (`rbw-tf`, `rbw-ts`, `rbw-to`)
- Sortie-related zipper enrollments (`rbw-Is`)

**Simplify but keep:**
- `rbev-vessels/rbev-bottle-ifrit/Dockerfile` — slim down to minimal base + network tools (no python, no nodejs, no Claude Code). The ifrit binary gets copied in at test time, not baked into the image.

**Establish:**
- Minimal BUK self-test (`butt_testbench.sh` or similar) that exercises the BUK test framework itself (kick-tires level) so butX files stay tested after RBK consumers are removed.

**Before removing:** run theurge full suite, confirm all ported cases pass, diff coverage against bash/python case lists to ensure nothing was dropped.

Update CLAUDE.md prefix mappings to reflect new `rbtr`/`rbid` trees and retired `rbtb`/`rbtid` entries.

**[260401-1203] rough**

## Character
Pruning — satisfying but irreversible. Verify full coverage first.

## Docket
Remove the bash test infrastructure and python sortie framework:
- `Tools/buk/bute_engine.sh`, `butd_dispatch.sh`, `butr_registry.sh`, `buto_operations.sh`
- `Tools/rbk/rbtb_testbench.sh` and `Tools/rbk/rbts/*.sh` (all bash test cases)
- `Tools/rbk/rbtid/` (python adjutant, roster, all sorties, debrief)
- `rbev-vessels/rbev-bottle-ifrit/Dockerfile` (rebuild as slim vessel for Rust ifrit)
- Test suite/fixture tabtargets that pointed to bash infrastructure
- Zipper enrollments for retired colophons

Before removing: run theurge suite, confirm all ported cases pass, diff coverage against bash/python case lists to ensure nothing was dropped.

Update CLAUDE.md prefix mappings to reflect new `rbtr` tree and retired `rbtb`/`rbtid` entries.

### deduplicate-ported-test-catalog (₢A1AAM) [complete]

**[260402-1041] complete**

## Character
Design judgment — now that all tests are ported faithfully, assess overlap and simplify.

## Docket
After all porting paces are complete and retirement is done, review the combined test catalog (ported bash cases + ported python sorties) for overlap. The bash tadmor DNS-blocked tests and the python dns_exfil_subdomain sortie test similar surfaces from different angles — some may collapse, some may be complementary.

Express deduplication pass:
- List all ifrit attack variants and theurge-only cases
- Identify overlapping coverage (same attack surface, same assertion)
- Merge where truly redundant, keep both where they test different aspects
- Document the final catalog

**[260401-1239] rough**

## Character
Design judgment — now that all tests are ported faithfully, assess overlap and simplify.

## Docket
After all porting paces are complete and retirement is done, review the combined test catalog (ported bash cases + ported python sorties) for overlap. The bash tadmor DNS-blocked tests and the python dns_exfil_subdomain sortie test similar surfaces from different angles — some may collapse, some may be complementary.

Express deduplication pass:
- List all ifrit attack variants and theurge-only cases
- Identify overlapping coverage (same attack surface, same assertion)
- Merge where truly redundant, keep both where they test different aspects
- Document the final catalog

### coordinated-arp-sentry-observation (₢A1AAS) [complete]

**[260402-1605] complete**

## Character
Architectural — first coordinated ifrit/theurge test pattern. Requires new theurge primitives (snapshot-attack-resnap) and judgment about where ARP hardening belongs (pentacle sysctl vs static entries vs container capabilities).

## Docket

Add a coordinated ARP poison test where theurge observes sentry state from outside while ifrit attacks from inside.

### New coordination primitive
- Theurge snapshots sentry ARP table via writ exec (`ip neigh show` on sentry container)
- Theurge triggers ifrit ARP attack via bark exec (attack selector on bottle)
- Theurge re-snapshots sentry ARP table
- Diff: verify sentry's ARP entry for bottle IP did not change MAC, no fake gateway entry appeared

### Test cases
- `coordinated-arp-gratuitous`: ifrit sends gratuitous ARP claiming sentry IP, verify sentry table unchanged
- `coordinated-arp-gateway-poison`: ifrit claims gateway IP at bottle MAC, verify sentry doesn't reroute
- `coordinated-arp-table-stability`: verify sentry ARP entries survive ifrit manipulation attempts

### Implementation
- New section in rbtdrc_crucible.rs: `tadmor-coordinated-attacks`
- New ifrit attack variants if needed, or reuse existing DirectArpPoison with theurge-side observation
- May require pentacle hardening (static ARP, arp_filter/arp_ignore sysctls) to make tests pass — document decisions

### Acceptance
- All new coordinated cases pass with SECURE verdicts
- Existing 34 tadmor cases unaffected
- Full fast + crucible suites green

**[260402-1508] rough**

## Character
Architectural — first coordinated ifrit/theurge test pattern. Requires new theurge primitives (snapshot-attack-resnap) and judgment about where ARP hardening belongs (pentacle sysctl vs static entries vs container capabilities).

## Docket

Add a coordinated ARP poison test where theurge observes sentry state from outside while ifrit attacks from inside.

### New coordination primitive
- Theurge snapshots sentry ARP table via writ exec (`ip neigh show` on sentry container)
- Theurge triggers ifrit ARP attack via bark exec (attack selector on bottle)
- Theurge re-snapshots sentry ARP table
- Diff: verify sentry's ARP entry for bottle IP did not change MAC, no fake gateway entry appeared

### Test cases
- `coordinated-arp-gratuitous`: ifrit sends gratuitous ARP claiming sentry IP, verify sentry table unchanged
- `coordinated-arp-gateway-poison`: ifrit claims gateway IP at bottle MAC, verify sentry doesn't reroute
- `coordinated-arp-table-stability`: verify sentry ARP entries survive ifrit manipulation attempts

### Implementation
- New section in rbtdrc_crucible.rs: `tadmor-coordinated-attacks`
- New ifrit attack variants if needed, or reuse existing DirectArpPoison with theurge-side observation
- May require pentacle hardening (static ARP, arp_filter/arp_ignore sysctls) to make tests pass — document decisions

### Acceptance
- All new coordinated cases pass with SECURE verdicts
- Existing 34 tadmor cases unaffected
- Full fast + crucible suites green

### single-case-runner-and-macro-refactor (₢A1AAU) [complete]

**[260402-1815] complete**

## Character
Infrastructure refinement — mechanical refactor of 50 case registrations plus new engine capability. The macro design is settled; execution is methodical.

## Docket

Unify case naming: one name per case (the function name), compiler-enforced uniqueness, plus a single-case runner tabtarget for crucible debugging.

### Macro refactor
- Create `case!()` macro using `stringify!()` to derive case name from function name
- Replace all 50 `rbtdre_Case { name: "...", func: ... }` registrations with `case!(rbtdrc_fn_name)`
- Add display-time transform in engine: strip `rbtdrc_` prefix, convert `_` to `-` for output
- Remove duplicate name strings — compiler enforces uniqueness via function names

### Single-case runner
- New tabtarget: `tt/rbtd-s.SingleCase.{fixture}.sh [case]`
- New theurge subcommand: `rbtd single [case]` — no charge/quench lifecycle
- Shell wrapper checks crucible is charged; if not, `buc_die` with charge instruction
- No case arg → list all cases for that fixture
- Case arg → run that one case against the already-charged crucible
- Case lookup by name across all fixtures (names are globally unique)

### CLAUDE.md updates
- Add single-case runner to test execution table
- Document workflow: charge → run single cases → quench

### Acceptance
- All 50 cases use macro registration
- `tt/rbtd-s.SingleCase.tadmor.sh` lists cases when no arg given
- `tt/rbtd-s.SingleCase.tadmor.sh coordinated-arp-gratuitous` runs one case
- Existing suite tabtargets unaffected
- All tests pass

**[260402-1604] rough**

## Character
Infrastructure refinement — mechanical refactor of 50 case registrations plus new engine capability. The macro design is settled; execution is methodical.

## Docket

Unify case naming: one name per case (the function name), compiler-enforced uniqueness, plus a single-case runner tabtarget for crucible debugging.

### Macro refactor
- Create `case!()` macro using `stringify!()` to derive case name from function name
- Replace all 50 `rbtdre_Case { name: "...", func: ... }` registrations with `case!(rbtdrc_fn_name)`
- Add display-time transform in engine: strip `rbtdrc_` prefix, convert `_` to `-` for output
- Remove duplicate name strings — compiler enforces uniqueness via function names

### Single-case runner
- New tabtarget: `tt/rbtd-s.SingleCase.{fixture}.sh [case]`
- New theurge subcommand: `rbtd single [case]` — no charge/quench lifecycle
- Shell wrapper checks crucible is charged; if not, `buc_die` with charge instruction
- No case arg → list all cases for that fixture
- Case arg → run that one case against the already-charged crucible
- Case lookup by name across all fixtures (names are globally unique)

### CLAUDE.md updates
- Add single-case runner to test execution table
- Document workflow: charge → run single cases → quench

### Acceptance
- All 50 cases use macro registration
- `tt/rbtd-s.SingleCase.tadmor.sh` lists cases when no arg given
- `tt/rbtd-s.SingleCase.tadmor.sh coordinated-arp-gratuitous` runs one case
- Existing suite tabtargets unaffected
- All tests pass

### crucible-active-lifecycle-verification (₢A1AAW) [complete]

**[260402-1842] complete**

## Character
Bug fix plus defensive assertion — mechanical once the quench bug is identified, but requires understanding the compose profile interaction.

## Docket

### Bug: bottle survives quench
`rbob_quench` calls `compose down --remove-orphans` without `--profile sessile`. The bottle container (started under the sessile profile) is not stopped. Fix `rbob_quench` to include `--profile sessile` so all containers are torn down.

### Lifecycle assertions in crucible fixtures
Add `rbw-ca` CrucibleActive checks to the existing crucible fixture flow in theurge:
- After charge succeeds, invoke `rbw-ca` and assert exit 0 (charged)
- After quench succeeds, invoke `rbw-ca` and assert exit non-zero (not charged)

This verifies that charge/quench actually achieve their intended state transitions, catching regressions like the sessile profile omission.

### Acceptance
- `rbob_quench` stops all containers including bottle
- After charge: `rbw-ca` returns 0 for tadmor, srjcl, pluml
- After quench: `rbw-ca` returns non-zero for tadmor, srjcl, pluml
- Existing crucible suite (tadmor) passes end-to-end

**[260402-1812] rough**

## Character
Bug fix plus defensive assertion — mechanical once the quench bug is identified, but requires understanding the compose profile interaction.

## Docket

### Bug: bottle survives quench
`rbob_quench` calls `compose down --remove-orphans` without `--profile sessile`. The bottle container (started under the sessile profile) is not stopped. Fix `rbob_quench` to include `--profile sessile` so all containers are torn down.

### Lifecycle assertions in crucible fixtures
Add `rbw-ca` CrucibleActive checks to the existing crucible fixture flow in theurge:
- After charge succeeds, invoke `rbw-ca` and assert exit 0 (charged)
- After quench succeeds, invoke `rbw-ca` and assert exit non-zero (not charged)

This verifies that charge/quench actually achieve their intended state transitions, catching regressions like the sessile profile omission.

### Acceptance
- `rbob_quench` stops all containers including bottle
- After charge: `rbw-ca` returns 0 for tadmor, srjcl, pluml
- After quench: `rbw-ca` returns non-zero for tadmor, srjcl, pluml
- Existing crucible suite (tadmor) passes end-to-end

### reimplement-srjcl-websocket-kernel-test (₢A1AAa) [complete]

**[260402-2335] complete**

## Character
Research and design — need to understand what the old Python test actually verified, then determine the right Rust implementation strategy. WebSocket protocol interaction from a compiled binary is non-trivial; may need a crate dependency (tungstenite or similar) which means a tethered kludge for the ifrit vessel.

## Docket

The `rbtdrc_srjcl_websocket_kernel` theurge case fails because it still shells out to `Tools/rbk/rbts/rbt_test_srjcl.py`, which was deleted in pace J (retire-bash-test-and-python-sortie-infrastructure). The case needs to be reimplemented in pure Rust.

### Research Phase
- Recover the deleted `rbt_test_srjcl.py` from git history — understand exactly what it tested
- Document what Jupyter WebSocket kernel interaction involves (protocol, endpoints, message format)
- Determine whether the test verifies kernel availability, code execution, or both
- Check what the other two passing srjcl cases (`_running`, `_connectivity`) already cover

### Design Phase
- Evaluate Rust WebSocket options: raw TCP with HTTP upgrade vs crate (tungstenite, tokio-tungstenite)
- If a crate is needed, assess ifrit vessel impact (new dependency = tethered kludge, layer cache invalidation)
- Consider whether the test belongs in ifrit (inside bottle) or theurge (from host via fiat/bark)
- Alternative: could a simpler HTTP-based check achieve equivalent coverage without full WebSocket?

### Implementation
- Build the Rust replacement
- If ifrit dependency added, rebuild vessel
- Verify all 3 srjcl cases pass

### Acceptance
- `rbtdrc_srjcl_websocket_kernel` passes
- Full srjcl fixture: 3 pass, 0 fail
- No new crate dependencies added without explicit justification

**[260402-2312] rough**

## Character
Research and design — need to understand what the old Python test actually verified, then determine the right Rust implementation strategy. WebSocket protocol interaction from a compiled binary is non-trivial; may need a crate dependency (tungstenite or similar) which means a tethered kludge for the ifrit vessel.

## Docket

The `rbtdrc_srjcl_websocket_kernel` theurge case fails because it still shells out to `Tools/rbk/rbts/rbt_test_srjcl.py`, which was deleted in pace J (retire-bash-test-and-python-sortie-infrastructure). The case needs to be reimplemented in pure Rust.

### Research Phase
- Recover the deleted `rbt_test_srjcl.py` from git history — understand exactly what it tested
- Document what Jupyter WebSocket kernel interaction involves (protocol, endpoints, message format)
- Determine whether the test verifies kernel availability, code execution, or both
- Check what the other two passing srjcl cases (`_running`, `_connectivity`) already cover

### Design Phase
- Evaluate Rust WebSocket options: raw TCP with HTTP upgrade vs crate (tungstenite, tokio-tungstenite)
- If a crate is needed, assess ifrit vessel impact (new dependency = tethered kludge, layer cache invalidation)
- Consider whether the test belongs in ifrit (inside bottle) or theurge (from host via fiat/bark)
- Alternative: could a simpler HTTP-based check achieve equivalent coverage without full WebSocket?

### Implementation
- Build the Rust replacement
- If ifrit dependency added, rebuild vessel
- Verify all 3 srjcl cases pass

### Acceptance
- `rbtdrc_srjcl_websocket_kernel` passes
- Full srjcl fixture: 3 pass, 0 fail
- No new crate dependencies added without explicit justification

### decompose-theurge-workbench-to-bcg (₢A1AAX) [complete]

**[260402-2346] complete**

## Character
Mechanical refactoring with clear before/after — every case arm in the existing workbench becomes a named BCG function. No new behavior, just structural decomposition.

## Docket

Extract theurge workbench inline logic into proper BCG module `rbte` (Test Engine).

### Files to create
- `Tools/rbk/rbtd/rbte_engine.sh` — implementation module (kindle, sentinel, public functions)
- `Tools/rbk/rbtd/rbte_cli.sh` — CLI entry point with differential furnish

### Functions to extract (from rbtw_workbench.sh case arms)
- `rbte_build` — cargo build for theurge crate (from `rbtd-b`)
- `rbte_test` — cargo test for theurge crate (from `rbtd-t`)
- `rbte_run` — build + invoke binary for single fixture (from `rbtd-r`)
- `rbte_suite` — build + invoke binary for suite of fixtures (from `rbtd-s` TestSuite)
- `rbte_single` — build + invoke binary for single case (from `rbtd-s` SingleCase)
- `rbte_probe` — access probe with heavy kindle chain (from `rbtd-ap`)

### Kindle owns
- Cargo manifest path (`RBTE_MANIFEST`)
- Theurge binary path (`ZRBTE_BINARY`)
- Suite-to-fixture mappings (fast, service, crucible, complete)

### Zipper manifest plumbing
- `rbte_cli.sh` furnish sources and kindles buz + rbz zippers
- `ZRBZ_COLOPHON_MANIFEST` passed directly to theurge binary
- Delete `zrbtw_resolve_manifest` entirely — Rust `rbtdrm_required_colophons` is sole source of truth for per-fixture requirements

### Differential furnish
- Light path (build/test/run/suite/single): just rbte kindle
- Heavy path (probe): regime + constants + OAuth + IAM + Payor kindle chain

### Drift fix
- Update `RBTDRM_COLOPHON_KLUDGE` in `rbtdrm_manifest.rs` from `"rbw-ak"` to `"rbw-LK"`

### rbtw_workbench.sh becomes pure router
- Routes colophons to `rbte_cli.sh` commands
- No inline logic, no fixture knowledge, no kindle chains

### Verification
- `tt/rbtd-t.Test.sh` passes (theurge unit tests)
- `tt/rbtd-s.TestSuite.fast.sh` passes (fast suite)
- `tt/rbw-Qf.QualifyFast.sh` passes (colophon health)

**[260402-1818] rough**

## Character
Mechanical refactoring with clear before/after — every case arm in the existing workbench becomes a named BCG function. No new behavior, just structural decomposition.

## Docket

Extract theurge workbench inline logic into proper BCG module `rbte` (Test Engine).

### Files to create
- `Tools/rbk/rbtd/rbte_engine.sh` — implementation module (kindle, sentinel, public functions)
- `Tools/rbk/rbtd/rbte_cli.sh` — CLI entry point with differential furnish

### Functions to extract (from rbtw_workbench.sh case arms)
- `rbte_build` — cargo build for theurge crate (from `rbtd-b`)
- `rbte_test` — cargo test for theurge crate (from `rbtd-t`)
- `rbte_run` — build + invoke binary for single fixture (from `rbtd-r`)
- `rbte_suite` — build + invoke binary for suite of fixtures (from `rbtd-s` TestSuite)
- `rbte_single` — build + invoke binary for single case (from `rbtd-s` SingleCase)
- `rbte_probe` — access probe with heavy kindle chain (from `rbtd-ap`)

### Kindle owns
- Cargo manifest path (`RBTE_MANIFEST`)
- Theurge binary path (`ZRBTE_BINARY`)
- Suite-to-fixture mappings (fast, service, crucible, complete)

### Zipper manifest plumbing
- `rbte_cli.sh` furnish sources and kindles buz + rbz zippers
- `ZRBZ_COLOPHON_MANIFEST` passed directly to theurge binary
- Delete `zrbtw_resolve_manifest` entirely — Rust `rbtdrm_required_colophons` is sole source of truth for per-fixture requirements

### Differential furnish
- Light path (build/test/run/suite/single): just rbte kindle
- Heavy path (probe): regime + constants + OAuth + IAM + Payor kindle chain

### Drift fix
- Update `RBTDRM_COLOPHON_KLUDGE` in `rbtdrm_manifest.rs` from `"rbw-ak"` to `"rbw-LK"`

### rbtw_workbench.sh becomes pure router
- Routes colophons to `rbte_cli.sh` commands
- No inline logic, no fixture knowledge, no kindle chains

### Verification
- `tt/rbtd-t.Test.sh` passes (theurge unit tests)
- `tt/rbtd-s.TestSuite.fast.sh` passes (fast suite)
- `tt/rbw-Qf.QualifyFast.sh` passes (colophon health)

### rename-crucible-active-to-crucible-is-charged (₢A1AAY) [abandoned]

**[260402-2340] abandoned**

## Character
Mechanical rename — grep-and-replace across shell, Rust, and tabtargets. No behavioral change.

## Docket

Rename colophon `rbw-ca` (CrucibleActive) to `rbw-cic` (CrucibleIsCharged) throughout.

### Touch points
- Mint `rbw-cic` colophon, retire `rbw-ca`
- Tabtarget: delete `tt/rbw-ca.CrucibleActive.sh`, create `tt/rbw-cic.CrucibleIsCharged.sh`
- Zipper enrollment in `rbz_zipper.sh`: rename constant (`RBZ_CRUCIBLE_IS_CHARGED`), update colophon string
- Theurge colophon manifest in `rbtdrm_manifest.rs`: update compiled-in colophon string
- Workbench colophon list in `rbtw_workbench.sh` (or `rbte` if decomposition landed)
- CLAUDE.md tabtarget reference

### Acceptance
- `tt/rbw-cic.CrucibleIsCharged.sh` returns correct predicate for charged/uncharged crucible
- `tt/rbw-Qf.QualifyFast.sh` passes (colophon health)
- `tt/rbtd-t.Test.sh` passes (theurge unit tests)
- No references to `rbw-ca` remain

**[260402-2222] rough**

## Character
Mechanical rename — grep-and-replace across shell, Rust, and tabtargets. No behavioral change.

## Docket

Rename colophon `rbw-ca` (CrucibleActive) to `rbw-cic` (CrucibleIsCharged) throughout.

### Touch points
- Mint `rbw-cic` colophon, retire `rbw-ca`
- Tabtarget: delete `tt/rbw-ca.CrucibleActive.sh`, create `tt/rbw-cic.CrucibleIsCharged.sh`
- Zipper enrollment in `rbz_zipper.sh`: rename constant (`RBZ_CRUCIBLE_IS_CHARGED`), update colophon string
- Theurge colophon manifest in `rbtdrm_manifest.rs`: update compiled-in colophon string
- Workbench colophon list in `rbtw_workbench.sh` (or `rbte` if decomposition landed)
- CLAUDE.md tabtarget reference

### Acceptance
- `tt/rbw-cic.CrucibleIsCharged.sh` returns correct predicate for charged/uncharged crucible
- `tt/rbw-Qf.QualifyFast.sh` passes (colophon health)
- `tt/rbtd-t.Test.sh` passes (theurge unit tests)
- No references to `rbw-ca` remain

### tectonic-colophon-reorganization (₢A1AAb) [complete]

**[260403-1026] complete**

## Character
Intricate but mechanical — large surface area of renames across zipper, tabtargets, and frontispieces, but each individual change is straightforward. Requires care to avoid breaking dispatch routing. Design decisions already made in conversation.

## Docket
Reorganize the `rbw` workbench colophon namespace from actor-organized (director/retriever/ledger) to domain-object-organized categories. This is a rename-only transformation — no new functionality, no behavior changes.

**Prerequisite**: ₢A1AAf (add-buz-group-to-buk-zipper), ₢A1AAe (rename-consecration-to-hallmark) must be complete first.

### Complete old→new colophon map

Accounts (rbw-a) — Google Cloud service accounts:

```
rbw-PM   PayorMantlesGovernor            →  rbw-aM   PayorMantlesGovernor
rbw-GC   GovernorChartersRetriever       →  rbw-aC   GovernorChartersRetriever
rbw-GK   GovernorKnightsDirector         →  rbw-aK   GovernorKnightsDirector
rbw-Gl   GovernorListsServiceAccounts    →  rbw-aL   GovernorListsServiceAccounts
rbw-GF   GovernorForfeitsServiceAccount  →  rbw-aF   GovernorForfeitsServiceAccount
```

Crucible (rbw-c) — container runtime (only the predicate rename):

```
rbw-ca   CrucibleActive                  →  rbw-cic  CrucibleIsCharged
```

Depot (rbw-d) — GCP project infrastructure and supply chain (UPPER=mutates GAR/GCP, lower=read only):

```
rbw-PL   PayorLeviesDepot                →  rbw-dL   PayorLeviesDepot
rbw-PU   PayorUnmakesDepot               →  rbw-dU   PayorUnmakesDepot
rbw-DE   DirectorEnshrinesVessel         →  rbw-dE   DirectorEnshrinesVessel
rbw-DI   DirectorInscribesReliquary      →  rbw-dI   DirectorInscribesReliquary
rbw-Pl   PayorListsDepots                →  rbw-dl   PayorListsDepots
```

Hallmark (rbw-h) — registry artifact lifecycle (UPPER=mutates GAR, lower=read/local):

```
rbw-DO   DirectorOrdainsHallmark         →  rbw-hO   DirectorOrdainsHallmark
rbw-DA   DirectorAbjuresHallmark         →  rbw-hA   DirectorAbjuresHallmark
rbw-DV   DirectorVouchesHallmarks        →  rbw-hV   DirectorVouchesHallmarks
rbw-Dt   DirectorTalliesHallmarks        →  rbw-ht   DirectorTalliesHallmarks
rbw-Rs   RetrieverSummonsHallmark        →  rbw-hs   RetrieverSummonsHallmark
rbw-LK   RetrieverKludgesHallmark        →  rbw-hk   RetrieverKludgesHallmark
rbw-RpF  RetrieverPlumbsFull             →  rbw-hpf  RetrieverPlumbsFull
rbw-Rpc  RetrieverPlumbsCompact          →  rbw-hpc  RetrieverPlumbsCompact
```

Image (rbw-i) — container image operations:

```
rbw-DJ   DirectorJettisonsImage          →  rbw-iJ   DirectorJettisonsImage
rbw-Rw   RetrieverWrestsImage            →  rbw-iw   RetrieverWrestsImage
```

Theurge (rbw-t) — test infrastructure:

```
rbw-Tk   TheurgeKludgesHallmark          →  rbw-tK   TheurgeKludgesHallmark
rbw-To   TheurgeOrdainsHallmark          →  rbw-tO   TheurgeOrdainsHallmark
rbw-Qf   QualifyFast                     →  rbw-tf   QualifyFast
rbw-QR   QualifyRelease                  →  rbw-tr   QualifyRelease
```

### Unchanged categories (no colophon renames needed)

- `rbw-c` Crucible — all except `ca` → `cic` stay as-is
- `rbw-g` Guide — human-directed procedures, unchanged
- `rbw-I` Ifrit — unchanged
- `rbw-M` Marshal — lifecycle, unchanged
- `rbw-r` Regime — config files, unchanged
- `rbw-n` Nameplate cross-cutting — unchanged

### Work items
- Update `rbz_zipper.sh` colophon strings and `buz_group` declarations to domain-organized categories
- Rename tabtarget files in `tt/` to match new colophons
- Update theurge colophon manifest (`rbtdrm_manifest.rs`)
- **Build after Rust changes**: run `tt/rbtd-b.Build.sh` after modifying manifest, before running tests
- Update `CLAUDE.consumer.md` colophon reference table
- Verify dispatch routing with smoke test

### Verification
- `tt/rbtd-b.Build.sh` succeeds (theurge build)
- `tt/rbw-cic.CrucibleIsCharged.sh` returns correct predicate
- `tt/rbw-tf.QualifyFast.sh` passes (colophon health)
- `tt/rbtd-t.Test.sh` passes (theurge unit tests)
- `tt/rbtd-s.TestSuite.fast.sh` passes (75 cases)
- No references to old colophons remain

### Acceptance
- All renamed tabtargets dispatch correctly
- Full fast suite passes
- Colophon qualify clean

**[260403-0935] rough**

## Character
Intricate but mechanical — large surface area of renames across zipper, tabtargets, and frontispieces, but each individual change is straightforward. Requires care to avoid breaking dispatch routing. Design decisions already made in conversation.

## Docket
Reorganize the `rbw` workbench colophon namespace from actor-organized (director/retriever/ledger) to domain-object-organized categories. This is a rename-only transformation — no new functionality, no behavior changes.

**Prerequisite**: ₢A1AAf (add-buz-group-to-buk-zipper), ₢A1AAe (rename-consecration-to-hallmark) must be complete first.

### Complete old→new colophon map

Accounts (rbw-a) — Google Cloud service accounts:

```
rbw-PM   PayorMantlesGovernor            →  rbw-aM   PayorMantlesGovernor
rbw-GC   GovernorChartersRetriever       →  rbw-aC   GovernorChartersRetriever
rbw-GK   GovernorKnightsDirector         →  rbw-aK   GovernorKnightsDirector
rbw-Gl   GovernorListsServiceAccounts    →  rbw-aL   GovernorListsServiceAccounts
rbw-GF   GovernorForfeitsServiceAccount  →  rbw-aF   GovernorForfeitsServiceAccount
```

Crucible (rbw-c) — container runtime (only the predicate rename):

```
rbw-ca   CrucibleActive                  →  rbw-cic  CrucibleIsCharged
```

Depot (rbw-d) — GCP project infrastructure and supply chain (UPPER=mutates GAR/GCP, lower=read only):

```
rbw-PL   PayorLeviesDepot                →  rbw-dL   PayorLeviesDepot
rbw-PU   PayorUnmakesDepot               →  rbw-dU   PayorUnmakesDepot
rbw-DE   DirectorEnshrinesVessel         →  rbw-dE   DirectorEnshrinesVessel
rbw-DI   DirectorInscribesReliquary      →  rbw-dI   DirectorInscribesReliquary
rbw-Pl   PayorListsDepots                →  rbw-dl   PayorListsDepots
```

Hallmark (rbw-h) — registry artifact lifecycle (UPPER=mutates GAR, lower=read/local):

```
rbw-DO   DirectorOrdainsHallmark         →  rbw-hO   DirectorOrdainsHallmark
rbw-DA   DirectorAbjuresHallmark         →  rbw-hA   DirectorAbjuresHallmark
rbw-DV   DirectorVouchesHallmarks        →  rbw-hV   DirectorVouchesHallmarks
rbw-Dt   DirectorTalliesHallmarks        →  rbw-ht   DirectorTalliesHallmarks
rbw-Rs   RetrieverSummonsHallmark        →  rbw-hs   RetrieverSummonsHallmark
rbw-LK   RetrieverKludgesHallmark        →  rbw-hk   RetrieverKludgesHallmark
rbw-RpF  RetrieverPlumbsFull             →  rbw-hpf  RetrieverPlumbsFull
rbw-Rpc  RetrieverPlumbsCompact          →  rbw-hpc  RetrieverPlumbsCompact
```

Image (rbw-i) — container image operations:

```
rbw-DJ   DirectorJettisonsImage          →  rbw-iJ   DirectorJettisonsImage
rbw-Rw   RetrieverWrestsImage            →  rbw-iw   RetrieverWrestsImage
```

Theurge (rbw-t) — test infrastructure:

```
rbw-Tk   TheurgeKludgesHallmark          →  rbw-tK   TheurgeKludgesHallmark
rbw-To   TheurgeOrdainsHallmark          →  rbw-tO   TheurgeOrdainsHallmark
rbw-Qf   QualifyFast                     →  rbw-tf   QualifyFast
rbw-QR   QualifyRelease                  →  rbw-tr   QualifyRelease
```

### Unchanged categories (no colophon renames needed)

- `rbw-c` Crucible — all except `ca` → `cic` stay as-is
- `rbw-g` Guide — human-directed procedures, unchanged
- `rbw-I` Ifrit — unchanged
- `rbw-M` Marshal — lifecycle, unchanged
- `rbw-r` Regime — config files, unchanged
- `rbw-n` Nameplate cross-cutting — unchanged

### Work items
- Update `rbz_zipper.sh` colophon strings and `buz_group` declarations to domain-organized categories
- Rename tabtarget files in `tt/` to match new colophons
- Update theurge colophon manifest (`rbtdrm_manifest.rs`)
- **Build after Rust changes**: run `tt/rbtd-b.Build.sh` after modifying manifest, before running tests
- Update `CLAUDE.consumer.md` colophon reference table
- Verify dispatch routing with smoke test

### Verification
- `tt/rbtd-b.Build.sh` succeeds (theurge build)
- `tt/rbw-cic.CrucibleIsCharged.sh` returns correct predicate
- `tt/rbw-tf.QualifyFast.sh` passes (colophon health)
- `tt/rbtd-t.Test.sh` passes (theurge unit tests)
- `tt/rbtd-s.TestSuite.fast.sh` passes (75 cases)
- No references to old colophons remain

### Acceptance
- All renamed tabtargets dispatch correctly
- Full fast suite passes
- Colophon qualify clean

**[260403-0930] rough**

## Character
Intricate but mechanical — large surface area of renames across zipper, tabtargets, and frontispieces, but each individual change is straightforward. Requires care to avoid breaking dispatch routing. Design decisions already made in conversation.

## Docket
Reorganize the `rbw` workbench colophon namespace from actor-organized (director/retriever/ledger) to domain-object-organized categories. This is a rename-only transformation — no new functionality, no behavior changes.

**Prerequisite**: ₢A1AAf (add-buz-group-to-buk-zipper), ₢A1AAe (rename-consecration-to-hallmark) must be complete first.

### Complete old→new colophon map

Accounts (rbw-a) — Google Cloud service accounts:

```
rbw-PM   PayorMantlesGovernor            →  rbw-aM   PayorMantlesGovernor
rbw-GC   GovernorChartersRetriever       →  rbw-aC   GovernorChartersRetriever
rbw-GK   GovernorKnightsDirector         →  rbw-aK   GovernorKnightsDirector
rbw-Gl   GovernorListsServiceAccounts    →  rbw-aL   GovernorListsServiceAccounts
rbw-GF   GovernorForfeitsServiceAccount  →  rbw-aF   GovernorForfeitsServiceAccount
```

Crucible (rbw-c) — container runtime (only the predicate rename):

```
rbw-ca   CrucibleActive                  →  rbw-cic  CrucibleIsCharged
```

Depot (rbw-d) — GCP project infrastructure and supply chain (UPPER=mutates GAR/GCP, lower=read only):

```
rbw-PL   PayorLeviesDepot                →  rbw-dL   PayorLeviesDepot
rbw-PU   PayorUnmakesDepot               →  rbw-dU   PayorUnmakesDepot
rbw-DE   DirectorEnshrinesVessel         →  rbw-dE   DirectorEnshrinesVessel
rbw-DI   DirectorInscribesReliquary      →  rbw-dI   DirectorInscribesReliquary
rbw-Pl   PayorListsDepots                →  rbw-dl   PayorListsDepots
```

Hallmark (rbw-h) — registry artifact lifecycle (UPPER=mutates GAR, lower=read/local):

```
rbw-DO   DirectorOrdainsHallmark         →  rbw-hO   DirectorOrdainsHallmark
rbw-DA   DirectorAbjuresHallmark         →  rbw-hA   DirectorAbjuresHallmark
rbw-DV   DirectorVouchesHallmarks        →  rbw-hV   DirectorVouchesHallmarks
rbw-Dt   DirectorTalliesHallmarks        →  rbw-ht   DirectorTalliesHallmarks
rbw-Rs   RetrieverSummonsHallmark        →  rbw-hs   RetrieverSummonsHallmark
rbw-LK   RetrieverKludgesHallmark        →  rbw-hk   RetrieverKludgesHallmark
rbw-RpF  RetrieverPlumbsFull             →  rbw-hpf  RetrieverPlumbsFull
rbw-Rpc  RetrieverPlumbsCompact          →  rbw-hpc  RetrieverPlumbsCompact
```

Image (rbw-i) — container image operations:

```
rbw-DJ   DirectorJettisonsImage          →  rbw-iJ   DirectorJettisonsImage
rbw-Rw   RetrieverWrestsImage            →  rbw-iw   RetrieverWrestsImage
```

Theurge (rbw-t) — test infrastructure:

```
rbw-Tk   TheurgeKludgesHallmark          →  rbw-tK   TheurgeKludgesHallmark
rbw-To   TheurgeOrdainsHallmark          →  rbw-tO   TheurgeOrdainsHallmark
rbw-Qf   QualifyFast                     →  rbw-tf   QualifyFast
rbw-QR   QualifyRelease                  →  rbw-tr   QualifyRelease
```

### Unchanged categories (no colophon renames needed)

- `rbw-c` Crucible — all except `ca` → `cic` stay as-is
- `rbw-g` Guide — human-directed procedures, unchanged
- `rbw-I` Ifrit — unchanged
- `rbw-M` Marshal — lifecycle, unchanged
- `rbw-r` Regime — config files, unchanged
- `rbw-n` Nameplate cross-cutting — unchanged

### Work items
- Update `rbz_zipper.sh` colophon strings and `buz_group` declarations to domain-organized categories
- Rename tabtarget files in `tt/` to match new colophons
- Update theurge colophon manifest (`rbtdrm_manifest.rs`)
- Update `CLAUDE.consumer.md` colophon reference table
- Verify dispatch routing with smoke test

### Verification
- `tt/rbw-cic.CrucibleIsCharged.sh` returns correct predicate
- `tt/rbw-tf.QualifyFast.sh` passes (colophon health)
- `tt/rbtd-t.Test.sh` passes (theurge unit tests)
- No references to old colophons remain

### Acceptance
- All renamed tabtargets dispatch correctly
- Full fast suite passes
- Colophon qualify clean

**[260403-0912] rough**

## Character
Intricate but mechanical — large surface area of renames across zipper, tabtargets, and frontispieces, but each individual change is straightforward. Requires care to avoid breaking dispatch routing. Design decisions already made in conversation.

## Docket
Reorganize the `rbw` workbench colophon namespace from actor-organized (director/retriever/ledger) to domain-object-organized categories. This is a rename-only transformation — no new functionality, no behavior changes.

**Prerequisite**: ₢A1AAf (add-buz-group-to-buk-zipper), ₢A1AAe (rename-consecration-to-hallmark) must be complete first.

### Complete old→new colophon map

Accounts (rbw-a) — Google Cloud service accounts:

```
rbw-PM   PayorMantlesGovernor            →  rbw-aM   PayorMantlesGovernor
rbw-GC   GovernorChartersRetriever       →  rbw-aC   GovernorChartersRetriever
rbw-GK   GovernorKnightsDirector         →  rbw-aK   GovernorKnightsDirector
rbw-Gl   GovernorListsServiceAccounts    →  rbw-aL   GovernorListsServiceAccounts
rbw-GF   GovernorForfeitsServiceAccount  →  rbw-aF   GovernorForfeitsServiceAccount
```

Crucible (rbw-c) — container runtime (only the predicate rename):

```
rbw-ca   CrucibleActive                  →  rbw-cic  CrucibleIsCharged
```

Depot (rbw-d) — GCP project infrastructure and supply chain (UPPER=mutates GAR/GCP, lower=read only):

```
rbw-PL   PayorLeviesDepot                →  rbw-dL   PayorLeviesDepot
rbw-PU   PayorUnmakesDepot               →  rbw-dU   PayorUnmakesDepot
rbw-DE   DirectorEnshrinesVessel         →  rbw-dE   DirectorEnshrinesVessel
rbw-DI   DirectorInscribesReliquary      →  rbw-dI   DirectorInscribesReliquary
rbw-Pl   PayorListsDepots                →  rbw-dl   PayorListsDepots
```

Hallmark (rbw-h) — registry artifact lifecycle (UPPER=mutates GAR, lower=read/local):

```
rbw-DO   DirectorOrdainsConsecration     →  rbw-hO   DirectorOrdainsHallmark
rbw-DA   DirectorAbjuresConsecration     →  rbw-hA   DirectorAbjuresHallmark
rbw-DV   DirectorVouchesConsecrations    →  rbw-hV   DirectorVouchesHallmarks
rbw-Dt   DirectorTalliesConsecrations    →  rbw-ht   DirectorTalliesHallmarks
rbw-Rs   RetrieverSummonsConsecration    →  rbw-hs   RetrieverSummonsHallmark
rbw-LK   LedgerKludgesVessel             →  rbw-hk   RetrieverKludgesHallmark
rbw-RpF  RetrieverPlumbsFull             →  rbw-hpf  RetrieverPlumbsFull
rbw-Rpc  RetrieverPlumbsCompact          →  rbw-hpc  RetrieverPlumbsCompact
```

Image (rbw-i) — container image operations:

```
rbw-DJ   DirectorJettisonsImage          →  rbw-iJ   DirectorJettisonsImage
rbw-Rw   RetrieverWrestsImage            →  rbw-iw   RetrieverWrestsImage
```

Theurge (rbw-t) — test infrastructure:

```
rbw-Tk   TheurgeKludgesVessel            →  rbw-tK   TheurgeKludgesHallmark
rbw-To   TheurgeOrdainsVessel            →  rbw-tO   TheurgeOrdainsHallmark
rbw-Qf   QualifyFast                     →  rbw-tf   QualifyFast
rbw-QR   QualifyRelease                  →  rbw-tr   QualifyRelease
```

### Unchanged categories (no colophon renames needed)

- `rbw-c` Crucible — all except `ca` → `cic` stay as-is
- `rbw-g` Guide — human-directed procedures, unchanged
- `rbw-I` Ifrit — unchanged
- `rbw-M` Marshal — lifecycle, unchanged
- `rbw-r` Regime — config files, unchanged
- `rbw-n` Nameplate cross-cutting — unchanged

### Work items
- Update `rbz_zipper.sh` colophon strings and buz_group declarations for all affected enrollments
- Rename tabtarget files in `tt/` to match new colophons
- Update theurge colophon manifest (`rbtdrm_manifest.rs`)
- Verify dispatch routing with smoke test

### Verification
- `tt/rbw-cic.CrucibleIsCharged.sh` returns correct predicate
- `tt/rbw-tf.QualifyFast.sh` passes (colophon health)
- `tt/rbtd-t.Test.sh` passes (theurge unit tests)
- No references to old colophons remain

### Acceptance
- All renamed tabtargets dispatch correctly
- Full fast suite passes
- Colophon qualify clean

**[260403-0906] rough**

## Character
Intricate but mechanical — large surface area of renames across zipper, tabtargets, and frontispieces, but each individual change is straightforward. Requires care to avoid breaking dispatch routing. Design decisions already made in conversation.

## Docket
Reorganize the `rbw` workbench colophon namespace from actor-organized (director/retriever/ledger) to domain-object-organized categories. This is a rename-only transformation — no new functionality, no behavior changes.

**Prerequisite**: ₢A1AAf (add-buz-group-to-buk-zipper), ₢A1AAe (rename-consecration-to-hallmark) must be complete first.

### Complete old→new colophon map

Accounts (rbw-a) — Google Cloud service accounts:

```
rbw-PM   PayorMantlesGovernor            →  rbw-aM   PayorMantlesGovernor
rbw-GC   GovernorChartersRetriever       →  rbw-aC   GovernorChartersRetriever
rbw-GK   GovernorKnightsDirector         →  rbw-aK   GovernorKnightsDirector
rbw-Gl   GovernorListsServiceAccounts    →  rbw-aL   GovernorListsServiceAccounts
rbw-GF   GovernorForfeitsServiceAccount  →  rbw-aF   GovernorForfeitsServiceAccount
```

Crucible (rbw-c) — container runtime (only the predicate rename):

```
rbw-ca   CrucibleActive                  →  rbw-cic  CrucibleIsCharged
```

Depot (rbw-d) — GCP project infrastructure:

```
rbw-PL   PayorLeviesDepot                →  rbw-dL   PayorLeviesDepot
rbw-PU   PayorUnmakesDepot               →  rbw-dU   PayorUnmakesDepot
rbw-Pl   PayorListsDepots                →  rbw-dl   PayorListsDepots
```

Hallmark (rbw-h) — registry artifact lifecycle (UPPER=mutates GAR, lower=read/local):

```
rbw-DO   DirectorOrdainsConsecration     →  rbw-hO   DirectorOrdainsHallmark
rbw-DA   DirectorAbjuresConsecration     →  rbw-hA   DirectorAbjuresHallmark
rbw-DV   DirectorVouchesConsecrations    →  rbw-hV   DirectorVouchesHallmarks
rbw-Dt   DirectorTalliesConsecrations    →  rbw-ht   DirectorTalliesHallmarks
rbw-Rs   RetrieverSummonsConsecration    →  rbw-hs   RetrieverSummonsHallmark
rbw-LK   LedgerKludgesVessel             →  rbw-hk   RetrieverKludgesHallmark
rbw-RpF  RetrieverPlumbsFull             →  rbw-hpf  RetrieverPlumbsFull
rbw-Rpc  RetrieverPlumbsCompact          →  rbw-hpc  RetrieverPlumbsCompact
```

Image (rbw-i) — container image operations:

```
rbw-DJ   DirectorJettisonsImage          →  rbw-iJ   DirectorJettisonsImage
rbw-Rw   RetrieverWrestsImage            →  rbw-iw   RetrieverWrestsImage
```

Theurge (rbw-t) — test infrastructure:

```
rbw-Tk   TheurgeKludgesVessel            →  rbw-tK   TheurgeKludgesHallmark
rbw-To   TheurgeOrdainsVessel            →  rbw-tO   TheurgeOrdainsHallmark
rbw-Qf   QualifyFast                     →  rbw-tf   QualifyFast
rbw-QR   QualifyRelease                  →  rbw-tr   QualifyRelease
```

Vessel (rbw-v) — build operations:

```
rbw-DE   DirectorEnshinesVessel          →  rbw-vE   DirectorEnshrinesVessel
```

### Unsettled — requires design decision before execution

Enshrine (`rbw-vE`) and Reliquary (`rbw-DI` → `rbw-RI`) placement — see paddock discussion.

### Unchanged categories (no colophon renames needed)

- `rbw-c` Crucible — all except `ca` → `cic` stay as-is
- `rbw-g` Guide — human-directed procedures, unchanged
- `rbw-I` Ifrit — unchanged
- `rbw-M` Marshal — lifecycle, unchanged
- `rbw-r` Regime — config files, unchanged
- `rbw-n` Nameplate cross-cutting — unchanged

### Work items
- Update `rbz_zipper.sh` colophon strings and buz_group declarations for all affected enrollments
- Rename tabtarget files in `tt/` to match new colophons
- Update theurge colophon manifest (`rbtdrm_manifest.rs`)
- Resolve enshrine/reliquary placement
- Verify dispatch routing with smoke test

### Verification
- `tt/rbw-cic.CrucibleIsCharged.sh` returns correct predicate
- `tt/rbw-tf.QualifyFast.sh` passes (colophon health)
- `tt/rbtd-t.Test.sh` passes (theurge unit tests)
- No references to old colophons remain

### Acceptance
- All renamed tabtargets dispatch correctly
- Full fast suite passes
- Colophon qualify clean

**[260403-0851] rough**

## Character
Intricate but mechanical — large surface area of renames across zipper, tabtargets, and frontispieces, but each individual change is straightforward. Requires care to avoid breaking dispatch routing. Design decisions already made in conversation.

## Docket
Reorganize the `rbw` workbench colophon namespace from actor-organized (director/retriever/ledger) to domain-object-organized categories. This is a rename-only transformation — no new functionality, no behavior changes.

**Prerequisite**: ₢A1AAe (rename-consecration-to-hallmark) must be complete first. This pace assumes "Hallmark" vocabulary is already in place.

### New category scheme

| Prefix | Domain | Case convention |
|--------|--------|-----------------|
| `rbw-c` | Crucible (runtime) | unchanged except `ca` → `cic` |
| `rbw-d` | Depot (GCP project infra) | absorbs `PL`, `PU`, `Pl` |
| `rbw-g` | Guide (human-directed procedures) | unchanged |
| `rbw-h` | Hallmark (registry artifact lifecycle) | uppercase = mutates GAR, lowercase = read/local |
| `rbw-i` | Image | absorbs `DJ`, `Rw` |
| `rbw-I` | Ifrit | unchanged |
| `rbw-M` | Marshal (lifecycle) | unchanged |
| `rbw-r` | Regime (config files) | unchanged |
| `rbw-R` | Reliquary | absorbs `DI` |
| `rbw-s` | Service account | absorbs `GC`, `GK`, `Gl`, `GF`, `PM` |
| `rbw-t` | Theurge/test | absorbs `Tk`, `To`, `Qf`, `QR` |
| `rbw-v` | Vessel (build operations) | absorbs `DE` |

### Complete old→new colophon map

Vessel (rbw-v) — build operations:

```
rbw-DE   DirectorEnshinesVessel          →  rbw-vE   DirectorEnshrinesVessel
```

Hallmark (rbw-h) — uppercase mutates GAR:

```
rbw-DO   DirectorOrdainsConsecration     →  rbw-hO   DirectorOrdainsHallmark
rbw-DA   DirectorAbjuresConsecration     →  rbw-hA   DirectorAbjuresHallmark
rbw-DV   DirectorVouchesConsecrations    →  rbw-hV   DirectorVouchesHallmarks
```

Hallmark (rbw-h) — lowercase reads GAR or local only:

```
rbw-Dt   DirectorTalliesConsecrations    →  rbw-ht   DirectorTalliesHallmarks
rbw-Rs   RetrieverSummonsConsecration    →  rbw-hs   RetrieverSummonsHallmark
rbw-LK   LedgerKludgesVessel             →  rbw-hk   RetrieverKludgesHallmark
rbw-RpF  RetrieverPlumbsFull             →  rbw-hpf  RetrieverPlumbsFull
rbw-Rpc  RetrieverPlumbsCompact          →  rbw-hpc  RetrieverPlumbsCompact
```

Crucible predicate rename (absorbed from dropped ₢A1AAY):

```
rbw-ca   CrucibleActive                  →  rbw-cic  CrucibleIsCharged
```

### Remaining categories (maps TBD in grooming)

Depot (rbw-d), Image (rbw-i), Reliquary (rbw-R), Service account (rbw-s), Theurge (rbw-t) — these categories need old→new colophon maps built out in the same format as above before execution.

### Service account frontispiece convention
Noun-Verb-Object when one account acts upon another:
- `rbw-sM.PayorMantlesGovernor.sh`
- `rbw-sK.GovernorKnightsDirector.sh`
- `rbw-sC.GovernorChartersRetriever.sh`
- `rbw-sL.ListServiceAccounts.sh`
- `rbw-sF.ForfeitServiceAccount.sh`

### Work items
- Update `rbz_zipper.sh` colophon strings for all affected enrollments
- Rename tabtarget files in `tt/` to match new colophons
- Update frontispieces where needed
- Rename `rbw-ca` → `rbw-cic` (CrucibleIsCharged) across all touch points
- Update theurge colophon manifest (`rbtdrm_manifest.rs`)
- Verify dispatch routing with smoke test
- Update CLAUDE.md acronym mappings if any reference colophons

### Verification
- `tt/rbw-cic.CrucibleIsCharged.sh` returns correct predicate
- `tt/rbw-Qf.QualifyFast.sh` passes (colophon health)
- `tt/rbtd-t.Test.sh` passes (theurge unit tests)
- No references to old colophons remain

### Acceptance
- All renamed tabtargets dispatch correctly
- Full fast suite passes
- Colophon qualify clean

**[260402-2340] rough**

## Character
Intricate but mechanical — large surface area of renames across zipper, tabtargets, and frontispieces, but each individual change is straightforward. Requires care to avoid breaking dispatch routing. Design decisions already made in conversation.

## Docket
Reorganize the `rbw` workbench colophon namespace from actor-organized (director/retriever/ledger) to domain-object-organized categories. This is a rename-only transformation — no new functionality, no behavior changes.

### New category scheme

| Prefix | Domain | Current prefixes being absorbed |
|--------|--------|---------------------------------|
| `rbw-a` | Ark (consecration lifecycle) | `DA`, `DV`, `Dt`, `RpF`, `Rpc` |
| `rbw-c` | Crucible (runtime) | `cC`, `cQ`, `cr`, `cs`, `cw`, `cf`, `cb`, `ch` (unchanged); `ca` renamed to `cic` (CrucibleIsCharged) |
| `rbw-d` | Depot (GCP project infra) | `PL`, `PU`, `Pl` |
| `rbw-g` | Guide (human-directed procedures) | `gPI`, `gPE`, `gPR`, `gq`, `gO` (unchanged) |
| `rbw-i` | Image | `DJ`, `Rw` |
| `rbw-I` | Ifrit | `Ic`, `Is` (unchanged) |
| `rbw-M` | Marshal (lifecycle) | `MZ`, `MP` (unchanged) |
| `rbw-r` | Regime (config files) | `rn*`, `rv*`, `rr*`, `rp*`, `ro*`, `rs*`, `ra*`, `ni`, `nv` (unchanged) |
| `rbw-R` | Reliquary | `DI` |
| `rbw-s` | Service account | `GC`, `GK`, `Gl`, `GF`, `PM` |
| `rbw-t` | Theurge/test | `Tk`, `To`, `Qf`, `QR` |
| `rbw-v` | Vessel (config + ark ops) | `DE`, `DO`, `LK`, `Rs` |

### Crucible predicate rename (absorbed from dropped ₢A1AAY)
- Colophon `rbw-ca` (CrucibleActive) → `rbw-cic` (CrucibleIsCharged)
- Tabtarget: delete `tt/rbw-ca.CrucibleActive.sh`, create `tt/rbw-cic.CrucibleIsCharged.sh`
- Zipper enrollment: rename constant to `RBZ_CRUCIBLE_IS_CHARGED`, update colophon string
- Theurge colophon manifest in `rbtdrm_manifest.rs`: update compiled-in colophon string
- Workbench/rbte colophon list: update reference

### Service account frontispiece convention
Noun-Verb-Object when one account acts upon another:
- `rbw-sM.PayorMantlesGovernor.sh`
- `rbw-sK.GovernorKnightsDirector.sh`
- `rbw-sC.GovernorChartersRetriever.sh`
- `rbw-sL.ListServiceAccounts.sh`
- `rbw-sF.ForfeitServiceAccount.sh`

### Case convention
Lowercase = high-frequency daily use, uppercase = specialized/rare (`i/I`, `r/R`)

### Vessel + ark collapse decision
Vessel and ark ops collapse into `rbw-v` — the vessel is the subject in all cases:
`vE` enshrine, `vO` ordain, `vK` kludge, `vS` summon, `vA` abjure, `vV` vouch, `vT` tally, `vP` plumb

### Work items
- Update `rbz_zipper.sh` colophon strings for all affected enrollments
- Rename tabtarget files in `tt/` to match new colophons
- Update frontispieces where needed (especially service account ops)
- Rename `rbw-ca` → `rbw-cic` (CrucibleIsCharged) across all touch points
- Verify dispatch routing with a smoke test
- Update CLAUDE.md acronym mappings if any reference colophons

### Verification
- `tt/rbw-cic.CrucibleIsCharged.sh` returns correct predicate
- `tt/rbw-Qf.QualifyFast.sh` passes (colophon health)
- `tt/rbtd-t.Test.sh` passes (theurge unit tests)
- No references to `rbw-ca` remain

### Acceptance
- All renamed tabtargets dispatch correctly
- Full fast suite passes
- Colophon qualify clean

**[260402-2314] rough**

## Character
Intricate but mechanical — large surface area of renames across zipper, tabtargets, and frontispieces, but each individual change is straightforward. Requires care to avoid breaking dispatch routing. Design decisions already made in conversation.

## Docket
Reorganize the `rbw` workbench colophon namespace from actor-organized (director/retriever/ledger) to domain-object-organized categories. This is a rename-only transformation — no new functionality, no behavior changes.

### New category scheme

| Prefix | Domain | Current prefixes being absorbed |
|--------|--------|---------------------------------|
| `rbw-a` | Ark (consecration lifecycle) | `DA`, `DV`, `Dt`, `RpF`, `Rpc` |
| `rbw-c` | Crucible (runtime) | `cC`, `cQ`, `cr`, `cs`, `cw`, `cf`, `cb`, `ch`, `ca` (unchanged) |
| `rbw-d` | Depot (GCP project infra) | `PL`, `PU`, `Pl` |
| `rbw-g` | Guide (human-directed procedures) | `gPI`, `gPE`, `gPR`, `gq`, `gO` (unchanged) |
| `rbw-i` | Image | `DJ`, `Rw` |
| `rbw-I` | Ifrit | `Ic`, `Is` (unchanged) |
| `rbw-M` | Marshal (lifecycle) | `MZ`, `MP` (unchanged) |
| `rbw-r` | Regime (config files) | `rn*`, `rv*`, `rr*`, `rp*`, `ro*`, `rs*`, `ra*`, `ni`, `nv` (unchanged) |
| `rbw-R` | Reliquary | `DI` |
| `rbw-s` | Service account | `GC`, `GK`, `Gl`, `GF`, `PM` |
| `rbw-t` | Theurge/test | `Tk`, `To`, `Qf`, `QR` |
| `rbw-v` | Vessel (config + ark ops) | `DE`, `DO`, `LK`, `Rs` |

### Service account frontispiece convention
Noun-Verb-Object when one account acts upon another:
- `rbw-sM.PayorMantlesGovernor.sh`
- `rbw-sK.GovernorKnightsDirector.sh`
- `rbw-sC.GovernorChartersRetriever.sh`
- `rbw-sL.ListServiceAccounts.sh`
- `rbw-sF.ForfeitServiceAccount.sh`

### Case convention
Lowercase = high-frequency daily use, uppercase = specialized/rare (`i/I`, `r/R`)

### Work items
- Update `rbz_zipper.sh` colophon strings for all affected enrollments
- Rename tabtarget files in `tt/` to match new colophons
- Update frontispieces where needed (especially service account ops)
- Verify dispatch routing with a smoke test
- Update CLAUDE.md acronym mappings if any reference colophons

### Vessel + ark collapse decision
Vessel and ark ops collapse into `rbw-v` — the vessel is the subject in all cases:
`vE` enshrine, `vO` ordain, `vK` kludge, `vS` summon, `vA` abjure, `vV` vouch, `vT` tally, `vP` plumb

### Open question
- New `rbw-cKB.KludgeBottle.sh` (₢A1AAZ) should be slated after this lands, or simultaneously

### crucible-verification-after-colophon-reorg (₢A1AAh) [complete]

**[260403-1037] complete**

## Character
Mechanical verification — no code changes, just running suites and investigating any failures.

## Docket
Run the crucible test suite to verify that the tectonic colophon reorganization didn't break any live colophon usage paths. The fast suite verifies colophon health and regime validation, but crucible actually exercises colophons through charge/writ/fiat/bark/ordain operations against real containers.

**Prerequisite**: ₢A1AAb (tectonic-colophon-reorganization) must be complete.

### Work items
- Run `tt/rbtd-s.TestSuite.crucible.sh` (117 cases — exercises container runtime colophons)
- If available, run `tt/rbtd-s.TestSuite.service.sh` (80 cases — exercises GCP credential colophons)
- Investigate and fix any failures (colophon references in Cloud Build JSON templates, bottle exec paths, etc.)

### Verification
- Crucible suite green
- Service suite green (if credentials available)

### Acceptance
- All colophon usage paths verified through live execution
- No stale colophon references remain in runtime paths

**[260403-0935] rough**

## Character
Mechanical verification — no code changes, just running suites and investigating any failures.

## Docket
Run the crucible test suite to verify that the tectonic colophon reorganization didn't break any live colophon usage paths. The fast suite verifies colophon health and regime validation, but crucible actually exercises colophons through charge/writ/fiat/bark/ordain operations against real containers.

**Prerequisite**: ₢A1AAb (tectonic-colophon-reorganization) must be complete.

### Work items
- Run `tt/rbtd-s.TestSuite.crucible.sh` (117 cases — exercises container runtime colophons)
- If available, run `tt/rbtd-s.TestSuite.service.sh` (80 cases — exercises GCP credential colophons)
- Investigate and fix any failures (colophon references in Cloud Build JSON templates, bottle exec paths, etc.)

### Verification
- Crucible suite green
- Service suite green (if credentials available)

### Acceptance
- All colophon usage paths verified through live execution
- No stale colophon references remain in runtime paths

### marshal-generates-claude-tabtarget-context (₢A1AAg) [complete]

**[260403-1100] complete**

## Character
Infrastructure plumbing with a design decision about buz_describe API shape. The generation script is mechanical once the API exists.

## Docket
Add per-colophon description support to BUK zipper and a Marshal operation that generates `rbk-claude-tabtarget-context.md` from the zipper registry. Both CLAUDE.md variants `@`-include the generated file, eliminating hand-maintained colophon tables.

**Prerequisite**: ₢A1AAb (tectonic-colophon-reorganization) must be complete — generates from final colophon names.

### Design decisions needed
- API shape for per-colophon descriptions: companion `buz_describe` call vs extra arg to `buz_enroll`
- `buz_enroll` already has variable trailing args (`param1`, `imprint`), so companion call is cleaner
- Generated file format: group headers from `buz_group`, colophon rows from `buz_enroll` + `buz_describe`

### Work items
- Add `buz_describe` (or chosen API) to BUK zipper infrastructure
- Add description strings to all `rbz_zipper.sh` enrollments
- Add Marshal generation operation: walk zipper declarations, emit `rbk-claude-tabtarget-context.md`
- Update both CLAUDE files to `@`-include generated file, remove hand-maintained colophon references
- Add qualify assertion: generated file matches committed version (freshness gate)
- Add proof step: regenerate as part of release proofing

### Verification
- Generated file matches zipper contents exactly
- Both CLAUDE files include it correctly
- Qualify detects stale file
- `tt/rbtd-s.TestSuite.fast.sh` passes

### Acceptance
- Single source of truth: zipper owns colophon descriptions
- Generated context file is a build artifact, never hand-edited
- Marshal proof ensures freshness at release time

**[260403-0928] rough**

## Character
Infrastructure plumbing with a design decision about buz_describe API shape. The generation script is mechanical once the API exists.

## Docket
Add per-colophon description support to BUK zipper and a Marshal operation that generates `rbk-claude-tabtarget-context.md` from the zipper registry. Both CLAUDE.md variants `@`-include the generated file, eliminating hand-maintained colophon tables.

**Prerequisite**: ₢A1AAb (tectonic-colophon-reorganization) must be complete — generates from final colophon names.

### Design decisions needed
- API shape for per-colophon descriptions: companion `buz_describe` call vs extra arg to `buz_enroll`
- `buz_enroll` already has variable trailing args (`param1`, `imprint`), so companion call is cleaner
- Generated file format: group headers from `buz_group`, colophon rows from `buz_enroll` + `buz_describe`

### Work items
- Add `buz_describe` (or chosen API) to BUK zipper infrastructure
- Add description strings to all `rbz_zipper.sh` enrollments
- Add Marshal generation operation: walk zipper declarations, emit `rbk-claude-tabtarget-context.md`
- Update both CLAUDE files to `@`-include generated file, remove hand-maintained colophon references
- Add qualify assertion: generated file matches committed version (freshness gate)
- Add proof step: regenerate as part of release proofing

### Verification
- Generated file matches zipper contents exactly
- Both CLAUDE files include it correctly
- Qualify detects stale file
- `tt/rbtd-s.TestSuite.fast.sh` passes

### Acceptance
- Single source of truth: zipper owns colophon descriptions
- Generated context file is a build artifact, never hand-edited
- Marshal proof ensures freshness at release time

### kludge-bottle-crucible-tabtarget (₢A1AAZ) [complete]

**[260403-1113] complete**

## Character
Mechanical wiring — zipper enrollment, tabtarget creation, and function implementation following established patterns.

## Docket
Add `rbw-cKB.KludgeBottle.sh` tabtarget that takes a nameplate as CLI argument (like `rbw-cr.Rack.sh`, not imprinted per-nameplate).

Work items:
- Enroll `rbw-cKB` colophon in `rbz_zipper.sh` with `param1` mode, routing to a new function in `rbob_cli.sh` or `rbob_bottle.sh`
- Create `tt/rbw-cKB.KludgeBottle.sh` launcher (standard 3-line pattern)
- Implement the kludge-bottle function: rebuild vessel image locally (`LK` logic) and re-charge into the running crucible
- Reuses `zrbob_furnish` nameplate-listing-when-missing behavior via `param1` zipper enrollment

## Context
- Design discussion established `cKB` over `cK` — the `B` documents bottle scope even without a sentry counterpart
- Follows `rbw-cr.Rack.sh` precedent: crucible command, argument-based nameplate, no imprints
- Spec: RBSAK-ark_kludge.adoc describes the existing `LK` kludge operation

**[260402-2247] rough**

## Character
Mechanical wiring — zipper enrollment, tabtarget creation, and function implementation following established patterns.

## Docket
Add `rbw-cKB.KludgeBottle.sh` tabtarget that takes a nameplate as CLI argument (like `rbw-cr.Rack.sh`, not imprinted per-nameplate).

Work items:
- Enroll `rbw-cKB` colophon in `rbz_zipper.sh` with `param1` mode, routing to a new function in `rbob_cli.sh` or `rbob_bottle.sh`
- Create `tt/rbw-cKB.KludgeBottle.sh` launcher (standard 3-line pattern)
- Implement the kludge-bottle function: rebuild vessel image locally (`LK` logic) and re-charge into the running crucible
- Reuses `zrbob_furnish` nameplate-listing-when-missing behavior via `param1` zipper enrollment

## Context
- Design discussion established `cKB` over `cK` — the `B` documents bottle scope even without a sentry counterpart
- Follows `rbw-cr.Rack.sh` precedent: crucible command, argument-based nameplate, no imprints
- Spec: RBSAK-ark_kludge.adoc describes the existing `LK` kludge operation

### fix-three-preexisting-sortie-failures (₢A1AAV) [complete]

**[260402-2315] complete**

## Character
Debugging — three independent failure investigations. Single-case runner (from previous pace) makes iteration fast. Each failure may need ifrit code fix, sentry hardening, or test expectation adjustment.

## Docket

Fix the 3 pre-existing tadmor-sortie-attacks failures: sortie-icmp-exfil-payload, sortie-net-srcip-spoof, sortie-ns-capability-escape.

### Workflow
- Charge crucible once
- Use single-case runner to iterate on each failure independently
- Diagnose root cause: is the sortie wrong, the sentry misconfigured, or the expectation incorrect?

### Cases to fix
- `sortie-icmp-exfil-payload`: ICMP covert channel — investigate what the ifrit reports and why it fails
- `sortie-net-srcip-spoof`: source IP spoofing via raw sockets — may be a capability/permission issue
- `sortie-ns-capability-escape`: namespace and capability escape probe — likely a container config finding

### Acceptance
- All 3 cases pass with correct verdicts
- Full crucible suite: 37 pass, 0 fail
- No regressions in other cases

**[260402-1604] rough**

## Character
Debugging — three independent failure investigations. Single-case runner (from previous pace) makes iteration fast. Each failure may need ifrit code fix, sentry hardening, or test expectation adjustment.

## Docket

Fix the 3 pre-existing tadmor-sortie-attacks failures: sortie-icmp-exfil-payload, sortie-net-srcip-spoof, sortie-ns-capability-escape.

### Workflow
- Charge crucible once
- Use single-case runner to iterate on each failure independently
- Diagnose root cause: is the sortie wrong, the sentry misconfigured, or the expectation incorrect?

### Cases to fix
- `sortie-icmp-exfil-payload`: ICMP covert channel — investigate what the ifrit reports and why it fails
- `sortie-net-srcip-spoof`: source IP spoofing via raw sockets — may be a capability/permission issue
- `sortie-ns-capability-escape`: namespace and capability escape probe — likely a container config finding

### Acceptance
- All 3 cases pass with correct verdicts
- Full crucible suite: 37 pass, 0 fail
- No regressions in other cases

### novel-coordinated-security-tests (₢A1AAT) [complete]

**[260403-1143] complete**

## Character
Mechanical with breadth — three independent unilateral attacks, each following the established ifrit attack pattern. New `rbida_Attack` variants, sortie implementations, and bark-invoked theurge cases. No coordinated observation needed.

## Docket

Add three novel unilateral security tests probing attack surfaces not covered by existing tadmor cases. Each test adds a new ifrit attack variant dispatched via bark, with verdict determined by exit code.

### New attacks

**Route table manipulation** — Ifrit attempts `ip route replace default` and `ip route add` to bypass the sentry gateway. Verifies that the container lacks permission to modify its routing table (expected: RTNETLINK operation not permitted). Tests a fundamental container escape vector that no current test exercises — existing `ns_capability_escape` checks capabilities but doesn't actually attempt routing changes.

**Enclave subnet escape** — Ifrit probes for hosts outside the /24 enclave but within the broader bridge network range (e.g., if enclave is 10.242.0.0/24, probe 10.242.1.x and 10.242.255.x). Also probe adjacent /16 ranges. Distinct from `net_forbidden_cidr` which tests internet-routable destinations — this tests the network scope isolation of the enclave itself.

**DNAT entry port reflection** — Ifrit attempts TCP connection to sentry_ip:entry_port from inside the bottle. The sentry DNATs this port for external access; from inside the enclave it should be unreachable. Existing `direct_sentry_probe` checks entry port accessibility but as part of a broad port scan — this is a focused test of the DNAT asymmetry with detailed diagnostics.

### Implementation

- Three new variants in `rbida_Attack` enum + selector mappings
- Sortie implementations in `rbida_sorties.rs` (route: shell out to `ip route`; subnet: TCP probes to computed addresses; DNAT: TCP probe to sentry:entry_port)
- Three new bark-invoke cases in `rbtdrc_crucible.rs`
- New cases added to `tadmor-sortie-attacks` section (or a new `tadmor-unilateral-novel` section if cleaner)

### Acceptance

- All 3 new cases pass with SECURE verdicts
- Total tadmor case count grows from 34 to 37
- No regressions in existing cases
- Full crucible suite green

**[260402-2337] rough**

## Character
Mechanical with breadth — three independent unilateral attacks, each following the established ifrit attack pattern. New `rbida_Attack` variants, sortie implementations, and bark-invoked theurge cases. No coordinated observation needed.

## Docket

Add three novel unilateral security tests probing attack surfaces not covered by existing tadmor cases. Each test adds a new ifrit attack variant dispatched via bark, with verdict determined by exit code.

### New attacks

**Route table manipulation** — Ifrit attempts `ip route replace default` and `ip route add` to bypass the sentry gateway. Verifies that the container lacks permission to modify its routing table (expected: RTNETLINK operation not permitted). Tests a fundamental container escape vector that no current test exercises — existing `ns_capability_escape` checks capabilities but doesn't actually attempt routing changes.

**Enclave subnet escape** — Ifrit probes for hosts outside the /24 enclave but within the broader bridge network range (e.g., if enclave is 10.242.0.0/24, probe 10.242.1.x and 10.242.255.x). Also probe adjacent /16 ranges. Distinct from `net_forbidden_cidr` which tests internet-routable destinations — this tests the network scope isolation of the enclave itself.

**DNAT entry port reflection** — Ifrit attempts TCP connection to sentry_ip:entry_port from inside the bottle. The sentry DNATs this port for external access; from inside the enclave it should be unreachable. Existing `direct_sentry_probe` checks entry port accessibility but as part of a broad port scan — this is a focused test of the DNAT asymmetry with detailed diagnostics.

### Implementation

- Three new variants in `rbida_Attack` enum + selector mappings
- Sortie implementations in `rbida_sorties.rs` (route: shell out to `ip route`; subnet: TCP probes to computed addresses; DNAT: TCP probe to sentry:entry_port)
- Three new bark-invoke cases in `rbtdrc_crucible.rs`
- New cases added to `tadmor-sortie-attacks` section (or a new `tadmor-unilateral-novel` section if cleaner)

### Acceptance

- All 3 new cases pass with SECURE verdicts
- Total tadmor case count grows from 34 to 37
- No regressions in existing cases
- Full crucible suite green

**[260402-1509] rough**

## Character
Mechanical with breadth — apply the coordination pattern from the previous pace across several new attack surfaces. Each test is independent; parallelizable research but sequential execution.

## Docket

Add several novel coordinated and unilateral tests exploiting new attack surfaces not covered by existing 34 tadmor cases. Show that the apparatus passes all.

### Candidate tests (select 4-6 that are load-bearing)
- **ARP table overflow**: flood sentry with fake ARP entries from bottle, verify legitimate entries survive
- **MAC flooding**: send frames from many fake MACs to overwhelm bridge CAM table, verify sentry forwarding unaffected
- **Sentry process integrity**: verify from outside (writ exec) that sentry's iptables rules and dnsmasq process haven't been altered by bottle activity
- **Route table manipulation**: attempt `ip route replace` or `ip route add` from inside bottle to bypass sentry gateway
- **Enclave subnet escape**: probe for hosts outside the /24 enclave from bottle, verify no responses
- **Bridge promiscuous mode**: attempt to put enclave interface into promiscuous mode from bottle, verify blocked

### Implementation
- New cases in `tadmor-coordinated-attacks` section (or new section if warranted)
- Each test follows snapshot-attack-resnap pattern where observation is needed
- Unilateral tests (route manipulation, subnet escape) use standard ifrit exit-code pattern

### Acceptance
- All new cases pass with SECURE verdicts
- Total tadmor case count grows by 4-6
- Full crucible suite green
- No regressions in existing cases

### coordinated-security-integrity-verification (₢A1AAc) [complete]

**[260403-1709] complete**

## Character
Design-heavy coordination work — each test requires careful orchestration of writ observation around ifrit attacks. Follows the snapshot-attack-resnap pattern established in pace S (coordinated ARP tests). May need 1-2 new ifrit primitives (DNS forge, MAC flood) but the primary complexity is in theurge observation logic.

## Docket

Add three coordinated security tests that verify the *persistence* of sentry security controls under active attack. Unlike unilateral tests (verdict from ifrit exit code), these use theurge to observe sentry state before and after ifrit attacks.

### New coordinated tests

**Sentry process integrity post-attack** — Run a battery of existing ifrit attacks (dns-blocked-google, direct-arp-poison, proto-smuggle-rawsock), then verify via writ that: (1) dnsmasq process is still running, (2) iptables rules match the pre-attack snapshot (no rules flushed or added), (3) sentry's network interfaces haven't changed. This is a meta-test — it verifies that exercising the security boundary doesn't degrade it.

**DNS cache integrity under attack** — New ifrit primitive sends forged DNS UDP responses to sentry's dnsmasq port (crafted packets claiming google.com resolves to 1.2.3.4). Theurge snapshots frozen DNS records via writ dig before and after, verifies they haven't changed. Tests the resolve-then-freeze invariant: in allowlist mode there's no upstream server= directive, so dnsmasq should ignore unsolicited responses.

**MAC flooding bridge resilience** — New ifrit primitive sends Ethernet frames from many fake source MACs via AF_PACKET (similar infrastructure to ARP sorties). Theurge verifies sentry→bottle connectivity survives the flood by running writ/fiat commands before and after. Tests L2 fabric resilience — existing ARP tests target the L3 ARP table, this targets the bridge's MAC learning table.

### Implementation

- 1-2 new ifrit attack primitives in `rbida_Attack` (dns-forge-response, mac-flood-bridge) + sortie implementations
- 3 new coordinated cases in `rbtdrc_crucible.rs` following snapshot-attack-resnap pattern
- New `tadmor-coordinated-integrity` section (distinct from existing `tadmor-coordinated-attacks` which covers ARP)

### Acceptance

- All 3 new coordinated cases pass
- Total tadmor case count grows by 3 (to 40 if combined with unilateral pace)
- Sentry integrity test exercises at least 3 different attack types before checking
- No regressions in existing cases
- Full crucible suite green

**[260402-2338] rough**

## Character
Design-heavy coordination work — each test requires careful orchestration of writ observation around ifrit attacks. Follows the snapshot-attack-resnap pattern established in pace S (coordinated ARP tests). May need 1-2 new ifrit primitives (DNS forge, MAC flood) but the primary complexity is in theurge observation logic.

## Docket

Add three coordinated security tests that verify the *persistence* of sentry security controls under active attack. Unlike unilateral tests (verdict from ifrit exit code), these use theurge to observe sentry state before and after ifrit attacks.

### New coordinated tests

**Sentry process integrity post-attack** — Run a battery of existing ifrit attacks (dns-blocked-google, direct-arp-poison, proto-smuggle-rawsock), then verify via writ that: (1) dnsmasq process is still running, (2) iptables rules match the pre-attack snapshot (no rules flushed or added), (3) sentry's network interfaces haven't changed. This is a meta-test — it verifies that exercising the security boundary doesn't degrade it.

**DNS cache integrity under attack** — New ifrit primitive sends forged DNS UDP responses to sentry's dnsmasq port (crafted packets claiming google.com resolves to 1.2.3.4). Theurge snapshots frozen DNS records via writ dig before and after, verifies they haven't changed. Tests the resolve-then-freeze invariant: in allowlist mode there's no upstream server= directive, so dnsmasq should ignore unsolicited responses.

**MAC flooding bridge resilience** — New ifrit primitive sends Ethernet frames from many fake source MACs via AF_PACKET (similar infrastructure to ARP sorties). Theurge verifies sentry→bottle connectivity survives the flood by running writ/fiat commands before and after. Tests L2 fabric resilience — existing ARP tests target the L3 ARP table, this targets the bridge's MAC learning table.

### Implementation

- 1-2 new ifrit attack primitives in `rbida_Attack` (dns-forge-response, mac-flood-bridge) + sortie implementations
- 3 new coordinated cases in `rbtdrc_crucible.rs` following snapshot-attack-resnap pattern
- New `tadmor-coordinated-integrity` section (distinct from existing `tadmor-coordinated-attacks` which covers ARP)

### Acceptance

- All 3 new coordinated cases pass
- Total tadmor case count grows by 3 (to 40 if combined with unilateral pace)
- Sentry integrity test exercises at least 3 different attack types before checking
- No regressions in existing cases
- Full crucible suite green

### rbida-selector-const-extraction (₢A1AAi) [complete]

**[260403-1734] complete**

## Character
Mechanical cleanup — no design judgment. Apply RCG Single Definition Rule to all 36 pre-existing attack selector strings in `rbida_attacks.rs`.

## Docket

Extract consts for all pre-existing `rbida_Attack` selector strings following the pattern established by `RBIDA_SEL_DNS_FORGE_RESPONSE` and `RBIDA_SEL_MAC_FLOOD_BRIDGE`. Currently 36 variants have their kebab-case selector string duplicated across `from_selector`, `selector()`, and `all_selectors()` — RCG String Boundary Discipline violation.

### Implementation

- Add ~36 `const RBIDA_SEL_*: &str` definitions to the selector constants block
- Replace all string literals in the three methods with const references
- Build, run theurge unit tests

### Acceptance

- `grep '"dns-allowed-anthropic"'` finds zero hits in `rbida_attacks.rs` (only the const definition)
- Theurge builds and unit tests pass
- No behavioral change

**[260403-1154] rough**

## Character
Mechanical cleanup — no design judgment. Apply RCG Single Definition Rule to all 36 pre-existing attack selector strings in `rbida_attacks.rs`.

## Docket

Extract consts for all pre-existing `rbida_Attack` selector strings following the pattern established by `RBIDA_SEL_DNS_FORGE_RESPONSE` and `RBIDA_SEL_MAC_FLOOD_BRIDGE`. Currently 36 variants have their kebab-case selector string duplicated across `from_selector`, `selector()`, and `all_selectors()` — RCG String Boundary Discipline violation.

### Implementation

- Add ~36 `const RBIDA_SEL_*: &str` definitions to the selector constants block
- Replace all string literals in the three methods with const references
- Build, run theurge unit tests

### Acceptance

- `grep '"dns-allowed-anthropic"'` finds zero hits in `rbida_attacks.rs` (only the const definition)
- Theurge builds and unit tests pass
- No behavioral change

### install-procps-in-sentry-image (₢A1AAk) [complete]

**[260403-1817] complete**

## Character
Targeted infrastructure fix — one Dockerfile change plus one theurge consumer update.

## Docket

The sentry integrity test (`rbtdrc_coordinated_sentry_integrity`) uses `cat /proc/[0-9]*/comm` with `; exit 0` to check if dnsmasq is running, because the sentry container lacks process inspection tools. This is fragile — the `/proc` glob races with transient processes and requires a forced-zero exit to avoid false failures.

### Fix

- Add `procps` package to the sentry Dockerfile (provides `pidof`, `pgrep`, `ps`)
- Replace the `/proc/*/comm` hack in `rbtdrc_coordinated_sentry_integrity` with `pidof dnsmasq`
- Kludge sentry image, run tadmor fixture

### Acceptance

- `pidof dnsmasq` works inside sentry container
- No `; exit 0` hack in theurge code
- Tadmor 43 cases green

**[260403-1244] rough**

## Character
Targeted infrastructure fix — one Dockerfile change plus one theurge consumer update.

## Docket

The sentry integrity test (`rbtdrc_coordinated_sentry_integrity`) uses `cat /proc/[0-9]*/comm` with `; exit 0` to check if dnsmasq is running, because the sentry container lacks process inspection tools. This is fragile — the `/proc` glob races with transient processes and requires a forced-zero exit to avoid false failures.

### Fix

- Add `procps` package to the sentry Dockerfile (provides `pidof`, `pgrep`, `ps`)
- Replace the `/proc/*/comm` hack in `rbtdrc_coordinated_sentry_integrity` with `pidof dnsmasq`
- Kludge sentry image, run tadmor fixture

### Acceptance

- `pidof dnsmasq` works inside sentry container
- No `; exit 0` hack in theurge code
- Tadmor 43 cases green

### replace-anthropic-allowlist-with-example-domains (₢A1AAj) [complete]

**[260404-0833] complete**

## Character
Mechanical but wide — touches both binaries, the nameplate regime, and crucible test code. No design judgment needed; the domain choice is settled. Careful grep-and-replace with build verification.

## Docket

Replace the vestigial `anthropic.com claude.ai claude.com` allowlist in tadmor with `example.com example.org` (IANA RFC 2606 reserved domains). Update CIDR to match. Exercises list treatment (2 domains) and removes the false implication that the bottle needs Anthropic access.

### Nameplate regime

- `.rbk/tadmor/rbrn.env`: change `RBRN_UPLINK_ALLOWED_DOMAINS` to `"example.com example.org"`
- `.rbk/tadmor/rbrn.env`: change `RBRN_UPLINK_ALLOWED_CIDRS` to cover the example.com/example.org IP range (currently 93.184.216.34 — use `93.184.216.0/24` or verify at build time)

### Ifrit binary

- `rbida_attacks.rs`: rename `DnsAllowedAnthropic` variant and `dns-allowed-anthropic` selector → `DnsAllowedExample` / `dns-allowed-example`
- Update all `anthropic.com` probe targets to `example.com`
- DNS exfiltration sorties: update `ALLOWED_DOMAINS` list from anthropic domains to example domains

### Theurge crucible tests

- `rbtdrc_crucible.rs`: rename `rbtdrc_ifrit_dns_allowed` internals, `rbtdrc_tcp443_allow_anthropic` → `rbtdrc_tcp443_allow_example`
- Update all `anthropic.com` dig/resolve references to `example.com`
- Coordinated DNS cache integrity test (from AAc): update if already landed, or already correct if this lands first
- Add a second allowed-domain DNS test for `example.org` to exercise list treatment

### Acceptance

- `grep -r 'anthropic' Tools/rbk/rbtd/ Tools/rbk/rbid/` returns zero hits
- `grep -r 'anthropic' .rbk/tadmor/` returns zero hits (except vessel name if unchanged)
- Theurge builds, unit tests pass
- Full crucible suite green (charge tadmor, run all tadmor cases)
- Both `example.com` and `example.org` exercised in at least one test each

**[260403-1244] rough**

## Character
Mechanical but wide — touches both binaries, the nameplate regime, and crucible test code. No design judgment needed; the domain choice is settled. Careful grep-and-replace with build verification.

## Docket

Replace the vestigial `anthropic.com claude.ai claude.com` allowlist in tadmor with `example.com example.org` (IANA RFC 2606 reserved domains). Update CIDR to match. Exercises list treatment (2 domains) and removes the false implication that the bottle needs Anthropic access.

### Nameplate regime

- `.rbk/tadmor/rbrn.env`: change `RBRN_UPLINK_ALLOWED_DOMAINS` to `"example.com example.org"`
- `.rbk/tadmor/rbrn.env`: change `RBRN_UPLINK_ALLOWED_CIDRS` to cover the example.com/example.org IP range (currently 93.184.216.34 — use `93.184.216.0/24` or verify at build time)

### Ifrit binary

- `rbida_attacks.rs`: rename `DnsAllowedAnthropic` variant and `dns-allowed-anthropic` selector → `DnsAllowedExample` / `dns-allowed-example`
- Update all `anthropic.com` probe targets to `example.com`
- DNS exfiltration sorties: update `ALLOWED_DOMAINS` list from anthropic domains to example domains

### Theurge crucible tests

- `rbtdrc_crucible.rs`: rename `rbtdrc_ifrit_dns_allowed` internals, `rbtdrc_tcp443_allow_anthropic` → `rbtdrc_tcp443_allow_example`
- Update all `anthropic.com` dig/resolve references to `example.com`
- Coordinated DNS cache integrity test (from AAc): update if already landed, or already correct if this lands first
- Add a second allowed-domain DNS test for `example.org` to exercise list treatment

### Acceptance

- `grep -r 'anthropic' Tools/rbk/rbtd/ Tools/rbk/rbid/` returns zero hits
- `grep -r 'anthropic' .rbk/tadmor/` returns zero hits (except vessel name if unchanged)
- Theurge builds, unit tests pass
- Full crucible suite green (charge tadmor, run all tadmor cases)
- Both `example.com` and `example.org` exercised in at least one test each

### crucible-active-check-race-investigation (₢A1AAl) [complete]

**[260404-0949] complete**

## Character
Mechanical fix with a small horizon documentation task. Straightforward.

## Docket

Add `--wait` to the `compose up -d` call in `rbob_charge` so that charge blocks until all container health checks pass before returning. This eliminates the race between `compose up -d` returning and the active-check assertion in theurge's `main.rs`.

The compose file already has a well-designed health chain (sentry healthy → pentacle starts → pentacle healthy → bottle starts). Docker Compose `--wait` (available since v2.1.1) makes the `up` command itself block until that chain completes.

### Steps

- Add `--wait` to `zrbob_compose --profile sessile up -d` in `rbob_charge` (`rbob_bottle.sh`)
- Add horizon roadmap entries under Container Runtime / Podman section:
  1. `podman-compose` lacks `--wait` support (GitHub issues containers/podman-compose#710, #1329 — open since 2023, still unresolved as of 2025). The charge→active-check race will resurface on Podman integration and need a different solution.
  2. The VM image supply chain (`rbv_PodmanVM.sh`) currently targets GHCR and has not been ported to GAR/Depot. This is prerequisite infrastructure for Podman deployment.
- Build theurge to confirm no compile errors

### Acceptance

- `rbob_charge` uses `--wait`
- Horizon roadmap documents both Podman concerns
- Clean build

**[260404-0942] rough**

## Character
Mechanical fix with a small horizon documentation task. Straightforward.

## Docket

Add `--wait` to the `compose up -d` call in `rbob_charge` so that charge blocks until all container health checks pass before returning. This eliminates the race between `compose up -d` returning and the active-check assertion in theurge's `main.rs`.

The compose file already has a well-designed health chain (sentry healthy → pentacle starts → pentacle healthy → bottle starts). Docker Compose `--wait` (available since v2.1.1) makes the `up` command itself block until that chain completes.

### Steps

- Add `--wait` to `zrbob_compose --profile sessile up -d` in `rbob_charge` (`rbob_bottle.sh`)
- Add horizon roadmap entries under Container Runtime / Podman section:
  1. `podman-compose` lacks `--wait` support (GitHub issues containers/podman-compose#710, #1329 — open since 2023, still unresolved as of 2025). The charge→active-check race will resurface on Podman integration and need a different solution.
  2. The VM image supply chain (`rbv_PodmanVM.sh`) currently targets GHCR and has not been ported to GAR/Depot. This is prerequisite infrastructure for Podman deployment.
- Build theurge to confirm no compile errors

### Acceptance

- `rbob_charge` uses `--wait`
- Horizon roadmap documents both Podman concerns
- Clean build

**[260403-1852] rough**

## Character
Diagnostic investigation — requires reading lifecycle verification code and understanding Docker health-check timing. May surface a real race condition or confirm adequate retry logic already exists.

## Docket

Investigate the transient "crucible not active after charge" failure observed during ₢A1AAj work. The charge succeeded but the subsequent active-verification failed, and a blind retry succeeded. This pattern masks potential infrastructure bugs.

### Questions to answer

- How does the crucible active-check work? What does it poll, what are its timeouts?
- Was the failure caused by the new kludge bottle image being slower to start than the cached sentry/pentacle?
- Is there a quench/charge overlap race if they run in quick succession?
- Does the active-check have adequate wait/retry logic, or does it fire once and fail?

### Scope

- Read the charge and active-verification code paths
- Determine if the check is a single-shot assertion or a polling loop with timeout
- If single-shot: recommend adding a polling loop with reasonable timeout
- If polling loop: determine why it timed out and whether the timeout is too aggressive
- Document findings in paddock

### Acceptance

- Root cause identified or narrowed to specific code path
- Recommendation: fix, extend timeout, or accept-as-is with rationale

### egress-control-verification-tests (₢A1AAm) [complete]

**[260404-1015] complete**

## Character
Focused test development — two new ifrit attacks and two new theurge crucible cases. Mechanical implementation following established patterns. Requires understanding the sentry's iptables rule structure to set correct expectations.

## Docket

Add two new security tests verifying egress control boundaries that are currently untested.

### Context

The sentry's CIDR allowlist rules accept all protocols and all destination ports to allowed IP ranges (by design — see RBSHR Network Security Model entry). The DNS allowlist provides name-level precision; the CIDR provides IP-level gating. These are independent reinforcing layers.

Current test coverage verifies: TCP 443 to allowed CIDR (pass), TCP 443 to non-allowed IP (blocked), DNS allowed/blocked, ICMP blocked cross-boundary. Missing: UDP non-DNS egress, and positive confirmation that all-port CIDR access works as designed.

### Test 1: udp-non-dns-to-non-allowed

From inside the bottle, attempt to send a UDP datagram on a non-DNS port (e.g., port 1234) to an IP outside the allowed CIDRs. Expect failure. This is the UDP analog of the existing tcp443-block test.

- New ifrit attack: `UdpNonDnsBlocked` / `udp-non-dns-blocked`
- Resolve google.com via `getent` (will fail since blocked) — instead, use a hardcoded non-allowed IP (e.g., 8.8.8.8)
- Send UDP datagram to 8.8.8.8:1234 with short timeout
- Verdict: SECURE if send fails or times out, BREACH if successful response received
- Theurge case: `rbtdrc_udp_non_dns_blocked` in tadmor-correlated section

### Test 2: cidr-all-ports-allowed

From inside the bottle, verify that TCP connections to an allowed CIDR IP succeed on multiple ports (80, 443, 8080). This is a positive test confirming the spec-intended behavior: CIDR allowlist is protocol-agnostic, not port-scoped.

- New ifrit attack: `CidrAllPortsAllowed` / `cidr-all-ports-allowed`
- Resolve example.com, attempt TCP connections on ports 80 and 443 with short timeout
- Verdict: PASS if connections succeed (or at least are not blocked by iptables — TCP RST from the remote is acceptable, timeout from iptables DROP is not)
- Theurge case: `rbtdrc_cidr_all_ports_allowed` in tadmor-correlated section
- Note: distinguishing "connection refused by remote" (iptables allowed, remote rejected) from "connection timed out" (iptables blocked) is the key signal

### Acceptance

- Both ifrit selectors implemented and registered
- Both theurge cases registered in tadmor-correlated section
- Theurge builds, unit tests pass
- Full tadmor crucible green (44 + 2 = 46 cases)

**[260404-0912] rough**

## Character
Focused test development — two new ifrit attacks and two new theurge crucible cases. Mechanical implementation following established patterns. Requires understanding the sentry's iptables rule structure to set correct expectations.

## Docket

Add two new security tests verifying egress control boundaries that are currently untested.

### Context

The sentry's CIDR allowlist rules accept all protocols and all destination ports to allowed IP ranges (by design — see RBSHR Network Security Model entry). The DNS allowlist provides name-level precision; the CIDR provides IP-level gating. These are independent reinforcing layers.

Current test coverage verifies: TCP 443 to allowed CIDR (pass), TCP 443 to non-allowed IP (blocked), DNS allowed/blocked, ICMP blocked cross-boundary. Missing: UDP non-DNS egress, and positive confirmation that all-port CIDR access works as designed.

### Test 1: udp-non-dns-to-non-allowed

From inside the bottle, attempt to send a UDP datagram on a non-DNS port (e.g., port 1234) to an IP outside the allowed CIDRs. Expect failure. This is the UDP analog of the existing tcp443-block test.

- New ifrit attack: `UdpNonDnsBlocked` / `udp-non-dns-blocked`
- Resolve google.com via `getent` (will fail since blocked) — instead, use a hardcoded non-allowed IP (e.g., 8.8.8.8)
- Send UDP datagram to 8.8.8.8:1234 with short timeout
- Verdict: SECURE if send fails or times out, BREACH if successful response received
- Theurge case: `rbtdrc_udp_non_dns_blocked` in tadmor-correlated section

### Test 2: cidr-all-ports-allowed

From inside the bottle, verify that TCP connections to an allowed CIDR IP succeed on multiple ports (80, 443, 8080). This is a positive test confirming the spec-intended behavior: CIDR allowlist is protocol-agnostic, not port-scoped.

- New ifrit attack: `CidrAllPortsAllowed` / `cidr-all-ports-allowed`
- Resolve example.com, attempt TCP connections on ports 80 and 443 with short timeout
- Verdict: PASS if connections succeed (or at least are not blocked by iptables — TCP RST from the remote is acceptable, timeout from iptables DROP is not)
- Theurge case: `rbtdrc_cidr_all_ports_allowed` in tadmor-correlated section
- Note: distinguishing "connection refused by remote" (iptables allowed, remote rejected) from "connection timed out" (iptables blocked) is the key signal

### Acceptance

- Both ifrit selectors implemented and registered
- Both theurge cases registered in tadmor-correlated section
- Theurge builds, unit tests pass
- Full tadmor crucible green (44 + 2 = 46 cases)

**[260404-0838] rough**

## Character
Focused test development — two new ifrit attacks and two new theurge crucible cases. Mechanical implementation following established patterns, but requires understanding iptables rule ordering to know what the expected behavior should be.

## Docket

Add two new security tests verifying that the sentry's egress controls are port-scoped, not just CIDR-scoped.

### Test 1: tcp-outbound-high-port

From inside the bottle, attempt TCP connections to an allowed CIDR IP on non-443 ports (80, 8080, 22). Expect all to fail. Verifies that iptables CIDR rules are not wide-open — if they are, this is a real finding.

- New ifrit attack: `TcpHighPort` / `tcp-high-port`
- Resolve example.com, attempt connections on ports 80, 8080, 22 with short timeout
- Verdict: SECURE if all fail, BREACH if any succeed
- Theurge case: `rbtdrc_tcp_high_port_blocked` in tadmor-correlated section

### Test 2: udp-data-exfil

From inside the bottle, attempt to send a UDP datagram on a non-DNS port (e.g., port 1234) to an external IP. Expect failure. Verifies that non-DNS UDP egress is blocked.

- New ifrit attack: `UdpDataExfil` / `udp-data-exfil`
- Send UDP datagram to example.com IP on port 1234
- Verdict: SECURE if send fails or packet is dropped, BREACH if send succeeds with no error
- Theurge case: `rbtdrc_udp_data_exfil_blocked` in tadmor-correlated section

### Acceptance

- Both ifrit selectors implemented and registered
- Both theurge cases registered in tadmor-correlated section
- Theurge builds, unit tests pass
- Full tadmor crucible green (44 + 2 = 46 cases)

### advanced-adversarial-probe-tests (₢A1AAn) [complete]

**[260404-1208] complete**

## Character
More intricate test development requiring careful raw socket construction and understanding of kernel security boundaries. Each test probes a distinct defense-in-depth layer.

## Docket

Add three advanced adversarial security tests exercising subtler boundaries.

### Test 3: dns-rebinding

From inside the bottle, rapidly re-resolve an allowed domain to check if the sentry's dnsmasq cache can be influenced through timing. Distinct from the existing cache integrity test (which tests stability across an external attack); this tests whether the bottle can manipulate its own future resolutions.

- New ifrit sortie: `sortie_dns_rebinding`
- Resolve example.com, wait, re-resolve, compare IPs — verify dnsmasq returns consistent cached results
- Also attempt to resolve with different record types (AAAA, MX) to probe cache behavior
- Theurge case in tadmor-sortie-attacks section

### Test 4: proc-sys-write

From inside the bottle, attempt to write to kernel tunables: `/proc/sys/net/ipv4/ip_forward`, `/proc/sys/net/ipv4/conf/*/rp_filter`, `/proc/sys/net/ipv6/conf/all/disable_ipv6`. If the bottle could modify these, it could undermine the sentry's network posture.

- New ifrit sortie: `sortie_proc_sys_write`
- Attempt to write each tunable, expect permission denied
- Complements existing `ns-capability-escape` (which tests network namespaces)
- Theurge case in tadmor-sortie-attacks section

### Test 5: tcp-rst-connection-hijack

From inside the bottle, forge TCP RST packets targeting the sentry's upstream DNS connection. Uses raw sockets (like existing raw socket test) but targets connection disruption rather than protocol smuggling.

- New ifrit sortie: `sortie_tcp_rst_hijack`
- Craft RST packet aimed at sentry IP, DNS port 53
- Verify sentry DNS continues working after the attempt (coordinated: attack from bottle, verify from writ)
- Theurge case in tadmor-coordinated-integrity or tadmor-sortie-attacks section

### Acceptance

- All three ifrit sorties implemented and registered
- All three theurge cases registered
- Theurge builds, unit tests pass
- Full tadmor crucible green (46 + 3 = 49 cases expected)

**[260404-0839] rough**

## Character
More intricate test development requiring careful raw socket construction and understanding of kernel security boundaries. Each test probes a distinct defense-in-depth layer.

## Docket

Add three advanced adversarial security tests exercising subtler boundaries.

### Test 3: dns-rebinding

From inside the bottle, rapidly re-resolve an allowed domain to check if the sentry's dnsmasq cache can be influenced through timing. Distinct from the existing cache integrity test (which tests stability across an external attack); this tests whether the bottle can manipulate its own future resolutions.

- New ifrit sortie: `sortie_dns_rebinding`
- Resolve example.com, wait, re-resolve, compare IPs — verify dnsmasq returns consistent cached results
- Also attempt to resolve with different record types (AAAA, MX) to probe cache behavior
- Theurge case in tadmor-sortie-attacks section

### Test 4: proc-sys-write

From inside the bottle, attempt to write to kernel tunables: `/proc/sys/net/ipv4/ip_forward`, `/proc/sys/net/ipv4/conf/*/rp_filter`, `/proc/sys/net/ipv6/conf/all/disable_ipv6`. If the bottle could modify these, it could undermine the sentry's network posture.

- New ifrit sortie: `sortie_proc_sys_write`
- Attempt to write each tunable, expect permission denied
- Complements existing `ns-capability-escape` (which tests network namespaces)
- Theurge case in tadmor-sortie-attacks section

### Test 5: tcp-rst-connection-hijack

From inside the bottle, forge TCP RST packets targeting the sentry's upstream DNS connection. Uses raw sockets (like existing raw socket test) but targets connection disruption rather than protocol smuggling.

- New ifrit sortie: `sortie_tcp_rst_hijack`
- Craft RST packet aimed at sentry IP, DNS port 53
- Verify sentry DNS continues working after the attempt (coordinated: attack from bottle, verify from writ)
- Theurge case in tadmor-coordinated-integrity or tadmor-sortie-attacks section

### Acceptance

- All three ifrit sorties implemented and registered
- All three theurge cases registered
- Theurge builds, unit tests pass
- Full tadmor crucible green (46 + 3 = 49 cases expected)

### network-path-verification (₢A1AAo) [complete]

**[260404-1232] complete**

## Character
Focused ifrit sortie development — two tests probing whether the sentry's data path and stateful firewall actually work as every other test assumes.

## Docket

Add two ifrit sorties testing the network data path and connection tracking state.

### Test: http-end-to-end

Full HTTP GET from inside the bottle to example.com (allowed domain). Verify a 200 response with recognizable body content ("Example Domain"). Distinct from existing tcp443-connect and cidr-all-ports-allowed which only verify TCP handshake — this proves NAT masquerade correctly returns actual data, not just SYN/ACK.

- New ifrit sortie: `sortie_http_end_to_end`
- Unilateral sortie (ifrit resolves + connects, no theurge coordination needed)
- Resolve example.com via getent to get IP, connect to IP on port 80 (plain HTTP — no TLS in ifrit)
- **Do not use `http_get_raw` as-is** — it uses the host param for both TCP address and Host header (hardcoded at line 117), and only reads first 200 bytes discarding body. Build a custom HTTP request: TCP connect to the resolved IP, send `GET / HTTP/1.0\r\nHost: example.com\r\n\r\n`, read full response, verify status 200 and body contains "Example Domain"
- Theurge case in tadmor-sortie-attacks section

### Test: conntrack-spoofed-ack

From inside the bottle, send a raw TCP ACK packet (without prior SYN) to an allowed CIDR IP on port 80. The sentry's RELATED,ESTABLISHED rule should drop it because no connection was established. If it passes through, the stateful firewall is misconfigured.

- New ifrit sortie: `sortie_conntrack_spoofed_ack`
- Resolve example.com via getent to get an allowed-CIDR IP (no theurge coordination)
- New `build_tcp_ack` helper needed (like existing `build_tcp_syn` with SYN 0x002 and `build_tcp_rst` with RST 0x004 — ACK flag is 0x010)
- Build IP header via existing `build_ip_header`, send via `send_raw_ip_and_listen`
- Expect Ok(false) — no response means conntrack correctly dropped the stateless ACK
- Ok(true) means BREACH — packet bypassed stateful firewall
- Theurge case in tadmor-sortie-attacks section

### Acceptance

- Both ifrit sorties implemented and registered
- Both theurge cases registered in tadmor-sortie-attacks
- Theurge builds, unit tests pass
- Full tadmor crucible green (49 + 2 = 51 cases expected)

**[260404-1212] rough**

## Character
Focused ifrit sortie development — two tests probing whether the sentry's data path and stateful firewall actually work as every other test assumes.

## Docket

Add two ifrit sorties testing the network data path and connection tracking state.

### Test: http-end-to-end

Full HTTP GET from inside the bottle to example.com (allowed domain). Verify a 200 response with recognizable body content ("Example Domain"). Distinct from existing tcp443-connect and cidr-all-ports-allowed which only verify TCP handshake — this proves NAT masquerade correctly returns actual data, not just SYN/ACK.

- New ifrit sortie: `sortie_http_end_to_end`
- Unilateral sortie (ifrit resolves + connects, no theurge coordination needed)
- Resolve example.com via getent to get IP, connect to IP on port 80 (plain HTTP — no TLS in ifrit)
- **Do not use `http_get_raw` as-is** — it uses the host param for both TCP address and Host header (hardcoded at line 117), and only reads first 200 bytes discarding body. Build a custom HTTP request: TCP connect to the resolved IP, send `GET / HTTP/1.0\r\nHost: example.com\r\n\r\n`, read full response, verify status 200 and body contains "Example Domain"
- Theurge case in tadmor-sortie-attacks section

### Test: conntrack-spoofed-ack

From inside the bottle, send a raw TCP ACK packet (without prior SYN) to an allowed CIDR IP on port 80. The sentry's RELATED,ESTABLISHED rule should drop it because no connection was established. If it passes through, the stateful firewall is misconfigured.

- New ifrit sortie: `sortie_conntrack_spoofed_ack`
- Resolve example.com via getent to get an allowed-CIDR IP (no theurge coordination)
- New `build_tcp_ack` helper needed (like existing `build_tcp_syn` with SYN 0x002 and `build_tcp_rst` with RST 0x004 — ACK flag is 0x010)
- Build IP header via existing `build_ip_header`, send via `send_raw_ip_and_listen`
- Expect Ok(false) — no response means conntrack correctly dropped the stateless ACK
- Ok(true) means BREACH — packet bypassed stateful firewall
- Theurge case in tadmor-sortie-attacks section

### Acceptance

- Both ifrit sorties implemented and registered
- Both theurge cases registered in tadmor-sortie-attacks
- Theurge builds, unit tests pass
- Full tadmor crucible green (49 + 2 = 51 cases expected)

**[260404-1203] rough**

## Character
Focused ifrit sortie development — two tests probing whether the sentry's data path and stateful firewall actually work as every other test assumes.

## Docket

Add two ifrit sorties testing the network data path and connection tracking state.

### Test: http-end-to-end

Full HTTP GET from inside the bottle to example.com (allowed domain). Verify a 200 response with recognizable body content. Distinct from existing tcp443-connect and cidr-all-ports-allowed which only verify TCP handshake — this proves NAT masquerade correctly returns actual data, not just SYN/ACK.

- New ifrit sortie: `sortie_http_end_to_end`
- Use the existing `http_get_raw` helper already in rbida_sorties.rs
- GET to example.com on port 80 (plain HTTP avoids TLS complexity)
- Verify HTTP status 200 and body contains recognizable content (e.g. "Example Domain")
- Theurge case in tadmor-correlated section (theurge resolves IP on sentry, passes to ifrit)

### Test: conntrack-spoofed-ack

From inside the bottle, send a raw TCP ACK packet (without prior SYN) to an allowed CIDR IP on port 80. The sentry's RELATED,ESTABLISHED rule should drop it because no connection was established. If it passes through, the stateful firewall is misconfigured.

- New ifrit sortie: `sortie_conntrack_spoofed_ack`
- Build TCP ACK packet (not SYN) via raw socket using existing `build_ip_header` + `send_raw_ip_and_listen` helpers
- Send to an allowed CIDR IP (resolved example.com) on port 80
- Expect no response (dropped by conntrack) — response means BREACH
- Theurge case in tadmor-sortie-attacks section

### Acceptance

- Both ifrit sorties implemented and registered
- Both theurge cases registered
- Theurge builds, unit tests pass
- Full tadmor crucible green (49 + 2 = 51 cases expected)

### sentry-self-protection-verification (₢A1AAp) [complete]

**[260404-1317] complete**

## Character
Coordinated test development requiring writ/fiat observation of sentry internals. Tests verify the sentry protects itself and maintains audit trails — probing boundaries visible only from the sentry side.

## Docket

Add three tests verifying sentry self-protection and audit capabilities.

### Test: sentry-egress-lockdown

Coordinated test: via writ, attempt outbound connections from the sentry itself to non-allowed destinations. The sentry's OUTPUT DROP policy should block these. Verifies the sentry can't be used as a pivot point even if an attacker gains code execution on it.

- Theurge coordinated case (no ifrit sortie — pure writ commands)
- **Sentry has no netcat** — available tools: bash, iptables, dnsmasq, iproute2, iputils-ping, dnsutils, procps. Use `timeout 2 bash -c 'echo > /dev/tcp/<ip>/443'` for TCP probes, or `ping -c 1 -W 2 <ip>` for ICMP
- Negative tests: attempt TCP to 1.1.1.1:443, 140.82.121.4:443 from sentry — expect failure
- Positive control: `dig @8.8.8.8 example.com` from sentry — must succeed (proves dnsmasq's egress path works)
- Theurge case in tadmor-coordinated-integrity section

### Test: dnsmasq-query-audit

Coordinated test: after ifrit makes DNS queries (both allowed and blocked), read dnsmasq log and verify the queries appear. Proves the audit trail is intact — blocked queries are logged, not silently dropped.

- Theurge coordinated case: invoke ifrit dns-allowed-example and dns-blocked-google via bark, then read `/var/log/dnsmasq.log` via writ (log path per sentry's `log-facility` config)
- Verify log contains query entries for both example.com and google.com
- Note: brief delay between dnsmasq processing and log flush may require a small sleep or grep retry
- Theurge case in tadmor-coordinated-integrity section

### Test: sentry-udp-non-dns

Ifrit sortie: send UDP datagrams to the sentry IP on ports other than 53 (e.g. 123, 161, 1234, 5353). The sentry's INPUT DROP should block all non-DNS UDP. Complements direct-sentry-probe which only scans TCP ports.

- New ifrit sortie: `sortie_sentry_udp_non_dns`
- Read sentry IP from RBRN_ENCLAVE_SENTRY_IP env var
- UDP probe to sentry IP on multiple non-53 ports, expect no response (timeout)
- Positive control: verify UDP 53 to sentry works via `dig +short @<sentry_ip> example.com`
- Theurge case in tadmor-sortie-attacks section

### Acceptance

- All three theurge cases registered (two coordinated-integrity, one sortie-attacks)
- One new ifrit sortie (sentry-udp-non-dns) implemented and registered
- Theurge builds, unit tests pass
- Full tadmor crucible green (51 + 3 = 54 cases expected, assuming prior pace complete)

**[260404-1212] rough**

## Character
Coordinated test development requiring writ/fiat observation of sentry internals. Tests verify the sentry protects itself and maintains audit trails — probing boundaries visible only from the sentry side.

## Docket

Add three tests verifying sentry self-protection and audit capabilities.

### Test: sentry-egress-lockdown

Coordinated test: via writ, attempt outbound connections from the sentry itself to non-allowed destinations. The sentry's OUTPUT DROP policy should block these. Verifies the sentry can't be used as a pivot point even if an attacker gains code execution on it.

- Theurge coordinated case (no ifrit sortie — pure writ commands)
- **Sentry has no netcat** — available tools: bash, iptables, dnsmasq, iproute2, iputils-ping, dnsutils, procps. Use `timeout 2 bash -c 'echo > /dev/tcp/<ip>/443'` for TCP probes, or `ping -c 1 -W 2 <ip>` for ICMP
- Negative tests: attempt TCP to 1.1.1.1:443, 140.82.121.4:443 from sentry — expect failure
- Positive control: `dig @8.8.8.8 example.com` from sentry — must succeed (proves dnsmasq's egress path works)
- Theurge case in tadmor-coordinated-integrity section

### Test: dnsmasq-query-audit

Coordinated test: after ifrit makes DNS queries (both allowed and blocked), read dnsmasq log and verify the queries appear. Proves the audit trail is intact — blocked queries are logged, not silently dropped.

- Theurge coordinated case: invoke ifrit dns-allowed-example and dns-blocked-google via bark, then read `/var/log/dnsmasq.log` via writ (log path per sentry's `log-facility` config)
- Verify log contains query entries for both example.com and google.com
- Note: brief delay between dnsmasq processing and log flush may require a small sleep or grep retry
- Theurge case in tadmor-coordinated-integrity section

### Test: sentry-udp-non-dns

Ifrit sortie: send UDP datagrams to the sentry IP on ports other than 53 (e.g. 123, 161, 1234, 5353). The sentry's INPUT DROP should block all non-DNS UDP. Complements direct-sentry-probe which only scans TCP ports.

- New ifrit sortie: `sortie_sentry_udp_non_dns`
- Read sentry IP from RBRN_ENCLAVE_SENTRY_IP env var
- UDP probe to sentry IP on multiple non-53 ports, expect no response (timeout)
- Positive control: verify UDP 53 to sentry works via `dig +short @<sentry_ip> example.com`
- Theurge case in tadmor-sortie-attacks section

### Acceptance

- All three theurge cases registered (two coordinated-integrity, one sortie-attacks)
- One new ifrit sortie (sentry-udp-non-dns) implemented and registered
- Theurge builds, unit tests pass
- Full tadmor crucible green (51 + 3 = 54 cases expected, assuming prior pace complete)

**[260404-1205] rough**

## Character
Coordinated test development requiring writ/fiat observation of sentry internals. Tests verify the sentry protects itself and maintains audit trails — probing boundaries visible only from the sentry side.

## Docket

Add three tests verifying sentry self-protection and audit capabilities.

### Test: sentry-egress-lockdown

Coordinated test: via writ, attempt outbound TCP connections from the sentry itself to non-allowed destinations (e.g. 1.1.1.1:443, 140.82.121.4:443). The sentry's OUTPUT DROP policy should block these. Verifies the sentry can't be used as a pivot point even if an attacker gains code execution on it.

- Theurge coordinated case: attempt connections via writ `nc -w 2 -zv <ip> 443`, expect failure
- Test multiple non-allowed IPs to avoid false negatives from routing
- Also verify sentry CAN reach 8.8.8.8:53 (positive control — DNS forwarding must work)
- Theurge case in tadmor-coordinated-integrity section

### Test: dnsmasq-query-audit

Coordinated test: after ifrit makes DNS queries (both allowed and blocked), read `/var/log/dnsmasq.log` via writ and verify the queries appear. Proves the audit trail is intact — blocked queries are logged, not silently dropped.

- Theurge coordinated case: invoke ifrit dns-allowed-example and dns-blocked-google, then read dnsmasq log via writ
- Verify log contains query entries for both example.com and google.com
- Theurge case in tadmor-coordinated-integrity section

### Test: sentry-udp-non-dns

Ifrit sortie: send UDP datagrams to the sentry IP on ports other than 53 (e.g. 123, 161, 1234, 5353). The sentry's INPUT DROP should block all non-DNS UDP. Complements direct-sentry-probe which only scans TCP ports.

- New ifrit sortie: `sortie_sentry_udp_non_dns`
- UDP probe to sentry IP on multiple non-53 ports, expect no response
- Also verify UDP 53 to sentry DOES work (positive control via dig)
- Theurge case in tadmor-sortie-attacks section

### Acceptance

- All three theurge cases registered (two coordinated, one sortie)
- One new ifrit sortie (sentry-udp-non-dns) implemented and registered
- Theurge builds, unit tests pass
- Full tadmor crucible green (51 + 3 = 54 cases expected, assuming prior pace complete)

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 f add-buz-group-to-buk-zipper
  2 e rename-consecration-to-hallmark
  3 d add-vessel-quoin-to-rbs0
  4 L reclaim-rbi-prefix
  5 A mint-rbtd-rbid-prefix-and-workspace
  6 K colophon-manifest-protocol
  7 N mint-writ-fiat-bark-colophons
  8 B theurge-reporting-and-case-framework
  9 O qualify-case-engine
  10 C theurge-container-interaction-layer
  11 D ifrit-binary-with-attack-selector
  12 P qualify-ifrit-vessel-standalone
  13 E first-end-to-end-tadmor-cases
  14 F port-remaining-bash-tadmor-security
  15 R kludge-consecration-override-design
  16 G port-python-sorties-to-ifrit
  17 H port-srjcl-pluml-service-tests
  18 I port-fast-tier-regime-enrollment-validation
  19 J retire-bash-test-and-python-sortie-infrastructure
  20 M deduplicate-ported-test-catalog
  21 S coordinated-arp-sentry-observation
  22 U single-case-runner-and-macro-refactor
  23 W crucible-active-lifecycle-verification
  24 a reimplement-srjcl-websocket-kernel-test
  25 X decompose-theurge-workbench-to-bcg
  26 b tectonic-colophon-reorganization
  27 h crucible-verification-after-colophon-reorg
  28 g marshal-generates-claude-tabtarget-context
  29 Z kludge-bottle-crucible-tabtarget
  30 V fix-three-preexisting-sortie-failures
  31 T novel-coordinated-security-tests
  32 c coordinated-security-integrity-verification
  33 i rbida-selector-const-extraction
  34 k install-procps-in-sentry-image
  35 j replace-anthropic-allowlist-with-example-domains
  36 l crucible-active-check-race-investigation
  37 m egress-control-verification-tests
  38 n advanced-adversarial-probe-tests
  39 o network-path-verification
  40 p sentry-self-protection-verification

fedLAKNBOCDPEFRGHIJMSUWaXbhgZVTcikjlmnop
·x··········xx·xxx··xx·x······xx·xx·xxxx rbtdrc_crucible.rs
··········x··x·x····x·········xxx·x·xxxx rbida_attacks.rs
·x···········x······x········xxx·xx·xxxx rbrn.env
xx···xx···x··xx···x··x···x·xx··········· rbz_zipper.sh
····xxxx··x·xx··x····xx···········x····· main.rs
···············x····x········xxx··x·xxxx rbida_sorties.rs
·x·xx···x·········x··x·····x······xx···· CLAUDE.md
········x···x···xxx··x··x··············· rbtw_workbench.sh
·······x·xx·x··x·x················x····· lib.rs
·x········xx··x·x···x····x·············· rbfd_FoundryDirectorBuild.sh
·x····x·······x······xx·····x······x···· rbob_bottle.sh
·······x········xx···x··xx·············· rbtdrm_manifest.rs
····x··········x····x··x··········x····· Cargo.toml
·············x··xxx····················· rbtb_testbench.sh
··········x··················x···xx····· Dockerfile
·······x········xx···x·················· rbtdtm_manifest.rs
····x···············x··x··········x····· Cargo.lock
···························xx·······x··· rbk-claude-tabtarget-context.md
··············x······x······x··········· rbob_cli.sh
··········xx······················x····· rbrv.env
·········x··x···x······················· rbtdri_invocation.rs
·······x·············x·······x·········· rbtdre_engine.rs
·x·······················x·x············ CLAUDE.consumer.md
·x··················x····x·············· RBSAK-ark_kludge.adoc
·x···············x···x·················· rbtdrf_fast.rs
·x·······x······x······················· rbtdti_invocation.rs
·xx······················x·············· RBS0-SpecTop.adoc
x··························x········x··· buz_zipper.sh
·····································xx· rbk-claude-theurge-ifrit-context.md
··················x········x············ buwz_zipper.sh
·············xx························· rbi-iK.IfritKludge.sh
··········x·······················x····· .dockerignore
··········x··x·························· rbi-iB.IfritBuild.sh
·······x·············x·················· rbtdrd_dummy.rs
·······xx······························· rbtdte_engine.rs
·x·························x············ rblm_cli.sh
·x·······················x·············· README.consumer.md, README.md, rbfc_FoundryCore.sh, rbk-prep-release.md, rbw_workbench.sh
····································x··· buk-claude-context.md
···································x···· RBSHR-HorizonRoadmap.adoc, RBSPV-PodmanVmSupplyChain.adoc, rbv_PodmanVM.sh
··································x····· rbgje01-enshrine-copy.sh
····························x··········· rbw-cKB.KludgeBottle.sh
···························x············ rbq_Qualify.sh, rbw-MG.MarshalGenerate.sh
························x··············· rbte_cli.sh, rbte_engine.sh
·····················x·················· rbtd-s.SingleCase.access-probe.sh, rbtd-s.SingleCase.enrollment-validation.sh, rbtd-s.SingleCase.four-mode.sh, rbtd-s.SingleCase.pluml.sh, rbtd-s.SingleCase.regime-smoke.sh, rbtd-s.SingleCase.regime-validation.sh, rbtd-s.SingleCase.srjcl.sh, rbtd-s.SingleCase.tadmor.sh, rbw-ca.CrucibleActive.sh
··················x····················· butcbe_BureEnvironment.sh, butckk_KickTires.sh, butctt.TestTarget.sh, butt_testbench.sh, buw-st.BukSelfTest.sh, rbt_test_srjcl.py, rbtcbe_BureEnvironment.sh, rbtckk_KickTires.sh, rbtcqa_QualifyAll.sh, rbtd-s.TestSuite.complete.sh, rbtd-s.TestSuite.crucible.sh, rbtd-s.TestSuite.fast.sh, rbtd-s.TestSuite.service.sh, rbw-tf.TestFixture.kick-tires.sh, rbw-tf.TestFixture.qualify-all.sh, rbw-tf.TestFixture.regime-credentials.sh, rbw-to.TestOne.sh, rbw-ts.TestSuite.complete.sh, rbw-ts.TestSuite.crucible.sh, rbw-ts.TestSuite.fast.sh, rbw-ts.TestSuite.service.sh
·················x······················ butcev_ChoiceTypes.sh, butcev_EnforceReport.sh, butcev_GateEnroll.sh, butcev_LengthTypes.sh, butcev_ListTypes.sh, butcev_NumericTypes.sh, butcev_RefTypes.sh, butcrg_RegimeSmoke.sh, rbtcrv_RegimeValidation.sh, rbtd-r.Run.enrollment-validation.sh, rbtd-r.Run.regime-smoke.sh, rbtd-r.Run.regime-validation.sh, rbw-tf.TestFixture.enrollment-validation.sh, rbw-tf.TestFixture.regime-smoke.sh, rbw-tf.TestFixture.regime-validation.sh
················x······················· rbtcap_AccessProbe.sh, rbtcfm_FourMode.sh, rbtcpl_PlumlDiagram.sh, rbtcsj_SrjclJupyter.sh, rbtd-ap.AccessProbe.director.sh, rbtd-ap.AccessProbe.governor.sh, rbtd-ap.AccessProbe.payor.sh, rbtd-ap.AccessProbe.retriever.sh, rbtd-r.Run.access-probe.sh, rbtd-r.Run.four-mode.sh, rbtd-r.Run.pluml.sh, rbtd-r.Run.srjcl.sh, rbw-tf.TestFixture.access-probe.sh, rbw-tf.TestFixture.four-mode.sh, rbw-tf.TestFixture.pluml-diagram.sh, rbw-tf.TestFixture.srjcl-jupyter.sh
··············x························· rbw-cK.Kludge.tadmor.sh, rbw-cO.Ordain.tadmor.sh
·············x·························· rbtcns_TadmorSecurity.sh, rbw-tf.TestFixture.tadmor-security.sh
············x··························· rbtd-r.Run.tadmor.sh
········x······························· launcher.rbtw_workbench.sh, rbtd-b.Build.sh, rbtd-t.Test.sh
······x································· rbw-cb.Bark.pluml.sh, rbw-cb.Bark.srjcl.sh, rbw-cb.Bark.tadmor.sh, rbw-cf.Fiat.pluml.sh, rbw-cf.Fiat.srjcl.sh, rbw-cf.Fiat.tadmor.sh, rbw-cw.Writ.pluml.sh, rbw-cw.Writ.srjcl.sh, rbw-cw.Writ.tadmor.sh
·····x·································· rbw-Ic.IfritClient.tadmor.sh, rbw-ch.Hail.sh, rbw-cr.Rack.sh, rbw-cs.Scry.sh
···x···································· rbi_Image.sh
·x······································ BUS0-BashUtilitiesSpec.adoc, RBRN-RegimeNameplate.adoc, RBSAA-ark_abjure.adoc, RBSAB-ark_about.adoc, RBSAC-ark_conjure.adoc, RBSAG-ark_graft.adoc, RBSAP-ark_plumb.adoc, RBSAS-ark_summon.adoc, RBSAV-ark_vouch.adoc, RBSBC-bottle_create.adoc, RBSCB-CloudBuildPosture.adoc, RBSCC-crucible_charge.adoc, RBSCL-consecration_tally.adoc, RBSDV-director_vouch.adoc, RBSTB-trigger_build.adoc, buv_validation.sh, rbcc_Constants.sh, rbfl_FoundryLedger.sh, rbfr_FoundryRetriever.sh, rbfv_FoundryVerify.sh, rbgc_Constants.sh, rbgja01-discover-platforms.py, rbgja02-syft-per-platform.sh, rbgja03-build-info-per-platform.py, rbgja04-assemble-push-about.sh, rbgjb01-derive-tag-base.sh, rbgjb04-per-platform-pullback.sh, rbgjb05-push-per-platform.sh, rbgjb06-imagetools-create.sh, rbgjb07-push-diags.sh, rbgjm01-mirror-image.sh, rbgjv02-verify-provenance.py, rbgjv03-assemble-push-vouch.sh, rbgm_ManualProcedures.sh, rbob_compose.yml, rbrn_cli.sh, rbrn_regime.sh

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 186 commits)

  1 j replace-anthropic-allowlist-with-example-domains
  2 l crucible-active-check-race-investigation
  3 m egress-control-verification-tests
  4 n advanced-adversarial-probe-tests
  5 o network-path-verification
  6 p sentry-self-protection-verification

123456789abcdefghijklmnopqrstuvwxyz
xx·xxxxxxx·························  j  9c
···············xx··················  l  2c
·················xxxx··············  m  4c
·····················xxx··x········  n  4c
···························xxxx····  o  4c
·······························xxxx  p  4c
```

## Steeplechase

### 2026-04-04 13:17 - ₢A1AAp - W

Implemented three sentry self-protection tests: sentry-egress-lockdown (coordinated writ TCP probes verifying OUTPUT DROP blocks sentry as pivot), dnsmasq-query-audit (coordinated bark+writ verifying both allowed and blocked DNS queries appear in audit log), sentry-udp-non-dns (ifrit sortie probing non-53 UDP ports against INPUT DROP). All 54 tadmor cases green on both kludge and ordained (c260404125932-r260404200037) images.

### 2026-04-04 13:08 - ₢A1AAp - n

Drive ordained hallmark c260404125932-r260404200037 into tadmor nameplate for final crucible verification

### 2026-04-04 12:38 - ₢A1AAp - n

Drive kludge hallmark k260404123835-e87fe15c into tadmor nameplate for sentry self-protection testing

### 2026-04-04 12:38 - ₢A1AAp - n

Implement three sentry self-protection tests: sentry-egress-lockdown (coordinated writ TCP probes verifying OUTPUT DROP), dnsmasq-query-audit (coordinated bark+writ verifying DNS audit trail), sentry-udp-non-dns (ifrit sortie probing non-53 UDP ports). Register all three in tadmor sections.

### 2026-04-04 12:32 - ₢A1AAo - W

Added two network path verification tests: http-end-to-end (full HTTP GET proving NAT masquerade returns actual data with 200 + body content) and conntrack-spoofed-ack (raw TCP ACK without prior SYN, verifying stateful firewall drops it). Built build_tcp_ack helper. All 51 tadmor cases green. Also strengthened theurge-ifrit context doc to make single-case-first the explicit standard operating procedure for all crucible verification.

### 2026-04-04 12:32 - ₢A1AAo - n

Strengthen crucible iteration guidance: reframe single-case-first as standard operating procedure (not just debugging), add explicit verification workflow to Adding a New Test section

### 2026-04-04 12:18 - ₢A1AAo - n

Drive kludge hallmark k260404121747-0f0a36a6 into tadmor nameplate for network path verification testing

### 2026-04-04 12:17 - ₢A1AAo - n

Implement two network path verification tests: http-end-to-end (full HTTP GET proving NAT masquerade returns data) and conntrack-spoofed-ack (raw TCP ACK without prior SYN, verifying stateful firewall drops it). Includes build_tcp_ack helper and theurge case registration.

### 2026-04-04 12:08 - ₢A1AAn - W

Implemented three advanced adversarial probe tests: dns-rebinding (dnsmasq cache manipulation via re-resolution timing and multi-record-type consistency), proc-sys-write (kernel tunable write attempts via file, sysctl, and truncate-open on 7 paths), tcp-rst-hijack (coordinated TCP RST forgery targeting sentry DNS with post-attack DNS verification). All 49 tadmor cases green. Also added iteration strategy guidance to theurge/ifrit context doc.

### 2026-04-04 12:05 - Heat - S

sentry-self-protection-verification

### 2026-04-04 12:03 - Heat - S

network-path-verification

### 2026-04-04 10:38 - ₢A1AAn - n

Add iteration strategy guidance noting full tadmor fixture takes ~10min, directing single-case debugging first

### 2026-04-04 10:25 - ₢A1AAn - n

Drive kludge hallmark k260404102425-e2922b9e into tadmor nameplate for adversarial probe testing

### 2026-04-04 10:24 - ₢A1AAn - n

Implement three advanced adversarial probe tests: dns-rebinding (cache manipulation via re-resolution timing), proc-sys-write (kernel tunable write attempts), tcp-rst-hijack (coordinated TCP RST forgery targeting sentry DNS). All ifrit sorties, attack enum variants, and theurge crucible cases wired and registered.

### 2026-04-04 10:15 - ₢A1AAm - W

Added Folio column to generated tabtarget context table (buz_emit_context), distinguishing imprint vs param1 channel invocation patterns. Fixes spook where Claude constructed wrong tabtarget paths for param1 colophons (e.g., rbw-cKB.KludgeBottle.tadmor.sh instead of rbw-cKB.KludgeBottle.sh tadmor). Added Folio and Channel to BUK vocabulary in buk-claude-context.md.

### 2026-04-04 10:06 - ₢A1AAm - n

Add Folio column to generated tabtarget context table, distinguishing imprint vs param1 channel invocation patterns. Spook fix: Claude was constructing wrong tabtarget paths for param1 colophons (e.g., rbw-cKB) because the table lacked channel information.

### 2026-04-04 09:56 - ₢A1AAm - n

Drive kludge hallmark k260404095639-7abf36cf into tadmor nameplate for egress control verification testing

### 2026-04-04 09:56 - ₢A1AAm - n

Implement two new egress control verification tests: udp-non-dns-blocked (UDP to non-allowed IPs on non-DNS ports) and cidr-all-ports-allowed (TCP to allowed CIDR on multiple ports confirms protocol-agnostic allowlist). Both ifrit attacks and theurge crucible cases wired and registered in tadmor-correlated section.

### 2026-04-04 09:49 - ₢A1AAl - W

Added --wait to compose charge to eliminate race between compose up and active-check assertion. Documented two podman-compose gaps in horizon roadmap: missing --wait support (issues #710, #1329) and GHCR-targeted VM image supply chain needing GAR migration. Moved podman files to FUTURE subdirectory and removed stale CLAUDE.md acronym mappings.

### 2026-04-04 09:46 - ₢A1AAl - n

Add --wait to compose charge, document podman-compose and VM supply chain gaps in horizon roadmap, move podman files to FUTURE subdirectory and remove from CLAUDE.md

### 2026-04-04 09:10 - Heat - n

Wire RBSHR into RBS0 (was missing include), fix stale RBSCB include filename (CloudBuildRoadmap→CloudBuildPosture), add Network Security Model horizon entry documenting CIDR breadth weakness with CDN-hosted domains and three mitigation options

### 2026-04-04 08:51 - Heat - n

Add theurge/ifrit context guide pointer to consumer-facing CLAUDE.md

### 2026-04-04 08:50 - Heat - n

Add theurge/ifrit crucible testing context guide covering architecture, iteration loop (kludge/charge/test/ordain), and how to add new security test cases

### 2026-04-04 08:39 - Heat - S

advanced-adversarial-probe-tests

### 2026-04-04 08:38 - Heat - S

egress-control-verification-tests

### 2026-04-04 08:33 - ₢A1AAj - W

Replaced anthropic.com/claude.ai/claude.com allowlist with RFC 2606 example.com/example.org across tadmor nameplate, ifrit attacks, and theurge crucible tests. Added DnsAllowedExampleOrg variant exercising list treatment. Fixed two downstream issues from domain switch: expanded CIDR to cover Cloudflare range (104.16.0.0/12), changed sortie control domain to evil-c2-server.invalid. Reverted enshrine script from openssl to sha256sum (skopeo reliquary lacks openssl). Relocated ifrit crate from Tools/rbk/rbid/ into rbev-vessels/rbev-bottle-ifrit/ to co-locate Dockerfile and build context, enabling first-ever Cloud Build ordain of the bottle vessel. Ordained hallmark c260404081037-r260404151140 driven into tadmor, 44/44 crucible cases green.

### 2026-04-04 08:20 - ₢A1AAj - n

Drive ordained bottle hallmark c260404081037-r260404151140 into tadmor nameplate

### 2026-04-04 08:10 - ₢A1AAj - n

Drive kludge hallmark k260404081007-fd251bdf (ifrit relocated to vessel directory) into tadmor nameplate

### 2026-04-04 08:10 - ₢A1AAj - n

Relocate ifrit crate from Tools/rbk/rbid/ into rbev-vessels/rbev-bottle-ifrit/ so Dockerfile and build context are co-located, matching sentry pattern and enabling Cloud Build ordain

### 2026-04-04 07:55 - ₢A1AAj - n

Revert enshrine script from openssl to sha256sum — openssl not available in skopeo reliquary container, broke first-ever enshrine of rust:slim-bookworm

### 2026-04-03 18:54 - ₢A1AAj - n

Drive kludge hallmark k260403185422-c5844534 (with sortie control domain fix) into tadmor nameplate

### 2026-04-03 18:54 - ₢A1AAj - n

Fix two test failures from domain switch: expand CIDR to cover Cloudflare range (104.16.0.0/12) for example.com, change sortie control domain from evil-c2-server.example.com to evil-c2-server.invalid (example.com is now an allowed domain)

### 2026-04-03 18:52 - Heat - S

crucible-active-check-race-investigation

### 2026-04-03 18:36 - ₢A1AAj - n

Drive kludge hallmark k260403183622-aa84c063 (with updated ifrit example.com/example.org selectors) into tadmor nameplate

### 2026-04-03 18:27 - ₢A1AAj - n

Replace anthropic.com/claude.ai/claude.com allowlist with RFC 2606 example.com/example.org in tadmor nameplate, ifrit attacks, and theurge crucible tests. Added DnsAllowedExampleOrg variant to exercise list treatment.

### 2026-04-03 18:17 - ₢A1AAk - W

Added procps to sentry Dockerfile, replaced fragile /proc/*/comm hack with pidof dnsmasq in sentry integrity test. Ordained new sentry hallmark c260403180049-r260404010221 and drove into tadmor nameplate. 43 tadmor cases green.

### 2026-04-03 18:06 - ₢A1AAk - n

Drive ordained sentry hallmark c260403180049-r260404010221 (with procps) into tadmor nameplate

### 2026-04-03 18:00 - ₢A1AAk - n

Add procps to sentry Dockerfile and replace /proc/*/comm hack with pidof dnsmasq in sentry integrity test

### 2026-04-03 17:34 - ₢A1AAi - W

Extracted 35 const RBIDA_SEL_* definitions for all pre-existing attack selector strings, replacing duplicated kebab-case literals across from_selector, selector, and all_selectors methods. Zero behavioral change, RCG String Boundary Discipline now satisfied for all 37 selectors.

### 2026-04-03 17:34 - ₢A1AAi - n

Enforce RCG String Boundary Discipline for all attack selector strings — extract remaining inline literals to RBIDA_SEL_ constants, completing single-definition-rule compliance across from_selector, selector, and all_selectors.

### 2026-04-03 17:09 - ₢A1AAc - W

All 3 coordinated integrity tests implemented and passing: sentry process integrity post-attack (exercises dns-blocked-google, direct-arp-poison, proto-smuggle-rawsock then verifies dnsmasq/iptables/interfaces unchanged), DNS cache integrity under forged response attack, MAC flood bridge resilience. Fixed sentry integrity to use /proc/*/comm instead of pgrep, filtered BUK log headers from writ snapshots. Tadmor at 43 cases, full crucible suite green (117 cases).

### 2026-04-03 12:47 - Heat - r

moved A1AAi after A1AAc

### 2026-04-03 12:46 - Heat - r

moved A1AAk after A1AAc

### 2026-04-03 12:45 - ₢A1AAc - n

Fix sentry integrity test: replace pgrep with /proc/*/comm (sentry lacks procps), filter BUK log headers from writ output before snapshot comparison. All 43 tadmor cases pass.

### 2026-04-03 12:44 - Heat - S

install-procps-in-sentry-image

### 2026-04-03 12:44 - Heat - S

replace-anthropic-allowlist-with-example-domains

### 2026-04-03 11:56 - ₢A1AAc - n

Drive kludge hallmark k260403115600-516b1782 into tadmor nameplate for coordinated integrity testing

### 2026-04-03 11:55 - ₢A1AAc - n

Add two ifrit coordinated primitives (dns-forge-response, mac-flood-bridge) and three theurge coordinated integrity cases (sentry process integrity, DNS cache integrity, MAC flood bridge resilience). New tadmor-coordinated-integrity section.

### 2026-04-03 11:54 - Heat - S

rbida-selector-const-extraction

### 2026-04-03 11:43 - ₢A1AAT - W

Added three novel unilateral security tests: route table manipulation (ip route replace/add/del blocked), enclave subnet escape (probes outside /24 unreachable), and DNAT entry port reflection (sentry entry ports unreachable from enclave). New tadmor-unilateral-novel section. Tadmor grows from 37 to 40 cases, all passing.

### 2026-04-03 11:34 - ₢A1AAT - n

Drive kludge hallmark k260403113424-0e9d647f into tadmor nameplate for novel attack testing

### 2026-04-03 11:34 - ₢A1AAT - n

Add three novel unilateral security tests: route table manipulation, enclave subnet escape, and DNAT entry port reflection. New ifrit attack variants, sortie implementations, and tadmor-unilateral-novel crucible section.

### 2026-04-03 11:13 - ₢A1AAZ - W

Added rbw-cKB KludgeBottle tabtarget with param1 dispatch, enrolling in crucible colophon group. Thin delegation to rbob_kludge with BCG-compliant sentinel. Qualify-fast and 136 tests pass.

### 2026-04-03 11:12 - ₢A1AAZ - n

Add rbw-cKB KludgeBottle tabtarget: param1 colophon enrollment, 3-line launcher, delegation function to rbob_kludge with BCG-compliant sentinel

### 2026-04-03 11:00 - ₢A1AAg - W

Added per-colophon description as 6th required arg to buz_enroll, made buz_group store metadata, added buz_emit_context to generate markdown from zipper registry. New rbw-MG marshal command generates rbk-claude-tabtarget-context.md (55 colophons across 11 groups). Qualify freshness gate ensures generated file stays current. Both CLAUDE files @-include the generated context, eliminating hand-maintained colophon tables. Updated buwz_zipper.sh for new 6-arg contract. 135 tests pass (9 BUK + 51 theurge unit + 75 fast suite).

### 2026-04-03 10:59 - ₢A1AAg - n

Added buz_describe as 6th required arg to buz_enroll, buz_group stores metadata, buz_emit_context generates markdown from zipper registry. New rbw-MG marshal command generates rbk-claude-tabtarget-context.md. Qualify freshness gate. Both CLAUDE files @-include generated context, replacing hand-maintained colophon tables. 135 tests pass.

### 2026-04-03 10:37 - ₢A1AAh - W

Crucible test suite (117 cases) passed clean after tectonic colophon reorganization. All live colophon usage paths verified: charge/quench/writ/fiat/bark operations against tadmor, srjcl, and pluml containers. No stale colophon references in runtime paths.

### 2026-04-03 10:26 - ₢A1AAb - W

Reorganized rbw colophon namespace from actor-organized (Director/Retriever/Payor/Governor/Ledger) to domain-object-organized categories (accounts/crucible/depot/guide/hallmark/ifrit/image/marshal/theurge). 25 tabtarget renames, zipper groups restructured from 13 to 11, Rust colophon manifest updated, all documentation references updated across CLAUDE.consumer.md, README.consumer.md, README.md, RBS0, RBSAK, rbk-prep-release, and bash source comments. Build passes, 51 unit tests pass, 75 fast suite cases pass, qualify-fast clean.

### 2026-04-03 10:25 - ₢A1AAb - n

Reorganize rbw colophon namespace from actor-organized (Director/Retriever/Payor/Governor) to domain-object-organized (accounts/crucible/depot/hallmark/image/theurge). 25 tabtarget renames, zipper groups and enrollments restructured, Rust manifest updated, all documentation references updated. Build passes, 51 unit tests pass, fast suite 75 cases pass, qualify-fast clean.

### 2026-04-03 10:13 - ₢A1AAd - W

Added [[rbtga_vessel]] quoin to RBS0 with complete linked term structure: attribute references (:rbtga_vessel:, :rbtga_vessel_s:) in mapping section, anchor and definition in Ark Definitions section capturing vessel as a named container image specification with mode/ark/hallmark relationships. Fixed stale {rbtgi_vessel} reference.

### 2026-04-03 10:13 - ₢A1AAd - n

Add [[rbtga_vessel]] quoin to RBS0: attribute references in mapping section, anchor and definition in Ark Definitions section. Fix stale {rbtgi_vessel} reference to {rbtga_vessel}.

### 2026-04-03 10:09 - Heat - d

paddock curried: fix stale consecration→hallmark in ifrit build model section

### 2026-04-03 10:04 - ₢A1AAe - W

Renamed domain concept Consecration to Hallmark across 56 files: 6 quoin families in RBS0, 15 AsciiDoc specs, 15 bash scripts, 9 cloud build jobs, 3 Python files, 3 Rust files, 3 env files, compose, consumer docs, README, CLAUDE.md. Build passes, 51 unit tests pass, fast suite 75 cases pass. Survivors are expected: filename RBSCL-consecration_tally.adoc and tabtarget frontispieces (deferred to tectonic-colophon-reorganization pace).

### 2026-04-03 09:56 - ₢A1AAe - n

Rename domain concept Consecration to Hallmark across entire system — quoin definitions, AsciiDoc specs, bash variables/functions/strings, Rust constants/literals, env files, cloud build jobs, Python scripts, consumer docs. Build passes, 51 unit tests pass, fast suite 75 cases pass.

### 2026-04-03 09:41 - ₢A1AAf - W

Added buz_group no-op function to BUK zipper and declared 13 pre-rename colophon group categories in rbz_zipper.sh

### 2026-04-03 09:41 - ₢A1AAf - n

Add buz_group no-op for colophon taxonomy and declare RBK group categories

### 2026-04-03 09:35 - Heat - S

crucible-verification-after-colophon-reorg

### 2026-04-03 09:28 - Heat - S

marshal-generates-claude-tabtarget-context

### 2026-04-03 08:59 - Heat - S

add-buz-group-to-buk-zipper

### 2026-04-03 08:50 - Heat - S

rename-consecration-to-hallmark

### 2026-04-03 08:30 - Heat - S

add-vessel-quoin-to-rbs0

### 2026-04-02 23:46 - ₢A1AAX - W

Decomposed theurge workbench into BCG module structure. Created rbte_engine.sh (kindle/sentinel + 6 public functions) and rbte_cli.sh (differential furnish + dispatch). Reduced rbtw_workbench.sh to pure router. Deleted zrbtw_resolve_manifest — ZRBZ_COLOPHON_MANIFEST passed directly to binary. Fixed drift: RBTDRM_COLOPHON_KLUDGE rbw-ak to rbw-LK. All 51 unit tests, 75 fast cases, qualify-fast green.

### 2026-04-02 23:46 - ₢A1AAX - n

Extract theurge workbench inline logic into rbte_cli/rbte_engine BCG modules with differential furnish, fix kludge colophon to rbw-LK

### 2026-04-02 23:40 - Heat - T

rename-crucible-active-to-crucible-is-charged

### 2026-04-02 23:38 - Heat - S

coordinated-security-integrity-verification

### 2026-04-02 23:35 - ₢A1AAa - W

Reimplemented srjcl WebSocket kernel test in pure Rust. Replaced Python script + external Docker container with native tungstenite WebSocket client in theurge. Test creates Jupyter session via curl, connects via WebSocket, executes Python code, verifies stream output. Added tungstenite 0.24 as theurge's first external dependency. Removed RBTDRC_SRJCL_TEST_IMAGE constant. All 3 srjcl cases pass, full crucible suite green (120 cases).

### 2026-04-02 23:35 - ₢A1AAa - n

Replace Docker-based Python WebSocket test with native Rust tungstenite implementation for SRJCL kernel execution

### 2026-04-02 23:23 - Heat - r

moved A1AAZ after A1AAb

### 2026-04-02 23:23 - Heat - r

moved A1AAb after A1AAY

### 2026-04-02 23:22 - Heat - r

moved A1AAX after A1AAa

### 2026-04-02 23:20 - Heat - n

Centralize hardcoded rbw-DO colophon literals in test code through RBTDRM_COLOPHON_ORDAIN constant, preparing for tectonic colophon reorganization

### 2026-04-02 23:15 - ₢A1AAV - W

Fixed 3 pre-existing sortie failures plus bonus ARP fix. Root causes: (1) ifrit binary lacked CAP_NET_RAW effective capability — added setcap+libcap2-bin to Dockerfile, (2) rp_filter check wrong for container architecture — removed (sentry enforces via iptables), (3) kernel tunnel pseudo-interfaces falsely flagged — expanded allowlist, (4) get_interface_info picked tunnel iface with 16-byte MAC — restricted to eth*, (5) ARP sortie verdict wrong — AF_PACKET expected with NET_RAW, sentry resilience verified by coordinated tests. Also removed rbtdre_display_name obfuscation so case names display as raw function names. Tadmor suite: 37/37 pass.

### 2026-04-02 23:14 - Heat - r

moved A1AAa to first

### 2026-04-02 23:14 - Heat - S

tectonic-colophon-reorganization

### 2026-04-02 23:12 - Heat - S

reimplement-srjcl-websocket-kernel-test

### 2026-04-02 22:52 - ₢A1AAV - n

Update tadmor consecration after ARP verdict fix rebuild

### 2026-04-02 22:52 - ₢A1AAV - n

Fix sortie_direct_arp_poison verdict: AF_PACKET availability is expected with NET_RAW, sentry resilience verified by coordinated tests

### 2026-04-02 22:47 - Heat - S

kludge-bottle-crucible-tabtarget

### 2026-04-02 22:41 - ₢A1AAV - n

Update tadmor consecration after ifrit rebuild with get_interface_info fix

### 2026-04-02 22:41 - ₢A1AAV - n

Fix get_interface_info to select eth* interfaces only (tunnel pseudo-ifaces have 16-byte MACs)

### 2026-04-02 22:41 - ₢A1AAV - n

Remove rbtdre_display_name obfuscation — case names now display as raw function names

### 2026-04-02 22:26 - ₢A1AAV - n

Update tadmor consecration after ifrit vessel rebuild with setcap NET_RAW

### 2026-04-02 22:24 - Heat - n

WIP disk check changes from concurrent session

### 2026-04-02 22:23 - ₢A1AAV - n

Fix three sortie failures: setcap NET_RAW on ifrit binary, remove rp_filter check (sentry enforces via iptables), expand interface allowlist for kernel tunnel pseudo-interfaces

### 2026-04-02 22:22 - Heat - S

rename-crucible-active-to-crucible-is-charged

### 2026-04-02 22:08 - Heat - r

moved A1AAV before A1AAX

### 2026-04-02 18:42 - ₢A1AAW - W

Fixed sessile profile omission in rbob_quench (and charge pre-cleanup) so bottle container is torn down; added rbw-ca lifecycle assertions in theurge suite runner — hard fail if crucible not active after charge, warning if still active after quench. Both assertions confirmed working in crucible run.

### 2026-04-02 18:36 - ₢A1AAW - n

Fix sessile profile omission in quench compose-down; add rbw-ca lifecycle assertions after charge (hard fail) and after quench (warning) in theurge suite runner

### 2026-04-02 18:20 - Heat - r

moved A1AAX after A1AAW

### 2026-04-02 18:18 - Heat - S

decompose-theurge-workbench-to-bcg

### 2026-04-02 18:17 - Heat - r

moved A1AAW to first

### 2026-04-02 18:15 - ₢A1AAU - W

case!() macro replaces all 126 case registrations with compiler-enforced unique names derived from function names via stringify!(). Display-time transform strips module prefix and converts underscores to hyphens. Single-case runner (rbtd single) with rbw-ca CrucibleActive tabtarget for charge detection. Fixture name constants (RBTDRM_FIXTURE_*) eliminate magic strings. BCG-principled rbob_charged_predicate with param1-channel zipper enrollment.

### 2026-04-02 18:14 - Heat - n

Remove dead retire-struct tests and unused imports that broke JJK test compilation after struct deletion

### 2026-04-02 18:12 - ₢A1AAU - n

Charge check runs before both list and run modes in single-case runner

### 2026-04-02 18:12 - Heat - S

crucible-active-lifecycle-verification

### 2026-04-02 18:03 - ₢A1AAU - n

Remove magic 'fast' string from manifest tests — use rbtdtm_manifest_for helper consistently

### 2026-04-02 18:01 - ₢A1AAU - n

CrucibleActive tabtarget (rbw-ca, param1 channel) with rbob_charged_predicate; fixture name constants eliminate magic strings; single-case runner checks charge state via tabtarget invocation; removed non-crucible SingleCase tabtargets

### 2026-04-02 17:20 - ₢A1AAU - n

case!() macro replaces all 126 case registrations with compiler-enforced unique names; single-case runner subcommand and tabtargets for all fixtures

### 2026-04-02 17:01 - Heat - r

moved A1AAT to last

### 2026-04-02 16:05 - ₢A1AAS - W

Added coordinated ARP attack test pattern: 2 new ifrit attack primitives (arp-send-gratuitous, arp-send-gateway-poison), 3 theurge-observed test cases in new tadmor-coordinated-attacks section verifying sentry ARP table immunity to bottle-originated L2 manipulation. Fixed pre-existing socket2 features=[all] for Type::RAW. Added dirty-tree guard to kludge build. All 3 coordinated cases pass SECURE. Tadmor cases: 34→37.

### 2026-04-02 16:05 - ₢A1AAS - n

Append git-describe provenance to kludge consecration tag for commit traceability

### 2026-04-02 16:04 - Heat - S

fix-three-preexisting-sortie-failures

### 2026-04-02 16:04 - Heat - S

single-case-runner-and-macro-refactor

### 2026-04-02 15:46 - ₢A1AAS - n

Kludge consecration k260402154611 for ifrit bottle with coordinated ARP attack variants

### 2026-04-02 15:46 - ₢A1AAS - n

Add coordinated ARP attack primitives (arp-send-gratuitous, arp-send-gateway-poison) to ifrit, 3 coordinated test cases to theurge (tadmor-coordinated-attacks section), fix pre-existing socket2 features=[all] for Type::RAW, add dirty-tree guard to kludge build

### 2026-04-02 15:09 - Heat - S

novel-coordinated-security-tests

### 2026-04-02 15:08 - Heat - S

coordinated-arp-sentry-observation

### 2026-04-02 10:41 - ₢A1AAM - W

Deduplication audit of 34 tadmor crucible cases across bash-ported ifrit-attacks (15) and python-ported sortie-attacks (11) plus theurge-only sections (8). No true redundancy found — partial overlap on 8.8.8.8:53 between dns-block-direct and net-forbidden-cidr is complementary (DNS interception bypass vs general CIDR egress). All cases are load-bearing with distinct attack hypotheses. Catalog confirmed clean as-is.

### 2026-04-02 10:38 - ₢A1AAJ - W

Retired bash test infrastructure: removed rbtb_testbench.sh, rbts/ case files, python srjcl test, old rbw-tf/ts/to tabtargets and zipper enrollments. Replaced suite tabtargets with rbtd-s colophon routing through RBTW workbench with manifest resolver and sequential fixture dispatch. Created BUK self-test (butt_testbench.sh, 9 cases) with buw-st enrollment. Updated CLAUDE.md. Inventory: 122 theurge cases across 8 fixtures, 51 unit tests, qualification clean.

### 2026-04-02 10:37 - ₢A1AAJ - n

Retire bash test infrastructure scaffolding: removed rbtb_testbench, rbts/ case files, python srjcl test, old fixture/suite/test-one tabtargets (rbw-tf, rbw-ts, rbw-to), and zipper enrollments. Replaced suite tabtargets with rbtd-s colophon routing through RBTW workbench with extracted manifest resolver and sequential fixture dispatch. Created BUK self-test (butt_testbench.sh) with kick-tires and bure-tweak cases (9 cases) routed via buw-st enrollment. Updated CLAUDE.md prefix mappings and test execution table. All 51 unit tests, 75 fast cases, 9 BUK self-test cases, and qualification check pass.

### 2026-04-02 10:19 - ₢A1AAI - W

Ported all 3 fast-tier bash test fixtures to theurge: enrollment-validation (47 cases), regime-validation (21 cases), regime-smoke (7 cases). Created rbtdrf_fast.rs with data-driven bash subprocess execution for BUV enrollment/vet tests, regime kindle/enforce negative+positive tests, and direct tabtarget invocation for smoke tests. Extended manifest system to accept fast fixtures with no colophon requirements (Option return type). Created 3 rbtd-r.Run tabtargets, retired 9 bash case files, 3 rbw-tf fixture tabtargets, and testbench enrollments. All 75 cases verified live with parity against bash originals. 51 unit tests, 75/75 fast cases.

### 2026-04-02 10:18 - ₢A1AAI - n

Port all 3 fast-tier bash test fixtures to theurge: enrollment-validation (47 cases), regime-validation (21 cases), regime-smoke (7 cases). Created rbtdrf_fast.rs with data-driven bash subprocess execution for BUV enrollment tests and regime kindle/enforce tests, plus direct tabtarget invocation for smoke tests. Extended manifest to accept fast fixtures with no colophon requirements. Retired 9 bash case files, 3 bash fixture tabtargets, and testbench enrollments. All 75 cases verified live with parity against bash originals. 51 unit tests pass.

### 2026-04-02 09:55 - ₢A1AAH - W

Ported all 4 remaining bash test fixtures to theurge: srjcl-jupyter (3 cases), pluml-diagram (5 cases), four-mode 15-step integration (1 case), access-probe (4 cases). Extended theurge from crucible-only to general test orchestrator with conditional charge/quench lifecycle, per-fixture manifests, global and imprint-scoped tabtarget discovery, BURV fact reading, docker helpers, and extra_env forwarding. Created rbtd-ap wrapper tabtargets for access-probe roles. Fixed two bugs: openssl base64 decode needing trailing newline in foundry, BURV output current/ subdirectory convention. Retired bash case files and fixture tabtargets. All 13 cases verified live. 50 unit tests, 85/85 fast suite.

### 2026-04-02 09:34 - ₢A1AAH - n

Fix two bugs blocking four-mode: openssl base64 decode needs trailing newline in foundry, BURV fact reader needs current/ subdirectory per BUK dispatch convention

### 2026-04-02 09:02 - ₢A1AAH - n

Fix access-probe workbench route: source regime modules (rbrr_regime.sh, rbrp_regime.sh) for kindle functions — all 4 access-probe cases now passing

### 2026-04-02 08:51 - ₢A1AAH - n

Retire bash case files and fixture tabtargets for srjcl-jupyter, pluml-diagram, four-mode, and access-probe — all now run via theurge

### 2026-04-02 08:48 - ₢A1AAH - n

Extend theurge to support non-crucible fixtures: invocation layer (global discovery, BURV output path, extra_env, imprint invoke, fact reader), per-fixture manifest verification, conditional charge/quench lifecycle, four-mode 15-step case with docker helpers, access-probe 4 cases with rbtd-ap wrapper tabtargets

### 2026-04-02 07:46 - ₢A1AAH - n

Port srjcl-jupyter (3 cases) and pluml-diagram (5 cases) to theurge crucible with host-side HTTP/curl helpers, nameplate-dispatched section selection, 30s service readiness delay, and run tabtargets for both nameplates

### 2026-04-02 07:36 - ₢A1AAG - W

Ported all 11 python sorties (rbtis_*.py) to Rust ifrit attack variants in new rbida_sorties.rs module. Added socket2+libc dependencies. Simple tier (dns-exfil-subdomain, meta-cloud-endpoint, net-forbidden-cidr, direct-sentry-probe, ns-capability-escape) uses std TcpStream/UdpSocket/Command. Raw socket tier (icmp-exfil-payload, net-ipv6-escape, net-srcip-spoof, proto-smuggle-rawsock, net-fragment-evasion, direct-arp-poison) uses socket2+libc for ICMP, IP_HDRINCL, custom protocols, AF_PACKET. Wired 11 new variants into rbida_Attack enum with selectors and dispatch. Added tadmor-sortie-attacks section to theurge crucible with 11 bark-only cases. Theurge builds clean, 85/85 fast suite passing. Verify-parity and retire phases deferred — require charged crucible to run both python sorties and theurge cases side-by-side.

### 2026-04-02 07:35 - ₢A1AAG - n

Port all 11 python sorties (rbtis_*.py) to Rust ifrit attack variants with socket2+libc dependencies, wire into attack enum dispatch, and add tadmor-sortie-attacks crucible section with 11 theurge cases

### 2026-04-02 07:15 - ₢A1AAR - W

Designed and implemented rbw-cK (kludge-install) and rbw-cO (ordain-install) bottle operations that auto-drive consecrations into nameplates via BCG-compliant zrbob_drive_consecration. Added charge gate rejecting uncommitted nameplate changes. Differential furnish kindles foundry core for kludge, full director for ordain. Retired rbfd_ifrit_kludge, kdev constant, rbi-iK tabtarget. 85/85 fast suite passing.

### 2026-04-02 06:58 - ₢A1AAR - n

Add rbw-cK (kludge-install) and rbw-cO (ordain-install) bottle operations with drive_consecration, charge gate for uncommitted nameplate changes, retire rbfd_ifrit_kludge and rbi-iK tabtarget, relax kludge sentinel to foundry core only

### 2026-04-01 19:16 - Heat - r

moved A1AAR to first

### 2026-04-01 19:15 - ₢A1AAF - W

Ported all 22 tadmor-security bash cases to theurge+ifrit (23 Rust cases total including correlated observation), verified 23/23 passing against live crucible, fixed sentry IP discovery (pentacle fiat instead of sentry writ) and BUK log header contamination in stdout parsing, renamed rbi-iB to rbi-iK (IfritKludge), retired bash fixture and case file. Pre-existing bash icmp_block_beyond regex bug (^ anchors to string start not line start) fixed in theurge implementation.

### 2026-04-01 19:15 - ₢A1AAF - n

Retire bash tadmor-security fixture — all 22 cases now run via theurge crucible (tt/rbtd-r.Run.tadmor.sh)

### 2026-04-01 19:14 - Heat - S

kludge-consecration-override-design

### 2026-04-01 19:07 - ₢A1AAF - n

Fix 3 crucible failures (sentry IP discovery via pentacle fiat instead of sentry writ, IP validation to filter BUK log headers), rename rbi-iB to rbi-iK (IfritKludge), switch tadmor to kdev consecration — 23/23 theurge cases passing

### 2026-04-01 18:32 - ₢A1AAF - n

Port remaining 17 tadmor-security bash cases to theurge+ifrit: 16 new ifrit attack variants (DNS protocol, DNS bypass, TCP 443 parameterized, ICMP traceroute), fiat helper for pentacle cases, correlated writ-resolve-then-bark pattern for tcp443, sentry IP discovery via resolv.conf

### 2026-04-01 15:42 - ₢A1AAE - W

Wired theurge+ifrit end-to-end against tadmor: 5 crucible cases (3 ifrit attacks via bark, 2 sentry observation via writ with iptables delta capture), charge/quench lifecycle in main.rs, rbtd-r workbench route with tabtarget, fixed invoke_ifrit to include rbid binary name in bark args

### 2026-04-01 15:42 - ₢A1AAE - n

Wire theurge+ifrit end-to-end: 5 tadmor crucible cases (3 ifrit attacks via bark, 2 sentry observation via writ), charge/quench lifecycle in main, rbtd-r workbench route and tabtarget, fix invoke_ifrit to include rbid binary name

### 2026-04-01 15:26 - ₢A1AAP - W

Qualified ifrit vessel standalone: fixed vessel sigil resolution (ifrit→rbev-bottle-ifrit) and base image tag (rust:bookworm-slim→rust:slim-bookworm), verified build/tools/all 3 attacks/error handling/wire protocol/layer caching

### 2026-04-01 15:26 - ₢A1AAP - n

Fix ifrit vessel resolution name to full directory name (rbev-bottle-ifrit) and correct Rust base image tag to rust:slim-bookworm

### 2026-04-01 15:13 - ₢A1AAD - W

Built ifrit binary with rbida_Attack enum (exhaustive match), 3 attacks ported from bash tadmor-security (dns-allowed-anthropic, dns-blocked-google, apt-get-blocked), IFRIT_VERDICT wire protocol. Rewrote vessel Dockerfile for single-stage rust:bookworm-slim with layer-cached dep build. Created rbi-iB kludge tabtarget with constant kdev consecration via rbfd_ifrit_kludge in foundry director, enrolled in zipper.

### 2026-04-01 15:09 - ₢A1AAD - n

Build ifrit binary with attack selector dispatch (3 attacks: dns-allowed-anthropic, dns-blocked-google, apt-get-blocked), rewrite vessel Dockerfile for single-stage Rust build with layer-cached deps, create rbi-iB kludge tabtarget with constant consecration

### 2026-04-01 14:48 - ₢A1AAC - W

Built theurge container interaction layer: tabtarget discovery (glob tt/{colophon}.*.{nameplate}.sh with exactly-one enforcement), tabtarget execution with per-invocation BURV isolation (invoke-NNNNN/output+temp dirs, env var passthrough), and ifrit verdict parsing (IFRIT_VERDICT wire protocol). 20 new tests covering discovery, parsing, invocation with BURV isolation, and error cases. All 39 tests pass.

### 2026-04-01 14:48 - ₢A1AAC - n

Add RBTDRI invocation layer with tabtarget discovery, BURV-isolated execution, ifrit verdict parsing, and 19 unit tests

### 2026-04-01 14:39 - ₢A1AAO - W

Qualified case engine with 8 new tests covering all docket scenarios (mixed verdicts, fail-fast, run-all, edge cases, temp dir isolation, trace file content). No engine bugs found. Created RBTW workbench and tabtargets (rbtd-b, rbtd-t) for theurge build/test, fully orthogonal from VOW pipeline. Recorded RBTW design decision in paddock including rbrt rejection rationale and ifrit macOS exclusion.

### 2026-04-01 14:38 - Heat - d

paddock curried: add RBTW workbench design decision, resolve tabtarget reorganization

### 2026-04-01 14:37 - ₢A1AAO - n

Create RBTW workbench and tabtargets for theurge build/test, fully orthogonal from VOW pipeline. Document in CLAUDE.md with acronym mapping.

### 2026-04-01 14:24 - ₢A1AAO - n

Add 8 engine qualification tests: zero sections, empty section, all-skip, single case pass/fail, run-all-despite-failures, temp dir isolation, skip trace content, case output file survival

### 2026-04-01 14:17 - ₢A1AAB - W

Built theurge case execution framework as proper RCG multi-module library: rbtdre_engine (Verdict/Case/Section types, sequential dispatch with per-case temp dir isolation, colored terminal output, trace file writing, fail-fast support, summary reporting), rbtdrm_manifest (colophon verification), rbtdrd_dummy (placeholder cases exercising all verdict paths). All declarations carry globally-unique classifier-qualified prefixes. 9 tests across two test modules.

### 2026-04-01 14:15 - ₢A1AAB - n

Build theurge case execution framework with proper RCG module structure — engine (verdict/case/section/dispatch/trace/summary), manifest verification, and dummy cases, all with globally-unique classifier-qualified prefixes

### 2026-04-01 13:58 - Heat - T

audit-coverage-parity

### 2026-04-01 13:53 - Heat - S

audit-coverage-parity

### 2026-04-01 13:53 - Heat - S

qualify-ifrit-vessel-standalone

### 2026-04-01 13:53 - Heat - S

qualify-case-engine

### 2026-04-01 13:47 - ₢A1AAN - W

Minted three non-interactive container exec colophons — writ (rbw-cw, sentry), fiat (rbw-cf, pentacle), bark (rbw-cb, bottle). Implemented backing functions in rbob_bottle.sh, enrolled in zipper with imprint channel, created 9 per-nameplate tabtargets, added to theurge compiled-in colophon set with proper RBTD_COLOPHON_* consts per String Boundary Discipline. Fast qualification and 312 Rust tests green.

### 2026-04-01 13:45 - ₢A1AAN - n

Minted writ/fiat/bark non-interactive container exec colophons (rbw-cw, rbw-cf, rbw-cb) with backing functions, zipper enrollment, per-nameplate tabtargets, and theurge compiled-in colophon consts per String Boundary Discipline

### 2026-04-01 13:37 - ₢A1AAK - W

Implemented colophon manifest protocol — bash side exposes ZRBZ_COLOPHON_MANIFEST from zipper roll in zrbz_kindle(), Rust side verifies compiled-in colophons (rbw-cC, rbw-cQ) against manifest at launch with specific fatal messages. Fixed interactive tabtarget line ordering (4 files) to satisfy qualification checker. Fast suite fully green (85 cases).

### 2026-04-01 13:31 - ₢A1AAK - n

Implement colophon manifest protocol — bash side exposes ZRBZ_COLOPHON_MANIFEST from zipper roll, Rust side verifies compiled-in colophons (rbw-cC, rbw-cQ) against manifest at launch. Fix interactive tabtarget line ordering to satisfy qualification checker.

### 2026-04-01 13:22 - ₢A1AAA - W

Minted rbtd (theurge) and rbid (ifrit) Rust crate skeletons — both compile, CLAUDE.md prefix mappings added, terminal exclusivity verified against existing rbtid Python ifrit.

### 2026-04-01 13:22 - ₢A1AAA - n

Add Cargo.lock files for rbid and rbtd crate skeletons

### 2026-04-01 13:22 - ₢A1AAA - n

Minted rbtd (theurge) and rbid (ifrit) Rust crate skeletons with Cargo.toml and main.rs, verified both compile, added RBTD and RBID prefix mappings to CLAUDE.md

### 2026-04-01 13:19 - ₢A1AAL - W

Confirmed rbi_Image.sh fully unused — no sourcing, no callers, no tabtargets. Deleted it and removed RBI entry from CLAUDE.md prefix mappings, freeing the rbi prefix for ifrit directory (rbid).

### 2026-04-01 13:18 - ₢A1AAL - n

Confirmed rbi_Image.sh fully unused (no sourcing, no callers, no tabtargets), deleted it to reclaim rbi prefix for ifrit directory rbid, removed RBI entry from CLAUDE.md prefix mappings

### 2026-04-01 13:15 - Heat - d

paddock curried: single Dockerfile model, removed copy-in concept

### 2026-04-01 12:58 - Heat - T

mint-rbtd-rbid-prefix-and-workspace

### 2026-04-01 12:58 - Heat - S

mint-writ-fiat-bark-colophons

### 2026-04-01 12:58 - Heat - d

paddock curried: fixed rbtr collision, settled ifrit build model

### 2026-04-01 12:40 - Heat - d

paddock curried: reslate pass incorporating all review feedback

### 2026-04-01 12:39 - Heat - S

deduplicate-ported-test-catalog

### 2026-04-01 12:37 - Heat - S

reclaim-rbi-prefix

### 2026-04-01 12:14 - Heat - S

colophon-manifest-protocol

### 2026-04-01 12:03 - Heat - f

racing

### 2026-04-01 12:03 - Heat - S

retire-bash-test-and-python-sortie-infrastructure

### 2026-04-01 12:03 - Heat - S

port-fast-tier-regime-enrollment-validation

### 2026-04-01 12:03 - Heat - S

port-srjcl-pluml-service-tests

### 2026-04-01 12:03 - Heat - S

port-python-sorties-to-ifrit

### 2026-04-01 12:02 - Heat - S

port-remaining-bash-tadmor-security

### 2026-04-01 12:02 - Heat - S

first-end-to-end-tadmor-cases

### 2026-04-01 12:02 - Heat - S

ifrit-binary-with-attack-selector

### 2026-04-01 12:02 - Heat - S

theurge-container-interaction-layer

### 2026-04-01 12:02 - Heat - S

theurge-reporting-and-case-framework

### 2026-04-01 12:01 - Heat - S

mint-rbtr-prefix-and-workspace

### 2026-04-01 12:01 - Heat - d

paddock curried: initial design from conversation

### 2026-04-01 12:01 - Heat - N

rbk-theurge-rust-test-infrastructure

