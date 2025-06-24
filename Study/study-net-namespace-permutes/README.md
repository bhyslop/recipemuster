# Network Namespace Permutation Study

## Overview
This directory contains scripts to test network namespace functionality across different Podman versions. The goal is to document how network namespace permissions and capabilities change between Podman releases.

## Study Process

### Prerequisites
- Podman VM running (`pdvm-rbw`)
- Access to container registry (ghcr.io)
- Study scripts in this directory

### VM Management Process

#### NUKE Process (When VM Image Changes)
When testing with a different VM image or after VM corruption:

1. **NUKE VM**: `bash tt/rbw-N.NUKE_PODMAN_MACHINE.sh`
   - This destroys and recreates the VM with the pinned image
   - Captures VM build date information during initialization
   - **Important**: Document the VM build date in the appendix below

2. **Start VM**: `bash tt/rbw-a.PodmanStart.sh`
3. **Verify VM Info**: Check that build date matches expected version

#### Standard Test Cycle
For each Podman version to be tested:

1. **Stop VM**: `bash tt/rbw-z.PodmanStop.sh`
2. **Start VM**: `bash tt/rbw-a.PodmanStart.sh` 
3. **Run Test Script**: `bash Study/study-net-namespace-permutes/[SCRIPT_NAME].sh`
4. **Document Results**: Update the script with version documentation block
5. **Clean Up**: Stop VM before next version

### Documentation Block Template
Each script should include this documentation block before the failing command:

```bash
# =============================================================================
# VERSION STUDY DOCUMENTATION BLOCK
# =============================================================================
# Date: [TIMESTAMP]
# 
# Podman Version: [VERSION] (Built: [BUILD_DATE])
# VM Build Date: [VM_BUILD_DATE]
# 
# Command: [EXACT_COMMAND_THAT_FAILS]
# 
# Expected Error from Next Command:
# [EXACT_ERROR_MESSAGE]
# =============================================================================
```

### Scripts to Test
- `Snnp-base-netns.sh` - Basic network namespace test
- `Snnp-cap-add.sh` - Capability addition test
- `Snnp-chmod-netns.sh` - Network namespace permission modification
- `Snnp-privileged-bottle.sh` - Privileged container test
- `Snnp-unshare-file-ns.sh` - File namespace unshare test
- `Snnp-unshare-proc-ns.sh` - Process namespace unshare test
- `Snnp-user-ns-basic.sh` - User namespace basic test
- `Snnp-podman-unshare.sh` - Podman unshare test
- `Snnp-podman-unshare-persistent.sh` - Persistent unshare test
- `Snnp-podman-unshare-persistent-v2.sh` - Persistent unshare v2 test
- `Snnp-podman-unshare-persistent-v3.sh` - Persistent unshare v3 test
- `Snnp-podman-unshare-persistent-v4.sh` - Persistent unshare v4 test

### Version Information to Capture
- **Podman Client Version**: `podman --version`
- **Podman Server Version**: `podman -c pdvm-rbw version`
- **VM Build Date**: `podman machine ssh pdvm-rbw "stat -c '%y' /usr/lib/os-release"`
- **VM Image SHA**: Currently pinned to `da977f55af1f69b6e4655b5a8faccc47b40034b29740f2d50e2b4d33cc1a7e16`

### Expected Failure Patterns
Most tests are expected to fail with permission-related errors:
- `Error: crun: cannot setns '/var/run/netns/[namespace]': Operation not permitted: OCI permission denied`
- `Error: permission denied`
- `Error: operation not permitted`

### Study Goals
1. **Document current failures** in Podman 5.3.2
2. **Test with different Podman versions** to identify when behaviors changed
3. **Identify workarounds** for network namespace limitations
4. **Create regression test suite** for future Podman releases

### Notes
- All tests run in rootless mode
- VM is restarted between tests to ensure clean state
- Network namespaces are manually created and configured
- Tests focus on container-to-namespace attachment failures

## Current Test Results
*[To be populated as tests are run]*

## Appendix: VM Build Information

### Current VM Configuration
- **VM Image**: `quay.io/podman/machine-os-wsl@sha256:da977f55af1f69b6e4655b5a8faccc47b40034b29740f2d50e2b4d33cc1a7e16`
- **VM Build Date**: `2024-11-17 16:00:00.000000000 -0800`
- **VM OS**: Fedora Linux 40 (Container Image)
- **VM Kernel**: 5.15.167.4-microsoft-standard-WSL2

### VM Build History
*[Document VM build dates from NUKE operations here]*

#### 2025-06-24 - Initial Setup
- **NUKE Date**: 2025-06-24 16:15:57 PDT
- **VM Build Date**: 2024-11-17 16:00:00.000000000 -0800
- **Podman Version**: 5.3.2 (client) / 5.3.1 (server)
- **Notes**: Initial VM setup for network namespace study

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


