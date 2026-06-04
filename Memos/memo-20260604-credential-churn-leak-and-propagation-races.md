# Credential lifecycle: the Binding-revoke hole (keyfile tier)

*2026-06-04. Surfaced in heat ₣BV while bringing the `blockade` (moriah / airgap)
crucible up on macOS Docker Desktop (branch `bv-macos-crucible-20260603-BVAAE`);
repair tracked under `rbk-08-credential-repairs`. Depot under study: the
canonical standing depot `cancbhm-d-canest3bhm100001`, org `247899326218`.*

## Headline

The keyfile credential tier (RBSHR's name for the current model: each role holds
an RBRA key file, possession = identity) **fuses three independent object
lifecycles into one operation**, and never wires the revoke half of one of them.
The result: IAM bindings accumulate without bound, and re-credentialing trips two
separate Google eventual-consistency edges. All of it follows from one structural
hole, stated precisely below. None of it is macOS-specific.

## Objects

- **SA** — a GCP service account, e.g. `director-canest-dir@…`. Identity is
  email **+ uid**; recreating the same name yields a *new* uid.
- **Key** — credential material *on* an SA. Serialized to disk as the **RBRA**
  file the operator holds. (Key = GCP-side; RBRA = its delivery.)
- **Binding** — one `(role → member)` entry; the member *is* an SA; it lives on
  a **Resource**.
- **Resource** — a thing carrying an IAM policy: the **project**, the **GAR
  repo**, the **Mason SA**.

## Three independent lifecycles

1. **SA**: create → exists → delete. *On delete, GCP cannot drop bindings whose
   member vanished — it retags them `deleted:…?uid=` and keeps them.*
2. **Key**: add → valid → delete. *Deleting a Key is a blunt kill of one
   credential; the SA is untouched.*
3. **Binding**: grant (add member) → active → **revoke (remove member)**. Keyed
   by member = SA.

Re-credentialing an operator *requires only the Key lifecycle*: add a new Key,
kill the old one.

## The hole

The code fuses "create the identity" with "issue a credential." One invest does
**SA-create + Binding-grant + Key-create** together; its only inverse (divest)
does **SA-delete** alone. From that fusion, two defects:

- **H1 — the Binding lifecycle is half-wired.** `grant` runs at invest;
  **`revoke` is never called anywhere.** Divest deletes the SA and stops
  (`zrbgg_divest_role` never opens an IAM policy). So Bindings are `+1` per
  invest and `0` per divest — monotonic, unbounded.
- **H2 — re-key invokes the wrong lifecycle.** "Re-key" runs the **SA**
  lifecycle (delete + recreate — chosen as a blunt guarantee the old Key is
  dead) when it only needed the **Key** lifecycle. So the SA *and its Bindings*
  churn every issuance, when only the Key had to.

## The three faces (all consequences of the hole)

- **Binding leak ← H1.** Bindings granted, never revoked, accumulate as
  `deleted:` ghosts on every Resource. Observed: **project 145** (69 director,
  46 retriever, 30 governor-owner), **GAR repo 23**, **Mason SA 23**. *Benign so
  far — Experiment A (below) shows the 145-ghost policy still resubmits cleanly —
  but it marches toward GCP's ~1500-member allow-policy cap independent of any
  org policy.* A comment in `rbgi_add_project_iam_role` already records the
  ghosts as a known, "accepted" cost; they have now become unbounded.
- **create→grant race ← H2.** Each cycle mints a *fresh* SA and immediately
  grants it. The org enforces Domain Restricted Sharing
  (`iam.allowedPolicyMemberDomains`, allow-listing customer `C02ystilf` =
  scaleinvariant.org); a brand-new SA is not yet recognized as in-customer, so
  the grant is rejected (HTTP 400, "incompatible with the provided policy"), and
  `rbgi`'s retry tolerance does **not** cover that error — so it dies on the
  first try. *This blocked `blockade`.* Google documents the propagation lag
  (≈2 min, up to 7+). [restricting-domains], [troubleshoot-org-policies]
