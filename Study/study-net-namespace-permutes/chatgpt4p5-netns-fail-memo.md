# Network Namespace Strategy Failure Summary

## Context

The project aimed to implement strict network isolation and DNS enforcement by leveraging Podman-managed Linux network namespaces. Central to this was the concept of a privileged container (**SENTRY**) responsible for imposing security controls upon another container (**BOTTLE**), ensuring that all outbound traffic and DNS requests from BOTTLE are explicitly managed and restricted.

This memo summarizes extensive testing that conclusively demonstrated the infeasibility of this approach in recent Podman versions (specifically 5.3.2 and 5.5.2).

## Strategy Overview

The original strategy involved:

- Creating and managing explicit Linux network namespaces.
- Establishing virtual Ethernet pairs (**veth**) to connect namespaces.
- Assigning namespace endpoints explicitly to container processes via Podman.
- Leveraging scripts executed within `podman unshare` contexts to maintain persistent namespaces and associated network interfaces.

## Categories of Failures

Testing across multiple scenarios and Podman configurations consistently produced the following categories of failures:

### 1. **Namespace Creation Permission Errors**

All attempts at directly creating and configuring network namespaces encountered consistent permissions failures:

- Attempts to write to `/var/run/netns` consistently failed:
  - Script references: `Snnp-base-netns.sh`, `Snnp-podman-unshare.sh`, `Snnp-podman-unshare-persistent.sh`
  - Typical error: `mkdir /var/run/netns failed: Permission denied`

### 2. **Namespace Attachment and `setns` Errors**

Attaching existing network namespaces to Podman containers universally failed:

- Consistent OCI runtime errors:
  - Script references: `Snnp-privileged-bottle.sh`, `Snnp-cap-add.sh`
  - Typical error: `crun: cannot setns '/var/run/netns/nsproto-ns': Operation not permitted: OCI permission denied`

### 3. **Interface Configuration Failures**

Even in scenarios where namespaces could momentarily be created, attempts to configure network interfaces failed:

- Errors reported by RTNETLINK:
  - Script references: `Snnp-podman-unshare-persistent-v2.sh`, `Snnp-podman-unshare-persistent-v3.sh`
  - Typical errors:
    - `RTNETLINK answers: Operation not permitted`
    - `RTNETLINK answers: No such process` (indicating veth pair movements failed)

### 4. **Persistent Namespace Maintenance Failures**

All approaches involving persistent namespace maintenance via long-lived processes within `podman unshare` contexts consistently failed:

- Inability to sustain namespaces due to fundamental permission constraints.
- Script references: `Snnp-podman-unshare-persistent.sh`, `Snnp-podman-unshare-persistent-v2.sh`, `Snnp-podman-unshare-persistent-v3.sh`

## Observations

- Both **rootless and rootful** Podman modes exhibited these failures.
- Permissions and capability restrictions within modern Podman significantly differ from earlier versions (prior to 5.2), rendering previously viable approaches nonfunctional.
- Even privileged container setups (`--privileged`, `--cap-add=SYS_ADMIN`) failed to overcome core limitations.

## Conclusion

Extensive attempts documented through this series of scripts conclusively demonstrate that leveraging Podman-managed network namespaces for explicit and persistent security control, as originally designed, is fundamentally untenable under modern Podman versions tested (5.3.2, 5.5.2).

This evidence compels a strategic pivot away from namespace-centric methods towards alternative models for container network isolation and DNS control within the project's SENTRY/BOTTLE architecture.

