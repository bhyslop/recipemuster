# Bash Kennel (BKK) — the station handbook

This handbook carries two things the kit's README does not: the receiving station's road to the whistle, and how the kennel behaves in a repository with more than one worktree.
The README, `Tools/bkk/README.md`, is the kennel itself: its doors, its collars and its refusals.

## The road to the whistle

This kit is delivered as source.
The parcel carries `Tools/bkk/` whole, and the station you install it on builds the kennel itself, through its own whistle.

This is the road from an installed parcel to a whistle that answers.
It is four acts.
Walk them in order, from the repository root.

The parcel install writes kit directories and nothing else: it does not touch your moorings layer, so the launcher stub and the doors are yours to mint.
That is one act apiece, through the substrate's own emitters.

## What the kennel asks of the station

**A tackroom.** The kennel homes the pinned Rust toolchain and the cargo registry in a store beside the clone, apart from the station user's own rustup and cargo homes.
One store serves every clone standing beside it.

**The pinned channel answering from it.** The kennel states its channel on every cargo invocation rather than inheriting whatever the store defaults to, and proves it by asking that toolchain's own compiler for its release before anything runs.
A routine invocation verifies and refuses; it never downloads unasked.
The toolchain is laid by the kennel's rebed door, on your word.

**A committed repository.** Every kennel door reads the repository a collar stands in and refuses while anything is uncommitted — a source file, a test file, an untracked file alike.
This is what makes each verdict map to a position, and it fires during the kennel's own build, so the acts below end with a commit before the whistle is blown.

**Kits under `Tools/`.** The kennel's own election file names its roots as `Tools/bkk/...`, so `BURC_TOOLS_DIR=Tools` in your `burc.env`.

**A pin at the repository root.** The kennel pins its own channel for its own crate, but every other crate it launches is answered for by the pin standing at your root:

```toml
[toolchain]
channel = "1.90.0"
```

A crate with no pin above it is refused by name rather than built against whatever the store defaults to.
This bites at the first collar you launch, not at the whistle.

**A `clippy.toml` at the repository root.** A collar names the directory whose lint governance it wears, and the substrate's own suite collar names `.` — your root.
The lint list is yours, not the kit's; an empty one is a valid answer:

```toml
disallowed-methods = []
```

A muzzle directory holding no lint list is refused as a muzzle nothing wears.

**The substrate already seated.** Act 2 is driven through the substrate's own emitter door, `tt/buw-tt-cl.CreateLauncher.sh`; a station that has not yet minted it mints it the same way, from `Tools/buk/README.md`.
Act 3 drives two more of its creation doors, `buw-tt-cbl` and `buw-tt-cbo`, minted the same way.

**Every exergue a collar declares struck once first.** A collar's own `BKRR_STRIKE` field names the door that writes the generated file its `BKRR_EXERGUE` field points at, and a fresh seat's first derby refuses at the first collar whose exergue no door has struck yet — the refusal names that door.
Run it once, ahead of the first derby.

## Act 1 — Declare the tackroom

Add the tackroom to your station file — the one your `burc.env` names in `BURC_STATION_FILE`:

```bash
BURS_TACKROOM=../tackroom-buk
```

It is relative to the project root, as the log directory is; the station regime refuses an absolute value, naming the variable.
It must not be the station user's home directory, nor anything inside that user's `.rustup` or `.cargo` — keeping the kennel's store apart from the station user's own toolchain is the whole point of the redirection, and the kennel refuses a declaration that collapses it.

The pinned channel — the one named in `Tools/bkk/bk0/rust-toolchain.toml` — is laid into that store by the kennel's own door rather than by hand, so nothing is installed here.
Act 3 mints that door beside the others, and Act 4 runs it once, ahead of the whistle.

## Act 2 — Mint the launcher stub

The stub delegates the kit's doors to the kennel's workbench.
It goes in your moorings launcher directory, `«moorings»/buml_launchers/` — create that directory first if this is the first kit you have minted for.

```bash
tt/buw-tt-cl.CreateLauncher.sh Tools/bkk/bk0/bkw_workbench.sh bkw_workbench
```

That writes `«moorings»/buml_launchers/launcher.bkw_workbench.sh` — four lines, sourcing the substrate's launcher and handing it the workbench path.

## Act 3 — Mint the door roster

Thirteen doors, each written by the substrate's creation doors over the launcher stub Act 2 made.
A kennel door's file carries nothing of the repository it stands in, so the door these write is the door the kennel ships.

Ten take the plain form:

```bash
tt/buw-tt-cbl.CreateTabTargetBatchLogging.sh «moorings»/buml_launchers/launcher.bkw_workbench.sh \
  bkw-w.Whistle bkw-l.Lineup bkw-t.Tattoo bkw-h.Heel bkw-z.Muzzle \
  bkw-g.Gangline bkw-s.Scoop bkw-n.Delouse bkw-k.Kennelman bkw-r.Rebed
```

Three — Mush, Derby and Futurity — take the form whose command writes its own record:

```bash
tt/buw-tt-cbo.CreateTabTargetBatchOwnlog.sh «moorings»/buml_launchers/launcher.bkw_workbench.sh \
  bkw-m.Mush bkw-d.Derby bkw-f.Futurity
```

That form's configuration line says `buuc_ownlog`: each of the three consumes its tenant's streams and renders the record itself, and under that word the dispatch composes the three log names and creates no file, leaving the kennel to write all three members or none — without it the dispatch curates the log line by line and destroys the interleaving the record exists to hold.
A door minted in the plain form, its configuration line saying no word, still runs; it is simply a poorer record than the kennel writes.