- **delete→recreate race ← H2.** Divest deletes the SA and reports it gone, but
  the immediate re-create's preflight still sees it ("already exists"). *This
  blocked `dogfight`* the next morning. A state check minutes later showed the SA
  fully gone with no soft-delete reservation — the create-preflight simply read
  a not-yet-converged view.

Both races are **intermittent and host-independent** (server-side); they pass
routinely on cerebro/Cygwin and lost on macOS two days running. The macOS
crucible itself is sound — `siege` is 60/60 green there.

## The fix — separate the lifecycles

Stop fusing. Split the one invest into two operations:

- **Establish** (one-time per SA — at depot setup / operator enrollment):
  ensure the SA exists and **grant its Bindings once**. Idempotent — a no-op when
  already established.
- **Re-key** (every issuance): **rotate the Key only** — delete all existing
  Keys (the blunt kill, preserved), add one. Never touches the SA or its
  Bindings.
- **Teardown** (rare — genuine retirement): the missing inverse — **revoke
  Bindings, then delete the SA** (H1's never-wired half), run once at end-of-life
  rather than every cycle.

This closes H1 (Bindings granted once, revoked on real teardown — never churned)
and H2 (re-key is Key-only; the SA never gets recreated). Scope notes:

1. It's a refactor, not a line: one fused verb → establish / re-key (+ a real
   teardown).
2. Establish must be **idempotent** ("ensure," not "create").
3. Re-key **presumes established** (assert SA + Bindings present, else establish).
4. Blunt-kill preserved: re-key = delete *all* Keys, add one.
5. The create→grant DRS race doesn't vanish — it **relocates to establish**
   (now ≈once per identity, off the hot path). Keep the DRS-400 in `rbgi`'s
   retry tolerance there as cheap insurance.
6. Existing ghosts (145 + 23 + 23) need a **one-time sweep** — the split stops
   new ones, doesn't clean old. Safe: only `deleted:*` removed (Experiment A
   shows the policy is writable).
7. Cross-host see-saw is **orthogonal** — a shared identity re-keyed bluntly
   invalidates other holders either way; unchanged.
8. **Governor out of scope** (deliberate single-governor, datestamped,
   low-churn simplification). Fix targets director/retriever.

## Footnote: federation

The roadmap's operator-federation tier (RBSHR; `memo-20260527-operator-credential-models.md`)
grants capabilities to stable `principal://` subjects with central revocation —
no per-cycle SA at all, so it would obviate this hole entirely. It is the
**eventual successor tier, not the MVP fix**; the keyfile tier stays and is
repaired in place as above.

## Evidence appendix

- DRS + policy state: `gcloud resource-manager org-policies …`,
  `gcloud projects get-iam-policy cancbhm-d-canest3bhm100001` (payor-token, 06-04).
  DRS allow-list = `C02ystilf`.
- **Experiment A** — `set-iam-policy` of the unchanged 145-ghost project policy
  → HTTP 200. Ghosts do not block grants.
- **Experiment B′** — grant `roles/cloudbuild.builds.editor` to the now-propagated
  director SA → HTTP 200. Freshness was the only variable.
- create→grant race: `logs-buk/hist-rbw-adI-sh-20260603-103842-26773-301.txt`.
- delete→recreate race: `logs-buk/hist-rbw-adI-sh-20260604-092730-7113-822.txt`,
  suite `logs-buk/hist-rbw-ts-dogfight-20260604-092601-3729-496.txt`.
- Code: `rbgi_add_project_iam_role` + the accepted-ghost comment
  (`Tools/rbk/rbgi_IAM.sh`); `zrbgg_divest_role`, `rbgg_invest_director`,
  `rbgg_invest_retriever` (`Tools/rbk/rbgg_Governor.sh`);
  `rbtdrk_role_invest_impl` (`Tools/rbk/rbtd/src/rbtdrk_canonical.rs`).

[restricting-domains]: https://cloud.google.com/resource-manager/docs/organization-policy/restricting-domains
[troubleshoot-org-policies]: https://docs.cloud.google.com/iam/docs/troubleshoot-org-policies
