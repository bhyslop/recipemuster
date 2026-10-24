# <a id="ReleaseRunbook"></a>Release Procedure

How the maintainer takes a candidate from repair to a promoted public release.
The [Hierophant](#Hierophant) conducts four commands and stops wherever your hands are needed; this procedure says what to do at each stop and between the commands.
The law behind it is the Marshal Operations branch of the spec: where the two disagree the law wins, and the disagreement is a finding to raise.
The words in capitals are defined in the glossary at the end.

Run every command from the maintainer repository, with its tree committed: each [Hierophant](#Hierophant) command refuses an uncommitted tree, and builds itself before it runs.

1. [Standing ground](#StandingGround) — once per station.
2. [The laps](#TheLaps) — [Essai](#Essai), repeated until a [Lap](#Lap) runs dry.
3. [The reveal](#TheReveal) — [Docimasy](#Docimasy), then [Ostend](#Ostend), then [Harbinger](#Harbinger).
4. [The walk](#GreenfieldWalk) — the greenfield manor walk against promoted public main.
5. [Closeout](#Closeout) — liveness, the spot-check, the deletes and the record.

## <a id="Rules"></a>Rules

- Only [Ostend](#Ostend) does anything irreversible. [Essai](#Essai), [Docimasy](#Docimasy) and [Harbinger](#Harbinger) may be re-run freely.
- The tool never pushes. When it prints a push line, type exactly that line yourself, then answer its prompt so it can check the result.
- A finding means re-cut, never patch forward: repair on the maintainer tree, commit, and run a new [Lap](#Lap).
- The [Regulus](#Regulus) the reveal shows is the one the dry [Lap](#Lap) proved, never a fresh cut.

<a id="RuledOrder"></a>The standing doctrine is that the walk gates promotion.
This release runs under the operator's ruled exception to it: [Ostend](#Ostend) stages and promotes in one run, and the walk follows against promoted public main.

## <a id="StandingGround"></a>Standing ground

Once per station, before the first lap.

1. Add the read-only base remote and neuter its push side.
   A push-capable remote to the public repository is the one forbidden catastrophe, and [Essai](#Essai) refuses to cut unless the push side reads exactly this sentinel:

   ```
   git remote add ENGROSSMENT_UPSTREAM git@github.com:scaleinv/recipebottle.git
   git remote set-url --push ENGROSSMENT_UPSTREAM DISABLED-ENGROSSMENT_UPSTREAM-IS-READ-ONLY
   ```
2. Bring the kennel current with `tt/bkw-w.Whistle.sh`.
   Run it again whenever a [Hierophant](#Hierophant) command refuses because the tree has outrun the kennel.
3. Prove that SSH reaches the public repository, since every push in the reveal travels it:

   ```
   git ls-remote git@github.com:scaleinv/recipebottle.git
   ```
4. At the cloud console, count the projects linked to the billing account and confirm three free link slots: two for the `gauntlet` and one for the walk.
5. Delete any working directory the retired bash release instruments left beside the maintainer repository; the [Hierophant](#Hierophant) never uses them.

## <a id="TheLaps"></a>The laps

1. Run `tt/rbvw-e.Essai.sh`.
   It asks nothing of you while it runs and makes no remote act: it gates the tree and the base, cuts the [Regulus](#Regulus) into `../rbthdr_regulus/regulus`, proves it, and stands up a coldwalk [Rig](#Rig).
   A [Regulus](#Regulus) already standing there is first moved aside by [Sepose](#Sepose).
2. Read its verdict and the base inventory it prints.
   On any red, repair on the maintainer tree, commit, and run it again.

### <a id="StrangerColdwalk"></a>The stranger coldwalk

Every green lap ends with a [Rig](#Rig) waiting to be walked.
Launch the walk by hand from the launch line and stranger prompt the lap printed: a fresh agent session with no context walks the shipped onboarding literally, with you at the keyboard.
Its findings go to the dated memo path the lap named.
A finding means a repair and another lap.
A lap whose battery is green and whose coldwalk finds nothing has run dry, and only then do the three steps below follow.

### <a id="CensusInquest"></a>The census inquest

From the outspan of a dispatched chat, run `tt/vom-i.Inquest.sh` and read its statistics table: the row `VOr_ew7`, the Bash guide's one-form substitution read, must stand at zero.
If it does not, repair and run another lap.

### <a id="RegulusReading"></a>Reading the regulus

The [Regulus](#Regulus) is one commit atop public main, so that commit is everything the public will newly see.
Read it yourself, privately, before the reveal; the file list [Ostend](#Ostend) shows later carries names only.

```
git -C ../rbthdr_regulus/regulus show --stat HEAD
git -C ../rbthdr_regulus/regulus diff HEAD^ HEAD > ../rbthdr_regulus/regulus.diff
git -C ../rbthdr_regulus/regulus ls-files
```

Read `regulus.diff` through.
Anything that should not be public means a repair and another lap.

### <a id="QuietTrunk"></a>Hold the trunk quiet

From the dry lap until promotion, land nothing on the trunk that changes delivered bytes, a kit parcel install above all.
The reveal re-checks that the [Regulus](#Regulus) still matches the trunk, and drift sends the cycle back to laps.

## <a id="TheReveal"></a>The reveal

### <a id="RevealPreparation"></a>Prepare

At the GitHub console, by hand:

1. Delete the [Narthex](#Narthex) repository `scaleinv/recipebottle-staging` and create it again, empty and private.
   [Docimasy](#Docimasy) refuses a [Narthex](#Narthex) holding an earlier cut.
2. Delete the public repository's `POSTULANT_LOCAL` branch left by the previous release, so the staging push creates it fresh.
   If a branch ruleset refuses the delete, relax the ruleset for the delete.
3. Confirm three free billing link slots again.
4. Confirm the Pages source is the README rendered from main root, and close any stale open pull request on the public repository.

### <a id="DocimasyAct"></a>Docimasy

Run `tt/rbvw-d.Docimasy.sh` inside a terminal multiplexer session, since its last hour runs unattended.
It gates the [Narthex](#Narthex), confirms the [Regulus](#Regulus) still matches the trunk, and then stops three times:

1. It prints the preview push line into the private [Narthex](#Narthex).
   Type it, then answer that it is pushed.
2. It opens a browser sign-in to renew your session.
   Complete it; this is the fresh sign-in the unattended hour needs.
3. It asks whether to proceed with marshal zero and the `gauntlet`.
   Answer yes and leave it: marshal zero blanks the regime and commits that, and the `gauntlet` runs for about an hour on two new cloud projects.

On green it grants the [Cachet](#Cachet) and points you to [Ostend](#Ostend).

On red:

- If the credential check fails, reinstall the payor credential with `tt/rbw-gPI.PayorInstall.sh «client secret JSON»`, then run [Docimasy](#Docimasy) again.
- If the `gauntlet` failed for a reason outside the candidate, such as a quota wall or a transient cloud fault, release the depot it stranded with `tt/rbw-dU.PayorUnmakesDepot.sh «project-id»` and run [Docimasy](#Docimasy) again; a preview already pushed is recognized and not asked for again.
- If the `gauntlet` found a real defect, release the stranded depot the same way, repair on the maintainer tree, and return to the laps.

### <a id="OstendAct"></a>Ostend

Run `tt/rbvw-o.Ostend.sh` and stay with it throughout; it is short.
It requires the [Cachet](#Cachet), re-checks the [Narthex](#Narthex) and the trunk, and then stops three times:

1. It shows the full file list of the [Regulus](#Regulus).
   Read it, then answer.
2. It prints the staging push line.
   This is the [Kerygma](#Kerygma), the one act that cannot be undone: type it, then answer.
   It confirms public main did not move and `POSTULANT_LOCAL` stands at the [Regulus](#Regulus) tip.
3. It prints the promotion line.
   Type it from a fresh clone of the public repository as below, then answer.
   It confirms public main now equals the [Regulus](#Regulus) tip.

If promotion is refused as a non-fast-forward, public main moved since the cut: stop, never force, and re-cut atop it.
Ostend closes by pointing you to [Harbinger](#Harbinger).

<a id="FreshCloneRecipe"></a>The printed promotion line names a local `POSTULANT_LOCAL` branch and is never run from the [Regulus](#Regulus) directory.
At the promotion stop, in a second terminal:

```
git clone git@github.com:scaleinv/recipebottle.git «fresh clone»
cd «fresh clone»
git fetch origin POSTULANT_LOCAL:POSTULANT_LOCAL
```

Then type the printed promotion line in that terminal.

### <a id="HarbingerAct"></a>Harbinger

Run `tt/rbvw-h.Harbinger.sh`.
It clones promoted public main over HTTPS, exactly as a stranger would, guards the clone so it can never push, and prints a launch line, a stranger prompt and a dated findings-memo path.
Launch the confirmation coldwalk by hand, with you at the keyboard, and review and commit its findings memo afterward.

## <a id="GreenfieldWalk"></a>The walk

Walk the whole manor yourself against promoted public main, as a stranger: from the onboarding entry to a green `gauntlet` on a fresh payor substrate.
It takes hours; start it fresh, not tired.
Record every divergence between the shipped documents and what actually happens.

Dispose of each finding:

- A documents-only divergence is fixed as a fast follow, or by re-walking only the steps it touched.
- A repair that changes delivered bytes reopens the whole cycle: laps to dry, the reveal, and a fresh walk.
  Until it lands, the defect stands on the promoted face, which is the accepted cost of promoting first.
- A repair that touches only withheld paths needs only a new lap, never a new walk or `gauntlet`.
- A re-walk reuses the standing walk setup and never levies a second payor project.

## <a id="Closeout"></a>Closeout

1. Confirm the public README serves at its URL and that every anchor in it resolves on the live page.
   Links into main go live only at promotion; before the push only their structure was checked, by the `pyx` sweep.
2. Spot-check the repository front page and the github.io render.
3. Delete the public `POSTULANT_LOCAL` branch, relaxing the ruleset again if it refuses, and then the [Narthex](#Narthex) repository.
4. Commit any coldwalk findings memo you mean to keep, since each stands inside its `rbthdr_coldwalk` directory; then delete every `rbthdr_*` directory beside the maintainer repository.
5. Record the release in `RBSPH-ProjectHistory.adoc`, the project's history; this procedure carries no dated record.

## Glossary

<a id="Regulus"></a>**Regulus**

The cut deliverable: exactly one commit atop the live public base, built by addition from the paths the perambulation ships, on the sole branch `POSTULANT_LOCAL`, holding zero remotes.
The dry [Regulus](#Regulus) the reveal discloses is bit-for-bit the [Regulus](#Regulus) the [Laps](#Lap) proved — a terminal re-cut would break exactly that identity.
Named for the pure metal that settles out of a crucible.

<a id="Kerygma"></a>**Kerygma**

The single irreversible public act: the [Regulus](#Regulus) pushed onto the public repository as the unmerged `POSTULANT_LOCAL` branch, `main` untouched.
The staging push, not the promotion, carries promotion-grade gravity — a public object store cannot be un-disclosed; promotion after it is discoverability, not [Kerygma](#Kerygma).
Named for a herald's public proclamation.

<a id="Hierophant"></a>**Hierophant**

The veiled application that conducts the release ceremony through four commands; it checks everything and pushes nothing.

<a id="Essai"></a>**Essai**

The repair command: gates, cuts the [Regulus](#Regulus), proves it and stands up a coldwalk [Rig](#Rig), with no remote act.

<a id="Lap"></a>**Lap**

One run of [Essai](#Essai), repeated until one runs dry: green, with a coldwalk that finds nothing.

<a id="Docimasy"></a>**Docimasy**

The proving command: previews the [Regulus](#Regulus) into the [Narthex](#Narthex), runs the `gauntlet`, and grants the [Cachet](#Cachet).

<a id="Narthex"></a>**Narthex**

The private repository the preview is pushed into, created and deleted by hand.

<a id="Cachet"></a>**Cachet**

The recorded verdict that the `gauntlet` ran green against exactly this [Regulus](#Regulus); [Ostend](#Ostend) will not run without it.

<a id="Ostend"></a>**Ostend**

The showing command: the file-list review, the [Kerygma](#Kerygma) and promotion.

<a id="Harbinger"></a>**Harbinger**

The command that stands up a stranger [Rig](#Rig) against promoted public main.

<a id="Rig"></a>**Rig**

A guarded, disposable clone a stranger walk runs in; it cannot push.

<a id="Sepose"></a>**Sepose**

Moving a prior working directory aside to a timestamped sibling instead of deleting it.
