# Controlled Test History

## Getting Podman and VM Image Info

```bash
Tools/rbw.workbench.mk: TEMPORARY: Log version info
podman --version
podman.exe version 5.3.2
Tools/rbw.workbench.mk: TEMPORARY: Tag below found at -> https://quay.io/repository/podman/machine-os-wsl?tab=tags
Tools/rbw.workbench.mk: TEMPORARY: init Podman machine pdvm-rbw
podman machine init --image docker://quay.io/podman/machine-os-wsl@sha256:da977f55af1f69b6e4655b5a8faccc47b40034b29740f2d50e2b4d33cc1a7e16   pdvm-rbw
Looking up Podman Machine image at quay.io/podman/machine-os-wsl@sha256:da977f55af1f69b6e4655b5a8faccc47b40034b29740f2d50e2b4d33cc1a7e16 to create VM
Getting image source signatures
Copying blob sha256:6898117ca935bae6cbdf680d5b8bb27c0f9fbdfe8799e5fe8aae7b87c17728d3
Copying config sha256:44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a
Writing manifest to image destination
6898117ca935bae6cbdf680d5b8bb27c0f9fbdfe8799e5fe8aae7b87c17728d3
Importing operating system into WSL (this may take a few minutes on a new WSL install)...
The operation completed successfully.
Configuring system...
Machine init complete
```


