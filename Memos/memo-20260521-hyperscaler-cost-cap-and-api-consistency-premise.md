# Hyperscaler Cost-Cap and API-Consistency Premise

Date: 2026-05-21

## Why this memo exists

Two recurring frustrations with cloud infrastructure are not bugs to
wait out — they are structural properties of the vendors we depend on,
and they impose permanent, budgetable engineering cost on Recipe
Bottle. This memo characterizes both across the providers with deep
staying power, so the next time we "strap on" a safety feature we treat
it as a known premise with a known shape, not a fresh surprise.

The two axes:

- **Axis A — Cost-overrun protection.** When an operator mistake (a
  runaway build loop, a forgotten worker pool, a leaked credential)
  occurs, how much unbounded spend can it obligate before anything
  stops it? What native brakes exist?
- **Axis B — Operation-completion contracts.** When a mutating API call
  returns, is the work *done*? Does the vendor expose a terminal-state
  contract ("the work is done" / "the work will never be done") with a
  bounded poll, or does it leave eventual-consistency race conditions in
  the customer's lap?

Both already have local precedent paying this tax:
`Memos/memo-20260513-iam-propagation-race-director-invest-gar.md` and
`Tools/rbk/vov_veiled/RBSCIP-IamPropagation.adoc` are Axis-B scaffolding;
`Memos/memo-20260517-cloudbuild-default-quota-wedge` is the Axis-A lever
in action. This memo is the premise they instantiate.

## The headline finding

For both axes, the capability to do the right thing demonstrably
**exists** and is sometimes delivered excellently — but is **withheld on
exactly the surfaces where it would constrain vendor revenue or has not
been prioritized**. Neither failure is a physics constraint. Both are
product choices. That reframing is the load-bearing premise: we are not
waiting for the vendor to fix an oversight; we are permanently absorbing
a deliberate posture.

---

## Axis A — Cost-overrun protection

### Provider characterization

| Provider | Native hard $-cap on general consumption? | Mechanism | Auto-stop destructiveness |
|----------|-------------------------------------------|-----------|---------------------------|
| **GCP** | No (lone exception: Gemini API project spend caps, launched 2026-03-16, scoped to that one API) | Budget alerts (monthly, soft). True hard stop = budget → Pub/Sub → Cloud Function that detaches billing. | **Most destructive.** Detaching billing shuts down all resources, possibly "irretrievably deleted," no graceful recovery. |
| **AWS** | No true hard cap | AWS Budgets + **Budget Actions**: attach a restrictive IAM policy or SCP that blocks *new* resource creation, or stop targeted EC2/RDS. Runs automatically or after manual approval; auto-reverses when spend drops (e.g. new month). | **Least destructive auto-stop.** Blocks new provisioning while leaving running resources alive; reversible. |
| **Azure** | Partial — a native "spending limit" exists **only for credit-based subscriptions** (free trial, MSDN/Visual Studio, dev/test); disables resources when the credit is exhausted. Not available on pay-as-you-go. | PAYG: budgets + Action Groups → Automation runbooks (custom, same pattern as the others). | Custom; whatever the runbook does. |
| **OCI (Oracle)** | No auto-stop from budgets, but **compartment quotas are first-class enforceable hard limits** that block resource creation. | Budgets (alert) + compartment quotas (enforced) + IAM deny policies; Events → Functions for remediation. | Quotas block creation non-destructively. |
| **GitHub (Microsoft)** | **Yes — clean native hard cap.** "Stop usage when budget limit is reached" blocks further usage until the next cycle or the limit is raised. No payment method on file = automatic hard cap at the included quota. | Budget with hard-stop toggle; alerts at 75/90/100%. | Clean — blocks usage, non-destructive. |

### What this establishes

1. **The three metered-consumption hyperscalers (GCP, AWS, Azure-PAYG)
   uniformly refuse a clean native dollar hard-cap.** A customer mistake
   can obligate unbounded spend; the simplest protective primitive is
   the one conspicuously not built into the unbounded surface.

