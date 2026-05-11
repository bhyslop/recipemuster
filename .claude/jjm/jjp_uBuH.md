## Context

Recipe Bottle's existing enshrinement (RBSAE `ark_enshrine`) is narrowly scoped: mirror upstream OCI base images into GAR with cloud-built provenance for vessel Dockerfile consumption. Two new capture needs have surfaced that share gestalt with that operation:

1. **WSL substrate**. Workload garrison-w on Windows imports a Linux rootfs tarball via `wsl --import`. Today caparison-w sources the seed by downloading `Ubuntu-24.04` via `wsl --install` under the admin user, then garrison-w `wsl --export`s admin's distro to produce the tar. The source is uncontrolled, mutable, and outside the SLSA trust chain.

2. **Podman VM substrate**. Podman machine images live on quay with hostile lifecycle properties — new images every ~3 hours, retention measured in days. Without project-controlled mirroring, past artifacts become unreachable within a week of release.

Both share the existing enshrinement gestalt: pull from upstream, wrap with cloud-built provenance, store in GAR with SLSA attestation. Rule of Three applies — three concrete instances (existing base + WSL + podman-vm) make the verb-generalization load-bearing.

## Locked decisions

### Enshrine as a verb across three kinds

Keep `enshrine` as the verb. Apply it to three kinds: `base` (current), `wsl`, `podman-vm`. Each kind is a parameter to the verb, not a new domain noun.

Pattern mirrors the SA-noun resolution: when verbs `invest`/`divest`/`roster` came to span Director and Retriever SA kinds, the resolution was sibling specs per noun-kind (RBSDK director_invest ↔ RBSRK retriever_invest) and tabtargets encoding the kind in the colophon (`rbw-arI` ↔ `rbw-adI`). No master parameterized spec.

Applied here:

- **Specs**: sibling per kind. RBSAE stays scoped to base. New siblings for WSL and podman-vm kinds (acronym letters open; see below).
- **Tabtargets**: rename `rbw-dE` → `rbw-dEb` (DirectorEnshrinesBaseImage). Add `rbw-dEw` (WSL) and `rbw-dEv` (podman-vm).
- **Cloud Build pipelines**: three, one per kind. Each kind's spec describes its own pipeline.

### Production forks, management polymorphs

Production differs per kind (upstream source, repackaging recipe, payload shape) and warrants per-kind specs and pipelines. Management ops on enshrinements operate at GAR-object level where kind is invisible — they stay polymorphic:

- `rbw-iJe` DirectorJettisonsEnshrinement — delete by GAR object identity; kind-agnostic
- `rbw-iae` DirectorAuditsEnshrinements — list all enshrinements; gains a kind column
- `rbw-iwe` DirectorWrestsEnshrinedImage — pull wrapper bytes; consumer decides what to do with payload

### Single-layer OCI as universal wire format

Store all enshrined artifacts in GAR as single-layer OCI images carrying the payload as the layer's content. Mild abuse of OCI semantics for non-base kinds (the "image" isn't meant to be run) but preserves the entire existing summon/vouch/SLSA chain unchanged.

Per kind:

- `base`: native OCI image; payload IS the layers
- `wsl`: rootfs tar (produced by Cloud Build `docker create` + `docker export`) wrapped as the single layer
- `podman-vm`: qcow2 bytes mirrored from quay, wrapped as the single layer

### Curia is byte pass-through

For non-base kinds, consumption requires getting the payload from GAR onto the target host:

- `wsl`: retriever pulls the enshrined artifact, extracts the layer payload (rootfs tar), `scp.exe`s to admin host's `C:\bujb-wsl\rbtww-seed.tar` over admin SSH; garrison-w `wsl --import` consumes it
- `podman-vm`: retriever pulls, extracts qcow2, hands to local podman machine

Curia performs no transformation — pulls bytes, forwards bytes. SLSA covers GAR-side production; curia is outside the trust boundary by design.

### Caparison-w / garrison-w consequences

When WSL-kind enshrinement is live, admin's WSL distribution disappears entirely:

