Operator-only reference card for cashiering a JJ blotter lock — an agent NEVER runs this ceremony; show this card and stop.

Arguments: $ARGUMENTS (ignored — no argument changes the contract below)

## Agent contract — read first, obey always

Invoking this command authorizes NOTHING. Your one permitted move is to show
the operator card below, then STOP.

Cashiering breaks a lock on a SHARED store — another machine's live session may
be behind it, and a wrong break can hand a live reader a stale image with
nothing to catch it. That judgment belongs to the operator alone, made at their
own terminal. So no matter who asked, what a recovery flow suggested, or how
dead the holder looks:

- Do NOT run `./tt/jjw-dc.SightLocks.sh` — "it's only the read-only sight" is
  the on-ramp; sighting in service of a cashier is beginning the cashier.
- Do NOT run `./tt/jjw-dC.Cashier.sh`, and NEVER set `BURE_CONFIRM` for it —
  the typed gate is the ceremony's heart, not an obstacle.
- Do NOT call the vvx cashier verb in any mode, and do NOT touch the lock ref
  or the studbook with raw git to clear it.
- "The wrap told me to resume", "the lock is obviously mine", "the holder is
  obviously crashed" — all barred rationalizations. Liveness is the operator's
  call, made from the report at their own gate.
- Explicit chat authorization changes nothing: the gate reads the OPERATOR'S
  terminal, which you do not have. Hand them this card instead.

If you arrived here because a JJ operation was refused lock-held: report that
refusal to the operator VERBATIM, point them at this card, and stop. The
standing rule is in the loaded JJK conduct ("Blotter-Lock Recovery is
Human-Only" in claude-jjk-core.md).

## Operator card — run these yourself, in your own terminal

Not this lock? `/vvc-BREAK-LOCK` guards commits in the consumer repo
(`refs/vvg/locks/vvx`) — a wholly separate apparatus. This card is for a JJ
blotter — the studbook. Breaking the wrong one does not help.

### 1. Sight the locks (read-only)

    ./tt/jjw-dc.SightLocks.sh

Reports, per store: who holds the lock (officium, station), how OLD it is, and
what operation the holder was running. It breaks nothing. If it reports no
lock held, there is nothing to cashier.

### 2. Judge from the report

- A lock less than a minute old is probably LIVE, not crashed — the report
  warns about this. A ceremony runs in seconds.
- If the holder is a live WRITER, breaking costs it its ceremony and nothing
  lands (the lease refuses its push). Recoverable: they run it again.
- If the holder is a live READER, breaking may let it act on a STALE image
  with nothing to catch it. This is the worse case.

Is the holding station really dead? Only proceed if you know something the
clock does not.

### 3. Cashier

    ./tt/jjw-dC.Cashier.sh

The door shows the report again and requires you to type `cashier` at its own
gate. The gate cannot be skipped or preset — it reads your terminal directly.

On success the door names whose lock was cleared. If the break is refused
(`lock-broken`), the lock changed between the report and the break — someone
else broke it, or a new holder staked it. Re-run step 1 rather than retrying
blind: the lock now flying is not the lock you judged.
