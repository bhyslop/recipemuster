# Bedrock Quire — Consume-Side Shaping

Date: 2026-06-09

Status: intent/shaping. Nothing here is built. This memo records the *shape* of a
future Recipe Bottle subsystem for safely consuming frontier models on AWS Bedrock,
so the durable conclusions survive into the roadmap and a later build. It supersedes
nothing; it **extends** `memo-20260415-crucible-conduit-architecture.md` (which predates
the identity model and the consume/host split) and **rides** the citizen/capability
identity model of heat ₣BZ (`rbk-14-mvp-citizen-model`).

This is a *successor* to the 2026-04-15 conduit memo, not an edit of it — that memo is a
dated snapshot and keeps its provenance. Where the two disagree, this one is current.

---

## 1. Scope — consume, not host

The need is **safe, private use of frontier models** (Claude and peers on Bedrock) for the
operator and the operator's users. Conversations must stay private. That is the whole point.

Explicitly **in scope**: invoking *native foundation models already on Bedrock* from a
contained client.

Explicitly **out of scope** (collapses away because we are not hosting our own weights):
the embassy / bullion / entrust / Custom-Model-Import delivery path. We are not pushing
models to AWS. The roadmap's `chantry` entry imagined both a custom-imported model and a
native one; only the native-consume half lives here.

A consequence worth stating plainly up front (it corrects the operator's initial mental
model): **Bedrock is not an "instance" you run and keep secure.** It is a fully managed,
serverless, *stateless* request/response API. There is no box to harden, and — per Recipe
Bottle's **asymmetric-sovereignty** stance — no AWS principal Recipe Bottle owns. What you
control is: which model you invoke, the IAM perimeter, the network path, the logging switch,
and the AWS account itself. "Secure within Recipe Bottle" therefore splits: the *consuming*
end is secured inside a bottle; the *serving* end is yours in AWS, stood up from
Recipe-Bottle-shipped templates but owned by you wearing an AWS hat.

## 2. Why Recipe Bottle holds the persistent state

Bedrock is stateless: each call is independent, and even the conversation history is held by
the *client* and re-sent every turn. So the durable truth of "which model, reached how, under
whose identity, at what cost ceiling" has **nowhere to live on the Bedrock side.** That is the
argument for a persistent Recipe Bottle config model: statelessness over there forces
persistence over here.

## 3. The privacy model — six boundaries, one Palisade

"Completely private" decomposes into distinct boundaries with distinct strengths. The first
four can be driven to near-airtight; the last two are the honest ceiling.

- **A — From the public internet (network).** PrivateLink keeps traffic on the AWS network;
  allowlist + TLS is the public-path fallback. Strong; delivered client-side by the conduit.
- **B — From the model provider.** Architectural: Bedrock runs each provider's model in an
  isolated deployment account the provider cannot reach; zero provider operator access. Under
  Commercial Terms (which Bedrock is), the provider does not train on your data. Strong.
- **C — From your own logs.** Bedrock defaults to zero-data-retention; model-invocation
  logging is opt-in, off by default. The main self-inflicted leak is turning logging on and
  under-securing the sink. Default: leave it off.
- **D — From AWS retention/training/sharing.** Contractual + ZDR; abuse detection is fully
  automated, no human review, no storage (verify per model — a few carry a 30-day
  classifier-flagged exception; Claude is in the clean automated-only camp).
- **E — From AWS hardware at the moment of inference.** The boundary **no hosted frontier
  model can cross.** The model must read your plaintext to answer; AWS silicon processes
  cleartext. The guarantee is *contractual + architectural + compliance-audited* — genuinely
  strong, the same class of trust under all enterprise cloud — but trust *placed*, not trust
  *eliminated*. If the threat model requires "the operator mathematically cannot see it,"
  nothing hosted meets that.
- **F — From server-side agency egress (NEW).** Plain inference (`Converse`/`InvokeModel`)
  has *zero* network reach — the model is a pure prompt→completion function and even a tool
  *decision* never touches the wire. But agentic surfaces — Bedrock Agents with Lambda action
  groups, and AgentCore Browser / Code Interpreter / the server-side tool execution of the
  Responses API — execute **server-side** and can reach the open internet **behind the
  Palisade, entirely outside the sentry's view.** This is opt-in (a different, more complex
  API surface; you never get it by accident), and the **only** lever over it is **IAM**, not
  the sentry.