- Caparison-w Phase 3 drops `wsl --install`, the multi-minute Ubuntu install cycle, and admin's `wsl --import` of rbtww-main
- Garrison-w Step 3 (admin `wsl --export` to seed tarball) deletes — seed already staged from GAR
- Garrison-w Phase 5 (workload-user vestige cleanup inside admin's rbtww-main) deletes — no admin distro exists
- Workload's `wsl --import` (Step 11 Session 1) is unchanged — still consumes a tar at the staged path
- Invigilate fact "rbtww-main WSL distribution registered (admin)" replaced with "C:\bujb-wsl\rbtww-seed.tar present, matching expected hallmark digest"

A pre-existing revert pace in ₣A- targeting the WSL-stage DEV CACHE shortcut becomes obsolete — the from-scratch path it would restore is itself being deleted. That pace wants to be dropped or transferred when this heat commits to development.

## Open decisions for expansion

### Spec acronym letters

Per discipline, 5-letter spec acronyms. Candidates need to follow RBSAE's tree without collision:

- Symmetric rename: RBSAE → RBSAEB (base), siblings RBSAEW (wsl), RBSAEV (podman-vm). Mirrors the `rbw-dEb`/`rbw-dEw`/`rbw-dEv` tabtarget shape.
- Asymmetric: leave RBSAE as base, mint RBSAEW and RBSAEV as siblings.

Defer until spec discipline is consulted and tinder/terminal-exclusivity rules are checked against the existing tree.

### Consumption verb for WSL and podman-vm kinds

Today's verbs:

- Base-kind enshrinements consumed by Cloud Build during conjure (implicit pull, no operator-facing verb)
- Hallmarks consumed by retriever via `summon` (`rbw-fs`)

WSL and podman-vm kinds need an operator-facing consumption op (pull + extract + transport). Two paths:

1. Broaden `summon` to cover any retriever-pulled GAR thing with kind switch. Risks blurring summon's clean current identity (vessel hallmarks specifically).
2. Mint a sibling consumption verb for substrate kinds. Cleaner taxonomically but adds vocabulary.

Defer until the first WSL retrieval op is sketched in code — implementation reveals what natural shape feels right.

### Delivery sequencing

Two options on heat content scope:

1. **All three kinds in one heat**: ship abstraction + base narrowing + WSL stack + podman-vm stack end-to-end. The Rule of Three earns its keep only with three real implementations — partial delivery is premature abstraction.
2. **Abstraction + WSL only**: ship the verb-generalization and prove it with WSL (operationally urgent for caparison-w unblock). Podman-vm slots in as a sibling heat once the pattern proves out.

Lean toward option 1 — designing for three with only two real implementations is the failure mode Rule of Three is supposed to prevent. But option 2 ships unblocking value sooner. Decision pending commit-to-development.

### WSL Dockerfile content

A bare `FROM ubuntu:24.04` rootfs has dpkg minimization (`policy-rc.d`, removed locales, no `/etc/wsl.conf`) that Microsoft's appx-delivered Ubuntu silently works around. The WSL-kind Dockerfile adds a thin layer re-adding those bits — small but must exist and be tested with the four-session garrison ceremony. Invigilate fact needed: `useradd`, `install`, `bash -lc` round-trip work inside the imported distro.

### Tarball transport for WSL

`scp.exe` over admin SSH from curia → Windows host: should work, but Windows OpenSSH has historically surprised people on binary transfers. Verify once before committing the transport pattern.

### Podman-vm source format and refresh cadence

Quay's hostile lifecycle (new image every ~3 hours, retention measured in days) means the podman-vm enshrinement pipeline needs scheduled or trigger-driven refresh, not on-demand. Pin discipline open: which quay digest gets enshrined, how often, who triggers, how aging is managed. Probably wants its own paddock subsection once design lands.

## Heat nature

Planning/design heat — no paces yet. Paddock captures locked decisions and open decisions for expansion once development is committed. Will revisit and slate paces against this content when surrounding heats clear and this work fits the queue.

## References

- ₣AV `rbw-implement-gar-mirroring` (retired 260511) — predecessor heat; absorbed motivation (Quay concern, pin discipline, hostile-upstream mirroring). Predated the enshrinement noun's formalization.
- RBSAE `ark_enshrine` — current narrow-scope enshrinement spec
- RBSAS `ark_summon` — retriever consumption of hallmarks (not enshrinements)
- RBSDK `director_invest` / RBSRK `retriever_invest` — sibling-specs-per-noun-kind pattern; verb-sharing precedent
- RBSIA Image Audit — three current artifact domains (hallmarks, reliquaries, enshrinements); enshrinement-domain audit ops gain kind column
- BUSJCW CaparisonWindows — Phase 3 WSL provisioning that will be deleted
- BUSJGW GarrisonWsl — Step 3 admin `wsl --export` and Phase 5 vestige cleanup that will be deleted
- BUSJIW InvigilateWindows — admin WSL distribution fact replaced with seed-tarball presence/digest fact