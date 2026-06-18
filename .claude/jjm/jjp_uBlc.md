## Shape
This heat grew past its original framing of two isolated schema edits into three layers.

A reusable schema-migration tolerance mechanism — "forgiveness" — is the spine.
It is a cited rivet (an inert token in shipped code, rationale in the veiled spec), a single read-only detection probe shared by the loader and a new open-time status line, a per-episode registry, and a self-reporting nag at officium open.
The nag tells the operator, per install, whether each forgiveness is still load-bearing or has gone dormant.

Two wire-format-breaking schema changes ride that spine.
The first turns tack docket text into a line array and moves tack history out of the JSON into git, keeping one edited-in-place current tack.
The second turns validate into a normalize-and-report pass that canonicalizes the store and carries its verdict in the exit code.
Each registers an episode against the mechanism rather than re-plumbing detection.

A coda then propagates the changes to every install and retires the spent episodes — but not the mechanism.
Multi-install schema drift is the steady state of this tool, not a transition to be ended: there is always a leading clone where a schema-impacting change lands first and lagging clones not yet converged.
So the registry, the shared probe, and the open-time nag are permanent infrastructure; only individual episodes — the per-version tolerances and their frozen legacy types — retire once every clone converges.

## Cinched
The forgiveness mechanism is the spine; the two schema changes register episodes against it, they do not each invent a trigger.
The rivet stays inert in shipped code; its rationale lives once in the veiled spec — the RBr_ rivet doctrine made JJK-native, because the rust ships and the specs do not.
The shared probe is the single source of "what counts as old-format"; the loader keeps no second copy.
The mechanism is per-episode from the start — more than one episode is live at once, so an episode's removal gate is "dormant on every operated clone", never a single boolean.
The open-time probe is read-only, best-effort, and never gating; officium open must still always succeed, which is a deliberate, logged softening of "open touches no persistent state" down to "open mutates nothing".
Mechanism and episode are distinct lifetimes: the registry, probe, and nag are permanent because drift is the steady state; the per-version tolerances and frozen legacy types are transient and retire on convergence.
The coda retires the spent episodes — this heat's tack-rework tolerance and the original V3-to-V4 tolerance with its frozen reference, consolidated here rather than left in a stale placeholder heat — and leaves the mechanism standing.
An episode's demolition condition — no pre-its-schema gallops survives in any operated clone — is checkable rather than an act of faith because the operator is sole, so the operated clones are a finite, enumerable set.
The durable "deep pattern" is not a torn-down cleverness but the standing mechanism plus its governing why — that we minimize the schemas we keep mutually readable (Zeroes Theory's version axis) — formalized cross-language in ₣Bb and consumed here.

## Done when
Every install runs a binary whose on-disk gallops is canonical at this heat's final schema and whose open-time nag reports every episode dormant;
this heat's tack-rework tolerance and the V3-to-V4 tolerance with its frozen reference are stripped and their registry entries removed;
the mechanism — registry, probe, open-time nag — stands as permanent infrastructure;
the forgiveness quoin describes that permanent mechanism with its per-episode lifecycle rather than a self-demolishing whole;
the crate builds and tests green.