**The Palisade.** Boundaries A, C, and the client side of B/D sit inside our realm and get
full sovereignty. E is irreducible foreign ground — characterize and contain, do not pretend
to govern. F is the subtle one: the sentry governs the *bottle's* egress and is structurally
blind to *Bedrock's* server-side agency, so F is governed only by IAM and by *not invoking
agentic surfaces*. The private Quire uses non-agentic surfaces only; agentic Bedrock is a
deliberate, named re-opening of the egress question on AWS's side of the Pale.

## 4. What Recipe Bottle does — and does not — do

Recipe Bottle **establishes and maintains the binding**: the config, the identity, the reach,
the cost envelope. The **inference calls are made by whatever runs inside the bottle** (an
app, a chat client, an orchestrator) — *not* by Recipe Bottle. There is therefore **no
"submit" verb**. Recipe Bottle's surface ends at "the path and identity are ready and
verified." The only place Recipe Bottle itself touches Bedrock is a one-shot *probe* (does it
answer?), a health check, not a workload call.

This single rule shapes the whole vocabulary: the verbs are *lifecycle of the binding*, never
invocation.

## 5. Vocabulary

The roadmap's working words `chantry` (target) and `conduit` (reach) both start with `C`,
which stumbles against Crucible / Census. Renamed to unbound primary letters:

