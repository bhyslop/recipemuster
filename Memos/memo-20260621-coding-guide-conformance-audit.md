# Memo — Coding-Guide Conformance Audit (2026-06-21)

*Authored by Claude Opus 4.8 with Brad, 2026-06-21. Provenance, not authority:
the homes for every claim here are the guides and acronym maps it points at —
this memo records an audit and a decision, it does not legislate. If a fact here
must stay true, it belongs in a spec, not in this memo.*

## What this is, and why it exists

Brad opened a session to "set up a workflow to repair all issues" where BCG / RCG
/ CBG conformance had drifted across the bash and Rust corpus. We ran a 16-agent
sampling audit (≈1.8M tokens, 2026-06-21) to ground a repair plan in real data
instead of guesses. The audit came back with a clear verdict — **the codebase is
already highly conformant; there is no rot** — and the planning that followed
concluded that a conformance *sweep* is the wrong shape of work for this project
right now (see **The decision** below).

So this memo banks two durable things and consciously declines the rest:

1. **The domain → guide map** — every code domain that exists and which guide(s)
   govern it. This is the part future readers should keep.
2. **The no-touch doctrine** — the citation/deviation mechanisms that make
   "obviously wrong" code actually correct, so a future reviewer (human or agent)
   does not "fix" a load-bearing deviation.

Everything else (the per-domain findings, the one eventual fix, the audit's own
gaps) is recorded for provenance.

## The domain → guide map (keep this)

Every persistent body of code in this repo answers to a guide. The guides are the
authority; this table is a navigational index into the acronym maps in
`CLAUDE.md`, `Tools/rbk/claude-rbk-acronyms.md`, `Tools/buk/claude-buk-core.md`,
`Tools/cmk/claude-cmk-core.md`, and `Tools/vok/claude-vok-context.md`.

| Domain | What it is | Where it lives | Governing guide(s) |
|--------|-----------|----------------|--------------------|
| **Host bash** | kit modules, CLIs, workbenches, zippers, dispatch | `Tools/**/*.sh` | **BCG** (+ ACG cross-cut) |
| **Tabtargets** | colophon trampolines (launchers) | `tt/*.sh` | **BCG** § Tabtarget Path Indirection |
| **Launchers** | per-workbench trampolines | `rbmm_moorings/rbml_launchers/launcher.*_workbench.sh` | **BCG** |
| **Rust crates** | jjk, rbtd (theurge), apck, vvc, vok, **rbid (ifrit)** | `Tools/**/src/`, `Tools/jjk/vov_veiled/`, **`rbmm_moorings/rbmv_vessels/common-ifrit*-context/`** | **RCG** (+ ACG cross-cut) |
| **Cloud-build steps** | polyglot bash/sh + python step bodies spliced into Cloud Build | `Tools/rbk/rbgj*/*` | **CBG** (+ ACG cross-cut) |
| **Windows transport** | ssh-to-Windows transport bash | `buw` jpS/jws launchers, `bujp` preflight, transport-touching code | **WSG** |
| **In-vessel jailer sh** | sentry + pentacle — zero-dependency POSIX `sh` run inside the security envelope | `rbmm_moorings/rbmv_vessels/common-sentry-context/{rbjs_sentry,rbjp_pentacle}.sh` | **JDG** (+ ACG cross-cut) |
| **Diagrams** | PlantUML source + rendered light/dark SVG pairs | `diagrams/rbdg*.puml` / `*.svg` | **PCG** |
| **Handbooks / procedures** | durable operator-facing procedures | handbook content (`rbh*`, onboarding) | **HCG** |
| **The guides themselves** | how to author a guide | `Tools/**/vov_veiled/*Guide*` | **GMG** (meta) |
| **Allocation (cross-cutting)** | *where a fact is homed* — spec ↔ source | **all code** | **ACG** — see caveat |

**Two corrections this audit forced on its own scope:**

- **The file universe is `Tools/` *and* `rbmm_moorings/rbmv_vessels/`.** The first
  enumeration stopped at `Tools/` and missed the `rbid` ifrit attack binary (a
  real Rust crate: `common-ifrit-context/src/{lib,main,rbida_attacks,rbida_sorties}.rs`
  plus the `common-ifrit-forge-context` crate) and the jailer `sh`. Any future
  conformance work must include the vessel-context tree.
