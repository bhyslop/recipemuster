export const meta = {
  name: 'lode-replan-recommendations',
  description: 'Ground-truth the four open calls in ₢BHAAB against landed code, verify, feed recommendations',
  phases: [
    { title: 'Investigate', detail: '5 parallel readers ground-truth the open calls against ₣BX-landed code' },
    { title: 'Verify', detail: 'independent skeptic re-checks load-bearing claims per area' },
  ],
}

const INVESTIGATION_SCHEMA = {
  type: 'object',
  properties: {
    area: { type: 'string' },
    findings: {
      type: 'array',
      items: {
        type: 'object',
        properties: {
          claim: { type: 'string' },
          evidence: { type: 'string', description: 'file:line or quoted snippet' },
          confidence: { type: 'string', enum: ['high', 'medium', 'low'] },
        },
        required: ['claim', 'evidence', 'confidence'],
      },
    },
    recommendation_input: { type: 'string', description: 'the assessment relevant to the planning call' },
  },
  required: ['area', 'findings', 'recommendation_input'],
}

const VERIFICATION_SCHEMA = {
  type: 'object',
  properties: {
    area: { type: 'string' },
    verdicts: {
      type: 'array',
      items: {
        type: 'object',
        properties: {
          claim: { type: 'string' },
          holds: { type: 'boolean' },
          note: { type: 'string' },
        },
        required: ['claim', 'holds', 'note'],
      },
    },
    corrections: { type: 'string', description: 'corrections/additions to the recommendation, or empty if it stands' },
  },
  required: ['area', 'verdicts', 'corrections'],
}

const SHARED = `You are investigating the RBK "Lode" subsystem (fetched-side capture of upstream artifacts into Google Artifact Registry / GAR) inside the repo at the current working directory, to inform a PLANNING decision. Read real files; cite file:line for every load-bearing claim.

Vocabulary you need:
- A *Lode* is ONE GAR package holding captured upstream bytes (the atomic-delete unit). A *touchmark* is its identifier (a stamp like b260605103342). Members + provenance ride as TAGS within the one package.
- Capture is per-kind. The *bole* kind (an upstream base image) is the LANDED PILOT: verb *ensconce* (colophon rbw-lE). Read-only *divine* (rbw-ld) enumerates/inspects Lodes; *banish* (rbw-lB) deletes one whole Lode. *augur* is a PLANNED read-only single-Lode inspect verb (rbw-la).
- Remaining kinds to build as "verticals": tool (verb fetter, rbw-lF), reliquary (conclave, rbw-lC, an N-member date-cohort), wsl (underpin, rbw-lU), podvm (immure, rbw-lI, multi-member, two quay families).
- A sibling heat ₣BX LANDED a data-driven "spine" (Tools/rbk/rblds_Spine.sh) so each kind is a thin body file (an ordered recipe of step rows + an opaque substitutions blob) the spine consumes — no per-kind branching in the spine. The bole body is Tools/rbk/rbldb_Bole.sh. Cloud-side capture steps live in Tools/rbk/rbgjl/ (rbgjl01-ensconce-capture.sh, rbgjl02-assemble-push-vouch.sh).
- Two live "forks" are slated for later retirement via cutover: enshrine (RBSAE, the old narrow base-mirror; GAR namespace rbi_es) and inscribe (rbfli_Inscribe.sh, the old reliquary tool-mirror; GAR namespace rbi_rq).
- Use Tools/rbk/claude-rbk-acronyms.md as the file/acronym map if you need to locate a module.`

