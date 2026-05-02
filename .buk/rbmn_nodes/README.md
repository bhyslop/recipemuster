# BUK Node Viceroyalties (BURN)

Per-node configuration profiles describing the shape of remote targets. Project-global, git-tracked, shared across station users.

## Structure

```
.buk/rbmn_nodes/<viceroyalty>/burn.env
```

Each subdirectory is a BURN viceroyalty — one node identity. The `<viceroyalty>` is the directory name; it is not redundantly inscribed inside the file.

## Fields

```
BURN_HOST=<ip-or-hostname>       # IP address or hostname of the remote node
BURN_PLATFORM=<platform>         # linux | mac | cygwin | wsl | powershell | localhost
```

## Git Safety

BURN profiles carry no key material — they describe node shape only. Safe to commit; shared across station users.

## Validation

```bash
tt/buw-rnl.ListNodeRegime.sh                  # all viceroyalties
tt/buw-rnv.ValidateNodeRegime.sh <viceroyalty>  # single viceroyalty
tt/buw-rnr.RenderNodeRegime.sh <viceroyalty>    # show resolved fields
```

## Companion: BURP Privileged Investitures

Privileged credentials live at `.buk/rbmu_users/<user>/<investiture>/burp.env` — per-station-user, operator-authored. Each BURP investiture references a viceroyalty by `BURP_NODE`.
