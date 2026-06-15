export const meta = {
  name: 'citizen-model-review',
  description: 'Exhaustive multi-lens, adversarially-verified review of the ₣BZ citizen-model paddock + mechanics memo; synthesize prioritized improvements',
  phases: [
    { title: 'Review', detail: 'nine independent lenses fan out over paddock + memo + sources' },
    { title: 'Verify', detail: 'adversarially verify each candidate finding against the real sources' },
    { title: 'Synthesize', detail: 'dedup, rank, and produce prioritized recommendations' },
  ],
}

const MEMO = 'Memos/memo-20260605-citizen-capability-model.md'
const SRC = {
  m527: 'Memos/memo-20260527-operator-credential-models.md',
  m604: 'Memos/memo-20260604-credential-churn-leak-and-propagation-races.md',
  mgov: 'Memos/memo-20260605-governor-mantle-tombstone-leak.md',
  rbshr: 'Tools/rbk/vov_veiled/RBSHR-HorizonRoadmap.adoc',
  invest: 'Tools/rbk/vov_veiled/RBSDK-director_invest.adoc',
  divest: 'Tools/rbk/vov_veiled/RBSDD-director_divest.adoc',
  rinvest: 'Tools/rbk/vov_veiled/RBSRK-retriever_invest.adoc',
  roster: 'Tools/rbk/vov_veiled/RBSDR-director_roster.adoc',
  mantle: 'Tools/rbk/vov_veiled/RBSGM-governor_mantle.adoc',
  cred: 'Tools/rbk/vov_veiled/RBSRA-CredentialFormat.adoc',
}

const base = `You are critically reviewing the design capture for recipe-bottle (RBK) heat ₣BZ
("rbk-14-citizen-model"). Two artifacts: a PADDOCK (shape only) and a MECHANICS MEMO (detail).

The PADDOCK (current committed text) is reproduced verbatim here:
----- PADDOCK BEGIN -----
${args.paddock}
----- PADDOCK END -----

The MECHANICS MEMO is the file ${MEMO} — READ IT.

Context you must hold:
- "Citizen" is a freshly-minted term for the KEYFILE-tier operator identity (a depot-issued
  service account holding one RBRA key). The future federation tier's identity is a "federate."
  The payor is the founder, outside both tiers.
- The model decouples identity lifecycle from capability lifecycle; capabilities are named
  "capability-sets" (IAM grant bundles in code); intent is stored in a "declared roster";
  an audit diffs declared-vs-IAM.
- The paddock holds SHAPE; the memo holds MECHANISM. Scoping/heat-chopping is INTENTIONALLY
  deferred — do NOT file "decide the scope" as a finding; that is a known open.
- A prior review pass already corrected ~11 issues, and the text above already reflects them.
  Do NOT re-report things already fixed in the current text. Focus on improvements NOT yet
  present. If you believe an already-present claim is wrong, say so explicitly.

Source documents you may cross-check (READ the ones relevant to your lens):
- ${SRC.m527} (committed two-tier credential plan; the model claims to supersede it)
- ${SRC.m604} (credential lifecycle split — the enabler)
- ${SRC.mgov} (standalone governor-mantle tombstone-leak fix)
- ${SRC.rbshr} ("Operator federation" + envoy/embassy roadmap entries)
- cult-verb specs under Tools/rbk/vov_veiled/: ${SRC.invest}, ${SRC.divest}, ${SRC.rinvest}, ${SRC.roster}, ${SRC.mantle}, ${SRC.cred}

Rules: Be a genuine, skeptical critic — only report REAL, substantiated issues, each with a
concrete fix. Cite evidence precisely (a quote, a spec name, a file:area, or a URL). It is fine
to find few issues. Do NOT pad. Prefer high-value findings. Your final output is structured data,
not a human message.`

const FINDINGS_SCHEMA = {
  type: 'object',
  additionalProperties: false,
  properties: {
    findings: {
      type: 'array',
      items: {
        type: 'object',
        additionalProperties: false,
        properties: {
          title: { type: 'string', description: 'short label' },
          location: { type: 'string', description: 'paddock §section / memo §section / file:area' },
          type: { type: 'string', enum: ['conflict','error','security','clarity','concise','completeness','posture'] },
          severity: { type: 'string', enum: ['high','medium','low'] },
          issue: { type: 'string', description: 'the problem, precisely' },
          evidence: { type: 'string', description: 'citation backing the issue' },
          suggested_fix: { type: 'string', description: 'concrete change' },
          confidence: { type: 'string', enum: ['high','medium','low'] },
        },
        required: ['title','location','type','severity','issue','evidence','suggested_fix','confidence'],
      },
    },
  },
  required: ['findings'],
}

