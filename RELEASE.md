# <a id="ReleaseRunbook"></a>Release Procedure

The release qualification ceremony for the project maintainer: how a candidate that has run dry in repair laps is proved, shown to the public, walked as a stranger would walk it, and recorded.
The [Hierophant](#Hierophant) conducts the acts and the operator conducts the hands.
This procedure is the operator's face of the Marshal Operations law of the spec: the order of the acts, the work between them that no tool does, and the commands to type.
It restates that law and never rules it.
Where this procedure and the Sheaves it restates disagree the Sheaves win, and the disagreement is a finding for the operator rather than a thing to settle at the keyboard.

The procedure runs in five movements, in this order:

1. [Standing ground](#StandingGround) — once per station, before the first lap.
2. [The laps](#TheLaps) — repair laps, repeated until one runs dry, and the readings taken at the dry lap alone.
3. [The reveal](#TheReveal) — the dry candidate proved, shown and promoted, then the stranger rig stood up against the promoted face.
4. [The walk](#GreenfieldWalk) — the greenfield manor walk, in full, against promoted public main.
5. [Closeout](#Closeout) — liveness, the spot-check, the three deletes and the record.

## Vocabulary

The ceremony's own words, as the spec defines them.

<a id="Hierophant"></a>**Hierophant**

The veiled Rust application that conducts the delivery ceremony, as the Theurge conducts the tests.
It owns four commands, one per ceremony act: [Essai](#Essai) the reversible repair lap, [Docimasy](#Docimasy) the reversible proving act of the reveal, [Ostend](#Ostend) the irreversible showing that closes the reveal, [Harbinger](#Harbinger) the stranger rig.
It sequences proven workers and absorbs orchestration, never authority — with the one ruled exception of the cut, which it owns outright in Rust (the cosmology's single-matcher section); it never holds a push.

<a id="Lap"></a>**Lap**

One [Essai](#Essai) cycle — gates, cut, prove, rig — repeated until a lap **runs dry**: a lap surfacing no finding.
Only a dry [Regulus](#Regulus) proceeds to the reveal; a finding means re-cut, never patch forward.

<a id="Regulus"></a>**Regulus**

The cut deliverable: exactly one commit atop the live public base, built by addition from the paths the perambulation ships, on the sole branch `POSTULANT_LOCAL`, holding zero remotes.
The dry [Regulus](#Regulus) the reveal discloses is bit-for-bit the [Regulus](#Regulus) the [Laps](#Lap) proved — a terminal re-cut would break exactly that identity.
Named for the pure metal that settles out of a crucible.

<a id="Narthex"></a>**Narthex**

The ephemeral private repository the reversible preview lands in — created empty by the operator before a cut, reached only by explicit URL, deleted by hand once the [Regulus](#Regulus) is dispositioned.
No tooling creates or destroys it, and no delete-scoped token is ever minted.
Containment only: the additive construction of the [Regulus](#Regulus) is the prevention.
Named for the church antechamber where those not yet admitted wait.

<a id="Rig"></a>**Rig**

The guarded disposable walk clone — severed origin, pre-push refusal hook, throwaway walk branch, dated findings-memo path, prior rig [Seposed](#Sepose) — that a stranger walk runs in.
One construction serves both [Essai](#Essai) (cloning the local [Regulus](#Regulus)) and [Harbinger](#Harbinger) (cloning promoted public `main`); the clone source is their sole divergence.

<a id="Sepose"></a>**Sepose**

The [Hierophant's](#Hierophant) dispose-by-rename: a prior artifact standing at a fixed sibling location — a [Rig](#Rig), a [Regulus](#Regulus), a freshness scratch — is renamed to a timestamped sibling and never deleted, so nothing the disposal touches can be destroyed even if a guard were wrong.
The renamed siblings stand until the operator sweeps them by hand.
Named for an obsolete English verb, to set a thing apart.

<a id="Kerygma"></a>**Kerygma**

The single irreversible public act: the [Regulus](#Regulus) pushed onto the public repository as the unmerged `POSTULANT_LOCAL` branch, `main` untouched.
The staging push, not the promotion, carries promotion-grade gravity — a public object store cannot be un-disclosed; promotion after it is discoverability, not [Kerygma](#Kerygma).
Named for a herald's public proclamation.

The [Hierophant](#Hierophant) owns four commands — [Essai](#Essai), [Docimasy](#Docimasy), [Ostend](#Ostend) and [Harbinger](#Harbinger) — each a Tabtarget given with its movement below, and the [Cachet](#Cachet) is the verdict [Docimasy](#Docimasy) grants and [Ostend](#Ostend) requires.
The glossary at the end of this document says what each of those words names.

## The law in brief

### Reversible and irreversible

The ceremony has one load-bearing boundary, between the reversible and the irreversible, and the commands are that boundary made executable.
[Essai](#Essai), [Docimasy](#Docimasy) and [Harbinger](#Harbinger) are wholly reversible: everything they make is local or private, disposable, and replaceable by another run.
[Ostend](#Ostend) alone crosses the line.
The reveal is conducted in two acts split exactly there: [Docimasy](#Docimasy) proves and grants the [Cachet](#Cachet), and [Ostend](#Ostend) shows and refuses to conduct without a standing [Cachet](#Cachet) whose tree hash matches the standing [Regulus](#Regulus).
Iteration belongs entirely to the reversible tier, and the `gauntlet` runs once per repair cycle, inside [Docimasy](#Docimasy), never inside a [Lap](#Lap).

### Every public act is yours

Every public act is human-hands-only by structural incapacity, never by discipline.
The [Hierophant](#Hierophant) never holds a push: it prints the push line, you type it, and it verifies the result.
Every push line in the reveal is therefore the one the tool prints, and this procedure does not restate it.
Every push is an explicit URL plus a single explicit refspec, and the fan-out forms are forbidden.

### The three public acts

There are three distinct public acts and one of them is irreversible.
The preview is reversible: the [Regulus](#Regulus) pushed into the private [Narthex](#Narthex), where a git commit being content-addressed makes the previewed object, bit for bit, the object later disclosed.
The [Kerygma](#Kerygma) is the single irreversible act: the [Regulus](#Regulus) pushed onto the public repository as the unmerged `POSTULANT_LOCAL` branch, with public main untouched.
Promotion is discoverability and not disclosure: fast-forward-only, from a fresh clone or fetch of the public repository, asserting tree-hash equality.
The standing doctrine is that the greenfield walk gates promotion and never the [Kerygma](#Kerygma).

<a id="RuledOrder"></a>This release runs under the operator's ruled exception to that doctrine: promotion comes before the walk, and the walk runs against promoted public main.
The order of the reveal is therefore [Docimasy](#Docimasy), then [Ostend](#Ostend) — which stages and promotes in one run — then [Harbinger](#Harbinger), then the walk.
The doctrine is restored for later releases once real users exist to protect.

## Standing ground

<a id="StandingGround"></a>Once per station, before the first lap.
Nothing here repeats per cycle except the billing headroom, which the reveal reads again.

### <a id="ReadOnlyBase"></a>The read-only base

The cut reads the real public repository through the remote `ENGROSSMENT_UPSTREAM`, and that remote is read-only by construction.
A push-capable remote to the public target in the maintainer repository is the one forbidden catastrophe, a stray push landing the private tree on public main.
The remote is added once and its push side neutered to the fixed sentinel, and a lap refuses to proceed from a base whose push side is anything else:

```
git remote add ENGROSSMENT_UPSTREAM git@github.com:scaleinv/recipebottle.git
git remote set-url --push ENGROSSMENT_UPSTREAM DISABLED-ENGROSSMENT_UPSTREAM-IS-READ-ONLY
```

### <a id="KennelAndBuild"></a>The kennel whistle, then the build

Take the whistle, which reports the kennel's own making and builds it first where the seat has outrun it, and then build the [Hierophant](#Hierophant):

```
tt/bkw-w.Whistle.sh
tt/rbvw-b.Build.sh
```

### <a id="SshProof"></a>SSH to the public repository

Prove that SSH reaches the public repository by a remote read of its refs, `ls-remote` over SSH.
The reveal's pushes travel the same road, and the road is proved here rather than found wanting at the irreversible act.

### <a id="BillingHeadroom"></a>Billing headroom

A console read with no tool behind it.
Count the projects linked to the billing account against its link quota, and require three free slots: two for the ladder and one for the walk.
A failed run strands a depot holding a link, the quota is small and the levy refuses at the quota wall, so [Unmake](#rbsgo_depot_unmake) any stranded depot once its diagnosis is extracted.

### <a id="RetiredLocations"></a>Retired working locations

Sweep by hand any working location standing beside the billets under a name the ceremony has retired.
The tooling never deletes one for you: its disposal of a prior artifact is the [Sepose](#Sepose) rename, and the renamed siblings stand until you sweep them.

## The laps

<a id="TheLaps"></a>Repair laps are repeated until one runs dry.
A lap is the reversible cycle of gates, cut, proof and rig, and it makes no remote act of any kind.

### Running a lap

One drive of [Essai](#Essai), `tt/rbvw-e.Essai.sh`, is one [Lap](#Lap).

In order, it gates the maintainer tree and the base: the tree clean and fully pushed, and the base remote's push side neutered.
It then runs the pre-cut assays on the maintainer tree, the `reveille` suite and the in-process veil assay.
It cuts the [Regulus](#Regulus) by addition atop the live public base, one commit on the sole branch `POSTULANT_LOCAL` holding zero remotes.
It proves the [Regulus](#Regulus) with its own battery: the delivered wiring, `cupel`, `pyx`, `damnatio`, the veil token in the transposed root context document, and a `reveille` from a feigned station at the consumer's seat.
It stands up the coldwalk [Rig](#Rig) from the local [Regulus](#Regulus) and hands off the walk.
The base inventory of already-disclosed withheld history is surfaced for you to acknowledge.

> [!NOTE]
> A finding means re-cut, never patch forward.
> A red assay abandons the [Regulus](#Regulus); the repair lands on the maintainer tree and the next [Lap](#Lap) runs from the top.

The battery is an unattended stretch.
Run the drive in a terminal multiplexer session, and take a fresh cloud sign-in just before any unattended stretch that spends credentials.

### <a id="StrangerColdwalk"></a>The stranger coldwalk

Each lap ends with a [Rig](#Rig) awaiting a stranger coldwalk, and a lap cannot be judged dry before it is walked.
You sit at the keyboard for each one: a fresh agent session with no context walks the shipped onboarding literally, launched by hand from the launch line and the stranger prompt the lap printed.
The walk's findings go to the dated memo path the lap named.
The pristine [Regulus](#Regulus) is untouched by the walk, and the disposable [Rig](#Rig) is what the walker may change.

### The dry lap

A lap runs dry when every assay is green, the base inventory is acknowledged, and its coldwalk surfaced no finding.
Only a dry [Regulus](#Regulus) proceeds to the reveal, and the three readings below are taken at the dry lap alone.

#### <a id="CensusInquest"></a>The census inquest

Drive the census inquest from the outspan of a dispatched chat:

```
tt/vom-i.Inquest.sh
```

In its statistics table the row for the Bash guide's one-form substitution read, `VOr_ew7`, must read zero.

#### <a id="RegulusReading"></a>Reading the regulus

The reveal's file-list review reads names only, so the content is read here first, by you alone and in private.
The base already carries what the first delivery disclosed, so a new leak can enter only through the delta atop it, and the [Regulus](#Regulus) is exactly one commit.
Read that commit's stat, then its diff against its parent written to a file beside the [Regulus](#Regulus), then its file list, with «regulus» standing for the [Regulus](#Regulus) directory the lap named:

```
git -C «regulus» show --stat HEAD
git -C «regulus» diff HEAD^ HEAD > «a file beside the regulus»
git -C «regulus» ls-files
```

Read the diff file through before the reveal.

#### <a id="QuietTrunk"></a>Hold the trunk quiet

From the dry lap until promotion, land nothing that carries delivered bytes on the trunk.
A kit parcel installed in that window is the practical hazard, since delivered-bytes drift reddens the freshness assert and returns the cycle to laps.

## The reveal

<a id="TheReveal"></a>The reveal takes the dry [Regulus](#Regulus) and never a terminal re-cut, so the bytes walked are bit-for-bit the bytes disclosed.
Its order is hand acts, then [Docimasy](#Docimasy), then [Ostend](#Ostend), then [Harbinger](#Harbinger).

### <a id="NarthexReset"></a>Dispose and recreate the narthex

At the console, dispose of the [Narthex](#Narthex) repository and create it again empty and private, at `git@github.com:scaleinv/recipebottle-staging.git`.
The previous cycle's preview still stands in it, and [Docimasy](#Docimasy) refuses a [Narthex](#Narthex) carrying a prior cut.
No tooling creates or destroys it, and no delete-scoped token is ever minted.

### <a id="StagingBranchDelete"></a>Delete the stale public staging branch

At the console, delete the public repository's `POSTULANT_LOCAL` branch left by the previous cycle, so the staging push creates the branch as the law describes.
Where the console refuses the delete, relax the repository's ruleset for it and restore the ruleset once the branch is gone.

### <a id="HeadroomReread"></a>Read the headroom again

Read the billing headroom as the standing ground does and require the same three free slots, since the cycle's `gauntlet` is about to spend two and the walk one.

### <a id="PromotionHousekeeping"></a>Promotion-day housekeeping

Before main moves, confirm the repository's Pages source: the README render from main root is the intended face.
Promotion deletes the public repository's untracked `index.html` and `.nojekyll` from main, and no landing page ships or moves to a side branch.
Close the public repository's stale open pull request, if one stands.

### <a id="DocimasyAct"></a>Docimasy

[Docimasy](#Docimasy), driven as `tt/rbvw-d.Docimasy.sh`, proves the standing dry [Regulus](#Regulus) without ever disclosing it.

In order, it gates the [Narthex](#Narthex): an anonymous read must answer 404, and the [Narthex](#Narthex) must hold no refs at all or exactly the preview of this same cut.
It asserts the freshness of the [Regulus](#Regulus) before any push, a scratch re-cut of the maintainer tree's shipped bytes being tree-equal to it, and drift sends the cycle back to laps.
It prints the preview push line, with the explicit [Narthex](#Narthex) URL, for you to type, and requires the [Narthex](#Narthex) tip to equal the [Regulus](#Regulus) tip once you report the push done.
It runs the credential preflight: it probes the [Payor Role](#rbsfr_payor) credential, then runs [Novate](#rbsgo_sederunt_novate) on the [Sederunt](#rbsfd_sederunt), which opens a browser sign-in.
It prompts before spending, since marshal zero blanks the regime and commits the blank state and the ladder costs about an hour and two cloud projects.
It runs marshal zero, the blank-template state the `gauntlet` demands, then the `gauntlet`, once per repair cycle.
It then grants the [Cachet](#Cachet), keyed to the [Regulus](#Regulus) tree hash, and hands off to [Ostend](#Ostend).
A red ladder returns the cycle to laps and grants no [Cachet](#Cachet).

<a id="UnattendedStretch"></a>You are attended through the preview push, the browser sign-in and the prompt, and the `gauntlet` then runs unattended.
Run the drive in a terminal multiplexer session so the `gauntlet` outlives the terminal, and take the sign-in fresh just before it, since the session's renewal needs a browser that no unattended run has.

### <a id="OstendAct"></a>Ostend

[Ostend](#Ostend), driven as `tt/rbvw-o.Ostend.sh`, conducts the once-per-cycle showing of the standing dry [Regulus](#Regulus), every assert performed by the tool and every push typed by you.

Be attended throughout.
In order, it requires a standing [Cachet](#Cachet) whose tree hash equals the [Regulus](#Regulus), and names [Docimasy](#Docimasy) as the remedy where there is none.
It re-asserts the ground: the [Narthex](#Narthex) still answers 404, its tip still equals the [Regulus](#Regulus) tip, and the [Regulus](#Regulus) is still fresh.
It shows the [Regulus](#Regulus) file list and waits, since no machine judgment substitutes for the maintainer reading what they are about to publish.
It conducts the [Kerygma](#Kerygma): it records the public main tip, prints the staging push line for you to type, and asserts afterward that main is unchanged and that `POSTULANT_LOCAL` stands at the [Regulus](#Regulus) tip.
It then conducts promotion: it prints the promotion line, which you type, and asserts afterward that main's tree hash equals the [Regulus](#Regulus) tip.
A refused fast-forward means main moved since the cut: stop, never force, and re-cut atop the moved base.
It closes by handing off to [Harbinger](#Harbinger) and reminding you of the deletes in closeout.

<a id="FreshCloneRecipe"></a>The promotion line assumes a local `POSTULANT_LOCAL` ref and is never typed from the [Regulus](#Regulus) directory.
Clone the public repository fresh, fetch its `POSTULANT_LOCAL` into a local branch of the same name, and then type the printed line with that clone as the repository:

```
git clone git@github.com:scaleinv/recipebottle.git «fresh clone»
git -C «fresh clone» fetch origin POSTULANT_LOCAL:POSTULANT_LOCAL
```

This recipe is one the practice rounds verify.

### <a id="HarbingerAct"></a>Harbinger

[Harbinger](#Harbinger), driven as `tt/rbvw-h.Harbinger.sh`, stands up the stranger [Rig](#Rig) against promoted public main, for the confirmation coldwalk.

It clones the public repository over HTTPS, the anonymous read and the exact face a real stranger lands on, and requires the clone to carry the README.
It severs the clone's origin, installs a pre-push refusal hook and cuts a throwaway walk branch, and the rig pushes nothing, ever.
It shows the dated findings-memo path, a sibling of the clone that survives the clone's discard, and the launch line and stranger prompt.
The coldwalk is launched by hand with you at the keyboard, and the findings memo is yours to review and commit afterward.

## The walk

<a id="GreenfieldWalk"></a>The greenfield manor walk runs in full against promoted public main, the default face a stranger lands on, in your own hands, from the onboarding entry through to a green `gauntlet` on a fresh payor substrate.
It is a multi-hour ceremony: run it fresh, not tired.

Because promotion precedes the walk in this release, the main-blob links are live during the walk and the stranger clones the default branch, so every divergence between the shipped docs and reality counts as a finding.
Record each one.
The cost of the order is that a finding which bears delivered bytes stands on the promoted face until its repair re-cut lands atop the moved base.

Dispose of the census as follows:

- A divergence in documents alone defaults to fast-follow disposition, or a targeted re-walk of the diverged steps alone.
- A repair that bears delivered bytes, measured as a changed candidate tree hash, re-opens the full gate: a fresh repair cycle of laps to dry and then the reveal, with a fresh walk and a fresh `gauntlet`.
- A repair touching only withheld paths re-opens the cut-local battery alone, never the walk or the `gauntlet`.
- Any re-walk reuses the standing fresh rig, and a second payor project is never levied for it, the billing-link quota being what it is.

## Closeout

<a id="Closeout"></a>Closeout follows a green walk with its census dispositioned.

### <a id="LivenessCheck"></a>Liveness

Link checking splits in two.
The structural check, taken before the push by the `pyx` anchor sweep, proves that every link would resolve.
The liveness check can be taken only after promotion, since the main-blob links go live at the merge and cannot be checked live earlier.
Confirm that the public README serves at its URL and that every anchor resolves against the live page.

### <a id="SpotCheck"></a>The spot-check

A thin check, since the walk has already proved the bytes: the repository front page, the github.io render, and a fresh default-branch clone taken a few minutes into the onboarding entry.

### <a id="ThreeDeletes"></a>The three deletes

Delete the public staging branch, as ceremony hygiene, then the private [Narthex](#Narthex) repository, then the [Regulus](#Regulus) directory.
Sweep by hand the timestamped siblings that [Sepose](#Sepose) left standing.

### <a id="ReleaseRecord"></a>Record the release

Record the release in `RBSPH-ProjectHistory.adoc`, the project-history Sheaf.
The release record, and the record of this release's exceptional order, are dated and belong there; this procedure carries no dated record.

## Glossary

<a id="Essai"></a>**Essai**

The [Hierophant's](#Hierophant) reversible repair lap: gates, assays, cut, proof and a stranger rig, with no remote act, repeated until a lap runs dry.

<a id="Docimasy"></a>**Docimasy**

The reveal's reversible proving act, run on the dry [Regulus](#Regulus): the [Narthex](#Narthex) gates, the freshness assert, the preview, the credential preflight and the `gauntlet`, granting the [Cachet](#Cachet) on green.

<a id="Ostend"></a>**Ostend**

The reveal's irreversible showing, run under a standing [Cachet](#Cachet): the re-asserted ground, the file-list review, the [Kerygma](#Kerygma) and promotion, every assert made by the tool and every push typed by the operator.

<a id="Harbinger"></a>**Harbinger**

The stranger [Rig](#Rig) stood up against promoted public main, for the confirmation coldwalk after a reveal.

<a id="Cachet"></a>**Cachet**

The durable verdict [Docimasy](#Docimasy) grants, standing beside the [Regulus](#Regulus) and keyed to its tree hash, attesting that the `gauntlet` ran green against exactly those bytes.
[Ostend](#Ostend) requires it before the [Kerygma](#Kerygma).
A re-cut [Regulus](#Regulus) carries none, and a [Seposed](#Sepose) one takes its verdict with it.

<a id="rbsfr_payor"></a>**Payor Role**

The role holding the billing account and the OAuth client, whose OAuth credential is the system's one standing secret.

<a id="rbsfd_sederunt"></a>**Sederunt**

The signed-in window a federated operator opens by a browser sign-in, and within which the operator dons a mantle for each operation.

<a id="rbsgo_sederunt_novate"></a>**Novate**

Extinguish any standing [Sederunt](#rbsfd_sederunt) and open a fresh full-window one by a browser sign-in.

<a id="rbsgo_depot_unmake"></a>**Unmake**

Destroy a depot's cloud project and everything in it, releasing its billing link.