const SPECS = [
  {
    key: 'augur',
    label: 'augur/divine reality',
    prompt: `${SHARED}

QUESTION: Does the \`augur\` read-only single-Lode inspect verb exist yet in CODE, or does \`divine\` (rbw-ld) currently handle BOTH the enumerate-all grain AND the inspect-one grain?

Read: Tools/rbk/rbld0_cli.sh, Tools/rbk/rbldl_Lifecycle.sh, Tools/rbk/rbld0_Lode.sh, Tools/rbk/vov_veiled/RBSLD-lode_divine.adoc, Tools/rbk/vov_veiled/RBSLA-lode_augur.adoc. Run \`ls tt/rbw-la* tt/rbw-ld*\` and grep Tools/rbk/rbtd/src/rbtdgc_consts.rs for DIVINE and AUGUR.

Determine precisely: (1) what \`rbw-ld\` does with NO argument vs WITH a touchmark argument (does one verb cover both grains today?); (2) whether \`rbw-la\` augur exists as a colophon / CLI facet / function anywhere; (3) whether the RBSLA spec describes augur as a separate verb that is written-but-not-coded, or is itself a stub; (4) exactly what realizing the augur split would require (new colophon file, a CLI multifacet entry, splitting divine's inspect branch into its own function, a theurge const, etc.).

recommendation_input: classify augur as (a) already done, (b) small scaffold-residue folded into the first pace, (c) its own tiny standalone pace, or (d) safely deferrable beyond this heat — and state the concrete work it entails either way.`,
  },
  {
    key: 'scaffold',
    label: 'scaffold residue',
    prompt: `${SHARED}

QUESTION: Re-derive the EXACT "scaffold residue" — the kind-registration surface that ₣BX did NOT build — that must land (serial, first) before the 4 remaining kind verticals can be slated.

Read/grep: \`ls tt/rbw-l*\` (existing Lode colophons); Tools/rbk/rbgc_Constants.sh (grep RBGC_LODE_KIND — which kind-letters exist, which are missing); Tools/rbk/rbgl_GarLayout.sh (the rbi_ld layout, RBGL_LODES_ROOT); Tools/rbk/rbtd/src/rbtdgc_consts.rs (RBTDGC_* Lode entries) and Tools/rbk/rbtd/src/rbtdrm_manifest.rs (fixture registry); the "divine-legend" kind-letter to kind-name mapping (grep across rbldl_Lifecycle.sh / rbld0_cli.sh / rbld0_Lode.sh / rbgc_Constants.sh); Tools/rbk/rbz_zipper.sh (how rbld commands/colophons are enrolled).

For EACH of the 4 remaining kinds (tool, reliquary, wsl, podvm) list which registration artifacts are MISSING vs present: capture colophon (rbw-lF/lC/lU/lI), kind-letter constant, GAR-layout entry, theurge colophon-const + manifest entry, divine-legend entry, zipper enrollment.

recommendation_input: a crisp inventory of what the scaffold-residue pace must contain, and a judgment on whether it is genuinely ONE serial pace or wants splitting.`,
  },
  {
    key: 'vertical',
    label: 'vertical shape / granularity',
    prompt: `${SHARED}

QUESTION: How much does each remaining kind vertical (tool, reliquary, wsl, podvm) SHARE with the landed bole pilot, and where does it DIVERGE? This decides whether to slate one pace per kind or to group/split.

Read: Tools/rbk/rbldb_Bole.sh (the pilot body: recipe + substitutions + envelope + extract loop), Tools/rbk/rblds_Spine.sh (what the spine already provides for free), Tools/rbk/rbgjl/rbgjl01-ensconce-capture.sh and rbgjl02-assemble-push-vouch.sh (the cloud steps a new kind would parallel), Tools/rbk/vov_veiled/FUTURE/rbv_PodmanVM.sh and Tools/rbk/vov_veiled/RBSPV* if present (the podvm prototype: fan-out, the two quay families machine-os-wsl / machine-os, crane manifest/blob navigation, recorded trust grade), Tools/rbk/rbfli_Inscribe.sh and Tools/rbk/vov_veiled/RBSDI-depot_inscribe.adoc (the CURRENT reliquary tool-mirror = the conclave kind's existing logic to absorb).

Assess per kind: member cardinality (1 vs N); payload shape (native layered OCI vs opaque blob wrapped as OCI); upstream fetch tool (skopeo vs crane vs curl+published-checksum); trust grade (verified-against-published vs recorded-at-acquisition); and roughly how much NET-NEW body-file logic each needs vs pure spine reuse. Note that wsl fetches a rootfs tarball over HTTPS with a published SHA-256 (not an OCI registry pull) — flag how far that diverges from the skopeo-based bole/spine path.

recommendation_input: a weight/complexity estimate per kind (small/medium/large) and a recommendation on vertical granularity — one-per-kind, or group the easy ones, or split podvm (e.g. native vs wsl family, or capture vs platform-selection).`,
  },
  {
    key: 'cutover',
    label: 'cutover chokepoints / bole timing',
    prompt: `${SHARED}

QUESTION: Is the bole/\`enshrine\` cutover truly INDEPENDENT of the other kind verticals (so it can land right after scaffold), and what exactly does each cutover repoint?

Read: Tools/rbk/rbfca_StepAssembly.sh (the function zrbfc_resolve_tool_images — the reliquary consumption chokepoint), Tools/rbk/vov_veiled/RBSRV-RegimeVessel.adoc (the ORIGIN/ANCHOR pattern for a base coordinate), Tools/rbk/vov_veiled/RBSAE-ark_enshrine.adoc (the enshrine path to retire), Tools/rbk/rbfli_Inscribe.sh (the inscribe path to retire). Grep for where a conjure build's base FROM coordinate is resolved today (the "base ANCHOR derived-pull"), and grep for consumers of the GAR namespaces \`rbi_es\` (enshrine) and \`rbi_rq\` (reliquary). Identify the single election variable per path (RBRV_RELIQUARY for reliquary; the base ANCHOR for bole).

Determine: (1) bole-cutover dependencies — does retiring enshrine and repointing the base derived-pull to read the ensconce capture-file depend on ANY other vertical, or only on the already-landed ensconce? (2) reliquary-cutover dependencies — confirm it needs the conclave vertical to exist first. (3) the concrete repoint steps, the "verify one conjure build green" gate, and what GAR namespace gets banished LAST.

recommendation_input: recommend whether to slate the bole-cutover EARLY (immediately after scaffold) or to keep it PAIRED with the reliquary cutover as "siblings" — with the dependency reasoning, and any risk of doing bole-cutover before the other verticals exist.`,
  },
  {
    key: 'provdocs',
    label: 'provenance attachment + public docs',
    prompt: `${SHARED}

QUESTION 1 (provenance attachment): Lode provenance currently rides as a reserved GAR tag \`:rbi_vouch\` (one per Lode). The plan flags the OCI referrers API as a FUTURE upgrade "once GAR maturity is confirmed." Read Tools/rbk/rbgjl/rbgjl01-ensconce-capture.sh and rbgjl02-assemble-push-vouch.sh (how :rbi_vouch is authored and pushed), and any Tools/rbk/vov_veiled/RBSCJ* / RBSCB* notes. Assess whether verifying GAR referrers support is a real near-term pace, a research spike, or safely deferred.

QUESTION 2 (public docs): How tightly is the public-facing surface coupled to per-kind VERB names? Read README.md (its glossary and roadmap sections) and Tools/rbk/vov_veiled/RBSHR-HorizonRoadmap.adoc. Determine whether the public glossary is concept-level (Lode / touchmark only) or enumerates per-kind verbs (ensconce / fetter / conclave / ...) — this decides whether any greenfield vertical must also touch README (a file-disjointness hazard for parallel chats).

recommendation_input: (a) recommendation on the provenance-attachment topic — defer / research-spike pace / fold into a vertical; and (b) recommendation on public-docs paces — standalone vs ride-the-cutovers, and whether the glossary should stay concept-level to keep verticals from colliding on README.`,
  },
]