const VERDICT_SCHEMA = {
  type: 'object',
  additionalProperties: false,
  properties: {
    verdict: { type: 'string', enum: ['real','partial','false'] },
    reasoning: { type: 'string', description: 'why, after independently checking the evidence' },
    refined_fix: { type: 'string', description: 'the corrected/sharpened fix, or why none is needed' },
    severity: { type: 'string', enum: ['high','medium','low'] },
  },
  required: ['verdict','reasoning','refined_fix','severity'],
}

const SYNTH_SCHEMA = {
  type: 'object',
  additionalProperties: false,
  properties: {
    recommendations: {
      type: 'array',
      items: {
        type: 'object',
        additionalProperties: false,
        properties: {
          theme: { type: 'string' },
          severity: { type: 'string', enum: ['high','medium','low'] },
          disposition: { type: 'string', enum: ['apply-now','operator-decision'] },
          issue: { type: 'string' },
          fix: { type: 'string' },
          target: { type: 'string', description: 'paddock / memo / both / a source doc' },
        },
        required: ['theme','severity','disposition','issue','fix','target'],
      },
    },
    overall: { type: 'string', description: 'one-paragraph assessment' },
    ordered_actions: { type: 'array', items: { type: 'string' } },
  },
  required: ['recommendations','overall','ordered_actions'],
}

const LENSES = [
  { key: 'internal-coherence', explore: false, focus: `LENS: internal coherence. Hunt contradictions WITHIN the paddock, WITHIN the memo, and BETWEEN them. Also audit whether the recently-applied amendments are themselves correct and self-consistent (e.g. the roster-write invariant, the repoAdmin/setIamPolicy scoping, the surplus qualifier, the "byte-identical modulo principal handle" phrasing, the ontology-scoped-to-operators line). Flag any place a cinched decision depends on an Open item, or two statements that cannot both be true.` },
  { key: 'source-conflict', explore: true, focus: `LENS: conflict with source docs. Read ${SRC.m527}, ${SRC.m604}, ${SRC.mgov}, ${SRC.rbshr}. Verify the model's claims about those docs are faithful (especially the three claimed divergences from memo-20260527). Find any silent contradiction with a committed source, any superseded claim not flagged, and any source that should be updated/cross-referenced but isn't.` },
  { key: 'spec-behavior', explore: true, focus: `LENS: current-behavior fidelity. Read ${SRC.invest}, ${SRC.divest}, ${SRC.rinvest}, ${SRC.roster}, ${SRC.mantle}, ${SRC.cred}. Verify every claim the model makes about how things work TODAY (invest fuses identity+grant; divest already revokes-before-delete; mantle deletes-without-revoke and uses datestamped names; roster filters by name regex; RBRA holds client_email/private_key/project_id). Flag mischaracterizations and name the specific spec surfaces this heat will have to edit.` },
  { key: 'bash-reality', explore: true, focus: `LENS: implementation reality. Grep/read the RBK bash under Tools/rbk/ (e.g. rbgg_Governor.sh, rbgp_Payor.sh, rba_Auth.sh, rbgv_AccessProbe.sh, rbdc_DerivedConstants.sh, rbcc_Constants.sh, rba_cli.sh). Verify the design's implicit assumptions hold in code: how the token accessor is keyed today, where RBRA file paths are consumed (the "~30 sites" claim), how SA names are composed (director-<identity>), how mantle/invest/divest are structured. Flag any design claim the code contradicts, and any migration hazard the design omits (e.g. existing callers, constants, derived paths).` },
  { key: 'gcp-facts', explore: false, focus: `LENS: GCP factual accuracy. WEB-VERIFY every platform claim the model rests on, using current (2026) Google Cloud docs. Specifically: (a) service accounts support TAGS but NOT resource labels, and SA tags are in Preview; (b) iam.googleapis.com/modifiedGrantsByRole limits grantable roles only on project/folder/org policies, not resource-level, max 10 roles; (c) roles/artifactregistry.repoAdmin includes artifactregistry.repositories.setIamPolicy; (d) a service account CAN exist with zero IAM bindings; (e) federated principals appear in IAM as principal:// member strings; (f) billing is a separate resource (billing account) from project IAM, so roles/owner excludes billing. Report any claim that is wrong, stale, or imprecise, with the authoritative URL. Also surface any relevant platform fact the model SHOULD account for but doesn't (e.g. SA key/tag quotas, propagation, condition CEL limits).` },
  { key: 'security-threat', explore: false, focus: `LENS: adversarial security. Try to BREAK the model. Enumerate escalation paths, blast-radius edges, and audit-evasion. Pressure-test: the roster-write-≥-grant invariant (is it sufficient? what writes the roster, with what auth?); the auto-converge-deficit behavior (can an attacker who can write the roster, or cause a "deficit", get an auto-grant?); the orphan sentinel/tags (can an attacker forge or strip it?); surplus reporting (can malicious activity masquerade as a benign definition-shrink surplus?); the governor topology per tier; multi-role union key compromise; the payor as third backing. Report concrete attack scenarios the design does not yet close, each with a mitigation.` },
  { key: 'clarity-minting', explore: false, focus: `LENS: vocabulary, minting, and load-bearing complexity (RBK MCM/ROE sensibilities). Check the minted terms (citizen, federate, holdings, capability-set, declared roster) for monosemy and collisions across the RBK namespace, prefix discipline, and whether each distinction is load-bearing. Flag ambiguous or overloaded terms, any term that needs a code prefix mint not yet noted, and any sentence that is unclear on its own. Also: is "federate" the right peer noun; does "holdings" collide with existing RBSHR usage (it currently calls arks/bullions depot "citizens")?` },
  { key: 'conciseness-shape', explore: false, focus: `LENS: conciseness + shape/mechanism discipline (JJK paddock posture). The paddock must be PURE SHAPE; the memo holds mechanism. Flag any remaining mechanism leaking into the paddock, any duplication between paddock and memo, any paddock line that is journal/progress rather than shape, and any bloat in either doc. Be specific about what to cut or move.` },
  { key: 'completeness-critic', explore: false, focus: `LENS: what is MISSING. Read the memo. Hunt for gaps: an identity not covered (mason/envoy/payor edge cases under the new verbs), a verb-dissolution row that is wrong or missing, a failure mode unhandled (concurrent grant/audit, partial multi-scope revoke, roster/IAM split-brain, propagation races interacting with auto-converge), a federation plug-in seam the model claims will "just work" but hasn't been checked, an interaction with rbk-08 (idempotent invest, Class-C tolerance) or the dual-pool/airgap posture, or any spec/handbook/test surface in "what done looks like" that is under-specified. Propose the single most valuable missing piece.` },
]

