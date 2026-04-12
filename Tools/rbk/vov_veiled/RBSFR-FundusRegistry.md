# Fundus Capability Registry (Prototype)

Agent-interpreted test target inventory. Not program-readable.
Maintained manually as environments are provisioned and verified.

Status key: **verified** (practiced and working), **provisioned** (set up but untested),
**TBD** (not yet attempted), **no** (known unsupported).

## localhost (macOS)

- status: verified
- shell: bash 5.x
- docker: Docker Desktop
- network-namespaces: yes
- reachable: direct
- buk: installed
- crucible: yes
- fundus-profiles: jjfu_full, jjfu_nokey, jjfu_norepo, jjfu_nogit
- test-suites: fast, service, crucible, complete
- notes: primary development machine

## rbhw-wsl (Windows / WSL rbtww-main)

- status: TBD
- shell: TBD
- docker: native dockerd (planned)
- network-namespaces: TBD
- reachable: ssh rbhw-wsl (key-routed via command= entrypoint)
- buk: TBD
- crucible: TBD
- fundus-profiles: TBD (jjfu_* via existing jjw-tfP1/P2)
- test-suites: TBD
- notes: primary Windows test target. Procedures in buw-HW*/rbw-HW* handbooks.

## rbhw-cygwin (Windows / Cygwin)

- status: TBD
- shell: TBD (bash >= 3.2 expected)
- docker: Docker Desktop (via Windows host)
- network-namespaces: no
- reachable: ssh rbhw-cygwin (key-routed via command= entrypoint)
- buk: TBD
- crucible: no (no network namespaces)
- fundus-profiles: TBD
- test-suites: TBD (BUK self-test, regime validation — no crucible)
- notes: POSIX compatibility testing. Cygwin bash may have path translation quirks.

## rbhw-win (Windows / PowerShell)

- status: TBD
- shell: pwsh
- docker: Docker Desktop
- network-namespaces: no
- reachable: ssh rbhw-win (key-routed via command= entrypoint)
- buk: no (bash required)
- crucible: no
- fundus-profiles: n/a (no BUK, no bash)
- test-suites: n/a
- notes: administrative access only. Not a test execution target.

## cerebro

- status: provisioned
- shell: bash
- docker: TBD
- network-namespaces: TBD
- reachable: ssh cerebro
- buk: installed
- crucible: TBD
- fundus-profiles: jjfu_full, jjfu_nokey, jjfu_norepo, jjfu_nogit
- test-suites: fundus-scenario (verified)
- notes: existing remote test host for JJK fundus scenarios.