- **ACG is a different kind of guide.** BCG/RCG/CBG/JDG/WSG are *code-shape*
  guides: a fix is bounded, lives in source, and has a green-build oracle. **ACG
  is a knowledge-allocation guide** — its core move is deciding whether a fact's
  home is a spec or the source, and when both carry it, resolving that means a
  rivet pointer, a minted quoin, or a "load-bearing duplication" ruling. That is
  bidirectional (touches `.adoc` specs), pulls in the whole MCM/lapidary minting
  discipline, and has **no build oracle** — only human judgment verifies it. ACG
  was therefore deliberately **excluded from the audit and from any sweep**. If it
  is ever worked, it is its own slow, file-by-file, human-paced effort — never a
  fan-out. (The sentry-pair ACG pilot, `Memos/retired/memo-20260612-acg-pilot-sentry-findings.md`,
  found its detected comments already homed — hinting low yield, high cost.)

## The no-touch doctrine (read before "fixing" anything)

The single biggest risk to any future conformance effort is a reviewer "cleaning
up" code that reads as wrong *because the correct form does*. Before changing
anything that looks off, check these membranes:

- **Cited deviations (rivets).** An opaque token — `RBr_<tail>`, `JJr_a7c`,
  `CBb_`/`CBi_`/`CBp_`/`CBh_`, `JDo_`/`JDp_`, `PCr_` — in a comment (or, in the
  jailer dialect, riding an execution-time phase announcement as a second token)
  is a *cited constraint*. Its rationale lives once in a closed `.adoc` spec.
  `grep` the token and read the spec; **never** restate the rationale beside the
  marker, **never** simplify the code it guards. Live instances found: the jjk
  reprieve `JJr_a7c` (frozen `jjrt_v3_types.rs`, the serde aliases, the V3→V4
  write-forward), the theurge `RBr_3d3` attack-test sites, and seven `RBr_*`
  tokens in `rbjs_sentry.sh` (→ RBSSS/RBSPT/RBSDS).
- **Build-generated files.** `Tools/rbk/rbtd/src/rbtdgc_consts.rs` and
  `Tools/rbk/claude-rbk-tabtarget-context.md` are regenerated from `rbz_zipper.sh`
  by the theurge build. Edit the zipper and rebuild (`tt/rbw-tb.Build.sh`); never
  hand-edit them. If they show modified in `git status` after a zipper edit +
  build, that is expected re-derivation.
- **Blessed v1 deviations.** JDG explicitly blesses the sentry's non-monotonic
  phase labels (`RBJp2 → RBJp2c → RBJp2b`, cited against JDo_101) and the
  pentacle's `RBJP: FATAL -` shape (cited against JDo_103). Do not repair on sight.
- **Centralized shellcheck.** All BCG-structural suppressions live in
  `Tools/buk/busc_shellcheckrc` with rationale; inline `# shellcheck disable=`/
  `source=` directives are prohibited. The *absence* of inline directives is the
  conformant state, not a gap.
- **Precision exit-code band.** Band codes are minted *only* in the
  `bubc_constants.sh` tinder block and carry through `|| buc_die` chains unchanged.
  Never re-literal a band code elsewhere; never `readonly` a tinder constant.
- **CBG step bodies are not BCG host modules.** The `rbgj*` files legitimately use
  pipelines-in-`$()`, evicted coreutils (busybox `sha256sum`/`cut`/`awk`), raw
  `echo`, and heredocs — all relaxed under CBG for minimal cloud containers, often
  cited (`CBb_101`). Walking the BCG checklist against them manufactures dozens of
  false positives. Same trap for the jailer dialect: `cut`/`awk`/`grep`/`dig`/
  `iptables`, bare `$var`, missing `local`, and heredocs are correct in-vessel
  POSIX `sh`, governed by JDG, not BCG.
- **Palisade / load-bearing code without markers.** Some fiddly code is correct
  because the real grammar is fiddly: the Anthropic model-ID walk in
  `vvcp_probe.rs` (bracket/dot handling for `claude-*` IDs), the vvc git
  rename/copy cost model, the apck no-JavaScript HTML/JS escapers, and vok's MCP
  stdout/stderr routing. No rivet marks them; treat them as load-bearing by
  inspection.
