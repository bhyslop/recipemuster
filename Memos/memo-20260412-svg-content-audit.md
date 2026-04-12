# SVG Content Audit: rbm-abstract-drawio.svg

Audit of the restored architecture diagram (48,452 bytes, commit `0adaa6ff`) against current Recipe Bottle architecture as defined in README.md (post-A8AAG) and RBSCO.

## Pre-Audit Inventory: What the SVG Depicts

All text labels extracted from the restored SVG:

| Label | Concept Represented |
|-------|-------------------|
| Workstation (Linux/Mac/Windows computer) | Developer machine — the host environment |
| Workbench Console / Bash 3.2 Scripts | CLI tooling layer |
| Image Build Service (github action) | Remote build — **stale**: says "github action", now Google Cloud Build |
| OCI Compliant Container Repository | Registry — lists "Offsite Sentry/Censer/Bottle images" and "Podman VM Images" |
| Bottle Service | The crucible runtime concept — **stale label**: now "Crucible" |
| Sentry Container | Sentry — current vocabulary |
| Censer Container (configured network namespace) | Pentacle — **stale label**: "Censer" renamed to "Pentacle" |
| Enclave Network | The isolated network namespace |
| host network | Host-side networking |
| Transit | Network transit path between sentry and internet |
| Workstation Web Browser or similar | User access point for bottle services |
| iptables/dnsmasq config filesystem | Sentry's policy configuration surface |
| Entry Port (guarded) | Inbound port exposure with sentry mediation |
| Uplink Conduit | Outbound path from bottle through sentry |
| volume mounts | Host-to-bottle filesystem sharing |
| (unguarded) | Annotation on a path (possibly direct host network) |
| Bottle Container (unprivileged) | The workload container |
| Internet | External network |

**Structural elements visible**: The diagram shows a two-panel layout:
1. **Top panel**: Workstation containing Workbench Console + Image Build Service + OCI Container Repository (the build/registry side)
2. **Bottom panel**: Bottle Service containing Sentry + Censer + Bottle + Enclave Network + connectivity paths (the runtime side)

## Concept-by-Concept Audit

### 1. Depot

**Verdict: ABSENT — correctly out of scope**

The diagram shows "OCI Compliant Container Repository" as a single box. The Depot concept (GCP project + registry + bucket as a unified facility) is an administrative/provisioning concern — it describes *who owns and funds* the registry, not the architectural data flow. The diagram is an architecture diagram showing how components interact, not an infrastructure ownership diagram. The registry box correctly represents the architectural role without needing to depict Depot-level administrative containment.

If the diagram were showing the provisioning flow (Payor levies Depot, Governor administers it), Depot would be essential. But the diagram shows data flow: code goes to build service, images go to registry, images get pulled to crucible. Depot is implicit in "the registry exists" — depicting it would add administrative detail that clutters an architecture view.

### 2. Vessel Modes (Conjure / Bind / Graft)

**Verdict: ABSENT — correctly out of scope**

The diagram shows a single "Image Build Service" box with a single arrow to the repository. The three vessel modes describe *how* images arrive in the registry — three distinct paths through the build service. This is a process distinction, not an architectural component distinction. All three modes produce the same output (an image in the registry); they differ in build-time behavior.

An architecture diagram correctly shows "build service produces images in registry" without decomposing the three arrival paths. A process/workflow diagram would be the right place for vessel modes. Adding three parallel arrows or three sub-boxes inside the build service would add workflow detail to an architecture view.

### 3. Hallmark

**Verdict: ABSENT — correctly out of scope**

Hallmarks are the provenance-tracking unit — timestamps, `-image`/`-about`/`-vouch` artifact triples. This is a data model concern (how builds are identified and tracked), not an architectural component. The diagram correctly shows the registry as a storage component without decomposing its internal artifact structure. Hallmark detail belongs in a data model or lifecycle diagram.

### 4. Nameplate as Policy Declaration

**Verdict: ABSENT — should consider adding**

The diagram shows "Bottle Service" (crucible) with its three containers, but there is no representation of *what configures which sentry policies apply to which bottle*. The nameplate is the binding: it maps a specific bottle vessel + sentry vessel + network policy into a runnable unit. The diagram shows the runtime result (sentry enforcing policy on a bottle) but not the configuration source.

**However**: adding nameplate to this diagram may not be the right call. The diagram's current scope is "how components interact at runtime." Nameplate is a configuration-time concept — it determines which containers start together, but it's not a runtime component that appears in the running system. The `iptables/dnsmasq config filesystem` label already gestures at "configuration drives policy." A nameplate box would need to connect to both the sentry and the bottle, adding a configuration-layer concept to a runtime-layer diagram.

