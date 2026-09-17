# Bash Kennel Master (BKK) — the receiving station's road to the whistle

This kit is delivered as source. The parcel carries `Tools/bkk/` whole, and the
station you install it on builds the kennel itself, through its own whistle.

This is the road from an installed parcel to a whistle that answers. It is four
acts. Walk them in order, from the repository root.

The parcel install writes kit directories and nothing else: it does not touch
your moorings layer, so the launcher stub and the doors are yours to mint. That
is one act apiece, through the substrate's own emitters.

---

## What the kennel asks of the station

**A tackroom.** The kennel homes the pinned Rust toolchain and the cargo
registry in a station-shared store, apart from the station user's own rustup and
cargo homes. One store serves every clone on the station.

**The pinned channel present in it.** The kennel states its channel on every
cargo invocation rather than inheriting whatever the store defaults to. A
routine invocation verifies and refuses; it never downloads unasked.

**A committed repository.** Every kennel door reads the repository a collar
stands in and refuses while anything is uncommitted — a source file, a test
file, an untracked file alike. This is what makes each verdict map to a
position, and it fires during the kennel's own build, so the acts below end with
a commit before the whistle is blown.

**Kits under `Tools/`.** The kennel's own election file names its roots as
`Tools/bkk/...`, so `BURC_TOOLS_DIR=Tools` in your `burc.env`.

**A pin at the repository root.** The kennel pins its own channel for its own
crate, but every other crate it launches is answered for by the pin standing at
your root:

```toml
[toolchain]
channel = "1.90.0"
```

A crate with no pin above it is refused by name rather than built against
whatever the store defaults to. This bites at the first collar you launch, not
at the whistle.

**A `clippy.toml` at the repository root.** A collar names the directory whose
lint governance it wears, and the substrate's own suite collar names `.` — your
root. The lint list is yours, not the kit's; an empty one is a valid answer:

```toml
disallowed-methods = []
```

A muzzle directory holding no lint list is refused as a muzzle nothing wears.

**The substrate already seated.** Acts 2 and 3 are driven through the
substrate's own emitter doors, `tt/buw-tt-cl.CreateLauncher.sh` and
`tt/buw-tt-cbl.CreateTabTargetBatchLogging.sh`. A station that has not yet
minted those mints them the same way, from `Tools/buk/README.md`.

**Every exergue a collar declares struck once first.** A collar's own
`BKRR_STRIKE` field names the door that writes the generated file its
`BKRR_EXERGUE` field points at, and a fresh seat's first derby refuses at the
first collar whose exergue no door has struck yet — the refusal names that
door. Run it once, ahead of the first derby.

---

## Act 1 — Declare the tackroom

Add the tackroom to your station file — the one your `burc.env` names in
`BURC_STATION_FILE`:

```bash
BURS_TACKROOM=«/absolute/path/to/tackroom»
```

It may be absolute, or relative to the project root as the log directory is.
It must not be the station user's home directory, nor anything inside that
user's `.rustup` or `.cargo` — keeping the kennel's store apart from the
station user's own toolchain is the whole point of the redirection, and the
kennel refuses a declaration that collapses it.

Install the pinned channel into that store. The channel is the one named in
`Tools/bkk/bk0/rust-toolchain.toml`:

```bash
RUSTUP_HOME=«tackroom»/rustup rustup toolchain install «channel» \
  --profile minimal --component clippy --component rustfmt
```

You may skip this and let the kennel refuse instead: its refusal prints this
command with your own paths already filled in.

---

## Act 2 — Mint the launcher stub

The stub delegates the kit's tabtargets to the kennel's workbench. It goes in
your moorings launcher directory, `«moorings»/rbml_launchers/` — create that
directory first if this is the first kit you have minted for.

```bash
tt/buw-tt-cl.CreateLauncher.sh Tools/bkk/bk0/bkw_workbench.sh bkw_workbench
```

That writes `«moorings»/rbml_launchers/launcher.bkw_workbench.sh` — four lines,
sourcing the substrate's launcher and handing it the workbench path.

---

## Act 3 — Mint the door roster

Ten doors, one act:

