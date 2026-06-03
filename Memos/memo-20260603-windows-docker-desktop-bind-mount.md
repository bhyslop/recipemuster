# Windows Docker Desktop — Cygwin crucible bind-mount wall and fix

*2026-06-03. Heat ₣BV (cygwin-spike). Written during the first-ever crucible
charge on an uncontrolled Cygwin + Docker Desktop host (`cygwin@rocket`).*

## TL;DR

Charging a Recipe Bottle crucible on an uncontrolled Cygwin box (repo checked
out on NTFS, Windows-native Docker Desktop on the WSL2 backend) hit **two**
Windows-native-docker walls beyond the already-known `docker build`/`docker cp`
path issues. Both are now fixed, and the bottle charges Healthy:

1. **`docker compose` CLI path args** (`-f` / `--env-file` / `--project-directory`)
   were read as Windows paths, so a `/cygdrive/c/...` absolute was misread.
   Fixed by `zrbob_native_path_capture` in `rbob_bottle.sh` — the `/cygdrive/X/...`
   → `X:/...` normalizer, the **fourth** copy of the same body alongside
   `zrbfc_native_path_capture` (rbfc_FoundryCore.sh) and `zrbndb_native_path_capture`
   (rbndb_base.sh). Pinning `--project-directory` to the native form also makes
   compose resolve the fragment's *relative* volume mounts to Windows-native
   paths, which Docker Desktop's WSL2 backend needs to bind-mount.

2. **Nested bind-mountpoint creation** failed with `EACCES`. Fixed by
   pre-creating the mountpoint in the bind source: tracked
   `Tools/rbk/rbtid/project/.gitkeep`.

## The nested-mountpoint wall (the interesting one)

The bottle's compose fragment (`tadmor`/`moriah` `rbnnh_compose.yml`) mounts two
**nested** binds, and the bottle runs as a non-root user:

```yaml
volumes:
  - ../Tools/rbk/rbtid : /workspace          :rw   # mount A (the ifrit driver + results channel)
  - ../                : /workspace/project   :ro   # mount B — nested INSIDE A
```

Charge error:

```
runc create failed: ... error mounting ".../rbm_alpha_recipemuster" to rootfs at
"/workspace/project": create mountpoint for /workspace/project mount:
mkdirat .../workspace/project: permission denied
```

**Mechanism.** A bind mountpoint must be created *in the filesystem of its
parent mount*:

- A **top-level** mountpoint (e.g. `/workspace`) is created in the container's
  **overlay rootfs** — a real Linux fs inside the DD VM. `mkdir` there always
  works. (This is why the sentry and pentacle — no nested binds — came up
  Healthy immediately.)
- A **nested** mountpoint (`/workspace/project`) must be created *inside mount A*
  — and mount A is the **Windows-hosted** `rbtid` directory, surfaced through
  Docker Desktop's WSL2 host-mount gateway (`/run/desktop/mnt/host/c/...`).
  `runc`'s `mkdir` through that gateway is **denied (EACCES)**.

On native Linux / WSL (ext4, no Windows gateway) the same nested bind works,
which is why this never surfaced before — it is specific to **(NTFS bind source)
× (nested bind mount) × (Docker Desktop WSL2)**.

**Fix.** Pre-create the mountpoint so `runc` binds onto an existing directory
instead of `mkdir`-ing through the gateway: a tracked, empty
`Tools/rbk/rbtid/project/`. Cross-platform no-op — on Linux/macOS the read-only
repo bind overlays the empty dir; on Cygwin-DD it is the pre-existing mountpoint.
Proven: with the dir present, all three containers charge Healthy.

**What does NOT work:** creating `/workspace/project` in the bottle *Dockerfile*.
Mount A (`rbtid → /workspace`) shadows the image's `/workspace` the instant it
mounts, so any image-baked nested dir vanishes. The mountpoint must live in the
bind **source**, not the image.

## Implications for other projects

- **Any container with nested bind mounts on Windows Docker Desktop where the
  outer mount's source is on NTFS** will hit this. Three remedies, in order of
  preference: (a) pre-create the nested mountpoint in the source dir (what we
  did — minimal, cross-platform), (b) de-nest so every mountpoint lands in the
  image overlay, (c) keep the checkout on the WSL2 ext4 side (the canonical
  Docker-on-Windows advice; avoids the NTFS gateway entirely).
- **APCK** (Ann's PHI Clipbuddy) bind-mounts `$HOME/apcjd/` into its container
  as a **single top-level** mount, and ships on macOS — so it is *not* exposed
  to this today. But if APCK ever nests a bind or runs on Windows DD, the same
  mountpoint discipline applies.
- **Recipe Bottle consumer credibility:** the paddock's locked "container tests
  run on WSL only" decision was a conservative hedge against exactly this wall.
  With these two membranes, crucibles now charge on an uncontrolled Cygwin DD
  box against an NTFS checkout — so that locked decision can be revisited.
  (Caveat: see `memo-20260603-cygwin-ifrit-first-run-conntrack.md` — one
  security sortie's *verdict* may differ on the DD-WSL2 network stack, which is
  a separate question from whether the crucible runs.)
- **Path-normalizer duplication is now 4×** (`zrbfc`, `zrbndb`, `zrbob`, plus the
  `rbgo_docker_login` auths bend). Consolidation into one Windows-docker adapter
  remains slated (heat ₣BV, step 3). The mountpoint discipline is a fifth
  Windows-DD membrane and should be folded into the same adapter's documentation.

## Pale-membrane status

Both fixes are membranes against a foreign (Docker Desktop WSL2) behavior we
cannot edit. Characterized (exact signatures above), contained (the normalizer
helpers; the one tracked mountpoint), logged (this memo + the `.gitkeep`
header), and given a retirement condition: retire when the mounts de-nest, the
foundry builds as a true Cygwin binary, or Docker Desktop fixes nested-mountpoint
creation on Windows-hosted binds.

## Reference

- WSG (Windows Scripting Guide) is the home for the Windows transport stack;
  this finding belongs in its orbit.
- Live host: `cygwin@rocket` (tailnet). Repo on NTFS at
  `C:\Users\cygwin\projects\rbm_alpha_recipemuster`.