function verifyPrompt(spec, investigation) {
  return `You are an independent skeptic verifying another agent's investigation of the RBK "Lode" subsystem (fetched-side artifact capture into GAR), in the repo at the current working directory. Do NOT trust their summary — RE-READ the cited files yourself.

Their area: ${spec.key} (${spec.label})

Their findings (JSON):
${JSON.stringify(investigation.findings, null, 2)}

Their recommendation_input:
${investigation.recommendation_input}

Independently check the LOAD-BEARING factual claims against the actual files. For each, return whether it holds (with a note citing what you found). Then — most important — state in \`corrections\` anything they got WRONG, any citation that's off, or anything they MISSED that would change the recommendation. If the recommendation stands as-is, set corrections to an empty string.`
}

phase('Investigate')
const results = await pipeline(
  SPECS,
  (spec) => agent(spec.prompt, { label: `investigate:${spec.key}`, phase: 'Investigate', schema: INVESTIGATION_SCHEMA }),
  (investigation, spec) => {
    if (!investigation) return { area: spec.key, label: spec.label, investigation: null, verification: null }
    return agent(verifyPrompt(spec, investigation), { label: `verify:${spec.key}`, phase: 'Verify', schema: VERIFICATION_SCHEMA })
      .then((v) => ({ area: spec.key, label: spec.label, investigation, verification: v }))
  },
)

return results