The workbench routes whatever doors stand, so mint the ones you use; the whole thirteen is the ordinary choice — and rebed is not optional, since every refusal that finds the pinned toolchain missing names it.
The substrate's roll call, `tt/buw-tt-rc.RollCall.sh`, lists any kennel door you have not stood, and names in red any that a later kennel edition no longer enrolls.

Then ignore the kennel's build directory, if your repository does not already:

```
**/target/
```

The kennel builds into `Tools/bkk/bk0/target/`, and the door law reads untracked files exactly as it reads modified ones.
Without this line the first whistle refuses on the output of its own build — it names the directory, so the refusal is legible, but it is cheaper not to meet it.

Now commit the moorings, the doors and that ignore line.
The kennel's build refuses a dirty tree, so this commit is part of the road rather than housekeeping after it.

## Act 4 — Lay the toolchain, then blow the whistle

```bash
tt/bkw-r.Rebed.sh
```

It takes no argument.
It stands on the tackroom fence and lays each channel the kennel requires whose compiler does not answer, uninstalling it under the tackroom and installing it at the minimal profile with clippy and rustfmt, then proving it again; a channel that answers is left as it stands.
It then converges the kennel and performs every heel and strike the derby's preflight names, and it is safe to drive at any time.
This is the one door that downloads a toolchain, and it does so only when you run it; drive it again whenever a refusal names it.

```bash
tt/bkw-w.Whistle.sh
```

It takes no argument.
It stands on the tackroom fence, proves the pinned channel's compiler answers, builds the kennel binary where the seat has outrun it, and then reports the kennel's own making in three lines: the position it was built at, the channel its pin asked for, and the compiler that answered.

The first run compiles the kennel and takes a few minutes.
Later runs rebuild only when the source has moved.

A whistle that answers is the road walked.
What you can now do with it — the doors, declaring a collar, and the refusals you will meet — is the kit's README, `Tools/bkk/README.md`.

## Working across worktrees

A plain clone needs none of this section.
It is for a repository that carries linked worktrees — `git worktree add` — beside its [main worktree](#bkscl_main_worktree).

<a id="bkscl_main_worktree"></a>**main worktree**

The worktree a repository's clone stands in, holding the administrative directory every other worktree of that repository shares — the concept is git's, defined in git's worktree documentation, and nothing here restates it.
To the kennel a [main worktree](#bkscl_main_worktree) is where self-geography runs, and it is the delivered root of every [linked worktree](#bkscl_linked_worktree) added from it.

<a id="bkscl_linked_worktree"></a>**linked worktree**

A further worktree added to a repository beside its [main worktree](#bkscl_main_worktree), sharing that worktree's administrative directory — the concept is git's, defined in git's worktree documentation, and nothing here restates it.
To the kennel a [linked worktree](#bkscl_linked_worktree) is the tree a door derives a delivered root for: its [main worktree](#bkscl_main_worktree), whose built binaries the binary election may borrow.

### Where a door learns its geography

A kennel door asks git, from inside the repository it entered, where that worktree's own git directory stands and where the git directory its worktrees share stands.
Nothing is configured, and no file or variable states the answer.

- **A main worktree** — a plain clone's own working tree among them — answers the same directory twice. The door runs over itself: the tree it entered is the work tree, and its own build output is where the binaries stand.
- **A linked worktree** answers its own administrative directory and, as the shared one, its main worktree's `.git`. That main worktree is the linked worktree's **delivered root**: the tree whose built binaries the binary election may borrow.
- **A linked worktree whose main worktree does not stand** — removed, or a shared directory that is no main worktree's `.git`, as a bare repository's is — refuses, saying the geography does not derive and naming what git answered. It never falls back to running over itself.

The answer is read only where a delivered root is consumed.
Suites, the muzzle, lineup, gangline, scoop and the leash never take it.

### Two residences

In a linked worktree an app has two residences, and every verdict names both: the borrow candidate at the delivered root, and its own residence in the worktree, each joined from the collar's `BKRR_RESIDENCE` and `BKRR_BYNAME`.

- A reader borrows the delivered root's binary while the tree under its roots is the same in both, and runs its own build once the worktree's source has outrun it. An own build that is absent or outrun refuses, naming heel.
- A writer is pinned to the delivered root's binary, so a binary built in a linked worktree never writes to a live store.
- Heel writes the worktree's own residence and nothing else, and refuses a residence that resolves inside the delivered root. A linked worktree added inside its own main worktree is the shape that meets this refusal.

Every verdict says which geography it read — self, or dispatched with its delivered root — on the election line and in the proclamation.

### The Loosebox under a dispatch

On a plain station the Loosebox stands beside the worktree, as the README's glossary says.
A dispatched session's work billet is a linked worktree of its sire's standing clone, and the stile that opens the session hands it no root of its own.
A door the session runs from its own shell keeps its Loosebox on station ground, beneath the root the billet's settings name and keyed by the billet's directory name, as a plain worktree's is.
A door the engine runs on the session's behalf is handed that run's roots, its Loosebox among them, under the dispatch's scratch container.
When the billet is cleared, the stile and the muck remove its Loosebox on station ground beside the scratch container, which goes with the rest.

On every road a door reads the root from one variable and computes nothing, and an absent root refuses rather than falling back.
A session drives a billet's door as an ordinary door of that billet — by its relative path, or by asking the engine to run it — so the door's working directory is the billet, and the geography above derives there.
