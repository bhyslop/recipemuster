export const meta = {
  name: 'citizen-model-verify',
  description: 'Verify the ultracode review sweep landed on the on-disk artifacts: finding coverage, rename completeness, regressions, cross-doc consistency',
  phases: [
    { title: 'Verify', detail: 'coverage, rename-sweep, regression, cross-doc — independent ground-truth checks' },
    { title: 'Synthesize', detail: 'collate residuals into a pass/fix report' },
  ],
}

const MEMO = 'Memos/memo-20260605-citizen-capability-model.md'
const M527 = 'Memos/memo-20260527-operator-credential-models.md'
const RBSHR = 'Tools/rbk/vov_veiled/RBSHR-HorizonRoadmap.adoc'

const FINDINGS = [
  "1: SA-naming migration section — identity-only SA names, enumerate RBSDK/RBSRK/RBSDD/RBSDR/RBSRA surfaces, delete+recreate cutover via rbk-08 revoke layer+Class-C [memo]",
  "2: member-first audit axis (enumerate actual members on repo/bucket/SA policies, flag non-ledger/non-system); correct audit-domain line; soften repoAdmin 'covers' claim [memo]",
  "3: divest/remove = withdraw ledger intent FIRST then revoke IAM then delete SA; 'no IAM revoke without prior ledger withdrawal' invariant [memo]",
  "4: citizen vs RBSHR — RBSHR artifact-'citizen' prose reworded to 'holding'; memo note recording the supersede [RBSHR+memo]",
  "5: rename 'declared roster' -> 'declared ledger' throughout, disambiguated from the actual-reading roster verb; audit = diff(actual-state roster, declared ledger) [memo]",
  "6: capability-set definition EXPANSION gated behind human adjudication (vs per-identity failed-grant auto-heal); 'who edits definitions' is first-class authority [memo]",
  "7: 10-keys-per-SA quota; rekey order create-new->deliver->verify->delete-old; reclaim stale keys [memo]",
  "8: concurrency — auto-converge re-reads ledger under etag before each grant; teardown writes ledger first [memo]",
  "9: half-failed REVOKE handled (idempotent retry / revoking-marker, not adversarial surplus); fix 'no verb auto-revokes' -> 'the audit never auto-revokes' [memo]",
  "10: orphan sweep is integrity-advisory; authoritative signal = USER_MANAGED key on an SA absent from the ledger [memo]",
  "11: ledger home is NOT the build bucket (director holds objectCreator); write-ACL governor/payor only; cinch 'ledger-write never a capability-set member' [memo]",
  "12: Writers rule topology-conditional (writer == grant-authority holder under active topology) [memo]",
  "13: 'mantle is not special' qualified to once-governor-is-idempotent [memo]",
  "14: migration described as one-time name-based bootstrap, distinct from retired 'derive from IAM' [memo migration section]",
  "15: migration stamps orphan marker onto grandfathered legacy SAs [memo]",
  "16: ephemeral-governor caveat — revoke strips binding not on-disk key; couple key-destruction [memo]",
  "20: federate vs Workload 'federates' seam pointer [memo]",
  "22: deferral-honoring breadcrumb added to memo-20260527 [memo-20260527]",
]

const ctx = "You are VERIFYING that an applied edit sweep landed correctly on heat BZ's citizen-model artifacts. Ground truth is the files ON DISK — READ them; do not trust any summary. Files: " + MEMO + " (the mechanics memo, just rewritten), " + RBSHR + " (roadmap, 2 prose edits), " + M527 + " (a breadcrumb added near its Status). The paddock is gallops-stored and is being verified separately by the main agent — do NOT flag paddock-only concerns. The sweep was meant to apply these findings (id: expected-change [target]):\n" + FINDINGS.map(function(f){return '- ' + f}).join('\n') + "\n\nReport only genuine problems (missing/partial/regression/inconsistency/stray-rename) on the ON-DISK files. It is fine to report zero issues. Use kind 'ok-note' sparingly only to record a checked-and-clean high-risk item."

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
          ref: { type: 'string' },
          location: { type: 'string' },
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

