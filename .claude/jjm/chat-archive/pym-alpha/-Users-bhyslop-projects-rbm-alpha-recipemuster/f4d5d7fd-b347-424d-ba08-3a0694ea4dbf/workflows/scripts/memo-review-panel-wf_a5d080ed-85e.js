export const meta = {
  name: 'memo-review-panel',
  description: 'Three independent reviewers vet the ultracode-process memo for accuracy, clarity, completeness',
  phases: [{ title: 'Review', detail: 'accuracy / clarity / completeness reviewers in parallel' }],
}

const REVIEW_SCHEMA = {
  type: 'object',
  properties: {
    reviewer: { type: 'string' },
    issues: {
      type: 'array',
      items: {
        type: 'object',
        properties: {
          severity: { type: 'string', enum: ['high', 'medium', 'low'] },
          where: { type: 'string', description: 'quoted phrase or section of the memo' },
          problem: { type: 'string' },
          fix: { type: 'string', description: 'concrete suggested change' },
        },
        required: ['severity', 'where', 'problem', 'fix'],
      },
    },
    verdict: { type: 'string', description: 'overall: is the memo accurate/clear/complete enough to ship, and the single most important change' },
  },
  required: ['reviewer', 'issues', 'verdict'],
}

const MEMO = '/Users/bhyslop/projects/rbm_alpha_recipemuster/Memos/memo-20260605-ultracode-replan-process.md'
const SCRIPT = '/Users/bhyslop/.claude/projects/-Users-bhyslop-projects-rbm-alpha-recipemuster/f4d5d7fd-b347-424d-ba08-3a0694ea4dbf/workflows/scripts/lode-replan-recommendations-wf_38301505-1ba.js'
const RESULT = '/private/tmp/claude-501/-Users-bhyslop-projects-rbm-alpha-recipemuster/4be0f726-e380-456c-98d6-46d8a2ee8da1/tasks/wl3aagbey.output'

const REVIEWERS = [
  {
    key: 'accuracy',
    prompt: `You are fact-checking a memo that describes a multi-agent "ultracode" workflow run. Read the memo at ${MEMO}. Then read the ACTUAL workflow script at ${SCRIPT} and the workflow's result file at ${RESULT} (large JSON — grep/scan it). Check EVERY factual claim the memo makes about the process against those two artifacts: the number of agents (investigators + verifiers), the pipeline-not-barrier structure, the structured-output schema (claims/evidence/confidence/recommendation_input), what the verify stage caught (the "byte-equivalent coordinate" overstatement, the over-assigned trust grades, citation slips, and the per-kind-verb false-positive), and the cost figures (~10 agents, ~937K tokens, ~200 tool uses, ~8 min). Flag anything the memo states that the artifacts do not support, or any number that is wrong. Be precise: cite what the artifact actually says.`,
  },
  {
    key: 'clarity',
    prompt: `You are reviewing a memo for an experienced engineer who asked specifically for PLAIN, SIMPLE language and to be EDUCATED about a process. Read the memo at ${MEMO}. Judge it purely as teaching: Is each idea explained before it is used? Is there unexplained jargon? Does it explain the WHY behind each design choice (fan-out, structured output, adversarial verify, pipeline-not-barrier, human-kept judgment), not just the what? Is anything too abstract to picture? Is the length right, or does it sag? Flag concrete spots where a smart but busy reader would stumble, and give a specific rewrite for each. Do NOT check facts (another reviewer owns that); focus only on clarity and teaching value.`,
  },
  {
    key: 'completeness',
    prompt: `You are an adversarial completeness-and-honesty critic for a memo describing a multi-agent "ultracode" workflow run. Read the memo at ${MEMO}; you may consult the script at ${SCRIPT} and results at ${RESULT} for context. Two jobs: (1) What important aspect of the process is MISSING — something a reader would need to truly understand or reproduce it (e.g. how producer/consumer steps were sequenced, why structured schemas allow model retries, the role of confidence levels, the limits of the approach, how synthesis actually worked)? (2) Where does the memo OVER-SELL ultracode — claim more value than the run delivered, or hide a weakness? Be skeptical: a memo that only praises the process is less useful than one that names its failure modes honestly. Flag both gaps and over-claims with concrete fixes.`,
  },
]

phase('Review')
const reviews = await parallel(
  REVIEWERS.map((r) => () =>
    agent(r.prompt, { label: `review:${r.key}`, phase: 'Review', schema: REVIEW_SCHEMA }),
  ),
)

return reviews.filter(Boolean)
