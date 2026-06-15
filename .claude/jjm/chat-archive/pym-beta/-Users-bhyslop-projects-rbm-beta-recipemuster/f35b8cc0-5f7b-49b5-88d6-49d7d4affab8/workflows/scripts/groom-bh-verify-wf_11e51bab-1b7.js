export const meta = {
  name: 'groom-bh-verify',
  description: 'Verify ₣BH lode-capture plan against repo: crane/gcrane, skopeo, test coverage, pace readiness',
  phases: [
    { title: 'Investigate', detail: 'parallel inventories: skopeo, crane/gcrane, docker/buildx, tests, pace-T state' },
    { title: 'Synthesize', detail: 'cross-check findings against dockets, answer the 4 questions' },
  ],
}

const ROOT = '/Users/bhyslop/projects/rbm_beta_recipemuster'

// The remaining paces of ₣BH and what each claims responsibility for.
const PACE_MAP = `
Remaining paces of heat ₣BH (rbk-11-mvp-lode-universal-capture), in heat order.
Each tool-eviction pace "claims" specific sites — your job is to map real code sites to these paces and flag any site NO pace claims.

- T (₢BHAAT lode-skopeo-eviction): skopeo->gcrane in the BOLE capture path only. Sites: rbgjl01-ensconce-capture.sh, the shared fingerprint snippet (rbgjs-skopeo-fingerprint.sh -> rbgjs-gcrane-fingerprint.sh rename), the bole step builder swapped to gcr.io/go-containerregistry/gcrane:debug, token-fetch include dropped. Sets the gcrane ambient-auth pattern inherited downstream. (Has 1 commit already landed — may be partial.)
- S (₢BHAAS theurge-fixture-fact-chain-fix): make the airgap-chain fixture actually exercise bole derived-pull election. Sites: zrbfd_elect_base_anchor (in rbfd_FoundryDirectorBuild.sh), rbtdri_invoke_impl burv_output isolation (theurge crate). Not a tool eviction.
- U (₢BHAAU lode-docker-eviction): docker pull/tag/push -> crane in the CONCLAVE capture step. Sites: rbgjl conclave capture step (rbgjl03-conclave-capture.sh).
- V (₢BHAAV lode-buildx-eviction): buildx -> crane append for two FROM-scratch builds: the shared VOUCH push step (used by every kind) and the UNDERPIN opaque-blob wrap. Sites: rbgjl02-assemble-push-vouch.sh / rbgjs vouch snippet, rbgjl04-underpin-capture.sh / rbgjs. curl+gpg stay.
- W (₢BHAAW lode-podvm-immure): NEW podvm immure vertical (verb rbw-lI), new rbldv_ body, kind registration, podvm-lifecycle fixture, RBSL podvm subdoc, RBS0 quoin, promote RBSPV out of FUTURE/. Greenfield.
- L (₢BHAAL lode-podvm-platform-fanout): extend immure to both quay families + curated multi-platform selection + refresh mode. Greenfield follow-on.
- M (₢BHAAM lode-reliquary-inscribe-cutover): retire inscribe reliquary-mirror path, repoint RBRV_RELIQUARY onto a conclave Lode via zrbfc_resolve_tool_images (in rbfca_StepAssembly.sh), banish rbi_rq.
- N (₢BHAAN lode-augur-inspect-split): split augur (rbw-la) out of divine, implement rbi_vouch envelope decode.
- O (₢BHAAO lode-public-docs-concept): README Enshrine/Reliquary -> Lode narrative conversion + RBSHR refresh.
- P (₢BHAAP lode-housekeeping-deferrals): mark rbhw* Windows onboarding tracks deferred; resolve ₣A- DEV-CACHE revert pace.
- R (₢BHAAR onboarding-clean-tree-gate): teach the "tools never commit, gate on clean tree" convention in onboarding tracks. Scope is contingent on (a) where pace S placed the gate and (b) whether bole election became a separate operator step.
- X (₢BHAAX lode-skopeo-reliquary-eviction): TERMINAL skopeo removal. Convert the made-side bind mirror (rbgjm01-mirror-image.sh / rbfd_mirror / zrbfd_mirror_submit) from 'skopeo copy --all' to 'crane cp'. Then drop skopeo from the cohort: conclave MANIFEST tool entry, z_rbfc_tool_skopeo plumbing (rbfc), RBGC constant, reliquary preflight check (rbgjr01-reliquary-preflight.sh), help-string mentions.
- D (₢BHAAD lode-vocab-finalization-scrub): terminal vocab + tool cross-check. Asserts zero skopeo anywhere, zero docker pull/tag/push or buildx invocations in capture+mirror steps (docker cohort tool-REF in MANIFEST data is allowed/expected). Runs LAST.
`

