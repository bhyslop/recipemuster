# Configuring a MacBook as an always-awake test host

Groundwork for the durable Windows/macOS always-on test-host procedure. Written from a
live investigation on `brads-macbook-air` (Apple Silicon MacBook Air, Darwin 25.5.0) on
2026-07-12, during the Ann-home-network work. **Revised the same day:** the original §4
("recalled from training, unverified") and its §5 confirmation queries were resolved by
three parallel web-research passes on 2026-07-12; §4 is now the research-backed
architecture and §5 records the per-claim verdicts with sources.

**Three evidence tiers, and the split is load-bearing.**

- *Measured* (§2–§3): observed on the Air itself — trust.
- *Researched* (§4–§5): confirmed against current Apple documentation and 2026-era
  firsthand reports, cited inline. One live check (the `powerd` display-on assertion, §4.1)
  ran on the operator's workstation Mac — same Darwin 25.5, but **not the Air**.
- *Undemonstrated*: nothing below has yet survived the §6 protocol on the Air. **Do not
  promote §4 into a spec until §6 passes on the real hardware** — especially the power-cut
  cycle (§6 step 4), which no public source has bench-tested in exactly our sequence.

Per project doctrine: this memo is provenance, not authority. Once the procedure is
verified and exercised, it needs a spec home — the current candidate is the mews aspirant
sheaf's *node availability posture* fork (`Tools/jjk/vov_veiled/JJSAM-mews.adoc`).

---

## 1. The problem, stated precisely

A laptop is *not* a server, and the default macOS posture actively fights always-on use. We
want: **lid closed or open, on AC, reachable over SSH/Tailscale indefinitely, and
self-recovering after a power cut.** A stock MacBook satisfies none of that.

## 2. Measured baseline (fact — observed on the Air, 2026-07-12)

`pmset -g` on the machine, on AC power, battery 100%, **lid closed**
(`AppleClamshellState = Yes`):

| Setting | Value | Consequence |
|---|---|---|
| `sleep` | `1` | system sleeps after **1 minute** idle |
| `disablesleep` | *(absent)* | lid closure **will** sleep it |
| `displaysleep` | `0` (AC) | display never sleeps — a system-sleep veto while the lid is open (§4.1) |
| `standby` | `1` | drops to deep standby eventually |
| `hibernatemode` | `3` | RAM + disk image |
| `powernap` | `1` | periodic wake |
| `womp` | `1` (AC) / `0` (battery) | wake-on-magic-packet armed on AC only — but see §4.3: inert without a sleep proxy |
| `tcpkeepalive` | `1` | keeps some TCP state during sleep |
| `ttyskeepawake` | `1` | **an active TTY/SSH session keeps it awake** |
| `autorestart` | *(absent)* | consistent with §4.2: the setting is dead on Apple Silicon laptops |
| assertions | `PreventSystemSleep 0`, `PreventUserIdleSystemSleep 0` | nothing holding it awake (lid was closed; contrast §4.1 lid-open) |

**Demonstrated failure:** while an SSH session was open, the host was reachable. The session
was closed; roughly one minute later `ssh` returned `Operation timed out`. Tailscale reported
`offline, last seen 4m ago`.

## 3. Three traps that make naive verification lie to you (fact — all observed)

**3.1 `ttyskeepawake` — the observer keeps the patient alive.** Any check you run *over SSH*
is itself preventing the sleep you are trying to detect. **A host that is reachable while you
are logged into it tells you nothing.** Every verification must be performed from a *second*
machine, after disconnecting, having waited longer than the idle timer.

**3.2 ICMP is not a liveness test for a Mac.** While the host was verifiably asleep (Tailscale
offline, SSH dead), it still answered pings:

```
ping 192.168.86.27  ->  0% packet loss, 160-238 ms      # asleep
                        (its awake RTT on the same link was ~10 ms)
```

