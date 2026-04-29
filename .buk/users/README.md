# BUK Users Directory

Per-user configuration profiles, keyed by `BURS_USER` from the station regime.

## Structure

```
.buk/users/<BURS_USER>/<alias>/burn.env
```

Each subdirectory under a user is a BURN node regime profile — one SSH connection configuration. The `<alias>` must match the `BURN_ALIAS` value inside the `burn.env` file.

`BURN_KEY_FILE` specifies the SSH private key filename in `~/.ssh/`. When empty, defaults to the alias name. Localhost profiles set `id_ed25519` to share the invoking user's default key.

## Git Safety

BURN profiles contain **public** key material only (`BURN_SSH_PUBKEY`). Private keys are never stored here. These files are safe to commit.

## Example

```
.buk/users/bhyslop/winhost-cyg/burn.env   # SSH to Windows via Cygwin bash
.buk/users/bhyslop/winhost-wsl/burn.env   # SSH to Windows via WSL
.buk/users/bhyslop/winhost-ps/burn.env    # SSH to Windows via PowerShell
```

## Validation

```bash
tt/buw-rhva.ValidateAllHostProfiles.sh     # all profiles for current BURS_USER
tt/buw-rhv.ValidateHostProfile.sh <alias>  # single profile
```