const SITE_SCHEMA = {
  type: 'object',
  additionalProperties: false,
  required: ['sites', 'orphan_sites', 'capability_findings', 'summary'],
  properties: {
    sites: {
      type: 'array',
      items: {
        type: 'object',
        additionalProperties: false,
        required: ['file', 'context', 'classification', 'claimed_by_pace'],
        properties: {
          file: { type: 'string', description: 'path:line or path' },
          context: { type: 'string', description: 'the actual code/text at the site, quoted briefly' },
          classification: { type: 'string', description: 'e.g. capture-invocation, made-side-invocation, cohort-tool-ref, plumbing/constant, fingerprint-snippet, help/prose, spec, test' },
          claimed_by_pace: { type: 'string', description: 'pace letter (T/U/V/W/X/D/...) or NONE if no pace claims it' },
          notes: { type: 'string' },
        },
      },
    },
    orphan_sites: { type: 'array', items: { type: 'string' }, description: 'sites that NO remaining pace claims — these are gaps in the plan' },
    capability_findings: { type: 'array', items: { type: 'string' }, description: 'tool-capability facts (e.g. does gcrane support append/cp/manifest/tag), terminology consistency observations' },
    summary: { type: 'string' },
  },
}

phase('Investigate')

const [skopeo, crane, docker, tests, stateT] = await parallel([
  () => agent(`You are auditing the repo at ${ROOT} for the SKOPEO inventory of heat ₣BH's tool-eviction plan.

${PACE_MAP}

TASK: Find EVERY occurrence of "skopeo" anywhere under Tools/ and tt/ (use: rg -n skopeo Tools/ tt/). For each occurrence:
1. Read enough surrounding context to classify it: capture-path invocation / made-side mirror invocation / cohort plumbing (z_rbfc_tool_skopeo, RBGC constant, conclave MANIFEST entry, reliquary preflight) / fingerprint snippet / help-string / spec prose / test.
2. Assign which remaining pace (T/X/D/...) is responsible for removing or converting it. Use NONE if no pace covers it.
3. CRITICAL: flag any skopeo site that no pace claims (orphan_sites) — that is a hole in the "skopeo fully removed" goal.

Also check: does rbgjs-skopeo-fingerprint.sh still exist, and does rbgjs-gcrane-fingerprint.sh also exist (pace T created it)? Is the bole capture path (rbgjl01-ensconce-capture.sh) still invoking skopeo or already on gcrane? This tells us if pace T's committed work is partial.

Return structured per the schema. Quote actual lines. Be exhaustive — this answers "is skopeo fully removed by the plan?"`, { label: 'skopeo-inventory', schema: SITE_SCHEMA }),

  () => agent(`You are auditing the repo at ${ROOT} for the CRANE vs GCRANE consistency of heat ₣BH's "crane embrace" plan.

${PACE_MAP}

KEY DESIGN FACT: the plan says "crane embrace" but the actual binary that auths GAR ambiently is GCRANE (crane's Google-auth sibling, image gcr.io/go-containerregistry/gcrane:debug). Plain crane has NO metadata-server auth and will FAIL against *.pkg.dev. So every registry step that hits GAR must use gcrane, not plain crane.

TASK:
1. Find every "crane" and "gcrane" occurrence under Tools/ and tt/ (rg -n -w crane; rg -n gcrane). Read context.
2. Where is gcrane:debug specified as a builder? Where is plain "crane" referenced (in code, snippets, specs, or docket-adjacent prose)?
3. Read rbgjs-gcrane-fingerprint.sh (the snippet pace T created) — what subcommands does it use? Read rblds_Spine.sh and rbgjl* steps for builder image references.
4. CAPABILITY CHECK (use your knowledge + any evidence in the repo/memos, esp. memo-20260608-lode-podvm-cerebro-experiment.md): does gcrane provide the subcommands the downstream paces rely on — cp (incl. multi-platform / --all equivalent for pace X replacing 'skopeo copy --all'), manifest, tag, digest, and especially APPEND (pace V's 'crane append' for FROM-scratch wraps)? gcrane is a superset of crane — confirm append/cp/manifest/tag are all present in gcrane. Flag any subcommand a pace relies on that gcrane might NOT have.
5. CONSISTENCY: the downstream eviction dockets (U/V/W/X) say "crane" / "crane builder" / "crane append" / "crane cp" and rely on "reuse the auth mechanism from the skopeo-eviction pace." Is that loose "crane" wording a real risk that a mount agent uses plain crane (no auth)? Or is gcrane:debug clearly established as the inherited builder?

Put capability + terminology observations in capability_findings. This answers "is crane fully replaced with gcrane everywhere?"`, { label: 'crane-gcrane', schema: SITE_SCHEMA }),

  () => agent(`You are auditing the repo at ${ROOT} for the DOCKER and BUILDX inventory of heat ₣BH's capture-tool plan.

${PACE_MAP}

TASK: Find every "docker" (especially pull/tag/push/inspect) and "buildx" occurrence under Tools/rbk/rbgjl, Tools/rbk/rbgjs, Tools/rbk/rbgjm, and tt/ (rg -nE 'docker (pull|tag|push|inspect)|docker build|buildx' Tools/rbk tt/; also rg -n 'cloud-builders/docker' Tools/rbk).

For each:
1. Read context. Classify INVOCATION (an actual docker/buildx command run in a build step) vs COHORT TOOL-REF (docker named as a captured tool in MANIFEST data the made-side build consumes — this is DATA, allowed to persist).
2. Map docker INVOCATIONS -> pace U (conclave). Map buildx INVOCATIONS -> pace V (shared vouch push step + underpin opaque-blob wrap). Flag any docker/buildx invocation claimed by NO pace (orphan).
3. Confirm pace V's two targets actually exist and use buildx today: the shared vouch push step (rbgjl02-assemble-push-vouch.sh or a rbgjs vouch snippet) and the underpin wrap (rbgjl04-underpin-capture.sh). Confirm pace U's target: the conclave capture step (rbgjl03-conclave-capture.sh) uses docker.
4. Note the gcrane:debug builder image is itself pulled by Cloud Build's docker substrate — do NOT count the builder-image reference as a docker invocation to evict.

This answers whether every docker/buildx invocation is claimed by a pace.`, { label: 'docker-buildx', schema: SITE_SCHEMA }),

  () => agent(`You are auditing TEST COVERAGE for heat ₣BH's transformation paces, repo at ${ROOT}.

${PACE_MAP}

Each transformation pace names a verify-gate fixture in its "Done when". Your job: determine whether each named fixture ACTUALLY EXISTS today, or must be authored.

TASK:
1. Read the theurge fixture registry: rbtdrc_crucible.rs and rbtdgc_consts.rs (RBTDRC_FIXTURES) in Tools/rbk/rbtd/. Enumerate all registered fixtures and which test-suite tier each is in (fast/service/crucible/complete).
2. For each pace's verify gate, state whether the fixture exists:
   - T (skopeo/bole): "lode-lifecycle service fixture" — exists?
   - U (docker/conclave): "reliquary-lifecycle service fixture" — EXISTS or to-be-created?
   - V (buildx/vouch+underpin): "lode-lifecycle, reliquary-lifecycle, AND wsl-lifecycle fixtures all green" — do reliquary-lifecycle and wsl-lifecycle exist?
   - W (podvm immure): "podvm-lifecycle service fixture" — explicitly NEW (pace creates it). Confirm it does not exist.
   - X (skopeo-reliquary/mirror): "a bind-mode build runs green" — is there a bind-mode build fixture? which one?
   - S (theurge fixture fix): the "airgap-chain fixture" — find it; confirm the false-green mechanism (zrbfd_elect_base_anchor reading an empty BURD_PREVIOUS_DIR because rbtdri_invoke_impl isolates each invoke in its own burv_output root). Read rbtdri_invocation.rs / the invoke-impl and rbfd_FoundryDirectorBuild.sh zrbfd_elect_base_anchor to confirm.
3. CRITICAL GAP CHECK: if a pace's verify gate names a fixture that does NOT exist yet and the pace docket does not own creating it, that pace has hidden test-authoring scope OR an unstated dependency on an earlier pace. Record which fixtures are missing and whether any pace owns their creation.

Put per-fixture existence in 'sites' (classification = exists/missing/to-create, claimed_by_pace = which pace's gate needs it). This answers "is there express testing for each transformation?"`, { label: 'test-coverage', schema: SITE_SCHEMA }),

  () => agent(`You are determining the ACTUAL committed state of pace T (₢BHAAT lode-skopeo-eviction) in heat ₣BH, repo at ${ROOT}.

${PACE_MAP}

Pace T has 1 commit already affiliated with it (per the swim lane) yet remains unwrapped and [rough]. The file-touch bitmap shows pace T touched: rbgjl01-ensconce-capture.sh, RBSLE-lode_ensconce.adoc, rbldb_Bole.sh, CBG-CloudBuildGuide.md, RBSCB-CloudBuildPosture.adoc, RBSCJ-CloudBuildJson.adoc, claude-rbk-acronyms.md, rbgjs-gcrane-fingerprint.sh, rbgjs-skopeo-fingerprint.sh, rblds_Spine.sh.

TASK:
1. Find the commit(s) affiliated with pace T: git log --oneline -20, look for the pace silks 'lode-skopeo-eviction' or coronet BHAAT, or recent commits touching rbgjs-gcrane-fingerprint.sh. Show the commit message(s).
2. Inspect what actually landed: did the bole capture path (rbgjl01-ensconce-capture.sh) get fully cut over to gcrane, or did the commit only lay groundwork (create gcrane-fingerprint snippet + update specs) while the bole step itself still invokes skopeo? git show the relevant diffs or read current file state.
3. Is rbgjs-skopeo-fingerprint.sh DELETED or still present? (Bitmap shows it touched — could be a delete.)
4. VERDICT: is pace T effectively DONE-but-unwrapped (just needs the live lode-lifecycle fixture green + wrap), or genuinely PARTIAL (real eviction work remains)? This is load-bearing for whether the docket is stale.

Use git and file reads. Return structured: put findings as 'sites', and the done-vs-partial verdict + any docket-staleness in 'capability_findings' and 'summary'.`, { label: 'pace-T-state', schema: SITE_SCHEMA }),
])