- **Bootstrap exemptions.** `vvi_install.sh` (ships standalone, cannot depend on
  BUK), the `bud_dispatch.sh` pre-`buc` bootstrap layer, and `buw-SI.StationInit.sh`
  legitimately use raw `echo` and do not route through `buc_*`.

## Audit method and scale

- 16 agents over two phases (2026-06-21). Phase 1: one agent per guide extracted a
  checkable rule catalog + the deviation mechanism (rule-family counts below).
  Phase 2: 11 agents sample-audited each file-class against the relevant catalog,
  reporting violation categories, prevalence, density, and landmines.
- **Sampling, not exhaustive.** Each recon agent read a handful of representative
  files fully and ran class-wide prevalence greps. Density verdicts are sound;
  specific counts are estimates.
- Rule-family sizes extracted: **BCG 61, RCG 31, WSG 23, CBG 18, JDG 12.** ACG was
  not extracted (excluded by design).

## Conformance verdict per domain

Overall: **highly conformant.** The recurring agent verdict was "strongly
conformant; do not manufacture findings." Findings clustered into a few themes,
almost all mechanical or low-severity. Organized by value × risk:

### Tier 1 — behavior-relevant (the only items with real teeth)
- **vok output discipline** — see the dedicated section below. The one item worth
  eventual standalone action.
- **RCG-01 crate-root attribute trio** — cheap one-liners with real teeth:
  `vok/vorm_main.rs` has no trio at all; `vvc/lib.rs` and `vok/vof/lib.rs` are
  missing `#![allow(private_interfaces)]`; `rbtd/main.rs` and a jjk file have an
  allow-before-deny ordering slip. Each needs a build (may surface latent
  `deny(warnings)` failures).
- **Jailer pentacle** — `rbjp_pentacle.sh` lacks the `RBJ_VERBOSE` trace toggle
  (JDG-04) the sentry has, and the JDp_101 transport probe (mitigated: its sole
  parameter is a single-token IP). Tiny; verified under `crucible`/`tadmor`.

### Tier 2 — mechanical hygiene on an already-clean codebase (low value)
- **RCG-07/11 test extraction** — a half-finished migration from inline
  `#[cfg(test)] mod tests` + `test_*` names to external `{cipher}t{classifier}_*.rs`
  files with prefixed `#[test]` fns. State: vvc 5/7 inline, vok all-inline (zero
  `vot` files), apck 3/6 inline, jjk 6 files inline. Each move cascades RCG-09/31
  (true-private → `pub(crate)`). Touches dozens of files and every test-fn name.
- **RCG-03 use-list reflow** — one-item-per-line, alphabetized, trailing comma —
  pervasive across apck/vvc/vok/rbtd/jjk; crates are internally inconsistent.
- **RCG-24 output sentinels** + **jjk RCG-04 alias sweep** (65 prefix-dropping
  `use … as` sites across 28 files, which defeat `grep`-provenance). Grep hygiene.
- **BCG exec-bit sweep** — ~11 non-`_cli` kit files carry a stray `+x`
  (`jjz_zipper.sh`, `vvw_workbench.sh`, `voa_arcanum.sh`, `vow_workbench.sh`,
  `apcw_workbench.sh`, etc.). Carve-outs: `vvi_install.sh` and every `*_cli.sh`
  keep `+x`. Trivial `find + chmod`.
- **rbk/buk surgical nits** — a few raw-echo display modules (`rbob_cli`,
  `rbflw_wrest`), one `&&{`-control site (`rbge_rest.sh:99/149`),
  `bud_dispatch.sh:254` (openssl `2>/dev/null` + declaration-capture), and unbraced
  expansions in `buc_command.sh`'s older help-render paths. Confined.

### Tier 3 — judgment calls (need an operator ruling, not an agent)
- **vvc public-facade aliases** (`commit`/`guard`/`marker` bare re-exports in
  `lib.rs`) — contract, or fixable? Check jjk/vok consumers first.
- **RCG-08 prefix exceptions** — jjk's deliberate `jjrx_` command-args family and
  impl-accessor methods (rbtd/vok): strict-prefix vs. blessed-convention
  dispensation.
- **apck category→string maps** (three of them) — some divergence is load-bearing
  (placeholder vs. assay-tag vs. UI-label surfaces).
