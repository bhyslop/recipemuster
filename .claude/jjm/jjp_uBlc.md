## Paddock: jjk-v4-1-isolated-schema-changes

## Shape
This heat grew past its original framing of two isolated schema edits into three layers.

A reusable schema-migration tolerance mechanism — "forgiveness" — is the spine.
It is a cited rivet (an inert token in shipped code, rationale in the veiled spec), a single read-only detection probe shared by the loader and a new open-time status line, a per-episode registry, and a self-reporting nag at officium open.
The nag tells the operator, per install, whether each forgiveness is still load-bearing or has gone dormant.

Two wire-format-breaking schema changes ride that spine.
The first turns tack docket text into a line array and moves tack history out of the JSON into git, keeping one edited-in-place current tack.
The second turns validate into a normalize-and-report pass that canonicalizes the store and carries its verdict in the exit code.
Each registers an episode against the mechanism rather than re-plumbing detection.

A coda then propagates the changes to every JJK install — this source clone and two binary-only remotes — and strips the whole scaffolding once all installs confirm converged.

## Cinched
The forgiveness mechanism is the spine; the two schema changes register episodes against it, they do not each invent a trigger.
The rivet stays inert in shipped code; its rationale and demolition condition live once in the veiled spec — the RBr_ rivet doctrine made JJK-native, because the rust ships and the specs do not.
The shared probe is the single source of "what counts as old-format"; the loader keeps no second copy.
The mechanism is per-episode from the start — at strip time more than one forgiveness is live, so the removal gate is "all dormant", never a single boolean.
The open-time probe is read-only, best-effort, and never gating; officium open must still always succeed, which is a deliberate, logged softening of "open touches no persistent state" down to "open mutates nothing".
The coda owns the full strip, including the original V3-to-V4 legacy tolerance; that removal is consolidated here rather than deferred to a separate stale placeholder heat.
The demolition condition — no pre-current-schema gallops survives in any operated clone — is checkable rather than an act of faith because the operator is sole, so the operated clones are a finite, enumerable set.

## Done when
All three installs run a binary whose on-disk gallops is canonical at this heat's final schema and whose open-time probe reports every episode dormant; all forgiveness scaffolding is stripped from source; the spec's forgiveness quoin is discharged; the crate builds and tests green after removal.