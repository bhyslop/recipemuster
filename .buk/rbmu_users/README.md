# BUK Privileged Investitures (BURP)

Per-station-user privileged-credential profiles, keyed by `BURS_USER` from the station regime.

## Structure

```
.buk/rbmu_users/<BURS_USER>/<investiture>/burp.env
```

Each subdirectory under a user is a BURP investiture — one privileged-credential identity granted over a node. The `<investiture>` is the directory name; it is not redundantly inscribed inside the file. The `<investiture>` name MUST equal a registered viceroyalty (i.e. a directory under `.buk/rbmn_nodes/`); the 1:1 correspondence is enforced by file-presence check at load time.

## Fields

```
BURP_PRIVILEGED_USER=<admin-user>   # remote admin OS user this investiture authenticates as
BURP_PRIVILEGED_KEY_FILE=<path>     # operator-managed SSH private key path for admin authentication
BURP_WORKLOAD_KEY_FILE=<path>       # operator-managed SSH private key path for workload authentication
```

## Git Safety

BURP profiles carry **no key material** — only paths to operator-managed private keys on the local station. Pubkeys are derived at use-time via `ssh-keygen -y`.

## Validation

```bash
tt/buw-rpl.ListPrivilegeRegime.sh                # all investitures for current BURS_USER
tt/buw-rpv.ValidatePrivilegeRegime.sh <invest>   # single investiture
tt/buw-rpr.RenderPrivilegeRegime.sh <invest>     # show resolved fields
```

## Companion: BURN Node Profiles

Node-shape profiles (host, platform) live at `.buk/rbmn_nodes/<viceroyalty>/burn.env` — project-global, git-tracked. Each BURP investiture name IS a viceroyalty name by construction; the BURP directory name must match a BURN profile dir.
