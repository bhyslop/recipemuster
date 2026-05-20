# BUK Node Investitures (BURN)

Per-node configuration profiles describing the shape of remote targets. Project-global, git-tracked, shared across station users.

## Structure

```
.buk/rbmn_nodes/<investiture>/burn.env
```

Each subdirectory is a BURN investiture — one node identity. The `<investiture>` is the directory name; it is not redundantly inscribed inside the file.

## Fields

```
BURN_HOST=<ip-or-hostname>       # IP address or hostname of the remote node
BURN_PLATFORM=<platform>         # bubep_linux | bubep_mac | bubep_windows
```

## Git Safety

BURN profiles carry no key material — they describe node shape only. Safe to commit; shared across station users.

## Validation

```bash
tt/buw-rnl.ListNodeRegime.sh                  # all investitures
tt/buw-rnv.ValidateNodeRegime.sh <investiture>  # single investiture
tt/buw-rnr.RenderNodeRegime.sh <investiture>    # show resolved fields
```

## Companion: BURP Privileged Investitures

Privileged credentials live at `.buk/rbmu_users/<user>/<investiture>/burp.env` — per-station-user, operator-authored. Each BURP investiture name IS a investiture name by construction; the `<investiture>` directory name must match a investiture directory under `.buk/rbmn_nodes/`, enforced by file-presence check at load time.