- **Legacy pre-BCG standalones** — `crgv.validate.sh`, `crgr.render.sh`,
  `lmci/strip.sh`, `lmci/bundle.sh`: `eval`-based `${!var}` deref (×18), zero-arg
  test-on-cmdsub, bash4 `mapfile` in a file claiming 3.2 compat. Real, but
  behavior-sensitive — a scoped migration decision, never a drive-by.

### Already conformant / nothing to do
- **CBG cloud-build steps** — strongly conformant, deviations cited; only cosmetic
  `rbgjb06-push-diags.sh` stray `+x` (inert — these files are read-and-spliced,
  not executed).
- **Tabtargets** — 207/208 byte-identical to the canonical trampoline;
  `buw-SI.StationInit.sh` is a self-documented bootstrap standalone.

## The one item worth eventual action: vok output discipline

**Honest framing — latent, not live.** Verified against the tree on 2026-06-21:
`vok/vorm_main.rs` is the `vvx` binary, and `vvx mcp` runs an MCP stdio server
(`run_mcp` → `jjk::jjrm_mcp::jjrm_serve_stdio`), where **stdout is the JSON-RPC
protocol channel**. The MCP path today is correctly stderr-only (every
diagnostic in `run_mcp` is `eprintln!`), and the actual server impl (jjk
`jjrm_mcp.rs`) routes protocol via `vvco_out!` and diagnostics via stderr. **There
is no active corruption bug.**

The gap is **structural**: the vok crate has ~57 naked print sites and **no output
module**, so the stdout=protocol / stderr=diagnostics separation rests entirely on
every author remembering it — in a 593-line file that mixes an MCP server with
CLI subcommands that legitimately `println!` their JSON results (`:443`, `:471`,
`:520`, `:563`). Nothing structural stops a future edit from putting a `println!`
on the MCP path and silently corrupting the transport in a way that is miserable
to debug.

**The fix** (when vok is quiescent): introduce an output module, mirroring vvc's
`vvco_Output` — which vok **already imports and uses** (`vorm_main.rs:231,245`) —
so the routing discipline is enforced by construction instead of remembered. This
also satisfies RCG-23/24. It is worth doing *eventually* to bank the discipline;
it is **not** an emergency and can wait.

## Audit gaps (for whoever picks this up)
- **The `rbid` ifrit Rust crate was never audited** (missed by the `Tools/`-only
  enumeration). RCG-governed; needs its own pass before any claim of completeness.
- **ACG was not audited** (excluded by design — see the caveat above).
- The `rbmm_moorings/rbml_launchers/*.sh` and vessel build/charge-hook scripts
  (`entrypoint.sh`, `rbnnh_post_charge.sh`) are a small BCG tail not sampled.

## The decision, and why

We considered a ~15-pace JJK heat (per-crate RCG passes + bash + jailer + the
judgment calls) and **declined it.** The reasoning:

A conformance sweep is the **most collision-prone shape of work** there is — it
rewrites many files across many modules for low value, and the files it touches
are exactly the ones live feature heats need. In a single-operator workflow where
parallelism across heats is the scarce resource, a sweep does not merely cost its
own serial attention; it **poisons the collision surface for weeks**, forcing every
future pace over those files to rebase across whitespace-and-test-relocation diffs.
Paying uncollided parallelism to make an already-clean codebase marginally tidier
is a bad trade.

**The chosen posture — lowest-conflict conformance:**

1. **Fold the rules into work already happening.** When a file is opened for real
   work, bring it into conformance *in that same pace* — zero added edit surface,
   so zero added collision. Conformance converges on the back of work that was
   happening anyway, and is never "complete" — which is fine, because the codebase
   is already clean.
2. **Do the few isolated, high-value fixes as one-offs, on quiescence.** The vok
   output module (when vok is quiet), and optionally the jailer pentacle (one
   file). The ifrit audit is read-only — anytime.
3. **Sweep a crate only when it goes quiescent** — operator-triggered, never
   committed upfront, so it collides with nothing by construction.
4. **ACG is its own future, deliberate, human-paced effort** — or never.

The honest bottom line: this is the rare case where the right answer is to do
**almost none of it now**, bank the audit (this memo), and consciously decline the
sweep rather than let "all guides should be followed" tax the one resource the
project is shortest on.