The Wi-Fi chip services ICMP and ARP in low-power offload while the OS is down. **A monitoring
check built on `ping` will report a sleeping host as healthy, indefinitely.** The inflated RTT
(≈20× normal) is the only tell. Probe a **service** — TCP 22, or the Tailscale daemon's own
status — never ICMP. *(Research corroboration: community reports of sleeping Macs answering
pings with a NIC-level proxy, TTL 32, while SSH/VNC are dead —
https://forums.macrumors.com/threads/wake-for-network-access-not-working.2400512/.)*

**3.3 Tailscale cannot wake a sleeping host.** It is a userspace tunnel, not a magic packet.
When the host sleeps, the tailnet node simply goes offline and the connection attempt fails
with nothing to trigger a wake. Any "wake-on-demand" plan that routes through Tailscale is
void.

## 4. Research-backed procedure (confirmed 2026-07-12; NOT yet demonstrated on the Air)

The architecture that survived research is simpler than the draft it replaces:

> **Never sleep on AC. No wake-from-sleep path at all. Treat battery-drain-to-death during
> an outage as the designed recovery path, ended by the firmware's power-connect auto-boot.**
> The host has exactly two legitimate states: *running*, or *dead and self-recovering*.

### 4.1 Keep it awake — two layered mechanisms

**Lid open on AC is sufficient by itself.** While the display is on, `powerd` holds a
first-class `PreventUserIdleSystemSleep` assertion (named "Prevent sleep while display is
on") — verified live via `pmset -g assertions` on a Darwin 25.5 Mac (the workstation, not
the Air). While that assertion stands, the machine cannot idle-sleep regardless of the
`sleep` timer; the Air's AC default `displaysleep 0` (§2) already keeps the display on.
(Corroboration: https://sixcolors.com/post/2026/05/keeping-and-losing-track-of-mac-sleep-settings/;
Apple's own AC toggle is phrased "Prevent automatic sleeping *when the display is off*" —
display on implies no automatic system sleep.) Set brightness to minimum — fanless Air,
panel wear. Cheap insurance for any display-dark edge case:

```sh
sudo pmset -c sleep 0
```

**`disablesleep` as the lid-close guard.** Confirmed still working on Apple Silicon under
macOS 26.x by multiple independent 2026 firsthand reports — it sets the kernel
`SleepDisabled` veto, which survives lid close, even on battery:

```sh
sudo pmset -a disablesleep 1        # verify: pmset -g | grep SleepDisabled
```

Evidence: Sleepless (firsthand on macOS 26.3, https://github.com/Aboudjem/Sleepless);
Lidless (verification spike on real Apple Silicon, https://github.com/nghialuong/Lidless).
Caveats, all real: **Apple-undocumented** — `disablesleep` appears nowhere in the macOS
26.5 `man pmset`, so Apple may change it without notice; **reset by reboot** on Apple
Silicon — re-assert at boot via a LaunchDaemon; historical perturbation on charger
plug/unplug cycles (the Amphetamine "Power Protect" saga); may leave the internal panel
lit under a closed lid (heat, on a fanless Air).

**Refuted alternatives** (do not reach for these):

- `caffeinate` in any flag combination does **not** survive lid close — its assertions veto
  *idle* sleep only (man page; every keep-awake project README agrees, e.g.
  https://github.com/newmarcel/KeepingYouAwake: "only prevent sleep on … portable Macs with
  an open lid").
- Standby/hibernate *tuning* is not viable on Apple Silicon laptops: the man page still
  defines `standbydelaylow/high` etc., but they are absent or ignored on laptop SMCs —
  hibernation timing is OS-managed. Never-sleep is the only strategy. Optional harmless
  belt-and-suspenders: `sudo pmset -a standby 0 hibernatemode 0`.

**Clamshell alternative** (if the lid must be closed and `disablesleep` is distrusted):
AC + HDMI dummy plug enables closed-display operation on Apple Silicon — empirical
2025–2026 reports (https://travis.media/blog/running-openclaw-headless-mac/, M1;
Macworld May 2026 recommends dummy plugs by name). Apple's sanctioned clamshell recipe
also lists external keyboard/mouse, but those exist to *wake and control* the machine
locally, not as a sleep veto — an SSH-driven host does not need them while awake.

### 4.2 Survive a power cut — firmware auto-boot, not `autorestart`

- **`autorestart` is dead on Apple Silicon laptops.** It does not appear in laptop
  `pmset -g` output at all (and §2 confirms it absent on the Air); Tahoe-era desktop
  reports show even desktops ignoring it. Do not use it; its absence is not a
  misconfiguration.
- **The real mechanism is firmware:** Apple documents that an Apple Silicon MacBook
  automatically powers on when the lid is opened or power is connected
  (https://support.apple.com/en-us/120622). The only knob is the NVRAM `BootPreference`
  variable (macOS 15+), which can *disable* the behavior — so ensure it is **unset**:

  ```sh
  nvram -p | grep BootPreference    # must return nothing
  sudo nvram -d BootPreference      # clears it if present
  ```

- **The designed outage sequence:** AC cut → runs on battery (`disablesleep` keeps it awake
  and draining — here that is *desirable*) → battery dies → AC returns → the re-energized
  charger presents a power-connect edge → firmware self-boots once the flat battery reaches
  a boot threshold. Expect **minutes to a couple of hours** of recovery latency while it
  recharges to that threshold (https://discussions.apple.com/thread/256030745). The final
  link — that wall-power restoration through an always-attached charger reads as a
  "connect" edge — is inference; no public source bench-tested exactly this sequence.
  **§6 step 4 exists to close it.**
- **Edge-trigger caveat:** the auto-boot is edge-based, not level-based. A machine *shut
  down* while AC stays continuously present does **not** boot itself
  (https://forums.macrumors.com/threads/charging-cable-turns-on-macbook-air-m1-automatically-how-to-disable-this.2334033/).
  Operational rule: never `shutdown` this host remotely — reboot, or use `fdesetup
  authrestart` (§4.4).

### 4.3 No wake path — by design

Confirmed: Wi-Fi "Wake for network access" requires a **Bonjour Sleep Proxy** — an Apple
802.11n-class device (Apple TV, HomePod, AirPort/Time Capsule) on the LAN
(https://support.apple.com/guide/mac-help/share-your-mac-resources-when-its-in-sleep-mh27905/mac).
The Ann house has none, so `womp 1` is dead weight there. Wired magic-packet WoL is a
separate, proxy-free path but needs Ethernet the Air lacks, and USB-C/Thunderbolt dock
adapters generally do not forward WoL in sleep. Design consequence: **never plan to wake
this host; prevent it from sleeping (§4.1).**

### 4.4 Reboot gates

- **FileVault — macOS 26 changed the calculus.** Pre-boot is no longer an absolute wall:
  on Apple Silicon with macOS 26+, if Remote Login is on and the network qualifies
  (open/unauthenticated Ethernet, or a previously-joined open or **WPA2-PSK** Wi-Fi
  network), the pre-boot environment runs a minimal SSH server and FileVault **can be
  unlocked over SSH** — password auth only (keys do not work pre-boot), LAN-level only
  (no Tailscale pre-boot)
  (https://support.apple.com/guide/deployment/intro-to-filevault-dep82064ec40/web;
  demonstrated: https://derflounder.wordpress.com/2025/10/11/unlocking-filevault-via-ssh-on-macos-tahoe/).
  For *planned* reboots, `sudo fdesetup authrestart` (Secure Enclave-backed, strictly
  one-shot; confirm capability with `fdesetup supportsauthrestart`) skips the gate
  entirely. But **fully zero-touch recovery still requires FileVault off**: then a reboot
  reaches the loginwindow with `sshd` (a LaunchDaemon) already up, no human anywhere.
- **Auto-login** requires FileVault off and a non-iCloud account password
  (https://support.apple.com/en-us/102316), and is needed only for user-*session* services
  — GUI apps and LaunchAgents. With daemon-variant Tailscale (next bullet) an SSH-only
  host may not need it.
- **Tailscale — the original claim was wrong.** *Neither* GUI variant survives to
  pre-login: the standalone/System Extension build ("macsys") and the App Store build
  **both require a logged-in user session**
  (https://tailscale.com/docs/concepts/macos-variants, validated 2026-01;
  https://tailscale.com/docs/how-to/run-unattended: "On macOS, there's no ability to run
  as the system just yet"). The only variant that runs as a true system daemon before
  login is **open-source `tailscaled`** (`sudo tailscaled install-system-daemon`, or the
  Homebrew *formula* — not the cask — plus `sudo brew services start tailscale`). A GUI
  variant and `tailscaled` cannot run simultaneously. For this host: run
  `tailscaled`-as-daemon, or accept auto-login + the GUI standalone build.
- **Radio choice** (unchanged): the most reliable SSID available, not the convenient one —
  in the Ann house, the AT&T gateway's own Wi-Fi at 159–186 Mbps, not the Google mesh at
  52–78. Note the WPA2-PSK constraint above if FileVault stays on.

## 5. Claim verdicts (research pass, 2026-07-12)

The §5 table of the original memo, resolved. Version caveat stands: evidence is pinned to
Apple Silicon + macOS 26.x where possible; stale-era sources are flagged in the agents'
detail (not restated here).

| # | Claim (as originally posed) | Verdict | Key evidence |
|---|---|---|---|
| C1 | `disablesleep 1` prevents lid-closed sleep on Apple Silicon, macOS 26 | **Confirmed**, with caveats: undocumented, reboot-reset, charger-cycle perturbation history | Sleepless (firsthand 26.3), Lidless spike; absent from `man pmset` 26.5 |
| C2 | `autorestart` honored on Apple Silicon | **Refuted** for laptops (setting absent) — but **substituted**: Apple-documented firmware auto-boot on lid-open/power-connect; drain-then-recharge self-boot is the recovery path | https://support.apple.com/en-us/120622; MacRumors Tahoe-era desktop failure reports |
| C3 | `hibernatemode`/`standby` mean what they meant on Intel | **Vestigial** on Apple Silicon laptops: defined in man page, timing knobs absent/ignored, OS-managed | man pmset; Monterey-era `pmset -g` dumps (stale — re-check with `pmset -g cap`, §6) |
| C4 | Wi-Fi wake needs a Bonjour Sleep Proxy | **Confirmed** — no Apple proxy device on the LAN ⇒ no Wi-Fi wake, period | https://support.apple.com/guide/mac-help/share-your-mac-resources-when-its-in-sleep-mh27905/mac |
| C5 | FileVault pre-boot blocks networking/SSH until physically unlocked | **Superseded by macOS 26**: pre-boot SSH unlock on Apple Silicon (password-only, qualifying LAN); `authrestart` confirmed for planned reboots (one-shot) | Apple deployment guide dep82064ec40; Der Flounder 2025-10 |
| C6 | Tailscale standalone ("macsys") runs as a system daemon surviving logout/reboot | **Refuted as stated** — only open-source `tailscaled` runs pre-login; both GUI variants need a user session | https://tailscale.com/docs/concepts/macos-variants; docs/how-to/run-unattended |
| C7 | `caffeinate` does not prevent lid-closed sleep | **Confirmed** (caffeinate is idle-sleep-only) | man caffeinate; KeepingYouAwake README |
| C8 | External display / HDMI dummy plug enables clamshell operation | **Confirmed** — dummy plugs work on Apple Silicon (empirical grade); Apple's kbd/mouse requirement is for local wake/control, not a sleep veto | travis.media M1 report; Macworld 2026-05 |
| C9 | *(new — operator observation)* lid open on AC never sleeps | **Confirmed at mechanism level**: `powerd` holds `PreventUserIdleSystemSleep` while the display is on (live-checked on Darwin 25.5 workstation) | `pmset -g assertions`; Six Colors 2026-05 |

**Folklore flag, recorded so the next researcher isn't spooked:** a widely-scraped uncited
blog claim ("since Ventura a hardware lid sensor forces sleep; neither pmset nor caffeinate
bypasses it") contaminates search-engine summaries. It cites no tests and is contradicted
by the firsthand 2026 reports above. The kernel of truth: *assertion-based tools* (old
One Switch, pre-5.3 Amphetamine) genuinely broke on M2/Ventura until updated — tool-level
breakage misread as flag-level.

## 6. Verification protocol (unchanged in spirit; the gate before spec promotion)

A procedure is not done until it survives this. Run **from a second machine**:

0. Capability census on the Air first: `pmset -g cap` (which knobs this SMC actually has);
   after setting §4.1, confirm `pmset -g | grep SleepDisabled` shows `1`.
1. Apply settings. **Disconnect every SSH session** (they mask the fault — §3.1).
2. Wait **≥ 3× the old idle timer** (observed default 1 min ⇒ ≥ 5 min; use 30 min to also
   clear any residual standby path). Test both lid open and lid closed.
3. Probe a **service**, not ICMP: `ssh -o BatchMode=yes -o ConnectTimeout=10 <host> true`.
   Also confirm the tailnet node is `active`, not `offline, last seen …`. **Do not use
   `ping` — it answers while asleep (§3.2).**
4. **The power-cut cycle — the step everyone skips, and the one §4.2 inference this memo
   cannot close from research:** cut AC and leave it cut until the battery is fully dead
   (hours; `disablesleep` accelerates this). Restore AC. Expect minutes-to-hours of
   recharge before self-boot; then re-probe SSH and the tailnet. Separately confirm the
   edge-trigger caveat: a deliberate `shutdown` while AC stays present should NOT
   self-boot — proving the "never remotely shutdown" rule rather than assuming it.
5. Leave a **heartbeat canary** — a `launchd` job appending a timestamp every minute — so a
   gap in the log proves a sleep after the fact, rather than relying on catching it live.
   Also confirm the `disablesleep` re-assert LaunchDaemon (§4.1) survived a reboot.

Step 5 is what turns this from a one-time check into a durable claim: uptime you can
*audit*, not uptime you *believe*.

## 7. Windows sibling

Not written. The same shape applies (idle sleep, hibernate, fast startup, BitLocker
pre-boot, wake-on-LAN, auto-logon, power-failure restart in BIOS/UEFI). The BitLocker gate
is the analogue of the FileVault gate (C5) — though Windows has no analogue of the macOS 26
pre-boot SSH unlock, so BitLocker-on is a harder wall; the UEFI "restore on AC power loss"
setting is the analogue of C2, and unlike the Mac laptop's firmware edge-trigger it is
level-based and reliably available. A decade-old thin client sidesteps most of this memo.

## 8. Open questions (post-research residue)

- **The §4.2 recovery edge** — the one load-bearing inference research could not close;
  §6 step 4 is the whole answer. Also measure the actual recovery latency once.
- **Is a laptop the right host at all?** Sharpened, not settled: Apple Silicon desktops
  gained "start up when power is connected" only in macOS 26.5 (backported to Sequoia
  15.7.7), and desktop `autorestart` reports are shaky — so a Mac mini is better than
  assumed *only* on current macOS, and the laptop's built-in battery is, under §4.2, a
  free UPS. The calculus is closer than the original memo guessed.
- **Thermal/longevity:** a fanless Air held awake indefinitely — acceptable at idle, but
  check temperature under actual test load before committing. Lid-open (§4.1) is the
  thermally kinder posture.
- **WPA3 / 802.1X pre-boot unlock:** Apple names only open and WPA2-PSK Wi-Fi as
  qualifying for the FileVault pre-boot SSH path; behavior on WPA3-only networks is
  undocumented. Only matters if FileVault stays on.
