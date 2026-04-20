# Remote Docker Access to Ann's MacBook via DOCKER_HOST=ssh://

**Date:** 2026-04-19
**Domain:** APCK deployment infrastructure; Docker Desktop on macOS multi-user
**Status:** Decision recorded — plan to implement

---

## Summary

To run Docker commands on `anns-macbook-air` from my laptop without fighting Docker Desktop's per-user socket ownership, the plan is:

1. Install my SSH public key into `/Users/annsegeren/.ssh/authorized_keys` on her Mac (one-time).
2. Add an SSH alias (e.g. `anns-mba`) in `~/.ssh/config` pointing at `annsegeren@<host>`.
3. Use Docker's native SSH transport: `DOCKER_HOST=ssh://annsegeren@anns-mba docker …` (or `docker -H ssh://…`).

No Docker Desktop reconfiguration. No TCP exposure. No chmod on the socket. No sudoers rule.

---

## Why This, Not the Alternatives

### What I observed

Docker Desktop 29.4.0 installed cleanly on her Mac. After first-launch privileged-helper setup, `/var/run/docker.sock` exists as a root-owned symlink to `/Users/annsegeren/.docker/run/docker.sock`, and `/usr/local/bin/docker` is symlinked to the app's CLI.

But my SSH login is `bhyslop@anns-macbook-air`, not `annsegeren`. The underlying socket is mode `srwxr-xr-x` owned by `annsegeren:staff` — others do **not** get write. So `docker version` from `bhyslop` returns "permission denied while trying to connect to the docker API at unix:///var/run/docker.sock".

### Options I ruled out

- **`sudo chmod 666` the socket.** Works for one session, but Docker Desktop recreates the socket on every VM start with Ann's perms. Also requires her admin password each time — no passwordless sudo on her machine.
- **Expose daemon on `tcp://localhost:2375`.** This toggle is Windows-only in Docker Desktop. **It has never existed on macOS** — not a v29.4.0 removal, an architectural gap Docker has acknowledged for years (docker/for-mac #6263).
- **socat TCP-bridge sidecar container.** Works, but more moving parts and still requires Ann to keep a bridge container running.
- **NOPASSWD sudoers rule to run `docker` as `annsegeren`.** Invasive, security-sensitive, and per-command.
- **Install/launch Docker Desktop under my own account on her Mac.** Docker Desktop is a single-user singleton at runtime: `/var/run/docker.sock` and `/usr/local/bin/docker` are system-wide symlinks that rebind to whichever user launched Docker Desktop most recently. Ann and I would be in a tug-of-war over the socket every time either of us launched the app. Also: Docker Desktop requires a logged-in Aqua session to keep running — an SSH-only session for `bhyslop` wouldn't sustain it. Docker's docs state plainly that "Docker is not designed to be securely shared among multiple users" (docker/for-mac #6781, #929).

### Why SSH transport wins

- **Docker-native.** `DOCKER_HOST=ssh://…` is the officially supported remote-access mechanism. No reconfig of the daemon.
- **No socket-permission fight.** The `docker` CLI on my machine forks `ssh` to her machine and talks to her socket **as her user** — which is exactly who owns it.
- **Survives reboots and Docker Desktop restarts.** Nothing to re-apply.
- **Keeps her Docker Desktop canonical.** One VM, one owner, no singleton thrash.
- **Zero ongoing human-in-the-loop.** No password prompts after the one-time key install.

---

## Implementation Checklist

- [ ] Append my public key (`~/.ssh/id_ed25519.pub`) to `/Users/annsegeren/.ssh/authorized_keys` on her Mac. Ensure permissions: `~/.ssh` = 700, `authorized_keys` = 600, owned by `annsegeren`.
- [ ] Verify `sshd_config` allows `annsegeren` (it currently allows `bhyslop`; confirm `AllowUsers` / `AllowGroups` isn't blocking her).
- [ ] Add to my `~/.ssh/config`:
  ```
  Host anns-mba
      HostName <annsegeren's host/IP>
      User annsegeren
      IdentityFile ~/.ssh/id_ed25519
  ```
- [ ] Smoke test: `ssh anns-mba whoami` → `annsegeren`.
- [ ] Smoke test: `DOCKER_HOST=ssh://anns-mba docker version` → both Client and Server sections populate.
- [ ] Smoke test image transfer: `docker save myimage:tag | docker -H ssh://anns-mba load`.

---

## Caveats to Remember

- SSH transport requires a reachable SSH endpoint. If her Mac sleeps or the network drops, commands fail until she's back online.
- Image build context is streamed over SSH; large build contexts will be slow. For large images, consider `docker save | ssh … docker load` instead of remote `docker build`.
- Targets must match her CPU: she is `arm64` (Apple M2). Build `linux/arm64` images or multi-arch manifests.

---

## Sources

- [Docker Desktop for MacOS has no "Expose daemon on tcp://localhost:2375" setting (docker/for-mac #6263)](https://github.com/docker/for-mac/issues/6263)
- [Docker Desktop for Mac does not work for multiple users (docker/for-mac #6781)](https://github.com/docker/for-mac/issues/6781)
- [Problem when multiple users on the same host want to use docker (docker/for-mac #929)](https://github.com/docker/for-mac/issues/929)
- [Configure remote access for Docker daemon (Docker Docs)](https://docs.docker.com/engine/daemon/remote-access/)
- [Mac permission requirements — Docker Desktop](https://docs.docker.com/desktop/setup/install/mac-permission-requirements/)
