# Remote fundus hosts: cargo must be on the non-interactive ssh PATH

*2026-06-05. Surfaced during heat ₣BV (rbk-10 uncontrolled-cygwin) while running
the `siege` suite on cerebro for the ₢BVAAG cross-platform requal. Filed toward
heat ₣A- (rbk-30-win-windows-remote-control), whose remit covers remote
test-host provisioning, so it surfaces when that heat is worked.*

## Symptom

Running a theurge suite over a plain non-interactive ssh —
`ssh <host> "cd <repo> && ./tt/rbw-ts.TestSuite.<x>.sh"` — dies immediately:

```
.../Tools/rbk/rbtd/rbte_engine.sh: line 102: cargo: command not found
rbte_suite ERROR: [rbte_suite] cargo build failed
```

theurge builds with `cargo` before running any suite; if `cargo` is not on PATH
the run never starts.

## Mechanism (the divergence)

rustup installs cargo at `~/.cargo/bin/cargo` and puts it on PATH by appending
`. "$HOME/.cargo/env"` to the user's shell init — `~/.profile` (login shells
only) and `~/.bashrc`. But Debian/Ubuntu's stock `~/.bashrc` returns early for
non-interactive shells, near the top:

```
case $- in
    *i*) ;;
      *) return;;
esac
```

rustup's line sits *after* that guard (line ~120 on cerebro). A non-interactive
`ssh host "cmd"` runs a non-login shell that *does* read `~/.bashrc` (the sshd
special case — which is why the guard exists) but hits the guard and returns
before reaching the cargo line. Result: the bare system PATH
(`/usr/local/sbin:...:/snap/bin`), with no `~/.cargo/bin`.

This is host provisioning, not project code. cygwin@rocket and wsl@rocket were
already set up to expose cargo to the non-interactive channel (cygwin's is the
"PATH visibility" item in ₣BV's paddock Known gaps); **cerebro was not** — it is
the one host where this bit.

## Fix applied on cerebro (2026-06-05)

Prepended to the TOP of `~/.bashrc`, *before* the non-interactive guard (backup
left at `~/.bashrc.bak-rbk`):

```
# rbk-cargo-path-noninteractive: put rustup cargo on PATH for non-interactive ssh.
# theurge builds over ssh; see Memos/memo-20260605-cerebro-noninteractive-cargo-path.md
. "$HOME/.cargo/env"
```

Verified: `ssh cerebro "command -v cargo"` → `/home/bhyslop/.cargo/bin/cargo`.

Chosen over `~/.ssh/environment` (the other robust option) because that needs a
global `PermitUserEnvironment yes` relaxation in `sshd_config` plus an sshd
reload — more invasive than a single-user `~/.bashrc` prepend.

## The durable contract (why this memo exists)

**Any host used as a remote theurge fundus must expose `~/.cargo/bin` to the
non-interactive ssh command channel.** Quickest check when adding or refreshing
a fundus host:

```
ssh <host> "command -v cargo"      # must print a path, not nothing
```

If it prints nothing, prepend the snippet above to that host's `~/.bashrc`,
before the interactive-shell guard.

## Pale framing

These fundus hosts are *ours* — editable at the source — so this is fixed
host-side (provision the host), not papered over with a tooling membrane in the
repo. A repo-side fallback (theurge self-locating cargo when PATH lacks it)
would only be warranted for genuinely uncontrolled *consumer* fundus hosts,
which this remote-test orchestration is not.