phase('Synthesize')

const SYNTH_SCHEMA = {
  type: 'object',
  additionalProperties: false,
  required: ['q1_crane_gcrane', 'q2_skopeo_removed', 'q3_express_testing', 'q4_mechanical_vs_open', 'cross_cutting', 'recommended_docket_actions'],
  properties: {
    q1_crane_gcrane: {
      type: 'object', additionalProperties: false,
      required: ['verdict', 'findings', 'docket_fixes_needed'],
      properties: {
        verdict: { type: 'string', enum: ['yes', 'no', 'partial'] },
        findings: { type: 'array', items: { type: 'string' } },
        docket_fixes_needed: { type: 'array', items: { type: 'string' } },
      },
    },
    q2_skopeo_removed: {
      type: 'object', additionalProperties: false,
      required: ['verdict', 'findings', 'orphan_sites'],
      properties: {
        verdict: { type: 'string', enum: ['yes', 'no', 'partial'] },
        findings: { type: 'array', items: { type: 'string' } },
        orphan_sites: { type: 'array', items: { type: 'string' } },
      },
    },
    q3_express_testing: {
      type: 'object', additionalProperties: false,
      required: ['verdict', 'per_pace_coverage', 'findings'],
      properties: {
        verdict: { type: 'string', enum: ['yes', 'no', 'partial'] },
        per_pace_coverage: {
          type: 'array',
          items: {
            type: 'object', additionalProperties: false,
            required: ['pace', 'fixture', 'exists', 'gap'],
            properties: {
              pace: { type: 'string' }, fixture: { type: 'string' },
              exists: { type: 'string', enum: ['exists', 'missing', 'to-create-by-this-pace'] },
              gap: { type: 'string' },
            },
          },
        },
        findings: { type: 'array', items: { type: 'string' } },
      },
    },
    q4_mechanical_vs_open: {
      type: 'object', additionalProperties: false,
      required: ['per_pace', 'findings'],
      properties: {
        per_pace: {
          type: 'array',
          items: {
            type: 'object', additionalProperties: false,
            required: ['pace', 'level', 'open_questions'],
            properties: {
              pace: { type: 'string' },
              level: { type: 'string', enum: ['mechanical', 'needs-chat'] },
              open_questions: { type: 'array', items: { type: 'string' } },
            },
          },
        },
        findings: { type: 'array', items: { type: 'string' } },
      },
    },
    cross_cutting: { type: 'array', items: { type: 'string' }, description: 'simplification/cleanliness observations across the whole plan: ordering issues, duplication, paces that could merge or drop' },
    recommended_docket_actions: {
      type: 'array',
      items: {
        type: 'object', additionalProperties: false,
        required: ['pace', 'action'],
        properties: { pace: { type: 'string' }, action: { type: 'string' } },
      },
    },
  },
}

