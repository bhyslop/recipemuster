# BPK Portage — how kit code crosses to a repository that receives it

This file is an **excursus**: held out of the standing session load and read on
demand, reached from the trigger line in `Tools/bpk/bpai0/bpais_bpk.md`. Read it
before you close a pace whose diff touches a file under a kit directory another
repository receives by parcel, before you slate any crossing, and when you mount
a receiving pace.

One disposition binds before this read, and the standing file carries it: a
received kit directory is never edited in place. A file under such a kit reaches
the receiving repository by a parcel and by no other road — never by a copy from
a peer clone, a paste, or an edit to the received copy, whatever the bytes would
be.

## Words

- **Forge**: the repository a kit is authored and tested in. Its **main
  checkout** is the clone's main worktree in git's own sense, never a worktree
  linked to it.
- **Consumer**: a repository that receives the kit. Its main checkout's settings
  file (`burc.env`) declares the kits it receives in `BURC_MANAGED_KITS`.
- **Parcel**: one tarball formed for one consumer, carrying exactly the kits that
  consumer declares, with its install engine at its root.
- **Tamga**: the parcel's identity, `vvmt_tamga.json` at the parcel's root and
  `.vvm/vvmt_tamga.json` in the consumer once installed. It names its number,
  the forge commit the parcel was formed from, the forge's endonym, the kit
  list and the target.
- **Parcel yard**: the directory beside the forge's main checkout, outside every
  clone, that the release door writes each parcel into.

## The two-pace law

A crossing is two paces, and they are separate slates.

The **forge-side pace** runs in the forge. It lands the change on the forge's
trunk and proves from its own billet what a parcel would carry: the kennel's
readiness converge then its walk, `tt/bkw-r.Rebed.sh` then `tt/bkw-d.Derby.sh`.
Its green means the parcel will form clean.

The **receiving pace** runs in the consumer and carries the whole movement from
the receiver's seat: it forms the parcel at the forge's main checkout, verifies
it, and installs it into its own billet. The receiving pace pulls; the forge
never pushes.

They are two slates because they write two sires and a pace holds one write
sire. The edge between them is authored: the receiving pace waits on the
forge-side pace.

## The edge rule

A forge-side close whose diff touches a file of a kit some consumer receives
names the receiving pace for each such consumer, or slates it. Every pace that
needs the change to have arrived waits on that receiving pace by an edge, never
by a sentence in its docket.

## The freeze

Where a consumer's release heat freezes files between two of its parcels, the
receiving pace's docket says whether the freeze binds it.

## The platform binding

A parcel carries the platform binaries of the station it was formed on, and a
clone on another platform fails at exec. Dispatch a receiving pace at a station
of the consumer's platform.

## What arrives by parcel

The families of a consumer's tree that arrive by parcel, and are therefore never
edited in place there, are read off two sources:

- the kit list the consumer's tamga carries, `.vvm/vvmt_tamga.json`, which the
  parcel's bundled engine writes under the consumer's tree at each install;
- the tackle's delivered rows, where the estate keeps a tackle.

A change to any of those files is a crossing, and an urgent one is still a
crossing: the two-pace shape is the short road already.

## The registry allocation

The release door commits the parcel's registry allocation on the forge's trunk
and pushes it before any tarball exists, so the number a parcel carries is
published before the parcel is applied anywhere. A push that fails refuses the
release; reconciling the forge's trunk is then the operator's.

## The receiving pace's docket

Copy this shape section by section into the receiving pace's docket, and fill
each `«...»` from the crossing at hand.

### Character

«One line: which kits cross from which forge, carrying which landed change.»
The pace forms the parcel at the forge's main checkout from its own session,
verifies it out of the extraction, and installs it through the parcel's bundled
engine, with the forced moves the new kit editions owe.

### The law

The forge-side pace «coronet», on whose landing this pace waits by edge.
This excursus, `Tools/bpk/bpai0/bpaie_portage.md`.
«Whether the consumer's release freeze binds this pace, and why.»

### The work

1. **The read list carries the forge's sire.** The pace is slated with the forge
   among its readable sires, so its kraal provisions the forge beside the
   consumer's billet.
2. **Form the parcel from this session at the forge's main checkout.** Drive the
   forge's portage survey, then its release door, at the forge's main checkout,
   handed the settings file of the consumer's main checkout — never a billet's,
   since the release names its target by that file's project root. Read the
   parcel's path and number off the door's own output.
3. **Where the fence grants neither that checkout nor the parcel yard**, compose
   each drive whole, one line to paste, for the operator's shell escape. Before
   the first drive, take an environment reading through the same escape: the
   bash kit's names (`BURC_`, `BURD_`, `BURE_`, `BURS_`, `BURV_`) beside the
   positive control `CLAUDECODE`, so the drive is known to run in the
   environment the reading shows.
4. **The operator's plain-terminal drive is the fallback only** where the escape
   does not carry the session's environment.
5. **Verify out of the extraction, never the forge's tree.** Extract the parcel
   into a directory of its own, outside every repository, and read its tamga
   there: it names the forge's endonym and the consumer's main checkout as its
   target, lists exactly the kits the consumer declares, and names the trunk
   position the drive was handed. Read each changed file out of the extraction
   and compare it with the forge's committed copy.
6. **Bank the parcel before the census.** The parcel's path, its number and every
   reading above go into an empty notch. A parcel that fails any check is spent:
   it is installed nowhere, and a fresh one is formed, since a number burns once.
7. **Derive the census of forced moves against the received copies.** Diff each
   received kit's new edition against the copy standing in the consumer, sorted
   by file class: renamed and retired functions against the consumer's own
   callers, settings fields added or made required, doors a kit no longer
   enrolls. Derive it; never copy it from a forge-side close, whose count is a
   floor. A standing context file a received kit newly carries is itself a forced
   move: the consumer's own `CLAUDE.md` gains its include line.
8. **One install commit.** Install through the parcel's bundled engine — its
   `bpi_install.sh`, handed the consumer's settings file — never a binary
   standing in the consumer. The one commit carries the kit directories, the
   tamga and the forced moves alone. Its message names the kit's home sire, the
   parcel's number and the forge commit it was formed from, and the paths it
   touched.
9. **The consumer's own proof doors follow** on the installed tree, driven, with
   an absent result read as absent.

### Cinched

- Kit directories the consumer receives are touched by the
  install alone.
- The parcel is formed at the forge's main checkout on its trunk and nowhere
  else.
«Any cinch of the crossing at hand.»

### Done when

The consumer's tamga names the parcel this pace verified, its kits stand at that
parcel's edition, the install commit carries the kits and the forced moves alone,
and the consumer's proof doors are green after the install.
