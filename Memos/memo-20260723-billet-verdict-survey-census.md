# Census: unclassified-panic sites in the plain-git farrier driver

Date: 2026-07-23. Provenance for the billet-verdict survey pace; the durable half
(the scope criterion, the two new rejection kinds) lands in JJSVF-farrier.adoc.

## Recipe

`grep -n zjjrfg_unexpected Tools/jjk/vov_veiled/src/jjrfg_plaingit.rs` — 44 call
sites across 21 ops. Each judged against the pace's cinched scope criterion:

> A git verdict is surveyed when it is **field-observed OR structurally relied
> upon by a cinched invariant**, AND **probe-detectable** (exit code, porcelain
> output, registry read — never stderr prose), AND **carries a remedy the refusal
> can name.**

Everything failing the test keeps the loud unclassified panic. The panic is the
discipline, not a defect.

## Verdict table

Sites are grouped by op; a group with one verdict is judged as a group.

| Op (sites) | What a failure means | Verdict |
|---|---|---|
| `identify` / seat probe (207) | a tree that answered `--show-toplevel` answers neither `--git-dir` nor `--git-common-dir` | **panic** — not observed; a broken git, not a domain verdict |
| `primary_root` (227) | a billet op handed a primary root | **panic** — in-house caller-contract violation, not a foreign verdict |
| `line_of_work` (250, 256), `counterfoil` (330) | `rev-parse HEAD` fails — reachable only on an unborn HEAD | **panic** — not observed; see Finding 1 |
| `comb` (299) | `status --porcelain` fails | **panic** — not observed; but see Finding 2 |
| `sync_state` (319, 323) | `rev-list --count` output unparseable | **panic** — would mean git changed its output format |
| `lodge` (354, 361) | `add` or `commit` refuses | **panic** — the notch toothing validates its own file list and never reaches here; no observed path |
| `advance` (382, 420, 422) | no upstream; `merge-base` returns neither 0 nor 1 | **panic** — guarded upstream by `sync_state`'s Untracked answer (`JJr_b52`) |
| `advance` (426) | `merge --ff-only` refuses a proven fast-forward | **panic** — see Finding 3 (nearest second-tier candidate) |
| `consign` (441), `proffer` (519), `stake` (550), `pluck` (564), `sight` (570, 582, 586), `bequeath` (723) | a push or remote read fails for a reason that is not a ref rejection: unreachable remote, refused auth, vanished remote | **panic** — fails *probe-detectable*: separating these from each other needs stderr prose, which the criterion bars. `glean` alone can afford an Unreachable outcome because it is opportunistic and mutates nothing |
| `proffer` (461, 467, 475, 482, 486, 491, 528, 532), `bequeath` (684, 693, 699, 707) | a step of an in-house object-database composition fails | **panic** — every input was composed by this driver moments earlier |
| `billet_create` (612) | `worktree add` refuses: branch-name collision, destination path occupied, or missing counterpart | **panic** — the collision arm is already ruled a caller-contract violation by JJSVF `billet_create`; see Finding 4 for the occupied-path arm |
| **`billet_seat` (627)** | `worktree add` refuses to seat an existing branch | **SURVEYED** — two signatures, below |
| `billet_detach` (645) | `checkout --detach` refuses | **panic** — dirt and untracked files are both caught by the `comb` gate above it |
| `line_exists` (656), `outstripped` (676) | `show-ref` / `merge-base` returns a status outside its documented pair | **panic** — would mean git changed its exit-code contract |
| `billet_remove` (742) | `worktree remove` refuses | **panic** — but see Finding 2 |
| `enfold` (770) | merge conflict | **panic** — ruled by standing decision (JJSVF `enfold`: "resolution belonging to the attended session"); no remedy the refusal can name that is not the attended session's own work |

## The surveyed pair — both at `billet_seat`

Both arise when `worktree add <path> <branch>` refuses because the constellation
already records a seat for that branch. **Git's message and exit code are byte
identical between them** — exit 128, `fatal: '<branch>' is already used by
worktree at '<path>'` — so the whole classification rests on a registry probe,
exactly as the docket rules. Empirically confirmed on git 2.50.1.

The probe is `git worktree list --porcelain`, matched on `branch refs/heads/<branch>`:

- The entry carries a **`prunable`** line → the recorded seat's root is gone.
  This is the incident's residue: an out-of-band `rm -rf` of a billet leaves the
  registration standing, and the next saddle dies here.
  Remedy the refusal names: prune the registry. **Never auto-prune** —
  advice-not-automatic.
- The entry carries **no `prunable` line** → the branch is genuinely seated in a
  live billet at that path. This is the exclusivity mechanism the catchword
  pace's *at-most-one-live-billet-per-coronet* invariant leans on: git enforces
  the pace half for free, and this refusal is where that enforcement becomes
  legible. Remedy: that path is the live billet — work there, or reap it first.

Both pass all three conjuncts. The first is field-observed (the incident); the
second is structurally relied upon by a cinched invariant.

## Findings recorded, not surveyed

1. **Unborn HEAD contradicts `identify`'s totality.** JJSVF calls `identify`
   *total* for a claimed tree, but a repo with no commits panics at
   `line_of_work`. Not field-observed (JJ requires a founded sire), and the
   remedy is thin. Recorded because it is a contract tension, not merely a gap.
2. **The vestige residue has a second bite site.** After an out-of-band removal,
   any billet-facet op reaching the vanished root — `billet_remove`,
   `billet_detach`, `enfold` — panics one layer earlier, at `comb` (git cannot
   `-C` into a directory that is not there), never reaching its own site. The
   muck reap of a vestige billet takes this path. Out of scope here (the ruling
   names billet *seat*); the catchword-billet build wants it.
3. **`advance` on a conflicting dirty path.** JJSVF promises unrelated scratch
   neither blocks nor is destroyed by the move; a *conflicting* dirty path does
   block, and lands in the panic. Probe-detectable (`comb` before the merge),
   remedy nameable, existing kind available (dirty-tree) — it fails only the
   first conjunct today. The nearest second-tier candidate.
4. **`billet_create` onto an occupied path.** The inverse residue: registration
   pruned, directory surviving. Not observed. The catchword build, which changes
   how billet dirnames are minted, should re-judge it.
