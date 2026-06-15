export const meta = {
  name: 'citizen-model-verify',
  description: 'Verify the ultracode review sweep landed: finding coverage, rename completeness, regressions, cross-doc consistency',
  phases: [
    { title: 'Verify', detail: 'coverage, rename-sweep, regression, cross-doc — independent ground-truth checks' },
    { title: 'Synthesize', detail: 'collate residuals into a pass/fix report' },
  ],
}

const MEMO = 'Memos/memo-20260605-citizen-capability-model.md'
const M527 = 'Memos/memo-20260527-operator-credential-models.md'
const RBSHR = 'Tools/rbk/vov_veiled/RBSHR-HorizonRoadmap.adoc'

const ISSUE_SCHEMA = {
  type: 'object',
  additionalProperties: false,
  properties: {
    issues: {
      type: 'array',
      items: {
        type: 'object',
        additionalProperties: false,
        properties: {
          kind: { type: 'string', enum: ['missing','partial','regression','inconsistency','stray-rename','ok-note'] },
          ref: { type: 'string', description: 'finding number or area' },
          location: { type: 'string', description: 'file:section' },
          detail: { type: 'string' },
          fix: { type: 'string' },
          severity: { type: 'string', enum: ['high','medium','low'] },
        },
        required: ['kind','ref','location','detail','fix','severity'],
      },
    },
  },
  required: ['issues'],
}

const ctx = `You are VERIFYING that an applied edit sweep landed correctly on heat ₣BZ's citizen-model
artifacts. Ground truth is the files ON DISK — READ them; do not trust any summary.
Files: ${MEMO} (the mechanics memo, just rewritten), ${RBSHR} (roadmap, 2 prose edits), ${M527}
(a breadcrumb added near its Status). The paddock is gallops-stored (not a flat file); judge paddock
expectations from the findings list's [paddock] items and the notes in args.paddock — if you cannot
read the paddock, say so and judge only the on-disk files.

The sweep was meant to apply these findings (id: expected-change [target]):
${args.findings.map(f => '- ' + f).join('\n')}

Paddock expectation notes:\n${args.paddock}

Report only genuine problems (missing/partial/regression/inconsistency/stray-rename). It is fine to
report zero issues. Use kind "ok-note" sparingly only to record a checked-and-clean high-risk item.`

phase('Verify')

const checks = [
  { key: 'coverage-A', focus: `COVERAGE (findings 1-11). For each, READ ${MEMO} (and ${RBSHR} for #4) and confirm the change is actually present and correct, not merely gestured at. Flag any finding not addressed or addressed wrongly.` },
  { key: 'coverage-B', focus: `COVERAGE (findings 12-22). For each, READ ${MEMO} and ${M527} (#22) and the paddock-expectation notes (#14,17,18,19,21 are paddock). Confirm each change is present and correct. Flag gaps.` },
  { key: 'rename-sweep', explore: true, focus: `RENAME COMPLETENESS. Grep the repo. (a) "declared roster" must no longer appear (should be "declared ledger"); flag any survivor. (b) Confirm the word "roster" still appears ONLY in legitimate senses — the actual-reading cult-verb, "actual-state roster", or quotes of memo-20260527 — and NOT as the intent store. (c) In ${RBSHR}, confirm "citizen" no longer denotes an artifact (the hoard and bullion lines should say "holding"); flag any remaining artifact-"citizen". (d) Confirm ${MEMO} uses "ledger" consistently for the intent store.` },
  { key: 'regression', focus: `REGRESSION HUNT. READ ${MEMO}. The sweep was large — find NEW problems it may have introduced: a sentence that still says "roster" where it now means the ledger, a broken/duplicated cross-reference, a claim that now contradicts another claim in the same memo (e.g. the divest ordering vs the verb table; the member-first axis vs the audit-domain line; the "audit never auto-revokes" vs divest revoking), or a dangling pointer to a renamed section. Be specific.` },
  { key: 'cross-doc', focus: `CROSS-DOC CONSISTENCY. READ ${MEMO}, ${RBSHR}, ${M527}. Check: (1) memo's "supersedes RBSHR prose citizen" note matches the actual RBSHR rewording; (2) the ${M527} breadcrumb exists, is near Status, names heat ₣BZ and the citizen memo, and honors the "record when frozen" deferral (it must NOT be the full supersede note); (3) the memo's claimed three divergences from ${M527} are still faithful to ${M527}'s actual text; (4) terminology (citizen/federate/holding/capability-set/declared ledger) is used consistently across the memo and these docs.` },
]

const results = await parallel(checks.map(c => () =>
  agent(`${ctx}\n\nYOUR CHECK: ${c.focus}`, {
    label: c.key,
    phase: 'Verify',
    schema: ISSUE_SCHEMA,
    agentType: c.explore ? 'Explore' : undefined,
  }).then(r => ({ check: c.key, issues: (r && r.issues) ? r.issues : [] })).catch(() => ({ check: c.key, issues: [] }))
))

const allIssues = results.flatMap(r => (r.issues || []).map(i => ({ ...i, check: r.check })))
const actionable = allIssues.filter(i => i.kind !== 'ok-note')
log(`Verification raised ${allIssues.length} notes; ${actionable.length} actionable`)

phase('Synthesize')
const SYNTH = {
  type: 'object',
  additionalProperties: false,
  properties: {
    clean: { type: 'boolean', description: 'true if no actionable residuals' },
    residuals: {
      type: 'array',
      items: {
        type: 'object',
        additionalProperties: false,
        properties: {
          severity: { type: 'string', enum: ['high','medium','low'] },
          location: { type: 'string' },
          issue: { type: 'string' },
          fix: { type: 'string' },
        },
        required: ['severity','location','issue','fix'],
      },
    },
    summary: { type: 'string' },
  },
  required: ['clean','residuals','summary'],
}

const report = await agent(`Synthesis: collate these verification findings into a residuals report for the operator. Deduplicate; drop false alarms and anything where the check itself was uncertain because it could not read the paddock; keep only real residuals with a concrete fix. Set clean=true only if there are no real actionable residuals. Findings JSON:\n${JSON.stringify(actionable)}`, {
  label: 'synthesize',
  phase: 'Synthesize',
  schema: SYNTH,
})

return { raised: allIssues.length, actionable: actionable.length, report }