2. **It is provably a choice, not a technical limit.** GitHub
   (Microsoft) ships a clean hard cap. Azure ships one — but only on
   *credit* subscriptions, where the cap protects Microsoft's giveaway
   rather than capping Microsoft's revenue. GCP shipped one for the
   Gemini API in 2026. The machinery exists; it is selectively withheld
   from pay-as-you-go consumption, where a cap would bound the bill.

3. **The portable, non-destructive native brake is the QUOTA, not the
   budget.** Budgets alert; quotas enforce. Cloud Build concurrency /
   daily-build quotas (GCP), compartment quotas (OCI), and service
   quotas / SCP-based blocks (AWS) are real, vendor-enforced ceilings on
   resource *rate or count* — they bound the dominant cost driver
   (build minutes) deterministically and without standing infrastructure.
   They cap units, not dollars, which is the honest shape of the
   protection actually available.

4. **Any true auto-stop is customer-built, laggy, and destructive — on a
   gradient.** Cost data lags hours (sometimes ~a day), so every
   automated cap overshoots. The least-bad model to emulate is AWS's
   policy-attach (block new spend, keep running resources, auto-reverse);
   the worst is GCP's detach-billing (nuke everything, manual recovery).

**Axis-A premise to cite:** *Unbounded-spend liability is a deliberate,
structural property of pay-as-you-go cloud. The responsible posture is
budget alerts (early warning) + quota caps (the real enforced brake on
the cost driver); a true dollar auto-stop is always customer-built
engineering with lag and destructiveness, and is a recurring,
non-optional cost line — not a one-time setup.*

---

## Axis B — Operation-completion contracts

### Provider characterization

| Provider | Terminal-state contract | Reality on the painful operations |
|----------|------------------------|-----------------------------------|
| **Azure** | **Best in class.** ARM async pattern: `201`/`202` + `Azure-AsyncOperation`/`Location` header + `Retry-After` + `provisioningState` with explicit terminal values (`Succeeded`/`Failed`/`Canceled`) + `200` on completion. This *is* the "work is done / work will never be done, with a reasonable poll interval" contract, specified and standardized. | Background propagation can still trail ARM acceptance, but the *completion contract is well defined*. |
| **GCP** | **Good spec, incomplete coverage.** AIP-151 defines long-running operations with a `done` field + terminal state; AIP-121 states that completion of a management-plane operation must mean the resource's existence and all user-settable values have reached steady state. | The spec is not applied to the operations that hurt: IAM grant propagation and billing linkage expose no terminal signal — hence our own handbook caveats ("GCP IAM changes are eventually consistent; wait and refresh"), `RBSCIP-IamPropagation.adoc`, and the 2026-05-13 race memo. |
| **AWS** | **No universal contract.** Some SDK "waiters" do client-side polling; there is no general server-side LRO. IAM is eventually consistent *by design* (~4s auth-plane enforcement lag, sometimes longer). | Cross-account assume-role and freshly-granted permissions fail intermittently; the documented workaround is retries and fixed `time_sleep` delays. AWS made S3 **strongly** read-after-write consistent in Dec 2020 at no cost or performance penalty — proof that consistency is an engineering choice — yet IAM stays eventual to serve >500M auth calls/sec globally. |
| **OCI** | Uses an async **work-request** resource with queryable state. | (Less first-hand data; pattern is present.) |

### What this establishes

1. **The contract the user wants already exists and is sometimes
   delivered excellently** — Azure ARM's async operation pattern and
   GCP's AIP-151 are genuinely good. The failure is not conceptual.

2. **It is delivered with selective coverage, and the gaps land on the
   painful operations.** IAM propagation (every provider) and GCP
   billing linkage — the operations that actually break our automation —
   are precisely the ones left without a terminal-state signal.

