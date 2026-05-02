# BUK Privileged Investitures (BURP)

Per-station-user privileged-credential profiles, keyed by `BURS_USER` from the station regime.

## Structure

```
.buk/rbmu_users/<BURS_USER>/<investiture>/burp.env
```

Each subdirectory under a user is a BURP investiture — one privileged-credential identity granted over a node. The `<investiture>` is the directory name; it is not redundantly inscribed inside the file.

## Fields

```
BURP_NODE=<viceroyalty>          # references .buk/rbmn_nodes/<viceroyalty>
BURP_USER=<remote-os-user>       # admin user authenticated by this investiture
BURP_SSH_PUBKEY='ssh-... ...'    # operator-managed admin keypair (public side)
BURP_KEY_FILE=<filename>         # SSH private key filename (empty = use investiture name)
BURP_COMMAND='<shell-command>'   # optional command= routing in administrators_authorized_keys
```

## Git Safety

BURP profiles contain **public** key material only (`BURP_SSH_PUBKEY`). Private keys are never stored here; they live at operator-managed paths.

## Validation

```bash
tt/buw-rpl.ListPrivilegeRegime.sh                # all investitures for current BURS_USER
tt/buw-rpv.ValidatePrivilegeRegime.sh <invest>   # single investiture
tt/buw-rpr.RenderPrivilegeRegime.sh <invest>     # show resolved fields
```

## Companion: BURN Node Profiles

Node-shape profiles (host, platform) live at `.buk/rbmn_nodes/<viceroyalty>/burn.env` — project-global, git-tracked. Each BURP investiture references a viceroyalty by `BURP_NODE`.
