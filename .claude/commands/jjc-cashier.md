Cashier a derelict JJ blotter lock-holder after a crash.

**WARNING**: This breaks a lock on a SHARED store. Normal ceremonies release their
lock automatically; a lock that is still flying usually means a station died
mid-ceremony — but it can also mean a writer is working right now, on another
machine.

Arguments: $ARGUMENTS (ignored)

## When to use

Use this when a JJ operation is refused with a held lock — for example:
- "lock-held" from a dispatch, a notch, or a wrap
- a ceremony that reports another station is mid-write, and stays that way

This is NOT the same lock as `/vvc-BREAK-LOCK`. That one guards commits in the
consumer repo (`refs/vvg/locks/vvx`); this one guards a JJ blotter — the
studbook — and is a wholly separate apparatus. Breaking the wrong one does not
help.

## Step 1: Sight the locks (read-only, always safe)

```bash
./tt/jjw-dc.SightLocks.sh
```

This reports, per store: who holds the lock (officium, station), how OLD the lock
is, and what operation the holder was running. It breaks nothing.

If it reports no lock held, say so and stop — there is nothing to cashier.

## Step 2: Show the operator the report and ask

Show the report verbatim. Do not summarize away the age or the warnings — they
are what the operator decides on.

Then put the decision to them plainly, including what it costs if they are wrong:

- If the holder is a live **writer**, breaking it costs that writer its ceremony
  and **nothing lands** on the store — the lease refuses its push. Recoverable:
  they just run it again.
- If the holder is a live **reader**, breaking it may let that reader **act on a
  stale image**, and nothing will catch it. This is the worse case.
- A lock less than a minute old is probably **live**, not crashed. The report
  warns about this.

Ask: is the holding station really dead?

**Wait for the operator to confirm.** Never run Step 3 unasked.

## Step 3: Cashier

```bash
./tt/jjw-dC.Cashier.sh
```

The door shows the report again and requires the operator to type `cashier` at
its own confirm gate — so the operator, not you, authorizes the break. If you are
not able to pass a typed confirmation through, hand the command to the operator to
run themselves rather than trying to route around the gate.

## Step 4: Report the result

On success, the door names whose lock was cleared. Report that verbatim — the
operator should know who they just dismissed.

If the break is refused (`lock-broken`), the lock changed between the report and
the break: someone else broke it, or a new holder staked it. Re-run Step 1 rather
than retrying blind — the lock now flying is not the lock the operator judged.