phase('Verify')

const checks = [
  { key: 'coverage-A', explore: false, focus: "COVERAGE (findings 1-11). READ " + MEMO + " (and " + RBSHR + " for #4) and confirm each change is actually present and correct, not merely gestured at. Flag any finding not addressed or addressed wrongly." },
  { key: 'coverage-B', explore: false, focus: "COVERAGE (findings 12-16, 20, 22). READ " + MEMO + " and " + M527 + " (#22). Confirm each change is present and correct. Flag gaps." },
  { key: 'rename-sweep', explore: true, focus: "RENAME COMPLETENESS via grep over the repo. (a) 'declared roster' must no longer appear in the citizen memo or paddock-adjacent docs (should be 'declared ledger'); flag survivors. (b) In " + MEMO + ", confirm 'roster' now appears ONLY as the actual-reading cult-verb, 'actual-state roster', or a quote of memo-20260527 — never as the intent store. (c) In " + RBSHR + ", confirm 'citizen' no longer denotes an artifact (hoard and bullion lines should say 'holding'); flag any remaining artifact-'citizen'. (d) Confirm " + MEMO + " uses 'ledger' consistently for the intent store." },
  { key: 'regression', explore: false, focus: "REGRESSION HUNT. READ " + MEMO + ". The sweep was large — find NEW problems it may have introduced: a sentence still saying 'roster' where it now means the ledger, a broken/duplicated cross-reference, a claim that now contradicts another in the same memo (e.g. divest ordering vs the verb table; member-first axis vs the audit-domain line; 'the audit never auto-revokes' vs divest legitimately revoking; the 'no IAM revoke without prior ledger withdrawal' invariant vs any other ordering statement), or a dangling pointer to a renamed section." },
  { key: 'cross-doc', explore: false, focus: "CROSS-DOC CONSISTENCY. READ " + MEMO + ", " + RBSHR + ", " + M527 + ". Check: (1) the memo's 'supersedes RBSHR prose citizen' note matches the actual RBSHR rewording; (2) the " + M527 + " breadcrumb exists, is near Status, names heat BZ and the citizen memo, and honors the 'record when frozen' deferral (must NOT be the full supersede note); (3) the memo's three claimed divergences from " + M527 + " are still faithful to " + M527 + "'s actual text; (4) terminology (citizen/federate/holding/capability-set/declared ledger) is consistent across these docs." },
]

const results = await parallel(checks.map(function(c){ return function(){
  return agent(ctx + "\n\nYOUR CHECK: " + c.focus, {
    label: c.key,
    phase: 'Verify',
    schema: ISSUE_SCHEMA,
    agentType: c.explore ? 'Explore' : undefined,
  }).then(function(r){ return { check: c.key, issues: (r && r.issues) ? r.issues : [] } }).catch(function(){ return { check: c.key, issues: [] } })
}}))

const allIssues = results.flatMap(function(r){ return (r.issues || []).map(function(i){ return Object.assign({}, i, { check: r.check }) }) })
const actionable = allIssues.filter(function(i){ return i.kind !== 'ok-note' })
log("Verification raised " + allIssues.length + " notes; " + actionable.length + " actionable")

phase('Synthesize')
const SYNTH = {
  type: 'object',
  additionalProperties: false,
  properties: {
    clean: { type: 'boolean' },
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

const report = await agent("Synthesis: collate these verification findings into a residuals report for the operator. Deduplicate; drop false alarms; keep only real residuals with a concrete fix. Set clean=true only if there are no real actionable residuals. Findings JSON:\n" + JSON.stringify(actionable), {
  label: 'synthesize',
  phase: 'Synthesize',
  schema: SYNTH,
})

return { raised: allIssues.length, actionable: actionable.length, report }
