# podman-5.3.2 Pod Commands

*This document contains Pod commands from the Podman documentation.*

## Table of Contents

- [podman-pod-clone - Create a copy of an existing pod](#podman-pod-clone)
- [podman-pod-create - Create a new pod](#podman-pod-create)
- [podman-pod-exists - Check if a pod exists in local storage](#podman-pod-exists)
- [podman-pod-inspect - Display information describing a pod](#podman-pod-inspect)
- [podman-pod-kill - Kill the main process of each container in one or
more pods](#podman-pod-kill)
- [podman-pod-logs - Display logs for pod with one or more
containers](#podman-pod-logs)
- [podman-pod-pause - Pause one or more pods](#podman-pod-pause)
- [podman-pod-prune - Remove all stopped pods and their containers](#podman-pod-prune)
- [podman-pod-ps - Print out information about pods](#podman-pod-ps)
- [podman-pod-restart - Restart one or more pods](#podman-pod-restart)
- [podman-pod-rm - Remove one or more stopped pods and containers](#podman-pod-rm)
- [podman-pod-start - Start one or more pods](#podman-pod-start)
- [podman-pod-stats - Display a live stream of resource usage stats for
containers in one or more pods](#podman-pod-stats)
- [podman-pod-stop - Stop one or more pods](#podman-pod-stop)
- [podman-pod-top - Display the running processes of containers in a
pod](#podman-pod-top)
- [podman-pod-unpause - Unpause one or more pods](#podman-pod-unpause)

<a id='podman-pod-clone'></a>

## podman-pod-clone - Create a copy of an existing pod

##  NAME

podman-pod-clone - Create a copy of an existing pod

##  SYNOPSIS

**podman pod clone** \[*options*\] *pod* *name*

##  DESCRIPTION

**podman pod clone** creates a copy of a pod, recreating the identical
config for the pod and for all of its containers. Users can modify the
pods new name and select pod details within the infra container

##  OPTIONS

#### **\--blkio-weight**=*weight*

Block IO relative weight. The *weight* is a value between **10** and
**1000**.

This option is not supported on cgroups V1 rootless systems.

#### **\--blkio-weight-device**=*device:weight*

Block IO relative device weight.

#### **\--cgroup-parent**=*path*

Path to cgroups under which the cgroup for the pod is created. If the
path is not absolute, the path is considered to be relative to the
cgroups path of the init process. Cgroups are created if they do not
already exist.

#### **\--cpu-shares**, **-c**=*shares*

CPU shares (relative weight).

By default, all containers get the same proportion of CPU cycles. This
proportion can be modified by changing the container\'s CPU share
weighting relative to the combined weight of all the running containers.
Default weight is **1024**.

The proportion only applies when CPU-intensive processes are running.
When tasks in one container are idle, other containers can use the
left-over CPU time. The actual amount of CPU time varies depending on
the number of containers running on the system.

For example, consider three containers, one has a cpu-share of 1024 and
two others have a cpu-share setting of 512. When processes in all three
containers attempt to use 100% of CPU, the first container receives 50%
of the total CPU time. If a fourth container is added with a cpu-share
of 1024, the first container only gets 33% of the CPU. The remaining
containers receive 16.5%, 16.5% and 33% of the CPU.

On a multi-core system, the shares of CPU time are distributed over all
CPU cores. Even if a container is limited to less than 100% of CPU time,
it can use 100% of each individual CPU core.

For example, consider a system with more than three cores. If the
container *C0* is started with **\--cpu-shares=512** running one
process, and another container *C1* with **\--cpu-shares=1024** running
two processes, this can result in the following division of CPU shares:

  PID   container   CPU   CPU share
  ----- ----------- ----- --------------
  100   C0          0     100% of CPU0
  101   C1          1     100% of CPU1
  102   C1          2     100% of CPU2

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--cpus**

Set a number of CPUs for the pod that overrides the original pods CPU
limits. If none are specified, the original pod\'s Nano CPUs are used.

#### **\--cpuset-cpus**=*number*

CPUs in which to allow execution. Can be specified as a comma-separated
list (e.g. **0,1**), as a range (e.g. **0-3**), or any combination
thereof (e.g. **0-3,7,11-15**).

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

If none are specified, the original pod\'s CPUset is used.

#### **\--cpuset-mems**=*nodes*

Memory nodes (MEMs) in which to allow execution (0-3, 0,1). Only
effective on NUMA systems.

If there are four memory nodes on the system (0-3), use
**\--cpuset-mems=0,1** then processes in the container only uses memory
from the first two memory nodes.

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--destroy**

Remove the original pod that we are cloning once used to mimic the
configuration.

#### **\--device**=*host-device\[:container-device\]\[:permissions\]*

Add a host device to the pod. Optional *permissions* parameter can be
used to specify device permissions by combining **r** for read, **w**
for write, and **m** for **mknod**(2).

Example: **\--device=/dev/sdc:/dev/xvdc:rwm**.

Note: if *host-device* is a symbolic link then it is resolved first. The
pod only stores the major and minor numbers of the host device.

Podman may load kernel modules required for using the specified device.
The devices that Podman loads modules for when necessary are: /dev/fuse.

In rootless mode, the new device is bind mounted in the container from
the host rather than Podman creating it within the container space.
Because the bind mount retains its SELinux label on SELinux systems, the
container can get permission denied when accessing the mounted device.
Modify SELinux settings to allow containers to use all device labels via
the following command:

\$ sudo setsebool -P container_use_devices=true

Note: the pod implements devices by storing the initial configuration
passed by the user and recreating the device on each container added to
the pod.

#### **\--device-read-bps**=*path:rate*

Limit read rate (in bytes per second) from a device (e.g.
**\--device-read-bps=/dev/sda:1mb**).

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--device-write-bps**=*path:rate*

Limit write rate (in bytes per second) to a device (e.g.
**\--device-write-bps=/dev/sda:1mb**).

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--gidmap**=*pod_gid:host_gid:amount*

GID map for the user namespace. Using this flag runs all containers in
the pod with user namespace enabled. It conflicts with the **\--userns**
and **\--subgidname** flags.

#### **\--gpus**=*ENTRY*

GPU devices to add to the container (\'all\' to pass all GPUs) Currently
only Nvidia devices are supported.

#### **\--help**, **-h**

Print usage statement.

#### **\--hostname**=*name*

Set the pod\'s hostname inside all containers.

The given hostname is also added to the `/etc/hosts` file using the
container\'s primary IP address (also see the **\--add-host** option).

#### **\--infra-command**=*command*

The command that is run to start the infra container. Default:
\"/pause\".

#### **\--infra-conmon-pidfile**=*file*

Write the pid of the infra container\'s **conmon** process to a file. As
**conmon** runs in a separate process than Podman, this is necessary
when using systemd to manage Podman containers and pods.

#### **\--infra-name**=*name*

The name that is used for the pod\'s infra container.

#### **\--label**, **-l**=*key=value*

Add metadata to a pod.

#### **\--label-file**=*file*

Read in a line-delimited file of labels.

#### **\--memory**, **-m**=*number\[unit\]*

Memory limit. A *unit* can be **b** (bytes), **k** (kibibytes), **m**
(mebibytes), or **g** (gibibytes).

Allows the memory available to a container to be constrained. If the
host supports swap memory, then the **-m** memory setting can be larger
than physical RAM. If a limit of 0 is specified (not using **-m**), the
container\'s memory is not limited. The actual limit may be rounded up
to a multiple of the operating system\'s page size (the value is very
large, that\'s millions of trillions).

This option is not supported on cgroups V1 rootless systems.

#### **\--memory-swap**=*number\[unit\]*

A limit value equal to memory plus swap. A *unit* can be **b** (bytes),
**k** (kibibytes), **m** (mebibytes), or **g** (gibibytes).

Must be used with the **-m** (**\--memory**) flag. The argument value
must be larger than that of **-m** (**\--memory**) By default, it is set
to double the value of **\--memory**.

Set *number* to **-1** to enable unlimited swap.

This option is not supported on cgroups V1 rootless systems.

#### **\--name**, **-n**

Set a custom name for the cloned pod. The default if not specified is of
the syntax: **\<ORIGINAL_NAME\>-clone**

#### **\--pid**=*pid*

Set the PID mode for the pod. The default is to create a private PID
namespace for the pod. Requires the PID namespace to be shared via
\--share.

    host: use the host’s PID namespace for the pod
    ns: join the specified PID namespace
    private: create a new namespace for the pod (default)

#### **\--restart**=*policy*

Restart policy to follow when containers exit. Restart policy does not
take effect if a container is stopped via the **podman kill** or
**podman stop** commands.

Valid *policy* values are:

-   `no` : Do not restart containers on exit
-   `never` : Synonym for **no**; do not restart containers on exit
-   `on-failure[:max_retries]` : Restart containers when they exit with
    a non-zero exit code, retrying indefinitely or until the optional
    *max_retries* count is hit
-   `always` : Restart containers when they exit, regardless of status,
    retrying indefinitely
-   `unless-stopped` : Identical to **always**

Podman provides a systemd unit file, podman-restart.service, which
restarts containers after a system reboot.

When running containers in systemd services, use the restart
functionality provided by systemd. In other words, do not use this
option in a container unit, instead set the `Restart=` systemd directive
in the `[Service]` section. See **podman-systemd.unit**(5) and
**systemd.service**(5).

Default restart policy for all the containers in a pod.

#### **\--security-opt**=*option*

Security Options

-   **apparmor=unconfined** : Turn off apparmor confinement for the pod

-   **apparmor**=*alternate-profile* : Set the apparmor confinement
    profile for the pod

-   **label=user:**\_USER\_: Set the label user for the pod processes

-   **label=role:**\_ROLE\_: Set the label role for the pod processes

-   **label=type:**\_TYPE\_: Set the label process type for the pod
    processes

-   **label=level:**\_LEVEL\_: Set the label level for the pod processes

-   **label=filetype:**\_TYPE\_: Set the label file type for the pod
    files

-   **label=disable**: Turn off label separation for the pod

Note: Labeling can be disabled for all pods/containers by setting
label=false in the **containers.conf**
(`/etc/containers/containers.conf` or
`$HOME/.config/containers/containers.conf`) file.

-   **label=nested**: Allows SELinux modifications within the container.
    Containers are allowed to modify SELinux labels on files and
    processes, as long as SELinux policy allows. Without **nested**,
    containers view SELinux as disabled, even when it is enabled on the
    host. Containers are prevented from setting any labels.

-   **mask**=*/path/1:/path/2*: The paths to mask separated by a colon.
    A masked path cannot be accessed inside the containers within the
    pod.

-   **no-new-privileges**: Disable container processes from gaining
    additional privileges.

-   **seccomp=unconfined**: Turn off seccomp confinement for the pod.

-   **seccomp=profile.json**: JSON file to be used as a seccomp filter.
    Note that the `io.podman.annotations.seccomp` annotation is set with
    the specified value as shown in `podman inspect`.

-   **proc-opts**=*OPTIONS* : Comma-separated list of options to use for
    the /proc mount. More details for the possible mount options are
    specified in the **proc(5)** man page.

-   **unmask**=*ALL* or */path/1:/path/2*, or shell expanded paths
    (/proc/\*): Paths to unmask separated by a colon. If set to **ALL**,
    it unmasks all the paths that are masked or made read-only by
    default. The default masked paths are **/proc/acpi, /proc/kcore,
    /proc/keys, /proc/latency_stats, /proc/sched_debug, /proc/scsi,
    /proc/timer_list, /proc/timer_stats, /sys/firmware, and
    /sys/fs/selinux**, **/sys/devices/virtual/powercap**. The default
    paths that are read-only are **/proc/asound**, **/proc/bus**,
    **/proc/fs**, **/proc/irq**, **/proc/sys**, **/proc/sysrq-trigger**,
    **/sys/fs/cgroup**.

Note: Labeling can be disabled for all containers by setting
**label=false** in the **containers.conf**(5) file.

#### **\--shm-size**=*number\[unit\]*

Size of */dev/shm*. A *unit* can be **b** (bytes), **k** (kibibytes),
**m** (mebibytes), or **g** (gibibytes). If the unit is omitted, the
system uses bytes. If the size is omitted, the default is **64m**. When
*size* is **0**, there is no limit on the amount of memory used for IPC
by the pod. This option conflicts with **\--ipc=host**.

#### **\--shm-size-systemd**=*number\[unit\]*

Size of systemd-specific tmpfs mounts such as /run, /run/lock,
/var/log/journal and /tmp. A *unit* can be **b** (bytes), **k**
(kibibytes), **m** (mebibytes), or **g** (gibibytes). If the unit is
omitted, the system uses bytes. If the size is omitted, the default is
**64m**. When *size* is **0**, the usage is limited to 50% of the
host\'s available memory.

#### **\--start**

When set to true, this flag starts the newly created pod after the clone
process has completed. All containers within the pod are started.

#### **\--subgidname**=*name*

Run the container in a new user namespace using the map with *name* in
the */etc/subgid* file. If running rootless, the user needs to have the
right to use the mapping. See **subgid**(5). This flag conflicts with
**\--userns** and **\--gidmap**.

#### **\--subuidname**=*name*

Run the container in a new user namespace using the map with *name* in
the */etc/subuid* file. If running rootless, the user needs to have the
right to use the mapping. See **subuid**(5). This flag conflicts with
**\--userns** and **\--uidmap**.

#### **\--sysctl**=*name=value*

Configure namespaced kernel parameters for all containers in the pod.

For the IPC namespace, the following sysctls are allowed:

-   kernel.msgmax
-   kernel.msgmnb
-   kernel.msgmni
-   kernel.sem
-   kernel.shmall
-   kernel.shmmax
-   kernel.shmmni
-   kernel.shm_rmid_forced
-   Sysctls beginning with fs.mqueue.\*

Note: if the ipc namespace is not shared within the pod, the above
sysctls are not allowed.

For the network namespace, only sysctls beginning with net.\* are
allowed.

Note: if the network namespace is not shared within the pod, the above
sysctls are not allowed.

#### **\--uidmap**=*container_uid:from_uid:amount*

Run all containers in the pod in a new user namespace using the supplied
mapping. This option conflicts with the **\--userns** and
**\--subuidname** options. This option provides a way to map host UIDs
to container UIDs. It can be passed several times to map different
ranges.

#### **\--userns**=*mode*

Set the user namespace mode for all the containers in a pod. It defaults
to the `PODMAN_USERNS` environment variable. An empty value (\"\") means
user namespaces are disabled.

Rootless user \--userns=Key mappings:

  ---------------------------------------------------------------------------------------------------------------------------
  Key            Host User                                                                    Container User
  -------------- ---------------------------------------------------------------------------- -------------------------------
  \"\"           [*UID*\|0(*DefaultUseraccountmappedtorootuserincontainer*.)*host*\|]{.math   0 (Default User account mapped
                 .inline}UID                                                                  to root user in container.)

  keep-id        [*UID*\|]{.math .inline}UID (Map user account to same UID within container.) 

  auto           [*UID*\|*nil*(*HostUserUIDisnotmappedintocontainer*.)*nomap*\|]{.math        nil (Host User UID is not
                 .inline}UID                                                                  mapped into container.)
  ---------------------------------------------------------------------------------------------------------------------------

Valid *mode* values are:

-   *auto\[:**OPTIONS,\...**\]*: automatically create a namespace. It is
    possible to specify these options to `auto`:

    -   *gidmapping=*\_CONTAINER_GID:HOST_GID:SIZE\_ to force a GID
        mapping to be present in the user namespace.

    -   *size=*\_SIZE\_: to specify an explicit size for the automatic
        user namespace. e.g. `--userns=auto:size=8192`. If `size` is not
        specified, `auto` estimates the size for the user namespace.

    -   *uidmapping=*\_CONTAINER_UID:HOST_UID:SIZE\_ to force a UID
        mapping to be present in the user namespace.

-   *host*: run in the user namespace of the caller. The processes
    running in the container have the same privileges on the host as any
    other process launched by the calling user (default).

-   *keep-id*: creates a user namespace where the current rootless
    user\'s UID:GID are mapped to the same values in the container. This
    option is not allowed for containers created by the root user.

-   *nomap*: creates a user namespace where the current rootless user\'s
    UID:GID are not mapped into the container. This option is not
    allowed for containers created by the root user.

#### **\--uts**=*mode*

Set the UTS namespace mode for the pod. The following values are
supported:

-   **host**: use the host\'s UTS namespace inside the pod.
-   **private**: create a new namespace for the pod (default).
-   **ns:\[path\]**: run the pod in the given existing UTS namespace.

#### **\--volume**, **-v**=*\[\[SOURCE-VOLUME\|HOST-DIR:\]CONTAINER-DIR\[:OPTIONS\]\]*

Create a bind mount. If `-v /HOST-DIR:/CONTAINER-DIR` is specified,
Podman bind mounts `/HOST-DIR` from the host into `/CONTAINER-DIR` in
the Podman container. Similarly, `-v SOURCE-VOLUME:/CONTAINER-DIR`
mounts the named volume from the host into the container. If no such
named volume exists, Podman creates one. If no source is given, the
volume is created as an anonymously named volume with a randomly
generated name, and is removed when the pod is removed via the `--rm`
flag or the `podman rm --volumes` command.

(Note when using the remote client, including Mac and Windows (excluding
WSL2) machines, the volumes are mounted from the remote server, not
necessarily the client machine.)

The *OPTIONS* is a comma-separated list and can be one or more of:

-   **rw**\|**ro**
-   **z**\|**Z**
-   \[**O**\]
-   \[**U**\]
-   \[**no**\]**copy**
-   \[**no**\]**dev**
-   \[**no**\]**exec**
-   \[**no**\]**suid**
-   \[**r**\]**bind**
-   \[**r**\]**shared**\|\[**r**\]**slave**\|\[**r**\]**private**\[**r**\]**unbindable**
    ^[\[1\]](#Footnote1)^
-   **idmap**\[=**options**\]

The `CONTAINER-DIR` must be an absolute path such as `/src/docs`. The
volume is mounted into the container at this directory.

If a volume source is specified, it must be a path on the host or the
name of a named volume. Host paths are allowed to be absolute or
relative; relative paths are resolved relative to the directory Podman
is run in. If the source does not exist, Podman returns an error. Users
must pre-create the source files or directories.

Any source that does not begin with a `.` or `/` is treated as the name
of a named volume. If a volume with that name does not exist, it is
created. Volumes created with names are not anonymous, and they are not
removed by the `--rm` option and the `podman rm --volumes` command.

Specify multiple **-v** options to mount one or more volumes into a pod.

`Write Protected Volume Mounts`

Add **:ro** or **:rw** option to mount a volume in read-only or
read-write mode, respectively. By default, the volumes are mounted
read-write. See examples.

`Chowning Volume Mounts`

By default, Podman does not change the owner and group of source volume
directories mounted into containers. If a pod is created in a new user
namespace, the UID and GID in the container may correspond to another
UID and GID on the host.

The `:U` suffix tells Podman to use the correct host UID and GID based
on the UID and GID within the pod, to change recursively the owner and
group of the source volume. Chowning walks the file system under the
volume and changes the UID/GID on each file. If the volume has thousands
of inodes, this process takes a long time, delaying the start of the
pod.

**Warning** use with caution since this modifies the host filesystem.

`Labeling Volume Mounts`

Labeling systems like SELinux require that proper labels are placed on
volume content mounted into a pod. Without a label, the security system
might prevent the processes running inside the pod from using the
content. By default, Podman does not change the labels set by the OS.

To change a label in the pod context, add either of two suffixes **:z**
or **:Z** to the volume mount. These suffixes tell Podman to relabel
file objects on the shared volumes. The **z** option tells Podman that
two or more pods share the volume content. As a result, Podman labels
the content with a shared content label. Shared volume labels allow all
containers to read/write content. The **Z** option tells Podman to label
the content with a private unshared label Only the current pod can use a
private volume. Note: all containers within a `pod` share the same
SELinux label. This means all containers within said pod can read/write
volumes shared into the container created with the `:Z` on any of one
the containers. Relabeling walks the file system under the volume and
changes the label on each file, if the volume has thousands of inodes,
this process takes a long time, delaying the start of the pod. If the
volume was previously relabeled with the `z` option, Podman is optimized
to not relabel a second time. If files are moved into the volume, then
the labels can be manually change with the
`chcon -Rt container_file_t PATH` command.

Note: Do not relabel system files and directories. Relabeling system
content might cause other confined services on the machine to fail. For
these types of containers we recommend disabling SELinux separation. The
option **\--security-opt label=disable** disables SELinux separation for
the pod. For example if a user wanted to volume mount their entire home
directory into a pod, they need to disable SELinux separation.

    $ podman pod clone --security-opt label=disable -v $HOME:/home/user fedora touch /home/user/file

`Overlay Volume Mounts`

The `:O` flag tells Podman to mount the directory from the host as a
temporary storage using the `overlay file system`. The pod processes can
modify content within the mountpoint which is stored in the container
storage in a separate directory. In overlay terms, the source directory
is the lower, and the container storage directory is the upper.
Modifications to the mount point are destroyed when the pod finishes
executing, similar to a tmpfs mount point being unmounted.

For advanced users, the **overlay** option also supports custom
non-volatile **upperdir** and **workdir** for the overlay mount. Custom
**upperdir** and **workdir** can be fully managed by the users
themselves, and Podman does not remove it on lifecycle completion.
Example **:O,upperdir=/some/upper,workdir=/some/work**

Subsequent executions of the container sees the original source
directory content, any changes from previous pod executions no longer
exist.

One use case of the overlay mount is sharing the package cache from the
host into the container to allow speeding up builds.

Note: The `O` flag conflicts with other options listed above.

Content mounted into the container is labeled with the private label. On
SELinux systems, labels in the source directory must be readable by the
pod infra container label. Usually containers can read/execute
`container_share_t` and can read/write `container_file_t`. If unable to
change the labels on a source volume, SELinux container separation must
be disabled for the pod or infra container to work.

Do not modify the source directory mounted into the pod with an overlay
mount, it can cause unexpected failures. Only modify the directory after
the container finishes running.

`Mounts propagation`

By default, bind-mounted volumes are `private`. That means any mounts
done inside the pod are not visible on the host and vice versa. One can
change this behavior by specifying a volume mount propagation property.
When a volume is `shared`, mounts done under that volume inside the pod
are visible on host and vice versa. Making a volume
**slave**^[\[1\]](#Footnote1)^ enables only one-way mount propagation:
mounts done on the host under that volume are visible inside the
container but not the other way around.

To control mount propagation property of a volume one can use the
\[**r**\]**shared**, \[**r**\]**slave**, \[**r**\]**private** or the
\[**r**\]**unbindable** propagation flag. Propagation property can be
specified only for bind mounted volumes and not for internal volumes or
named volumes. For mount propagation to work the source mount point (the
mount point where source dir is mounted on) has to have the right
propagation properties. For shared volumes, the source mount point has
to be shared. And for slave volumes, the source mount point has to be
either shared or slave. ^[\[1\]](#Footnote1)^

To recursively mount a volume and all of its submounts into a pod, use
the **rbind** option. By default the bind option is used, and submounts
of the source directory is not mounted into the pod.

Mounting the volume with a **copy** option tells podman to copy content
from the underlying destination directory onto newly created internal
volumes. The **copy** only happens on the initial creation of the
volume. Content is not copied up when the volume is subsequently used on
different containers. The **copy** option is ignored on bind mounts and
has no effect.

Mounting volumes with the **nosuid** options means that SUID executables
on the volume can not be used by applications to change their privilege.
By default volumes are mounted with **nosuid**.

Mounting the volume with the **noexec** option means that no executables
on the volume can be executed within the pod.

Mounting the volume with the **nodev** option means that no devices on
the volume can be used by processes within the pod. By default volumes
are mounted with **nodev**.

If the *HOST-DIR* is a mount point, then **dev**, **suid**, and **exec**
options are ignored by the kernel.

Use **df HOST-DIR** to figure out the source mount, then use **findmnt
-o TARGET,PROPAGATION *source-mount-dir*** to figure out propagation
properties of source mount. If **findmnt**(1) utility is not available,
then one can look at the mount entry for the source mount point in
*/proc/self/mountinfo*. Look at the \"optional fields\" and see if any
propagation properties are specified. In there, **shared:N** means the
mount is shared, **master:N** means mount is slave, and if nothing is
there, the mount is private. ^[\[1\]](#Footnote1)^

To change propagation properties of a mount point, use **mount**(8)
command. For example, if one wants to bind mount source directory
*/foo*, one can do **mount \--bind /foo /foo** and **mount
\--make-private \--make-shared /foo**. This converts /foo into a shared
mount point. Alternatively, one can directly change propagation
properties of source mount. Say */* is source mount for */foo*, then use
**mount \--make-shared /** to convert */* into a shared mount.

Note: if the user only has access rights via a group, accessing the
volume from inside a rootless pod fails.

`Idmapped mount`

If `idmap` is specified, create an idmapped mount to the target user
namespace in the container. The idmap option supports a custom mapping
that can be different than the user namespace used by the container. The
mapping can be specified after the idmap option like:
`idmap=uids=0-1-10#10-11-10;gids=0-100-10`. For each triplet, the first
value is the start of the backing file system IDs that are mapped to the
second value on the host. The length of this mapping is given in the
third value. Multiple ranges are separated with #.

#### **\--volumes-from**=*CONTAINER\[:OPTIONS\]*

Mount volumes from the specified container(s). Used to share volumes
between containers and pods. The *options* is a comma-separated list
with the following available elements:

-   **rw**\|**ro**
-   **z**

Mounts already mounted volumes from a source container onto another pod.
*CONTAINER* may be a name or ID. To share a volume, use the
\--volumes-from option when running the target container. Volumes can be
shared even if the source container is not running.

By default, Podman mounts the volumes in the same mode (read-write or
read-only) as it is mounted in the source container. This can be changed
by adding a `ro` or `rw` *option*.

Labeling systems like SELinux require that proper labels are placed on
volume content mounted into a pod. Without a label, the security system
might prevent the processes running inside the container from using the
content. By default, Podman does not change the labels set by the OS.

To change a label in the pod context, add `z` to the volume mount. This
suffix tells Podman to relabel file objects on the shared volumes. The
`z` option tells Podman that two entities share the volume content. As a
result, Podman labels the content with a shared content label. Shared
volume labels allow all containers to read/write content.

If the location of the volume from the source container overlaps with
data residing on a target pod, then the volume hides that data on the
target.

##  EXAMPLES

Clone the specified pod to a new pod.

    # podman pod clone pod-name
    6b2c73ff8a1982828c9ae2092954bcd59836a131960f7e05221af9df5939c584

Clone the specified pod to a new pod with a new name.

    # podman pod clone pod-name --name=cloned-pod
    d0cf1f782e2ed67e8c0050ff92df865a039186237a4df24d7acba5b1fa8cc6e7
    6b2c73ff8a1982828c9ae2092954bcd59836a131960f7e05221af9df5939c584

Clone and remove the specified pod to a new pod, modifying its cpus.

    # podman pod clone --destroy --cpus=5 d0cf1
    6b2c73ff8a1982828c9ae2092954bcd59836a131960f7e05221af9df5939c584

Clone the specified pod to a new named pod.

    # podman pod clone 2d4d4fca7219b4437e0d74fcdc272c4f031426a6eacd207372691207079551de new_name
    5a9b7851013d326aa4ac4565726765901b3ecc01fcbc0f237bc7fd95588a24f9

##  SEE ALSO

**[podman-pod-create(1)](podman-pod-create.html)**

##  HISTORY

May 2022, Originally written by Charlie Doern <cdoern@redhat.com>

##  FOOTNOTES

[1]{#Footnote1}: The Podman project is committed to inclusivity, a core
value of open source. The `master` and `slave` mount propagation
terminology used here is problematic and divisive, and needs to be
changed. However, these terms are currently used within the Linux kernel
and must be used as-is at this time. When the kernel maintainers rectify
this usage, Podman will follow suit immediately.


---

<a id='podman-pod-create'></a>

## podman-pod-create - Create a new pod

##  NAME

podman-pod-create - Create a new pod

##  SYNOPSIS

**podman pod create** \[*options*\] \[*name*\]

##  DESCRIPTION

Creates an empty pod, or unit of multiple containers, and prepares it to
have containers added to it. The pod can be created with a specific
name. If a name is not given a random name is generated. The pod ID is
printed to STDOUT. You can then use **podman create \--pod
`<pod_id|pod_name>` \...** to add containers to the pod, and **podman
pod start `<pod_id|pod_name>`** to start the pod.

The operator can identify a pod in three ways: UUID long identifier
("f78375b1c487e03c9438c729345e54db9d20cfa2ac1fc3494b6eb60872e74778")
UUID short identifier ("f78375b1c487") Name ("jonah")

podman generates a UUID for each pod, and if a name is not assigned to
the container with **\--name** then a random string name is generated
for it. This name is useful to identify a pod.

Note: resource limit related flags work by setting the limits explicitly
in the pod\'s cgroup parent for all containers joining the pod. A
container can override the resource limits when joining a pod. For
example, if a pod was created via **podman pod create \--cpus=5**,
specifying **podman container create \--pod=`<pod_id|pod_name>`
\--cpus=4** causes the container to use the smaller limit. Also,
containers which specify their own cgroup, such as **\--cgroupns=host**,
do NOT get the assigned pod level cgroup resources.

##  OPTIONS

#### **\--add-host**=*hostname\[;hostname\[;\...\]\]*:*ip*

Add a custom host-to-IP mapping to the pod\'s `/etc/hosts` file.

The option takes one or multiple semicolon-separated hostnames to be
mapped to a single IPv4 or IPv6 address, separated by a colon. It can
also be used to overwrite the IP addresses of hostnames Podman adds to
`/etc/hosts` by default (also see the **\--name** and **\--hostname**
options). This option can be specified multiple times to add additional
mappings to `/etc/hosts`. It conflicts with the **\--no-hosts** option
and conflicts with *no_hosts=true* in `containers.conf`.

Instead of an IP address, the special flag *host-gateway* can be given.
This resolves to an IP address the container can use to connect to the
host. The IP address chosen depends on your network setup, thus there\'s
no guarantee that Podman can determine the *host-gateway* address
automatically, which will then cause Podman to fail with an error
message. You can overwrite this IP address using the
*host_containers_internal_ip* option in *containers.conf*.

The *host-gateway* address is also used by Podman to automatically add
the `host.containers.internal` and `host.docker.internal` hostnames to
`/etc/hosts`. You can prevent that by either giving the **\--no-hosts**
option, or by setting *host_containers_internal_ip=\"none\"* in
*containers.conf*. If no *host-gateway* address was configured manually
and Podman fails to determine the IP address automatically, Podman will
silently skip adding these internal hostnames to `/etc/hosts`. If Podman
is running in a virtual machine using `podman machine` (this includes
Mac and Windows hosts), Podman will silently skip adding the internal
hostnames to `/etc/hosts`, unless an IP address was configured manually;
the internal hostnames are resolved by the gvproxy DNS resolver instead.

Podman will use the `/etc/hosts` file of the host as a basis by default,
i.e. any hostname present in this file will also be present in the
`/etc/hosts` file of the container. A different base file can be
configured using the *base_hosts_file* config in `containers.conf`.

The /etc/hosts file is shared between all containers in the pod.

#### **\--blkio-weight**=*weight*

Block IO relative weight. The *weight* is a value between **10** and
**1000**.

This option is not supported on cgroups V1 rootless systems.

#### **\--blkio-weight-device**=*device:weight*

Block IO relative device weight.

#### **\--cgroup-parent**=*path*

Path to cgroups under which the cgroup for the pod is created. If the
path is not absolute, the path is considered to be relative to the
cgroups path of the init process. Cgroups are created if they do not
already exist.

#### **\--cpu-shares**, **-c**=*shares*

CPU shares (relative weight).

By default, all containers get the same proportion of CPU cycles. This
proportion can be modified by changing the container\'s CPU share
weighting relative to the combined weight of all the running containers.
Default weight is **1024**.

The proportion only applies when CPU-intensive processes are running.
When tasks in one container are idle, other containers can use the
left-over CPU time. The actual amount of CPU time varies depending on
the number of containers running on the system.

For example, consider three containers, one has a cpu-share of 1024 and
two others have a cpu-share setting of 512. When processes in all three
containers attempt to use 100% of CPU, the first container receives 50%
of the total CPU time. If a fourth container is added with a cpu-share
of 1024, the first container only gets 33% of the CPU. The remaining
containers receive 16.5%, 16.5% and 33% of the CPU.

On a multi-core system, the shares of CPU time are distributed over all
CPU cores. Even if a container is limited to less than 100% of CPU time,
it can use 100% of each individual CPU core.

For example, consider a system with more than three cores. If the
container *C0* is started with **\--cpu-shares=512** running one
process, and another container *C1* with **\--cpu-shares=1024** running
two processes, this can result in the following division of CPU shares:

  PID   container   CPU   CPU share
  ----- ----------- ----- --------------
  100   C0          0     100% of CPU0
  101   C1          1     100% of CPU1
  102   C1          2     100% of CPU2

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--cpus**=*amount*

Set the total number of CPUs delegated to the pod. Default is 0.000
which indicates that there is no limit on computation power.

#### **\--cpuset-cpus**=*number*

CPUs in which to allow execution. Can be specified as a comma-separated
list (e.g. **0,1**), as a range (e.g. **0-3**), or any combination
thereof (e.g. **0-3,7,11-15**).

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--cpuset-mems**=*nodes*

Memory nodes (MEMs) in which to allow execution (0-3, 0,1). Only
effective on NUMA systems.

If there are four memory nodes on the system (0-3), use
**\--cpuset-mems=0,1** then processes in the container only uses memory
from the first two memory nodes.

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--device**=*host-device\[:container-device\]\[:permissions\]*

Add a host device to the pod. Optional *permissions* parameter can be
used to specify device permissions by combining **r** for read, **w**
for write, and **m** for **mknod**(2).

Example: **\--device=/dev/sdc:/dev/xvdc:rwm**.

Note: if *host-device* is a symbolic link then it is resolved first. The
pod only stores the major and minor numbers of the host device.

Podman may load kernel modules required for using the specified device.
The devices that Podman loads modules for when necessary are: /dev/fuse.

In rootless mode, the new device is bind mounted in the container from
the host rather than Podman creating it within the container space.
Because the bind mount retains its SELinux label on SELinux systems, the
container can get permission denied when accessing the mounted device.
Modify SELinux settings to allow containers to use all device labels via
the following command:

\$ sudo setsebool -P container_use_devices=true

Note: the pod implements devices by storing the initial configuration
passed by the user and recreating the device on each container added to
the pod.

#### **\--device-read-bps**=*path:rate*

Limit read rate (in bytes per second) from a device (e.g.
**\--device-read-bps=/dev/sda:1mb**).

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--device-write-bps**=*path:rate*

Limit write rate (in bytes per second) to a device (e.g.
**\--device-write-bps=/dev/sda:1mb**).

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--dns**=*ipaddr*

Set custom DNS servers in the /etc/resolv.conf file that is shared
between all containers in the pod. A special option, \"none\" is allowed
which disables creation of /etc/resolv.conf for the pod.

#### **\--dns-option**=*option*

Set custom DNS options in the /etc/resolv.conf file that is shared
between all containers in the pod.

#### **\--dns-search**=*domain*

Set custom DNS search domains in the /etc/resolv.conf file that is
shared between all containers in the pod.

#### **\--exit-policy**=**continue** \| *stop*

Set the exit policy of the pod when the last container exits. Supported
policies are:

  --------------------------------------------------------------------------
  Exit Policy  Description
  ------------ -------------------------------------------------------------
  *continue*   The pod continues running, by keeping its infra container
               alive, when the last container exits. Used by default.

  *stop*       The pod (including its infra container) is stopped when the
               last container exits. Used in `kube play`.
  --------------------------------------------------------------------------

#### **\--gidmap**=*pod_gid:host_gid:amount*

GID map for the user namespace. Using this flag runs all containers in
the pod with user namespace enabled. It conflicts with the **\--userns**
and **\--subgidname** flags.

#### **\--gpus**=*ENTRY*

GPU devices to add to the container (\'all\' to pass all GPUs) Currently
only Nvidia devices are supported.

#### **\--help**, **-h**

Print usage statement.

#### **\--hostname**=*name*

Set the pod\'s hostname inside all containers.

The given hostname is also added to the `/etc/hosts` file using the
container\'s primary IP address (also see the **\--add-host** option).

#### **\--infra**

Create an infra container and associate it with the pod. An infra
container is a lightweight container used to coordinate the shared
kernel namespace of a pod. Default: true.

#### **\--infra-command**=*command*

The command that is run to start the infra container. Default:
\"/pause\".

#### **\--infra-conmon-pidfile**=*file*

Write the pid of the infra container\'s **conmon** process to a file. As
**conmon** runs in a separate process than Podman, this is necessary
when using systemd to manage Podman containers and pods.

#### **\--infra-image**=*image*

The custom image that is used for the infra container. Unless specified,
Podman builds a custom local image which does not require pulling down
an image.

#### **\--infra-name**=*name*

The name that is used for the pod\'s infra container.

#### **\--ip**=*ipv4*

Specify a static IPv4 address for the pod, for example **10.88.64.128**.
This option can only be used if the pod is joined to only a single
network - i.e., **\--network=network-name** is used at most once - and
if the pod is not joining another container\'s network namespace via
**\--network=container:*id***. The address must be within the network\'s
IP address pool (default **10.88.0.0/16**).

To specify multiple static IP addresses per pod, set multiple networks
using the **\--network** option with a static IP address specified for
each using the `ip` mode for that option.

#### **\--ip6**=*ipv6*

Specify a static IPv6 address for the pod, for example
**fd46:db93:aa76:ac37::10**. This option can only be used if the pod is
joined to only a single network - i.e., **\--network=network-name** is
used at most once - and if the pod is not joining another container\'s
network namespace via **\--network=container:*id***. The address must be
within the network\'s IPv6 address pool.

To specify multiple static IPv6 addresses per pod, set multiple networks
using the **\--network** option with a static IPv6 address specified for
each using the `ip6` mode for that option.

#### **\--label**, **-l**=*key=value*

Add metadata to a pod.

#### **\--label-file**=*file*

Read in a line-delimited file of labels.

#### **\--mac-address**=*address*

Pod network interface MAC address (e.g. 92:d0:c6:0a:29:33) This option
can only be used if the pod is joined to only a single network - i.e.,
**\--network=*network-name*** is used at most once - and if the pod is
not joining another container\'s network namespace via
**\--network=container:*id***.

Remember that the MAC address in an Ethernet network must be unique. The
IPv6 link-local address is based on the device\'s MAC address according
to RFC4862.

To specify multiple static MAC addresses per pod, set multiple networks
using the **\--network** option with a static MAC address specified for
each using the `mac` mode for that option.

#### **\--memory**, **-m**=*number\[unit\]*

Memory limit. A *unit* can be **b** (bytes), **k** (kibibytes), **m**
(mebibytes), or **g** (gibibytes).

Allows the memory available to a container to be constrained. If the
host supports swap memory, then the **-m** memory setting can be larger
than physical RAM. If a limit of 0 is specified (not using **-m**), the
container\'s memory is not limited. The actual limit may be rounded up
to a multiple of the operating system\'s page size (the value is very
large, that\'s millions of trillions).

This option is not supported on cgroups V1 rootless systems.

#### **\--memory-swap**=*number\[unit\]*

A limit value equal to memory plus swap. A *unit* can be **b** (bytes),
**k** (kibibytes), **m** (mebibytes), or **g** (gibibytes).

Must be used with the **-m** (**\--memory**) flag. The argument value
must be larger than that of **-m** (**\--memory**) By default, it is set
to double the value of **\--memory**.

Set *number* to **-1** to enable unlimited swap.

This option is not supported on cgroups V1 rootless systems.

#### **\--name**, **-n**=*name*

Assign a name to the pod.

#### **\--network**=*mode*, **\--net**

Set the network mode for the pod.

Valid *mode* values are:

-   **bridge\[:OPTIONS,\...\]**: Create a network stack on the default
    bridge. This is the default for rootful containers. It is possible
    to specify these additional options:

    -   **alias=**\_name\_: Add network-scoped alias for the container.
    -   **ip=**\_IPv4\_: Specify a static IPv4 address for this
        container.
    -   **ip6=**\_IPv6\_: Specify a static IPv6 address for this
        container.
    -   **mac=**\_MAC\_: Specify a static MAC address for this
        container.
    -   **interface_name=**\_name\_: Specify a name for the created
        network interface inside the container.

    For example, to set a static ipv4 address and a static mac address,
    use `--network bridge:ip=10.88.0.10,mac=44:33:22:11:00:99`.

-   *\<network name or ID\>***\[:OPTIONS,\...\]**: Connect to a
    user-defined network; this is the network name or ID from a network
    created by **[podman network create](podman-network-create.html)**.
    It is possible to specify the same options described under the
    bridge mode above. Use the **\--network** option multiple times to
    specify additional networks.\
    For backwards compatibility it is also possible to specify
    comma-separated networks on the first **\--network** argument,
    however this prevents you from using the options described under the
    bridge section above.

-   **none**: Create a network namespace for the container but do not
    configure network interfaces for it, thus the container has no
    network connectivity.

-   **container:**\_id\_: Reuse another container\'s network stack.

-   **host**: Do not create a network namespace, the container uses the
    host\'s network. Note: The host mode gives the container full access
    to local system services such as D-bus and is therefore considered
    insecure.

-   **ns:**\_path\_: Path to a network namespace to join.

-   **private**: Create a new namespace for the container. This uses the
    **bridge** mode for rootful containers and **slirp4netns** for
    rootless ones.

-   **slirp4netns\[:OPTIONS,\...\]**: use **slirp4netns**(1) to create a
    user network stack. It is possible to specify these additional
    options, they can also be set with `network_cmd_options` in
    containers.conf:

    -   **allow_host_loopback=true\|false**: Allow slirp4netns to reach
        the host loopback IP (default is 10.0.2.2 or the second IP from
        slirp4netns cidr subnet when changed, see the cidr option
        below). The default is false.
    -   **mtu=**\_MTU\_: Specify the MTU to use for this network.
        (Default is `65520`).
    -   **cidr=**\_CIDR\_: Specify ip range to use for this network.
        (Default is `10.0.2.0/24`).
    -   **enable_ipv6=true\|false**: Enable IPv6. Default is true.
        (Required for `outbound_addr6`).
    -   **outbound_addr=**\_INTERFACE\_: Specify the outbound interface
        slirp binds to (ipv4 traffic only).
    -   **outbound_addr=**\_IPv4\_: Specify the outbound ipv4 address
        slirp binds to.
    -   **outbound_addr6=**\_INTERFACE\_: Specify the outbound interface
        slirp binds to (ipv6 traffic only).
    -   **outbound_addr6=**\_IPv6\_: Specify the outbound ipv6 address
        slirp binds to.
    -   **port_handler=rootlesskit**: Use rootlesskit for port
        forwarding. Default.\
        Note: Rootlesskit changes the source IP address of incoming
        packets to an IP address in the container network namespace,
        usually `10.0.2.100`. If the application requires the real
        source IP address, e.g. web server logs, use the slirp4netns
        port handler. The rootlesskit port handler is also used for
        rootless containers when connected to user-defined networks.
    -   **port_handler=slirp4netns**: Use the slirp4netns port
        forwarding, it is slower than rootlesskit but preserves the
        correct source IP address. This port handler cannot be used for
        user-defined networks.

-   **pasta\[:OPTIONS,\...\]**: use **pasta**(1) to create a user-mode
    networking stack.\
    This is the default for rootless containers and only supported in
    rootless mode.\
    By default, IPv4 and IPv6 addresses and routes, as well as the pod
    interface name, are copied from the host. If port forwarding isn\'t
    configured, ports are forwarded dynamically as services are bound on
    either side (init namespace or container namespace). Port forwarding
    preserves the original source IP address. Options described in
    pasta(1) can be specified as comma-separated arguments.\
    In terms of pasta(1) options, **\--config-net** is given by default,
    in order to configure networking when the container is started, and
    **\--no-map-gw** is also assumed by default, to avoid direct access
    from container to host using the gateway address. The latter can be
    overridden by passing **\--map-gw** in the pasta-specific options
    (despite not being an actual pasta(1) option).\
    Also, **-t none** and **-u none** are passed if, respectively, no
    TCP or UDP port forwarding from host to container is configured, to
    disable automatic port forwarding based on bound ports. Similarly,
    **-T none** and **-U none** are given to disable the same
    functionality from container to host.\
    Some examples:

    -   **pasta:\--map-gw**: Allow the container to directly reach the
        host using the gateway address.
    -   **pasta:\--mtu,1500**: Specify a 1500 bytes MTU for the *tap*
        interface in the container.
    -   **pasta:\--ipv4-only,-a,10.0.2.0,-n,24,-g,10.0.2.2,\--dns-forward,10.0.2.3,-m,1500,\--no-ndp,\--no-dhcpv6,\--no-dhcp**,
        equivalent to default slirp4netns(1) options: disable IPv6,
        assign `10.0.2.0/24` to the `tap0` interface in the container,
        with gateway `10.0.2.3`, enable DNS forwarder reachable at
        `10.0.2.3`, set MTU to 1500 bytes, disable NDP, DHCPv6 and DHCP
        support.
    -   **pasta:-I,tap0,\--ipv4-only,-a,10.0.2.0,-n,24,-g,10.0.2.2,\--dns-forward,10.0.2.3,\--no-ndp,\--no-dhcpv6,\--no-dhcp**,
        equivalent to default slirp4netns(1) options with Podman
        overrides: same as above, but leave the MTU to 65520 bytes
    -   **pasta:-t,auto,-u,auto,-T,auto,-U,auto**: enable automatic port
        forwarding based on observed bound ports from both host and
        container sides
    -   **pasta:-T,5201**: enable forwarding of TCP port 5201 from
        container to host, using the loopback interface instead of the
        tap interface for improved performance

Invalid if using **\--dns**, **\--dns-option**, or **\--dns-search**
with **\--network** set to **none** or **container:**\_id\_.

#### **\--network-alias**=*alias*

Add a network-scoped alias for the pod, setting the alias for all
networks that the container joins. To set a name only for a specific
network, use the alias option as described under the **\--network**
option. If the network has DNS enabled
(`podman network inspect -f {{.DNSEnabled}} <name>`), these aliases can
be used for name resolution on the given network. This option can be
specified multiple times. NOTE: When using CNI a pod only has access to
aliases on the first network that it joins. This limitation does not
exist with netavark/aardvark-dns.

#### **\--no-hosts**

Do not modify the `/etc/hosts` file in the pod.

Podman assumes control over the pod\'s `/etc/hosts` file by default and
adds entries for the container\'s name (see **\--name** option) and
hostname (see **\--hostname** option), the internal
`host.containers.internal` and `host.docker.internal` hosts, as well as
any hostname added using the **\--add-host** option. Refer to the
**\--add-host** option for details. Passing **\--no-hosts** disables
this, so that the image\'s `/etc/hosts` file is kept unmodified. The
same can be achieved globally by setting *no_hosts=true* in
`containers.conf`.

This option conflicts with **\--add-host**.

#### **\--pid**=*pid*

Set the PID mode for the pod. The default is to create a private PID
namespace for the pod. Requires the PID namespace to be shared via
\--share.

    host: use the host’s PID namespace for the pod
    ns: join the specified PID namespace
    private: create a new namespace for the pod (default)

#### **\--pod-id-file**=*path*

Write the pod ID to the file.

#### **\--publish**, **-p**=*\[\[ip:\]\[hostPort\]:\]containerPort\[/protocol\]*

Publish a container\'s port, or range of ports, within this pod to the
host.

Both *hostPort* and *containerPort* can be specified as a range of
ports. When specifying ranges for both, the number of container ports in
the range must match the number of host ports in the range.

If host IP is set to 0.0.0.0 or not set at all, the port is bound on all
IPs on the host.

By default, Podman publishes TCP ports. To publish a UDP port instead,
give `udp` as protocol. To publish both TCP and UDP ports, set
`--publish` twice, with `tcp`, and `udp` as protocols respectively.
Rootful containers can also publish ports using the `sctp` protocol.

Host port does not have to be specified (e.g.
`podman run -p 127.0.0.1::80`). If it is not, the container port is
randomly assigned a port on the host.

Use **podman port** to see the actual mapping:
`podman port $CONTAINER $CONTAINERPORT`.

Note that the network drivers `macvlan` and `ipvlan` do not support port
forwarding, it will have no effect on these networks.

**Note:** You must not publish ports of containers in the pod
individually, but only by the pod itself.

**Note:** This cannot be modified once the pod is created.

#### **\--replace**

If another pod with the same name already exists, replace and remove it.
The default is **false**.

#### **\--restart**=*policy*

Restart policy to follow when containers exit. Restart policy does not
take effect if a container is stopped via the **podman kill** or
**podman stop** commands.

Valid *policy* values are:

-   `no` : Do not restart containers on exit
-   `never` : Synonym for **no**; do not restart containers on exit
-   `on-failure[:max_retries]` : Restart containers when they exit with
    a non-zero exit code, retrying indefinitely or until the optional
    *max_retries* count is hit
-   `always` : Restart containers when they exit, regardless of status,
    retrying indefinitely
-   `unless-stopped` : Identical to **always**

Podman provides a systemd unit file, podman-restart.service, which
restarts containers after a system reboot.

When running containers in systemd services, use the restart
functionality provided by systemd. In other words, do not use this
option in a container unit, instead set the `Restart=` systemd directive
in the `[Service]` section. See **podman-systemd.unit**(5) and
**systemd.service**(5).

Default restart policy for all the containers in a pod.

#### **\--security-opt**=*option*

Security Options

-   **apparmor=unconfined** : Turn off apparmor confinement for the pod

-   **apparmor**=*alternate-profile* : Set the apparmor confinement
    profile for the pod

-   **label=user:**\_USER\_: Set the label user for the pod processes

-   **label=role:**\_ROLE\_: Set the label role for the pod processes

-   **label=type:**\_TYPE\_: Set the label process type for the pod
    processes

-   **label=level:**\_LEVEL\_: Set the label level for the pod processes

-   **label=filetype:**\_TYPE\_: Set the label file type for the pod
    files

-   **label=disable**: Turn off label separation for the pod

Note: Labeling can be disabled for all pods/containers by setting
label=false in the **containers.conf**
(`/etc/containers/containers.conf` or
`$HOME/.config/containers/containers.conf`) file.

-   **label=nested**: Allows SELinux modifications within the container.
    Containers are allowed to modify SELinux labels on files and
    processes, as long as SELinux policy allows. Without **nested**,
    containers view SELinux as disabled, even when it is enabled on the
    host. Containers are prevented from setting any labels.

-   **mask**=*/path/1:/path/2*: The paths to mask separated by a colon.
    A masked path cannot be accessed inside the containers within the
    pod.

-   **no-new-privileges**: Disable container processes from gaining
    additional privileges.

-   **seccomp=unconfined**: Turn off seccomp confinement for the pod.

-   **seccomp=profile.json**: JSON file to be used as a seccomp filter.
    Note that the `io.podman.annotations.seccomp` annotation is set with
    the specified value as shown in `podman inspect`.

-   **proc-opts**=*OPTIONS* : Comma-separated list of options to use for
    the /proc mount. More details for the possible mount options are
    specified in the **proc(5)** man page.

-   **unmask**=*ALL* or */path/1:/path/2*, or shell expanded paths
    (/proc/\*): Paths to unmask separated by a colon. If set to **ALL**,
    it unmasks all the paths that are masked or made read-only by
    default. The default masked paths are **/proc/acpi, /proc/kcore,
    /proc/keys, /proc/latency_stats, /proc/sched_debug, /proc/scsi,
    /proc/timer_list, /proc/timer_stats, /sys/firmware, and
    /sys/fs/selinux**, **/sys/devices/virtual/powercap**. The default
    paths that are read-only are **/proc/asound**, **/proc/bus**,
    **/proc/fs**, **/proc/irq**, **/proc/sys**, **/proc/sysrq-trigger**,
    **/sys/fs/cgroup**.

Note: Labeling can be disabled for all containers by setting
**label=false** in the **containers.conf**(5) file.

#### **\--share**=*namespace*

A comma-separated list of kernel namespaces to share. If none or \"\" is
specified, no namespaces are shared, and the infra container is not
created unless explicitly specified via **\--infra=true**. The
namespaces to choose from are cgroup, ipc, net, pid, uts. If the option
is prefixed with a \"+\", the namespace is appended to the default list.
Otherwise, it replaces the default list. Defaults match Kubernetes
default (ipc, net, uts)

#### **\--share-parent**

This boolean determines whether or not all containers entering the pod
use the pod as their cgroup parent. The default value of this option is
true. Use the **\--share** option to share the cgroup namespace rather
than a cgroup parent in a pod.

Note: This option conflicts with the **\--share=cgroup** option since
that option sets the pod as the cgroup parent but enters the container
into the same cgroupNS as the infra container.

#### **\--shm-size**=*number\[unit\]*

Size of */dev/shm*. A *unit* can be **b** (bytes), **k** (kibibytes),
**m** (mebibytes), or **g** (gibibytes). If the unit is omitted, the
system uses bytes. If the size is omitted, the default is **64m**. When
*size* is **0**, there is no limit on the amount of memory used for IPC
by the pod. This option conflicts with **\--ipc=host**.

#### **\--shm-size-systemd**=*number\[unit\]*

Size of systemd-specific tmpfs mounts such as /run, /run/lock,
/var/log/journal and /tmp. A *unit* can be **b** (bytes), **k**
(kibibytes), **m** (mebibytes), or **g** (gibibytes). If the unit is
omitted, the system uses bytes. If the size is omitted, the default is
**64m**. When *size* is **0**, the usage is limited to 50% of the
host\'s available memory.

#### **\--subgidname**=*name*

Run the container in a new user namespace using the map with *name* in
the */etc/subgid* file. If running rootless, the user needs to have the
right to use the mapping. See **subgid**(5). This flag conflicts with
**\--userns** and **\--gidmap**.

#### **\--subuidname**=*name*

Run the container in a new user namespace using the map with *name* in
the */etc/subuid* file. If running rootless, the user needs to have the
right to use the mapping. See **subuid**(5). This flag conflicts with
**\--userns** and **\--uidmap**.

#### **\--sysctl**=*name=value*

Configure namespaced kernel parameters for all containers in the pod.

For the IPC namespace, the following sysctls are allowed:

-   kernel.msgmax
-   kernel.msgmnb
-   kernel.msgmni
-   kernel.sem
-   kernel.shmall
-   kernel.shmmax
-   kernel.shmmni
-   kernel.shm_rmid_forced
-   Sysctls beginning with fs.mqueue.\*

Note: if the ipc namespace is not shared within the pod, the above
sysctls are not allowed.

For the network namespace, only sysctls beginning with net.\* are
allowed.

Note: if the network namespace is not shared within the pod, the above
sysctls are not allowed.

#### **\--uidmap**=*container_uid:from_uid:amount*

Run all containers in the pod in a new user namespace using the supplied
mapping. This option conflicts with the **\--userns** and
**\--subuidname** options. This option provides a way to map host UIDs
to container UIDs. It can be passed several times to map different
ranges.

#### **\--userns**=*mode*

Set the user namespace mode for all the containers in a pod. It defaults
to the `PODMAN_USERNS` environment variable. An empty value (\"\") means
user namespaces are disabled.

Rootless user \--userns=Key mappings:

  ---------------------------------------------------------------------------------------------------------------------------
  Key            Host User                                                                    Container User
  -------------- ---------------------------------------------------------------------------- -------------------------------
  \"\"           [*UID*\|0(*DefaultUseraccountmappedtorootuserincontainer*.)*host*\|]{.math   0 (Default User account mapped
                 .inline}UID                                                                  to root user in container.)

  keep-id        [*UID*\|]{.math .inline}UID (Map user account to same UID within container.) 

  auto           [*UID*\|*nil*(*HostUserUIDisnotmappedintocontainer*.)*nomap*\|]{.math        nil (Host User UID is not
                 .inline}UID                                                                  mapped into container.)
  ---------------------------------------------------------------------------------------------------------------------------

Valid *mode* values are:

-   *auto\[:**OPTIONS,\...**\]*: automatically create a namespace. It is
    possible to specify these options to `auto`:

    -   *gidmapping=*\_CONTAINER_GID:HOST_GID:SIZE\_ to force a GID
        mapping to be present in the user namespace.

    -   *size=*\_SIZE\_: to specify an explicit size for the automatic
        user namespace. e.g. `--userns=auto:size=8192`. If `size` is not
        specified, `auto` estimates the size for the user namespace.

    -   *uidmapping=*\_CONTAINER_UID:HOST_UID:SIZE\_ to force a UID
        mapping to be present in the user namespace.

-   *host*: run in the user namespace of the caller. The processes
    running in the container have the same privileges on the host as any
    other process launched by the calling user (default).

-   *keep-id*: creates a user namespace where the current rootless
    user\'s UID:GID are mapped to the same values in the container. This
    option is not allowed for containers created by the root user.

-   *nomap*: creates a user namespace where the current rootless user\'s
    UID:GID are not mapped into the container. This option is not
    allowed for containers created by the root user.

#### **\--uts**=*mode*

Set the UTS namespace mode for the pod. The following values are
supported:

-   **host**: use the host\'s UTS namespace inside the pod.
-   **private**: create a new namespace for the pod (default).
-   **ns:\[path\]**: run the pod in the given existing UTS namespace.

#### **\--volume**, **-v**=*\[\[SOURCE-VOLUME\|HOST-DIR:\]CONTAINER-DIR\[:OPTIONS\]\]*

Create a bind mount. If `-v /HOST-DIR:/CONTAINER-DIR` is specified,
Podman bind mounts `/HOST-DIR` from the host into `/CONTAINER-DIR` in
the Podman container. Similarly, `-v SOURCE-VOLUME:/CONTAINER-DIR`
mounts the named volume from the host into the container. If no such
named volume exists, Podman creates one. If no source is given, the
volume is created as an anonymously named volume with a randomly
generated name, and is removed when the pod is removed via the `--rm`
flag or the `podman rm --volumes` command.

(Note when using the remote client, including Mac and Windows (excluding
WSL2) machines, the volumes are mounted from the remote server, not
necessarily the client machine.)

The *OPTIONS* is a comma-separated list and can be one or more of:

-   **rw**\|**ro**
-   **z**\|**Z**
-   \[**O**\]
-   \[**U**\]
-   \[**no**\]**copy**
-   \[**no**\]**dev**
-   \[**no**\]**exec**
-   \[**no**\]**suid**
-   \[**r**\]**bind**
-   \[**r**\]**shared**\|\[**r**\]**slave**\|\[**r**\]**private**\[**r**\]**unbindable**
    ^[\[1\]](#Footnote1)^
-   **idmap**\[=**options**\]

The `CONTAINER-DIR` must be an absolute path such as `/src/docs`. The
volume is mounted into the container at this directory.

If a volume source is specified, it must be a path on the host or the
name of a named volume. Host paths are allowed to be absolute or
relative; relative paths are resolved relative to the directory Podman
is run in. If the source does not exist, Podman returns an error. Users
must pre-create the source files or directories.

Any source that does not begin with a `.` or `/` is treated as the name
of a named volume. If a volume with that name does not exist, it is
created. Volumes created with names are not anonymous, and they are not
removed by the `--rm` option and the `podman rm --volumes` command.

Specify multiple **-v** options to mount one or more volumes into a pod.

`Write Protected Volume Mounts`

Add **:ro** or **:rw** option to mount a volume in read-only or
read-write mode, respectively. By default, the volumes are mounted
read-write. See examples.

`Chowning Volume Mounts`

By default, Podman does not change the owner and group of source volume
directories mounted into containers. If a pod is created in a new user
namespace, the UID and GID in the container may correspond to another
UID and GID on the host.

The `:U` suffix tells Podman to use the correct host UID and GID based
on the UID and GID within the pod, to change recursively the owner and
group of the source volume. Chowning walks the file system under the
volume and changes the UID/GID on each file. If the volume has thousands
of inodes, this process takes a long time, delaying the start of the
pod.

**Warning** use with caution since this modifies the host filesystem.

`Labeling Volume Mounts`

Labeling systems like SELinux require that proper labels are placed on
volume content mounted into a pod. Without a label, the security system
might prevent the processes running inside the pod from using the
content. By default, Podman does not change the labels set by the OS.

To change a label in the pod context, add either of two suffixes **:z**
or **:Z** to the volume mount. These suffixes tell Podman to relabel
file objects on the shared volumes. The **z** option tells Podman that
two or more pods share the volume content. As a result, Podman labels
the content with a shared content label. Shared volume labels allow all
containers to read/write content. The **Z** option tells Podman to label
the content with a private unshared label Only the current pod can use a
private volume. Note: all containers within a `pod` share the same
SELinux label. This means all containers within said pod can read/write
volumes shared into the container created with the `:Z` on any of one
the containers. Relabeling walks the file system under the volume and
changes the label on each file, if the volume has thousands of inodes,
this process takes a long time, delaying the start of the pod. If the
volume was previously relabeled with the `z` option, Podman is optimized
to not relabel a second time. If files are moved into the volume, then
the labels can be manually change with the
`chcon -Rt container_file_t PATH` command.

Note: Do not relabel system files and directories. Relabeling system
content might cause other confined services on the machine to fail. For
these types of containers we recommend disabling SELinux separation. The
option **\--security-opt label=disable** disables SELinux separation for
the pod. For example if a user wanted to volume mount their entire home
directory into a pod, they need to disable SELinux separation.

    $ podman pod create --security-opt label=disable -v $HOME:/home/user fedora touch /home/user/file

`Overlay Volume Mounts`

The `:O` flag tells Podman to mount the directory from the host as a
temporary storage using the `overlay file system`. The pod processes can
modify content within the mountpoint which is stored in the container
storage in a separate directory. In overlay terms, the source directory
is the lower, and the container storage directory is the upper.
Modifications to the mount point are destroyed when the pod finishes
executing, similar to a tmpfs mount point being unmounted.

For advanced users, the **overlay** option also supports custom
non-volatile **upperdir** and **workdir** for the overlay mount. Custom
**upperdir** and **workdir** can be fully managed by the users
themselves, and Podman does not remove it on lifecycle completion.
Example **:O,upperdir=/some/upper,workdir=/some/work**

Subsequent executions of the container sees the original source
directory content, any changes from previous pod executions no longer
exist.

One use case of the overlay mount is sharing the package cache from the
host into the container to allow speeding up builds.

Note: The `O` flag conflicts with other options listed above.

Content mounted into the container is labeled with the private label. On
SELinux systems, labels in the source directory must be readable by the
pod infra container label. Usually containers can read/execute
`container_share_t` and can read/write `container_file_t`. If unable to
change the labels on a source volume, SELinux container separation must
be disabled for the pod or infra container to work.

Do not modify the source directory mounted into the pod with an overlay
mount, it can cause unexpected failures. Only modify the directory after
the container finishes running.

`Mounts propagation`

By default, bind-mounted volumes are `private`. That means any mounts
done inside the pod are not visible on the host and vice versa. One can
change this behavior by specifying a volume mount propagation property.
When a volume is `shared`, mounts done under that volume inside the pod
are visible on host and vice versa. Making a volume
**slave**^[\[1\]](#Footnote1)^ enables only one-way mount propagation:
mounts done on the host under that volume are visible inside the
container but not the other way around.

To control mount propagation property of a volume one can use the
\[**r**\]**shared**, \[**r**\]**slave**, \[**r**\]**private** or the
\[**r**\]**unbindable** propagation flag. Propagation property can be
specified only for bind mounted volumes and not for internal volumes or
named volumes. For mount propagation to work the source mount point (the
mount point where source dir is mounted on) has to have the right
propagation properties. For shared volumes, the source mount point has
to be shared. And for slave volumes, the source mount point has to be
either shared or slave. ^[\[1\]](#Footnote1)^

To recursively mount a volume and all of its submounts into a pod, use
the **rbind** option. By default the bind option is used, and submounts
of the source directory is not mounted into the pod.

Mounting the volume with a **copy** option tells podman to copy content
from the underlying destination directory onto newly created internal
volumes. The **copy** only happens on the initial creation of the
volume. Content is not copied up when the volume is subsequently used on
different containers. The **copy** option is ignored on bind mounts and
has no effect.

Mounting volumes with the **nosuid** options means that SUID executables
on the volume can not be used by applications to change their privilege.
By default volumes are mounted with **nosuid**.

Mounting the volume with the **noexec** option means that no executables
on the volume can be executed within the pod.

Mounting the volume with the **nodev** option means that no devices on
the volume can be used by processes within the pod. By default volumes
are mounted with **nodev**.

If the *HOST-DIR* is a mount point, then **dev**, **suid**, and **exec**
options are ignored by the kernel.

Use **df HOST-DIR** to figure out the source mount, then use **findmnt
-o TARGET,PROPAGATION *source-mount-dir*** to figure out propagation
properties of source mount. If **findmnt**(1) utility is not available,
then one can look at the mount entry for the source mount point in
*/proc/self/mountinfo*. Look at the \"optional fields\" and see if any
propagation properties are specified. In there, **shared:N** means the
mount is shared, **master:N** means mount is slave, and if nothing is
there, the mount is private. ^[\[1\]](#Footnote1)^

To change propagation properties of a mount point, use **mount**(8)
command. For example, if one wants to bind mount source directory
*/foo*, one can do **mount \--bind /foo /foo** and **mount
\--make-private \--make-shared /foo**. This converts /foo into a shared
mount point. Alternatively, one can directly change propagation
properties of source mount. Say */* is source mount for */foo*, then use
**mount \--make-shared /** to convert */* into a shared mount.

Note: if the user only has access rights via a group, accessing the
volume from inside a rootless pod fails.

`Idmapped mount`

If `idmap` is specified, create an idmapped mount to the target user
namespace in the container. The idmap option supports a custom mapping
that can be different than the user namespace used by the container. The
mapping can be specified after the idmap option like:
`idmap=uids=0-1-10#10-11-10;gids=0-100-10`. For each triplet, the first
value is the start of the backing file system IDs that are mapped to the
second value on the host. The length of this mapping is given in the
third value. Multiple ranges are separated with #.

#### **\--volumes-from**=*CONTAINER\[:OPTIONS\]*

Mount volumes from the specified container(s). Used to share volumes
between containers and pods. The *options* is a comma-separated list
with the following available elements:

-   **rw**\|**ro**
-   **z**

Mounts already mounted volumes from a source container onto another pod.
*CONTAINER* may be a name or ID. To share a volume, use the
\--volumes-from option when running the target container. Volumes can be
shared even if the source container is not running.

By default, Podman mounts the volumes in the same mode (read-write or
read-only) as it is mounted in the source container. This can be changed
by adding a `ro` or `rw` *option*.

Labeling systems like SELinux require that proper labels are placed on
volume content mounted into a pod. Without a label, the security system
might prevent the processes running inside the container from using the
content. By default, Podman does not change the labels set by the OS.

To change a label in the pod context, add `z` to the volume mount. This
suffix tells Podman to relabel file objects on the shared volumes. The
`z` option tells Podman that two entities share the volume content. As a
result, Podman labels the content with a shared content label. Shared
volume labels allow all containers to read/write content.

If the location of the volume from the source container overlaps with
data residing on a target pod, then the volume hides that data on the
target.

##  EXAMPLES

Create a named pod.

    $ podman pod create --name test

Create a named pod.

    $ podman pod create mypod

Create a pod without an infra container.

    $ podman pod create --infra=false

Create a named pod with infra container command to run.

    $ podman pod create --infra-command /top toppod

Create a pod with published ports on the host.

    $ podman pod create --publish 8443:443

Create a pod with the specified network configuration.

    $ podman pod create --network slirp4netns:outbound_addr=127.0.0.1,allow_host_loopback=true

Create a pod with the specified network.

    $ podman pod create --network pasta

Create a pod on two networks.

    $ podman pod create --network net1:ip=10.89.1.5 --network net2:ip=10.89.10.10

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-pod(1)](podman-pod.html)**,
**[podman-kube-play(1)](podman-kube-play.html)**,
**containers.conf(1)**,
**[cgroups(7)](https://man7.org/linux/man-pages/man7/cgroups.7.html)**

### Troubleshooting

See
[podman-troubleshooting(7)](https://github.com/containers/podman/blob/main/troubleshooting.md)
for solutions to common issues.

##  HISTORY

July 2018, Originally compiled by Peter Hunt <pehunt@redhat.com>

##  FOOTNOTES

[1]{#Footnote1}: The Podman project is committed to inclusivity, a core
value of open source. The `master` and `slave` mount propagation
terminology used here is problematic and divisive, and needs to be
changed. However, these terms are currently used within the Linux kernel
and must be used as-is at this time. When the kernel maintainers rectify
this usage, Podman will follow suit immediately.


---

<a id='podman-pod-exists'></a>

## podman-pod-exists - Check if a pod exists in local storage

##  NAME

podman-pod-exists - Check if a pod exists in local storage

##  SYNOPSIS

**podman pod exists** *pod*

##  DESCRIPTION

**podman pod exists** checks if a pod exists in local storage. The
**ID** or **Name** of the pod may be used as input. Podman returns an
exit code of `0` when the pod is found. A `1` is returned otherwise. An
exit code of `125` indicates there was an issue accessing the local
storage.

##  EXAMPLES

Check if specified pod exists in local storage (the pod does actually
exist):

    $ sudo podman pod exists web; echo $?
    0

Check if specified pod exists in local storage (the pod does not
actually exist):

    $ sudo podman pod exists backend; echo $?
    1

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-pod(1)](podman-pod.html)**

##  HISTORY

December 2018, Originally compiled by Brent Baude (bbaude at redhat dot
com)


---

<a id='podman-pod-inspect'></a>

## podman-pod-inspect - Display information describing a pod

##  NAME

podman-pod-inspect - Display information describing a pod

##  SYNOPSIS

**podman pod inspect** \[*options*\] *pod* \...

##  DESCRIPTION

Displays configuration and state information about a given pod. It also
displays information about containers that belong to the pod.

##  OPTIONS

#### **\--format**, **-f**=*format*

Change the default output format. This can be of a supported type like
\'json\' or a Go template. Valid placeholders for the Go template are
listed below:

  **Placeholder**        **Description**
  ---------------------- ----------------------------------------
  .BlkioDeviceReadBps    Block I/O Device Read, in bytes/sec
  .BlkioDeviceWriteBps   Block I/O Device Read, in bytes/sec
  .BlkioWeight           Block I/O Weight
  .BlkioWeightDevice     Block I/O Device Weight
  .CgroupParent          Pod cgroup parent
  .CgroupPath            Pod cgroup path
  .Containers            Pod containers
  .CPUPeriod             CPU period
  .CPUQuota              CPU quota
  .CPUSetCPUs            CPU Set CPUs
  .CPUSetMems            CPU Set Mems
  .CPUShares             CPU Shares
  .CreateCgroup          Whether cgroup was created
  .CreateCommand         Create command
  .Created \...          Time when the pod was created
  .CreateInfra           Whether infrastructure created
  .Devices               Devices
  .ExitPolicy            Exit policy
  .Hostname              Pod hostname
  .ID                    Pod ID
  .InfraConfig \...      Infra config (contains further fields)
  .InfraContainerID      Pod infrastructure ID
  .InspectPodData \...   Nested structure, for experts only
  .Labels \...           Pod labels
  .LockNumber            Number of the pod\'s Libpod lock
  .MemoryLimit           Memory limit, bytes
  .MemorySwap            Memory swap limit, in bytes
  .Mounts                Mounts
  .Name                  Pod name
  .Namespace             Namespace
  .NumContainers         Number of containers in the pod
  .RestartPolicy         Restart policy of the pod
  .SecurityOpts          Security options
  .SharedNamespaces      Pod shared namespaces
  .State                 Pod state
  .VolumesFrom           Volumes from

#### **\--latest**, **-l**

Instead of providing the pod name or ID, use the last created pod. Note:
the last started pod can be from other users of Podman on the host
machine. (This option is not available with the remote Podman client,
including Mac and Windows (excluding WSL2) machines)

##  EXAMPLE

Inspect specified pod:

    # podman pod inspect foobar
    [
         {
             "Id": "3513ca70583dd7ef2bac83331350f6b6c47d7b4e526c908e49d89ebf720e4693",
             "Name": "foobar",
             "Labels": {},
             "CgroupParent": "/libpod_parent",
             "CreateCgroup": true,
             "Created": "2018-08-08T11:15:18.823115347-05:00"
             "State": "created",
             "Hostname": "",
             "SharedNamespaces": [
                  "uts",
                  "ipc",
                  "net"
             ]
             "CreateInfra": false,
             "InfraContainerID": "1020dd70583dd7ff2bac83331350f6b6e007de0d026c908e49d89ebf891d4699"
             "CgroupPath": ""
             "Containers": [
                  {
                       "id": "d53f8bf1e9730281264aac6e6586e327429f62c704abea4b6afb5d8a2b2c9f2c",
                       "state": "configured"
                  }
             ]
         }
    ]

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-pod(1)](podman-pod.html)**,
**[podman-inspect(1)](podman-inspect.html)**

##  HISTORY

August 2018, Originally compiled by Brent Baude <bbaude@redhat.com>


---

<a id='podman-pod-kill'></a>

## podman-pod-kill - Kill the main process of each container in one or
more pods

##  NAME

podman-pod-kill - Kill the main process of each container in one or more
pods

##  SYNOPSIS

**podman pod kill** \[*options*\] *pod* \...

##  DESCRIPTION

The main process of each container inside the pods specified is sent
SIGKILL, or any signal specified with option \--signal.

##  OPTIONS

#### **\--all**, **-a**

Sends signal to all containers associated with a pod.

#### **\--latest**, **-l**

Instead of providing the pod name or ID, use the last created pod. Note:
the last started pod can be from other users of Podman on the host
machine. (This option is not available with the remote Podman client,
including Mac and Windows (excluding WSL2) machines)

#### **\--signal**, **-s**=**signal**

Signal to send to the containers in the pod. For more information on
Linux signals, refer to *signal(7)*. The default is **SIGKILL**.

##  EXAMPLE

Kill pod with a given name:

    podman pod kill mywebserver

Kill pod with a given ID:

    podman pod kill 860a4b23

Terminate pod by sending `TERM` signal:

    podman pod kill --signal TERM 860a4b23

Kill the latest pod. (This option is not available with the remote
Podman client, including Mac and Windows (excluding WSL2) machines):

    podman pod kill --latest

Terminate all pods by sending `KILL` signal:

    podman pod kill --all

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-pod(1)](podman-pod.html)**,
**[podman-pod-stop(1)](podman-pod-stop.html)**

##  HISTORY

July 2018, Originally compiled by Peter Hunt <pehunt@redhat.com>


---

<a id='podman-pod-logs'></a>

## podman-pod-logs - Display logs for pod with one or more
containers

##  NAME

podman-pod-logs - Display logs for pod with one or more containers

##  SYNOPSIS

**podman pod logs** \[*options*\] *pod*

##  DESCRIPTION

The podman pod logs command batch-retrieves whatever logs are present
with all the containers of a pod. Pod logs can be filtered by container
name or ID using flag **-c** or **\--container** if needed.

Note: A long-running `podman pod log` command with a `-f` or `--follow`
option needs to be reinvoked if a new container is added to the pod
dynamically; otherwise, logs of newly added containers are not visible
in the log stream.

##  OPTIONS

#### **\--color**

Output the containers with different colors in the log.

#### **\--container**, **-c**

By default, `podman pod logs` retrieves logs for all the containers
available within the pod, differentiated by the field `container`.
However, there are use cases where the user wants to limit the log
stream only to a particular container of a pod. For such cases, `-c` can
be used like `podman pod logs -c ctrNameorID podname`.

#### **\--follow**, **-f**

Follow log output. Default is false.

Note: When following a pod which is removed by `podman pod rm` or
removed on exit (`podman run --rm ...`), there is a chance that the log
file is removed before `podman pod logs` reads the final content.

#### **\--latest**, **-l**

Instead of providing the pod name or ID, use the last created pod. Note:
the last started pod can be from other users of Podman on the host
machine. (This option is not available with the remote Podman client,
including Mac and Windows (excluding WSL2) machines)

#### **\--names**, **-n**

Output the container names instead of the container IDs in the log.

#### **\--since**=*TIMESTAMP*

Show logs since TIMESTAMP. The \--since option can be Unix timestamps,
date formatted timestamps, or Go duration strings (e.g. 10m, 1h30m)
computed relative to the client machine\'s time. Supported formats for
date formatted time stamps include RFC3339Nano, RFC3339,
2006-01-02T15:04:05, 2006-01-02T15:04:05.999999999, 2006-01-02Z07:00,
and 2006-01-02.

#### **\--tail**=*LINES*

Output the specified number of LINES at the end of the logs. LINES must
be an integer. Defaults to -1, which prints all lines

#### **\--timestamps**, **-t**

Show timestamps in the log outputs. The default is false

#### **\--until**=*TIMESTAMP*

Show logs until TIMESTAMP. The \--until option can be Unix timestamps,
date formatted timestamps, or Go duration strings (e.g. 10m, 1h30m)
computed relative to the client machine\'s time. Supported formats for
date formatted time stamps include RFC3339Nano, RFC3339,
2006-01-02T15:04:05, 2006-01-02T15:04:05.999999999, 2006-01-02Z07:00,
and 2006-01-02.

##  EXAMPLE

To view a pod\'s logs:

    podman pod logs -t podIdorName

To view logs of a specific container on the pod:

    podman pod logs -c ctrIdOrName podIdOrName

To view all pod logs:

    podman pod logs -t --since 0 myserver-pod-1

To view a pod\'s logs since a certain time:

    podman pod logs -t --since 2017-08-07T10:10:09.055837383-04:00 myserver-pod-1

To view a pod\'s logs generated in the last 10 minutes:

    podman pod logs --since 10m myserver-pod-1

To view a pod\'s logs until 30 minutes ago:

    podman pod logs --until 30m myserver-pod-1

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-pod(1)](podman-pod.html)**,
**[podman-pod-rm(1)](podman-pod-rm.html)**,
**[podman-logs(1)](podman-logs.html)**


---

<a id='podman-pod-pause'></a>

## podman-pod-pause - Pause one or more pods

##  NAME

podman-pod-pause - Pause one or more pods

##  SYNOPSIS

**podman pod pause** \[*options*\] *pod* \...

##  DESCRIPTION

Pauses all the running processes in the containers of one or more pods.
You may use pod IDs or names as input.

##  OPTIONS

#### **\--all**, **-a**

Pause all pods.

#### **\--latest**, **-l**

Instead of providing the pod name or ID, pause the last created pod.
(This option is not available with the remote Podman client, including
Mac and Windows (excluding WSL2) machines)

##  EXAMPLE

Pause a pod with a given name:

    podman pod pause mywebserverpod

Pause a pod with a given ID:

    podman pod pause 860a4b23

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-pod(1)](podman-pod.html)**,
**[podman-pod-unpause(1)](podman-pod-unpause.html)**,
**[podman-pause(1)](podman-pause.html)**

##  HISTORY

July 2018, Originally compiled by Peter Hunt <pehunt@redhat.com>


---

<a id='podman-pod-prune'></a>

## podman-pod-prune - Remove all stopped pods and their containers

##  NAME

podman-pod-prune - Remove all stopped pods and their containers

##  SYNOPSIS

**podman pod prune** \[*options*\]

##  DESCRIPTION

**podman pod prune** removes all stopped pods and their containers from
local storage.

##  OPTIONS

#### **\--force**, **-f**

Force removal of all running pods and their containers. The default is
false.

##  EXAMPLES

Remove all stopped pods and their containers from local storage.

    $ sudo podman pod prune
    22b8813332948064b6566370088c5e0230eeaf15a58b1c5646859fd9fc364fe7
    2afb26869fe5beab979c234afb75c7506063cd4655b1a73557c9d583ff1aebe9
    49161ad2a722cf18722f0e17199a9e840703a17d1158cdeda502b6d54080f674
    5ca429f37fb83a9f54eea89e3a9102b7780a6e6ae5f132db0672da551d862c4a
    6bb06573787efb8b0675bc88ebf8361f1a56d3ac7922d1a6436d8f59ffd955f1

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-pod(1)](podman-pod.html)**

##  HISTORY

April 2019, Originally compiled by Peter Hunt (pehunt at redhat dot com)


---

<a id='podman-pod-ps'></a>

## podman-pod-ps - Print out information about pods

##  NAME

podman-pod-ps - Print out information about pods

##  SYNOPSIS

**podman pod ps** \[*options*\]

##  DESCRIPTION

**podman pod ps** lists the pods on the system. By default it lists:

-   pod ID
-   pod name
-   the time the pod was created
-   number of containers attached to pod
-   container ID of the pod infra container
-   status of pod as defined by the following table

  **Status**   **Description**
  ------------ -------------------------------------------------
  Created      No containers running nor stopped
  Running      at least one container is running
  Stopped      At least one container stopped and none running
  Exited       All containers stopped in pod
  Dead         Error retrieving state

##  OPTIONS

#### **\--ctr-ids**

Display the container IDs

#### **\--ctr-names**

Display the container names

#### **\--ctr-status**

Display the container statuses

#### **\--filter**, **-f**=*filter*

Provide filter values.

The *filters* argument format is of `key=value`. If there is more than
one *filter*, then pass multiple OPTIONS: **\--filter** *foo=bar*
**\--filter** *bif=baz*.

Supported filters:

  ----------------------------------------------------------------------------
  Filter       Description
  ------------ ---------------------------------------------------------------
  ctr-ids      Filter by container ID within the pod. (CID prefix match by
               default; accepts regex)

  ctr-names    Filter by container name within the pod.

  ctr-number   Filter by number of containers in the pod.

  ctr-status   Filter by container status within the pod.

  id           Filter by pod ID. (Prefix match by default; accepts regex)

  label        Filter by container with (or without, in the case of
               label!=\[\...\] is used) the specified labels.

  name         Filter by pod name.

  network      Filter by network name or full ID of network.

  status       Filter by pod status.

  until        Filter by pods created before given timestamp.
  ----------------------------------------------------------------------------

The `ctr-ids`, `ctr-names`, `id`, `name` filters accept `regex` format.

The `ctr-status` filter accepts values: `created`, `running`, `paused`,
`stopped`, `exited`, `unknown`.

The `label` *filter* accepts two formats. One is the `label`=*key* or
`label`=*key*=*value*, which removes containers with the specified
labels. The other format is the `label!`=*key* or
`label!`=*key*=*value*, which removes containers without the specified
labels.

The `until` *filter* can be Unix timestamps, date formatted timestamps,
or Go duration strings (e.g. 10m, 1h30m) computed relative to the
machine's time.

The `status` filter accepts values: `stopped`, `running`, `paused`,
`exited`, `dead`, `created`, `degraded`.

#### **\--format**=*format*

Pretty-print containers to JSON or using a Go template

Valid placeholders for the Go template are listed below:

  -------------------------------------------------------------------------
  **Placeholder**       **Description**
  --------------------- ---------------------------------------------------
  .Cgroup               Cgroup path of pod

  .ContainerIds         Comma-separated list of container IDs in the pod

  .ContainerNames       Comma-separated list of container names in the pod

  .ContainerStatuses    Comma-separated list of container statuses

  .Created              Creation time of pod

  .ID                   Container ID

  .InfraID              Pod infra container ID

  .Label *string*       Specified label of the pod

  .Labels \...          All the labels assigned to the pod

  .Name                 Name of pod

  .Networks             Show all networks connected to the infra container

  .NumberOfContainers   Show the number of containers attached to pod

  .Restarts             Show the total number of container restarts in a
                        pod

  .Status               Status of pod
  -------------------------------------------------------------------------

#### **\--help**, **-h**

Print usage statement

#### **\--latest**, **-l**

Show the latest pod created (all states) (This option is not available
with the remote Podman client, including Mac and Windows (excluding
WSL2) machines)

#### **\--namespace**, **\--ns**

Display namespace information of the pod

#### **\--no-trunc**

Do not truncate the output (default *false*).

#### **\--noheading**, **-n**

Omit the table headings from the listing.

#### **\--quiet**, **-q**

Print the numeric IDs of the pods only

#### **\--sort**

Sort by created, ID, name, status, or number of containers

Default: created

##  EXAMPLES

List all running pods.

    $ podman pod ps
    POD ID         NAME              STATUS    CREATED          INFRA ID       # OF CONTAINERS
    00dfd6fa02c0   jolly_goldstine   Running   31 hours ago     ba465ab0a3a4   1
    f4df8692e116   nifty_torvalds    Created   10 minutes ago   331693bff40a   2

List all running pods along with container names within the pods.

    $ podman pod ps --ctr-names
    POD ID         NAME              STATUS    CREATED          INFRA ID       NAMES
    00dfd6fa02c0   jolly_goldstine   Running   31 hours ago     ba465ab0a3a4   loving_archimedes
    f4df8692e116   nifty_torvalds    Created   10 minutes ago   331693bff40a   thirsty_hawking,wizardly_golick

List all running pods along with status, names and ids.

    $ podman pod ps --ctr-status --ctr-names --ctr-ids
    POD ID         NAME              STATUS    CREATED          INFRA ID       IDS                         NAMES                             STATUS
    00dfd6fa02c0   jolly_goldstine   Running   31 hours ago     ba465ab0a3a4   ba465ab0a3a4                loving_archimedes                 running
    f4df8692e116   nifty_torvalds    Created   10 minutes ago   331693bff40a   331693bff40a,8e428daeb89e   thirsty_hawking,wizardly_golick   configured,configured

List all running pods and print ID, Container Names, and cgroups.

    $ podman pod ps --format "{{.ID}}  {{.ContainerNames}}  {{.Cgroup}}"
    00dfd6fa02c0   loving_archimedes   /libpod_parent
    f4df8692e116   thirsty_hawking,wizardly_golick   /libpod_parent

List all running pods with two containers sorted by pod ID.

    $ podman pod ps --sort id --filter ctr-number=2
    POD ID         NAME             STATUS    CREATED          INFRA ID       # OF CONTAINERS
    f4df8692e116   nifty_torvalds   Created   10 minutes ago   331693bff40a   2

List all running pods with their container ids.

    $ podman pod ps  --ctr-ids
    POD ID         NAME              STATUS    CREATED          INFRA ID       IDS
    00dfd6fa02c0   jolly_goldstine   Running   31 hours ago     ba465ab0a3a4   ba465ab0a3a4
    f4df8692e116   nifty_torvalds    Created   10 minutes ago   331693bff40a   331693bff40a,8e428daeb89e

List all running pods with container ids without truncating IDs.

    $ podman pod ps --no-trunc --ctr-ids
    POD ID                                                             NAME              STATUS    CREATED          INFRA ID                                                           IDS
    00dfd6fa02c0a2daaedfdf8fcecd06f22ad114d46d167d71777224735f701866   jolly_goldstine   Running   31 hours ago     ba465ab0a3a4e15e3539a1e79c32d1213a02b0989371e274f98e0f1ae9de7050   ba465ab0a3a4e15e3539a1e79c32d1213a02b0989371e274f98e0f1ae9de7050
    f4df8692e116a3e6d1d62572644ed36ca475d933808cc3c93435c45aa139314b   nifty_torvalds    Created   10 minutes ago   331693bff40a926b6d52b184e116afd15497610c378d5d4c42945dd6e33b75b0   331693bff40a926b6d52b184e116afd15497610c378d5d4c42945dd6e33b75b0,8e428daeb89e69b71e7916a13accfb87d122889442b5c05c2d99cf94a3230e9d

List all running pods with container names.

    $ podman pod ps --ctr-names
    POD ID         NAME   STATUS    CREATED        INFRA ID       NAMES
    314f4da82d74   hi     Created   17 hours ago   a9f2d2165675   jovial_jackson,hopeful_archimedes,vibrant_ptolemy,heuristic_jennings,keen_raman,hopeful_newton,mystifying_bose,silly_lalande,serene_lichterman ...

##  pod ps

Print a list of pods

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-pod(1)](podman-pod.html)**

##  HISTORY

July 2018, Originally compiled by Peter Hunt <pehunt@redhat.com>


---

<a id='podman-pod-restart'></a>

## podman-pod-restart - Restart one or more pods

##  NAME

podman-pod-restart - Restart one or more pods

##  SYNOPSIS

**podman pod restart** \[*options*\] *pod* \...

##  DESCRIPTION

Restart containers in one or more pods. Running containers are stopped
and restarted. Stopped containers are started. You may use pod IDs or
names as input. The pod ID is printed upon successful restart. When
restarting multiple pods, an error from restarting one pod does not
effect restarting other pods.

##  OPTIONS

#### **\--all**, **-a**

Restarts all pods

#### **\--latest**, **-l**

Instead of providing the pod name or ID, restart the last created pod.
(This option is not available with the remote Podman client, including
Mac and Windows (excluding WSL2) machines)

##  EXAMPLE

Restart pod with a given name:

    podman pod restart mywebserverpod
    cc8f0bea67b1a1a11aec1ecd38102a1be4b145577f21fc843c7c83b77fc28907

Restart multiple pods with given IDs:

    podman pod restart 490eb 3557fb
    490eb241aaf704d4dd2629904410fe4aa31965d9310a735f8755267f4ded1de5
    3557fbea6ad61569de0506fe037479bd9896603c31d3069a6677f23833916fab

Restart the last created pod:

    podman pod restart --latest
    3557fbea6ad61569de0506fe037479bd9896603c31d3069a6677f23833916fab

Restart all pods:

    podman pod restart --all
    19456b4cd557eaf9629825113a552681a6013f8c8cad258e36ab825ef536e818
    3557fbea6ad61569de0506fe037479bd9896603c31d3069a6677f23833916fab
    490eb241aaf704d4dd2629904410fe4aa31965d9310a735f8755267f4ded1de5
    70c358daecf71ef9be8f62404f926080ca0133277ef7ce4f6aa2d5af6bb2d3e9
    cc8f0bea67b1a1a11aec1ecd38102a1be4b145577f21fc843c7c83b77fc28907

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-pod(1)](podman-pod.html)**,
**[podman-restart(1)](podman-restart.html)**

##  HISTORY

July 2018, Originally compiled by Peter Hunt <pehunt@redhat.com>


---

<a id='podman-pod-rm'></a>

## podman-pod-rm - Remove one or more stopped pods and containers

##  NAME

podman-pod-rm - Remove one or more stopped pods and containers

##  SYNOPSIS

**podman pod rm** \[*options*\] *pod*

##  DESCRIPTION

**podman pod rm** removes one or more stopped pods and their containers
from the host. The pod name or ID can be used. The -f option stops all
containers and then removes them before removing the pod. If all
containers added by the user are in an exited state, the pod is removed.

##  OPTIONS

#### **\--all**, **-a**

Remove all pods. Can be used in conjunction with -f as well.

#### **\--force**, **-f**

Stop running containers and delete all stopped containers before removal
of pod.

#### **\--ignore**, **-i**

Ignore errors when specified pods are not in the container store. A user
might have decided to manually remove a pod which leads to a failure
during the ExecStop directive of a systemd service referencing that pod.

#### **\--latest**, **-l**

Instead of providing the pod name or ID, use the last created pod. Note:
the last started pod can be from other users of Podman on the host
machine. (This option is not available with the remote Podman client,
including Mac and Windows (excluding WSL2) machines)

#### **\--pod-id-file**=*file*

Read pod ID from the specified *file* and rm the pod. Can be specified
multiple times.

If specified, the pod-id-file is removed along with the pod.

#### **\--time**, **-t**=*seconds*

Seconds to wait before forcibly stopping running containers within the
pod. Use -1 for infinite wait.

The \--force option must be specified to use the \--time option.

##  EXAMPLE

Remove pod with a given name:

    podman pod rm mywebserverpod

Remove multiple pods with given names and/or IDs:

    podman pod rm mywebserverpod myflaskserverpod 860a4b23

Forcefully remove pod with a given ID:

    podman pod rm -f 860a4b23

Forcefully remove all pods:

    podman pod rm -f -a
    podman pod rm -fa

Remove pod using ID specified in a given file:

    podman pod rm --pod-id-file /path/to/id/file

##  Exit Status

**0** All specified pods removed

**1** One of the specified pods did not exist, and no other failures

**2** One of the specified pods is attached to a container

**125** The command fails for any other reason

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-pod(1)](podman-pod.html)**

##  HISTORY

July 2018, Originally compiled by Peter Hunt <pehunt@redhat.com>


---

<a id='podman-pod-start'></a>

## podman-pod-start - Start one or more pods

##  NAME

podman-pod-start - Start one or more pods

##  SYNOPSIS

**podman pod start** \[*options*\] *pod* \...

##  DESCRIPTION

Start containers in one or more pods. You may use pod IDs or names as
input. The pod must have a container attached to be started.

##  OPTIONS

#### **\--all**, **-a**

Starts all pods

#### **\--latest**, **-l**

Instead of providing the pod name or ID, use the last created pod. Note:
the last started pod can be from other users of Podman on the host
machine. (This option is not available with the remote Podman client,
including Mac and Windows (excluding WSL2) machines)

#### **\--pod-id-file**=*file*

Read pod ID from the specified *file* and start the pod. Can be
specified multiple times.

##  EXAMPLE

Start pod with a given name:

    podman pod start mywebserverpod

Start pods with given IDs:

    podman pod start 860a4b23 5421ab4

Start the latest pod. (This option is not available with the remote
Podman client, including Mac and Windows (excluding WSL2) machines):

    podman pod start --latest

Start all pods:

    podman pod start --all

Start pod using ID specified in a given file:

    podman pod start --pod-id-file /path/to/id/file

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-pod(1)](podman-pod.html)**,
**[podman-pod-stop(1)](podman-pod-stop.html)**

##  HISTORY

July 2018, Adapted from podman start man page by Peter Hunt
<pehunt@redhat.com>


---

<a id='podman-pod-stats'></a>

## podman-pod-stats - Display a live stream of resource usage stats for
containers in one or more pods

##  NAME

podman-pod-stats - Display a live stream of resource usage stats for
containers in one or more pods

##  SYNOPSIS

**podman pod stats** \[*options*\] \[*pod*\]

##  DESCRIPTION

Display a live stream of containers in one or more pods resource usage
statistics. Running rootless is only supported on cgroups v2.

##  OPTIONS

#### **\--all**, **-a**

Show all containers. Only running containers are shown by default

#### **\--format**=*template*

Pretty-print container statistics to JSON or using a Go template

Valid placeholders for the Go template are listed below:

  **Placeholder**   **Description**
  ----------------- --------------------
  .BlockIO          Block IO
  .CID              Container ID
  .CPU              CPU percentage
  .Mem              Memory percentage
  .MemUsage         Memory usage
  .MemUsageBytes    Memory usage (IEC)
  .Name             Container Name
  .NetIO            Network IO
  .PIDS             Number of PIDs
  .Pod              Pod ID

When using a Go template, precede the format with `table` to print
headers.

#### **\--latest**, **-l**

Instead of providing the pod name or ID, use the last created pod. Note:
the last started pod can be from other users of Podman on the host
machine. (This option is not available with the remote Podman client,
including Mac and Windows (excluding WSL2) machines)

#### **\--no-reset**

Do not clear the terminal/screen in between reporting intervals

#### **\--no-stream**

Disable streaming pod stats and only pull the first result, default
setting is false

##  EXAMPLE

List statistics about all pods without streaming:

    # podman pod stats -a --no-stream
    ID             NAME              CPU %   MEM USAGE / LIMIT   MEM %   NET IO    BLOCK IO   PIDS
    a9f807ffaacd   frosty_hodgkin    --      3.092MB / 16.7GB    0.02%   -- / --   -- / --    2
    3b33001239ee   sleepy_stallman   --      -- / --             --      -- / --   -- / --    --

List statistics about specified pod without streaming:

    # podman pod stats --no-stream a9f80
    ID             NAME             CPU %   MEM USAGE / LIMIT   MEM %   NET IO    BLOCK IO   PIDS
    a9f807ffaacd   frosty_hodgkin   --      3.092MB / 16.7GB    0.02%   -- / --   -- / --    2

List statistics about specified pod in JSON format without streaming:

    # podman pod stats --no-stream --format=json a9f80
    [
        {
            "id": "a9f807ffaacd",
            "name": "frosty_hodgkin",
            "cpu_percent": "--",
            "mem_usage": "3.092MB / 16.7GB",
            "mem_percent": "0.02%",
            "netio": "-- / --",
            "blocki": "-- / --",
            "pids": "2"
        }
    ]

List selected statistics formatted in a table about specified pod:

    # podman pod stats --no-stream --format "table {{.ID}} {{.Name}} {{.MemUsage}}" 6eae
    ID             NAME           MEM USAGE / LIMIT
    6eae9e25a564   clever_bassi   3.031MB / 16.7GB

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-pod(1)](podman-pod.html)**

##  HISTORY

February 2019, Originally compiled by Dan Walsh <dwalsh@redhat.com>


---

<a id='podman-pod-stop'></a>

## podman-pod-stop - Stop one or more pods

##  NAME

podman-pod-stop - Stop one or more pods

##  SYNOPSIS

**podman pod stop** \[*options*\] *pod* \...

##  DESCRIPTION

Stop containers in one or more pods. You may use pod IDs or names as
input.

##  OPTIONS

#### **\--all**, **-a**

Stops all pods

#### **\--ignore**, **-i**

Ignore errors when specified pods are not in the container store. A user
might have decided to manually remove a pod which leads to a failure
during the ExecStop directive of a systemd service referencing that pod.

#### **\--latest**, **-l**

Instead of providing the pod name or ID, use the last created pod. Note:
the last started pod can be from other users of Podman on the host
machine. (This option is not available with the remote Podman client,
including Mac and Windows (excluding WSL2) machines)

#### **\--pod-id-file**=*file*

Read pod ID from the specified *file* and stop the pod. Can be specified
multiple times.

#### **\--time**, **-t**=*seconds*

Seconds to wait before forcibly stopping running containers within the
pod. Use -1 for infinite wait.

##  EXAMPLE

Stop pod with a given name.

    $ podman pod stop mywebserverpod
    cc8f0bea67b1a1a11aec1ecd38102a1be4b145577f21fc843c7c83b77fc28907

Stop multiple pods with given IDs.

    $ podman pod stop 490eb 3557fb
    490eb241aaf704d4dd2629904410fe4aa31965d9310a735f8755267f4ded1de5
    3557fbea6ad61569de0506fe037479bd9896603c31d3069a6677f23833916fab

Stop the last created pod. (This option is not available with the remote
Podman client, including Mac and Windows (excluding WSL2) machines)

    $ podman pod stop --latest
    3557fbea6ad61569de0506fe037479bd9896603c31d3069a6677f23833916fab

Stop all pods.

    $ podman pod stop --all
    19456b4cd557eaf9629825113a552681a6013f8c8cad258e36ab825ef536e818
    3557fbea6ad61569de0506fe037479bd9896603c31d3069a6677f23833916fab
    490eb241aaf704d4dd2629904410fe4aa31965d9310a735f8755267f4ded1de5
    70c358daecf71ef9be8f62404f926080ca0133277ef7ce4f6aa2d5af6bb2d3e9
    cc8f0bea67b1a1a11aec1ecd38102a1be4b145577f21fc843c7c83b77fc28907

Stop two pods via \--pod-id-file.

    $ podman pod stop --pod-id-file file1 --pod-id-file file2
    19456b4cd557eaf9629825113a552681a6013f8c8cad258e36ab825ef536e818
    cc8f0bea67b1a1a11aec1ecd38102a1be4b145577f21fc843c7c83b77fc28907

Stop all pods with a timeout of 1 second.

    $ podman pod stop -a -t 1
    3557fbea6ad61569de0506fe037479bd9896603c31d3069a6677f23833916fab
    490eb241aaf704d4dd2629904410fe4aa31965d9310a735f8755267f4ded1de5
    70c358daecf71ef9be8f62404f926080ca0133277ef7ce4f6aa2d5af6bb2d3e9

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-pod(1)](podman-pod.html)**,
**[podman-pod-start(1)](podman-pod-start.html)**

##  HISTORY

July 2018, Originally compiled by Peter Hunt <pehunt@redhat.com>


---

<a id='podman-pod-top'></a>

## podman-pod-top - Display the running processes of containers in a
pod

##  NAME

podman-pod-top - Display the running processes of containers in a pod

##  SYNOPSIS

**podman pod top** \[*options*\] *pod* \[*format-descriptors*\]

##  DESCRIPTION

Display the running processes of containers in a pod. The
*format-descriptors* are ps (1) compatible AIX format descriptors but
extended to print additional information, such as the seccomp mode or
the effective capabilities of a given process. The descriptors can
either be passed as separate arguments or as a single comma-separated
argument.

##  OPTIONS

#### **\--help**, **-h**

Print usage statement

#### **\--latest**, **-l**

Instead of providing the pod name or ID, use the last created pod. Note:
the last started pod can be from other users of Podman on the host
machine. (This option is not available with the remote Podman client,
including Mac and Windows (excluding WSL2) machines)

##  FORMAT DESCRIPTORS

For a full list of available descriptors, see podman-top(1)

##  EXAMPLES

Print top data for the specified pod. By default, `podman-pod-top`
prints data similar to `ps -ef`:

    $ podman pod top b031293491cc
    USER   PID   PPID   %CPU    ELAPSED             TTY   TIME   COMMAND
    root   1     0      0.000   2h5m38.737137571s   ?     0s     top
    root   8     0      0.000   2h5m15.737228361s   ?     0s     top

The output can be controlled by specifying format descriptors as
arguments after the pod.

Print the pod top data fields pid,seccomp, args and %C on the latest pod
created. (This -l option is not available with the remote Podman client,
including Mac and Windows (excluding WSL2) machines)

    $ podman pod top -l pid seccomp args %C
    PID   SECCOMP   COMMAND   %CPU
    1     filter    top       0.000
    1     filter    /bin/sh   0.000

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-pod(1)](podman-pod.html)**,
**ps(1)**, **seccomp(2)**, **proc(5)**, **capabilities(7)**

##  HISTORY

August 2018, Originally compiled by Peter Hunt <pehunt@redhat.com>


---

<a id='podman-pod-unpause'></a>

## podman-pod-unpause - Unpause one or more pods

##  NAME

podman-pod-unpause - Unpause one or more pods

##  SYNOPSIS

**podman pod unpause** \[*options*\] *pod* \...

##  DESCRIPTION

Unpauses all the paused processes in the containers of one or more pods.
You may use pod IDs or names as input.

##  OPTIONS

#### **\--all**, **-a**

Unpause all pods.

#### **\--latest**, **-l**

Instead of providing the pod name or ID, unpause the last created pod.
(This option is not available with the remote Podman client, including
Mac and Windows (excluding WSL2) machines)

##  EXAMPLE

Unpause pod with a given name:

    podman pod unpause mywebserverpod

Unpause pod with a given ID:

    podman pod unpause 860a4b23

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-pod(1)](podman-pod.html)**,
**[podman-pod-pause(1)](podman-pod-pause.html)**

##  HISTORY

July 2018, Originally compiled by Peter Hunt <pehunt@redhat.com>


---