3. **Some eventual consistency is legitimately hard; the missing
   *contract* is the unforced error.** IAM's eventual consistency has a
   real justification (global propagation, half-billion calls/sec).
   Fairness requires conceding that. But "we cannot make authorization
   instantly consistent at that scale" does **not** excuse "we will not
   give you a pollable completion signal with a bounded SLA." The first
   is physics; the second is a contract decision. S3-2020 proves the
   vendor closes consistency gaps when motivated.

**Axis-B premise to cite:** *A terminal-state operation contract is a
solved, available pattern that the giants apply unevenly, omitting it on
exactly the IAM/billing operations that cause races. Eventual
consistency is occasionally a legitimate scale necessity; the absence of
a completion contract with a bounded timeout is not. Retry/poll/timeout
scaffolding is therefore a permanent design premise for us, not a
transient workaround awaiting a vendor fix.*

---

## Implications for Recipe Bottle

- **As a cost driver.** Safety features are recurring engineering, not
  setup. The honest, achievable shape is: native budget alert (warn) +
  native quota cap on Cloud Build (the enforced brake on the dominant
  driver) + accept that any dollar auto-stop is custom, laggy, and
  destructive. If we ever build the auto-stop, emulate AWS's policy-block
  model, not GCP's detach-billing. The Cloud Build quota wedge
  (`memo-20260517-cloudbuild-default-quota-wedge`) is this premise made
  concrete.

- **As a risk-management concept.** Both axes are *premises to design
  against*, not defects to file and wait on. The consistency tax is
  already being paid (`RBSCIP-IamPropagation.adoc`,
  `memo-20260513-iam-propagation-race-director-invest-gar.md`); this memo
  names *why* it is permanent. The cost-liability tax is the open one:
  until we add the alert+quota layer, an operator mistake during
  onboarding has unbounded downside.

- **Vendor selection.** When weighing where to host artifact registries
  and build pipelines, these two axes are first-class criteria. No
  hyperscaler scores well on Axis A. On Axis B, Azure's ARM contract is
  the strongest among the three; GCP has the better *spec* but applies it
  incompletely; AWS leans on client-side waivers and design-time
  eventual consistency.

## Sources

- [GCP: Quotas and limits — Cloud Billing](https://docs.cloud.google.com/billing/quotas)
- [Cyclenerd/poweroff-google-cloud-cap-billing (the detach-billing pattern, prebuilt)](https://github.com/Cyclenerd/poweroff-google-cloud-cap-billing)
- [Gemini API Spend Caps (launched 2026-03-16)](https://gemilab.net/en/articles/gemini-api/gemini-api-spend-caps-guide)
- [AWS Budgets — Configuring budget actions](https://docs.aws.amazon.com/cost-management/latest/userguide/budgets-controls.html)
- [AWS: Introducing Budget Controls](https://aws.amazon.com/blogs/aws-cloud-financial-management/introducing-budget-controls-for-aws-automatically-manage-your-cloud-costs/)
- [Azure spending limit (credit-based subscriptions only)](https://learn.microsoft.com/en-us/azure/cost-management-billing/manage/spending-limit)
- [Oracle: Enforced budgets on OCI using functions and quotas](https://blogs.oracle.com/cloud-infrastructure/enforced-budgets-on-oci-using-functions-and-quotas)
- [GitHub: Setting up budgets to control spending on metered products](https://docs.github.com/en/billing/how-tos/set-up-budgets)
- [Azure ARM: Status of asynchronous operations](https://learn.microsoft.com/en-us/azure/azure-resource-manager/management/async-operations)
- [Google AIP-151: Long-running operations](https://google.aip.dev/151)
- [AWS S3 Update — Strong Read-After-Write Consistency (Dec 2020)](https://aws.amazon.com/blogs/aws/amazon-s3-update-strong-read-after-write-consistency/)
- [AWS IAM eventual consistency and Terraform (the time_sleep workaround)](https://blog.pesky.moe/posts/2023-09-11-iam-consistency-terraform/)

<!-- eof -->