| Concept | Noun | Verbs | Register | Cardinality |
|---|---|---|---|---|
| The model endpoint a bottle consumes | **Quire** | `endow` / `dissolve` | ecclesiastical (heir to *chantry*'s "where the voice issues") | 0..N |
| The peer-bearing private reach into an AWS VPC | **Outpost** | `raise` / `strike` | camp/diplomatic (sibling to *embassy*) | 0..1 |

All four verbs are collision-free across RB and JJK today (`grep` lands clean — the
findability rationale the operator asked us to favor).

**Two tiers of lifecycle.** The Outpost is the heavy, shared, singleton reach (raise once,
strike once). Quires are the light bindings that ride it (endow/dissolve freely). This is the
conduit-vs-target split, resolved: the reach object is first-class, not footprint — its 0..1
cardinality is exactly what earns it a noun and its own verbs.

**The peer is *not* separately named.** It is the Outpost's compute; `raise`/`strike` operate
on the Outpost, which includes the peer. No peer-lifecycle verbs — the peer is hourly-cheap,
stop/start saves only noise, and the endpoint/EIP are standing anyway.

**Prefix allocation (open minting item).** A Quire regime is a natural sibling to
nameplate/vessel; `RBRQ` is a free slot in the `RBR*` family. The Outpost cannot be `RBRO`
(taken by the OAuth regime), so its config home needs a non-`RBRO` slot — deferred to the
later minting pass (which also owns the unrelated Census/Crucible `C` collision).

## 6. Config shape — three orthogonal layers

A "Bedrock configuration" is not one blob; it is three independently-variable layers (same
model reachable two ways; same reach under two identities; same identity to three models).
They want composable homes, not a monolith.

- **Target — the Quire.** Fields: model(s); version-pin (Bedrock model IDs drift — pin for
  provenance, as we pin everything else); region + inference profile (single-region or
  geographic, **not global** by default — a silent residency/privacy knob); **`agency`** (see
  below); a capability reference (who may invoke); a reach selector; logging posture
  (`off` by default).
- **Reach — `allowlist` or `outpost`.** This extends the nameplate's network posture. The
  MVP value is `allowlist` with **no Outpost in existence**; the fast-follow raises an Outpost
  and repoints. Egress allowlist scope for a private Quire: the Bedrock endpoint, plus the STS
  endpoint if federating, and **nothing else** — the egress-lockdown crown jewel.
- **Identity — the invoking principal.** MVP: a confined credential placed in the bottle
  (RBRA-style regime), because ₣BZ's federation is future. Fast-follow: federated short-lived
  STS (the envoy pattern), no resting key. Either way the identity is decoupled from
  capability per ₣BZ.

**The `agency` field (boundary F's declared home).** A lattice, not a boolean:

- `none` — plain inference, no server-side tools. (privacy-max; the MVP default and the only
  MVP value)
- `internal` — server-side tools, but only your resources (e.g. a knowledge base over your
  data); no internet.
- `internet` — server-side tools that reach the open web (AgentCore Browser, etc.).
  (privacy-off)

`agency` is *declared intent that IAM enforces* — the same intent-vs-enforcement spine as
₣BZ. The Quire's invocation principal carries an agency clause; an audit can diff declared vs.
actual IAM. Even though every MVP Quire is `agency: none`, declaring it earns its keep: it
makes the privacy guarantee an explicit, enforced property rather than an absence, and gives
the IAM *denial* a documented home.

**Note the two outward directions, kept distinct.** `reach` is how the *bottle* gets *in* to
Bedrock (sentry-governed). `agency` is how far the *model* may act *out* (IAM-governed). Same
Quire, opposite directions, one field each — do not let one blur into the other.

## 7. Staging

**MVP** — allowlist Quire, single Quire, `agency: none`, plain inference, confined credential
in the bottle, no Outpost, no VPC, no standing AWS infrastructure. This already delivers the
privacy story: boundaries A (TLS + egress-lockdown), B, C, D hold; the bottle excludes the
workstation from the data path and can leak nowhere but Bedrock. **The VPC is not what keeps
conversations private — this MVP already does.**

**Fast-follow** — raise an Outpost (PrivateLink), repoint Quires to `reach: outpost`, and
upgrade identity to federation. Build the VPC only when one of three triggers fires:
1. egress **exclusivity** is required — allowlisting Bedrock by static CIDR is ≈ "all of
   Amazon" (no dedicated published range); domain/SNI is tighter but shares IPs; PrivateLink
   is the one *exclusive*, stable-CIDR answer. **This is the VPC's one non-premature job** —
   exclusivity, distinct from the privacy boundaries A–D, which the MVP already covers.
2. you need to hide network metadata (that you talk to Bedrock at all), or
3. you want private paths to *several* AWS services and want to amortize the peer.

Until then the Outpost is premature — and it spends the "no instance to secure" win, since a
private tunnel into a VPC requires a standing termination point (the peer or a pricier managed
VPN; there is no serverless inbound tunnel).

## 8. Cost

The Outpost's *always-on* floor (accrues whether or not anyone infers):

| Component | ~Monthly |
|---|---|
| EC2 `t3.nano` peer | $3.80 |
| Elastic IP (1 public IPv4) | $3.65 (possibly ~$0 under the 750-hr in-use allowance) |
| PrivateLink interface endpoint (1 AZ) | $7.30 |
| Data processing (text payloads) | pennies |
| VPC / subnets / route tables / security groups | free |
| **Total (single-AZ)** | **≈ $15** |

Dual-AZ for High Availability (HA = a redundant copy in a second Availability Zone, so one
datacenter outage doesn't kill the path) adds ~$7/mo. **Skip HA** for a solo/small private
path; reconnect when the AZ recovers.

The **one** cost to actively avoid is a **NAT gateway** (~$32/mo + data). The design routes
through the peer and never needs one; keep it out of the provisioning templates. (A NAT
gateway is for private subnets reaching the *public* internet — which this design never does.)

Comparison: Claude on Bedrock is Sonnet **$3/$15** and Opus **$5/$25** per million in/out
tokens (+~10% cross-region; caching up to 90% off cached input). The entire ~$15/mo floor ≈ 1
million Sonnet output tokens ≈ **one active user's day or two** of real work. At any genuine
usage the conduit hosting is rounding error. **Optimize the Outpost for correctness and
security, not for its hosting cost.**

## 9. Horizon — JJK-choreographed multi-Quire interactions

This is the forward vision that shapes the present design without being built here.

Future Job-Jockey-choreographed LLM interactions enlist *several Quires of different natures*
— different models, different `agency` levels, different residency — across one tightly
managed conversation. A private-counsel Quire (Opus, `agency: none`), a research Quire
(`agency: internet`), a fast-triage Quire (Haiku, `none`); an orchestrator routes among them.
This is the argument for a Quire being a **typed, governed object**, not a thin model pointer.

**"A Quire identity enables this," grounded.** A Quire's identity is, concretely, its
**governed invocation principal** — the IAM role/policy that encodes its nature (model access,
agency ceiling, residency, capability). Enlisting a Quire = using its principal. The identity
is not person-like; it is a *named policy bundle* that makes Quires addressable and governable
in a choreography rather than merely callable.

**The central open question: is a Quire an *instance* or a *menu*?**

- *Instance* framing: a Quire is one concrete configured endpoint; "different natures" means
  several Quires; choreography routes among them.
- *Menu* framing (the operator's lean, and the more promising one): a Quire is a declaration
  of *what is allowed in the current situation* — an envelope of permitted (model, agency,
  cost/availability) combinations. The consumer (JJK) **subselects** a concrete invocation
  from the menu at use-time.

The menu framing yields a clean division of labor that matches everything else in this memo:
**Recipe Bottle governs the envelope — approvals, allowances, availability, cost management —
and the consumer picks within it.** Under it, the Quire's fields become *ceilings and
constraints* rather than fixed values: `agency` is the *maximum* permitted, region is a
*constraint*, model is a *list of approved options*, cost is an *allowance/cap*; a
subselection must fall within. This also strengthens §2 — what Recipe Bottle persists is
precisely the *menu* (the governed, stateless-side-absent envelope), and the concrete
invocation stays ephemeral, mirroring Bedrock's own statelessness. It is offered as the
candidate resolution, **not decreed**; settling instance-vs-menu (or a both-at-different-
altitudes synthesis) is deferred.

**Privacy becomes a graph property.** The deepest consequence: in a multi-Quire choreography,
confidentiality no longer lives in the nodes. If a private Quire's output (`agency: none`)
flows into an agentic Quire's input (`agency: internet`), the private data can exfiltrate
**through the sibling's agency** — and no per-Quire guarantee catches it, because each Quire
is individually well-behaved. So "tightly managed conversations" must carry **information-flow
rules** — which Quire's output may feed which Quire's input — and the confidentiality lives in
the *edges* of the conversation graph.

Elegantly, the `agency` lattice (`none < internal < internet`) **doubles as the clearance
lattice** for that flow control: *output from a lower-agency Quire may not flow into a
higher-agency Quire without an explicit declassification step.* The field added for the MVP is
the same primitive the future choreography needs — a sign the shape is right.

The orchestration itself is a separate future effort (JJK-adjacent) and must not touch the
MVP. Recorded here as a shaping force and one named hazard, nothing more.

## 10. Open / deferred

- **Prefix minting** — `RBRQ` candidate for the Quire regime; Outpost needs a non-`RBRO` slot
  (RBRO taken by OAuth). Deferred to a minting pass that also resolves the Census/Crucible `C`
  collision (parked by the operator for another session).
- **Instance vs menu** — §9's central question; the menu/envelope resolution is the candidate.
- **Cross-Quire information flow** — the graph-property hazard; the agency lattice is its
  clearance lattice. Belongs to the choreography effort, not here.
- **Identity upgrade** — confined-credential MVP → federated STS (envoy) fast-follow; rides
  ₣BZ landing federation.
- **HA** — skip for the private path.
- **Agentic surfaces** — a non-goal for the private Quire unless deliberately re-opened (and
  then governed by IAM, never the sentry).
- **Roadmap correction** — RBSHR's "VPC Service Controls perimeter" entry cross-references the
  conduit VPC as shareable with the VPC-SC perimeter "to serve both consumers." That reads as
  literal sharing and does not hold: VPC-SC is a *GCP* service perimeter around the depot
  project; the Outpost VPC is an *AWS* network. Different clouds; neither shares. Fix the
  cross-reference when RBSHR's conduit/chantry stubs get breadcrumbed to this memo.
- **Terminus consumer class** — §11's open question: container-only consumers (plain
  container networking) vs. a host-resident consumer (needs a local forward-proxy / endpoint
  seam from the terminus, still no host VPN). Container-only is the lean; unsettled.

## 11. Launch shape — the Outpost's local terminus

A same-session addendum extending §5 and §7. The shaping above left one thing unsaid: the
Outpost is "peer-bearing" on the *AWS* side, but where does the tunnel's *local* end live?
Three candidate homes were weighed; two are rejected here and the third adopted as the working
shape.

**Not in the sentry.** Folding the tunnel client into the sentry inverts the sentry's purpose.
The sentry is a *containment-harness* component — it jails a possibly-hostile bottle and exists
to prove the jail holds (the Ifrit/tadmor stance: the inside is the threat). A tunnel terminus
serving a *trusted* reach is the opposite stance — trusted infrastructure, not a contained
prisoner. Crossing the two makes the sentry mean two contradictory things at once. The
predecessor's answer was WireGuard-in-sentry (the 2026-04-15 conduit memo, §12); it is rejected
here.

**Not a bottle-less crucible either.** The tempting shortcut — a crucible charged with no
bottle — is a category error. The crucible *is* the containment harness; a bottle-less crucible
is a jail with no prisoner, and hollowing out its one purpose to borrow its lifecycle plumbing
is abstraction strain, not a feature.

**The factoring that holds.** The image ecosystem (vessel → hallmark → nameplate) is separable
from the runtime harness (crucible). The crucible is one *consumer* of hallmark images. The
local terminus is a **second launch shape** beside it — same build pipeline, different runtime —
not a variant of the first. "A whole different notion of launched container" is the correct
read, and the signal that it belongs beside the crucible, not inside it.

**Its home in the vocabulary: the Outpost's local end.** The terminus is the client-side
counterpart of the EC2 peer — built as its own vessel, launched standalone. The Outpost now has
two ends (AWS peer + local terminus), both operated by `raise`/`strike`; no new verbs, no new
top-level concept. This finishes the migration this memo began: away from WireGuard-in-sentry,
toward a dedicated terminus.

**"No host WireGuard" is a design constraint, and the terminus honors it.** The tunnel client
lives in a *container*; its config rides inside the hallmark through the same pipeline as
everything else, and the workstation host never joins a VPN. That is the "path based on the
container image ecosystem" — literally.

**MVP untouched.** The terminus is fast-follow only: it exists only once there is an Outpost to
terminate. The MVP (allowlist reach, sentry egress-lockdown, no Outpost, no VPC) is unaffected,
and there the sentry keeps doing exactly what it does today. This addendum adds no MVP surface —
consistent with §9's rule that the forward vision must not touch the MVP.

**One open question, named not solved (§10): how a consumer reaches the terminus.** Container
consumers reach it by plain container networking — trivial, never touching the host. A
host-resident consumer (operator tooling outside any container) would need the terminus to
expose a local seam — most plausibly a forward proxy / endpoint-override the SDK points at —
*still* no host VPN. Which consumer class is in scope changes the design; container-only is
dramatically simpler and is the current lean.

## 12. Sources

- `Memos/memo-20260415-crucible-conduit-architecture.md` — the predecessor (approaches,
  WireGuard-in-sentry, service-compatibility survey, threat-model table).
- RBSHR Horizon Roadmap — the `chantry`, `conduit`, `embassy`/`envoy`/`bullion`, and
  `VPC Service Controls` entries (stubs; to be breadcrumbed to this memo).
- Heat ₣BZ (`rbk-14-mvp-citizen-model`) — the citizen/capability identity model this rides;
  intent-vs-enforcement and the declared ledger.
- AWS, verified 2026-06-09: Bedrock is serverless/stateless and ZDR-by-default
  (data-protection docs); provider isolation via dedicated deployment accounts; model
  invocation logging opt-in/off; abuse detection automated/no-human-review; Claude on Bedrock
  under Commercial Terms (no training); cross-region inference residency (geographic vs
  global); plain inference has no network reach while AgentCore Browser/Code-Interpreter and
  Lambda action groups execute server-side and can reach the internet; pricing — Sonnet
  $3/$15, Opus $5/$25 per M tokens, PrivateLink endpoint ~$0.01/hr/AZ, t3.nano ~$3.80/mo,
  public IPv4 ~$3.65/mo, NAT gateway ~$0.045/hr.
