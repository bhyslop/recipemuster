# Network Namespace Permutation Study

## Overview
This directory contains scripts to test network namespace functionality across different Podman versions. The goal is to document how network namespace permissions and capabilities change between Podman releases.

**STUDY PARAMETER**: The execution mode (rootless/rootful) is determined by the VM configuration during NUKE operations. This is a key variable that significantly impacts network namespace functionality and should be documented for each test run.

## Study Process

### Prerequisites
- Podman VM running (`pdvm-rbw`) with documented mode (rootless/rootful)
- Access to container registry (ghcr.io)
- Study scripts in this directory

### VM Management Process

#### NUKE Process (When VM Image Changes)
When testing with a different VM image or after VM corruption:

1. **NUKE VM**: `bash tt/rbw-N.NUKE_PODMAN_MACHINE.sh`
   - This destroys and recreates the VM with the pinned image
   - Captures VM build date information during initialization
   - **Important**: Document the VM build date in the appendix below
   - **Important**: VM mode (rootless/rootful) is set during initialization

2. **Start VM**: `bash tt/rbw-a.PodmanStart.sh`
3. **Verify VM Info**: Check that build date matches expected version
4. **Verify Mode**: Confirm mode (`Rootful: true/false`) in machine inspection

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

### Version Information to Capture
- **Podman Client Version**: `podman --version`
- **Podman Server Version**: `podman -c pdvm-rbw version`
- **VM Build Date**: `podman machine ssh pdvm-rbw "stat -c '%y' /usr/lib/os-release"`
- **VM Image SHA**: Currently pinned to `f6e8175cd5921caa091794719787c2c889837dc74f989f0088ab5c5bde6c5b8e`

### Expected Failure Patterns
Most tests are expected to fail with permission-related errors:
- `Error: crun: cannot setns '/var/run/netns/[namespace]': Operation not permitted: OCI permission denied`
- `Error: permission denied`
- `Error: operation not permitted`

### Study Goals
1. **Document current failures** in Podman 5.3.2 across different modes (rootless/rootful)
2. **Test with different Podman versions** to identify when behaviors changed
3. **Identify workarounds** for network namespace limitations in different modes
4. **Create regression test suite** for future Podman releases
5. **Compare rootless vs rootful behavior** to isolate mode-specific limitations

### Notes
- **Mode (rootless/rootful) is a key variable** that should be documented for each test run
- VM is restarted between tests to ensure clean state
- Network namespaces are manually created and configured
- Tests focus on container-to-namespace attachment failures
- **Mode significantly impacts network namespace operations** - rootless mode has more restrictions
- Future studies should systematically compare rootless vs rootful behavior



## Appendix: VM Build Information

### Current VM Configuration
- **VM Image**: `quay.io/podman/machine-os-wsl@sha256:f6e8175cd5921caa091794719787c2c889837dc74f989f0088ab5c5bde6c5b8e`
- **VM Build Date**: `2025-04-22 17:00:00.000000000 -0700`
- **VM Creation Date**: `2025-06-24T18:46:25.2112966-07:00`
- **VM OS**: Fedora Linux 41 (Container Image)
- **VM Kernel**: 5.15.167.4-microsoft-standard-WSL2
- **Current Mode**: Rootless (`Rootful: false`)

### VM Build History
*[Document VM build dates and modes from NUKE operations here]*

#### 2025-06-24 - Podman 5.5.2 Update
- **NUKE Date**: 2025-06-24 18:29:09 PDT
- **VM Build Date**: 2025-04-22 17:00:00.000000000 -0700
- **Podman Version**: 5.5.2 (client) / 5.5.1 (server)
- **VM Image SHA**: f6e8175cd5921caa091794719787c2c889837dc74f989f0088ab5c5bde6c5b8e
- **Mode**: Rootless (`Rootful: false`)
- **Notes**: NUKE operation completed successfully. Updated to Podman 5.5.2. Test Snnp-base-netns.sh executed successfully with expected failure pattern.

#### 2025-06-24 - Network Namespace Study Setup
- **NUKE Date**: 2025-06-24 16:39:22 PDT
- **VM Build Date**: 2024-11-17 16:00:00.000000000 -0800
- **Podman Version**: 5.3.2 (client)
- **VM Image SHA**: da977f55af1f69b6e4655b5a8faccc47b40034b29740f2d50e2b4d33cc1a7e16
- **Mode**: Rootless (`Rootful: false`)
- **Notes**: NUKE operation completed successfully for network namespace study

#### 2025-06-24 - Initial Setup
- **NUKE Date**: 2025-06-24 16:15:57 PDT
- **VM Build Date**: 2024-11-17 16:00:00.000000000 -0800
- **Podman Version**: 5.3.2 (client) / 5.3.1 (server)
- **Mode**: Rootless (`Rootful: false`)
- **Notes**: Initial VM setup for network namespace study