**Recommendation**: ABSENT — correctly out of scope for a runtime architecture diagram. The configuration surface is already represented by the "iptables/dnsmasq config filesystem" label. Nameplate is an operational concept, not an architectural component.

### 5. SLSA Provenance / Vouch Chain

**Verdict: ABSENT — correctly out of scope**

SLSA attestation is a property of the build output, not an architectural component. The diagram shows "Image Build Service" producing images — the attestation chain is an attribute of that production process. Depicting SLSA would require showing metadata flows (attestation artifacts alongside images) which is a supply-chain data-flow concern, not a component architecture concern.

### 6. Enshrine (Base Image Mirror)

**Verdict: ABSENT — correctly out of scope**

Enshrining mirrors upstream base images into the depot registry. This is a provisioning operation — it populates the registry before builds run. The diagram shows the steady-state architecture (build service uses registry, crucible uses images from registry). Enshrine is a setup step, not a runtime component or data path. It would belong in a provisioning/setup workflow diagram.

### 7. Role Separation (Payor / Governor / Director / Retriever)

**Verdict: ABSENT — correctly out of scope**

Roles describe *who* can perform operations, not *what components exist*. The diagram is a component/data-flow architecture, not an access control diagram. Adding roles would require a different diagram type (perhaps overlaying permission boundaries on the existing architecture). The four roles are organizational concepts that govern access to the components shown, but they are not components themselves.

### 8. Cloud Build Dual-Pool / Egress Lockdown

**Verdict: ABSENT — correctly out of scope**

The current label says "Image Build Service (github action)" — already stale in naming the build service. The dual-pool architecture (private pools for egress lockdown) is a deployment configuration detail of the build service, not a separate architectural component. The diagram correctly shows "build service" as a single box. Internal build-service architecture (pool types, egress configuration) would belong in a build-infrastructure-specific diagram.

Note: the "github action" label is a vocabulary rename issue for sibling pace A5AAF, not a content-scope issue for this audit.

### 9. Pentacle as Namespace Establisher

**Verdict: PRESENT — depicted as "Censer Container (configured network namespace)"**

The pentacle concept is fully represented in the diagram's structure. The "Censer Container" box shows:
- Its role: "(configured network namespace)" — establishing the namespace
- Its position: between the sentry and the bottle in the containment hierarchy
- Its relationship: the enclave network connects through it

The label is stale ("Censer" should be "Pentacle"), but that's a vocabulary issue for A5AAF, not a content gap. The *concept* — a container whose job is to establish a privileged network namespace that the bottle then shares — is correctly depicted.

## Summary

| # | Concept | Verdict | Rationale |
|---|---------|---------|-----------|
| 1 | Depot | Absent — correct | Administrative ownership, not architectural data flow |
| 2 | Vessel modes | Absent — correct | Process distinction (how images arrive), not component distinction |
| 3 | Hallmark | Absent — correct | Data model (artifact identification), not architectural component |
| 4 | Nameplate | Absent — correct | Configuration-time binding, not runtime component; config surface already shown |
| 5 | SLSA / Vouch | Absent — correct | Build output property, not architectural component |
| 6 | Enshrine | Absent — correct | Provisioning operation, not runtime data path |
| 7 | Roles | Absent — correct | Access control overlay, not component architecture |
| 8 | Cloud Build details | Absent — correct | Build service internal configuration, not architectural component |
| 9 | Pentacle | **Present** | Depicted as "Censer Container" — concept correct, label stale |

**Result: 1 of 9 concepts present; 8 absent and all correctly out of scope.**

The diagram's scope is runtime component architecture — how the workstation, build service, registry, and crucible containers interact. All 8 absent concepts belong to other diagram types (provisioning workflow, data model, access control, build-internal configuration). The diagram does not need content additions.

## Stale Vocabulary (for A5AAF, not this audit)

The following labels need renaming (documented here for reference, not acted on):

| Current SVG Label | Should Be |
|---|---|
| Bottle Service | Crucible |
| Censer Container | Pentacle |
| github action | Google Cloud Build |
| Podman VM Images | (remove or update — podman deferred, docker is current runtime) |
| OCI Compliant Container Repository | Google Artifact Registry (or keep generic) |

## Recommendation

**No content additions needed.** The diagram's architectural scope is coherent and complete for what it depicts. All absent concepts belong to different diagram types. Proceed directly to A5AAF vocabulary rename.

No deferred items for `jji_itch.md` — the absent concepts don't represent gaps in *this* diagram; they represent content that belongs in diagrams that don't exist yet. If future work produces provisioning-flow or access-control diagrams, those would be new artifacts, not additions to this one.
