# Release Procedure

The release qualification ceremony for the project maintainer — five operator steps, roughly one hour wall-clock, with cloud cost on the order of two GCP projects per run.
The ceremony exists to catch silent first-build assumptions that the routine `tt/rbw-tf.QualifyFast.sh` and `tt/rbw-tr.QualifyRelease.sh` tiers tolerate by design.

[Payor](README.md#Payor) OAuth is the only prerequisite credential.
All other credentials — [Governor](README.md#Governor), [Director](README.md#Director), and [Retriever](README.md#Retriever) service-account [RBRA](README.md#RBRA) files — are minted by the qualification itself, not restored from backup.
The entire credential chain must be reproducible from a clean local state.

**Failure mode contract.** Mid-qualification failure means start over from step 2, not patch-and-continue.
Patching forward is exactly the bug class this ceremony exists to catch — accumulated state hides first-build assumptions.

## 1. Confirm Payor health

- [Refreshes](README.md#Refresh) the [Payor](README.md#Payor) OAuth token so subsequent [Payor](README.md#Payor)-authority operations do not fail mid-qualification on token expiry
- Use `tt/rbw-gPI.PayorInstall.sh` instead if [Payor](README.md#Payor) credentials have never been [Installed](README.md#Install) on this workstation
- Success: the command exits clean
- Failure: re-[Establish](README.md#Establish) the [Manor](README.md#Manor) before proceeding — the ceremony cannot run without a healthy [Payor](README.md#Payor)

```
tt/rbw-gPR.PayorRefresh.sh
```

## 2. Marshal zero, then commit

The marshal-zero operation returns local state to a blank template:

- Blanks [Repo Regime](README.md#RBRR) fields ([Depot](README.md#Depot) identity, project ID, prefixes)
- Deletes all [RBRA](README.md#RBRA) credential files ([Governor](README.md#Governor), [Director](README.md#Director), [Retriever](README.md#Retriever))
- Blanks [Hallmark](README.md#Hallmark) pins in each [Nameplate](README.md#Nameplate) regime file ([RBRN](README.md#RBRN))
- Blanks depot-scoped fields in each [Vessel](README.md#Vessel) regime file ([RBRV](README.md#RBRV))

Then commit the result with the marshal-zero signature in the commit message — step 3 detects that signature on `HEAD` and refuses to run otherwise.

- Success: working tree clean and `HEAD` carries the signature
- Failure: review the marshal output for files the operation would not zero, resolve, and retry

```
tt/rbw-MZ.MarshalZeroes.sh
```

## 3. Run the pristine qualification

The release gate.

**Entry contract** — `rbw-tP` refuses to run unless ALL of the following hold:

- marshal-zero signature on `HEAD`
- working tree clean
- [RBRA](README.md#RBRA) files absent
- [Repo Regime](README.md#RBRR) fields blank
- [Hallmark](README.md#Hallmark) pins blank in every [Nameplate](README.md#Nameplate) regime ([RBRN](README.md#RBRN))
- depot-scoped fields blank in every [Vessel](README.md#Vessel) regime file ([RBRV](README.md#RBRV))

The contract is enforced by the test itself, not by ceremony or operator discipline — that property is what lets this tier catch silent-first-build assumptions by construction.

**Cost** — ~1 hour wall-clock plus two GCP projects per run:

- one throwaway [Depot](README.md#Depot) created and torn down by the `pristine-lifecycle` fixture's `depot-lifecycle` case
- one canonical [Depot](README.md#Depot) [Levied](README.md#Levy) by the canonical-infrastructure setup phase
- GCP project deletion is asynchronous (30-day soft delete) — each run leaves pending-delete projects in the project list; this is accepted cost

**Sequence**:

- `pristine-lifecycle` fixture — marshal-zero gate plus idempotent throwaway lifecycles for [Depot](README.md#Depot), [Governor](README.md#Governor), [Retriever](README.md#Retriever), and [Director](README.md#Director)
- canonical infrastructure setup — [Mantle](README.md#Mantle) the [Governor](README.md#Governor) + deploy [RBRA](README.md#RBRA), [Charter](README.md#Charter) the [Retriever](README.md#Retriever) + deploy [RBRA](README.md#RBRA), [Knight](README.md#Knight) the [Director](README.md#Director) + deploy [RBRA](README.md#RBRA), [Levy](README.md#Levy) the canonical [Depot](README.md#Depot)
- supply chain — inscribe [Reliquary](README.md#Reliquary), [Ordain](README.md#Ordain) all [Hallmarks](README.md#Hallmark)
- [Crucible](README.md#Crucible) suite
- cleanup

Outcome:

- Success: green tally and clean exit
- Failure: return to step 2 — never patch-and-continue

```
tt/rbw-tP.QualifyPristine.sh
```

## 4. Prepare the upstream release

- Claude Code slash command, not a tabtarget — the contribution ceremony is interactive by design
- Drives the upstream contribution review and produces the contribution-ready branch
- Success: the ceremony reports a clean upstream-ready state
- Failure: address findings and re-run; if the underlying issue is structural, return to step 2

```
/rbk-prep-release
```

## 5. Push to the upstream remote

- Universal git operation; no tabtarget
- Pushes the contribution-ready branch produced by step 4
- Success: the upstream accepts the push
- Failure: resolve the upstream-side reason (branch protection, force-push policy, etc.) and retry — the local state from step 4 remains valid

```
git push <upstream> <branch>
```
