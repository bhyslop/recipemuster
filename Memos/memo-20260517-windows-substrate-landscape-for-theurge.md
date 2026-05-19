# Windows Substrate Landscape for Theurge

Date: 2026-05-17

## Why this memo exists

Recipe Bottle's gauntlet — and even the smallest fast-tier test suite —
runs through the theurge Rust binary, which spawns bash sub-processes
and embeds filesystem paths in their scripts. That makes theurge the
substrate-sensitive piece of the Recipe Bottle stack: it has to agree
with its surrounding shell about what a path looks like.

On a Windows host, there are two readily-available shell substrates that
*both* present a bash-ish interface — Cygwin and WSL2. They are not
interchangeable from theurge's perspective. This memo records what we
observed from each, without picking a winner. The cost ledger that
determines which substrate is right for a given operator includes
factors outside the substrate itself (multi-distro coexistence with
Docker Desktop's WSL machinery, account topology, garrison roadmap)
that this memo does not attempt to weigh.

The investigation was prompted by a first-time bring-up of Recipe Bottle
on a fresh Cygwin account (`brad@rocket`) where `tt/rbw-tP.QualifyPristine.sh`
aborted in `enrollment-validation` with a substrate-level path-mangling
failure. Both substrates were probed in the same session.

## The Cygwin substrate, today

### Observed failure mode

After installing rustup as `x86_64-pc-windows-gnu` and building theurge
cleanly under a Cygwin shell, the gauntlet aborted on its first fixture
(`enrollment-validation`) with `0 passed, 1 failed (1 total)` — but the
fast suite normally runs 47 cases in that fixture alone. The Rust
binary's reported trace directory was

```
Trace dir: /cygdrive/c/Users/<user>/projects/rbm_alpha_recipemuster/../temp-buk/temp-<stamp>\rbtd
```

— note the mixed `/` and `\` separators in a single path. The on-disk
state varied between runs:

- One run produced a *sibling* directory `temp-<stamp>rbtd` (no
  separator between the timestamp and `rbtd`), parallel to the expected
  `temp-<stamp>` rather than nested inside it.
- A subsequent run produced no `*rbtd*` directory anywhere visible to
  Cygwin — the `create_dir_all` call did not raise an error to the
  Rust binary, but the directory was not present.

`ls -b` confirmed the sibling-directory name contained no literal
backslash character; Windows had elided the `\` separator entirely
during path normalization of the mixed-slash input.

### Root cause shape

The Rust source is structurally correct: `main.rs` uses
`burd_temp.join("rbtd")` and `rbtdre_engine.rs` uses
`root_temp.join(case.name)`. There is no string-concatenation bug. The
failure occurs at a *substrate boundary*: a Windows-native binary
consuming Cygwin POSIX paths (`/cygdrive/c/...`) and combining them
with Windows-native separators via `PathBuf::join`. Windows path
normalization of the result is implementation-defined for these inputs,
which is why behavior varies between runs.

### Three boundary classes (scope estimate if revisited)

For a future attempt at making theurge work under Cygwin without
abandoning the Windows-native target, three boundary classes need
systematic handling:

1. **Env-var intake from Cygwin bash → Rust.** Single chokepoint:
   `rbtdb_read_dispatch_dir` in `Tools/rbk/rbtd/src/main.rs` reads
   `BURD_TEMP_DIR`, `BURD_OUTPUT_DIR`, etc. as POSIX strings. A
   conversion helper (e.g., `cygpath -w` equivalent) would convert
   incoming paths to Windows-native `PathBuf` at this single site.
2. **Script-embedded paths from Rust → Cygwin bash.** The chokepoint
   is `rbtdrf_run_bash` in `Tools/rbk/rbtd/src/rbtdrf_fast.rs` (called
   ~11 times in that file alone), but the *interpolation sites* are
   the callers that build the script string via
   `format!("source '{}'", path.display())`. Every such site needs to
   replace `path.display()` with a POSIX-form conversion. Discovery
   recipe: `grep -rn "source '{}'" Tools/rbk/rbtd/src/` plus adjacent
   `format!()` blocks. ~9-15 sites in `rbtdrf_fast.rs`; the
   `rbtdrc_crucible.rs` sibling may have more.
3. **Env vars on tabtarget spawn.** `Command::new(&tt)` sites in
   `rbtdrf_fast.rs`, `rbtdrf_handbook.rs`, `rbtdri_invocation.rs`
   inherit the parent's env. If intake conversion (boundary 1) is
   applied, env vars need re-conversion back to POSIX before any
   tabtarget spawn, since tabtargets re-enter `bud_dispatch` and read
   POSIX paths.

Windows-native tool spawns (`Command::new("git")`, `"curl"`,
`"docker"`) need no conversion — Windows-form paths are what those
tools natively want.

### Unknown tail

Even with all three boundaries handled, BUK-side bash itself may have
Cygwin assumptions that aren't yet visible. `${BASH_SOURCE[0]%/*}` slash
trimming, `realpath` semantics, `readlink -f` behavior, and similar
patterns could surface as the next blocker. The mechanical theurge fix
unlocks visibility into the next layer; whether that layer is clean is
empirically unknown.

## Cygwin substrate, revisited 2026-05-19

Pace ₢BOAAM (path-transmute) landed at commit `ea2d04b` and was
verified on `cygwin@rocket`. The path-transmutation code itself is
correct, but two BUK-side / theurge-invocation blockers surface
behind it. This section retires the "Unknown tail" prediction with
empirical content.

### Path-transmutation is correct but doesn't engage by default

`Tools/rbk/rbtd/src/rbtdrx_platform.rs:45` keys Cygwin detection on
`std::env::var("OSTYPE").as_deref() == Ok("cygwin")`. The module
docstring (line 42) asserts *"the launching bash exports
`OSTYPE=cygwin`"* — empirically false. Bash sets `OSTYPE` as a
non-exported shell variable; `env | grep ^OSTYPE=` from a Cygwin
session returns nothing, and the Windows-native `rbtd.exe` spawned
by a tabtarget sees no such env var. `rbtdrx_is_cygwin()` returns
false, so every intake/outflow site in the module operates as
identity. The mixed-separator failure described in "Observed
failure mode" above reproduces unchanged on the landed code.

Forcing `export OSTYPE` in the parent shell before invoking
`tt/rbtd-s.TestSuite.fast.sh` exercises the transmutation. Same
suite, two runs:

```
default:        Trace dir: /cygdrive/c/Users/.../temp-...\rbtd
                                      (mixed-slash, no rbtd/ created)
export OSTYPE:  Trace dir: C:\Users\cygwin\projects\...\temp-...\rbtd
                                      (clean native, rbtd/ exists)
```

`grep -rn OSTYPE Tools/buk Tools/rbk/rbtd` finds zero export sites
in the launcher chain. Either the BUK launcher needs to learn
`export OSTYPE`, or `rbtdrx_is_cygwin()` needs a detection signal
that does propagate to child processes (e.g., `cygpath --version`
succeeding, or shelling out to `uname -o` returning `Cygwin`).

### `Command::new("bash")` finds Windows WSL stub, not Cygwin bash

With OSTYPE forcibly exported, path-transmutation produces clean
native trace paths and the per-case directory is created. The
first case (`rbtdrf_ev_string_valid`) then fails with UTF-16LE
stderr from `wsl.exe`:

```
Windows Subsystem for Linux has no installed distributions.
```

`Tools/rbk/rbtd/src/rbtdrf_fast.rs:68` calls
`Command::new("bash").arg("-c").arg(script)`. From a Windows-
native process, `CreateProcessW` uses the standard search order:
application directory, current directory, *system directory
(`C:\Windows\System32`)*, Windows directory, then `PATH`.
System32 ships `bash.exe` — the WSL launcher stub installed by
default on Windows 10+. Cygwin's `C:\cygwin64\bin\bash.exe`
appears later via `PATH` and is never reached.

`cmd.exe /c where bash` from a Cygwin shell returns
`C:\cygwin64\bin\bash.exe` first because `where` performs a
`PATH`-only lookup; CreateProcessW's broader search is what trips
the Rust call. Reproduced with a 10-line standalone Rust binary
independent of theurge.

Fix path (out of scope for ₢BOAAM): resolve the bash absolute
path once at startup (e.g., via `cygpath -w /usr/bin/bash` →
`C:\cygwin64\bin\bash.exe`) and pass it to `Command::new`, rather
than relying on lookup order. Affects `rbtdrf_run_bash` and any
sibling site that spawns `bash` (or other Cygwin utilities whose
name collides with a System32 binary).

### Done condition for ₢BOAAM, literal vs effective

The pace's Done condition was *"`tt/rbtd-s.TestSuite.fast.sh`
reaches a verdict on Cygwin."* Literally: met — the binary
terminates with exit 1 and emits `0 passed, 1 failed (1 total)`.
Effectively: the suite exercises only 1 of 75 cases before the
bash-lookup blocker stops it, which is not a transmutation gap.
The transmutation code does what it claims when its detection
signal is present.

### rustup install prerequisites on Cygwin

Standing up a fresh `cygwin@rocket` → x86_64-pc-windows-gnu
rustup install. Three gotchas not in the rustup docs:

1. **No MinGW system install needed.** Rust 1.95.0's
   `x86_64-pc-windows-gnu` toolchain ships a self-contained
   linker. `cargo build` on theurge linked clean without
   `mingw64-x86_64-gcc-core` or any `setup-x86_64.exe -P`
   step. This eliminates the elevation barrier — the cygwin
   user does not need admin to install rust.

2. **`TMPDIR` must be a Windows-form path.** PATH on rocket
   resolves `curl` to `C:\WINDOWS\system32\curl.exe` (Schannel-
   backed Windows curl), not Cygwin's curl. rustup-init.sh's
   bootstrap downloader hands curl a Cygwin POSIX `/tmp/...`
   destination; Windows curl writes ~5KB then fails with
   `curl: (23) client returned ERROR on write` (curl error 23
   is "write failed"). Workaround:

   ```
   mkdir -p ~/rust-install-tmp
   export TMPDIR="$(cygpath -w ~/rust-install-tmp)"
   sh rustup-init.sh -y --default-toolchain stable \
       --profile minimal --no-modify-path
   ```

3. **Do not pass `--default-host`.** The rustup-init.sh wrapper
   detects Cygwin via uname and inserts its own
   `--default-host x86_64-pc-windows-gnu` before invoking the
   downloaded rustup-init.exe. Passing the flag again from the
   command line produces:

   ```
   error: the argument '--default-host <DEFAULT_HOST>' cannot
   be used multiple times
   ```

   Let the wrapper auto-detect; the installer line
   `info: setting default host triple to x86_64-pc-windows-gnu`
   confirms the correct host was chosen.

### SSH transport detail for cygwin@rocket

`memo-20260516-windows-headless-account-anatomy.md`'s
noninteractive forced-command for `cygwin@rocket` is
`bash --login -c "$SSH_ORIGINAL_COMMAND"`. Empirically, argv-mode
commands containing `$var` references arrive at the remote bash
without parameter expansion:

```
$ ssh cygwin@rocket 'echo $OSTYPE'
$OSTYPE                                  # literal, not 'cygwin'
$ printf '%s\n' 'echo $OSTYPE' | ssh cygwin@rocket bash -l
cygwin                                   # correct
```

Cause unconfirmed (the forced-command's shell quoting interacts
with how sshd reconstructs `SSH_ORIGINAL_COMMAND` through Windows
OpenSSH's argv-to-string join); the empirical pattern is
reliable. For noninteractive automation against `cygwin@rocket`,
use the stdin-pipe form. Argv-mode is acceptable for commands
that contain no `$` expansion and no operator chaining.

## The WSL substrate, today

### Observed result

A fresh Ubuntu-24.04 distro provisioned on the same Windows host, with
rustup installed and the repo cloned into ext4, ran the fast suite as
follows:

- Theurge built cleanly (`Finished dev profile target(s) in 4.27s`).
- `enrollment-validation`: **47/47 passed** — the exact fixture that
  could not complete a single case on Cygwin.
- `regime-validation`: 24/25 passed. The one failure
  (`rbtdrf_rv_rbrr_repo`) emitted
  `RBRR_CLOUD_PREFIX: RBRR_CLOUD_PREFIX must not be empty` — a
  config-gap in the clone-of-clone chain (Mac → fresh Cygwin checkout
  → WSL clone) that would surface identically on any never-configured
  checkout regardless of OS.

Substrate-attributable cases: 71/71 passed. No mixed-separator
artifacts; trace directories landed where the Rust binary said they
would.

### Setup prerequisites observed

- `wsl --install -d Ubuntu-24.04 --no-launch` — see operational traps
  below; `--no-launch` is load-bearing for remote invocation.
- `apt-get install -y build-essential` — rustup warns "no default
  linker (`cc`) was found in your PATH" after a minimal-profile
  install; theurge builds C-using crates (e.g., libc build script,
  typenum, zerocopy) and fails to link without `cc`.
- A station regime file at `../station-files/burs.env` — BUK's
  first-run gate. The error message enumerates the three required
  variables (`BURS_LOG_DIR`, `BURS_USER`, `BURS_TINCTURE`).

## Operational traps when driving WSL from another machine

These applied when driving the Windows host from a Mac through the
existing `tt/buw-jpS.PrivilegedSsh.sh` chain (Mac bash → ssh →
Windows OpenSSH → cmd.exe → PowerShell → wsl.exe → bash). They are
specific to that transit, not to WSL itself.

### `wsl --install` over SSH

A bare `wsl --install -d <distro>` from a privileged SSH session
fails with

```
The requested operation requires elevation.
This operation requires an interactive window station.
Error code: Wsl/0x800705b3
```

The elevation part is solved by being on the privileged SSH path. The
window-station part is the post-install distro launch, which wants a
console for the first-time user-creation prompt. **Fix**: append
`--no-launch`. The distro installs successfully and remains in
"Stopped" state with `root` as the only account.

### `wsl.exe` UTF-16LE output

`wsl -l -v` (and `wsl --status` and friends) emit UTF-16LE directly.
Through cmd-mode SSH transport, the interleaved null bytes terminate
C-string handling and the output appears empty. **Fix**: invoke
through a PowerShell wrapper (PowerShell handles UTF-16 natively as
its console encoding) or set `WSL_UTF8=1` in the environment before
invoking wsl.

### PowerShell 5.1 quote-stripping

```
powershell -Command 'wsl -d Ubuntu-24.04 -u root -- bash -lc "cd /path && tt/foo.sh"'
```

fails with

```
The token '&&' is not a valid statement separator in this version.
```

The cmd.exe → PowerShell argv shuffle strips the inner double quotes
around the `bash -lc` argument. PowerShell then receives the bash
command unquoted, parses `&&` as a PowerShell operator, and rejects it
(PS 5.1 has no `&&`; PS 7+ has it as a pipeline chain operator,
which still doesn't match the bash semantic we wanted). **Fix**: use
`wsl --cd <dir>` to set the working directory natively in WSL, then
invoke a single command without a `cd && ...` chain:

```
powershell -Command 'wsl -d <distro> -u root --cd /root/proj -- bash -lc tt/foo.sh'
```

### `/mnt/c` local-clone

The Windows filesystem is mounted at `/mnt/c/` inside WSL. Cygwin's
existing checkout at `/cygdrive/c/Users/<user>/projects/<repo>` is
therefore reachable from WSL as
`/mnt/c/Users/<user>/projects/<repo>`. Running tabtargets directly
from that mount surfaces FS-metadata problems (executable bits,
filename case, performance under high-fanout test fixtures).
**Workaround**:

```
wsl -d <distro> -u root -- git clone /mnt/c/Users/<user>/projects/<repo> /root/projects/<repo>
```

A local clone copies into ext4 cleanly, and tests run from the WSL
home directory rather than the Windows mount. The same trick lets you
copy single files across (e.g., `cp /mnt/c/.../station-files/burs.env
/root/.../station-files/burs.env`) without needing a separate
network transport.

## Live work captured in ₣BO

Two paces in heat ₣BO carry the substrate work forward:

- **₢BOAAM (cygwin-theurge-path-transmute)** — landed at commit
  `ea2d04b` on 2026-05-19, verified on `cygwin@rocket`. The
  path-transmutation code itself is correct, but two follow-on
  BUK-side / theurge-invocation blockers surface behind it (OSTYPE
  not exported by bash so the detection signal never reaches the
  Windows-native binary; `Command::new("bash")` resolves to
  System32's WSL stub before Cygwin's bash via CreateProcessW
  search order). Detail in the "Cygwin substrate, revisited
  2026-05-19" section above.
- **₢BOAAN (cygwin-wsl-noninteractive-ssh-personas)** — set up
  `cygwin@rocket` and `wsl@rocket` as parallel headless SSH accounts,
  each forced-command'd to one substrate. Independent of the
  substrate-viability question; useful for either substrate, since it
  collapses the multi-step transit chain in this memo's "Operational
  traps" section to a single `ssh wsl@rocket <cmd>` or
  `ssh cygwin@rocket <cmd>` invocation.

Both paces remain live. This memo records what we know, not which
substrate to invest in next.

## See also

- `Memos/memo-20260516-windows-headless-account-anatomy.md` — SSH
  account auth chain (recipe for both `cygwin@rocket` and
  `wsl@rocket` setup); the "Noninteractive forced-command variants"
  section covers the noninteractive personas and the WSL per-user
  registration finding (`wsl --install` needs admin; `wsl --import`
  does not)
- `Memos/memo-20260508-windows-transport-experiments.md` and the
  `memo-2026051*-windows-transport-*` series — broader empirical
  record of cmd.exe / PowerShell / wsl.exe transport quoting; the
  trap in this memo's PowerShell-quote-stripping section is one
  instance of that family
- `Tools/buk/vov_veiled/WSG-WindowsScriptingGuide.md` — Windows
  scripting guide; the codified body-authoring rules for transit
  through cmd.exe → wsl.exe / cygwin / PowerShell stacks
- `Tools/rbk/rbtd/src/main.rs` and `Tools/rbk/rbtd/src/rbtdrf_fast.rs`
  — chokepoints for the three boundary classes in the Cygwin section
- `Tools/buk/bujb_jurisdiction.sh` — `bujb_privileged_ssh` (the
  underpinning of `tt/buw-jpS.PrivilegedSsh.sh`)