phase('Review')
log(`Fanning out ${LENSES.length} review lenses over the citizen-model artifacts`)

const reviewed = await pipeline(
  LENSES,
  (lens) => agent(`${base}\n\n${lens.focus}`, {
    label: `find:${lens.key}`,
    phase: 'Review',
    schema: FINDINGS_SCHEMA,
    agentType: lens.explore ? 'Explore' : undefined,
  }),
  (res, lens) => {
    const findings = (res && res.findings) ? res.findings : []
    if (!findings.length) return []
    return parallel(findings.map((f, i) => () =>
      agent(`${base}\n\nADVERSARIALLY VERIFY this candidate finding. Independently check its evidence — READ the cited file/spec, or WEB-SEARCH the cited fact; do NOT trust the finding's own evidence text. Default to verdict "false" if you cannot substantiate it. If it is real but the stated fix is wrong or imprecise, mark "partial" and give a corrected refined_fix.\n\nFINDING (JSON):\n${JSON.stringify(f)}`, {
        label: `verify:${lens.key}.${i}`,
        phase: 'Verify',
        schema: VERDICT_SCHEMA,
      }).then(v => ({ lens: lens.key, finding: f, verdict: v })).catch(() => null)
    ))
  },
)

const all = reviewed.flat().filter(Boolean)
const confirmed = all.filter(x => x.verdict && x.verdict.verdict !== 'false')
log(`Candidate findings: ${all.length}; survived adversarial verify: ${confirmed.length}`)

phase('Synthesize')
const report = await agent(`${base}\n\nYou are the SYNTHESIS step. Below are findings that survived adversarial verification (verdict real or partial), each with the verifier's reasoning and refined_fix. Deduplicate overlapping findings, rank by severity and value, and produce a prioritized recommendation set for the operator (Brad).\n\nFor each recommendation: theme, severity, disposition ("apply-now" for safe, unambiguous improvements vs "operator-decision" for scope/judgment calls), the issue (1-2 sentences), a concrete fix, and the target doc. Then give a one-paragraph overall assessment and an ordered action list (most valuable first). Prefer the verifier's refined_fix over the original where they differ. Do not include findings the current text already satisfies.\n\nVERIFIED FINDINGS (JSON):\n${JSON.stringify(confirmed.map(x => ({ lens: x.lens, finding: x.finding, verdict: x.verdict })))}`, {
  label: 'synthesize',
  phase: 'Synthesize',
  schema: SYNTH_SCHEMA,
})

return { candidate_count: all.length, confirmed_count: confirmed.length, report }
