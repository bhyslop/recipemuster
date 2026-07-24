# Field observation: a TOCTOU race in `jjrfr_sight` promotes one signature the survey census kept as panic

Date: 2026-07-24. Provenance for a pace the operator will slate against the
plain-git farrier driver. This memo records a *field observation* — a live panic
seen during a `jjy_saddle` launch — and argues it overturns one row of the
2026-07-23 verdict census (`memo-20260723-billet-verdict-survey-census.md`). The
durable half, if the pace lands, belongs in JJSVF-farrier.adoc's survey section.

## The incident

A saddle launch panicked on first invocation, then succeeded verbatim on the
second:

```
% ../jjy_saddle CAABW
thread 'main' panicked at .../Tools/jjk/vov_veiled/src/jjrfg_plaingit.rs:143:5:
plain-git sight hit an unclassified git failure at .../jjqs_studbook:
  exit 128 | stderr: fatal: couldn't find remote ref refs/jjv/guidon | stdout:
% ../jjy_saddle CAABW      # <- clean, launched the billet
```

The panic is independent of whatever pace ₢CAABW carries. It is the farrier's
plain-git driver crashing while *reading* the blotter lock at the studbook.

## Mechanism — a two-read race on a deliberately transient ref

`jjrfr_sight` reads the guidon in **two separate git round-trips** against the
shared origin:

1. `git ls-remote origin refs/jjv/guidon` — get the ref's SHA
2. `git fetch origin refs/jjv/guidon` — pull the blob so `cat-file` can read the
   holder's mark
3. `git cat-file -p <sha>` — read it

The stderr `fatal: couldn't find remote ref refs/jjv/guidon` is **`git fetch`'s**
message, not `ls-remote`'s. `ls-remote` exits 0 with empty output when a ref is
absent, and that absence is already the graceful `Ok(None)` path. So the sequence
is unambiguous:

- **ls-remote saw the guidon** — it returned a SHA and took the `Some(sha)` arm;
- **fetch, a moment later, could not find it** — the ref was **deleted between
  the two calls**.

That deletion is a concurrent `pluck`: another officium released its blotter lock
in the exact window between sight's two reads. `refs/jjv/guidon` is the one
transient lock ref — `stake` creates it, `pluck` deletes it — and `sight` is the
only reader that reads it in two non-atomic round-trips. The retry hit a stable
state (lock either steadily held, or cleanly absent → `Ok(None)`), which is why
the second launch sailed through. Classic TOCTOU; no data corruption, because the
lock *protocol* (lease-guarded CAS on `stake`/`pluck`) is untouched — only the
reader crashes on a legitimate concurrent state.

## Why this overturns the census's `sight` row

The 2026-07-23 census judged sight's remote-read failures as **panic**, grouping
them into one row:

> `consign`, `proffer`, `stake`, `pluck`, **`sight`**, `bequeath` | a push or
> remote read fails for a reason that is not a ref rejection: unreachable remote,
> refused auth, vanished remote | **panic** — fails *probe-detectable*:
> separating these from each other needs stderr prose, which the criterion bars.

That reasoning is sound for the failure modes it names — telling *unreachable
remote* from *refused auth* from *vanished remote* does need stderr prose. But it
**under-resolved sight**: sight's fetch step has a fourth failure mode the row did
not enumerate — *the guidon ref specifically came down between my two reads* — and
that one is **not** stderr-bound. It is separable by a **registry re-probe**, the
very evidence class the criterion permits ("exit code, porcelain output, registry
read — never stderr prose").

Against the three conjuncts, the released-mid-sight signature now passes all three
where the census said it could not:

- **field-observed** — this incident is the observation the census lacked;
- **probe-detectable without stderr prose** — after the fetch fails, re-run
  `ls-remote origin refs/jjv/guidon`. Absent now → the guidon came down. This is a
  registry read, not a string match. No need to distinguish *why* the remote
  might be unhappy; the only question is "does the ref still fly," and ls-remote
  answers exactly that;
- **remedy the refusal can name** — none is needed: the honest return is
  `Ok(None)`. At the moment sight returns, no guidon flies; that is literally
  true, not a paper-over.

The census's "vanished remote" (the whole origin is gone → needs stderr) and this
"vanished *ref*" (the guidon alone came down → registry-detectable) are different
verdicts that the one row conflated.

## The trap the criterion forbids

The tempting one-liner is to string-match `"couldn't find remote ref"` in fetch's
stderr and return `Ok(None)`. The criterion's second conjunct bars exactly this —
stderr wording "is the neighbor's to change without notice." The classifier must
be the **ls-remote re-probe**, not the message text. If the re-probe still shows
the ref present (or errors itself, e.g. genuinely unreachable), keep the loud
panic — that residue is still unsurveyed and should stay so.

## Interaction with sight's callers — `Ok(None)` is benign everywhere

Collapsing the released-mid-sight case to `Ok(None)` is safe across sight's
callers:

- **derelict-lock reporting / break** (the muck/cashier path): a lock that
  released between the two reads is simply not held now — `Ok(None)` is correct.
- **stake-then-self-confirm** (`jjrdm_muck.rs`, "sight after stake did not
  confirm our own guidon"): if `Ok(None)` reaches it, it converts to that path's
  own *classified, clear* panic instead of a raw plumbing one — strictly better.

## Disposition

Not in the census's deliberately-declined set (that list holds only
unreachable-remote and the enfold conflict). This is a first field observation of
a not-yet-encountered signature, and by the survey's own logic an unclassified
panic is correct only until a signature is surveyed. Recommend a pace that amends
the `sight` verdict in JJSVF and teaches `jjrfr_sight` the ls-remote re-probe
classifier. Implementation shape is the pace's to settle at mount; this memo fixes
only the finding and its grounding in the criterion.