const findings = { skopeo, crane, docker, tests, stateT }

const synthesis = await agent(`You are the synthesis + adversarial-verification step for grooming heat ₣BH (rbk-11-mvp-lode-universal-capture). Five investigators audited the repo at ${ROOT}. Their structured findings:

SKOPEO INVENTORY:
${JSON.stringify(skopeo, null, 2)}

CRANE/GCRANE CONSISTENCY:
${JSON.stringify(crane, null, 2)}

DOCKER/BUILDX INVENTORY:
${JSON.stringify(docker, null, 2)}

TEST COVERAGE:
${JSON.stringify(tests, null, 2)}

PACE-T ACTUAL STATE:
${JSON.stringify(stateT, null, 2)}

${PACE_MAP}

The operator asked four questions about whether the plan is clean, complete, and simplifying. Answer each from the evidence above — do NOT invent facts; if the evidence is thin on a point, say so and (if needed) re-grep the repo yourself to confirm before asserting. You have full repo access; spot-check any claim that looks shaky.

Q1: Is crane fully replaced with gcrane everywhere? (verdict + the specific terminology/auth risks; what docket text must change so a mount agent uses gcrane not plain crane.)
Q2: Is skopeo fully removed? (verdict + every orphan skopeo site no pace claims; confirm the eviction chain T->...->X->D leaves zero skopeo.)
Q3: Is there express testing for each transformation? (per-pace verify-gate fixture, whether it EXISTS today, and which paces rely on a fixture that doesn't exist yet without owning its creation — that is a real gap.)
Q4: Are paces defined at the level of mechanical implementation, or do some have open issues that need resolving through chat first? (per-pace: mechanical vs needs-chat, with the specific open question.)

Also: cross_cutting observations (ordering hazards, paces that duplicate/could merge/could drop, simplification opportunities) and a concrete list of recommended docket actions (reslate X, add fixture-creation scope to pace Y, etc.).

Be exhaustive and concrete. Quote files/lines. This is the spine of a groom presented to the operator.`, { label: 'synthesize', schema: SYNTH_SCHEMA })

return synthesis