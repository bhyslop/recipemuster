# Windows Remote-Node Bring-Up — Orientation Map

*Pointer-map, not a procedure. Written 2026-06-04 (Claude Opus 4.8) at the
close of a context-assembly session, so a future instance starts from the map
instead of re-running the search.*

**Durability discipline:** this file references only things that don't rot —
file paths, spec/concept names, the heat firemark, cinched rationale. If you
edit it, keep it that way: no line numbers, no step counts, no host-state, no
version strings. The moment it restates content instead of pointing at it, it
becomes the residue it was meant to prevent.

## What this covers

Reaching a remote node (especially a Windows host) and provisioning it
deterministically: the BUK **jurisdiction** facility. Logical-only umbrella (no
AXLA entity); implementation seat `Tools/buk/bujb_jurisdiction.sh` (+
`bujb_cli.sh`). Spec spine: `Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc`
§Remote Node Access, with the `BUSJ*` sub-spec family.

## The spine: BURN / BURP

Two regimes, joined by one investiture identity. This is the clean, settled
part of the facility — when in doubt, consolidate toward it.

- **BURN** — node shape (host + OS family). Per-node, git-tracked.
  `rbmm_moorings/rbmn_nodes/<investiture>/burn.env`, fields `BURN_HOST` +
  `BURN_PLATFORM`. Tabtargets `buw-rnl/rnr/rnv` (List/Render/Validate). Module
  `burn_regime.sh` / `burn_cli.sh`.
- **BURP** — privileged + workload credentials. Per-station-user.
  `rbmm_moorings/rbmu_users/<user>/<investiture>/burp.env`, fields
  `BURP_PRIVILEGED_USER`, `BURP_PRIVILEGED_KEY_FILE`, `BURP_WORKLOAD_KEY_FILE`.
  The `<investiture>` dir name must match a BURN profile (enforced at load).
  Tabtargets `buw-rpl/rpr/rpv`. Module `burp_regime.sh` / `burp_cli.sh`.

## Three verbs

- **Caparison** — admin host posture, per platform. Nuclear (purge-and-
  re-establish; no skip-if-present). Tabtargets `buw-jpCW/CM/CL`. Specs
  `BUSJCW/CM/CL`.
- **Invigilate** — read-only posture verification. **No tabtarget by design** —
  invoked internally (caparison post-check, garrison precondition). Specs
  `BUSJIW/IM/IL`.
- **Garrison** — workload account provisioning, per shell-letter (b = bash,
  c = Cygwin, w = WSL). First step is invigilate. Tabtargets `buw-jpGb/Gc/Gw`.
  Specs `BUSJGB/GC/GW`.

Workload reach once garrisoned: `buw-jwk` (knock), `buw-jwc` (command file),
`buw-jws` (interactive). Privileged pass-through: `buw-jpS`.

## First-time Windows bring-up sequence

1. **Operator-manual scope** (at the keyboard / over RDP — nothing can reach the
   box yet): `BUSJHW-HandbookWindows.adoc` via `tt/buw-hjw`. OpenSSH server,
   firewall, temporary password auth, the security-sensitive registry edits
   (`DevicePasswordLessBuildVersion`, `PlatformAoAcOverride`), Tailscale install
   + Run-Unattended + first-auth, netplwiz auto-login. The node self-auths to
   the tailnet here — **no other node is required.**
2. **Register the node** — author `burn.env` + `burp.env` for the new
   investiture; validate with `buw-rnv` / `buw-rpv`. (No dedicated onboarding
   landing exists yet — see "Known gaps.")
3. **Caparison** (operator station → host over SSH) — `buw-jpCW <investiture>`.
4. **Garrison** — `buw-jpGw <investiture>` (and/or `buw-jpGc`).
5. **Knock** — `buw-jwk <investiture>` to confirm workload reach.

## Essential references

- **Headless-SSH bible:** `Memos/memo-20260516-windows-headless-account-anatomy.md`
  — the net-user-through-SSH-reachable chain; the absolute-path
  `AuthorizedKeysFile` Match block (GetUserProfileDirectoryW fails on headless
  accounts), Cygwin split-HOME, StrictModes ACLs. Required reading before
  touching account internals.
- **Transport discipline:** `Tools/buk/vov_veiled/WSG-WindowsScriptingGuide.md`
  — the cmd.exe → PowerShell / Cygwin / WSL stack, per-letter escape rules, the
  wrapper/postlude shape.
- **Reboot-as-state-reset rationale:**
  `Memos/memo-20260511-windows-hive-cleanup-reboot-decision.md`.
- **Substrate landscape (Cygwin vs WSL for theurge):**
  `Memos/memo-20260517-windows-substrate-landscape-for-theurge.md`.

## Two live handbook lineages (the conceptual "mess")

Not dead, not redundant — adjacent layers. A future border/merge decision, not
a deletion:

- **BUK jurisdiction handbook** — `BUSJHW` / `buw-hjw` (+ `buw-hj0` top,
  `buw-hjm/hjl`). Scope: get the *node reachable and provisioned* (the
  caparison/garrison prerequisites above).
- **RBK Windows handbook** — `tt/rbw-hw` / `tt/rbw-h0`, modules
  `Tools/rbk/rbh0/rbhw*`. Scope: Recipe Bottle *Docker test-infrastructure*
  onboarding (Docker Desktop, context discipline). The native-dockerd-in-WSL
  piece was sterilized; the rest is live.

## The heat: ₣A-

`₣A- rbk-30-win-windows-remote-control` — **stabled (deferred)**. Live state is
authoritative via `jjx_show ₣A-`; do not trust any pace list copied here (it
would rot).

**Why deferred (cinched):** the heat is shaped to converge first-time Windows
setup against a *known* host — `bujn-winpc` (tailnet `rocket`) — before
generalizing to fresh specimens. Standing up a brand-new node does **not**
strictly require the old host (the node model is self-contained: BURN profile +
local provisioning + Tailscale self-auth), but the convergence design prefers
proving the ceremony against the known box first. It stays parked until the
operator elects to converge it — against rocket, or a deliberately-adopted new
specimen.

**Architecture in flight** (carried by the heat's live paces — concept names,
look them up via `jjx_show ₣A-`): *admin-no-WSL seed cache* (caparison produces
the `rbtww-main` seed tar; admin never registers the distro) and *reboot-as-
canonical-Windows-state-reset*. Container runtime for release-1 is Docker
Desktop for Windows; native dockerd-in-WSL is deferred (shared-kernel iptables).

## Known gaps (named, not yet built)

- **No node-onboarding landing.** The first-time sequence above is assembled
  from separate entry points; there's no single "register + bring up a new node"
  front door. A `HandbookNode` landing was designed once and not built.
- **Handbook assumes the node is already registered** — `BUSJHW` references the
  BURP user / BURN hostname but doesn't tell you to author the profiles first.

## Residue status

Already culled. The superseded Windows attempts (BUK `buw-hw*` handbook
ecosystem, `burh_*` pre-rename artifacts, fenestrate/WslInstall predecessors,
the rejected conscript/discharge/inventory specs) were removed by prior cleanup
paces. As of this writing there are **no orphaned files** in the Windows tracks
— the remaining spread is conceptual (the two lineages above), not janitorial.