```bash
tt/buw-tt-cbl.CreateTabTargetBatchLogging.sh \
  «moorings»/rbml_launchers/launcher.bkw_workbench.sh \
  bkw-w.Whistle bkw-m.Mush bkw-d.Derby bkw-l.Lineup bkw-t.Tattoo \
  bkw-h.Heel bkw-z.Muzzle bkw-g.Gangline bkw-s.Scoop bkw-k.Kennelman
```

Each door written is three lines: the shebang, the launcher basename, and the
exec line into your `tt/z-launcher.sh` trampoline. The exec line is
byte-identical in every tabtarget the emitter writes.

Then add one line to the mush door alone, above its exec line:

```bash
export BURD_AMANUENSIS=1
```

Mush consumes its tenant's streams and renders the record itself. Under that
line the dispatch composes the three log names and creates no file, leaving the
kennel to write all three members or none. Without it the dispatch curates the
log line by line and destroys the interleaving the record exists to hold. No
other door needs it, and no emitter writes it.

Then ignore the kennel's build directory, if your repository does not already:

```
**/target/
```

The kennel builds into `Tools/bkk/bk0/target/`, and the door law reads untracked
files exactly as it reads modified ones. Without this line the first whistle
refuses on the output of its own build — it names the directory, so the refusal
is legible, but it is cheaper not to meet it.

Now commit the moorings, the doors and that ignore line. The kennel's build
refuses a dirty tree, so this commit is part of the road rather than
housekeeping after it.

---

## Act 4 — Blow the whistle

```bash
tt/bkw-w.Whistle.sh
```

It takes no argument. It stands on the tackroom fence, verifies the pinned
channel is present, builds the kennel binary where the seat has outrun it, and
then reports the kennel's own making in three lines: the position it was built
at, the channel its pin asked for, and the compiler that answered.

The first run compiles the kennel and takes a few minutes. Later runs rebuild
only when the source has moved.

A whistle that answers is the road walked. Everything below is what you can now
do with it.

---

## The doors

Each door wears its own tabtarget and takes a collar as its argument. No door is
reached through another.

| Door | Argument | What it does |
|---|---|---|
| `bkw-w.Whistle` | none | Report the kennel's own making, building it first where the seat has outrun it |
| `bkw-m.Mush` | collar | Launch what one collar names — a suite under its declared runner, an app under the binary election |
| `bkw-d.Derby` | none | Run every suite collar the walk finds, sequentially, stopping at the first red course |
| `bkw-l.Lineup` | collar | List a suite's hurdles as its runner names them, running nothing |
| `bkw-t.Tattoo` | collar | Proclaim one collar and report every conformance finding, changing nothing on disk |
| `bkw-h.Heel` | collar | Build a collar's launchable current through the leash and install it at its residence |
| `bkw-z.Muzzle` | none | Lint every collar the walk finds at the muzzle directory it declares |
| `bkw-g.Gangline` | collar | Re-derive one collar's lock — the one act in which the leash lifts the lock flag |
| `bkw-s.Scoop` | none | Remove every collar's target directory, on your word alone |
| `bkw-k.Kennelman` | filter | Run this kit's own hurdles outside the kennel, so a broken kennel can still be told it is broken |

A collar is a declaration standing beside a crate, one per launchable, named by
the directory it sits in. The parcel carries one for the substrate's own suite,
`bun_suite`, so the shortest proof that the kennel works on your station is:

```bash
tt/bkw-t.Tattoo.sh bun_suite
tt/bkw-m.Mush.sh bun_suite
```

The first reads the collar and reports; the second runs it.

---

## Refusals you will meet

Every kennel refusal names what it found and the act that answers it. Three are
worth knowing before you meet them:

**The repository carries uncommitted changes.** Commit, then drive the door
again. This is the house rule that no test runs without a commit, so every
record maps to a position.

**`BURS_TACKROOM` is unset.** Act 1. The kennel will not fall back to a
per-clone copy of the toolchain.

**The pinned channel is absent from the tackroom.** The refusal prints the
`rustup toolchain install` line with your own paths in it. Run it deliberately;
the kennel never downloads a toolchain on your behalf.

---

## Where the rest is written

The kennel's design lives with the source, not here. This file is the road onto
a station and stops where the whistle answers.
