# CMK Rules of Engagement — Detail & Lineage

Companion to `claude-cmk-rules-of-engagement.md`. **Not `@`-included** — read on demand when a theme needs depth, or when filing a new instance. The always-on ROE carries the stance; this carries the mechanics, the ledger, and the history, so the always-on layer stays weightless.

## The verdict-less boundary

A boundary is *verdict-less* when it emits a success/fail verdict uncorrelated with the operation's true terminal state. It collapses three distinct conditions — done / not-yet / broken — into one undecidable signal and dumps the disambiguation on the caller, whose only tool is retry/poll/timeout. Two mirror-image species:

- **Premature acknowledgment** — the boundary says *done* before it is. The far side is behind its own acknowledgment. Canonical: GCP IAM propagation and billing linkage — "eventually consistent," with no pollable completion contract. (See the hyperscaler API-consistency premise memo and `RBSCIP-IamPropagation`.)
- **Premature abandonment** — the boundary says *failed* before a healthy backend answered. The *client* is impatient. Canonical: `docker login`'s hardcoded, non-configurable 15s registry-auth timeout (moby/moby#44350), which fires against a slow-but-fine backend; it is the one verb among login/pull/push that carries no internal retry of its own.

Both are the same lie in opposite directions — a false negative and a false positive on "is it finished?" The giant shipped the 95% that works and withheld the last 5%: an honest, bounded, pollable terminal-state signal. That 5% is permanently ours to absorb.

## The membrane pattern

A membrane is the single contained place where our sovereign disciplines bend to match a foreign reality, re-minting a clean verdict for the interior. The conduct from the ROE, expanded:

1. **Characterize** — pin the exact signature. For premature abandonment, match the stable substring (e.g., the Go-stdlib `Client.Timeout exceeded while awaiting headers`, which is version-invariant because it comes from the standard library, not docker's own message formatting). Never "it's flaky."
2. **Contain** — one helper, one boundary-crossing verb. Do not scatter tolerance across call sites; route them all through the membrane.
3. **Absorb the surveyed signature only** — a *classified* retry, not a blind one. A blind retry violates crash-fast (it hides real failures) and Zeroes Theory (it adds unbounded paths to the state space). The signature allowlist keeps the added state-space to exactly the enumerated transient; everything else fails fast.
4. **Log the bend** — `buc_warn` (or the local equivalent) on every absorbed transient. Crash-fast survives because the papering-over is *visible*, not silent.
5. **Retire on heal** — tie the membrane to the specific grievance (issue number, signature) so it carries a removal condition. Consistency gaps do close — AWS S3 went strongly read-after-write consistent in 2020 at no cost, proof that consistency is an engineering choice. A membrane without a demolition date calcifies into permanent cruft that outlives the bug.

### Relationship to the interior disciplines

- This is the **external dual of Interface Contamination Discipline**. That one guards what we *accept inward* (one canonical form, no tolerances). This governs what we *trust* from a boundary we do not own. One guards the input edge; the other guards the outcome edge.
- It is the **single named exception to Zeroes Theory**. Tolerances are presumptively forbidden. A membrane is the one licensed tolerance class — licensed *because* the nondeterminism is the environment's, not ours, and disciplined (signature-scoped) so the license cannot become a backdoor for general sloppiness.

## The recursion: the ungovernable includes us

The boundary we cannot fix includes our own. The Digital Mind resets each session (no memory across the seam); reaches for plausible-but-unasked niceness; conflates planning vocabulary with product vocabulary. The collaboration's own machinery is a set of membranes against exactly these:

- The **officium / steeplechase / gazette** (JJK) — membranes against session-amnesia: bounded sessions, git-as-journal, structured I/O across statelessness.
- The **equestrian vocabulary** (JJK) — a membrane against plan/product collision, exploiting that vivid sensory metaphor persists across sessions where abstract labels do not.
- The **BCG / RCG disciplines** — membranes against specific LLM failure modes: web-brain input-accommodation, commit-narration in code comments, invention of unspecified features.

Engaging the ungovernable responsibly is one skill, whether the counterparty is a vendor who will not finish their work or a collaborator who cannot hold memory. The human partner's biological context erosion, the Digital Mind's session reset, and the vendor's verdict-less API are the same structural fact through a rhyme, not a shared construction. These documents are the membrane across all three.

## Instance ledger

Membranes currently standing in this project. Append as new ones are built; prune as neighbors heal.

| Membrane | Boundary | Species | Retirement condition |
|----------|----------|---------|----------------------|
| `rbgo_docker_login`, `zrbndb_docker_login` | docker daemon → GAR registry auth | premature abandonment | moby/moby#44350 ships a configurable timeout |
| `rbuh_json` transient retry | curl → GCP REST | network transient (curl 7/28/35/56) | n/a — generic network blips |
| IAM propagation backoff (`rbgi`, `RBSCIP`) | GCP IAM grant visibility | premature acknowledgment | GCP exposes a terminal-state contract for grants |
| Platform-variant command wrapping (BCG) | GNU vs BSD CLI flag drift | interface divergence | n/a — structural |
| `openssl` declared dependency | base64 / sha256 platform variance | interface divergence | n/a — structural |

The formal premise voicing lives in domain specs (e.g., `RBSCIP-IamPropagation` voices `axk_premise`); this ledger is the human-facing catalogue, not the formal vocabulary.

## Lineage — the headwaters

This stance descends from a long, mostly-solo experiment in human-AI rules of engagement, conducted before the CLAUDE.md `@`-include era — when "loading" a ROE meant pasting it into a chat window. Those experiments mostly did not land as durable practice, but two ideas did, and they live on here:

- **Name your failure modes precisely** — from the ANNEAL "fissure" method; it is the structure of the verdict-less-boundary taxonomy above.
- **Make your own state legible** — from the state-reflection experiments; recovered as the two-way salutation. The ritual (mandated tuples, weather emojis) was dropped; the gift — disclosing your weather so your partner can adapt to you — was kept.

The original lenses survive at `../cnmp_CellNodeMessagePrototype/lenses/a-roe-*` (relative to this project root): ANCIENT (ANCHOR-ROE, the meta), ANNEAL, CRAFT, METAL, MIND. Roughly 796 commits, October 2024 – January 2026. MCM and AXLA crystallized out of that same crucible and now live in CMK; this document is the next thing to make the crossing from dormant lenses to living context. Treat the lenses as honored source, not active practice.

> **Distribution note.** The stance (salutation, ROE, the taxonomy and membrane pattern above) is universal and shippable with CMK. The instance ledger and the local lenses path are rbm-specific. If CMK is pushed upstream, those two sections are the local parts to hold back or template.
